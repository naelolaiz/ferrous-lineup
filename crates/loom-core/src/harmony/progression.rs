//! Which chord could come next: the common moves between the chords of a
//! major or minor key.
//!
//! These are not rules. They are the moves that centuries of music (and
//! most pop songs) use most, because of how chords pull on each other:
//! the dominant (V) pulls home to the tonic (I), the subdominant (IV)
//! leads away from home toward the dominant, and roots that fall by a
//! fifth sound strongest of all. Knowing them helps you guess what comes
//! next in a song, and choose on purpose when to surprise.

use super::{Chord, ChordQuality, HintLevel, Scale, ScaleKind};

/// A chord that often follows another one, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suggestion {
    /// The degree of the key the chord is built on, from 0 (the tonic).
    pub degree: usize,
    pub chord: Chord,
    /// Why, in a sentence: "IV to V: a step up to the dominant, ready to
    /// come home."
    pub reason: &'static str,
    /// The same reason in a few words: "step up to the dominant".
    pub short: &'static str,
}

impl Suggestion {
    /// The reason at `level`: the sentence, the few words, or nothing.
    pub fn why(&self, level: HintLevel) -> Option<&'static str> {
        match level {
            HintLevel::Detailed => Some(self.reason),
            HintLevel::Short => Some(self.short),
            HintLevel::Minimal => None,
        }
    }
}

/// A move from one degree of the key to another (`to`), with its reason
/// in a few words and in a sentence.
struct Move {
    to: usize,
    short: &'static str,
    reason: &'static str,
}

/// A `const fn` can be called where a constant is built, such as in the
/// tables below, to keep each move on one line.
const fn go(to: usize, short: &'static str, reason: &'static str) -> Move {
    Move { to, short, reason }
}

/// For each degree of a major key, the degrees that often come next.
const MAJOR: [&[Move]; 7] = [
    // I
    &[
        go(
            3,
            "away from home",
            "I to IV: moving away from home, toward the dominant.",
        ),
        go(
            4,
            "tension, wants to go home",
            "I to V: tension that wants to come back to I.",
        ),
        go(
            5,
            "relative minor, darker",
            "I to vi: the relative minor, the same notes with a darker colour.",
        ),
        go(
            1,
            "leads on to V",
            "I to ii: a minor chord that leads on to V.",
        ),
    ],
    // ii
    &[
        go(
            4,
            "root falls a fifth (ii V I)",
            "ii to V: the root falls a fifth. ii, V, I is everywhere, from jazz to pop.",
        ),
        go(
            6,
            "also leads home",
            "ii to vii°: another chord that leads home to I.",
        ),
    ],
    // iii
    &[
        go(
            5,
            "root falls a fifth",
            "iii to vi: the root falls a fifth.",
        ),
        go(
            3,
            "step up to the subdominant",
            "iii to IV: a step up to the subdominant.",
        ),
    ],
    // IV
    &[
        go(
            4,
            "step up to the dominant",
            "IV to V: a step up to the dominant, ready to come home.",
        ),
        go(
            0,
            "amen (plagal) cadence",
            "IV to I: the \"amen\" (plagal) cadence.",
        ),
        go(
            1,
            "both lead on to V",
            "IV to ii: two chords that lead away from home, then on to V.",
        ),
    ],
    // V
    &[
        go(
            0,
            "strongest ending",
            "V to I: the strongest ending. The leading tone rises to the root.",
        ),
        go(
            5,
            "deceptive cadence",
            "V to vi: the \"deceptive\" cadence. You expect I and get its relative minor.",
        ),
    ],
    // vi
    &[
        go(1, "root falls a fifth", "vi to ii: the root falls a fifth."),
        go(
            3,
            "as in I V vi IV",
            "vi to IV, as in the I, V, vi, IV progression of countless songs.",
        ),
        go(4, "on to the dominant", "vi to V: on to the dominant."),
    ],
    // vii°
    &[
        go(
            0,
            "resolves home, like V7",
            "vii° to I: vii° is V7 without its root, so it resolves home too.",
        ),
        go(
            2,
            "root falls a fifth",
            "vii° to iii: the root falls a fifth.",
        ),
    ],
];

