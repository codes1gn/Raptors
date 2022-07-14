// LICENSE PLACEHOLDER

use crate::messages::{OpCode, Workload};

/// Workload Builder helps create the workload.
/// ```
/// use crate::messages::*;
/// 
/// let builder = WorkloadBuilder::new(1, OpCode::DummyOp);
/// 
/// assert_eq!(builder::payload(), 1);
/// assert_eq!(builder::op(), OpCode::DummyOp);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct WorkloadBuilder {
    payload: usize,
    op: OpCode,
}

impl WorkloadBuilder {
    pub fn new(payload: usize, op: OpCode) -> Self {
        return Self {
            payload: payload,
            op: op,
        };
    }

    pub fn payload(&self) -> usize {
        self.payload.clone()
    }

    pub fn op(&self) -> OpCode {
        self.op.clone()
    }

    pub fn set_payload(&mut self, payload: usize) {
        self.payload = payload;
    }
 
    pub fn set_op(&mut self, op: OpCode) {
        self.op = op;
    }

    pub fn build_workload(&self) -> Workload {
        return Workload ( self::payload(), self::op() );
    }
}

impl Default for WorkloadBuilder {
    fn default() -> Self {
        return Self {
            payload: 0,
            op: OpCode::DummyOp,
        };
    }
}
// unit tests
#[cfg(test)]
mod WorkloadBuilder_tests {
    use super::*;

    #[test]
    fn create_builder_test() {
        let builder = WorkloadBuilder::new(1, OpCode::DummyOp);

        assert_eq!(builder.payload, 1);
        assert_eq!(builder.op, OpCode::DummyOp);
    }

    #[test]
    fn set_payload_test() {
        let mut builder = WorkloadBuilder::new(1, OpCode::DummyOp);
        builder::set_payload(20);

        assert_eq!(builder::payload(), 20);
    }

    #[test]
    fn set_op_test() {
        let mut builder = WorkloadBuilder::new(1, OpCode::DummyOp);
        builder::set_op(OpCode::AddOp);
        
        assert_eq!(builder::payload(), OpCode::AddOp);
    }
    
    #[test]
    fn build_workload_test() {
        let mut builder = WorkloadBuilder::new(1, OpCode::DummyOp);
        let workload_built = builder::build_workload();
        
        assert_eq!(
            workload_built,
            Workload ( builder::payload(), builder::op() )
        );
    }
}
