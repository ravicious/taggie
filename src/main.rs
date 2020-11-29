use std::env;
use std::process::{self};
use taggie::editor;
use taggie::{AudioFile, UpdateError};

fn main() -> Result<(), std::io::Error> {
    let editor = env::var("VISUAL")
        .or_else(|_| env::var("EDITOR"))
        .unwrap_or_else(|_| "vi".to_string());

    let mut audio_files = AudioFile::from_current_dir()?;

    let output = editor::edit_content(
        &editor,
        &AudioFile::collection_to_editable_content(&audio_files),
    )
    .unwrap_or_else(|e| {
        eprintln!("Editing the file failed: {}", e);
        process::exit(1);
    });

    AudioFile::update_tags_from_edited_content(audio_files.as_mut_slice(), output).unwrap_or_else(
        |e| {
            if let UpdateError::UpdateAborted = e {
                eprintln!("Update aborted");
            } else {
                eprintln!("Updating tags failed: {}", e);
            }
            process::exit(1);
        },
    );

    Ok(())
}
