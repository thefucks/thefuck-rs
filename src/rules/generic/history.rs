use crate::rules::util::get_single_closest_match;
use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use itertools::Itertools;

fn get_history_entries_excluding_command<'a>(
    command: &'_ Command,
    session_metadata: &'a SessionMetadata,
) -> Vec<&'a str> {
    session_metadata
        .history
        .iter()
        .copied()
        .filter(|c| c != &command.input)
        .collect_vec()
}

/// Suggest a command based on user's history
pub(crate) struct History;

impl Rule for History {
    default_rule_id!(History);

    fn matches(&self, _command: &Command, _session_metadata: &SessionMetadata) -> bool {
        // There's not much to test for here other than checking if the history has a similar command,
        // which we simply do in `generate_command_corrections` instead.
        true
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let history_entries = get_history_entries_excluding_command(command, session_metadata);
        let fix = get_single_closest_match(command.input, history_entries)?;
        Some(vec![fix.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_utils::regular_corrections, Command, SessionMetadata};
    const HISTORY: &[&str] = &["./super-script -f", "git checkout master"];

    #[test]
    fn test_executable_correction() {
        let command = Command::new("./superscript -f", "no such file or directory", 127.into());
        let mut metadata = SessionMetadata::new();
        metadata.set_history(HISTORY.iter().copied());

        assert_eq!(
            regular_corrections(command, &metadata),
            vec!["./super-script -f"]
        );
    }
}
