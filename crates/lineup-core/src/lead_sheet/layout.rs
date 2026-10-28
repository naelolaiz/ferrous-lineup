//! Where everything goes: the symbols, lines and words of each bar, bars
//! on lines of music ("systems"), and lines on pages.
//!
//! Inside a bar, notes get room by how long they last, but not in
//! proportion: a half note gets less than twice the room of a quarter
//! note (the room grows with the square root of the length), which is
//! how engravers space music so that both quick and slow notes read
//! well. A line holds up to four bars, a new section starts a new line,
//! and each line is then stretched to the width of the page.
//!
//! Positions here are in points. Inside a line, x counts from the start
//! of the line and y from the staff's top line, going down; the page
//! layout moves each line to its place.

use std::ops::Range;

use super::glyphs::{self, Glyph, engraving};
use super::rhythm::{Base, Value};
use super::score::{BarScore, Content, Piece, Score};
use super::spelling::Clef;
use super::transpose::Transposition;
use super::{BarBox, LeadSheet, Mark, Options, Page, Point, TextFont, TextWidth};
use crate::chord_sheet::HarmonyName;
use crate::meter::TimeSignature;
use crate::pitch::NoteName;

/// Points per staff space. A staff is four spaces, 21.2 points (7.5 mm)
/// high, a usual size for a lead sheet.
pub const SPACE: f32 = 5.3;
/// The music font's size: one em is the height of the staff.
const MUSIC_SIZE: f32 = 4.0 * SPACE;
/// The page's margins, 18 mm.
const MARGIN: f32 = 51.0;
const MAX_BARS: usize = 4;
/// The least a line is squeezed to fit one more bar.
const MIN_STRETCH: f32 = 0.7;
/// The size of a chord symbol's letters.
const CHORD_SIZE: f32 = 3.2 * SPACE;
/// How tall the text font's capital letters are, as a part of its size.
const CAP_HEIGHT: f32 = 0.727;
const TITLE_SIZE: f32 = 20.0;
/// The size of the words that say who reads a part: "B♭ instruments".
const PART_SIZE: f32 = 12.0;
const SMALL_SIZE: f32 = 9.0;
const REHEARSAL_SIZE: f32 = 13.0;
const TEMPO_SIZE: f32 = 11.0;
/// The room left between two lines of music.
const SYSTEM_GAP: f32 = 3.0 * SPACE;

fn sp(spaces: f32) -> f32 {
    spaces * SPACE
}

fn width_of(glyph: Glyph) -> f32 {
    sp(glyph.width)
}

/// The y of a line or space ([`super::spelling::staff_step`]), from the
/// staff's top line.
fn y_of(clef: Clef, step: i32) -> f32 {
    (clef.bottom_line() + 8 - step) as f32 * SPACE / 2.0
}

fn music(glyph: Glyph, x: f32, y: f32) -> Mark {
    Mark::Music {
        glyph,
        at: Point::new(x, y),
        size: MUSIC_SIZE,
    }
}

fn line(x1: f32, y1: f32, x2: f32, y2: f32, width: f32) -> Mark {
    Mark::Line {
        from: Point::new(x1, y1),
        to: Point::new(x2, y2),
        width,
    }
}

fn text(text: impl Into<String>, font: TextFont, x: f32, y: f32, size: f32) -> Mark {
    Mark::Text {
        text: text.into(),
        font,
        at: Point::new(x, y),
        size,
    }
}

/// A line across the staff, `width` thick, with its left edge at `x`.
/// It covers the staff lines' outer halves too, so the corners are
/// clean.
fn bar_line(x: f32, width: f32) -> Mark {
    let overhang = sp(engraving::STAFF_LINE) / 2.0;
    let middle = x + width / 2.0;
    line(middle, -overhang, middle, sp(4.0) + overhang, width)
}

/// How much room a note or rest gets for lasting `steps` sixteenths,
/// before the line is stretched: about 2.6 spaces for a sixteenth, 4.2
/// for a quarter, 7.4 for a whole note.
fn spring(steps: usize) -> f32 {
    sp(1.0 + 1.6 * (steps as f32).sqrt())
}

/// How high and how low a mark reaches.
fn extent(mark: &Mark) -> (f32, f32) {
    match mark {
        Mark::Music { glyph, at, size } => (
            at.y - glyph.top * size / 4.0,
            at.y - glyph.bottom * size / 4.0,
        ),
        Mark::Text { at, size, .. } => (at.y - size * 0.75, at.y + size * 0.25),
        Mark::Line { from, to, width } => (
            from.y.min(to.y) - width / 2.0,
            from.y.max(to.y) + width / 2.0,
        ),
        Mark::Beam {
            from,
            to,
            thickness,
        } => {
            let ys = [from.y, to.y, from.y + thickness, to.y + thickness];
            (
                ys.iter().copied().fold(f32::MAX, f32::min),
                ys.iter().copied().fold(f32::MIN, f32::max),
            )
        }
        Mark::Tie { from, to, .. } => (from.y.min(to.y) - sp(1.0), from.y.max(to.y) + sp(1.0)),
    }
}

// ----- Chord symbols -----

/// A piece of a chord symbol: letters, or an accidental drawn with the
/// music font, each at its own size.
#[derive(Debug, Clone, PartialEq)]
enum Run {
    Text(String, f32),
    Accidental(i8, f32),
}

/// The pieces of a chord symbol: the root large, the quality a little
/// smaller, and real ♭ and ♯ signs for the accidentals: "B♭m7♭5/A♭".
fn chord_runs(harmony: HarmonyName) -> Vec<Run> {
    let HarmonyName::Chord(symbol) = harmony else {
        // N.C., sets, micromodes and row forms are written as they are
        // named on the chord sheet.
        return vec![Run::Text(harmony.to_string(), CHORD_SIZE * 0.75)];
    };
    let mut runs = Vec::new();
    let name = |runs: &mut Vec<Run>, name: NoteName| {
        runs.push(Run::Text(name.letter.as_char().to_string(), CHORD_SIZE));
        if name.accidental != 0 {
            runs.push(Run::Accidental(name.accidental, CHORD_SIZE));
        }
    };
    name(&mut runs, symbol.root);
    let small = CHORD_SIZE * 0.8;
    let quality: Vec<char> = symbol.quality.symbol().chars().collect();
    let mut letters = String::new();
    for (index, &c) in quality.iter().enumerate() {
        let before_digit = quality.get(index + 1).is_some_and(char::is_ascii_digit);
        if (c == 'b' || c == '#') && before_digit {
            if !letters.is_empty() {
                runs.push(Run::Text(std::mem::take(&mut letters), small));
            }
            runs.push(Run::Accidental(if c == '#' { 1 } else { -1 }, small));
        } else {
            letters.push(c);
        }
    }
    if !letters.is_empty() {
        runs.push(Run::Text(letters, small));
    }
    if let Some(bass) = symbol.bass {
        runs.push(Run::Text("/".to_string(), CHORD_SIZE));
        name(&mut runs, bass);
    }
    runs
}

