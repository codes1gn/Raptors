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

    // TODO(albert) op can take vec or just op
    pub fn build(&self, payload: Option<usize>, op: Option<OpCode>) -> Workload {
        return Workload::new(
            payload.expect("No valid payload value"),
            op.expect("No valid operation"),
        );
    }

    pub fn build_many(&self, payload_list: Vec<usize>, op_list: Vec<OpCode>) -> Vec<Workload> {
        op_list
            .into_iter()
            .map(|x| Workload::new(1, x))
            .collect::<Vec<Workload>>()
    }

    pub fn build_many_msg(&self, payload: Vec<usize>, op: Vec<OpCode>) -> Vec<TypedMessage> {
        self.build_many(payload, op)
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<TypedMessage>>()
    }

    pub fn build_msg(&self, payload: Option<usize>, op: Option<OpCode>) -> TypedMessage {
        self.build(payload, op).into()
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
    fn workload_build_many_test() {
        let builder = WorkloadBuilder::new();
        let workloads = builder.build_many(vec![1, 1], vec![OpCode::DummyOp, OpCode::DummyOp]);
        assert_eq!(workloads[0], Workload::new(1, OpCode::DummyOp));
        assert_eq!(
            workloads,
            vec![
                Workload::new(1, OpCode::DummyOp),
                Workload::new(1, OpCode::DummyOp)
            ]
        );
    }

    fn msg_build_many_test() {
        let builder = WorkloadBuilder::new();
        let messages = builder.build_many_msg(vec![1, 1], vec![OpCode::DummyOp, OpCode::DummyOp]);
        assert_eq!(messages[0], Workload::new(1, OpCode::DummyOp).into());
        assert_eq!(
            messages,
            vec![
                Workload::new(1, OpCode::DummyOp).into(),
                Workload::new(1, OpCode::DummyOp).into(),
            ]
        );
    }

    #[test]
    fn msg_build_test() {
        let builder = WorkloadBuilder::new();
        let msg = builder.build_msg(Some(2), Some(OpCode::AddOp));
        let workload = builder.build(Some(2), Some(OpCode::AddOp));
        assert_eq!(msg, TypedMessage::WorkloadMsg(workload));
    }
}
