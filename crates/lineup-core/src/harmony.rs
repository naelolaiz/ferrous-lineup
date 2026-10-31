//! Music theory: scales, chords, and naming what is being played.
//!
//! - [`PitchSet`] is a set of pitch classes, the basic building block.
//! - [`Scale`] is a root and a [`ScaleKind`], such as "A minor".
//! - [`Chord`] is a root and a [`ChordQuality`], such as "Am7".
//! - [`ChordSymbol`] is a chord as a lead sheet writes it, read from
//!   text: "F#m7b5", "Bb13", "G/B".
//! - [`analyze`] names a group of notes: a single note, an interval or a chord.
//! - [`hearing`] names the harmony of a stretch of music, with its
//!   arpeggios, passing notes and chords played over other chords.
//! - [`ChordStamp`] says what the piano roll places with one click.
//! - [`next_chords`] suggests chords that often come next in a key.
//! - [`HintLevel`] says how much the hints explain.
//! - [`keys`] are the harmony keys: the seven chords of a key, to play.
//! - [`ToneRow`] is a twelve-tone row, and [`SetClass`] names any group
//!   of notes, chord or not: music after tonality.
//! - [`Micromode`] is one of Francisco Kröpfl's twelve groups of three
//!   notes, the smallest sets, which larger sets are made of.
//! - [`Degree`] names a form of a set by where it sits in a key made of
//!   that set, as a Roman numeral names a chord in a key: `T2`, `T5I`.
//! - [`key_fits`] finds the key a piece is in from how long each note
//!   sounds.
//! - [`voice_leading`] checks how the parts move from chord to chord:
//!   parallel fifths and octaves, leading tones and sevenths.
//! - [`voice_motion`] says how each part moves there: by step, skip or
//!   leap, and in which motion against the others.

mod analysis;
mod chord;
pub(crate) mod chord_scale;
pub mod degree;
pub mod hearing;
mod hints;
pub mod key_finding;
pub mod keys;
mod micromode;
mod pitch_set;
mod progression;
mod row;
mod scale;
mod set_class;
mod symbol;
pub mod voice_leading;
pub mod voice_motion;

pub use analysis::{
    Harmony, analyze, describe_chord, interval_name, interval_with_article, shown_interval,
    spelled_interval_name,
};
pub use chord::{Chord, ChordFamily, ChordQuality, ChordStamp};
pub use chord_scale::{chord_scale, chord_scale_name};
pub use degree::Degree;
pub use hearing::{HeardNote, Hearing, Listening, Reading, hear};
pub use hints::HintLevel;
pub use key_finding::key_fits;
pub use micromode::{Micromode, MicromodeFamily, micromode_content, micromodes_in, trichords};
pub use pitch_set::PitchSet;
pub use progression::{Suggestion, next_chords, next_chords_over};
pub(crate) use row::Random;
pub use row::{ROW_PRESETS, RowForm, RowLabel, RowPreset, ToneRow};
pub use scale::{MinorNumerals, Scale, ScaleKind, with_figures};
pub use set_class::{SetClass, digits, pc_digit};
pub use symbol::ChordSymbol;
