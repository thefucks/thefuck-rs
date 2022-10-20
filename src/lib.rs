use std::{borrow::Cow, collections::HashSet};

use itertools::Itertools;

mod rules;

#[cfg(test)]
mod test_utils;

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

#[derive(PartialEq, Eq)]
pub struct ExitCode(usize);

impl ExitCode {
    pub fn is_success(&self) -> bool {
        self.0 == 0
    }

    pub fn is_error(&self) -> bool {
        !self.is_success()
    }

    pub fn raw(&self) -> usize {
        self.0
    }
}

impl From<usize> for ExitCode {
    fn from(code: usize) -> Self {
        ExitCode(code)
    }
}

/// A Command represents a shell command that the user executed along
/// with its metadata. This is used to determine which corrections
/// make sense in the context of the command.
pub struct Command<'a> {
    input: &'a str,
    output: &'a str,

    /// TODO: by default, a rule should only be run if the command failed.
    /// Rules that should run if a command _succeeded_ should have to opt-in.
    #[allow(dead_code)]
    exit_code: ExitCode,

    lowercase_output: String,
    input_parts: Vec<String>,
}

impl<'a> Command<'a> {
    pub fn new(input: &'a str, output: &'a str, exit_code: ExitCode) -> Self {
        // TODO: We need to re-escape multiword parts in the input after splitting. Thefuck has a
        // terribly hacky way of doing this here: https://github.com/nvbn/thefuck/blob/4c7479b3adcf8715a93d0c48e1ece83a35cda50d/thefuck/shells/generic.py#L87
        let input_parts = shlex::split(input).unwrap_or_default();
        let lowercase_output = output.to_lowercase();

        Self {
            input,
            output,
            lowercase_output,
            input_parts,
            exit_code,
        }
    }

    pub fn input_parts(&self) -> &[String] {
        &self.input_parts
    }

    pub fn lowercase_output(&self) -> &str {
        self.lowercase_output.as_str()
    }
}

type AliasName<'a> = &'a str;
type ExecutableName<'a> = &'a str;
type FunctionName<'a> = &'a str;

/// TODO: add support for shell type and user's history (with a limit?)
#[derive(Default)]
struct SessionMetadata<'a> {
    pub aliases: HashSet<AliasName<'a>>,
    pub executables: HashSet<ExecutableName<'a>>,
    pub functions: HashSet<FunctionName<'a>>,
}

/// A CommandCorrector is a wrapper around the main `correct_command` API
/// so that clients can set any extra metadata (if available) that they have
/// so that we can surface more intelligent corrections.
#[derive(Default)]
pub struct CommandCorrector<'a> {
    session_metadata: SessionMetadata<'a>,
}

impl<'a> CommandCorrector<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns a list of command corrections given a command. This is _heavily_ inspired
    /// by The Fuck (https://github.com/nvbn/thefuck).
    pub fn correct_command(&self, command: Command) -> Vec<String> {
        let rules = &*rules::RULES;

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
                rule.matches(&command, &self.session_metadata)
                    .then(|| rule.generate_command_corrections(&command, &self.session_metadata))
                    .flatten()
            })
            .flatten()
            .map(|correction| correction.to_command_string())
            .collect()
    }

    pub fn set_aliases(&mut self, aliases: impl Iterator<Item = AliasName<'a>>) {
        self.session_metadata.aliases = HashSet::from_iter(aliases);
    }

    pub fn set_functions(&mut self, functions: impl Iterator<Item = FunctionName<'a>>) {
        self.session_metadata.functions = HashSet::from_iter(functions);
    }

    pub fn set_executables(&mut self, executables: impl Iterator<Item = ExecutableName<'a>>) {
        self.session_metadata.executables = HashSet::from_iter(executables);
    }
}
