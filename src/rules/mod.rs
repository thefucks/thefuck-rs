mod cargo;
mod cat;
mod generic;
mod git;
mod java;

mod util;

use crate::{Command, Correction, SessionMetadata};
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Arc};

lazy_static! {
    pub(crate) static ref RULES: Rules = Rules(HashMap::from([
        cargo::rules_for_command(),
        git::rules_for_command(),
        cat::rules_for_command(),
        java::rules_for_command(),
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

    /// Whether the rule should even be considered. If true, we check
    /// if the rule `matches` the command.
    // If this check ever needs to be more sophisticated than just whether or
    // not we should run on failure, we should rename this to be more generic.
    fn only_run_on_failure(&self) -> bool {
        true
    }

    /// Whether the command matches this rule. If true, we try to
    /// `generate_command_corrections` for the rule.
    fn matches(&self, command: &Command, session_metadata: &SessionMetadata) -> bool;

    /// Generates a list of command corrections for a command.
    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<Correction<'a>>>;
}
