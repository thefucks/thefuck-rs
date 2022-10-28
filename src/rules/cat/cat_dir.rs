use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new("(?i)cat: (.+): is a directory").unwrap();
}

/// Replaces cat with ls when command tries to cat a directory.
/// See more here: https://github.com/nvbn/thefuck/blob/5198b34f24ca4bc414a5bf1b0288ee86ea2529a8/thefuck/rules/cat_dir.py
pub(crate) struct CatDir;
impl Rule for CatDir {
    default_rule_id!(CatDir);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        // Note: thefuck does this by checking that the second input part is a directory itself.
        // This can be slightly problematic if there are options between cat and the arg (e.g. `cat -b src`).
        RE.is_match(command.output)
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let dirname = RE
            .captures(command.output)
            .and_then(|captures| captures.get(1))
            .map(|regex_match| regex_match.as_str())?;

        // We can discard all other options since they won't be applicable to `ls`
        let new_command = vec!["ls".to_owned(), dirname.to_owned()];
        Some(vec![new_command.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_cat_dir() {
        assert_eq!(
            basic_corrections("cat -b src", "cat: src: Is a directory"),
            vec!["ls src"]
        )
    }

    #[test]
    fn test_cat_dir_with_spaces() {
        assert_eq!(
            basic_corrections("cat foo\\ bar", "cat: foo bar: Is a directory"),
            vec![r#"ls "foo bar""#]
        )
    }
}
