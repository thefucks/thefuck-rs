use crate::rules::util::{get_single_closest_match, new_commands_from_suggestions};
use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

const GIT_BISECT_COMMANDS: &[&str] = &[
    "help",
    "start",
    "bad",
    "good",
    "new",
    "old",
    "terms",
    "skip",
    "next",
    "reset",
    "visualize",
    "view",
    "replay",
    "log",
    "run",
];

/// Corrects a misspelled git bisect subcommand
pub(crate) struct GitBisect;
impl Rule for GitBisect {
    default_rule_id!(GitBisect);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command
            .input_parts()
            .get(1)
            .map_or(false, |s| s == "bisect")
            && command.lowercase_output().contains("usage: git bisect")
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let bisect_pos = command.input_parts().iter().position(|s| s == "bisect")?;
        let broken = command.input_parts().get(bisect_pos + 1)?;

        let fix = get_single_closest_match(broken, GIT_BISECT_COMMANDS.to_vec())?;
        new_commands_from_suggestions([fix], command.input_parts(), broken)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_git_bisect() {
        assert_eq!(
            basic_corrections(
                "git bisect strt",
                "usage: git bisect [help|start|bad|good|new|old|terms|skip|next|reset|visualize|view|replay|log|run]"
            ),
            vec!["git bisect start"]
        )
    }
}
