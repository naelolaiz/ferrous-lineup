//! Which chord could come next: the common moves between the chords of a
//! major or minor key.
//!
//! These are not rules. They are the moves that centuries of music (and
//! most pop songs) use most, because of how chords pull on each other:
//! the dominant (V) pulls home to the tonic (I), the subdominant (IV)
//! leads away from home toward the dominant, and roots that fall by a
//! fifth sound strongest of all. Knowing them helps you guess what comes
//! next in a song, and choose on purpose when to surprise.
//!
//! A chord is found in the tables by where its root is (how many
//! semitones above the tonic) and by its kind of triad: major, minor or
//! diminished. So G7 in C major is the V, but Gm is not: a minor chord
//! on G has no leading tone, and does not do what V does. Major keys
//! also know the chords they most often borrow from the minor key: iv,
//! bIII, bVI and bVII.

use super::{Chord, ChordFamily, ChordQuality, HintLevel, Scale, ScaleKind};

/// A chord that often follows another one, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suggestion {
    /// The degree of the scale the chord is built on, from 0 (the tonic).
    /// Only chords whose root is a note of the scale are suggested.
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

/// A move to the chord whose root is `to` semitones above the tonic,
/// with its reason in a few words and in a sentence.
struct Move {
    to: u8,
    short: &'static str,
    reason: &'static str,
}

/// A `const fn` can be called where a constant is built, such as in the
/// tables below, to keep each move on one line.
const fn go(to: u8, short: &'static str, reason: &'static str) -> Move {
    Move { to, short, reason }
}

/// A chord of the key, found by its root (semitones above the tonic) and
/// its kind of triad, and the moves that often follow it.
struct Slot {
    root: u8,
    family: ChordFamily,
    moves: &'static [Move],
}

const fn slot(root: u8, family: ChordFamily, moves: &'static [Move]) -> Slot {
    Slot {
        root,
        family,
        moves,
    }
}

use ChordFamily::{Diminished, Major, Minor};

/// The chords suggestions lead to, in a major key: the seven triads of
/// the major scale, by their roots in semitones above the tonic.
const MAJOR_CHORDS: [(u8, ChordQuality); 7] = [
    (0, ChordQuality::Major),
    (2, ChordQuality::Minor),
    (4, ChordQuality::Minor),
    (5, ChordQuality::Major),
    (7, ChordQuality::Major),
    (9, ChordQuality::Minor),
    (11, ChordQuality::Diminished),
];

/// The chords suggestions lead to, in a minor key: those of natural
/// minor, but with a major V and the leading-tone vii° of harmonic
/// minor, as most music in minor keys plays them.
const MINOR_CHORDS: [(u8, ChordQuality); 8] = [
    (0, ChordQuality::Minor),
    (2, ChordQuality::Diminished),
    (3, ChordQuality::Major),
    (5, ChordQuality::Minor),
    (7, ChordQuality::Major),
    (8, ChordQuality::Major),
    (10, ChordQuality::Major),
    (11, ChordQuality::Diminished),
];

