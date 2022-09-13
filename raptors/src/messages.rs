// LICENSE PLACEHOLDER

use std::any::Any;
use std::{thread, time};

use crate::cost_model::OpCode;
use crate::executor::*;
use crate::tensor_types::{TensorLike, Workload};

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
/// let msg: TypedMessage<Workload> = build_msg!("spawn", 1);
///
/// # // define a test function for type check
/// pub fn test_msg_type(msg: TypedMessage<Workload>) -> bool {
///     true
/// }
/// assert!(test_msg_type(msg.into()));
///```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypedMessage<T>
where
    T: TensorLike + Clone,
{
    SystemMsg(SystemCommand),
    ActorMsg(ActorCommand),
    WorkloadMsg(T),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActorCommand {
    Available(usize),
    PLACEHOLDER,
}

impl<T: TensorLike + Clone> Into<TypedMessage<T>> for ActorCommand {
    fn into(self) -> TypedMessage<T> {
        TypedMessage::<T>::ActorMsg(self)
    }
}

/// SystemMsg indicates the message of the system.
#[derive(Clone, Debug, PartialEq)]
pub struct ActorMsg {
    cmd: ActorCommand,
}

impl ActorMsg {
    pub fn new(cmd: ActorCommand) -> Self {
        return Self { cmd: cmd };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SystemCommand {
    HaltAll, // add more accurate destroy control msg when needed
    HaltOn(usize),
    Spawn(usize),
}

impl<T: TensorLike + Clone> Into<TypedMessage<T>> for SystemCommand {
    fn into(self) -> TypedMessage<T> {
        TypedMessage::<T>::SystemMsg(self)
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
