use std::{fs, path::Path};

use tempfile::{tempdir, TempDir};

use crate::{correct_command, Command, ExitCode, SessionMetadata};

/// Doesn't make use of any session metadata.
pub fn basic_corrections(input: &str, output: &str) -> Vec<String> {
    let metadata = SessionMetadata::new();
    let command = Command::new(input, output, ExitCode(1));
    regular_corrections(command, &metadata)
}

/// Same API as `correct_command` but simpler return type.
pub fn regular_corrections(command: Command, metadata: &SessionMetadata) -> Vec<String> {
    correct_command(command, metadata)
        .into_iter()
        .map(|correction| correction.command)
        .collect()
}

pub fn with_temp_directories(dir_paths: &[impl AsRef<Path>], test: impl Fn(TempDir)) {
    let tmpdir = tempdir().unwrap();
    for path in dir_paths {
        fs::create_dir_all(tmpdir.path().join(path.as_ref())).unwrap();
    }

    test(tmpdir)
}
