use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

/// Fixes a docker command where you have to login first.
pub(crate) struct DockerLogin;
impl Rule for DockerLogin {
    default_rule_id!(DockerLogin);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        let lowercase_output = command.lowercase_output();
        lowercase_output.contains("access denied")
            || lowercase_output.contains("may require 'docker login'")
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        Some(vec![RuleCorrection::and("docker login", command.input)])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_docker_login() {
        assert_eq!(
            basic_corrections(
                "docker push repo/image",
                r#"The push refers to repository repo/image.
                push access denied for repo/image, repository does not exist or may require 'docker login'"#
            ),
            vec!["docker login && docker push repo/image"]
        );
    }
}
