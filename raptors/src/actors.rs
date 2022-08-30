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

    fn fetch_and_handle_message(&mut self, msg: TypedMessage) -> Result<(), String> {
        thread::sleep(time::Duration::from_millis(1000 as u64));
        match msg {
            TypedMessage::WorkloadMsg(_wkl) => {
                info!("actor #{} - COMPUTE {:#?}", self.id, _wkl);
                self.on_compute(_wkl)
            }
            TypedMessage::ActorMsg(_amsg) => {
                info!("actor #{} - HANDLE ActorMSG - {:#?}", self.id, _amsg);
                Ok(())
            }
            _ => panic!("Unknown actormessage not implemented"),
        }
    }

    pub async fn run(&mut self) -> u32 {
        loop {
            info!("actor #{} - IDLE", self.id);
            match self.receiver.recv().await {
                Some(msg) => {
                    let status = self.fetch_and_handle_message(msg);
                }
                None => {
                    // if senders are dropped, should halt the corresponding actor
                    info!("actor #{} - DROPPED BY SUPERVISOR -> HALTING", self.id);
                    break 1;
                }
                _ => (),
            }
        }
    }

    fn on_compute(&self, workload: Workload) -> Result<(), String> {
        workload.mock_run();
        Ok(())
    }
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
}
