use crate::rules::brew::{
    brew_install::BrewInstall, brew_link::BrewLink, brew_reinstall::BrewReinstall,
    brew_uninstall::BrewUninstall, brew_unknown_command::BrewUnknownCommand,
    brew_update_upgrade::BrewUpdateUpgrade,
};
use crate::rules::Rule;

use super::CommandGroup;

mod brew_install;
mod brew_link;
mod brew_reinstall;
mod brew_uninstall;
mod brew_unknown_command;
mod brew_update_upgrade;

pub(crate) fn command_group() -> CommandGroup {
    CommandGroup {
        command_names: &["brew"],
        rules: vec![
            BrewInstall.to_arc(),
            BrewLink.to_arc(),
            BrewReinstall.to_arc(),
            BrewUninstall.to_arc(),
            BrewUnknownCommand.to_arc(),
            BrewUpdateUpgrade.to_arc(),
        ],
    }
}
