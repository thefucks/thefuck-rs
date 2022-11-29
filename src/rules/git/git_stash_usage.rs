use crate::rules::util::{get_single_closest_match, new_commands_from_suggestions};
use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(
        "(?i)subcommand wasn't specified; 'push' can't be assumed due to unexpected token '(.+)'"
    )
    .unwrap();
}

const GIT_STASH_SUBCOMMANDS: &[&str] = &[
    "list", "show", "drop", "pop", "apply", "branch", "push", "clear", "create", "store",
];

/// Corrects misspelled git stash subcommands.
pub(crate) struct GitStashUsage;
impl Rule for GitStashUsage {
    default_rule_id!(GitStashUsage);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command.input_parts().get(1).map_or(false, |p| p == "stash")
            && RE.is_match(command.lowercase_output())
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let wrong_subcommand = RE
            .captures(command.output)
            .and_then(|captures| captures.get(1))
            .map(|to_fix| to_fix.as_str())?;

        let closest_subcommand_match =
            get_single_closest_match(wrong_subcommand, GIT_STASH_SUBCOMMANDS.to_vec())?;
        new_commands_from_suggestions(
            [closest_subcommand_match],
            command.input_parts(),
            wrong_subcommand,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_git_stash_usage() {
        assert_eq!(
            basic_corrections(
                "git stash aply",
                "fatal: subcommand wasn't specified; 'push' can't be assumed due to unexpected token 'aply'"
            ),
            vec!["git stash apply"]
        )
    }
}
