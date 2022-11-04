use std::path::Path;

use crate::rules::util::correct_path_at_every_level;
use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // TODO: handle -L/-P options
    static ref RE: Regex = Regex::new("cd (.+)").unwrap();
}

/// This rule corrects a cd command when the directory doesn't exist
pub(crate) struct CdCorrection;
impl Rule for CdCorrection {
    default_rule_id!(CdCorrection);

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
        let wrong_dirname = RE
            .captures(command.input)
            .and_then(|captures| captures.get(1))
            .map(|regex_match| Path::new(regex_match.as_str()))?;

        let corrected_path =
            correct_path_at_every_level(wrong_dirname, command.working_dir?, |path| path.is_dir())?;

        Some(vec![vec!["cd".to_owned(), corrected_path].into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::with_temp_directories;
    use crate::{test_utils::regular_corrections, Command, ExitCode, SessionMetadata};

    const SAMPLE_DIR_PATHS: &[&str] = &[
        "apples/bananas/oranges/mangos",
        "apples/dir2/dir3/dir4",
        "acrobat",
    ];

    #[test]
    fn test_cd_correction_relative() {
        with_temp_directories(SAMPLE_DIR_PATHS, |tempdir| {
            let command = Command::new(
                "cd aples/banannas/oranges/mans",
                "cd: no such file or directory: aples",
                ExitCode(1),
            )
            .set_working_dir(tempdir.path().to_str().unwrap());

            assert!(regular_corrections(command, &SessionMetadata::default())
                .contains(&"cd apples/bananas/oranges/mangos".to_owned()))
        });
    }
}
