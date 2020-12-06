use std::time::Instant;
use taggie::AudioFile;

fn main() {
    let mut audio_files = AudioFile::from_current_dir().unwrap();
    let content = AudioFile::collection_to_editable_content(&audio_files);
    let mut lines = content.lines().collect::<Vec<_>>();
    let (header, tags) = lines.split_at_mut(1);
    fastrand::shuffle(tags);
    let output = [header, tags].concat().join("\n");
    let now = Instant::now();
    AudioFile::update_tags_from_edited_content(audio_files.as_mut_slice(), output).unwrap();
    println!(
        "Updated {} tags in {} ms",
        audio_files.len(),
        now.elapsed().as_millis()
    );
}
