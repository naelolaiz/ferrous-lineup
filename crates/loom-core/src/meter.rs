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
//!
//! A song can change time signature partway through, like a hymn that
//! goes from 4/4 to 3/4 for its last line. Each change is a
//! [`TimeMarker`] on the playlist, as in FL Studio, and a [`Meter`]
//! works out where every bar of such a song starts.

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

/// A change of time signature partway through a song: from step `start`
/// on, bars are counted in `time`. A new bar always starts on the marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TimeMarker {
    /// The step it starts on, counted from the start of the song.
    pub start: usize,
    pub time: TimeSignature,
}

impl TimeMarker {
    pub fn new(start: usize, time: TimeSignature) -> Self {
        TimeMarker { start, time }
    }
}

/// One bar of a song: where it starts, how long it is, and its time
/// signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bar {
    /// Counted from 0; the first bar is drawn as "1".
    pub number: usize,
    pub start: usize,
    /// Steps. Shorter than a whole bar only right before a marker that
    /// is not on a bar line.
    pub length: usize,
    pub time: TimeSignature,
}

impl Bar {
    /// The step just after its last one: where the next bar starts.
    pub fn end(&self) -> usize {
        self.start + self.length
    }

    /// Whether `step` starts a beat of this bar.
    pub fn is_beat_start(&self, step: usize) -> bool {
        step >= self.start && (step - self.start).is_multiple_of(self.time.steps_per_beat())
    }
}

/// The time signatures of a whole song: the one it starts in and every
/// change after that.
///
/// A song with no changes is counted exactly like its [`TimeSignature`]
/// alone. With changes, the bars of each stretch are counted from the
/// marker that starts it:
///
/// ```text
/// 4/4 from step 0, 3/4 from step 32:
/// | 1: 16 steps | 2: 16 steps | 3: 12 steps | 4: 12 steps | ...
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Meter {
    first: TimeSignature,
    /// Sorted by start, all after step 0, no two on the same step.
    markers: Vec<TimeMarker>,
}

impl Meter {
    /// A meter that starts in `first` and changes at each of `markers`.
    /// Markers on step 0 or on the same step as another are left out.
    pub fn new(first: TimeSignature, markers: impl IntoIterator<Item = TimeMarker>) -> Self {
        let mut markers: Vec<TimeMarker> = markers
            .into_iter()
            .filter(|marker| marker.start > 0)
            .collect();
        markers.sort_by_key(|marker| marker.start);
        markers.dedup_by_key(|marker| marker.start);
        Meter { first, markers }
    }

    /// A meter that never changes.
    pub fn steady(time: TimeSignature) -> Self {
        Meter {
            first: time,
            markers: Vec::new(),
        }
    }

    /// The time signature the song starts in.
    pub fn first(&self) -> TimeSignature {
        self.first
    }

    /// The changes after the start, in order.
    pub fn markers(&self) -> &[TimeMarker] {
        &self.markers
    }

