use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new("(?i)a branch named '.*' already exists").unwrap();
}

/// Suggests to checkout a branch instead of create one if it already exists.
pub(crate) struct GitCheckoutExists;
impl Rule for GitCheckoutExists {
    default_rule_id!(GitCheckoutExists);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command.input_parts().iter().any(|p| p == "checkout")
            && command.input_parts().iter().any(|p| p == "-b")
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let mut new_command = command.input_parts().to_vec();
        let b_flag_pos = new_command.iter().position(|p| p == "-b")?;
        new_command.remove(b_flag_pos);
        Some(vec![new_command.into()])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_git_checkout_exists() {
        assert!(basic_corrections(
            "git checkout -b master",
            "fatal: a branch named 'main' already exists"
        )
        .contains(&"git checkout master".to_owned()))
    }
}
