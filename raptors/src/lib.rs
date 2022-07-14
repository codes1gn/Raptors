pub mod actors;
pub mod estimator;
pub mod executor;
pub mod mailbox;
pub mod messages;
pub mod system;
pub mod system_builder;
pub mod system_config;

/// Prelude module for users to import
///
/// ```
/// use raptors::prelude::*;
/// ```
pub mod prelude {
    pub use crate::actors::Actor;
    pub use crate::executor::Executor;
    pub use crate::mailbox::{Len, Mailbox};
    pub use crate::messages::{OpCode, Workload};
    pub use crate::messages::{SystemCommand, TypedMessage};
    pub use crate::system::System;
    pub use crate::system_builder::SystemBuilder;
    pub use crate::system_config::SystemConfig;
}
