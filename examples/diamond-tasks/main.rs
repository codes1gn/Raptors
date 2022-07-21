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

    // init system
    let sys_builder = SystemBuilder::new();
    let mut sys_config = SystemConfig::new();
    sys_config.set_amount_of_actors(4 as usize);
    let mut syst = sys_builder.build_with_config("mock system", sys_config);
    assert_eq!(syst.name(), "mock system".to_string());

    // create 4 actors
    let msg = SystemCommand::CreateActor(4, String::from("raptor"));
    syst.on_receive(msg.into());
    let actor_reg = syst.actor_registry();
    assert_eq!(actor_reg.len(), 4);
    // println!("{:?}", actor_reg);

    // create a list of workload
    // TODO we need workload builder later
    let mut workloads: Vec<TypedMessage> = vec![];

    workloads.push(TypedMessage::WorkloadMsg(Workload::new(16, OpCode::AddOp)));
    workloads.push(TypedMessage::WorkloadMsg(Workload::new(16, OpCode::SinOp)));
    workloads.push(TypedMessage::WorkloadMsg(Workload::new(16, OpCode::ConvOp)));
    workloads.push(TypedMessage::WorkloadMsg(Workload::new(
        16,
        OpCode::MatmulOp,
    )));
    workloads.push(TypedMessage::WorkloadMsg(Workload::new(16, OpCode::AddOp)));
    workloads.push(TypedMessage::WorkloadMsg(Workload::new(16, OpCode::ExpOp)));
    // println!("{:?}", workloads);

    // send workload vector to system then system dispatch to actors
    syst.on_dispatch(workloads);

    let actor_reg = syst.actor_registry();
    assert_eq!(actor_reg.len(), 4);
    let actors: Vec<&Actor> = actor_reg.values().collect();
    // println!("{:?}", actors[0].mailbox);

    // destroy all actors
    // TODO we need msg builder
    let msg = SystemCommand::DestroyAllActors;
    syst.on_receive(msg.into());
    let actor_reg = syst.actor_registry();
    assert_eq!(actor_reg.len(), 0);
}
