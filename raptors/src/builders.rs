use log::{debug, info};

use crate::actors::*;
use crate::messages::*;
use crate::system::*;
use crate::workloads::*;

/// Test build_system! macro
/// TODO, make tokio::main bind with systemconfig once started
/// ```
/// use raptors::prelude::*;
///
/// let mut system = build_system!("Raptors");
/// assert_eq!(system.name(), "Raptors");
/// ```
///
/// ```
/// use raptors::prelude::*;
///
/// let mut system = build_system!("Raptors", 3);
/// assert_eq!(system.name(), "Raptors");
/// ```
#[macro_export]
macro_rules! build_system {
    ($name:expr) => {{
        let mut sys_builder = SystemBuilder::new();
        let mut sys_config = SystemConfig::new($name, "info");
        sys_config.set_ranks(0 as usize);
        let system = sys_builder.build_with_config(sys_config);
        system
    }};
    ($name:expr, $cnt:expr) => {{
        let mut sys_builder = SystemBuilder::new();
        let mut sys_config = SystemConfig::new($name, "info");
        sys_config.set_ranks($cnt as usize);
        let system = sys_builder.build_with_config(sys_config);
        system
    }};
    ($name:expr, $log_level:expr) => {{
        let mut sys_builder = SystemBuilder::new();
        let mut sys_config = SystemConfig::new($name, $log_level);
        sys_config.set_ranks(0 as usize);
        let system = sys_builder.build_with_config(sys_config);
        system
    }};
    ($name:expr, $log_level:expr, $cnt:expr) => {{
        let mut sys_builder = SystemBuilder::new();
        let mut sys_config = SystemConfig::new($name, $log_level);
        sys_config.set_ranks($cnt usize);
        let system = sys_builder.build_with_config(sys_config);
        system
    }};
}

#[macro_export]
macro_rules! build_msg {
    ("destroy-all") => {
        TypedMessage::SystemMsg(SystemCommand::DestroyAll)
    };
    ("create-actors", $num:expr, $name:expr) => {
        TypedMessage::SystemMsg(SystemCommand::CreateActors($num, $name.to_string()))
    };
    ("start") => {
        TypedMessage::SystemMsg(SystemCommand::StartExecution)
    };
    ("test-one") => {
        TypedMessage::Testone
    };
    ("test-zero") => {
        TypedMessage::Testzero
    };
}

#[macro_export]
macro_rules! build_workload {
    ($x:expr) => {{
        $x.into_iter()
            .map(|x| Workload::new(x).into())
            .collect::<Vec<TypedMessage>>()
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn build_system_using_macro_test() {
    //     let system = build_system!("raptor");
    //     assert_eq!(system.name(), "raptor");
    // }

    #[test]
    fn build_system_with_config_using_macro_test() {
        let system = build_system!("raptor", 4);
        assert_eq!(system.name(), "raptor");
    }

    #[test]
    fn actor_destroy_macro_test() {
        let msg = build_msg!("destroy-all");
        assert_eq!(msg, TypedMessage::SystemMsg(SystemCommand::DestroyAll));
    }

    #[test]
    fn actor_create_macro_test() {
        let msg = build_msg!("create-actors", 3, "namebase");
        assert_eq!(
            msg,
            TypedMessage::SystemMsg(SystemCommand::CreateActors(3, "namebase".to_string()))
        );
    }
}
