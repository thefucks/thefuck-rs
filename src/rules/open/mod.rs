use crate::rules::open::open_does_not_exist::OpenDoesNotExist;
use crate::rules::Rule;

use super::CommandGroup;

mod open_does_not_exist;

pub(crate) fn command_group() -> CommandGroup {
    CommandGroup {
        command_names: &["open"],
        rules: vec![OpenDoesNotExist.to_arc()],
    }
}
