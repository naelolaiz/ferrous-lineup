//! The music symbols a lead sheet is drawn with, and how big they are.
//!
//! Symbols come from a music font that follows SMuFL, the Standard Music
//! Font Layout: every symbol has a fixed place in Unicode's private use
//! area (the treble clef is U+E050 in every SMuFL font), and the font is
//! drawn so that one em is the height of a five-line staff, four staff
//! spaces. The app uses Bravura, the font SMuFL was designed with.
//!
//! The layout only needs to know how wide each symbol is and how far it
//! reaches up and down. Those numbers are copied from the metadata file
//! that comes with Bravura (`bravura_metadata.json`), in staff spaces,
//! with y going *up* as SMuFL counts it.

/// A symbol of the music font: its SMuFL code point, its advance width,
/// and its bounding box (bottom and top), in staff spaces.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Glyph {
    pub code: char,
    pub width: f32,
    pub bottom: f32,
    pub top: f32,
}

const fn glyph(code: char, width: f32, bottom: f32, top: f32) -> Glyph {
    Glyph {
        code,
        width,
        bottom,
        top,
    }
}

pub const G_CLEF: Glyph = glyph('\u{E050}', 2.684, -2.632, 4.392);
pub const F_CLEF: Glyph = glyph('\u{E062}', 2.736, -2.54, 1.048);

pub const NOTEHEAD_WHOLE: Glyph = glyph('\u{E0A2}', 1.688, -0.5, 0.5);
pub const NOTEHEAD_HALF: Glyph = glyph('\u{E0A3}', 1.18, -0.5, 0.5);
pub const NOTEHEAD_BLACK: Glyph = glyph('\u{E0A4}', 1.18, -0.5, 0.5);
/// A rhythm slash, for a bar with chords but no melody.
pub const SLASH: Glyph = glyph('\u{E101}', 2.124, -1.0, 1.0);

pub const FLAT: Glyph = glyph('\u{E260}', 0.904, -0.7, 1.756);
pub const NATURAL: Glyph = glyph('\u{E261}', 0.672, -1.34, 1.364);
pub const SHARP: Glyph = glyph('\u{E262}', 0.996, -1.392, 1.4);
pub const DOUBLE_SHARP: Glyph = glyph('\u{E263}', 1.0, -0.5, 0.508);
pub const DOUBLE_FLAT: Glyph = glyph('\u{E264}', 1.652, -0.7, 1.748);

pub const FLAG_8TH_UP: Glyph = glyph('\u{E240}', 1.056, -3.24, 0.036);
pub const FLAG_8TH_DOWN: Glyph = glyph('\u{E241}', 1.224, -0.056, 3.232);
pub const FLAG_16TH_UP: Glyph = glyph('\u{E242}', 1.116, -3.252, 0.008);
pub const FLAG_16TH_DOWN: Glyph = glyph('\u{E243}', 1.168, -0.036, 3.248);

pub const REST_WHOLE: Glyph = glyph('\u{E4E3}', 1.132, -0.54, 0.036);
pub const REST_HALF: Glyph = glyph('\u{E4E4}', 1.132, -0.008, 0.568);
pub const REST_QUARTER: Glyph = glyph('\u{E4E5}', 1.08, -1.5, 1.492);
pub const REST_8TH: Glyph = glyph('\u{E4E6}', 1.0, -1.004, 0.696);
pub const REST_16TH: Glyph = glyph('\u{E4E7}', 1.28, -2.0, 0.716);

pub const DOT: Glyph = glyph('\u{E1E7}', 0.4, -0.2, 0.2);
pub const REPEAT_DOT: Glyph = glyph('\u{E044}', 0.4, -0.2, 0.2);

/// The digits of a time signature, 0 to 9.
pub const TIME_DIGITS: [Glyph; 10] = [
    glyph('\u{E080}', 1.88, -1.0, 1.004),
    glyph('\u{E081}', 1.336, -1.0, 1.004),
    glyph('\u{E082}', 1.784, -1.028, 1.016),
    glyph('\u{E083}', 1.684, -1.004, 0.996),
    glyph('\u{E084}', 1.88, -1.0, 1.004),
    glyph('\u{E085}', 1.612, -1.004, 0.984),
    glyph('\u{E086}', 1.736, -0.996, 1.004),
    glyph('\u{E087}', 1.764, -1.0, 0.996),
    glyph('\u{E088}', 1.744, -1.036, 1.036),
    glyph('\u{E089}', 1.736, -0.996, 1.004),
];
/// The "+" between the groups of a time signature such as 2+2+3/8.
pub const TIME_PLUS: Glyph = glyph('\u{E08C}', 2.0, -1.0, 1.0);