/// The chords of a major key, and the chords it borrows from the minor
/// key, with the moves that often follow each.
const MAJOR: &[Slot] = &[
    slot(
        0,
        Major,
        &[
            go(
                5,
                "away from home",
                "I to IV: moving away from home, toward the dominant.",
            ),
            go(
                7,
                "tension, wants to go home",
                "I to V: tension that wants to come back to I.",
            ),
            go(
                9,
                "relative minor, darker",
                "I to vi: the relative minor, a darker colour. It shares two of its three notes with I.",
            ),
            go(
                2,
                "leads on to V",
                "I to ii: a minor chord that leads on to V.",
            ),
        ],
    ),
    slot(
        2,
        Minor,
        &[
            go(
                7,
                "root falls a fifth (ii V I)",
                "ii to V: the root falls a fifth. ii, V, I is everywhere, from jazz to pop.",
            ),
            go(
                11,
                "also leads home",
                "ii to vii°: another chord that leads home to I.",
            ),
        ],
    ),
    slot(
        4,
        Minor,
        &[
            go(
                9,
                "root falls a fifth",
                "iii to vi: the root falls a fifth.",
            ),
            go(
                5,
                "step up to the subdominant",
                "iii to IV: a step up to the subdominant.",
            ),
        ],
    ),
    slot(
        5,
        Major,
        &[
            go(
                7,
                "step up to the dominant",
                "IV to V: a step up to the dominant, ready to come home.",
            ),
            go(
                0,
                "amen (plagal) cadence",
                "IV to I: the \"amen\" (plagal) cadence.",
            ),
            go(
                2,
                "both lead on to V",
                "IV to ii: two chords that lead away from home, then on to V.",
            ),
        ],
    ),
    slot(
        7,
        Major,
        &[
            go(
                0,
                "strongest ending",
                "V to I: the strongest ending. The leading tone rises to the root.",
            ),
            go(
                9,
                "deceptive cadence",
                "V to vi: the \"deceptive\" cadence. You expect I and get its relative minor.",
            ),
        ],
    ),
    slot(
        9,
        Minor,
        &[
            go(2, "root falls a fifth", "vi to ii: the root falls a fifth."),
            go(
                5,
                "as in I V vi IV",
                "vi to IV, as in the I, V, vi, IV progression of countless songs.",
            ),
            go(7, "on to the dominant", "vi to V: on to the dominant."),
        ],
    ),
    slot(
        11,
        Diminished,
        &[
            go(
                0,
                "resolves home, like V7",
                "vii° to I: vii° is V7 without its root, so it resolves home too.",
            ),
            go(
                4,
                "root falls a fifth",
                "vii° to iii: the root falls a fifth.",
            ),
        ],
    ),
    // Borrowed from the minor key.
    slot(
        5,
        Minor,
        &[
            go(
                0,
                "minor plagal cadence",
                "iv to I: the minor plagal cadence. The iv is borrowed from the minor key, and its flat sixth falls to the fifth of I.",
            ),
            go(7, "on to the dominant", "iv to V: on to the dominant."),
        ],
    ),
    slot(
        2,
        Diminished,
        &[go(
            7,
            "borrowed ii°, falls a fifth to V",
            "ii° to V: the ii chord of the minor key, borrowed, falls a fifth to V, as in a minor ii, V, i.",
        )],
    ),
    slot(
        3,
        Major,
        &[go(
            5,
            "step up to IV",
            "bIII to IV: a step up from a chord borrowed from the minor key, as in rock's I bIII IV.",
        )],
    ),
    slot(
        8,
        Major,
        &[go(
            7,
            "slides down to V",
            "bVI to V: the root slides down a semitone onto the dominant.",
        )],
    ),
    slot(
        10,
        Major,
        &[
            go(
                0,
                "a step up to home",
                "bVII to I: a whole step up to home, the rock cadence (and jazz's \"backdoor\").",
            ),
            go(
                5,
                "root rises a fifth (plagal)",
                "bVII to IV: the root rises a fifth, a plagal move like IV to I. I, bVII, IV, I is the \"double plagal\" cadence.",
            ),
        ],
    ),
];

/// What often follows V in a minor key (or v, which the minor key's
/// own scale gives).
const MINOR_FIVE: &[Move] = &[
    go(
        0,
        "back home",
        "V to i: back home. The pull is strongest when V is major (its third is the leading tone).",
    ),
    go(
        8,
        "deceptive cadence",
        "V to bVI: the deceptive cadence of minor keys.",
    ),
];

