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
//!
//! Functions belong to major and minor keys. The modes (Dorian,
//! Phrygian, Lydian, Mixolydian, Locrian) have no dominant that pulls
//! home the way V does, so their keys get no function: in C Locrian the
//! chord on the fifth note is Gb major, which does not pull to C at all.
//!
//! Music without a key has no home and no functions. For a twelve-tone
//! row or a pitch-class set, the keys play groups of its notes instead:
//! see [`group_keys`]. A key of three notes, one of Kröpfl's micromodes,
//! has the micromode's own forms as its keys, each named by its degree:
//! see [`form_keys`].

use super::degree::{self, Degree};
use super::{Chord, Harmony, Micromode, PitchSet, Scale, ScaleKind, analyze};
use crate::library::voicing::voice;
use crate::pitch::{Note, NoteName, PitchClass};

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
            Function::Dominant => "tension: pulls back home to the tonic",
        }
    }

    /// The feel in one word.
    pub fn short_feel(self) -> &'static str {
        match self {
            Function::Tonic => "home",
            Function::Subdominant => "away",
            Function::Dominant => "tension",
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
    /// What the chord does in the key, or `None` in a mode, where the
    /// chords have no functions (see the module documentation).
    pub function: Option<Function>,
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
                function: has_functions(scale.kind).then(|| Function::of(degree)),
            })
        })
        .collect()
}

/// Whether the chords of a scale do the jobs of tonic, subdominant and
/// dominant: in major and minor keys, not in the modes.
pub fn has_functions(kind: ScaleKind) -> bool {
    matches!(
        kind,
        ScaleKind::Major
            | ScaleKind::NaturalMinor
            | ScaleKind::HarmonicMinor
            | ScaleKind::MelodicMinor
    )
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
/// key's chords: the same root, and the same kind of triad (major,
/// minor or diminished) as the key's chord there. G7 lights V in C
/// major; D7 lights nothing, because the key's chord on D is minor: D7
/// comes from outside the key. Bm lights nothing either, since the
/// key's chord on B is diminished.
pub fn key_for(keys: &[ChordKey], chord: Chord) -> Option<usize> {
    keys.iter().position(|key| {
        key.chord.root == chord.root && key.chord.quality.family() == chord.quality.family()
    })
}

/// A harmony key for music without a key: a group of notes, such as a
/// trichord of a twelve-tone row, or a chord stacked from a set's notes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupKey {
    /// What the key is called: "1–3" for the first three notes of a row;
    /// for a set, the chord its notes make, such as "Cm/Eb", or its set
    /// class, such as "3-5", if they make no chord with a name.
    pub label: String,
    /// The notes in the order they are stacked, lowest first.
    pub notes: Vec<PitchClass>,
    /// How each note is written. Notes that make a chord are spelled as
    /// that chord: Eb G C, not D# G C, for C minor over Eb.
    pub names: Vec<NoteName>,
    /// The chord the notes make when played from the lowest up, such as
    /// "Cm/Eb", if it has a name.
    pub chord: Option<String>,
    /// For a form of a micromode key: its degree in the key (see
    /// [`form_keys`]).
    pub degree: Option<Degree>,
    /// For a form of a micromode key: how many notes it shares with the
    /// key's own form, 3 for that form itself.
    pub kept: Option<usize>,
}

