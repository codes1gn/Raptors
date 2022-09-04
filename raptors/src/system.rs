use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
// use log::{info};
use std::io::Write;
use tokio::sync::{mpsc, oneshot};
use tracing::info;
// use tracing::instrument;
use uuid::Uuid;

use std::collections::HashMap;
use std::{thread, time};

use crate::actors::*;
use crate::mailbox::*;
use crate::messages::*;
use crate::prelude::*;
use crate::workloads::*;

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
///     assert_eq!(system.ranks(), 2);
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

    pub fn build_with_config(&mut self, config: SystemConfig) -> ActorSystem {
        self.cfg = Some(config);
        let mut system = ActorSystem::new(&self.config().name().to_owned());
        system.spawn_actors(self.config().ranks());
        system
    }

    fn config(&self) -> &SystemConfig {
        &self.cfg.as_ref().unwrap()
    }
}

#[derive(Debug)]
pub struct ActorSystem {
    // TODO need a state machine that monitor actors
    // and allow graceful shutdown
    name: String,
    ranks: usize,
    pub mails: Vec<mpsc::Sender<TypedMessage>>,
    pub availables: Vec<usize>,
}

impl ActorSystem {
    pub fn new(name: &str) -> Self {
        // refer to stackoverflow.com/questions/48850403/change-timestamp-format-used-by-env-logger
        // set default usage of info log level

        let mut mailboxes: Vec<mpsc::Sender<TypedMessage>> = vec![];
        Self {
            name: String::from(name),
            ranks: 0,
            mails: mailboxes,
            availables: vec![],
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
    pub fn poll_ready_actor(&mut self) -> usize {
        assert_eq!(self.availables.is_empty(), false);
        self.availables.remove(0)
    }

    #[tracing::instrument(name = "actor_system", skip(self))]
    pub fn spawn_actors(&mut self, cnt: usize) -> Result<(), String> {
        for id in self.ranks..(self.ranks + cnt) {
            info!("creating actor with id = #{}", id);
            let (sender, receiver) = mpsc::channel(16);
            self.mails.push(sender);
            let mut actor = Actor::new(id, receiver);
            info!("on aspvr #{}", id);
            tokio::spawn(async move { actor.run().await });

            self.availables.push(id);
        }
        self.ranks += cnt;
        Ok(())
    }

    #[tracing::instrument(name = "actor_system", skip(self))]
    pub fn halt_actor(&mut self, index: usize) -> Result<(), String> {
        info!("triggering drop");
        if index >= self.mails.len() {
            return Err(String::from("halt cmd out of actor id range"));
        }
        self.mails.remove(index);
        Ok(())
    }

    #[tracing::instrument(name = "actor_system", skip(self))]
    pub fn halt_all(&mut self) -> Result<(), String> {
        info!("triggering drop all");
        self.mails.clear();
        Ok(())
    }

    #[tracing::instrument(name = "actor_system", skip(self))]
    pub async fn deliver_to(&self, msg: TypedMessage, to: usize) {
        info!("WIP: deliver message to {}", to);
        self.mails[to].send(msg).await;
        info!("FINISH: deliver message to {}", to);
    }

    #[tracing::instrument(name = "actor_system", skip(self))]
    pub async fn broadcast(&self, msg: TypedMessage) {
        info!("WIP: broadcast message");
        for mail in &self.mails {
            mail.send(msg.clone()).await;
        }
        info!("FINISH: broadcast message");
    }

    #[allow(unreachable_patterns)]
    #[tracing::instrument(name = "actor_system", skip(self))]
    pub fn on_receive(&mut self, msg: TypedMessage) -> Result<(), String> {
        match msg {
            TypedMessage::SystemMsg(cmd) => match cmd {
                SystemCommand::Spawn(cnt) => self.spawn_actors(cnt),
                SystemCommand::HaltOn(idx) => self.halt_actor(idx),
                SystemCommand::HaltAll => self.halt_all(),
                _ => Err("not implemented".to_string()),
            },
            _ => Err("not implemented".to_string()),
        }
    }

    // #[allow(unreachable_patterns)]
    // pub fn on_deliver(&mut self, evlp: Envelope) -> Result<(), String> {
    //     let status = self
    //         .actor_registry
    //         .get_mut(&evlp.receiver.into_aid())
    //         .unwrap()
    //         .mailbox_mut()
    //         .enqueue(evlp.msg.clone());
    //     status
    // }

    // pub fn on_dispatch_workloads(&mut self, workloads: Vec<TypedMessage>) -> Result<(), String> {
    //     let status = workloads
    //         .into_iter()
    //         .map(|msg| -> Result<(), String> { self.on_receive(msg) })
    //         .collect::<Result<(), String>>();
    //     status
    // }

    // pub fn on_dispatch_envelopes(&mut self, envelopes: Vec<Envelope>) -> Result<(), String> {
    //     let status = envelopes
    //         .into_iter()
    //         .map(|envelope| -> Result<(), String> { self.on_deliver(envelope) })
    //         .collect::<Result<(), String>>();
    //     status
    // }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_system_with_new_test_1() {
        let system = ActorSystem::new("raptor system");
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
        assert_eq!(system.ranks(), 2);
    }
}
