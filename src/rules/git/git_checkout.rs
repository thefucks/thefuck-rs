/*
Fixes error for `git checkout branch_that_doesnt_exist` to be `git checkout -b branch_that_doesnt_exist`.
See more here: https://github.com/nvbn/thefuck/blob/5198b34f24ca4bc414a5bf1b0288ee86ea2529a8/thefuck/rules/git_checkout.py
*/

use crate::rules::Rule;
use crate::{Command, Correction};
use lazy_static::lazy_static;
use regex::Regex;

pub(crate) struct GitCheckout;

lazy_static! {
    static ref RE: Regex =
        Regex::new("(?i)error: pathspec '[^']*' did not match any file").unwrap();
}

impl Rule for GitCheckout {
    fn matches(&self, command: &Command) -> bool {
        command.input_parts().iter().any(|part| part == "checkout")
            && RE.is_match(command.lowercase_output())
    }

    fn generate_command_corrections(&self, command: &Command) -> Option<Vec<Correction>> {
        let mut replacement = command.input_parts().to_vec();
        let checkout_pos = replacement.iter().position(|p| p == "checkout")?;
        replacement.insert(checkout_pos + 1, "-b".to_owned());
        Some(vec![replacement.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::command_corrections;

    #[test]
    fn test_git_checkout() {
        assert_eq!(
            command_corrections(
                "git checkout some-branch",
                "error: pathspec 'some-branch' did not match any file(s) known to git"
            ),
            vec!["git checkout -b some-branch"]
        )
    }
}
