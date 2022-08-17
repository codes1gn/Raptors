// LICENSE PLACEHOLDER
//
use polars::df;
use polars::export::num::ToPrimitive;
use polars::prelude::*;
use tokio::sync::mpsc::{self, Receiver, Sender};
use uuid::Uuid;
// use chrono; // use chrono for nano-precision time
use std::fs::File;

use crate::messages::{SystemCommand, SystemMsg, TypedMessage, WorkloadMsg};

// Profiler, a dedicated actor to record the performance of computation
pub struct Profiler {
    id: Uuid,
    mbx: Vec<TypedMessage>,
    tx: Sender<TypedMessage>,
    rx: Receiver<TypedMessage>,
    table: polars::frame::DataFrame,
    timestamp: usize, // TODO: replace timestamp with REAL_TIME
}

// Profiler should access its own mbx to retrieve msg
impl Profiler {
    pub fn new() -> Profiler {
        let new_uuid = Uuid::new_v4();
        let mbx = vec![];
        let (tx, rx) = mpsc::channel::<TypedMessage>(16);
        // TODO: convert to chunedarry/series in Polaris to contruct the dataframe
        // Currently match TypedMessage as
        // SystemMsg -> 0;  ActorMsg -> 1;  WorkloadMsg -> 2;
        // Here first element in Type is 0, since I consider CreateProfiler belongs to SystemMsg
        let df = df!("Time" => vec![0], "ActorId" => vec![0], "Type" => vec![0],
            "Operation" => vec![0], "Info" => vec![0])
        .expect("Fail to create Profiler's table");
        return Self {
            id: new_uuid,
            mbx: mbx,
            tx: tx,
            rx: rx,
            table: df,
            timestamp: 0,
        };
    }

    pub fn send(&self, msg: TypedMessage) {
        self.tx.try_send(msg).expect("Failt to send the msg")
    }

    pub fn receive(&mut self) {
        let msg = self.rx.try_recv().expect("Received not valid msg");
        self.mbx.push(msg);
    }

    pub fn profiling(&mut self) {
        // Using 'loop' here, since we want Profiler keeping working
        'start_profiling: loop {
            if self.mbx.len() > 0 {
                let msg = self.mbx.pop().expect("No msg in Profiler's mailbox");
                self.recording(msg);
            } else {
                break;
            }
        }
    }

    pub fn recording(&mut self, msg: TypedMessage) {
        match msg {
            TypedMessage::SystemMsg(cmd) => match cmd {
                SystemCommand::DummySysCmd => {
                    let curr_time = self.timestamp + 1;
                    self.timestamp = curr_time;
                    let data = df!("Time" => vec![curr_time.to_u32()], "ActorId" => vec![100], "Type" => vec![0],
                    "Operation" => vec![0], "Info" => vec![0])
                    .expect("Fail to profile SystemMsg");
                    // TODO: FIX here, no idea why 'vstack_mut' not work
                    if self.table.vstack_mut(&data).is_err() {
                        panic!("Fail to cancatenate dataframes");
                    }
                }
                SystemCommand::CreateActor(num, _) => {
                    let curr_time = self.timestamp + 1;
                    self.timestamp = curr_time;
                    let data = df!("Time" => vec![curr_time.to_u32()], "ActorId" => vec![100], "Type" => vec![0],
                    "Operation" => vec![1], "Info" => vec![num.to_u32()])
                    .expect("Fail to profile SystemMsg");
                    self.table.vstack_mut(&data);
                }
                SystemCommand::DestroyAllActors => {
                    let curr_time = self.timestamp + 1;
                    self.timestamp = curr_time;
                    let data = df!("Time" => vec![curr_time.to_u32()], "ActorId" => vec![100], "Type" => vec![0],
                    "Operation" => vec![2], "Info" => vec![0])
                    .expect("Fail to profile SystemMsg");
                    let a = self.table.vstack_mut(&data);
                }
                _ => {
                    panic!("Not a valid SystemCommand sent to Profiler")
                }
            },
            TypedMessage::ActorMsg => {
                let curr_time = self.timestamp + 1;
                self.timestamp = curr_time;
                let data = df!("Time" => vec![curr_time.to_u32()], "ActorId" => vec![100], "Type" => vec![1],
                        "Operation" => vec![0], "Info" => vec![0])
                        .expect("Fail to profile ActorMsg");
                self.table.vstack_mut(&data);
            }
            TypedMessage::WorkloadMsg(workload) => {
                let curr_time = self.timestamp + 1;
                self.timestamp = curr_time;
                let data = df!("Time" => vec![curr_time.to_u32()], "ActorId" => vec![100], "Type" => vec![2],
                    "Operation" => vec![0], "Info" => vec![workload.payload().to_u32()])
                    .expect("Fail to profile WorkloadMsg");
                self.table.vstack_mut(&data);
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
    use super::*;

    #[test]
    fn create_profielr_test() {
        let mut profiler = Profiler::new();
        assert_eq!(profiler.mbx.len(), 0);
        profiler.send(SystemCommand::default().into());
        profiler.receive();
        assert_eq!(profiler.mbx.len(), 1);
    }

    #[test]
    #[should_panic]
    fn write_table_test() {
        let mut profiler = Profiler::new();
        assert_eq!(profiler.mbx.len(), 0);
        profiler.send(SystemCommand::default().into());
        profiler.send(TypedMessage::ActorMsg);
        profiler.send(SystemCommand::DestroyAllActors.into());
        profiler.receive();
        profiler.receive();
        profiler.receive();
        assert_eq!(profiler.mbx.len(), 3);
        profiler.profiling();
        profiler.write("profiling_test.csv");
        assert_eq!(profiler.mbx.len(), 0);
    }
}
