// LICENSE PLACEHOLDER

use std::any::Any;
use std::{thread, time};

use tokio::sync::oneshot;

use crate::cost_model::OpCode;
use crate::executor::*;
use crate::tensor_types::{TensorLike, Workload};

// Message Trait
pub trait MessageLike {}

// Raptor Top-level Message Type
#[derive(Debug)]
pub enum RaptorMessage<T>
where
    T: TensorLike + Clone,
{
    LoadfreeMSG(LoadfreeMessage<T>),
    PayloadMSG(PayloadMessage<T>),
}

impl<T> MessageLike for RaptorMessage<T> where T: TensorLike + Clone {}
impl<T> MessageLike for LoadfreeMessage<T> where T: TensorLike + Clone {}
impl<T> MessageLike for PayloadMessage<T> where T: TensorLike + Clone {}

// LoadfreeMessage without carrying payloads
///```
/// use raptors::prelude::*;
///
/// let msg: LoadfreeMessage<Workload> = build_loadfree_msg!("spawn", 1);
///
/// # // define a test function for type check
/// pub fn test_msg_type(msg: LoadfreeMessage<Workload>) -> bool {
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
    WorkloadMsg(T),
}

// PayloadMessage with payloads
#[derive(Debug)]
pub enum PayloadMessage<T>
where
    T: TensorLike + Clone,
{
    ComputeFunctorMsg {
        op: OpCode,
        lhs: T,
        rhs: T,
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
    Spawn(usize),
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
mod tests {
    use super::*;
}
