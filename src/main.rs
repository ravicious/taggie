use std::env;
use std::process::{self};
use taggie::editor;
use taggie::tag;

fn main() -> Result<(), std::io::Error> {
    let editor = env::var("VISUAL")
        .or_else(|_| env::var("EDITOR"))
        .unwrap_or_else(|_| "vi".to_string());

    let list_of_tags = tag::get_list_of_tags()?;

    let output = editor::edit_content(&editor, &list_of_tags).unwrap_or_else(|e| {
        eprintln!("Editing the file failed: {}", e);
        process::exit(1);
    });

    println!("{}", output);

    Ok(())
}
