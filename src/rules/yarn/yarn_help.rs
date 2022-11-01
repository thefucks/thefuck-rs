use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new("(?i)Visit (.+) (to|for)").unwrap();
}

/// Suggests a command to open the yarn docs page if the user ran `yarn help`
pub(crate) struct YarnHelp;
impl Rule for YarnHelp {
    default_rule_id!(YarnHelp);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command.input_parts().get(1).map(|s| s.as_str()) == Some("help")
            && RE.is_match(command.lowercase_output())
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let url = RE
            .captures(command.output)
            .and_then(|captures| captures.get(1))
            .map(|regex_match| regex_match.as_str())?;

        let new_command = vec!["open", url];
        Some(vec![new_command.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_yarn_help() {
        assert_eq!(
            basic_corrections(
                "yarn help",
                "Visit https://yarnpkg.com/en/docs/cli/ to learn more about Yarn."
            ),
            vec!["open https://yarnpkg.com/en/docs/cli/"]
        )
    }

    #[test]
    fn test_yarn_help_subcommand() {
        assert_eq!(
            basic_corrections(
                "yarn help why",
                "Visit https://yarnpkg.com/en/docs/cli/why for documentation about this command."
            ),
            vec!["open https://yarnpkg.com/en/docs/cli/why"]
        )
    }
}
