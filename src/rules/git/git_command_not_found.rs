use crate::rules::util::new_commands_from_suggestions;
use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NOT_GIT_COMMAND_RE: Regex =
        Regex::new("(?i)git: '([^']*)' is not a git command").unwrap();
    static ref MOST_SIMILAR_RE: Regex =
        Regex::new("(?is)The most similar command[s]? (?:is|are)(.*)").unwrap();
    static ref DID_YOU_MEAN_RE: Regex = Regex::new("(?is)Did you mean(.*)").unwrap();
}

/*
Corrects a misspelled `git *` command based on the recommendations from git.
See more here: https://github.com/nvbn/thefuck/blob/5198b34f24ca4bc414a5bf1b0288ee86ea2529a8/thefuck/rules/git_not_command.py
*/
pub(crate) struct GitCommandNotFound;
impl Rule for GitCommandNotFound {
    default_rule_id!(GitCommandNotFound);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        let lowercase_output = command.lowercase_output();
        NOT_GIT_COMMAND_RE.is_match(lowercase_output)
            && (MOST_SIMILAR_RE.is_match(lowercase_output)
                || DID_YOU_MEAN_RE.is_match(lowercase_output))
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let lowercase_output = command.lowercase_output();

        let incorrect_command = NOT_GIT_COMMAND_RE
            .captures(lowercase_output)
            .and_then(|captures| captures.get(1))
            .map(|regex_match| regex_match.as_str())?;

        let most_similar_commands = MOST_SIMILAR_RE
            .captures(lowercase_output)
            .and_then(|captures| captures.get(1))
            .map(|regex_match| regex_match.as_str());

        if let Some(most_similar_commands) = most_similar_commands {
            new_commands_from_suggestions(
                most_similar_commands.lines(),
                command.input_parts(),
                incorrect_command,
            )
        } else {
            let did_you_mean_commands = DID_YOU_MEAN_RE
                .captures(lowercase_output)
                .and_then(|captures| captures.get(1))
                .map(|regex_match| regex_match.as_str());

            did_you_mean_commands.and_then(|did_you_mean_commands| {
                new_commands_from_suggestions(
                    did_you_mean_commands.lines(),
                    command.input_parts(),
                    incorrect_command,
                )
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_git_most_similar_command() {
        assert_eq!(
            basic_corrections(
                "git psuh --force-with-lease",
                "git: 'psuh' is not a git command. See 'git --help'.

                The most similar command is
                    push
                "
            ),
            vec!["git push --force-with-lease"]
        )
    }

    #[test]
    fn test_git_most_similar_commands() {
        assert_eq!(
            basic_corrections(
                "git st",
                "git: 'st' is not a git command. See 'git --help'.

                The most similar commands are
                    status
                    reset
                    s
                    stage
                    stash
                "
            ),
            vec!["git status", "git reset", "git s", "git stage", "git stash"]
        )
    }

    #[test]
    fn test_git_did_you_mean() {
        assert_eq!(
            basic_corrections(
                "git st",
                "git: 'st' is not a git command. See 'git --help'.

                Did you mean
                    status
                    reset
                    s
                    stage
                    stash
                "
            ),
            vec!["git status", "git reset", "git s", "git stage", "git stash"]
        )
    }

    #[test]
    fn test_git_commit() {
        assert_eq!(
            basic_corrections(
                r#"git comit -m "my fancy message""#,
                "git: 'comit' is not a git command. See 'git --help'.
                The most similar command is
                    commit
                "
            ),
            vec![r#"git commit -m "my fancy message""#]
        )
    }
}
