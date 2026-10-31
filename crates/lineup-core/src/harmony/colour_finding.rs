//! Find the colour: the micromode or the set a passage is made of.
//!
//! Find the key ranks the 24 major and minor keys by how well the notes
//! fit them (see [`key_finding`](super::key_finding)). Music built on a
//! micromode or a pitch-class set has a home of another kind: one set,
//! moved and turned upside down, heard over and over. This module
//! proposes it, in two ways:
//!
//! - **A micromode.** The notes are read as forms of each of Kröpfl's
//!   twelve micromodes, the way the colour check reads them where the
//!   key is a micromode (see
//!   [`voice_leading::colour`](super::voice_leading::colour)): every
//!   note in a form, every form heard whole. A micromode is proposed when
//!   at least [`MICROMODE_FIT`] of the notes are in place in its forms;
//!   the best first, then any within [`CLOSE`] of it. Its key is the form
//!   heard longest, as the key of a tonal piece is the chord it comes
//!   home to.
//! - **A collection.** The fewest notes that sound [`COLLECTION_TIME`] of
//!   the time, as a scale is the notes a melody is made of. Their set is
//!   proposed when it has [`SMALLEST`] to six notes (the whole-tone scale,
//!   6-35), or up to [`LARGEST`] when it is a collection music is made of:
//!   one with a name (the octatonic scale, 8-28; the harmonic minor,
//!   7-32) or the diatonic 7-35. Seven or eight notes without a name are
//!   most often a key with a chromatic note or two, which tells nothing.
//!   A collection that is just the major or minor key found already is
//!   left out too: the key says it.
//!
//! Reading the notes twelve times costs time in a long passage, so a
//! micromode that cannot fit is left out first, cheaply: a group of notes
//! that start together with more than three different notes, or three or
//! fewer that are in no form of the micromode, has notes out of place
//! however it is read.

use super::voice_leading::colour;
use super::{Micromode, PitchSet, Scale, SetClass};
use crate::{NoteEvent, PitchClass};

/// The share of the notes a micromode must hold in place to be proposed.
pub const MICROMODE_FIT: f32 = 0.9;

/// Other micromodes are proposed too, fainter, when they fit almost as
/// well: at most this much less.
pub const CLOSE: f32 = 0.05;

/// At most this many micromodes are proposed beside the best.
const MAX_ALTERNATIVES: usize = 2;

/// The share of the time the notes of a collection sound, together.
pub const COLLECTION_TIME: f32 = 0.97;

/// A collection is proposed when it has this many notes or more...
pub const SMALLEST: usize = 4;
/// ...and, when it has a name, this many or fewer: ten or eleven notes
/// are nearly all twelve.
pub const LARGEST: usize = 9;

/// The most notes a collection without a name can have.
const LARGEST_UNNAMED: usize = 6;

/// What a passage is made of.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colour {
    /// One of the twelve micromodes, heard as its forms.
    Micromode(&'static Micromode),
    /// The notes of a set, as a scale: a collection.
    Collection(&'static SetClass),
}

/// A colour proposed for a passage.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColourGuess {
    pub colour: Colour,
    /// The key it makes: for a micromode, its form heard longest; for a
    /// collection, its notes.
    pub key: Scale,
    /// How much of the passage it holds, from 0 to 1: for a micromode,
    /// the share of the notes in place in its forms; for a collection,
    /// the share of the time its notes sound.
    pub fit: f32,
}

/// The colours of a passage.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Colours {
    /// The micromodes that fit, best first; empty when none does.
    pub micromodes: Vec<ColourGuess>,
    /// The collection its notes make, if it is worth proposing.
    pub collection: Option<ColourGuess>,
}

impl Colours {
    /// Whether nothing is proposed.
    pub fn is_empty(&self) -> bool {
        self.micromodes.is_empty() && self.collection.is_none()
    }
}

/// The colours of `notes`, a passage: the micromodes it fits and the
/// collection it is made of (see the module documentation). `key` is
/// the major or minor key found in it, if one was: a collection that is
/// just that key's notes is not proposed.
pub fn find_colours(notes: &[NoteEvent], key: Option<Scale>) -> Colours {
    let micromodes = micromodes(notes);
    // The collection starts where the micromode's home does, so the two
    // keys proposed share their note; or else on the key's.
    let roots = [
        micromodes.first().map(|guess| guess.key.root),
        key.map(|key| key.root),
    ];
    Colours {
        collection: collection(notes, key, &roots),
        micromodes,
    }
}

