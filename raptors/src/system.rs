// use log::{info};
use std::fmt::Debug;
use std::marker::PhantomData;

use tokio::sync::mpsc;
use tokio::task::spawn_blocking;
use tracing::{debug, info};

use crate::actors::*;
use crate::cost_model::OpCodeLike;
use crate::executor_types::ExecutorLike;
use crate::messages::*;
use crate::prelude::*;
use crate::tensor_types::*;

/// TODO(short-term) make dedicated mod and move it to there, maybe name it system_config.rs
/// test SystemConfig creation and get
///
/// Definition: SystemConfig contains the static data to describe the actor system
/// and used for builder to build the system.
/// It also contains the strategies, hardware/software environment info used for
/// query purpose.
///
///
#[derive(Default, Debug)]
pub struct SystemConfig {
    name: String,
    log_level: String,
    ranks: Option<usize>,
}

impl SystemConfig {
    pub fn new(name: &str, log_level: &str) -> Self {
        try_init_raptors!(log_level);
        SystemConfig {
            name: name.to_string(),
            ranks: Default::default(),
            log_level: log_level.to_owned(),
        }
    }

    pub fn set_ranks(&mut self, ranks: usize) -> () {
        self.ranks = Some(ranks);
    }

    pub fn ranks(&self) -> usize {
        self.ranks.unwrap_or_else(|| 0)
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

/// Definition: The helper that provide helper functions for system creation
/// this class wraps all the complex logic used to build elements in;
/// actor system, for convenient.
///
/// backdoors for mocking tests are also provided by this helper
///
/// ```
/// use raptors::prelude::*;
///
/// #[tokio::main]
/// async fn main() {
///     let system = build_mock_system!("mock system", 2);
///     assert_eq!(system.name(), "mock system".to_string());
///     // TODO-FIX#1, currently not spawn at creation due to async-sync
///     // assert_eq!(system.ranks(), 2);
/// }
/// ```
///
#[derive(Default)]
pub struct SystemBuilder {
    cfg: Option<SystemConfig>,
}

impl SystemBuilder {
    pub fn new() -> Self {
        info!("SystemBuilder::new");
        SystemBuilder::default()
    }

    pub fn build_with_config<
        T: 'static + ExecutorLike<TensorType = U, OpCodeType = O> + Send + Sync,
        U: 'static + TensorLike + Clone + Send + Sync + Debug,
        O: 'static + OpCodeLike + Debug + Send + Sync,
    >(
        &mut self,
        config: SystemConfig,
    ) -> ActorSystemHandle<T, U, O> {
        self.cfg = Some(config);
        let mut system = ActorSystemHandle::<T, U, O>::new(&self.config().name().to_owned());
        // TODO-FIX#1 make issue_order sync func
        // let cmd = build_loadfree_msg!("spawn", self.config().ranks());
        // system.issue_order(cmd).await;
        system
    }

    fn config(&self) -> &SystemConfig {
        &self.cfg.as_ref().unwrap()
    }
}

#[derive(Debug)]
pub struct ActorSystemHandle<T, U, O>
where
    T: 'static + ExecutorLike<TensorType = U, OpCodeType = O> + Send + Sync,
    U: 'static + TensorLike + Clone + Send + Sync + Debug,
    O: 'static + OpCodeLike + Debug + Send + Sync,
{
    name: String,
    system_cmd_sendbox: mpsc::Sender<RaptorMessage<U, O>>,
    _markerT: PhantomData<T>,
    _markerO: PhantomData<O>,
}

impl<T, U, O> ActorSystemHandle<T, U, O>
where
    T: 'static + ExecutorLike<TensorType = U, OpCodeType = O> + Send + Sync,
    U: 'static + TensorLike + Clone + Send + Sync + Debug,
    O: 'static + OpCodeLike + Debug + Send + Sync,
{
    pub fn new(name: &str) -> Self {
        let (sender, receiver) = mpsc::channel(100);
        let mut system = ActorSystem::<T, U, O>::new(name, receiver, sender.clone());
        // spawn_blocking(move || { system.run() });
        tokio::spawn(async move { system.run().await });
        // tokio::spawn(async move { system.blocking_run() });
        // rayon::spawn(move || {
        //     system.blocking_run()
        // });
        Self {
            name: name.to_string(),
            system_cmd_sendbox: sender,
            _markerT: PhantomData,
            _markerO: PhantomData,
        }
    }

    // pub fn init() -> () {
    //     let mut system = ActorSystem::<T, U, O>::new(name, receiver, sender.clone());
    //     tokio::spawn(async move { system.run().await });
    // }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub async fn issue_order(&mut self, msg: RaptorMessage<U, O>) -> () {
        debug!("::actor-system-handler::send msg {:?}", msg);
        self.system_cmd_sendbox.send(msg).await;
    }

    pub async fn spawn(&mut self, executor_type: &str, cnt: usize) {
        panic!("deprecated");
        let cmd: LoadfreeMessage<U> = build_loadfree_msg!("spawn", executor_type, cnt);
        self.issue_order(RaptorMessage::LoadfreeMSG(cmd)).await
    }
}

#[derive(Debug)]
pub struct ActorSystem<T, U, O>
where
    T: 'static + ExecutorLike<TensorType = U, OpCodeType = O> + Send,
    U: 'static + TensorLike + Clone + Send + Sync + Debug,
    O: 'static + OpCodeLike + Debug + Send,
{
    // TODO need a state machine that monitor actors
    // and allow graceful shutdown
    name: String,
    ranks: usize,
    pub mails: Vec<mpsc::Sender<RaptorMessage<U, O>>>,
    pub availables: Vec<usize>,
    system_cmd_recvbox: mpsc::Receiver<RaptorMessage<U, O>>,
    cloned_sendbox: mpsc::Sender<RaptorMessage<U, O>>,
    delayed_tensor_types: Vec<RaptorMessage<U, O>>,
    _markerT: PhantomData<T>,
    _markerO: PhantomData<O>,
}

impl<T, U, O> ActorSystem<T, U, O>
where
    T: 'static + ExecutorLike<TensorType = U, OpCodeType = O> + Send,
    U: 'static + TensorLike + Clone + Send + Sync + Debug,
    O: 'static + OpCodeLike + Debug + Send,
{
    pub fn new(
        name: &str,
        receiver: mpsc::Receiver<RaptorMessage<U, O>>,
        cloned_sender: mpsc::Sender<RaptorMessage<U, O>>,
    ) -> Self {
        // refer to stackoverflow.com/questions/48850403/change-timestamp-format-used-by-env-logger
        // set default usage of info log level

        let mut mailboxes: Vec<mpsc::Sender<RaptorMessage<U, O>>> = vec![];
        Self {
            name: String::from(name),
            ranks: 0,
            mails: mailboxes,
            availables: vec![],
            system_cmd_recvbox: receiver,
            cloned_sendbox: cloned_sender,
            delayed_tensor_types: vec![],
            _markerT: PhantomData,
            _markerO: PhantomData,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn ranks(&self) -> usize {
        self.ranks
    }

    // get the first idle/available actor tid
    // replace this into mpsc receiver with multiple actor to generate key back
    pub fn poll_ready_actor(&mut self) -> Option<usize> {
        if self.availables.is_empty() == true {
            None
        } else {
            Some(self.availables.remove(0))
        }
    }

    #[tracing::instrument(name = "actor_system", skip(self, typeid, cnt))]
    pub fn spawn_actors(&mut self, typeid: usize, cnt: usize) -> Result<(), String> {
        for id in self.ranks..(self.ranks + cnt) {
            info!("::actor_system::new actor #{}", id);
            let (sender, receiver) = mpsc::channel(16);
            self.mails.push(sender);
            let mut actor =
                Actor::<T, U, O>::new(id, receiver, self.cloned_sendbox.clone(), typeid);
            info!("::actor_system::run-event-loop actor #{}", id);
            // LEGACY WAY - all async
            // tokio::spawn(async move { actor.run_async().await });
            // NEW WAY - actor actions in blocking
            tokio::spawn(async move { actor.run().await });

            self.availables.push(id);
            info!("::actor-system::enqueue actor-#{} to avlb-queue", id);
        }
        self.ranks += cnt;
        Ok(())
    }

    #[tracing::instrument(name = "actor_system", skip(self))]
    pub fn halt_actor(&mut self, index: usize) -> Result<(), String> {
        if index >= self.mails.len() {
            return Err(String::from("halt cmd out of actor id range"));
        }
        self.mails.remove(index);
        Ok(())
    }

    #[tracing::instrument(name = "actor_system", skip(self))]
    pub fn halt_all(&mut self) -> Result<(), String> {
        self.mails.clear();
        Ok(())
    }

    pub fn blocking_deliver_to(&self, msg: RaptorMessage<U, O>, to: usize) {
        // if to > self.mails.len() {
        //     panic!("device id to deliver msg succeeds max device id");
        // }
        self.mails[to].blocking_send(msg);
        debug!("::actor_system::send msg to actor #{:?}", to);
    }

    #[tracing::instrument(name = "actor_system", skip(self, msg, to))]
    pub async fn deliver_to(&self, msg: RaptorMessage<U, O>, to: usize) {
        // if to > self.mails.len() {
        //     panic!("device id to deliver msg succeeds max device id");
        // }
        self.mails[to].send(msg).await;
        debug!("::actor_system::send msg to actor #{:?}", to);
    }

    #[tracing::instrument(name = "actor_system", skip(self, msg))]
    pub async fn broadcast(&self, msg: LoadfreeMessage<U>) {
        for mail in &self.mails {
            mail.send(RaptorMessage::LoadfreeMSG(msg.clone())).await;
        }
        debug!("::actor_system::send msg to all actors");
    }

    pub fn blocking_run(&mut self) -> () {
        info!("::actor-system::start-event-loop");
        loop {
            match self.system_cmd_recvbox.blocking_recv() {
                Some(gmsg) => {
                    match gmsg {
                        RaptorMessage::PayloadMSG(ref msg) => {
                            match msg {
                                PayloadMessage::NonRetTenaryComputeFunctorMsg {
                                    dev_at, ..
                                } => {
                                    // TODO dispatch on dev_at idx
                                    debug!("::actor-system::recv payload-msg {:?}", msg);
                                    info!("ANCHOR DEV-AT = {:?}", dev_at);
                                    let idle_actor = self.poll_ready_actor();
                                    match dev_at.clone() {
                                        Some(_dev_at) => {
                                            info!("::actor-system::pre-dispatch payload-msg to actor #{:?}", _dev_at);
                                            self.blocking_deliver_to(gmsg, _dev_at as usize);
                                            Ok(())
                                        }
                                        None => {
                                            let idle_actor = self.poll_ready_actor();
                                            match idle_actor {
                                                None => {
                                                    info!("::actor-system::not-find avlb-actor");
                                                    info!("::actor-system::delay this unary-compute-task");
                                                    self.delayed_tensor_types.push(gmsg);
                                                    Ok(())
                                                }
                                                Some(idx) => {
                                                    info!(
                                                        "::actor-system::find avlb-actor-#{:?}",
                                                        idx
                                                    );
                                                    info!("::actor-system::poll actor-#{} out from avlb-queue", idx);
                                                    info!("::actor-system::dispatch payload-msg to actor #{:?}", idx);
                                                    self.blocking_deliver_to(gmsg, idx);
                                                    Ok(())
                                                }
                                            }
                                        }
                                    }
                                }
                                PayloadMessage::NonRetBinaryComputeFunctorMsg {
                                    dev_at, ..
                                } => {
                                    // TODO dispatch on dev_at idx
                                    debug!("::actor-system::recv payload-msg {:?}", msg);
                                    info!("ANCHOR DEV-AT = {:?}", dev_at);
                                    let idle_actor = self.poll_ready_actor();
                                    match dev_at.clone() {
                                        Some(_dev_at) => {
                                            info!("::actor-system::pre-dispatch payload-msg to actor #{:?}", _dev_at);
                                            self.blocking_deliver_to(gmsg, _dev_at as usize);
                                            Ok(())
                                        }
                                        None => {
                                            let idle_actor = self.poll_ready_actor();
                                            match idle_actor {
                                                None => {
                                                    info!("::actor-system::not-find avlb-actor");
                                                    info!("::actor-system::delay this unary-compute-task");
                                                    self.delayed_tensor_types.push(gmsg);
                                                    Ok(())
                                                }
                                                Some(idx) => {
                                                    info!(
                                                        "::actor-system::find avlb-actor-#{:?}",
                                                        idx
                                                    );
                                                    info!("::actor-system::poll actor-#{} out from avlb-queue", idx);
                                                    info!("::actor-system::dispatch payload-msg to actor #{:?}", idx);
                                                    self.blocking_deliver_to(gmsg, idx);
                                                    Ok(())
                                                }
                                            }
                                        }
                                    }
                                }
                                PayloadMessage::DMAOperationMsg { dev_at, .. } => {
                                    // TODO dispatch on dev_at idx
                                    debug!("::actor-system::recv payload-msg {:?}", msg);
                                    info!("ANCHOR DEV-AT = {:?}", dev_at);
                                    match dev_at.clone() {
                                        Some(_dev_at) => {
                                            info!("::actor-system::pre-dispatch payload-msg to actor #{:?}", _dev_at);
                                            self.blocking_deliver_to(gmsg, _dev_at as usize);
                                            Ok(())
                                        }
                                        None => {
                                            let idle_actor = self.poll_ready_actor();
                                            match idle_actor {
                                                None => {
                                                    info!("::actor-system::not-find avlb-actor");
                                                    info!("::actor-system::delay this unary-compute-task");
                                                    self.delayed_tensor_types.push(gmsg);
                                                    Ok(())
                                                }
                                                Some(idx) => {
                                                    info!(
                                                        "::actor-system::find avlb-actor-#{:?}",
                                                        idx
                                                    );
                                                    info!("::actor-system::poll actor-#{} out from avlb-queue", idx);
                                                    info!("::actor-system::dispatch payload-msg to actor #{:?}", idx);
                                                    self.blocking_deliver_to(gmsg, idx);
                                                    Ok(())
                                                }
                                            }
                                        }
                                    }
                                }
                                PayloadMessage::NonRetUnaryComputeFunctorMsg { dev_at, .. } => {
                                    // TODO dispatch on dev_at idx
                                    debug!("::actor-system::recv payload-msg {:?}", msg);
                                    info!("ANCHOR DEV-AT = {:?}", dev_at);
                                    match dev_at.clone() {
                                        Some(_dev_at) => {
                                            info!("::actor-system::pre-dispatch payload-msg to actor #{:?}", _dev_at);
                                            self.blocking_deliver_to(gmsg, _dev_at as usize);
                                            Ok(())
                                        }
                                        None => {
                                            let idle_actor = self.poll_ready_actor();
                                            match idle_actor {
                                                None => {
                                                    info!("::actor-system::not-find avlb-actor");
                                                    info!("::actor-system::delay this unary-compute-task");
                                                    self.delayed_tensor_types.push(gmsg);
                                                    Ok(())
                                                }
                                                Some(idx) => {
                                                    info!(
                                                        "::actor-system::find avlb-actor-#{:?}",
                                                        idx
                                                    );
                                                    info!("::actor-system::poll actor-#{} out from avlb-queue", idx);
                                                    info!("::actor-system::dispatch payload-msg to actor #{:?}", idx);
                                                    self.blocking_deliver_to(gmsg, idx);
                                                    Ok(())
                                                }
                                            }
                                        }
                                    }
                                }
                                PayloadMessage::UnaryComputeFunctorMsg { .. } => {
                                    debug!("::actor-system::recv payload-msg {:?}", msg);
                                    let idle_actor = self.poll_ready_actor();
                                    match idle_actor {
                                        None => {
                                            info!("::actor-system::not-find avlb-actor");
                                            info!("::actor-system::delay this unary-compute-task");
                                            self.delayed_tensor_types.push(gmsg);
                                            Ok(())
                                        }
                                        Some(idx) => {
                                            info!("::actor-system::find avlb-actor-#{:?}", idx);
                                            info!("::actor-system::poll actor-#{} out from avlb-queue", idx);
                                            info!("::actor-system::dispatch payload-msg to actor #{:?}", idx);
                                            self.blocking_deliver_to(gmsg, idx);
                                            Ok(())
                                        }
                                    }
                                }
                                PayloadMessage::ComputeFunctorMsg { .. } => {
                                    debug!("::actor-system::recv payload-msg {:?}", msg);
                                    let idle_actor = self.poll_ready_actor();
                                    match idle_actor {
                                        None => {
                                            info!("::actor-system::not-find avlb-actor");
                                            info!("::actor-system::delay this binary-compute-task");
                                            self.delayed_tensor_types.push(gmsg);
                                            Ok(())
                                        }
                                        Some(idx) => {
                                            info!("::actor-system::find avlb-actor-#{:?}", idx);
                                            info!("::actor-system::poll actor-#{} out from avlb-queue", idx);
                                            info!("::actor-system::dispatch payload-msg to actor #{:?}", idx);
                                            self.blocking_deliver_to(gmsg, idx);
                                            Ok(())
                                        }
                                    }
                                }
                            }
                        }
                        RaptorMessage::LoadfreeMSG(ref msg) => {
                            match msg {
                                LoadfreeMessage::SystemMsg(cmd) => match cmd {
                                    SystemCommand::Spawn(typeid, cnt) => {
                                        info!("::actor-system::recv loadfree-msg-spawn {:?}", cnt);
                                        self.spawn_actors(*typeid, *cnt)
                                    }
                                    SystemCommand::HaltOn(idx) => self.halt_actor(*idx),
                                    SystemCommand::HaltAll => self.halt_all(),
                                    _ => Err("not implemented".to_string()),
                                },
                                LoadfreeMessage::MockTensorMsg(_) => {
                                    let idle_actor = self.poll_ready_actor();
                                    match idle_actor {
                                        None => {
                                            self.delayed_tensor_types.push(gmsg);
                                            Ok(())
                                        }
                                        Some(idx) => {
                                            info!("::actor-system::dispatch payload-msg to actor #{:?}", idx);
                                            self.blocking_deliver_to(gmsg, idx);
                                            Ok(())
                                        }
                                    }
                                }
                                LoadfreeMessage::ActorMsg(_amsg) => match _amsg {
                                    ActorCommand::Available(idx) => {
                                        info!(
                                            "::actor-system::enqueue actor-#{} to avlb-queue",
                                            idx
                                        );
                                        self.availables.push(*idx);
                                        if self.delayed_tensor_types.is_empty() == false {
                                            let idle_actor = self.poll_ready_actor().unwrap();
                                            let _delayed_wkl = self.delayed_tensor_types.remove(0);
                                            info!(
                                                "::actor-system::dispatch delayed payload-msg to actor-#{}",
                                                idle_actor
                                            );
                                            info!("::actor-system::poll actor-#{} out from avlb-queue", idx);
                                            self.blocking_deliver_to(_delayed_wkl, idle_actor);
                                        }
                                        Ok(())
                                    }
                                    _ => panic!("not implemented"),
                                },
                            }
                        }
                        // not handle msg other than system cmd
                        _ => Ok(()),
                    }
                }
                _ => Ok(()),
            };
        }
    }

    #[tracing::instrument(name = "system::run", skip(self))]
    pub async fn run(&mut self) -> () {
        info!("::actor-system::start-event-loop");
        loop {
            match self.system_cmd_recvbox.recv().await {
                Some(gmsg) => {
                    match gmsg {
                        RaptorMessage::PayloadMSG(ref msg) => {
                            match msg {
                                PayloadMessage::NonRetBinaryComputeFunctorMsg {
                                    dev_at, ..
                                } => {
                                    // TODO dispatch on dev_at idx
                                    debug!("::actor-system::recv payload-msg {:?}", msg);
                                    info!("ANCHOR DEV-AT = {:?}", dev_at);
                                    let idle_actor = self.poll_ready_actor();
                                    match dev_at.clone() {
                                        Some(_dev_at) => {
                                            info!("::actor-system::pre-dispatch payload-msg to actor #{:?}", _dev_at);
                                            self.deliver_to(gmsg, _dev_at as usize).await;
                                            Ok(())
                                        }
                                        None => {
                                            let idle_actor = self.poll_ready_actor();
                                            match idle_actor {
                                                None => {
                                                    info!("::actor-system::not-find avlb-actor");
                                                    info!("::actor-system::delay this unary-compute-task");
                                                    self.delayed_tensor_types.push(gmsg);
                                                    Ok(())
                                                }
                                                Some(idx) => {
                                                    info!(
                                                        "::actor-system::find avlb-actor-#{:?}",
                                                        idx
                                                    );
                                                    info!("::actor-system::poll actor-#{} out from avlb-queue", idx);
                                                    info!("::actor-system::dispatch payload-msg to actor #{:?}", idx);
                                                    self.deliver_to(gmsg, idx).await;
                                                    Ok(())
                                                }
                                            }
                                        }
                                    }
                                }
                                PayloadMessage::NonRetTenaryComputeFunctorMsg {
                                    dev_at, ..
                                } => {
                                    // TODO dispatch on dev_at idx
                                    debug!("::actor-system::recv payload-msg {:?}", msg);
                                    info!("ANCHOR DEV-AT = {:?}", dev_at);
                                    let idle_actor = self.poll_ready_actor();
                                    match dev_at.clone() {
                                        Some(_dev_at) => {
                                            info!("::actor-system::pre-dispatch payload-msg to actor #{:?}", _dev_at);
                                            self.deliver_to(gmsg, _dev_at as usize).await;
                                            Ok(())
                                        }
                                        None => {
                                            let idle_actor = self.poll_ready_actor();
                                            match idle_actor {
                                                None => {
                                                    info!("::actor-system::not-find avlb-actor");
                                                    info!("::actor-system::delay this unary-compute-task");
                                                    self.delayed_tensor_types.push(gmsg);
                                                    Ok(())
                                                }
                                                Some(idx) => {
                                                    info!(
                                                        "::actor-system::find avlb-actor-#{:?}",
                                                        idx
                                                    );
                                                    info!("::actor-system::poll actor-#{} out from avlb-queue", idx);
                                                    info!("::actor-system::dispatch payload-msg to actor #{:?}", idx);
                                                    self.deliver_to(gmsg, idx).await;
                                                    Ok(())
                                                }
                                            }
                                        }
                                    }
                                }
                                PayloadMessage::DMAOperationMsg { dev_at, .. } => {
                                    // TODO dispatch on dev_at idx
                                    debug!("::actor-system::recv payload-msg {:?}", msg);
                                    info!("ANCHOR DEV-AT = {:?}", dev_at);
                                    match dev_at.clone() {
                                        Some(_dev_at) => {
                                            info!("::actor-system::pre-dispatch payload-msg to actor #{:?}", _dev_at);
                                            self.deliver_to(gmsg, _dev_at as usize).await;
                                            Ok(())
                                        }
                                        None => {
                                            let idle_actor = self.poll_ready_actor();
                                            match idle_actor {
                                                None => {
                                                    info!("::actor-system::not-find avlb-actor");
                                                    info!("::actor-system::delay this unary-compute-task");
                                                    self.delayed_tensor_types.push(gmsg);
                                                    Ok(())
                                                }
                                                Some(idx) => {
                                                    info!(
                                                        "::actor-system::find avlb-actor-#{:?}",
                                                        idx
                                                    );
                                                    info!("::actor-system::poll actor-#{} out from avlb-queue", idx);
                                                    info!("::actor-system::dispatch payload-msg to actor #{:?}", idx);
                                                    self.deliver_to(gmsg, idx).await;
                                                    Ok(())
                                                }
                                            }
                                        }
                                    }
                                }
                                PayloadMessage::NonRetUnaryComputeFunctorMsg { dev_at, .. } => {
                                    // TODO dispatch on dev_at idx
                                    debug!("::actor-system::recv payload-msg {:?}", msg);
                                    info!("ANCHOR DEV-AT = {:?}", dev_at);
                                    match dev_at.clone() {
                                        Some(_dev_at) => {
                                            info!("::actor-system::pre-dispatch payload-msg to actor #{:?}", _dev_at);
                                            self.deliver_to(gmsg, _dev_at as usize).await;
                                            Ok(())
                                        }
                                        None => {
                                            let idle_actor = self.poll_ready_actor();
                                            match idle_actor {
                                                None => {
                                                    info!("::actor-system::not-find avlb-actor");
                                                    info!("::actor-system::delay this unary-compute-task");
                                                    self.delayed_tensor_types.push(gmsg);
                                                    Ok(())
                                                }
                                                Some(idx) => {
                                                    info!(
                                                        "::actor-system::find avlb-actor-#{:?}",
                                                        idx
                                                    );
                                                    info!("::actor-system::poll actor-#{} out from avlb-queue", idx);
                                                    info!("::actor-system::dispatch payload-msg to actor #{:?}", idx);
                                                    self.deliver_to(gmsg, idx).await;
                                                    Ok(())
                                                }
                                            }
                                        }
                                    }
                                }
                                PayloadMessage::UnaryComputeFunctorMsg { .. } => {
                                    debug!("::actor-system::recv payload-msg {:?}", msg);
                                    let idle_actor = self.poll_ready_actor();
                                    match idle_actor {
                                        None => {
                                            info!("::actor-system::not-find avlb-actor");
                                            info!("::actor-system::delay this unary-compute-task");
                                            self.delayed_tensor_types.push(gmsg);
                                            Ok(())
                                        }
                                        Some(idx) => {
                                            info!("::actor-system::find avlb-actor-#{:?}", idx);
                                            info!("::actor-system::poll actor-#{} out from avlb-queue", idx);
                                            info!("::actor-system::dispatch payload-msg to actor #{:?}", idx);
                                            self.deliver_to(gmsg, idx).await;
                                            Ok(())
                                        }
                                    }
                                }
                                PayloadMessage::ComputeFunctorMsg { .. } => {
                                    debug!("::actor-system::recv payload-msg {:?}", msg);
                                    let idle_actor = self.poll_ready_actor();
                                    match idle_actor {
                                        None => {
                                            info!("::actor-system::not-find avlb-actor");
                                            info!("::actor-system::delay this binary-compute-task");
                                            self.delayed_tensor_types.push(gmsg);
                                            Ok(())
                                        }
                                        Some(idx) => {
                                            info!("::actor-system::find avlb-actor-#{:?}", idx);
                                            info!("::actor-system::poll actor-#{} out from avlb-queue", idx);
                                            info!("::actor-system::dispatch payload-msg to actor #{:?}", idx);
                                            self.deliver_to(gmsg, idx).await;
                                            Ok(())
                                        }
                                    }
                                }
                            }
                        }
                        RaptorMessage::LoadfreeMSG(ref msg) => {
                            match msg {
                                LoadfreeMessage::SystemMsg(cmd) => match cmd {
                                    SystemCommand::Spawn(typeid, cnt) => {
                                        info!("::actor-system::recv loadfree-msg-spawn {:?}", cnt);
                                        self.spawn_actors(*typeid, *cnt)
                                    }
                                    SystemCommand::HaltOn(idx) => self.halt_actor(*idx),
                                    SystemCommand::HaltAll => self.halt_all(),
                                    _ => Err("not implemented".to_string()),
                                },
                                LoadfreeMessage::MockTensorMsg(_) => {
                                    let idle_actor = self.poll_ready_actor();
                                    match idle_actor {
                                        None => {
                                            self.delayed_tensor_types.push(gmsg);
                                            Ok(())
                                        }
                                        Some(idx) => {
                                            info!("::actor-system::dispatch payload-msg to actor #{:?}", idx);
                                            self.deliver_to(gmsg, idx).await;
                                            Ok(())
                                        }
                                    }
                                }
                                LoadfreeMessage::ActorMsg(_amsg) => match _amsg {
                                    ActorCommand::Available(idx) => {
                                        info!(
                                            "::actor-system::enqueue actor-#{} to avlb-queue",
                                            idx
                                        );
                                        self.availables.push(*idx);
                                        if self.delayed_tensor_types.is_empty() == false {
                                            let idle_actor = self.poll_ready_actor().unwrap();
                                            let _delayed_wkl = self.delayed_tensor_types.remove(0);
                                            info!(
                                                "::actor-system::dispatch delayed payload-msg to actor-#{}",
                                                idle_actor
                                            );
                                            info!("::actor-system::poll actor-#{} out from avlb-queue", idx);
                                            self.deliver_to(_delayed_wkl, idle_actor).await;
                                        }
                                        Ok(())
                                    }
                                    _ => panic!("not implemented"),
                                },
                            }
                        }
                        // not handle msg other than system cmd
                        _ => Ok(()),
                    }
                }
                _ => Ok(()),
            };
        }
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_system_with_new_test_1() {
        let system =
            ActorSystemHandle::<MockExecutor, MockTensor, MockOpCode>::new("raptor system");
        assert_eq!(system.name(), "raptor system");
    }

    #[tokio::test]
    async fn create_system_with_macro_test_1() {
        let mut system = build_mock_system!("Raptors");
        assert_eq!(system.name(), "Raptors");
    }

    #[tokio::test]
    async fn create_system_with_macro_test_2() {
        let mut system = build_mock_system!("Raptors", 2);
        assert_eq!(system.name(), "Raptors");
        // TODO-FIX#1, currently not spawn at creation due to async-sync
        // assert_eq!(system.ranks(), 2);
    }
}
