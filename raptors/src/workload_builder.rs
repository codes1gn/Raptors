// LICENSE PLACEHOLDER

use crate::messages::{OpCode, OpCode::AddOp, TypedMessage, Workload, WorkloadMsg};

/// Workload Builder helps create the workload.
#[derive(Default)]
pub struct WorkloadBuilder;

impl WorkloadBuilder {
    // Definer fns
    pub fn new() -> Self {
        WorkloadBuilder::default()
    }

    pub fn build(&self, payload: Option<usize>, op: Option<OpCode>) -> Workload {
        return Workload::new(
            payload.expect("No valid payload value"),
            op.expect("No valid operation"),
        );
    }

    pub fn build_msg(&self, payload: Option<usize>, op: Option<OpCode>) -> TypedMessage {
        self.build(payload, op).into()
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{build, build_msg};

    #[test]
    fn workload_build_test() {
        let builder = WorkloadBuilder::new();
        let workload = builder.build(Some(2), Some(OpCode::DummyOp));
        assert_eq!(workload, Workload::new(2, OpCode::DummyOp));
    }

    #[test]
    fn msg_build_test() {
        let builder = WorkloadBuilder::new();
        let msg = builder.build_msg(Some(2), Some(OpCode::AddOp));
        let workload = builder.build(Some(2), Some(OpCode::AddOp));
        assert_eq!(msg, TypedMessage::WorkloadMsg(workload));
    }

    #[test]
    fn macro_build_msg_test() {
        let wl_macro_1: TypedMessage = build_msg!("workload", (2, AddOp));
        // If I pass OpCode::AddOp, cannot convert it to tt for macro
        assert_eq!(wl_macro_1, Workload::new(2, OpCode::AddOp).into());

        //     let wl_macro_2: TypedMessage = build_msg!("workload", (100, OpCode::ConvOp));
        //     assert_eq!(wl_macro_2, Workload::new(100, OpCode::ConvOp).into());
    }
}
