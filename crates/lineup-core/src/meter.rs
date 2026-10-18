//! Time signatures: how many beats make a bar.
//!
//! Music is counted in bars. A waltz counts "one two three, one two
//! three", a march "one two, one two", most pop "one two three four". A
//! time signature writes this as two numbers, like a fraction: 3/4 means
//! three beats to the bar, and each beat is a quarter note (a crotchet).
//! In 6/8 there are six eighth notes to the bar, felt as two groups of
//! three.
//!
//! Ferrous Lineup's grid is always sixteenth notes (steps), so a time
//! signature mostly says how many steps a bar has, and where the beats
//! fall inside it. The notes themselves never change: a time signature
//! changes where the bar lines are drawn, not what is played.
//!
//! A song can change time signature partway through, like a hymn that
//! goes from 4/4 to 3/4 for its last line. Each change is a
//! [`TimeMarker`] on the playlist, as in FL Studio, and a [`Meter`]
//! works out where every bar of such a song starts.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::pattern::MAX_STEPS;

/// How beats are grouped into bars, such as 4/4, 6/8, or 7/8 counted
/// "one two, one two, one two three" (2+2+3).
///
/// Any top number works, from 1 up to as many as fit in a pattern, and
/// the bottom number is a whole, half, quarter, eighth or sixteenth
/// note: 1, 2, 4, 8 or 16. Thirty-seconds would not fit the grid of
/// sixteenth-note steps.
///
/// Irregular ("aksak", or additive) meters such as the 7/8 of Balkan
/// dances group their notes into beats of two and three. The grouping is
/// part of the time signature, written as the top number: 2+2+3/8.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "SavedTime", into = "SavedTime")]
pub struct TimeSignature {
    /// How many notes of `unit` length make a bar: the top number.
    pub beats: u8,
    /// Which note counts as a beat: 4 for quarter notes, 8 for eighths.
    /// The bottom number.
    pub unit: u8,
    /// Where the felt beats start, when they are not the usual ones: bit
    /// `i` is set when a beat starts on the bar's `i`-th `unit` note.
    /// 0 means the usual beats (see [`TimeSignature::usual_groups`]).
    ///
    /// A `u64` keeps `TimeSignature` `Copy`, so it can be passed around
    /// as freely as a number; a `Vec` could not be.
    groups: u64,
}

/// What a metronome plays on a step: nothing, a beat, or the first beat
/// of a bar, which it plays higher so you can hear where "one" is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Click {
    #[default]
    None,
    Beat,
    Bar,
}

/// One felt beat of a bar: which one it is and where it lies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Beat {
    /// Counted from 0: the first beat of the bar is 0.
    pub index: usize,
    /// The step it starts on, counted from the start of the bar.
    pub start: usize,
    /// How many steps it lasts.
    pub length: usize,
}

impl TimeSignature {
    /// 4/4, "common time": four quarter-note beats to the bar.
    pub const COMMON: TimeSignature = TimeSignature::new(4, 4);

    /// The time signatures the menus offer with one click. Any other can
    /// be typed in (see [`TimeSignature::from_str`](std::str::FromStr)).
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

    /// The bottom numbers a time signature can have.
    pub const UNITS: [u8; 5] = [1, 2, 4, 8, 16];

    /// The most notes a grouped bar can have: one bit each in `groups`.
    pub const MAX_GROUPED_BEATS: u8 = 64;

    /// A `const fn` can be called where Rust needs a value at compile
    /// time, such as in the `ALL` array above.
    pub const fn new(beats: u8, unit: u8) -> Self {
        TimeSignature {
            beats,
            unit,
            groups: 0,
        }
    }

    /// A time signature whose beats are `groups` notes of `unit` long,
    /// such as `[2, 2, 3]` and 8 for 2+2+3/8. The top number is their
    /// sum. `None` if a group is empty or there are too many notes.
    pub fn grouped(groups: &[u8], unit: u8) -> Option<Self> {
        let total: usize = groups.iter().map(|&g| usize::from(g)).sum();
        if groups.is_empty() || groups.contains(&0) || total > usize::from(Self::MAX_GROUPED_BEATS)
        {
            return None;
        }
        Some(Self::of_groups(groups, unit))
    }

