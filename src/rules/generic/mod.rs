use crate::rules::generic::{
    leading_shell_prompt::LeadingShellPrompt, no_command::NoCommand, repetition::Repetition,
    sudo::Sudo,
};
use crate::rules::Rule;
use std::sync::Arc;

mod leading_shell_prompt;
mod no_command;
mod repetition;
mod sudo;

pub(crate) fn rules() -> Vec<Arc<dyn Rule>> {
    vec![
        LeadingShellPrompt.to_arc(),
        Repetition.to_arc(),
        Sudo.to_arc(),
        NoCommand.to_arc(),
    ]
}
