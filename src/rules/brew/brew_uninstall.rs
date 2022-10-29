use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

/// Suggests to force uninstall if regular uninstall didn't work.
pub(crate) struct BrewUninstall;
impl Rule for BrewUninstall {
    default_rule_id!(BrewUninstall);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command
            .input_parts()
            .iter()
            .any(|p| p == "uninstall" || p == "rm" || p == "remove")
            && command
                .lowercase_output()
                .contains("brew uninstall --force")
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let uninstall_pos = command
            .input_parts()
            .iter()
            .position(|p| p == "uninstall" || p == "rm" || p == "remove")?;
        let mut replacement = command.input_parts().to_vec();
        replacement.insert(uninstall_pos + 1, "--force".to_owned());
        Some(vec![replacement.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_brew_uninstall() {
        assert_eq!(
            basic_corrections("brew rm jq", "brew uninstall --force"),
            vec!["brew rm --force jq"]
        )
    }
}
