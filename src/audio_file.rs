use std::env;
use std::fmt;
use std::fs;
use std::io::{self};
use std::path::{Path, PathBuf};

pub struct AudioFile {
    path: PathBuf,
    tag: Box<dyn Tag>,
}

impl AudioFile {
    pub fn new(path: PathBuf) -> Result<Self, io::Error> {
        let is_file = path
            .metadata()
            .map(|metadata| metadata.is_file())
            .unwrap_or(false);

        if !is_file {
            return Err(io::Error::new(io::ErrorKind::Other, "not a file"));
        }

        let tag = Self::read_from_path(&path)
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "not an audio file"))?;

        Ok(AudioFile { path, tag })
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
        format!(
            "{}\t{}\n",
            self.tag.title().unwrap_or_default(),
            self.tag.artist().unwrap_or_default()
        )
    }

    pub fn update_tag_from_line(&mut self, line: &str) -> Result<(), UpdateError> {
        if let [title, artist] = line.split('\t').collect::<Vec<_>>()[..] {
            self.tag.set_title(title);
            self.tag.set_artist(artist);
            self.tag
                .write_to_path(&self.path)
                .map_err(UpdateError::ExternalCrateError)
        } else {
            Err(UpdateError::InvalidLine(line.to_string()))
        }
    }

    fn read_from_path(path: &Path) -> Option<Box<dyn Tag>> {
        match path
            .extension()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or_default()
            .to_lowercase()
            .as_str()
        {
            "mp3" => id3::Tag::read_from_path(path)
                .map(|tag| Box::new(tag) as Box<dyn Tag>)
                .ok(),
            "m4a" | "m4b" | "m4p" | "m4v" => mp4ameta::Tag::read_from_path(path)
                .map(|tag| Box::new(tag) as Box<dyn Tag>)
                .ok(),
            _ => None,
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
    ExternalCrateError(ExternalCrateError),
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
            ExternalCrateError(error) => write!(f, "Error from external library {:?}", error),
        }
    }
}

fn pluralize(count: usize, singular: &str, plural: &str) -> String {
    format!("{} {}", count, if count == 1 { singular } else { plural })
}

#[derive(Debug)]
pub enum ExternalCrateError {
    Id3(id3::Error),
    Mp4ameta(mp4ameta::Error),
}

trait Tag: std::fmt::Debug {
    fn artist(&self) -> Option<&str>;
    fn album(&self) -> Option<&str>;
    fn title(&self) -> Option<&str>;

    fn set_artist(&mut self, new_value: &str);
    fn set_album(&mut self, new_value: &str);
    fn set_title(&mut self, new_value: &str);

    fn write_to_path(&self, path: &Path) -> Result<(), ExternalCrateError>;
}

impl Tag for id3::Tag {
    fn artist(&self) -> Option<&str> {
        self.artist()
    }
    fn album(&self) -> Option<&str> {
        self.album()
    }
    fn title(&self) -> Option<&str> {
        self.title()
    }

    fn set_artist(&mut self, new_value: &str) {
        self.set_artist(new_value);
    }
    fn set_album(&mut self, new_value: &str) {
        self.set_album(new_value);
    }
    fn set_title(&mut self, new_value: &str) {
        self.set_title(new_value);
    }

    fn write_to_path(&self, path: &Path) -> Result<(), ExternalCrateError> {
        self.write_to_path(path, id3::Version::Id3v24)
            .map_err(ExternalCrateError::Id3)
    }
}

impl Tag for mp4ameta::Tag {
    fn artist(&self) -> Option<&str> {
        self.artist()
    }
    fn album(&self) -> Option<&str> {
        self.album()
    }
    fn title(&self) -> Option<&str> {
        self.title()
    }

    fn set_artist(&mut self, new_value: &str) {
        self.set_artist(new_value);
    }
    fn set_album(&mut self, new_value: &str) {
        self.set_album(new_value);
    }
    fn set_title(&mut self, new_value: &str) {
        self.set_title(new_value);
    }

    fn write_to_path(&self, path: &Path) -> Result<(), ExternalCrateError> {
        self.write_to_path(path)
            .map_err(ExternalCrateError::Mp4ameta)
    }
}
