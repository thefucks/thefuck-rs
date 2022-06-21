use crate::rules::Rule;
use crate::Command;

pub(crate) struct Repetition;
impl Rule for Repetition {
    // TODO: once Command carries contextual info (like exit codes), we should
    // only run this rule if the command failed (since there could be commands
    // with subcommands that have the same name as the top-level command).
    fn matches(&self, command: &Command) -> bool {
        let input_parts = command.input_parts();
        if let (Some(first_part), Some(second_part)) = (input_parts.get(0), input_parts.get(1)) {
            return first_part == second_part;
        }
        false
    }

    fn generate_command_corrections(&self, command: &Command) -> Option<Vec<String>> {
        if command.input_parts().len() < 2 {
            None
        } else {
            Some(vec![command.input_parts()[1..].join(" ")])
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::command_corrections;

    #[test]
    fn test_repetitions() {
        assert_eq!(
            command_corrections("git git status", "some random error"),
            vec!["git status"]
        )
    }

    #[test]
    fn test_repetitions_with_one_part() {
        let empty_corrections: Vec<String> = Vec::new();
        assert_eq!(
            command_corrections("git", "some random error"),
            empty_corrections
        )
    }
}
