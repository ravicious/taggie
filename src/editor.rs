use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::process::{Command, ExitStatus};

pub enum EditorError {
    IoError(std::io::Error),
    NonZeroExitStatus(ExitStatus),
}

impl From<io::Error> for EditorError {
    fn from(error: io::Error) -> Self {
        EditorError::IoError(error)
    }
}

impl fmt::Display for EditorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EditorError::IoError(error) => write!(f, "{}", error),
            EditorError::NonZeroExitStatus(status) => write!(
                f,
                "Editor exited with {} exit code",
                status.code().map_or("no".to_string(), |x| x.to_string())
            ),
        }
    }
}

pub fn edit_content(editor: &str, content: &str) -> Result<String, EditorError> {
    let mut file = tempfile::Builder::new()
        .prefix("taggie-")
        .suffix(".tsv")
        .rand_bytes(5)
        .tempfile()?;
    file.write_all(content.as_bytes())?;

    let path = file.into_temp_path();

    let status = Command::new(&editor)
        .arg(path.to_str().unwrap())
        .status()
        .unwrap_or_else(|_| panic!("Failed to start editor ({})", editor));

    if !status.success() {
        return Err(EditorError::NonZeroExitStatus(status));
    }

    fs::read_to_string(path).map_err(EditorError::from)
}
