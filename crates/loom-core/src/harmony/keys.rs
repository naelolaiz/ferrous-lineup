//! The harmony keys: one big key for each chord of the key you are in.
//!
//! A major key has seven notes, and a chord can be built on each of them
//! by stacking every other note of the scale: in C major, C E G is the
//! chord on the first note, D F A on the second, and so on. Musicians
//! number these chords with Roman numerals, I to vii°, so that "I IV V"
//! means the same progression in every key.
//!
//! The seven chords do three jobs, called their **function**:
//!
//! - **Tonic** (I, iii, vi): home. The music can rest here.
//! - **Subdominant** (ii, IV): leaving home. They lead on to the dominant.
//! - **Dominant** (V, vii°): tension. They pull back home to I.
//!
//! Most songs are these three jobs in turn: home, away, tension, home.
//! The harmony keys are coloured by function, so you can see the pull.

use super::{Chord, Scale, ScaleKind};
use crate::library::voicing::voice;
use crate::pitch::Note;

/// How many notes of the scale each chord stacks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChordSize {
    /// Three notes: C E G.
    #[default]
    Triads,
    /// Four notes, adding the seventh: C E G B, "Cmaj7".
    Sevenths,
    /// Five notes, adding the ninth: C E G B D, "Cmaj9". Where the ninth
    /// would clash (a minor ninth above the root), the seventh chord is
    /// played instead, as a pianist would.
    Ninths,
}

impl ChordSize {
    pub const ALL: [ChordSize; 3] = [ChordSize::Triads, ChordSize::Sevenths, ChordSize::Ninths];

    pub fn label(self) -> &'static str {
        match self {
            ChordSize::Triads => "Triads",
            ChordSize::Sevenths => "7ths",
            ChordSize::Ninths => "9ths",
        }
    }
}

/// The job a chord does in its key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Function {
    Tonic,
    Subdominant,
    Dominant,
}

impl Function {
    /// The function of the chord on `degree` (0 for I, 6 for vii).
    pub fn of(degree: usize) -> Function {
        match degree % 7 {
            1 | 3 => Function::Subdominant,
            4 | 6 => Function::Dominant,
            _ => Function::Tonic,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Function::Tonic => "tonic",
            Function::Subdominant => "subdominant",
            Function::Dominant => "dominant",
        }
    }

    /// What the function feels like, in a few words.
    pub fn feel(self) -> &'static str {
        match self {
            Function::Tonic => "home: the music can rest here",
            Function::Subdominant => "away from home: leads on to the dominant",
            Function::Dominant => "tension: pulls back home to I",
        }
    }
}

/// One of the harmony keys.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChordKey {
    /// The degree of the scale the chord is built on, from 0 (I) to 6 (vii).
    pub degree: usize,
    pub chord: Chord,
    /// The chord as a Roman numeral, such as "V7" or "vii°".
    pub numeral: String,
    pub function: Function,
}

/// The seven-note scale the harmony keys take their chords from.
///
/// It is `scale` itself if it has seven notes. Pentatonic, blues and
/// chromatic scales have no "chords of the key" of their own, so the
/// keys use the major or minor scale with the same root.
pub fn chord_scale(scale: Scale) -> Scale {
    if scale.is_heptatonic() {
        return scale;
    }
    let kind = match scale.kind {
        ScaleKind::MinorPentatonic | ScaleKind::Blues => ScaleKind::NaturalMinor,
        _ => ScaleKind::Major,
    };
    Scale::new(scale.root, kind)
}

/// The chord of `size` on `degree` of a seven-note `scale`.
fn chord_on(scale: Scale, degree: usize, size: ChordSize) -> Option<Chord> {
    match size {
        ChordSize::Triads => scale.triad(degree),
        ChordSize::Sevenths => scale.seventh(degree),
        ChordSize::Ninths => scale.ninth(degree).or_else(|| scale.seventh(degree)),
    }
}

/// The seven harmony keys for `scale`, from I to vii.
///
/// In a minor key the chord on V is major (with a major seventh chord,
/// V7), as in most music in minor keys: its raised third is the "leading
/// tone" that pulls up to the tonic. That is the harmonic minor scale's V.
pub fn chord_keys(scale: Scale, size: ChordSize) -> Vec<ChordKey> {
    let scale = chord_scale(scale);
    let leading_tone = Scale::new(scale.root, ScaleKind::HarmonicMinor);
    (0..7)
        .filter_map(|degree| {
            let from = if scale.kind == ScaleKind::NaturalMinor && degree == 4 {
                leading_tone
            } else {
                scale
            };
            let chord = chord_on(from, degree, size)?;
            Some(ChordKey {
                degree,
                chord,
                numeral: scale.roman_numeral(chord)?,
                function: Function::of(degree),
            })
        })
        .collect()
}

/// The notes a harmony key plays: the chord voiced close to `previous`
/// (the voicing the last key played, if any), so that going from key to
/// key moves the fingers as little as possible.
///
/// Returns the voicing, to pass as `previous` next time, and the notes to
/// play: the voicing with the root added below it, as a left hand would.
pub fn play(chord: Chord, previous: Option<&[Note]>) -> (Vec<Note>, Vec<Note>) {
    let voicing = voice(chord, previous);
    let mut notes = voicing.clone();
    if let Some(&lowest) = voicing.first() {
        // The root below the lowest note: an octave below if the lowest
        // note is the root itself.
        let down = match chord.root.semitones_up_to(lowest.pitch_class()) {
            0 => 12,
            gap => gap,
        };
        if let Some(root) = lowest.transpose(-i32::from(down)) {
            notes.insert(0, root);
        }
    }
    (voicing, notes)
}

