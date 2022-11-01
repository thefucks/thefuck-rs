mod brew;
mod cargo;
mod cat;
mod cd;
mod cp;
mod docker;
mod generic;
mod git;
mod grep;
mod java;
mod ls;
mod npm;
mod pip;
mod rails;
mod sed;
mod sudo;
mod touch;
mod yarn;

mod util;

use crate::{Command, RuleCorrection, SessionMetadata};
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Arc};

lazy_static! {
    // Only used internally by this mod
    static ref COMMAND_GROUPS: Vec<CommandGroup> =
        vec![
            cargo::command_group(),
            cat::command_group(),
            git::command_group(),
            java::command_group(),
            cd::command_group(),
            brew::command_group(),
            ls::command_group(),
            touch::command_group(),
            sudo::command_group(),
            cp::command_group(),
            rails::command_group(),
            docker::command_group(),
            sed::command_group(),
            pip::command_group(),
            npm::command_group(),
            yarn::command_group(),
            grep::command_group(),
        ];

    pub(crate) static ref RULES_BY_COMMAND: Rules = Rules::from_command_groups(COMMAND_GROUPS.iter());
    pub(crate) static ref GENERIC_RULES: Vec<Arc<dyn Rule>> = generic::rules();
}

/// A list of command names that have common rules. Often times, this will
/// be a list of size 1, but not always -- for example,
/// ["python", "python3"] is an example of a command group
pub struct CommandGroup {
    command_names: &'static [&'static str],
    rules: Vec<Arc<dyn Rule>>,
}

/// Map of a command to the `Rule`s that may apply for the given command.
pub(crate) struct Rules(HashMap<&'static str, &'static [Arc<dyn Rule>]>);

impl Rules {
    pub fn get(&self, command_name: &str) -> Option<&'static [Arc<dyn Rule>]> {
        self.0.get(command_name).copied()
    }

    fn from_command_groups(command_groups: impl Iterator<Item = &'static CommandGroup>) -> Self {
        Rules(HashMap::from_iter(command_groups.flat_map(|cmd_group| {
            cmd_group
                .command_names
                .iter()
                .map(|cmd| (*cmd, cmd_group.rules.as_slice()))
        })))
    }
}

pub(crate) trait Rule: Send + Sync {
    fn to_arc(self) -> Arc<dyn Rule>
    where
        Self: 'static + Sized,
    {
        Arc::new(self)
    }

    /// The name of the rule. See default_rule_id! for a default implementation.
    /// This should be unique for each rule.
    // TODO: we can write a procedural macro to just `#[derive(RuleId)]` and
    // include a trait bound wherever we expect a `Rule`: Rule + RuleId
    fn id(&self) -> &'static str;

    /// Whether the rule should even be considered. If true, we check
    /// if the rule `matches` the command.
    // If this check ever needs to be more sophisticated than just whether or
    // not we should run on failure, we should rename this to be more generic.
    fn only_run_on_failure(&self) -> bool {
        true
    }

    /// Whether the command matches this rule. If true,
    /// we'll try to `generate_command_corrections` for this rule.
    fn matches(&self, command: &Command, session_metadata: &SessionMetadata) -> bool;

    /// Generates a list of command corrections for a command.
    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>>;
}

#[macro_export]
macro_rules! default_rule_id {
    ($t:ty) => {
        fn id(&self) -> &'static str {
            stringify!($t)
        }
    };
}
