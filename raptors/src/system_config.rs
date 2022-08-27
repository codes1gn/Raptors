use log::{debug, info};

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
/// let sys_config = SystemConfig::new("named");
/// let num_actor = sys_config.ranks().unwrap_or_default();
/// assert_eq!(num_actor, 0);
///
/// ```
///
#[derive(Default, Debug)]
pub struct SystemConfig {
    name: String,
    ranks: Option<usize>,
}

impl SystemConfig {
    pub fn new(name: &str) -> Self {
        debug!("SystemConfig::new");
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

// unit tests
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn config_test() {
        let sys_config = SystemConfig::new("raptors");
        let na = sys_config.ranks().unwrap_or_default();
        assert_eq!(na, 0);
    }
}
