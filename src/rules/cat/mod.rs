use crate::rules::cat::cat_dir::CatDir;
use crate::rules::Rule;
use std::sync::Arc;

mod cat_dir;

pub(crate) fn rules_for_command() -> (&'static str, Vec<Arc<dyn Rule>>) {
    ("cat", vec![CatDir.to_arc()])
}
