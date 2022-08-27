use log::{debug, info};

use crate::actors::*;
use crate::messages::*;
use crate::system::*;

#[macro_export]
macro_rules! build_system {
    ($name:expr) => {{
        System::new($name)
    }};
    ($name:expr, $actor_cnt:expr) => {{
        let mut sys_builder = SystemBuilder::new();
        let mut sys_config = SystemConfig::new($name);
        sys_config.set_ranks($actor_cnt as usize);
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_system_using_macro_test() {
        let system = build_system!("raptor");
        assert_eq!(system.name(), "raptor");
    }

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
