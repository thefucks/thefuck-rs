/// Returns new commands where the to_replace string in
/// input is replaced with the suggestions.
pub(crate) fn new_commands_from_suggestions<'a>(
    suggestions: impl IntoIterator<Item = &'a str>,
    input_parts: &[String],
    to_replace: &str,
) -> Option<Vec<String>> {
    let mut replacement = input_parts.to_vec();
    let replacement_index = input_parts.iter().position(|part| part == to_replace)?;

    Some(
        suggestions
            .into_iter()
            .filter_map(|cmd| {
                let cmd = cmd.trim();
                if !cmd.is_empty() {
                    *replacement.get_mut(replacement_index)? = cmd.to_owned();
                    Some(replacement.join(" "))
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
    use crate::Command;

    #[test]
    fn test_new_commands_from_suggestions() {
        let command = Command::new("git p", "bogus");
        let suggestions = ["push", "pull"];
        assert_eq!(
            new_commands_from_suggestions(suggestions, command.input_parts(), "p"),
            Some(vec![String::from("git push"), String::from("git pull")])
        );
    }

    #[test]
    fn test_new_commands_from_suggestions_with_none_to_replace() {
        let command = Command::new("git p", "bogus");
        let suggestions = ["push", "pull"];
        assert_eq!(
            new_commands_from_suggestions(suggestions, command.input_parts(), "w"),
            None
        );
    }
}
