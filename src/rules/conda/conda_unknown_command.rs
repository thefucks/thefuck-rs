use crate::rules::util::new_commands_from_suggestions;
use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref WRONG_COMMAND_RE: Regex = Regex::new("(?i)No command 'conda (.+)'").unwrap();
    static ref CORRECT_COMMAND_RE: Regex = Regex::new("(?i)Did you mean 'conda (.+)'").unwrap();
}

/// Corrects a misspelled conda command
pub(crate) struct CondaUnknownCommand;
impl Rule for CondaUnknownCommand {
    default_rule_id!(CondaUnknownCommand);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        let lowercase_output = command.lowercase_output();
        WRONG_COMMAND_RE.is_match(lowercase_output) && CORRECT_COMMAND_RE.is_match(lowercase_output)
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let lowercase_output = command.lowercase_output();

        let wrong_command = WRONG_COMMAND_RE
            .captures(lowercase_output)
            .and_then(|captures| captures.get(1))
            .map(|regex_match| regex_match.as_str())?;

        let correct_command = CORRECT_COMMAND_RE
            .captures(lowercase_output)
            .and_then(|captures| captures.get(1))
            .map(|regex_match| regex_match.as_str())?;

        new_commands_from_suggestions([correct_command], command.input_parts(), wrong_command)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_conda_unknown_command() {
        assert_eq!(
            basic_corrections(
                "conda cln --force",
                "CommandNotFoundError: No command 'conda cln'.
                Did you mean 'conda clean'?"
            ),
            vec!["conda clean --force"]
        )
    }
}
