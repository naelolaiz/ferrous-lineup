//! The library: harmony styles, rhythms and songs, written as data.
//!
//! Everything here is knowledge a musician carries around, written down
//! in small tables:
//!
//! - [`numeral`]: chords as Roman numerals, so a progression works in
//!   every key ("I V vi IV").
//! - [`chart`]: lead sheets as text, with chords by the bar and melodies
//!   note by note.
//! - [`voicing`]: which notes of a chord to play, moving smoothly from
//!   one chord to the next.
//! - [`comping`]: rhythms for playing chords and bass lines.
//! - [`style`]: harmony styles, a progression with its way of playing.
//! - [`rhythm`]: drum patterns, one line per drum.
//! - [`song`]: well-known songs in the public domain, with their melodies.
//! - [`listen`]: hearing which chords a pattern's notes play.
//! - [`apply`]: writing a style or a rhythm into a pattern, and changing
//!   the rhythm of a whole song, chords and bass included.
//! - [`follow`]: the accompaniment playing whatever chord you hold, as on
//!   an arranger keyboard.
//! - [`demo`]: whole projects, arranged from styles, rhythms and songs.

pub mod apply;
pub mod chart;
pub mod comping;
pub mod demo;
pub mod follow;
pub mod listen;
pub mod numeral;
pub mod rhythm;
pub mod song;
pub mod style;
pub mod voicing;

pub use comping::{Bass, Comping};
pub use rhythm::{FILLS, RHYTHMS, Rhythm};
pub use song::{SONGS, Song};
pub use style::{STYLES, Style};

/// The families the library is sorted into.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Genre {
    Pop,
    Rock,
    Blues,
    Jazz,
    Soul,
    HipHop,
    Electronic,
    Latin,
    Tango,
    Flamenco,
    Reggae,
    Folk,
    Classical,
}

impl Genre {
    /// All genres, in the order the library lists them.
    pub const ALL: [Genre; 13] = [
        Genre::Pop,
        Genre::Rock,
        Genre::Blues,
        Genre::Jazz,
        Genre::Soul,
        Genre::HipHop,
        Genre::Electronic,
        Genre::Latin,
        Genre::Tango,
        Genre::Flamenco,
        Genre::Reggae,
        Genre::Folk,
        Genre::Classical,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Genre::Pop => "Pop",
            Genre::Rock => "Rock",
            Genre::Blues => "Blues",
            Genre::Jazz => "Jazz",
            Genre::Soul => "Soul, funk and R&B",
            Genre::HipHop => "Hip hop and lo-fi",
            Genre::Electronic => "Electronic",
            Genre::Latin => "Latin",
            Genre::Tango => "Tango",
            Genre::Flamenco => "Flamenco",
            Genre::Reggae => "Reggae and ska",
            Genre::Folk => "Country and folk",
            Genre::Classical => "Classical and film",
        }
    }
}
