use crate::rules::generic::{
    cd_parent::CdParent, chmod_x::ChmodX, leading_shell_prompt::LeadingShellPrompt,
    no_command::NoCommand, repetition::Repetition, sudo::Sudo,
};
use crate::rules::Rule;
use std::sync::Arc;

mod cd_parent;
mod chmod_x;
mod leading_shell_prompt;
mod no_command;
mod repetition;
mod sudo;

pub(crate) fn rules() -> Vec<Arc<dyn Rule>> {
    vec![
        ChmodX.to_arc(),
        LeadingShellPrompt.to_arc(),
        Repetition.to_arc(),
        Sudo.to_arc(),
        NoCommand.to_arc(),
        CdParent.to_arc(),
    ]
}
