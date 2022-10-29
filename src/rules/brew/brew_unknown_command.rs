use crate::rules::util::{get_single_closest_match, new_commands_from_suggestions};
use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new("(?i)Unknown command: (.+)").unwrap();
}

/// Subcommands for V??
const BREW_SUBCOMMANDS: &[&str] = &[
    "analytics",
    "autoremove",
    "casks",
    "cleanup",
    "commands",
    "completions",
    "config",
    "deps",
    "desc",
    "developer",
    "docs",
    "doctor",
    "dr",
    "fetch",
    "formulae",
    "gist-logs",
    "home",
    "homepage",
    "info",
    "abv",
    "install",
    "leaves",
    "link",
    "ln",
    "list",
    "ls",
    "log",
    "migrate",
    "missing",
    "options",
    "outdated",
    "pin",
    "postinstall",
    "readall",
    "reinstall",
    "search",
    "shellenv",
    "tap",
    "tap-info",
    "uninstall",
    "remove",
    "rm",
    "unlink",
    "unpin",
    "untap",
    "update",
    "update-reset",
    "upgrade",
    "uses",
];
/// Corrects an unknown brew subcommand.
pub(crate) struct BrewUnknownCommand;
impl Rule for BrewUnknownCommand {
    default_rule_id!(BrewUnknownCommand);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command.lowercase_output().contains("unknown command")
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let to_fix = RE
            .captures(command.output)
            .and_then(|captures| captures.get(1))
            .map(|to_fix| to_fix.as_str())?;

        let subcommands = BREW_SUBCOMMANDS.to_vec();

        // TODO: maybe we should fetch more than one eventually?
        let fix = get_single_closest_match(to_fix, subcommands)?;

        new_commands_from_suggestions([fix], command.input_parts(), to_fix)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_brew_unknown_command() {
        assert_eq!(
            basic_corrections("brew instll jq", "Error: Unknown command: instll"),
            vec!["brew install jq"]
        )
    }
}
