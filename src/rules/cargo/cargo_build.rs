use crate::rules::Rule;
use crate::Command;

/*
Corrects a command that looks like "cargo" to "cargo build".
See more here: https://github.com/nvbn/thefuck/blob/5198b34f24ca4bc414a5bf1b0288ee86ea2529a8/thefuck/rules/cargo.py
*/
pub(crate) struct Cargo;
impl Rule for Cargo {
    fn matches(&self, command: &Command) -> bool {
        command.input == "cargo"
    }

    fn generate_command_corrections(&self, _command: &Command) -> Option<Vec<String>> {
        Some(vec!["cargo build".into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::command_corrections;

    #[test]
    fn test_cargo() {
        assert_eq!(command_corrections("cargo", ""), vec!["cargo build"])
    }
}
