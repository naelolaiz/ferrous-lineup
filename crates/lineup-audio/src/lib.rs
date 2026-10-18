//! The real-time audio engine for Ferrous Lineup.
//!
//! - [`instrument`] holds the sound generators (the sampler and the Lineup
//!   Osc synth) and the MIDI Out, [`timbre`] the Lineup Osc's sounds for the
//!   General MIDI instruments, [`soundfont`] plays real instrument sounds
//!   from a SoundFont file, [`drums`] synthesizes the built-in drum kit and [`dsp`]
//!   has the small filters and smoothers they share. They are plain
//!   structs that compute samples, so they can be tested without a sound card.
//! - `metronome` makes the metronome's clicks and times the count-in.
//! - [`sample`] loads WAV files.
//! - [`phrase`] plays a few notes apart from the song, on instruments of
//!   their own: the questions of the ear training.
//! - [`engine`] opens the sound card with `cpal`, runs the sequencer and
//!   the instruments on the audio thread, and receives [`Command`]s from
//!   the UI thread and keys from a MIDI keyboard.
//! - [`render`] runs the same audio thread without a sound card, to save
//!   a song as a WAV file.

pub mod drums;
pub mod dsp;
pub mod engine;
pub mod instrument;
mod metronome;
pub mod phrase;
pub mod render;
mod reverb;
pub mod sample;
pub mod soundfont;
pub mod timbre;

pub use engine::{
    AudioEngine, ChannelNotes, Command, EngineError, EngineInfo, LiveInput, MidiOutput, PlayMode,
};
pub use instrument::Instrument;
pub use phrase::{Phrase, PhraseNote};
pub use render::{Renderer, export_wav};
pub use sample::{SampleCache, SampleData, SampleError};
pub use soundfont::{SoundFont, SoundFontError};
pub use timbre::{Timbre, Wave};
