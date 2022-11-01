use crate::rules::yarn::{
    yarn_alias::YarnAlias, yarn_command_not_found::YarnCommandNotFound,
    yarn_command_replaced::YarnCommandReplaced, yarn_help::YarnHelp,
};
use crate::rules::Rule;

use super::CommandGroup;

mod yarn_alias;
mod yarn_command_not_found;
mod yarn_command_replaced;
mod yarn_help;

pub(crate) fn command_group() -> CommandGroup {
    CommandGroup {
        command_names: &["yarn"],
        rules: vec![
            YarnHelp.to_arc(),
            YarnAlias.to_arc(),
            YarnCommandNotFound.to_arc(),
            YarnCommandReplaced.to_arc(),
        ],
    }
}
