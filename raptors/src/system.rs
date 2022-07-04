
use crate::actors::{Actor};

pub struct System {}

/// System is the actor system that manages all the actors, supervisors and message channels
///
/// status: WIP
///
/// ```
/// use raptors::system::System;
/// use raptors::actors::Actor;
///
/// let syst = System::new();
/// let actor = syst.create_actor("dummy_name".to_string(), 17);
/// assert_eq!(actor.id(), 17);
/// assert_eq!(actor.name(), &"dummy_name".to_string());
/// ```
///
impl System {
    pub fn new() -> Self {
        return Self {}
    }

    pub fn create_actor(&self, actor_name: String, actor_id: usize) -> Actor {
        Actor::new(actor_name, actor_id)
    }
}


// unit tests
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn create_system() {
        let system = System::new();
        assert!(1 == 1);
    }
}
