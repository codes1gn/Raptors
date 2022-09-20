use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
// use log::{info};
use std::fmt::Debug;
use std::io::Write;
use tokio::sync::{mpsc, oneshot};
use tracing::info;
// use tracing::instrument;
use uuid::Uuid;

use std::collections::HashMap;
use std::marker::PhantomData;
use std::{thread, time};

use crate::actors::*;
use crate::executor::{Executor, ExecutorLike};
use crate::mailbox::*;
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
///     let system = build_system!("mock system", 2);
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
        T: 'static + ExecutorLike<TensorType = U> + Send + Sync,
        U: 'static + TensorLike + Clone + Send + Sync + Debug,
    >(
        &mut self,
        config: SystemConfig,
    ) -> ActorSystemHandle<T, U> {
        self.cfg = Some(config);
        let mut system = ActorSystemHandle::<T, U>::new(&self.config().name().to_owned());
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
pub struct ActorSystemHandle<T, U>
where
    T: 'static + ExecutorLike<TensorType = U> + Send + Sync,
    U: 'static + TensorLike + Clone + Send + Sync + Debug,
{
    name: String,
    system_cmd_sendbox: mpsc::Sender<RaptorMessage<U>>,
    _marker: PhantomData<T>,
}

impl<T, U> ActorSystemHandle<T, U>
where
    T: 'static + ExecutorLike<TensorType = U> + Send + Sync,
    U: 'static + TensorLike + Clone + Send + Sync + Debug,
{
    pub fn new(name: &str) -> Self {
        let (sender, receiver) = mpsc::channel(100);
        let mut system = ActorSystem::<T, U>::new(name, receiver, sender.clone());
        tokio::spawn(async move { system.run().await });
        Self {
            name: name.to_string(),
            system_cmd_sendbox: sender,
            _marker: PhantomData,
        }
    }

    // pub fn init() -> () {
    //     let mut system = ActorSystem::<T, U>::new(name, receiver, sender.clone());
    //     tokio::spawn(async move { system.run().await });
    // }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub async fn issue_order(&mut self, msg: RaptorMessage<U>) -> () {
        println!("issue order = {:#?}", msg);
        info!("issue order = {:#?}", msg);
        self.system_cmd_sendbox.send(msg).await;
    }

    pub async fn spawn(&mut self, cnt: usize) {
        let cmd: LoadfreeMessage<U> = build_loadfree_msg!("spawn", cnt);
        self.issue_order(RaptorMessage::LoadfreeMSG(cmd)).await
    }
}

#[derive(Debug)]
pub struct ActorSystem<T, U>
where
    T: 'static + ExecutorLike<TensorType = U> + Send,
    U: 'static + TensorLike + Clone + Send + Sync + Debug,
{
    // TODO need a state machine that monitor actors
    // and allow graceful shutdown
    name: String,
    ranks: usize,
    pub mails: Vec<mpsc::Sender<RaptorMessage<U>>>,
    pub availables: Vec<usize>,
    system_cmd_recvbox: mpsc::Receiver<RaptorMessage<U>>,
    cloned_sendbox: mpsc::Sender<RaptorMessage<U>>,
    delayed_tensor_types: Vec<RaptorMessage<U>>,
    _marker: PhantomData<T>,
}

