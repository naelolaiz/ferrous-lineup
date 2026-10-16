//! Listening to a recording: what Ferrous Loom needs to play along with it.
//!
//! This crate is a spike (an experiment, to see what works) for importing
//! an MP3 or WAV file and building a harmony track that follows it.
//!
//! - [`decode`] reads the file into samples, with Symphonia.
//! - [`analyze`] finds the beats (with the `beat-track-rs` crate), the
//!   key (with the `bpm-key` crate), and the chord of every beat (our
//!   own [`chroma`] and [`chords`] modules: no crate did this well).
//! - [`meter`] guesses how many beats make a bar, and where bar 1 is.
//! - [`arrange`] adds the recording and the chords to a project.

pub mod arrange;
pub mod chords;
pub mod chroma;
pub mod decode;
pub mod meter;

use loom_core::{Chord, PitchClass, Scale, ScaleKind};

pub use decode::{DecodeError, Recording, decode_file};

/// The file extensions [`decode_file`] can read.
pub const AUDIO_EXTENSIONS: [&str; 4] = ["mp3", "wav", "flac", "ogg"];

/// Whether `path` names an audio file, going by its extension.
pub fn is_audio_file(path: &std::path::Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            AUDIO_EXTENSIONS
                .iter()
                .any(|known| extension.eq_ignore_ascii_case(known))
        })
}

/// What we heard in a recording.
#[derive(Debug, Clone, PartialEq)]
pub struct Analysis {
    /// When each beat falls, in seconds from the start of the recording.
    pub beats: Vec<f64>,
    /// The average time between two beats, as a tempo.
    pub bpm: f64,
    /// How many beats make a bar: 3 or 4.
    pub beats_per_bar: usize,
    /// Which beat is the first downbeat (the "one" of a bar).
    pub first_downbeat: usize,
    /// The key, if the recording has one.
    pub scale: Option<Scale>,
    /// The chord from each beat to the next, one per beat (`None` where
    /// there is no chord, such as during silence or drums alone).
    pub chords: Vec<Option<Chord>>,
}

impl Analysis {
    /// How long a beat lasts, in seconds.
    pub fn beat_seconds(&self) -> f64 {
        60.0 / self.bpm
    }
}

/// Listens to the whole recording. Takes a few seconds for a song.
pub fn analyze(recording: &Recording) -> Analysis {
    let beats = beat_track_rs::analyze_samples(
        &recording.samples,
        recording.sample_rate,
        &beat_track_rs::Params::default(),
    )
    .beat_times_secs;
    let bpm = tempo_of(&beats);

    let scale = bpm_key::analyze(
        &recording.samples,
        1,
        recording.sample_rate,
        &bpm_key::Config::default(),
    )
    .ok()
    .and_then(|features| features.key)
    .map(|estimate| {
        let tonic = PitchClass::from_semitone(estimate.key.tonic.index());
        let kind = if estimate.key.is_minor() {
            ScaleKind::NaturalMinor
        } else {
            ScaleKind::Major
        };
        Scale::new(tonic, kind)
    });

    let frames = chroma::frames(recording);
    let beat_chromas: Vec<_> = beats
        .iter()
        .enumerate()
        .map(|(i, &start)| {
            let end = beats.get(i + 1).copied().unwrap_or(start + 60.0 / bpm);
            chroma::average(&frames, start, end)
        })
        .collect();
    let chords = chords::chords_per_beat(&beat_chromas, scale);
    let (beats_per_bar, first_downbeat) = meter::guess(&chords);

    Analysis {
        beats,
        bpm,
        beats_per_bar,
        first_downbeat,
        scale,
        chords,
    }
}

/// The tempo that best fits the beats: the slope of the straight line
/// closest to "time of beat n" (a least-squares fit), which averages out
/// the small wobbles of each beat.
fn tempo_of(beats: &[f64]) -> f64 {
    let n = beats.len() as f64;
    if beats.len() < 2 {
        return 120.0;
    }
    let mean_index = (n - 1.0) / 2.0;
    let mean_time = beats.iter().sum::<f64>() / n;
    let (mut covariance, mut variance) = (0.0, 0.0);
    for (i, &time) in beats.iter().enumerate() {
        let di = i as f64 - mean_index;
        covariance += di * (time - mean_time);
        variance += di * di;
    }
    let seconds_per_beat = covariance / variance;
    if seconds_per_beat > 0.0 {
        60.0 / seconds_per_beat
    } else {
        120.0
    }
}
