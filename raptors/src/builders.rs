use tracing::{debug, info};
// use tracing_subscriber::{registry::Registry, prelude::*};
// use tracing_chrome::ChromeLayerBuilder;

use crate::actors::*;
use crate::cost_model::OpCode;
use crate::executor::*;
use crate::messages::*;
use crate::system::*;
use crate::tensor_types::*;

/// Test build_system! macro
///
/// ```
/// use raptors::prelude::*;
///
/// #[tokio::main]
/// async fn main() {
///     let mut system = build_system!("Raptors", 3);
///     assert_eq!(system.name(), "Raptors");
///     // TODO-FIX#1
///     // assert_eq!(system.ranks(), 3);
/// }
/// ```
#[macro_export]
macro_rules! build_system {
    ($name:expr) => {{
        let mut sys_config = SystemConfig::new($name, "info");
        let mut sys_builder = SystemBuilder::new();
        sys_config.set_ranks(0 as usize);
        let system = sys_builder.build_with_config::<Executor, Workload>(sys_config);
        system
    }};
    ($name:expr, $cnt:expr) => {{
        let mut sys_config = SystemConfig::new($name, "info");
        let mut sys_builder = SystemBuilder::new();
        sys_config.set_ranks($cnt as usize);
        let system = sys_builder.build_with_config::<Executor, Workload>(sys_config);
        system
    }};
}

#[macro_export]
macro_rules! try_init_raptors {
    ($log_level:expr) => {
        // std::env::set_var("RUST_LOG", $log_level);
        // to make more precise timestamps
        //
        // TODO this part maybe replaced by tracing::info, since it can do the both
        //
        // Builder::new()
        //     .format(|buf, record| {
        //         writeln!(
        //             buf,
        //             "{} {}: {}",
        //             record.level(),
        //             Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
        //             record.args()
        //         )
        //     })
        //     .filter(None, LevelFilter::Info)
        //     .try_init();

        // let _guard = if std::env::args().any(|arg| arg == "--no-trace") {
        //     None
        // } else {
        //     let (chrome_layer, guard) = tracing_chrome::ChromeLayerBuilder::new()
        //         .include_args(true)
        //         .build();
        //     tracing_subscriber::registry().with(chrome_layer).try_init();
        //     Some(guard)
        // };
    };
}

// msg used for CRT compute, all messages are based on <U = TensorView<f32>>
#[macro_export]
macro_rules! build_comp_msg {
    ("add-op") => {
        LoadfreeMessage::WorkloadMsg(Workload::new(OpCode::AddOp))
    };
    ("sub-op") => {
        LoadfreeMessage::WorkloadMsg(Workload::new(OpCode::SubOp))
    };
}

#[macro_export]
macro_rules! build_msg {
    ("halt-all") => {
        LoadfreeMessage::SystemMsg(SystemCommand::HaltAll)
    };
    ("halt", $index:expr) => {
        LoadfreeMessage::SystemMsg(SystemCommand::HaltOn($index))
    };
    ("spawn", $num:expr) => {
        LoadfreeMessage::SystemMsg(SystemCommand::Spawn($num))
    };

    // actor msg
    ("available", $num:expr) => {
        LoadfreeMessage::ActorMsg(ActorCommand::Available($num))
    };

    // operation workload msg
    ("identity-op") => {
        LoadfreeMessage::WorkloadMsg(Workload::new(OpCode::IdentityOp))
    };
    ("add-op") => {
        LoadfreeMessage::WorkloadMsg(Workload::new(OpCode::AddOp))
    };
    ("sub-op") => {
        LoadfreeMessage::WorkloadMsg(Workload::new(OpCode::SubOp))
    };
    ("exp-op") => {
        LoadfreeMessage::WorkloadMsg(Workload::new(OpCode::ExpOp))
    };
    ("sin-op") => {
        LoadfreeMessage::WorkloadMsg(Workload::new(OpCode::SinOp))
    };
    ("Matmul-op") => {
        LoadfreeMessage::WorkloadMsg(Workload::new(OpCode::MatmulOp))
    };
    ("Conv-op") => {
        LoadfreeMessage::WorkloadMsg(Workload::new(OpCode::ConvOp))
    };
}

#[macro_export]
macro_rules! build_workload {
    ($x:expr) => {{
        $x.into_iter()
            .map(|x| Workload::new(x).into())
            .collect::<Vec<LoadfreeMessage>>()
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
        // TODO-FIX#1, currently not spawn at creation due to async-sync
        // assert_eq!(system.ranks(), 4);
    }

    #[test]
    fn build_halt_all_test() {
        let msg: LoadfreeMessage<Workload> = build_msg!("halt-all");
        assert_eq!(msg, LoadfreeMessage::SystemMsg(SystemCommand::HaltAll));
    }

    #[test]
    fn build_spawn_msg_test() {
        let msg: LoadfreeMessage<Workload> = build_msg!("spawn", 3);
        assert_eq!(msg, LoadfreeMessage::SystemMsg(SystemCommand::Spawn(3)));
    }

    #[test]
    fn build_halt_msg_test() {
        let msg: LoadfreeMessage<Workload> = build_msg!("halt", 3);
        assert_eq!(msg, LoadfreeMessage::SystemMsg(SystemCommand::HaltOn(3)));
    }

    #[test]
    fn build_add_op_test() {
        let msg: LoadfreeMessage<Workload> = build_msg!("add-op");
        assert_eq!(msg, LoadfreeMessage::WorkloadMsg(Workload::new(OpCode::AddOp)));
    }

    #[test]
    fn build_exp_op_test() {
        let msg: LoadfreeMessage<Workload> = build_msg!("exp-op");
        assert_eq!(msg, LoadfreeMessage::WorkloadMsg(Workload::new(OpCode::ExpOp)));
    }

    #[test]
    fn build_actor_available_msg_test() {
        let msg: LoadfreeMessage<Workload> = build_msg!("available", 0);
        assert_eq!(
            msg,
            LoadfreeMessage::<Workload>::ActorMsg(ActorCommand::Available(0))
        );
    }
}
