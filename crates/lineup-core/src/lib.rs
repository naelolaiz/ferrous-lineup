//! Core music types for Ferrous Lineup.
//!
//! This crate knows about notes, tempo, time signatures, patterns,
//! harmony, the playlist and its sections, the project as a whole, how to save a
//! project to a file, what changed between two versions of a project,
//! undo and redo, the piano roll's tools for changing many notes at once,
//! MIDI note messages, MIDI files, swing, turning keys played on a keyboard into
//! notes, twelve-tone rows and serialism, Kröpfl's micromodes, and a
//! library of styles, rhythms and songs. It also holds the backing band
//! that plays from the chord sheet ([`band`], [`chord_sheet`]), the lead
//! sheet, the ear training, hearing chords and keys in the notes
//! ([`harmony`]), putting a song in another time signature ([`remeter`]),
//! and the mixer's settings. It has no audio and no UI code, which keeps
//! it small, fast to compile and easy to test with plain `cargo test`.

pub mod arrangement;
pub mod band;
pub mod bars;
pub mod chord_sheet;
pub mod diff;
pub mod ear_training;
pub mod effects;
pub mod file;
pub mod general_midi;
pub mod harmony;
pub mod history;
pub mod i18n;
pub mod key_map;
pub mod lead_sheet;
pub mod library;
pub mod lock;
pub mod meter;
pub mod micromodal;
pub mod midi;
pub mod midi_file;
pub mod mixer;
pub mod pattern;
pub mod pitch;
pub mod playlist;
pub mod project;
pub mod record;
pub mod remeter;
pub mod score_order;
pub mod sequencer;
pub mod serial;
pub mod sources;
pub mod swing;
pub mod take;
pub mod tempo;
pub mod tools;

// Re-export the main types so other crates can write `lineup_core::Note`
// instead of `lineup_core::pitch::Note`.
pub use arrangement::{Arrangement, Pass, Play};
pub use band::{Band, BandDrums, BandStyle, Player, band_styles};
pub use chord_sheet::{ChordMarker, HarmonyName};
pub use diff::{Change, diff};
pub use effects::{Compressor, Delay, Effects, Equalizer};
pub use harmony::{Chord, ChordQuality, ChordStamp, Harmony, PitchSet, Scale, ScaleKind};
pub use history::History;
pub use key_map::{KeyMap, KeyMarker};
pub use lock::Locked;
pub use meter::{Bar, Click, Meter, TimeMarker, TimeSignature};
pub use midi::{DRUM_CHANNEL, MidiMessage};
pub use midi_file::{general_midi_drum, midi_channels, pattern_to_midi, song_to_midi};
pub use mixer::{Mixer, ReverbSettings};
pub use pattern::{
    DEFAULT_VELOCITY, MAX_STEPS, NoteEvent, Pattern, STEP_NOTE, TICKS_PER_STEP, tidy_notes,
};
pub use pitch::{Letter, Note, NoteName, NoteNaming, PitchClass, Spelling};
pub use playlist::{
    Clip, LoopRegion, MAX_REPEATS, MAX_SONG_STEPS, MAX_TRACKS, Playlist, SECTION_NAMES,
    STEPS_PER_BAR, Section, Track,
};
pub use project::{
    Channel, ChannelKind, Drum, MAX_CHANNELS, MAX_LOOSE_TIMING, MAX_PATTERNS, Project, SampleSource,
};
pub use sequencer::{STEPS_PER_BEAT, Sequencer};
pub use sources::{PartOrigin, PartSet, SongPart, Source, SourceKind};
pub use swing::MAX_SWING;
pub use tempo::{Tempo, TempoMap, TempoMarker};