impl GroupKey {
    /// The key for `notes`, called `label` (for a set, `None` means "call
    /// it by its chord, or else its set class").
    fn new(scale: Scale, label: Option<String>, notes: Vec<PitchClass>) -> GroupKey {
        let harmony = analyze(stacked(&notes));
        let names = match harmony {
            // A row's notes keep the names the row writes them by: its
            // group A F F# is not written A E# F#, as the chord F#m(maj7)
            // it sounds like would be.
            Harmony::Chord { .. } if matches!(scale.kind, ScaleKind::Row(_)) => {
                notes.iter().map(|&p| scale.spell(p)).collect()
            }
            Harmony::Chord { chord, .. } => {
                let root = scale.spell_root(chord);
                notes
                    .iter()
                    .map(|&p| chord.spell_note(root, p).unwrap_or_else(|| scale.spell(p)))
                    .collect()
            }
            _ => notes.iter().map(|&p| scale.spell(p)).collect(),
        };
        let chord = match harmony {
            Harmony::Chord { .. } => harmony.label_in(scale),
            _ => None,
        };
        let label = label.unwrap_or_else(|| {
            chord.clone().unwrap_or_else(|| {
                notes
                    .iter()
                    .copied()
                    .collect::<PitchSet>()
                    .set_class()
                    .name()
            })
        });
        GroupKey {
            label,
            notes,
            names,
            chord,
            degree: None,
            kept: None,
        }
    }

    /// The key for `form`, a form of the micromode the key `scale` is:
    /// called by its degree, its notes from the root up for a triad and
    /// in normal form otherwise, and named as a chord only when it is
    /// one (a triad). C C# E is Menor 3, not an incomplete C#m(maj7).
    fn form(scale: Scale, form: PitchSet, degree: Degree) -> GroupKey {
        let chord = Micromode::of(form).and_then(|micromode| micromode.chord(form));
        let (notes, names) = match chord {
            Some(chord) => {
                let names = chord.spell(scale.spell_root(chord));
                (names.iter().map(|name| name.pitch_class()).collect(), names)
            }
            None => {
                let notes = form.normal_form();
                let names = notes.iter().map(|&p| scale.spell(p)).collect();
                (notes, names)
            }
        };
        let chord = chord.and_then(|chord| {
            Harmony::Chord {
                chord,
                bass: chord.root,
            }
            .label_in(scale)
        });
        GroupKey {
            label: degree.to_string(),
            notes,
            names,
            chord,
            degree: Some(degree),
            kept: Some(degree::kept(scale.pitch_set(), form)),
        }
    }

    pub fn pitch_set(&self) -> PitchSet {
        self.notes.iter().copied().collect()
    }

    /// The notes to play: the first one in the octave from G3, and each
    /// next one just above the one before, so a row's group sounds in the
    /// row's order from the bottom up.
    pub fn play(&self) -> Vec<Note> {
        stacked(&self.notes)
    }
}

/// `notes` stacked upwards: the first in the octave from G3, and each
/// next one just above the one before.
fn stacked(notes: &[PitchClass]) -> Vec<Note> {
    let mut played: Vec<Note> = Vec::with_capacity(notes.len());
    for &pitch_class in notes {
        let note = match played.last() {
            None => Note::new(55 + PitchClass::G.semitones_up_to(pitch_class)),
            Some(&below) => {
                let up = match below.pitch_class().semitones_up_to(pitch_class) {
                    0 => 12,
                    gap => gap,
                };
                below.transpose(i32::from(up)).unwrap_or(below)
            }
        };
        played.push(note);
    }
    played
}

/// The chord sizes the harmony keys offer for `scale`: all three for a
/// key or a row, and for a set only those it has enough notes for. A
/// set of three notes makes three-note chords and nothing bigger.
pub fn chord_sizes(scale: Scale) -> Vec<ChordSize> {
    let notes = match scale.kind {
        ScaleKind::Set(set) => set.len(),
        _ => 12,
    };
    ChordSize::ALL
        .into_iter()
        .filter(|&size| size == ChordSize::Triads || size as usize + 3 <= notes)
        .collect()
}

/// How many notes a group key of a row holds for each chord size:
/// trichords, tetrachords or hexachords, which divide the row evenly.
pub fn row_group(size: ChordSize) -> usize {
    match size {
        ChordSize::Triads => 3,
        ChordSize::Sevenths => 4,
        ChordSize::Ninths => 6,
    }
}

