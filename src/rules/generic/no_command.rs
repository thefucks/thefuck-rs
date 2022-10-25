use crate::rules::util::{get_single_closest_match, new_commands_from_suggestions};
use crate::rules::Rule;
use crate::{Command, Correction, SessionMetadata};
use itertools::Itertools;

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

    /// We want one match per group, where we have two groups
    /// - Group 1: executables, functions, aliases, builtins
    /// - Group 2: commands from history
    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<Correction<'a>>> {
        let to_fix = command.input_parts().first()?.as_str();

        // Get a match from the top level commands
        let top_level_commands = session_metadata.top_level_commands().collect_vec();
        let top_level_command_fix = get_single_closest_match(to_fix, top_level_commands);

        // Get a match from the shell history. But don't try to match against
        // things that aren't even top level commands.
        let history_commands = session_metadata
            .top_level_commands_from_history()
            .filter(|s| *s != to_fix && session_metadata.is_top_level_command(s))
            .collect_vec();
        let history_command_fix = get_single_closest_match(to_fix, history_commands);

        // Favor the history match over the top level command match. We use
        // the history as an ordering trick to suggest something that's more relevant
        // to the user.
        let suggestions = history_command_fix.into_iter().chain(top_level_command_fix);
        new_commands_from_suggestions(suggestions, command.input_parts(), to_fix)
    }
}

#[cfg(test)]
mod tests {
    use crate::{correct_command, Command, SessionMetadata};
    const EXECUTABLES: &[&str] = &["git", "cargo"];
    const ALIASES: &[&str] = &["foo", "bar", "gt"];
    const FUNCTIONS: &[&str] = &["func", "meth"];
    const BUILTINS: &[&str] = &["print"];
    const HISTORY: &[&str] = &["gitz random", "gtii cmd", "git cmd"];

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
    fn test_shell_builtins() {
        let command = Command::new("pirnt -f", "command not found", 127.into());
        let mut metadata = SessionMetadata::new();
        metadata.set_builtins(BUILTINS.iter().copied());

        assert_eq!(correct_command(command, &metadata), vec!["print -f"]);
    }

    #[test]
    fn test_history() {
        let command = Command::new("gits commit", "command not found", 127.into());
        let mut metadata = SessionMetadata::new();
        metadata.set_executables(EXECUTABLES.iter().copied());
        metadata.set_history(HISTORY.iter().copied());

        assert_eq!(correct_command(command, &metadata), vec!["git commit"]);
    }

    #[test]
    fn test_all() {
        let command = Command::new("gti commit", "command not found", 127.into());
        let mut metadata = SessionMetadata::new();
        metadata.set_executables(EXECUTABLES.iter().copied());
        metadata.set_aliases(ALIASES.iter().copied());
        metadata.set_functions(FUNCTIONS.iter().copied());
        metadata.set_builtins(BUILTINS.iter().copied());
        metadata.set_history(HISTORY.iter().copied());

        assert_eq!(
            correct_command(command, &metadata),
            vec!["git commit", "gt commit"]
        );
    }
}
