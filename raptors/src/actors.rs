// LICENSE PLACEHOLDER
use log::{debug, info};
use uuid::{Urn, Uuid};

use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::Bytes;

use crate::mailbox::*;
use crate::messages::TypedMessage;
use crate::workloads::{OpCode, Workload};

// placehold for actors
#[derive(Debug, Eq)]
pub struct Actor {
    name: String,
    // TODO(long-term) use v5 uuid, and give a hardcoded namespace, for removing randomness, also to allow
    // testing
    id: Uuid,
    pub addr: Address,
    // TODO(albert), how to access mailboxes from actor if mailboxes are owned by
    // system/context/environment
    pub mbx: Mailbox,
}

impl Actor {
    pub fn new(name: &str) -> Actor {
        let new_uuid = Uuid::new_v4();
        // TODO addr use uuid instead of string
        // let _addr = new_uuid.clone().hyphenated().to_string();
        return Self {
            name: String::from(name),
            id: new_uuid,
            addr: Address::new(new_uuid),
            mbx: Mailbox::new(),
        };
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn addr(&self) -> Address {
        self.addr.clone()
    }

    pub fn mailbox(&self) -> &Mailbox {
        &self.mbx
    }

    pub fn mailbox_mut(&mut self) -> &mut Mailbox {
        &mut self.mbx
    }

    pub fn start(&mut self) -> Result<(), String> {
        info!("Actor {:#?} is start running >>>", self.name);
        let status = loop {
            let msg = self.mbx.dequeue();
            match msg {
                Some(TypedMessage::WorkloadMsg(_wkl)) => {
                    info!("on processing {:#?}", _wkl);
                    self.on_compute(_wkl);
                }
                None => {
                    info!("Actor {:#?} is finish running >>>", self.name);
                    break Ok(());
                }
                _ => {
                    break Err(String::from("Unknown msg type for actor to process"));
                }
            }
        };
        status
    }

    fn on_compute(&self, workload: Workload) -> () {
        workload.mock_run();
    }
}

impl PartialOrd for Actor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Actor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

// TODO fix duplicate with uuid add to name
impl PartialEq for Actor {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

// unit tests
#[cfg(test)]

mod tests {
    use super::*;
    use std::time;

    // test visibility
    #[test]
    fn create_dummy_workload_test() {
        let load = Workload::new(OpCode::AddOp);
        assert_eq!(load.payload(), 11 as usize);
    }

    #[test]
    fn workload_mock_run_test() {
        let load = Workload::new(OpCode::AddOp);
        let now = time::Instant::now();
        load.mock_run();
        assert!(now.elapsed() >= time::Duration::from_millis(11));
    }

    #[test]
    fn query_actor_name() {
        let actor = Actor::new("A");
        assert_eq!(actor.name(), "A");
    }

    #[test]
    fn receive_workload() {
        let actor = Actor::new("A");
        let load = Workload::new(OpCode::AddOp);
        let now = time::Instant::now();
        actor.on_compute(load);
        assert!(now.elapsed() >= time::Duration::from_millis(11));
    }
}
