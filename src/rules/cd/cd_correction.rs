use std::fs;
use std::path::{Component, Path};

use crate::rules::util::get_single_closest_match;
use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // TODO: handle -L/-P options
    static ref RE: Regex = Regex::new("cd (.+)").unwrap();
}

fn get_directory_names_at_path(path: &Path) -> Vec<String> {
    match fs::read_dir(path) {
        Ok(dir) => dir
            .into_iter()
            .filter_map(|dir_entry| {
                let path = dir_entry.ok()?.path();
                let file_name = path.file_name()?.to_str()?;
                path.is_dir().then(|| file_name.to_owned())
            })
            .collect(),
        _ => vec![],
    }
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

        // so_far represents what the corrected path looks like so far.
        // - If the command was looking for an absolute path, we have to search each part so we start off with nothing.
        // - If the command was looking for a relative path, it's relative to the working dir
        //   (which we know is a valid dir) so we start with that.
        // At any point, if we can't make progress, we don't generate any corrections
        // (and instead defer to the cd_mkdir rule).
        let mut so_far = if wrong_dirname.is_absolute() {
            Path::new("").to_owned()
        } else {
            Path::new(command.working_dir?).to_owned()
        };

        for part in wrong_dirname.components() {
            match part {
                Component::CurDir => {}
                Component::ParentDir => so_far = so_far.parent()?.to_owned(),
                Component::RootDir => so_far = Path::new("/").to_owned(),
                Component::Prefix(_) => { /* TODO: support windows */ }
                Component::Normal(p) => {
                    // If this part of the path is correct, just add it to so_far and move on
                    let path_with_part = so_far.join(p);
                    if path_with_part.exists() {
                        so_far = path_with_part;
                    } else {
                        // Otherwise, get the directories under so_far and find the
                        // closest matching one. Add the match to so_far and continue.
                        let dir_names = get_directory_names_at_path(so_far.as_path());
                        let dir_name_strs = dir_names.iter().map(|d| d.as_str()).collect();
                        let matches = get_single_closest_match(p.to_str()?, dir_name_strs)?;
                        so_far = so_far.join(matches);
                    }
                }
            }
        }

        // If the working dir is a prefix of the corrected path, then just trim it.
        // TODO: we can even get fancier and use `..` (at most k times) to get a relative path if possible.
        let correction = if wrong_dirname.is_relative() && so_far.starts_with(command.working_dir?)
        {
            so_far.strip_prefix(command.working_dir?).ok()?.to_str()?
        } else {
            so_far.to_str()?
        };

        Some(vec![vec!["cd".to_owned(), correction.to_owned()].into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_utils::regular_corrections, Command, ExitCode, SessionMetadata};
    use std::fs;
    use tempfile::{tempdir, TempDir};

    fn with_temp_directories(test: impl Fn(TempDir)) {
        let tmpdir = tempdir().unwrap();
        fs::create_dir_all(tmpdir.path().join("apples/bananas/oranges/mangos")).unwrap();
        fs::create_dir_all(tmpdir.path().join("apples/dir2/dir3/dir4")).unwrap();
        fs::create_dir_all(tmpdir.path().join("acrobat")).unwrap();

        test(tmpdir)
    }

    #[test]
    fn test_cd_correction_relative() {
        with_temp_directories(|tmpdir| {
            let command = Command::new(
                "cd aples/banannas/oranges/mans",
                "cd: no such file or directory: aples",
                ExitCode(1),
            )
            .set_working_dir(tmpdir.path().to_str().unwrap());

            assert!(regular_corrections(command, &SessionMetadata::default())
                .contains(&"cd apples/bananas/oranges/mangos".to_owned()))
        });
    }

    #[test]
    fn test_cd_correction_absolute() {
        with_temp_directories(|tmpdir| {
            let abs_path = tmpdir.path().to_str().unwrap();
            let cd_str = format!("cd {abs_path}/aples/banannas/ranges/mans");
            let command = Command::new(
                cd_str.as_str(),
                "cd: no such file or directory: aples",
                ExitCode(1),
            )
            .set_working_dir(tmpdir.path().to_str().unwrap());

            let corrections = regular_corrections(command, &SessionMetadata::default());
            assert!(corrections.contains(&format!("cd {abs_path}/apples/bananas/oranges/mangos")))
        });
    }

    #[test]
    fn test_cd_correction_with_curr_and_parent_references() {
        with_temp_directories(|tmpdir| {
            let command = Command::new(
                "cd aples/./banannas/../banango",
                "cd: no such file or directory: aples",
                ExitCode(1),
            )
            .set_working_dir(tmpdir.path().to_str().unwrap());

            assert!(regular_corrections(command, &SessionMetadata::default())
                .contains(&"cd apples/bananas".to_owned()))
        });
    }
}
