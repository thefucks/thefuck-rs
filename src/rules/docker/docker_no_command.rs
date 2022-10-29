use crate::rules::util::{get_single_closest_match, new_commands_from_suggestions};
use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new("(?i)'(.+)' is not a docker command").unwrap();
}

const DOCKER_SUBCOMMANDS: &[&str] = &[
    "attach", "build", "commit", "cp", "create", "diff", "events", "exec", "export", "history",
    "images", "import", "info", "inspect", "kill", "load", "login", "logout", "logs", "pause",
    "port", "ps", "pull", "push", "rename", "restart", "rm", "rmi", "run", "save", "search",
    "start", "stats", "stop", "tag", "top", "unpause", "update", "version", "wait",
];

const DOCKER_BUILDER_SUBCOMMANDS: &[&str] = &[
    "builder",
    "buildx",
    "compose",
    "config",
    "container",
    "context",
    "extension",
    "image",
    "manifest",
    "network",
    "node",
    "plugin",
    "sbom",
    "scan",
    "secret",
    "service",
    "stack",
    "swarm",
    "system",
    "trust",
    "volume",
];

/// Corrects an unknown docker subcommand.
// TODO: eventually we should support corrections for nested subcommands.
// e.g. "docker compose" has its own set of commands.
pub(crate) struct DockerNoCommand;
impl Rule for DockerNoCommand {
    default_rule_id!(DockerNoCommand);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        RE.is_match(command.lowercase_output())
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

        let subcommands = DOCKER_SUBCOMMANDS
            .iter()
            .chain(DOCKER_BUILDER_SUBCOMMANDS)
            .copied()
            .collect_vec();

        let fix = get_single_closest_match(to_fix, subcommands)?;
        new_commands_from_suggestions([fix], command.input_parts(), to_fix)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_docker_no_command() {
        assert_eq!(
            basic_corrections(
                "docker img",
                "docker: 'img' is not a docker command.
                See 'docker --help'"
            ),
            vec!["docker image"]
        )
    }
}
