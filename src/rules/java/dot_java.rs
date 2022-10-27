use crate::rules::Rule;
use crate::{Command, Correction, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r".+\.java$").unwrap();
}

pub(crate) struct DotJava;
impl Rule for DotJava {
    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        RE.is_match(command.input)
            && command
                .lowercase_output()
                .contains("could not find or load main")
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<Correction<'a>>> {
        let mut replacement = command.input_parts().to_vec();
        let pos = replacement.iter().position(|p| (p.ends_with(".java")))?;
        *replacement.get_mut(pos)? = replacement.get(pos)?.trim_end_matches(".java").to_owned();

        Some(vec![replacement.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;
    #[test]
    fn test_dot_java() {
        assert_eq!(
            basic_corrections(
                "java boo.java",
                "Error: Could not find or load main class boo.java"
            ),
            vec!["java boo"]
        )
    }
}
