//! Chord scales: the notes a soloist plays over a chord.
//!
//! A chord has three or four notes, but a melody over it moves through
//! seven or eight: the chord's own notes and the ones between them. Which
//! notes go between depends on the chord **and the key**: over Dm7 in C
//! major the B is natural, D E F G A B C (D Dorian), because the music
//! is in C. Jazz teaching calls these chord scales. Ferrous Lineup
//! chooses them as a player thinking in the key would:
//!
//! - a chord of the key takes the key's scale, from its own root: G7 in
//!   C major plays G Mixolydian, Dm7 D Dorian, and E7 in A minor (with
//!   the raised seventh) A harmonic minor from E, E Phrygian dominant;
//! - a chord borrowed from the parallel key takes that key's scale: Fm in
//!   C major plays C minor from F, F Dorian;
//! - a secondary dominant takes the scale of the chord it leads to: D7,
//!   the V7 of G in C major, plays G major from D (D Mixolydian), and E7,
//!   the V7 of Am, A harmonic minor from E;
//! - any other chord, and every chord in a key that is not a seven-note
//!   scale, takes the scale its kind usually gets: Lydian for a major
//!   chord from outside the key, Dorian for a minor chord, Mixolydian for
//!   a dominant seventh (Lydian dominant when it comes from outside the
//!   key and leads nowhere), half-whole diminished for 7b9 and 7#9, whole
//!   tone for an augmented chord, Locrian for a half-diminished one.
//!
//! The chord's own notes always win: a note of the scale written with
//! the same letter as a chord note is replaced by it, so E9 over A
//! harmonic minor has F# (its ninth) in place of F.

use super::{Chord, ChordQuality, PitchSet, Scale, ScaleKind};
use crate::PitchClass;

/// The scales chords get by their kind, as semitones above the root.
const IONIAN: &[u8] = &[0, 2, 4, 5, 7, 9, 11];
const LYDIAN: &[u8] = &[0, 2, 4, 6, 7, 9, 11];
const DORIAN: &[u8] = &[0, 2, 3, 5, 7, 9, 10];
const MELODIC_MINOR: &[u8] = &[0, 2, 3, 5, 7, 9, 11];
const MIXOLYDIAN: &[u8] = &[0, 2, 4, 5, 7, 9, 10];
const LYDIAN_DOMINANT: &[u8] = &[0, 2, 4, 6, 7, 9, 10];
const HALF_WHOLE: &[u8] = &[0, 1, 3, 4, 6, 7, 9, 10];
const WHOLE_HALF: &[u8] = &[0, 2, 3, 5, 6, 8, 9, 11];
const WHOLE_TONE: &[u8] = &[0, 2, 4, 6, 8, 10];
const LYDIAN_AUGMENTED: &[u8] = &[0, 2, 4, 6, 8, 9, 11];
const LOCRIAN: &[u8] = &[0, 1, 3, 5, 6, 8, 10];

/// The names of the scales a chord scale can be, by their notes above
/// the chord's root.
const NAMES: &[(&[u8], &str)] = &[
    (IONIAN, "major (Ionian)"),
    (DORIAN, "Dorian"),
    (&[0, 1, 3, 5, 7, 8, 10], "Phrygian"),
    (LYDIAN, "Lydian"),
    (MIXOLYDIAN, "Mixolydian"),
    (&[0, 2, 3, 5, 7, 8, 10], "natural minor (Aeolian)"),
    (LOCRIAN, "Locrian"),
    (&[0, 2, 3, 5, 7, 8, 11], "harmonic minor"),
    (&[0, 1, 3, 5, 6, 9, 10], "Locrian ♮6"),
    (&[0, 2, 4, 5, 8, 9, 11], "Ionian ♯5"),
    (&[0, 2, 3, 6, 7, 9, 10], "Dorian ♯4"),
    (&[0, 1, 4, 5, 7, 8, 10], "Phrygian dominant"),
    (&[0, 3, 4, 6, 7, 9, 11], "Lydian ♯2"),
    (&[0, 1, 3, 4, 6, 8, 9], "altered ♭♭7"),
    (MELODIC_MINOR, "melodic minor"),
    (&[0, 1, 3, 5, 7, 9, 10], "Dorian ♭2"),
    (LYDIAN_AUGMENTED, "Lydian augmented"),
    (LYDIAN_DOMINANT, "Lydian dominant"),
    (&[0, 2, 4, 5, 7, 8, 10], "Mixolydian ♭6"),
    (&[0, 2, 3, 5, 6, 8, 10], "Locrian ♮2"),
    (&[0, 1, 3, 4, 6, 8, 10], "altered"),
    (&[0, 2, 4, 5, 7, 8, 11], "harmonic major"),
    (HALF_WHOLE, "half-whole diminished"),
    (WHOLE_HALF, "whole-half diminished"),
    (WHOLE_TONE, "whole tone"),
];

