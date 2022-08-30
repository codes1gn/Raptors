extern crate raptors;
extern crate uuid;

use log::{debug, info};

use std::{thread, time};

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
// TODO make [tokio::main] a integrated annotation of raptors
#[tokio::main]
async fn main() {
    let mut system = build_system!("Raptors");
    system.spawn_actors(6);
    assert_eq!(system.mails.len(), 6);
    let msg0 = build_msg!("test-zero");
    let msg1 = build_msg!("test-one");

    info!("{:#?}", msg0);
    info!("{:#?}", msg1);

    system.deliver_to(msg1.clone(), 0).await;
    system.deliver_to(msg0.clone(), 4).await;

    system.halt_actor(3);

    system.broadcast(msg1.clone()).await;
    system.broadcast(msg0.clone()).await;

    // info!("================ Running raptors::diamond-tasks example ================");
    // // STEP 1 system init
    // let mut syst = build_system!("mock system", 4);
    // assert_eq!(syst.name(), "mock system".to_string());

    // // STEP 3 build actors with cmds
    // // create 4 actors
    // let cmd = build_msg!("create-actors", 4, "raptor");
    // syst.on_receive(cmd);

    // // STEP 4 build workloads and dispatch
    // // create a list of workload
    // let mut workloads = build_workload!(vec![
    //     OpCode::AddOp,
    //     OpCode::SinOp,
    //     OpCode::ConvOp,
    //     OpCode::MatmulOp,
    //     OpCode::AddOp,
    //     OpCode::ExpOp,
    //     OpCode::ConvOp,
    //     OpCode::SinOp,
    //     OpCode::ConvOp,
    //     OpCode::MatmulOp,
    //     OpCode::MatmulOp,
    //     OpCode::AddOp,
    //     OpCode::ExpOp,
    //     OpCode::ConvOp,
    //     OpCode::SinOp,
    //     OpCode::ConvOp,
    //     OpCode::MatmulOp,
    //     OpCode::MatmulOp,
    //     OpCode::AddOp,
    // ]);

    // let envelopes = pre_schedule(&syst, workloads);
    // // syst.on_dispatch_workloads(workloads);
    // syst.on_dispatch_envelopes(envelopes);
    // // TODO(albert): pretty fmt debug display
    // debug!("{:#?}", syst.actor_registry().values());

    // // STEP 5 start all actors and perform
    // syst.start();

    // // STEP 6 destroy context and finish
    // // destroy all actors
    // // TODO we need msg builder
    // let cmd = build_msg!("destroy-all");
    // syst.on_receive(cmd);
}