    /// The same as [`grouped`](Self::grouped), for tables Rust writes
    /// down before the program runs, such as the library's songs and
    /// rhythms: `TimeSignature::of_groups(&[3, 2, 2], 8)` is 3+2+2/8, the
    /// 7/8 of the Greek kalamatianos. A `const fn` cannot return an
    /// error to look at, so groupings that `grouped` refuses stop the
    /// build instead.
    pub const fn of_groups(groups: &[u8], unit: u8) -> Self {
        let mut mask = 0_u64;
        let mut at = 0;
        let mut index = 0;
        // `for` loops are not allowed in a `const fn` yet: `while` is.
        while index < groups.len() {
            assert!(groups[index] > 0, "a group of no notes");
            assert!(at < Self::MAX_GROUPED_BEATS as usize, "too many notes");
            mask |= 1 << at;
            at += groups[index] as usize;
            index += 1;
        }
        assert!(at > 0 && at <= Self::MAX_GROUPED_BEATS as usize);
        let mut time = TimeSignature::new(at as u8, unit);
        // The usual grouping is written the plain way, so 3+3/8 and 6/8
        // are the same time signature.
        if mask != time.usual_mask() {
            time.groups = mask;
        }
        time
    }

    /// Whether the beats are grouped in some other way than the usual.
    pub fn is_grouped(self) -> bool {
        self.mask() != self.usual_mask()
    }

    /// The same top and bottom numbers, in the usual beats.
    pub fn ungrouped(self) -> Self {
        TimeSignature::new(self.beats, self.unit)
    }

    /// How many `unit` notes each beat has, in order: `[2, 2, 3]` for
    /// 2+2+3/8, `[3, 3]` for 6/8, `[1, 1, 1]` for 3/4.
    pub fn groups(self) -> Vec<u8> {
        let mask = self.mask();
        let beats = usize::from(self.beats);
        let starts: Vec<usize> = (0..beats).filter(|&i| mask & (1 << i) != 0).collect();
        starts
            .iter()
            .zip(starts.iter().skip(1).copied().chain([beats]))
            .map(|(&from, to)| (to - from) as u8)
            .collect()
    }

    /// How many `unit` notes a usual beat has. In 6/8, 9/8 and 12/8 the
    /// eighth notes are felt in groups of three, so a beat is three
    /// eighths (and the same for sixteenths: 6/16, 9/16, 12/16). Every
    /// other time signature counts each note as a beat, including 5/8
    /// and 7/8, until it is given a grouping such as 3+2/8.
    pub const fn usual_groups(self) -> u8 {
        if self.unit >= 8 && self.beats.is_multiple_of(3) && self.beats > 3 {
            3
        } else {
            1
        }
    }

    /// How many `unit` notes each beat has as a player leans on it, which
    /// may be fewer beats than a metronome clicks: a grouping when there
    /// is one ([`groups`](Self::groups)), the three-note beats of 6/8,
    /// 9/8 and 12/8, and otherwise beats of two with a three at the end
    /// for an odd number: 4/4 is felt in two halves (2+2), 5/4 as 2+3,
    /// 7/8 as 2+2+3. Bars of three notes or fewer, such as 3/4, are one
    /// beat. A band's "oom-pa" puts the bass on each of these beats.
    pub fn felt_groups(self) -> Vec<u8> {
        let groups = self.groups();
        if groups.iter().any(|&group| group > 1) {
            return groups;
        }
        let beats = self.beats.max(1);
        if beats <= 3 {
            return vec![beats];
        }
        let mut felt = vec![2; usize::from(beats / 2)];
        if beats % 2 == 1
            && let Some(last) = felt.last_mut()
        {
            *last = 3;
        }
        felt
    }

    /// The beat starts of the usual grouping, as in `groups`.
    const fn usual_mask(self) -> u64 {
        let every = self.usual_groups() as usize;
        let beats = if self.beats < Self::MAX_GROUPED_BEATS {
            self.beats
        } else {
            Self::MAX_GROUPED_BEATS
        } as usize;
        let mut mask = 0;
        let mut at = 0;
        while at < beats {
            mask |= 1 << at;
            at += every;
        }
        mask
    }

    /// The beat starts in use: `groups` if it fits the top number (it
    /// always does unless `beats` was changed by hand), else the usual.
    fn mask(self) -> u64 {
        let fits = self.beats <= Self::MAX_GROUPED_BEATS
            && self.groups & 1 == 1
            && self.groups >> self.beats == 0;
        if fits { self.groups } else { self.usual_mask() }
    }