/// The words at the top left of a part for a transposing instrument,
/// where parts name who reads them: "B♭ instruments". None at concert
/// pitch.
fn part_label(transposition: Transposition) -> Option<Vec<Run>> {
    if transposition == Transposition::Concert {
        return None;
    }
    let note = transposition.note();
    let letter = note.letter.shown();
    // The words go where the language puts them: "Instrumentos en Si♭".
    let (before, after) = crate::i18n::tr("{} instruments")
        .split_once("{}")
        .unwrap_or(("", " instruments"));
    let mut runs = Vec::new();
    if !before.is_empty() {
        runs.push(Run::Text(before.to_string(), PART_SIZE));
    }
    runs.push(Run::Text(letter, PART_SIZE));
    if note.accidental != 0 {
        runs.push(Run::Accidental(note.accidental, PART_SIZE));
    }
    if !after.is_empty() {
        runs.push(Run::Text(after.to_string(), PART_SIZE));
    }
    Some(runs)
}

/// The music font size of an accidental among letters of `size`.
fn accidental_size(size: f32) -> f32 {
    size * 0.75
}

fn run_width(run: &Run, measure: &dyn TextWidth) -> f32 {
    match run {
        Run::Text(letters, size) => measure.text_width(letters, TextFont::SemiBold, *size),
        Run::Accidental(accidental, size) => {
            let glyph = glyphs::chord_accidental(*accidental);
            glyph.width * accidental_size(*size) / 4.0 + size * 0.04
        }
    }
}

/// Draws a chord symbol with its baseline starting at `x`, `y`.
fn chord_marks(runs: &[Run], x: f32, y: f32, measure: &dyn TextWidth) -> Vec<Mark> {
    let mut marks = Vec::new();
    let mut x = x;
    for run in runs {
        match run {
            Run::Text(letters, size) => {
                marks.push(text(letters.clone(), TextFont::SemiBold, x, y, *size));
            }
            Run::Accidental(accidental, size) => {
                let glyph = glyphs::chord_accidental(*accidental);
                let music_size = accidental_size(*size);
                // Raised so its top is level with the capital letters.
                let raise = CAP_HEIGHT * size - glyph.top * music_size / 4.0;
                marks.push(Mark::Music {
                    glyph,
                    at: Point::new(x + size * 0.02, y - raise),
                    size: music_size,
                });
            }
        }
        x += run_width(run, measure);
    }
    marks
}

// ----- Bars -----

/// A place where something starts in a bar: a note, a rest, a slash or
/// a chord.
#[derive(Debug, Clone, Copy)]
struct Column {
    offset: usize,
    spring: f32,
    /// Room needed left of its x: an accidental.
    left: f32,
    /// Room needed right of its x: the notehead, its dot, its flag.
    right: f32,
}

/// What can be worked out about a bar once, however much it is
/// stretched.
struct Prepared {
    columns: Vec<Column>,
    /// For each piece: whether its stem goes up.
    up: Vec<bool>,
    /// For each piece: whether it is under a beam.
    beamed: Vec<bool>,
    /// For each chord: its column, its pieces and its width.
    chords: Vec<(usize, Vec<Run>, f32)>,
}

/// The line drawn at the right end of a bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Barline {
    /// None: the next bar starts with a repeat sign.
    None,
    Normal,
    Double,
    Final,
    RepeatEnd,
}

impl Barline {
    fn width(self) -> f32 {
        use engraving::*;
        match self {
            Barline::None => 0.0,
            Barline::Normal => sp(THIN_BARLINE),
            Barline::Double => sp(2.0 * THIN_BARLINE + BARLINE_SEPARATION),
            Barline::Final => sp(THIN_BARLINE + BARLINE_SEPARATION + THICK_BARLINE),
            Barline::RepeatEnd => sp(glyphs::REPEAT_DOT.width
                + REPEAT_DOT_SEPARATION
                + THIN_BARLINE
                + BARLINE_SEPARATION
                + THICK_BARLINE),
        }
    }

    /// Draws it with its left edge at `x`.
    fn draw(self, x: f32, marks: &mut Vec<Mark>) {
        use engraving::*;
        let thin = sp(THIN_BARLINE);
        let thick = sp(THICK_BARLINE);
        let gap = sp(BARLINE_SEPARATION);
        match self {
            Barline::None => {}
            Barline::Normal => marks.push(bar_line(x, thin)),
            Barline::Double => {
                marks.push(bar_line(x, thin));
                marks.push(bar_line(x + thin + gap, thin));
            }
            Barline::Final => {
                marks.push(bar_line(x, thin));
                marks.push(bar_line(x + thin + gap, thick));
            }
            Barline::RepeatEnd => {
                repeat_dots(x, marks);
                let x = x + width_of(glyphs::REPEAT_DOT) + sp(REPEAT_DOT_SEPARATION);
                marks.push(bar_line(x, thin));
                marks.push(bar_line(x + thin + gap, thick));
            }
        }
    }
}

/// The two dots of a repeat sign, in the spaces either side of the
/// middle line.
fn repeat_dots(x: f32, marks: &mut Vec<Mark>) {
    marks.push(music(glyphs::REPEAT_DOT, x, sp(1.5)));
    marks.push(music(glyphs::REPEAT_DOT, x, sp(2.5)));
}

/// Where a bar sits among the others, which changes what it draws.
#[derive(Debug, Clone, Copy)]
struct Place {
    /// First on its line: it starts with the clef and key signature.
    system_start: bool,
    /// Right after a bar ending in a repeat sign on the same line, which
    /// a repeat sign here shares its thick line with.
    after_repeat_end: bool,
    barline: Barline,
}

/// A notehead drawn, for stems, beams and ties.
#[derive(Debug, Clone, Copy)]
struct Head {
    x: f32,
    y: f32,
    width: f32,
    up: bool,
    value: Value,
}

impl Head {
    fn stem_x(&self) -> f32 {
        let half = sp(engraving::STEM) / 2.0;
        if self.up {
            self.x + self.width - half
        } else {
            self.x + half
        }
    }

    /// Where the stem leaves the notehead.
    fn stem_root(&self) -> f32 {
        let attach = sp(engraving::STEM_ATTACH);
        if self.up {
            self.y - attach
        } else {
            self.y + attach
        }
    }
}

/// A note's notehead, for the tie that may start or end on it.
#[derive(Debug, Clone, Copy)]
struct Anchor {
    /// Which note of the tune it is.
    index: usize,
    left: f32,
    right: f32,
    y: f32,
    stem_up: bool,
    tie: bool,
}

impl Anchor {
    fn moved(self, dx: f32) -> Anchor {
        Anchor {
            left: self.left + dx,
            right: self.right + dx,
            ..self
        }
    }
}

/// A bar drawn with its left edge at x = 0.
#[derive(Default)]
struct DrawnBar {
    marks: Vec<Mark>,
    width: f32,
    /// Where its notes start, after the clef and signatures.
    content_start: f32,
    anchors: Vec<Anchor>,
    /// The x of each of its chords.
    chords: Vec<f32>,
}

/// A line of music drawn with its left end at x = 0.
struct DrawnSystem {
    bars: Range<usize>,
    marks: Vec<Mark>,
    width: f32,
    content_start: f32,
    /// For each bar: its left and right x.
    edges: Vec<(f32, f32)>,
    anchors: Vec<Anchor>,
    /// For each chord: its x, bar and number in the bar.
    chords: Vec<(f32, usize, usize)>,
}

struct Engraver<'a> {
    score: &'a Score,
    measure: &'a dyn TextWidth,
    prepared: Vec<Prepared>,
}

impl<'a> Engraver<'a> {
    fn new(score: &'a Score, measure: &'a dyn TextWidth) -> Self {
        let mut engraver = Engraver {
            score,
            measure,
            prepared: Vec::new(),
        };
        engraver.prepared = score.bars.iter().map(|bar| engraver.prepare(bar)).collect();
        engraver
    }

