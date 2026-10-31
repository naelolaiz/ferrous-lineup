//! How the drums play a rhythm over the parts of a song.
//!
//! A drummer does not play one bar over and over from the first bar to
//! the last. They follow the song's form:
//!
//! - **Phrases.** A part is played in phrases of eight bars. A part of
//!   fewer than eight is one phrase; a longer one is cut in eights, and
//!   what is left over is a phrase of its own when it is four bars or
//!   more, or else goes with the last one.
//! - **Fills.** The last bar of a part of two bars or more gets a fill,
//!   and the drums go into the next part on its first beat. Inside a part of sixteen bars
//!   or more, the end of each eight gets a small one, on the last beat
//!   or two. The last part before the ending builds over its last two
//!   bars. The fills take turns, so a song does not hear the same one at
//!   every turn, and a song always plays the same ones in the same
//!   places.
//! - **Crashes.** A part that comes after a fill starts with the crash
//!   cymbal on its first beat, as a drummer lands a fill.
//! - **Grooves.** A part that sounds like an earlier one plays its
//!   groove again; a part of another kind, a B after the A, a refrain
//!   after the verse, plays a contrasting one: the hi-hat moves to the
//!   ride, as variation b of a style plays it.
//!
//! A rhythm can also be written as more than a loop (see [`Layout`]): an
//! opening that starts the song, grooves for each kind of part, and a
//! closing that ends each phrase, which is the rhythm's own fill. A
//! candombe opens with the clave on the drum's shell, grooves with the
//! repique's base, and closes each phrase with the repique's call.
//!
//! The fills of a rhythm are the ones it names, then the usual ones of
//! its genre. A genre played without a drum kit, such as folklore with
//! its bombo, fills with its own drums (see [`own_fill`]); one played
//! with no drums at all, such as the tango, has none.

use std::ops::Range;

use crate::meter::{Bar, TimeSignature};
use crate::pattern::{NoteEvent, STEP_NOTE};
use crate::playlist::Section;
use crate::project::Drum;

use super::Genre;
use super::rhythm::{FILLS, Layout, Rhythm};

/// The bars of a phrase.
const PHRASE: usize = 8;

/// How hard the crash cymbal lands a fill.
const CRASH: u8 = 100;

/// What comes after a part.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Then {
    /// Another part of the song.
    Part,
    /// The ending, or the end of the song.
    Ending,
}

/// A part of a song, as the drums play it.
#[derive(Debug, Clone, Copy)]
pub struct DrumPart<'a> {
    /// Its bars, on the timeline the notes go on: the song's, or a
    /// pattern's.
    pub bars: &'a [Bar],
    /// Whether it opens the song: its first part, or its intro.
    pub opens: bool,
    /// Which kind of part it is: 0 for the first kind (A), 1 for the
    /// next (B), and so on. A part like an earlier one has its number.
    pub groove: usize,
    /// Whether the drums play a groove of another kind in it, as a
    /// rhythm with one groove does in a B part (see [`contrasting`]).
    /// Off where the song says itself which variation plays where.
    pub contrast: bool,
    /// What comes after it.
    pub then: Then,
    /// Whether it is the lead-in to the song's first part: a pickup the
    /// tune starts in. It plays the end of a phrase into the first part.
    pub lead_in: bool,
    /// Whether it starts with the crash cymbal on its first beat: it comes
    /// after a part that ends with a fill (see [`ends_with_fill`]).
    pub crash: bool,
    /// How many phrases of its kind the song played before it: a rhythm
    /// whose phrases take turns with their grooves and closings (see
    /// [`Layout`]) goes on from there.
    pub turn: usize,
}

/// How big a fill is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    /// The last beat or two of a bar (see [`small_from`]), the groove
    /// playing before it.
    Small,
    /// A whole bar.
    Whole,
}

/// One of the fills a rhythm plays.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Fill {
    /// A fill of the library's [`FILLS`].
    Library(&'static Rhythm),
    /// One made from the rhythm's own drums (see [`own_fill`]).
    Own,
}

