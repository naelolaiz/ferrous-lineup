//! The chord sheet's harmony in other notations: as chord symbols, as
//! pitch-class sets or as micromodes, and the notes a song plays cut
//! into sets, vertically or horizontally.
//!
//! A [`Notation`] renames the harmonies written on the sheet and keeps
//! their notes, the way a key shown as a pitch-class set keeps every
//! note of the song: C major is the set 3-11 T7I and the micromode
//! Acorde menor-mayor T7I, and the set D F A is the chord Dm. Two chords
//! at once stay two groups, so the Augurs chord Eb7|Fb is the set of
//! Eb7 over the set of Fb.
//!
//! Sets and micromodes can also be read from the notes themselves, in
//! the two ways an analyst cuts music into sets ([`Grouping`]):
//!
//! - **Vertically**, the notes sounding together: wherever a note
//!   starts, all the notes held then, from the lowest up, in groups of
//!   so many different notes. The Petrushka chord, C E G under F# A# C#,
//!   in groups of three is two trichords, written upper first as two
//!   chords at once are: "3-11 T1I|3-11 T7I".
//! - **Horizontally**, the notes one after another, as a twelve-tone row
//!   is cut into groups: the first so many notes are a group, the next
//!   ones the next group, and so on. The row of Webern's Concerto Op. 24
//!   cut into threes is four groups of the one micromode Menor 3 (3-3):
//!   B Bb D, Eb G F#, Ab E F, C C# A; in sixes, its two halves are each
//!   the hexatonic set 6-20.
//! - **Mixed**, where the key is a set or a micromode: the forms of the
//!   key's set as the colour check reads them
//!   ([`crate::harmony::voice_leading::colour`]), each heard partly
//!   together and partly one after another, as the music has it; and
//!   where two tracks, or a top line and the chords under it, are each in
//!   a form of their own at once, both forms, written as two chords at
//!   once are.
//!
//! Whichever way the groups are read, [`most_used`] adds up how long
//! each class of sets or micromodes sounds in them, the class that
//! sounds longest first: the most used micromode of a passage, or its
//! most used set.
//!
//! Nothing here changes the song: the chord sheet shows what these
//! functions find, and what is written stays as it was.

use serde::{Deserialize, Serialize};

use super::HarmonyName;
use crate::harmony::voice_leading::{FormStatement, HeardIn, Voice, colour, voices_of};
use crate::harmony::{
    ChordQuality, ChordSymbol, Harmony, HeardNote, Micromode, PitchSet, Scale, ScaleKind,
};
use crate::pitch::Note;
use crate::{Chord, KeyMap, NoteEvent, Project};

/// How the chord sheet names each harmony.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Notation {
    /// In the notation of the key the harmony is in (see
    /// [`Notation::in_key`]): chords in a major or minor key or a mode,
    /// pitch-class sets in a set or a twelve-tone row, micromodes in a
    /// micromode. A song in C major is not shown in micromodes because
    /// the last song was.
    #[default]
    Key,
    /// As it is written: chords as chords, sets as sets.
    Written,
    /// Chord symbols, wherever the notes make a chord the app knows. A
    /// set that makes no chord keeps its name.
    Chords,
    /// Pitch-class sets, by Forte name and how each is moved from its
    /// prime form.
    Sets,
    /// Kröpfl's micromodes: every group of three notes is one of them.
    /// A harmony of more or fewer notes is named as its set.
    Micromodes,
}

impl Notation {
    pub const ALL: [Notation; 5] = [
        Notation::Key,
        Notation::Written,
        Notation::Chords,
        Notation::Sets,
        Notation::Micromodes,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Notation::Key => "As the key",
            Notation::Written => "As written",
            Notation::Chords => "Chords",
            Notation::Sets => "Pitch-class sets",
            Notation::Micromodes => "Micromodes",
        }
    }

    /// What it shows, for hints.
    pub fn describe(self) -> &'static str {
        match self {
            Notation::Key => {
                "Each harmony in the notation of the key it is in: chords in a major or minor \
                 key or a mode, pitch-class sets in a set or a twelve-tone row, micromodes in \
                 a micromode"
            }
            Notation::Written => {
                "Each harmony as it is written: chords as chords, sets and micromodes as \
                 they were typed or heard"
            }
            Notation::Chords => {
                "Chord symbols with their Roman numerals: a set or a micromode whose notes \
                 make a chord is written as that chord (D F A is Dm); one that makes none \
                 keeps its name"
            }
            Notation::Sets => {
                "Every harmony as its pitch-class set, by Forte name, on the same notes: \
                 C major is 3-11 T7I. Two chords at once are two sets, the upper first"
            }
            Notation::Micromodes => {
                "Every group of three notes as one of Kröpfl's micromodes, on the same \
                 notes: C major is Acorde menor-mayor T7I. A harmony of more or fewer \
                 notes is shown as its set"
            }
        }
    }

    /// The notation [`Notation::Key`] stands for where the song is in
    /// `key`: chords in a key of tonal music, one of its modes or the
    /// chromatic scale; micromodes in a key of three notes named as one
    /// (see [`Scale::micromode`]); pitch-class sets in any other set,
    /// and in a twelve-tone row. Any other notation is itself.
    pub fn in_key(self, key: Scale) -> Notation {
        match self {
            Notation::Key if key.micromode().is_some() => Notation::Micromodes,
            Notation::Key if matches!(key.kind, ScaleKind::Set(_) | ScaleKind::Row(_)) => {
                Notation::Sets
            }
            Notation::Key => Notation::Chords,
            other => other,
        }
    }

    /// Whether the song's own notes can be read in this notation, in
    /// groups ([`Grouping`]): only sets and micromodes, which name any
    /// group of notes. [`Notation::Key`] stands for one of them only in
    /// a key (see [`Notation::in_key`]), so it says `false`.
    pub fn groups(self) -> bool {
        matches!(self, Notation::Sets | Notation::Micromodes)
    }

    /// The most notes a group can hold, or `None` for as many as there
    /// are: three in a micromode, which is three notes by definition.
    pub fn fixed_size(self) -> Option<usize> {
        (self == Notation::Micromodes).then_some(3)
    }

    /// `harmony` named in this notation where the song is in `key`, as
    /// groups of notes from the lowest up: a single one for a chord or a
    /// set, the lower chord and then the upper for two chords at once.
    /// No chord and a row form stay as they are: a row is a line of
    /// twelve notes, not a group.
    pub fn rename(self, harmony: HarmonyName, key: Scale) -> Vec<HarmonyName> {
        let notation = self.in_key(key);
        let group = |set: PitchSet| notation.name_group(set, key);
        match (notation, harmony) {
            (Notation::Written, _)
            | (_, HarmonyName::NoChord)
            | (_, HarmonyName::Row(_))
            | (Notation::Chords, HarmonyName::Chord(_) | HarmonyName::Polychord(..)) => {
                vec![harmony]
            }
            (Notation::Chords, HarmonyName::Set(set) | HarmonyName::Micromode(set)) => {
                match chord_of(set) {
                    Some(chord) => vec![HarmonyName::Chord(ChordSymbol::new(
                        key.spell_root(chord),
                        chord.quality,
                    ))],
                    None => vec![harmony],
                }
            }
            (_, HarmonyName::Polychord(upper, lower)) => {
                // The lower chord with its bass, then the upper chord.
                let lower = notes_of(lower);
                vec![group(lower), group(upper.chord().pitch_set())]
            }
            (_, HarmonyName::Chord(symbol)) => vec![group(notes_of(symbol))],
            (_, HarmonyName::Set(set) | HarmonyName::Micromode(set)) => vec![group(set)],
        }
    }

    /// A group of notes named in this notation: a micromode for three
    /// notes in [`Notation::Micromodes`], a set otherwise, or the chord
    /// it makes in [`Notation::Chords`].
    pub fn name_group(self, set: PitchSet, key: Scale) -> HarmonyName {
        match self.in_key(key) {
            Notation::Micromodes if Micromode::of(set).is_some() => HarmonyName::Micromode(set),
            Notation::Chords | Notation::Written => match chord_of(set) {
                Some(chord) => {
                    HarmonyName::Chord(ChordSymbol::new(key.spell_root(chord), chord.quality))
                }
                None => HarmonyName::Set(set),
            },
            _ => HarmonyName::Set(set),
        }
    }
}

