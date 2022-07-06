// LICENSE PLACEHOLDER
//
use crate::executor;
use crate::messages;

// placehold for actors
#[derive(Debug)]
pub struct Actor {
    // TODO use &str is better for short piece of name string
    name: String,
    id: usize,
}

impl Actor {
    pub fn new(name: &str, id: usize) -> Actor {
        return Self { name: String::from(name), id: id };
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    // TODO: make it message passing, test with inter-threads
    // TODO: gradually support higher granularity parallelism
    pub fn receive(&self, msg: messages::DummyWorkload) -> () {
        self.on_compute(msg);
    }

    fn on_compute(&self, workload: messages::DummyWorkload) -> () {
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
        let load = messages::DummyWorkload::new(16);
        assert_eq!(load.payload(), 16 as usize);
    }

    #[test]
    fn workload_mock_run_test() {
        let load = messages::DummyWorkload::new(16);
        let now = time::Instant::now();
        load.mock_run();
        assert!(now.elapsed() >= time::Duration::from_millis(16));
    }

    #[test]
    fn query_actor_id() {
        let actor = Actor::new("A", 17);
        assert_eq!(actor.id(), 17);
    }

    #[test]
    fn query_actor_name() {
        let actor = Actor::new("A", 17);
        assert_eq!(actor.name(), "A");
    }

    #[test]
    fn receive_workload() {
        let actor = Actor::new("A", 1);
        let load = messages::DummyWorkload::new(16);
        let now = time::Instant::now();
        actor.receive(load);
        assert!(now.elapsed() >= time::Duration::from_millis(16));
    }
}
