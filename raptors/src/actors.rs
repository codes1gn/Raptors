// LICENSE PLACEHOLDER
//
use uuid::Uuid;
use tokio::sync::mpsc::{Sender, Receiver};

use crate::messages;
use crate::mailbox::*;
use crate::messages::{TypedMessage, SystemCommand};

// placehold for actors
#[derive(Debug)]
pub struct Actor {
    name: String,
    // TODO(long-term) use v5 uuid, and give a hardcoded namespace, for removing randomness, also to allow
    // testing
    id: Uuid,
    mbx: Mailbox,
}

impl Actor {
    pub fn new(name: &str) -> Actor {
        let new_uuid = Uuid::new_v4();
        let new_mbx = Mailbox::new();
        return Self {
            name: String::from(name),
            id: new_uuid,
            mbx: new_mbx,
        };
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    // TODO: make it message passing, test with inter-threads
    // TODO: gradually support higher granularity parallelism
    pub fn receive(&self, msg: messages::Workload) -> () {
        self.on_compute(msg);
    }

    fn on_compute(&self, workload: messages::Workload) -> () {
        workload.mock_run();
    }

    pub fn set_sender(&mut self, sender: Sender<TypedMessage>) {
        self.mbx.set_sender(sender);
    }

    pub fn send_msg(&self, msg: TypedMessage) {
        self.mbx.send(msg);
    }

    pub fn recv_msg(&mut self) -> Result<(), String> {
        self.mbx.receive()
    }

    pub fn deal_msg(&mut self) -> Result<String, String> {
        self.mbx.deal()
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
        let load = messages::Workload::new(16, messages::OpCode::AddOp);
        assert_eq!(load.payload(), 16 as usize);
    }

    #[test]
    fn workload_mock_run_test() {
        let load = messages::Workload::new(16, messages::OpCode::AddOp);
        let now = time::Instant::now();
        load.mock_run();
        assert!(now.elapsed() >= time::Duration::from_millis(16));
    }

    #[test]
    fn query_actor_name() {
        let actor = Actor::new("A");
        assert_eq!(actor.name(), "A");
    }

    #[test]
    fn receive_workload() {
        let actor = Actor::new("A");
        let load = messages::Workload::new(16, messages::OpCode::AddOp);
        let now = time::Instant::now();
        actor.receive(load);
        assert!(now.elapsed() >= time::Duration::from_millis(16));
    }
}

#[cfg(test)]
mod msg_passing_test {
    use super::*;

    #[test]
    fn send_among_mailboxes_test() {
        let mut actor1 = Actor::new("rap1");
        let mut actor2 = Actor::new("rap2");
        assert_eq!(actor1.mbx.len(), 0);
        assert_eq!(actor2.mbx.len(), 0);

        let msg = SystemCommand::CreateActor(4, String::from("raptor"));
        actor1.send_msg(msg.into());
        let res = actor1.recv_msg();
        assert_eq!(actor1.mbx.len(), 1);
        assert!(res.is_ok());

        let new_tx = actor2.mbx.sender();
        actor1.set_sender(new_tx);
        let msg = SystemCommand::CreateActor(4, String::from("raptor"));
        actor1.send_msg(msg.into());
        let res = actor2.recv_msg();
        assert_eq!(actor2.mbx.len(), 1);
        assert!(res.is_ok());
        let res = actor2.deal_msg();
        assert_eq!(res.unwrap(), "Received SystemMsg".to_owned());
    }
}
