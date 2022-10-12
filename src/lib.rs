use std::borrow::Cow;

use itertools::Itertools;

mod rules;

/// A correction is a collection of shell command parts.
/// Example: if the corrected command is `git commit -m "best fix"`, then the correction is
/// ["git", "commit", "-m", "best fix"]. Note that "best fix" is a single part. This is important
/// as it guarantees the correct shell-escaped command is returned in `to_command_string`.
/// Note: although translating into a Correction seems expensive (using Vecs), very few corrections
/// are actually computed (only for matching top-level command and if the rule `matches`)
///
/// Note about Cow<str>: we use a Cow here since the string will either be a reference
/// to one of the input parts, a static string, or a computed string over an input part.
/// In the first two cases, a borrowed string suffices, but in the last case, we need an owned string.
/// Using a Cow allows us to avoid `clone`s for the first two types of strings.
#[derive(Debug, PartialEq)]
struct Correction<'a>(pub Vec<Cow<'a, str>>);
impl<'a> Correction<'a> {
    fn to_command_string(&self) -> String {
        shlex::join(self.0.iter().map(|part| part.as_ref()))
    }
}
impl<'a, T> From<Vec<T>> for Correction<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(parts: Vec<T>) -> Self {
        Correction(parts.into_iter().map(Into::into).collect_vec())
    }
}
impl<'a, T> From<&'a [T]> for Correction<'a>
where
    T: AsRef<str>,
{
    fn from(parts: &'a [T]) -> Self {
        Correction(
            parts
                .iter()
                .map(AsRef::as_ref)
                .map(Into::into)
                .collect_vec(),
        )
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
        &self.input_parts
    }

    pub fn lowercase_output(&self) -> &str {
        self.lowercase_output.as_str()
    }
}
