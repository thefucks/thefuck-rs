use crate::rules::Rule;
use crate::Command;
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

    fn generate_command_corrections(&self, command: &Command) -> Option<Vec<String>> {
        RE.captures(command.input)
            .and_then(|captures| captures.get(1))
            .map(|regex_match| vec![regex_match.as_str().to_owned()])
    }
}

#[cfg(test)]
mod tests {
    use crate::command_corrections;

    #[test]
    fn test_leading_shell_prompt() {
        assert_eq!(
            command_corrections("$ git status", "zsh: command not found: $"),
            vec!["git status"]
        );
    }
}
