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
    pub use crate::messages::{TypedMessage, SystemCommand, Workload};
    pub use crate::actors::{Actor};
    pub use crate::system::{SystemBuilder, System};
    pub use crate::system_config::{SystemConfig};
    pub use crate::executor::{Executor};
}