    fn clef(&self) -> Clef {
        self.score.clef
    }

    fn middle(&self) -> i32 {
        self.clef().bottom_line() + 4
    }

    fn prepare(&self, bar: &BarScore) -> Prepared {
        let (up, beamed) = match &bar.content {
            Content::Notes { pieces, beams } => {
                let middle = self.middle();
                let mut up: Vec<bool> = pieces
                    .iter()
                    .map(|piece| piece.note.is_some_and(|note| note.step < middle))
                    .collect();
                let mut beamed = vec![false; pieces.len()];
                for beam in beams {
                    // The whole group goes the way of the note farthest
                    // from the middle line.
                    let steps = pieces[beam.clone()]
                        .iter()
                        .filter_map(|piece| piece.note.map(|note| note.step));
                    let (low, high) = steps.fold((i32::MAX, i32::MIN), |(low, high), step| {
                        (low.min(step), high.max(step))
                    });
                    let group_up = middle - low > high - middle;
                    up[beam.clone()].fill(group_up);
                    beamed[beam.clone()].fill(true);
                }
                (up, beamed)
            }
            _ => (Vec::new(), Vec::new()),
        };

        let length = bar.bar.length;
        let mut offsets: Vec<usize> = match &bar.content {
            Content::Notes { pieces, .. } => pieces.iter().map(|piece| piece.from).collect(),
            Content::Slashes => bar
                .bar
                .time
                .beat_starts()
                .into_iter()
                .filter(|&offset| offset < length)
                .collect(),
            Content::WholeRest => vec![0],
        };
        offsets.extend(bar.chords.iter().map(|(offset, _)| *offset));
        offsets.sort_unstable();
        offsets.dedup();
        let columns: Vec<Column> = offsets
            .iter()
            .enumerate()
            .map(|(index, &offset)| {
                let next = offsets.get(index + 1).copied().unwrap_or(length);
                let (left, right) = self.room(bar, offset, &up, &beamed);
                Column {
                    offset,
                    spring: spring(next - offset),
                    left,
                    right,
                }
            })
            .collect();

        let chords = bar
            .chords
            .iter()
            .map(|(offset, harmony)| {
                let column = offsets.iter().position(|o| o == offset).unwrap_or(0);
                let runs = chord_runs(*harmony);
                let width = runs.iter().map(|run| run_width(run, self.measure)).sum();
                (column, runs, width)
            })
            .collect();
        Prepared {
            columns,
            up,
            beamed,
            chords,
        }
    }

    /// The room what starts at `offset` needs on each side.
    fn room(&self, bar: &BarScore, offset: usize, up: &[bool], beamed: &[bool]) -> (f32, f32) {
        let dot = sp(0.25) + width_of(glyphs::DOT);
        match &bar.content {
            Content::Notes { pieces, .. } => {
                let Some(index) = pieces.iter().position(|piece| piece.from == offset) else {
                    return (0.0, 0.0);
                };
                let piece = &pieces[index];
                let dotted = if piece.value.dotted { dot } else { 0.0 };
                match piece.note {
                    Some(note) => {
                        let left = note
                            .accidental
                            .map_or(0.0, |a| width_of(glyphs::accidental(a)) + sp(0.25));
                        let head = width_of(head_glyph(piece.value));
                        let flag = if up[index] && !beamed[index] && piece.value.beams() > 0 {
                            width_of(glyphs::FLAG_8TH_UP) - sp(engraving::STEM)
                        } else {
                            0.0
                        };
                        (left, head + dotted.max(flag))
                    }
                    None => (0.0, width_of(rest_glyph(piece.value.base)) + dotted),
                }
            }
            Content::Slashes => (0.0, width_of(glyphs::SLASH)),
            Content::WholeRest => (0.0, width_of(glyphs::REST_WHOLE) + sp(1.0)),
        }
    }

    /// Draws bar `index` stretched by `stretch`.
    fn draw_bar(&self, index: usize, place: Place, stretch: f32) -> DrawnBar {
        let bar = &self.score.bars[index];
        let prepared = &self.prepared[index];
        let clef = self.clef();
        let mut drawn = DrawnBar::default();
        let marks = &mut drawn.marks;

        // The clef and signatures.
        let mut x = 0.0;
        if place.system_start {
            x += sp(0.6);
            let (glyph, y) = match clef {
                Clef::Treble => (glyphs::G_CLEF, sp(3.0)),
                Clef::Bass => (glyphs::F_CLEF, sp(1.0)),
            };
            marks.push(music(glyph, x, y));
            x += width_of(glyph) + sp(1.0);
            x = self.key_signature(bar, x, marks);
        } else if let Some(old) = bar.key_change {
            x += sp(0.8);
            let naturals = old.cancellations(bar.signature, clef);
            for step in &naturals {
                marks.push(music(glyphs::NATURAL, x, y_of(clef, *step)));
                x += width_of(glyphs::NATURAL) + sp(0.15);
            }
            if !naturals.is_empty() {
                x += sp(0.5);
            }
            x = self.key_signature(bar, x, marks);
        }
        if bar.time_change {
            x = time_signature(bar.bar.time, x.max(sp(0.6)), marks);
        }
        if bar.repeat_start {
            use engraving::*;
            if !place.after_repeat_end || place.system_start || x > 0.0 {
                if x > 0.0 {
                    x += sp(0.4);
                }
                marks.push(bar_line(x, sp(THICK_BARLINE)));
                x += sp(THICK_BARLINE);
            }
            x += sp(BARLINE_SEPARATION);
            marks.push(bar_line(x, sp(THIN_BARLINE)));
            x += sp(THIN_BARLINE + REPEAT_DOT_SEPARATION);
            repeat_dots(x, marks);
            x += width_of(glyphs::REPEAT_DOT);
        }
        // A little more room after a plain bar line than after a clef
        // or signature, which have room of their own.
        let content_start = x + if x == 0.0 {
            sp(1.4)
        } else if !place.system_start && bar.key_change.is_some() {
            sp(1.6)
        } else {
            sp(1.0)
        };

        // Where each column goes.
        let columns = &prepared.columns;
        let mut xs = Vec::with_capacity(columns.len() + 1);
        let mut at = content_start + columns.first().map_or(0.0, |column| column.left);
        for (number, column) in columns.iter().enumerate() {
            xs.push(at);
            let next_left = columns.get(number + 1).map_or(sp(0.8), |next| next.left);
            at += (column.spring * stretch).max(column.right + sp(0.5) + next_left);
        }
        xs.push(at);
        // Chord symbols must not run into the next one.
        for (number, (column, _, width)) in prepared.chords.iter().enumerate() {
            let next = prepared.chords.get(number + 1);
            let until = next.map_or(columns.len(), |next| next.0);
            let needed = width + if next.is_some() { sp(1.0) } else { sp(0.3) };
            let missing = needed - (xs[until] - xs[*column]);
            if missing > 0.0 {
                for x in &mut xs[until..] {
                    *x += missing;
                }
            }
        }
        let end = *xs.last().unwrap_or(&content_start);
        let column_x = |offset: usize| {
            columns
                .iter()
                .position(|column| column.offset == offset)
                .map_or(content_start, |index| xs[index])
        };

        match &bar.content {
            Content::Notes { pieces, beams } => {
                self.draw_notes(pieces, beams, prepared, &column_x, &mut drawn);
            }
            Content::WholeRest => {
                let glyph = glyphs::REST_WHOLE;
                let x = (content_start + end - width_of(glyph)) / 2.0;
                drawn.marks.push(music(glyph, x, sp(1.0)));
            }
            Content::Slashes => {
                for column in columns {
                    if bar.bar.time.beat_starts().contains(&column.offset) {
                        drawn
                            .marks
                            .push(music(glyphs::SLASH, column_x(column.offset), sp(2.0)));
                    }
                }
            }
        }

        place.barline.draw(end, &mut drawn.marks);
        drawn.width = end + place.barline.width();
        drawn.content_start = content_start;
        drawn.chords = prepared
            .chords
            .iter()
            .map(|(column, ..)| xs[*column])
            .collect();
        drawn
    }

