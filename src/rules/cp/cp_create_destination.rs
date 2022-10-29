use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new("(?i)directory (.+) does not exist").unwrap();
}

/// Corrects a cp command by creating the dest directory if it doesn't already exist.
pub(crate) struct CpCreateDestination;
impl Rule for CpCreateDestination {
    default_rule_id!(CpCreateDestination);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        RE.is_match(command.lowercase_output())
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let dirname = RE
            .captures(command.output)
            .and_then(|captures| captures.get(1))
            .map(|regex_match| regex_match.as_str())?;

        let new_command = vec!["mkdir", "-p", dirname];
        Some(vec![RuleCorrection::and(
            new_command,
            command.input_parts(),
        )])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_cp_create_destination() {
        assert_eq!(
            basic_corrections("cp foo bar/", "cp: directory bar does not exist"),
            vec!["mkdir -p bar && cp foo bar/"]
        )
    }
}
