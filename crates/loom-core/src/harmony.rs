//! Music theory: scales, chords, and naming what is being played.
//!
//! - [`PitchSet`] is a set of pitch classes, the basic building block.
//! - [`Scale`] is a root and a [`ScaleKind`], such as "A minor".
//! - [`Chord`] is a root and a [`ChordQuality`], such as "Am7".
//! - [`analyze`] names a group of notes: a single note, an interval or a chord.
//! - [`ChordStamp`] says what the piano roll places with one click.
//! - [`next_chords`] suggests chords that often come next in a key.
//! - [`keys`] are the harmony keys: the seven chords of a key, to play.

mod analysis;
mod chord;
pub mod keys;
mod pitch_set;
mod progression;
mod scale;

pub use analysis::{Harmony, analyze, interval_name};
pub use chord::{Chord, ChordQuality, ChordStamp};
pub use pitch_set::PitchSet;
pub use progression::{Suggestion, next_chords};
pub use scale::{Scale, ScaleKind};