impl Notation {
    /// `harmony`, as heard in the notes (see [`crate::harmony::analyze`]),
    /// named in this notation where the song is in `key`, in groups from
    /// the lowest up as [`Notation::rename`] gives them: one for a chord
    /// or a set of notes, the lower chord with the bass and then the upper
    /// for two chords at once. `None` where the notation is not sets or
    /// micromodes, and for silence and a single note, which have no set
    /// to name: those keep their own names, as the chord lane shows them.
    ///
    /// `sounding` is the pitch classes that sound: a chord heard without
    /// its fifth, C E for C, or C D E for Cadd9, is the set of the notes
    /// it has, not of the chord's every note.
    pub fn name_heard(
        self,
        harmony: &Harmony,
        sounding: PitchSet,
        key: Scale,
    ) -> Option<Vec<HarmonyName>> {
        let notation = self.in_key(key);
        if !notation.groups() {
            return None;
        }
        let heard = |harmony: Harmony| {
            let all = harmony.pitch_set();
            let heard = all.intersection(sounding);
            notation.name_group(if heard.len() >= 2 { heard } else { all }, key)
        };
        match *harmony {
            Harmony::Silence | Harmony::Single(_) => None,
            Harmony::Polychord { upper, lower, bass } => Some(vec![
                heard(Harmony::Chord { chord: lower, bass }),
                heard(Harmony::Chord {
                    chord: upper,
                    bass: upper.root,
                }),
            ]),
            other => Some(vec![heard(other)]),
        }
    }
}

/// The notes of a chord symbol, its bass's too.
fn notes_of(symbol: ChordSymbol) -> PitchSet {
    let mut set = symbol.chord().pitch_set();
    set.insert(symbol.bass_pitch_class());
    set
}

/// The chord made of exactly the notes of `set`, in root position, as a
/// set has no bass: the simplest one when the notes make several (C E G
/// A is Am7 before C6), on the first of its roots in normal form.
/// `None` if the notes make no chord the app knows.
pub fn chord_of(set: PitchSet) -> Option<Chord> {
    let roots = set.normal_form();
    ChordQuality::ALL.into_iter().find_map(|quality| {
        roots
            .iter()
            .find(|&&root| ChordQuality::identify(set.relative_to(root)) == Some(quality))
            .map(|&root| Chord::new(root, quality))
    })
}

/// How the notes are put into groups to be named as sets or micromodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Grouping {
    /// The harmonies written on the sheet, renamed.
    #[default]
    Written,
    /// The notes sounding together, from the lowest up: vertically.
    Together,
    /// The notes one after another: horizontally, as a row is cut.
    InLine,
    /// The forms of the key's set heard in the notes, together and one
    /// after another, as the colour check reads them.
    Mixed,
}

impl Grouping {
    pub const ALL: [Grouping; 4] = [
        Grouping::Written,
        Grouping::Together,
        Grouping::InLine,
        Grouping::Mixed,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Grouping::Written => "The sheet's harmonies",
            Grouping::Together => "Notes together (vertical)",
            Grouping::InLine => "Notes in a line (serial)",
            Grouping::Mixed => "Forms of the key (mixed)",
        }
    }

    /// Whether the groups are the forms of the key's set, as big as the
    /// set, rather than so many notes.
    pub fn reads_forms(self) -> bool {
        self == Grouping::Mixed
    }

    /// What it shows, for hints.
    pub fn describe(self) -> &'static str {
        match self {
            Grouping::Written => {
                "The harmonies written on the sheet, each named in the notation chosen"
            }
            Grouping::Together => {
                "The notes sounding at the same time, wherever a note starts, from the lowest \
                 up, in groups of so many different notes, as stacked chords are heard; the \
                 groups are written upper first, as two chords at once are"
            }
            Grouping::InLine => {
                "The notes one after another, in groups of so many notes, as a twelve-tone \
                 row is cut into trichords or hexachords; notes struck together count from \
                 the lowest up, and a note struck again at once counts once"
            }
            Grouping::Mixed => {
                "The forms of the key's set heard in the notes, as the colour check reads \
                 them: a form may sound partly together and partly one after another, and \
                 two tracks, or a top line and the chords under it, may each be in a form of \
                 their own at once. Only where the key is a pitch-class set or a micromode"
            }
        }
    }
}

/// A group of notes heard, to be named as a set: its notes in the order
/// they come, from the lowest up when they sound together (one of each
/// pitch class), or one after another in a line, where a note can come
/// back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteGroup {
    pub notes: Vec<Note>,
    /// The form of the key's set the notes were read as, for
    /// [`Grouping::Mixed`]: the group is named by it, though a first or
    /// a last form may not sound whole.
    pub form: Option<PitchSet>,
}

impl NoteGroup {
    /// A group of these notes, named by their own set.
    pub fn new(notes: Vec<Note>) -> Self {
        NoteGroup { notes, form: None }
    }

    pub fn set(&self) -> PitchSet {
        self.form
            .unwrap_or_else(|| self.notes.iter().map(|note| note.pitch_class()).collect())
    }

    /// Its notes spelled in `key`, in their order: "B Bb D". Notes that
    /// make a chord are spelled as that chord, up from its root, so a
    /// diminished triad is C Eb Gb, not C Eb F#.
    pub fn spelled(&self, key: Scale) -> String {
        let chord = chord_of(self.set());
        let names: Vec<String> = self
            .notes
            .iter()
            .map(|note| {
                let pitch_class = note.pitch_class();
                chord
                    .and_then(|chord| chord.spell_note(key.spell_root(chord), pitch_class))
                    .unwrap_or_else(|| key.spell(pitch_class))
                    .to_string()
            })
            .collect();
        names.join(" ")
    }
}

/// From timeline step `start` on, until the next, the song plays these
/// groups of notes, the lowest first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeardGroups {
    pub start: usize,
    pub groups: Vec<NoteGroup>,
}