/// For each degree of a minor key, the degrees that often come next.
const MINOR: [&[Move]; 7] = [
    // i
    &[
        go(
            3,
            "minor subdominant, away from home",
            "i to iv: the minor subdominant, moving away from home.",
        ),
        go(
            5,
            "bright major chord",
            "i to VI: a bright major chord, a third below home.",
        ),
        go(
            6,
            "step below home, rock and film",
            "i to VII: the major chord a step below home, common in rock and film music.",
        ),
        go(
            4,
            "tension, wants to go home",
            "i to V: tension that wants to come back to i.",
        ),
    ],
    // ii°
    &[go(
        4,
        "root falls a fifth",
        "ii° to V: the root falls a fifth, toward home.",
    )],
    // III
    &[
        go(
            5,
            "root falls a fifth",
            "III to VI: the root falls a fifth.",
        ),
        go(
            3,
            "step up to the subdominant",
            "III to iv: a step up to the subdominant.",
        ),
    ],
    // iv
    &[
        go(
            4,
            "step up to the dominant",
            "iv to V: a step up to the dominant.",
        ),
        go(
            0,
            "minor plagal cadence",
            "iv to i: the minor plagal cadence.",
        ),
        go(
            6,
            "then on to III or i",
            "iv to VII: then on to III or back to i.",
        ),
    ],
    // V (or v)
    &[
        go(
            0,
            "back home",
            "V to i: back home. The pull is strongest when V is major (its third is the leading tone).",
        ),
        go(
            5,
            "deceptive cadence",
            "V to VI: the deceptive cadence of minor keys.",
        ),
    ],
    // VI
    &[
        go(
            6,
            "rising toward i",
            "VI to VII: rising step by step, toward i.",
        ),
        go(
            3,
            "on to the subdominant",
            "VI to iv: on to the subdominant.",
        ),
        go(
            1,
            "root falls a fifth",
            "VI to ii°: the root falls a fifth.",
        ),
    ],
    // VII
    &[
        go(
            2,
            "V of the relative major",
            "VII to III: VII is the dominant of III, the relative major.",
        ),
        go(0, "step up to home", "VII to i: a step up to home."),
    ],
];

const SECONDARY_DOMINANT_SHORT: &str = "secondary dominant: V of the chord a fifth down";
const SECONDARY_DOMINANT: &str = "Not in the key: a major or dominant chord here acts as a \
    \"secondary dominant\". It is the V of the chord a fifth below, and pulls to it as V pulls to I.";

