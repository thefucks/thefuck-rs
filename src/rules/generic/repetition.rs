use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

/*
Fixes error for commands that accidentally repeat the top-level command, e.g. "git git status".
See more here: https://github.com/nvbn/thefuck/blob/5198b34f24ca4bc414a5bf1b0288ee86ea2529a8/thefuck/rules/dry.py
*/
pub(crate) struct Repetition;
impl Rule for Repetition {
    default_rule_id!(Repetition);

    // TODO: once Command carries contextual info (like exit codes), we should
    // only run this rule if the command failed (since there could be commands
    // with subcommands that have the same name as the top-level command).
    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        let input_parts = command.input_parts();
        if let (Some(first_part), Some(second_part)) = (input_parts.get(0), input_parts.get(1)) {
            return first_part == second_part;
        }
        false
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        if command.input_parts().len() < 2 {
            None
        } else {
            Some(vec![command.input_parts()[1..].into()])
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_repetitions() {
        assert_eq!(
            basic_corrections("git git status", "some random error"),
            vec!["git status"]
        )
    }

    #[test]
    fn test_repetitions_with_one_part() {
        let empty_corrections: Vec<String> = Vec::new();
        assert_eq!(
            basic_corrections("git", "some random error"),
            empty_corrections
        )
    }
}
