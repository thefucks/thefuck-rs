use crate::rules::docker::{
    docker_image_rm::DockerImageRm, docker_login::DockerLogin, docker_no_command::DockerNoCommand,
};
use crate::rules::Rule;

use super::CommandGroup;

mod docker_image_rm;
mod docker_login;
mod docker_no_command;

pub(crate) fn command_group() -> CommandGroup {
    CommandGroup {
        command_names: &["docker"],
        rules: vec![
            DockerImageRm.to_arc(),
            DockerNoCommand.to_arc(),
            DockerLogin.to_arc(),
        ],
    }
}
