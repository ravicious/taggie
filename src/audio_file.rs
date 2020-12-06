use std::env;
use std::fmt;
use std::fs;
use std::io::{self};
use std::path::PathBuf;

pub struct AudioFile {
    path: PathBuf,
    file: taglib::File,
}

pub enum FileError {
    NotAFile,
    TaglibError(taglib::FileError),
}

impl From<taglib::FileError> for FileError {
    fn from(error: taglib::FileError) -> Self {
        FileError::TaglibError(error)
    }
}

impl AudioFile {
    pub fn new(path: PathBuf) -> Result<Self, FileError> {
        let is_file = path
            .metadata()
            .map(|metadata| metadata.is_file())
            .unwrap_or(false);

        if !is_file {
            return Err(FileError::NotAFile);
        }

        let file = taglib::File::new(&path)?;

        Ok(AudioFile { path, file })
    }

    pub fn collection_to_editable_content(audio_files: &[AudioFile]) -> String {
        let lines_with_tags: String = audio_files
            .iter()
            .map(AudioFile::to_editable_content_line)
            .collect();

        format!(
            "{}\n{}",
            "Title\tArtist\t(Remove all lines to abort the update)", lines_with_tags
        )
    }

    pub fn from_current_dir() -> Result<Vec<AudioFile>, io::Error> {
        let dir = env::current_dir()?;
        let entries: Result<Vec<_>, _> = fs::read_dir(dir)?.collect();

        Ok(entries?
            .iter()
            .filter_map(|entry| AudioFile::new(entry.path()).ok())
            .collect())
    }

    pub fn update_tags_from_edited_content(
        audio_files: &mut [AudioFile],
        content: String,
    ) -> Result<(), UpdateError> {
        if content.trim().is_empty() {
            return Err(UpdateError::UpdateAborted);
        }

        let number_of_lines = content.lines().count() - 1; // -1 for the header line

        if audio_files.len() != number_of_lines {
            return Err(UpdateError::LineNumberMismatch {
                number_of_files: audio_files.len(),
                number_of_lines,
            });
        }

        for (audio_file, line) in audio_files.iter_mut().zip(
            // Skip the header line.
            content.lines().skip(1),
        ) {
            audio_file.update_tag_from_line(line)?
        }

        Ok(())
    }

    pub fn to_editable_content_line(&self) -> String {
        let tag = self
            .file
            .tag()
            .expect("Failed to get tag inside to_editable_content_line");

        format!(
            "{}\t{}\n",
            tag.title().unwrap_or_default(),
            tag.artist().unwrap_or_default()
        )
    }

    pub fn update_tag_from_line(&mut self, line: &str) -> Result<(), UpdateError> {
        let mut tag = match self.file.tag() {
            Ok(tag) => tag,
            Err(e) => return Err(UpdateError::CannotReadTag(e)),
        };

        if let [title, artist] = line.split('\t').collect::<Vec<_>>()[..] {
            tag.set_title(title);
            tag.set_artist(artist);

            if self.file.save() {
                Ok(())
            } else {
                Err(UpdateError::SaveFailed(self.path.clone()))
            }
        } else {
            Err(UpdateError::InvalidLine(line.to_string()))
        }
    }
}

#[derive(Debug)]
pub enum UpdateError {
    UpdateAborted,
    LineNumberMismatch {
        number_of_files: usize,
        number_of_lines: usize,
    },
    InvalidLine(String),
    CannotReadTag(taglib::FileError),
    SaveFailed(PathBuf),
}

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use UpdateError::*;

        match self {
            UpdateAborted => write!(f, "Update aborted"),
            LineNumberMismatch {
                number_of_files,
                number_of_lines,
            } => write!(
                f,
                "Found {}, so expected {}, but found only {}",
                pluralize(*number_of_files, "file", "files"),
                pluralize(*number_of_files, "line", "lines"),
                pluralize(*number_of_lines, "line", "lines"),
            ),
            InvalidLine(invalid_line) => write!(
                f,
                "Expected the line to have format\n\n\t`title<TAB>artist`\n\nbut found this instead:\n\n\t{}",
                invalid_line.replace('\t', "<TAB>")
            ),
            SaveFailed(path) => write!(f, "Failed to save updated tags to file {:?}", path.clone().into_os_string()),
            CannotReadTag(error) => write!(f, "Failed to read tags from the file {:?}", error)
        }
    }
}

fn pluralize(count: usize, singular: &str, plural: &str) -> String {
    format!("{} {}", count, if count == 1 { singular } else { plural })
}
