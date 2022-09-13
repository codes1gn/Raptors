// LICENSE PLACEHOLDER
use tracing::info;
// use tracing::instrument;
// use tracing::{span, Level};
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::{mpsc, oneshot};
use uuid::{Urn, Uuid};

use std::cmp::Ordering;
use std::collections::HashMap;
use std::marker::Send;
use std::str::Bytes;
use std::{thread, time};

use crate::build_msg;
use crate::executor::{Executor, ExecutorTrait, TensorTrait};
use crate::mailbox::*;
use crate::messages::{ActorCommand, TypedMessage};
use crate::workloads::{OpCode, Workload};

// placehold for actors
#[derive(Debug)]
pub struct Actor<T, U>
where
    T: ExecutorTrait<TensorLike = U>,
    U: TensorTrait + Clone,
{
    id: usize,
    uuid: Uuid,
    receiver: mpsc::Receiver<TypedMessage<U>>,
    respond_to: mpsc::Sender<TypedMessage<U>>,
    executor: T,
}

impl<T, U> Actor<T, U>
where
    T: ExecutorTrait<TensorLike = U>,
    U: TensorTrait + Clone,
{
    pub fn new(
        id: usize,
        receiver: mpsc::Receiver<TypedMessage<U>>,
        respond_to: mpsc::Sender<TypedMessage<U>>,
    ) -> Self {
        let new_uuid = Uuid::new_v4();
        let exec = T::new();
        Actor {
            id: id,
            receiver: receiver,
            uuid: new_uuid,
            respond_to: respond_to,
            executor: exec,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    fn fetch_and_handle_message(&mut self, msg: TypedMessage<U>) -> Result<(), String> {
        match msg {
            TypedMessage::WorkloadMsg(_wkl) => {
                // info!("ACT#{} - COMPUTE {:?}", self.id, _wkl);
                self.on_compute(_wkl)
            }
            TypedMessage::ActorMsg(_amsg) => {
                info!("ACT#{} - HANDLE ActorMSG - {:#?}", self.id, _amsg);
                Ok(())
            }
            _ => panic!("Unknown actormessage not implemented"),
        }
    }

    #[tracing::instrument(name = "actor::run", skip(self))]
    pub async fn run(&mut self) -> u32 {
        loop {
            match self.receiver.try_recv() {
                Ok(_msg) => {
                    info!("ACT#{} - receive msg from system ENTER", self.id);
                    let status = self.fetch_and_handle_message(_msg);
                    info!("ACT#{} - receive msg from system EXIT", self.id);
                }
                Err(TryRecvError::Empty) => {
                    let msg = build_msg!("available", self.id);
                    self.respond_to.try_send(msg);
                    info!("ACT#{} - tell supervisor i am available", self.id);
                    match self.receiver.recv().await {
                        Some(_msg) => {
                            info!("ACT#{} - receive msg from system", self.id);
                            let status = self.fetch_and_handle_message(_msg);
                        }
                        None => {
                            info!("ACT#{} - DROPPED BY SUPERVISOR -> HALTING", self.id);
                            break 1;
                        }
                    }
                }
                Err(TryRecvError::Disconnected) => {
                    info!("ACT#{} - DROPPED BY SUPERVISOR -> HALTING", self.id);
                    break 1;
                }
                _ => (),
            }
        }
    }

    #[tracing::instrument(name = "actor::on_compute", skip(self, workload))]
    fn on_compute(&self, workload: U) -> Result<(), String> {
        // ExecutorTrait::compute_wkl(&mut self.executor, workload);
        self.executor.compute_it(workload);
        Ok(())
    }
}

impl<T, U> Drop for Actor<T, U>
where
    T: ExecutorTrait<TensorLike = U>,
    U: TensorTrait + Clone,
{
    fn drop(&mut self) {
        info!("ACT#{} - DROP", self.id);
    }
}

impl<T, U> PartialOrd for Actor<T, U>
where
    T: ExecutorTrait<TensorLike = U>,
    U: TensorTrait + Clone,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, U> Ord for Actor<T, U>
where
    T: ExecutorTrait<TensorLike = U>,
    U: TensorTrait + Clone,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

// TODO fix duplicate with uuid add to name
impl<T, U> PartialEq for Actor<T, U>
where
    T: ExecutorTrait<TensorLike = U>,
    U: TensorTrait + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T, U> Eq for Actor<T, U>
where
    T: ExecutorTrait<TensorLike = U>,
    U: TensorTrait + Clone,
{
}

// unit tests
#[cfg(test)]

mod tests {
    use super::*;
}
