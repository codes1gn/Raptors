// LICENSE PLACEHOLDER

use tokio::sync::oneshot;

use crate::cost_model::OpCodeLike;
use crate::tensor_types::TensorLike;

// Message Trait
pub trait MessageLike {}

// Raptor Top-level Message Type
#[derive(Debug)]
pub enum RaptorMessage<T, O>
where
    T: TensorLike + Clone,
    O: OpCodeLike,
{
    LoadfreeMSG(LoadfreeMessage<T>),
    PayloadMSG(PayloadMessage<T, O>),
}

impl<T, O> MessageLike for RaptorMessage<T, O>
where
    T: TensorLike + Clone,
    O: OpCodeLike,
{
}
impl<T> MessageLike for LoadfreeMessage<T> where T: TensorLike + Clone {}
impl<T, O> MessageLike for PayloadMessage<T, O>
where
    T: TensorLike + Clone,
    O: OpCodeLike,
{
}

// LoadfreeMessage without carrying payloads
///```
/// use raptors::prelude::*;
///
/// let msg: LoadfreeMessage<MockTensor> = build_loadfree_msg!("spawn", "mock", 1);
///
/// # // define a test function for type check
/// pub fn test_msg_type(msg: LoadfreeMessage<MockTensor>) -> bool {
///     true
/// }
/// assert!(test_msg_type(msg.into()));
///```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadfreeMessage<T>
where
    T: TensorLike + Clone,
{
    SystemMsg(SystemCommand),
    ActorMsg(ActorCommand),
    MockTensorMsg(T),
}

// PayloadMessage with payloads
#[derive(Debug)]
pub enum PayloadMessage<T, O>
where
    T: TensorLike + Clone,
    O: OpCodeLike,
{
    ComputeFunctorMsg {
        op: O,
        lhs: T,
        rhs: T,
        respond_to: oneshot::Sender<T>,
    },
    UnaryComputeFunctorMsg {
        op: O,
        inp: T,
        respond_to: oneshot::Sender<T>,
    },
}

// SystemMsg that received and processed only by actor_system
#[derive(Clone, Debug, PartialEq)]
pub struct SystemMsg {
    cmd: SystemCommand,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SystemCommand {
    HaltAll,
    HaltOn(usize),
    // WIP first usize is typeid, second usize is cnt
    Spawn(usize, usize),
}

impl SystemMsg {
    pub fn new(cmd: SystemCommand) -> Self {
        return Self { cmd: cmd };
    }
}

impl<T: TensorLike + Clone> Into<LoadfreeMessage<T>> for SystemCommand {
    fn into(self) -> LoadfreeMessage<T> {
        LoadfreeMessage::<T>::SystemMsg(self)
    }
}

// ActorMsg received and processed by working actors
#[derive(Clone, Debug, PartialEq)]
pub struct ActorMsg {
    cmd: ActorCommand,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActorCommand {
    Available(usize),
    PLACEHOLDER,
}

impl ActorMsg {
    pub fn new(cmd: ActorCommand) -> Self {
        return Self { cmd: cmd };
    }
}

impl<T: TensorLike + Clone> Into<LoadfreeMessage<T>> for ActorCommand {
    fn into(self) -> LoadfreeMessage<T> {
        LoadfreeMessage::<T>::ActorMsg(self)
    }
}

#[cfg(test)]
mod tests {}