/// The harmony keys of a twelve-tone row or a pitch-class set, or `None`
/// for the scales of tonal music, which have [`chord_keys`].
///
/// - A row is cut into groups in order: four trichords, three
///   tetrachords or two hexachords. This is how twelve-tone composers
///   most often turn a row into chords.
/// - A set gets a chord on each of its notes, stacking every other note
///   of the set, as the chords of a key stack every other note of the
///   scale: three, four or five notes. A set with no more notes than
///   that plays all of them, starting from each in turn.
///
/// A set's keys are called by the chord their notes make, from the
/// lowest up: the set C Eb G gives Cm, Cm/Eb (the same chord with Eb in
/// the bass) and Cm/G. Groups with no chord name are called by their
/// set class.
pub fn group_keys(scale: Scale, size: ChordSize) -> Option<Vec<GroupKey>> {
    if let Some(forms) = form_keys(scale, false) {
        return Some(forms);
    }
    let notes: Vec<PitchClass> = scale.pitch_classes().collect();
    match scale.kind {
        ScaleKind::Row(_) => {
            let group = row_group(size);
            Some(
                notes
                    .chunks(group)
                    .enumerate()
                    .map(|(i, chunk)| {
                        let label = format!("{}–{}", i * group + 1, i * group + chunk.len());
                        GroupKey::new(scale, Some(label), chunk.to_vec())
                    })
                    .collect(),
            )
        }
        ScaleKind::Set(_) => {
            let len = notes.len();
            let count = size as usize + 3;
            let stack = |start: usize| -> Vec<PitchClass> {
                if len <= count {
                    // All the notes, from this one round.
                    return (0..len).map(|i| notes[(start + i) % len]).collect();
                }
                // Every other note; once that comes round to a note it
                // already has (in a set with an even number of notes),
                // the ones in between.
                let mut chord = Vec::with_capacity(count);
                for i in (0..len)
                    .map(|i| start + 2 * i)
                    .chain((0..len).map(|i| start + 2 * i + 1))
                {
                    let note = notes[i % len];
                    if chord.len() < count && !chord.contains(&note) {
                        chord.push(note);
                    }
                }
                chord
            };
            Some(
                (0..len)
                    .map(|start| GroupKey::new(scale, None, stack(start)))
                    .collect(),
            )
        }
        _ => None,
    }
}

