// LICENSE PLACEHOLDER

use crate::messages::*;

/// Messages Builder helps to create workload and messages
//
// Build from Bottom to Up
// TODO Idiomatic Build Pattern, we should call Workload::new() -> Builder
//      currently we directly define the builder with new() -> Self
//
/// WrkMsgBuilder
#[derive(Clone, Debug, PartialEq)]
pub struct WrkMsgBuilder {
    payload: usize,
    op: messages::OpCode,
}

impl WrkMsgBuilder {
    pub fn new(payload: usize, op: OpCode) -> Self {
        return Self { 
            payload: payload, 
            op: op,
        };
    }

    pub fn set_payload(&mut self, payload: usize) {
        self.payload = payload;
    }
 
    pub fn set_OpCode(&mut self, op: OpCode) {
        self.op = op;
    }

    pub fn build_workload(&self) -> Workload {
        return Workload (self.payload.clone(), self.op.clone());
    }

    pub fn build_msg(&self) -> WorkloadMsg {
        return WorkloadMsg (
            Workload (self.payload.clone(), self.op.clone())
        );
    }
}

/// SysMsgBuilder
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SysMsgBuilder {
    cmd: SystemCommand,
}

impl SysMsgBuilder {
    fn new() -> Self {
        SysMsgBuilder::default()
    }

    fn set_cmd(&mut self, cmd: SystemCommand) {
        self.cmd = cmd;
    }

    fn build_cmd(&self) -> SystemCommand {
        self.cmd.clone()
    }

    fn build_msg(&self) -> SystemMsg {
        SystemMsg( self.cmd.clone )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MessageBuilder {
    SysMsgBuilder,
    AcrMsgBuilder,
    WrkMsgBuilder,
}

trait BuildMsg {
    fn build() -> TypedMessage;
}

impl BuildMsg for MessageBuilder {
    fn build_msg(&self) -> TypedMessage {
        match self {
            WrkMsgBuilder => {
                return WrkMsgBuilder::build_msg();
            },

            SysMsgBuilder => {
                return SysMsgBuilder::build_msg();
            }

            AcrMsgBuilder => {
                return 0;
            },
        }
    }
} 

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_builder_test() {
        let builder = WrkMsgBuilder::new(1, OpCode::DummyOp);

        assert_eq!(builder.payload, 1);
        assert_eq!(builder.op, OpCode::DummyOp);
    }
}
