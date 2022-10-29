use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

/// Corrects command if trying to run script without setting permissions first
pub(crate) struct ChmodX;
impl Rule for ChmodX {
    default_rule_id!(ChmodX);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        let lowercase_output = command.output.to_lowercase();
        let input_starts_with_dir =
            command.input.starts_with('.') || command.input.starts_with(std::path::MAIN_SEPARATOR);
        let output_contains_error_msg = lowercase_output.contains("permission denied")
            || lowercase_output.contains("not an executable file");

        input_starts_with_dir && output_contains_error_msg
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let original_command = command.input_parts().to_vec();
        let script_name = original_command.first()?.to_owned();

        let new_command = vec!["chmod".to_owned(), "+x".to_owned(), script_name];

        Some(vec![RuleCorrection::and(
            new_command,
            command.input_parts(),
        )])
    }
}

#[cfg(test)]
// TODO: add better testing utilities to test rules in isolation.
// Below, we need to use `contains` to avoid equality tests because
// the `sudo` rule also surfaces corrections.
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_chmod_x() {
        assert_eq!(
            basic_corrections("./foo --flag", "zsh: permission denied: ./foo"),
            vec!["chmod +x ./foo && ./foo --flag", "sudo ./foo --flag"]
        );
    }

    #[test]
    fn test_chmod_x_fish() {
        assert_eq!(
            basic_corrections("./foo --flag", "fish: Unknown command. '/Users/user/dir/foo' exists but is not an executable file."),
            vec!["chmod +x ./foo && ./foo --flag"]
        );
    }

    #[test]
    fn test_chmod_x_when_not_in_same_dir() {
        assert_eq!(
            basic_corrections("../foo", "zsh: permission denied: ../foo"),
            vec!["chmod +x ../foo && ../foo", "sudo ../foo"]
        );
    }

    #[test]
    fn test_chmod_x_with_absolute_path() {
        assert_eq!(
            basic_corrections("/bin/foo", "zsh: permission denied: /bin/foo"),
            vec!["chmod +x /bin/foo && /bin/foo", "sudo /bin/foo"]
        );
    }
}
