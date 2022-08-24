extern crate raptors;
extern crate uuid;

use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use uuid::Uuid;

use raptors::prelude::*;

// Util function that randomly allocates an executor to each workloads
// Take vector of workloads, produce vector of envelope
fn fixed_executor_allocation(system: &System, workloads: Vec<TypedMessage>) -> Vec<Envelope> {
    let mut actor_ids = Vec::from_iter(system.actor_registry().keys());
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..system.actor_registry().keys().len());
    workloads
        .into_iter()
        .map(|wkl| -> Envelope {
            let index = die.sample(&mut rng);
            println!("index = {:?}", index);
            Envelope {
                msg: wkl,
                receiver: Address::new(*actor_ids[index]),
            }
        })
        .collect::<Vec<Envelope>>()
}
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
    let cmd_builder = CommandBuilder::new();
    let wld_builder = WorkloadBuilder::new();

    // STEP 2 system init
    // init system
    let mut sys_config = SystemConfig::new();
    sys_config.set_amount_of_actors(4 as usize);
    let mut syst = sys_builder.build_with_config("mock system", sys_config);
    assert_eq!(syst.name(), "mock system".to_string());

    // STEP 3 build actors with cmds
    // create 4 actors
    let cmd = cmd_builder.build(
        "create-actor",
        Some(vec![4]),
        Some(vec![String::from("raptor")]),
    );
    syst.on_receive(cmd);

    // STEP 4 build workloads and dispatch
    // create a list of workload
    let mut workloads: Vec<TypedMessage> = wld_builder.build(vec![
        OpCode::AddOp,
        OpCode::SinOp,
        OpCode::ConvOp,
        OpCode::MatmulOp,
        OpCode::AddOp,
        OpCode::ExpOp,
        OpCode::ConvOp,
        OpCode::SinOp,
        OpCode::ConvOp,
        OpCode::MatmulOp,
        OpCode::MatmulOp,
        OpCode::AddOp,
        OpCode::ExpOp,
        OpCode::ConvOp,
        OpCode::SinOp,
        OpCode::ConvOp,
        OpCode::MatmulOp,
        OpCode::MatmulOp,
        OpCode::AddOp,
    ]);
    let envelopes: Vec<Envelope> = fixed_executor_allocation(&syst, workloads);
    // println!("{:?}", envelopes);
    // syst.on_dispatch_workloads(workloads);
    syst.on_dispatch_envelopes(envelopes);
    // TODO(albert): pretty fmt debug display
    println!("{:?}", syst.actor_registry().values());

    // STEP 5 start all actors and perform

    // STEP 6 destroy context and finish
    // destroy all actors
    // TODO we need msg builder
    let cmd = cmd_builder.build("destroy-actor", None, None);
    syst.on_receive(cmd);
}
