use crate::rules::Rule;
use crate::Command;
use lazy_static::lazy_static;
use regex::Regex;

pub(crate) struct CargoNoCommand;

lazy_static! {
    static ref RE: Regex = Regex::new("(?i)Did you mean `([^`]*)").unwrap();
}

impl Rule for CargoNoCommand {
    fn for_commands(&self) -> Vec<&'static str> {
        vec!["cargo"]
    }

    fn matches(&self, command: &Command) -> bool {
        command.output.to_lowercase().contains("no such subcommand") && RE.is_match(command.output)
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
