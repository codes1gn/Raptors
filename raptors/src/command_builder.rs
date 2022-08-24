// LICENSE PLACEHOLDER

use core::panic;

use crate::{
    messages::{SystemCommand, TypedMessage},
    prelude::SystemMsg,
};

/// SystemCommand Builder (SysCmdBuilder) helps to create system command.
//
#[derive(Default)]
pub struct CommandBuilder;

// TODO: we need to derive a Macro to wrap the arguments for build function
impl CommandBuilder {
    // Definer fns
    pub fn new() -> Self {
        CommandBuilder::default()
    }

    // Builder fns
    pub fn build(
        &self,
        cmd: &str,
        numeric_args: Option<Vec<usize>>,
        literal_args: Option<Vec<String>>,
    ) -> SystemCommand {
        match cmd {
            "create-actor" => {
                let nums = numeric_args.expect("No numbers provided");
                let liters = literal_args.expect("No string provided");
                SystemCommand::CreateActor(
                    *nums.get(0).unwrap(),
                    liters.get(0).unwrap().to_string(),
                )
            }
            "destroy-actor" => SystemCommand::DestroyAllActors,
            _ => panic!("Unknown builder command"),
        }
    }

    pub fn build_msg(
        &self,
        cmd: &str,
        numeric_args: Option<Vec<usize>>,
        literal_args: Option<Vec<String>>,
    ) -> TypedMessage {
        self.build(cmd, numeric_args, literal_args).into()
    }
}

// unit tests
#[cfg(test)]
mod syscmd_builder_tests {
    use super::*;

    #[test]
    fn command_build_test() {
        let builder = CommandBuilder::new();
        let cmd = builder.build(
            "create-actor",
            Some(vec![1]),
            Some(vec!["Raptor".to_owned()]),
        );
        assert_eq!(cmd, SystemCommand::CreateActor(1, "Raptor".to_owned()));
    }

    #[test]
    fn msg_build_test() {
        let builder = CommandBuilder::new();
        let cmd = builder.build(
            "create-actor",
            Some(vec![1]),
            Some(vec!["Raptor".to_owned()]),
        );
        let msg = builder.build_msg(
            "create-actor",
            Some(vec![1]),
            Some(vec!["Raptor".to_owned()]),
        );
        assert_eq!(msg, TypedMessage::SystemMsg(cmd));
    }
}
