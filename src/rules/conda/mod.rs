use crate::rules::conda::conda_unknown_command::CondaUnknownCommand;
use crate::rules::Rule;

use super::CommandGroup;

mod conda_unknown_command;

pub(crate) fn command_group() -> CommandGroup {
    CommandGroup {
        command_names: &["conda"],
        rules: vec![CondaUnknownCommand.to_arc()],
    }
}
