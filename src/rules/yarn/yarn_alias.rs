use crate::rules::util::new_commands_from_suggestions;
use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref WRONG_COMMAND_RE: Regex = Regex::new("(?i)Command \"(.+)\" not found").unwrap();
    static ref DID_YOU_MEAN_RE: Regex = Regex::new("(?i)Did you mean \"(.+)\"").unwrap();
}

/// Suggests a correction based on yarn's "did you mean" suggestion
pub(crate) struct YarnAlias;
impl Rule for YarnAlias {
    default_rule_id!(YarnAlias);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command.lowercase_output().contains("did you mean")
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let wrong_command = WRONG_COMMAND_RE
            .captures(command.output)
            .and_then(|captures| captures.get(1))
            .map(|regex_match| regex_match.as_str())?;

        let fix = DID_YOU_MEAN_RE
            .captures(command.output)
            .and_then(|captures| captures.get(1))
            .map(|regex_match| regex_match.as_str())?;

        new_commands_from_suggestions([fix], command.input_parts(), wrong_command)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_yarn_alias() {
        assert_eq!(
            basic_corrections(
                "yarn run strt",
                r#"error Command "strt" not found. Did you mean "start"?
                info Visit https://yarnpkg.com/en/docs/cli/run for documentation about this command."#
            ),
            vec!["yarn run start"]
        )
    }
}
