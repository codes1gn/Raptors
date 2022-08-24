pub mod actors;
pub mod command_builder;
pub mod estimator;
pub mod executor;
pub mod mailbox;
pub mod messages;
pub mod system;
pub mod system_builder;
pub mod system_config;
pub mod workload_builder;

/// Prelude module for users to import
///
/// ```
/// use raptors::prelude::*;
/// ```
pub mod prelude {
    pub use crate::actors::Actor;
    pub use crate::command_builder::CommandBuilder;
    pub use crate::estimator::WorkloadEstimator;
    pub use crate::executor::Executor;
    pub use crate::mailbox::{Address, Envelope, Len, Mailbox};
    pub use crate::messages::{
        OpCode, SystemCommand, SystemMsg, TypedMessage, Workload, WorkloadMsg,
    };
    pub use crate::system::System;
    pub use crate::system_builder::SystemBuilder;
    pub use crate::system_config::SystemConfig;
    pub use crate::workload_builder::WorkloadBuilder;
}
