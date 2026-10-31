//! Rhythms: drum patterns written one line per drum.
//!
//! Drummers and drum machines think of a beat as a grid: one row per
//! drum, one column per step. The library writes exactly that, as text:
//!
//! ```text
//! Kick   "X... .... X... ...."
//! Snare  ".... X... .... X..."
//! Hi-hat "x.x. x.x. x.x. x.x."
//! ```
//!
//! Each character is one step. `X` is a loud hit, `x` a normal one, `o`
//! a soft "ghost" note, and `.` silence. `h` and `l` play the drum five
//! semitones higher or lower, for the open and closed sounds of congas
//! and toms. Spaces only make the line easier to read, and `|` separates
//! bars when a rhythm is longer than one.

use std::sync::{Mutex, PoisonError};

use crate::meter::TimeSignature;
use crate::pattern::{NoteEvent, STEP_NOTE};
use crate::project::Drum;

use super::Genre;

/// One drum pattern.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rhythm {
    pub name: &'static str,
    pub genre: Genre,
    pub time: TimeSignature,
    /// The tempo it is usually played at, in BPM.
    pub tempo: f32,
    /// One line per drum.
    pub lines: &'static [(Drum, &'static str)],
    /// The name of a fill in [`FILLS`] that ends a section, or "".
    pub fill: &'static str,
    pub about: &'static str,
    /// How much its eighth notes swing (see [`crate::swing`]): 0 for
    /// even eighths. A swing beat is written in even eighths, as in sheet
    /// music, and played long-short by this much.
    pub swing: u8,
    /// Whether it plays every bar in the bar's own meter, for music whose
    /// meter changes from bar to bar: what each drum plays on the first
    /// beat of its bar goes on the first beat of every bar, and what it
    /// plays on its second felt beat goes on each other beat the bar is
    /// felt in (see [`TimeSignature::felt_groups`]). Its lines are what
    /// it plays in its own time signature (see [`Rhythm::notes_in`]).
    pub follows_the_meter: bool,
    /// How its bars are laid out over a part of a song, for a rhythm
    /// whose lines are an opening, grooves and a closing rather than one
    /// loop (see [`Layout`]).
    pub layout: Option<Layout>,
    /// The fills it plays at the ends of parts, after its own [`fill`]:
    /// `None` for the usual ones of its genre (see
    /// [`super::drum_form`]).
    ///
    /// [`fill`]: Rhythm::fill
    pub fills: Option<&'static [&'static str]>,
}

/// How a rhythm whose lines are more than a loop lays them out over a
/// part of a song. Its lines are, bar after bar, the opening, each groove
/// and each closing (see [`super::drum_form`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Layout {
    /// The bars it opens the song with.
    pub opening: usize,
    /// How many bars each groove lasts. Each kind of part has `turns` of
    /// them, which its phrases take turns with: the first ones for the
    /// song's first kind of part (A), the next for the next (B), and so
    /// on round.
    pub grooves: &'static [usize],
    pub turns: usize,
    /// The bars that close each phrase, and how many closings the
    /// phrases take turns with.
    pub closing: usize,
    pub closings: usize,
}

impl Layout {
    /// How many bars its lines last.
    pub fn bars(&self) -> usize {
        self.opening + self.grooves.iter().sum::<usize>() + self.closing * self.closings.max(1)
    }

    /// The bar of the lines the groove of kind `kind` starts on, in a
    /// phrase that takes turn `turn`, and how long it lasts.
    pub fn groove(&self, kind: usize, turn: usize) -> (usize, usize) {
        let turns = self.turns.max(1);
        let kinds = (self.grooves.len() / turns).max(1);
        let groove = ((kind % kinds) * turns + turn % turns) % self.grooves.len().max(1);
        let from = self.opening + self.grooves[..groove].iter().sum::<usize>();
        (from, self.grooves.get(groove).copied().unwrap_or(1).max(1))
    }

    /// The bar of the lines the first closing starts on.
    pub fn closing_from(&self) -> usize {
        self.opening + self.grooves.iter().sum::<usize>()
    }

    /// The bar of the lines the closing of a phrase that takes turn
    /// `turn` starts on.
    pub fn closing_at(&self, turn: usize) -> usize {
        self.closing_from() + (turn % self.closings.max(1)) * self.closing
    }
}

/// One hit: how hard, and how many semitones up or down.
type Hit = (u8, i32);

/// Reads one character of a drum line.
fn hit(character: char) -> Option<Option<Hit>> {
    match character {
        '.' => Some(None),
        'X' => Some(Some((120, 0))),
        'x' => Some(Some((96, 0))),
        'o' => Some(Some((56, 0))),
        'h' => Some(Some((96, 5))),
        'l' => Some(Some((96, -5))),
        _ => None,
    }
}

/// Reads a drum line into bars of steps. Returns `None` if a character
/// is not one of the ones above.
pub fn parse_line(line: &str) -> Option<Vec<Vec<Option<Hit>>>> {
    line.split('|')
        .map(|bar| {
            bar.chars()
                .filter(|character| !character.is_whitespace())
                .map(hit)
                .collect()
        })
        .collect()
}

impl Rhythm {
    /// How many bars the rhythm lasts before it repeats.
    pub fn bars(&self) -> usize {
        self.lines
            .first()
            .map_or(1, |(_, line)| line.split('|').count())
    }

    /// How many steps the rhythm lasts before it repeats.
    pub fn length(&self) -> usize {
        self.bars() * self.time.steps_per_bar()
    }

    /// The drums the rhythm plays.
    pub fn drums(&self) -> impl Iterator<Item = Drum> + '_ {
        self.lines.iter().map(|(drum, _)| *drum)
    }

    /// The notes `drum` plays over `length` steps of a pattern whose bars
    /// last `bar` steps, repeating the rhythm bar by bar.
    ///
    /// The rhythm is laid out a bar at a time, so it keeps its place in
    /// the bar even when the pattern's bars are another length: a 4/4
    /// beat in 3/4 loses its last beat in every bar instead of drifting
    /// across the bar lines.
    pub fn notes(&self, drum: Drum, bar: usize, length: usize) -> Vec<NoteEvent> {
        let Some((_, line)) = self.lines.iter().find(|(d, _)| *d == drum) else {
            return Vec::new();
        };
        line_notes(line, bar, length)
    }

    /// The notes `drum` plays over `length` steps of bars of `time`. A
    /// rhythm that follows the meter is written out again for `time`, so
    /// each bar keeps its felt beats; any other is laid out a bar at a
    /// time, as [`Rhythm::notes`] does.
    pub fn notes_in(&self, drum: Drum, time: TimeSignature, length: usize) -> Vec<NoteEvent> {
        if !self.follows_the_meter || time == self.time {
            return self.notes(drum, time.steps_per_bar(), length);
        }
        let Some((_, line)) = self.lines.iter().find(|(d, _)| *d == drum) else {
            return Vec::new();
        };
        line_notes(
            &in_meter(line, self.time, time),
            time.steps_per_bar(),
            length,
        )
    }

    /// The notes `drum` plays in a bar of `time` lasting `length` steps,
    /// from its first step: bar `index` of its line, counted round. A
    /// rhythm that follows the meter is written out again for `time`, as
    /// [`Rhythm::notes_in`] does.
    pub fn bar_notes(
        &self,
        drum: Drum,
        index: usize,
        time: TimeSignature,
        length: usize,
    ) -> Vec<NoteEvent> {
        let Some((_, line)) = self.lines.iter().find(|(d, _)| *d == drum) else {
            return Vec::new();
        };
        let bars = if self.follows_the_meter && time != self.time {
            parse_line(&in_meter(line, self.time, time))
        } else {
            parse_line(line)
        }
        .unwrap_or_default();
        if bars.is_empty() {
            return Vec::new();
        }
        let steps = &bars[index % bars.len()];
        steps
            .iter()
            .enumerate()
            .take(time.steps_per_bar().min(length))
            .filter_map(|(at, hit)| {
                let (velocity, shift) = (*hit)?;
                let pitch = STEP_NOTE.transpose(shift).unwrap_or(STEP_NOTE);
                Some(NoteEvent {
                    velocity,
                    ..NoteEvent::new(pitch, at, 1)
                })
            })
            .collect()
    }

    /// The rhythm called `name` in [`RHYTHMS`] or [`FILLS`].
    pub fn named(name: &str) -> Option<&'static Rhythm> {
        RHYTHMS
            .iter()
            .chain(FILLS)
            .find(|rhythm| rhythm.name == name)
    }

    /// The fill that goes with this rhythm, if any.
    pub fn fill(&self) -> Option<&'static Rhythm> {
        FILLS.iter().find(|fill| fill.name == self.fill)
    }
}

/// The notes of one drum's `line` over `length` steps of a pattern whose
/// bars last `bar` steps, as [`Rhythm::notes`] lays them out.
fn line_notes(line: &str, bar: usize, length: usize) -> Vec<NoteEvent> {
    let bars = parse_line(line).unwrap_or_default();
    if bars.is_empty() || bar == 0 {
        return Vec::new();
    }
    let mut notes = Vec::new();
    for start in (0..length).step_by(bar) {
        let steps = &bars[(start / bar) % bars.len()];
        for (step, hit) in steps.iter().enumerate().take(bar) {
            let at = start + step;
            if let (Some((velocity, shift)), true) = (hit, at < length) {
                let pitch = STEP_NOTE.transpose(*shift).unwrap_or(STEP_NOTE);
                notes.push(NoteEvent {
                    velocity: *velocity,
                    ..NoteEvent::new(pitch, at, 1)
                });
            }
        }
    }
    notes
}

/// Where each beat of a bar of `time` starts, as the bar is felt (see
/// [`TimeSignature::felt_groups`]): once in 2/4 and 3/4, on 1 and 3 in
/// 4/4, on 1, 4 and 7 in 9/8, on 1 and 3 in 5/8.
fn felt_starts(time: TimeSignature) -> Vec<usize> {
    let unit = time.unit_steps();
    let mut at = 0;
    time.felt_groups()
        .into_iter()
        .map(|group| {
            let start = at;
            at += usize::from(group) * unit;
            start
        })
        .collect()
}

/// A drum line of a rhythm that follows the meter, written in `own`,
/// written again for a bar of `time`: its first beat's hit on the first
/// beat, and its second felt beat's hit on each other felt beat.
fn in_meter(line: &str, own: TimeSignature, time: TimeSignature) -> String {
    let steps: Vec<char> = line
        .split('|')
        .next()
        .unwrap_or_default()
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect();
    let first = steps.first().copied().unwrap_or('.');
    let other = felt_starts(own)
        .get(1)
        .and_then(|&start| steps.get(start).copied())
        .unwrap_or('.');
    let mut bar = vec!['.'; time.steps_per_bar()];
    for (index, start) in felt_starts(time).into_iter().enumerate() {
        if let Some(step) = bar.get_mut(start) {
            *step = if index == 0 { first } else { other };
        }
    }
    bar.into_iter().collect()
}

/// The rhythms a song in a time signature plays when nothing says which:
/// the first one in the song's new time signature.
const USUAL: [&str; 4] = ["Pop beat", "Waltz", "Six-eight", "Slow blues"];

