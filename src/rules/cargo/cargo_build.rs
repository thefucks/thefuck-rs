use crate::rules::Rule;
use crate::Command;

pub(crate) struct Cargo;
impl Rule for Cargo {
    fn for_commands(&self) -> Vec<&'static str> {
        vec!["cargo"]
    }

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
