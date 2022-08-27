extern crate raptors;
extern crate uuid;

use log::{debug, info};
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
            debug!("receiver's actor index = {:#?}", index);
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
    env_logger::init();
    info!("================ Running raptors::diamond-tasks example ================");

    // STEP 1 create builders
    // create all builders
    let wld_builder = WorkloadBuilder::new();

    // STEP 2 system init
    let mut syst = build_system!("mock system", 4);
    assert_eq!(syst.name(), "mock system".to_string());

    // STEP 3 build actors with cmds
    // create 4 actors
    let cmd = build_msg!("create-actors", 4, "raptor");
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
    // syst.on_dispatch_workloads(workloads);
    syst.on_dispatch_envelopes(envelopes);
    // TODO(albert): pretty fmt debug display
    debug!("{:#?}", syst.actor_registry().values());

    // STEP 5 start all actors and perform
    let cmd = build_msg!("start");
    syst.on_receive(cmd);

    // STEP 6 destroy context and finish
    // destroy all actors
    // TODO we need msg builder
    let cmd = build_msg!("destroy-all");
    syst.on_receive(cmd);
}
