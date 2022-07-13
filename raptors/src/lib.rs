pub mod actors;
pub mod estimator;
pub mod estimator_builder;
pub mod executor;
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
    pub use crate::estimator::WorkloadEstimator;
    pub use crate::estimator_builder::WorkloadEstimatorBuilder;
    pub use crate::executor::Executor;
    pub use crate::messages::{OpCode, SystemCommand, TypedMessage, Workload};
    pub use crate::system::System;
    pub use crate::system_builder::SystemBuilder;
    pub use crate::system_config::SystemConfig;
}
