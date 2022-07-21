use uuid::Uuid;

use std::collections::HashMap;

use crate::actors::*;
use crate::mailbox::*;
use crate::messages::*;
use crate::system_config::SystemConfig;

/// System is the actor system that manages all the actors, supervisors and message channels
///
/// status: WIP
///
/// test system create
/// ```
/// use raptors::prelude::*;
///
/// let syst = System::new("system 1");
/// assert_eq!(syst.name(), "system 1".to_string());
/// ```
///
/// workflow of System
/// step 1: new a system_builder
/// step 2: customise a actor_config
/// step 3: create system by system_builder with name and config
/// step 4: create actor with name and config
/// step 5: create a context from system? (pros: we can make a registry for that to query quickly)
/// step 6: actor start or init with ctx
///
#[derive(Debug)]
pub struct System {
    name: String,
    actor_registry: HashMap<Uuid, Actor>,
    mailbox_registry: HashMap<Address, Mailbox>,
}

impl Default for System {
    fn default() -> Self {
        System {
            name: String::from("Raptor System"),
            actor_registry: HashMap::new(),
            mailbox_registry: HashMap::new(),
        }
    }
}

// TODO(max): make return type as Result<(Actor/Vec<Actor>), Err> to meld ErrMsg
impl System {
    pub fn new(name: &str) -> Self {
        return Self {
            name: String::from(name),
            ..Default::default()
        };
    }

    fn create_actor(&self, actor_name: &str) -> Actor {
        Actor::new(actor_name)
    }

    // use base name and base id for temp use
    // TODO(albert, short-term): name redirection, maybe append one region from uuid
    // ```
    // use raptors::prelude::*;
    //
    // let syst = System::new("raptor system");
    // let actors = syst.create_actors(4, "raptor");
    // assert_eq!(actors.len(), 4);
    // ```
    fn create_actors(&self, count: usize, base_name: &str) -> Vec<Actor> {
        let mut akts: Vec<Actor> = vec![];
        for idx in 0..count {
            let akt = Actor::new(format!("{} #{}", base_name, idx).as_str());
            akts.push(akt);
        }
        akts
    }

    pub fn destroy_actors(&mut self) -> Result<(), String> {
        self.actor_registry.clear();
        Ok(())
    }

    pub fn destroy_mailboxes(&mut self) -> Result<(), String> {
        self.mailbox_registry.clear();
        Ok(())
    }

    pub fn register_actor(&mut self, actor: Actor) -> Result<(), String> {
        let mailbox = Mailbox::new();
        self.mailbox_registry.insert(actor.addr(), mailbox);
        self.actor_registry.insert(actor.id(), actor);
        Ok(())
    }

    pub fn register_actors(&mut self, mut actors: Vec<Actor>) -> Result<(), String> {
        // println!("before register {:?}", self.actor_registry);
        actors
            .into_iter()
            .map(|actor| {
                self.actor_registry.insert(actor.id(), actor);
            })
            .collect::<()>();
        // TODO use more elegant way to logging, such as auto-enable/config for methods logging
        // println!("after register {:?}", self.actor_registry);
        Ok(())
    }

    // TODO support register multiple
    // TODO support MSG to create actor and register in system

    /// TODO support id and name for create actor MSG command
    /// TODO convert String into ErrMsg in future
    /// TODO test run on_receive twice, and add naming redirection in it
    ///
    /// ```
    /// use raptors::prelude::*;
    ///
    /// let mut syst = System::new("system #1");
    /// let msg = SystemCommand::CreateActor(1, String::from("raptor"));
    /// syst.on_receive(msg.into());
    /// let actor_reg = syst.actor_registry();
    /// assert_eq!(actor_reg.len(), 1);
    /// let actors: Vec<&Actor> = actor_reg.values().collect();
    /// assert_eq!(actors[0].name(), "raptor #0".to_string());
    /// ```
    ///
    /// ```
    /// use raptors::prelude::*;
    ///
    /// let mut syst = System::new("system #1");
    /// let msg = SystemCommand::CreateActor(2, String::from("raptor"));
    /// syst.on_receive(msg.into());
    /// let actor_reg = syst.actor_registry();
    /// assert_eq!(actor_reg.len(), 2);
    /// let mut actors: Vec<&Actor> = actor_reg.values().collect();
    /// actors.sort_unstable();
    /// println!("{:?}", actors);
    /// assert_eq!(actors[0].name(), "raptor #0".to_string());
    /// assert_eq!(actors[1].name(), "raptor #1".to_string());
    /// ```
    #[allow(unreachable_patterns)]
    pub fn on_receive(&mut self, msg: TypedMessage) -> Result<(), String> {
        match msg {
            TypedMessage::WorkloadMsg(ref workload) => {
                // TODO dispatch it to actors
                // use a tmp queue to manage it and later switch to mailbox
                // TODO need a envelope object in message types, later
                let mut actors: Vec<&mut Actor> = self
                    .actor_registry
                    .values_mut()
                    .collect::<Vec<&mut Actor>>();
                // TODO replace this hardcode logic, with envelope with uuid, or add addr for
                // contain msgs, and actors to receive by themself once started.
                actors[0].receive_msg(msg)
            }
            TypedMessage::SystemMsg(cmd) => {
                match cmd {
                    SystemCommand::CreateActor(cnt, base_name) => {
                        // let actor = self.create_actor("raptor");
                        let actors = self.create_actors(cnt, &base_name);
                        let status = self.register_actors(actors);
                        // return usize currently
                        match status {
                            Ok(_) => Ok(()),
                            Err(_e) => Err("Fail to register the actor".to_string()),
                        }
                    }
                    SystemCommand::DestroyAllActors => self.destroy_actors(),
                    _ => Err("not implemented".to_string()),
                }
            }
            _ => Err("not implemented".to_string()),
        }
    }