impl HeardGroups {
    /// The sets of its groups, the lowest first.
    pub fn sets(&self) -> Vec<PitchSet> {
        self.groups.iter().map(NoteGroup::set).collect()
    }
}

/// The notes of the song read as groups, as `grouping` says: `size`
/// notes a group, or all the notes sounding together in one when `size`
/// is `None`; `channel` keeps only that channel's notes. A group that
/// is the one before it again is written once, where it starts.
/// [`Grouping::Written`] reads no notes, and gives nothing.
pub fn heard_groups(
    project: &Project,
    grouping: Grouping,
    size: Option<usize>,
    channel: Option<usize>,
) -> Vec<HeardGroups> {
    let end = project.song_length();
    let notes: Vec<HeardNote> = project
        .heard_notes(0, end)
        .into_iter()
        .filter(|note| channel.is_none_or(|channel| note.part == channel))
        .collect();
    let groups = match grouping {
        Grouping::Written => Vec::new(),
        Grouping::Together => together(&notes, size),
        Grouping::InLine => in_line(&notes, size.unwrap_or(3)),
        Grouping::Mixed => mixed(project, channel),
    };
    merge_repeats(groups)
}

/// The forms of the key's set the colour check reads in the song's
/// notes (of channel `channel` only, if given), where the key is a set:
/// at every step where a form starts, each form heard then, the lowest
/// first. A form read in all the parts at once is one group; two tracks,
/// or a top line and the chords under it, each in a form of its own are
/// two, as two chords at once are.
pub fn mixed(project: &Project, channel: Option<usize>) -> Vec<HeardGroups> {
    let mut tracks: Vec<(usize, Vec<NoteEvent>)> = Vec::new();
    for (part, note) in project.melodic_parts() {
        if channel.is_some_and(|channel| channel != part) {
            continue;
        }
        match tracks.iter_mut().find(|(track, _)| *track == part) {
            Some((_, notes)) => notes.push(note),
            None => tracks.push((part, vec![note])),
        }
    }
    let voices: Vec<Voice> = tracks
        .iter()
        .flat_map(|(track, notes)| voices_of(*track, notes))
        .collect();
    // The stretches from one key marker to the next, each read on its
    // own, as the check reads a pattern in its keys.
    let keys = project.key_map();
    let end = project.song_length();
    let mut edges: Vec<usize> = std::iter::once(0)
        .chain(keys.markers().iter().map(|marker| marker.start))
        .chain(std::iter::once(end))
        .filter(|&edge| edge <= end)
        .collect();
    edges.sort_unstable();
    edges.dedup();
    let spans: Vec<(usize, usize)> = edges.windows(2).map(|pair| (pair[0], pair[1])).collect();
    let forms = colour::forms_heard(&voices, &spans, |step| keys.key_at(step));

    let mut starts: Vec<usize> = forms.iter().map(|form| form.start).collect();
    starts.sort_unstable();
    starts.dedup();
    starts
        .into_iter()
        .map(|start| {
            let mut groups: Vec<(u32, NoteGroup)> = forms
                .iter()
                .filter(|form| (form.start..form.end).contains(&start))
                .map(|form| {
                    let notes = notes_of_form(form, &forms, &voices);
                    let height = notes.iter().map(|note| u32::from(note.midi())).sum::<u32>() * 100
                        / notes.len().max(1) as u32;
                    let group = NoteGroup {
                        notes,
                        form: Some(form.form),
                    };
                    (height, group)
                })
                .collect();
            groups.sort_by_key(|(height, _)| *height);
            HeardGroups {
                start,
                groups: groups.into_iter().map(|(_, group)| group).collect(),
            }
        })
        .collect()
}

/// The notes `form` was read in, one of each of its pitch classes, in
/// the order they come (the lowest first, where they start together).
/// A track whose top line was read on its own (in a form of `forms`
/// heard at the same time) leaves that line out.
fn notes_of_form(form: &FormStatement, forms: &[FormStatement], voices: &[Voice]) -> Vec<Note> {
    let apart: Vec<usize> = forms
        .iter()
        .filter(|other| other.start < form.end && form.start < other.end)
        .filter_map(|other| match other.heard_in {
            HeardIn::Line(voice) => Some(voice),
            _ => None,
        })
        .collect();
    let mut notes: Vec<&NoteEvent> = voices
        .iter()
        .enumerate()
        .filter(|(index, voice)| match form.heard_in {
            HeardIn::All => true,
            HeardIn::Track(track) => voice.channel == track && !apart.contains(index),
            HeardIn::Line(line) => *index == line,
        })
        .flat_map(|(_, voice)| voice.notes.iter())
        .filter(|note| (form.start..form.end).contains(&note.start))
        .filter(|note| form.form.contains(note.pitch.pitch_class()))
        .collect();
    notes.sort_by_key(|note| (note.start_tick(), note.pitch));
    distinct(notes.into_iter().map(|note| note.pitch).collect())
}

/// The notes sounding together, from each step where a note starts:
/// all the notes held then, from the lowest up, one of each pitch
/// class, in groups of `size` different notes (or all of them in one).
/// A note left over at the top joins the group under it, and a step
/// where fewer than two different notes sound is left out: a note alone
/// is no harmony. So the Augurs chord, Fb Ab Cb under G Bb Db Eb, in
/// threes is Fb major under Eb7, the two chords Stravinsky put together.
pub fn together(notes: &[HeardNote], size: Option<usize>) -> Vec<HeardGroups> {
    let mut sorted: Vec<&HeardNote> = notes.iter().collect();
    sorted.sort_by(|a, b| a.start.total_cmp(&b.start));
    let mut heard = Vec::new();
    // The notes held at a step: they come in as the steps reach their
    // start, and go when they have ended.
    let mut next = 0;
    let mut held: Vec<&HeardNote> = Vec::new();
    while next < sorted.len() {
        let step = sorted[next].start.max(0.0).floor();
        while next < sorted.len() && sorted[next].start < step + 1.0 {
            held.push(sorted[next]);
            next += 1;
        }
        held.retain(|note| note.end > step);
        let mut sounding: Vec<Note> = held.iter().map(|note| note.pitch).collect();
        sounding.sort();
        let groups: Vec<NoteGroup> = chunks(distinct(sounding), size)
            .into_iter()
            .filter(|group| group.notes.len() >= 2)
            .collect();
        if !groups.is_empty() {
            heard.push(HeardGroups {
                start: step as usize,
                groups,
            });
        }
    }
    heard
}

/// The notes one after another, in groups of `size` notes, as a row is
/// cut into groups: the first `size` notes are a group, the next `size`
/// the next one. A note struck again at the same pitch straight after
/// itself is one note played again, but the same note in another octave
/// counts, as it does where one row form ends on the note the next one
/// starts with. Notes struck together are taken from the lowest up. A
/// group of only one pitch class is left out, as no harmony.
pub fn in_line(notes: &[HeardNote], size: usize) -> Vec<HeardGroups> {
    let size = size.max(1);
    let mut line: Vec<&HeardNote> = notes.iter().collect();
    line.sort_by(|a, b| {
        a.start
            .total_cmp(&b.start)
            .then(a.pitch.midi().cmp(&b.pitch.midi()))
    });
    let mut groups: Vec<(usize, Vec<Note>)> = Vec::new();
    let mut last: Option<Note> = None;
    for note in line {
        if last == Some(note.pitch) {
            continue;
        }
        last = Some(note.pitch);
        match groups.last_mut() {
            Some((_, group)) if group.len() < size => group.push(note.pitch),
            _ => groups.push((note.start.max(0.0) as usize, vec![note.pitch])),
        }
    }
    groups
        .into_iter()
        .map(|(start, notes)| HeardGroups {
            start,
            groups: vec![NoteGroup::new(notes)],
        })
        .filter(|heard| heard.sets()[0].len() >= 2)
        .collect()
}