impl<T, U> ActorSystem<T, U>
where
    T: 'static + ExecutorLike<TensorType = U> + std::marker::Send,
    U: 'static + TensorLike + Clone + Send + Sync + Debug,
{
    pub fn new(
        name: &str,
        receiver: mpsc::Receiver<RaptorMessage<U>>,
        cloned_sender: mpsc::Sender<RaptorMessage<U>>,
    ) -> Self {
        // refer to stackoverflow.com/questions/48850403/change-timestamp-format-used-by-env-logger
        // set default usage of info log level

        let mut mailboxes: Vec<mpsc::Sender<RaptorMessage<U>>> = vec![];
        Self {
            name: String::from(name),
            ranks: 0,
            mails: mailboxes,
            availables: vec![],
            system_cmd_recvbox: receiver,
            cloned_sendbox: cloned_sender,
            delayed_tensor_types: vec![],
            _marker: PhantomData,
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

    #[tracing::instrument(name = "actor_system", skip(self))]
    pub fn spawn_actors(&mut self, cnt: usize) -> Result<(), String> {
        for id in self.ranks..(self.ranks + cnt) {
            info!("ASYS - creating actor with id = #{}", id);
            let (sender, receiver) = mpsc::channel(16);
            self.mails.push(sender);
            let mut actor = Actor::<T, U>::new(id, receiver, self.cloned_sendbox.clone());
            tokio::spawn(async move { actor.run().await });

            self.availables.push(id);
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

    #[tracing::instrument(name = "actor_system", skip(self, msg))]
    pub async fn deliver_to(&self, msg: RaptorMessage<U>, to: usize) {
        self.mails[to].send(msg).await;
        info!("ASYS - deliver message to {}", to);
    }

    #[tracing::instrument(name = "actor_system", skip(self, msg))]
    pub async fn broadcast(&self, msg: LoadfreeMessage<U>) {
        for mail in &self.mails {
            mail.send(RaptorMessage::LoadfreeMSG(msg.clone())).await;
        }
        info!("ASYS - broadcast message to all");
    }

    #[tracing::instrument(name = "system::run", skip(self))]
    pub async fn run(&mut self) -> () {
        info!("ASys - enter actor-system event-loop");
        loop {
            match self.system_cmd_recvbox.recv().await {
                Some(gmsg) => {
                    match gmsg {
                        RaptorMessage::PayloadMSG(ref msg) => {
                            match msg {
                                PayloadMessage::ComputeFunctorMsg { .. } => {
                                    let idle_actor = self.poll_ready_actor();
                                    match idle_actor {
                                        None => {
                                            self.delayed_tensor_types.push(gmsg);
                                            Ok(())
                                        }
                                        Some(idx) => {
                                            info!("ASYS - dispatch workload msg to first idle actor #{}", idx);
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
                                    SystemCommand::Spawn(cnt) => {
                                        info!("ASYS - received spawn-actors cmd with #{}", cnt);
                                        self.spawn_actors(*cnt)
                                    }
                                    SystemCommand::HaltOn(idx) => self.halt_actor(*idx),
                                    SystemCommand::HaltAll => self.halt_all(),
                                    _ => Err("not implemented".to_string()),
                                },
                                LoadfreeMessage::WorkloadMsg(_) => {
                                    let idle_actor = self.poll_ready_actor();
                                    match idle_actor {
                                        None => {
                                            self.delayed_tensor_types.push(gmsg);
                                            Ok(())
                                        }
                                        Some(idx) => {
                                            info!("ASYS - dispatch workload msg to first idle actor #{}", idx);
                                            self.deliver_to(gmsg, idx).await;
                                            Ok(())
                                        }
                                    }
                                }
                                LoadfreeMessage::ActorMsg(_amsg) => match _amsg {
                                    ActorCommand::Available(idx) => {
                                        info!("ASYS - requeue available actor #{}", idx);
                                        self.availables.push(*idx);
                                        if self.delayed_tensor_types.is_empty() == false {
                                            let idle_actor = self.poll_ready_actor().unwrap();
                                            let _delayed_wkl = self.delayed_tensor_types.remove(0);
                                            info!(
                                                "ASYS - delayed workload dispatched to actor #{}",
                                                idle_actor
                                            );
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
        let system = ActorSystemHandle::<Executor, Workload>::new("raptor system");
        assert_eq!(system.name(), "raptor system");
    }

    #[tokio::test]
    async fn create_system_with_macro_test_1() {
        let mut system = build_system!("Raptors");
        assert_eq!(system.name(), "Raptors");
    }

    #[tokio::test]
    async fn create_system_with_macro_test_2() {
        let mut system = build_system!("Raptors", 2);
        assert_eq!(system.name(), "Raptors");
        // TODO-FIX#1, currently not spawn at creation due to async-sync
        // assert_eq!(system.ranks(), 2);
    }
}