/// The usual fills of `genre`, for a rhythm that names no others:
/// `None` for genres a drummer fills with the rhythm's own drums.
fn genre_fills(genre: Genre) -> Option<&'static [&'static str]> {
    Some(match genre {
        Genre::Pop => &["Snare roll", "Tom fill"],
        Genre::Rock => &["Tom fill", "Snare roll"],
        Genre::Blues => &["Snare roll", "Tom fill"],
        Genre::Jazz => &["Jazz fill", "Snare roll"],
        Genre::Soul => &["Snare roll", "Hip hop fill"],
        Genre::HipHop => &["Hip hop fill"],
        Genre::Electronic => &["House build", "Snare roll"],
        Genre::Latin => &["Latin fill"],
        Genre::Reggae => &["Tom fill", "Snare roll"],
        // A tango orchestra and an orchestra play no drum fills.
        Genre::Tango | Genre::Classical => &[],
        Genre::Folklore | Genre::Balkan | Genre::Flamenco | Genre::Folk => return None,
    })
}

/// The fills `rhythm` plays, in turn: the one it names, then the ones it
/// lists, or else the usual ones of its genre. A rhythm of a genre played
/// on its own drums fills with them too.
pub fn fills_of(rhythm: &Rhythm) -> Vec<Fill> {
    let mut fills: Vec<Fill> = Vec::new();
    let mut add = |name: &str| {
        if let Some(fill) = FILLS.iter().find(|fill| fill.name == name)
            && !fills.contains(&Fill::Library(fill))
        {
            fills.push(Fill::Library(fill));
        }
    };
    add(rhythm.fill);
    let listed = rhythm.fills.or_else(|| genre_fills(rhythm.genre));
    for name in listed.unwrap_or_default() {
        add(name);
    }
    let own = rhythm.fills.is_none()
        && genre_fills(rhythm.genre).is_none()
        && own_drums(rhythm).is_some();
    if own {
        fills.push(Fill::Own);
    }
    fills
}

/// The drums `rhythm` plays over any part of a song: its own, the ones
/// it plays them on in a contrasting part, and its fills'.
pub fn drums_used(rhythm: &Rhythm) -> Vec<Drum> {
    let mut drums: Vec<Drum> = Vec::new();
    let own = rhythm.drums();
    let contrast = rhythm.drums().map(|drum| contrasting(rhythm, drum));
    let fills = fills_of(rhythm).into_iter().flat_map(|fill| match fill {
        Fill::Library(fill) => fill.drums().chain([Drum::Crash]).collect::<Vec<_>>(),
        Fill::Own => Vec::new(),
    });
    for drum in own.chain(contrast).chain(fills) {
        if !drums.contains(&drum) {
            drums.push(drum);
        }
    }
    drums
}

/// The drum that plays `drum`'s line in a part of another kind, for a
/// rhythm with one groove: the ride for the hi-hat, and for pop and rock
/// the snare for the rim, as variation b of a style plays them.
pub fn contrasting(rhythm: &Rhythm, drum: Drum) -> Drum {
    let has = |wanted: Drum| rhythm.drums().any(|drum| drum == wanted);
    match drum {
        Drum::HiHat if !has(Drum::Ride) => Drum::Ride,
        Drum::Rim
            if matches!(rhythm.genre, Genre::Pop | Genre::Rock)
                && !has(Drum::Snare)
                && !has(Drum::Clap) =>
        {
            Drum::Snare
        }
        _ => drum,
    }
}

/// How a part of `bars` bars is cut into phrases: their first bars and
/// lengths.
pub fn phrases(bars: usize) -> Vec<(usize, usize)> {
    if bars < PHRASE {
        return vec![(0, bars)];
    }
    let mut lengths = vec![PHRASE; bars / PHRASE];
    let left = bars % PHRASE;
    if left >= PHRASE / 2 {
        lengths.push(left);
    } else if let Some(last) = lengths.last_mut() {
        *last += left;
    }
    let mut at = 0;
    lengths
        .into_iter()
        .map(|length| {
            let phrase = (at, length);
            at += length;
            phrase
        })
        .collect()
}

