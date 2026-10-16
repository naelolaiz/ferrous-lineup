//! Core music types for Ferrous Loom.
//!
//! This crate knows about notes, tempo, patterns, harmony and the project
//! as a whole. It has no audio and no UI code, which keeps it small, fast
//! to compile and easy to test with plain `cargo test`.

pub mod harmony;
pub mod pattern;
pub mod pitch;
pub mod project;
pub mod sequencer;
pub mod tempo;

// Re-export the main types so other crates can write `loom_core::Note`
// instead of `loom_core::pitch::Note`.
pub use harmony::{Chord, ChordQuality, ChordStamp, Harmony, PitchSet, Scale, ScaleKind};
pub use pattern::{DEFAULT_VELOCITY, MAX_STEPS, NoteEvent, PATTERN_LENGTHS, Pattern, STEP_NOTE};
pub use pitch::{Note, PitchClass, Spelling};
pub use project::{Channel, ChannelKind, Drum, MAX_CHANNELS, Project, SampleSource};
pub use sequencer::{STEPS_PER_BEAT, Sequencer};
pub use tempo::Tempo;