    fn key_signature(&self, bar: &BarScore, x: f32, marks: &mut Vec<Mark>) -> f32 {
        let mut x = x;
        let accidentals = bar.signature.accidentals(self.clef());
        for (accidental, step) in &accidentals {
            let glyph = glyphs::accidental(*accidental);
            marks.push(music(glyph, x, y_of(self.clef(), *step)));
            x += width_of(glyph) + sp(0.12);
        }
        if !accidentals.is_empty() {
            x += sp(0.9);
        }
        x
    }

    fn draw_notes(
        &self,
        pieces: &[Piece],
        beams: &[Range<usize>],
        prepared: &Prepared,
        column_x: &dyn Fn(usize) -> f32,
        drawn: &mut DrawnBar,
    ) {
        let clef = self.clef();
        let middle_y = sp(2.0);
        let mut heads: Vec<Option<Head>> = Vec::with_capacity(pieces.len());
        for (index, piece) in pieces.iter().enumerate() {
            let x = column_x(piece.from);
            let Some(note) = piece.note else {
                let glyph = rest_glyph(piece.value.base);
                let y = if piece.value.base == Base::Whole {
                    sp(1.0)
                } else {
                    middle_y
                };
                drawn.marks.push(music(glyph, x, y));
                if piece.value.dotted {
                    let dot_x = x + width_of(glyph) + sp(0.25);
                    drawn.marks.push(music(glyphs::DOT, dot_x, sp(1.5)));
                }
                heads.push(None);
                continue;
            };
            let glyph = head_glyph(piece.value);
            let y = y_of(clef, note.step);
            let width = width_of(glyph);
            drawn.marks.push(music(glyph, x, y));
            if let Some(accidental) = note.accidental {
                let sign = glyphs::accidental(accidental);
                drawn
                    .marks
                    .push(music(sign, x - width_of(sign) - sp(0.2), y));
            }
            self.ledger_lines(note.step, x, width, &mut drawn.marks);
            let mut right = x + width;
            if piece.value.dotted {
                let on_line = (note.step - clef.bottom_line()).rem_euclid(2) == 0;
                let dot_y = if on_line { y - sp(0.5) } else { y };
                let dot_x = x + width + sp(0.25);
                drawn.marks.push(music(glyphs::DOT, dot_x, dot_y));
                right = dot_x + width_of(glyphs::DOT);
            }
            let head = Head {
                x,
                y,
                width,
                up: prepared.up[index],
                value: piece.value,
            };
            drawn.anchors.push(Anchor {
                index: note.index,
                left: x,
                right,
                y,
                stem_up: head.up,
                tie: note.tie,
            });
            if piece.value.base != Base::Whole && !prepared.beamed[index] {
                draw_stem(&head, &mut drawn.marks);
            }
            heads.push(Some(head));
        }
        for beam in beams {
            let group: Vec<Head> = heads[beam.clone()].iter().flatten().copied().collect();
            draw_beam(&group, &mut drawn.marks);
        }
    }

    fn ledger_lines(&self, step: i32, x: f32, width: f32, marks: &mut Vec<Mark>) {
        let bottom = self.clef().bottom_line();
        let top = bottom + 8;
        let reach = sp(engraving::LEGER_EXTENSION);
        let thickness = sp(engraving::LEGER_LINE);
        let mut line_at = |at: i32| {
            let y = y_of(self.clef(), at);
            marks.push(line(x - reach, y, x + width + reach, y, thickness));
        };
        let mut at = bottom - 2;
        while at >= step {
            line_at(at);
            at -= 2;
        }
        let mut at = top + 2;
        while at <= step {
            line_at(at);
            at += 2;
        }
    }

    /// The line after bar `index`, on a line of music that ends before
    /// bar `system_end`.
    fn barline(&self, index: usize, system_end: usize) -> Barline {
        let bars = &self.score.bars;
        let bar = &bars[index];
        let following = bars.get(index + 1);
        let next = following.filter(|_| index + 1 < system_end);
        if bar.repeat_end.is_some() {
            Barline::RepeatEnd
        } else if bar.last {
            Barline::Final
        } else if next
            .is_some_and(|next| next.repeat_start && next.key_change.is_none() && !next.time_change)
        {
            Barline::None
        } else if bar.double_bar || following.is_some_and(|next| next.key_change.is_some()) {
            // Before a new section, and before a change of key.
            Barline::Double
        } else {
            Barline::Normal
        }
    }

    fn draw_system(&self, bars: Range<usize>, stretch: f32) -> DrawnSystem {
        let mut system = DrawnSystem {
            bars: bars.clone(),
            marks: Vec::new(),
            width: 0.0,
            content_start: 0.0,
            edges: Vec::new(),
            anchors: Vec::new(),
            chords: Vec::new(),
        };
        let mut x = 0.0;
        for index in bars.clone() {
            let place = Place {
                system_start: index == bars.start,
                after_repeat_end: index > bars.start
                    && self.score.bars[index - 1].repeat_end.is_some(),
                barline: self.barline(index, bars.end),
            };
            let bar = self.draw_bar(index, place, stretch);
            if index == bars.start {
                system.content_start = bar.content_start;
            }
            system
                .marks
                .extend(bar.marks.iter().map(|mark| mark.moved(x, 0.0)));
            system
                .anchors
                .extend(bar.anchors.iter().map(|anchor| anchor.moved(x)));
            system.chords.extend(
                bar.chords
                    .iter()
                    .enumerate()
                    .map(|(number, chord_x)| (x + chord_x, index, number)),
            );
            system.edges.push((x, x + bar.width));
            x += bar.width;
        }
        system.width = x;
        system
    }

    /// How much to stretch `bars` so they fill `width`.
    fn fit(&self, bars: Range<usize>, width: f32) -> f32 {
        let wide = |stretch: f32| self.draw_system(bars.clone(), stretch).width;
        let (mut low, mut high) = (0.25_f32, 6.0_f32);
        if wide(high) <= width {
            return high;
        }
        if wide(low) >= width {
            return low;
        }
        for _ in 0..24 {
            let middle = (low + high) / 2.0;
            if wide(middle) > width {
                high = middle;
            } else {
                low = middle;
            }
        }
        low
    }

