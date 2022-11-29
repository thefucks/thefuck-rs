use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

/// Corrects a git command that first requires a git stash
pub(crate) struct GitStash;
impl Rule for GitStash {
    default_rule_id!(GitStash);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command.lowercase_output().contains("or stash them")
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        Some(vec![RuleCorrection::and("git stash", command.input)])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_git_stash() {
        assert_eq!(
            basic_corrections(
                "git checkout master",
                r#"error: Your local changes to the following files would be overwritten by checkout:
                foo/bar.rs
                Please commit your changes or stash them before you switch branches.
                Aborting"#
            ),
            vec!["git stash && git checkout master"]
        )
    }
}
