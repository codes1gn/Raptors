// LICENSE PLACEHOLDER
use tracing::info;
// use tracing::instrument;
// use tracing::{span, Level};
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::{mpsc, oneshot};
use uuid::{Urn, Uuid};

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::Send;
use std::str::Bytes;
use std::{thread, time};

use crate::build_msg;
use crate::cost_model::OpCode;
use crate::executor::{Executor, ExecutorLike};
use crate::mailbox::*;
use crate::messages::{ActorCommand, LoadfreeMessage, MessageLike, PayloadMessage, RaptorMessage};
use crate::tensor_types::{TensorLike, Workload};

// placehold for actors
#[derive(Debug)]
pub struct Actor<T, U>
where
    T: ExecutorLike<TensorType = U>,
    U: TensorLike + Clone + Debug,
{
    id: usize,
    uuid: Uuid,
    receiver: mpsc::Receiver<RaptorMessage<U>>,
    respond_to: mpsc::Sender<RaptorMessage<U>>,
    executor: T,
}

impl<T, U> Actor<T, U>
where
    T: ExecutorLike<TensorType = U>,
    U: TensorLike + Clone + Debug,
{
    pub fn new(
        id: usize,
        receiver: mpsc::Receiver<RaptorMessage<U>>,
        respond_to: mpsc::Sender<RaptorMessage<U>>,
    ) -> Self {
        let new_uuid = Uuid::new_v4();
        let mut exec = T::new();
        exec.init();
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

    fn fetch_and_handle(&mut self, msg: RaptorMessage<U>) -> Result<(), String> {
        match msg {
            RaptorMessage::LoadfreeMSG(_msg) => self.fetch_and_handle_message(_msg),
            RaptorMessage::PayloadMSG(_msg) => self.fetch_and_handle_payload(_msg),
        }
    }

    fn fetch_and_handle_payload(&mut self, msg: PayloadMessage<U>) -> Result<(), String> {
        match msg {
            PayloadMessage::ComputeFunctorMsg {
                op,
                lhs,
                rhs,
                respond_to,
            } => {
                // TODO need unary branch
                println!("received");
                let outs = self.on_compute_new(op, lhs, rhs).expect("compute failed");
                println!("sending callback");
                respond_to.send(outs);
                println!("Dona - sending callback");
                // TODO sendback
                Ok(())
            }
        }
    }

    fn fetch_and_handle_message(&mut self, msg: LoadfreeMessage<U>) -> Result<(), String> {
        match msg {
            LoadfreeMessage::WorkloadMsg(_wkl) => {
                // info!("ACT#{} - COMPUTE {:?}", self.id, _wkl);
                self.on_compute(_wkl)
            }
            // LoadfreeMessage::ComputeFunctorMsg { op, lhs, rhs, respond_to } => {
            // LoadfreeMessage::ComputeFunctorMsg { op, lhs, rhs } => {
            //     // TODO need unary branch
            //     println!("received");
            //     let outs = self.on_compute_new(op, lhs, rhs);
            //     // TODO sendback
            //     Ok(())
            // },
            LoadfreeMessage::ActorMsg(_amsg) => {
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
                    let status = self.fetch_and_handle(_msg);
                    info!("ACT#{} - receive msg from system EXIT", self.id);
                }
                Err(TryRecvError::Empty) => {
                    let msg = build_msg!("available", self.id);
                    // TODO update build_msg with generalmessage
                    self.respond_to.try_send(RaptorMessage::LoadfreeMSG(msg));
                    info!("ACT#{} - tell supervisor i am available", self.id);
                    match self.receiver.recv().await {
                        Some(_msg) => {
                            info!("ACT#{} - receive msg from system", self.id);
                            let status = self.fetch_and_handle(_msg);
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
    fn on_compute(&mut self, workload: U) -> Result<(), String> {
        self.executor.compute_mock(workload);
        Ok(())
    }

    #[tracing::instrument(name = "actor::on_compute", skip(self, lhs, rhs))]
    fn on_compute_new(&mut self, op: OpCode, lhs: U, rhs: U) -> Result<U, String> {
        let outs = self.executor.compute_binary(op, lhs, rhs);
        Ok(outs)
    }
}

impl<T, U> Drop for Actor<T, U>
where
    T: ExecutorLike<TensorType = U>,
    U: TensorLike + Clone + Debug,
{
    fn drop(&mut self) {
        info!("ACT#{} - DROP", self.id);
    }
}

impl<T, U> PartialOrd for Actor<T, U>
where
    T: ExecutorLike<TensorType = U>,
    U: TensorLike + Clone + Debug,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, U> Ord for Actor<T, U>
where
    T: ExecutorLike<TensorType = U>,
    U: TensorLike + Clone + Debug,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

// TODO fix duplicate with uuid add to name
impl<T, U> PartialEq for Actor<T, U>
where
    T: ExecutorLike<TensorType = U>,
    U: TensorLike + Clone + Debug,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T, U> Eq for Actor<T, U>
where
    T: ExecutorLike<TensorType = U>,
    U: TensorLike + Clone + Debug,
{
}

// unit tests
#[cfg(test)]

mod tests {
    use super::*;
}
