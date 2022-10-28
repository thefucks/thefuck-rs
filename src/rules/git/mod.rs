use crate::rules::git::{
    git_checkout::GitCheckout, git_command_not_found::GitCommandNotFound,
    git_push_force::GitPushForce, git_push_set_upstream::GitPushSetUpstream,
};
use crate::rules::Rule;

use super::CommandGroup;

mod git_checkout;
mod git_command_not_found;
mod git_push_force;
mod git_push_set_upstream;

pub(crate) fn command_group() -> CommandGroup {
    CommandGroup {
        command_names: &["git"],
        rules: vec![
            GitPushSetUpstream.to_arc(),
            GitPushForce.to_arc(),
            GitCommandNotFound.to_arc(),
            GitCheckout.to_arc(),
        ],
    }
}
