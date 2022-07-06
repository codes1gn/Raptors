
use crate::actors::*;
use crate::messages::*;

/// TODO make dedicated mod and move it to there, maybe name it system_config.rs
/// test SystemConfig creation and get
///
/// Definition: SystemConfig contains the static data to describe the actor system
/// and used for builder to build the system.
/// It also contains the strategies, hardware/software environment info used for 
/// query purpose.
/// 
/// ```
/// use raptors::system;
///
/// let sys_config = system::SystemConfig::new();
/// let num_actor = sys_config.num_of_actors().unwrap_or_default();
/// assert_eq!(num_actor, 0);
///
/// ```
///
#[derive(Default)]
pub struct SystemConfig {
    num_of_actors: Option<usize>,
}

impl SystemConfig {
    pub fn new() -> Self {
        SystemConfig::default()
    }

    pub fn num_of_actors(&self) -> Option<usize> {
        self.num_of_actors
    }
}

/// Definition: The helper that provide helper functions for system creation
/// this class wraps all the complex logic used to build elements in;
/// actor system, for convenient.
///
/// backdoors for mocking tests are also provided by this helper 
///
/// ```
/// use raptors::system;
///
/// let sys_builder = system::SystemBuilder::new();
/// let sys_config = system::SystemConfig::new();
/// let syst = sys_builder.build_with_config("mock system".to_string(), sys_config);
/// assert_eq!(syst.name(), &"mock system".to_string());
/// ```
///
#[derive(Default)]
pub struct SystemBuilder {
    cfg: Option<SystemConfig>,
}

impl SystemBuilder {
    pub fn new() -> Self {
        SystemBuilder::default()
    }

    pub fn use_config(&mut self, config: SystemConfig) -> () {
        self.cfg = Some(config);
    }

    pub fn build_with_config(&self, name: String, config: SystemConfig) -> System {
        let num_of_actors = config.num_of_actors().unwrap_or_default();
        // panic!("{:?}", num_of_actors);
        System::new(name)
    }

    pub fn build(&self, name: String) -> System {
        let config = self.cfg.as_ref().expect("failed to unwrap config");
        System::new(name)
    }
    
}

/// System is the actor system that manages all the actors, supervisors and message channels
///
/// status: WIP
///
/// test system create
/// ```
/// use raptors::system::System;
///
/// let syst = System::new("system 1".to_string());
/// assert_eq!(syst.name(), &"system 1".to_string());
/// ```
///
/// test actor create
/// ```
/// use raptors::system::System;
/// use raptors::actors::Actor;
///
/// let syst = System::new("system 1".to_string());
/// let actor = syst.create_actor("raptor".to_string(), 17);
/// assert_eq!(actor.id(), 17);
/// assert_eq!(actor.name(), &"raptor".to_string());
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

impl System {
    pub fn new(name: String) -> Self {
        return Self {
            name: name,
            ..Default::default()
        }
    }

    pub fn create_actor(&self, actor_name: String, actor_id: usize) -> Actor {
        Actor::new(actor_name, actor_id)
    }

    // use base name and base id for temp use
    pub fn create_actors(&self, count: usize, base_name: String, base_id: usize) -> Vec<Actor> {
        let mut akts: Vec<Actor> = vec![];
        for idx in 1..count {
            let akt = Actor::new(format!("{} #{}", base_name, idx), base_id - 1 + idx);
            akts.push(akt);
        };
        akts
    }

    // TODO make the status code into Result struct
    ///
    ///
    /// ```
    /// use raptors::system;
    ///
    /// let mut syst = system::System::new("system #1".to_string());
    /// let actor = syst.create_actor("raptor".to_string(), 17);
    /// let status = syst.register_actor(actor);
    ///
    /// let query_actors = syst.actors().unwrap();
    ///
    /// assert_eq!(status, 0)
    /// ```
    pub fn register_actor(&mut self, actor: Actor) -> usize {
        match &mut self.actors {
            Some(v) => {
                v.push(actor);
            },
            None => { self.actors = Some(vec![actor]); },
        };
        0
    }
    // TODO support register multiple
    // TODO support MSG to create actor and register in system
    

    /// TODO support id and name for create actor MSG command
    ///
    /// ```
    /// use raptors::system::*;
    /// use raptors::messages::*;
    ///
    /// let mut syst = System::new("system #1".to_string());
    /// let msg = SystemCommand::CreateActor;
    /// syst.on_receive(msg.into());
    /// let query_actors = syst.actors().unwrap();
    /// assert_eq!(query_actors.len(), 1);
    /// assert_eq!(query_actors[0].name(), &"raptor".to_string());
    ///
    /// # // create one more actor
    /// let msg = SystemCommand::CreateActor;
    /// syst.on_receive(msg.into());
    /// let query_actors = syst.actors().unwrap();
    /// assert_eq!(query_actors.len(), 2);
    /// assert_eq!(query_actors[0].name(), &"raptor".to_string());
    /// ```
    pub fn on_receive(&mut self, msg: TypedMessage) -> usize {
        match msg {
            TypedMessage::SystemMsg(cmd) => {
                match cmd {
                    SystemCommand::CreateActor => {
                        let actor = self.create_actor("raptor".to_string(), 17);
                        let status = self.register_actor(actor);
                        // return usize currently
                        status
                    },
                    _ => panic!("not implemented"),
                }
            },
            _ => panic!("not implemented"),
        }
    }

    pub fn actors(&self) -> Option<&Vec<Actor>> {
        self.actors.as_ref()
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}


// unit tests
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn create_system() {
        let syst = System::new("raptor system".to_string());
        assert_eq!(syst.name(), &"raptor system".to_string());
    }

    #[test]
    fn config_test() {
        let sys_config = SystemConfig::new();
        let na = sys_config.num_of_actors().unwrap_or_default();
        assert_eq!(na, 0);
    }

    #[test]
    fn system_create_actor_test() {
        let syst = System::new("raptor system".to_string());
        let actor = syst.create_actor("raptor".to_string(), 17);
        assert_eq!(actor.id(), 17);
        assert_eq!(actor.name(), &"raptor".to_string());
    }

    #[test]
    fn system_create_actors_test() {
        let syst = System::new("raptor system".to_string());
        let actors = syst.create_actors(4, "raptor".to_string(), 17);
        assert_eq!(1, 1);
    }

    #[test]
    fn system_create_register_then_query_actor_test() {
        let mut syst = System::new("raptor system".to_string());

        // register
        let actor = syst.create_actor("raptor".to_string(), 17);
        let status = syst.register_actor(actor);
        
        // check result
        assert_eq!(status, 0);
        let query_actors = syst.actors().unwrap();
        assert_eq!(query_actors.len(), 1);
        assert_eq!(query_actors[0].name(), &"raptor".to_string());

        // register twice
        // TODO duplicating and identification of Actor
        // TODO duplicating and identification of System
        let actor = syst.create_actor("raptor2".to_string(), 19);
        let status = syst.register_actor(actor);
        
        // check result
        assert_eq!(status, 0);
        let query_actors = syst.actors().unwrap();
        assert_eq!(query_actors.len(), 2);
        assert_eq!(query_actors[0].name(), &"raptor".to_string());
        assert_eq!(query_actors[1].name(), &"raptor2".to_string());
    }
}
