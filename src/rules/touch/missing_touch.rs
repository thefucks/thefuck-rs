use std::path::Path;

use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new("(?i)touch: (.+):").unwrap();
}

/// Suggests to create a directory/directories before touch'ing file
pub(crate) struct MissingTouch;
impl Rule for MissingTouch {
    default_rule_id!(MissingTouch);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command
            .lowercase_output()
            .contains("no such file or directory")
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let path_str = RE
            .captures(command.output)
            .and_then(|captures| captures.get(1))
            .map(|regex_match| regex_match.as_str())?;
        let directory_path = Path::new(path_str).parent()?.to_str()?;

        let new_command = vec!["mkdir", "-p", directory_path];
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
    fn test_touch() {
        assert_eq!(
            basic_corrections(
                "touch random/foo",
                "touch: random/foo: No such file or directory"
            ),
            vec!["mkdir -p random && touch random/foo"]
        )
    }
}
