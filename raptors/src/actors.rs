// LICENSE PLACEHOLDER
use log::{debug, info};
use tokio::sync::{mpsc, oneshot};
use uuid::{Urn, Uuid};

use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::Bytes;
use std::{thread, time};

use crate::mailbox::*;
use crate::messages::TypedMessage;
use crate::workloads::{OpCode, Workload};

// placehold for actors
#[derive(Debug)]
pub struct Actor {
    id: usize,
    uuid: Uuid,
    receiver: mpsc::Receiver<TypedMessage>,
}

impl Actor {
    pub fn new(id: usize, receiver: mpsc::Receiver<TypedMessage>) -> Self {
        let new_uuid = Uuid::new_v4();
        Actor {
            id: id,
            receiver: receiver,
            uuid: new_uuid,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    fn fetch_and_handle_message(&mut self, msg: TypedMessage) -> u32 {
        thread::sleep(time::Duration::from_millis(1000 as u64));
        info!("actor #{} - HANDLE MSG", self.id);
        match msg {
            TypedMessage::Testone => 1,
            TypedMessage::Testzero => 0,
            _ => panic!("message not implemented"),
        }
    }

    pub async fn run(&mut self) -> u32 {
        loop {
            info!("actor #{} - IDLE", self.id);
            match self.receiver.recv().await {
                Some(msg) => {
                    let status = self.fetch_and_handle_message(msg);
                    match status {
                        0 => {
                            info!("actor #{} - HALT", self.id);
                            break 0;
                        }
                        1 => info!("actor #{} - CONTINUE", self.id),
                        _ => panic!("not implemented #2424"),
                    }
                }
                None => {
                    info!("actor #{} - DROPPED BY SUPERVISOR -> HALTING", self.id);
                    break 1;
                }
                _ => (),
            }
        }
    }

    // pub fn start(&mut self) -> Result<(), String> {
    //     info!("Actor {:#?} is start running >>>", self.name);
    //     let status = loop {
    //         let msg = self.mbx.dequeue();
    //         match msg {
    //             Some(TypedMessage::WorkloadMsg(_wkl)) => {
    //                 info!("on processing {:#?}", _wkl);
    //                 self.on_compute(_wkl);
    //             }
    //             None => {
    //                 info!("Actor {:#?} is finish running >>>", self.name);
    //                 break Ok(());
    //             }
    //             _ => {
    //                 break Err(String::from("Unknown msg type for actor to process"));
    //             }
    //         }
    //     };
    //     status
    // }

    // fn on_compute(&self, workload: Workload) -> () {
    //     workload.mock_run();
    // }
}

impl Drop for Actor {
    fn drop(&mut self) {
        info!("actor #{} - DROP", self.id);
    }
}

impl PartialOrd for Actor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Actor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

// TODO fix duplicate with uuid add to name
impl PartialEq for Actor {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Actor {}

// unit tests
#[cfg(test)]

mod tests {
    use super::*;
    use std::time;

    // test visibility
    // #[test]
    // fn create_dummy_workload_test() {
    //     let load = Workload::new(OpCode::AddOp);
    //     assert_eq!(load.payload(), 11 as usize);
    // }

    // #[test]
    // fn workload_mock_run_test() {
    //     let load = Workload::new(OpCode::AddOp);
    //     let now = time::Instant::now();
    //     load.mock_run();
    //     assert!(now.elapsed() >= time::Duration::from_millis(11));
    // }
}
