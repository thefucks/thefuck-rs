use crate::rules::util::{get_single_closest_match, new_commands_from_suggestions};
use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new("(?i)Unknown command: \"(.+)\"").unwrap();
}

const NPM_SUBCOMMANDS: &[&str] = &[
    "access",
    "adduser",
    "audit",
    "bin",
    "bugs",
    "cache",
    "ci",
    "completion",
    "config",
    "dedupe",
    "deprecate",
    "diff",
    "dist-tag",
    "docs",
    "doctor",
    "edit",
    "exec",
    "explain",
    "explore",
    "find-dupes",
    "fund",
    "get",
    "help",
    "hook",
    "init",
    "install",
    "install-ci-test",
    "install-test",
    "link",
    "ll",
    "login",
    "logout",
    "ls",
    "org",
    "outdated",
    "owner",
    "pack",
    "ping",
    "pkg",
    "prefix",
    "profile",
    "prune",
    "publish",
    "query",
    "rebuild",
    "repo",
    "restart",
    "root",
    "run-script",
    "search",
    "set",
    "set-script",
    "shrinkwrap",
    "star",
    "stars",
    "start",
    "stop",
    "team",
    "test",
    "token",
    "uninstall",
    "unpublish",
    "unstar",
    "update",
    "version",
    "view",
    "whoami",
];

/// Corrects an unknown npm subcommand.
// TODO: eventually support errors at nested subcommands (e.g. corrections
// for `npm access <word>)
pub(crate) struct NpmUnknownCommand;
impl Rule for NpmUnknownCommand {
    default_rule_id!(NpmUnknownCommand);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command.lowercase_output().contains("unknown command")
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let to_replace = RE
            .captures(command.output)
            .and_then(|captures| captures.get(1))
            .map(|to_fix| to_fix.as_str())?;

        let subcommands = NPM_SUBCOMMANDS.to_vec();
        let fix = get_single_closest_match(to_replace, subcommands)?;

        new_commands_from_suggestions([fix], command.input_parts(), to_replace)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_npm_unknown_command() {
        assert_eq!(
            basic_corrections(
                "npm insll",
                r#"npm insll
                Unknown command: "insll"
                
                To see a list of supported npm commands, run:
                npm help"#
            ),
            vec!["npm install"]
        )
    }
}
