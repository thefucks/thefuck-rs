use crate::rules::util::{correct_path_at_every_level, new_commands_from_suggestions};
use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new("pathspec '(.+)' did not match any file").unwrap();
}

/// This rule corrects a git add command when the file isn't found
pub(crate) struct GitAdd;
impl Rule for GitAdd {
    default_rule_id!(GitAdd);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command.input_parts().get(1).map_or(false, |p| p == "add")
            && RE.is_match(command.lowercase_output())
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let wrong_filename = RE
            .captures(command.lowercase_output())
            .and_then(|captures| captures.get(1))
            .map(|regex_match| regex_match.as_str())?;

        let corrected_path =
            correct_path_at_every_level(wrong_filename, command.working_dir?, |_| true)?;

        new_commands_from_suggestions([corrected_path], command.input_parts(), wrong_filename)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{
        test_utils::{regular_corrections, with_temp_directories},
        Command, ExitCode, SessionMetadata,
    };

    #[test]
    fn test_git_add() {
        with_temp_directories(&["dir"], |tmpdir| {
            fs::File::create(tmpdir.path().join("dir/random.rs")).unwrap();
            let command = Command::new(
                "git add -- dir/randm.rs",
                "fatal: pathspec 'dir/randm.rs' did not match any files",
                ExitCode(1),
            )
            .set_working_dir(tmpdir.path().to_str().unwrap());

            assert_eq!(
                regular_corrections(command, &SessionMetadata::default()),
                vec!["git add -- dir/random.rs"]
            )
        });
    }
}
