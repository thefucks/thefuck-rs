use crate::rules::cd::{cd_correction::CdCorrection, cd_mkdir::CdMkdir};

use crate::rules::Rule;

use super::CommandGroup;

mod cd_correction;
mod cd_mkdir;

pub(crate) fn command_group() -> CommandGroup {
    CommandGroup {
        command_names: &["cd"],
        rules: vec![CdCorrection.to_arc(), CdMkdir.to_arc()],
    }
}
