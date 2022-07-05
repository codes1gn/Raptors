use crate::actors::Actor;

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
/// let syst = sys_builder.build("mock system".to_string(), sys_config);
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

    pub fn build(&self, name: String, config: SystemConfig) -> System {
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
/// let actor = syst.create_actor("dummy_name".to_string(), 17);
/// assert_eq!(actor.id(), 17);
/// assert_eq!(actor.name(), &"dummy_name".to_string());
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
pub struct System {
    name: String,
}

impl System {
    pub fn new(name: String) -> Self {
        return Self { name: name };
    }

    pub fn create_actor(&self, actor_name: String, actor_id: usize) -> Actor {
        Actor::new(actor_name, actor_id)
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
        let syst = System::new("system 1".to_string());
        assert_eq!(syst.name(), &"system 1".to_string());
    }

    #[test]
    fn config_test() {
        let sys_config = SystemConfig::new();
        let na = sys_config.num_of_actors().unwrap_or_default();
        assert_eq!(na, 0);
    }
}
