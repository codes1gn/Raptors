// LICENSE PLACEHOLDER

use crate::{messages::SystemCommand, prelude::SystemMsg};

/// SystemCommand Builder (SysCmdBuilder) helps to create system command.
//
#[derive(Default)]
pub struct SysCmdBuilder {
    cmd: SystemCommand,
}

impl SysCmdBuilder {
    // Definer fns
    pub fn new() -> Self {
        SysCmdBuilder::default()
    }

    pub fn build_with_cmd(cmd: SystemCommand) -> Self {
        return Self { cmd: cmd };
    }

    // Setter fns
    pub fn set_cmd(&mut self, cmd: SystemCommand) {
        self.cmd = cmd;
    }

    // Getter fns
    pub fn cmd(&self) -> SystemCommand {
        self.cmd.clone()
    }

    // Builder fns
    pub fn build_cmd(&self) -> SystemCommand {
        return self.cmd().clone();
    }

    pub fn build_msg(&self) -> SystemMsg {
        return SystemMsg::new(self.build_cmd());
    }
}

// unit tests
#[cfg(test)]
mod syscmd_builder_tests {
    use super::*;

    #[test]
    fn create_builder() {
        let builder = SysCmdBuilder::new();
        assert_eq!(builder.cmd(), SystemCommand::default());

        let builder = SysCmdBuilder::build_with_cmd(SystemCommand::DestroyAllActors);
        assert_eq!(builder.cmd(), SystemCommand::DestroyAllActors);
    }

    #[test]
    fn set_cmd_test() {
        let mut builder = SysCmdBuilder::new();
        assert_eq!(builder.cmd(), SystemCommand::DummySysCmd);

        builder.set_cmd(SystemCommand::CreateActor(1, "actor".to_owned()));
        assert_eq!(
            builder.cmd(),
            SystemCommand::CreateActor(1, "actor".to_owned())
        );
    }

    #[test]
    fn build_cmd_test() {
        let builder = SysCmdBuilder::build_with_cmd(SystemCommand::DestroyAllActors);

        assert_eq!(builder.build_cmd(), SystemCommand::DestroyAllActors);
    }

    #[test]
    fn build_msg_test() {
        let builder = SysCmdBuilder::build_with_cmd(SystemCommand::DestroyAllActors);

        assert_eq!(
            builder.build_msg(),
            SystemMsg::new(SystemCommand::DestroyAllActors)
        );
    }
}