/// One note of each pitch class, the first (lowest) of each.
fn distinct(notes: Vec<Note>) -> Vec<Note> {
    let mut seen = PitchSet::EMPTY;
    notes
        .into_iter()
        .filter(|note| {
            let new = !seen.contains(note.pitch_class());
            seen.insert(note.pitch_class());
            new
        })
        .collect()
}

/// `notes` in groups of `size`, the last one maybe smaller, or all in
/// one group. A single note left over at the end joins the group before
/// it.
fn chunks(notes: Vec<Note>, size: Option<usize>) -> Vec<NoteGroup> {
    if notes.is_empty() {
        return Vec::new();
    }
    let Some(size) = size else {
        return vec![NoteGroup::new(notes)];
    };
    let mut groups: Vec<NoteGroup> = notes
        .chunks(size.max(1))
        .map(|chunk| NoteGroup::new(chunk.to_vec()))
        .collect();
    if groups.len() > 1 && groups.last().is_some_and(|last| last.notes.len() == 1) {
        let left = groups.pop().map(|last| last.notes).unwrap_or_default();
        if let Some(under) = groups.last_mut() {
            under.notes.extend(left);
        }
    }
    groups
}

/// Leaves out a group that is the one before it again: the sheet writes
/// a harmony where it changes.
fn merge_repeats(groups: Vec<HeardGroups>) -> Vec<HeardGroups> {
    let mut merged: Vec<HeardGroups> = Vec::new();
    for heard in groups {
        if merged.last().is_none_or(|last| last.sets() != heard.sets()) {
            merged.push(heard);
        }
    }
    merged
}

/// A name for each group, the lowest first, joined as two chords at
/// once are written, the upper first: "3-11 T1I|3-11 T7I".
pub fn stacked_text(names: &[HarmonyName]) -> String {
    let texts: Vec<String> = names.iter().rev().map(ToString::to_string).collect();
    texts.join("|")
}

/// The names of `names` by their classes alone, upper first as
/// [`stacked_text`] writes them: "Menor 3" for "Menor 3 T8I", "3-11" for
/// "3-11 T7I", for a place too narrow for the whole name. `None` when
/// none is a set or a micromode.
pub fn stacked_classes(names: &[HarmonyName]) -> Option<String> {
    let class = |name: &HarmonyName| match *name {
        HarmonyName::Set(set) => set.set_class().name(),
        HarmonyName::Micromode(set) => {
            Micromode::of(set).map_or_else(|| name.to_string(), |micromode| micromode.name.into())
        }
        other => other.to_string(),
    };
    names
        .iter()
        .any(|name| matches!(name, HarmonyName::Set(_) | HarmonyName::Micromode(_)))
        .then(|| {
            let texts: Vec<String> = names.iter().rev().map(class).collect();
            texts.join("|")
        })
}

/// The spelled notes of each group, upper first, as [`stacked_text`]
/// writes the names.
pub fn stacked_notes(groups: &[NoteGroup], key: Scale) -> String {
    let texts: Vec<String> = groups.iter().rev().map(|g| g.spelled(key)).collect();
    texts.join(" | ")
}

/// The notes of `set` in normal form, spelled as [`NoteGroup::spelled`]
/// spells a group's.
pub fn spelled_set(set: PitchSet, key: Scale) -> String {
    let notes: Vec<Note> = set
        .normal_form()
        .into_iter()
        .map(|p| Note::new(60 + p.semitone()))
        .collect();
    NoteGroup::new(notes).spelled(key)
}

/// How much a song uses one class of sets or micromodes, as the sheet
/// names its groups: see [`most_used`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassUse {
    /// The class, by its name alone, as [`stacked_classes`] writes it:
    /// "Menor 3" for a micromode, "3-11" for a set.
    pub class: String,
    /// The steps where one of its groups sounds, each from where it
    /// starts to where the next harmony starts, as the sheet writes a
    /// harmony lasting. Two groups of it at once count once.
    pub steps: usize,
    /// How many of its groups start: two at once are two.
    pub times: usize,
    /// Its forms, each with the steps it sounds, the most used first:
    /// "Menor 3 T4I" for 16 steps.
    pub forms: Vec<(HarmonyName, usize)>,
}

/// The classes of sets or micromodes a song uses, the most used first,
/// and how long the song is, to say how much of it each one fills.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Tally {
    pub classes: Vec<ClassUse>,
    /// The steps the harmonies were counted over: the song's, or the
    /// sheet's where more bars are written than the song plays.
    pub length: usize,
}

impl Tally {
    /// How much of the song `steps` is, in hundredths, rounded.
    pub fn percent(&self, steps: usize) -> usize {
        (steps * 100 + self.length / 2) / self.length.max(1)
    }
}

/// The class of a group as the sheet names it, or `None` for a chord, a
/// row form or no chord, which are no group of a class.
fn class_of(name: &HarmonyName) -> Option<String> {
    match *name {
        HarmonyName::Set(set) => Some(set.set_class().name()),
        HarmonyName::Micromode(set) => Some(
            Micromode::of(set)
                .map_or_else(|| set.set_class().name(), |micromode| micromode.name.into()),
        ),
        _ => None,
    }
}

/// The classes of sets and micromodes among `harmonies`, the one that
/// sounds longest first (then the one heard more times, then by name).
/// Each entry of `harmonies` is a step and the groups named there, the
/// lowest first, and lasts until the next entry's step, the last until
/// `end`. A chord, a row form or no chord among them is left out: the
/// tally is of the sets and micromodes the sheet shows.
pub fn most_used(harmonies: &[(usize, Vec<HarmonyName>)], end: usize) -> Tally {
    let mut classes: Vec<ClassUse> = Vec::new();
    for (index, (start, names)) in harmonies.iter().enumerate() {
        let until = harmonies
            .get(index + 1)
            .map_or(end, |(next, _)| *next)
            .min(end);
        let steps = until.saturating_sub(*start);
        let mut counted: Vec<String> = Vec::new();
        for name in names {
            let Some(class) = class_of(name) else {
                continue;
            };
            let entry = match classes.iter().position(|entry| entry.class == class) {
                Some(found) => &mut classes[found],
                None => {
                    classes.push(ClassUse {
                        class: class.clone(),
                        steps: 0,
                        times: 0,
                        forms: Vec::new(),
                    });
                    classes.last_mut().expect("just pushed")
                }
            };
            entry.times += 1;
            if !counted.contains(&class) {
                entry.steps += steps;
                counted.push(class);
            }
            match entry.forms.iter_mut().find(|(form, _)| form == name) {
                Some((_, form_steps)) => *form_steps += steps,
                None => entry.forms.push((*name, steps)),
            }
        }
    }
    for entry in &mut classes {
        entry.forms.sort_by(|(a, a_steps), (b, b_steps)| {
            b_steps
                .cmp(a_steps)
                .then_with(|| a.to_string().cmp(&b.to_string()))
        });
    }
    classes.sort_by(|a, b| {
        b.steps
            .cmp(&a.steps)
            .then(b.times.cmp(&a.times))
            .then_with(|| a.class.cmp(&b.class))
    });
    Tally {
        classes,
        length: end,
    }
}

