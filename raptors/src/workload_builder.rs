// LICENSE PLACEHOLDER

use crate::messages::{OpCode, Workload, WorkloadMsg};

/// Workload Builder helps create the workload.
#[derive(Default)]
pub struct WorkloadBuilder {
    payload: usize,
    op: OpCode,
}

impl WorkloadBuilder {
    // Definer fns
    pub fn new() -> Self {
        WorkloadBuilder::default()
    }

    pub fn build_with_workload(payload: usize, op: OpCode) -> Self {
        return Self {
            payload: payload,
            op: op,
        };
    }

    // Getter fns
    pub fn payload(&self) -> usize {
        self.payload.clone()
    }

    pub fn op(&self) -> OpCode {
        self.op.clone()
    }

    // Setter fns
    pub fn set_payload(&mut self, payload: usize) {
        self.payload = payload;
    }

    pub fn set_op(&mut self, op: OpCode) {
        self.op = op;
    }

    pub fn set_workload(&mut self, workload: Workload) {
        self.payload = workload.payload();
        self.op = workload.op();
    }

    // Builder fns
    pub fn build_workload(&self) -> Workload {
        return Workload::new(self.payload(), self.op());
    }

    pub fn build_msg(&self) -> WorkloadMsg {
        return WorkloadMsg::new(self.build_workload());
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_builder_test() {
        let builder = WorkloadBuilder::new();
        assert_eq!(builder.payload, 0);
        assert_eq!(builder.op, OpCode::DummyOp);

        let builder = WorkloadBuilder::build_with_workload(1, OpCode::DummyOp);
        assert_eq!(builder.payload, 1);
        assert_eq!(builder.op, OpCode::DummyOp);
    }

    #[test]
    fn set_payload_test() {
        let mut builder = WorkloadBuilder::new();
        builder.set_payload(20);

        assert_eq!(builder.payload(), 20);
    }

    #[test]
    fn set_op_test() {
        let mut builder = WorkloadBuilder::new();
        builder.set_op(OpCode::AddOp);

        assert_eq!(builder.op(), OpCode::AddOp);
    }

    #[test]
    fn set_workload_test() {
        let mut builder = WorkloadBuilder::new();
        builder.set_workload(Workload::new(0, OpCode::AddOp));

        assert_eq!(builder.build_workload(), Workload::new(0, OpCode::AddOp));
    }

    #[test]
    fn build_workload_test() {
        let mut builder = WorkloadBuilder::build_with_workload(1, OpCode::DummyOp);
        let workload_built = builder.build_workload();

        assert_eq!(workload_built, Workload::new(1, OpCode::DummyOp));
    }

    #[test]
    fn build_msg_test() {
        let mut builder = WorkloadBuilder::build_with_workload(1, OpCode::DummyOp);
        let msg_built = builder.build_msg();

        assert_eq!(
            msg_built,
            WorkloadMsg::new(Workload::new(1, OpCode::DummyOp))
        );
    }
}
