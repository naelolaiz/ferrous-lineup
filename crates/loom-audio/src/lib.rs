//! The real-time audio engine for Ferrous Loom.
//!
//! - [`synth`] holds the sound generators. They are plain structs that
//!   fill buffers with samples, so they can be tested without a sound card.
//! - [`engine`] opens the sound card with `cpal`, runs the generators on
//!   the audio thread and receives [`Command`]s from the UI thread.

pub mod engine;
pub mod synth;

pub use engine::{AudioEngine, Command, EngineError, EngineInfo};
