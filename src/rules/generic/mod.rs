use crate::rules::generic::repetition::Repetition;
use crate::rules::Rule;
use std::sync::Arc;

mod repetition;

pub(crate) fn rules() -> Vec<Arc<dyn Rule>> {
    vec![Repetition.to_arc()]
}
