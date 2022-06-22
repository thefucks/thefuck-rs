mod cargo;
mod generic;
mod git;

mod util;

use crate::Command;
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Arc};

lazy_static! {
    pub(crate) static ref RULES: Rules = Rules(HashMap::from([
        cargo::rules_for_command(),
        git::rules_for_command()
    ]));
    pub(crate) static ref GENERIC_RULES: Vec<Arc<dyn Rule>> = generic::rules();
}

/// Map of a command to the `Rule`s that may apply for the given command.
pub(crate) struct Rules(HashMap<&'static str, Vec<Arc<dyn Rule>>>);

impl Rules {
    pub fn get(&self, command_name: &str) -> Option<&Vec<Arc<dyn Rule>>> {
        self.0.get(command_name)
    }
}

pub(crate) trait Rule: Send + Sync {
    fn to_arc(self) -> Arc<dyn Rule>
    where
        Self: 'static + Sized,
    {
        Arc::new(self)
    }

    /// Whether the command matches this rule.
    fn matches(&self, command: &Command) -> bool;

    /// Generates a list of command corrections for a command. This is only called if `matches`
    /// returns true.
    fn generate_command_corrections(&self, command: &Command) -> Option<Vec<String>>;
}