    pub fn on_dispatch(&mut self, workloads: Vec<TypedMessage>) -> Result<(), String> {
        let status = workloads
            .into_iter()
            .map(|msg| -> Result<(), String> { self.on_receive(msg) })
            .collect::<Result<(), String>>();
        status
    }

    pub fn actor_registry(&self) -> &HashMap<Uuid, Actor> {
        &self.actor_registry
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_system() {
        let syst = System::new("raptor system");
        assert_eq!(syst.name(), "raptor system");
    }

    #[test]
    fn system_create_actor_test() {
        let syst = System::new("raptor system");
        let actor = syst.create_actor("raptor");
        assert_eq!(actor.name(), "raptor");
    }

    #[test]
    fn system_create_actors_test() {
        let syst = System::new("raptor system");
        let actors = syst.create_actors(4, "raptor");
        assert_eq!(actors.len(), 4);
        // TODO add more asserts
    }

    #[test]
    fn system_create_register_then_query_actor_from_map_test() {
        let mut syst = System::new("raptor system");

        // register
        let actor = syst.create_actor("raptor");
        let status = syst.register_actor(actor);

        // check result
        assert!(status.is_ok());
        let mactor = syst.actor_registry();
        assert_eq!(mactor.len(), 1);
    }

    #[test]
    fn on_receive_not_systemmsg_test() {
        let mut syst = System::new("system #1");
        let msg = TypedMessage::ActorMsg;
        let status = syst.on_receive(msg.into());
        assert!(status.is_err());
        assert_eq!(status.unwrap_err(), "not implemented".to_string());
    }

    #[test]
    fn system_create_then_register_multiple_actors_test() {
        let mut syst = System::new("raptor system");

        // register
        let actors = syst.create_actors(2, "raptor");
        let query_id = actors
            .iter()
            .map(|actor| actor.id().clone())
            .collect::<Vec<Uuid>>();
        let status = syst.register_actors(actors);

        // check result
        assert_eq!(status.is_ok(), true);
        let query_actors = syst.actor_registry();
        assert_eq!(query_actors.len(), 2);
        assert_eq!(
            query_actors.get(&query_id[0]).unwrap().name(),
            "raptor #0".to_string()
        );
        assert_eq!(
            query_actors.get(&query_id[1]).unwrap().name(),
            "raptor #1".to_string()
        );
    }

    #[test]
    fn system_dispatch_workloadmsg_to_actors_test() {
        let mut syst = System::new("system #1");

        // create two actors
        let msg = SystemCommand::CreateActor(2, String::from("raptor"));
        syst.on_receive(msg.into());

        // create two workload msg
        let mut workloads: Vec<TypedMessage> = vec![];

        workloads.push(TypedMessage::WorkloadMsg(Workload::new(16, OpCode::AddOp)));
        workloads.push(TypedMessage::WorkloadMsg(Workload::new(16, OpCode::SinOp)));
        syst.on_dispatch(workloads);

        // check if take effects
        let actor_reg = syst.actor_registry();
        assert_eq!(actor_reg.len(), 2);
        let actors: Vec<&Actor> = actor_reg.values().collect();
        // panic!("{:?}", actors[0]);
        // panic!("{:?}", actors[1]);
        // assert_eq!(actors[0].mailbox.len(), 2);
    }
}
