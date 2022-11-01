use crate::rules::util::is_file;
use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

/// Corrects grep command by putting file name at the end
pub(crate) struct GrepArgumentsOrder;
impl Rule for GrepArgumentsOrder {
    default_rule_id!(GrepArgumentsOrder);

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
        let working_dir = command.working_dir?;
        let (filename_pos, filename) = command
            .input_parts()
            .iter()
            .enumerate()
            .find(|(_, part)| is_file(part, working_dir))?;

        // If the filename is already at the end, then don't try to re-position it.
        if filename_pos == (command.input_parts().len() - 1) {
            return None;
        }

        let mut new_command = command.input_parts().to_vec();
        new_command.remove(filename_pos);
        new_command.push(filename.to_owned());

        Some(vec![new_command.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_utils::regular_corrections, Command, ExitCode, SessionMetadata};
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_grep_arguments_order_with_existing_file() {
        let tempdir = tempdir().unwrap();
        fs::create_dir(tempdir.path().join("dir")).unwrap();

        let command = Command::new(
            "grep -r dir -A 5 query",
            "grep: query: No such file or directory",
            ExitCode(1),
        )
        .set_working_dir(tempdir.path().as_os_str().to_str().unwrap());
        assert_eq!(
            regular_corrections(command, &SessionMetadata::new()),
            vec!["grep -r -A 5 query dir"]
        )
    }

    #[test]
    fn test_grep_arguments_order_with_non_existant_file() {
        let tempdir = tempdir().unwrap();

        let command = Command::new(
            "grep -r dir query",
            "grep: query: No such file or directory",
            ExitCode(1),
        )
        .set_working_dir(tempdir.path().as_os_str().to_str().unwrap());

        assert!(regular_corrections(command, &SessionMetadata::new()).is_empty())
    }
}