impl Rhythm {
    /// The rhythm a song plays when its time signature is changed to
    /// `time` by hand, while it played `like`: a rhythm of the same kind
    /// in `time` if the library has one (a tango in 3/4 is a tango vals),
    /// else the usual one in `time` (a waltz for 3/4), else one with the
    /// same notes to the bar grouped its own way (7/8 plays as 2+2+3/8,
    /// a rachenitsa, or 3+2+2/8 for a song that is already Balkan), else
    /// a plain beat made for `time` (see [`Rhythm::basic`]). A rhythm
    /// that follows the meter is in no time signature of its own, so it
    /// is never the one picked.
    pub fn for_time(time: TimeSignature, like: Option<&Rhythm>) -> &'static Rhythm {
        let in_time = |rhythm: &&'static Rhythm| rhythm.time == time && !rhythm.follows_the_meter;
        let same_kind =
            |rhythm: &&'static Rhythm| like.is_some_and(|like| like.genre == rhythm.genre);
        RHYTHMS
            .iter()
            .filter(in_time)
            .find(same_kind)
            .or_else(|| {
                USUAL
                    .iter()
                    .filter_map(|name| Rhythm::named(name))
                    .find(in_time)
            })
            .or_else(|| RHYTHMS.iter().find(in_time))
            .or_else(|| Rhythm::grouped_for(time, same_kind))
            .unwrap_or_else(|| Rhythm::basic(time))
    }

    /// A rhythm for `time` with its notes grouped: `time` is an odd
    /// meter written without its grouping, such as 5/4 or 7/8, and the
    /// rhythm has as many of the same notes to the bar. One of the kind
    /// `same_kind` picks comes first, then one grouped the way `time` is
    /// felt (see [`TimeSignature::felt_groups`]), then any.
    fn grouped_for(
        time: TimeSignature,
        same_kind: impl Fn(&&'static Rhythm) -> bool,
    ) -> Option<&'static Rhythm> {
        if time.is_grouped() || time.usual_groups() != 1 {
            return None;
        }
        let grouped = |rhythm: &&'static Rhythm| {
            rhythm.time.beats == time.beats
                && rhythm.time.unit == time.unit
                && !rhythm.follows_the_meter
        };
        let felt = time.felt_groups();
        RHYTHMS
            .iter()
            .filter(grouped)
            .find(&same_kind)
            .or_else(|| {
                RHYTHMS
                    .iter()
                    .filter(grouped)
                    .find(|rhythm| rhythm.time.groups() == felt)
            })
            .or_else(|| RHYTHMS.iter().find(grouped))
    }

    /// A plain beat for `time`, for the time signatures the library has
    /// no rhythm in, such as 5/4 or 2+2+3/8. The kick plays the first
    /// beat, and with four beats or more every other one; the snare
    /// plays the beats between (with three beats, two light ones, as in
    /// a waltz); the hi-hat plays every eighth note, a little louder on
    /// the beats.
    ///
    /// The beat is made the first time `time` asks for it, and kept for
    /// as long as the app runs: a [`Rhythm`] holds `'static` text, like
    /// the ones written in [`RHYTHMS`].
    pub fn basic(time: TimeSignature) -> &'static Rhythm {
        static MADE: Mutex<Vec<&'static Rhythm>> = Mutex::new(Vec::new());
        let mut made = MADE.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(beat) = made.iter().find(|beat| beat.time == time) {
            return beat;
        }
        let lines: Vec<(Drum, &'static str)> = basic_lines(time)
            .into_iter()
            .map(|(drum, line)| (drum, &*String::leak(line)))
            .collect();
        let beat: &'static Rhythm = Box::leak(Box::new(Rhythm {
            name: "Basic beat",
            genre: Genre::Pop,
            time,
            tempo: 100.0,
            lines: Vec::leak(lines),
            layout: None,
            fills: None,
            fill: "",
            about: "A plain beat on the beats of the bar.",
            swing: 0,
            follows_the_meter: false,
        }));
        made.push(beat);
        beat
    }
}

/// Where the beats of a bar of `time` start, as a drummer feels them:
/// odd numbers of eighths and sixteenths that are not grouped yet (5/8,
/// 7/8) are played in twos with a three at the end (2+3, 2+2+3).
fn felt_beats(time: TimeSignature) -> Vec<usize> {
    let beats = usize::from(time.beats);
    if time.unit >= 8 && !time.is_grouped() && time.usual_groups() == 1 && beats > 3 {
        let mut groups = vec![2; beats / 2];
        if beats % 2 == 1
            && let Some(last) = groups.last_mut()
        {
            *last = 3;
        }
        if let Some(grouped) = TimeSignature::grouped(&groups, time.unit) {
            return grouped.beat_starts();
        }
    }
    time.beat_starts()
}

/// The drum lines of [`Rhythm::basic`].
fn basic_lines(time: TimeSignature) -> Vec<(Drum, String)> {
    let bar = time.steps_per_bar();
    let beats = felt_beats(time);
    let mut kick = vec!['.'; bar];
    let mut snare = vec!['.'; bar];
    let mut hat = vec!['.'; bar];
    for (index, &start) in beats.iter().enumerate() {
        match (beats.len(), index) {
            (_, 0) => kick[start] = 'X',
            (2, _) => snare[start] = 'X',
            (3, _) => snare[start] = 'o',
            (_, index) if index % 2 == 0 => kick[start] = 'x',
            _ => snare[start] = 'X',
        }
    }
    for step in (0..bar).step_by(2) {
        hat[step] = if beats.contains(&step) { 'x' } else { 'o' };
    }
    vec![
        (Kick, kick.into_iter().collect()),
        (Snare, snare.into_iter().collect()),
        (HiHat, hat.into_iter().collect()),
    ]
}

/// A shorter way to write a 4/4 rhythm in the tables below.
const fn rhythm(
    name: &'static str,
    genre: Genre,
    tempo: f32,
    lines: &'static [(Drum, &'static str)],
    fill: &'static str,
    about: &'static str,
) -> Rhythm {
    Rhythm {
        name,
        genre,
        time: TimeSignature::COMMON,
        tempo,
        lines,
        fill,
        about,
        swing: 0,
        follows_the_meter: false,
        layout: None,
        fills: None,
    }
}

impl Rhythm {
    const fn in_time(self, time: TimeSignature) -> Rhythm {
        Rhythm { time, ..self }
    }

    const fn swung(self, swing: u8) -> Rhythm {
        Rhythm { swing, ..self }
    }

    const fn laid_out(self, layout: Layout) -> Rhythm {
        Rhythm {
            layout: Some(layout),
            ..self
        }
    }

    const fn filling_with(self, fills: &'static [&'static str]) -> Rhythm {
        Rhythm {
            fills: Some(fills),
            ..self
        }
    }

    const fn following_the_meter(self) -> Rhythm {
        Rhythm {
            follows_the_meter: true,
            ..self
        }
    }
}

const TWO_FOUR: TimeSignature = TimeSignature::new(2, 4);

/// How the candombes lay out their lines: four bars of opening; two
/// grooves of four bars for A parts and two for B parts, which their
/// phrases take turns with; and two closings of two bars.
const CANDOMBE: Layout = Layout {
    opening: 4,
    grooves: &[4, 4, 4, 4],
    turns: 2,
    closing: 2,
    closings: 2,
};
const THREE_FOUR: TimeSignature = TimeSignature::new(3, 4);
const SIX_EIGHT: TimeSignature = TimeSignature::new(6, 8);
const TWELVE_EIGHT: TimeSignature = TimeSignature::new(12, 8);

use Drum::*;
use Genre::*;

