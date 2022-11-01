use crate::rules::python::{python_execute::PythonExecute, python_module_error::PythonModuleError};
use crate::rules::Rule;

use super::CommandGroup;

mod python_execute;
mod python_module_error;

pub(crate) fn command_group() -> CommandGroup {
    CommandGroup {
        command_names: &["python", "python3"],
        rules: vec![PythonExecute.to_arc(), PythonModuleError.to_arc()],
    }
}
