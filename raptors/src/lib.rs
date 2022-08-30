pub mod actors;
pub mod builders;
pub mod estimator;
pub mod executor;
pub mod mailbox;
pub mod messages;
pub mod scheduler;
pub mod system;
pub mod workloads;

/// Prelude module for users to import
///
/// ```
/// use raptors::prelude::*;
/// ```
pub mod prelude {
    pub use crate::actors::Actor;
    pub use crate::estimator::WorkloadEstimator;
    pub use crate::executor::Executor;
    pub use crate::mailbox::{Address, Envelope, Len, Mailbox};
    pub use crate::messages::{SystemCommand, SystemMsg, TypedMessage};
    // pub use crate::scheduler::pre_schedule;
    pub use crate::system::{ActorSystem, SystemBuilder, SystemConfig};
    pub use crate::workloads::{OpCode, Workload, WorkloadMsg};

    // macros that simplifies the interfaces
    pub use crate::{build_msg, build_system, build_workload};
}
