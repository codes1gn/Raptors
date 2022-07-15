pub mod actors;
pub mod estimator;
pub mod executor;
pub mod messages;
pub mod messages_builder;
pub mod syscmd_builder;
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
    pub use crate::estimator::WorkloadEstimator;
    pub use crate::executor::Executor;
    pub use crate::messages::{
        OpCode, SystemCommand, SystemMsg, TypedMessage, Workload, WorkloadMsg,
    };
    pub use crate::system::System;
    pub use crate::system_builder::SystemBuilder;
    pub use crate::system_config::SystemConfig;
}
