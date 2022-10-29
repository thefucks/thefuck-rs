use crate::rules::util::new_commands_from_suggestions;
use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

/// Replaces update with upgrade.
pub(crate) struct BrewUpdateUpgrade;
impl Rule for BrewUpdateUpgrade {
    default_rule_id!(BrewUpdateUpgrade);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command.input_parts().iter().any(|p| p == "update")
            && command
                .lowercase_output()
                .contains("this command updates brew itself")
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        new_commands_from_suggestions(["upgrade"], command.input_parts(), "update")
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_brew_update_upgrade() {
        assert_eq!(
            basic_corrections(
                "brew update jq",
                r#"Error: This command updates brew itself, and does not take formula names.
                Use `brew upgrade jq` instead."#
            ),
            vec!["brew upgrade jq"]
        )
    }
}
