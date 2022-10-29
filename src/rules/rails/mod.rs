use crate::rules::rails::rails_pending_migrations::RailsPendingMigrations;
use crate::rules::Rule;

use super::CommandGroup;

mod rails_pending_migrations;

pub(crate) fn command_group() -> CommandGroup {
    CommandGroup {
        command_names: &["rails"],
        rules: vec![RailsPendingMigrations.to_arc()],
    }
}