/// The scale a soloist plays over `chord` in `key`, as pitch classes.
/// It always holds the chord's notes. See the module's documentation for
/// how it is chosen.
pub fn chord_scale(chord: Chord, key: Scale) -> PitchSet {
    if !key.kind.is_tonal() || !key.is_heptatonic() {
        return usual_scale(chord, false);
    }
    let harmonic = |root: PitchClass| Scale::new(root, ScaleKind::HarmonicMinor);
    let source = if key.is_chord_of_key(chord) {
        match key.uses_raised_seventh(chord) {
            true => harmonic(key.root),
            false => key,
        }
    } else if let Some(parallel) = key.borrowed_from(chord) {
        match parallel.is_diatonic(chord) {
            true => parallel,
            false => harmonic(key.root),
        }
    } else if let Some(target) = key.secondary_dominant_target(chord) {
        match target.quality.is_minor() {
            true => harmonic(target.root),
            false => Scale::new(target.root, ScaleKind::Major),
        }
    } else {
        return usual_scale(chord, true);
    };
    with_chord_notes(source, chord)
}

/// The scale `chord` usually gets by its kind. `outside` says it comes
/// from outside the key: a major chord there plays Lydian, and a
/// dominant seventh Lydian dominant.
fn usual_scale(chord: Chord, outside: bool) -> PitchSet {
    use ChordQuality::*;
    let steps = match chord.quality {
        Major | Add9 | Major6 | Six9 | Major7 | Major9 | Major13 if outside => LYDIAN,
        Major | Add9 | Major6 | Six9 | Major7 | Major9 | Major13 | Sus2 => IONIAN,
        Minor | Minor6 | Minor7 | Minor9 | Minor11 | MinorAdd9 => DORIAN,
        MinorMajor7 => MELODIC_MINOR,
        Dominant7 | Dominant9 | Dominant13 if outside => LYDIAN_DOMINANT,
        Dominant7 | Dominant9 | Dominant11 | Dominant13 | Sus4 | Dominant7Sus4 => MIXOLYDIAN,
        Dominant7Flat9 | Dominant7Sharp9 => HALF_WHOLE,
        Dominant7Flat5 | Augmented7 | Augmented => WHOLE_TONE,
        AugmentedMajor7 => LYDIAN_AUGMENTED,
        Diminished | HalfDiminished7 => LOCRIAN,
        Diminished7 => WHOLE_HALF,
    };
    let scale = PitchSet::from_semitones(steps).transpose(chord.root.semitone());
    PitchSet::from_bits(scale.bits() | chord.pitch_set().bits())
}

/// The notes of `source` with the chord's notes from outside it in
/// place of those written with the same letter: E9 over A harmonic minor
/// has F#, its ninth, in place of F.
fn with_chord_notes(source: Scale, chord: Chord) -> PitchSet {
    let names = source.chord_notes(chord);
    let mut set = chord.pitch_set();
    for pitch_class in source.pitch_classes() {
        let letter = source.spell(pitch_class).letter;
        let replaced = names.iter().any(|name| {
            name.letter == letter
                && name.pitch_class() != pitch_class
                && !source.contains(name.pitch_class())
        });
        if !replaced {
            set.insert(pitch_class);
        }
    }
    set
}

