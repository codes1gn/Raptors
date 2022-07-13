// LICENSE PLACEHOLDER

use crate::messages::{OpCode, Workload};

use std::collections::HashMap;

/// Definition: The estimator helps to compute the estimated cost for different ops.
///
/// backdoors for mocking tests are also provided by this class.
#[derive(Clone, Debug, PartialEq)]
pub struct WorkloadEstimator {
    cost_model: HashMap<OpCode, usize>,
}

/// TODO estimator writes the current model into files, like json
/// TODO estimator reads files, like json, to update the cost model
/// TODO whether estimator reads files to update model, or the helper
///      reads the files to set up a new estimator
//
impl WorkloadEstimator {
    pub fn new(cost_model: HashMap<OpCode, usize>) -> Self {
        return Self {
            cost_model: cost_model,
        };
    }

    pub fn cost_model(&self) -> HashMap<OpCode, usize> {
        self.cost_model.clone()
    }

    pub fn estimate(&self, workload: Workload) -> usize {
        *self.cost_model.get(&workload.op()).unwrap() * workload.payload()
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
    fn estimator_new_test() {
        let mut model = HashMap::new();
        model.insert(OpCode::DummyOp, 4);

        let est = WorkloadEstimator::new(model.clone());

        assert_eq!(
            est.cost_model.get_key_value(&OpCode::DummyOp),
            Some((&OpCode::DummyOp, &4))
        );
        assert_eq!(
            est.cost_model.get_key_value(&OpCode::DummyOp),
            model.get_key_value(&OpCode::DummyOp)
        );
    }

    #[test]
    fn estimate_computation_cost_test() {
        let mut model = HashMap::new();
        model.insert(OpCode::DummyOp, 4);
        model.insert(OpCode::AddOp, 2);

        let est = WorkloadEstimator::new(model.clone());

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

        let mut est = WorkloadEstimator::new(model.clone());

        assert!(est.cost_model.contains_key(&OpCode::DummyOp));
        assert_eq!(
            est.cost_model.get_key_value(&OpCode::DummyOp),
            Some((&OpCode::DummyOp, &4))
        );

        est.update_model(OpCode::DummyOp, 8);
        assert_eq!(
            est.cost_model.get_key_value(&OpCode::DummyOp),
            Some((&OpCode::DummyOp, &8))
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
