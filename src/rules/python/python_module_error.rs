use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new("(?i)ModuleNotFoundError: No module named '(.+)'").unwrap();
}

/// Suggests to pip install a missing module
pub(crate) struct PythonModuleError;
impl Rule for PythonModuleError {
    default_rule_id!(PythonModuleError);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        RE.is_match(command.lowercase_output())
    }

    // TODO: this rule should be smarter in that it should first check if there is a similarly
    // named module before suggesting to install a whole new module.
    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let module_name = RE
            .captures(command.output)
            .and_then(|captures| captures.get(1))
            .map(|regex_match| regex_match.as_str())?;

        let new_command = vec!["pip", "install", module_name];
        Some(vec![RuleCorrection::and(
            new_command,
            command.input_parts(),
        )])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_python_module_error() {
        assert_eq!(
            basic_corrections(
                "python test.py",
                r#"Traceback (most recent call last):
            File "/Users/suraj/command-corrections/test.py", line 1, in <module>
              import numpy
            ModuleNotFoundError: No module named 'numpy'"#
            ),
            vec!["pip install numpy && python test.py"]
        )
    }
}
