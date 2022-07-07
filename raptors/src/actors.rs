// LICENSE PLACEHOLDER
//
use crate::executor;
use crate::messages;
use uuid::Uuid;

// placehold for actors
#[derive(Debug)]
pub struct Actor {
    name: String,
    // TODO(long-term) use v5 uuid, and give a hardcoded namespace, for removing randomness, also to allow
    // testing
    id: Uuid,
}

impl Actor {
    pub fn new(name: &str) -> Actor {
        let new_uuid = Uuid::new_v4();
        return Self {
            name: String::from(name),
            id: new_uuid,
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
}

// unit tests
#[cfg(test)]

mod tests {
    use super::*;
    use std::{thread, time};

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
