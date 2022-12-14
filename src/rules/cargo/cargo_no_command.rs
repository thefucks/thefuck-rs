use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new("(?i)Did you mean `([^`]*)").unwrap();
}

/*
Corrects a misspelled `cargo *` command based on the `Did you mean` recommendations from cargo.
See more here: https://github.com/nvbn/thefuck/blob/5198b34f24ca4bc414a5bf1b0288ee86ea2529a8/thefuck/rules/cargo_no_command.py
*/
pub(crate) struct CargoNoCommand;
impl Rule for CargoNoCommand {
    default_rule_id!(CargoNoCommand);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command.lowercase_output().contains("no such subcommand") && RE.is_match(command.output)
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let fix = RE
            .captures(command.output)
            .and_then(|captures| captures.get(1))
            .map(|regex_match| regex_match.as_str())?;

        let mut replacement = command.input_parts().to_vec();
        *replacement.get_mut(1)? = fix.to_owned();

        Some(vec![replacement.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_cargo_incorrect_command() {
        assert_eq!(
            basic_corrections(
                "cargo buildd",
                "error: no such subcommand: `buildd`

        Did you mean `build`?"
            ),
            vec!["cargo build"]
        )
    }
}
