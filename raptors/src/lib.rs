pub mod actors;
pub mod executor;
pub mod messages;
pub mod system;
pub mod system_config;

/// Prelude module for users to import
///
/// ```
/// use raptors::prelude::*;
/// ```
pub mod prelude {
    pub use crate::actors::Actor;
    pub use crate::executor::Executor;
    pub use crate::messages::{SystemCommand, TypedMessage, Workload};
    pub use crate::system::{System, SystemBuilder};
    pub use crate::system_config::SystemConfig;
}
