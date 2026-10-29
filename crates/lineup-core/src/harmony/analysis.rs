//! Naming what is being played: a note, an interval or a chord.

use super::{Chord, ChordQuality, PitchSet, Scale, with_figures};
use crate::i18n::{Language, key_name, tr, trf};
use crate::{Note, NoteName, PitchClass, Spelling};

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

/// How many letters apart the notes of each interval are usually
/// written, by its size in semitones: a minor third (3) is two letters
/// up, C to Eb. The tritone (6) is written as an augmented fourth, C to
/// F#.
const INTERVAL_LETTERS: [u8; 12] = [0, 1, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6];

/// The full name of the interval from `low` up to `high`, as their
/// letters and their distance in semitones say, such as "minor third",
/// "augmented second" or "diminished fifth".
///
/// The letters give the number: C to E is a third (C, D, E), and so is
/// C to Eb or C# to E. The semitones give the quality: a third of four
/// semitones is major, of three minor. C up to D# is an augmented
/// second, even though it sounds like the minor third C up to Eb.
pub fn spelled_interval_name(low: NoteName, high: NoteName) -> String {
    const NUMBERS: [&str; 7] = [
        "unison", "second", "third", "fourth", "fifth", "sixth", "seventh",
    ];
    // The size of the major (or perfect) interval of each number.
    const MAJOR: [i8; 7] = [0, 2, 4, 5, 7, 9, 11];
    let letters = usize::from(low.letter.steps_up_to(high.letter));
    let semitones = low.pitch_class().semitones_up_to(high.pitch_class()) as i8;
    // How far it is from the major or perfect interval, from -6 to 5.
    let off = (semitones - MAJOR[letters] + 18).rem_euclid(12) - 6;
    let perfect = matches!(letters, 0 | 3 | 4);
    let quality = match (perfect, off) {
        (true, 0) => "perfect",
        (false, 0) => "major",
        (false, -1) => "minor",
        (_, 1) => "augmented",
        (true, -1) | (false, -2) => "diminished",
        (_, 2) => "doubly augmented",
        _ => "doubly diminished",
    };
    format!("{quality} {}", NUMBERS[letters])
}

/// The two notes of an interval written in `scale`, and the interval's
/// name. Both notes are written as the key writes them, unless that
/// makes an odd interval out of a note from outside the scale: then the
/// upper note is written the usual way for the interval's size. In C
/// major, C and Eb make a minor third; in A harmonic minor, F and G#
/// really are an augmented second, the scale's famous gap.
fn spell_interval(scale: Scale, low: PitchClass, high: PitchClass) -> (NoteName, NoteName, String) {
    let low_name = scale.spell(low);
    let key_high = scale.spell(high);
    let name = spelled_interval_name(low_name, key_high);
    let semitones = low.semitones_up_to(high);
    let plain =
        name.starts_with("perfect") || name.starts_with("major") || name.starts_with("minor");
    let tritone = matches!(name.as_str(), "augmented fourth" | "diminished fifth");
    let both_in_scale = scale.is_heptatonic() && scale.contains(low) && scale.contains(high);
    if plain || tritone || both_in_scale {
        return (low_name, key_high, name);
    }
    let high_name = low_name.above(semitones, INTERVAL_LETTERS[semitones as usize]);
    (
        low_name,
        high_name,
        spelled_interval_name(low_name, high_name),
    )
}

/// "a" or "an", whichever goes before `word`.
fn article(word: &str) -> &'static str {
    if word.starts_with(['a', 'e', 'i', 'o', 'u']) {
        "an"
    } else {
        "a"
    }
}

