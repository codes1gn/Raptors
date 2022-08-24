// LICENSE PLACEHOLDER

use crate::messages::TypedMessage;
use crate::workloads::{OpCode, Workload, WorkloadMsg};

/// Workload Builder helps create the workload.
#[derive(Default)]
pub struct WorkloadBuilder;

impl WorkloadBuilder {
    // Definer fns
    pub fn new() -> Self {
        WorkloadBuilder::default()
    }

    pub fn build_workloads(&self, ops: Vec<OpCode>) -> Vec<Workload> {
        ops.into_iter()
            .map(|x| Workload::new(x))
            .collect::<Vec<Workload>>()
    }

    pub fn build(&self, op: Vec<OpCode>) -> Vec<TypedMessage> {
        self.build_workloads(op)
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<TypedMessage>>()
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workload_build_workloads_test() {
        let builder = WorkloadBuilder::new();
        let workloads = builder.build_workloads(vec![OpCode::DummyOp, OpCode::DummyOp]);
        assert_eq!(workloads[0], Workload::new(OpCode::DummyOp));
        assert_eq!(
            workloads,
            vec![
                Workload::new(OpCode::DummyOp),
                Workload::new(OpCode::DummyOp)
            ]
        );
    }

    fn build_messages_test() {
        let builder = WorkloadBuilder::new();
        let messages = builder.build(vec![OpCode::DummyOp, OpCode::DummyOp]);
        assert_eq!(messages[0], Workload::new(OpCode::DummyOp).into());
        assert_eq!(
            messages,
            vec![
                Workload::new(OpCode::DummyOp).into(),
                Workload::new(OpCode::DummyOp).into(),
            ]
        );
    }
}