/// Which bar of its lines `rhythm` plays in each bar of `part`.
fn line_bars(rhythm: &Rhythm, part: &DrumPart) -> Vec<usize> {
    let bars = part.bars.len();
    let Some(layout) = rhythm.layout else {
        return (0..bars).map(|bar| bar % rhythm.bars().max(1)).collect();
    };
    if part.lead_in {
        return laid_out(&layout, bars, 0, 0, 0);
    }
    let mut lines = Vec::with_capacity(bars);
    for (index, (_, length)) in phrases(bars).into_iter().enumerate() {
        let opening = if part.opens && index == 0 {
            layout.opening
        } else {
            0
        };
        let turn = part.turn + index;
        lines.extend(laid_out(&layout, length, opening, part.groove, turn));
    }
    lines
}

/// The bars of `layout`'s lines a phrase of `length` bars plays, opening
/// with `opening` bars of the opening and grooving with the groove of
/// kind `groove` and turn `turn`: the opening, the groove as long as
/// there is room, and the turn's closing. A phrase too short for them all
/// plays the opening's start and the closing, or the closing's end.
fn laid_out(
    layout: &Layout,
    length: usize,
    opening: usize,
    groove: usize,
    turn: usize,
) -> Vec<usize> {
    let closing = layout.closing;
    let close = layout.closing_at(turn);
    let (from, groove_bars) = layout.groove(groove, turn);
    (0..length)
        .map(|bar| {
            if length <= closing {
                close + closing - length + bar
            } else if bar >= length - closing {
                close + bar - (length - closing)
            } else if bar < opening {
                bar
            } else {
                from + (bar - opening) % groove_bars
            }
        })
        .collect()
}

/// Where the fills go in `part`: each one's bar and size. A fill that
/// builds over two bars is two, a small one and then a whole one.
fn fill_places(rhythm: &Rhythm, part: &DrumPart) -> Vec<(usize, Size)> {
    let bars = part.bars.len();
    if rhythm.layout.is_some() || part.lead_in || bars == 0 || fills_of(rhythm).is_empty() {
        return Vec::new();
    }
    let mut places = Vec::new();
    if bars >= 2 * PHRASE {
        let inner = (PHRASE..bars)
            .step_by(PHRASE)
            .filter(|&end| end + PHRASE / 2 <= bars);
        places.extend(inner.map(|end| (end - 1, Size::Small)));
    }
    let ends = part.then == Then::Ending;
    if bars >= 2 || ends {
        if ends && bars >= 4 {
            places.push((bars - 2, Size::Small));
        }
        places.push((bars - 1, Size::Whole));
    }
    places
}

/// Where a small fill starts in a bar of `time`: on its last felt beat,
/// the second half of a 4/4 bar, the second three eighths of 6/8; or on
/// the last beat of a bar felt as one, such as 3/4.
pub fn small_from(time: TimeSignature) -> usize {
    let unit = time.unit_steps();
    let groups = time.felt_groups();
    let bar = time.steps_per_bar();
    match groups.split_last() {
        Some((last, rest)) if !rest.is_empty() => bar - usize::from(*last) * unit,
        _ => bar.saturating_sub(unit),
    }
}

/// Where a fill on a rhythm's own drums that lasts a whole bar starts:
/// on its last two felt beats, the whole of a 4/4 or a 6/8 bar; or two
/// beats from the end of a bar felt as one, such as 3/4.
fn whole_from(time: TimeSignature) -> usize {
    let unit = time.unit_steps();
    let groups = time.felt_groups();
    let bar = time.steps_per_bar();
    let last_two: usize = groups
        .iter()
        .rev()
        .take(2)
        .map(|&group| usize::from(group))
        .sum();
    if groups.len() >= 2 {
        bar.saturating_sub(last_two * unit)
    } else {
        bar.saturating_sub(2 * unit)
    }
}

