extern crate raptors;
extern crate uuid;

use uuid::Uuid;

use raptors::prelude::*;

/// Routine of this example
///
/// new a system SystemBuilder
/// new a config
/// build a system
/// create 4 actors with msg
/// refactor vec to register on actors
/// dispatch workload to 4 actors
/// actors execute on receive
/// actors send back msg of results
/// destroy 4 actors with msg after all finished
///
fn main() {
    println!("================ Running raptors::diamond-tasks example ================");

    // STEP 1 create builders
    // create all builders
    let sys_builder = SystemBuilder::new();
    let msg_builder = CommandBuilder::new();
    let wld_builder = WorkloadBuilder::new();

    // STEP 2 system init
    // init system
    let mut sys_config = SystemConfig::new();
    sys_config.set_amount_of_actors(4 as usize);
    let mut syst = sys_builder.build_with_config("mock system", sys_config);
    assert_eq!(syst.name(), "mock system".to_string());

    // STEP 3 build actors with cmds
    // create 4 actors
    let msg = msg_builder.build(
        "create-actor",
        Some(vec![4]),
        Some(vec![String::from("raptor")]),
    );
    syst.on_receive(msg.into());

    // STEP 4 build workloads and dispatch
    // create a list of workload
    let mut workloads: Vec<TypedMessage> = wld_builder.build_many_msg(
        vec![1, 1, 1, 1, 1, 1],
        vec![
            OpCode::AddOp,
            OpCode::SinOp,
            OpCode::ConvOp,
            OpCode::MatmulOp,
            OpCode::AddOp,
            OpCode::ExpOp,
        ],
    );
    // send workload vector to system then system dispatch to actors
    syst.on_dispatch(workloads);

    // STEP 5 start all actors and perform

    // STEP 6 destroy context and finish
    // destroy all actors
    // TODO we need msg builder
    let msg = msg_builder.build("destroy-actor", None, None);
    syst.on_receive(msg.into());
}
