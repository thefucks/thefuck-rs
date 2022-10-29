use crate::rules::ls::ls_all::LsAll;
use crate::rules::Rule;

use super::CommandGroup;

mod ls_all;

pub(crate) fn command_group() -> CommandGroup {
    CommandGroup {
        command_names: &["ls"],
        rules: vec![LsAll.to_arc()],
    }
}