/// The chords of a minor key, with the moves that often follow each.
/// Melodic minor's ii and IV are here too.
const MINOR: &[Slot] = &[
    slot(
        0,
        Minor,
        &[
            go(
                5,
                "minor subdominant, away from home",
                "i to iv: the minor subdominant, moving away from home.",
            ),
            go(
                8,
                "bright major chord",
                "i to bVI: a bright major chord, a major third below home.",
            ),
            go(
                10,
                "step below home, rock and film",
                "i to bVII: the major chord a whole step below home, common in rock and film music.",
            ),
            go(
                7,
                "tension, wants to go home",
                "i to V: tension that wants to come back to i.",
            ),
        ],
    ),
    slot(
        2,
        Diminished,
        &[go(
            7,
            "root falls a fifth",
            "ii° to V: the root falls a fifth, toward home.",
        )],
    ),
    slot(
        2,
        Minor,
        &[go(
            7,
            "root falls a fifth",
            "ii to V: the root falls a fifth, toward home.",
        )],
    ),
    slot(
        3,
        Major,
        &[
            go(
                8,
                "root falls a fifth",
                "bIII to bVI: the root falls a fifth.",
            ),
            go(
                5,
                "step up to the subdominant",
                "bIII to iv: a step up to the subdominant.",
            ),
        ],
    ),
    slot(
        5,
        Minor,
        &[
            go(
                7,
                "step up to the dominant",
                "iv to V: a step up to the dominant.",
            ),
            go(
                0,
                "minor plagal cadence",
                "iv to i: the minor plagal cadence.",
            ),
            go(
                10,
                "then on to bIII or i",
                "iv to bVII: then on to bIII or back to i.",
            ),
        ],
    ),
    slot(
        5,
        Major,
        &[go(
            7,
            "step up to the dominant",
            "IV to V: a step up to the dominant. The raised sixth of melodic minor rises to the leading tone.",
        )],
    ),
    slot(7, Major, MINOR_FIVE),
    slot(7, Minor, MINOR_FIVE),
    slot(
        8,
        Major,
        &[
            go(
                10,
                "rising toward i",
                "bVI to bVII: rising step by step, toward i.",
            ),
            go(
                5,
                "on to the subdominant",
                "bVI to iv: on to the subdominant.",
            ),
            go(
                2,
                "root falls a diminished fifth",
                "bVI to ii°: the root falls a diminished fifth, the one fifth of the scale that is not perfect.",
            ),
        ],
    ),
    slot(
        10,
        Major,
        &[
            go(
                3,
                "V of the relative major",
                "bVII to bIII: bVII is the dominant of bIII, the relative major.",
            ),
            go(0, "step up to home", "bVII to i: a whole step up to home."),
        ],
    ),
    slot(
        11,
        Diminished,
        &[go(
            0,
            "resolves home, like V7",
            "vii° to i: the leading-tone chord is V7 without its root, so it resolves home too.",
        )],
    ),
];

const SECONDARY_DOMINANT_SHORT: &str = "secondary dominant: V of the chord a fifth down";
const SECONDARY_DOMINANT: &str = "Not in the key: a major or dominant chord here acts as a \
    \"secondary dominant\". It is the V of the chord a fifth below, and pulls to it as V pulls to I.";

