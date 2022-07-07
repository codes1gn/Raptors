// LICENSE PLACEHOLDER

use std::any::Any;
use std::{thread, time};

// message trait is the definition of behaviours that the concept
// `message` shall obey, in other words, two properties referred.
// 1. sendable via mailboxes
// 2. tracable on its sender and receiver
//
// TODO(long-term):
// 1. make msg async to passing with non-blocking style
// 2. make it typed to build the effect system/handlers.
// 3. support Se/Des in future
// 4. consider stream processing and compression designs
#[allow(dead_code)]
type Message = Box<dyn Any + Send>;

// TODO(albert, short-term) complete the family of MessageTypes
// test with simple design at first
///```
/// use raptors::prelude::*;
///
/// let msg = TypedMessage::ActorMsg;
/// assert_eq!(msg, TypedMessage::ActorMsg);
///
/// let msg = SystemCommand::CreateActor;
/// assert_eq!(msg, SystemCommand::CreateActor);
///
/// # // define a test function for type check
/// pub fn test_msg_type(msg: TypedMessage) -> bool {
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
// TODO(short-term): add OpCode enums as class of workload
// considering some elementwise ops, and some binary ops;
//
// TODO(short-term): set up the cost-model, and make workload query
// payload capacity from it, by opcode; future should extend to polymorphic
// querying on both opcode and scale.
//
// TODO(long-term): extend this desing into typed messages
// 1. WorkloadMsg, contains bytecode modules
// 2. DataMsg, support data exchange
// 3. CommandMsg, operations that instruct the action of
// each actor
//
//
pub struct Workload {
    payload: usize,
    op: OpCode,
}

impl Workload {
    pub fn new(payload: usize, op: OpCode) -> Workload {
        return Self {
            payload: payload,
            op: op,
        };
    }

    pub fn payload(&self) -> usize {
        self.payload
    }

    pub fn op(&self) -> OpCode {
        self.op
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
/// ```
/// # // Test default function for OpCode
/// use raptors::messages::OpCode;
///
/// assert_eq!(OpCode::default(), OpCode::DummyOp);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OpCode {
    MatmulOp,
    ConvOp,
    AddOp,
    SubOp,
    ExpOp,
    SinOp,
    DummyOp,
}

impl Default for OpCode {
    fn default() -> Self {
        OpCode::DummyOp
    }
}
// TODO: More Ops to add; Other way to implement Opcode

// unit tests
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn create_dummy_workload_test() {
        let load = Workload::new(16, OpCode::AddOp);
        assert_eq!(load.payload(), 16 as usize);
        assert_eq!(load.op(), OpCode::AddOp);
    }

    #[test]
    fn worklaod_mock_run_test() {
        let load = Workload::new(16, OpCode::ConvOp);
        let now = time::Instant::now();
        load.mock_run();
        assert!(now.elapsed() >= time::Duration::from_millis(16));
        assert_eq!(load.op(), OpCode::ConvOp);
    }

    #[test]
    fn workload_ops_default_test() {
        let load = Workload::new(16, OpCode::default());
        assert_eq!(load.op(), OpCode::DummyOp);
    }

    #[test]
    fn workload_ops_matmul_test() {
        let load = Workload::new(16, OpCode::MatmulOp);
        assert_eq!(load.op(), OpCode::MatmulOp);
    }

    #[test]
    fn workload_ops_exp_test() {
        let load = Workload::new(16, OpCode::ExpOp);
        assert_eq!(load.op(), OpCode::ExpOp);
    }
}
