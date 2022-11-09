use crate::rules::cd::{cd_correction::CdCorrection, cd_mkdir::CdMkdir};

use crate::rules::Rule;

use super::CommandGroup;
use crate::{Command, SessionMetadata};
mod cd_correction;
mod cd_mkdir;

pub(crate) fn command_group() -> CommandGroup {
    CommandGroup {
        command_names: &["cd"],
        rules: vec![CdCorrection.to_arc(), CdMkdir.to_arc()],
    }
}

/// Returns true iff the command's output matches the output of a cd command
/// when the argument (directory) doesn't exist.
/// Note: cd corrections don't work well for remote sessions yet (since we don't support
/// dynamic rules over SSH), so don't offer them for non-local sessions.
// TODO: eventually, use a callback to just check if the directory exists
fn matches_cd_doesnt_exist(command: &Command, session_metadata: &SessionMetadata) -> bool {
    let lowercase_output = command.lowercase_output();
    session_metadata.session_type.is_local()
        && (lowercase_output.contains("does not exist")
            || lowercase_output.contains("no such file or directory"))
}
