// LICENSE PLACEHOLDER

use crate::actors::Actor;
use std::any::Any;
use std::{thread, time};
use std::hash::{Hash, Hasher};
use std::sync::{Arc};

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
//===----------------------------------------------------------------------===//
// Review following code to adjust TODO
//===----------------------------------------------------------------------===//
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

//===----------------------------------------------------------------------===//
// Message of Actors:
//      1. Keyword: based on keyword to distinguish Action Msg and Error Msg
//      2. Workload: code and data, if needed
//===----------------------------------------------------------------------===//
#[derive(Debug)]
pub enum Keyword {
    //===-----------------------===//
    //  Action keyword
    //===-----------------------===//
    Create, 
    Stop, 
    Receive,
    Pass, // pass workload
    Transfer, // transfer onwership
    //===-----------------------===//
    // Error keyword
    //===-----------------------===//
    Timeout,
    WrongMsg,
    AlreadyStopped,
}

#[derive(Debug)]
pub struct Workload {
    code: String, // Note!!! Change here
    data: String, // Note!!! Change here
}

#[derive(Debug)]
pub struct ActorContent {
    keyword: Keyword,
    wordload: Workload,
}
impl std::fmt::Display for ActorContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.keyword)
    }
}

#[derive(Debug)]
pub struct ActorMsg {
    actor_from: Actor,
    actor_to: Actor,
    content: ActorContent,
}
impl std::fmt::Display for ActorMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?},{:?},{:?}", self.actor_from, self.actor_to, self.content.keyword)
    }
}

impl ActorMsg {
    pub fn new<T>(from: Actor, to: Actor, content: ActorContent) -> ActorMsg {
        return Self {
            actor_from: from,
            actor_to: to,
            content: content,
        }
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