/// The chords that often follow `current` in `scale`, the most usual first.
///
/// Suggestions are given for major and minor keys. In minor keys they
/// use the chords of the natural minor scale, but with a major V, as most
/// music in minor keys does. Nothing is suggested for other scales, or
/// when the chord's root is not in the key.
pub fn next_chords(scale: Scale, current: Chord) -> Vec<Suggestion> {
    let minor = match scale.kind {
        ScaleKind::Major => false,
        ScaleKind::NaturalMinor | ScaleKind::HarmonicMinor | ScaleKind::MelodicMinor => true,
        _ => return Vec::new(),
    };
    let chord_on = |degree: usize| -> Option<Chord> {
        if !minor {
            return scale.triad(degree);
        }
        if degree == 4 {
            let root = scale.pitch_classes().nth(4)?;
            return Some(Chord::new(root, ChordQuality::Major));
        }
        Scale::new(scale.root, ScaleKind::NaturalMinor).triad(degree)
    };
    let Some(degree) = scale.degree_of(current.root) else {
        return Vec::new();
    };

    // A major chord on a degree where the key has none (D major in C) is
    // most often the dominant of the chord a fifth below it (G).
    let target = current.root.transpose(5);
    let dominant_kind = matches!(
        current.quality,
        ChordQuality::Major
            | ChordQuality::Dominant7
            | ChordQuality::Dominant9
            | ChordQuality::Dominant7Flat9
            | ChordQuality::Dominant7Sharp9
            | ChordQuality::Dominant11
            | ChordQuality::Dominant13
    );
    if dominant_kind
        && !scale.is_diatonic(current)
        && let Some(target_degree) = scale.degree_of(target)
        && target_degree != 0
        && let Some(chord) = chord_on(target_degree)
        && chord.quality != ChordQuality::Diminished
    {
        return vec![Suggestion {
            degree: target_degree,
            chord,
            reason: SECONDARY_DOMINANT,
            short: SECONDARY_DOMINANT_SHORT,
        }];
    }

    let table = if minor { &MINOR } else { &MAJOR };
    table[degree]
        .iter()
        .filter_map(|step| {
            Some(Suggestion {
                degree: step.to,
                chord: chord_on(step.to)?,
                reason: step.reason,
                short: step.short,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PitchClass, Spelling};
    use PitchClass::*;

    fn names(suggestions: &[Suggestion]) -> Vec<String> {
        suggestions
            .iter()
            .map(|s| s.chord.name(Spelling::Sharps))
            .collect()
    }

    fn major(root: PitchClass, quality: ChordQuality) -> Chord {
        Chord::new(root, quality)
    }

    #[test]
    fn the_dominant_goes_home() {
        let c_major = Scale::new(C, ScaleKind::Major);
        let after_g = next_chords(c_major, major(G, ChordQuality::Major));
        assert_eq!(names(&after_g), ["C", "Am"]);
        // G7 is in the key too, and goes to the same places.
        let after_g7 = next_chords(c_major, major(G, ChordQuality::Dominant7));
        assert_eq!(after_g7, after_g);
    }

    #[test]
    fn every_degree_of_major_and_minor_has_suggestions() {
        for kind in [
            ScaleKind::Major,
            ScaleKind::NaturalMinor,
            ScaleKind::HarmonicMinor,
        ] {
            let scale = Scale::new(A, kind);
            for degree in 0..7 {
                let chord = scale.triad(degree).unwrap();
                let suggestions = next_chords(scale, chord);
                assert!(!suggestions.is_empty(), "{kind:?} degree {degree}");
                assert!(suggestions.iter().all(|s| s.degree != degree));
            }
        }
    }

    #[test]
    fn minor_keys_use_a_major_five() {
        let a_minor = Scale::new(A, ScaleKind::NaturalMinor);
        let after_dm = next_chords(a_minor, major(D, ChordQuality::Minor));
        assert_eq!(names(&after_dm), ["E", "Am", "G"]);
        // E major is not in A natural minor (G#), but it is V, not a
        // secondary dominant: it goes home.
        let after_e = next_chords(a_minor, major(E, ChordQuality::Major));
        assert_eq!(names(&after_e)[0], "Am");
    }

    #[test]
    fn secondary_dominants_pull_a_fifth_down() {
        let c_major = Scale::new(C, ScaleKind::Major);
        // D major is V of G; A7 is V of Dm; E is V of Am; C7 is V of F.
        for (chord, target) in [
            (major(D, ChordQuality::Major), "G"),
            (major(A, ChordQuality::Dominant7), "Dm"),
            (major(E, ChordQuality::Major), "Am"),
            (major(C, ChordQuality::Dominant7), "F"),
        ] {
            let suggestions = next_chords(c_major, chord);
            assert_eq!(names(&suggestions), [target], "{chord:?}");
            assert_eq!(suggestions[0].reason, SECONDARY_DOMINANT);
        }
    }

    #[test]
    fn short_reasons_are_shorter() {
        for scale in [
            Scale::new(C, ScaleKind::Major),
            Scale::new(A, ScaleKind::NaturalMinor),
        ] {
            for degree in 0..7 {
                for s in next_chords(scale, scale.triad(degree).unwrap()) {
                    assert!(s.short.len() < s.reason.len(), "{:?}", s.short);
                    assert_eq!(s.why(HintLevel::Short), Some(s.short));
                    assert_eq!(s.why(HintLevel::Minimal), None);
                }
            }
        }
    }

    #[test]
    fn nothing_for_other_scales_or_roots() {
        let blues = Scale::new(C, ScaleKind::Blues);
        assert!(next_chords(blues, major(C, ChordQuality::Major)).is_empty());
        let c_major = Scale::new(C, ScaleKind::Major);
        assert!(next_chords(c_major, major(FSharp, ChordQuality::Major)).is_empty());
    }
}
