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

// TODO complete the family of MessageTypes
// test with simple design at first
///```
/// use raptors::messages;
///
/// let msg = messages::TypedMessage::ActorMsg;
/// assert_eq!(msg, messages::TypedMessage::ActorMsg);
///
/// let msg = messages::SystemCommand::CreateActor;
/// assert_eq!(msg, messages::SystemCommand::CreateActor);
///
/// # // define a test function for type check
/// pub fn test_msg_type(msg: messages::TypedMessage) -> bool {
///     true
/// }
/// assert!(test_msg_type(msg.into()));
///```
#[derive(Clone, Debug, PartialEq)]
pub enum TypedMessage {
    SystemMsg(SystemCommand),
    ActorMsg,
}


#[derive(Clone, Debug, PartialEq)]
pub enum SystemCommand {
    CreateActor,
}


impl Into<TypedMessage> for SystemCommand {
    fn into(self) -> TypedMessage {
        TypedMessage::SystemMsg(self)
    }
}

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
        return Self { payload: payload };
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

// Definition for Opcode
//
//TODO: 
//  1. Replace 'Vec<u32>' with more suitable type
//  2. More dedicated functions for Ops
// TODO complete the family of Opcodes
// test with simple design at first
//
/// ```
/// use raptors::messages;
/// use crate::raptors::messages::SetOpcode;
/// 
/// let matmul = messages::Opcode::MatmulOp;
/// assert_eq!("Set the MatmulOp".to_owned(), matmul.new());
/// ```

#[derive(Clone, Debug)]
pub enum Opcode {
    MatmulOp,
}

pub trait SetOpcode {
    fn new(&self) -> String;
}

impl SetOpcode for Opcode {
    fn new(&self) -> String {
        match self {
            MatmulOp => {
                "Set the MatmulOp".to_owned()
            }
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
