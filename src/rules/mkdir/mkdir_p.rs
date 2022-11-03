use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

/// Suggests to run mkdir -p if missing intermediate directories
pub(crate) struct MkdirP;
impl Rule for MkdirP {
    default_rule_id!(MkdirP);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command
            .lowercase_output()
            .contains("no such file or directory")
            && !command.input_parts().contains(&"-p".to_owned())
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let mut new_command = command.input_parts().to_vec();
        new_command.insert(1, "-p".to_owned());
        Some(vec![new_command.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_mkdir_p() {
        assert_eq!(
            basic_corrections("mkdir foo/bar", "mkdir: foo: No such file or directory"),
            vec!["mkdir -p foo/bar"]
        )
    }

    #[test]
    fn test_mkdir_p_flag_already_exists() {
        assert!(
            basic_corrections("mkdir -p foo/bar", "mkdir: foo: No such file or directory")
                .is_empty()
        )
    }
}
