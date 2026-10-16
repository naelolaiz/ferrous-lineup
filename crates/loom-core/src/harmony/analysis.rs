//! Naming what is being played: a note, an interval or a chord.

use super::{Chord, ChordQuality, PitchSet, Scale};
use crate::{Note, PitchClass, Spelling};

/// The names of the intervals within one octave, by size in semitones.
const INTERVAL_NAMES: [&str; 12] = [
    "unison",
    "minor second",
    "major second",
    "minor third",
    "major third",
    "perfect fourth",
    "tritone",
    "perfect fifth",
    "minor sixth",
    "major sixth",
    "minor seventh",
    "major seventh",
];

/// The name of an interval, such as "perfect fifth" for 7 semitones.
/// Intervals larger than an octave are named like the smaller one.
pub fn interval_name(semitones: u8) -> &'static str {
    INTERVAL_NAMES[(semitones % 12) as usize]
}

/// What a group of notes sounds like, harmonically.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Harmony {
    /// No notes at all.
    Silence,
    /// One pitch class, maybe in several octaves.
    Single(PitchClass),
    /// Two pitch classes. `low` is the one in the bass, and `semitones`
    /// counts up from it to the other.
    Interval {
        low: PitchClass,
        high: PitchClass,
        semitones: u8,
    },
    /// A chord we know. When `bass` is not the chord's root, the chord is
    /// an inversion, written with a slash: C/E is C major with E in the bass.
    Chord { chord: Chord, bass: PitchClass },
    /// Three or more pitch classes that are not a chord we know.
    Unknown(PitchSet),
}

/// Works out the harmony of some notes, for example all the notes that
/// sound at one step of the piano roll.
pub fn analyze(notes: impl IntoIterator<Item = Note>) -> Harmony {
    let mut pitch_set = PitchSet::EMPTY;
    let mut lowest: Option<Note> = None;
    for note in notes {
        pitch_set.insert(note.pitch_class());
        lowest = Some(match lowest {
            Some(low) => low.min(note),
            None => note,
        });
    }

    let Some(lowest) = lowest else {
        return Harmony::Silence;
    };
    let bass = lowest.pitch_class();

    match pitch_set.len() {
        1 => Harmony::Single(bass),
        2 => {
            let high = pitch_set.iter().find(|&p| p != bass).unwrap_or(bass);
            Harmony::Interval {
                low: bass,
                high,
                semitones: bass.semitones_up_to(high),
            }
        }
        _ => match find_chord(pitch_set, bass) {
            Some(chord) => Harmony::Chord { chord, bass },
            None => Harmony::Unknown(pitch_set),
        },
    }
}

/// Finds a chord made of exactly the pitch classes in `pitch_set`.
///
/// Any note of the set could be the root, so each one is tried, starting
/// with the bass (chords are most often played with the root lowest) and
/// going up from there. If nothing fits, it tries again allowing the
/// fifth to be missing, because players often leave it out of seventh
/// chords: C, E and Bb is still heard as C7.
fn find_chord(pitch_set: PitchSet, bass: PitchClass) -> Option<Chord> {
    let roots = || {
        (0..12)
            .map(move |semitones| bass.transpose(semitones))
            .filter(move |&root| pitch_set.contains(root))
    };

    let exact = roots().find_map(|root| {
        ChordQuality::identify(pitch_set.relative_to(root)).map(|quality| Chord::new(root, quality))
    });

    exact.or_else(|| {
        roots().find_map(|root| {
            let mut with_fifth = pitch_set;
            with_fifth.insert(root.transpose(7));
            if with_fifth == pitch_set {
                return None; // the fifth was already there
            }
            ChordQuality::identify(with_fifth.relative_to(root))
                .filter(|quality| quality.intervals().len() >= 4)
                .map(|quality| Chord::new(root, quality))
        })
    })
}

impl Harmony {
    /// A short label for the chord lane above the piano roll, such as
    /// "Am", "C/E" or "C5". `None` for silence.
    pub fn label(&self, spelling: Spelling) -> Option<String> {
        match *self {
            Harmony::Silence => None,
            Harmony::Single(pitch_class) => Some(pitch_class.name(spelling).to_string()),
            // A root and its fifth is a "power chord", written C5.
            Harmony::Interval {
                low, semitones: 7, ..
            } => Some(format!("{}5", low.name(spelling))),
            Harmony::Interval { low, high, .. } => {
                Some(format!("{}+{}", low.name(spelling), high.name(spelling)))
            }
            Harmony::Chord { chord, bass } if bass != chord.root => {
                Some(format!("{}/{}", chord.name(spelling), bass.name(spelling)))
            }
            Harmony::Chord { chord, .. } => Some(chord.name(spelling)),
            Harmony::Unknown(_) => Some("?".to_string()),
        }
    }

