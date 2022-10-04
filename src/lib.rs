use std::ops::Deref;

use itertools::Itertools;

mod rules;

/// A correction is a collection of shell command parts.
/// Example: if the corrected command is `git commit -m "best fix"`, then the correction is
/// ["git", "commit", "-m", "best fix"]. Note that "best fix" is a single part. This is important
/// as it guarantees the correct shell-escaped command is returned in `to_command_string`.
/// TODO: investigate if we can use Cow<str> instead of String's here.
/// TODO: add a more ergonomic API for testing with `Correction`s
#[derive(Debug, PartialEq)]
struct Correction(pub Vec<String>);
impl Correction {
    fn to_command_string(&self) -> String {
        shlex::join(self.0.iter().map(|part| part.as_str()))
    }
}

impl From<Vec<String>> for Correction {
    fn from(parts: Vec<String>) -> Self {
        Correction(parts)
    }
}
impl From<&[&str]> for Correction {
    fn from(parts: &[&str]) -> Self {
        Correction(parts.iter().map(|p| p.to_string()).collect_vec())
    }
}
impl From<&[String]> for Correction {
    fn from(parts: &[String]) -> Self {
        Correction(parts.to_vec())
    }
}
impl From<Vec<&str>> for Correction {
    fn from(parts: Vec<&str>) -> Self {
        parts[..].into()
    }
}

/// Returns a list of command corrections given a command and its output. This is _heavily_ inspired
/// by The Fuck (https://github.com/nvbn/thefuck).
pub fn command_corrections(command_input: &str, command_output: &str) -> Vec<String> {
    let rules = &*rules::RULES;
    let command = Command::new(command_input, command_output);

    let command_name = match command.input_parts.first() {
        None => return vec![],
        Some(first) => first,
    };

    rules
        .get(command_name)
        .into_iter()
        .flatten()
        .chain(rules::GENERIC_RULES.iter())
        .filter_map(|rule| {
            rule.matches(&command)
                .then(|| rule.generate_command_corrections(&command))
                .flatten()
        })
        .flatten()
        .map(|correction| correction.to_command_string())
        .collect()
}

struct Command<'a> {
    input: &'a str,
    output: &'a str,
    lowercase_output: String,
    input_parts: Vec<String>,
}

impl<'a> Command<'a> {
    pub fn new(input: &'a str, output: &'a str) -> Self {
        // TODO: We need to re-escape multiword parts in the input after splitting. Thefuck has a
        // terribly hacky way of doing this here: https://github.com/nvbn/thefuck/blob/4c7479b3adcf8715a93d0c48e1ece83a35cda50d/thefuck/shells/generic.py#L87
        let input_parts = shlex::split(input).unwrap_or_default();
        let lowercase_output = output.to_lowercase();

        Self {
            input,
            output,
            lowercase_output,
            input_parts,
        }
    }

    pub fn input_parts(&self) -> &[String] {
        self.input_parts.deref()
    }

    pub fn lowercase_output(&self) -> &str {
        self.lowercase_output.as_str()
    }
}