/// The library of drum patterns, sorted by genre.
pub static RHYTHMS: &[Rhythm] = &[
    // ----- Pop -----
    rhythm(
        "Pop beat",
        Pop,
        110.0,
        &[
            (Kick, "X... .... X.x. ...."),
            (Snare, ".... X... .... X..."),
            (HiHat, "x.x. x.x. x.x. x.x."),
        ],
        "Snare roll",
        "The backbeat: kick on 1 and 3, snare on 2 and 4, hi-hat on every eighth note.",
    ),
    rhythm(
        "Pop ballad",
        Pop,
        72.0,
        &[
            (Kick, "X... .... ..x. ...."),
            (Rim, ".... x... .... x..."),
            (HiHat, "x.x. x.x. x.x. x.x."),
        ],
        "Tom fill",
        "A soft beat with a rim click in place of the snare.",
    ),
    rhythm(
        "Dance pop",
        Pop,
        122.0,
        &[
            (Kick, "X... X... X... X..."),
            (Clap, ".... X... .... X..."),
            (OpenHat, "..x. ..x. ..x. ..x."),
            (Shaker, "xoxo xoxo xoxo xoxo"),
        ],
        "House build",
        "A kick on every beat, a clap on 2 and 4, and an open hat between the beats.",
    ),
    rhythm(
        "Afrobeats",
        Pop,
        105.0,
        &[
            (Kick, "X... ..X. ..X. ...."),
            (Clap, ".... X... .... X..."),
            (Rim, "...x ..x. ...x ..x."),
            (Shaker, "x.xx x.xx x.xx x.xx"),
            (Conga, "..o. .o.. ..o. h.h."),
        ],
        "Latin fill",
        "A rolling West African groove: the kick plays 3 + 3 + 2 and the shaker never stops.",
    ),
    rhythm(
        "Highlife",
        Pop,
        116.0,
        &[
            (Cowbell, "x..x ..x. ..x. x..."),
            (Conga, "..o. h.h. ..o. h.hh"),
            (Kick, "X... ..x. X... ...."),
            (Shaker, "x.x. x.x. x.x. x.x."),
        ],
        "Latin fill",
        "Ghana's highlife: the bell's time line, 3 + 3 + 4 + 2 + 4 sixteenths, congas and a \
         light kick, under bright, ringing guitars.",
    ),
    rhythm(
        "Soukous",
        Pop,
        132.0,
        &[
            (Kick, "X... X... X... X..."),
            (Snare, "x.xx x.xx x.xx x.xx"),
            (HiHat, "..x. ..x. ..x. ..x."),
        ],
        "Snare roll",
        "Congolese soukous, the dance music of Kinshasa: the snare gallops in sixteenths, \
         the \"cavacha\", over a kick on every beat.",
    ),
    rhythm(
        "Kizomba",
        Pop,
        92.0,
        &[
            (Kick, "X... ..x. X... ...."),
            (Rim, ".... x... .... x..."),
            (Shaker, "xoox ooxo xoox ooxo"),
            (Conga, "...o ..o. ...o .o.."),
        ],
        "",
        "Angolan kizomba, born in the 1980s from semba and the zouk of the French Antilles: \
         slow and rolling, the bass drum on 1 and 3 with a pickup between them, a soft rim on \
         2 and 4, and the shaker leaning on 3 + 3 + 2.",
    ),
    // ----- Rock -----
    rhythm(
        "Rock beat",
        Rock,
        120.0,
        &[
            (Kick, "X... ..x. X.x. ...."),
            (Snare, ".... X... .... X..."),
            (HiHat, "x.x. x.x. x.x. x.x."),
        ],
        "Tom fill",
        "The backbeat with an extra kick pushing into beat 3.",
    ),
    rhythm(
        "Half-time rock",
        Rock,
        140.0,
        &[
            (Kick, "X... .... ..x. ...."),
            (Snare, ".... .... X... ...."),
            (HiHat, "x.x. x.x. x.x. x.x."),
        ],
        "Tom fill",
        "The snare only on beat 3: the song feels half as fast as its tempo.",
    ),
    rhythm(
        "Punk",
        Rock,
        170.0,
        &[
            (Kick, "X... ..X. X... ..X."),
            (Snare, ".... X... .... X..."),
            (HiHat, "X.x. X.x. X.x. X.x."),
        ],
        "Snare roll",
        "The backbeat, fast and hard.",
    ),
    rhythm(
        "Rock ride",
        Rock,
        128.0,
        &[
            (Kick, "X... ..x. X... ...."),
            (Snare, ".... X... .... X..."),
            (Ride, "x.x. x.x. x.x. x.x."),
        ],
        "Tom fill",
        "The backbeat with the eighths on the ride cymbal, for a bigger, washier sound.",
    ),
    rhythm(
        "Surf beat",
        Rock,
        160.0,
        &[
            (Kick, "X... X... X... X..."),
            (Snare, ".... X... .... X..."),
            (LowTom, "..x. ..x. ..x. .xx."),
            (HiHat, "x.x. x.x. x.x. x.x."),
        ],
        "Tom fill",
        "A kick on every beat and rumbling toms: 1960s surf rock.",
    ),
    rhythm(
        "Rock and roll",
        Rock,
        168.0,
        &[
            (Kick, "X... ..x. X... ..x."),
            (Snare, ".... X... .... X..."),
            (Ride, "x.x. x.x. x.x. x.x."),
        ],
        "Snare roll",
        "The 1950s backbeat, with a long-short ride cymbal halfway between swing and straight.",
    )
    .swung(50),
    rhythm(
        "Metal",
        Rock,
        180.0,
        &[
            (Kick, "XxXx XxXx XxXx XxXx"),
            (Snare, ".... X... .... X..."),
            (Crash, "X... .... .... ...."),
            (Ride, "x... x... x... x..."),
        ],
        "Tom fill",
        "Two bass drums in sixteenths under a heavy backbeat.",
    ),
    rhythm(
        "Five-four rock",
        Rock,
        120.0,
        &[
            (Kick, "X... ..x. .... X... ...."),
            (Snare, ".... .... X... .... X..."),
            (HiHat, "x.x. x.x. x.x. x.x. x.x."),
        ],
        "Snare roll",
        "A rock beat in 5/4, counted 3 + 2: kick on 1 and 4, snare on 3 and 5.",
    )
    .in_time(TimeSignature::of_groups(&[3, 2], 4)),
    // ----- Blues -----
    rhythm(
        "Shuffle",
        Blues,
        96.0,
        &[
            (Kick, "X... .... X... ...."),
            (Snare, ".... X... .... X..."),
            (HiHat, "x.x. x.x. x.x. x.x."),
        ],
        "Snare roll",
        "Long-short pairs: each beat split into a triplet's first two thirds and its last.",
    )
    .swung(100),
    rhythm(
        "Slow blues",
        Blues,
        66.0,
        &[
            (Kick, "X..... ...... X..... ......"),
            (Snare, "...... X..... ...... X....."),
            (HiHat, "x.x.x. x.x.x. x.x.x. x.x.x."),
        ],
        "",
        "Twelve-eight: four slow beats, each split in three, for the blues ballad.",
    )
    .in_time(TWELVE_EIGHT),
    // ----- Jazz -----
    rhythm(
        "Swing",
        Jazz,
        140.0,
        &[
            (Ride, "x... x.x. x... x.x."),
            (HiHat, ".... x... .... x..."),
            (Kick, "o... .... o... ...."),
        ],
        "",
        "Ding, ding-a, ding, ding-a on the ride, and the hi-hat pedal on 2 and 4.",
    )
    .swung(100),
    rhythm(
        "Jazz ballad",
        Jazz,
        68.0,
        &[
            (Ride, "o... o.o. o... o.o."),
            (Shaker, "o.o. o.o. o.o. o.o."),
            (HiHat, ".... x... .... x..."),
            (Kick, "o... .... .... ...."),
        ],
        "",
        "Brushes and a quiet ride: the drums only keep time.",
    )
    .swung(100),
    rhythm(
        "Jazz waltz",
        Jazz,
        150.0,
        &[
            (Ride, "x... x.x. x.x."),
            (HiHat, ".... x... x..."),
            (Kick, "o... .... ...."),
        ],
        "",
        "The swing ride pattern in three beats.",
    )
    .in_time(THREE_FOUR)
    .swung(100),
    rhythm(
        "Second line",
        Jazz,
        100.0,
        &[
            (Kick, "X... ..X. X... ..X."),
            (Snare, ".oxo X.o. .oxo X.ox"),
            (Cowbell, "x... x... x... x..."),
        ],
        "Snare roll",
        "New Orleans street parade drumming: a bass drum that answers itself under a busy snare.",
    ),
    rhythm(
        "Five-four swing",
        Jazz,
        170.0,
        &[
            (Ride, "x... x.x. x... x... x.x."),
            (HiHat, ".... x... .... .... x..."),
            (Kick, "o... .... .... o... ...."),
        ],
        "",
        "Swing in five, counted 3 + 2: a jazz waltz bar and a bar of two, the ride \
         marking where each starts.",
    )
    .in_time(TimeSignature::of_groups(&[3, 2], 4))
    .swung(100),
    rhythm(
        "Five-four spy",
        Jazz,
        160.0,
        &[
            (Kick, "X... ..x. .... X... x..."),
            (Conga, "h.x. x.h. x.x. h.x. h.x."),
            (Shaker, "o.o. o.o. o.o. o.o. o.o."),
        ],
        "",
        "The long, long, short, short of a 1960s spy theme in 5/4: the bar's ten \
         eighths go 3 + 3 + 2 + 2, the bass drum and the high conga on each start.",
    )
    .in_time(TimeSignature::of_groups(&[3, 2], 4)),
    rhythm(
        "Cakewalk",
        Jazz,
        104.0,
        &[
            (Kick, "X... x... X... x..."),
            (Snare, ".... ..x. .... ..x."),
            (Rim, ".o.o .... .o.o ...."),
        ],
        "Snare roll",
        "The drummer of a ragtime band around 1900, when the cakewalk was the dance craze: \
         the bass drum on every beat, the rim completing the cakewalk's own figure on the \
         first, short-long-short, and the snare after the second with the piano's chord. \
         Ragtime is written in 2/4; each bar here lasts two.",
    ),
    rhythm(
        "Stride",
        Jazz,
        176.0,
        &[
            (HiHat, ".... x... .... x..."),
            (Shaker, "o.o. o.o. o.o. o.o."),
            (Kick, "o... .... o... ...."),
        ],
        "",
        "Brushes behind a Harlem stride pianist: the hi-hat on 2 and 4 with the left hand's \
         chords, and a soft sweep in swung eighths.",
    )
    .swung(100),
    rhythm(
        "Big band swing",
        Jazz,
        160.0,
        &[
            (
                Ride,
                "x... x.x. x... x.x. | x... x.x. x... x.x. | x... x.x. x... x.x. | x... x.x. x... x.x.",
            ),
            (
                HiHat,
                ".... x... .... x... | .... x... .... x... | .... x... .... x... | .... x... .... x...",
            ),
            (
                Kick,
                "o... o... o... o... | o... o... o... o... | o... o... o... o... | o... o... o... o...",
            ),
            (
                Snare,
                ".... ..o. .... .... | .... .... ..o. ..x. | .... ..o. .... .... | .... .... ..o. ..x.",
            ),
            (
                Crash,
                "x... .... .... .... | .... .... .... .... | .... .... .... .... | .... .... .... ....",
            ),
        ],
        "Snare roll",
        "The drummer of a swing big band in the 1930s and 1940s: the ride and the hi-hat as in \
         a small group, the bass drum \"feathered\" softly on every beat, the snare kicking \
         the horns' figures, and a crash every four bars where the phrase starts again.",
    )
    .swung(100),
    rhythm(
        "Big band ballad",
        Jazz,
        66.0,
        &[
            (Ride, "o... o.o. o... o.o."),
            (Shaker, "o.o. o.o. o.o. o.o."),
            (HiHat, ".... x... .... x..."),
            (Kick, "o... .... o... ...."),
        ],
        "",
        "Brushes under a big band's ballad: the saxophones hold the chords while the drums \
         only keep time.",
    )
    .swung(100),
    // ----- Soul, funk and R&B -----
    rhythm(
        "Motown",
        Soul,
        112.0,
        &[
            (Kick, "X... ..x. X... ..x."),
            (Snare, "x... X... x... X..."),
            (Shaker, ".... X... .... X..."),
            (HiHat, "x.x. x.x. x.x. x.x."),
        ],
        "Snare roll",
        "The snare on all four beats and a tambourine on 2 and 4.",
    ),
    rhythm(
        "Funk",
        Soul,
        102.0,
        &[
            (Kick, "X..x ..X. ..x. ...."),
            (Snare, ".... X..o .o.. X..o"),
            (HiHat, "xoxo xoxo xoxo xoxo"),
        ],
        "Snare roll",
        "Sixteenths on the hi-hat, ghost notes on the snare, and a kick that never sits still.",
    ),
    rhythm(
        "Neo soul",
        Soul,
        84.0,
        &[
            (Kick, "X... ...x ..x. ...."),
            (Snare, ".... X... .... X..."),
            (HiHat, "x.xo x.x. x.xo x.x."),
        ],
        "Hip hop fill",
        "A laid-back backbeat with a kick that comes a little early.",
    ),
    rhythm(
        "R&B",
        Soul,
        90.0,
        &[
            (Kick, "X... ...x .x.. ...."),
            (Clap, ".... X... .... X..."),
            (HiHat, "x.x. x.x. x.xx x.x."),
        ],
        "Hip hop fill",
        "A clap on 2 and 4, a syncopated kick and a quick double hi-hat.",
    ),
    rhythm(
        "New jack swing",
        Soul,
        104.0,
        &[
            (Kick, "X..X ..X. X..X ..X."),
            (Snare, ".... X... .... X..."),
            (Clap, ".... X... .... X..."),
            (HiHat, "x.xx x.xx x.xx x.xx"),
        ],
        "Hip hop fill",
        "Late-1980s R&B: a hip hop kick and snare with a skipping hi-hat.",
    ),
    rhythm(
        "Afrobeat",
        Soul,
        108.0,
        &[
            (Kick, "X..x ..x. X... ..x."),
            (Snare, "..o. x.o. .o.. x..o"),
            (HiHat, "xxXx xxXx xxXx xxXx"),
            (Rim, "x..x ..x. ..x. x..."),
        ],
        "Snare roll",
        "Fela Kuti's Afrobeat, from 1970s Lagos, after Tony Allen's drumming: a busy hi-hat \
         in sixteenths, ghost notes on the snare, a kick that dodges the beat, and claves \
         keeping a time line under the long, funky vamp.",
    ),
    // ----- Hip hop -----
    rhythm(
        "Boom bap",
        HipHop,
        90.0,
        &[
            (Kick, "X... ...x ..X. ...."),
            (Snare, ".... X... .... X..."),
            (HiHat, "x.x. x.x. x.x. x.x."),
        ],
        "Hip hop fill",
        "The \"boom\" of the kick and the \"bap\" of the snare: 1990s hip hop.",
    ),
    rhythm(
        "Lo-fi",
        HipHop,
        75.0,
        &[
            (Kick, "X... .... ..X. ...."),
            (Snare, ".... X... .... X..."),
            (HiHat, "x.x. x.x. x.x. x.xo"),
        ],
        "Hip hop fill",
        "A slow, soft hip hop beat to study to.",
    ),
    rhythm(
        "Trap",
        HipHop,
        140.0,
        &[
            (Kick, "X... .... ..X. .... | .... ..X. .... ...."),
            (Clap, ".... .... X... .... | .... .... X... ...."),
            (HiHat, "x.x. x.x. xxx. x.x. | x.x. x.xx x.x. xxxx"),
        ],
        "",
        "Half-time: the clap only on beat 3, with quick rolls on the hi-hat.",
    ),
    rhythm(
        "Drill",
        HipHop,
        142.0,
        &[
            (Kick, "X... .... ..X. ..X."),
            (Snare, ".... .... X... ...."),
            (HiHat, "x..x ..x. x..x ..x."),
            (Rim, "...x .... ...x ...."),
        ],
        "",
        "Half-time, with a hi-hat in 3 + 3 + 2 and kicks that dodge the snare.",
    ),
    rhythm(
        "G-funk",
        HipHop,
        92.0,
        &[
            (Kick, "X... ..x. X.X. ...."),
            (Clap, ".... X... .... X..."),
            (HiHat, "x.x. x.x. x.x. x.xx"),
            (Shaker, "..x. ..x. ..x. ..x."),
        ],
        "Hip hop fill",
        "1990s West Coast hip hop: a laid-back, funky kick and a clap on 2 and 4.",
    ),
    // ----- Electronic -----
    rhythm(
        "Four on the floor",
        Electronic,
        124.0,
        &[
            (Kick, "X... X... X... X..."),
            (Clap, ".... X... .... X..."),
            (OpenHat, "..x. ..x. ..x. ..x."),
            (Shaker, "xoxo xoxo xoxo xoxo"),
        ],
        "House build",
        "A kick on every beat: the heartbeat of house and disco.",
    ),
    rhythm(
        "Techno",
        Electronic,
        130.0,
        &[
            (Kick, "X... X... X... X..."),
            (OpenHat, "..x. ..x. ..x. ..x."),
            (HiHat, "xoxo xoxo xoxo xoxo"),
            (Rim, "...x ..x. ...x .x.."),
        ],
        "House build",
        "A relentless kick with rim shots weaving between the beats.",
    ),
    rhythm(
        "Trance",
        Electronic,
        138.0,
        &[
            (Kick, "X... X... X... X..."),
            (Clap, ".... X... .... X..."),
            (OpenHat, "..x. ..x. ..x. ..x."),
            (HiHat, "xox. xox. xox. xox."),
        ],
        "House build",
        "Four on the floor, a little faster, built for long builds and drops.",
    ),
    rhythm(
        "Disco",
        Electronic,
        118.0,
        &[
            (Kick, "X... X... X... X..."),
            (Snare, ".... X... .... X..."),
            (HiHat, "x... x... x... x..."),
            (OpenHat, "..x. ..x. ..x. ..x."),
        ],
        "Tom fill",
        "Closed hat on the beat, open hat between: the 1970s dance floor.",
    ),
    rhythm(
        "Drum and bass",
        Electronic,
        174.0,
        &[
            (Kick, "X... .... ..X. ...."),
            (Snare, ".... X... .... X..."),
            (HiHat, "x.x. x.x. x.x. x.x."),
        ],
        "Snare roll",
        "A fast two-step: the kick on 1 and the \"and\" of 3.",
    ),
    rhythm(
        "Dubstep",
        Electronic,
        140.0,
        &[
            (Kick, "X... .... .... ..x."),
            (Snare, ".... .... X... ...."),
            (HiHat, "x.x. x.x. x.x. x.x."),
        ],
        "",
        "Half-time, heavy and slow-feeling, with a big snare on beat 3.",
    ),
    rhythm(
        "UK garage",
        Electronic,
        132.0,
        &[
            (Kick, "X... ...x ..X. ...."),
            (Snare, ".... X... .... X..."),
            (HiHat, "..x. ..x. ..x. ..x."),
            (Shaker, "xoxo xoxo xoxo xoxo"),
        ],
        "",
        "The skippy two-step of UK garage: a kick that dodges the beat.",
    ),
    rhythm(
        "Breakbeat",
        Electronic,
        130.0,
        &[
            (Kick, "X.x. .... ..X. ...."),
            (Snare, ".... X..o .o.. X..o"),
            (HiHat, "x.x. x.x. x.x. x.x."),
        ],
        "Snare roll",
        "A funky drum break, the raw material of hip hop and jungle.",
    ),
    rhythm(
        "Synthwave",
        Electronic,
        100.0,
        &[
            (Kick, "X... .... X... ...."),
            (Snare, ".... X... .... X..."),
            (Clap, ".... X... .... X..."),
            (HiHat, "x.x. x.x. x.x. x.x."),
        ],
        "Tom fill",
        "A big 1980s backbeat, with a clap on top of the snare.",
    ),
    rhythm(
        "Moombahton",
        Electronic,
        108.0,
        &[
            (Kick, "X... X... X... X..."),
            (Snare, "...X ..X. ...X ..X."),
            (OpenHat, "..x. ..x. ..x. ..x."),
        ],
        "House build",
        "House slowed down to reggaeton's tempo: four on the floor under a dembow snare.",
    ),
    rhythm(
        "Electro",
        Electronic,
        128.0,
        &[
            (Kick, "X... .... ..X. ...."),
            (Clap, ".... X... .... X..."),
            (HiHat, "xoxo xoxo xoxo xoxo"),
            (Cowbell, "..x. .... x... ..x."),
        ],
        "Snare roll",
        "The drum machines of early-1980s electro: a broken kick, sixteenths on the hat, a cowbell.",
    ),
    // ----- Latin -----
    rhythm(
        "Bossa nova",
        Latin,
        130.0,
        &[
            (Kick, "X..x X... X..x X... | X..x X... X..x X..."),
            (Rim, "x..x ..x. ..x. .x.. | ..x. ..x. .x.. x..."),
            (HiHat, "o.o. o.o. o.o. o.o. | o.o. o.o. o.o. o.o."),
        ],
        "Latin fill",
        "A quiet kick, soft eighths, and the rim playing the bossa clave across two bars.",
    ),
    rhythm(
        "Samba",
        Latin,
        100.0,
        &[
            (Kick, "X..x X..x X..x X..x"),
            (LowTom, ".... X... .... X..."),
            (Snare, "x.oo x.oo x.oo x.oo"),
            (Shaker, "Xoxx Xoxx Xoxx Xoxx"),
            (Cowbell, "x.x. ..x. x... x.h."),
        ],
        "Latin fill",
        "The low drum (surdo) strongest on beat 2, shakers and bells in sixteenths.",
    ),
    rhythm(
        "Salsa",
        Latin,
        95.0,
        &[
            (Rim, "x..x ..x. ..x. x..."),
            (Cowbell, "X.x. X.x. X.x. X.x."),
            (Conga, "o.x. o.hh o.x. o.hh"),
        ],
        "Latin fill",
        "The son clave (3 + 2) on the rim, the bell on the beats, and the conga's tumbao.",
    ),
    rhythm(
        "Cumbia",
        Latin,
        95.0,
        &[
            (Kick, "X... .... X... ...."),
            (Shaker, "x.xx x.xx x.xx x.xx"),
            (Conga, "..x. ..x. ..x. ..h."),
            (Rim, ".... x... .... x..."),
        ],
        "Latin fill",
        "The scraper's long-short-short, and congas between the beats.",
    ),
    rhythm(
        "Reggaeton",
        Latin,
        95.0,
        &[
            (Kick, "X... X... X... X..."),
            (Snare, "...x ..x. ...x ..x."),
            (HiHat, "x.x. x.x. x.x. x.x."),
        ],
        "",
        "The dembow: a kick on every beat and a snare in 3 + 3 + 2.",
    ),
    rhythm(
        "Bolero",
        Latin,
        80.0,
        &[
            (Kick, "X... .... X... ...."),
            (Rim, ".... x... .... x..."),
            (Conga, "..o. ..hh ..o. hh.."),
            (Shaker, "x.x. x.x. x.x. x.x."),
        ],
        "Latin fill",
        "The slow Latin love song: soft bongos and maracas, and the rim on 2 and 4.",
    ),
    rhythm(
        "Bachata",
        Latin,
        128.0,
        &[
            (Kick, "X... X... X... X..."),
            (Conga, "x.hh x.h. x.hh x.hl"),
            (Shaker, "xxxx xxxx xxxx xxxx"),
        ],
        "Latin fill",
        "From the Dominican Republic: bongos in a steady pattern and the güira scraping every \
         sixteenth.",
    ),
    rhythm(
        "Merengue",
        Latin,
        132.0,
        &[
            (Kick, "X... X... X... X..."),
            (LowTom, "X..x .x.x X..x .xxx"),
            (Shaker, "x.xx x.xx x.xx x.xx"),
        ],
        "Latin fill",
        "Fast and in two: the tambora drum's rolls and a scraper that never stops.",
    ),
    rhythm(
        "Cha-cha-chá",
        Latin,
        116.0,
        &[
            (Kick, "X... .... X... ...."),
            (Cowbell, "X... X... X... X..."),
            (Conga, ".... ..h. ..hh h..."),
            (Rim, ".... .... .... x.x."),
        ],
        "Latin fill",
        "Named after its sound: the two quick notes and one long, \"cha-cha-chá\", on beat 4 and \
         the next beat 1.",
    ),
    rhythm(
        "Son",
        Latin,
        192.0,
        &[
            (Rim, ".... x... x... .... | x... ..x. .... x..."),
            (Conga, "h.o. o.o. h.o. o.hh | h.o. o.o. h.o. o.hh"),
            (Shaker, "X.x. x.x. X.x. x.x. | X.x. x.x. X.x. x.x."),
        ],
        "Latin fill",
        "The Cuban son of the 1920s, salsa's parent: the clave across two bars, here 2 + 3 (its \
         bar of two hits first), the bongo's steady \"hammer\" and maracas in eighths, and no \
         drum kit.",
    ),
    rhythm(
        "Choro",
        Latin,
        100.0,
        &[
            (Kick, "o... x... o... x..."),
            (Shaker, "Xoox Xoox Xoox Xoox"),
            (Rim, ".... ..x. .... ..x."),
        ],
        "Latin fill",
        "Rio de Janeiro's choro, played by a pandeiro (a tambourine): jingles in sixteenths, the \
         low thump stronger on the second beat. Choro is in 2/4; each bar here lasts two.",
    ),
    rhythm(
        "Maxixe",
        Latin,
        90.0,
        &[
            (Kick, "X... x... X... x..."),
            (Shaker, "xoxx xoxo xoxx xoxo"),
            (Rim, ".x.x ..x. .x.x ..x."),
        ],
        "Latin fill",
        "Rio de Janeiro's maxixe, the \"Brazilian tango\" of the 1900s: the rim plays its \
         syncopation, short, long, short, then two even notes, over a steady bass drum. The \
         maxixe is in 2/4; each bar here lasts two.",
    ),
    rhythm(
        "Marchinha",
        Latin,
        116.0,
        &[
            (Kick, "x... X... x... X..."),
            (Snare, "x.xx x.x. x.xx x.x."),
        ],
        "Latin fill",
        "The carnival march of Rio de Janeiro: the surdo (a big bass drum) on both beats, \
         the second one stronger, under the snare's rattle. The marchinha is in 2/4; each bar \
         here lasts two.",
    ),
    rhythm(
        "Ranchera",
        Latin,
        150.0,
        &[(Kick, "X... .... ...."), (Rim, ".... x.o. x.o.")],
        "",
        "The Mexican mariachi waltz has no drums: the guitarrón (a big bass guitar) plays beat 1, \
         the vihuela strums beats 2 and 3. A soft kick and rim stand in for them.",
    )
    .in_time(THREE_FOUR)
    // No drums, so no drum fills.
    .filling_with(&[]),
    rhythm(
        "Mambo",
        Latin,
        104.0,
        &[
            (Rim, "x.xx .x.x x.x. xx.x"),
            (Cowbell, "X... X... X... X..."),
            (Conga, "o.x. o.hh o.x. o.hh"),
        ],
        "Latin fill",
        "The mambo of 1950s Havana, Mexico City and New York: the timbales' sticks on the \
         drum's shell, the bongo player's bell on every beat, and the conga's tumbao.",
    ),
    rhythm(
        "Guaguancó",
        Latin,
        100.0,
        &[
            (Rim, "x..x ...x ..x. x..."),
            (Conga, "..o. h.hl ..o. h.hh"),
            (Shaker, "x.x. x.x. x.x. x.x."),
        ],
        "Latin fill",
        "The Cuban street rumba: the rumba clave, whose third stroke comes a step later than \
         the son clave's, under the conversation of the conga drums.",
    ),
    rhythm(
        "Afro-Cuban 6/8",
        Latin,
        150.0,
        &[
            (Cowbell, "x...x. ..x.x. ..x... x...x."),
            (Kick, "X..... x..... X..... x....."),
            (Conga, "....h. ....h. ....h. ....h."),
            (Shaker, "x.x.x. x.x.x. x.x.x. x.x.x."),
        ],
        "",
        "The 6/8 of Cuba's drums of African origin, as in the bembé: a bell plays the \
         seven-stroke pattern, twelve eighths long, that the whole band listens to, over four \
         slow beats. Afro-Cuban jazz took it up.",
    )
    .in_time(TWELVE_EIGHT),
    rhythm(
        "Baião",
        Latin,
        104.0,
        &[
            (Kick, "X..X .... X..X ...."),
            (Rim, "..x. ..x. ..x. ..x."),
            (HiHat, "x.xx x.xx x.xx x.xx"),
        ],
        "Latin fill",
        "The baião of Brazil's northeast, Luiz Gonzaga's dance: the zabumba drum's long-short \
         on the beat, its stick between the beats, and a triangle that never stops. The baião \
         is in 2/4; each bar here lasts two.",
    ),
    rhythm(
        "Vallenato",
        Latin,
        100.0,
        &[
            (Kick, "X... .... X... ...."),
            (Conga, "h..x h.x. h..x h.x."),
            (Shaker, "x.xx x.xx x.xx x.xx"),
        ],
        "Latin fill",
        "The paseo of Colombia's vallenato: the caja, a small drum played with the hands, and \
         the guacharaca, a ridged stick scraped with a fork, under the accordion.",
    ),
    // ----- Tango -----
    rhythm(
        "Tango marcato",
        Tango,
        118.0,
        &[(Kick, "X... x... X... x..."), (Rim, ".... .... .... ..xx")],
        "",
        "Tango has no drums: the bass and piano mark every beat. A soft kick does it here, \
         with the \"drag\" into the next bar.",
    ),
    rhythm(
        "Tango síncopa",
        Tango,
        118.0,
        &[(Kick, "X... ..X. .... X..."), (Rim, ".... .... x... ....")],
        "",
        "Three plus three plus two: the accents of the síncopa.",
    ),
    rhythm(
        "Milonga",
        Tango,
        110.0,
        &[
            (Kick, "X... ..X. .... X..."),
            (Shaker, "x.x. x.x. x.x. x.x."),
            (Rim, ".... x... .... x..."),
        ],
        "",
        "3 + 3 + 2 in every bar, light and quick.",
    ),
    rhythm(
        "Habanera",
        Tango,
        76.0,
        &[(Kick, "X..x X.x. X..x X.x."), (Rim, ".... x... .... x...")],
        "",
        "Long, short, long, long, twice a bar.",
    ),
    rhythm(
        "Tango vals",
        Tango,
        180.0,
        &[(Kick, "X... .... ...."), (Rim, ".... x... x...")],
        "",
        "A fast waltz: a strong one and two light beats.",
    )
    .in_time(THREE_FOUR),
    rhythm(
        "Tango nuevo",
        Tango,
        132.0,
        &[
            (Kick, "X..X ..X. .X.. ...."),
            (Rim, ".... .... .... X.X."),
            (Shaker, "x.xx x.xx x.xx x.xx"),
        ],
        "",
        "3 + 3 + 3 + 3 + 2 + 2 in sixteenths: the restless accents of the new tango of the 1950s.",
    ),
    // ----- Latin American folklore -----
    rhythm(
        "Zamba",
        Folklore,
        84.0,
        &[(LowTom, "X..... X....."), (Rim, "....x. ..x.x.")],
        "",
        "The slow Argentine dance in 6/8: the bombo legüero's deep head on the two beats, \
         its wooden rim between them.",
    )
    .in_time(SIX_EIGHT),
    rhythm(
        "Chacarera",
        Folklore,
        132.0,
        &[(LowTom, "X..... x.x..."), (Rim, "....x. ..x.x.")],
        "",
        "Santiago del Estero's dance: the bombo plays 6/8 on the head and 3/4 on the rim at \
         the same time, the two against three that the whole genre lives on.",
    )
    .in_time(SIX_EIGHT),
    rhythm(
        "Gato",
        Folklore,
        126.0,
        &[
            (LowTom, "X..... X..x.."),
            (Rim, "..x.x. ..x..."),
            (Shaker, "x.x.x. x.x.x."),
        ],
        "",
        "A quick, bright 6/8 for the gato, the pericón, the cielito, the escondido and the \
         other old dances of the Pampas.",
    )
    .in_time(SIX_EIGHT),
    rhythm(
        "Malambo",
        Folklore,
        144.0,
        &[(LowTom, "X..x.. X..x.."), (Rim, "x.x.x. x.x.x.")],
        "",
        "The gaucho's solo dance: bombo and the dancer's boots, in 3 + 3 against the eighths.",
    )
    .in_time(SIX_EIGHT),
    rhythm(
        "Chamamé",
        Folklore,
        120.0,
        &[(Kick, "X...x. ..x..."), (HiHat, "X.x.x. X.x.x.")],
        "",
        "From Corrientes: the guitars' bass walks in 3/4 while the accordion plays 6/8, \
         three against two in every bar.",
    )
    .in_time(SIX_EIGHT),
    rhythm(
        "Cueca",
        Folklore,
        126.0,
        &[
            (Kick, "X..... X....."),
            (Clap, "..X.X. ..X.X."),
            (Shaker, "x.x.x. x.x.x."),
        ],
        "",
        "Chile's handkerchief dance: guitar on the two beats, hand claps on the eighths between.",
    )
    .in_time(SIX_EIGHT),
    rhythm(
        "Marinera",
        Folklore,
        120.0,
        &[
            (LowTom, "X..x.. X....."),
            (Snare, "....x. ..x.x."),
            (Shaker, "x.x.x. x.x.x."),
        ],
        "",
        "Peru's courtship dance on the cajón: a low tone, slaps that cross into 3/4, and \
         the quijada's rattle.",
    )
    .in_time(SIX_EIGHT),
    rhythm(
        "Bambuco",
        Folklore,
        108.0,
        &[
            (Rim, "X..... X....."),
            (Kick, "....X. ..X..."),
            (Shaker, "x.x.x. x.x.x."),
        ],
        "",
        "Colombia's bambuco: the tiple keeps 6/8 while the bass lands on the second and third \
         beats of 3/4, never on the downbeat.",
    )
    .in_time(SIX_EIGHT),
    rhythm(
        "Joropo",
        Folklore,
        176.0,
        &[
            (Kick, "X... ..X. ...."),
            (Shaker, "X.x. X.x. X.x."),
            (Rim, ".... x... x..."),
        ],
        "",
        "The Venezuelan and Colombian plains: maracas in 3/4, the bass in 6/8, and the cuatro \
         strumming across both.",
    )
    .in_time(THREE_FOUR),
    rhythm(
        "Vidala",
        Folklore,
        72.0,
        &[(Snare, "X... o... o.o.")],
        "",
        "The caja, a small hand drum, under a slow song of the Andean north-west of Argentina.",
    )
    .in_time(THREE_FOUR),
    rhythm(
        "Carnavalito",
        Folklore,
        116.0,
        &[
            (LowTom, "X... X... X... X..."),
            (Snare, "x.xx x.xx x.xx x.xx"),
            (Shaker, "x.x. x.x. x.x. x.x."),
        ],
        "",
        "The carnival dance of the Quebrada de Humahuaca: bombo on every beat and the caja's \
         long-short-short, in a quick 2/4 written here as pairs of bars.",
    ),
    rhythm(
        "Carnavalito (2/4)",
        Folklore,
        88.0,
        &[(LowTom, "X... X..."), (HighTom, "..xx ..xx")],
        "",
        "The carnavalito with a bar of 2/4 for each two beats, as Ariel Ramírez writes it \
         in the Gloria of the Misa Criolla: the bombo strikes each beat and answers it with \
         two quick sixteenths.",
    )
    .in_time(TWO_FOUR),
    rhythm(
        "Yaraví",
        Folklore,
        44.0,
        &[(LowTom, "X.x.x.x.x.x.")],
        "",
        "The yaraví, the slow lament of the Andes: the bombo, softly, on every eighth note \
         of a 3/4 bar, as under the middle of the Gloria of the Misa Criolla.",
    )
    .in_time(THREE_FOUR),
    rhythm(
        "Chacarera trunca",
        Folklore,
        152.0,
        &[
            (LowTom, "X...x....... | ........X..."),
            (Rim, "........xxxx | x.........x."),
            (HighTom, "..x.x...x... | ..x.x...x..."),
        ],
        "",
        "The chacarera trunca in a quick 3/4, as in the Credo of the Misa Criolla: two bars \
         of the bombo, its head on the first two beats and then on the third, its rim \
         answering with four sixteenths, while the sung phrases stop short before the \
         third beat of their second bar.",
    )
    .in_time(THREE_FOUR),
    rhythm(
        "Carnaval cochabambino",
        Folklore,
        152.0,
        &[
            (HighTom, "x.....x....."),
            (LowTom, "....x...x..."),
            (Rim, "x.....x...x."),
            (Snare, "....XXXXx..."),
        ],
        "",
        "The carnival of Cochabamba, in Bolivia, in a quick 3/4, as in the Sanctus of the \
         Misa Criolla: two drums answering each other, a wood block, and a snare's burst of \
         four loud sixteenths on the second beat.",
    )
    .in_time(THREE_FOUR),
    rhythm(
        "Huayno",
        Folklore,
        100.0,
        &[
            (LowTom, "X... x... X... x..."),
            (Rim, "..x. ..x. ..x. ..x."),
            (Shaker, "x.x. x.x. x.x. x.x."),
        ],
        "",
        "The Andean huayno of Peru and Bolivia: a bombo on the beat, a tap off it, and the \
         charango's steady strum; 2/4, written as pairs of bars.",
    ),
    // Two candombe percussion sheets ("Materiales de la cuerda del
    // Candombe" and a murga sheet's candombe page) give each drum's
    // patterns; the rhythm lays them out over each part of a song (see
    // `Layout`): four bars of opening; two grooves of four bars for A
    // parts and two for B parts, which a part's phrases take turns with,
    // so no two phrases in a row sound the same; and two closings of two
    // bars, taken in turns too. The repique plays the 3-2 clave on the
    // wood (Rim) in the opening and in the first bar of every second
    // phrase and of every second closing, its "continuo" base (High
    // tom) in the grooves, the "continuo variante" in every second bar
    // of B parts and in three of four in their second turn, and its
    // "fraseo" to close. The piano (Low tom) plays its base: "tapado"
    // strokes, stick and hand together, and loud open strokes just
    // before beats 2 and 4, with accents on beats 3 and 4 as recordings
    // of candombe drumming measure them (Maia, Rocamora, Biscainho and
    // Fuentes, "Adapting meter tracking models to Latin American music",
    // ISMIR 2022), and its "variación" every fourth bar of a first turn,
    // every second bar of a second turn and of B parts, three of four in
    // B's second turn, and in the closings. The chico (Conga) keeps its
    // ostinato, the hand's accent then two stick strokes, and plays
    // every sixteenth ("repicado") in each closing's last bar.
    rhythm(
        "Candombe",
        Folklore,
        110.0,
        &[
            (
                Rim,
                "x..x ..x. ..x. x... | x..x ..x. ..x. x... | \
                 x..x ..x. ..x. x... | x..x ..x. ..x. x... | \
                 .... .... .... .... | .... .... .... .... | \
                 .... .... .... .... | .... .... .... .... | \
                 x..x ..x. ..x. x... | .... .... .... .... | \
                 .... .... .... .... | .... .... .... .... | \
                 .... .... .... .... | .... .... .... .... | \
                 .... .... .... .... | .... .... .... .... | \
                 .... .... .... .... | .... .... .... .... | \
                 .... .... .... .... | .... .... .... .... | \
                 .... .... .... .... | .... .... .... .... | \
                 x..x ..x. ..x. x... | .... .... .... ....",
            ),
            (
                HighTom,
                ".... .... .... .... | .... .... .... .... | \
                 .... .... .... .... | .... .... .... .... | \
                 xx.x xx.x xx.x xx.x | xx.x xx.x xx.x xx.x | \
                 xx.x xx.x xx.x xx.x | xx.x xx.x xx.x xx.x | \
                 .... .... .... .... | xx.x xx.x xx.x xx.x | \
                 xx.x xx.x xx.x xx.x | xx.x xx.x xx.x xx.x | \
                 xxxx xx.x xx.x xx.x | xx.x xx.x xx.x xx.x | \
                 xxxx xx.x xx.x xx.x | xx.x xx.x xx.x xx.x | \
                 xxxx xx.x xx.x xx.x | xxxx xx.x xx.x xx.x | \
                 xx.x xx.x xx.x xx.x | xxxx xx.x xx.x xx.x | \
                 .x.x x... .x.x x... | .x.x xx.x xx.x x... | \
                 .... .... .... .... | .x.x xx.x xx.x x...",
            ),
            (
                LowTom,
                "x..X .x.. X..X X... | x..X .x.. X..X X... | \
                 x..X .x.. X..X X... | x..X .xx. X..X X.x. | \
                 x..X .x.. X..X X... | x..X .x.. X..X X... | \
                 x..X .x.. X..X X... | x..X .xx. X..X X.x. | \
                 x..X .x.. X..X X... | x..X .xx. X..X X.x. | \
                 x..X .x.. X..X X... | x..X .xx. X..X X.x. | \
                 x..X .x.. X..X X... | x..X .xx. X..X X.x. | \
                 x..X .x.. X..X X... | x..X .xx. X..X X.x. | \
                 x..X .xx. X..X X.x. | x..X .x.. X..X X... | \
                 x..X .xx. X..X X.x. | x..X .xx. X..X X.x. | \
                 x..X .x.. X..X X... | x..X .xx. X..X X.x. | \
                 x..X .xx. X..X X.x. | x..X .xx. X..X X.x.",
            ),
            (
                Conga,
                ".Xxx .Xxx .Xxx .Xxx | .Xxx .Xxx .Xxx .Xxx | \
                 .Xxx .Xxx .Xxx .Xxx | .Xxx .Xxx .Xxx .Xxx | \
                 .Xxx .Xxx .Xxx .Xxx | .Xxx .Xxx .Xxx .Xxx | \
                 .Xxx .Xxx .Xxx .Xxx | .Xxx .Xxx .Xxx .Xxx | \
                 .Xxx .Xxx .Xxx .Xxx | .Xxx .Xxx .Xxx .Xxx | \
                 .Xxx .Xxx .Xxx .Xxx | .Xxx .Xxx .Xxx .Xxx | \
                 .Xxx .Xxx .Xxx .Xxx | .Xxx .Xxx .Xxx .Xxx | \
                 .Xxx .Xxx .Xxx .Xxx | .Xxx .Xxx .Xxx .Xxx | \
                 .Xxx .Xxx .Xxx .Xxx | .Xxx .Xxx .Xxx .Xxx | \
                 .Xxx .Xxx .Xxx .Xxx | .Xxx .Xxx .Xxx .Xxx | \
                 .Xxx .Xxx .Xxx .Xxx | xXxx xXxx xXxx xXxx | \
                 .Xxx .Xxx .Xxx .Xxx | xXxx xXxx xXxx xXxx",
            ),
        ],
        "",
        "Montevideo's candombe, in the Cuareim toque, laid out over each part of a song. The \
         song opens with the repique playing the 3-2 clave on the drum's wooden shell for \
         four bars; then it plays its base, and closes every phrase with its two-bar call. \
         The chico keeps its ostinato, the three sixteenths after each beat, the hand's \
         accent on the first. The piano's open strokes ring just before beats 2 and 4 among \
         its muffled ones, which it accents on beats 3 and 4, with a variation every \
         fourth bar. Phrases take turns, so no two in a row sound the same: every second \
         one opens with a bar of clave on the wood and varies the piano every second bar, \
         and every second call starts on the wood too. A part of another kind, a refrain \
         after the verse, mixes the repique's busier base into its own and varies the \
         piano every second bar, more often still on its second turn.",
    )
    .laid_out(CANDOMBE),
    rhythm(
        "Candombe Ansina",
        Folklore,
        110.0,
        &[
            (
                Rim,
                "x..x .x.x x..x x..x | x..x .x.x x..x x..x | \
                 x..x .x.x x..x x..x | x..x .x.x x..x x..x | \
                 .... .... .... .... | .... .... .... .... | \
                 .... .... .... .... | .... .... .... .... | \
                 x..x .x.x x..x x..x | .... .... .... .... | \
                 .... .... .... .... | .... .... .... .... | \
                 .... .... .... .... | .... .... .... .... | \
                 .... .... .... .... | .... .... .... .... | \
                 .... .... .... .... | .... .... .... .... | \
                 .... .... .... .... | .... .... .... .... | \
                 .... .... .... .... | .... .... .... .... | \
                 x..x .x.x x..x x..x | .... .... .... ....",
            ),
            (
                HighTom,
                ".... .... .... .... | .... .... .... .... | \
                 .... .... .... .... | .... .... .... .... | \
                 xx.x xx.x xx.x xx.x | xx.x xx.x xx.x xx.x | \
                 xx.x xx.x xx.x xx.x | xx.x xx.x xx.x xx.x | \
                 .... .... .... .... | xx.x xx.x xx.x xx.x | \
                 xx.x xx.x xx.x xx.x | xx.x xx.x xx.x xx.x | \
                 xxxx xx.x xx.x xx.x | xx.x xx.x xx.x xx.x | \
                 xxxx xx.x xx.x xx.x | xx.x xx.x xx.x xx.x | \
                 xxxx xx.x xx.x xx.x | xxxx xx.x xx.x xx.x | \
                 xx.x xx.x xx.x xx.x | xxxx xx.x xx.x xx.x | \
                 .x.x x... .x.x x... | .x.x xx.x xx.x x... | \
                 .... .... .... .... | .x.x xx.x xx.x x...",
            ),
            (
                LowTom,
                "x..X .x.. XxX. X... | x..X .x.. XxX. X... | \
                 x..X .x.. XxX. X... | x..X .x.. XxXX X... | \
                 x..X .x.. XxX. X... | x..X .x.. XxX. X... | \
                 x..X .x.. XxX. X... | x..X .x.. XxXX X... | \
                 x..X .x.. XxX. X... | x..X .x.. XxXX X... | \
                 x..X .x.. XxX. X... | x..X .x.. XxXX X... | \
                 x..X .x.. XxX. X... | x..X .x.. XxXX X... | \
                 x..X .x.. XxX. X... | x..X .x.. XxXX X... | \
                 x..X .x.. XxXX X... | x..X .x.. XxX. X... | \
                 x..X .x.. XxXX X... | x..X .x.. XxXX X... | \
                 x..X .x.. XxX. X... | x..X .x.. XxXX X... | \
                 x..X .x.. XxXX X... | x..X .x.. XxXX X...",
            ),
            (
                Conga,
                ".Xxx .Xxx .Xxx .Xxx | .Xxx .Xxx .Xxx .Xxx | \
                 .Xxx .Xxx .Xxx .Xxx | .Xxx .Xxx .Xxx .Xxx | \
                 .Xxx .Xxx .Xxx .Xxx | .Xxx .Xxx .Xxx .Xxx | \
                 .Xxx .Xxx .Xxx .Xxx | .Xxx .Xxx .Xxx .Xxx | \
                 .Xxx .Xxx .Xxx .Xxx | .Xxx .Xxx .Xxx .Xxx | \
                 .Xxx .Xxx .Xxx .Xxx | .Xxx .Xxx .Xxx .Xxx | \
                 .Xxx .Xxx .Xxx .Xxx | .Xxx .Xxx .Xxx .Xxx | \
                 .Xxx .Xxx .Xxx .Xxx | .Xxx .Xxx .Xxx .Xxx | \
                 .Xxx .Xxx .Xxx .Xxx | .Xxx .Xxx .Xxx .Xxx | \
                 .Xxx .Xxx .Xxx .Xxx | .Xxx .Xxx .Xxx .Xxx | \
                 .Xxx .Xxx .Xxx .Xxx | xXxx xXxx xXxx xXxx | \
                 .Xxx .Xxx .Xxx .Xxx | xXxx xXxx xXxx xXxx",
            ),
        ],
        "",
        "Montevideo's candombe in the Ansina toque, from the Palermo neighbourhood: the \
         repique opens on the wooden shell with the Ansina madera, eight strokes a bar, and \
         the piano answers its open stroke before beat 2 with three quick strokes around \
         beat 3, the open hand between two open sticks. It is laid out over each part of a \
         song as the Cuareim candombe is.",
    )
    .laid_out(CANDOMBE),
    rhythm(
        "Polca",
        Folklore,
        120.0,
        &[(Kick, "X... .... X... ...."), (Rim, ".... x.o. .... x.o.")],
        "",
        "The polka of Mexico's corridos and norteño bands, and of Rio's polcas: the bass on \
         beat 1, the guitars strumming beat 2 and its \"and\". The polka is in 2/4; each bar \
         here lasts two.",
    ),
    rhythm(
        "Pasillo",
        Folklore,
        190.0,
        &[(Kick, "X... .... ...."), (Rim, ".... ..x. x...")],
        "",
        "The quick waltz of Colombia and Ecuador: the bass on 1, the tiple and guitar on the \
         \"and\" of 2 and on 3, so the second beat comes late.",
    )
    .in_time(THREE_FOUR),
    rhythm(
        "Son jalisciense",
        Folklore,
        180.0,
        &[
            (Kick, "X.....X..... | X...X...X..."),
            (Rim, "..x.x...x.x. | ..x...x...x."),
        ],
        "",
        "The mariachi son of Jalisco, which the jarabe strings together: 6/8 and 3/4 in turn, \
         the guitarrón on the two beats of one bar and the three of the next, the vihuela and \
         guitar strumming between them.",
    )
    .in_time(SIX_EIGHT),
    rhythm(
        "Peruvian waltz",
        Folklore,
        156.0,
        &[(Kick, "X... .... ...."), (Rim, "..x. x.x. x...")],
        "",
        "The vals criollo of Lima, the waltz of Chabuca Granda: quick, with the cajón's bass \
         on beat 1 and its slaps between the beats, under two guitars.",
    )
    .in_time(THREE_FOUR),
    rhythm(
        "Festejo",
        Folklore,
        165.0,
        &[
            (Kick, "X..... X....."),
            (Rim, "...x.x ...x.x"),
            (Cowbell, "x...x. x.x.x."),
        ],
        "",
        "The Afro-Peruvian festejo, fast and syncopated: the cajón and the cowbell in 6/8, \
         while the guitar's chords fall in 3/4, three against two.",
    )
    .in_time(SIX_EIGHT),
    rhythm(
        "Rasguido doble",
        Folklore,
        100.0,
        &[(Kick, "X..x .... X..x ...."), (Rim, ".... x.x. .... x.x.")],
        "",
        "The rasguido doble of Corrientes and Chaco, \"a tango played badly\": a lively dance \
         in two whose guitar strums the habanera's long-short, the bass strings on the beat \
         and the muted treble strings after it. Its time is 2/4; each bar here lasts two.",
    ),
    rhythm(
        "Murga",
        Folklore,
        116.0,
        &[
            (Kick, "X.ox ..o. X.x. ..o."),
            (HiHat, "x... x... x... x..."),
            (OpenHat, "..x. ..x. ..x. ..x."),
            (Snare, "x..o x..o x..o x..o"),
            (Rim, ".ox. .ox. .ox. .ox."),
        ],
        "",
        "The marcha camión of Montevideo's carnival murgas, named after the trucks that \
         carried them through the city, as its percussionists write it: the bombo struck on \
         1, on the sixteenth before 2, on 3 and on its \"and\", the left hand damping the head \
         between; the platillo (a hi-hat) closed on the beats and open on the \"ands\"; and \
         the redoblante four to the beat, a stroke, two stick-on-stick clicks, the second \
         accented, and a buzz.",
    ),
    // ----- Flamenco -----
    rhythm(
        "Rumba flamenca",
        Flamenco,
        105.0,
        &[
            (Kick, "X... ..X. X... ...."),
            (Rim, ".... X... .... X..x"),
            (Clap, "x.x. ..x. x.x. ..x."),
        ],
        "Latin fill",
        "Hand claps (palmas) and the bass and slap of a cajón.",
    ),
    rhythm(
        "Pasodoble",
        Flamenco,
        120.0,
        &[
            (Kick, "X... X..."),
            (Crash, "x... ...."),
            (Snare, "..x. ..x."),
        ],
        "",
        "The pasodoble of bullfights and village bands: a march in 2/4, the bass drum on the \
         beats, a cymbal on the first, and the snare between them.",
    )
    .in_time(TWO_FOUR),
    rhythm(
        "Twelve-beat compás",
        Flamenco,
        110.0,
        &[
            (Clap, "X.o.o. X.o.o. X.o. X.o. X.o."),
            (Kick, "X..... ...... x... .... ...."),
            (Rim, "...... x..... .... x... x..."),
        ],
        "",
        "The twelve beats of soleá, alegrías and bulerías, counted from 12 as flamenco \
         does: 12 1 2, 3 4 5, 6 7, 8 9, 10 11, the palmas loudest on 12, 3, 6, 8 and 10. \
         Written here as 3 + 3 + 2 + 2 + 2 eighths.",
    )
    .in_time(TimeSignature::of_groups(&[3, 3, 2, 2, 2], 8)),
    // ----- Balkan and Turkish -----
    // Dances in bars of short and long beats: a long beat is three
    // eighths, a short one two. The big drum (the tapan, the daouli, the
    // davul) strikes the long beats and the first of the bar; a thin
    // stick on the other side plays the rest.
    rhythm(
        "Paidushko",
        Balkan,
        200.0,
        &[(Kick, "X... x....."), (Rim, "..o. ..x.o.")],
        "",
        "Bulgaria's limping dance in 5/8: a short beat and a long one, 2 + 3, \
         quick and slow, over and over.",
    )
    .in_time(TimeSignature::of_groups(&[2, 3], 8)),
    rhythm(
        "Kalamatianos",
        Balkan,
        180.0,
        &[
            (Kick, "X..... .... ...."),
            (Snare, "...... x... x..."),
            (Rim, "..o.o. ..o. ..o."),
        ],
        "",
        "Greece's most loved dance, in 7/8: a long beat and two short ones, 3 + 2 + 2, \
         slow, quick, quick.",
    )
    .in_time(TimeSignature::of_groups(&[3, 2, 2], 8)),
    rhythm(
        "Lesnoto",
        Balkan,
        140.0,
        &[(Kick, "X..... x... ...."), (Rim, "..o.x. ..o. x.o.")],
        "",
        "The \"easy\" dance of Macedonia, in 7/8 like the kalamatianos (3 + 2 + 2) but \
         slower and heavier, the big drum on the first two beats.",
    )
    .in_time(TimeSignature::of_groups(&[3, 2, 2], 8)),
    rhythm(
        "Rachenitsa",
        Balkan,
        220.0,
        &[(Kick, "X... .... x....."), (Rim, "..o. x.o. ..x.o.")],
        "",
        "Bulgaria's fast dance in 7/8 the other way round: two short beats and a long \
         one, 2 + 2 + 3, quick, quick, slow.",
    )
    .in_time(TimeSignature::of_groups(&[2, 2, 3], 8)),
    rhythm(
        "Daichovo",
        Balkan,
        220.0,
        &[
            (Kick, "X... .... .... x....."),
            (Rim, "..o. x.o. x.o. ..x.o."),
        ],
        "",
        "A Bulgarian dance in 9/8 that is not three times three: four beats, the last \
         one long, 2 + 2 + 2 + 3.",
    )
    .in_time(TimeSignature::of_groups(&[2, 2, 2, 3], 8)),
    rhythm(
        "Karşılama",
        Balkan,
        150.0,
        &[
            (Kick, "X... .... x... ......"),
            (Conga, "l... h... l... h.h..."),
            (Shaker, "x.o. x.o. x.o. x.o.o."),
        ],
        "",
        "The Turkish 9/8 of Thrace, 2 + 2 + 2 + 3, on the darbuka: düm, tek, düm, \
         tek-tek, the low and the high sound of the drum.",
    )
    .in_time(TimeSignature::of_groups(&[2, 2, 2, 3], 8)),
    rhythm(
        "Raks aksağı",
        Balkan,
        200.0,
        &[
            (Kick, "X... ...... X... ...."),
            (Conga, "l... h...h. l... h.h."),
            (Shaker, "x.o. x.o.o. x.o. x.o."),
        ],
        "",
        "The Turkish \"dancing aksak\", 9/8 the other way round: a five and a four, \
         2 + 3 + 2 + 2, the darbuka's low düm on the first and third beats.",
    )
    .in_time(TimeSignature::of_groups(&[2, 3, 2, 2], 8)),
    rhythm(
        "Kopanitsa",
        Balkan,
        220.0,
        &[
            (Kick, "X... .... x..... .... ...."),
            (Rim, "..o. x.o. ..o.o. x.o. x.o."),
        ],
        "",
        "Bulgaria's 11/8, 2 + 2 + 3 + 2 + 2: five beats with the long one in the \
         middle, the dance of Gankino horo.",
    )
    .in_time(TimeSignature::of_groups(&[2, 2, 3, 2, 2], 8)),
    rhythm(
        "Bučimiš",
        Balkan,
        260.0,
        &[
            (Kick, "X... .... .... .... x..... .... ...."),
            (Rim, "..o. x.o. x.o. x.o. ..o.o. x.o. x.o."),
        ],
        "",
        "Fifteen eighths in seven beats, 2 + 2 + 2 + 2 + 3 + 2 + 2: one of the longest \
         bars Bulgarian dancers count, with the long beat near the end.",
    )
    .in_time(TimeSignature::of_groups(&[2, 2, 2, 2, 3, 2, 2], 8)),
    rhythm(
        "Tsakonikos",
        Balkan,
        140.0,
        &[(Kick, "X..... x..."), (Rim, "..o.o. ..o.")],
        "",
        "The dance of Tsakonia, in the Peloponnese, in 5/8 the other way round from the \
         paidushko: a long beat and a short one, 3 + 2, slow, quick, the big drum on both.",
    )
    .in_time(TimeSignature::of_groups(&[3, 2], 8)),
    rhythm(
        "Five-four",
        Balkan,
        192.0,
        &[
            (Kick, "X... .... .... x... ...."),
            (Rim, "..o. x.o. x.o. ..o. x.o."),
        ],
        "",
        "A folk dance in five quarter notes, counted 3 + 2: the big drum on 1 and 4, \
         where each group starts, the stick on the other beats and lightly between them.",
    )
    .in_time(TimeSignature::of_groups(&[3, 2], 4)),
    // The Turkish usuls, on the darbuka: `l` is its low sound, düm, `h`
    // its high one, tek (or te, ke), and the big drum doubles the düms.
    //
    // Aksak: nine beats and six strokes, ending 2 + 1 (Cinuçen
    // Tanrıkorur, "Aksak", TDV İslâm Ansiklopedisi 2, 1989), a sofyan and a
    // türk aksağı, 4 + 5 (İsmail Hakkı Özkan, "Raks aksağı", TDV İslâm
    // Ansiklopedisi 34, 2007): düm . te ke | düm . tek . tek.
    rhythm(
        "Aksak",
        Balkan,
        90.0,
        &[
            (Kick, "X... .... X... ......"),
            (Conga, "l... h.h. l... h...h."),
            (Shaker, "x.o. x.o. x.o. x.o.o."),
        ],
        "",
        "The Turkish aksak, 9/8 counted 2 + 2 + 2 + 3, on the darbuka: düm, te-ke, \
         düm, tek-tek, the low düm on the first and third beats, the long last beat \
         split two and one.",
    )
    .in_time(TimeSignature::of_groups(&[2, 2, 2, 3], 8)),
    // Devr-i hindî: seven beats and five strokes, a semai and a sofyan
    // (İsmail Hakkı Özkan, "Devr-i hindî", TDV İslâm Ansiklopedisi 9,
    // 1994): düm tek tek | düm . | tek . (the semai's three strokes, then
    // the sofyan's düm and tek).
    rhythm(
        "Devr-i hindi",
        Balkan,
        120.0,
        &[
            (Kick, "X..... X... ...."),
            (Conga, "l.h.h. l... h..."),
            (Shaker, "x.o.o. x.o. x.o."),
        ],
        "",
        "The Turkish devr-i hindî, 7/8 counted 3 + 2 + 2: düm, tek, tek on the three \
         eighths of the long beat, then düm and tek on the two short ones.",
    )
    .in_time(TimeSignature::of_groups(&[3, 2, 2], 8)),
    // Curcuna as Turkish folk musicians count it, "Dü-ü-üm Te-ek Dü-üm
    // Te-e-ek", 3 + 2 + 2 + 3 (E. Düzenli and İ. Doğan, "Sözlü Türk Halk
    // Müziği Eserlerinde Ölçü Tespitine Yönelik Bir Analiz Modeli",
    // Eurasian Journal of Music and Dance 17, 2020).
    rhythm(
        "Curcuna",
        Balkan,
        96.0,
        &[
            (Kick, "X..... .... X... ......"),
            (Conga, "l..... h... l... h....."),
            (Shaker, "x.o.o. x.o. x.o. x.o.o."),
        ],
        "",
        "The Turkish curcuna, ten eighths counted 3 + 2 + 2 + 3: düm on the long first \
         beat, tek on the second, düm on the third and tek on the long last one.",
    )
    .in_time(TimeSignature::of_groups(&[3, 2, 2, 3], 8)),
    rhythm(
        "Freylekhs",
        Balkan,
        136.0,
        &[
            (Kick, "X... ..X. .... X..."),
            (Snare, "..x. x... x.x. ..x."),
        ],
        "",
        "The freylekhs, the joyful circle dance of klezmer weddings: the bass drum plays the \
         bulgar's 3 + 3 + 2 in eighths, and the snare fills the eighths between.",
    ),
    // ----- Reggae and ska -----
    rhythm(
        "One drop",
        Reggae,
        76.0,
        &[
            (Kick, ".... .... X... ...."),
            (Rim, ".... .... X... ...."),
            (HiHat, "x.x. x.x. x.x. x.x."),
        ],
        "",
        "Nothing on beat 1: kick and rim fall together on beat 3.",
    ),
    rhythm(
        "Steppers",
        Reggae,
        74.0,
        &[
            (Kick, "X... X... X... X..."),
            (Rim, ".... .... X... ...."),
            (HiHat, "x.x. x.x. x.x. x.x."),
        ],
        "",
        "A kick on every beat, marching forward.",
    ),
    rhythm(
        "Ska",
        Reggae,
        160.0,
        &[
            (Kick, "X... .... X... ...."),
            (Snare, ".... X... .... X..."),
            (HiHat, "..x. ..x. ..x. ..x."),
        ],
        "Snare roll",
        "A fast backbeat with the hi-hat on the offbeats, like the guitar.",
    ),
    rhythm(
        "Rocksteady",
        Reggae,
        80.0,
        &[
            (Kick, "X... .... ..X. ...."),
            (Rim, ".... X... .... X..."),
            (HiHat, "..x. ..x. ..x. ..x."),
        ],
        "",
        "Between ska and reggae, in the mid-1960s: slower, with the hi-hat between the beats.",
    ),
    rhythm(
        "Dancehall",
        Reggae,
        96.0,
        &[
            (Kick, "X..X ..X. X..X ..X."),
            (Snare, ".... X... .... X..."),
            (HiHat, "x.x. x.x. x.x. x.x."),
        ],
        "",
        "Digital Jamaican dance music: the kick in 3 + 3 + 2 under a snare on 2 and 4.",
    ),
    rhythm(
        "Calypso",
        Reggae,
        108.0,
        &[
            (Kick, "X... ..x. X... ..x."),
            (Rim, "..x. x... ..x. x..."),
            (Cowbell, "x.xx x.xx x.xx x.xx"),
        ],
        "Latin fill",
        "Trinidad's calypso: the bass drum in 3 + 3 + 2 eighths, rim clicks after the beats, \
         and the iron, a brake drum struck with a metal rod, in long-short-short.",
    ),
    rhythm(
        "Soca",
        Reggae,
        128.0,
        &[
            (Kick, "X... X... X... X..."),
            (Snare, ".... ..x. .... x..."),
            (HiHat, "..x. ..x. ..x. ..x."),
            (Cowbell, "x.xx x.x. x.xx x.x."),
        ],
        "Snare roll",
        "Soca, the \"soul of calypso\" from 1970s Trinidad: faster, with a kick on every beat \
         and the iron's syncopations, for the carnival's road marches.",
    ),
    // ----- Country and folk -----
    rhythm(
        "Country train",
        Folk,
        112.0,
        &[
            (Kick, "X... .... X... ...."),
            (Snare, "o.oo X.oo o.oo X.oo"),
        ],
        "Snare roll",
        "Brushes on the snare in sixteenths, like a train on the tracks.",
    ),
    rhythm(
        "Folk",
        Folk,
        100.0,
        &[
            (Kick, "X... .... X... ...."),
            (Shaker, "x.x. x.x. x.x. x.x."),
            (Rim, ".... x... .... x..."),
        ],
        "",
        "Light percussion that stays out of the guitar's way.",
    ),
    rhythm(
        "Two-step",
        Folk,
        120.0,
        &[
            (Kick, "X... .... X... ...."),
            (Snare, ".... X... .... X..."),
            (HiHat, "..x. ..x. ..x. ..x."),
        ],
        "Snare roll",
        "Boom-chick: the bass on 1 and 3, the snare on 2 and 4.",
    ),
    rhythm(
        "Waltz",
        Folk,
        150.0,
        &[(Kick, "X... .... ...."), (Snare, ".... o... o...")],
        "",
        "Oom-pah-pah: one strong beat and two light ones.",
    )
    .in_time(THREE_FOUR),
    rhythm(
        "Two-four polka",
        Folk,
        120.0,
        &[(Kick, "X... ...."), (Snare, ".... o...")],
        "",
        "The polka in 2/4, as it is written: oom-pa, one strong beat and one light one. \
         Taking turns with the waltz, it makes the bars of a zwiefacher.",
    )
    .in_time(TWO_FOUR),
    rhythm(
        "Six-eight",
        Folk,
        90.0,
        &[
            (Kick, "X..... x....."),
            (Rim, "...... X....."),
            (HiHat, "x.x.x. x.x.x."),
        ],
        "",
        "Two beats to the bar, each made of three eighth notes.",
    )
    .in_time(SIX_EIGHT),
    rhythm(
        "Bluegrass",
        Folk,
        180.0,
        &[
            (Kick, "X... .... X... ...."),
            (Snare, ".... x... .... x..."),
            (Shaker, "x.x. x.x. x.x. x.x."),
        ],
        "",
        "Fast and light, with the snare soft on 2 and 4 like the mandolin's chop.",
    ),
    rhythm(
        "Tarantella",
        Folk,
        160.0,
        &[
            (Kick, "X..... X....."),
            (Shaker, "x.x.x. x.x.x."),
            (Rim, "..x.x. ..x.x."),
        ],
        "",
        "A whirling southern Italian dance in fast 6/8, with a tambourine.",
    )
    .in_time(SIX_EIGHT),
    rhythm(
        "Western swing",
        Folk,
        176.0,
        &[
            (Snare, "x.x. X.x. x.x. X.x."),
            (Kick, "X... .... X... ...."),
            (HiHat, ".... x... .... x..."),
        ],
        "",
        "Texas dance halls of the 1930s and 1940s, Bob Wills's band: the snare brushed in \
         swung eighths, louder on 2 and 4, the bass drum on 1 and 3.",
    )
    .swung(100),
    rhythm(
        "Honky-tonk shuffle",
        Folk,
        116.0,
        &[
            (Kick, "X... .... X... ...."),
            (Snare, "o.o. X.o. o.o. X.o."),
            (HiHat, "x.x. x.x. x.x. x.x."),
        ],
        "Snare roll",
        "The Texas shuffle of honky-tonk bars: the snare shuffles long-short and cracks on 2 \
         and 4.",
    )
    .swung(100),
    rhythm(
        "Cajun two-step",
        Folk,
        120.0,
        &[
            (Kick, "X... x... X... x..."),
            (Snare, "..o. ..x. ..o. ..x."),
            (HiHat, "x.X. x.X. x.X. x.X."),
        ],
        "",
        "The two-step of Louisiana's Cajun dance halls: the triangle (the \"tit fer\") ringing \
         every eighth, louder off the beat, over the bass drum on the beats. The dance is in \
         2/4; each bar here lasts two.",
    ),
    rhythm(
        "Musette",
        Folk,
        184.0,
        &[
            (Kick, "X... .... ...."),
            (Snare, ".... o... o..."),
            (HiHat, ".... x... x..."),
        ],
        "",
        "The valse musette of the Paris dance halls (the bals musette): a fast waltz felt in \
         one, for accordion, guitar and brushes.",
    )
    .in_time(THREE_FOUR),
    // ----- Classical and film -----
    rhythm(
        "Epic toms",
        Classical,
        90.0,
        &[
            (Kick, "X... .... X... ...."),
            (Snare, ".... .... X... ...."),
            (LowTom, "X.x. ..x. X.x. ..x."),
            (HighTom, "...x .... ...x .x.."),
        ],
        "Tom fill",
        "Big toms in a driving pattern, as in film trailers.",
    ),
    rhythm(
        "March",
        Classical,
        110.0,
        &[
            (Kick, "X... .... X... ...."),
            (Snare, "X.xx X... X.xx X..."),
        ],
        "Snare roll",
        "The military snare: a roll into every other beat.",
    ),
    rhythm(
        "Six-eight march",
        Classical,
        180.0,
        &[(Kick, "X..... x....."), (Snare, "x...x. x.x.x.")],
        "",
        "A march in 6/8, as in Sousa's Washington Post: two beats to the bar, each made of \
         three eighths, the bass drum on both and the snare in their long-short lilt.",
    )
    .in_time(SIX_EIGHT),
    rhythm(
        "Orchestra",
        Classical,
        100.0,
        &[(Kick, "X... .... o... ....")],
        "",
        "The bass drum of an orchestra on the first beat of every bar, and softly on each \
         other beat the bar is felt in, whatever the bar's meter: in 2/4 and 3/4 once a bar, \
         in 9/8 on 1, 4 and 7. It follows music whose meter changes from bar to bar.",
    )
    .following_the_meter(),
];

