extern crate raptors;
use raptors::prelude::*;

fn main() {
    println!("================ Running raptors::diamond-tasks example ================");
    let sys_builder = SystemBuilder::new();
    let mut sys_config = SystemConfig::new();
    sys_config.set_amount_of_actors(4 as usize);

    let mut syst = sys_builder.build_with_config("mock system", sys_config);
    assert_eq!(syst.name(), "mock system".to_string());

    let msg = SystemCommand::CreateActor(4, String::from("raptor"));
    syst.on_receive(msg.into());

    // check actor creation
    let query_actors = syst.actors().unwrap();
    assert_eq!(query_actors.len(), 4);
    assert_eq!(query_actors[0].name(), "raptor #0".to_string());
    assert_eq!(query_actors[1].name(), "raptor #1".to_string());
    assert_eq!(query_actors[2].name(), "raptor #2".to_string());
    assert_eq!(query_actors[3].name(), "raptor #3".to_string());
}
