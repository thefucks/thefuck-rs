use crate::rules::cat::cat_dir::CatDir;
use crate::rules::Rule;

use super::CommandGroup;

mod cat_dir;

pub(crate) fn command_group() -> CommandGroup {
    CommandGroup {
        command_names: &["cat"],
        rules: vec![CatDir.to_arc()],
    }
}
