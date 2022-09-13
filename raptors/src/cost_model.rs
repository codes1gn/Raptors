// LICENSE PLACEHOLDER
use std::collections::HashMap;

use crate::tensor_types::Workload;

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

/// Definition: The estimator helps to compute the estimated cost for different ops.
///
/// backdoors for mocking tests are also provided by this class.
#[derive(Clone, Debug, PartialEq)]
pub struct CostModel {
    cost_model: HashMap<OpCode, usize>,
}

impl Default for CostModel {
    fn default() -> Self {
        let mut cost_model = HashMap::new();
        cost_model.insert(OpCode::IdentityOp, 2);
        cost_model.insert(OpCode::AddOp, 11);
        cost_model.insert(OpCode::ConvOp, 107);
        cost_model.insert(OpCode::ExpOp, 173);
        cost_model.insert(OpCode::MatmulOp, 57);
        cost_model.insert(OpCode::SinOp, 127);
        cost_model.insert(OpCode::SubOp, 17);
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
    pub fn set_model(cost_model: HashMap<OpCode, usize>) -> Self {
        return Self {
            cost_model: cost_model,
        };
    }

    pub fn cost_model(&self) -> HashMap<OpCode, usize> {
        self.cost_model.clone()
    }

    pub fn estimate(&self, workload: &Workload) -> usize {
        *self.cost_model.get(&workload.op()).unwrap()
    }

    // TODO support update with moving average strategy
    pub fn update_model(&mut self, op: OpCode, new_cost: usize) -> () {
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
            model.get_key_value(&OpCode::AddOp),
            Some((&OpCode::AddOp, &11))
        );
        assert_eq!(
            model.get_key_value(&OpCode::ConvOp),
            Some((&OpCode::ConvOp, &107))
        );
        assert_eq!(
            model.get_key_value(&OpCode::MatmulOp),
            Some((&OpCode::MatmulOp, &57))
        );
    }

    #[test]
    fn create_estimator_with_model_test() {
        let mut model = HashMap::new();
        model.insert(OpCode::IdentityOp, 4);
        model.insert(OpCode::AddOp, 2);

        let est = CostModel::set_model(model);
        let model = est.cost_model();
        assert_eq!(
            model.get_key_value(&OpCode::AddOp),
            Some((&OpCode::AddOp, &2))
        );
        assert_eq!(
            model.get_key_value(&OpCode::IdentityOp),
            Some((&OpCode::IdentityOp, &4))
        );
    }

    #[test]
    fn estimate_computation_cost_test() {
        let est = CostModel::new();

        let load_1 = Workload::new(OpCode::AddOp);
        let cost_1 = est.estimate(&load_1);
        assert_eq!(cost_1, 11);

        let load_2 = Workload::new(OpCode::ConvOp);
        let cost_2 = est.estimate(&load_2);
        assert_eq!(cost_2, 107);
    }

    #[test]
    fn update_model_test() {
        let mut model = HashMap::new();
        model.insert(OpCode::IdentityOp, 4);

        let mut est = CostModel::set_model(model);

        assert!(est.cost_model.contains_key(&OpCode::IdentityOp));
        assert_eq!(
            est.cost_model.get_key_value(&OpCode::IdentityOp),
            Some((&OpCode::IdentityOp, &4))
        );
        est.update_model(OpCode::IdentityOp, 8);
        assert_eq!(
            est.cost_model.get_key_value(&OpCode::IdentityOp),
            Some((&OpCode::IdentityOp, &8))
        );

        assert_eq!(est.cost_model.contains_key(&OpCode::ConvOp), false);
        est.update_model(OpCode::ConvOp, 100);
        assert!(est.cost_model.contains_key(&OpCode::ConvOp));
        assert_eq!(
            est.cost_model.get_key_value(&OpCode::ConvOp),
            Some((&OpCode::ConvOp, &100))
        );
    }
}
