//! Voicing: choosing which notes, in which octaves, play a chord.
//!
//! "C major" says C, E and G, but not where. A pianist playing C, then F,
//! then G does not jump up and down the keyboard: from C E G they move
//! to C F A, then to B D G, each finger going to the nearest note of the
//! next chord. That is **voice leading**, and it is most of what makes a
//! progression sound smooth instead of mechanical. This module does the
//! same, by trying every inversion of the next chord and keeping the one
//! closest to the last.

use crate::harmony::Chord;
use crate::pitch::{Note, PitchClass};

/// The lowest note a chord voicing may start on: F3.
const LOWEST: u8 = 53;

/// Where the first chord of a progression is centred: around D#4.
const FIRST_CENTRE: f32 = 63.0;

/// The bass plays the root of each chord between these notes: C2 to B2.
const BASS_LOWEST: u8 = 36;

/// How low a bass line may step down: E1, the lowest string of a bass
/// guitar.
const BASS_FLOOR: u8 = 28;

/// How high a bass line may step up: D3.
const BASS_CEILING: u8 = 50;

/// The most notes a voicing keeps. Bigger chords drop notes, the way a
/// pianist does: first the fifth, which adds little colour, then the
/// root, which the bass is already playing.
const MAX_VOICES: usize = 4;

/// The pitch classes a voicing plays, as semitones above the root (0 to
/// 11), after dropping the notes a pianist would leave out.
fn voiced_tones(chord: Chord) -> Vec<u8> {
    let mut tones: Vec<u8> = Vec::new();
    for &interval in chord.quality.intervals() {
        let tone = interval % 12;
        if !tones.contains(&tone) {
            tones.push(tone);
        }
    }
    for drop in [7, 0] {
        if tones.len() > MAX_VOICES {
            tones.retain(|&tone| tone != drop);
        }
    }
    tones
}

/// Every inversion of the chord, in close position (all notes within an
/// octave), with its lowest note between F3 and E4.
fn inversions(chord: Chord) -> Vec<Vec<Note>> {
    let root = chord.root.semitone();
    let tones = voiced_tones(chord);
    tones
        .iter()
        .map(|&bottom| {
            // The bottom note's pitch class, placed in the octave from F3.
            let bottom_class = (root + bottom) % 12;
            let lowest = LOWEST + (bottom_class + 12 - LOWEST % 12) % 12;
            let mut notes: Vec<Note> = tones
                .iter()
                .map(|&tone| {
                    let above_bottom = (tone + 12 - bottom) % 12;
                    Note::new(lowest + above_bottom)
                })
                .collect();
            notes.sort();
            notes
        })
        .collect()
}

/// How far apart two voicings are: for each note of one, the distance to
/// the nearest note of the other, added up both ways.
fn distance(a: &[Note], b: &[Note]) -> u32 {
    let nearest = |note: Note, others: &[Note]| {
        others
            .iter()
            .map(|other| note.midi().abs_diff(other.midi()) as u32)
            .min()
            .unwrap_or(0)
    };
    let there: u32 = a.iter().map(|&note| nearest(note, b)).sum();
    let back: u32 = b.iter().map(|&note| nearest(note, a)).sum();
    there + back
}

/// The voicing of `chord` that moves least from `previous`, or, for the
/// first chord, the one closest to the middle of the keyboard.
pub fn voice(chord: Chord, previous: Option<&[Note]>) -> Vec<Note> {
    let candidates = inversions(chord);
    let centre = |notes: &Vec<Note>| {
        let sum: f32 = notes.iter().map(|note| f32::from(note.midi())).sum();
        sum / notes.len().max(1) as f32
    };
    let best = match previous {
        Some(previous) => candidates
            .into_iter()
            .min_by_key(|notes| distance(notes, previous)),
        None => candidates.into_iter().min_by(|a, b| {
            let a = (centre(a) - FIRST_CENTRE).abs();
            let b = (centre(b) - FIRST_CENTRE).abs();
            a.total_cmp(&b)
        }),
    };
    best.unwrap_or_default()
}

/// Voices a whole progression, each chord led smoothly from the last.
pub fn voice_progression(chords: &[Chord]) -> Vec<Vec<Note>> {
    let mut voicings: Vec<Vec<Note>> = Vec::with_capacity(chords.len());
    for &chord in chords {
        let voicing = voice(chord, voicings.last().map(Vec::as_slice));
        voicings.push(voicing);
    }
    voicings
}

/// The root of `chord` in the bass, between C2 and B2.
pub fn bass_root(chord: Chord) -> Note {
    Note::new(BASS_LOWEST + chord.root.semitone())
}