    /// Splits the bars into lines of music, each with its stretch.
    fn systems(&self, width: f32) -> Vec<(Range<usize>, f32)> {
        let bars = &self.score.bars;
        let mut lines: Vec<(Range<usize>, bool)> = Vec::new();
        let mut start = 0;
        while start < bars.len() {
            let mut end = start + 1;
            let mut full = false;
            while end < bars.len() && bars[end].section.is_none() {
                if end - start >= MAX_BARS
                    || self.draw_system(start..end + 1, MIN_STRETCH).width > width
                {
                    full = true;
                    break;
                }
                end += 1;
            }
            full |= end - start >= MAX_BARS;
            lines.push((start..end, full));
            start = end;
        }
        // Full lines are stretched to the page's width. A short line, at
        // the end of a section or the song, is spaced like the full ones
        // and left shorter.
        let fits: Vec<f32> = lines
            .iter()
            .map(|(bars, _)| self.fit(bars.clone(), width))
            .collect();
        let full: Vec<f32> = lines
            .iter()
            .zip(&fits)
            .filter(|((_, full), _)| *full)
            .map(|(_, fit)| *fit)
            .collect();
        let usual = if full.is_empty() {
            1.3
        } else {
            (full.iter().sum::<f32>() / full.len() as f32).max(1.0)
        };
        lines
            .into_iter()
            .zip(fits)
            .map(|((bars, full), fit)| (bars, if full { fit } else { fit.min(usual) }))
            .collect()
    }
}

fn head_glyph(value: Value) -> Glyph {
    match value.base {
        Base::Whole => glyphs::NOTEHEAD_WHOLE,
        Base::Half => glyphs::NOTEHEAD_HALF,
        _ => glyphs::NOTEHEAD_BLACK,
    }
}

fn rest_glyph(base: Base) -> Glyph {
    match base {
        Base::Whole => glyphs::REST_WHOLE,
        Base::Half => glyphs::REST_HALF,
        Base::Quarter => glyphs::REST_QUARTER,
        Base::Eighth => glyphs::REST_8TH,
        Base::Sixteenth => glyphs::REST_16TH,
    }
}

/// A stem of its own, with a flag for an eighth or a sixteenth. Notes
/// far from the staff get stems reaching the middle line.
fn draw_stem(head: &Head, marks: &mut Vec<Mark>) {
    let length = if head.value.beams() >= 2 {
        sp(3.75)
    } else {
        sp(3.5)
    };
    let middle = sp(2.0);
    let tip = if head.up {
        (head.y - length).min(middle)
    } else {
        (head.y + length).max(middle)
    };
    let x = head.stem_x();
    marks.push(line(x, head.stem_root(), x, tip, sp(engraving::STEM)));
    let flag = match (head.value.beams(), head.up) {
        (0, _) => return,
        (1, true) => glyphs::FLAG_8TH_UP,
        (1, false) => glyphs::FLAG_8TH_DOWN,
        (_, true) => glyphs::FLAG_16TH_UP,
        (_, false) => glyphs::FLAG_16TH_DOWN,
    };
    marks.push(music(flag, x - sp(engraving::STEM) / 2.0, tip));
}

/// Stems and beams for a group of eighths and sixteenths.
fn draw_beam(group: &[Head], marks: &mut Vec<Mark>) {
    let (Some(first), Some(last)) = (group.first(), group.last()) else {
        return;
    };
    let up = first.up;
    let (x1, x2) = (first.stem_x(), last.stem_x());
    let run = (x2 - x1).max(1.0);
    // The beam follows the notes, but never slants more than a space.
    let slope = (last.y - first.y).clamp(-sp(1.0), sp(1.0)) / run;
    let levels = group
        .iter()
        .map(|head| head.value.beams())
        .max()
        .unwrap_or(1);
    let shortest = sp(3.25) + sp(0.75) * (levels as f32 - 1.0);
    let starts = group
        .iter()
        .map(|head| head.y - slope * (head.stem_x() - x1));
    let start = if up {
        starts.fold(f32::MAX, |low, y| low.min(y - shortest))
    } else {
        starts.fold(f32::MIN, |high, y| high.max(y + shortest))
    };
    let beam_y = |x: f32| start + slope * (x - x1);
    let half = sp(engraving::STEM) / 2.0;
    let thickness = if up {
        sp(engraving::BEAM)
    } else {
        -sp(engraving::BEAM)
    };
    for head in group {
        let x = head.stem_x();
        marks.push(line(x, head.stem_root(), x, beam_y(x), sp(engraving::STEM)));
    }
    let beam = |from: f32, to: f32, level: usize| {
        let shift = (engraving::BEAM + engraving::BEAM_SPACING) * level as f32;
        let shift = if up { sp(shift) } else { -sp(shift) };
        Mark::Beam {
            from: Point::new(from, beam_y(from) + shift),
            to: Point::new(to, beam_y(to) + shift),
            thickness,
        }
    };
    marks.push(beam(x1 - half, x2 + half, 0));
    // Second beams join neighbouring sixteenths; a sixteenth alone gets
    // a short one pointing into its beat.
    for (index, head) in group.iter().enumerate() {
        if head.value.beams() < 2 {
            continue;
        }
        let x = head.stem_x();
        let next = group.get(index + 1).filter(|next| next.value.beams() >= 2);
        let previous = index
            .checked_sub(1)
            .and_then(|previous| group.get(previous))
            .filter(|previous| previous.value.beams() >= 2);
        if let Some(next) = next {
            marks.push(beam(x - half, next.stem_x() + half, 1));
        } else if previous.is_none() {
            let stub = sp(1.1);
            if index == 0 {
                marks.push(beam(x - half, x + stub, 1));
            } else {
                marks.push(beam(x - stub, x + half, 1));
            }
        }
    }
}

/// Draws a time signature at `x` and returns where it ends.
fn time_signature(time: TimeSignature, x: f32, marks: &mut Vec<Mark>) -> f32 {
    let top: Vec<Glyph> = if time.is_grouped() {
        let mut glyphs_ = Vec::new();
        for (number, group) in time.groups().iter().enumerate() {
            if number > 0 {
                glyphs_.push(glyphs::TIME_PLUS);
            }
            glyphs_.extend(digits(u32::from(*group)));
        }
        glyphs_
    } else {
        digits(u32::from(time.beats))
    };
    let bottom = digits(u32::from(time.unit));
    let wide = |glyphs_: &[Glyph]| glyphs_.iter().map(|glyph| width_of(*glyph)).sum::<f32>();
    let width = wide(&top).max(wide(&bottom));
    for (row, y) in [(&top, sp(1.0)), (&bottom, sp(3.0))] {
        let mut at = x + (width - wide(row)) / 2.0;
        for glyph in row.iter() {
            marks.push(music(*glyph, at, y));
            at += width_of(*glyph);
        }
    }
    x + width + sp(1.0)
}

fn digits(number: u32) -> Vec<Glyph> {
    number
        .to_string()
        .bytes()
        .map(|digit| glyphs::TIME_DIGITS[usize::from(digit - b'0')])
        .collect()
}

/// A text drawn in a box, as rehearsal marks are; returns its marks and
/// width.
fn boxed(label: &str, x: f32, baseline: f32, measure: &dyn TextWidth) -> (Vec<Mark>, f32) {
    let pad = sp(0.5);
    let width = measure.text_width(label, TextFont::SemiBold, REHEARSAL_SIZE) + 2.0 * pad;
    let top = baseline - CAP_HEIGHT * REHEARSAL_SIZE - pad;
    let bottom = baseline + pad;
    let stroke = sp(engraving::ENCLOSURE);
    let right = x + width;
    let marks = vec![
        text(label, TextFont::SemiBold, x + pad, baseline, REHEARSAL_SIZE),
        line(x, top, right, top, stroke),
        line(x, bottom, right, bottom, stroke),
        line(x, top - stroke / 2.0, x, bottom + stroke / 2.0, stroke),
        line(
            right,
            top - stroke / 2.0,
            right,
            bottom + stroke / 2.0,
            stroke,
        ),
    ];
    (marks, width)
}

