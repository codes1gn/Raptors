// LICENSE PLACEHOLDER

use crate::{
    messages::{SystemCommand, TypedMessage},
    prelude::SystemMsg,
};
use core::panic;

/// SystemCommand Builder (SysCmdBuilder) helps to create system command.
//
#[derive(Default, Debug)]
pub struct SystemCmdBuilder;

// TODO: we need to derive a Macro to wrap the arguments for build function
impl SystemCmdBuilder {
    // Definer fns
    pub fn new() -> Self {
        SystemCmdBuilder::default()
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
            _ => {
                panic!("Not implemented")
            }
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
    use crate::build_msg;

    use super::*;
    use core::panic;

    #[test]
    fn command_build_test() {
        let builder = SystemCmdBuilder::new();
        let cmd = builder.build(
            "create-actor",
            Some(vec![1]),
            Some(vec!["Raptor".to_owned()]),
        );
        assert_eq!(cmd, SystemCommand::CreateActor(1, "Raptor".to_owned()));
    }

    #[test]
    fn msg_build_test() {
        let builder = SystemCmdBuilder::new();
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

    #[test]
    //#[should_panic]
    fn macro_build_msg_test() {
        let destroy_cmd_macro: TypedMessage = build_msg!("destroy-actor");
        assert_eq!(destroy_cmd_macro, SystemCommand::DestroyAllActors.into());

        let dummy_cmd_macro: TypedMessage = build_msg!("dummy");
        assert_eq!(dummy_cmd_macro, SystemCommand::DummySysCmd.into());
    }
}
