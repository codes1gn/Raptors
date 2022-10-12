// LICENSE PLACEHOLDER
use std::sync::Arc;
use tracing::{debug, info};
// use tracing::instrument;
// use tracing::{span, Level};
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

use std::cmp::Ordering;
use std::fmt::Debug;

use crate::build_loadfree_msg;
use crate::cost_model::OpCodeLike;
use crate::executor_types::ExecutorLike;
use crate::messages::{ActorCommand, LoadfreeMessage, PayloadMessage, RaptorMessage};
use crate::tensor_types::TensorLike;

// T: executor type
// U: Tensor type
// O: OpCode type
#[derive(Debug)]
pub struct Actor<T, U, O>
where
    T: ExecutorLike<TensorType = U, OpCodeType = O>,
    U: TensorLike + Clone + Debug,
    O: OpCodeLike + Debug,
{
    id: usize,
    uuid: Uuid,
    receiver: mpsc::Receiver<RaptorMessage<U, O>>,
    respond_to: mpsc::Sender<RaptorMessage<U, O>>,
    executor: T,
}

impl<T, U, O> Actor<T, U, O>
where
    T: ExecutorLike<TensorType = U, OpCodeType = O>,
    U: TensorLike + Clone + Debug,
    O: OpCodeLike + Debug,
{
    pub fn new(
        id: usize,
        receiver: mpsc::Receiver<RaptorMessage<U, O>>,
        respond_to: mpsc::Sender<RaptorMessage<U, O>>,
        // WIP executor's typeid
        typeid: usize,
    ) -> Self {
        let new_uuid = Uuid::new_v4();
        let mut exec = T::new_with_typeid(typeid);
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

    async fn fetch_and_handle(&mut self, msg: RaptorMessage<U, O>) -> Result<(), String> {
        match msg {
            RaptorMessage::LoadfreeMSG(_msg) => self.fetch_and_handle_message(_msg).await,
            RaptorMessage::PayloadMSG(_msg) => self.fetch_and_handle_payload(_msg).await,
        }
    }

    async fn fetch_and_handle_payload(&mut self, msg: PayloadMessage<U, O>) -> Result<(), String> {
        match msg {
            // WIP add non-ret unary msg, just return a u8 signal that receives
            // TODO maybe send a () is better
            PayloadMessage::NonRetUnaryComputeFunctorMsg {
                op,
                inp,
                inp_ready_checker,
                respond_to,
                respond_id,
            } => {
                // TODO need unary branch
                info!("::actors#{}::inp-ready-checker checking", self.id);
                inp_ready_checker.await;
                info!("::actors#{}::inp-ready-checker ready", self.id);
                info!("::actor#{}::enter-computation", self.id);
                let outs = self.on_unary_compute(op, inp).expect("compute failed");
                info!("::actor#{}::exit-computation", self.id);
                // respond_to.into_iter().map(|x| x.send(0u8) );
                respond_to
                    .into_iter()
                    .map(|x| {
                        info!(
                            "::actors#{}::out-ready-checker set-ready to var #{}",
                            self.id, respond_id
                        );
                        x.send(0u8);
                        ()
                    })
                    .collect::<()>();
                Ok(())
            }
            // TODO need MSG to handle unary operations
            PayloadMessage::UnaryComputeFunctorMsg {
                op,
                inp,
                respond_to,
            } => {
                // TODO need unary branch
                info!("::actor#{}::enter-computation", self.id);
                let outs = self.on_unary_compute(op, inp).expect("compute failed");
                info!("::actor#{}::exit-computation", self.id);
                respond_to.send(outs);
                Ok(())
            }
            PayloadMessage::ComputeFunctorMsg {
                op,
                lhs,
                rhs,
                respond_to,
            } => {
                // TODO need unary branch
                info!("::actor#{}::enter-computation", self.id);
                let outs = self
                    .on_binary_compute(op, lhs, rhs)
                    .expect("compute failed");
                info!("::actor#{}::exit-computation", self.id);
                respond_to.send(outs);
                Ok(())
            }
        }
    }

    async fn fetch_and_handle_message(&mut self, msg: LoadfreeMessage<U>) -> Result<(), String> {
        match msg {
            LoadfreeMessage::MockTensorMsg(_wkl) => {
                // info!("::actor#{}::COMPUTE {:?}", self.id, _wkl);
                self.on_simulate(_wkl)
            }
            LoadfreeMessage::ActorMsg(_amsg) => {
                info!("::actor#{}::HANDLE ActorMSG - {:#?}", self.id, _amsg);
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
                    info!("::actor#{}::receive msg from system", self.id);
                    let status = self.fetch_and_handle(_msg).await;
                }
                Err(TryRecvError::Empty) => {
                    let msg = build_loadfree_msg!("available", self.id);
                    // TODO update build_msg with generalmessage
                    self.respond_to.try_send(RaptorMessage::LoadfreeMSG(msg));
                    info!("::actor#{}::tell supervisor i am available", self.id);
                    match self.receiver.recv().await {
                        Some(_msg) => {
                            info!("::actor#{}::receive msg from system", self.id);
                            let status = self.fetch_and_handle(_msg).await;
                        }
                        None => {
                            info!("::actor#{}::DROPPED BY SUPERVISOR -> HALTING", self.id);
                            break 1;
                        }
                    }
                }
                Err(TryRecvError::Disconnected) => {
                    info!("::actor#{}::DROPPED BY SUPERVISOR -> HALTING", self.id);
                    break 1;
                }
                _ => (),
            }
        }
    }

    #[tracing::instrument(name = "actor::on_compute", skip(self, workload))]
    fn on_simulate(&mut self, workload: U) -> Result<(), String> {
        self.executor.mock_compute(workload);
        Ok(())
    }

    #[tracing::instrument(name = "actor::on_binary_compute", skip(self, lhs, rhs))]
    fn on_binary_compute(&mut self, op: O, lhs: Arc<U>, rhs: Arc<U>) -> Result<U, String> {
        let outs = self.executor.binary_compute(op, lhs, rhs);
        Ok(outs)
    }

    #[tracing::instrument(name = "actor::on_unary_compute", skip(self, operand))]
    fn on_unary_compute(&mut self, op: O, operand: Arc<U>) -> Result<U, String> {
        let outs = self.executor.unary_compute(op, operand);
        Ok(outs)
    }
}

impl<T, U, O> Drop for Actor<T, U, O>
where
    T: ExecutorLike<TensorType = U, OpCodeType = O>,
    U: TensorLike + Clone + Debug,
    O: OpCodeLike + Debug,
{
    fn drop(&mut self) {
        info!("::actor#{}::DROP", self.id);
    }
}

impl<T, U, O> PartialOrd for Actor<T, U, O>
where
    T: ExecutorLike<TensorType = U, OpCodeType = O>,
    U: TensorLike + Clone + Debug,
    O: OpCodeLike + Debug,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, U, O> Ord for Actor<T, U, O>
where
    T: ExecutorLike<TensorType = U, OpCodeType = O>,
    U: TensorLike + Clone + Debug,
    O: OpCodeLike + Debug,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

// TODO fix duplicate with uuid add to name
impl<T, U, O> PartialEq for Actor<T, U, O>
where
    T: ExecutorLike<TensorType = U, OpCodeType = O>,
    U: TensorLike + Clone + Debug,
    O: OpCodeLike + Debug,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T, U, O> Eq for Actor<T, U, O>
where
    T: ExecutorLike<TensorType = U, OpCodeType = O>,
    U: TensorLike + Clone + Debug,
    O: OpCodeLike + Debug,
{
}

// unit tests
#[cfg(test)]

mod tests {}