/// The drums a rhythm fills with on its own: its deepest drum, and one
/// to answer it, as the bombo's head and its wooden rim. `None` for a
/// rhythm with no drum to fill on.
fn own_drums(rhythm: &Rhythm) -> Option<(Drum, Option<Drum>)> {
    let has = |drum: &Drum| rhythm.drums().any(|own| own == *drum);
    let deep = [
        Drum::LowTom,
        Drum::Conga,
        Drum::HighTom,
        Drum::Snare,
        Drum::Kick,
    ]
    .into_iter()
    .find(has)?;
    let answer = [
        Drum::Rim,
        Drum::HighTom,
        Drum::Clap,
        Drum::Cowbell,
        Drum::Snare,
        Drum::Kick,
    ]
    .into_iter()
    .filter(|drum| *drum != deep)
    .find(has);
    Some((deep, answer))
}

/// A fill on `rhythm`'s own drums from step `from` of a bar of `time`
/// lasting `length` steps: a stroke on every eighth note, the two drums
/// taking turns, growing louder to the last, the deep one.
fn own_fill(rhythm: &Rhythm, from: usize, length: usize) -> Vec<(Drum, NoteEvent)> {
    let Some((deep, answer)) = own_drums(rhythm) else {
        return Vec::new();
    };
    let strokes: Vec<usize> = (from..length).step_by(2).collect();
    let count = strokes.len();
    strokes
        .iter()
        .enumerate()
        .map(|(index, &at)| {
            let from_last = count - 1 - index;
            let drum = if from_last % 2 == 1 {
                answer.unwrap_or(deep)
            } else {
                deep
            };
            let rise = if count > 1 {
                48 * index / (count - 1)
            } else {
                48
            };
            let velocity = u8::try_from(72 + rise).unwrap_or(120);
            (
                drum,
                NoteEvent {
                    velocity,
                    ..NoteEvent::new(crate::pattern::STEP_NOTE, at, 1)
                },
            )
        })
        .collect()
}

/// The notes of `fill` in a bar of `time` lasting `length` steps, from
/// step `from` on.
fn fill_notes(
    rhythm: &Rhythm,
    fill: Fill,
    time: TimeSignature,
    length: usize,
    from: usize,
) -> Vec<(Drum, NoteEvent)> {
    match fill {
        Fill::Library(fill) => fill
            .drums()
            .flat_map(|drum| {
                fill.notes(drum, time.steps_per_bar(), length)
                    .into_iter()
                    .filter(|note| note.start >= from)
                    .map(move |note| (drum, note))
            })
            .collect(),
        Fill::Own => own_fill(rhythm, from, length),
    }
}

/// Whether the drums end `part` with a fill on the drum kit, which the
/// next part lands with a crash.
pub fn ends_with_fill(rhythm: &Rhythm, part: &DrumPart) -> bool {
    let bars = part.bars.len();
    bars > 0
        && fill_places(rhythm, part).contains(&(bars - 1, Size::Whole))
        && fills_of(rhythm)
            .iter()
            .any(|fill| matches!(fill, Fill::Library(_)))
}

