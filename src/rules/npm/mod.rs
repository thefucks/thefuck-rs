use crate::rules::npm::npm_unknown_command::NpmUnknownCommand;
use crate::rules::Rule;

use super::CommandGroup;

mod npm_unknown_command;

pub(crate) fn command_group() -> CommandGroup {
    CommandGroup {
        command_names: &["npm"],
        rules: vec![NpmUnknownCommand.to_arc()],
    }
}
