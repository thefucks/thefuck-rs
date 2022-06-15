use crate::rules::cargo::cargo_build::Cargo;
use crate::rules::cargo::cargo_no_command::CargoNoCommand;
use crate::rules::Rule;
use std::sync::Arc;

mod cargo_build;
mod cargo_no_command;

pub(crate) fn rules() -> impl Iterator<Item = Arc<dyn Rule>> {
    [Cargo.to_arc(), CargoNoCommand.to_arc()].into_iter()
}
