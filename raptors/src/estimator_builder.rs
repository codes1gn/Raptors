use crate::estimator::WorkloadEstimator;
use crate::messages::OpCode;

use std::collections::HashMap;

/// Definition: The helper that provide helper functions for estimator creation.
/// This class wraps all the complex logic used to build elements in.
///
/// backdoors for mocking tests are also provided by this helper
///
/// TODO helper reads files to set up a new estimator
//
#[derive(Clone, Debug, PartialEq)]
pub struct WorkloadEstimatorBuilder {
    cost_model: HashMap<OpCode, usize>,
}

impl WorkloadEstimatorBuilder {
    pub fn new() -> Self {
        WorkloadEstimatorBuilder::default()
    }

    pub fn set_model(cost_model: HashMap<OpCode, usize>) -> Self {
        return Self {
            cost_model: cost_model,
        };
    }

    pub fn cost_model(&self) -> HashMap<OpCode, usize> {
        self.cost_model.clone()
    }

    pub fn build(&self) -> WorkloadEstimator {
        WorkloadEstimator::new(self.cost_model())
    }
}

impl Default for WorkloadEstimatorBuilder {
    fn default() -> Self {
        let mut cost_model = HashMap::new();
        cost_model.insert(OpCode::DummyOp, 4);
        cost_model.insert(OpCode::AddOp, 2);
        cost_model.insert(OpCode::ConvOp, 8);
        cost_model.insert(OpCode::ExpOp, 1);
        cost_model.insert(OpCode::MatmulOp, 10);
        cost_model.insert(OpCode::SinOp, 1);
        cost_model.insert(OpCode::SubOp, 2);
        return Self {
            cost_model: cost_model,
        };
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_builder_test() {
        let builder = WorkloadEstimatorBuilder::default();

        assert_eq!(
            builder.cost_model.get_key_value(&OpCode::DummyOp),
            Some((&OpCode::DummyOp, &4))
        );
        assert_eq!(
            builder.cost_model.get_key_value(&OpCode::AddOp),
            Some((&OpCode::AddOp, &2))
        );
    }

    #[test]
    fn set_model_test() {
        let mut model = HashMap::new();
        model.insert(OpCode::DummyOp, 40);
        model.insert(OpCode::AddOp, 20);

        let builder = WorkloadEstimatorBuilder::set_model(model.clone());

        assert_eq!(
            builder.cost_model.get_key_value(&OpCode::DummyOp),
            Some((&OpCode::DummyOp, &40))
        );
        assert_eq!(
            builder.cost_model.get_key_value(&OpCode::AddOp),
            Some((&OpCode::AddOp, &20))
        );
    }

    #[test]
    fn cost_model_test() {
        let mut model = HashMap::new();
        model.insert(OpCode::DummyOp, 40);
        model.insert(OpCode::AddOp, 20);

        let builder = WorkloadEstimatorBuilder::set_model(model.clone());

        assert_eq!(builder.cost_model(), model);
    }

    #[test]
    fn build_WorkloadEstimator_test() {
        let mut model = HashMap::new();
        model.insert(OpCode::DummyOp, 40);
        model.insert(OpCode::AddOp, 20);

        let builder = WorkloadEstimatorBuilder::set_model(model.clone());
        let est_from_builder = builder.build();

        let est_from_origin = WorkloadEstimator::new(model.clone());

        assert_eq!(est_from_builder, est_from_origin);
    }
}
