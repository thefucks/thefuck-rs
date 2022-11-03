use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

/*
Corrects a command that looks like "cargo" to "cargo build".
See more here: https://github.com/nvbn/thefuck/blob/5198b34f24ca4bc414a5bf1b0288ee86ea2529a8/thefuck/rules/cargo.py
*/
pub(crate) struct Cargo;
impl Rule for Cargo {
    default_rule_id!(NoCommand);

    fn should_be_considered_by_default(
        &self,
        _command: &Command,
        _session_metadata: &SessionMetadata,
    ) -> bool {
        // Without any subcommands, the cargo rule exits successfully,
        // but we still want to apply this rule.
        true
    }

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command.input == "cargo"
    }

    fn generate_command_corrections<'a>(
        &self,
        _command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        Some(vec![vec!["cargo", "build"].into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_cargo() {
        assert_eq!(basic_corrections("cargo", ""), vec!["cargo build"])
    }
}
