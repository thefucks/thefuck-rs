use crate::rules::cargo::cargo_build::Cargo;
use crate::rules::cargo::cargo_no_command::CargoNoCommand;
use crate::rules::Rule;

use super::CommandGroup;

mod cargo_build;
mod cargo_no_command;

pub(crate) fn command_group() -> CommandGroup {
    CommandGroup {
        command_names: &["cargo"],
        rules: vec![CargoNoCommand.to_arc(), Cargo.to_arc()],
    }
}
