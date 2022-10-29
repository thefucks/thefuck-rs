use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

/// This rule corrects a cd command that looks like "cd.."
/// Note: this is a "generic" rule because `cd..` doesn't apply to `cd`
pub(crate) struct CdParent;
impl Rule for CdParent {
    default_rule_id!(CdParent);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command.input == "cd.."
    }

    fn generate_command_corrections<'a>(
        &self,
        _command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        Some(vec![vec!["cd", ".."].into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_cd_parent() {
        assert_eq!(
            basic_corrections("cd..", "zsh: command not found: cd.."),
            vec!["cd .."]
        )
    }
}
