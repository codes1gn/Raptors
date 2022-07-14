// LICENSE PLACEHOLDER

use crate::messages::SystemCommand;

/// SystemCommand Builder (SysCmdBuilder) helps to create system command.
//
#[derive(Clone, Debug, PartialEq)]
pub struct SysCmdBuilder {
    cmd: SystemCommand,
}

impl SysCmdBuilder {
    pub fn new(cmd: SystemCommand) -> Self {
        return Self {
            cmd: cmd,
        };
    }

    pub fn set_cmd(&mut self, cmd: SystemCommand) {
        self.cmd = cmd;
    }

    pub fn cmd(&self) -> SystemCommand {
        self.cmd.clone()
    }
}

impl Default for SysCmdBuilder {
    fn default() -> Self {
        return Self {
            cmd: SystemCommand::default(),
        }
    }
}

// unit tests
#[cfg(test)]
mod SysCmdBuilder_tests {
    use super::*;

    #[test]
    fn create_builder() {
        let builder = SysCmdBuilder::new(SystemCommand::DestroyAllActors);

        assert_eq!(builder.cmd, SystemCommand::DestroyAllActors);
    }

    #[test]
    fn set_cmd_test() {
        let builder = SysCmdBuilder::new(SystemCommand::DestroyAllActors);
        assert_eq!(builder::cmd(), SystemCommand::DestroyAllActors);

        builder::set_cmd(SystemCommand::CreateActor(1, "actor".to_owned()));
        assert_eq!(builder::cmd(), SystemCommand::CreateActor(1, "actor".to_owned()));
    }
}

