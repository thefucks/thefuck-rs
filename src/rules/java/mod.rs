use crate::rules::java::dot_java::DotJava;
use crate::rules::Rule;
use std::sync::Arc;

mod dot_java;

pub(crate) fn rules_for_command() -> (&'static str, Vec<Arc<dyn Rule>>) {
    ("java", vec![DotJava.to_arc()])
}
