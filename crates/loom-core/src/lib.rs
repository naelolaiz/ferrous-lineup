//! Core music types for Ferrous Loom.
//!
//! This crate knows about notes, tempo, patterns and the project as a
//! whole. It has no audio and no UI code, which keeps it small, fast to
//! compile and easy to test with plain `cargo test`.

pub mod pattern;
pub mod pitch;
pub mod project;
pub mod sequencer;
pub mod tempo;

// Re-export the main types so other crates can write `loom_core::Note`
// instead of `loom_core::pitch::Note`.
pub use pattern::{MAX_STEPS, PATTERN_LENGTHS, Pattern, StepRow};
pub use pitch::Note;
pub use project::{Channel, ChannelKind, Drum, MAX_CHANNELS, Project, SampleSource};
pub use sequencer::Sequencer;
pub use tempo::Tempo;
