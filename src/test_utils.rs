use crate::{correct_command, Command, ExitCode, SessionMetadata};

/// Doesn't make use of any session metadata.
pub fn basic_corrections(input: &str, output: &str) -> Vec<String> {
    let metadata = SessionMetadata::new();
    let command = Command::new(input, output, ExitCode(0));
    correct_command(command, &metadata)
}
