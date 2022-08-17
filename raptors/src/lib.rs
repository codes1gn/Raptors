// TODO: reorganize 'mod' and 'prelude' to control the visibility
pub mod actors;
pub mod context;
pub mod estimator;
pub mod executor;
pub mod mailbox;
pub mod mailbox_new;
pub mod messages;
pub mod profiler;
// pub mod ring_buffer;
pub mod system;
pub mod system_builder;
pub mod system_command_builder;
pub mod system_config;
pub mod workload_builder;
/// Prelude module for users to import
///
/// ```
/// use raptors::prelude::*;
/// ```
pub mod prelude {
    pub use crate::actors::Actor;
    pub use crate::estimator::WorkloadEstimator;
    pub use crate::executor::Executor;
    pub use crate::mailbox::{Len, Mailbox};
    pub use crate::messages::{
        OpCode, SystemCommand, SystemMsg, TypedMessage, Workload, WorkloadMsg,
    };
    pub use crate::system::System;
    pub use crate::system_builder::SystemBuilder;
    pub use crate::system_config::SystemConfig;
}

// ===----------------------------------------------===
// Macros
// ===----------------------------------------------===

// Idea: based on the number of inputs to construct the message
// 2 kinds:
//      1. Binary: besides the keyword, require other args to construct the message
//          for example: to construct a WorkloadMsg,
//              3 args: 'workload' as the keyword, payload(usize) and op(OpCode, Operation Type)
//      2. Unary: only require the keyword
//          for example: to construct a SystemCommand::DestroyAllActors
//              1 arg: 'destroy' as the keyword

// FIXME!!!
// in Binary part, we cannot convert 'expr' to different type
//  for 'workload': $arg2:expr should be converted into OpCode
//  for 'create-actor': $arg2:expr should be converted into String
//  This is the conflict.
//  A possible solution might be to use '$arg1:tt' and '$arg2:tt' to replace 'expr'.
//  If this could work, we could use another macro: crate::build! to build the sub-level data
//  @keyword might be useful, if we need to use this crate::build!

#[macro_export]
macro_rules! build_msg {
    ( $binary: expr, ($arg1: expr, $arg2: expr) ) => {
        {

                let keyword: &str = $binary;
                match keyword {
                    "workload" => {
                        // let msg: TypedMessage = Workload::new($arg1 as usize, $arg2 as OpCode).into();
                        // msg
                        let msg: TypedMessage = crate::build!(@workload, $arg1, $arg2).into();
                        msg
                    }

                    // ===----------------------===
                    // Fix Here!!!
                    // ===----------------------===
                    // "create-actor" => {
                    //     // let msg:TypedMessage = SystemCommand::CreateActor($arg1 as usize, $arg2).into();
                    //     // msg
                    //     crate::build!(@create, $arg1, $arg2)
                    // }

                    _ => {
                        panic!("Unknow Keyword, or number of vars not match with Keyword");
                    }
                }

        }
    };

    ( $unary:expr ) => {{
        let keyword: &str = $unary;
        match keyword {
            "destroy-actor" => {
                let msg: TypedMessage = SystemCommand::DestroyAllActors.into();
                msg
            }
            "dummy" => {
                let msg: TypedMessage = SystemCommand::DummySysCmd.into();
                msg
            }
            _ => {
                panic!("Unknow Keyword, or number of vars not match with Keyword");
            }
        }
    }};
}

// @keyword is required in the input, and used to specify which block it will fall into
#[macro_export]
macro_rules! build {
    (@workload,  $arg1: expr, $arg2: expr) => {{
        let payload: usize = $arg1;
        let op: OpCode = $arg2;
        let workload: Workload = Workload::new(payload, op);
        workload
    }};
    (@create,  $arg1: expr, $arg2: expr) => {{
        let count: usize = $arg1;
        let name: &str = $arg2;
        let msg: TypedMessage = SystemCommand::CreateActor(count, name.to_owned()).into();
        msg
    }};
}