/// The note under each chord of a progression. Each entry of `basses`
/// is a chord's lowest note (its root, or the bass of a slash chord)
/// and whether it is a slash chord's bass.
///
/// A root is played between C2 and B2, as [`bass_root`] does. Slash
/// chords are mostly written for the line their bass notes make: C G/B
/// Am Am/G F walks down C, B, A, G, F. So the bass of a slash chord, and
/// the chord after one, go the short way when they are a step (one or
/// two semitones) from the note before, down to E1. Otherwise they are
/// played between C2 and B2 too, and a progression with no slash chords
/// keeps every root there.
pub fn bass_line(basses: &[(PitchClass, bool)]) -> Vec<Note> {
    let mut line: Vec<Note> = Vec::with_capacity(basses.len());
    let mut slash_before = false;
    for &(pitch_class, slash) in basses {
        let usual = Note::new(BASS_LOWEST + pitch_class.semitone());
        let stepped = match line.last() {
            Some(&before) if slash || slash_before => step_to(before, pitch_class),
            _ => None,
        };
        line.push(stepped.unwrap_or(usual));
        slash_before = slash;
    }
    line
}

/// The note of `pitch_class` at most two semitones from `before`, if
/// there is one between E1 and D3.
fn step_to(before: Note, pitch_class: PitchClass) -> Option<Note> {
    let up = before.pitch_class().semitones_up_to(pitch_class);
    let note = if up <= 2 {
        before.transpose(i32::from(up))
    } else if up >= 10 {
        before.transpose(i32::from(up) - 12)
    } else {
        None
    }?;
    (BASS_FLOOR..=BASS_CEILING)
        .contains(&note.midi())
        .then_some(note)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harmony::ChordQuality::*;
    use crate::pitch::PitchClass::*;

    fn midi(notes: &[Note]) -> Vec<u8> {
        notes.iter().map(|note| note.midi()).collect()
    }

    #[test]
    fn without_slash_chords_the_bass_plays_every_root_from_c2() {
        let roots: Vec<(PitchClass, bool)> = [C, B, A, G, F, E, D, ASharp]
            .into_iter()
            .map(|root| (root, false))
            .collect();
        let line = bass_line(&roots);
        let expected: Vec<Note> = roots
            .iter()
            .map(|&(root, _)| bass_root(Chord::new(root, Major)))
            .collect();
        assert_eq!(line, expected);
    }

    #[test]
    fn slash_chords_make_the_bass_walk_by_step() {
        // C G/B Am Am/G F C/E Dm G: down by step to E1, then up an
        // octave, since D1 is below a bass guitar's lowest string.
        let basses = [
            (C, false),
            (B, true),
            (A, false),
            (G, true),
            (F, false),
            (E, true),
            (D, false),
            (G, false),
        ];
        assert_eq!(midi(&bass_line(&basses)), [36, 35, 33, 31, 29, 28, 38, 43]);
        // A bass a leap away is played where the roots are: F/C is a C
        // under F, at C2.
        assert_eq!(midi(&bass_line(&[(G, false), (C, true)])), [43, 36]);
        // Stepping up from a high root goes past B2.
        assert_eq!(midi(&bass_line(&[(B, false), (C, true)])), [47, 48]);
    }

    #[test]
    fn big_chords_drop_the_fifth_then_the_root() {
        let c9 = Chord::new(C, Dominant9); // C E G Bb D
        assert_eq!(voiced_tones(c9), [0, 4, 10, 2]);
        let c13 = Chord::new(C, Dominant13); // C E G Bb D A
        assert_eq!(voiced_tones(c13), [4, 10, 2, 9]);
        assert_eq!(voiced_tones(Chord::new(C, Major)), [0, 4, 7]);
    }

    #[test]
    fn every_inversion_stays_in_its_octave() {
        for notes in inversions(Chord::new(A, Minor7)) {
            assert!(notes[0].midi() >= LOWEST && notes[0].midi() < LOWEST + 12);
            assert!(notes.last().unwrap().midi() - notes[0].midi() < 12);
        }
    }

    #[test]
    fn chords_move_to_the_nearest_notes() {
        // C, F, G, C in C major: the classic "keyboard" voicings.
        let chords = [
            Chord::new(C, Major),
            Chord::new(F, Major),
            Chord::new(G, Major),
            Chord::new(C, Major),
        ];
        let voicings = voice_progression(&chords);
        assert_eq!(midi(&voicings[0]), [60, 64, 67]); // C4 E4 G4
        assert_eq!(midi(&voicings[1]), [60, 65, 69]); // C4 F4 A4
        assert_eq!(midi(&voicings[2]), [59, 62, 67]); // B3 D4 G4
        assert_eq!(midi(&voicings[3]), [60, 64, 67]);
    }

    #[test]
    fn the_bass_plays_the_root_low() {
        assert_eq!(bass_root(Chord::new(C, Major)).midi(), 36);
        assert_eq!(bass_root(Chord::new(B, Minor)).midi(), 47);
    }
}
