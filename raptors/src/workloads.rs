// LICENSE PLACEHOLDER
use std::any::Any;
use std::{thread, time};
use tracing::info;

use crate::cost_model::CostModel;
use crate::messages::TypedMessage;
use crate::prelude::*;

// dummy workload as dummy message but has a timeout for
// emulating the execution
//
// TODO(short-term): set up the cost-model, and make workload query
// payload capacity from it, by opcode; future should extend to polymorphic
// querying on both opcode and scale.
//
// TODO(long-term): extend this desing into typed messages
// 1. WorkloadMsg, contains bytecode modules
// 2. DataMsg, support data exchange
// 3. CommandMsg, operations that instruct the action of
// each actor
//
//
#[derive(Clone, Debug, PartialEq, Default, Eq)]
pub struct Workload {
    op: OpCode,
}

impl TensorTrait for Workload {}

impl Workload {
    pub fn new(op: OpCode) -> Workload {
        return Self { op: op };
    }

    pub fn payload(&self) -> usize {
        CostModel::new().estimate(self)
    }

    pub fn op(&self) -> OpCode {
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

impl Into<TypedMessage<Workload>> for Workload {
    fn into(self) -> TypedMessage<Workload> {
        TypedMessage::<Workload>::WorkloadMsg(self)
    }
}

/// WorkloadMsg indicates the workload to be taken.
#[derive(Clone, Debug, PartialEq)]
pub struct WorkloadMsg {
    workload: Workload,
}

impl WorkloadMsg {
    pub fn new(workload: Workload) -> Self {
        return Self { workload: workload };
    }
}

// Definition for Opcode
/// ```
/// # // Test default function for OpCode
/// use raptors::prelude::*;
///
/// assert_eq!(OpCode::default(), OpCode::IdentityOp);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
// Copy trait is necessary, otherwise ownership will transit into the cost model
pub enum OpCode {
    IdentityOp,
    AddOp,
    ConvOp,
    ExpOp,
    MatmulOp,
    SinOp,
    SubOp,
}

impl Default for OpCode {
    fn default() -> Self {
        OpCode::IdentityOp
    }
}
// TODO: More Ops to add; Other way to implement Opcode

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_dummy_workload_test() {
        let load = Workload::new(OpCode::AddOp);
        assert_eq!(load.payload(), 11 as usize);
        assert_eq!(load.op(), OpCode::AddOp);
    }

    #[test]
    fn worklaod_mock_run_test() {
        let load = Workload::new(OpCode::ConvOp);
        let now = time::Instant::now();
        load.mock_run();
        assert_eq!(load.op(), OpCode::ConvOp);
    }

    #[test]
    fn workload_ops_default_test() {
        let load = Workload::new(OpCode::default());
        assert_eq!(load.op(), OpCode::IdentityOp);
    }

    #[test]
    fn workload_ops_matmul_test() {
        let load = Workload::new(OpCode::MatmulOp);
        assert_eq!(load.op(), OpCode::MatmulOp);
    }

    #[test]
    fn workload_ops_exp_test() {
        let load = Workload::new(OpCode::ExpOp);
        assert_eq!(load.op(), OpCode::ExpOp);
    }

    #[test]
    fn workload_message_test() {
        let load = Workload::new(OpCode::ExpOp);
        let wlmsg = TypedMessage::<Workload>::WorkloadMsg(load);
        assert_eq!(
            wlmsg,
            TypedMessage::<Workload>::WorkloadMsg(Workload::new(OpCode::ExpOp))
        );
    }
}
