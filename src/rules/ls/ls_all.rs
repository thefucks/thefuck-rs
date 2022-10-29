use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

/// This rule is to show hidden directories when ls doesn't have any output
pub(crate) struct LsAll;
impl Rule for LsAll {
    default_rule_id!(LsAll);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        // TODO: simplify flag parsing at a higher level
        // If the -a or -A flag was already supplied, we don't want to correct this
        // command since it's already attempting to show hidden directories
        !command
            .input_parts()
            .iter()
            .any(|p| p.starts_with('-') && (p.contains('a') || p.contains('A')))
            && command.output.is_empty()
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let mut replacement = command.input_parts().to_vec();
        replacement.insert(1, "-a".to_owned());

        Some(vec![replacement.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_ls_all() {
        assert_eq!(basic_corrections("ls -G", ""), vec!["ls -a -G"])
    }

    #[test]
    fn test_ls_all_with_hidden_dirs() {
        assert!(basic_corrections("ls -A -G", "").is_empty())
    }

    #[test]
    fn test_ls_all_with_complicated_flags() {
        assert!(basic_corrections("ls -GA", "").is_empty())
    }
}
