use crate::rules::Rule;
use crate::{Command, Correction, SessionMetadata};
use difflib::get_close_matches;
use itertools::Itertools;

// TODO: eventually make this configurable
/// The score here refers to the ratio used by difflib.
const MATCH_SCORE_CUTOFF: f32 = 0.6;

/// We only want one match per group, where we have two groups
/// - Group 1: executables, functions, aliases, builtins
/// - Group 2: commands from history
const NUM_MATCHES_DESIRED_PER_GROUP: usize = 1;

/// The NoCommand rule is meant to address failures when the first word
/// in the command is not recognized by the shell.
pub(crate) struct NoCommand;
impl Rule for NoCommand {
    fn matches(&self, command: &Command, session_metadata: &SessionMetadata) -> bool {
        // TODO: use a execute_command callback to just check `which`
        // Checking output is too brittle since it'll vary from shell to shell
        command.input_parts().first().map_or(false, |command_name| {
            command.exit_code.raw() == 127
                && !session_metadata.is_top_level_command(command_name.as_str())
        })
    }

    // TODO: also use history and shell builtins once they're available
    // in session metadata
    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<Correction<'a>>> {
        let to_fix = command.input_parts().first()?.as_str();
        let words = session_metadata
            .aliases
            .iter()
            .chain(session_metadata.executables.iter())
            .chain(session_metadata.functions.iter())
            .copied()
            .collect_vec();

        let matches = get_close_matches(
            to_fix,
            words,
            NUM_MATCHES_DESIRED_PER_GROUP,
            MATCH_SCORE_CUTOFF,
        );
        let fix = matches.first()?;

        let mut replacement = command.input_parts().to_vec();
        *replacement.first_mut()? = fix.to_string();
        Some(vec![replacement.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::{correct_command, Command, SessionMetadata};
    const EXECUTABLES: &[&str] = &["git", "cargo"];
    const ALIASES: &[&str] = &["foo", "bar", "gt"];
    const FUNCTIONS: &[&str] = &["func", "meth"];

    #[test]
    fn test_executable_correction() {
        let command = Command::new("gitt checkout", "command not found", 127.into());
        let mut metadata = SessionMetadata::new();
        metadata.set_executables(EXECUTABLES.iter().copied());

        assert_eq!(correct_command(command, &metadata), vec!["git checkout"]);
    }

    #[test]
    fn test_alias_correction() {
        let command = Command::new("fob access", "command not found", 127.into());
        let mut metadata = SessionMetadata::new();
        metadata.set_aliases(ALIASES.iter().copied());

        assert_eq!(correct_command(command, &metadata), vec!["foo access"]);
    }

    #[test]
    fn test_function_correction() {
        let command = Command::new("funky call", "command not found", 127.into());
        let mut metadata = SessionMetadata::new();
        metadata.set_functions(FUNCTIONS.iter().copied());

        assert_eq!(correct_command(command, &metadata), vec!["func call"]);
    }

    #[test]
    fn test_all() {
        let command = Command::new("gti commit", "command not found", 127.into());
        let mut metadata = SessionMetadata::new();
        metadata.set_executables(EXECUTABLES.iter().copied());
        metadata.set_aliases(ALIASES.iter().copied());
        metadata.set_functions(FUNCTIONS.iter().copied());

        assert_eq!(correct_command(command, &metadata), vec!["gt commit"]);
    }
}
