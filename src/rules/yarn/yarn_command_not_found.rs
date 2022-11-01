use crate::rules::util::{get_single_closest_match, new_commands_from_suggestions};
use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref WRONG_COMMAND_RE: Regex = Regex::new("(?i)Command \"(.+)\" not found").unwrap();
}

const YARN_SUBCOMMANDS: &[&str] = &[
    "access",
    "add",
    "audit",
    "autoclean",
    "bin",
    "cache",
    "check",
    "config",
    "create",
    "exec",
    "generate-lock-entry",
    "generateLockEntry",
    "global",
    "help",
    "import",
    "info",
    "init",
    "install",
    "licenses",
    "link",
    "list",
    "login",
    "logout",
    "node",
    "outdated",
    "owner",
    "pack",
    "policies",
    "publish",
    "remove",
    "run",
    "tag",
    "team",
    "unlink",
    "unplug",
    "upgrade",
    "upgrade-interactive",
    "upgradeInteractive",
    "version",
    "versions",
    "why",
    "workspace",
];

/// Corrects an unknown yarn subcommand.
pub(crate) struct YarnCommandNotFound;
impl Rule for YarnCommandNotFound {
    default_rule_id!(YarnCommandNotFound);

    // TODO: we shouldn't run this if yarn alias is being run
    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        WRONG_COMMAND_RE.is_match(command.output)
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let to_replace = WRONG_COMMAND_RE
            .captures(command.output)
            .and_then(|captures| captures.get(1))
            .map(|to_fix| to_fix.as_str())?;

        let subcommands = YARN_SUBCOMMANDS.to_vec();
        let fix = get_single_closest_match(to_replace, subcommands)?;
        new_commands_from_suggestions([fix], command.input_parts(), to_replace)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_yarn_unknown_command() {
        assert_eq!(
            basic_corrections(
                "yarn rn start",
                r#"error Command "rn" not found.
                info Visit https://yarnpkg.com/en/docs/cli/run for documentation about this command."#
            ),
            vec!["yarn run start"]
        )
    }
}
