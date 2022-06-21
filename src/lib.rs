use std::ops::Deref;

mod rules;

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
            if rule.matches(&command) {
                rule.generate_command_corrections(&command)
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

struct Command<'a> {
    input: &'a str,
    output: &'a str,
    input_parts: Vec<String>,
}

impl<'a> Command<'a> {
    pub fn new(input: &'a str, output: &'a str) -> Self {
        // TODO: We need to re-escape multiword parts in the input after splitting. Thefuck has a
        // terribly hacky way of doing this here: https://github.com/nvbn/thefuck/blob/4c7479b3adcf8715a93d0c48e1ece83a35cda50d/thefuck/shells/generic.py#L87
        let script_parts = shlex::split(input).unwrap_or_default();

        Self {
            input,
            output,
            input_parts: script_parts,
        }
    }

    pub fn input_parts(&self) -> &[String] {
        self.input_parts.deref()
    }
}