/// The groups read from the notes, each named in `notation` in the key
/// on its step, for [`most_used`].
pub fn heard_names(
    groups: &[HeardGroups],
    notation: Notation,
    keys: &KeyMap,
) -> Vec<(usize, Vec<HarmonyName>)> {
    groups
        .iter()
        .map(|heard| {
            let key = keys.key_at(heard.start);
            let names = heard
                .sets()
                .into_iter()
                .map(|set| notation.name_group(set, key))
                .collect();
            (heard.start, names)
        })
        .collect()
}

/// The harmonies written on the sheet, each renamed in `notation` in
/// the key on its step, for [`most_used`], and where the last one ends:
/// the song's end, or the end of the last bar written where the sheet
/// goes on past the song.
pub fn written_names(
    project: &Project,
    notation: Notation,
) -> (Vec<(usize, Vec<HarmonyName>)>, usize) {
    let keys = project.key_map();
    let markers = project.playlist().chord_markers();
    let names = markers
        .iter()
        .map(|marker| {
            let key = keys.key_at(marker.start);
            (marker.start, notation.rename(marker.harmony, key))
        })
        .collect();
    let last_bar = markers
        .last()
        .map_or(0, |last| project.meter().bar_at(last.start).end());
    (names, project.song_length().max(last_bar))
}

