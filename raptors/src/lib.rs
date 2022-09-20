pub mod actors;
pub mod builders;
pub mod cost_model;
pub mod executor;
pub mod mailbox;
pub mod messages;
pub mod system;
pub mod tensor_types;

/// Prelude module for users to import
///
/// ```
/// use raptors::prelude::*;
/// ```
pub mod prelude {
    pub use crate::actors::Actor;
    pub use crate::cost_model::{CostModel, OpCode};
    pub use crate::executor::{Executor, ExecutorLike};
    pub use crate::mailbox::{Address, Envelope, Len, Mailbox};
    pub use crate::messages::{
        ActorCommand, ActorMsg, LoadfreeMessage, MessageLike, PayloadMessage, RaptorMessage,
        SystemCommand, SystemMsg,
    };
    pub use crate::system::{ActorSystem, ActorSystemHandle, SystemBuilder, SystemConfig};
    pub use crate::tensor_types::{TensorLike, Workload, WorkloadMsg};

    // macros that simplifies the interfaces
    pub use crate::{build_loadfree_msg, build_msg, build_system, try_init_raptors};
}
