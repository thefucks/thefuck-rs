/*
Fixes error for `git checkout branch_that_doesnt_exist` to be `git checkout -b branch_that_doesnt_exist`.
See more here: https://github.com/nvbn/thefuck/blob/5198b34f24ca4bc414a5bf1b0288ee86ea2529a8/thefuck/rules/git_checkout.py
*/

use crate::rules::util::{get_single_closest_match, new_commands_from_suggestions};
use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

pub(crate) struct GitCheckout;

lazy_static! {
    static ref RE: Regex =
        Regex::new("(?i)error: pathspec '([^']*)' did not match any file").unwrap();
}

impl Rule for GitCheckout {
    default_rule_id!(GitCheckout);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command.input_parts().iter().any(|part| part == "checkout")
            && RE.is_match(command.lowercase_output())
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let mut corrections = vec![];

        // The first correction is replacing the unfound branch name with
        // an existing, similar branch name (if any).
        let wrong_git_branch_name = RE
            .captures(command.output)
            .and_then(|captures| captures.get(1))
            .map(|regex_match| regex_match.as_str())?;

        let branch_names = session_metadata.git_branches.iter().copied().collect_vec();
        let closest_git_branch = get_single_closest_match(wrong_git_branch_name, branch_names);

        if let Some(closest_git_branch) = closest_git_branch {
            corrections.extend(
                new_commands_from_suggestions(
                    [closest_git_branch],
                    command.input_parts(),
                    wrong_git_branch_name,
                )
                .into_iter()
                .flatten(),
            );
        }

        // The second correction is a suggestion to create the missing branch.
        let mut new_branch_correction = command.input_parts().to_vec();
        let checkout_pos = new_branch_correction.iter().position(|p| p == "checkout")?;
        new_branch_correction.insert(checkout_pos + 1, "-b".to_owned());
        corrections.push(new_branch_correction.into());

        Some(corrections)
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_utils::regular_corrections, Command, ExitCode, SessionMetadata};

    const GIT_BRANCHES: &[&str] = &["master", "main", "develop"];

    #[test]
    fn test_git_checkout_with_similar_branch() {
        let command = Command::new(
            "git checkout mster",
            "error: pathspec 'mster' did not match any file(s) known to git",
            ExitCode(1),
        );
        let mut session_metadata = SessionMetadata::new();
        session_metadata.set_git_branches(GIT_BRANCHES.iter().copied());

        assert_eq!(
            regular_corrections(command, &session_metadata),
            vec!["git checkout master", "git checkout -b mster"]
        )
    }

    #[test]
    fn test_git_checkout_with_new_branch() {
        let command = Command::new(
            "git checkout some-new-branch",
            "error: pathspec 'some-new-branch' did not match any file(s) known to git",
            ExitCode(1),
        );
        let mut session_metadata = SessionMetadata::new();
        session_metadata.set_git_branches(GIT_BRANCHES.iter().copied());

        assert_eq!(
            regular_corrections(command, &session_metadata),
            vec!["git checkout -b some-new-branch"]
        )
    }
}
