use crate::rules::Rule;
use crate::{Command, Correction, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new("[\\s]*\\$[\\s]*(\\S.*)").unwrap();
}

/*
Fixes error for commands that begin with the shell prompt '$'.
This happens when copy-pasta'ing code-blocks in documentation.
See more here: https://github.com/nvbn/thefuck/blob/5198b34f24ca4bc414a5bf1b0288ee86ea2529a8/thefuck/rules/remove_shell_prompt_literal.py#L1
*/
pub(crate) struct LeadingShellPrompt;
impl Rule for LeadingShellPrompt {
    fn matches(&self, command: &Command) -> bool {
        command.output.to_lowercase().contains("command not found") && RE.is_match(command.input)
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<Correction<'a>>> {
        Some(vec![command.input_parts()[1..].into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_leading_shell_prompt() {
        assert_eq!(
            basic_corrections("$ git status", "zsh: command not found: $"),
            vec!["git status"]
        );
    }
}