/// The micromodes `notes` fit, best first.
fn micromodes(notes: &[NoteEvent]) -> Vec<ColourGuess> {
    if notes.len() < 3 {
        return Vec::new();
    }
    let groups = groups(notes);
    let mut guesses: Vec<ColourGuess> = Micromode::all()
        .iter()
        .filter(|micromode| {
            let least = least_out(&groups, micromode);
            1.0 - least as f32 / notes.len() as f32 >= MICROMODE_FIT
        })
        .filter_map(|micromode| {
            let fit = colour::fit(notes, micromode.key(PitchClass::C));
            let share = 1.0 - fit.out as f32 / fit.notes.max(1) as f32;
            if share < MICROMODE_FIT {
                return None;
            }
            // The form heard longest, the first to come of those heard as
            // long: the home of the passage.
            let mut heard: Vec<(PitchSet, usize)> = Vec::new();
            for form in &fit.forms {
                let length = form.end - form.start;
                match heard.iter_mut().find(|(set, _)| *set == form.form) {
                    Some((_, total)) => *total += length,
                    None => heard.push((form.form, length)),
                }
            }
            let home = heard
                .iter()
                .rev()
                .max_by_key(|(_, total)| *total)
                .map_or(micromode.prime, |&(set, _)| set);
            Some(ColourGuess {
                colour: Colour::Micromode(micromode),
                key: key_of(home, &[]),
                fit: share.max(0.0),
            })
        })
        .collect();
    guesses.sort_by(|a, b| b.fit.total_cmp(&a.fit));
    if let Some(best) = guesses.first().map(|guess| guess.fit) {
        let close = guesses
            .iter()
            .skip(1)
            .take(MAX_ALTERNATIVES)
            .take_while(|guess| guess.fit >= best - CLOSE)
            .count();
        guesses.truncate(1 + close);
    }
    guesses
}

/// The different notes of each group of `notes` that start together.
fn groups(notes: &[NoteEvent]) -> Vec<PitchSet> {
    let mut starts: Vec<(usize, PitchClass)> = notes
        .iter()
        .map(|note| (note.start_tick(), note.pitch.pitch_class()))
        .collect();
    starts.sort_by_key(|&(tick, _)| tick);
    starts
        .chunk_by(|a, b| a.0 == b.0)
        .map(|group| group.iter().map(|&(_, pitch_class)| pitch_class).collect())
        .collect()
}

/// The fewest notes out of place that a reading of `groups` as forms of
/// `micromode` can have: a group of more than three different notes
/// leaves all but three out, and a group of three or fewer that is in no
/// form leaves one out at least.
fn least_out(groups: &[PitchSet], micromode: &Micromode) -> usize {
    let forms = micromode.members();
    groups
        .iter()
        .map(|&group| match group.len() {
            n if n > 3 => n - 3,
            _ if forms
                .iter()
                .any(|form| form.bits() & group.bits() == group.bits()) =>
            {
                0
            }
            _ => 1,
        })
        .sum()
}

/// The collection `notes` are made of, if it is worth proposing.
fn collection(
    notes: &[NoteEvent],
    key: Option<Scale>,
    roots: &[Option<PitchClass>],
) -> Option<ColourGuess> {
    let mut time = [0usize; 12];
    for note in notes {
        time[usize::from(note.pitch.pitch_class().semitone())] += note.length;
    }
    let total: usize = time.iter().sum();
    if total == 0 {
        return None;
    }
    let mut order: Vec<usize> = (0..12).filter(|&p| time[p] > 0).collect();
    // The longest first; of notes as long, the lowest pitch class, so the
    // same notes always give the same collection.
    order.sort_by_key(|&p| (std::cmp::Reverse(time[p]), p));
    let mut set = PitchSet::EMPTY;
    let mut held = 0;
    for &p in &order {
        if held as f32 >= COLLECTION_TIME * total as f32 {
            break;
        }
        set.insert(PitchClass::from_semitone(p as u8));
        held += time[p];
    }
    let class = set.set_class();
    let named = class.nickname().is_some() || class.name() == "7-35";
    let largest = if named { LARGEST } else { LARGEST_UNNAMED };
    if !(SMALLEST..=largest).contains(&set.len()) {
        return None;
    }
    if key.is_some_and(|key| key.kind.is_tonal() && key.pitch_set() == set) {
        return None;
    }
    Some(ColourGuess {
        colour: Colour::Collection(class),
        key: key_of(set, roots),
        fit: held as f32 / total as f32,
    })
}