/// The name of `scale` played from `root`, as jazz teaching names chord
/// scales: "Dorian", "Phrygian dominant". `None` if it has no usual name.
pub fn chord_scale_name(scale: PitchSet, root: PitchClass) -> Option<&'static str> {
    let relative = scale.relative_to(root);
    NAMES
        .iter()
        .find(|(steps, _)| PitchSet::from_semitones(steps) == relative)
        .map(|(_, name)| *name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harmony::ChordSymbol;

    fn chord(name: &str) -> Chord {
        ChordSymbol::parse(name).unwrap().chord()
    }

    fn key(root: PitchClass, kind: ScaleKind) -> Scale {
        Scale::new(root, kind)
    }

    /// The scale's notes from the chord's root up, the chord's own as
    /// the chord spells them and the others as `key` does.
    fn notes(name: &str, key: Scale) -> String {
        let chord = chord(name);
        let scale = chord_scale(chord, key);
        let spelled = key.chord_notes(chord);
        let mut semitones: Vec<u8> = scale.relative_to(chord.root).semitones();
        semitones.sort_unstable();
        semitones
            .iter()
            .map(|&up| {
                let pitch_class = chord.root.transpose(i32::from(up));
                spelled
                    .iter()
                    .find(|name| name.pitch_class() == pitch_class)
                    .copied()
                    .unwrap_or_else(|| key.spell(pitch_class))
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn name(chord_name: &str, key: Scale) -> &'static str {
        let chord = chord(chord_name);
        chord_scale_name(chord_scale(chord, key), chord.root).unwrap()
    }

    #[test]
    fn chords_of_the_key_play_its_modes() {
        let c = key(PitchClass::C, ScaleKind::Major);
        assert_eq!(notes("Dm7", c), "D E F G A B C");
        assert_eq!(name("Dm7", c), "Dorian");
        assert_eq!(name("Em", c), "Phrygian");
        assert_eq!(name("Fmaj7", c), "Lydian");
        assert_eq!(name("G7", c), "Mixolydian");
        assert_eq!(name("Am", c), "natural minor (Aeolian)");
        assert_eq!(name("Bm7b5", c), "Locrian");
        assert_eq!(name("C", c), "major (Ionian)");
    }

    #[test]
    fn the_dominant_of_a_minor_key_plays_harmonic_minor() {
        let a_minor = key(PitchClass::A, ScaleKind::NaturalMinor);
        assert_eq!(notes("E7", a_minor), "E F G# A B C D");
        assert_eq!(name("E7", a_minor), "Phrygian dominant");
        assert_eq!(name("Dm", a_minor), "Dorian");
    }

    #[test]
    fn borrowed_chords_play_the_parallel_key() {
        let c = key(PitchClass::C, ScaleKind::Major);
        assert_eq!(notes("Fm", c), "F G Ab Bb C D Eb");
        assert_eq!(name("Ab", c), "Lydian");
        assert_eq!(name("Bb7", c), "Mixolydian");
        // A major key borrows from harmonic minor too.
        assert_eq!(notes("G7b9", c), "G Ab B C D Eb F");
    }

    #[test]
    fn secondary_dominants_play_the_scale_of_their_target() {
        let c = key(PitchClass::C, ScaleKind::Major);
        assert_eq!(notes("D7", c), "D E F# G A B C");
        assert_eq!(notes("E7", c), "E F G# A B C D");
        assert_eq!(notes("A7", c), "A Bb C# D E F G");
        // The chord's own ninth replaces the F of A harmonic minor.
        assert_eq!(notes("E9", c), "E F# G# A B C D");
        assert_eq!(name("E9", c), "Mixolydian ♭6");
    }

    #[test]
    fn other_chords_play_the_scale_of_their_kind() {
        let c = key(PitchClass::C, ScaleKind::Major);
        // The Neapolitan chord, from outside the key: Lydian.
        assert_eq!(notes("Db", c), "Db Eb F G Ab Bb C");
        // A dominant seventh that leads nowhere: Lydian dominant.
        assert_eq!(name("Eb7", c), "Lydian dominant");
        // In a blues key, each chord its own.
        let blues = key(PitchClass::C, ScaleKind::Blues);
        assert_eq!(name("F7", blues), "Mixolydian");
        assert_eq!(name("C7#9", blues), "half-whole diminished");
        assert_eq!(name("Cdim7", blues), "whole-half diminished");
        assert_eq!(name("C+", blues), "whole tone");
    }

    #[test]
    fn every_chord_scale_holds_its_chord() {
        let keys = [
            key(PitchClass::C, ScaleKind::Major),
            key(PitchClass::A, ScaleKind::NaturalMinor),
            key(PitchClass::D, ScaleKind::Dorian),
            key(PitchClass::E, ScaleKind::Blues),
            key(PitchClass::G, ScaleKind::HarmonicMinor),
        ];
        for key in keys {
            for root in 0..12 {
                for quality in ChordQuality::ALL {
                    let chord = Chord::new(PitchClass::from_semitone(root), quality);
                    let scale = chord_scale(chord, key);
                    assert!(
                        chord.pitch_set().is_subset_of(scale),
                        "{chord:?} in {key:?}"
                    );
                    assert!((6..=9).contains(&scale.len()), "{chord:?} in {key:?}");
                }
            }
        }
    }
}
