//! Song tempo, and how it changes through a song.
//!
//! Most songs keep one tempo, but music played by people breathes: a
//! pianist slows down at the end of a piece (a *ritardando*, "rit."),
//! an orchestra speeds up towards a climax (an *accelerando*), and after
//! a pause a new section may start at a new speed ("Più mosso", more
//! movement; "Tempo I", the first tempo again). A held note marked with
//! a fermata (𝄐) lasts as long as the player likes before the music
//! carries on.
//!
//! Each change is a [`TempoMarker`] on the playlist, like the time
//! signature markers: from its step on, the song plays at its tempo. A
//! marker can change the tempo at once, or glide to it over a number of
//! steps, which is how a ritardando or an accelerando is written. A
//! [`TempoMap`] works out the tempo of every step from the markers.

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

/// A change of tempo partway through a song: from step `start` on, the
/// song plays at `tempo`.
///
/// With `glide` at 0 the change is sudden, as after a pause. Otherwise
/// the tempo moves step by step from the one before the marker to the
/// new one, reaching it `glide` steps after `start`: a ritardando when
/// the new tempo is slower, an accelerando when it is faster.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TempoMarker {
    /// The step it starts on, counted from the start of the song.
    pub start: usize,
    pub tempo: Tempo,
    /// How many steps the change takes; 0 for at once. Project files
    /// leave it out when it is 0.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub glide: usize,
}

fn is_zero(steps: &usize) -> bool {
    *steps == 0
}

impl TempoMarker {
    /// A sudden change to `bpm` on step `start`.
    pub fn new(start: usize, bpm: f32) -> Self {
        TempoMarker {
            start,
            tempo: Tempo::new(bpm),
            glide: 0,
        }
    }

    /// A gradual change, from `start`, that reaches `bpm` after `glide`
    /// steps.
    pub fn gliding(start: usize, bpm: f32, glide: usize) -> Self {
        TempoMarker {
            glide,
            ..TempoMarker::new(start, bpm)
        }
    }

    /// The tempo on `step`, at or after `start`, when the tempo just
    /// before the marker was `from` BPM.
    ///
    /// During a glide the tempo moves by the same amount on every step,
    /// and the last step of the glide is already at the new tempo:
    /// a glide of 4 steps from 100 to 60 BPM plays 90, 80, 70 and 60.
    fn bpm_on(&self, step: usize, from: f32) -> f32 {
        let into = step - self.start + 1;
        if into >= self.glide {
            return self.tempo.bpm();
        }
        let done = into as f32 / self.glide as f32;
        from + (self.tempo.bpm() - from) * done
    }
}

/// The tempo of every step of a song: the one it starts at, and every
/// change after that.
///
/// The tempo changes only from one step to the next, never in the middle
/// of a step: a step is a sixteenth note, short enough that a glide made
/// of such small changes sounds smooth.
#[derive(Debug, Clone, PartialEq)]
pub struct TempoMap {
    first: Tempo,
    /// Sorted by start, all after step 0, no two on the same step.
    markers: Vec<TempoMarker>,
}

impl TempoMap {
    /// A map that starts at `first` and changes at each of `markers`.
    /// Markers on step 0, or on the same step as another, are left out.
    pub fn new(first: Tempo, markers: impl IntoIterator<Item = TempoMarker>) -> Self {
        let mut markers: Vec<TempoMarker> = markers
            .into_iter()
            .filter(|marker| marker.start > 0)
            .collect();
        markers.sort_by_key(|marker| marker.start);
        markers.dedup_by_key(|marker| marker.start);
        TempoMap { first, markers }
    }

    /// A tempo that never changes.
    pub fn steady(tempo: Tempo) -> Self {
        TempoMap {
            first: tempo,
            markers: Vec::new(),
        }
    }

    /// The tempo the song starts at.
    pub fn first(&self) -> Tempo {
        self.first
    }

    /// The changes after the start, in order.
    pub fn markers(&self) -> &[TempoMarker] {
        &self.markers
    }

    /// Whether the tempo never changes.
    pub fn is_steady(&self) -> bool {
        self.markers.is_empty()
    }