/// The harmony key of a chord played elsewhere, if it is one of the
/// key's chords: the same root, and major or minor like the key's chord
/// there. G7 lights V in C major; D7 lights nothing, because the key's
/// chord on D is minor: D7 comes from outside the key.
pub fn key_for(keys: &[ChordKey], chord: Chord) -> Option<usize> {
    keys.iter().position(|key| {
        key.chord.root == chord.root && key.chord.quality.is_minor() == chord.quality.is_minor()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ChordQuality, PitchClass, Spelling};

    fn names(scale: Scale, size: ChordSize) -> Vec<String> {
        chord_keys(scale, size)
            .iter()
            .map(|key| format!("{} {}", key.numeral, key.chord.name(Spelling::Sharps)))
            .collect()
    }

    #[test]
    fn a_major_key_has_seven_chords() {
        let c = Scale::new(PitchClass::C, ScaleKind::Major);
        assert_eq!(
            names(c, ChordSize::Triads),
            [
                "I C",
                "ii Dm",
                "iii Em",
                "IV F",
                "V G",
                "vi Am",
                "vii° Bdim"
            ]
        );
        assert_eq!(
            names(c, ChordSize::Sevenths),
            [
                "Imaj7 Cmaj7",
                "ii7 Dm7",
                "iii7 Em7",
                "IVmaj7 Fmaj7",
                "V7 G7",
                "vi7 Am7",
                "viiø7 Bm7b5"
            ]
        );
    }

    #[test]
    fn ninths_fall_back_to_sevenths_where_they_clash() {
        let c = Scale::new(PitchClass::C, ScaleKind::Major);
        let chords: Vec<String> = chord_keys(c, ChordSize::Ninths)
            .iter()
            .map(|key| key.chord.name(Spelling::Sharps))
            .collect();
        assert_eq!(
            chords,
            ["Cmaj9", "Dm9", "Em7", "Fmaj9", "G9", "Am9", "Bm7b5"]
        );
    }

    #[test]
    fn a_minor_key_has_a_major_five() {
        let a = Scale::new(PitchClass::A, ScaleKind::NaturalMinor);
        assert_eq!(
            names(a, ChordSize::Triads),
            ["i Am", "ii° Bdim", "III C", "iv Dm", "V E", "VI F", "VII G"]
        );
        assert_eq!(names(a, ChordSize::Sevenths)[4], "V7 E7");
    }

    #[test]
    fn scales_without_seven_notes_borrow_major_or_minor() {
        let blues = Scale::new(PitchClass::E, ScaleKind::Blues);
        assert_eq!(
            chord_scale(blues),
            Scale::new(PitchClass::E, ScaleKind::NaturalMinor)
        );
        let pentatonic = Scale::new(PitchClass::G, ScaleKind::MajorPentatonic);
        assert_eq!(
            chord_keys(pentatonic, ChordSize::Triads)[0].chord.root,
            PitchClass::G
        );
        assert_eq!(
            chord_keys(
                Scale::new(PitchClass::D, ScaleKind::Chromatic),
                ChordSize::Triads
            )
            .len(),
            7
        );
    }

    #[test]
    fn functions() {
        let functions: Vec<Function> = (0..7).map(Function::of).collect();
        use Function::*;
        assert_eq!(
            functions,
            [
                Tonic,
                Subdominant,
                Tonic,
                Subdominant,
                Dominant,
                Tonic,
                Dominant
            ]
        );
    }

    #[test]
    fn keys_play_the_root_below_a_led_voicing() {
        let c = Scale::new(PitchClass::C, ScaleKind::Major);
        let keys = chord_keys(c, ChordSize::Triads);
        let (voicing, notes) = play(keys[0].chord, None);
        assert_eq!(notes.len(), voicing.len() + 1);
        let root = notes[0];
        assert_eq!(root.pitch_class(), PitchClass::C);
        assert!(root < voicing[0] && voicing[0].midi() - root.midi() <= 12);

        // From C to F, the voices move by a step or two at most.
        let (next, _) = play(keys[3].chord, Some(&voicing));
        let moved: u8 = voicing
            .iter()
            .zip(&next)
            .map(|(a, b)| a.midi().abs_diff(b.midi()))
            .sum();
        assert!(moved <= 4, "{voicing:?} to {next:?}");
    }

    #[test]
    fn a_chord_played_elsewhere_finds_its_key() {
        let c = Scale::new(PitchClass::C, ScaleKind::Major);
        let keys = chord_keys(c, ChordSize::Triads);
        let g7 = Chord::new(PitchClass::G, ChordQuality::Dominant7);
        assert_eq!(key_for(&keys, g7), Some(4));
        let d7 = Chord::new(PitchClass::D, ChordQuality::Dominant7);
        assert_eq!(key_for(&keys, d7), None, "a secondary dominant");
        assert_eq!(
            key_for(&keys, Chord::new(PitchClass::CSharp, ChordQuality::Major)),
            None
        );
    }
}
