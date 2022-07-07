use crate::actors::*;

/// TODO(short-term) make dedicated mod and move it to there, maybe name it system_config.rs
/// test SystemConfig creation and get
///
/// Definition: SystemConfig contains the static data to describe the actor system
/// and used for builder to build the system.
/// It also contains the strategies, hardware/software environment info used for
/// query purpose.
///
/// ```
/// use raptors::prelude::*;
///
/// let sys_config = SystemConfig::new();
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


// unit tests
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn config_test() {
        let sys_config = SystemConfig::new();
        let na = sys_config.num_of_actors().unwrap_or_default();
        assert_eq!(na, 0);
    }
}
