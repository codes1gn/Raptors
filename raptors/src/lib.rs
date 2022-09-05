pub mod actors;
pub mod builders;
pub mod cost_model;
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
    pub use crate::cost_model::CostModel;
    pub use crate::executor::Executor;
    pub use crate::mailbox::{Address, Envelope, Len, Mailbox};
    pub use crate::messages::{ActorCommand, ActorMsg, SystemCommand, SystemMsg, TypedMessage};
    pub use crate::system::{ActorSystem, ActorSystemHandle, SystemBuilder, SystemConfig};
    pub use crate::workloads::{OpCode, Workload, WorkloadMsg};

    // macros that simplifies the interfaces
    pub use crate::{build_msg, build_system, build_workload, try_init_raptors};
}
