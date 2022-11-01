use std::path::Path;

use difflib::get_close_matches;

use crate::RuleCorrection;

// TODO: eventually make this configurable
/// The score here refers to the ratio used by difflib.
const MATCH_SCORE_CUTOFF: f32 = 0.6;

/// Returns new commands where the to_replace string in
/// input is replaced with the suggestions.
pub(crate) fn new_commands_from_suggestions<'a>(
    suggestions: impl IntoIterator<Item = &'a str>,
    input_parts: &[String],
    to_replace: &str,
) -> Option<Vec<RuleCorrection<'a>>> {
    let replacement = input_parts.to_vec();
    let replacement_index = input_parts.iter().position(|part| part == to_replace)?;

    Some(
        suggestions
            .into_iter()
            .filter_map(|cmd| {
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
    use crate::{Command, ExitCode};

    use super::{get_single_closest_match, is_file};

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
