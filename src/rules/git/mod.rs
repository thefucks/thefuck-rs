use crate::rules::Rule;
use std::sync::Arc;

use crate::rules::git::git_push::GitPush;

mod git_push;

pub(crate) fn rules() -> impl Iterator<Item = Arc<dyn Rule>> {
    [GitPush.to_arc()].into_iter()
}
