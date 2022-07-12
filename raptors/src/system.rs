use crate::actors::*;
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
#[derive(Default)]
pub struct System {
    name: String,
    actors: Option<Vec<Actor>>,
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

    pub fn destroy_all_actors(&mut self) -> Result<(), String> {
        match &mut self.actors {
            Some(v) => {
                v.clear();
                Ok(())
            }
            None => {
                Ok(())
            }
        }
    }

    // TODO(short-term)(Almost Done) make the status code into Result struct
    // TODO(max): extend ErrMsg for fn: register_actor, currently it always returns Ok()
    pub fn register_actor(&mut self, actor: Actor) -> Result<(), String> {
        match &mut self.actors {
            Some(v) => {
                v.push(actor);
                Ok(())
            }
            None => {
                self.actors = Some(vec![actor]);
                Ok(())
            }
        }
    }

    fn register_actors(&mut self, mut actors: Vec<Actor>) -> Result<(), String> {
        match &mut self.actors {
            Some(v) => {
                v.append(&mut actors);
                Ok(())
            }
            None => {
                self.actors = Some(actors);
                Ok(())
            }
        }
    }

    // TODO support register multiple
    // TODO support MSG to create actor and register in system

    /// TODO support id and name for create actor MSG command
    /// TODO convert String into ErrMsg in future
    ///
    /// ```
    /// use raptors::prelude::*;
    ///
    /// let mut syst = System::new("system #1");
    /// let msg = SystemCommand::CreateActor(1, String::from("raptor"));
    /// syst.on_receive(msg.into());
    /// let query_actors = syst.actors().unwrap();
    /// assert_eq!(query_actors.len(), 1);
    /// assert_eq!(query_actors[0].name(), "raptor #0".to_string());
    ///
    /// # // create one more actor
    /// let msg = SystemCommand::CreateActor(1, String::from("raptor"));
    /// syst.on_receive(msg.into());
    /// let query_actors = syst.actors().unwrap();
    /// assert_eq!(query_actors.len(), 2);
    /// assert_eq!(query_actors[0].name(), "raptor #0".to_string());
    /// ```
    #[allow(unreachable_patterns)]
    pub fn on_receive(&mut self, msg: TypedMessage) -> Result<(), String> {
        match msg {
            TypedMessage::SystemMsg(cmd) => {
                match cmd {
                    SystemCommand::CreateActor(cnt, base_name) => {
                        // let actor = self.create_actor("raptor");
                        let mut actors = self.create_actors(cnt, &base_name);
                        let status = self.register_actors(actors);
                        // return usize currently
                        match status {
                            Ok(_) => Ok(()),
                            Err(_e) => Err("Fail to register the actor".to_string()),
                        }
                    }
                    SystemCommand::DestroyAllActors => {
                        self.destroy_all_actors()
                    }
                    _ => Err("not implemented".to_string()),
                }
            }
            _ => Err("not implemented".to_string()),
        }
    }

    pub fn actors(&self) -> Option<&Vec<Actor>> {
        self.actors.as_ref()
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
    fn system_create_register_then_query_actor_test() {
        let mut syst = System::new("raptor system");

        // register
        let actor = syst.create_actor("raptor");
        let status = syst.register_actor(actor);

        // check result
        assert!(status.is_ok());
        let query_actors = syst.actors().unwrap();
        assert_eq!(query_actors.len(), 1);
        assert_eq!(query_actors[0].name(), "raptor".to_string());

        // register twice
        // duplicating and identification of Actor
        // duplicating and identification of System
        let actor = syst.create_actor("raptor2");
        let status = syst.register_actor(actor);

        // check result
        assert!(status.is_ok());
        let query_actors = syst.actors().unwrap();
        assert_eq!(query_actors.len(), 2);
        assert_eq!(query_actors[0].name(), "raptor".to_string());
        assert_eq!(query_actors[1].name(), "raptor2".to_string());
    }

    #[test]
    #[ignore]
    fn register_actor_fail_test() {
        let mut syst = System::new("raptor system");

        // register
        let actor = syst.create_actor("raptor");
        let status = syst.register_actor(actor);
        // register_actor only returns Ok() currently, extend its ErrorMsg in the future
        assert!(status.is_err());
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
        let status = syst.register_actors(actors);

        // check result
        assert_eq!(status.is_ok(), true);
        let query_actors = syst.actors().unwrap();
        assert_eq!(query_actors.len(), 2);
        assert_eq!(query_actors[0].name(), "raptor #0".to_string());
        assert_eq!(query_actors[1].name(), "raptor #1".to_string());
    }
}
