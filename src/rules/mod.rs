mod cargo;

use crate::Command;
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Arc};

lazy_static! {
    pub(crate) static ref RULES: Rules = Rules::new(cargo::rules());
}

/// Map of a command to the `Rule`s that may apply for the given command.
pub(crate) struct Rules(HashMap<&'static str, Vec<Arc<dyn Rule>>>);

impl Rules {
    pub fn new(rules: impl IntoIterator<Item = Arc<dyn Rule>>) -> Self {
        let mut map = HashMap::new();
        for rule in rules {
            for command in rule.for_commands() {
                map.entry(command)
                    .or_insert_with(Vec::new)
                    .push(rule.clone())
            }
        }

        Self(map)
    }

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

    /// Which commands the rule applies to. If the rule applies to the given command, then `matches`
    /// will be called to determine if the `Rule` is an exact match for the given command.
    fn for_commands(&self) -> Vec<&'static str>;

    /// Whether the command matches this rule.
    fn matches(&self, command: &Command) -> bool;

    /// Generates a list of command corrections for a command. This is only called if `matches`
    /// returns true.
    fn generate_command_corrections(&self, command: &Command) -> Option<Vec<String>>;
}