/// The harmony keys of a key of three notes, one of Kröpfl's
/// micromodes, or `None` for any other key.
///
/// Music on one micromode makes its chords and its tunes of the
/// micromode's own forms, moved and turned upside down, so those forms
/// are the keys: the key's own form first, then the forms that keep two
/// of its notes, then one (see [`degree::forms_by_nearness`]), each
/// called by its degree (`T2`, `T5I`). Unless `all`, the forms that share
/// no note with home are left out, except when that would leave fewer
/// than four keys: the augmented triad's four forms share no notes at
/// all, so all four are its keys.
pub fn form_keys(scale: Scale, all: bool) -> Option<Vec<GroupKey>> {
    let ScaleKind::Set(_) = scale.kind else {
        return None;
    };
    let key = scale.pitch_set();
    if key.len() != 3 {
        return None;
    }
    let forms = degree::forms_by_nearness(scale);
    let near = forms
        .iter()
        .filter(|&&(form, _)| degree::kept(key, form) > 0)
        .count();
    let shown = if all || near < 4 { forms.len() } else { near };
    Some(
        forms
            .into_iter()
            .take(shown)
            .map(|(form, degree)| GroupKey::form(scale, form, degree))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ChordQuality, PitchClass, Spelling};

    #[test]
    fn a_rows_groups_keep_the_rows_names() {
        let webern = crate::harmony::RowPreset::named("Webern's Concerto")
            .unwrap()
            .row;
        let row = Scale::row(webern.starting_on(PitchClass::C));
        let groups: Vec<String> = group_keys(row, ChordSize::Triads)
            .unwrap()
            .iter()
            .map(|key| {
                key.names
                    .iter()
                    .map(|name| name.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect();
        assert_eq!(groups, ["C B Eb", "E Ab G", "A F F#", "C# D Bb"]);
    }

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
            [
                "i Am",
                "ii° Bdim",
                "bIII C",
                "iv Dm",
                "V E",
                "bVI F",
                "bVII G"
            ]
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

    #[test]
    fn modes_have_no_functions() {
        let locrian = chord_keys(
            Scale::new(PitchClass::C, ScaleKind::Locrian),
            ChordSize::Triads,
        );
        assert!(locrian.iter().all(|key| key.function.is_none()));
        assert_eq!(locrian[4].numeral, "bV");
        let minor = chord_keys(
            Scale::new(PitchClass::A, ScaleKind::HarmonicMinor),
            ChordSize::Triads,
        );
        assert_eq!(minor[6].function, Some(Function::Dominant));
        // The blues borrows the chords of the minor key, functions too.
        let blues = chord_keys(
            Scale::new(PitchClass::A, ScaleKind::Blues),
            ChordSize::Triads,
        );
        assert_eq!(blues[0].function, Some(Function::Tonic));
    }

    #[test]
    fn a_diminished_key_is_not_lit_by_a_minor_chord() {
        let c = Scale::new(PitchClass::C, ScaleKind::Major);
        let keys = chord_keys(c, ChordSize::Sevenths);
        let b_minor = Chord::new(PitchClass::B, ChordQuality::Minor);
        assert_eq!(key_for(&keys, b_minor), None);
        let b_dim = Chord::new(PitchClass::B, ChordQuality::Diminished);
        assert_eq!(key_for(&keys, b_dim), Some(6));
    }

    #[test]
    fn a_set_names_its_keys_by_their_chords() {
        // Every other note of a four-note set, from each of its notes.
        let seventh = Scale::set(PitchClass::C, PitchSet::from_semitones(&[0, 4, 7, 10]));
        let keys = group_keys(seventh, ChordSize::Triads).unwrap();
        let labels: Vec<&str> = keys.iter().map(|key| key.label.as_str()).collect();
        assert_eq!(labels, ["C", "Edim", "3-7", "C7/Bb"]);
        let names: Vec<String> = keys[1].names.iter().map(|n| n.to_string()).collect();
        assert_eq!(names, ["E", "Bb", "G"]);
        // A group with no chord name is called by its set class.
        assert_eq!(keys[2].chord, None);
        // A three-note set offers only three-note chords.
        let viennese = Scale::set(PitchClass::C, PitchSet::from_semitones(&[0, 1, 6]));
        assert_eq!(chord_sizes(viennese), [ChordSize::Triads]);
        let octatonic = Scale::set(
            PitchClass::C,
            PitchSet::from_semitones(&[0, 1, 3, 4, 6, 7, 9, 10]),
        );
        assert_eq!(chord_sizes(octatonic).len(), 3);
        assert_eq!(chord_sizes(Scale::default()).len(), 3);
    }

    #[test]
    fn a_row_plays_its_trichords_in_order() {
        let row = crate::harmony::ROW_PRESETS[0].row;
        let keys = group_keys(Scale::row(row), ChordSize::Triads).unwrap();
        let labels: Vec<&str> = keys.iter().map(|key| key.label.as_str()).collect();
        assert_eq!(labels, ["1–3", "4–6", "7–9", "10–12"]);
        assert_eq!(keys[0].notes, [PitchClass::E, PitchClass::F, PitchClass::G]);
        assert_eq!(keys[0].chord, None);
        let played: Vec<u8> = keys[0].play().iter().map(|n| n.midi()).collect();
        assert_eq!(played, [64, 65, 67]);
        assert_eq!(
            group_keys(Scale::row(row), ChordSize::Ninths)
                .unwrap()
                .len(),
            2
        );
        assert_eq!(group_keys(Scale::default(), ChordSize::Triads), None);
        // The four triads of the "Four triads" row are chords, spelled as such.
        let triads = crate::harmony::RowPreset::named("Four triads").unwrap().row;
        let keys = group_keys(Scale::row(triads), ChordSize::Triads).unwrap();
        let chords: Vec<String> = keys.iter().filter_map(|key| key.chord.clone()).collect();
        assert_eq!(chords, ["C", "D", "Bbm", "G#m/B"]);
        assert_eq!(keys[1].label, "4–6");
    }

    #[test]
    fn a_set_stacks_every_other_note() {
        // The octatonic scale on C: C C# D# E F# G A A#.
        let octatonic = Scale::set(
            PitchClass::C,
            PitchSet::from_semitones(&[0, 1, 3, 4, 6, 7, 9, 10]),
        );
        let keys = group_keys(octatonic, ChordSize::Triads).unwrap();
        assert_eq!(keys.len(), 8);
        // Every other note from C: C D# F#, a diminished triad.
        assert_eq!(keys[0].pitch_set().set_class().name(), "3-10");
        // The whole-tone scale has only three every-other notes; the
        // fourth is the next one.
        let whole_tone = Scale::set(
            PitchClass::C,
            PitchSet::from_semitones(&[0, 2, 4, 6, 8, 10]),
        );
        let keys = group_keys(whole_tone, ChordSize::Sevenths).unwrap();
        assert_eq!(keys[0].pitch_set(), PitchSet::from_semitones(&[0, 4, 8, 2]));
    }

    #[test]
    fn a_micromode_key_has_its_forms_as_keys() {
        // C minor: home, then the three triads that keep two of its
        // notes (Ab, C and Eb major), then those that keep one.
        let minor = Scale::set(PitchClass::C, PitchSet::from_semitones(&[0, 3, 7]));
        let keys = group_keys(minor, ChordSize::Triads).unwrap();
        let labels: Vec<&str> = keys.iter().take(4).map(|key| key.label.as_str()).collect();
        assert_eq!(labels, ["T0", "T3I", "T7I", "T10I"]);
        let chords: Vec<String> = keys
            .iter()
            .take(4)
            .filter_map(|k| k.chord.clone())
            .collect();
        assert_eq!(chords, ["Cm", "Ab", "C", "Eb"]);
        let kept: Vec<usize> = keys.iter().filter_map(|key| key.kept).collect();
        assert_eq!(kept[..4], [3, 2, 2, 2]);
        assert!(kept.iter().all(|&kept| kept > 0), "none shares no note");
        let names: Vec<String> = keys[1].names.iter().map(|n| n.to_string()).collect();
        assert_eq!(names, ["Ab", "C", "Eb"]);
        // All of them: the 24 major and minor triads.
        assert_eq!(form_keys(minor, true).unwrap().len(), 24);
        // Menor 3 is not an incomplete C#m(maj7): it has no chord name.
        let menor_3 = Scale::set(PitchClass::C, PitchSet::from_semitones(&[0, 1, 4]));
        let keys = group_keys(menor_3, ChordSize::Sevenths).unwrap();
        assert_eq!(keys[0].label, "T0");
        assert!(keys.iter().all(|key| key.chord.is_none()));
        assert!(keys.iter().all(|key| key.notes.len() == 3));
        assert_eq!(keys.len(), 13);
        let played: Vec<u8> = keys[0].play().iter().map(|n| n.midi()).collect();
        assert_eq!(played, [60, 61, 64]);
        // The augmented triad's forms share no note: all four are keys.
        let augmented = Scale::set(PitchClass::C, PitchSet::from_semitones(&[0, 4, 8]));
        assert_eq!(group_keys(augmented, ChordSize::Triads).unwrap().len(), 4);
        // Only a key of three notes has forms as its keys.
        assert_eq!(form_keys(Scale::default(), false), None);
        let seventh = Scale::set(PitchClass::C, PitchSet::from_semitones(&[0, 4, 7, 10]));
        assert_eq!(form_keys(seventh, false), None);
    }
}
