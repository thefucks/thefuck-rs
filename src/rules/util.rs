use crate::Correction;

/// Returns new commands where the to_replace string in
/// input is replaced with the suggestions.
pub(crate) fn new_commands_from_suggestions<'a>(
    suggestions: impl IntoIterator<Item = &'a str>,
    input_parts: &[String],
    to_replace: &str,
) -> Option<Vec<Correction<'a>>> {
    let replacement = input_parts.to_vec();
    let replacement_index = input_parts.iter().position(|part| part == to_replace)?;

    Some(
        suggestions
            .into_iter()
            .filter_map(|cmd| {
                let cmd = cmd.trim();
                if !cmd.is_empty() {
                    let mut new_command = replacement.clone();
                    *new_command.get_mut(replacement_index)? = cmd.to_owned();
                    Some(new_command.into())
                } else {
                    None
                }
            })
            .collect(),
    )
}

#[cfg(test)]
mod test {
    use crate::rules::util::new_commands_from_suggestions;
    use crate::{Command, ExitCode};

    #[test]
    fn test_new_commands_from_suggestions() {
        let command = Command::new("git p", "bogus", ExitCode(0));
        let suggestions = ["push", "pull"];
        let corrections = new_commands_from_suggestions(suggestions, command.input_parts(), "p");
        assert_eq!(
            corrections,
            Some(vec![vec!["git", "push"].into(), vec!["git", "pull"].into()])
        );
    }

    #[test]
    fn test_new_commands_from_suggestions_with_none_to_replace() {
        let command = Command::new("git p", "bogus", ExitCode(0));
        let suggestions = ["push", "pull"];
        assert_eq!(
            new_commands_from_suggestions(suggestions, command.input_parts(), "w"),
            None
        );
    }
}
