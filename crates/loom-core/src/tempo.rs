//! Song tempo.

use serde::{Deserialize, Serialize};

/// A tempo in beats per minute (BPM), always inside `Tempo::MIN..=Tempo::MAX`.
///
/// The field is private, so the only way to build a `Tempo` is through
/// [`Tempo::new`], which clamps the value. That guarantees every `Tempo`
/// in the program is valid.
///
/// The same goes for project files: a tempo is saved as a plain number,
/// and reading it back goes through `From<f32>`, which calls
/// [`Tempo::new`]. A file that says `tempo: 9000.0` loads as 522 BPM.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(from = "f32", into = "f32")]
pub struct Tempo {
    bpm: f32,
}

impl Tempo {
    /// The slowest tempo allowed.
    pub const MIN: f32 = 10.0;
    /// The fastest tempo allowed.
    pub const MAX: f32 = 522.0;

    /// Creates a tempo, clamping `bpm` into the allowed range.
    /// A NaN value becomes the default tempo.
    pub fn new(bpm: f32) -> Self {
        if bpm.is_nan() {
            return Self::default();
        }
        Tempo {
            bpm: bpm.clamp(Self::MIN, Self::MAX),
        }
    }

    /// Beats per minute.
    pub fn bpm(self) -> f32 {
        self.bpm
    }

    /// How long one beat (a quarter note) lasts, in seconds.
    pub fn seconds_per_beat(self) -> f32 {
        60.0 / self.bpm
    }

    /// How many audio samples one beat lasts at the given sample rate.
    pub fn samples_per_beat(self, sample_rate: u32) -> f64 {
        sample_rate as f64 * self.seconds_per_beat() as f64
    }
}

impl From<f32> for Tempo {
    fn from(bpm: f32) -> Self {
        Tempo::new(bpm)
    }
}

impl From<Tempo> for f32 {
    fn from(tempo: Tempo) -> Self {
        tempo.bpm
    }
}

impl Default for Tempo {
    fn default() -> Self {
        Tempo { bpm: 130.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps_to_range() {
        assert_eq!(Tempo::new(1.0).bpm(), Tempo::MIN);
        assert_eq!(Tempo::new(9000.0).bpm(), Tempo::MAX);
        assert_eq!(Tempo::new(f32::NAN), Tempo::default());
    }

    #[test]
    fn beat_length() {
        let tempo = Tempo::new(120.0);
        assert_eq!(tempo.seconds_per_beat(), 0.5);
        assert_eq!(tempo.samples_per_beat(48_000), 24_000.0);
    }
}