/// One-bar fills, for the end of a section.
pub static FILLS: &[Rhythm] = &[
    rhythm(
        "Snare roll",
        Pop,
        110.0,
        &[
            (Kick, "X... .... .... ...."),
            (Snare, "..x. x.x. xxxx XXXX"),
        ],
        "",
        "A roll on the snare that gets louder and faster.",
    ),
    rhythm(
        "Tom fill",
        Rock,
        120.0,
        &[
            (Kick, "X... .... .... ...."),
            (Snare, "x.x. x.x. .... ...."),
            (HighTom, ".... .... xxx. ...."),
            (LowTom, ".... .... ...x xXxX"),
        ],
        "",
        "Down the toms, from high to low.",
    ),
    rhythm(
        "House build",
        Electronic,
        124.0,
        &[
            (Kick, "X... X... X... X..."),
            (Snare, "x.x. x.x. xxxx XXXX"),
            (OpenHat, "..x. ..x. .... ...."),
        ],
        "",
        "Snare hits doubling up toward the drop.",
    ),
    rhythm(
        "Latin fill",
        Latin,
        100.0,
        &[
            (HighTom, "x.x. ..x. xxx. ...."),
            (LowTom, ".... .... ...x xxX."),
            (Cowbell, "X... .... .... ...."),
        ],
        "",
        "A timbales run into the next bar.",
    ),
    rhythm(
        "Hip hop fill",
        HipHop,
        90.0,
        &[
            (Kick, "X... ..x. .xx. ...."),
            (Snare, ".... X... ..x. x.xx"),
            (HiHat, "x.x. x.x. .... ...."),
        ],
        "",
        "The beat stutters before the next bar.",
    ),
    rhythm(
        "Jazz fill",
        Jazz,
        140.0,
        &[
            (Ride, "x... x.x. .... ...."),
            (Snare, ".... ..x. x.x. x.X."),
            (Kick, "o... .... ...x ...."),
        ],
        "",
        "The snare talks back to the band over the last two beats, the kick drops a bomb, \
         and the ride comes in again on the next one.",
    ),
    rhythm(
        "Crash ending",
        Pop,
        110.0,
        &[
            (Kick, "X... .... .... ...."),
            (Crash, "X... .... .... ...."),
        ],
        "",
        "One last hit, and the cymbal rings out.",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn named(name: &str) -> &'static Rhythm {
        Rhythm::named(name).unwrap()
    }

    #[test]
    fn a_new_time_signature_keeps_the_kind_of_rhythm() {
        // A tango put in 3/4 by hand plays a tango vals.
        let tango = named("Tango marcato");
        assert_eq!(Rhythm::for_time(THREE_FOUR, Some(tango)).name, "Tango vals");
        // Pop has no 3/4 rhythm: the usual waltz.
        let pop = named("Pop beat");
        assert_eq!(Rhythm::for_time(THREE_FOUR, Some(pop)).name, "Waltz");
        assert_eq!(Rhythm::for_time(SIX_EIGHT, None).name, "Six-eight");
        assert_eq!(
            Rhythm::for_time(TimeSignature::COMMON, None).name,
            "Pop beat"
        );
        // A zamba (6/8) in 3/4 stays Latin American folklore.
        let zamba = named("Zamba");
        assert_eq!(
            Rhythm::for_time(THREE_FOUR, Some(zamba)).genre,
            Genre::Folklore
        );
    }

    #[test]
    fn an_odd_meter_gets_a_rhythm_grouped_its_way() {
        let seven = TimeSignature::new(7, 8);
        // 7/8 is felt as 2+2+3, a rachenitsa...
        let pop = named("Pop beat");
        let rhythm = Rhythm::for_time(seven, Some(pop));
        assert_eq!(rhythm.name, "Rachenitsa");
        assert_eq!(rhythm.time.to_string(), "2+2+3/8");
        // ...unless the song is Balkan already: a kalamatianos stays
        // with its kind, the first of them in 7/8.
        let kalamatianos = named("Kalamatianos");
        assert_eq!(
            Rhythm::for_time(seven, Some(kalamatianos)).name,
            "Kalamatianos"
        );
        // 5/4 swings in a jazz song, and rocks in any other.
        let swing = named("Swing");
        let five = TimeSignature::new(5, 4);
        assert_eq!(Rhythm::for_time(five, Some(swing)).name, "Five-four swing");
        assert_eq!(Rhythm::for_time(five, Some(pop)).name, "Five-four rock");
        assert_eq!(
            Rhythm::for_time(TimeSignature::new(11, 8), None).name,
            "Kopanitsa"
        );
        // 9/8 is three beats of three, not 2+2+2+3: nothing the library
        // has, so a basic beat.
        let nine = TimeSignature::new(9, 8);
        assert_eq!(Rhythm::for_time(nine, Some(pop)).name, "Basic beat");
    }

    #[test]
    fn a_time_signature_with_no_rhythm_gets_a_basic_beat() {
        let thirteen = TimeSignature::new(13, 8);
        let beat = Rhythm::for_time(thirteen, Some(named("Pop beat")));
        assert_eq!(beat.name, "Basic beat");
        assert_eq!(beat.time, thirteen);
        let five_four = TimeSignature::new(5, 4);
        let beat = Rhythm::basic(five_four);
        assert_eq!(beat.time, five_four);
        let line = |beat: &Rhythm, drum: Drum| {
            beat.lines
                .iter()
                .find(|(d, _)| *d == drum)
                .map(|(_, line)| line.to_string())
                .unwrap()
        };
        // Five beats: kick on 1, 3 and 5, snare on 2 and 4.
        assert_eq!(line(beat, Kick), "X.......x.......x...");
        assert_eq!(line(beat, Snare), "....X.......X.......");
        assert_eq!(line(beat, HiHat), "x.o.x.o.x.o.x.o.x.o.");
        // 7/8 is felt as 2+2+3: one strong beat and two light ones.
        let beat = Rhythm::basic(TimeSignature::new(7, 8));
        assert_eq!(line(beat, Kick), "X.............");
        assert_eq!(line(beat, Snare), "....o...o.....");
        assert_eq!(line(beat, HiHat), "x.o.x.o.x.o.o.");
        // Made once, then kept.
        assert!(std::ptr::eq(beat, Rhythm::basic(TimeSignature::new(7, 8))));
        assert!(parse_line(&line(beat, HiHat)).is_some());
    }

    #[test]
    fn every_line_is_whole_bars_of_known_characters() {
        for rhythm in RHYTHMS.iter().chain(FILLS) {
            let bar = rhythm.time.steps_per_bar();
            let mut drums = HashSet::new();
            for (drum, line) in rhythm.lines {
                assert!(drums.insert(*drum), "{}: {drum:?} twice", rhythm.name);
                let bars = parse_line(line)
                    .unwrap_or_else(|| panic!("{}: bad character in {line}", rhythm.name));
                assert_eq!(bars.len(), rhythm.bars(), "{}: {line}", rhythm.name);
                for steps in bars {
                    assert_eq!(steps.len(), bar, "{}: {line}", rhythm.name);
                }
            }
        }
    }

    #[test]
    fn layouts_cover_their_lines_and_listed_fills_exist() {
        for rhythm in RHYTHMS.iter().chain(FILLS) {
            if let Some(layout) = rhythm.layout {
                assert_eq!(layout.bars(), rhythm.bars(), "{}", rhythm.name);
                assert!(!layout.grooves.is_empty(), "{}", rhythm.name);
            }
            for name in rhythm.fills.unwrap_or_default() {
                assert!(
                    FILLS.iter().any(|fill| fill.name == *name),
                    "{}: no fill called {name}",
                    rhythm.name
                );
            }
        }
    }

    #[test]
    fn names_are_unique_and_fills_exist() {
        let mut names = HashSet::new();
        for rhythm in RHYTHMS.iter().chain(FILLS) {
            assert!(names.insert(rhythm.name), "{} twice", rhythm.name);
            assert!(
                rhythm.fill.is_empty() || rhythm.fill().is_some(),
                "{}: no fill called {}",
                rhythm.name,
                rhythm.fill
            );
        }
        assert!(RHYTHMS.len() >= 45, "a large library");
    }

    #[test]
    fn notes_follow_the_grid() {
        let beat = Rhythm::named("Pop beat").unwrap();
        let kick: Vec<usize> = beat
            .notes(Kick, 16, 32)
            .iter()
            .map(|note| note.start)
            .collect();
        assert_eq!(kick, [0, 8, 10, 16, 24, 26]);
        let loud = beat.notes(Snare, 16, 16);
        assert!(loud.iter().all(|note| note.velocity == 120));
        assert!(beat.notes(Conga, 16, 16).is_empty(), "not in this rhythm");
    }

    #[test]
    fn rhythms_keep_their_place_in_other_bars() {
        // A 4/4 beat in 3/4 loses its last beat in each bar.
        let beat = Rhythm::named("Pop beat").unwrap();
        let snare: Vec<usize> = beat
            .notes(Snare, 12, 24)
            .iter()
            .map(|note| note.start)
            .collect();
        assert_eq!(snare, [4, 16]);
        // Two-bar rhythms alternate their bars.
        let trap = Rhythm::named("Trap").unwrap();
        let kick: Vec<usize> = trap
            .notes(Kick, 16, 48)
            .iter()
            .map(|note| note.start)
            .collect();
        assert_eq!(kick, [0, 10, 22, 32, 42]);
    }

    #[test]
    fn a_rhythm_that_follows_the_meter_plays_each_bar_in_its_own() {
        let orchestra = named("Orchestra");
        assert!(orchestra.follows_the_meter);
        let hits = |time: TimeSignature| -> Vec<(usize, u8)> {
            orchestra
                .notes_in(Kick, time, time.steps_per_bar())
                .iter()
                .map(|note| (note.start, note.velocity))
                .collect()
        };
        // In its own 4/4, its line: loud on 1, soft on 3.
        assert_eq!(hits(TimeSignature::COMMON), [(0, 120), (8, 56)]);
        assert_eq!(
            orchestra.notes_in(Kick, TimeSignature::COMMON, 32),
            orchestra.notes(Kick, 16, 32)
        );
        // 2/4 and 3/4 are felt in one, 9/8 in three, 5/8 as 2+3, 7/8 as
        // 2+2+3, 3+2+2/8 with its long beat first.
        assert_eq!(hits(TimeSignature::new(2, 4)), [(0, 120)]);
        assert_eq!(hits(THREE_FOUR), [(0, 120)]);
        assert_eq!(
            hits(TimeSignature::new(9, 8)),
            [(0, 120), (6, 56), (12, 56)]
        );
        assert_eq!(hits(TimeSignature::new(5, 8)), [(0, 120), (4, 56)]);
        assert_eq!(hits(TimeSignature::new(7, 8)), [(0, 120), (4, 56), (8, 56)]);
        let kalamatianos = TimeSignature::of_groups(&[3, 2, 2], 8);
        assert_eq!(hits(kalamatianos), [(0, 120), (6, 56), (10, 56)]);
        // Bar after bar, each in its own place.
        let starts: Vec<usize> = orchestra
            .notes_in(Kick, TimeSignature::new(5, 8), 30)
            .iter()
            .map(|note| note.start)
            .collect();
        assert_eq!(starts, [0, 4, 10, 14, 20, 24]);
        // It has no time signature of its own to put a song in.
        for time in TimeSignature::ALL {
            assert_ne!(Rhythm::for_time(time, Some(orchestra)).name, "Orchestra");
        }
    }

    #[test]
    fn congas_play_high_and_low() {
        let salsa = Rhythm::named("Salsa").unwrap();
        let pitches: Vec<u8> = salsa
            .notes(Conga, 16, 16)
            .iter()
            .map(|note| note.pitch.midi())
            .collect();
        assert_eq!(pitches[..4], [60, 60, 60, 65]);
    }
}
