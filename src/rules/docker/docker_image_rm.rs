use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

/// Fixes a docker command where removing an image fails.
pub(crate) struct DockerImageRm;
impl Rule for DockerImageRm {
    default_rule_id!(DockerImageRm);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        // Differs from thefuck since we don't specifically check if it failed
        // because of a running container (docker error messages can differ from
        // version to version).
        command.input.contains("image rm")
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let mut replacement = command.input_parts().to_vec();
        let rm_pos = replacement.iter().position(|p| p == "rm")?;
        replacement.insert(rm_pos + 1, "--force".to_owned());
        Some(vec![replacement.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_docker_image_rm() {
        assert_eq!(
            basic_corrections(
                "docker image rm ssh_image",
                r#"Error response from daemon: conflict: unable to remove repository reference "ssh_image" (must force) 
                - container 6e6714ce8662 is using its referenced image afc220a774e6"#
            ),
            vec!["docker image rm --force ssh_image"]
        );
    }
}
