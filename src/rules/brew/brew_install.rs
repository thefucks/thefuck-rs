use crate::rules::util::new_commands_from_suggestions;
use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref WRONG_FORMULA_RE: Regex =
        Regex::new("(?i)No available formula with the name \"(.+)\". Did you mean (?:.+)?")
            .unwrap();
    static ref NEW_FORMULAE_RE: Regex =
        Regex::new("(?i)These similarly named formulae were found:\n((?:.+\n)*).+To install")
            .unwrap();
}

/// Corrects a "brew install <package>" command where package is mis-spelt.
pub(crate) struct BrewInstall;
impl Rule for BrewInstall {
    default_rule_id!(BrewInstall);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        command.input.contains("install")
            && WRONG_FORMULA_RE.is_match(command.lowercase_output())
            && NEW_FORMULAE_RE.is_match(command.lowercase_output())
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        _session_metadata: &SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        let to_fix = WRONG_FORMULA_RE
            .captures(command.output)
            .and_then(|captures| captures.get(1))
            .map(|to_fix| to_fix.as_str())?;

        let corrected_formulae = NEW_FORMULAE_RE
            .captures(command.output)
            .and_then(|captures| captures.get(1))
            .map(|corrected_formulae| {
                corrected_formulae
                    .as_str()
                    .split_whitespace()
                    .collect::<Vec<&str>>()
            })?;

        new_commands_from_suggestions(corrected_formulae, command.input_parts(), to_fix)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::basic_corrections;

    #[test]
    fn test_brew_install() {
        assert_eq!(
            basic_corrections(
                "brew install crome",
                r#"Warning: No available formula with the name "crome". Did you mean rome, croc or chroma?
                ==> Searching for similarly named formulae...
                These similarly named formulae were found:
                rome                                 croc                                 chroma
                drome
                To install one of them, run (for example):
                  brew install rome
                ==> Searching for a previously deleted formula (in the last month)...
                Error: No previously deleted formula found.
                ==> Searching taps on GitHub...
                Warning: Error searching on GitHub: GitHub API Error: Bad credentials
                HOMEBREW_GITHUB_API_TOKEN may be invalid or expired; check:
                  https://github.com/settings/tokens
                
                Error: No formulae found in taps."#
            ),
            vec![
                "brew install rome",
                "brew install croc",
                "brew install chroma",
                "brew install drome",
            ]
        )
    }
}
