// LICENSE PLACEHOLDER
//
use chrono::{DateTime, Utc}; // use chrono for nano-precision time
use polars::df;
use polars::prelude::*;
use std::fs::File;
use tokio::sync::mpsc::{self, Receiver, Sender};
use uuid::Uuid;

use crate::messages::{SystemCommand, TypedMessage};

// TODO(short-term): we need to construct a new TypedMessage - LogMsg

// Logger, a dedicated actor to record actions of system and actors
pub struct Logger {
    id: Uuid,
    mbx: Vec<TypedMessage>,
    tx: Sender<TypedMessage>,
    rx: Receiver<TypedMessage>,
    table: polars::frame::DataFrame,
}

// Profiler should access its own mbx to retrieve msg
impl Logger {
    pub fn new() -> Logger {
        let uuid = Uuid::new_v4();
        let mbx = vec![];
        let (tx, rx) = mpsc::channel::<TypedMessage>(16);
        let start = Utc::now();
        let formatted = Logger::format_time(start);
        // TODO(maybe): convert to chunkedarry/series in Polaris to contruct the dataframe
        let df = df!("Time" => vec![formatted], "ActorId" => vec![uuid.to_string()], "Type" => vec!["System Operation"],
            "Operation" => vec!["Start Logger"], "Info" => vec![""])
        .expect("Fail to create Logger's table");
        return Self {
            id: uuid,
            mbx: mbx,
            tx: tx,
            rx: rx,
            table: df,
        };
    }

    pub fn format_time(timestamp: DateTime<Utc>) -> String {
        format!("{}", timestamp.format("%Y/%m/%d-%H:%M:%S%.3f"))
    }

    pub fn time_duration(start: DateTime<Utc>, end: DateTime<Utc>) -> i64 {
        (end - start)
            .num_microseconds()
            .expect("Fail to compute the time duration")
    }

    pub fn send(&mut self, msg: TypedMessage, sender: Option<Sender<TypedMessage>>) {
        match sender {
            Some(new_tx) => self.tx = new_tx,
            None => (),
        }
        self.tx.try_send(msg).expect("Fail to send the msg")
    }

    pub fn receive(&mut self) {
        let msg = self.rx.try_recv().expect("Received not valid msg");
        self.mbx.push(msg);
    }

    // TODO: later we need to control the 'loop': add break and jump-in control
    // TODO: currently each time we could only receive one message
    //       we could combine 'receive' and 'profiling' into one single function which keeps running
    pub fn profiling(&mut self) {
        // Using 'loop' here, since we want Logger keeping working
        'start_profiling: loop {
            if self.mbx.len() > 0 {
                // not a good 'if condition'
                let msg = self.mbx.remove(0);
                self.recording(msg);
            } else {
                break;
            }
        }
    }

    pub fn recording(&mut self, msg: TypedMessage) {
        let curr_time = Utc::now();
        match msg {
            TypedMessage::SystemMsg(cmd) => match cmd {
                SystemCommand::DummySysCmd => {
                    let data = df!("Time" => vec![Logger::format_time(curr_time)], "ActorId" => vec!["100".to_string()], "Type" => vec!["SystemMsg"],
                    "Operation" => vec!["SystemCommand"], "Info" => vec!["DummySysCmd"])
                    .expect("Fail to log SystemMsg");
                    // TODO: FIX here, no idea why 'vstack_mut' not work
                    if self.table.vstack_mut(&data).is_err() {
                        panic!("Fail to cancatenate dataframes with SystemMsg(DummySysCmd)");
                    }
                }
                SystemCommand::CreateActor(num, liter) => {
                    let info = format!("CreateActor-{}-{}", num.to_string(), liter);
                    let data = df!("Time" => vec![Logger::format_time(curr_time)], "ActorId" => vec!["100".to_string()], "Type" => vec!["SystemMsg"],
                    "Operation" => vec!["SystemCommand"], "Info" => vec![&info[..]])
                    .expect("Fail to log SystemMsg");
                    if self.table.vstack_mut(&data).is_err() {
                        panic!("Fail to cancatenate dataframes with SystemMsg(CreateActor)");
                    }
                }
                SystemCommand::DestroyAllActors => {
                    let data = df!("Time" => vec![Logger::format_time(curr_time)], "ActorId" => vec!["100".to_string()], "Type" => vec!["SystemMsg"],
                    "Operation" => vec!["SystemCommand"], "Info" => vec!["DestroyAllActors"])
                    .expect("Fail to log SystemMsg");
                    self.table.vstack_mut(&data);
                    if self.table.vstack_mut(&data).is_err() {
                        panic!("Fail to cancatenate dataframes with SystemMsg(DestroyAllActors)");
                    }
                }
                _ => {
                    panic!("Not a valid SystemCommand sent to Profiler")
                }
            },
            TypedMessage::ActorMsg => {
                let data = df!("Time" => vec![Logger::format_time(curr_time)], "ActorId" => vec!["100".to_string()], "Type" => vec!["ActorMsg"],
                        "Operation" => vec!["No op"], "Info" => vec!["No info"])
                        .expect("Fail to log ActorMsg");
                if self.table.vstack_mut(&data).is_err() {
                    panic!("Fail to cancatenate dataframes with ActorMsg");
                }
            }
            TypedMessage::WorkloadMsg(workload) => {
                let op = workload.op().to_string();
                let payload = &(workload.payload().to_string())[..];
                let data = df!("Time" => vec![Logger::format_time(curr_time)], "ActorId" => vec!["100".to_string()], "Type" => vec!["WorkloadMsg"],
                    "Operation" => vec![op], "Info" => vec![payload])
                    .expect("Fail to log WorkloadMsg");
                if self.table.vstack_mut(&data).is_err() {
                    panic!(
                        "Fail to cancatenate dataframes with WorkloadMsg({}-{})",
                        workload.op(),
                        workload.payload()
                    );
                }
            }
            _ => {
                panic!("Not a TypedMessage sent to Profiler")
            }
        }
    }

    pub fn write(&mut self, filename: &str) {
        let mut file = File::create(filename).expect("Fail to create file");

        CsvWriter::new(&mut file)
            .has_header(true)
            .with_delimiter(b',')
            .finish(&mut self.table)
            .expect("Fail to write table into csv")
    }
}

#[cfg(test)]
mod tests {
    use crate::messages::{OpCode, Workload};

    use super::*;

    #[test]
    fn create_profielr_test() {
        let mut profiler = Logger::new();
        assert_eq!(profiler.mbx.len(), 0);
        profiler.send(SystemCommand::default().into(), None);
        profiler.receive();
        assert_eq!(profiler.mbx.len(), 1);
    }

    #[test]
    fn write_table_test() {
        let mut logger = Logger::new();
        assert_eq!(logger.mbx.len(), 0);
        // send different messages
        logger.send(SystemCommand::default().into(), None);
        logger.send(TypedMessage::ActorMsg, None);
        logger.send(SystemCommand::DestroyAllActors.into(), None);
        logger.send(
            SystemCommand::CreateActor(4, String::from("raptor")).into(),
            None,
        );
        logger.send(Workload::new(4, OpCode::AddOp).into(), None);
        // start receiving
        logger.receive();
        logger.receive();
        logger.receive();
        logger.receive();
        logger.receive();
        assert_eq!(logger.mbx.len(), 5);
        // start profiling
        logger.profiling();
        logger.write("logging/logging_test.csv");
        assert_eq!(logger.mbx.len(), 0);
    }
}
