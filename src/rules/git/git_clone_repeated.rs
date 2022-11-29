use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

/// Removes an extra `git clone` prefix. Useful when copying from
/// a source that includes the entire command instead of just the remote ref.
pub(crate) struct GitCloneRepeated;
impl Rule for GitCloneRepeated {
    default_rule_id!(GitCloneRepeated);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command.input.starts_with("git clone git clone")
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        // remove the first instance of "git clone" from the command
        let new_command = command.input.strip_prefix("git clone")?;
        Some(vec![new_command.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_git_clone_repeated() {
        assert_eq!(
            basic_corrections(
                "git clone git clone git@github.com:Homebrew/brew.git",
                r#"fatal: Too many arguments
                usage: git clone [<options>] [--] <repo> [<dir>]
                ...
                "#
            ),
            vec!["git clone git@github.com:Homebrew/brew.git"]
        )
    }
}
