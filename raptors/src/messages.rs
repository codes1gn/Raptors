// LICENSE PLACEHOLDER

use std::any::Any;
use std::{thread, time};

use crate::system::System;
use crate::workloads::OpCode;
use crate::workloads::Workload;

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
/// let msg = SystemCommand::CreateActor(1, String::from("raptor"));
/// assert_eq!(msg, SystemCommand::CreateActor(1, String::from("raptor")));
///
/// # // define a test function for type check
/// pub fn test_msg_type(msg: TypedMessage) -> bool {
///     true
/// }
/// assert!(test_msg_type(msg.into()));
///```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypedMessage {
    SystemMsg(SystemCommand),
    ActorMsg,
    WorkloadMsg(Workload),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SystemCommand {
    DummySysCmd,
    CreateActor(usize, String),
    DestroyAllActors, // add more accurate destroy control msg when needed
}

impl Into<TypedMessage> for SystemCommand {
    fn into(self) -> TypedMessage {
        TypedMessage::SystemMsg(self)
    }
}

impl Default for SystemCommand {
    fn default() -> Self {
        SystemCommand::DummySysCmd
    }
}

/// SystemMsg indicates the message of the system.
#[derive(Clone, Debug, PartialEq)]
pub struct SystemMsg {
    cmd: SystemCommand,
}

impl SystemMsg {
    pub fn new(cmd: SystemCommand) -> Self {
        return Self { cmd: cmd };
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;
}