/// The name of an interval, named in English by [`interval_name`] or
/// [`spelled_interval_name`], in the current language: "minor third" is
/// "tercera menor" in Spanish, where the number comes first and the
/// quality agrees with it.
pub fn shown_interval(name: &str) -> String {
    if Language::current() == Language::English {
        return name.to_string();
    }
    let (quality, number) = name.rsplit_once(' ').unwrap_or(("", name));
    let (number, masculine) = match number {
        "unison" => ("unísono", true),
        "second" => ("segunda", false),
        "third" => ("tercera", false),
        "fourth" => ("cuarta", false),
        "fifth" => ("quinta", false),
        "sixth" => ("sexta", false),
        "seventh" => ("séptima", false),
        "octave" => ("octava", false),
        "tritone" => ("tritono", true),
        _ => return crate::i18n::tr_str(name).to_string(),
    };
    let quality = match (quality, masculine) {
        ("", _) => "",
        ("perfect", false) => "justa",
        ("perfect", true) => "justo",
        ("major", _) => "mayor",
        ("minor", _) => "menor",
        ("augmented", false) => "aumentada",
        ("augmented", true) => "aumentado",
        ("diminished", false) => "disminuida",
        ("diminished", true) => "disminuido",
        ("doubly augmented", false) => "doblemente aumentada",
        ("doubly augmented", true) => "doblemente aumentado",
        ("doubly diminished", false) => "doblemente disminuida",
        (_, _) => "doblemente disminuido",
    };
    if quality.is_empty() {
        number.to_string()
    } else {
        format!("{number} {quality}")
    }
}

