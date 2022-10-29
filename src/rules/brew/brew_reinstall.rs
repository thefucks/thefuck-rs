use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

/// Suggests a reinstall command if CLI is already installed.
pub(crate) struct BrewReinstall;
impl Rule for BrewReinstall {
    default_rule_id!(BrewReinstall);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command.input_parts().iter().any(|p| p == "install")
            && command
                .lowercase_output()
                .contains("is already installed and up-to-date")
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let install_pos = command.input_parts().iter().position(|p| p == "install")?;
        let mut replacement = command.input_parts().to_vec();
        *replacement.get_mut(install_pos)? = "reinstall".to_owned();
        Some(vec![replacement.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_brew_reinstall() {
        assert_eq!(
            basic_corrections(
                "brew install jq",
                r#"Warning: jq 1.6 is already installed and up-to-date.
                To reinstall 1.6, run:
                  brew reinstall jq"#
            ),
            vec!["brew reinstall jq"]
        )
    }
}
