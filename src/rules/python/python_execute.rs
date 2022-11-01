use crate::rules::util::is_file;
use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new("(?i)can't open file '(.+)'").unwrap();
}

/// Adds .py extension to command if missing
pub(crate) struct PythonExecute;
impl Rule for PythonExecute {
    default_rule_id!(PythonExecute);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        let filename = RE
            .captures(command.output)
            .and_then(|captures| captures.get(1))
            .map(|regex_match| regex_match.as_str());
        filename.map_or(false, |f| !f.ends_with(".py"))
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let captured_filename = RE
            .captures(command.output)
            .and_then(|captures| captures.get(1))
            .map(|regex_match| regex_match.as_str())?;

        // Note: we specifically check for `ends_with` here in case the prefix was stripped off. See tests below
        let (wrong_filename_pos, wrong_filename) = command
            .input_parts()
            .iter()
            .enumerate()
            .find(|(_idx, p)| captured_filename.ends_with(p.as_str()))?;
        let new_filename = format!("{wrong_filename}.py");

        // If this isn't even an existing file, then don't suggest this correction
        if !is_file(new_filename.as_str(), command.working_dir?) {
            return None;
        }

        let mut new_command = command.input_parts().to_vec();
        *new_command.get_mut(wrong_filename_pos)? = new_filename;
        Some(vec![new_command.into()])
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use crate::{test_utils::regular_corrections, Command, ExitCode, SessionMetadata};

    #[test]
    fn test_python_execute() {
        let tempdir = tempdir().unwrap();
        fs::File::create(tempdir.path().join("test.py")).unwrap();

        let command = Command::new(
            "python test -d",
            "python: can't open file 'test': [Errno 2] No such file or directory",
            ExitCode(1),
        )
        .set_working_dir(tempdir.path().to_str().unwrap());

        assert_eq!(
            regular_corrections(command, &SessionMetadata::new()),
            vec!["python test.py -d"]
        )
    }

    #[test]
    fn test_python_execute_long_path() {
        let tempdir = tempdir().unwrap();
        fs::File::create(tempdir.path().join("test.py")).unwrap();

        let output = format!(
            "python: can't open file '{}/test': [Errno 2] No such file or directory",
            tempdir.path().to_str().unwrap()
        );

        let command = Command::new("python test -d", &output, ExitCode(1))
            .set_working_dir(tempdir.path().to_str().unwrap());

        assert_eq!(
            regular_corrections(command, &SessionMetadata::new()),
            vec!["python test.py -d"]
        )
    }

    #[test]
    fn test_python_execute_still_not_a_file() {
        let tempdir = tempdir().unwrap();

        let command = Command::new(
            "python test -d",
            "python: can't open file 'test': [Errno 2] No such file or directory",
            ExitCode(1),
        )
        .set_working_dir(tempdir.path().to_str().unwrap());

        assert!(regular_corrections(command, &SessionMetadata::new()).is_empty())
    }
}
