// LICENSE PLACEHOLDER
//
use crate::cost_model::MockOpCode;
use crate::tensor_types::Workload;

// wrap a dedicated executor module that only consider how to do computations
//
// TODO(long-term):
// as a interface, make refactor as Trait and expose to CRT level,
// make CRT vm to impl this trait
#[derive(Debug)]
pub struct Executor {}

// desugarized trait bounds in trait
// trait-name {
//     fn func(&self) -> impl TraitB;
// }
// trait-name {
//     type AssociateTypeA: TraitB;
//     fn func(&self) -> AssociateTypeA;
// }
pub trait ExecutorLike {
    type TensorType;
    type OpCodeType;
    fn new() -> Self;
    fn init(&mut self) -> ();
    fn compute_mock(&mut self, arg: Self::TensorType) -> Self::TensorType;
    fn compute_unary(&mut self, op: Self::OpCodeType, arg: Self::TensorType) -> Self::TensorType;
    fn compute_binary(
        &mut self,
        op: Self::OpCodeType,
        lhs: Self::TensorType,
        rhs: Self::TensorType,
    ) -> Self::TensorType;
}

// impl Executor {
//     pub fn new() -> Self {
//         return Self {};
//     }
// }

impl ExecutorLike for Executor {
    type OpCodeType = MockOpCode;
    type TensorType = Workload;
    fn new() -> Executor {
        Self {}
    }

    fn init(&mut self) -> () {}

    fn compute_mock(&mut self, arg: Self::TensorType) -> Self::TensorType {
        arg.mock_run();
        arg
    }

    fn compute_unary(&mut self, op: Self::OpCodeType, arg: Self::TensorType) -> Self::TensorType {
        arg.mock_run();
        arg
    }

    fn compute_binary(
        &mut self,
        op: Self::OpCodeType,
        lhs: Self::TensorType,
        rhs: Self::TensorType,
    ) -> Self::TensorType {
        lhs.mock_run();
        lhs
    }
}

// unit tests
#[cfg(test)]

mod tests {
    use super::*;
    use std::time;

    #[test]
    fn compute_workload() {
        let mut exec = Executor::new();
        let load = Workload::new(MockOpCode::AddOp);
        let now = time::Instant::now();
        exec.compute_mock(load);
        assert!(now.elapsed() >= time::Duration::from_millis(11));
    }
}
