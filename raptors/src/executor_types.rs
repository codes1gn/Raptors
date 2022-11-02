// LICENSE PLACEHOLDER
//
use std::sync::{Arc, RwLock};
use std::{thread, time};
use tracing::{debug, info};

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
        arg: Arc<RwLock<Self::TensorType>>,
    ) -> Self::TensorType;
    fn dma_operation(
        &mut self,
        op: Self::OpCodeType,
        arg: Arc<RwLock<Self::TensorType>>,
        out: Arc<RwLock<Self::TensorType>>,
        shape: Vec<usize>,
    ) -> ();
    fn unary_compute_v2(
        &mut self,
        op: Self::OpCodeType,
        arg: Arc<RwLock<Self::TensorType>>,
        out: Arc<RwLock<Self::TensorType>>,
    ) -> ();
    // TODO need to support monomorphism for SupportedDataType
    fn binary_compute(
        &mut self,
        op: Self::OpCodeType,
        lhs: Arc<RwLock<Self::TensorType>>,
        rhs: Arc<RwLock<Self::TensorType>>,
    ) -> Self::TensorType;
    fn binary_compute_v2(
        &mut self,
        op: Self::OpCodeType,
        lhs: Arc<RwLock<Self::TensorType>>,
        rhs: Arc<RwLock<Self::TensorType>>,
        out: Arc<RwLock<Self::TensorType>>,
    ) -> ();
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
    pub fn mock_unary<T: TensorLike + Clone>(&mut self, op: MockOpCode, arg: Arc<RwLock<T>>) -> T {
        thread::sleep(time::Duration::from_millis((1000) as u64));
        let _y: T = (*arg).read().unwrap().clone();
        _y
    }

    pub fn mock_binary<T: TensorLike + Clone>(
        &mut self,
        op: MockOpCode,
        lhs: Arc<RwLock<T>>,
        rhs: Arc<RwLock<T>>,
    ) -> T {
        thread::sleep(time::Duration::from_millis((1000) as u64));
        let _y: T = (*lhs).read().unwrap().clone();
        _y
    }

    // ANCHOR
    pub fn dma_operation<T: TensorLike + Clone>(
        &mut self,
        op: MockOpCode,
        arg: Arc<RwLock<T>>,
        ret: Arc<RwLock<T>>,
        shape: Vec<usize>,
    ) -> () {
        info!("anchor - dma - {:?}", op);
        thread::sleep(time::Duration::from_millis((1000) as u64));
    }

    // ANCHOR
    pub fn mock_unary_v2<T: TensorLike + Clone>(
        &mut self,
        op: MockOpCode,
        arg: Arc<RwLock<T>>,
        ret: Arc<RwLock<T>>,
    ) -> () {
        info!("anchor - unary - {:?}", op);
        thread::sleep(time::Duration::from_millis((1000) as u64));
    }

    // ANCHOR
    pub fn mock_binary_v2<T: TensorLike + Clone>(
        &mut self,
        op: MockOpCode,
        lhs: Arc<RwLock<T>>,
        rhs: Arc<RwLock<T>>,
        out: Arc<RwLock<T>>,
    ) -> () {
        info!("anchor - binary - {:?}", op);
        thread::sleep(time::Duration::from_millis((1000) as u64));
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
        arg: Arc<RwLock<Self::TensorType>>,
    ) -> Self::TensorType {
        self.mock_unary::<Self::TensorType>(op, arg)
    }

    fn dma_operation(
        &mut self,
        op: Self::OpCodeType,
        arg: Arc<RwLock<Self::TensorType>>,
        out: Arc<RwLock<Self::TensorType>>,
        shape: Vec<usize>,
    ) -> () {
        self.dma_operation::<Self::TensorType>(op, arg, out, shape);
    }

    fn unary_compute_v2(
        &mut self,
        op: Self::OpCodeType,
        arg: Arc<RwLock<Self::TensorType>>,
        out: Arc<RwLock<Self::TensorType>>,
    ) -> () {
        self.mock_unary_v2::<Self::TensorType>(op, arg, out);
    }

    fn binary_compute(
        &mut self,
        op: Self::OpCodeType,
        lhs: Arc<RwLock<Self::TensorType>>,
        rhs: Arc<RwLock<Self::TensorType>>,
    ) -> Self::TensorType {
        info!("point enter #1");
        let _i = self.mock_binary::<Self::TensorType>(op, lhs, rhs);
        info!("point exit #1");
        _i
    }

    fn binary_compute_v2(
        &mut self,
        op: Self::OpCodeType,
        lhs: Arc<RwLock<Self::TensorType>>,
        rhs: Arc<RwLock<Self::TensorType>>,
        out: Arc<RwLock<Self::TensorType>>,
    ) -> () {
        info!("correct");
        info!("point enter #2");
        self.mock_binary_v2::<Self::TensorType>(op, lhs, rhs, out);
        info!("point exit #2");
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
