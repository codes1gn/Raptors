// LICENSE PLACEHOLDER

use crate::messages::*;

/// Messages Builder helps create messages
//
// Build from Bottom to Up
// TODO In the future, do we want to expose the trait for the enum of builder,
//      Once we impl this trait, we could generate the corresponding msg
//      But the drawback is: it will be complex

//===--------------------------------------------------------------------===//
/// WrkMsgBuilder to create WorkloadMsg
<<<<<<< HEAD
#[derive(Clone, Debug, Default, PartialEq)]
=======
#[derive(Clone, Debug, PartialEq, Default)]
>>>>>>> e776add... fmt checked, tests passed
pub struct WrkMsgBuilder {
    workload: Workload,
}

impl WrkMsgBuilder {
    // make it Default in the future
    pub fn new() -> Self {
<<<<<<< HEAD
        return Self {
<<<<<<< HEAD
            workload: Workload ( 0, OpCode::default() ),
=======
            workload: Workload::new(0, OpCode::DummyOp),
>>>>>>> 00414d0... Add builder
        };
=======
        WrkMsgBuilder::default()
    }

    pub fn build_with_workload(workload: Workload) -> Self {
        return Self { workload: workload };
>>>>>>> e776add... fmt checked, tests passed
    }

    pub fn set_workload(&mut self, workload: Workload) {
        self.workload = workload;
    }

    pub fn workload(&self) -> Workload {
        self.workload.clone()
    }

    pub fn build_msg(&self) -> WorkloadMsg {
<<<<<<< HEAD
        return WorkloadMsg ( self.workload.clone() );
=======
        return WorkloadMsg::new(self.workload());
>>>>>>> 00414d0... Add builder
    }
}
//===--------------------------------------------------------------------===//

/// SysMsgBuilder
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SysMsgBuilder {
    cmd: SystemCommand,
}

impl SysMsgBuilder {
    pub fn new() -> Self {
        SysMsgBuilder::default()
    }

    pub fn build_with_cmd(cmd: SystemCommand) -> Self {
        return Self { cmd: cmd };
    }

    pub fn set_cmd(&mut self, cmd: SystemCommand) {
        self.cmd = cmd;
    }

    pub fn cmd(&self) -> SystemCommand {
        self.cmd.clone()
    }
<<<<<<< HEAD
<<<<<<< HEAD

    fn build_msg(&self) -> SystemMsg {
        SystemMsg ( self.cmd.clone() )
=======
    
=======

>>>>>>> e776add... fmt checked, tests passed
    pub fn build_msg(&self) -> SystemMsg {
        return SystemMsg::new(self.cmd());
>>>>>>> 00414d0... Add builder
    }
}
//===--------------------------------------------------------------------===//

// TODOs: complete AcrMsgBuilder part in the future
#[derive(Clone, Debug, PartialEq)]
pub enum MessageBuilder {
    SysMsgBuilder,
    AcrMsgBuilder,
    WrkMsgBuilder,
}
<<<<<<< HEAD

trait BuildMsg {
    fn build() -> TypedMessage;
}

impl BuildMsg for MessageBuilder {
    fn build_msg(&self) -> TypedMessage {
        match self {
            WrkMsgBuilder => {
                return WrkMsgBuilder::build_msg(&self);
            },

            SysMsgBuilder => {
                return SysMsgBuilder::build_msg(&self);
            },

            AcrMsgBuilder => {
                return 0;
            },
        }
    }
} 
=======
//===--------------------------------------------------------------------===//
>>>>>>> 00414d0... Add builder

// unit tests
#[cfg(test)]
mod WrkMsgBuilder_tests {
    use super::*;

    #[test]
    fn create_builder_test() {
        let builder = WrkMsgBuilder::new();
        assert_eq!(builder.workload, Workload::new(0, OpCode::DummyOp));

        let builder = WrkMsgBuilder::build_with_workload(Workload::new(2, OpCode::AddOp));
        assert_eq!(builder.workload, Workload::new(2, OpCode::AddOp));
    }

    #[test]
    fn set_workload_test() {
        let mut builder = WrkMsgBuilder::new();
        builder.set_workload(Workload::new(20, OpCode::AddOp));

        assert_eq!(builder.workload(), Workload::new(20, OpCode::AddOp));
    }

    #[test]
    fn build_msg_test() {
        let builder = WrkMsgBuilder::new();
        let msg_built = builder.build_msg();

        assert_eq!(
            msg_built,
            WorkloadMsg::new(Workload::new(0, OpCode::DummyOp))
        );
    }
}

// unit tests
#[cfg(test)]
mod SysMsgBuilder_tests {
    use super::*;

    #[test]
    fn create_builder_test() {
        let builder = SysMsgBuilder::new();
        assert_eq!(builder.cmd, SystemCommand::default());

        let builder = SysMsgBuilder::build_with_cmd(SystemCommand::DestroyAllActors);
        assert_eq!(builder.cmd, SystemCommand::DestroyAllActors);
    }

    #[test]
    fn set_cmd_test() {
        let mut builder = SysMsgBuilder::new();
        builder.set_cmd(SystemCommand::DestroyAllActors);

        assert_eq!(builder.cmd(), SystemCommand::DestroyAllActors);
    }

    #[test]
    fn build_msg_test() {
        let builder = SysMsgBuilder::new();

        assert_eq!(
            builder.build_msg(),
            SystemMsg::new(SystemCommand::default())
        );
    }
}
