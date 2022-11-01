use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

/// Corrects a command that looks like ./file.py if insufficient permissions
/// or is being interpreted as a traditional shell script (doesn't have the
/// python shebang at the start)
pub(crate) struct Python;
impl Rule for Python {
    default_rule_id!(Python);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        let lowercase_output = command.lowercase_output();
        command
            .input_parts()
            .first()
            .map_or(false, |p| p.ends_with(".py"))
            && (lowercase_output.contains("permission denied")
                || lowercase_output.contains("command not found"))
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let mut original_command = command.input_parts().to_vec();
        original_command.insert(0, "python".to_owned());
        Some(vec![original_command.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_python() {
        assert_eq!(
            basic_corrections("./test.py --flag", "./test.py: command not found"),
            vec!["python ./test.py --flag"]
        )
    }
}
