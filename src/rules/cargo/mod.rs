use crate::rules::cargo::cargo_build::Cargo;
use crate::rules::cargo::cargo_no_command::CargoNoCommand;
use crate::rules::Rule;
use std::sync::Arc;

mod cargo_build;
mod cargo_no_command;

pub(crate) fn rules_for_command() -> (&'static str, Vec<Arc<dyn Rule>>) {
    ("cargo", vec![Cargo.to_arc(), CargoNoCommand.to_arc()])
}
