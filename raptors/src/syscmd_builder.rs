// LICENSE PLACEHOLDER

use crate::messages::SystemCommand;

/// SystemCommand Builder (SysCmdBuilder) helps to create system command.
//
#[derive(Default)]
pub struct SysCmdBuilder {
    cmd: SystemCommand,
}

impl SysCmdBuilder {
    pub fn new() -> Self {
        SysCmdBuilder::default()
    }

    pub fn set_cmd(&mut self, cmd: SystemCommand) {
        self.cmd = cmd;
    }

    pub fn cmd(&self) -> SystemCommand {
        self.cmd.clone()
    }

    pub fn build_with_cmd(cmd: SystemCommand) -> Self {
        return Self { 
            cmd: cmd,
        };
    }
}

// unit tests
#[cfg(test)]
mod SysCmdBuilder_tests {
    use super::*;

    #[test]
    fn create_builder() {
        let builder = SysCmdBuilder::build_with_cmd(SystemCommand::DestroyAllActors);

        assert_eq!(builder.cmd, SystemCommand::DestroyAllActors);
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
}