    /// How many steps one `unit` note lasts: 4 for a quarter, 2 for an
    /// eighth.
    pub fn unit_steps(self) -> usize {
        (16 / usize::from(self.unit.clamp(1, 16))).max(1)
    }

    /// How many steps (sixteenth notes) a bar lasts: 16 in 4/4, 12 in
    /// 3/4 and 6/8, 14 in 7/8. Never less than one.
    pub fn steps_per_bar(self) -> usize {
        let unit = usize::from(self.unit.clamp(1, 16));
        (usize::from(self.beats) * 16 / unit).max(1)
    }

    /// Whether Ferrous Lineup can count this time signature: a top number
    /// from 1, a bottom number from [`UNITS`](Self::UNITS), and bars that
    /// fit in a pattern.
    pub fn is_valid(self) -> bool {
        self.beats > 0 && Self::UNITS.contains(&self.unit) && self.steps_per_bar() <= MAX_STEPS
    }

    /// The biggest top number that fits in a pattern over `unit`: 64 for
    /// quarter notes, 128 for eighths.
    pub fn max_beats(unit: u8) -> u8 {
        let unit = usize::from(unit.clamp(1, 16));
        (MAX_STEPS * unit / 16).min(usize::from(u8::MAX)) as u8
    }

    /// The steps, counted from the start of a bar, on which its beats
    /// start: `[0, 4, 8]` in 3/4, `[0, 4, 8]` in 2+2+3/8 too, `[0, 6]`
    /// in 6/8.
    pub fn beat_starts(self) -> Vec<usize> {
        let unit = self.unit_steps();
        let bar = self.steps_per_bar();
        let mut at = 0;
        let mut starts = Vec::new();
        for group in self.groups() {
            if at >= bar {
                break;
            }
            starts.push(at);
            at += usize::from(group) * unit;
        }
        if starts.is_empty() {
            starts.push(0);
        }
        starts
    }

    /// The beat that `offset` steps into a bar is in. Offsets past the
    /// end of the bar count on into the next one.
    pub fn beat_at(self, offset: usize) -> Beat {
        let bar = self.steps_per_bar();
        let offset = offset % bar;
        let starts = self.beat_starts();
        let index = starts
            .iter()
            .rposition(|&start| start <= offset)
            .unwrap_or(0);
        let start = starts[index];
        let end = starts.get(index + 1).copied().unwrap_or(bar);
        Beat {
            index,
            start,
            length: (end - start).max(1),
        }
    }

    /// How many steps the first beat of a bar lasts: in 3/4 and 4/4 a
    /// quarter note, four steps; in 6/8 three eighths, six steps; in
    /// 3+2+2/8 also six. Other beats of a grouped bar may be shorter or
    /// longer: see [`beat_at`](Self::beat_at).
    pub fn first_beat(self) -> usize {
        self.beat_at(0).length
    }

    /// Whether `step` starts a bar.
    pub fn is_bar_start(self, step: usize) -> bool {
        step.is_multiple_of(self.steps_per_bar())
    }

    /// Whether `step` starts a beat.
    pub fn is_beat_start(self, step: usize) -> bool {
        let offset = step % self.steps_per_bar();
        self.beat_at(offset).start == offset
    }

    /// What a metronome plays on `step`, counted from the start of a bar:
    /// in 6/8 it clicks the two dotted quarters, in 2+2+3/8 the three
    /// groups.
    pub fn click(self, step: usize) -> Click {
        if self.is_bar_start(step) {
            Click::Bar
        } else if self.is_beat_start(step) {
            Click::Beat
        } else {
            Click::None
        }
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
    /// Written the usual way, "3/4", with the grouping as the top number
    /// if it has one: "2+2+3/8".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_grouped() {
            let groups: Vec<String> = self.groups().iter().map(u8::to_string).collect();
            write!(f, "{}/{}", groups.join("+"), self.unit)
        } else {
            write!(f, "{}/{}", self.beats, self.unit)
        }
    }
}

impl FromStr for TimeSignature {
    type Err = String;

