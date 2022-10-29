use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

/// Corrects a sed command when it has an expression like "s/from/to" (missing final slash)
pub(crate) struct SedUnterminatedS;
impl Rule for SedUnterminatedS {
    default_rule_id!(SedUnterminatedS);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command.lowercase_output().contains("unterminated")
    }

    // TODO: the quoting for this rule should be improved. Specifically,
    // the input part should have the original quotes around it (seems like they're lost at this point)
    // and the correction should have the pattern quoted.
    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let mut new_command = command.input_parts().to_vec();
        let s_string_pos = new_command
            .iter()
            .position(|p| (p.starts_with("s/") || p.starts_with("-es/")) && !p.ends_with('/'))?;
        new_command.get_mut(s_string_pos)?.push('/');

        Some(vec![new_command.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_sed_unterminated() {
        assert_eq!(
            basic_corrections(
                "sed 's/e/d' file.txt",
                r#"sed: 1: "s/e/d": unterminated substitute in regular expression"#
            ),
            vec!["sed s/e/d/ file.txt"]
        )
    }

    #[test]
    fn test_sed_terminated() {
        // this test is contrived since the output isn't actually possible given this input
        assert!(basic_corrections(
            "sed 's/e/d/' file.txt",
            r#"sed: 1: "s/e/d/": unterminated substitute in regular expression"#
        )
        .is_empty())
    }

    #[test]
    fn test_sed_escaped() {
        assert_eq!(
            basic_corrections(
                "sed 's/e f/d' file.txt",
                r#"sed: 1: "s/e f/d/": unterminated substitute in regular expression"#
            ),
            vec!["sed \"s/e f/d/\" file.txt"]
        )
    }
}
