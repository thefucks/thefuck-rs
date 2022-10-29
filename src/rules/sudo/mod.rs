use crate::rules::sudo::unsudo::Unsudo;
use crate::rules::Rule;

use super::CommandGroup;

mod unsudo;

pub(crate) fn command_group() -> CommandGroup {
    CommandGroup {
        command_names: &["sudo"],
        rules: vec![Unsudo.to_arc()],
    }
}
