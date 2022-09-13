// LICENSE PLACEHOLDER
//
use crate::workloads::{OpCode, Workload};

pub trait TensorTrait {}

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
pub trait ExecutorTrait {
    type TensorLike;
    fn new() -> Self;
    fn compute_it(&self, wkl: Self::TensorLike) -> Self::TensorLike;
    fn compute_wkl(&self, workload: Self::TensorLike) -> Self::TensorLike;
}

// impl Executor {
//     pub fn new() -> Self {
//         return Self {};
//     }
// }

impl ExecutorTrait for Executor {
    type TensorLike = Workload;
    fn new() -> Executor {
        Self {}
    }
    fn compute_it(&self, wkl: Self::TensorLike) -> Self::TensorLike {
        wkl.mock_run();
        wkl
    }
    fn compute_wkl(&self, workload: Self::TensorLike) -> Self::TensorLike {
        // workload.mock_run();
        workload
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
        exec.compute_it(load);
        assert!(now.elapsed() >= time::Duration::from_millis(11));
    }
}
