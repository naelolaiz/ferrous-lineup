//! Listening to recordings: reading audio files, and hearing in them the
//! notes someone sings or whistles, the bass line, the drums, and the
//! chords, for the takes the app records.
//!
//! - [`decode`] reads a file into samples, with Symphonia.
//! - [`voice`] hears the notes of one voice: when each starts and ends,
//!   its pitch, and how loud it was sung.
//! - [`bass`] hears the bass line: each note's start, end, pitch and loudness.
//! - [`drums`] hears the drums: when each drum is hit, and how hard.
//! - [`chords`] hears the chords: which, and from when to when.
//! - [`wav`] saves samples as a WAV file, to keep a take.

pub mod bass;
pub mod chords;
pub mod decode;
pub mod drums;
mod spectrum;
pub mod voice;
pub mod wav;

pub use decode::{DecodeError, Recording, decode_file};

/// The file extensions [`decode_file`] can read.
pub const AUDIO_EXTENSIONS: [&str; 4] = ["mp3", "wav", "flac", "ogg"];

/// Whether `path` names an audio file, going by its extension.
pub fn is_audio_file(path: &std::path::Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            AUDIO_EXTENSIONS
                .iter()
                .any(|known| extension.eq_ignore_ascii_case(known))
        })
}
