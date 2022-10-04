/*
Fixes error for commands that need to be run with "sudo".
See more here: https://github.com/nvbn/thefuck/blob/5198b34f24ca4bc414a5bf1b0288ee86ea2529a8/thefuck/rules/sudo.py
*/

use crate::rules::Rule;
use crate::{Command, Correction};

pub(crate) struct Sudo;

static PATTERNS: &[&str] = &[
    "permission denied",
    "eacces",
    "pkg: insufficient privileges",
    "you cannot perform this operation unless you are root",
    "non-root users cannot",
    "operation not permitted",
    "not super-user",
    "superuser privilege",
    "root privilege",
    "this command has to be run under the root user",
    "this operation requires root",
    "requested operation requires superuser privilege",
    "must be run as root",
    "must run as root",
    "must be superuser",
    "must be root",
    "need to be root",
    "need root",
    "needs to be run as root",
    "only root can",
    "you don't have access to the history db",
    "authentication is required",
    "edspermissionerror",
    "you don't have write permissions",
    "use `sudo`",
    "sudorequirederror",
    "error: insufficient privileges",
    "updatedb: can not open a temporary file",
];

impl Rule for Sudo {
    fn matches(&self, command: &Command) -> bool {
        // If user already tried sudo, no point in suggesting it again.
        if let Some("sudo") = command.input_parts().first().map(String::as_str) {
            return false;
        }

        let lowercase_output = command.lowercase_output();
        PATTERNS
            .iter()
            .any(|pattern| lowercase_output.contains(pattern))
    }

    fn generate_command_corrections(&self, command: &Command) -> Option<Vec<Correction>> {
        let new_command = [&["sudo".to_owned()], command.input_parts()].concat();
        Some(vec![new_command.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::command_corrections;

    #[test]
    fn test_single_command() {
        assert_eq!(
            command_corrections("rm file", "permission denied"),
            vec!["sudo rm file"]
        )
    }
}
