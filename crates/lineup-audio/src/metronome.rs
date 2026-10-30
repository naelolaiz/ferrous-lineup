//! The metronome: a click on every beat, higher on the first of a bar,
//! and the count-in, a bar of clicks before playback starts.
//!
//! The clicks are not a channel of the project. They are made here, on
//! the audio thread, so they land exactly on the beat and never end up
//! in a saved song or a WAV export.

use lineup_core::{Click, STEPS_PER_BEAT};

/// Steps in a beat, as a `usize` to count steps with.
const BEAT: usize = STEPS_PER_BEAT as usize;

/// How long a click rings, in seconds: long enough to hear its pitch,
/// short enough not to blur the next beat.
const RING: f32 = 0.03;

/// A short sine "tick" that dies away quickly.
#[derive(Debug, Clone)]
pub(crate) struct ClickVoice {
    sample_rate: f32,
    phase: f32,
    /// How far the phase moves each frame, as a fraction of a turn.
    step: f32,
    level: f32,
    /// What `level` is multiplied by each frame.
    decay: f32,
}

impl ClickVoice {
    pub(crate) fn new(sample_rate: u32) -> Self {
        let sample_rate = sample_rate as f32;
        ClickVoice {
            sample_rate,
            phase: 0.0,
            step: 0.0,
            level: 0.0,
            // After `RING` seconds the click is down to 1/1000 of its
            // start (-60 dB): e^(-6.9) is 0.001.
            decay: (-6.9 / (RING * sample_rate)).exp(),
        }
    }

    /// Starts a click: an A6 on the first beat of a bar, an E6 a fourth
    /// lower on the others, as a metronome's bell and tick differ.
    pub(crate) fn strike(&mut self, click: Click) {
        let (frequency, level) = match click {
            Click::None => return,
            Click::Bar => (1760.0, 0.5),
            Click::Beat => (1318.5, 0.32),
        };
        self.phase = 0.0;
        self.step = frequency / self.sample_rate;
        self.level = level;
    }

    pub(crate) fn next_sample(&mut self) -> f32 {
        if self.level < 1e-4 {
            return 0.0;
        }
        let sample = (self.phase * std::f32::consts::TAU).sin() * self.level;
        self.phase = (self.phase + self.step).fract();
        self.level *= self.decay;
        sample
    }
}

/// What the metronome plays on step `step` of a loop whose clicks are
/// `clicks`. Without clicks (none sent yet), it counts 4/4.
pub(crate) fn click_at(clicks: &[Click], step: usize) -> Click {
    if clicks.is_empty() {
        return match step % (BEAT * 4) {
            0 => Click::Bar,
            offset if offset % BEAT == 0 => Click::Beat,
            _ => Click::None,
        };
    }
    clicks.get(step).copied().unwrap_or_default()
}

/// The bar the count-in counts: the one `start` is in, as the first step
/// of that bar in `clicks` and its length. Playback that starts in the
/// middle of a bar still hears the whole bar counted, then starts where
/// the playhead is.
pub(crate) fn count_in_bar(clicks: &[Click], start: usize) -> (usize, usize) {
    if clicks.is_empty() {
        return (0, BEAT * 4);
    }
    let start = start.min(clicks.len() - 1);
    let from = (0..=start)
        .rev()
        .find(|&step| clicks[step] == Click::Bar)
        .unwrap_or(0);
    let end = (from + 1..clicks.len())
        .find(|&step| clicks[step] == Click::Bar)
        .unwrap_or(clicks.len());
    (from, end - from)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn waltz() -> Vec<Click> {
        // Two bars of 3/4: clicks on steps 0, 4, 8, 12, 16, 20.
        (0..24)
            .map(|step| match step {
                0 | 12 => Click::Bar,
                step if step % 4 == 0 => Click::Beat,
                _ => Click::None,
            })
            .collect()
    }

    #[test]
    fn the_count_in_counts_the_bar_the_playhead_is_in() {
        let clicks = waltz();
        assert_eq!(count_in_bar(&clicks, 0), (0, 12));
        // From beat 2 of bar 2: the whole of bar 2 is counted.
        assert_eq!(count_in_bar(&clicks, 17), (12, 12));
        // Past the end, the last bar.
        assert_eq!(count_in_bar(&clicks, 99), (12, 12));
        // Nothing sent yet: a bar of 4/4.
        assert_eq!(count_in_bar(&[], 5), (0, 16));
    }

    #[test]
    fn without_clicks_it_counts_four() {
        let clicks: Vec<Click> = (0..16).map(|step| click_at(&[], step)).collect();
        assert_eq!(clicks[0], Click::Bar);
        assert_eq!(clicks[4], Click::Beat);
        assert_eq!(clicks[12], Click::Beat);
        assert_eq!(clicks[2], Click::None);
    }

    #[test]
    fn a_click_rings_and_dies_away() {
        let mut voice = ClickVoice::new(48_000);
        assert_eq!(voice.next_sample(), 0.0);
        voice.strike(Click::Bar);
        let first: f32 = (0..48).map(|_| voice.next_sample().abs()).sum();
        assert!(first > 1.0, "it sounds at once");
        for _ in 0..4_800 {
            voice.next_sample();
        }
        assert_eq!(
            voice.next_sample(),
            0.0,
            "and is silent a tenth of a second later"
        );
        voice.strike(Click::None);
        assert_eq!(voice.next_sample(), 0.0, "Click::None plays nothing");
    }
}
