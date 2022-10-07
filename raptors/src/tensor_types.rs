// LICENSE PLACEHOLDER
use std::{thread, time};
use tracing::info;

use crate::cost_model::CostModel;
use crate::messages::LoadfreeMessage;
use crate::prelude::*;

// defining TensorLike trait
// TODO(albert), change naming at last, see how these abstractions evolves
// tensortype -> payload
// tensorlike -> computable
// compute(a: Computable) -> ??
pub trait TensorLike {}

// dummy workload as dummy message but has a timeout for
// emulating the execution
//
// TODO(short-term): set up the cost-model, and make workload query
// payload capacity from it, by opcode; future should extend to polymorphic
// querying on both opcode and scale.
//
// TODO(long-term): extend this desing into typed messages
// 1. MockTensorMsg, contains bytecode modules
// 2. DataMsg, support data exchange
// 3. CommandMsg, operations that instruct the action of
// each actor
//
//
#[derive(Clone, Debug, PartialEq, Default, Eq)]
pub struct MockTensor {
    op: MockOpCode,
}

impl TensorLike for MockTensor {}

impl MockTensor {
    pub fn new(op: MockOpCode) -> MockTensor {
        return Self { op: op };
    }

    pub fn payload(&self) -> usize {
        CostModel::new().estimate(self)
    }

    pub fn op(&self) -> MockOpCode {
        self.op.clone()
    }

    // mock function that will fakely run for that period long
    // TODO:
    // 1. change signiture to return values
    // 2. values may use a value type that defined include possible results
    #[tracing::instrument(name = "workload::mock_run", skip(self))]
    pub fn mock_run(&self) -> () {
        info!("WKL - compute");
        thread::sleep(time::Duration::from_millis((self.payload() * 10) as u64));
    }
}

impl Into<LoadfreeMessage<MockTensor>> for MockTensor {
    fn into(self) -> LoadfreeMessage<MockTensor> {
        LoadfreeMessage::<MockTensor>::MockTensorMsg(self)
    }
}

/// MockTensorMsg indicates the workload to be taken.
#[derive(Clone, Debug, PartialEq)]
pub struct MockTensorMsg {
    workload: MockTensor,
}

impl MockTensorMsg {
    pub fn new(workload: MockTensor) -> Self {
        return Self { workload: workload };
    }
}

// TODO: More Ops to add; Other way to implement Opcode

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_dummy_workload_test() {
        let load = MockTensor::new(MockOpCode::AddOp);
        assert_eq!(load.payload(), 11 as usize);
        assert_eq!(load.op(), MockOpCode::AddOp);
    }

    #[test]
    fn worklaod_mock_run_test() {
        let load = MockTensor::new(MockOpCode::ConvOp);
        let now = time::Instant::now();
        load.mock_run();
        assert_eq!(load.op(), MockOpCode::ConvOp);
    }

    #[test]
    fn workload_ops_default_test() {
        let load = MockTensor::new(MockOpCode::default());
        assert_eq!(load.op(), MockOpCode::IdentityOp);
    }

    #[test]
    fn workload_ops_matmul_test() {
        let load = MockTensor::new(MockOpCode::MatmulOp);
        assert_eq!(load.op(), MockOpCode::MatmulOp);
    }

    #[test]
    fn workload_ops_exp_test() {
        let load = MockTensor::new(MockOpCode::ExpOp);
        assert_eq!(load.op(), MockOpCode::ExpOp);
    }

    #[test]
    fn workload_message_test() {
        let load = MockTensor::new(MockOpCode::ExpOp);
        let wlmsg = LoadfreeMessage::<MockTensor>::MockTensorMsg(load);
        assert_eq!(
            wlmsg,
            LoadfreeMessage::<MockTensor>::MockTensorMsg(MockTensor::new(MockOpCode::ExpOp))
        );
    }
}
