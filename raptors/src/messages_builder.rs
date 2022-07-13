// LICENSE PLACEHOLDER

use crate::messages::*;

/// Messages Builder helps to create workload and messages
//
// Build from Bottom to Up
// TODO Idiomatic Build Pattern, we should call Workload::new() -> Builder
//      currently we directly define the builder with new() -> Self
// TODO In the future, do we want to expose the trait for the enum of builder,
//      Once we impl this trait, we could generate the corresponding msg
//      But the drawback is: it will be complex
/// WrkMsgBuilder
#[derive(Clone, Debug, PartialEq)]
pub struct WrkMsgBuilder {
    payload: usize,
    op: OpCode, // messages::
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
 
    pub fn set_op(&mut self, op: OpCode) {
        self.op = op;
    }

    pub fn payload(&self) -> usize {
        self.payload.clone()
    }

    pub fn op(&self) -> OpCode {
        self.op.clone()
    }

    pub fn build_workload(&self) -> Workload {
        return Workload ( self::payload(), self::op() );
    }

    pub fn build_msg(&self) -> WorkloadMsg {
        return WorkloadMsg (
            Workload ( self::payload(), self::op() )
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

    fn cmd(&self) -> SystemCommand {
        self.cmd.clone()
    }

    fn build_cmd(&self) -> SystemCommand {
        self.cmd.clone()
    }

    fn build_msg(&self) -> SystemMsg {
        SystemMsg( self.cmd.clone )
    }
}

// TODOs: complete AcrMsgBuilder part in the future
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
mod WrkMsgBuilder_tests {
    use super::*;

    #[test]
    fn create_builder_test() {
        let builder = WrkMsgBuilder::new(1, OpCode::DummyOp);

        assert_eq!(builder.payload, 1);
        assert_eq!(builder.op, OpCode::DummyOp);
    }

    #[test]
    fn set_payload_test() {
        let mut builder = WrkMsgBuilder::new(1, OpCode::DummyOp);
        builder::set_payload(20);

        assert_eq!(builder::payload(), 20);
    }

    #[test]
    fn set_op_test() {
        let mut builder = WrkMsgBuilder::new(1, OpCode::DummyOp);
        builder::set_op(OpCode::AddOp);
        
        assert_eq!(builder::payload(), OpCode::AddOp);
    }
    
    #[test]
    fn build_workload_test() {
        let mut builder = WrkMsgBuilder::new(1, OpCode::DummyOp);
        let workload_built = builder::build_workload();
        
        assert_eq!(
            workload_built,
            Workload ( builder::payload(), builder::op() )
        );
    }
    
    #[test]
    fn build_msg_test() {
        let mut builder = WrkMsgBuilder::new(1, OpCode::DummyOp);
        let msg_build = builder::build_msg();
        
        assert_eq!(
            msg_built,
            WorkloadMsg (
                Workload ( builder::payload(), builder::op() )
            )
        );
    }
}

mod SysMsgBuilder_tests {
    use super::*;

    #[test]
    fn create_builder_test() {
        let builder = SysMsgBuilder::new();

        assert_eq!(builder, SysMsgBuilder::default());
    }

    #[test]
    fn set_cmd_test() {
        let mut builder = SysMsgBuilder::new();
        builder::set_cmd(SystemCommand::DestroyAllActors);

        assert_eq!(builder::cmd(), SystemCommand::DestroyAllActors);
    }
    
    #[test]
    fn build_cmd_test() {
        let mut builder = SysMsgBuilder::new();
        builder::set_cmd(SystemCommand::DestroyAllActors);

        assert_eq!(
            builder::build_cmd(),
            SystemCommand::DestroyAllActors
        );
    }
    
    #[test]
    fn build_msg_test() {
        let mut builder = SysMsgBuilder::new();
        builder::set_cmd(SystemCommand::DestroyAllActors);
        
        assert_eq!(
            builder::build_msg(),
            SystemMsg ( SystemCommand::DestroyAllActors )
        );
    }
}
