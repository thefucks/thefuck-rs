use crate::rules::mkdir::mkdir_p::MkdirP;
use crate::rules::Rule;

use super::CommandGroup;

mod mkdir_p;

pub(crate) fn command_group() -> CommandGroup {
    CommandGroup {
        command_names: &["mkdir"],
        rules: vec![MkdirP.to_arc()],
    }
}