/// The quarter note and dot of a metronome mark, "♩ = 120".
pub const METRONOME_QUARTER: Glyph = glyph('\u{ECA5}', 1.328, -0.564, 2.752);
pub const METRONOME_DOT: Glyph = glyph('\u{ECB7}', 0.4, -0.2, 0.2);

/// Accidentals drawn to sit among the letters of a chord symbol: "B♭m7".
/// They are three staff spaces tall, as tall as a capital letter of
/// text whose size is about a staff height.
pub const CHORD_FLAT: Glyph = glyph('\u{ED60}', 1.136, -0.004, 3.0);
pub const CHORD_NATURAL: Glyph = glyph('\u{ED61}', 0.736, 0.0, 3.004);
pub const CHORD_SHARP: Glyph = glyph('\u{ED62}', 1.24, -0.052, 3.048);
pub const CHORD_DOUBLE_SHARP: Glyph = glyph('\u{ED63}', 1.776, 0.0, 1.796);
pub const CHORD_DOUBLE_FLAT: Glyph = glyph('\u{ED64}', 2.096, -0.004, 3.0);

/// Every symbol a lead sheet uses, to check them against the font.
pub const ALL: [Glyph; 40] = [
    G_CLEF,
    F_CLEF,
    NOTEHEAD_WHOLE,
    NOTEHEAD_HALF,
    NOTEHEAD_BLACK,
    SLASH,
    FLAT,
    NATURAL,
    SHARP,
    DOUBLE_SHARP,
    DOUBLE_FLAT,
    FLAG_8TH_UP,
    FLAG_8TH_DOWN,
    FLAG_16TH_UP,
    FLAG_16TH_DOWN,
    REST_WHOLE,
    REST_HALF,
    REST_QUARTER,
    REST_8TH,
    REST_16TH,
    DOT,
    REPEAT_DOT,
    TIME_DIGITS[0],
    TIME_DIGITS[1],
    TIME_DIGITS[2],
    TIME_DIGITS[3],
    TIME_DIGITS[4],
    TIME_DIGITS[5],
    TIME_DIGITS[6],
    TIME_DIGITS[7],
    TIME_DIGITS[8],
    TIME_DIGITS[9],
    TIME_PLUS,
    METRONOME_QUARTER,
    METRONOME_DOT,
    CHORD_FLAT,
    CHORD_NATURAL,
    CHORD_SHARP,
    CHORD_DOUBLE_SHARP,
    CHORD_DOUBLE_FLAT,
];

/// The accidental drawn before a note: -2 to 2 flats or sharps, 0 for a
/// natural sign.
pub fn accidental(accidental: i8) -> Glyph {
    match accidental {
        i8::MIN..=-2 => DOUBLE_FLAT,
        -1 => FLAT,
        0 => NATURAL,
        1 => SHARP,
        _ => DOUBLE_SHARP,
    }
}

/// The accidental written in a chord symbol.
pub fn chord_accidental(accidental: i8) -> Glyph {
    match accidental {
        i8::MIN..=-2 => CHORD_DOUBLE_FLAT,
        -1 => CHORD_FLAT,
        0 => CHORD_NATURAL,
        1 => CHORD_SHARP,
        _ => CHORD_DOUBLE_SHARP,
    }
}

/// Thicknesses and distances, in staff spaces, from Bravura's
/// "engraving defaults".
pub mod engraving {
    pub const STAFF_LINE: f32 = 0.13;
    pub const STEM: f32 = 0.12;
    pub const BEAM: f32 = 0.5;
    /// The gap between two beams.
    pub const BEAM_SPACING: f32 = 0.25;
    pub const LEGER_LINE: f32 = 0.16;
    /// How far a leger line reaches out on each side of the notehead.
    pub const LEGER_EXTENSION: f32 = 0.4;
    pub const THIN_BARLINE: f32 = 0.16;
    pub const THICK_BARLINE: f32 = 0.5;
    /// The gap between the two lines of a double or final bar line.
    pub const BARLINE_SEPARATION: f32 = 0.4;
    /// The gap between a repeat's dots and its thin line.
    pub const REPEAT_DOT_SEPARATION: f32 = 0.16;
    pub const TIE_MIDDLE: f32 = 0.22;
    pub const TIE_ENDS: f32 = 0.1;
    pub const ENCLOSURE: f32 = 0.16;
    /// Where a stem joins a notehead: on its right edge going up, on
    /// its left edge going down, this far from its middle.
    pub const STEM_ATTACH: f32 = 0.168;
}
