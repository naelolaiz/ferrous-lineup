//! Core music types for Ferrous Loom.
//!
//! This crate knows about notes, tempo and (later) patterns and harmony.
//! It has no audio and no UI code, which keeps it small, fast to compile
//! and easy to test with plain `cargo test`.

pub mod pitch;
pub mod tempo;

// Re-export the main types so other crates can write `loom_core::Note`
// instead of `loom_core::pitch::Note`.
pub use pitch::Note;
pub use tempo::Tempo;