/// The chords that often follow `current` in `scale`, the most usual first.
///
/// Suggestions are given for major and minor keys (natural, harmonic or
/// melodic). In minor keys they lead to the chords of natural minor,
/// with a major V and the leading-tone vii°, as most music in minor
/// keys plays them. Only chords whose root is a note of `scale` are
/// suggested, so that each one is on a degree of the scale. Nothing is
/// suggested for other scales, or for a chord the tables do not know,
/// such as a minor chord on the fifth note of a major key.
pub fn next_chords(scale: Scale, current: Chord) -> Vec<Suggestion> {
    let minor = match scale.kind {
        ScaleKind::Major => false,
        ScaleKind::NaturalMinor | ScaleKind::HarmonicMinor | ScaleKind::MelodicMinor => true,
        _ => return Vec::new(),
    };
    let chords: &[(u8, ChordQuality)] = if minor { &MINOR_CHORDS } else { &MAJOR_CHORDS };
    // The key's chord with its root `to` semitones above the tonic, if
    // that root is a note of the scale.
    let suggest = |to: u8, short: &'static str, reason: &'static str| -> Option<Suggestion> {
        let &(_, quality) = chords.iter().find(|(root, _)| *root == to)?;
        let root = scale.root.transpose(i32::from(to));
        Some(Suggestion {
            degree: scale.degree_of(root)?,
            chord: Chord::new(root, quality),
            reason,
            short,
        })
    };
    let root = scale.root.semitones_up_to(current.root);

    // A major or dominant chord from outside the key, a fifth above one
    // of the key's chords, is most often its dominant: D major in C
    // major leads to G. (Not to the tonic: in a minor key, that is the
    // key's own V, which is not "outside" it.)
    let dominant_kind = current.quality == ChordQuality::Major || current.quality.is_dominant();
    let target = (root + 5) % 12;
    if dominant_kind
        && !scale.is_chord_of_key(current)
        && target != 0
        && let Some(suggestion) = suggest(target, SECONDARY_DOMINANT_SHORT, SECONDARY_DOMINANT)
        && suggestion.chord.quality != ChordQuality::Diminished
    {
        return vec![suggestion];
    }

    let table = if minor { MINOR } else { MAJOR };
    let family = current.quality.family();
    table
        .iter()
        .find(|slot| slot.root == root && slot.family == family)
        .map(|slot| {
            slot.moves
                .iter()
                .filter_map(|step| suggest(step.to, step.short, step.reason))
                .collect()
        })
        .unwrap_or_default()
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
    fn a_fifth_down_or_up_is_what_it_says() {
        // Every move whose reason says the root falls (or rises) a fifth
        // goes from a root to the one seven semitones below (or above).
        for slot in MAJOR.iter().chain(MINOR) {
            for go in slot.moves {
                let down = (slot.root + 12 - go.to) % 12;
                if go.reason.contains("falls a fifth") {
                    assert_eq!(down, 7, "{}", go.reason);
                }
                if go.reason.contains("rises a fifth") {
                    assert_eq!(down, 5, "{}", go.reason);
                }
            }
        }
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
            Scale::new(A, ScaleKind::HarmonicMinor),
            Scale::new(A, ScaleKind::MelodicMinor),
        ] {
            for degree in 0..7 {
                for s in next_chords(scale, scale.triad(degree).unwrap()) {
                    // Every suggestion is on a note of the scale.
                    assert_eq!(scale.degree_of(s.chord.root), Some(s.degree));
                    assert!(s.short.len() < s.reason.len(), "{:?}", s.short);
                    assert_eq!(s.why(HintLevel::Short), Some(s.short));
                    assert_eq!(s.why(HintLevel::Minimal), None);
                }
            }
        }
    }

    #[test]
    fn harmonic_and_melodic_minor_know_their_own_chords() {
        let harmonic = Scale::new(A, ScaleKind::HarmonicMinor);
        // vii° is the leading-tone chord: it goes home, not to bIII.
        for quality in [ChordQuality::Diminished, ChordQuality::Diminished7] {
            let after = next_chords(harmonic, major(GSharp, quality));
            assert_eq!(names(&after), ["Am"]);
            assert!(after[0].reason.starts_with("vii° to i"));
        }
        // G is not in A harmonic minor, so it is never suggested there.
        let after_am = next_chords(harmonic, major(A, ChordQuality::Minor));
        assert_eq!(names(&after_am), ["Dm", "F", "E"]);
        let melodic = Scale::new(A, ScaleKind::MelodicMinor);
        let after_bm = next_chords(melodic, major(B, ChordQuality::Minor));
        assert!(after_bm[0].reason.starts_with("ii to V"));
        let after_d = next_chords(melodic, major(D, ChordQuality::Major));
        assert!(after_d[0].reason.starts_with("IV to V"));
    }

    #[test]
    fn a_chord_must_be_the_kind_the_key_has() {
        let c_major = Scale::new(C, ScaleKind::Major);
        // Gm has no leading tone: it is not V.
        assert!(next_chords(c_major, major(G, ChordQuality::Minor)).is_empty());
        // Fm is the iv borrowed from C minor, which goes home or to V.
        let after_fm = next_chords(c_major, major(F, ChordQuality::Minor));
        assert_eq!(names(&after_fm), ["C", "G"]);
        assert!(after_fm[0].reason.starts_with("iv to I"));
        let after_bb = next_chords(c_major, major(ASharp, ChordQuality::Major));
        assert_eq!(names(&after_bb), ["C", "F"]);
        let after_ab = next_chords(c_major, major(GSharp, ChordQuality::Major));
        assert_eq!(names(&after_ab), ["G"]);
    }

    #[test]
    fn nothing_for_other_scales_or_roots() {
        let blues = Scale::new(C, ScaleKind::Blues);
        assert!(next_chords(blues, major(C, ChordQuality::Major)).is_empty());
        let c_major = Scale::new(C, ScaleKind::Major);
        assert!(next_chords(c_major, major(FSharp, ChordQuality::Major)).is_empty());
    }
}
