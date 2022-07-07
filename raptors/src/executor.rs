// LICENSE PLACEHOLDER
//
use crate::actors;
use crate::messages;

// wrap a dedicated executor module that only consider how to do computations
//
// TODO(long-term):
// as a interface, make refactor as Trait and expose to CRT level,
// make CRT vm to impl this trait
pub struct Executor {}

impl Executor {
    pub fn new() -> Self {
        return Self {};
    }
    pub fn compute(&self, workload: messages::Workload) -> () {
        workload.mock_run();
    }
}

// unit tests
#[cfg(test)]

mod tests {
    use super::*;
    use std::{thread, time};

    #[test]
    fn compute_workload() {
        let exec = Executor::new();
        let load = messages::Workload::new(16, messages::OpCode::AddOp);
        let now = time::Instant::now();
        exec.compute(load);
        assert!(now.elapsed() >= time::Duration::from_millis(16));
    }
}