/// An interval named in English, with "a" or "an" before it, in the
/// current language: "a minor third", "una tercera menor".
pub fn interval_with_article(name: &str) -> String {
    match Language::current() {
        Language::English => format!("{} {name}", article(name)),
        Language::Spanish => {
            let shown = shown_interval(name);
            let article = if shown.starts_with("unísono") || shown.starts_with("tritono") {
                "un"
            } else {
                "una"
            };
            format!("{article} {shown}")
        }
    }
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
    /// Two chords at once, one over the other: a **polychord**, such as
    /// the E♭7 over F♭ major that opens the Augurs of Spring in The Rite
    /// of Spring. Written with a bar between them, the upper chord first:
    /// "E♭7|F♭". `bass` is the lowest note, usually the lower chord's
    /// root. Only [`hear`](super::hear) finds these: the notes alone
    /// cannot say which belong to which chord.
    Polychord {
        upper: Chord,
        lower: Chord,
        bass: PitchClass,
    },
    /// Three or more pitch classes that are not a chord we know, named
    /// by their set class, such as "4-23" for a stack of fourths.
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
    /// Every pitch class of the harmony, the bass's too: both chords of a
    /// polychord.
    pub fn pitch_set(&self) -> PitchSet {
        let mut set = match *self {
            Harmony::Silence => PitchSet::EMPTY,
            Harmony::Single(pitch_class) => [pitch_class].into_iter().collect(),
            Harmony::Interval { low, high, .. } => [low, high].into_iter().collect(),
            Harmony::Chord { chord, .. } => chord.pitch_set(),
            Harmony::Polychord { upper, lower, .. } => upper
                .pitch_set()
                .iter()
                .chain(lower.pitch_set().iter())
                .collect(),
            Harmony::Unknown(set) => set,
        };
        if let Harmony::Chord { bass, .. } | Harmony::Polychord { bass, .. } = *self {
            set.insert(bass);
        }
        set
    }

    /// A short label for the chord lane above the piano roll, such as
    /// "Am", "C/E" or "C5", with the black keys named by `spelling`.
    /// `None` for silence. [`Harmony::label_in`] spells it in a key.
    pub fn label(&self, spelling: Spelling) -> Option<String> {
        match *self {
            Harmony::Silence => None,
            Harmony::Single(pitch_class) => Some(pitch_class.name(spelling).to_string()),
            // A root and its fifth is a "power chord", written C5.
            Harmony::Interval {
                low, semitones: 7, ..
            } => Some(format!("{}5", low.name(spelling))),
            // A dash, not a plus: "C+" would read as C augmented.
            Harmony::Interval { low, high, .. } => {
                Some(format!("{}–{}", low.name(spelling), high.name(spelling)))
            }
            Harmony::Chord { chord, bass } if bass != chord.root => {
                Some(format!("{}/{}", chord.name(spelling), bass.name(spelling)))
            }
            Harmony::Chord { chord, .. } => Some(chord.name(spelling)),
            Harmony::Polychord {
                upper, lower, bass, ..
            } => {
                let lower = Harmony::Chord { chord: lower, bass }.label(spelling)?;
                Some(format!("{}|{lower}", upper.name(spelling)))
            }
            Harmony::Unknown(set) => Some(set.set_class().name()),
        }
    }

    /// The short label spelled in `scale`, the way the key writes its
    /// notes: "Eb" rather than "D#" in C major, "A/C#" in D minor, and
    /// "E#dim" in F# major. `None` for silence.
    pub fn label_in(&self, scale: Scale) -> Option<String> {
        match *self {
            Harmony::Silence => None,
            Harmony::Single(pitch_class) => Some(scale.spell(pitch_class).to_string()),
            Harmony::Interval {
                low, semitones: 7, ..
            } => Some(format!("{}5", scale.spell(low))),
            Harmony::Interval { low, high, .. } => {
                let (low, high, _) = spell_interval(scale, low, high);
                Some(format!("{low}–{high}"))
            }
            Harmony::Chord { chord, bass } => {
                let root = scale.spell_root(chord);
                let name = chord.name_from(root);
                match chord.spell_note(root, bass) {
                    Some(bass) if bass.pitch_class() != chord.root => {
                        Some(format!("{name}/{bass}"))
                    }
                    _ => Some(name),
                }
            }
            Harmony::Polychord { upper, lower, bass } => {
                let lower = Harmony::Chord { chord: lower, bass }.label_in(scale)?;
                Some(format!("{}|{lower}", scale.chord_name(upper)))
            }
            Harmony::Unknown(set) => Some(set.set_class().name()),
        }
    }

    /// A sentence describing the harmony in the context of a key, for
    /// the line under the piano roll toolbar.
    ///
    /// A chord is named by its Roman numeral and what it is to the key:
    /// one of the key's chords, a chord **borrowed** from the parallel
    /// key (Fm in C major comes from C minor), a **secondary dominant**
    /// (E major in C major is the V of Am, "V/vi"), or simply a chord
    /// from outside the key.
    pub fn describe(&self, scale: Scale) -> String {
        let key = key_name(scale);
        match *self {
            Harmony::Silence => tr("Nothing is playing here.").to_string(),
            Harmony::Single(pitch_class) => {
                let name = scale.spell(pitch_class);
                match scale.degree_of(pitch_class) {
                    Some(degree) => trf("{}: note {} of {}.", &[&name, &(degree + 1), &key]),
                    None => trf("{}: not in {}.", &[&name, &key]),
                }
            }
            Harmony::Interval { low, high, .. } => {
                let (low, high, name) = spell_interval(scale, low, high);
                let tritone = if matches!(name.as_str(), "augmented fourth" | "diminished fifth") {
                    tr(", a tritone")
                } else {
                    ""
                };
                trf(
                    "{} up to {}: {}{}.",
                    &[&low, &high, &interval_with_article(&name), &tritone],
                )
            }
            Harmony::Chord { chord, bass } => format!(
                "{}: {}",
                self.label_in(scale).unwrap_or_default(),
                describe_chord(
                    scale,
                    chord,
                    bass,
                    scale.spell_root(chord),
                    scale.figured_numeral(chord, bass)
                )
            ),
            Harmony::Polychord { upper, lower, bass } => {
                let numerals = scale
                    .figured_numeral(upper, upper.root)
                    .zip(scale.figured_numeral(lower, bass))
                    .map(|(upper, lower)| trf(" ({} over {} in {})", &[&upper, &lower, &key]))
                    .unwrap_or_default();
                trf(
                    "{}: {} {} over {} {}, two chords at once: a polychord{}.",
                    &[
                        &self.label_in(scale).unwrap_or_default(),
                        &scale.spell_root(upper),
                        &tr(upper.quality.name()),
                        &scale.spell_root(lower),
                        &tr(lower.quality.name()),
                        &numerals,
                    ],
                )
            }
            Harmony::Unknown(pitch_set) => {
                let names: Vec<String> = pitch_set
                    .normal_form()
                    .into_iter()
                    .map(|p| scale.spell(p).to_string())
                    .collect();
                let class = pitch_set.set_class();
                let nickname = class
                    .nickname()
                    .map(|name| format!(", {}", tr(name)))
                    .unwrap_or_default();
                trf(
                    "{}: no chord name for these notes; set class {} {}{}.",
                    &[
                        &names.join(" "),
                        &class.name(),
                        &class.prime_text(),
                        &nickname,
                    ],
                )
            }
        }
    }
}

