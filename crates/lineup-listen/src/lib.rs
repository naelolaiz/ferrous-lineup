//! Listening to recordings: reading audio files, and hearing the notes
//! someone sings or whistles in them, for the takes the app records.
//!
//! - [`decode`] reads a file into samples, with Symphonia.
//! - [`voice`] hears the notes of one voice: when each starts and ends,
//!   its pitch, and how loud it was sung.
//! - [`wav`] saves samples as a WAV file, to keep a take.

pub mod decode;
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
