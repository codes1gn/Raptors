use crate::actors::*;
use crate::messages::*;
use crate::system::System;
use crate::system_config::SystemConfig;

/// Definition: The helper that provide helper functions for system creation
/// this class wraps all the complex logic used to build elements in;
/// actor system, for convenient.
///
/// backdoors for mocking tests are also provided by this helper
///
/// ```
/// use raptors::prelude::*;
///
/// let sys_builder = SystemBuilder::new();
/// let sys_config = SystemConfig::new();
/// let syst = sys_builder.build_with_config("mock system", sys_config);
/// assert_eq!(syst.name(), "mock system".to_string());
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

    pub fn build_with_config(&self, name: &str, config: SystemConfig) -> System {
        let num_of_actors = config.num_of_actors().unwrap_or_default();
        println!("{:?}", num_of_actors);
        System::new(name)
    }

    pub fn build(&self, name: &str) -> System {
        let config = self.cfg.as_ref().expect("failed to unwrap config");
        println!("{:?}", config);
        // TODO build system with configs
        System::new(name)
    }
}


#[cfg(test)]

mod tests {
    use super::*;

}