    /// A sentence describing the harmony in the context of a key, for
    /// the line under the piano roll toolbar.
    pub fn describe(&self, scale: Scale) -> String {
        let spelling = scale.spelling();
        let key = scale.name();
        match *self {
            Harmony::Silence => "Nothing is playing here.".to_string(),
            Harmony::Single(pitch_class) => {
                let name = pitch_class.name(spelling);
                match scale.degree_of(pitch_class) {
                    Some(degree) => format!("{name}: note {} of {key}.", degree + 1),
                    None => format!("{name}: not in {key}."),
                }
            }
            Harmony::Interval {
                low,
                high,
                semitones,
            } => format!(
                "{} up to {}: a {}.",
                low.name(spelling),
                high.name(spelling),
                interval_name(semitones)
            ),
            Harmony::Chord { chord, bass } => {
                let mut text = format!(
                    "{}: {} {}",
                    self.label(spelling).unwrap_or_default(),
                    chord.root.name(spelling),
                    chord.quality.name()
                );
                if bass != chord.root {
                    text += &format!(" with {} in the bass", bass.name(spelling));
                }
                match scale.roman_numeral(chord) {
                    Some(numeral) if scale.is_diatonic(chord) => {
                        text += &format!(", chord {numeral} of {key}.");
                    }
                    Some(numeral) => {
                        text += &format!(", {numeral} borrowed from outside {key}.");
                    }
                    None if scale.is_diatonic(chord) => text += &format!(", in {key}."),
                    None => text += &format!(", outside {key}."),
                }
                text
            }
            Harmony::Unknown(pitch_set) => {
                let names: Vec<&str> = pitch_set.iter().map(|p| p.name(spelling)).collect();
                format!("{}: no chord name for these notes.", names.join(" "))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harmony::ScaleKind;
    use PitchClass::*;

    fn notes(midi: &[u8]) -> Vec<Note> {
        midi.iter().map(|&m| Note::new(m)).collect()
    }

    fn label(midi: &[u8]) -> Option<String> {
        analyze(notes(midi)).label(Spelling::Sharps)
    }

    #[test]
    fn silence_single_notes_and_intervals() {
        assert_eq!(analyze(Vec::new()), Harmony::Silence);
        assert_eq!(analyze(notes(&[60, 72, 48])), Harmony::Single(C));
        assert_eq!(
            analyze(notes(&[64, 60])),
            Harmony::Interval {
                low: C,
                high: E,
                semitones: 4
            }
        );
        assert_eq!(label(&[60, 67]).unwrap(), "C5");
        assert_eq!(label(&[60, 64]).unwrap(), "C+E");
    }

    #[test]
    fn triads_in_any_order_and_octave() {
        assert_eq!(label(&[60, 64, 67]).unwrap(), "C");
        assert_eq!(label(&[57, 60, 64]).unwrap(), "Am");
        assert_eq!(label(&[48, 67, 76, 72]).unwrap(), "C");
        assert_eq!(label(&[59, 62, 65]).unwrap(), "Bdim");
    }

    #[test]
    fn inversions_get_a_slash() {
        // E G C: C major with E in the bass.
        assert_eq!(label(&[64, 67, 72]).unwrap(), "C/E");
        assert_eq!(label(&[55, 60, 64]).unwrap(), "C/G");
    }

    #[test]
    fn the_bass_decides_between_equal_sets() {
        // C E G A is both C6 and Am7: the lowest note decides.
        assert_eq!(label(&[60, 64, 67, 69]).unwrap(), "C6");
        assert_eq!(label(&[57, 60, 64, 67]).unwrap(), "Am7");
        // C D G is both Csus2 and Gsus4.
        assert_eq!(label(&[60, 62, 67]).unwrap(), "Csus2");
        assert_eq!(label(&[55, 60, 62]).unwrap(), "Gsus4");
    }

    #[test]
    fn sevenths_without_their_fifth() {
        assert_eq!(label(&[48, 64, 70]).unwrap(), "C7");
        assert_eq!(label(&[48, 64, 71]).unwrap(), "Cmaj7");
    }

    #[test]
    fn unknown_clusters() {
        assert_eq!(label(&[60, 61, 62]).unwrap(), "?");
    }

    #[test]
    fn descriptions_use_the_key() {
        let c_major = Scale::default();
        let describe = |midi: &[u8]| analyze(notes(midi)).describe(c_major);
        assert_eq!(describe(&[57, 60, 64]), "Am: A minor, chord vi of C major.");
        assert_eq!(
            describe(&[64, 68, 71]),
            "E: E major, III borrowed from outside C major."
        );
        assert_eq!(describe(&[60, 67]), "C up to G: a perfect fifth.");
        assert_eq!(describe(&[69]), "A: note 6 of C major.");
        assert_eq!(
            describe(&[64, 67, 72]),
            "C/E: C major with E in the bass, chord I of C major."
        );

        let f_major = Scale::new(F, ScaleKind::Major);
        assert_eq!(
            analyze(notes(&[58, 62, 65])).describe(f_major),
            "Bb: Bb major, chord IV of F major."
        );
    }

    #[test]
    fn interval_names() {
        assert_eq!(interval_name(7), "perfect fifth");
        assert_eq!(interval_name(6), "tritone");
        assert_eq!(interval_name(19), "perfect fifth");
    }
}
