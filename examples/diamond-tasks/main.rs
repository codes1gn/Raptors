extern crate raptors;
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
    let query_actors = syst.actors().expect("None of actors in system");
    assert_eq!(query_actors.len(), 4);
    assert_eq!(query_actors[0].name(), "raptor #0".to_string());
    assert_eq!(query_actors[1].name(), "raptor #1".to_string());
    assert_eq!(query_actors[2].name(), "raptor #2".to_string());
    assert_eq!(query_actors[3].name(), "raptor #3".to_string());

    // create a list of workload
    // TODO we need workload builder later
    let mut workloads: Vec<TypedMessage> = vec![];
    
    workloads.push(TypedMessage::WorkloadMsg(Workload::new(16, OpCode::AddOp)));
    workloads.push(TypedMessage::WorkloadMsg(Workload::new(16, OpCode::SinOp)));
    workloads.push(TypedMessage::WorkloadMsg(Workload::new(16, OpCode::ConvOp)));
    workloads.push(TypedMessage::WorkloadMsg(Workload::new(16, OpCode::MatmulOp)));
    workloads.push(TypedMessage::WorkloadMsg(Workload::new(16, OpCode::AddOp)));
    workloads.push(TypedMessage::WorkloadMsg(Workload::new(16, OpCode::ExpOp)));
    println!("{:?}", workloads);

    // send workload vector to system then system pass to scheduler

    // destroy all actors
    // TODO we need msg builder
    let msg = SystemCommand::DestroyAllActors;
    syst.on_receive(msg.into());
    let query_actors = syst.actors().expect("None of actors in system");
    assert_eq!(query_actors.len(), 0);
}
