use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

/*
Fixes a git push command that actually requires a "--force-with-lease".
See more here: https://github.com/nvbn/thefuck/blob/5198b34f24ca4bc414a5bf1b0288ee86ea2529a8/thefuck/rules/git_push_force.py
*/
pub(crate) struct GitPushForce;
impl Rule for GitPushForce {
    default_rule_id!(GitPushForce);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        let lowercase_output = command.lowercase_output();
        command.input_parts().iter().any(|part| part == "push")
            && lowercase_output.contains("! [rejected]")
            && lowercase_output.contains("failed to push some refs")
            && lowercase_output
                .contains("updates were rejected because the tip of your current branch is behind")
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let mut new_command = command.input_parts().to_vec();
        let push_index = new_command.iter().position(|part| part == "push")?;
        new_command.insert(push_index + 1, "--force-with-lease".to_owned());
        Some(vec![new_command.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_git_push_force() {
        assert_eq!(
            basic_corrections(
                "git push some-other-arg",
                "To github.com:org/repo.git
                  ! [rejected]        branch/name -> branch/name (non-fast-forward)
                 error: failed to push some refs to 'github.com:org/repo.git'
                 hint: Updates were rejected because the tip of your current branch is behind
                 hint: its remote counterpart. Integrate the remote changes (e.g.
                 hint: 'git pull ...') before pushing again.
                 hint: See the 'Note about fast-forwards' in 'git push --help' for details.
                "
            ),
            vec!["git push --force-with-lease some-other-arg"]
        )
    }
}
