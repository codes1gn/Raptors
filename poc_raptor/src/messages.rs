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
// 3. support Se/Des in future
// 4. consider stream processing and compression designs
type Message = Box<dyn Any + Send>;


// dummy workload as dummy message but has a timeout for 
// emulating the execution
//
// TODO: extend this desing into typed messages
// 1. WorkloadMsg, contains bytecode modules
// 2. DataMsg, support data exchange
// 3. CommandMsg, operations that instruct the action of 
// each actor
// 
//
pub struct DummyWorkload {
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

    // mock function that will fakely run for that period long
    // TODO:
    // 1. change signiture to return values
    // 2. values may use a value type that defined include possible results
    pub fn mock_run(&self) -> () {
        thread::sleep(time::Duration::from_millis(self.payload as u64));
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

    #[test]
    fn worklaod_mock_run_test() {
        let load = DummyWorkload::new(16);
        let now = time::Instant::now();
        load.mock_run();
        assert!(now.elapsed() >= time::Duration::from_millis(16));
    }
}