    /// Reads a time signature as it is written: "7/8", "2+2+3/8" or
    /// "(2+2+3)/8". The error says what is wrong, for the menus to show.
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let (top, bottom) = text
            .split_once('/')
            .ok_or("write it as two numbers, like 7/8")?;
        let unit: u8 = bottom
            .trim()
            .parse()
            .map_err(|_| "the bottom number must be a number")?;
        if !Self::UNITS.contains(&unit) {
            return Err("the bottom number must be 1, 2, 4, 8 or 16".to_string());
        }
        let top = top.trim().trim_start_matches('(').trim_end_matches(')');
        let groups: Vec<u8> = top
            .split('+')
            .map(|group| group.trim().parse::<u8>())
            .collect::<Result<_, _>>()
            .map_err(|_| "the top number must be a number, or numbers joined by +")?;
        if groups.contains(&0) {
            return Err("the top number must be 1 or more".to_string());
        }
        let time = if let [beats] = groups[..] {
            TimeSignature::new(beats, unit)
        } else {
            TimeSignature::grouped(&groups, unit).ok_or(format!(
                "a grouped bar can have at most {} notes",
                Self::MAX_GROUPED_BEATS
            ))?
        };
        if time.steps_per_bar() > MAX_STEPS {
            return Err(format!(
                "a bar of {time} would not fit in a pattern (at most {}/{unit})",
                Self::max_beats(unit)
            ));
        }
        Ok(time)
    }
}

/// How a time signature is written in project files: the two numbers,
/// and the grouping only when it has one, as `groups: [2, 2, 3]`. So a
/// file without groupings reads exactly as before.
#[derive(Serialize, Deserialize)]
struct SavedTime {
    beats: u8,
    unit: u8,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    groups: Vec<u8>,
}

impl From<TimeSignature> for SavedTime {
    fn from(time: TimeSignature) -> Self {
        SavedTime {
            beats: time.beats,
            unit: time.unit,
            groups: if time.is_grouped() {
                time.groups()
            } else {
                Vec::new()
            },
        }
    }
}

impl From<SavedTime> for TimeSignature {
    /// A grouping that does not add up to the top number is left out.
    fn from(saved: SavedTime) -> Self {
        let sum: usize = saved.groups.iter().map(|&g| usize::from(g)).sum();
        if sum == usize::from(saved.beats) {
            TimeSignature::grouped(&saved.groups, saved.unit)
                .unwrap_or(TimeSignature::new(saved.beats, saved.unit))
        } else {
            TimeSignature::new(saved.beats, saved.unit)
        }
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
        step >= self.start
            && step < self.start + self.time.steps_per_bar()
            && self.time.is_beat_start(step - self.start)
    }

    /// The beat that `step`, inside this bar, is in. Its start is
    /// counted from the start of the song. In a short bar, the last beat
    /// ends with the bar.
    pub fn beat_at(&self, step: usize) -> Beat {
        let beat = self.time.beat_at(step.saturating_sub(self.start));
        let start = self.start + beat.start;
        Beat {
            start,
            length: beat.length.min(self.end().saturating_sub(start)).max(1),
            ..beat
        }
    }

    /// The steps on which this bar's beats start, counted from the start
    /// of the song.
    pub fn beat_starts(&self) -> impl Iterator<Item = usize> + '_ {
        let end = self.end();
        self.time
            .beat_starts()
            .into_iter()
            .map(move |offset| self.start + offset)
            .take_while(move |&step| step < end)
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

    /// What a metronome plays on `step`.
    pub fn click(&self, step: usize) -> Click {
        let bar = self.bar_at(step);
        if bar.start == step {
            Click::Bar
        } else if bar.is_beat_start(step) {
            Click::Beat
        } else {
            Click::None
        }
    }

    /// The start of the bar `step` is in.
    pub fn floor_to_bar(&self, step: usize) -> usize {
        self.bar_at(step).start
    }

    /// The start of the beat `step` is in.
    pub fn floor_to_beat(&self, step: usize) -> usize {
        self.bar_at(step).beat_at(step).start
    }

    /// The end of the beat `step` is in: where the next one starts.
    pub fn beat_end(&self, step: usize) -> usize {
        let beat = self.bar_at(step).beat_at(step);
        beat.start + beat.length
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
        let beat = bar.beat_at(step);
        (bar.number + 1, beat.index + 1, step - beat.start + 1)
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
        assert_eq!(signature(3, 4).first_beat(), 4);
        assert_eq!(signature(6, 8).first_beat(), 6);
        assert_eq!(signature(12, 8).first_beat(), 6);
        assert_eq!(signature(7, 8).first_beat(), 2);
        assert_eq!(signature(9, 16).beat_starts(), [0, 3, 6]);
    }

