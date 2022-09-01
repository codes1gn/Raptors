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
    std::env::set_var("RUST_LOG", "info");
    // let subscriber = tracing_subscriber::FmtSubscriber::new();
    // tracing::subscriber::set_global_default(subscriber);
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
    //
    // to make more precise timestamps
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
    //         .include_locations(true)
    //         .build();
    //     tracing_subscriber::registry().with(chrome_layer).try_init();
    //     Some(guard)
    // };

    info!("================ Running raptors::diamond-tasks example ================");
    let mut system = build_system!("Raptors");
    // alt! system.spawn_actors(6);
    // need #[tokio::main]
    let cmd = build_msg!("spawn", 6);
    system.on_receive(cmd);
    assert_eq!(system.ranks(), 6);

    let msg0 = build_msg!("add-op");
    let msg1 = build_msg!("exp-op");

    info!("{:#?}", msg0);
    info!("{:#?}", msg1);

    system.deliver_to(msg1.clone(), 0).await;
    system.deliver_to(msg0.clone(), 4).await;

    let halt_msg = build_msg!("halt", 3);
    system.on_receive(halt_msg);
    // alt! system.halt_actor(3);

    system.broadcast(msg1.clone()).await;
    system.broadcast(msg0.clone()).await;

    let halt_all = build_msg!("halt-all");
    system.on_receive(halt_all);
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