    /// The stretches of one time signature: (start, end, time). The last
    /// one never ends.
    fn stretches(&self) -> impl Iterator<Item = (usize, usize, TimeSignature)> + '_ {
        let starts =
            std::iter::once(TimeMarker::new(0, self.first)).chain(self.markers.iter().copied());
        let ends = self
            .markers
            .iter()
            .map(|marker| marker.start)
            .chain(std::iter::once(usize::MAX));
        starts
            .zip(ends)
            .map(|(from, end)| (from.start, end, from.time))
    }

    /// The time signature on `step`.
    pub fn time_at(&self, step: usize) -> TimeSignature {
        self.markers
            .iter()
            .take_while(|marker| marker.start <= step)
            .last()
            .map_or(self.first, |marker| marker.time)
    }

    /// The bar that `step` is in.
    pub fn bar_at(&self, step: usize) -> Bar {
        let mut bars_before = 0;
        for (start, end, time) in self.stretches() {
            let bar = time.steps_per_bar();
            if step < end {
                let index = (step - start) / bar;
                let bar_start = start + index * bar;
                return Bar {
                    number: bars_before + index,
                    start: bar_start,
                    length: bar.min(end - bar_start),
                    time,
                };
            }
            bars_before += (end - start).div_ceil(bar);
        }
        unreachable!("the last stretch never ends")
    }

    /// Bar number `number`, counted from 0.
    pub fn bar(&self, number: usize) -> Bar {
        let mut bars_before = 0;
        for (start, end, time) in self.stretches() {
            let bar = time.steps_per_bar();
            let here = (end - start).div_ceil(bar);
            if number < bars_before + here {
                let bar_start = start + (number - bars_before) * bar;
                return Bar {
                    number,
                    start: bar_start,
                    length: bar.min(end - bar_start),
                    time,
                };
            }
            bars_before += here;
        }
        unreachable!("the last stretch never ends")
    }

    /// Every bar from the start of the song, without end.
    pub fn bars(&self) -> impl Iterator<Item = Bar> + '_ {
        let mut next = 0;
        std::iter::from_fn(move || {
            let bar = self.bar_at(next);
            next = bar.end();
            Some(bar)
        })
    }

    /// Whether `step` starts a bar.
    pub fn is_bar_start(&self, step: usize) -> bool {
        self.bar_at(step).start == step
    }

    /// Whether `step` starts a beat.
    pub fn is_beat_start(&self, step: usize) -> bool {
        self.bar_at(step).is_beat_start(step)
    }

    /// The start of the bar `step` is in.
    pub fn floor_to_bar(&self, step: usize) -> usize {
        self.bar_at(step).start
    }

    /// The start of the beat `step` is in.
    pub fn floor_to_beat(&self, step: usize) -> usize {
        let bar = self.bar_at(step);
        let beat = bar.time.steps_per_beat();
        bar.start + (step - bar.start) / beat * beat
    }

    /// `steps` rounded up to the next bar line.
    pub fn round_up_to_bar(&self, steps: usize) -> usize {
        let bar = self.bar_at(steps);
        if bar.start == steps { steps } else { bar.end() }
    }

    /// Where `step` is, as bar, beat and step inside the beat, each
    /// counted from 1: the way a song position is shown.
    pub fn position(&self, step: usize) -> (usize, usize, usize) {
        let bar = self.bar_at(step);
        let beat = bar.time.steps_per_beat();
        let offset = step - bar.start;
        (bar.number + 1, offset / beat + 1, offset % beat + 1)
    }

    /// The markers that change nothing left out: those that keep the
    /// time signature before them and fall on one of its bar lines.
    pub fn tidy(&self) -> Meter {
        let mut tidy = Meter::steady(self.first);
        for &marker in &self.markers {
            if tidy.time_at(marker.start) != marker.time || !tidy.is_bar_start(marker.start) {
                tidy.markers.push(marker);
            }
        }
        tidy
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

    /// 4/4 for two bars, then 3/4.
    fn hymn() -> Meter {
        Meter::new(
            TimeSignature::COMMON,
            [TimeMarker::new(32, signature(3, 4))],
        )
    }

    #[test]
    fn a_steady_meter_counts_like_its_time_signature() {
        let waltz = Meter::steady(signature(3, 4));
        for step in 0..60 {
            assert_eq!(waltz.is_bar_start(step), signature(3, 4).is_bar_start(step));
            assert_eq!(
                waltz.is_beat_start(step),
                signature(3, 4).is_beat_start(step)
            );
        }
        assert_eq!(waltz.round_up_to_bar(13), 24);
        assert_eq!(waltz.bar(3).start, 36);
    }

    #[test]
    fn bars_after_a_marker_are_counted_in_its_time() {
        let hymn = hymn();
        let starts: Vec<usize> = hymn.bars().take(5).map(|bar| bar.start).collect();
        assert_eq!(starts, [0, 16, 32, 44, 56]);
        assert_eq!(hymn.time_at(31), TimeSignature::COMMON);
        assert_eq!(hymn.time_at(32), signature(3, 4));
        assert_eq!(hymn.bar_at(50).number, 3);
        assert_eq!(hymn.bar(3), hymn.bar_at(50));
        assert_eq!(hymn.position(48), (4, 2, 1));
        assert_eq!(hymn.round_up_to_bar(33), 44);
        assert_eq!(hymn.floor_to_beat(51), 48);
    }

    #[test]
    fn a_marker_off_the_bar_line_leaves_a_short_bar() {
        let meter = Meter::new(
            TimeSignature::COMMON,
            [TimeMarker::new(20, signature(6, 8))],
        );
        let bars: Vec<(usize, usize)> = meter
            .bars()
            .take(4)
            .map(|bar| (bar.start, bar.length))
            .collect();
        assert_eq!(bars, [(0, 16), (16, 4), (20, 12), (32, 12)]);
        assert_eq!(meter.bar(2).start, 20);
    }

    #[test]
    fn markers_that_change_nothing_are_tidied_away() {
        let meter = Meter::new(
            TimeSignature::COMMON,
            [
                TimeMarker::new(16, TimeSignature::COMMON),
                TimeMarker::new(32, signature(3, 4)),
                TimeMarker::new(44, signature(3, 4)),
                TimeMarker::new(50, signature(3, 4)),
            ],
        );
        let kept: Vec<usize> = meter.tidy().markers().iter().map(|m| m.start).collect();
        assert_eq!(kept, [32, 50], "50 is not on a bar line of 3/4");
    }

    #[test]
    fn it_is_written_as_a_fraction() {
        assert_eq!(signature(6, 8).to_string(), "6/8");
    }
}
