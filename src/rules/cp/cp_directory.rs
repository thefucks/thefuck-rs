use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

/// Corrects a command to properly cp a directory
pub(crate) struct CpDirectory;
impl Rule for CpDirectory {
    default_rule_id!(CpDirectory);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        let lowercase_output = command.lowercase_output();
        lowercase_output.contains("omitting directory")
            || lowercase_output.contains("is a directory")
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let mut new_command = command.input_parts().to_vec();
        new_command.insert(1, "-a".to_owned());
        Some(vec![new_command.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_cp_omitting_directory() {
        assert_eq!(
            basic_corrections("cp old_dir new_dir", "cp: src is a directory (not copied)."),
            vec!["cp -a old_dir new_dir"]
        )
    }
}
