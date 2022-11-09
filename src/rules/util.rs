use std::borrow::Cow;
use std::convert::AsRef;
use std::fs;
use std::path::{Component, Path};

use difflib::get_close_matches;

use crate::RuleCorrection;

// TODO: eventually make this configurable
/// The score here refers to the ratio used by difflib.
const MATCH_SCORE_CUTOFF: f32 = 0.6;

/// Returns new commands where the to_replace string in
/// input is replaced with the suggestions.
pub(crate) fn new_commands_from_suggestions<'a>(
    suggestions: impl IntoIterator<Item = impl Into<Cow<'a, str>>>,
    input_parts: &[String],
    to_replace: &str,
) -> Option<Vec<RuleCorrection<'a>>> {
    let replacement = input_parts.to_vec();
    let replacement_index = input_parts.iter().position(|part| part == to_replace)?;

    Some(
        suggestions
            .into_iter()
            .filter_map(|cmd| {
                let cmd = cmd.into();
                let cmd = cmd.trim();
                if !cmd.is_empty() {
                    let mut new_command = replacement.clone();
                    *new_command.get_mut(replacement_index)? = cmd.to_owned();
                    Some(new_command.into())
                } else {
                    None
                }
            })
            .collect(),
    )
}

// TODO: the only reason possiblities is a Vec here is because
// difflib::get_close_matches takes in a vec instead of an iterator.
// This should also take an iter eventually.
pub fn get_single_closest_match<'a>(to_match: &str, possiblities: Vec<&'a str>) -> Option<&'a str> {
    get_close_matches(to_match, possiblities, 1, MATCH_SCORE_CUTOFF)
        .first()
        .copied()
}

fn get_files_at_path_with_filter_at_level<F>(path: &Path, filter_at_level: F) -> Vec<String>
where
    F: Fn(&Path) -> bool,
{
    // TODO: this assumes that we're reading a local session. Eventually,
    // we should be using a callback provided by the client to read this data.
    match fs::read_dir(path) {
        Ok(dir) => dir
            .into_iter()
            .filter_map(|dir_entry| {
                let path = dir_entry.ok()?.path();
                let file_name = path.file_name()?.to_str()?;
                filter_at_level(&path).then(|| file_name.to_owned())
            })
            .collect(),
        _ => vec![],
    }
}

pub fn correct_path_at_every_level<F>(
    path_to_correct: impl AsRef<Path>,
    working_dir: impl AsRef<Path>,
    filter_at_level: F,
) -> Option<String>
where
    F: Fn(&Path) -> bool,
{
    let path_to_correct = path_to_correct.as_ref();
    let working_dir = working_dir.as_ref();

    // so_far represents what the corrected path looks like so far.
    // - If the command was looking for an absolute path, we have to search each part so we start off with nothing.
    // - If the command was looking for a relative path, it's relative to the working dir
    //   (which we know is a valid dir) so we start with that.
    // At any point, if we can't make progress, we don't generate any corrections
    // (and instead defer to the cd_mkdir rule).
    let mut so_far = if path_to_correct.is_absolute() {
        Path::new("").to_owned()
    } else {
        Path::new(working_dir).to_owned()
    };

    for part in path_to_correct.components() {
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
                    let dir_names =
                        get_files_at_path_with_filter_at_level(so_far.as_path(), &filter_at_level);
                    let dir_name_strs = dir_names.iter().map(|d| d.as_str()).collect();
                    let matches = get_single_closest_match(p.to_str()?, dir_name_strs)?;
                    so_far = so_far.join(matches);
                }
            }
        }
    }

    // If the working dir is a prefix of the corrected path, then just trim it.
    // TODO: we can even get fancier and use `..` (at most k times) to get a relative path if possible.
    let correction = if path_to_correct.is_relative() && so_far.starts_with(working_dir) {
        so_far.strip_prefix(working_dir).ok()?.to_str()?
    } else {
        so_far.to_str()?
    };

    Some(correction.to_owned())
}

