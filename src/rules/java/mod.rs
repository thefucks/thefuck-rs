use crate::rules::java::dot_java::DotJava;
use crate::rules::Rule;

use super::CommandGroup;

mod dot_java;

pub(crate) fn command_group() -> CommandGroup {
    CommandGroup {
        command_names: &["java"],
        rules: vec![DotJava.to_arc()],
    }
}
