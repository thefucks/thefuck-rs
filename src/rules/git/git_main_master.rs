use crate::rules::util::new_commands_from_suggestions;
use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex =
        Regex::new("(?i)error: pathspec '(main|master)' did not match any file").unwrap();
}

/// Suggests to checkout master if it exists and the user is trying to checkout main.
/// And vice-versa.
pub(crate) struct GitMainMaster;
impl Rule for GitMainMaster {
    default_rule_id!(GitMainMaster);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command.input_parts().iter().any(|p| p == "checkout")
            && RE.is_match(command.lowercase_output())
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let wrong_branch = RE
            .captures(command.lowercase_output())
            .and_then(|captures| captures.get(1))
            .map(|regex_match| regex_match.as_str())?;

        let master_exists = session_metadata.git_branches.contains("master");
        let main_exists = session_metadata.git_branches.contains("main");

        match wrong_branch {
            "main" if master_exists => {
                new_commands_from_suggestions(["master"], command.input_parts(), "main")
            }
            "master" if main_exists => {
                new_commands_from_suggestions(["main"], command.input_parts(), "master")
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::regular_corrections;
    use crate::{Command, ExitCode, SessionMetadata};

    #[test]
    fn test_git_main_master_with_master() {
        let command = Command::new(
            "git checkout master",
            "error: pathspec 'master' did not match any file(s) known to git",
            ExitCode(1),
        );
        let mut session_metadata = SessionMetadata::new();
        session_metadata.set_git_branches(["main"]);

        assert!(regular_corrections(command, &session_metadata)
            .contains(&"git checkout main".to_owned()))
    }

    #[test]
    fn test_git_main_master_with_main() {
        let command = Command::new(
            "git checkout main",
            "error: pathspec 'main' did not match any file(s) known to git",
            ExitCode(1),
        );
        let mut session_metadata = SessionMetadata::new();
        session_metadata.set_git_branches(["master"]);

        assert!(regular_corrections(command, &session_metadata)
            .contains(&"git checkout master".to_owned()))
    }
}
