// LICENSE PLACEHOLDER

use crate::messages::{OpCode, Workload};

use std::collections::HashMap;

/// Definition: The estimator helps to compute the estimated cost for different ops.
///
/// backdoors for mocking tests are also provided by this class.
#[derive(Clone, Debug, PartialEq)]
pub struct Estimator {
    model: HashMap<OpCode, usize>,
}
// For the estimation process, we need
// 1) kind of ops, like AddOp, MulOp, MatmulOp, etc.
// 2) the size of inputs
// 3) compute and return the estimated cost
// ======================================================
// What an estimator could do:
// 1) set up an estimator
// 2) compute the cost
// 3) renew the cost model (Which data structure for the cost model)
// =======================================================
// What an estimator should hold:
// 1) cost model
impl Estimator {
    // An initial model is necessary when setting up the estimator
    pub fn new() -> Self {
        let mut model = HashMap::new();
        model.insert(OpCode::DummyOp, 4);
        model.insert(OpCode::AddOp, 2);
        model.insert(OpCode::ConvOp, 8);
        model.insert(OpCode::ExpOp, 1);
        model.insert(OpCode::MatmulOp, 10);
        model.insert(OpCode::SinOp, 1);
        model.insert(OpCode::SubOp, 2);
        return Self { model: model };
    }

    pub fn new_with_model(mut model: HashMap<OpCode, usize>) -> Self {
        return Self { model: model };
    }
    // Attention!!! How should we deal with model(ownership)
    pub fn model(&self) -> HashMap<OpCode, usize> {
        self.model.clone()
    }

    pub fn estimate(&self, workload: Workload) -> usize {
        *self.model.get(&workload.op()).unwrap() * workload.payload()
    }

    pub fn update_model(&mut self, op: OpCode, new_cost: usize) -> () {
        match self.model.get_mut(&op) {
            Some(cost) => {
                *cost = new_cost;
            }
            _ => {
                self.model.insert(op, new_cost);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_estimator_wtihout_model_test() {
        let est = Estimator::new();
        let model = est.model();
        assert_eq!(
            model.get_key_value(&OpCode::AddOp),
            Some((&OpCode::AddOp, &2))
        );
        assert_eq!(
            model.get_key_value(&OpCode::ConvOp),
            Some((&OpCode::ConvOp, &8))
        );
        assert_eq!(
            model.get_key_value(&OpCode::MatmulOp),
            Some((&OpCode::MatmulOp, &10))
        );
    }

    #[test]
    fn create_estimator_with_model_test() {
        let mut model = HashMap::new();
        model.insert(OpCode::DummyOp, 4);
        model.insert(OpCode::AddOp, 2);

        let est = Estimator::new_with_model(model);
        let model = est.model();
        assert_eq!(
            model.get_key_value(&OpCode::AddOp),
            Some((&OpCode::AddOp, &2))
        );
        assert_eq!(
            model.get_key_value(&OpCode::DummyOp),
            Some((&OpCode::DummyOp, &4))
        );
    }

    #[test]
    fn estimate_computation_cost_test() {
        let est = Estimator::new();

        let load_1 = Workload::new(16, OpCode::AddOp);
        let cost_1 = est.estimate(load_1);
        assert_eq!(cost_1, 32);

        let load_2 = Workload::new(4, OpCode::DummyOp);
        let cost_2 = est.estimate(load_2);
        assert_eq!(cost_2, 16);
    }

    #[test]
    fn update_model_test() {
        let mut model = HashMap::new();
        model.insert(OpCode::DummyOp, 4);

        let mut est = Estimator::new_with_model(model);

        assert!(est.model.contains_key(&OpCode::DummyOp));
        assert_eq!(
            est.model.get_key_value(&OpCode::DummyOp),
            Some((&OpCode::DummyOp, &4))
        );
        est.update_model(OpCode::DummyOp, 8);
        assert_eq!(
            est.model.get_key_value(&OpCode::DummyOp),
            Some((&OpCode::DummyOp, &8))
        );

        assert_eq!(est.model.contains_key(&OpCode::ConvOp), false);
        est.update_model(OpCode::ConvOp, 100);
        assert!(est.model.contains_key(&OpCode::ConvOp));
        assert_eq!(
            est.model.get_key_value(&OpCode::ConvOp),
            Some((&OpCode::ConvOp, &100))
        );
    }
}
