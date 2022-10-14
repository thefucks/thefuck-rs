use crate::{Command, CommandCorrector, ExitCode};

/// Doesn't make use of any session metadata.
pub fn basic_corrections(input: &str, output: &str) -> Vec<String> {
    let corrector = CommandCorrector::new();
    let command = Command::new(input, output, ExitCode(0));
    corrector.correct_command(command)
}
