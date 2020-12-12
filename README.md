# Taggie

Edit audio tags in your favorite text editor!

Here's how it works:

1. You run `taggie` in a shell, from a directory which contains audio files you want to edit.
2. Taggie opens your favorite editor with the title and artist tags separated by tabs.
3. You edit the tags, save the file and exit the editor.
4. Taggie updates the tags according to your changes.

## Supported audio formats and tags

Taggie edits tags through [TagLib](https://taglib.org/), so it aims to support whatever TagLib
supports:

> Currently it supports both ID3v1 and ID3v2 for MP3 files, Ogg Vorbis comments and ID3 tags and
> Vorbis comments in FLAC, MPC, Speex, WavPack, TrueAudio, WAV, AIFF, MP4 and ASF files.

When it comes to editable tags, right now only title and artist tags are available with [track number
on the way](https://github.com/ravicious/taggie/issues/7).

## Installation

There are two prerequisites: TagLib and Cargo.

On macOS, you can install TagLib with `brew install taglib`. Check out [taglib-rust
README](https://github.com/ebassi/taglib-rust/blob/8395821edc9950462c8274a81dc7e0da0b305a42/README.md#requirements)
for a list of Linux packages for different distros. [TagLib website](https://taglib.org/) may
contain some other helpful info as well.

[Cargo](https://doc.rust-lang.org/cargo/) is the Rust package manager. If you're a developer,
installing it on your computer should be rather straightforward. Check [the Installation chapter
from The Cargo Book](https://doc.rust-lang.org/cargo/getting-started/installation.html) for more
details.

## Rationale

From time to time I download a release from Bandcamp where the tags are messed up, especially when
we're talking about compilation albums from various artists. [MusicBrainz
Picard](https://picard.musicbrainz.org/) doesn't help there if the release is fresh.

It's easy enough to modify the album or album artist tags in iTunes because it's setting one value
for all tracks. However, sometimes there's more you need to change: the "title" tag is in the format
"[title] - [artist]" or each title contains some junk that you want to remove.

This requires some text processing capabilities, and—if you're a developer—what's better for text
processing than your favorite text editor?

## FAQ

### How do I configure the editor which Taggie uses?

Taggie inspects the `VISUAL` and `EDITOR` environment variables before defaulting to `vi`. Change
one of those variables, preferably `VISUAL`.

### How do I use my GUI editor with Taggie?

Taggie integrates with a text editor in a way that's similar to how `git commit` does it. Usually it
involves setting the editor's CLI tool with some additional options as the `VISUAL` env variable.
Search for how to set your editor as the default commit editor for git.

### Can I pass a path to a directory as an argument?

That's currently not supported – make sure you're in the target directory before running `taggie`
from there. I can add this option if there's a use case for it.

### Taggie doesn't see the tags in certain M4A files, why is that?

I found that tracks bought from iTunes have the "sort artist" and "sort name" tags filled out
instead of "artist" and "title". See [issue #2](https://github.com/ravicious/taggie/issues/2) for
the progress on this or to suggest a solution.

### Are you going to add support for more tags?

From the UX standpoint of editing one file per line, I think it only makes sense to add tags which
are usually unique to a single track. "artist", "title" and "track number" seem to handle most
common use cases. I'm open to suggestions though.
