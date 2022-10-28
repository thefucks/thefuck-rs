use std::{borrow::Cow, collections::HashSet};

use itertools::Itertools;

mod rules;

#[cfg(test)]
mod test_utils;

/// A Correction could either be a fully formed command or a series
/// of command parts that need to be combined to form a command.
///
/// Note about Cow<str>: we use a Cow here since the string will either be a reference
/// to one of the input parts, a static string, or a computed string over an input part.
/// In the first two cases, a borrowed string suffices, but in the last case, we need an owned string.
/// Using a Cow allows us to avoid `clone`s for the first two types of strings.
#[derive(Debug, PartialEq)]
enum Correction<'a> {
    Command(Cow<'a, str>),

    /// Example: if the corrected command is `git commit -m "best fix"`, then the correction is
    /// ["git", "commit", "-m", "best fix"]. Note that "best fix" is a single part. This is important
    /// as it guarantees the correct shell-escaped command is returned in `to_command_string`.
    /// Note: although translating into a Correction seems expensive (using Vecs), very few corrections
    /// are actually computed (only for matching top-level command and if the rule `matches`)
    CommandParts(Vec<Cow<'a, str>>),

    // TODO: This is a temporary solution to a better escaping strategy
    /// The `And` variant is used for corrections that involve two other
    /// Corrections, meant to be combined with the shell AND operator. For example,
    /// the command "mkdir -p dir && cd dir" can be expressed with an And variant like so:
    /// And(CommandParts(["mkdir", "-p", "dir"]), Command("cd dir"))
    And(Box<Correction<'a>>, Box<Correction<'a>>),
}

impl<'a> Correction<'a> {
    fn to_command_string(&self, shell: &Shell) -> String {
        use Correction::*;
        match self {
            // base cases
            Command(str) => str.to_string(),
            CommandParts(parts) => shlex::join(parts.iter().map(|part| part.as_ref())),

            // recursive case
            And(first, second) => {
                let first_str = first.to_command_string(shell);
                let second_str = second.to_command_string(shell);
                [first_str.as_str(), shell.and(), second_str.as_str()].join(" ")
            }
        }
    }

    /// Utility to create the And variant (without fussing with Box at callsites)
    #[allow(dead_code)]
    fn and(first: impl Into<Correction<'a>>, second: impl Into<Correction<'a>>) -> Self {
        Correction::And(Box::new(first.into()), Box::new(second.into()))
    }
}

// Note: need two separate From impl's because we
// can't do From<T> where T: Cow<str> (compiler will complain
// about conflicting with the other From impl's)
impl<'a> From<String> for Correction<'a> {
    fn from(command: String) -> Self {
        Correction::Command(command.into())
    }
}
impl<'a> From<&'a str> for Correction<'a> {
    fn from(command: &'a str) -> Self {
        Correction::Command(command.into())
    }
}

impl<'a, T> From<Vec<T>> for Correction<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(parts: Vec<T>) -> Self {
        Correction::CommandParts(parts.into_iter().map(Into::into).collect_vec())
    }
}
impl<'a, T> From<&'a [T]> for Correction<'a>
where
    T: AsRef<str>,
{
    fn from(parts: &'a [T]) -> Self {
        Correction::CommandParts(
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

/// The shells supported by this crate.
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

impl Shell {
    #[allow(dead_code)]
    fn and(&self) -> &'static str {
        use Shell::*;
        match self {
            Bash | Zsh => "&&",
            Fish => "and",
        }
    }
}

impl Default for Shell {
    fn default() -> Self {
        Shell::Bash
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
type BuiltinName<'a> = &'a str;
type ExecutableName<'a> = &'a str;
type FunctionName<'a> = &'a str;

type HistoryItem<'a> = &'a str;

#[derive(Default)]
pub struct SessionMetadata<'a> {
    shell: Shell,

    aliases: HashSet<AliasName<'a>>,
    builtins: HashSet<BuiltinName<'a>>,
    executables: HashSet<ExecutableName<'a>>,
    functions: HashSet<FunctionName<'a>>,

    history: Vec<HistoryItem<'a>>,
}

impl<'a> SessionMetadata<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_aliases(&mut self, aliases: impl IntoIterator<Item = AliasName<'a>>) {
        self.aliases = HashSet::from_iter(aliases);
    }

    pub fn set_builtins(&mut self, builtins: impl IntoIterator<Item = BuiltinName<'a>>) {
        self.builtins = HashSet::from_iter(builtins);
    }

    pub fn set_functions(&mut self, functions: impl IntoIterator<Item = FunctionName<'a>>) {
        self.functions = HashSet::from_iter(functions);
    }

    pub fn set_executables(&mut self, executables: impl IntoIterator<Item = ExecutableName<'a>>) {
        self.executables = HashSet::from_iter(executables);
    }

    pub fn set_history(&mut self, history: impl IntoIterator<Item = HistoryItem<'a>>) {
        self.history = Vec::from_iter(history);
    }

    pub fn set_shell(&mut self, shell: Shell) {
        self.shell = shell;
    }

    fn is_top_level_command(&self, command: &str) -> bool {
        self.executables.contains(command)
            || self.aliases.contains(command)
            || self.functions.contains(command)
            || self.builtins.contains(command)
    }

    fn top_level_commands(&self) -> impl Iterator<Item = &str> {
        self.executables
            .iter()
            .chain(self.aliases.iter())
            .chain(self.functions.iter())
            .chain(self.builtins.iter())
            .copied()
    }

    /// Returns the command name for each history item
    /// e.g. `git checkout` => `git`
    fn top_level_commands_from_history(&self) -> impl Iterator<Item = &str> {
        self.history
            .iter()
            .filter_map(|s| s.split_whitespace().next())
    }
}

/// Returns a list of command corrections given a command. This is _heavily_ inspired
/// by The Fuck (https://github.com/nvbn/thefuck).
// TODO: add tests for this function
pub fn correct_command(command: Command, session_metadata: &SessionMetadata) -> Vec<String> {
    let rules = &*rules::RULES_BY_COMMAND;

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
            // Only check a rule if the command failed or if the rule
            // is meant to run irrespective of failures.
            let should_check_matches = !rule.only_run_on_failure() || command.exit_code.is_error();

            (should_check_matches && rule.matches(&command, session_metadata))
                .then(|| rule.generate_command_corrections(&command, session_metadata))
                .flatten()
        })
        .flatten()
        .filter_map(|correction| {
            let cmd_string = correction.to_command_string(&session_metadata.shell);
            (cmd_string != command.input).then_some(cmd_string)
        })
        .unique()
        .collect()
}
