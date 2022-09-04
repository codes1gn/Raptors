extern crate raptors;
extern crate uuid;

use opentelemetry::global;
use tokio::io::Result;
use tracing::info;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
//
// use tracing::instrument;
// use tracing_subscriber::{registry::Registry, prelude::*};
// use tracing_chrome::ChromeLayerBuilder;
//
// use tracing::{info, span, Level};

use std::{thread, time};

use raptors::prelude::*;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    if std::env::args().any(|arg| arg == "--trace") {
        global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
        let tracer = opentelemetry_jaeger::new_pipeline()
            .with_service_name("raptors")
            .install_simple()
            .unwrap();

        let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
        tracing_subscriber::registry()
            .with(opentelemetry)
            .with(fmt::Layer::default())
            .try_init()
            .unwrap();
    } else {
        tracing_subscriber::fmt::try_init().unwrap();
    };


    info!("================ Running raptors::diamond-tasks example ================");
    let mut system = build_system!("Raptors");
    let cmd = build_msg!("spawn", 2);
    system.on_receive(cmd);
    assert_eq!(system.ranks(), 2);

    let msg0 = build_msg!("add-op");
    let msg1 = build_msg!("exp-op");
    let msg2 = build_msg!("sub-op");

    // deliver msg to first idle
    let idle_actor = system.poll_ready_actor();
    system.deliver_to(msg1.clone(), idle_actor).await;

    // deliver msg to first idle
    let idle_actor = system.poll_ready_actor();
    system.deliver_to(msg0.clone(), idle_actor).await;

    // TODO to fix current one-off actors in available queue
    // // deliver msg to first idle
    // let idle_actor = system.poll_ready_actor();
    // system.deliver_to(msg2.clone(), idle_actor).await;

    // // deliver msg to first idle
    // let idle_actor = system.poll_ready_actor();
    // system.deliver_to(msg2.clone(), idle_actor).await;


    // TODO not use
    // let halt_msg = build_msg!("halt", 3);
    // system.on_receive(halt_msg);
    // alt! system.halt_actor(3);
    // system.broadcast(msg1.clone()).await;
    // system.broadcast(msg0.clone()).await;
    // let halt_all = build_msg!("halt-all");
    // system.on_receive(halt_all);
    ()

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
    // info!("{:#?}", syst.actor_registry().values());

    // // STEP 5 start all actors and perform
    // syst.start();

    // // STEP 6 destroy context and finish
    // // destroy all actors
    // // TODO we need msg builder
    // let cmd = build_msg!("destroy-all");
    // syst.on_receive(cmd);
}
