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
            // path_from_output is always an absolute path, while path_from_input could be relative or absolute (user-defined)
            let path_from_output = RE
                .captures(command.output)
                .and_then(|captures| captures.get(1))
                .map(|regex_match| Path::new(regex_match.as_str()))?;

            // We want to correct the path the user provided, but the input could have contained flags, so we
            // strategically find the part of the input that is the path (the output always contains an absolute path).
            // To find this part, we know that the absolute path (i.e. path_from_output) must end with the input path,
            // e.g. open foo => The file /Users/bob/foo does not exist
            //      open /Users/bob/foo => The file /Users/bob/foo does not exist
            let path_from_input = command
                .input_parts()
                .iter()
                .find(|part| path_from_output.ends_with(part))?;

            let corrected_path =
                correct_path_at_every_level(path_from_input, command.working_dir?, |path| {
                    path.exists()
                });

            match corrected_path {
                Some(path) => {
                    return new_commands_from_suggestions(
                        [path],
                        command.input_parts(),
                        path_from_input,
                    );
                }
                None => {
                    let open_arg_string = path_from_output.to_str()?;
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
    fn test_open_with_relative() {
        with_temp_directories(SAMPLE_DIR_PATHS, |tmpdir| {
            let output = format!(
                "The file {}/aples/banannas/oranges/mans does not exist.",
                tmpdir.path().to_str().unwrap()
            );
            let command = Command::new(
                "open aples/banannas/oranges/mans",
                output.as_str(),
                ExitCode(1),
            )
            .set_working_dir(tmpdir.path().to_str().unwrap());

            assert!(regular_corrections(command, &SessionMetadata::default())
                .contains(&"open apples/bananas/oranges/mangos".to_owned()))
        });
    }

    #[test]
    fn test_open_with_absolute() {
        with_temp_directories(SAMPLE_DIR_PATHS, |tmpdir| {
            let input_absolute_path = format!(
                "open {}/aples/banannas/oranges/mans",
                tmpdir.path().to_str().unwrap()
            );
            let output = format!(
                "The file {}/aples/banannas/oranges/mans does not exist.",
                tmpdir.path().to_str().unwrap()
            );
            let command = Command::new(&input_absolute_path, &output, ExitCode(1))
                .set_working_dir(tmpdir.path().to_str().unwrap());
            let correct_output = format!(
                "open {}/apples/bananas/oranges/mangos",
                tmpdir.path().to_str().unwrap()
            );

            assert!(
                regular_corrections(command, &SessionMetadata::default()).contains(&correct_output)
            );
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