pub fn is_file(filename: impl AsRef<Path>, working_dir: impl AsRef<Path>) -> bool {
    let filename_path = Path::new(filename.as_ref());
    if filename_path.is_absolute() {
        filename_path.exists()
    } else {
        let mut working_dir_path = Path::new(working_dir.as_ref()).to_owned();
        working_dir_path.push(filename);
        working_dir_path.exists()
    }
}

#[cfg(test)]
mod test {
    use std::fs;
    use tempfile::tempdir;

    use crate::rules::util::new_commands_from_suggestions;
    use crate::test_utils::with_temp_directories;
    use crate::{Command, ExitCode};

    use super::correct_path_at_every_level;

    use super::{get_single_closest_match, is_file};

    const SAMPLE_DIR_PATHS: &[&str] = &[
        "apples/bananas/oranges/mangos",
        "apples/dir2/dir3/dir4",
        "acrobat",
    ];

    #[test]
    fn test_correct_path_at_every_level_relative() {
        with_temp_directories(SAMPLE_DIR_PATHS, |tmpdir| {
            let result = correct_path_at_every_level(
                "aples/banannas/oranges/mans",
                tmpdir.path().to_str().unwrap(),
                |path| path.is_dir(),
            );

            assert_eq!(result.unwrap(), "apples/bananas/oranges/mangos");
        });
    }

    #[test]
    fn test_correct_path_at_every_level_absolute() {
        with_temp_directories(SAMPLE_DIR_PATHS, |tmpdir| {
            let abs_path = tmpdir.path().to_str().unwrap();
            let result = correct_path_at_every_level(
                abs_path.to_owned() + "/aples/banannas/ranges/mans",
                tmpdir.path().to_str().unwrap(),
                |path| path.is_dir(),
            );

            assert_eq!(
                result.unwrap(),
                abs_path.to_owned() + "/apples/bananas/oranges/mangos"
            );
        });
    }

    #[test]
    fn test_correct_path_at_every_level_with_curr_and_parent_references() {
        with_temp_directories(SAMPLE_DIR_PATHS, |tmpdir| {
            let result = correct_path_at_every_level(
                "aples/./banannas/../banango",
                tmpdir.path().to_str().unwrap(),
                |path| path.is_dir(),
            );

            assert_eq!(result.unwrap(), "apples/bananas");
        });
    }

    #[test]
    fn test_new_commands_from_suggestions() {
        let command = Command::new("git p", "bogus", ExitCode(0));
        let suggestions = ["push", "pull"];
        let corrections = new_commands_from_suggestions(suggestions, command.input_parts(), "p");
        assert_eq!(
            corrections,
            Some(vec![vec!["git", "push"].into(), vec!["git", "pull"].into()])
        );
    }

    #[test]
    fn test_new_commands_from_suggestions_with_none_to_replace() {
        let command = Command::new("git p", "bogus", ExitCode(0));
        let suggestions = ["push", "pull"];
        assert_eq!(
            new_commands_from_suggestions(suggestions, command.input_parts(), "w"),
            None
        );
    }

    #[test]
    fn test_get_single_closest_match() {
        let to_match = "poll";
        let possiblities = vec!["pull", "pole", "random"];
        assert_eq!(
            get_single_closest_match(to_match, possiblities),
            Some("pull")
        )
    }

    #[test]
    fn test_get_single_closest_match_no_match() {
        let to_match = "abc";
        let possiblities = vec!["pull", "pole", "random"];
        assert_eq!(get_single_closest_match(to_match, possiblities), None)
    }

    #[test]
    fn is_file_with_simple_file() {
        let tempdir = tempdir().unwrap();
        fs::File::create(tempdir.path().join("file")).unwrap();

        assert!(is_file(tempdir.path().join("file"), tempdir.path()))
    }

    #[test]
    fn is_file_with_dir() {
        let tempdir = tempdir().unwrap();
        fs::create_dir(tempdir.path().join("dir")).unwrap();

        assert!(is_file(tempdir.path().join("dir"), tempdir.path()))
    }

    #[test]
    fn is_file_with_nonexistant() {
        let tempdir = tempdir().unwrap();

        assert!(!is_file(tempdir.path().join("dir"), tempdir.path()))
    }
}