/// What the drums play over `part`, each note with its drum, on the
/// part's timeline. `fills` counts the fills played so far in the song,
/// so that they take turns.
pub fn part_drums(rhythm: &Rhythm, part: &DrumPart, fills: &mut usize) -> Vec<(Drum, NoteEvent)> {
    let lines = line_bars(rhythm, part);
    let places = fill_places(rhythm, part);
    let choices = fills_of(rhythm);
    let contrast = part.contrast && rhythm.layout.is_none() && contrasts(part.groove);
    let mut notes = Vec::new();
    // A fill that builds over two bars is one fill: its small part
    // and its whole one are the same.
    let mut chosen: Option<Fill> = None;
    for (index, bar) in part.bars.iter().enumerate() {
        let place = places
            .iter()
            .find(|(at, _)| *at == index)
            .map(|&(_, size)| size);
        let fill = place.and_then(|size| {
            let builds =
                size == Size::Whole && index > 0 && places.contains(&(index - 1, Size::Small));
            let fill = match chosen.filter(|_| builds) {
                Some(fill) => fill,
                None => {
                    let fill = choices.get(*fills % choices.len().max(1)).copied()?;
                    if !(size == Size::Small && places.contains(&(index + 1, Size::Whole))) {
                        *fills += 1;
                    }
                    fill
                }
            };
            chosen = Some(fill);
            Some((size, fill))
        });
        let from = match fill {
            Some((Size::Small, _)) => small_from(bar.time).min(bar.length),
            Some((Size::Whole, Fill::Own)) => whole_from(bar.time).min(bar.length),
            Some((Size::Whole, Fill::Library(_))) => 0,
            None => bar.length,
        };
        for drum in rhythm.drums() {
            let played = if contrast {
                contrasting(rhythm, drum)
            } else {
                drum
            };
            for note in rhythm.bar_notes(drum, lines[index], bar.time, bar.length) {
                if note.start < from {
                    notes.push((
                        played,
                        NoteEvent {
                            start: bar.start + note.start,
                            ..note
                        },
                    ));
                }
            }
        }
        if let Some((_, fill)) = fill {
            for (drum, note) in fill_notes(rhythm, fill, bar.time, bar.length, from) {
                notes.push((
                    drum,
                    NoteEvent {
                        start: bar.start + note.start,
                        ..note
                    },
                ));
            }
        }
    }
    if part.crash
        && let Some(first) = part.bars.first()
        && !notes
            .iter()
            .any(|(drum, note)| *drum == Drum::Crash && note.start == first.start)
    {
        notes.push((
            Drum::Crash,
            NoteEvent {
                velocity: CRASH,
                ..NoteEvent::new(STEP_NOTE, first.start, 1)
            },
        ));
    }
    notes
}

/// How many phrases of each kind of part a song has played so far, for
/// the parts' turns (see [`DrumPart::turn`]).
#[derive(Debug, Clone, Default)]
pub struct Turns(Vec<usize>);

impl Turns {
    /// The turn the next part of `kind` takes.
    pub fn of(&self, kind: &PartKind) -> usize {
        self.0.get(kind.groove).copied().unwrap_or(0)
    }

    /// Counts the phrases of a part of `kind`, `bars` long.
    pub fn played(&mut self, kind: &PartKind, bars: usize) {
        if kind.lead_in {
            return;
        }
        if self.0.len() <= kind.groove {
            self.0.resize(kind.groove + 1, 0);
        }
        self.0[kind.groove] += phrases(bars).len();
    }
}

/// What kind of part of the song the drums play (see [`form_parts`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartKind {
    /// Its section's kind: the same number for sections of the same name.
    pub groove: usize,
    /// Whether it opens the song.
    pub opens: bool,
    /// Whether it is a pickup before the song's first section.
    pub lead_in: bool,
}

/// The kind of each of `form`'s sections, as the drums play them: the
/// same number for sections of the same name ("Verse 2" is a verse), in
/// the order they first come, an intro and a coda as the first kind; and
/// whether it is an intro.
pub fn section_kinds(form: &[Section]) -> Vec<(usize, bool)> {
    name_kinds(form.iter().map(|section| section.name.as_str()))
}

/// The kinds of sections called `names`, as [`section_kinds`] gives
/// them.
pub fn name_kinds<'a>(names: impl IntoIterator<Item = &'a str>) -> Vec<(usize, bool)> {
    let mut known: Vec<String> = Vec::new();
    names
        .into_iter()
        .map(|name| {
            let name = name
                .trim()
                .trim_end_matches(|c: char| c.is_ascii_digit() || c.is_whitespace())
                .to_lowercase();
            let intro = name.starts_with("intro");
            let first = intro
                || ["coda", "outro", "ending", "final"]
                    .iter()
                    .any(|word| name.starts_with(word));
            if first {
                return (0, intro);
            }
            let kind = known
                .iter()
                .position(|known| *known == name)
                .unwrap_or_else(|| {
                    known.push(name);
                    known.len() - 1
                });
            (kind, false)
        })
        .collect()
}

/// Whether a part of kind `kind` (see [`section_kinds`]) is of another
/// kind than the song's first, such as a refrain after the verse: the
/// second kind, the fourth and so on, so that kinds take turns as the
/// drums' grooves do. The band plays such a part otherwise where its
/// style does (see [`crate::library::Comping::in_contrast`]).
pub fn contrasts(kind: usize) -> bool {
    kind % 2 == 1
}

