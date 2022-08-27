use log::{debug, info};

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

    pub fn build_with_config(&mut self, config: SystemConfig) -> System {
        self.cfg = Some(config);
        System::new(&self.config().name().to_owned())
    }

    fn config(&self) -> &SystemConfig {
        &self.cfg.as_ref().unwrap()
    }
}

#[macro_export]
macro_rules! build_system {
    ($name:expr) => {{
        System::new($name)
    }};
    ($name:expr, $actor_cnt:expr) => {{
        let mut sys_builder = SystemBuilder::new();
        let mut sys_config = SystemConfig::new($name);
        sys_config.set_ranks($actor_cnt as usize);
        let system = sys_builder.build_with_config(sys_config);
        system
    }};
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn build_system_using_macro_test() {
        let system = build_system!("raptor");
        assert_eq!(system.name(), "raptor");
    }

    #[test]
    fn build_system_with_config_using_macro_test() {
        let system = build_system!("raptor", 4);
        assert_eq!(system.name(), "raptor");
    }
}
