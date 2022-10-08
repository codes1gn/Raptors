// LICENSE PLACEHOLDER
//
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
    fn unary_compute(&mut self, op: Self::OpCodeType, arg: Self::TensorType) -> Self::TensorType;
    // TODO need to support monomorphism for SupportedDataType
    fn binary_compute(
        &mut self,
        op: Self::OpCodeType,
        lhs: Self::TensorType,
        rhs: Self::TensorType,
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
    pub fn mock_unary(&mut self, arg: MockTensor) -> MockTensor {
        arg
    }

    // WIP pub fn mock_binary(&mut self, lhs: MockTensor, rhs: MockTensor) -> MockTensor {
    // WIP     lhs
    // WIP }

    pub fn mock_binary<T: TensorLike>(&mut self, lhs: T, rhs: T) -> T {
        lhs
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
        self.mock_unary(arg)
    }

    fn unary_compute(&mut self, op: Self::OpCodeType, arg: Self::TensorType) -> Self::TensorType {
        self.mock_unary(arg)
    }

    fn binary_compute(
        &mut self,
        op: Self::OpCodeType,
        lhs: Self::TensorType,
        rhs: Self::TensorType,
    ) -> Self::TensorType {
        self.mock_binary::<Self::TensorType>(lhs, rhs)
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