/// The tally of the classes the sheet shows in `notation`, with its
/// groups read as `grouping` says (see [`heard_groups`]).
pub fn tally(
    project: &Project,
    notation: Notation,
    grouping: Grouping,
    size: Option<usize>,
    channel: Option<usize>,
) -> Tally {
    match grouping {
        Grouping::Written => {
            let (names, end) = written_names(project, notation);
            most_used(&names, end)
        }
        _ => {
            let groups = heard_groups(project, grouping, size, channel);
            let names = heard_names(&groups, notation, &project.key_map());
            most_used(&names, project.song_length())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harmony::{ROW_PRESETS, ScaleKind};
    use crate::pitch::PitchClass;

    const C_MAJOR: Scale = Scale::new(PitchClass::C, ScaleKind::Major);

    fn set(semitones: &[u8]) -> PitchSet {
        PitchSet::from_semitones(semitones)
    }

    fn name(text: &str) -> HarmonyName {
        HarmonyName::parse(text).unwrap()
    }

    fn renamed(notation: Notation, text: &str) -> String {
        stacked_text(&notation.rename(name(text), C_MAJOR))
    }

    /// A note from `start` to `end`, on part 0.
    fn note(midi: u8, start: f32, end: f32) -> HeardNote {
        HeardNote {
            pitch: Note::new(midi),
            start,
            end,
            part: 0,
        }
    }

    #[test]
    fn the_same_notes_in_every_notation() {
        // The owner's examples: C major's triads are 3-11, B dim is 3-10.
        assert_eq!(renamed(Notation::Sets, "C"), "3-11 T7I");
        assert_eq!(renamed(Notation::Sets, "Am"), "3-11 T9");
        assert_eq!(renamed(Notation::Sets, "Bdim"), "3-10 T11");
        assert_eq!(renamed(Notation::Sets, "G7"), "4-27 T7I");
        assert_eq!(renamed(Notation::Micromodes, "C"), "Acorde menor-mayor T7I");
        assert_eq!(
            renamed(Notation::Micromodes, "Bdim"),
            "Acorde disminuido T11"
        );
        // Four notes are no micromode: the set is shown.
        assert_eq!(renamed(Notation::Micromodes, "G7"), "4-27 T7I");
        // The bass is one of the notes.
        assert_eq!(
            renamed(Notation::Sets, "C/D"),
            name("{C D E G}").to_string()
        );
        // Every renamed harmony has the very notes of the written one.
        for text in ["C", "Am7", "F#m7b5", "G/B", "Ebmaj7", "C/D"] {
            for notation in Notation::ALL {
                let names = notation.rename(name(text), C_MAJOR);
                let notes: PitchSet = names
                    .iter()
                    .flat_map(|n| n.pitch_set().unwrap().iter())
                    .collect();
                assert_eq!(
                    Some(notes),
                    name(text).pitch_set(),
                    "{text} as {notation:?}"
                );
            }
        }
    }

    #[test]
    fn sets_that_make_a_chord_are_written_as_the_chord() {
        assert_eq!(renamed(Notation::Chords, "3-11 T2"), "Dm");
        assert_eq!(renamed(Notation::Chords, "{C E G}"), "C");
        assert_eq!(renamed(Notation::Chords, "Acorde disminuido T11"), "Bdim");
        // C E G A is both C6 and Am7: the simpler, more common chord.
        assert_eq!(renamed(Notation::Chords, "{C E G A}"), "Am7");
        // Menor 3 makes no chord, and keeps its name.
        assert_eq!(renamed(Notation::Chords, "Menor 3"), "Menor 3");
        assert_eq!(renamed(Notation::Chords, "3-1 T4"), "3-1 T4");
        // A stack of fourths, E A D G, is the seventh chord A D E G.
        assert_eq!(renamed(Notation::Chords, "4-23 T2"), "A7sus4");
        // As written, nothing changes.
        assert_eq!(renamed(Notation::Written, "3-11 T2"), "3-11 T2");
        // Chords stay as they are written, with their bass.
        assert_eq!(renamed(Notation::Chords, "G7/B"), "G7/B");
    }

    #[test]
    fn as_the_key_follows_the_key_each_harmony_is_in() {
        use crate::harmony::{Micromode, ScaleKind};
        let shown = |key: Scale, text: &str| stacked_text(&Notation::Key.rename(name(text), key));
        // In C major, and in a mode, chords: a micromode typed there that
        // makes a chord is that chord.
        assert_eq!(shown(C_MAJOR, "Acorde disminuido T11"), "Bdim");
        assert_eq!(shown(C_MAJOR, "Am7"), "Am7");
        let d_dorian = Scale::new(PitchClass::D, ScaleKind::Dorian);
        assert_eq!(shown(d_dorian, "3-11 T2"), "Dm");
        // In a micromode, micromodes; the same notes named as a set, sets.
        let menor_3 = Micromode::named("Menor 3").unwrap().key(PitchClass::C);
        assert_eq!(shown(menor_3, "C"), "Acorde menor-mayor T7I");
        assert_eq!(shown(menor_3.named_as_set(), "C"), "3-11 T7I");
        assert_eq!(shown(menor_3.named_as_set(), "Menor 3"), "3-3");
        // In a set or a row, sets.
        let octatonic = Scale::set(
            PitchClass::C,
            PitchSet::from_semitones(&[0, 1, 3, 4, 6, 7, 9, 10]),
        );
        assert_eq!(shown(octatonic, "C"), "3-11 T7I");
        let row = Scale::row(crate::harmony::ToneRow::CHROMATIC);
        assert_eq!(shown(row, "C"), "3-11 T7I");
        assert_eq!(Notation::Key.in_key(row), Notation::Sets);
        assert_eq!(Notation::Key.in_key(C_MAJOR), Notation::Chords);
        assert_eq!(Notation::Key.in_key(menor_3), Notation::Micromodes);
        // Any other notation stays itself in every key.
        for notation in Notation::ALL.into_iter().skip(1) {
            assert_eq!(notation.in_key(menor_3), notation);
        }
        // The chord lane of the piano roll reads the notes heard the
        // same way: in C major, the chord's own name.
        let heard = crate::harmony::Harmony::Chord {
            chord: crate::Chord::new(PitchClass::C, ChordQuality::Major),
            bass: PitchClass::C,
        };
        let c_e_g = PitchSet::from_semitones(&[0, 4, 7]);
        assert_eq!(Notation::Key.name_heard(&heard, c_e_g, C_MAJOR), None);
        assert_eq!(
            Notation::Key.name_heard(&heard, c_e_g, menor_3),
            Some(vec![HarmonyName::Micromode(c_e_g)])
        );
    }

    #[test]
    fn two_chords_at_once_are_two_sets() {
        // The Augurs chord: Eb7 over Fb major.
        let eb7 = name("Eb7").pitch_set().unwrap();
        let fb = name("Fb").pitch_set().unwrap();
        let names = Notation::Sets.rename(name("Eb7|Fb"), C_MAJOR);
        assert_eq!(names, [HarmonyName::Set(fb), HarmonyName::Set(eb7)]);
        assert_eq!(stacked_text(&names), "4-27 T3I|3-11 T11I");
        assert_eq!(
            renamed(Notation::Micromodes, "Eb7|Fb"),
            "4-27 T3I|Acorde menor-mayor T11I"
        );
        assert_eq!(renamed(Notation::Chords, "Eb7|Fb"), "Eb7|Fb");
    }

    #[test]
    fn the_harmony_heard_is_named_as_the_sheet_names_it() {
        use crate::harmony::analyze;
        let notes = |midi: &[u8]| -> Vec<Note> { midi.iter().map(|&m| Note::new(m)).collect() };
        let heard = |midi: &[u8]| analyze(notes(midi));
        let sounding =
            |midi: &[u8]| -> PitchSet { notes(midi).iter().map(|n| n.pitch_class()).collect() };
        let as_text = |notation: Notation, harmony: &Harmony, midi: &[u8]| {
            notation
                .name_heard(harmony, sounding(midi), C_MAJOR)
                .map(|names| stacked_text(&names))
        };
        // C/E has the notes of C: as sets, one and the same.
        let c = heard(&[48, 64, 67]);
        let c_over_e = heard(&[52, 60, 67]);
        assert_ne!(c, c_over_e);
        assert_eq!(
            as_text(Notation::Sets, &c, &[48, 64, 67]).as_deref(),
            Some("3-11 T7I")
        );
        assert_eq!(
            as_text(Notation::Sets, &c_over_e, &[52, 60, 67]),
            as_text(Notation::Sets, &c, &[48, 64, 67])
        );
        assert_eq!(
            as_text(Notation::Micromodes, &c, &[48, 64, 67]).as_deref(),
            Some("Acorde menor-mayor T7I")
        );
        // As chords, and for one note or none, the harmony keeps its own
        // name.
        assert_eq!(
            Notation::Chords.name_heard(&c, sounding(&[48, 64, 67]), C_MAJOR),
            None
        );
        assert_eq!(as_text(Notation::Sets, &heard(&[60, 72]), &[60, 72]), None);
        assert_eq!(as_text(Notation::Sets, &Harmony::Silence, &[]), None);
        // The set is the notes that sound: C D E is heard as Cadd9, whose
        // G does not sound, and is the micromode Mayor 1.
        let c_d_e = heard(&[60, 62, 64]);
        assert_eq!(
            as_text(Notation::Micromodes, &c_d_e, &[60, 62, 64]).as_deref(),
            Some("Mayor 1")
        );
        assert_eq!(
            as_text(Notation::Sets, &c_d_e, &[60, 62, 64]).as_deref(),
            Some("3-6")
        );
        // Two chords at once: two sets, the upper first, with their
        // classes alone for a narrow block.
        let chord = |text: &str| match name(text) {
            HarmonyName::Chord(symbol) => symbol.chord(),
            _ => unreachable!(),
        };
        let augurs = Harmony::Polychord {
            upper: chord("Eb7"),
            lower: chord("Fb"),
            bass: chord("Fb").root,
        };
        let both = augurs.pitch_set();
        let names = Notation::Sets.name_heard(&augurs, both, C_MAJOR).unwrap();
        assert_eq!(stacked_text(&names), "4-27 T3I|3-11 T11I");
        assert_eq!(stacked_classes(&names).as_deref(), Some("4-27|3-11"));
        let names = Notation::Micromodes
            .name_heard(&augurs, both, C_MAJOR)
            .unwrap();
        assert_eq!(
            stacked_classes(&names).as_deref(),
            Some("4-27|Acorde menor-mayor")
        );
    }

    #[test]
    fn the_petrushka_chord_read_vertically_in_threes() {
        // C E G under F# A# C#, as one beat.
        let notes: Vec<HeardNote> = [60, 64, 67, 66 + 12, 70 + 12, 73 + 12]
            .iter()
            .map(|&midi| note(midi, 0.0, 4.0))
            .collect();
        let heard = together(&notes, Some(3));
        assert_eq!(heard.len(), 1);
        assert_eq!(heard[0].sets(), [set(&[0, 4, 7]), set(&[6, 10, 1])]);
        let names: Vec<HarmonyName> = heard[0]
            .sets()
            .into_iter()
            .map(|s| Notation::Sets.name_group(s, C_MAJOR))
            .collect();
        assert_eq!(stacked_text(&names), "3-11 T1I|3-11 T7I");
        // All together: one set of six notes, the Petrushka hexachord.
        let all = together(&notes, None);
        assert_eq!(all[0].sets(), [set(&[0, 1, 4, 6, 7, 10])]);
        assert_eq!(HarmonyName::Set(all[0].sets()[0]).to_string(), "6-30 T1I");
    }

    #[test]
    fn together_reads_the_notes_held_where_a_note_starts() {
        // A chord held over two beats, a tune note in the second, and
        // silence in the third.
        let notes = [
            note(48, 0.0, 8.0),
            note(52, 0.0, 8.0),
            note(55, 0.0, 8.0),
            note(74, 4.0, 6.0),
            note(76, 12.0, 14.0),
        ];
        let heard = together(&notes, None);
        assert_eq!(heard.len(), 2, "a note alone says nothing");
        assert_eq!(heard[0].sets(), [set(&[0, 4, 7])]);
        assert_eq!(heard[1].start, 4);
        assert_eq!(heard[1].sets(), [set(&[0, 2, 4, 7])]);
        // In twos from the lowest: C E, then G D.
        let twos = together(&notes, Some(2));
        assert_eq!(twos[1].sets(), [set(&[0, 4]), set(&[7, 2])]);
        // A single note left at the top joins the group under it.
        let threes = together(&notes, Some(3));
        assert_eq!(threes[1].sets(), [set(&[0, 2, 4, 7])]);
        // A chord struck on the last step of a beat is its own.
        let moving = [
            note(60, 0.0, 6.0),
            note(64, 0.0, 6.0),
            note(67, 0.0, 6.0),
            note(62, 6.0, 12.0),
            note(65, 6.0, 12.0),
            note(69, 6.0, 12.0),
        ];
        let heard = together(&moving, None);
        assert_eq!(heard.len(), 2);
        assert_eq!(
            (heard[1].start, heard[1].sets()),
            (6, vec![set(&[2, 5, 9])])
        );
    }

    #[test]
    fn the_augurs_chord_in_threes_is_fb_under_eb7() {
        // Fb Ab Cb in the low strings, Eb7 above them with Eb on top.
        let notes: Vec<HeardNote> = [40, 47, 52, 56, 67, 70, 73, 75]
            .iter()
            .map(|&midi| note(midi, 0.0, 2.0))
            .collect();
        let heard = together(&notes, Some(3));
        let fb = name("Fb").pitch_set().unwrap();
        let eb7 = name("Eb7").pitch_set().unwrap();
        assert_eq!(heard[0].sets(), [fb, eb7]);
        // All of it: the notes of Ab harmonic minor, 7-32.
        let all = together(&notes, None);
        assert_eq!(HarmonyName::Set(all[0].sets()[0]).to_string(), "7-32 T7");
        assert_eq!(all[0].groups[0].spelled(C_MAJOR).split(' ').count(), 7);
    }

    #[test]
    fn webern_row_in_a_line_is_four_groups_of_menor_3() {
        let row = ROW_PRESETS
            .iter()
            .find(|preset| preset.name == "Webern's Concerto")
            .unwrap()
            .row;
        let notes: Vec<HeardNote> = row
            .pitch_classes()
            .iter()
            .enumerate()
            .map(|(i, p)| note(60 + p.semitone(), i as f32 * 2.0, i as f32 * 2.0 + 2.0))
            .collect();
        let heard = in_line(&notes, 3);
        assert_eq!(heard.len(), 4);
        for (i, group) in heard.iter().enumerate() {
            assert_eq!(group.start, i * 6);
            let name = Notation::Micromodes.name_group(group.sets()[0], C_MAJOR);
            assert!(name.to_string().starts_with("Menor 3"), "{name}");
        }
        let spelled: Vec<String> = heard
            .iter()
            .map(|group| group.groups[0].spelled(C_MAJOR))
            .collect();
        assert_eq!(spelled[0], "B Bb D");
        // Hexachords: the row's two halves, each the hexatonic
        // collection, 6-20, that the four trichords make two by two.
        let halves = in_line(&notes, 6);
        assert_eq!(halves.len(), 2);
        assert_eq!(halves[0].sets()[0].set_class().name(), "6-20");
        assert_eq!(halves[1].sets()[0].set_class().name(), "6-20");
        // The whole row is the aggregate.
        let whole = in_line(&notes, 12);
        assert_eq!(whole[0].sets()[0].len(), 12);
    }

    #[test]
    fn webern_demo_lead_reads_as_menor_3_and_its_chords_too() {
        use crate::library::{apply, demo, rows::ROW_STYLES};
        let style = ROW_STYLES
            .iter()
            .find(|style| style.name == "Webern's trichords")
            .unwrap();
        let project = demo::row_demo(style);
        let channel = |name: &str| {
            project
                .channels()
                .iter()
                .position(|channel| channel.name == name)
        };
        let lead = channel(apply::LEAD).unwrap();
        let line = heard_groups(&project, Grouping::InLine, Some(3), Some(lead));
        assert!(line.len() >= 4);
        for heard in &line {
            let name = Notation::Micromodes.name_group(heard.sets()[0], project.scale);
            assert!(
                name.to_string().starts_with("Menor 3"),
                "{name} at {}",
                heard.start
            );
        }
        // The keys play each group as a chord: read vertically on its
        // own track, every chord is Menor 3 too.
        let keys = channel(apply::KEYS).unwrap();
        let chords = heard_groups(&project, Grouping::Together, Some(3), Some(keys));
        assert!(!chords.is_empty());
        for heard in &chords {
            for set in heard.sets() {
                assert_eq!(set.set_class().name(), "3-3", "at {}", heard.start);
            }
        }
        assert!(heard_groups(&project, Grouping::Written, None, None).is_empty());
    }

    #[test]
    fn mixed_reads_the_forms_a_micromode_demo_plays() {
        use crate::library::{MicromodeStyle, demo};
        // The band plays the forms written on the sheet, each partly as
        // chords and partly as a tune: read from the notes, they are
        // the forms written, bar by bar.
        let style = MicromodeStyle::named("Menor 3 colour").unwrap();
        let project = demo::micromode_demo(style);
        let read = mixed(&project, None);
        let markers = project.playlist().chord_markers();
        assert!(markers.len() >= 8);
        for marker in markers {
            let heard = read
                .iter()
                .find(|heard| heard.start == marker.start)
                .unwrap_or_else(|| panic!("nothing read at {}", marker.start));
            assert_eq!(
                Some(heard.sets()[0]),
                marker.harmony.pitch_set(),
                "at {}",
                marker.start
            );
        }
        // Named as the key is: Menor 3 and its forms.
        let names: Vec<String> = read[..3]
            .iter()
            .map(|heard| {
                Notation::Key
                    .name_group(heard.sets()[0], project.scale)
                    .to_string()
            })
            .collect();
        assert_eq!(names, ["Menor 3", "Menor 3 T4I", "Menor 3 T3"]);
    }

    #[test]
    fn mixed_reads_only_where_the_key_is_a_set() {
        use crate::library::{demo, rows::ROW_STYLES};
        let style = ROW_STYLES
            .iter()
            .find(|style| style.name == "Webern's trichords")
            .unwrap();
        // In the row as the key, there are no forms of a set to read.
        let mut project = demo::row_demo(style);
        assert!(mixed(&project, None).is_empty());
        // In its first trichord as a set, 3-3, every trichord of the row
        // is one of its forms, the tune's and the chords' alike.
        let first = project.scale.tone_row().unwrap().segments(3)[0];
        project.scale = Scale::set(project.scale.root, first).named_as_set();
        let read = mixed(&project, None);
        assert!(read.len() >= 8);
        for heard in &read {
            for set in heard.sets() {
                assert_eq!(set.set_class().name(), "3-3", "at {}", heard.start);
            }
        }
        assert_eq!(
            stacked_notes(&read[0].groups, project.scale),
            "C B Eb",
            "the row's first trichord, in the order it comes"
        );
    }

    #[test]
    fn mixed_writes_two_forms_at_once_upper_first() {
        use crate::{Channel, Clip, NoteEvent};
        // A tune in Menor 3's T0 (C C# E) over chords in its T7 (G Ab B),
        // on two tracks.
        let mut project = Project::empty();
        project.add_channel(Channel::synth());
        project.add_channel(Channel::synth());
        let pattern = project.pattern_mut(0).unwrap();
        pattern.set_length(16);
        let tune = [72, 73, 76]
            .iter()
            .enumerate()
            .map(|(index, &midi)| NoteEvent::new(Note::new(midi), index * 4, 4));
        pattern.set_notes(0, tune);
        pattern.set_notes(
            1,
            [55, 56, 59].map(|midi| NoteEvent::new(Note::new(midi), 0, 12)),
        );
        project.add_clip(Clip::new(0, 0, 0)).unwrap();
        project.scale = Scale::set(PitchClass::C, set(&[0, 1, 4]));
        let read = mixed(&project, None);
        assert_eq!(read.len(), 1);
        let names: Vec<HarmonyName> = read[0]
            .sets()
            .into_iter()
            .map(|set| Notation::Sets.name_group(set, project.scale))
            .collect();
        assert_eq!(stacked_text(&names), "3-3|3-3 T7");
        assert_eq!(
            stacked_notes(&read[0].groups, project.scale),
            "C C# E | G Ab B"
        );
    }

    #[test]
    fn every_note_of_a_line_counts_but_a_repeated_one() {
        // C C D E | F G: the C struck again is one note.
        let line = |midis: &[u8]| -> Vec<HeardNote> {
            midis
                .iter()
                .enumerate()
                .map(|(i, &midi)| note(midi, i as f32, i as f32 + 1.0))
                .collect()
        };
        let heard = in_line(&line(&[60, 60, 62, 64, 65, 67]), 3);
        assert_eq!(heard.len(), 2);
        assert_eq!(heard[0].sets(), [set(&[0, 2, 4])]);
        assert_eq!(heard[1].start, 4);
        assert_eq!(heard[1].sets(), [set(&[5, 7])]);
        // C D C: the C comes back, and the group has two notes.
        let heard = in_line(&line(&[60, 62, 60, 64, 65, 67]), 3);
        assert_eq!(heard[0].sets(), [set(&[0, 2])]);
        assert_eq!(heard[0].groups[0].spelled(C_MAJOR), "C D C");
        assert_eq!(heard[1].sets(), [set(&[4, 5, 7])]);
        // A row form that ends on C, and the next that starts on C an
        // octave lower: the C counts in both.
        let heard = in_line(&line(&[63, 83, 72, 60, 85, 69]), 3);
        assert_eq!(heard.len(), 2);
        assert_eq!(heard[1].sets(), [set(&[0, 1, 9])]);
        // Struck together, the lower note comes first.
        let chord = [note(67, 0.0, 1.0), note(60, 0.0, 1.0), note(64, 0.0, 1.0)];
        let heard = in_line(&chord, 2);
        assert_eq!(heard[0].groups[0].notes[0], Note::new(60));
    }

    #[test]
    fn groups_spell_chords_as_chords() {
        let group = NoteGroup::new(vec![Note::new(60), Note::new(63), Note::new(66)]);
        assert_eq!(group.spelled(C_MAJOR), "C Eb Gb");
        assert_eq!(spelled_set(set(&[2, 5, 9]), C_MAJOR), "D F A");
    }

    #[test]
    fn the_most_used_class_sounds_longest() {
        // C for two bars, Bdim for one, then two forms of 3-3 at once.
        let harmonies = vec![
            (0, vec![name("3-11 T7I")]),
            (16, vec![name("3-11 T2")]),
            (32, vec![name("3-10 T11")]),
            (48, vec![name("{C C# E}"), name("{G G# B}")]),
        ];
        let tally = most_used(&harmonies, 64);
        let classes: Vec<(&str, usize, usize)> = tally
            .classes
            .iter()
            .map(|class| (class.class.as_str(), class.steps, class.times))
            .collect();
        // The two forms of 3-3 at once fill one bar, not two.
        assert_eq!(classes, [("3-11", 32, 2), ("3-3", 16, 2), ("3-10", 16, 1)]);
        assert_eq!(tally.percent(32), 50);
        assert_eq!(tally.percent(16), 25);
        let forms: Vec<String> = tally.classes[0]
            .forms
            .iter()
            .map(|(form, steps)| format!("{form} {steps}"))
            .collect();
        assert_eq!(forms, ["3-11 T2 16", "3-11 T7I 16"]);
    }

    #[test]
    fn the_tally_leaves_out_chords_rows_and_no_chord() {
        let harmonies = vec![
            (0, vec![name("C")]),
            (16, vec![name("N.C.")]),
            (32, vec![name("Menor 3 T2")]),
            (40, vec![name("P0")]),
        ];
        let tally = most_used(&harmonies, 64);
        assert_eq!(tally.classes.len(), 1);
        assert_eq!(tally.classes[0].class, "Menor 3");
        // Menor 3 lasts until the row form starts.
        assert_eq!(tally.classes[0].steps, 8);
        assert!(most_used(&[], 0).classes.is_empty());
    }

    #[test]
    fn the_sheets_chords_tallied_as_sets() {
        use crate::ChordMarker;
        let mut project = Project::empty();
        let markers = ["C", "Am", "G7", "C"]
            .iter()
            .enumerate()
            .map(|(bar, text)| ChordMarker::new(bar * 16, name(text)))
            .collect::<Vec<_>>();
        assert!(project.set_chords(0, 64, &markers));
        let tally = tally(&project, Notation::Sets, Grouping::Written, None, None);
        assert_eq!(tally.length, 64);
        let classes: Vec<(&str, usize)> = tally
            .classes
            .iter()
            .map(|class| (class.class.as_str(), tally.percent(class.steps)))
            .collect();
        assert_eq!(classes, [("3-11", 75), ("4-27", 25)]);
        // C is played twice: its form is the most used.
        assert_eq!(tally.classes[0].forms[0], (name("3-11 T7I"), 32));
        // As chords, nothing is a set.
        let chords = super::tally(&project, Notation::Chords, Grouping::Written, None, None);
        assert!(chords.classes.is_empty());
    }

    #[test]
    fn a_micromode_demo_uses_its_own_micromode_most() {
        use crate::library::{MicromodeStyle, demo};
        let style = MicromodeStyle::named("Menor 3 colour").unwrap();
        let project = demo::micromode_demo(style);
        let mixed = tally(&project, Notation::Key, Grouping::Mixed, None, None);
        assert_eq!(mixed.classes.len(), 1, "{:?}", mixed.classes);
        assert_eq!(mixed.classes[0].class, "Menor 3");
        assert!(mixed.percent(mixed.classes[0].steps) >= 90);
        // The sheet's own harmonies are its forms too.
        let written = tally(&project, Notation::Key, Grouping::Written, None, None);
        assert_eq!(written.classes[0].class, "Menor 3");
        assert!(written.classes[0].forms.len() >= 3);
    }
}