/// Whether step `at` is in a part of another kind (see [`contrasts`]),
/// among `form`'s sections.
pub fn contrasts_at(form: &[Section], at: usize) -> bool {
    let kinds = section_kinds(form);
    form.iter()
        .position(|section| section.contains(at))
        .is_some_and(|section| contrasts(kinds[section].0))
}

/// The parts of a song the drums play, as runs of `bars` up to step
/// `end`: cut where `form`'s sections start and end, and at the steps in
/// `cuts`. Each has what kind of part it is, by its section's name. The
/// first part opens the song, and so do the parts before step
/// `intro_end`; a bar or two before the first section is a pickup the
/// drums lead in with.
pub fn form_parts(
    bars: &[Bar],
    form: &[Section],
    end: usize,
    cuts: &[usize],
    intro_end: usize,
) -> Vec<(Range<usize>, PartKind)> {
    let bars_to_end = bars.partition_point(|bar| bar.start < end);
    if bars_to_end == 0 {
        return Vec::new();
    }
    let bar_of = |step: usize| {
        bars.partition_point(|bar| bar.start <= step)
            .saturating_sub(1)
    };
    let mut starts: Vec<usize> = form
        .iter()
        .flat_map(|section| [section.start, section.end()])
        .chain(cuts.iter().copied())
        .filter(|&start| start > 0 && start < end)
        .map(bar_of)
        .collect();
    starts.push(0);
    starts.sort_unstable();
    starts.dedup();
    let kinds = section_kinds(form);
    let first_section = form.first().map_or(0, |section| bar_of(section.start));
    let mut parts: Vec<(Range<usize>, PartKind)> = Vec::new();
    for (index, &from) in starts.iter().enumerate() {
        let to = starts.get(index + 1).copied().unwrap_or(bars_to_end);
        if from >= to {
            continue;
        }
        let at = bars[from].start;
        let section = form.iter().position(|section| section.contains(at));
        let (groove, named_intro) = section.map_or((0, false), |section| kinds[section]);
        let lead_in = section.is_none() && to <= first_section && first_section - from <= 2;
        let opens = !lead_in
            && (named_intro || at < intro_end || parts.iter().all(|(_, kind)| kind.lead_in));
        parts.push((
            from..to,
            PartKind {
                groove,
                opens,
                lead_in,
            },
        ));
    }
    parts
}

