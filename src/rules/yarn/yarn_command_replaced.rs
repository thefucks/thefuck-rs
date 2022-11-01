use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new("(?i)Run \"(.*)\" instead").unwrap();
}

/// Corrects an obsolete yarn command
pub(crate) struct YarnCommandReplaced;
impl Rule for YarnCommandReplaced {
    default_rule_id!(YarnCommandReplaced);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        RE.is_match(command.output)
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let fix = RE
            .captures(command.output)
            .and_then(|captures| captures.get(1))
            .map(|regex_match| regex_match.as_str())?;

        Some(vec![fix.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_yarn_command_replaced() {
        assert_eq!(
            basic_corrections(
                "yarn install random",
                r#"error `install` has been replaced with `add` to add new dependencies. Run "yarn add random" instead.
                info Visit https://yarnpkg.com/en/docs/cli/install for documentation about this command."#
            ),
            vec!["yarn add random"]
        )
    }
}
