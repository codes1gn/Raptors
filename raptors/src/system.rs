use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use log::{debug, info};
use std::io::Write;
use tokio::sync::{mpsc, oneshot};
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
    ranks: Option<usize>,
}

impl SystemConfig {
    pub fn new(name: &str, log_level: &str) -> Self {
        std::env::set_var("RUST_LOG", log_level);
        // to make more precise timestamps
        Builder::new()
            .format(|buf, record| {
                writeln!(
                    buf,
                    "{} {}: {}",
                    record.level(),
                    Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                    record.args()
                )
            })
            .filter(None, LevelFilter::Info)
            .init();

        SystemConfig {
            name: name.to_string(),
            ranks: Default::default(),
        }
    }

    pub fn set_ranks(&mut self, ranks: usize) -> () {
        self.ranks = Some(ranks);
    }

    pub fn ranks(&self) -> Option<usize> {
        self.ranks
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
/// let syst = build_system!("mock system", 2);
/// assert_eq!(syst.name(), "mock system".to_string());
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
        ActorSystem::new(&self.config().name().to_owned())
    }

    fn config(&self) -> &SystemConfig {
        &self.cfg.as_ref().unwrap()
    }
}

#[derive(Debug)]
pub struct ActorSystem {
    // TODO need a state machine that monitor actors
    // and allow graceful shutdown
    pub name: String,
    pub ranks: usize,
    pub mails: Vec<mpsc::Sender<TypedMessage>>,
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
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn spawn_actors(&mut self, cnt: usize) {
        for id in self.ranks..(self.ranks + cnt) {
            info!("creating actor with id = #{}", id);
            let (sender, receiver) = mpsc::channel(16);
            self.mails.push(sender);
            let mut actor = Actor::new(id, receiver);
            info!("on aspvr #{}", id);
            tokio::spawn(async move { actor.run().await });
        }
        self.ranks += cnt;
    }

    pub fn halt_actor(&mut self, index: usize) {
        info!("triggering drop");
        self.mails.remove(index);
    }

    pub async fn deliver_to(&self, msg: TypedMessage, to: usize) {
        info!("WIP: deliver message to {}", to);
        self.mails[to].send(msg).await;
        info!("FINISH: deliver message to {}", to);
    }

    pub async fn broadcast(&self, msg: TypedMessage) {
        info!("WIP: broadcast message");
        for mail in &self.mails {
            mail.send(msg.clone()).await;
        }
        info!("FINISH: broadcast message");
    }

    // #[allow(unreachable_patterns)]
    // pub fn on_receive(&mut self, msg: TypedMessage) -> Result<(), String> {
    //     match msg {
    //         TypedMessage::SystemMsg(cmd) => {
    //             match cmd {
    //                 SystemCommand::CreateActors(cnt, base_name) => {
    //                     // let actor = self.create_actor("raptor");
    //                     let actors = self.create_actors(cnt, &base_name);
    //                     let status = self.register_actors(actors);
    //                     // return usize currently
    //                     match status {
    //                         Ok(_) => Ok(()),
    //                         Err(_e) => Err("Fail to register the actor".to_string()),
    //                     }
    //                 }
    //                 SystemCommand::StartExecution => {
    //                     info!(">>>>>> Raptors System Start Exec <<<<<<");
    //                     let mut actors: Vec<&mut Actor> = self
    //                         .actor_registry
    //                         .values_mut()
    //                         .collect::<Vec<&mut Actor>>();
    //                     let status = actors
    //                         .into_iter()
    //                         .map(|x| x.start())
    //                         .collect::<Result<(), String>>();
    //                     info!(">>>>>> Raptors System Stop Exec <<<<<<");
    //                     status
    //                 }
    //                 SystemCommand::DestroyAll => self.destroy_actors(),
    //                 _ => Err("not implemented".to_string()),
    //             }
    //         }
    //         _ => Err("not implemented".to_string()),
    //     }
    // }

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
    fn create_system() {
        let syst = ActorSystem::new("raptor system");
        assert_eq!(syst.name(), "raptor system");
    }

    // #[test]
    // fn system_create_actor_test() {
    //     let syst = ActorSystem::new("raptor system");
    //     let actor = syst.create_actor("raptor");
    //     assert_eq!(actor.name(), "raptor");
    // }

    // #[test]
    // fn system_create_actors_test() {
    //     let syst = ActorSystem::new("raptor system");
    //     let actors = syst.create_actors(4, "raptor");
    //     assert_eq!(actors.len(), 4);
    //     // TODO add more asserts
    // }

    // #[test]
    // fn system_create_register_then_query_actor_from_map_test() {
    //     let mut syst = ActorSystem::new("raptor system");

    //     // register
    //     let actor = syst.create_actor("raptor");
    //     let status = syst.register_actor(actor);

    //     // check result
    //     assert!(status.is_ok());
    //     let mactor = syst.actor_registry();
    //     assert_eq!(mactor.len(), 1);
    // }

    // #[test]
    // fn on_receive_not_systemmsg_test() {
    //     let mut syst = ActorSystem::new("system #1");
    //     let msg = TypedMessage::ActorMsg;
    //     let status = syst.on_receive(msg.into());
    //     assert!(status.is_err());
    //     assert_eq!(status.unwrap_err(), "not implemented".to_string());
    // }

    // #[test]
    // fn system_create_then_register_multiple_actors_test() {
    //     let mut syst = ActorSystem::new("raptor system");

    //     // register
    //     let actors = syst.create_actors(2, "raptor");
    //     let query_id = actors
    //         .iter()
    //         .map(|actor| actor.id().clone())
    //         .collect::<Vec<Uuid>>();
    //     let status = syst.register_actors(actors);

    //     // check result
    //     assert_eq!(status.is_ok(), true);
    //     let query_actors = syst.actor_registry();
    //     assert_eq!(query_actors.len(), 2);
    //     assert_eq!(
    //         query_actors.get(&query_id[0]).unwrap().name(),
    //         "raptor #0".to_string()
    //     );
    //     assert_eq!(
    //         query_actors.get(&query_id[1]).unwrap().name(),
    //         "raptor #1".to_string()
    //     );
    // }

    // #[test]
    // fn system_dispatch_workloadmsg_to_actors_test() {
    //     let mut syst = ActorSystem::new("system #1");

    //     // create two actors
    //     let msg = SystemCommand::CreateActors(2, String::from("raptor"));
    //     syst.on_receive(msg.into());

    //     // create two workload msg
    //     let mut workloads: Vec<TypedMessage> = vec![];

    //     workloads.push(TypedMessage::WorkloadMsg(Workload::new(OpCode::AddOp)));
    //     workloads.push(TypedMessage::WorkloadMsg(Workload::new(OpCode::SinOp)));
    //     syst.on_dispatch_workloads(workloads);

    //     // check if take effects
    //     let actor_reg = syst.actor_registry();
    //     assert_eq!(actor_reg.len(), 2);
    //     let actors: Vec<&Actor> = actor_reg.values().collect();
    //     // panic!("{:?}", actors[0]);
    //     // panic!("{:?}", actors[1]);
    //     // assert_eq!(actors[0].mailbox.len(), 2);
    // }
}