    fn typed(text: &str) -> TimeSignature {
        text.parse().unwrap()
    }

    #[test]
    fn any_numbers_can_be_typed() {
        assert_eq!(typed("7/8"), signature(7, 8));
        assert_eq!(typed(" 13 / 16 "), signature(13, 16));
        assert_eq!(typed("1/1").steps_per_bar(), 16);
        assert_eq!(typed("3/2").steps_per_bar(), 24);
        assert_eq!(typed("64/4").steps_per_bar(), 256);
        assert!("65/4".parse::<TimeSignature>().is_err(), "too long a bar");
        assert!("7/6".parse::<TimeSignature>().is_err());
        assert!("7/32".parse::<TimeSignature>().is_err());
        assert!("0/4".parse::<TimeSignature>().is_err());
        assert!("seven".parse::<TimeSignature>().is_err());
        for time in TimeSignature::ALL {
            assert_eq!(typed(&time.to_string()), time);
        }
    }

    #[test]
    fn irregular_meters_group_their_beats() {
        let balkan = typed("2+2+3/8");
        assert_eq!(balkan.beats, 7);
        assert_eq!(balkan.steps_per_bar(), 14);
        assert_eq!(balkan.groups(), [2, 2, 3]);
        assert_eq!(balkan.beat_starts(), [0, 4, 8]);
        assert_eq!(balkan.beat_at(9).length, 6);
        assert_eq!(balkan.beat_at(9).index, 2);
        assert_eq!(balkan.to_string(), "2+2+3/8");
        assert_eq!(typed("(2+2+3)/8"), balkan);
        assert_ne!(balkan, signature(7, 8));
        assert_eq!(typed("3+2+2/8").first_beat(), 6);
        let beats: Vec<usize> = (0..28).filter(|&s| balkan.is_beat_start(s)).collect();
        assert_eq!(beats, [0, 4, 8, 14, 18, 22]);
    }

    #[test]
    fn the_metronome_clicks_the_felt_beats() {
        let clicks = |time: TimeSignature| -> Vec<usize> {
            (0..time.steps_per_bar())
                .filter(|&step| time.click(step) != Click::None)
                .collect()
        };
        assert_eq!(clicks(signature(4, 4)), [0, 4, 8, 12]);
        // Two dotted quarters in 6/8, not six eighths.
        assert_eq!(clicks(signature(6, 8)), [0, 6]);
        assert_eq!(clicks(typed("2+2+3/8")), [0, 4, 8]);
        assert_eq!(signature(3, 4).click(0), Click::Bar);
        assert_eq!(signature(3, 4).click(4), Click::Beat);
        assert_eq!(signature(3, 4).click(12), Click::Bar, "the next bar");
    }

    #[test]
    fn the_usual_grouping_is_written_plainly() {
        assert_eq!(typed("3+3/8"), signature(6, 8));
        assert_eq!(typed("1+1+1/4"), signature(3, 4));
        assert!(!signature(6, 8).is_grouped());
        assert_eq!(typed("2+2+3/8").ungrouped(), signature(7, 8));
    }

    #[test]
    fn a_grouping_is_saved_only_when_there_is_one() {
        let plain = ron::to_string(&signature(7, 8)).unwrap();
        assert_eq!(plain, "(beats:7,unit:8)", "files read as before");
        let grouped = ron::to_string(&typed("3+2+2/8")).unwrap();
        assert_eq!(grouped, "(beats:7,unit:8,groups:[3,2,2])");
        let back: TimeSignature = ron::from_str(&grouped).unwrap();
        assert_eq!(back, typed("3+2+2/8"));
        let wrong: TimeSignature = ron::from_str("(beats:7,unit:8,groups:[3,3])").unwrap();
        assert_eq!(wrong, signature(7, 8), "a grouping that does not add up");
    }

    #[test]
    fn a_meter_counts_grouped_beats() {
        let meter = Meter::steady(typed("3+2+2/8"));
        assert_eq!(meter.position(7), (1, 2, 2));
        assert_eq!(meter.floor_to_beat(11), 10);
        assert_eq!(meter.beat_end(6), 10);
        assert_eq!(meter.position(14), (2, 1, 1));
        let bar = meter.bar(1);
        assert_eq!(bar.beat_starts().collect::<Vec<_>>(), [14, 20, 24]);
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
