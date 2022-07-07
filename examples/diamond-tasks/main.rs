extern crate raptors;
use raptors::prelude::*;

fn main() {
    let sys_builder = SystemBuilder::new();
    let sys_config = SystemConfig::new();
    let syst = sys_builder.build_with_config("mock system", sys_config);
    assert_eq!(syst.name(), "mock system".to_string());
}