/// What `chord` is in `scale`, with `bass` lowest, written with the root
/// `root` and numbered `numeral`: "A dominant seventh, V7/ii in C
/// major." The harmony keys spell and number a chord from its sound; the
/// chord sheet passes the root and numeral as they were written.
pub fn describe_chord(
    scale: Scale,
    chord: Chord,
    bass: PitchClass,
    root: NoteName,
    numeral: Option<String>,
) -> String {
    let key = key_name(scale);
    let mut text = format!("{root} {}", tr(chord.quality.name()));
    if bass != chord.root
        && let Some(bass) = chord.spell_note(root, bass)
    {
        text += &trf(" with {} in the bass", &[&bass]);
    }
    let figures = chord.inversion_figures(bass);
    text += &match numeral {
        Some(numeral) if scale.is_diatonic(chord) => trf(", chord {} of {}.", &[&numeral, &key]),
        Some(numeral) if scale.uses_raised_seventh(chord) => {
            let leading = scale.spell(scale.root.transpose(11));
            trf(
                ", chord {} of {}, with the raised seventh ({}) of harmonic minor.",
                &[&numeral, &key, &leading],
            )
        }
        Some(numeral) => match (
            scale.borrowed_from(chord),
            scale.secondary_dominant_target(chord),
        ) {
            (Some(parallel), _) => trf(", {} borrowed from {}.", &[&numeral, &key_name(parallel)]),
            (None, Some(target)) => trf(
                ", {} in {}: a secondary dominant, the dominant of {}.",
                &[
                    &with_figures(
                        &format!(
                            "V{}/{}",
                            chord.quality.numeral_suffix(),
                            scale.roman_numeral(target).unwrap_or_default()
                        ),
                        figures,
                    ),
                    &key,
                    &scale.chord_name(target),
                ],
            ),
            (None, None) => trf(", {}, from outside {}.", &[&numeral, &key]),
        },
        None if scale.is_diatonic(chord) => trf(", in {}.", &[&key]),
        None => trf(", outside {}.", &[&key]),
    };
    text
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
        assert_eq!(label(&[60, 64]).unwrap(), "C–E");
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
    fn extended_chords() {
        assert_eq!(label(&[48, 64, 67, 69, 74]).unwrap(), "C6/9");
        assert_eq!(label(&[55, 59, 62, 65, 68]).unwrap(), "G7b9");
        // E, G#, B, D and G: the "Hendrix chord".
        assert_eq!(label(&[52, 56, 59, 62, 67]).unwrap(), "E7#9");
        assert_eq!(label(&[55, 59, 62, 65, 69, 72]).unwrap(), "G11");
        assert_eq!(label(&[45, 48, 52, 55, 59, 62]).unwrap(), "Am11");
        assert_eq!(label(&[48, 64, 67, 71, 74, 81]).unwrap(), "Cmaj13");
        // Thirteenth chords are often played without their fifth too.
        assert_eq!(label(&[48, 64, 70, 74, 81]).unwrap(), "C13");
    }

    #[test]
    fn dominants_with_an_altered_fifth() {
        // C E Gb Bb: the French sixth sound, named from its bass.
        assert_eq!(label(&[48, 64, 66, 70]).unwrap(), "C7b5");
        assert_eq!(label(&[54, 58, 60, 64]).unwrap(), "F#7b5");
        assert_eq!(label(&[48, 64, 68, 70]).unwrap(), "C7#5");
        let g7_sharp5 = analyze(notes(&[55, 59, 63, 65]));
        assert_eq!(g7_sharp5.label_in(Scale::default()).unwrap(), "G7#5");
        let Harmony::Chord { chord, .. } = g7_sharp5 else {
            panic!("G B D# F is a chord");
        };
        let names: Vec<String> = Scale::default()
            .chord_notes(chord)
            .iter()
            .map(|name| name.to_string())
            .collect();
        assert_eq!(names, ["G", "B", "D#", "F"]);
    }

    #[test]
    fn unknown_clusters() {
        assert_eq!(label(&[60, 61, 62]).unwrap(), "3-1");
    }

    #[test]
    fn descriptions_use_the_key() {
        let c_major = Scale::default();
        let describe = |midi: &[u8]| analyze(notes(midi)).describe(c_major);
        assert_eq!(describe(&[57, 60, 64]), "Am: A minor, chord vi of C major.");
        assert_eq!(
            describe(&[64, 68, 71]),
            "E: E major, V/vi in C major: a secondary dominant, the dominant of Am."
        );
        assert_eq!(describe(&[60, 67]), "C up to G: a perfect fifth.");
        assert_eq!(describe(&[69]), "A: note 6 of C major.");
        assert_eq!(
            describe(&[64, 67, 72]),
            "C/E: C major with E in the bass, chord I⁶ of C major."
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

    fn describe_in(scale: Scale, midi: &[u8]) -> String {
        analyze(notes(midi)).describe(scale)
    }

    #[test]
    fn inversions_get_the_figures_of_figured_bass() {
        let c = Scale::default();
        // C major: root position, first and second inversion.
        assert!(describe_in(c, &[60, 64, 67]).ends_with("chord I of C major."));
        let first = describe_in(c, &[52, 60, 67]);
        assert!(
            first.starts_with("C/E: C major with E in the bass"),
            "{first}"
        );
        assert!(first.ends_with("chord I⁶ of C major."), "{first}");
        assert!(describe_in(c, &[55, 60, 64]).ends_with("chord I⁶₄ of C major."));
        // G7 and its three inversions: V⁶₅, V⁴₃, V⁴₂.
        assert!(describe_in(c, &[55, 59, 62, 65]).ends_with("chord V7 of C major."));
        assert!(describe_in(c, &[59, 62, 65, 67]).ends_with("chord V⁶₅ of C major."));
        assert!(describe_in(c, &[50, 65, 67, 71]).ends_with("chord V⁴₃ of C major."));
        assert!(describe_in(c, &[53, 67, 71, 74]).ends_with("chord V⁴₂ of C major."));
        // A secondary dominant keeps its target: E7/G# is V⁶₅/vi.
        let e7 = describe_in(c, &[56, 62, 64, 71]);
        assert!(e7.starts_with("E7/G#"), "{e7}");
        assert!(e7.contains(", V⁶₅/vi in C major"), "{e7}");
        // iiø7 in A minor, with its seventh in the bass.
        let a = Scale::new(A, ScaleKind::NaturalMinor);
        let half = describe_in(a, &[57, 59, 62, 65]);
        assert!(half.starts_with("Bm7b5/A"), "{half}");
        assert!(half.contains("chord iiø⁴₂ of A minor"), "{half}");
    }

    #[test]
    fn labels_are_spelled_in_the_key() {
        let in_key = |scale: Scale, midi: &[u8]| analyze(notes(midi)).label_in(scale).unwrap();
        let c = Scale::default();
        // Eb major, the chord borrowed from C minor, not D# major.
        assert_eq!(in_key(c, &[63, 67, 70]), "Eb");
        assert_eq!(in_key(c, &[60, 63]), "C–Eb");
        let d_minor = Scale::new(D, ScaleKind::NaturalMinor);
        assert_eq!(in_key(d_minor, &[61, 64, 69]), "A/C#");
        let f_sharp = Scale::new(FSharp, ScaleKind::Major);
        assert_eq!(in_key(f_sharp, &[65, 68, 71]), "E#dim");
        assert_eq!(in_key(f_sharp, &[65]), "E#");
    }

    #[test]
    fn intervals_are_named_by_their_spelling() {
        let c = Scale::default();
        assert_eq!(describe_in(c, &[60, 63]), "C up to Eb: a minor third.");
        assert_eq!(describe_in(c, &[60, 70]), "C up to Bb: a minor seventh.");
        assert_eq!(describe_in(c, &[60, 68]), "C up to Ab: a minor sixth.");
        assert_eq!(
            describe_in(c, &[60, 66]),
            "C up to F#: an augmented fourth, a tritone."
        );
        let f = Scale::new(F, ScaleKind::Major);
        assert_eq!(describe_in(f, &[62, 66]), "D up to F#: a major third.");
        // The gap of harmonic minor is an augmented second.
        let a_harmonic = Scale::new(A, ScaleKind::HarmonicMinor);
        assert_eq!(
            describe_in(a_harmonic, &[65, 68]),
            "F up to G#: an augmented second."
        );
        assert_eq!(
            spelled_interval_name(
                NoteName::new(crate::Letter::B, 0),
                NoteName::new(crate::Letter::F, 0)
            ),
            "diminished fifth"
        );
    }

    #[test]
    fn a_minor_key_owns_its_raised_dominant() {
        let a_minor = Scale::new(A, ScaleKind::NaturalMinor);
        assert_eq!(
            describe_in(a_minor, &[52, 56, 59, 62]),
            "E7: E dominant seventh, chord V7 of A minor, with the raised seventh (G#) of harmonic minor."
        );
        assert_eq!(
            describe_in(a_minor, &[56, 59, 62, 65]),
            "G#dim7: G# diminished seventh, chord vii°7 of A minor, with the raised seventh (G#) of harmonic minor."
        );
        let d_minor = Scale::new(D, ScaleKind::NaturalMinor);
        assert_eq!(
            describe_in(d_minor, &[57, 61, 64]),
            "A: A major, chord V of D minor, with the raised seventh (C#) of harmonic minor."
        );
    }

    #[test]
    fn borrowed_chords_and_secondary_dominants() {
        let c = Scale::default();
        assert_eq!(
            describe_in(c, &[53, 56, 60]),
            "Fm: F minor, iv borrowed from C minor."
        );
        assert_eq!(
            describe_in(c, &[58, 62, 65]),
            "Bb: Bb major, bVII borrowed from C minor."
        );
        assert_eq!(
            describe_in(c, &[57, 61, 64, 67]),
            "A7: A dominant seventh, V7/ii in C major: a secondary dominant, the dominant of Dm."
        );
        // The Neapolitan chord is not in C minor either.
        assert_eq!(
            describe_in(c, &[61, 65, 68]),
            "Db: Db major, bII, from outside C major."
        );
        assert_eq!(
            describe_in(c, &[54, 58, 61]),
            "F#: F# major, #IV, from outside C major."
        );
        // A minor key borrows from its parallel major.
        let a_minor = Scale::new(A, ScaleKind::NaturalMinor);
        assert_eq!(
            describe_in(a_minor, &[57, 61, 64]),
            "A: A major, I borrowed from A major."
        );
    }
}
