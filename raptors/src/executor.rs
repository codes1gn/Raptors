// LICENSE PLACEHOLDER
//
use crate::cost_model::OpCode;
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
    fn new() -> Self;
    fn compute(&self, wkl: Self::TensorType) -> Self::TensorType;
}

// impl Executor {
//     pub fn new() -> Self {
//         return Self {};
//     }
// }

impl ExecutorLike for Executor {
    type TensorType = Workload;
    fn new() -> Executor {
        Self {}
    }
    fn compute(&self, wkl: Self::TensorType) -> Self::TensorType {
        wkl.mock_run();
        wkl
    }
}

// unit tests
#[cfg(test)]

mod tests {
    use super::*;
    use std::time;

    #[test]
    fn compute_workload() {
        let exec = Executor::new();
        let load = Workload::new(OpCode::AddOp);
        let now = time::Instant::now();
        exec.compute(load);
        assert!(now.elapsed() >= time::Duration::from_millis(11));
    }
}
