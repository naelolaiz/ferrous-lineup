//! The real-time audio engine for Ferrous Loom.
//!
//! - [`instrument`] holds the sound generators (the sampler and the Loom
//!   Osc synth), [`drums`] synthesizes the built-in drum kit and [`dsp`]
//!   has the small filters and smoothers they share. They are plain
//!   structs that compute samples, so they can be tested without a sound card.
//! - [`sample`] loads WAV files.
//! - [`engine`] opens the sound card with `cpal`, runs the sequencer and
//!   the instruments on the audio thread, and receives [`Command`]s from
//!   the UI thread.

pub mod drums;
pub mod dsp;
pub mod engine;
pub mod instrument;
pub mod sample;

pub use engine::{AudioEngine, Command, EngineError, EngineInfo};
pub use instrument::Instrument;
pub use sample::{SampleData, SampleError};