/// A line of music finished: everything drawn, with how high and low it
/// reaches.
struct System {
    marks: Vec<Mark>,
    top: f32,
    bottom: f32,
    /// For each bar: the steps it covers and its left and right x.
    bars: Vec<(usize, usize, f32, f32)>,
    /// The y of the top of the chord symbols.
    chord_top: f32,
}

/// Lays the score out on pages of `options.paper`.
pub fn lay_out(score: &Score, options: &Options, measure: &dyn TextWidth) -> LeadSheet {
    let (page_width, page_height) = options.paper.size();
    let width = page_width - 2.0 * MARGIN;
    let engraver = Engraver::new(score, measure);
    let lines = engraver.systems(width);

    let mut systems = Vec::new();
    // A tie still open at the end of a line: the note it goes on to.
    let mut carried: Option<Anchor> = None;
    for (number, (bars, stretch)) in lines.iter().enumerate() {
        let drawn = engraver.draw_system(bars.clone(), *stretch);
        systems.push(finish_system(&engraver, drawn, number == 0, &mut carried));
    }

    // Lines on pages: as many as fit, spread out on every page but the
    // last. The first page starts with the title and, for a transposing
    // instrument, who reads the part: on the title's line, left of it,
    // or under it when the title leaves no room.
    let title_width = measure.text_width(&options.title, TextFont::SemiBold, TITLE_SIZE);
    let label = part_label(options.transposition);
    let label_width: f32 = label
        .iter()
        .flatten()
        .map(|run| run_width(run, measure))
        .sum();
    let label_below = label.is_some()
        && !options.title.is_empty()
        && MARGIN + label_width + sp(2.0) > (page_width - title_width) / 2.0;
    let title_room = if options.title.is_empty() && label.is_none() {
        0.0
    } else if label_below {
        TITLE_SIZE + PART_SIZE + 24.0
    } else {
        TITLE_SIZE + 18.0
    };
    let footer = SMALL_SIZE + 6.0;
    let mut pages: Vec<Vec<usize>> = vec![Vec::new()];
    let mut used = title_room;
    for (index, system) in systems.iter().enumerate() {
        let height = system.bottom - system.top;
        let room = page_height - 2.0 * MARGIN - footer;
        let page = pages.last_mut().expect("there is always a page");
        let gap = if page.is_empty() { 0.0 } else { SYSTEM_GAP };
        if !page.is_empty() && used + gap + height > room {
            pages.push(vec![index]);
            used = height;
        } else {
            page.push(index);
            used += gap + height;
        }
    }

    let total = pages.len();
    let mut sheet = LeadSheet {
        pages: Vec::new(),
        bars: Vec::new(),
        clef: score.clef,
    };
    for (page_number, page) in pages.iter().enumerate() {
        let mut marks = Vec::new();
        let mut y = MARGIN;
        if page_number == 0 {
            let baseline = MARGIN + TITLE_SIZE * CAP_HEIGHT;
            if !options.title.is_empty() {
                marks.push(text(
                    options.title.clone(),
                    TextFont::SemiBold,
                    (page_width - title_width) / 2.0,
                    baseline,
                    TITLE_SIZE,
                ));
            }
            if let Some(runs) = &label {
                let y = if label_below {
                    baseline + PART_SIZE + 6.0
                } else {
                    baseline
                };
                marks.extend(chord_marks(runs, MARGIN, y, measure));
            }
            y += title_room;
        }
        if score.bars.is_empty() {
            marks.push(text(
                crate::i18n::tr("The song has no bars yet."),
                TextFont::Regular,
                MARGIN,
                y + 20.0,
                TEMPO_SIZE,
            ));
        }
        let heights: f32 = page
            .iter()
            .map(|&index| systems[index].bottom - systems[index].top)
            .sum();
        let room = page_height - 2.0 * MARGIN - footer - (y - MARGIN);
        let gaps = page.len().saturating_sub(1) as f32;
        let gap = if page_number + 1 < total && gaps > 0.0 {
            ((room - heights) / gaps).clamp(SYSTEM_GAP, SYSTEM_GAP + sp(5.0))
        } else {
            SYSTEM_GAP
        };
        for &index in page {
            let system = &systems[index];
            let staff_y = y - system.top;
            marks.extend(system.marks.iter().map(|mark| mark.moved(MARGIN, staff_y)));
            for &(start, end, left, right) in &system.bars {
                sheet.bars.push(BarBox {
                    page: page_number,
                    start,
                    end,
                    top_left: Point::new(MARGIN + left, staff_y + system.chord_top),
                    bottom_right: Point::new(MARGIN + right, staff_y + sp(5.5)),
                });
            }
            y = staff_y + system.bottom + gap;
        }
        if total > 1 {
            let label = format!("{} / {}", page_number + 1, total);
            let label_width = measure.text_width(&label, TextFont::Regular, SMALL_SIZE);
            marks.push(text(
                label,
                TextFont::Regular,
                (page_width - label_width) / 2.0,
                page_height - MARGIN + footer,
                SMALL_SIZE,
            ));
        }
        sheet.pages.push(Page {
            width: page_width,
            height: page_height,
            marks,
        });
    }
    sheet
}

/// Adds what goes above and around the bars of a line: staff lines,
/// ties, chord symbols, rehearsal marks, bar numbers and the tempo.
fn finish_system(
    engraver: &Engraver,
    drawn: DrawnSystem,
    first: bool,
    carried: &mut Option<Anchor>,
) -> System {
    let score = engraver.score;
    let measure = engraver.measure;
    let mut marks = drawn.marks;

    // Ties, including those that cross from the line before or on to
    // the next.
    let tie = |from: &Anchor, from_x: f32, to_x: f32| {
        let above = !from.stem_up;
        let y = if above {
            from.y - sp(0.55)
        } else {
            from.y + sp(0.55)
        };
        Mark::Tie {
            from: Point::new(from_x, y),
            to: Point::new(to_x, y),
            up: above,
            thickness: sp(engraving::TIE_MIDDLE),
        }
    };
    let mut previous: Option<Anchor> = None;
    for anchor in &drawn.anchors {
        if let Some(open) = carried.take()
            && open.index == anchor.index
        {
            let from = Anchor {
                left: anchor.left - sp(2.5),
                right: anchor.left - sp(2.5),
                ..*anchor
            };
            marks.push(tie(&from, from.right, anchor.left - sp(0.15)));
        }
        if let Some(before) = previous
            && before.tie
            && before.index == anchor.index
        {
            marks.push(tie(
                &before,
                before.right + sp(0.15),
                anchor.left - sp(0.15),
            ));
        }
        previous = Some(*anchor);
    }
    *carried = None;
    if let Some(last) = previous.filter(|last| last.tie) {
        marks.push(tie(&last, last.right + sp(0.15), drawn.width - sp(0.2)));
        *carried = Some(last);
    }

    // The highest point of the music, which the chords go above.
    let music_top = marks
        .iter()
        .map(|mark| extent(mark).0)
        .fold(0.0_f32, f32::min);

    for line_number in 0..5 {
        let y = sp(line_number as f32);
        marks.push(line(0.0, y, drawn.width, y, sp(engraving::STAFF_LINE)));
    }

    let chord_baseline = (music_top - sp(1.2)).min(-sp(2.5));
    let chord_top = chord_baseline - CAP_HEIGHT * CHORD_SIZE - sp(0.5);
    for &(x, bar, number) in &drawn.chords {
        let runs = &engraver.prepared[bar].chords[number].1;
        marks.extend(chord_marks(runs, x, chord_baseline, measure));
    }
    let above = if drawn.chords.is_empty() {
        music_top.min(0.0) - sp(1.5)
    } else {
        chord_top - sp(0.8)
    };

    // Rehearsal marks for sections, "×3" for sections played three
    // times or more, and the tempo at the start of the song.
    let mut tempo_x = 0.0;
    for (index, &(left, right)) in drawn.bars.clone().zip(&drawn.edges) {
        let bar = &score.bars[index];
        if let Some(name) = &bar.section {
            let (boxed_marks, width) = boxed(name, left, above, measure);
            marks.extend(boxed_marks);
            if index == 0 {
                tempo_x = width + sp(1.5);
            }
        }
        if let Some(times) = bar.repeat_end.filter(|&times| times > 2) {
            let label = format!("×{times}");
            let label_width = measure.text_width(&label, TextFont::SemiBold, REHEARSAL_SIZE);
            marks.push(text(
                label,
                TextFont::SemiBold,
                right - label_width,
                above,
                REHEARSAL_SIZE,
            ));
        }
    }
    let start = drawn.bars.start;
    if start > 0 {
        let number = score.bars[start].bar.number + 1;
        marks.push(text(
            number.to_string(),
            TextFont::Regular,
            0.0,
            -sp(2.1),
            SMALL_SIZE,
        ));
    }
    if first && let Some(bar) = score.bars.first() {
        marks.extend(tempo_mark(
            score.tempo,
            bar.bar.time,
            tempo_x,
            above,
            measure,
        ));
    }

    let (top, bottom) = marks
        .iter()
        .map(extent)
        .fold((f32::MAX, f32::MIN), |(top, bottom), (high, low)| {
            (top.min(high), bottom.max(low))
        });
    System {
        bars: drawn
            .bars
            .clone()
            .zip(&drawn.edges)
            .map(|(index, &(left, right))| {
                let bar = &score.bars[index].bar;
                (bar.start, bar.end(), left, right)
            })
            .collect(),
        marks,
        top: top.min(chord_top),
        bottom: bottom.max(sp(4.0)),
        chord_top,
    }
}

