use std::env;
use std::fs;
use std::io::{self};
use std::path::Path;

pub fn get_list_of_tags() -> Result<String, io::Error> {
    let dir = env::current_dir()?;
    let entries: Result<Vec<_>, _> = fs::read_dir(dir)?.collect();

    let tags: String = entries?
        .iter()
        .filter(|entry| {
            entry
                .metadata()
                .map_or(false, |metadata| metadata.is_file())
        })
        .filter_map(|entry| read_from_path(&entry.path()))
        .map(|tag| {
            format!(
                "{}\t{}\n",
                tag.title().unwrap_or_default(),
                tag.artist().unwrap_or_default()
            )
        })
        .collect();

    Ok(format!("{}\n{}", "Title\tArtist", tags))
}

trait Tag: std::fmt::Debug {
    fn artist(&self) -> Option<&str>;
    fn album(&self) -> Option<&str>;
    fn title(&self) -> Option<&str>;
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