    /// The tempo on `step`.
    ///
    /// A glide starts from whatever the tempo was on the step before its
    /// marker, even if that was halfway through another glide.
    pub fn tempo_at(&self, step: usize) -> Tempo {
        // The marker in effect so far, and the tempo its glide started from.
        let mut current: Option<(&TempoMarker, f32)> = None;
        let bpm_on = |current: Option<(&TempoMarker, f32)>, step: usize| match current {
            Some((marker, from)) => marker.bpm_on(step, from),
            None => self.first.bpm(),
        };
        for marker in self.markers.iter().take_while(|m| m.start <= step) {
            let from = bpm_on(current, marker.start - 1);
            current = Some((marker, from));
        }
        Tempo::new(bpm_on(current, step))
    }

    /// The tempo just before `step`: what a glide starting there starts
    /// from. On step 0, the first tempo.
    pub fn tempo_before(&self, step: usize) -> Tempo {
        if step == 0 {
            self.first
        } else {
            self.tempo_at(step - 1)
        }
    }

    /// The slowest and the fastest tempo anywhere up to `steps`, to scale
    /// a drawing of the tempo.
    pub fn range(&self, steps: usize) -> (f32, f32) {
        let mut low = self.first.bpm();
        let mut high = low;
        for marker in self.markers.iter().filter(|m| m.start < steps) {
            low = low.min(marker.tempo.bpm());
            high = high.max(marker.tempo.bpm());
        }
        (low, high)
    }

    /// The same map without markers that change nothing: a sudden change
    /// to the tempo that is already playing.
    pub fn tidy(&self) -> TempoMap {
        let mut markers = Vec::new();
        for marker in &self.markers {
            let before = TempoMap::new(self.first, markers.iter().copied());
            let changes = before.tempo_at(marker.start) != marker.tempo;
            if changes || marker.glide > 0 && before.tempo_at(marker.start - 1) != marker.tempo {
                markers.push(*marker);
            }
        }
        TempoMap::new(self.first, markers)
    }
}

/// How many seconds `steps` steps last, one after the other, with step
/// `n` at `tempo(n)`.
pub fn seconds(steps: usize, tempo: impl Fn(usize) -> Tempo) -> f64 {
    (0..steps)
        .map(|step| f64::from(tempo(step).seconds_per_beat()) / 4.0)
        .sum()
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

    #[test]
    fn a_sudden_change_plays_from_its_step() {
        let map = TempoMap::new(Tempo::new(120.0), [TempoMarker::new(16, 90.0)]);
        assert_eq!(map.tempo_at(15).bpm(), 120.0);
        assert_eq!(map.tempo_at(16).bpm(), 90.0);
        assert_eq!(map.tempo_at(1000).bpm(), 90.0);
    }

    #[test]
    fn a_glide_moves_a_little_every_step() {
        let map = TempoMap::new(Tempo::new(100.0), [TempoMarker::gliding(8, 60.0, 4)]);
        let played: Vec<f32> = (7..13).map(|step| map.tempo_at(step).bpm()).collect();
        assert_eq!(played, vec![100.0, 90.0, 80.0, 70.0, 60.0, 60.0]);
    }

    #[test]
    fn a_glide_starts_from_where_the_last_one_got_to() {
        // A ritardando cut short by an "a tempo" on step 10: the tempo
        // jumps back from 80 to 100, then a new glide starts from there.
        let map = TempoMap::new(
            Tempo::new(100.0),
            [
                TempoMarker::gliding(8, 60.0, 4),
                TempoMarker::new(10, 100.0),
                TempoMarker::gliding(12, 120.0, 2),
            ],
        );
        assert_eq!(map.tempo_at(9).bpm(), 80.0);
        assert_eq!(map.tempo_at(10).bpm(), 100.0);
        assert_eq!(map.tempo_at(12).bpm(), 110.0);
        assert_eq!(map.tempo_at(13).bpm(), 120.0);
        assert_eq!(map.tempo_before(12).bpm(), 100.0);
    }

    #[test]
    fn tidy_drops_markers_that_change_nothing() {
        let map = TempoMap::new(
            Tempo::new(100.0),
            [TempoMarker::new(8, 100.0), TempoMarker::new(16, 80.0)],
        );
        assert_eq!(map.tidy().markers(), &[TempoMarker::new(16, 80.0)]);
    }

    #[test]
    fn seconds_add_up_every_step() {
        // Four steps at 60 BPM are one second; four at 120, half of one.
        let map = TempoMap::new(Tempo::new(60.0), [TempoMarker::new(4, 120.0)]);
        assert_eq!(seconds(8, |step| map.tempo_at(step)), 1.5);
    }
}
