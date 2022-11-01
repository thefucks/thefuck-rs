use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

/// Corrects command if trying to grep a directory
pub(crate) struct GrepRecursive;
impl Rule for GrepRecursive {
    default_rule_id!(GrepRecursive);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command.lowercase_output().contains("is a directory")
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let mut new_command = command.input_parts().to_vec();
        new_command.insert(1, "--recursive".to_owned());
        Some(vec![new_command.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_grep_recursive() {
        assert_eq!(
            basic_corrections("grep test dir", "grep: dir: Is a directory"),
            vec!["grep --recursive test dir"]
        )
    }
}
