use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // TODO: handle -L/-P options
    static ref RE: Regex = Regex::new("cd (.+)").unwrap();
}

/// This rule corrects a cd command when the directory doesn't exist
pub(crate) struct CdMkdir;
impl Rule for CdMkdir {
    default_rule_id!(CdMkdir);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        // TODO: eventually, use a callback to just check if the directory exists
        let lowercase_output = command.lowercase_output();
        lowercase_output.contains("does not exist")
            || lowercase_output.contains("no such file or directory")
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let dirname = RE
            .captures(command.input)
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
    fn test_cd_mkdir_for_bash_and_zsh() {
        assert_eq!(
            basic_corrections("cd app", "cd: no such file or directory: app"),
            vec!["mkdir -p app && cd app"]
        )
    }

    #[test]
    fn test_cd_mkdir_for_fish() {
        assert_eq!(
            basic_corrections("cd app", "cd: The directory 'app' does not exist"),
            vec!["mkdir -p app && cd app"]
        )
    }
}
