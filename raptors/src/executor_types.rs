// LICENSE PLACEHOLDER
//
use std::sync::Arc;
use std::{thread, time};

use crate::cost_model::MockOpCode;
use crate::tensor_types::{MockTensor, TensorLike};

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
    fn new_with_typeid(typeid: usize) -> Self;
    fn init(&mut self) -> ();
    fn mock_compute(&mut self, arg: Self::TensorType) -> Self::TensorType;
    fn unary_compute(
        &mut self,
        op: Self::OpCodeType,
        arg: Arc<Self::TensorType>,
    ) -> Self::TensorType;
    // TODO need to support monomorphism for SupportedDataType
    fn binary_compute(
        &mut self,
        op: Self::OpCodeType,
        lhs: Arc<Self::TensorType>,
        rhs: Arc<Self::TensorType>,
    ) -> Self::TensorType;
}

// wrap a dedicated executor module that only consider how to do computations
//
// TODO(long-term):
// as a interface, make refactor as Trait and expose to CRT level,
// make CRT vm to impl this trait
#[derive(Debug)]
pub struct MockExecutor {}

impl MockExecutor {
    pub fn new() -> Self {
        return Self {};
    }

    // TODO handle op
    pub fn mock_unary<T: TensorLike + Clone>(&mut self, op: MockOpCode, arg: Arc<T>) -> T {
        thread::sleep(time::Duration::from_millis((1000) as u64));
        let _y: T = (*arg).clone();
        _y
    }

    pub fn mock_binary<T: TensorLike + Clone>(
        &mut self,
        op: MockOpCode,
        lhs: Arc<T>,
        rhs: Arc<T>,
    ) -> T {
        thread::sleep(time::Duration::from_millis((2000) as u64));
        let _y: T = (*lhs).clone();
        _y
    }
}

impl ExecutorLike for MockExecutor {
    type OpCodeType = MockOpCode;
    type TensorType = MockTensor;
    fn new_with_typeid(typeid: usize) -> MockExecutor {
        Self::new()
    }

    fn init(&mut self) -> () {}

    fn mock_compute(&mut self, arg: Self::TensorType) -> Self::TensorType {
        arg
    }

    fn unary_compute(
        &mut self,
        op: Self::OpCodeType,
        arg: Arc<Self::TensorType>,
    ) -> Self::TensorType {
        self.mock_unary::<Self::TensorType>(op, arg)
    }

    fn binary_compute(
        &mut self,
        op: Self::OpCodeType,
        lhs: Arc<Self::TensorType>,
        rhs: Arc<Self::TensorType>,
    ) -> Self::TensorType {
        self.mock_binary::<Self::TensorType>(op, lhs, rhs)
    }
}

// unit tests
#[cfg(test)]

mod tests {
    use super::*;
    use std::time;

    #[test]
    fn mock_exector_dummy_test() {
        assert_eq!(0, 0);
    }
}
