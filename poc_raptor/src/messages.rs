// LICENSE PLACEHOLDER

use std::any::Any;
use std::{thread, time};


// message trait is the definition of behaviours that the concept
// `message` shall obey, in other words, two properties referred.
// 1. sendable via mailboxes
// 2. tracable on its sender and receiver
//
// TODO:
// 1. make msg async to passing with non-blocking style
// 2. make it typed to build the effect system/handlers.
type Message = Box<dyn Any + Send>;


// dummy workload as dummy message but has a timeout for 
// emulating the execution
//
struct DummyWorkload {
    payload: usize,
}

impl DummyWorkload {
    pub fn new(payload: usize) -> DummyWorkload {
        return Self {
            payload: payload,
        }
    }
    
    pub fn payload(&self) -> usize {
        self.payload
    }

    pub fn mock_run(&self) -> () {
        thread::sleep(time::Duration::from_millis(1000));
    }
}


// unit tests
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn create_dummy_workload_test() {
        let load = DummyWorkload::new(16);
        assert_eq!(load.payload(), 16 as usize);
    }
}
