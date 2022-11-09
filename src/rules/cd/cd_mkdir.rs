use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

use super::matches_cd_doesnt_exist;

lazy_static! {
    // TODO: handle -L/-P options
    static ref RE: Regex = Regex::new("cd (.+)").unwrap();
}

/// This rule corrects a cd command when the directory doesn't exist
pub(crate) struct CdMkdir;
impl Rule for CdMkdir {
    default_rule_id!(CdMkdir);

    fn matches(&self, command: &Command, session_metadata: &SessionMetadata) -> bool {
        matches_cd_doesnt_exist(command, session_metadata)
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
    use crate::{
        test_utils::{basic_corrections, regular_corrections},
        Command, ExitCode, SessionMetadata, SessionType,
    };

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

    #[test]
    fn test_cd_mkdir_with_remote_session() {
        let command = Command::new("cd app", "cd: no such file or directory: app", ExitCode(1));
        let mut session = SessionMetadata::default();
        session.set_session_type(SessionType::Remote);
        assert!(regular_corrections(command, &session).is_empty())
    }
}
