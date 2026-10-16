//! Time signatures: how many beats make a bar.
//!
//! Music is counted in bars. A waltz counts "one two three, one two
//! three", a march "one two, one two", most pop "one two three four". A
//! time signature writes this as two numbers, like a fraction: 3/4 means
//! three beats to the bar, and each beat is a quarter note (a crotchet).
//! In 6/8 there are six eighth notes to the bar, felt as two groups of
//! three.
//!
//! Ferrous Loom's grid is always sixteenth notes (steps), so a time
//! signature mostly says how many steps a bar has, and where the beats
//! fall inside it. The notes themselves never change: a time signature
//! changes where the bar lines are drawn, not what is played.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::pattern::MAX_STEPS;

/// How beats are grouped into bars, such as 4/4 or 6/8.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TimeSignature {
    /// How many notes of `unit` length make a bar: the top number.
    pub beats: u8,
    /// Which note counts as a beat: 4 for quarter notes, 8 for eighths.
    /// The bottom number.
    pub unit: u8,
}

impl TimeSignature {
    /// 4/4, "common time": four quarter-note beats to the bar.
    pub const COMMON: TimeSignature = TimeSignature::new(4, 4);

    /// The time signatures the menu offers, from two beats to twelve.
    pub const ALL: [TimeSignature; 10] = [
        TimeSignature::new(2, 4),
        TimeSignature::new(3, 4),
        TimeSignature::new(4, 4),
        TimeSignature::new(5, 4),
        TimeSignature::new(3, 8),
        TimeSignature::new(5, 8),
        TimeSignature::new(6, 8),
        TimeSignature::new(7, 8),
        TimeSignature::new(9, 8),
        TimeSignature::new(12, 8),
    ];

    /// A `const fn` can be called where Rust needs a value at compile
    /// time, such as in the `ALL` array above.
    pub const fn new(beats: u8, unit: u8) -> Self {
        TimeSignature { beats, unit }
    }

    /// How many steps (sixteenth notes) a bar lasts: 16 in 4/4, 12 in
    /// 3/4 and 6/8, 14 in 7/8. Never less than one.
    pub fn steps_per_bar(self) -> usize {
        let unit = usize::from(self.unit.clamp(1, 16));
        (usize::from(self.beats) * 16 / unit).max(1)
    }

    /// How many steps each counted beat lasts, for drawing beat lines and
    /// grouping the step buttons.
    ///
    /// In 3/4 and 4/4 that is a quarter note, four steps. In 6/8, 9/8
    /// and 12/8 the eighth notes are felt in groups of three, so a beat
    /// is three eighths, six steps. Other eighth-note signatures, like
    /// 5/8 and 7/8, mix groups of two and three, so their lines go on
    /// every eighth note.
    pub fn steps_per_beat(self) -> usize {
        let unit_steps = (16 / usize::from(self.unit.clamp(1, 16))).max(1);
        if self.unit == 8 && self.beats.is_multiple_of(3) && self.beats > 3 {
            unit_steps * 3
        } else {
            unit_steps
        }
    }

    /// Whether `step` starts a bar.
    pub fn is_bar_start(self, step: usize) -> bool {
        step.is_multiple_of(self.steps_per_bar())
    }

    /// Whether `step` starts a beat.
    pub fn is_beat_start(self, step: usize) -> bool {
        (step % self.steps_per_bar()).is_multiple_of(self.steps_per_beat())
    }

    /// `steps` rounded up to a whole number of bars.
    pub fn round_up_to_bar(self, steps: usize) -> usize {
        let bar = self.steps_per_bar();
        steps.div_ceil(bar) * bar
    }

    /// The pattern lengths the UI offers, in steps: whole numbers of bars,
    /// from one bar to sixteen, as long as they fit in a pattern.
    pub fn pattern_lengths(self) -> impl Iterator<Item = usize> {
        const BARS: [usize; 8] = [1, 2, 3, 4, 6, 8, 12, 16];
        let bar = self.steps_per_bar();
        BARS.into_iter()
            .map(move |bars| bars * bar)
            .filter(|&steps| steps <= MAX_STEPS)
    }

    /// A length written in bars if it is a whole number of them ("1 bar",
    /// "4 bars"), or in steps if not ("13 steps").
    pub fn describe_length(self, steps: usize) -> String {
        let bar = self.steps_per_bar();
        match (steps / bar, steps % bar) {
            (1, 0) => "1 bar".to_string(),
            (bars, 0) => format!("{bars} bars"),
            _ => format!("{steps} steps"),
        }
    }

    /// Whether this is 4/4, which project files leave out.
    pub fn is_common(&self) -> bool {
        *self == TimeSignature::COMMON
    }
}

impl Default for TimeSignature {
    fn default() -> Self {
        TimeSignature::COMMON
    }
}

impl fmt::Display for TimeSignature {
    /// Written the usual way, "3/4".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.beats, self.unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn signature(beats: u8, unit: u8) -> TimeSignature {
        TimeSignature::new(beats, unit)
    }

    #[test]
    fn bars_are_counted_in_sixteenths() {
        assert_eq!(signature(4, 4).steps_per_bar(), 16);
        assert_eq!(signature(3, 4).steps_per_bar(), 12);
        assert_eq!(signature(2, 4).steps_per_bar(), 8);
        assert_eq!(signature(6, 8).steps_per_bar(), 12);
        assert_eq!(signature(7, 8).steps_per_bar(), 14);
        assert_eq!(signature(12, 8).steps_per_bar(), 24);
    }

    #[test]
    fn eighths_in_threes_make_longer_beats() {
        assert_eq!(signature(3, 4).steps_per_beat(), 4);
        assert_eq!(signature(6, 8).steps_per_beat(), 6);
        assert_eq!(signature(12, 8).steps_per_beat(), 6);
        assert_eq!(signature(7, 8).steps_per_beat(), 2);
    }

    #[test]
    fn beats_start_again_in_every_bar() {
        let waltz = signature(3, 4);
        let beats: Vec<usize> = (0..24).filter(|&s| waltz.is_beat_start(s)).collect();
        assert_eq!(beats, [0, 4, 8, 12, 16, 20]);
        let bars: Vec<usize> = (0..24).filter(|&s| waltz.is_bar_start(s)).collect();
        assert_eq!(bars, [0, 12]);
        assert_eq!(waltz.round_up_to_bar(13), 24);
        assert_eq!(waltz.round_up_to_bar(0), 0);
    }

    #[test]
    fn nonsense_does_not_divide_by_zero() {
        assert_eq!(signature(0, 4).steps_per_bar(), 1);
        assert_eq!(signature(3, 0).steps_per_bar(), 48);
        assert!(signature(4, 0).is_beat_start(0));
    }

    #[test]
    fn patterns_are_whole_bars_long() {
        let common: Vec<usize> = TimeSignature::COMMON.pattern_lengths().collect();
        assert_eq!(common, [16, 32, 48, 64, 96, 128, 192, 256]);
        let twelve: Vec<usize> = signature(12, 8).pattern_lengths().collect();
        assert_eq!(
            twelve,
            [24, 48, 72, 96, 144, 192],
            "twelve bars would not fit"
        );
        let waltz = signature(3, 4);
        assert_eq!(waltz.describe_length(12), "1 bar");
        assert_eq!(waltz.describe_length(48), "4 bars");
        assert_eq!(waltz.describe_length(16), "16 steps");
    }

    #[test]
    fn it_is_written_as_a_fraction() {
        assert_eq!(signature(6, 8).to_string(), "6/8");
    }
}
