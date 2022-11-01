use crate::rules::grep::{grep_arguments_order::GrepArgumentsOrder, grep_recursive::GrepRecursive};
use crate::rules::Rule;

use super::CommandGroup;

mod grep_arguments_order;
mod grep_recursive;

pub(crate) fn command_group() -> CommandGroup {
    CommandGroup {
        command_names: &["grep", "egrep"],
        rules: vec![GrepArgumentsOrder.to_arc(), GrepRecursive.to_arc()],
    }
}
