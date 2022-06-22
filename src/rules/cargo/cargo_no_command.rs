use crate::rules::Rule;
use crate::Command;
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
    fn matches(&self, command: &Command) -> bool {
        command.lowercase_output().contains("no such subcommand") && RE.is_match(command.output)
    }

    fn generate_command_corrections(&self, command: &Command) -> Option<Vec<String>> {
        let fix = RE
            .captures(command.output)
            .and_then(|captures| captures.get(1))
            .map(|regex_match| regex_match.as_str())?;

        let mut replacement = command.input_parts().to_vec();
        *replacement.get_mut(1)? = fix.to_owned();

        Some(vec![replacement.join(" ")])
    }
}

#[cfg(test)]
mod tests {
    use crate::command_corrections;

    #[test]
    fn test_cargo_incorrect_command() {
        assert_eq!(
            command_corrections(
                "cargo buildd",
                "error: no such subcommand: `buildd`

	Did you mean `build`?"
            ),
            vec!["cargo build"]
        )
    }
}
