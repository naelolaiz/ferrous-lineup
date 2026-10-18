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
//! - [`rows`]: twelve-tone styles, ways of playing a row.
//! - [`micromodes`]: styles in one of Kröpfl's micromodes.
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
pub mod interpret;
pub mod listen;
pub mod micromodes;
pub mod numeral;
pub mod rhythm;
pub mod rows;
pub mod song;
pub mod style;
pub mod user_style;
pub mod voicing;
pub use user_style::{StyleLane, UserStyle};

pub use comping::{Bass, Comping};
pub use micromodes::{MICROMODE_STYLES, MicromodeStyle};
pub use rhythm::{FILLS, RHYTHMS, Rhythm};
pub use rows::{ROW_STYLES, RowStyle};
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
    Folklore,
    Flamenco,
    Balkan,
    Reggae,
    Folk,
    Classical,
}

impl Genre {
    /// All genres, in the order the library lists them.
    pub const ALL: [Genre; 15] = [
        Genre::Pop,
        Genre::Rock,
        Genre::Blues,
        Genre::Jazz,
        Genre::Soul,
        Genre::HipHop,
        Genre::Electronic,
        Genre::Latin,
        Genre::Tango,
        Genre::Folklore,
        Genre::Flamenco,
        Genre::Balkan,
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
            Genre::Folklore => "Latin American folklore",
            Genre::Flamenco => "Flamenco",
            Genre::Balkan => "Balkan and Turkish",
            Genre::Reggae => "Reggae and ska",
            Genre::Folk => "Country and folk",
            Genre::Classical => "Classical and film",
        }
    }
}
