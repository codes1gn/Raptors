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
/// let num_actor = sys_config.amount_of_actors().unwrap_or_default();
/// assert_eq!(num_actor, 0);
///
/// ```
///
#[derive(Default, Debug)]
pub struct SystemConfig {
    amount_of_actors: Option<usize>,
}

impl SystemConfig {
    pub fn new() -> Self {
        println!("SystemConfig::new");
        SystemConfig::default()
    }

    pub fn set_amount_of_actors(&mut self, num: usize) -> () {
        self.amount_of_actors = Some(num);
    }

    pub fn amount_of_actors(&self) -> Option<usize> {
        self.amount_of_actors
    }
}

// unit tests
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn config_test() {
        let sys_config = SystemConfig::new();
        let na = sys_config.amount_of_actors().unwrap_or_default();
        assert_eq!(na, 0);
    }
}
