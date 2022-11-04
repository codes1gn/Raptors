// LICENSE PLACEHOLDER
use std::sync::{Arc, RwLock};
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

    async fn fetch_and_handle_async(&mut self, msg: RaptorMessage<U, O>) -> Result<(), String> {
        match msg {
            RaptorMessage::LoadfreeMSG(_msg) => self.fetch_and_handle_message_async(_msg).await,
            RaptorMessage::PayloadMSG(_msg) => self.fetch_and_handle_payload_async(_msg).await,
        }
    }

    // fn fetch_and_handle(&mut self, msg: RaptorMessage<U, O>) -> Result<(), String> {
    //     let future1 = async move {
    //         self.fetch_and_handle_async(msg).await
    //     };
    // }

    fn fetch_and_handle(&mut self, msg: RaptorMessage<U, O>) -> Result<(), String> {
        match msg {
            RaptorMessage::LoadfreeMSG(_msg) => self.fetch_and_handle_message(_msg),
            RaptorMessage::PayloadMSG(_msg) => self.fetch_and_handle_payload(_msg),
        }
    }

    async fn fetch_and_handle_payload_async(
        &mut self,
        msg: PayloadMessage<U, O>,
    ) -> Result<(), String> {
        match msg {
            PayloadMessage::NonRetTenaryComputeFunctorMsg {
                op,
                first,
                second,
                third,
                out,
                first_ready_checker,
                second_ready_checker,
                third_ready_checker,
                respond_to,
                respond_id,
                ..
            } => {
                info!("::actors#{}::first-ready-checker checking", self.id);
                first_ready_checker.await;
                info!("::actors#{}::first-ready-checker ready", self.id);

                info!("::actors#{}::second-ready-checker checking", self.id);
                second_ready_checker.await;
                info!("::actors#{}::second-ready-checker ready", self.id);

                info!("::actors#{}::third-ready-checker checking", self.id);
                third_ready_checker.await;
                info!("::actors#{}::third-ready-checker ready", self.id);

                info!("::actor#{}::enter-computation", self.id);
                let if_compute_successed = self.on_tenary_compute_v2(op, first, second, third, out);
                // WIP let outs = self.on_unary_compute(op, inp).expect("compute failed");
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
            PayloadMessage::NonRetBinaryComputeFunctorMsg {
                op,
                lhs,
                rhs,
                out,
                lhs_ready_checker,
                rhs_ready_checker,
                respond_to,
                respond_id,
                ..
            } => {
                info!("::actors#{}::lhs-ready-checker checking", self.id);
                lhs_ready_checker.await;
                info!("::actors#{}::lhs-ready-checker ready", self.id);

                info!("::actors#{}::rhs-ready-checker checking", self.id);
                rhs_ready_checker.await;
                info!("::actors#{}::rhs-ready-checker ready", self.id);

                info!("::actor#{}::enter-computation", self.id);
                let if_compute_successed = self.on_binary_compute_v2(op, lhs, rhs, out);
                // WIP let outs = self.on_unary_compute(op, inp).expect("compute failed");
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
            // WIP add non-ret unary msg, just return a u8 signal that receives
            // TODO maybe send a () is better
            PayloadMessage::NonRetUnaryComputeFunctorMsg {
                op,
                inp,
                out,
                inp_ready_checker,
                respond_to,
                respond_id,
                ..
            } => {
                // TODO need unary branch
                info!("::actors#{}::inp-ready-checker checking", self.id);
                inp_ready_checker.await;
                info!("::actors#{}::inp-ready-checker ready", self.id);
                info!("::actor#{}::enter-computation", self.id);
                let if_compute_successed = self.on_unary_compute_v2(op, inp, out);
                // WIP let outs = self.on_unary_compute(op, inp).expect("compute failed");
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
            PayloadMessage::DMAOperationMsg {
                op,
                inp,
                out,
                inp_ready_checker,
                respond_to,
                respond_id,
                shape,
                ..
            } => {
                // TODO need unary branch
                info!("::actors#{}::inp-ready-checker checking - DMA", self.id);
                inp_ready_checker.await;
                info!("::actors#{}::inp-ready-checker ready - DMA", self.id);
                info!("::actor#{}::enter-computation - DMA", self.id);
                let if_compute_successed = self.on_dma_operation(op, inp, out, shape);
                // WIP let outs = self.on_unary_compute(op, inp).expect("compute failed");
                info!("::actor#{}::exit-computation - DMA", self.id);
                // respond_to.into_iter().map(|x| x.send(0u8) );
                respond_to
                    .into_iter()
                    .map(|x| {
                        info!(
                            "::actors#{}::out-ready-checker set-ready to var #{} - DMA",
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

    fn fetch_and_handle_payload(&mut self, msg: PayloadMessage<U, O>) -> Result<(), String> {
        match msg {
            PayloadMessage::NonRetTenaryComputeFunctorMsg {
                op,
                first,
                second,
                third,
                out,
                mut first_ready_checker,
                mut second_ready_checker,
                mut third_ready_checker,
                respond_to,
                respond_id,
                ..
            } => {
                info!("::actors#{}::first-ready-checker checking", self.id);
                first_ready_checker.blocking_recv();
                // first_ready_checker.try_recv();
                info!("::actors#{}::first-ready-checker ready", self.id);

                info!("::actors#{}::second-ready-checker checking", self.id);
                second_ready_checker.blocking_recv();
                info!("::actors#{}::second-ready-checker ready", self.id);

                info!("::actors#{}::third-ready-checker checking", self.id);
                third_ready_checker.blocking_recv();
                info!("::actors#{}::third-ready-checker ready", self.id);

                info!("::actor#{}::enter-computation", self.id);
                let if_compute_successed = self.on_tenary_compute_v2(op, first, second, third, out);
                // WIP let outs = self.on_unary_compute(op, inp).expect("compute failed");
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
            PayloadMessage::NonRetBinaryComputeFunctorMsg {
                op,
                lhs,
                rhs,
                out,
                mut lhs_ready_checker,
                mut rhs_ready_checker,
                respond_to,
                respond_id,
                ..
            } => {
                info!("::actors#{}::lhs-ready-checker checking", self.id);
                lhs_ready_checker.blocking_recv();
                info!("::actors#{}::lhs-ready-checker ready", self.id);

                info!("::actors#{}::rhs-ready-checker checking", self.id);
                rhs_ready_checker.blocking_recv();
                info!("::actors#{}::rhs-ready-checker ready", self.id);

                info!("::actor#{}::enter-computation", self.id);
                let if_compute_successed = self.on_binary_compute_v2(op, lhs, rhs, out);
                // WIP let outs = self.on_unary_compute(op, inp).expect("compute failed");
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
            // WIP add non-ret unary msg, just return a u8 signal that receives
            // TODO maybe send a () is better
            PayloadMessage::NonRetUnaryComputeFunctorMsg {
                op,
                inp,
                out,
                mut inp_ready_checker,
                respond_to,
                respond_id,
                ..
            } => {
                // TODO need unary branch
                info!("::actors#{}::inp-ready-checker try-recv checking", self.id);
                // use tokio::runtime::Handle;
                // let handle = Handle::current();
                // handle.spawn(async {
                //     inp_ready_checker.await;
                // });
                inp_ready_checker.blocking_recv();
                info!("::actors#{}::inp-ready-checker try-recv ready", self.id);
                info!("::actor#{}::enter-computation", self.id);
                let if_compute_successed = self.on_unary_compute_v2(op, inp, out);
                // WIP let outs = self.on_unary_compute(op, inp).expect("compute failed");
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
            PayloadMessage::DMAOperationMsg {
                op,
                inp,
                out,
                mut inp_ready_checker,
                respond_to,
                respond_id,
                shape,
                ..
            } => {
                // TODO need unary branch
                info!("::actors#{}::inp-ready-checker checking - DMA", self.id);
                inp_ready_checker.blocking_recv();
                info!("::actors#{}::inp-ready-checker ready - DMA", self.id);
                info!("::actor#{}::enter-computation - DMA", self.id);
                let if_compute_successed = self.on_dma_operation(op, inp, out, shape);
                // WIP let outs = self.on_unary_compute(op, inp).expect("compute failed");
                info!("::actor#{}::exit-computation - DMA", self.id);
                // respond_to.into_iter().map(|x| x.send(0u8) );
                respond_to
                    .into_iter()
                    .map(|x| {
                        info!(
                            "::actors#{}::out-ready-checker set-ready to var #{} - DMA",
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

    async fn fetch_and_handle_message_async(
        &mut self,
        msg: LoadfreeMessage<U>,
    ) -> Result<(), String> {
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

    fn fetch_and_handle_message(&mut self, msg: LoadfreeMessage<U>) -> Result<(), String> {
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

    pub fn run_blocking(&mut self) -> u32 {
        loop {
            match self.receiver.try_recv() {
                Ok(_msg) => {
                    info!("::actor#{}::receive msg from system", self.id);
                    let status = self.fetch_and_handle(_msg);
                }
                Err(TryRecvError::Empty) => {
                    let msg = build_loadfree_msg!("available", self.id);
                    // TODO update build_msg with generalmessage
                    self.respond_to.try_send(RaptorMessage::LoadfreeMSG(msg));
                    info!("::actor#{}::tell supervisor i am available", self.id);
                    use tokio::runtime::Handle;
                    let handle = Handle::current();
                    match handle.block_on(async { self.receiver.recv().await }) {
                        // match self.receiver.recv().await {
                        Some(_msg) => {
                            info!("::actor#{}::receive msg from system", self.id);
                            let status = self.fetch_and_handle(_msg);
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

    pub async fn run(&mut self) -> u32 {
        loop {
            match self.receiver.try_recv() {
                Ok(_msg) => {
                    info!("::actor#{}::receive msg from system", self.id);
                    let status = self.fetch_and_handle(_msg);
                }
                Err(TryRecvError::Empty) => {
                    let msg = build_loadfree_msg!("available", self.id);
                    // TODO update build_msg with generalmessage
                    self.respond_to.try_send(RaptorMessage::LoadfreeMSG(msg));
                    info!("::actor#{}::tell supervisor i am available", self.id);
                    match self.receiver.recv().await {
                        Some(_msg) => {
                            info!("::actor#{}::receive msg from system", self.id);
                            let status = self.fetch_and_handle(_msg);
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

    #[tracing::instrument(name = "actor::run", skip(self))]
    pub async fn run_async(&mut self) -> u32 {
        loop {
            match self.receiver.try_recv() {
                Ok(_msg) => {
                    info!("::actor#{}::receive msg from system", self.id);
                    let status = self.fetch_and_handle_async(_msg).await;
                }
                Err(TryRecvError::Empty) => {
                    let msg = build_loadfree_msg!("available", self.id);
                    // TODO update build_msg with generalmessage
                    self.respond_to.try_send(RaptorMessage::LoadfreeMSG(msg));
                    info!("::actor#{}::tell supervisor i am available", self.id);
                    match self.receiver.recv().await {
                        Some(_msg) => {
                            info!("::actor#{}::receive msg from system", self.id);
                            let status = self.fetch_and_handle_async(_msg).await;
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
    fn on_binary_compute(
        &mut self,
        op: O,
        lhs: Arc<RwLock<U>>,
        rhs: Arc<RwLock<U>>,
    ) -> Result<U, String> {
        let outs = self.executor.binary_compute(op, lhs, rhs);
        Ok(outs)
    }

    #[tracing::instrument(name = "actor::on_binary_compute", skip(self, lhs, rhs, out))]
    fn on_binary_compute_v2(
        &mut self,
        op: O,
        lhs: Arc<RwLock<U>>,
        rhs: Arc<RwLock<U>>,
        out: Arc<RwLock<U>>,
    ) -> Result<(), String> {
        let outs = self.executor.binary_compute_v2(op, lhs, rhs, out);
        Ok(outs)
    }

    #[tracing::instrument(name = "actor::on_unary_compute", skip(self, operand))]
    fn on_unary_compute(&mut self, op: O, operand: Arc<RwLock<U>>) -> Result<U, String> {
        let outs = self.executor.unary_compute(op, operand);
        Ok(outs)
    }

    // v2 consumes output, mutable into inner value, and returns status
    #[tracing::instrument(name = "actor::on_unary_compute_v2", skip(self, operand, result))]
    fn on_unary_compute_v2(
        &mut self,
        op: O,
        operand: Arc<RwLock<U>>,
        result: Arc<RwLock<U>>,
    ) -> Result<(), String> {
        let status = self.executor.unary_compute_v2(op, operand, result);
        Ok(status)
    }

    fn on_dma_operation(
        &mut self,
        op: O,
        operand: Arc<RwLock<U>>,
        result: Arc<RwLock<U>>,
        shape: Vec<usize>,
    ) -> Result<(), String> {
        let status = self.executor.dma_operation(op, operand, result, shape);
        Ok(status)
    }

    fn on_tenary_compute_v2(
        &mut self,
        op: O,
        first: Arc<RwLock<U>>,
        second: Arc<RwLock<U>>,
        third: Arc<RwLock<U>>,
        out: Arc<RwLock<U>>,
    ) -> Result<(), String> {
        let outs = self
            .executor
            .tenary_compute_v2(op, first, second, third, out);
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
