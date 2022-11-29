use super::CommandGroup;
use crate::rules::git::{
    git_add::GitAdd, git_bisect::GitBisect, git_checkout::GitCheckout,
    git_checkout_exists::GitCheckoutExists, git_clone_repeated::GitCloneRepeated,
    git_command_not_found::GitCommandNotFound, git_main_master::GitMainMaster,
    git_push_force::GitPushForce, git_push_set_upstream::GitPushSetUpstream, git_stash::GitStash,
    git_stash_usage::GitStashUsage, git_two_dashes::GitTwoDashes,
};
use crate::rules::Rule;

mod git_add;
mod git_bisect;
mod git_checkout;
mod git_checkout_exists;
mod git_clone_repeated;
mod git_command_not_found;
mod git_main_master;
mod git_push_force;
mod git_push_set_upstream;
mod git_stash;
mod git_stash_usage;
mod git_two_dashes;

pub(crate) fn command_group() -> CommandGroup {
    CommandGroup {
        command_names: &["git"],
        rules: vec![
            GitCommandNotFound.to_arc(),
            GitMainMaster.to_arc(),
            GitCheckout.to_arc(),
            GitPushSetUpstream.to_arc(),
            GitPushForce.to_arc(),
            GitStash.to_arc(),
            GitAdd.to_arc(),
            GitCheckoutExists.to_arc(),
            GitTwoDashes.to_arc(),
            GitBisect.to_arc(),
            GitCloneRepeated.to_arc(),
            GitStashUsage.to_arc(),
        ],
    }
}
