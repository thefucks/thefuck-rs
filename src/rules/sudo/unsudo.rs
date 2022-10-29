use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

/// Removes sudo from a command if not allowed to run with escalated priviledges
pub(crate) struct Unsudo;
impl Rule for Unsudo {
    default_rule_id!(Unsudo);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command
            .lowercase_output()
            .contains("you cannot perform this operation as root")
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let new_command = command.input_parts().get(1..)?;
        Some(vec![new_command.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_unsudo() {
        assert_eq!(
            basic_corrections("sudo ls", "you cannot perform this operation as root"),
            vec!["ls"]
        )
    }
}
