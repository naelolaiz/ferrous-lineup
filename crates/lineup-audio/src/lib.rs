//! The real-time audio engine for Ferrous Lineup.
//!
//! - [`instrument`] holds the sound generators (the sampler and the Lineup
//!   Osc synth) and the MIDI Out, [`timbre`] the Lineup Osc's sounds for the
//!   General MIDI instruments, [`soundfont`] plays real instrument sounds
//!   from a SoundFont file, [`drums`] synthesizes the built-in drum kit and [`dsp`]
//!   has the small filters and smoothers they share. They are plain
//!   structs that compute samples, so they can be tested without a sound card.
//! - `metronome` makes the metronome's clicks and times the count-in,
//!   [`effects`] is each channel's equalizer, compressor and delay, and
//!   `reverb` is the master reverb each channel sends to.
//! - [`sample`] loads WAV files.
//! - [`phrase`] plays a few notes apart from the song, on instruments of
//!   their own: the questions of the ear training.
//! - [`engine`] opens the sound card with `cpal`, runs the sequencer and
//!   the instruments on the audio thread, and receives [`Command`]s from
//!   the UI thread and keys from a MIDI keyboard.
//! - [`input`] records a take from the microphone while the song plays,
//!   and places it in the song.
//! - [`render`] runs the same audio thread without a sound card, to save
//!   a song as a WAV file.
//! - [`trace`] writes down every note the audio thread plays, to check
//!   playback by reading instead of listening.
//! - [`levels`] measures how loud each channel and the master play, for
//!   the mixer's meters.

pub mod drums;
pub mod dsp;
pub mod effects;
pub mod engine;
pub mod input;
pub mod instrument;
pub mod levels;
mod metronome;
pub mod phrase;
pub mod render;
mod reverb;
pub mod sample;
pub mod soundfont;
pub mod timbre;
pub mod trace;

pub use effects::DelayLine;
pub use engine::{
    AudioEngine, ChannelNotes, Command, EngineError, EngineInfo, Heard, LapTempos, LiveInput,
    MidiOutput, PlayMode, SongClock,
};
pub use instrument::Instrument;
pub use levels::Levels;
pub use phrase::{Phrase, PhraseNote};
pub use render::{Renderer, WavExport, export_wav, export_wav_cancellable};
pub use sample::{SampleCache, SampleData, SampleError};
pub use soundfont::{SoundFont, SoundFontError};
pub use timbre::{Timbre, Wave};
pub use trace::{Trace, TraceReader, TracedNote, Tracer, trace_queue};
