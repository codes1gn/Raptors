// use tracing_subscriber::{registry::Registry, prelude::*};
// use tracing_chrome::ChromeLayerBuilder;

use crate::cost_model::MockOpCode;
use crate::executor::*;
use crate::messages::*;
use crate::system::*;
use crate::tensor_types::*;

/// Test build_mock_system! macro
///
/// ```
/// use raptors::prelude::*;
///
/// #[tokio::main]
/// async fn main() {
///     let mut system = build_mock_system!("Raptors", 3);
///     assert_eq!(system.name(), "Raptors");
///     // TODO-FIX#1
///     // assert_eq!(system.ranks(), 3);
/// }
/// ```
#[macro_export]
macro_rules! build_mock_system {
    ($name:expr) => {{
        let mut sys_config = SystemConfig::new($name, "info");
        let mut sys_builder = SystemBuilder::new();
        sys_config.set_ranks(0 as usize);
        let system =
            sys_builder.build_with_config::<MockExecutor, MockTensor, MockOpCode>(sys_config);
        system
    }};
    ($name:expr, $cnt:expr) => {{
        let mut sys_config = SystemConfig::new($name, "info");
        let mut sys_builder = SystemBuilder::new();
        sys_config.set_ranks($cnt as usize);
        let system =
            sys_builder.build_with_config::<MockExecutor, MockTensor, MockOpCode>(sys_config);
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

// TODO add builder for payload msg
// #[macro_export]
// macro_rules! build_payload_msg {
//     ("add-op") => {
//         LoadfreeMessage::MockTensorMsg(MockTensor::new(MockOpCode::AddOp))
//     };
//     ("sub-op") => {
//         LoadfreeMessage::MockTensorMsg(MockTensor::new(MockOpCode::SubOp))
//     };
//     };
//     ("exp-op") => {
//         LoadfreeMessage::MockTensorMsg(MockTensor::new(MockOpCode::ExpOp))
//     };
//     ("sin-op") => {
//         LoadfreeMessage::MockTensorMsg(MockTensor::new(MockOpCode::SinOp))
//     };
//     ("Matmul-op") => {
//         LoadfreeMessage::MockTensorMsg(MockTensor::new(MockOpCode::MatmulOp))
//     };
//     ("Conv-op") => {
//         LoadfreeMessage::MockTensorMsg(MockTensor::new(MockOpCode::ConvOp))
//     };
// }

#[macro_export]
macro_rules! build_loadfree_msg {
    ("halt-all") => {
        LoadfreeMessage::SystemMsg(SystemCommand::HaltAll)
    };
    ("halt", $index:expr) => {
        LoadfreeMessage::SystemMsg(SystemCommand::HaltOn($index))
    };
    ("spawn", $typestr:expr, $num:expr) => {
        match $typestr {
            "mock" => LoadfreeMessage::SystemMsg(SystemCommand::Spawn(0 as usize, $num)),
            "vulkan" => LoadfreeMessage::SystemMsg(SystemCommand::Spawn(1 as usize, $num)),
            _ => panic!("fail to spawn"),
        }
    };

    // actor msg
    ("available", $num:expr) => {
        LoadfreeMessage::ActorMsg(ActorCommand::Available($num))
    };

    // operation workload msg
    ("identity-op") => {
        LoadfreeMessage::MockTensorMsg(MockTensor::new(MockOpCode::IdentityOp))
    };
    ("add-op") => {
        LoadfreeMessage::MockTensorMsg(MockTensor::new(MockOpCode::AddOp))
    };
    ("sub-op") => {
        LoadfreeMessage::MockTensorMsg(MockTensor::new(MockOpCode::SubOp))
    };
    ("exp-op") => {
        LoadfreeMessage::MockTensorMsg(MockTensor::new(MockOpCode::ExpOp))
    };
    ("sin-op") => {
        LoadfreeMessage::MockTensorMsg(MockTensor::new(MockOpCode::SinOp))
    };
    ("Matmul-op") => {
        LoadfreeMessage::MockTensorMsg(MockTensor::new(MockOpCode::MatmulOp))
    };
    ("Conv-op") => {
        LoadfreeMessage::MockTensorMsg(MockTensor::new(MockOpCode::ConvOp))
    };
}

#[macro_export]
macro_rules! build_msg {
    ("halt-all") => {
        RaptorMessage::LoadfreeMSG(LoadfreeMessage::SystemMsg(SystemCommand::HaltAll))
    };
    ("halt", $index:expr) => {
        RaptorMessage::LoadfreeMSG(LoadfreeMessage::SystemMsg(SystemCommand::HaltOn($index)))
    };
    ("spawn", $num:expr) => {
        RaptorMessage::LoadfreeMSG(LoadfreeMessage::SystemMsg(SystemCommand::Spawn($num)))
    };

    // actor msg
    ("available", $num:expr) => {
        RaptorMessage::LoadfreeMSG(LoadfreeMessage::ActorMsg(ActorCommand::Available($num)))
    };

    // operation workload msg
    ("identity-op") => {
        RaptorMessage::LoadfreeMSG(LoadfreeMessage::MockTensorMsg(MockTensor::new(
            MockOpCode::IdentityOp,
        )))
    };
    ("add-op") => {
        RaptorMessage::LoadfreeMSG(LoadfreeMessage::MockTensorMsg(MockTensor::new(
            MockOpCode::AddOp,
        )))
    };
    ("sub-op") => {
        RaptorMessage::LoadfreeMSG(LoadfreeMessage::MockTensorMsg(MockTensor::new(
            MockOpCode::SubOp,
        )))
    };
    ("exp-op") => {
        RaptorMessage::LoadfreeMSG(LoadfreeMessage::MockTensorMsg(MockTensor::new(
            MockOpCode::ExpOp,
        )))
    };
    ("sin-op") => {
        RaptorMessage::LoadfreeMSG(LoadfreeMessage::MockTensorMsg(MockTensor::new(
            MockOpCode::SinOp,
        )))
    };
    ("Matmul-op") => {
        RaptorMessage::LoadfreeMSG(LoadfreeMessage::MockTensorMsg(MockTensor::new(
            MockOpCode::MatmulOp,
        )))
    };
    ("Conv-op") => {
        RaptorMessage::LoadfreeMSG(LoadfreeMessage::MockTensorMsg(MockTensor::new(
            MockOpCode::ConvOp,
        )))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn build_system_using_macro_test0() {
        let system = build_mock_system!("raptor");
        assert_eq!(system.name(), "raptor");
    }

    #[tokio::test]
    async fn build_system_using_macro_test1() {
        let system = build_mock_system!("raptor", 4);
        assert_eq!(system.name(), "raptor");
        // TODO-FIX#1, currently not spawn at creation due to async-sync
        // assert_eq!(system.ranks(), 4);
    }

    #[test]
    fn build_halt_all_test() {
        let msg: LoadfreeMessage<MockTensor> = build_loadfree_msg!("halt-all");
        assert_eq!(msg, LoadfreeMessage::SystemMsg(SystemCommand::HaltAll));
    }

    #[test]
    fn build_spawn_msg_test() {
        let msg: LoadfreeMessage<MockTensor> = build_loadfree_msg!("spawn", "mock", 3);
        assert_eq!(msg, LoadfreeMessage::SystemMsg(SystemCommand::Spawn(0, 3)));
        let msg: LoadfreeMessage<MockTensor> = build_loadfree_msg!("spawn", "vulkan", 3);
        assert_eq!(msg, LoadfreeMessage::SystemMsg(SystemCommand::Spawn(1, 3)));
    }

    #[test]
    fn build_halt_msg_test() {
        let msg: LoadfreeMessage<MockTensor> = build_loadfree_msg!("halt", 3);
        assert_eq!(msg, LoadfreeMessage::SystemMsg(SystemCommand::HaltOn(3)));
    }

    #[test]
    fn build_add_op_test() {
        let msg: LoadfreeMessage<MockTensor> = build_loadfree_msg!("add-op");
        assert_eq!(
            msg,
            LoadfreeMessage::MockTensorMsg(MockTensor::new(MockOpCode::AddOp))
        );
    }

    #[test]
    fn build_exp_op_test() {
        let msg: LoadfreeMessage<MockTensor> = build_loadfree_msg!("exp-op");
        assert_eq!(
            msg,
            LoadfreeMessage::MockTensorMsg(MockTensor::new(MockOpCode::ExpOp))
        );
    }

    #[test]
    fn build_actor_available_msg_test() {
        let msg: LoadfreeMessage<MockTensor> = build_loadfree_msg!("available", 0);
        assert_eq!(
            msg,
            LoadfreeMessage::<MockTensor>::ActorMsg(ActorCommand::Available(0))
        );
    }
}