/// What the drums play over a whole song of `bars`, up to step `end`:
/// `rhythm` laid out over the parts of `form` (see [`form_parts`]), the
/// last one going into the end. Each note has its drum, on the song's
/// timeline.
pub fn song_drums(
    rhythm: &Rhythm,
    bars: &[Bar],
    form: &[Section],
    end: usize,
) -> Vec<(Drum, NoteEvent)> {
    let parts = form_parts(bars, form, end, &[], 0);
    let last = parts.len().saturating_sub(1);
    let mut fills = 0;
    let mut crash = false;
    let mut turns = Turns::default();
    let mut notes = Vec::new();
    for (index, (range, kind)) in parts.into_iter().enumerate() {
        let part = DrumPart {
            bars: &bars[range],
            opens: kind.opens,
            groove: kind.groove,
            contrast: true,
            then: if index == last {
                Then::Ending
            } else {
                Then::Part
            },
            lead_in: kind.lead_in,
            crash,
            turn: turns.of(&kind),
        };
        notes.extend(part_drums(rhythm, &part, &mut fills));
        crash = ends_with_fill(rhythm, &part);
        turns.played(&kind, part.bars.len());
    }
    notes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meter::Meter;

    fn bars(count: usize, time: TimeSignature) -> Vec<Bar> {
        Meter::new(time, []).bars().take(count).collect()
    }

    fn named(name: &str) -> &'static Rhythm {
        Rhythm::named(name).unwrap()
    }

    fn part(bars: &[Bar]) -> DrumPart<'_> {
        DrumPart {
            bars,
            opens: false,
            groove: 0,
            contrast: true,
            then: Then::Part,
            lead_in: false,
            crash: false,
            turn: 0,
        }
    }

    #[test]
    fn parts_are_cut_in_phrases_of_eight() {
        assert_eq!(phrases(4), vec![(0, 4)]);
        assert_eq!(phrases(7), vec![(0, 7)]);
        assert_eq!(phrases(8), vec![(0, 8)]);
        assert_eq!(phrases(10), vec![(0, 10)]);
        assert_eq!(phrases(12), vec![(0, 8), (8, 4)]);
        assert_eq!(phrases(16), vec![(0, 8), (8, 8)]);
        assert_eq!(phrases(19), vec![(0, 8), (8, 11)]);
    }

    #[test]
    fn a_part_ends_with_a_fill_and_long_parts_fill_inside() {
        let rhythm = named("Rock beat");
        let bars = bars(16, TimeSignature::COMMON);
        let places = fill_places(rhythm, &part(&bars));
        assert_eq!(places, vec![(7, Size::Small), (15, Size::Whole)]);
        let short = fill_places(rhythm, &part(&bars[..8]));
        assert_eq!(short, vec![(7, Size::Whole)]);
        let ending = DrumPart {
            then: Then::Ending,
            ..part(&bars[..8])
        };
        assert_eq!(
            fill_places(rhythm, &ending),
            vec![(6, Size::Small), (7, Size::Whole)]
        );
    }

    #[test]
    fn the_fills_take_turns() {
        let rhythm = named("Rock beat");
        assert_eq!(
            fills_of(rhythm),
            vec![
                Fill::Library(named("Tom fill")),
                Fill::Library(named("Snare roll"))
            ]
        );
        let bars = bars(8, TimeSignature::COMMON);
        let mut fills = 0;
        let first = part_drums(rhythm, &part(&bars), &mut fills);
        let second = part_drums(rhythm, &part(&bars), &mut fills);
        assert_eq!(fills, 2);
        assert_ne!(first, second);
        let last_bar = |notes: &[(Drum, NoteEvent)]| {
            notes
                .iter()
                .filter(|(_, note)| note.start >= 7 * 16)
                .map(|(drum, _)| *drum)
                .collect::<Vec<_>>()
        };
        assert!(
            last_bar(&first).contains(&Drum::LowTom),
            "the tom fill first"
        );
        assert!(
            !last_bar(&second).contains(&Drum::LowTom),
            "then the snare roll"
        );
    }

    #[test]
    fn a_b_part_moves_the_hi_hat_to_the_ride() {
        let rhythm = named("Pop beat");
        let bars = bars(4, TimeSignature::COMMON);
        let b = DrumPart {
            groove: 1,
            ..part(&bars)
        };
        let notes = part_drums(rhythm, &b, &mut 0);
        assert!(notes.iter().any(|(drum, _)| *drum == Drum::Ride));
        assert!(!notes.iter().any(|(drum, _)| *drum == Drum::HiHat));
        let a = part_drums(rhythm, &part(&bars), &mut 0);
        assert!(!a.iter().any(|(drum, _)| *drum == Drum::Ride));
    }

    #[test]
    fn folklore_fills_on_its_own_drums_and_the_tango_not_at_all() {
        let zamba = named("Zamba");
        assert_eq!(fills_of(zamba), vec![Fill::Own]);
        let time = zamba.time;
        let bars = bars(4, time);
        let notes = part_drums(zamba, &part(&bars), &mut 0);
        let last: Vec<&(Drum, NoteEvent)> = notes
            .iter()
            .filter(|(_, note)| note.start >= 3 * time.steps_per_bar())
            .collect();
        assert!(
            last.iter()
                .all(|(drum, _)| zamba.drums().any(|own| own == *drum))
        );
        assert_eq!(last.last().map(|(drum, _)| *drum), Some(Drum::LowTom));
        assert!(fills_of(named("Tango marcato")).is_empty());
    }

    #[test]
    fn the_part_after_a_fill_lands_with_a_crash() {
        let rhythm = named("Pop beat");
        let bars = bars(16, TimeSignature::COMMON);
        let crashes = |notes: &[(Drum, NoteEvent)], step: usize| {
            notes
                .iter()
                .filter(|(drum, note)| *drum == Drum::Crash && note.start == step)
                .count()
        };
        let landing = DrumPart {
            crash: true,
            ..part(&bars[8..])
        };
        let landed = part_drums(rhythm, &landing, &mut 0);
        assert_eq!(crashes(&landed, 8 * 16), 1);
        let plain = part_drums(rhythm, &part(&bars[8..]), &mut 0);
        assert_eq!(crashes(&plain, 8 * 16), 0);
        // Over a song: the verse ends with a fill, so the chorus lands
        // with a crash; a tango has no fills, and no crash.
        let form = [
            Section::new("Verse", 0, 8 * 16),
            Section::new("Chorus", 8 * 16, 8 * 16),
        ];
        assert!(ends_with_fill(rhythm, &part(&bars[..8])));
        let song = song_drums(rhythm, &bars, &form, 16 * 16);
        assert_eq!(crashes(&song, 8 * 16), 1);
        assert_eq!(crashes(&song, 0), 0);
        let tango = named("Tango marcato");
        let bars = self::bars(16, tango.time);
        let at = bars[8].start;
        let form = [
            Section::new("Verse", 0, at),
            Section::new("Chorus", at, bars[15].end() - at),
        ];
        assert!(!ends_with_fill(tango, &part(&bars[..8])));
        let song = song_drums(tango, &bars, &form, bars[15].end());
        assert_eq!(crashes(&song, at), 0);
    }

    #[test]
    fn the_candombe_phrases_take_turns() {
        for name in ["Candombe", "Candombe Ansina"] {
            let rhythm = named(name);
            let layout = rhythm.layout.unwrap();
            assert_eq!(rhythm.bars(), layout.bars(), "{name}");
            // An A part of two phrases: the groove, the second phrase's
            // groove, and the two closings in turn.
            let bars = bars(16, TimeSignature::COMMON);
            let lines = line_bars(rhythm, &part(&bars));
            assert_eq!(lines[..8], [4, 5, 6, 7, 4, 5, 20, 21], "{name}");
            assert_eq!(lines[8..], [8, 9, 10, 11, 8, 9, 22, 23], "{name}");
            // A B part's two phrases have grooves of their own.
            let b = DrumPart {
                groove: 1,
                ..part(&bars)
            };
            let lines = line_bars(rhythm, &b);
            assert_eq!(lines[..4], [12, 13, 14, 15], "{name}");
            assert_eq!(lines[8..12], [16, 17, 18, 19], "{name}");
            // A part that comes back carries on the turns: its first
            // phrase is the second turn.
            let again = DrumPart {
                turn: 1,
                ..part(&bars[..8])
            };
            assert_eq!(line_bars(rhythm, &again)[0], 8, "{name}");
            // And no two turns sound the same.
            let first = part_drums(rhythm, &part(&bars[..8]), &mut 0);
            let second = part_drums(rhythm, &again, &mut 0);
            assert_ne!(first, second, "{name}");
            // The second turn opens with the clave on the wood.
            assert!(
                second
                    .iter()
                    .any(|(drum, note)| *drum == Drum::Rim && note.start < 16),
                "{name}"
            );
            assert!(
                !first
                    .iter()
                    .any(|(drum, note)| *drum == Drum::Rim && note.start < 16),
                "{name}"
            );
            // Never the clave just before a new part: both calls close
            // on the drum.
            for notes in [&first, &second] {
                assert!(
                    !notes
                        .iter()
                        .any(|(drum, note)| *drum == Drum::Rim && note.start >= 6 * 16),
                    "{name}"
                );
            }
        }
    }

    #[test]
    fn a_small_fill_starts_on_the_last_felt_beat() {
        assert_eq!(small_from(TimeSignature::COMMON), 8);
        assert_eq!(small_from(TimeSignature::new(3, 4)), 8);
        assert_eq!(small_from(TimeSignature::new(6, 8)), 6);
        assert_eq!(small_from(TimeSignature::new(2, 4)), 4);
    }
}
