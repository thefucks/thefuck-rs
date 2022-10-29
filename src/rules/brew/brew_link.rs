use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new("(?i)brew link --overwrite --dry-run (.+)").unwrap();
}

/// Corrects a brew command that requires overwriting links
pub(crate) struct BrewLink;
impl Rule for BrewLink {
    default_rule_id!(BrewLink);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command
            .input_parts()
            .iter()
            .any(|p| p == "ln" || p == "link")
            && command
                .lowercase_output()
                .contains("brew link --overwrite --dry-run")
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let link_index = command
            .input_parts()
            .iter()
            .position(|p| p == "ln" || p == "link")?;

        let mut replacement = command.input_parts().to_vec();
        replacement.insert(link_index + 1, "--overwrite".to_owned());
        replacement.insert(link_index + 2, "--dry-run".to_owned());

        Some(vec![replacement.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_brew_link() {
        assert_eq!(
            basic_corrections(
                "brew link kubernetes-cli",
                r#"To force the link and overwrite all conflicting files:
            brew link --overwrite kubernetes-cli
          
          To list all files that would be deleted:
            brew link --overwrite --dry-run kubernetes-cli"#
            ),
            vec!["brew link --overwrite --dry-run kubernetes-cli"]
        )
    }
}
