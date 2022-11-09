use crate::rules::util::{correct_path_at_every_level, new_commands_from_suggestions};
use std::path::Path;

use crate::rules::Rule;
use crate::{default_rule_id, Command, RuleCorrection, SessionMetadata};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?i)the file (.+) does not exist").unwrap();
}

fn is_bad_url(input: &str) -> bool {
    (input.contains(".com")
        || input.contains(".edu")
        || input.contains(".info")
        || input.contains(".io")
        || input.contains(".ly")
        || input.contains(".me")
        || input.contains(".net")
        || input.contains(".org")
        || input.contains(".se")
        || input.contains("www."))
        && (!input.contains("http://") && !input.contains("https://"))
}

// Open files or url's
// Ref: https://github.com/nvbn/thefuck/blob/master/thefuck/rules/open.py
pub(crate) struct OpenDoesNotExist;
impl Rule for OpenDoesNotExist {
    default_rule_id!(OpenDoesNotExist);

    fn matches(&self, command: &Command, _session_metadata: &SessionMetadata) -> bool {
        // These two strings are specific to macOS output
        RE.is_match(command.lowercase_output())
    }

    fn generate_command_corrections<'a>(
        &self,
        command: &'a Command,
        session_metadata: &'a SessionMetadata,
    ) -> Option<Vec<RuleCorrection<'a>>> {
        // Check if opening an URL
        if let Some((url_pos, url)) = command
            .input_parts()
            .iter()
            .enumerate()
            .find(|(_index, p)| is_bad_url(p))
        {
            let mut replacement = command.input_parts().to_vec();
            *replacement.get_mut(url_pos)? = "http://".to_owned() + url;
            Some(vec![replacement.into()])
        } else if session_metadata.session_type.is_local() {
            // Check for a file / dir
            let open_arg = RE
                .captures(command.output)
                .and_then(|captures| captures.get(1))
                .map(|regex_match| Path::new(regex_match.as_str()))?;

            let corrected_path =
                correct_path_at_every_level(open_arg, command.working_dir?, |path| path.exists());

            match corrected_path {
                Some(path) => {
                    return new_commands_from_suggestions(
                        [path],
                        command.input_parts(),
                        open_arg.to_str()?,
                    );
                }
                None => {
                    let open_arg_string = open_arg.to_str()?;
                    let touch_replacement = vec!["touch", open_arg_string];
                    let mkdir_replacement = vec!["mkdir", open_arg_string];
                    Some(vec![touch_replacement.into(), mkdir_replacement.into()])
                }
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        test_utils::{basic_corrections, regular_corrections, with_temp_directories},
        Command, ExitCode, SessionMetadata,
    };

    const SAMPLE_DIR_PATHS: &[&str] = &["apples/bananas/oranges/mangos"];

    #[test]
    fn test_open_url() {
        assert_eq!(
            basic_corrections(
                "open github.com",
                "The file ~/github.com does not exist.
                Perhaps you meant 'http://github.com'?"
            ),
            vec!["open http://github.com"]
        )
    }

    #[test]
    fn test_open_correcting_path() {
        with_temp_directories(SAMPLE_DIR_PATHS, |tmpdir| {
            let command = Command::new(
                "open aples/banannas/oranges/mans",
                "The file aples/banannas/oranges/mans does not exist.",
                ExitCode(1),
            )
            .set_working_dir(tmpdir.path().to_str().unwrap());

            assert!(regular_corrections(command, &SessionMetadata::default())
                .contains(&"open apples/bananas/oranges/mangos".to_owned()))
        });
    }

    #[test]
    fn test_open_no_file_exist() {
        with_temp_directories(SAMPLE_DIR_PATHS, |tmpdir| {
            let command = Command::new(
                "open apples/bananas/beef",
                "The file apples/bananas/beef does not exist.",
                ExitCode(1),
            )
            .set_working_dir(tmpdir.path().to_str().unwrap());

            let res = regular_corrections(command, &SessionMetadata::default());
            assert_eq!(res[0], "touch apples/bananas/beef");
            assert_eq!(res[1], "mkdir apples/bananas/beef");
        });
    }
}