/// "♩ = 120", or "♩. = 80" when the beats are dotted quarters (6/8).
fn tempo_mark(
    bpm: f32,
    time: TimeSignature,
    x: f32,
    baseline: f32,
    measure: &dyn TextWidth,
) -> Vec<Mark> {
    let starts = time.beat_starts();
    let bar = time.steps_per_bar();
    let dotted = starts
        .iter()
        .zip(starts.iter().skip(1).copied().chain([bar]))
        .all(|(from, to)| to - from == 6);
    let size = TEMPO_SIZE * 1.2;
    let note = glyphs::METRONOME_QUARTER;
    let mut marks = vec![Mark::Music {
        glyph: note,
        at: Point::new(x, baseline),
        size,
    }];
    let mut at = x + note.width * size / 4.0;
    if dotted {
        at += size * 0.06;
        marks.push(Mark::Music {
            glyph: glyphs::METRONOME_DOT,
            at: Point::new(at, baseline),
            size,
        });
        at += glyphs::METRONOME_DOT.width * size / 4.0;
    }
    let beats = if dotted { bpm / 1.5 } else { bpm };
    at += measure.text_width(" ", TextFont::Regular, TEMPO_SIZE);
    let label = format!("= {}", beats.round());
    marks.push(text(label, TextFont::Regular, at, baseline, TEMPO_SIZE));
    marks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chord_sheet::ChordMarker;
    use crate::lead_sheet::{Paper, score};
    use crate::pattern::NoteEvent;
    use crate::pitch::Note;
    use crate::playlist::{Clip, Section};
    use crate::project::Project;

    /// Every letter half a size wide.
    struct Even;

    impl TextWidth for Even {
        fn text_width(&self, text: &str, _: TextFont, size: f32) -> f32 {
            text.chars().count() as f32 * size * 0.5
        }
    }

    fn options() -> Options {
        Options {
            title: "Test".to_string(),
            melody: None,
            clef: None,
            paper: Paper::A4,
            transposition: Transposition::Concert,
        }
    }

    /// A tune of quarter notes going up and down, `bars` bars long, with
    /// a chord on every bar.
    fn song(bars: usize) -> (Project, usize) {
        let mut project = Project::empty();
        let channel = crate::library::apply::synth_named(&mut project, "Lead").unwrap();
        let pattern = project.pattern_mut(0).unwrap();
        pattern.set_length(16 * bars);
        let notes: Vec<NoteEvent> = (0..bars * 4)
            .map(|i| NoteEvent::new(Note::new(60 + (i % 8) as u8), i * 4, 4))
            .collect();
        pattern.set_notes(channel, notes);
        project.add_clip(Clip::new(0, 0, 0));
        let names = ["C", "Am7", "Dm7", "G7"];
        let chords: Vec<ChordMarker> = (0..bars)
            .map(|bar| ChordMarker::new(bar * 16, HarmonyName::parse(names[bar % 4]).unwrap()))
            .collect();
        assert!(project.set_chords(0, bars * 16, &chords));
        (project, channel)
    }

    fn sheet(project: &Project, channel: Option<usize>) -> LeadSheet {
        let options = Options {
            melody: channel,
            ..options()
        };
        let score = score::build(project, options.melody, options.clef, options.transposition);
        lay_out(&score, &options, &Even)
    }

    fn glyph_count(sheet: &LeadSheet, glyph: Glyph) -> usize {
        sheet
            .pages
            .iter()
            .flat_map(|page| &page.marks)
            .filter(|mark| matches!(mark, Mark::Music { glyph: g, .. } if *g == glyph))
            .count()
    }

    #[test]
    fn every_note_gets_a_notehead_and_every_bar_a_box() {
        let (project, channel) = song(8);
        let sheet = sheet(&project, Some(channel));
        assert_eq!(sheet.pages.len(), 1);
        assert_eq!(glyph_count(&sheet, glyphs::NOTEHEAD_BLACK), 32);
        assert_eq!(sheet.bars.len(), 8);
        // Four bars a line, so two lines, each starting with a clef.
        assert_eq!(glyph_count(&sheet, glyphs::G_CLEF), 2);
    }

    #[test]
    fn everything_stays_on_the_page() {
        let (project, channel) = song(40);
        let sheet = sheet(&project, Some(channel));
        assert!(sheet.pages.len() >= 2);
        for page in &sheet.pages {
            for mark in &page.marks {
                let (top, bottom) = extent(mark);
                assert!(top >= 0.0 && bottom <= page.height, "{mark:?}");
                if let Mark::Music { at, .. } | Mark::Text { at, .. } = mark {
                    assert!(at.x >= 0.0 && at.x <= page.width, "{mark:?}");
                }
            }
        }
        // Bars come in order, and full lines end at the right margin.
        let right = sheet.pages[0].width - MARGIN;
        let ends: Vec<f32> = sheet.bars.iter().map(|bar| bar.bottom_right.x).collect();
        assert!((ends[3] - right).abs() < 0.5, "{} vs {right}", ends[3]);
    }

    #[test]
    fn a_section_starts_a_new_line() {
        let (mut project, channel) = song(6);
        project.add_section(Section::new("A", 0, 32)).unwrap();
        project.add_section(Section::new("B", 32, 64)).unwrap();
        let sheet = sheet(&project, Some(channel));
        // Bars 1-2, then 3-6.
        assert_eq!(glyph_count(&sheet, glyphs::G_CLEF), 2);
        let tops: Vec<f32> = sheet.bars.iter().map(|bar| bar.top_left.y).collect();
        assert_eq!(tops[0], tops[1]);
        assert!(tops[2] > tops[1]);
        // The short first line is not stretched to the margin.
        assert!(sheet.bars[1].bottom_right.x < sheet.pages[0].width - MARGIN - 50.0);
    }

    #[test]
    fn chords_alone_are_written_over_slashes() {
        let (project, _) = song(4);
        let sheet = sheet(&project, None);
        assert_eq!(glyph_count(&sheet, glyphs::SLASH), 16);
        assert_eq!(glyph_count(&sheet, glyphs::NOTEHEAD_BLACK), 0);
    }

    #[test]
    fn chord_symbols_do_not_run_into_each_other() {
        let (mut project, channel) = song(1);
        let long = HarmonyName::parse("C#m7b5/G").unwrap();
        let chords: Vec<ChordMarker> = (0..4)
            .map(|beat| ChordMarker::new(beat * 4, long))
            .collect();
        assert!(project.set_chords(0, 16, &chords));
        let sheet = sheet(&project, Some(channel));
        let roots: Vec<f32> = sheet.pages[0]
            .marks
            .iter()
            .filter_map(|mark| match mark {
                Mark::Text { text, at, size, .. } if text == "C" && *size == CHORD_SIZE => {
                    Some(at.x)
                }
                _ => None,
            })
            .collect();
        assert_eq!(roots.len(), 4);
        let width: f32 = chord_runs(long)
            .iter()
            .map(|run| run_width(run, &Even))
            .sum();
        for pair in roots.windows(2) {
            assert!(pair[1] - pair[0] >= width, "{roots:?}, {width}");
        }
    }

    #[test]
    fn chord_symbols_use_real_accidentals() {
        let runs = chord_runs(HarmonyName::parse("Bbm7b5/Ab").unwrap());
        let small = CHORD_SIZE * 0.8;
        assert_eq!(
            runs,
            [
                Run::Text("B".into(), CHORD_SIZE),
                Run::Accidental(-1, CHORD_SIZE),
                Run::Text("m7".into(), small),
                Run::Accidental(-1, small),
                Run::Text("5".into(), small),
                Run::Text("/".into(), CHORD_SIZE),
                Run::Text("A".into(), CHORD_SIZE),
                Run::Accidental(-1, CHORD_SIZE),
            ]
        );
        assert_eq!(
            chord_runs(HarmonyName::NoChord),
            [Run::Text("N.C.".into(), CHORD_SIZE * 0.75)]
        );
    }

    #[test]
    fn eighths_are_beamed_and_tied_notes_get_ties() {
        let mut project = Project::empty();
        let channel = crate::library::apply::synth_named(&mut project, "Lead").unwrap();
        let pattern = project.pattern_mut(0).unwrap();
        pattern.set_length(32);
        // Eight eighths, then a note held over the bar line.
        let mut notes: Vec<NoteEvent> = (0..8)
            .map(|i| NoteEvent::new(Note::new(67 + (i % 3) as u8), i * 2, 2))
            .collect();
        notes.push(NoteEvent::new(Note::new(72), 16, 4));
        notes.push(NoteEvent::new(Note::new(74), 20, 12));
        pattern.set_notes(channel, notes);
        project.add_clip(Clip::new(0, 0, 0));
        let sheet = sheet(&project, Some(channel));
        let count =
            |kind: fn(&Mark) -> bool| sheet.pages[0].marks.iter().filter(|m| kind(m)).count();
        assert_eq!(count(|m| matches!(m, Mark::Beam { .. })), 4);
        assert_eq!(glyph_count(&sheet, glyphs::FLAG_8TH_UP), 0);
        // D5 from beat 2 to the end: a quarter tied to a half.
        assert_eq!(count(|m| matches!(m, Mark::Tie { .. })), 1);
    }

    #[test]
    fn repeats_and_endings_are_drawn() {
        let (mut project, channel) = song(4);
        let mut section = Section::new("Chorus", 0, 64);
        section.repeats = 3;
        project.add_section(section).unwrap();
        let sheet = sheet(&project, Some(channel));
        // Two dots at the start, two at the end.
        assert_eq!(glyph_count(&sheet, glyphs::REPEAT_DOT), 4);
        let texts: Vec<&str> = sheet.pages[0]
            .marks
            .iter()
            .filter_map(|mark| match mark {
                Mark::Text { text, .. } => Some(text.as_str()),
                _ => None,
            })
            .collect();
        assert!(texts.contains(&"Chorus"));
        assert!(texts.contains(&"×3"));
        assert!(texts.contains(&"Test"));
    }

    #[test]
    fn a_part_says_who_reads_it() {
        let (project, channel) = song(4);
        let part = |title: &str, transposition| {
            let options = Options {
                title: title.to_string(),
                melody: Some(channel),
                transposition,
                ..options()
            };
            let score = score::build(
                &project,
                options.melody,
                options.clef,
                options.transposition,
            );
            lay_out(&score, &options, &Even)
        };
        let texts = |sheet: &LeadSheet| -> Vec<(String, f32)> {
            sheet.pages[0]
                .marks
                .iter()
                .filter_map(|mark| match mark {
                    Mark::Text { text, at, .. } => Some((text.clone(), at.y)),
                    _ => None,
                })
                .collect()
        };
        let has = |sheet: &LeadSheet, word: &str| texts(sheet).iter().any(|(text, _)| text == word);
        // C Am7 Dm7 G7 at concert pitch, with no words but the title.
        let concert = part("Test", Transposition::Concert);
        assert!(!has(&concert, " instruments"));
        assert!(has(&concert, "C") && has(&concert, "G"));
        assert_eq!(glyph_count(&concert, glyphs::CHORD_FLAT), 0);
        // For B-flat instruments, "B♭ instruments" at the top (the only
        // flat: the chords are D Bm7 Em7 A7, a whole tone up).
        let trumpet = part("Test", Transposition::BFlat);
        assert!(has(&trumpet, "B") && has(&trumpet, " instruments"));
        assert_eq!(glyph_count(&trumpet, glyphs::CHORD_FLAT), 1);
        assert!(has(&trumpet, "D") && has(&trumpet, "E") && has(&trumpet, "A"));
        assert!(!has(&trumpet, "C") && !has(&trumpet, "G"));
        // The label sits on the title's line, or under a title too wide
        // to leave it room.
        let y_of = |sheet: &LeadSheet, word: &str| {
            texts(sheet)
                .into_iter()
                .find(|(text, _)| text == word)
                .map(|(_, y)| y)
                .unwrap()
        };
        assert_eq!(y_of(&trumpet, " instruments"), y_of(&trumpet, "Test"));
        let long = "A title long enough to fill the line from side to side";
        let wide = part(long, Transposition::EFlat);
        assert!(y_of(&wide, " instruments") > y_of(&wide, long));
        assert!(has(&wide, "E"));
    }

    #[test]
    fn a_tie_is_a_closed_crescent() {
        let [outer, inner] =
            Mark::tie_curves(Point::new(0.0, 0.0), Point::new(20.0, 0.0), true, 1.0);
        assert_eq!(outer[0], inner[3]);
        assert_eq!(outer[3], inner[0]);
        // Bulging up: the control points are above, the outer more so.
        assert!(outer[1].y < inner[2].y && inner[2].y < 0.0);
    }
}
