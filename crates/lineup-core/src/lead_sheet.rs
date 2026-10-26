//! Lead sheets: the tune in notation on a staff, with the chord symbols
//! above it, laid out on pages ready to show or print.
//!
//! A lead sheet is what a band reads from: one staff with the melody,
//! the harmony written over it as chord symbols ("Am7", "G/B"), and the
//! song's sections, repeats and key and time signatures. Everything here
//! is read from the song as it is: the melody from the notes of one
//! channel, the chords from the chord sheet's markers. Nothing is
//! written back.
//!
//! The work is split in steps, each in its own module:
//!
//! - [`melody`] picks the tune out of a channel's notes, one note at a
//!   time, and rounds it to the grid;
//! - `spelling` decides how each note is written (A# or Bb?) from the
//!   key and the chord, and which accidentals a bar needs;
//! - `rhythm` writes each length as note values the eye can count,
//!   tied across beats where needed, and groups eighths under beams;
//! - [`transpose`] moves the key, the notes and the chords for players
//!   of B♭, E♭ and F instruments, who read in keys of their own;
//! - `score` puts these together, bar by bar;
//! - `layout` places everything on pages, as a list of [`Mark`]s: music
//!   symbols, text, lines, beams and ties, in points.
//!
//! The app draws the marks on screen and writes the same marks to a PDF,
//! so what is printed is exactly what is shown.

pub mod glyphs;
mod layout;
pub mod melody;
mod rhythm;
mod score;
mod spelling;
pub mod transpose;

pub use melody::{MelodyNote, default_melody_channel, melody_channels};
pub use spelling::Clef;
pub use transpose::Transposition;

use serde::{Deserialize, Serialize};

use crate::project::Project;

/// A sheet of paper, in points (1/72 inch), portrait.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Paper {
    /// 210 × 297 mm, used almost everywhere.
    #[default]
    A4,
    /// 8.5 × 11 inches, used in the USA and Canada.
    Letter,
}

impl Paper {
    pub const ALL: [Paper; 2] = [Paper::A4, Paper::Letter];

    /// Width and height in points.
    pub fn size(self) -> (f32, f32) {
        match self {
            Paper::A4 => (595.28, 841.89),
            Paper::Letter => (612.0, 792.0),
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Paper::A4 => "A4",
            Paper::Letter => "Letter",
        }
    }
}

/// What to write and how.
#[derive(Debug, Clone, PartialEq)]
pub struct Options {
    /// Printed at the top of the first page.
    pub title: String,
    /// The channel whose notes are the melody, or `None` to write only
    /// the chords, over rhythm slashes.
    pub melody: Option<usize>,
    /// The clef, or `None` to choose it from the melody's notes. Parts
    /// for transposing instruments are in the treble clef unless told.
    pub clef: Option<Clef>,
    pub paper: Paper,
    /// Who reads it: concert pitch, or a B♭, E♭ or F instrument.
    pub transposition: Transposition,
}

/// The two weights of the text font.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextFont {
    Regular,
    SemiBold,
}

/// Measures text, so the layout knows how much room words and chord
/// symbols take. The app measures with the very fonts it draws with.
pub trait TextWidth {
    /// The width of `text` in `font` at `size` points, in points.
    fn text_width(&self, text: &str, font: TextFont, size: f32) -> f32;
}

/// A place on a page, in points from its top left corner.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    fn moved(self, dx: f32, dy: f32) -> Self {
        Point::new(self.x + dx, self.y + dy)
    }
}

/// One thing drawn on a page. All sizes and places are in points, with
/// y going down the page.
#[derive(Debug, Clone, PartialEq)]
pub enum Mark {
    /// A symbol of the music font with its origin (the SMuFL baseline)
    /// at `at`; `size` is the font size, the height of a staff.
    Music {
        glyph: glyphs::Glyph,
        at: Point,
        size: f32,
    },
    /// Text with the left end of its baseline at `at`.
    Text {
        text: String,
        font: TextFont,
        at: Point,
        size: f32,
    },
    /// A straight line `width` thick, with square ends.
    Line { from: Point, to: Point, width: f32 },
    /// A beam: the four-sided shape between the line from `from` to `to`
    /// and the same line moved `thickness` down (up, if negative).
    Beam {
        from: Point,
        to: Point,
        thickness: f32,
    },
    /// A tie between two notes: a thin crescent from `from` to `to`,
    /// bulging up if `up`, otherwise down.
    Tie {
        from: Point,
        to: Point,
        up: bool,
        /// How thick it is in the middle.
        thickness: f32,
    },
}

impl Mark {
    /// The same mark moved by `dx`, `dy`.
    pub fn moved(&self, dx: f32, dy: f32) -> Mark {
        let mut mark = self.clone();
        match &mut mark {
            Mark::Music { at, .. } | Mark::Text { at, .. } => *at = at.moved(dx, dy),
            Mark::Line { from, to, .. }
            | Mark::Beam { from, to, .. }
            | Mark::Tie { from, to, .. } => {
                *from = from.moved(dx, dy);
                *to = to.moved(dx, dy);
            }
        }
        mark
    }

    /// The four corners of a beam, going round.
    pub fn beam_corners(from: Point, to: Point, thickness: f32) -> [Point; 4] {
        [
            from,
            to,
            to.moved(0.0, thickness),
            from.moved(0.0, thickness),
        ]
    }

    /// The outline of a tie as two cubic Bézier curves, each given by
    /// its four points: the outer edge from `from` to `to`, then the
    /// inner edge back. Filling the shape they enclose draws the tie.
    pub fn tie_curves(from: Point, to: Point, up: bool, thickness: f32) -> [[Point; 4]; 2] {
        let length = (to.x - from.x).abs();
        // Short ties are flat, long ones a little higher, as engravers
        // draw them.
        let height = (length * 0.18).clamp(thickness * 2.5, thickness * 9.0);
        let sign = if up { -1.0 } else { 1.0 };
        // A cubic curve reaches 3/4 of the way to its control points.
        let outer = sign * height / 0.75;
        let inner = sign * (height - thickness) / 0.75;
        let along = |part: f32| from.x + (to.x - from.x) * part;
        let at = |part: f32, lift: f32| {
            let y = from.y + (to.y - from.y) * part;
            Point::new(along(part), y + lift)
        };
        [
            [from, at(0.25, outer), at(0.75, outer), to],
            [to, at(0.75, inner), at(0.25, inner), from],
        ]
    }
}

/// One page of a lead sheet.
#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    pub width: f32,
    pub height: f32,
    pub marks: Vec<Mark>,
}

/// Where a bar of the song is on the pages, to light it up while it
/// plays.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BarBox {
    pub page: usize,
    /// The steps of the song it covers, from `start` up to `end`.
    pub start: usize,
    pub end: usize,
    pub top_left: Point,
    pub bottom_right: Point,
}

/// A lead sheet laid out on pages.
#[derive(Debug, Clone, PartialEq)]
pub struct LeadSheet {
    pub pages: Vec<Page>,
    pub bars: Vec<BarBox>,
    /// The clef it is written in, chosen or given.
    pub clef: Clef,
}

/// Writes the lead sheet of `project`.
pub fn lead_sheet(project: &Project, options: &Options, measure: &dyn TextWidth) -> LeadSheet {
    let score = score::build(project, options.melody, options.clef, options.transposition);
    layout::lay_out(&score, options, measure)
}
