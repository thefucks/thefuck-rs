use crate::rules::Rule;
use crate::Command;
use lazy_static::lazy_static;
use regex::Regex;

const SET_UPSTREAM_LONG_NAME: &str = "--set-upstream";
const SET_UPSTREAM_SHORT_NAME: &str = "-u";

lazy_static! {
    static ref RE: Regex =
        Regex::new(format!("(git push {SET_UPSTREAM_LONG_NAME} .*)").as_str()).unwrap();
}

/*
Fixes a git push command that is missing the "--set-upstream" option and arg.
See more here: https://github.com/nvbn/thefuck/blob/5198b34f24ca4bc414a5bf1b0288ee86ea2529a8/thefuck/rules/git_push.py
*/
pub(crate) struct GitPushSetUpstream;
impl Rule for GitPushSetUpstream {
    fn matches(&self, command: &Command) -> bool {
        command.input_parts().iter().any(|part| part == "push") && RE.is_match(command.output)
    }

    fn generate_command_corrections(&self, command: &Command) -> Option<Vec<String>> {
        let command_parts = command.input_parts();
        let mut new_command_parts = vec![];
        let mut idx = 0;

        // Get the suggested git command
        let vanilla_push_with_upstream = RE
            .captures(command.output)
            .and_then(|captures| captures.get(1))
            .map(|regex_match| regex_match.as_str())?;

        // Add the suggested git command as the first part of the corrected comamnd
        new_command_parts.push(vanilla_push_with_upstream);

        // Add any options except --set-upstream and -u back to the command
        // because the suggested git command wouldn't have included them
        while idx < command_parts.len() {
            let part = &command_parts[idx];
            if part.starts_with('-') {
                if part == SET_UPSTREAM_LONG_NAME {
                    // --set-upstream also has an arg so skip that as well
                    idx += 1;
                } else if part != SET_UPSTREAM_SHORT_NAME {
                    new_command_parts.push(part);
                }
            }
            idx += 1;
        }

        Some(vec![new_command_parts.join(" ")])
    }
}

#[cfg(test)]
mod tests {
    use crate::command_corrections;

    #[test]
    fn test_git_push_incorrect_command() {
        assert_eq!(
            command_corrections(
                "git push",
                "fatal: The current branch random has no upstream branch.
                To push the current branch and set the remote as upstream, use
                
                    git push --set-upstream origin random
                "
            ),
            vec!["git push --set-upstream origin random"]
        )
    }

    #[test]
    fn test_git_push_incorrect_command_with_more_output() {
        assert_eq!(
            command_corrections(
                "git push",
                "fatal: The current branch random has no upstream branch.
                To push the current branch and set the remote as upstream, use
                
                    git push --set-upstream origin random

                Some other string here that doesn't mean anything
                "
            ),
            vec!["git push --set-upstream origin random"]
        )
    }

    #[test]
    fn test_git_push_incorrect_command_with_options() {
        assert_eq!(
            command_corrections(
                "git push --force-with-lease -u",
                "fatal: The current branch random has no upstream branch.
                To push the current branch and set the remote as upstream, use
                
                    git push --set-upstream origin random
                "
            ),
            vec!["git push --set-upstream origin random --force-with-lease"]
        )
    }
}
