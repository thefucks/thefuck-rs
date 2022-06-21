use crate::rules::git::git_push::GitPush;
use crate::rules::Rule;
use std::sync::Arc;

mod git_push;

pub(crate) fn rules_for_command() -> (&'static str, Vec<Arc<dyn Rule>>) {
    ("git", vec![GitPush.to_arc()])
}