/// A key made of `set`: on the first of `roots` that is one of its notes,
/// else on the first note of its normal form.
fn key_of(set: PitchSet, roots: &[Option<PitchClass>]) -> Scale {
    let root = roots
        .iter()
        .flatten()
        .copied()
        .find(|&root| set.contains(root))
        .or_else(|| set.normal_form().first().copied())
        .unwrap_or(PitchClass::C);
    Scale::set(root, set)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Note;

    /// Notes from MIDI numbers, each group of three starting together,
    /// a beat apart.
    fn chords(groups: &[[u8; 3]]) -> Vec<NoteEvent> {
        groups
            .iter()
            .enumerate()
            .flat_map(|(index, group)| {
                group
                    .iter()
                    .map(move |&midi| NoteEvent::new(Note::new(midi), index * 4, 4))
            })
            .collect()
    }

    #[test]
    fn forms_of_menor_3_are_found_as_menor_3() {
        // C C# E, then T2 D D# F#, T5I F E C#... all forms of Menor 3.
        let notes = chords(&[
            [60, 61, 64],
            [62, 63, 66],
            [65, 64, 61],
            [67, 68, 71],
            [60, 61, 64],
            [60, 63, 64],
        ]);
        let colours = find_colours(&notes, None);
        let best = colours.micromodes[0];
        assert_eq!(
            best.colour,
            Colour::Micromode(Micromode::named("Menor 3").unwrap())
        );
        assert_eq!(best.fit, 1.0);
        // C C# E is heard longest: twice.
        assert_eq!(best.key.pitch_set(), PitchSet::from_semitones(&[0, 1, 4]));
        assert_eq!(best.key.root, PitchClass::C);
        // Six notes, C C# D D# E F F# G G# B: no collection of nine or fewer
        // holds them.
        assert!(colours.collection.is_none(), "{colours:?}");
    }

    #[test]
    fn notes_that_fit_no_micromode_find_none() {
        // Six chords of six micromodes: Menor 1, Mayor 3, the augmented,
        // the diminished, Mayor 4 and Menor 5.
        let notes = chords(&[
            [60, 61, 62],
            [60, 62, 66],
            [60, 64, 68],
            [60, 63, 66],
            [60, 62, 67],
            [60, 61, 66],
        ]);
        assert!(find_colours(&notes, None).micromodes.is_empty());
        // A chromatic line, though, is four forms of Menor 1 in a row: C
        // C# D, D# E F, F# G G#, A A# B.
        let line: Vec<NoteEvent> = (0..12)
            .map(|step| NoteEvent::new(Note::new(60 + step as u8), step * 2, 2))
            .collect();
        let best = find_colours(&line, None).micromodes[0];
        assert_eq!(
            best.colour,
            Colour::Micromode(Micromode::named("Menor 1").unwrap())
        );
    }

    #[test]
    fn a_whole_tone_passage_is_the_whole_tone_collection() {
        let notes: Vec<NoteEvent> = [60u8, 62, 64, 66, 68, 70, 72, 70, 68, 66, 64, 62]
            .iter()
            .enumerate()
            .map(|(step, &midi)| NoteEvent::new(Note::new(midi), step * 2, 2))
            .collect();
        let collection = find_colours(&notes, None).collection.unwrap();
        assert_eq!(
            collection.colour,
            Colour::Collection(PitchSet::from_semitones(&[0, 2, 4, 6, 8, 10]).set_class())
        );
        assert_eq!(collection.fit, 1.0);
        assert_eq!(collection.key.root, PitchClass::C);
        // Heard in D major, the collection still starts where the
        // micromode's home does.
        let d_major = Scale::new(PitchClass::D, crate::ScaleKind::Major);
        let colours = find_colours(&notes, Some(d_major));
        let home = colours.micromodes[0].key;
        assert_eq!(home.pitch_set(), PitchSet::from_semitones(&[0, 2, 4]));
        assert_eq!(colours.collection.unwrap().key.root, home.root);
    }

    #[test]
    fn the_notes_of_the_key_found_are_not_proposed_again() {
        // C major, up and down.
        let notes: Vec<NoteEvent> = [60u8, 62, 64, 65, 67, 69, 71, 72]
            .iter()
            .enumerate()
            .map(|(step, &midi)| NoteEvent::new(Note::new(midi), step * 2, 2))
            .collect();
        let c_major = Scale::new(PitchClass::C, crate::ScaleKind::Major);
        assert!(find_colours(&notes, Some(c_major)).collection.is_none());
        // Without a key, they are the diatonic collection, 7-35.
        let collection = find_colours(&notes, None).collection.unwrap();
        assert_eq!(
            collection.colour,
            Colour::Collection(c_major.pitch_set().set_class())
        );
        // A line in A minor with the harmonic minor's G# is another
        // collection, 7-32, on A.
        let minor: Vec<NoteEvent> = [69u8, 71, 72, 74, 76, 77, 80, 81]
            .iter()
            .enumerate()
            .map(|(step, &midi)| NoteEvent::new(Note::new(midi), step * 2, 2))
            .collect();
        let a_minor = Scale::new(PitchClass::A, crate::ScaleKind::NaturalMinor);
        let collection = find_colours(&minor, Some(a_minor)).collection.unwrap();
        assert_eq!(collection.key.root, PitchClass::A);
        assert_eq!(
            collection.colour,
            Colour::Collection(
                Scale::new(PitchClass::A, crate::ScaleKind::HarmonicMinor)
                    .pitch_set()
                    .set_class()
            )
        );
    }
}
