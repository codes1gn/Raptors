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
/// #[tokio::main]
/// async fn main() {
///     let mut system = build_system!("Raptors", 3);
///     assert_eq!(system.name(), "Raptors");
///     assert_eq!(system.ranks(), 3);
/// }
/// ```
#[macro_export]
macro_rules! build_system {
    ($name:expr) => {{
        let mut sys_config = SystemConfig::new($name, "info");
        let mut sys_builder = SystemBuilder::new();
        sys_config.set_ranks(0 as usize);
        let system = sys_builder.build_with_config(sys_config);
        system
    }};
    ($name:expr, $cnt:expr) => {{
        let mut sys_config = SystemConfig::new($name, "info");
        let mut sys_builder = SystemBuilder::new();
        sys_config.set_ranks($cnt as usize);
        let system = sys_builder.build_with_config(sys_config);
        system
    }};
}

#[macro_export]
macro_rules! try_init_raptors {
    ($log_level:expr) => {
        std::env::set_var("RUST_LOG", $log_level);
        // to make more precise timestamps
        Builder::new()
            .format(|buf, record| {
                writeln!(
                    buf,
                    "{} {}: {}",
                    record.level(),
                    Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                    record.args()
                )
            })
            .filter(None, LevelFilter::Info)
            .try_init();
    };
}

#[macro_export]
macro_rules! build_msg {
    ("halt-all") => {
        TypedMessage::SystemMsg(SystemCommand::HaltAll)
    };
    ("halt", $index:expr) => {
        TypedMessage::SystemMsg(SystemCommand::HaltOn($index))
    };
    ("spawn", $num:expr) => {
        TypedMessage::SystemMsg(SystemCommand::Spawn($num))
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

    #[tokio::test]
    async fn build_system_using_macro_test0() {
        let system = build_system!("raptor");
        assert_eq!(system.name(), "raptor");
    }

    #[tokio::test]
    async fn build_system_using_macro_test1() {
        let system = build_system!("raptor", 4);
        assert_eq!(system.name(), "raptor");
        assert_eq!(system.ranks(), 4);
    }

    #[test]
    fn build_halt_all_test() {
        let msg = build_msg!("halt-all");
        assert_eq!(msg, TypedMessage::SystemMsg(SystemCommand::HaltAll));
    }

    #[test]
    fn build_spawn_msg_test() {
        let msg = build_msg!("spawn", 3);
        assert_eq!(msg, TypedMessage::SystemMsg(SystemCommand::Spawn(3)));
    }

    #[test]
    fn build_halt_msg_test() {
        let msg = build_msg!("halt", 3);
        assert_eq!(msg, TypedMessage::SystemMsg(SystemCommand::HaltOn(3)));
    }
}
