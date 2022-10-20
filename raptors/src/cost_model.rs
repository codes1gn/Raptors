// LICENSE PLACEHOLDER
use std::collections::HashMap;

use crate::tensor_types::MockTensor;

// Definition for Opcode
/// ```
/// # // Test default function for OpCode
/// use raptors::prelude::*;
///
/// assert_eq!(MockOpCode::default(), MockOpCode::IdentityOp);
/// ```
///
pub trait OpCodeLike {}

impl OpCodeLike for MockOpCode {}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
// Copy trait is necessary, otherwise ownership will transit into the cost model
pub enum MockOpCode {
    IdentityOp,
    AddOp,
    SubOp,
    MulOp,
    DivOp,
    ConvOp,
    ExpOp,
    ReshapeOp,
    MatmulOp,
    SinOp,
}

impl Default for MockOpCode {
    fn default() -> Self {
        MockOpCode::IdentityOp
    }
}

/// Definition: The estimator helps to compute the estimated cost for different ops.
///
/// backdoors for mocking tests are also provided by this class.
#[derive(Clone, Debug, PartialEq)]
pub struct CostModel {
    cost_model: HashMap<MockOpCode, usize>,
}

impl Default for CostModel {
    fn default() -> Self {
        let mut cost_model = HashMap::new();
        cost_model.insert(MockOpCode::IdentityOp, 2);
        cost_model.insert(MockOpCode::AddOp, 11);
        cost_model.insert(MockOpCode::SubOp, 17);
        cost_model.insert(MockOpCode::MulOp, 71);
        cost_model.insert(MockOpCode::DivOp, 87);
        cost_model.insert(MockOpCode::ConvOp, 107);
        cost_model.insert(MockOpCode::ExpOp, 173);
        cost_model.insert(MockOpCode::ReshapeOp, 37);
        cost_model.insert(MockOpCode::MatmulOp, 57);
        cost_model.insert(MockOpCode::SinOp, 127);
        return Self {
            cost_model: cost_model,
        };
    }
}

/// TODO set a helper to build the estimator
/// TODO estimator read files, like json, to update the cost model
//
impl CostModel {
    pub fn new() -> Self {
        CostModel::default()
    }

    // TODO support load cost model from deserialize from proto files
    pub fn set_model(cost_model: HashMap<MockOpCode, usize>) -> Self {
        return Self {
            cost_model: cost_model,
        };
    }

    pub fn cost_model(&self) -> HashMap<MockOpCode, usize> {
        self.cost_model.clone()
    }

    pub fn estimate(&self, workload: &MockTensor) -> usize {
        *self.cost_model.get(&workload.op()).unwrap()
    }

    // TODO support update with moving average strategy
    pub fn update_model(&mut self, op: MockOpCode, new_cost: usize) -> () {
        match self.cost_model.get_mut(&op) {
            Some(cost) => {
                *cost = new_cost;
            }
            _ => {
                self.cost_model.insert(op, new_cost);
            }
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_estimator_wtihout_model_test() {
        let est = CostModel::new();
        let model = est.cost_model();
        assert_eq!(
            model.get_key_value(&MockOpCode::AddOp),
            Some((&MockOpCode::AddOp, &11))
        );
        assert_eq!(
            model.get_key_value(&MockOpCode::ConvOp),
            Some((&MockOpCode::ConvOp, &107))
        );
        assert_eq!(
            model.get_key_value(&MockOpCode::MatmulOp),
            Some((&MockOpCode::MatmulOp, &57))
        );
    }

    #[test]
    fn create_estimator_with_model_test() {
        let mut model = HashMap::new();
        model.insert(MockOpCode::IdentityOp, 4);
        model.insert(MockOpCode::AddOp, 2);

        let est = CostModel::set_model(model);
        let model = est.cost_model();
        assert_eq!(
            model.get_key_value(&MockOpCode::AddOp),
            Some((&MockOpCode::AddOp, &2))
        );
        assert_eq!(
            model.get_key_value(&MockOpCode::IdentityOp),
            Some((&MockOpCode::IdentityOp, &4))
        );
    }

    #[test]
    fn estimate_computation_cost_test() {
        let est = CostModel::new();

        let load_1 = MockTensor::new(MockOpCode::AddOp);
        let cost_1 = est.estimate(&load_1);
        assert_eq!(cost_1, 11);

        let load_2 = MockTensor::new(MockOpCode::ConvOp);
        let cost_2 = est.estimate(&load_2);
        assert_eq!(cost_2, 107);
    }

    #[test]
    fn update_model_test() {
        let mut model = HashMap::new();
        model.insert(MockOpCode::IdentityOp, 4);

        let mut est = CostModel::set_model(model);

        assert!(est.cost_model.contains_key(&MockOpCode::IdentityOp));
        assert_eq!(
            est.cost_model.get_key_value(&MockOpCode::IdentityOp),
            Some((&MockOpCode::IdentityOp, &4))
        );
        est.update_model(MockOpCode::IdentityOp, 8);
        assert_eq!(
            est.cost_model.get_key_value(&MockOpCode::IdentityOp),
            Some((&MockOpCode::IdentityOp, &8))
        );

        assert_eq!(est.cost_model.contains_key(&MockOpCode::ConvOp), false);
        est.update_model(MockOpCode::ConvOp, 100);
        assert!(est.cost_model.contains_key(&MockOpCode::ConvOp));
        assert_eq!(
            est.cost_model.get_key_value(&MockOpCode::ConvOp),
            Some((&MockOpCode::ConvOp, &100))
        );
    }
}
