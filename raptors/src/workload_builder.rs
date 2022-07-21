// LICENSE PLACEHOLDER

use crate::messages::{OpCode, Workload, WorkloadMsg};

/// Workload Builder helps create the workload.
#[derive(Default)]
pub struct WorkloadBuilder;

impl WorkloadBuilder {
    // Definer fns
    pub fn new() -> Self {
        WorkloadBuilder::default()
    }

    pub fn build(&self, payload: Option<usize>, op: Option<OpCode>) -> Workload {
        return Workload::new(
            payload.expect("No valid payload value"),
            op.expect("No valid operation"),
        );
    }

    pub fn build_msg(&self, payload: Option<usize>, op: Option<OpCode>) -> WorkloadMsg {
        return WorkloadMsg::new(self.build(payload, op));
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use crate::messages::SystemMsg;

    use super::*;

    #[test]
    fn workload_build_test() {
        let builder = WorkloadBuilder::new();
        let workload = builder.build(Some(2), Some(OpCode::DummyOp));
        assert_eq!(workload, Workload::new(2, OpCode::DummyOp));
    }

    #[test]
    fn msg_build_test() {
        let builder = WorkloadBuilder::new();
        let msg = builder.build_msg(Some(2), Some(OpCode::AddOp));
        let workload = builder.build(Some(2), Some(OpCode::AddOp));
        assert_eq!(msg, WorkloadMsg::new(workload));
    }
}
