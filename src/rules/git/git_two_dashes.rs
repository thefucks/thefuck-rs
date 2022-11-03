use crate::rules::util::new_commands_from_suggestions;
use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new("(?i)did you mean `(.+)` (with two dashes)?").unwrap();
}

/// Corrects git commands that only use a single dash
pub(crate) struct GitTwoDashes;
impl Rule for GitTwoDashes {
    default_rule_id!(GitTwoDashes);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        RE.is_match(command.lowercase_output())
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

        let to_replace = command
            .input_parts()
            .iter()
            .find(|s| fix.ends_with(s.as_str()))?;
        new_commands_from_suggestions([fix], command.input_parts(), to_replace)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_git_two_dashes() {
        assert_eq!(
            basic_corrections(
                "git commit -amend",
                "error: did you mean `--amend` (with two dashes)?"
            ),
            vec!["git commit --amend"]
        )
    }
}
