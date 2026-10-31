//! The colour check, where the key is a pitch-class set or a micromode.
//!
//! Music built on one pitch-class set takes its sound, its colour, from
//! the set, as tonal music takes its sound from its key. Its notes come
//! from forms of the set, the set moved (`T2`) or turned upside down and
//! moved (`T5I`), one form after another, as the chords of a tonal piece
//! come one after another from its key. What the check asks of the notes
//! depends on how big the set is:
//!
//! - **A micromode**, a set of three notes as in Kröpfl's teaching, is a
//!   sound heard whole: its three notes together as a chord, one after
//!   another as a figure of a line, or shared out among the parts. Every
//!   note belongs to a form, and a form does not stop before its three
//!   notes have sounded: two notes alone make an interval, which several
//!   micromodes share, and the colour is lost.
//! - **A larger set** is a collection, as a scale is: the music takes a
//!   few of its notes at a time, and what matters is that they come from
//!   it. Every note belongs to the form heard around it.
//!
//! ## Reading the forms
//!
//! Which form sounds where is not written down: the check finds it, as
//! it finds the forms of a row. The notes that start together make a
//! group, and the groups are shared out among stretches, each read as
//! one form of the set, so that as little as possible is out of place:
//! the notes outside each stretch's form and, for a micromode, the notes
//! of its form that never sound in it. A new stretch costs something
//! too, so that one stray note is not read as a form of its own (that
//! takes two new stretches, one in and one out). For a micromode, whose
//! forms change every few notes, it costs half as much as a note out of
//! place: one note is enough to show the music has moved on. The forms
//! of a larger set share most of their notes (C major and G major share
//! six), so a few notes fit many of them, and its forms last longer: a
//! new one costs as much as two notes out of place. The cheapest reading
//! is found by dynamic programming. Of readings as cheap, the check
//! keeps the one with the fewest notes out of place, then the one with
//! the fewest notes missing from the first and the last forms, then the
//! one with the fewest stretches, then the one that changes form as late
//! as it can (a stray note stays with the form it came in), then the one
//! whose forms are nearest the key's own.
//!
//! A pattern is read on its own, but a form may come into the first
//! stretch from the pattern before, and go on from the last into the
//! next pattern, so neither counts as unfinished.

use std::cmp::Reverse;

use super::{PartNote, Slip, SlipKind, Voice, VoiceLeading};
use crate::harmony::degree::forms_by_nearness;
use crate::harmony::{Degree, Micromode, PitchSet, Scale, ScaleKind};
use crate::i18n::{list, tr, trf};
use crate::{Note, NoteEvent, PitchClass};

/// The most groups of notes one stretch is read over. A longer passage
/// in one form is read as several stretches of it, which costs little.
const LONGEST: usize = 64;

/// What a reading costs for each note outside its stretch's form, and
/// for each note of a micromode's form that never sounds in it.
const OUT_OF_PLACE: u32 = 4;

/// What a new stretch costs in a micromode, and in a larger set.
const NEW_FORM: u32 = 2;
const NEW_FORM_OF_A_COLLECTION: u32 = 8;

/// One form of the key's set heard through a stretch of the pattern,
/// for the colour check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormStatement {
    pub form: PitchSet,
    /// From its first note's step to where the next form starts, or
    /// where its last note ends.
    pub start: usize,
    pub end: usize,
    /// The key there, the set.
    pub key: Scale,
}

impl FormStatement {
    /// Its degree in the key: `T2I`.
    pub fn degree(&self) -> Option<Degree> {
        Degree::of(self.key, self.form)
    }

    /// Its degree, how it is made from the key's own form, and its notes:
    /// "T2: the key's form moved up 2 semitones: D D# F#."
    pub fn describe(&self) -> String {
        let notes = form_notes(self.key, self.form);
        match self.degree() {
            Some(degree) => format!("{degree}: {}: {notes}.", degree.describe(self.key)),
            None => format!("{notes}."),
        }
    }
}

/// Where a slip of the colour check stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColourPlace {
    /// The form heard there.
    pub form: PitchSet,
    /// For a form left unfinished: the notes of it that sounded, and the
    /// notes that never did.
    pub heard: PitchSet,
    pub missing: PitchSet,
}

/// A stretch of the pattern in one key that is a set.
struct InSet {
    start: usize,
    end: usize,
    key: Scale,
}

/// The stretches of `beats` where the key is a pitch-class set.
fn stretches(beats: &[(usize, usize)], key_at: &impl Fn(usize) -> Scale) -> Vec<InSet> {
    let mut stretches: Vec<InSet> = Vec::new();
    for &(start, end) in beats {
        let key = key_at(start);
        if !matches!(key.kind, ScaleKind::Set(_)) {
            continue;
        }
        match stretches.last_mut() {
            Some(last) if last.end == start && last.key == key => last.end = end,
            _ => stretches.push(InSet { start, end, key }),
        }
    }
    stretches
}

/// Checks the colour where the key is a set: the notes outside the form
/// heard around them, and the forms of a micromode left unfinished. Also
/// returns the forms read in the notes, in order.
pub(super) fn check(
    voices: &[Voice],
    beats: &[(usize, usize)],
    key_at: &impl Fn(usize) -> Scale,
) -> (Vec<Slip>, Vec<FormStatement>) {
    let mut slips = Vec::new();
    let mut statements = Vec::new();
    for stretch in stretches(beats, key_at) {
        let mut notes: Vec<PartNote> = voices
            .iter()
            .enumerate()
            .flat_map(|(voice, part)| {
                part.notes
                    .iter()
                    .filter(|note| (stretch.start..stretch.end).contains(&note.start))
                    .map(move |&note| PartNote { voice, note })
            })
            .collect();
        notes.sort_by_key(|played| (played.note.start_tick(), Reverse(played.note.pitch)));
        read(&notes, stretch.key, &mut slips, &mut statements);
    }
    statements.sort_by_key(|statement| statement.start);
    (slips, statements)
}

/// How well some notes fit the set `key` is, read as the check reads
/// them: how many notes there are, how many are out of place (outside
/// the form heard around them, or missing from a micromode's form left
/// unfinished), and the forms read, in order.
pub(crate) struct Fit {
    pub notes: usize,
    pub out: usize,
    pub forms: Vec<FormStatement>,
}

/// Reads `notes` as forms of the set `key` is, all as one part, and says
/// how well they fit (see [`Fit`]). Find the key uses it to find the
/// micromode a passage is made of.
pub(crate) fn fit(notes: &[NoteEvent], key: Scale) -> Fit {
    let mut parts: Vec<PartNote> = notes
        .iter()
        .map(|&note| PartNote { voice: 0, note })
        .collect();
    parts.sort_by_key(|played| (played.note.start_tick(), Reverse(played.note.pitch)));
    let (mut slips, mut forms) = (Vec::new(), Vec::new());
    read(&parts, key, &mut slips, &mut forms);
    let out = slips
        .iter()
        .map(|slip| match (slip.kind, slip.colour) {
            (SlipKind::FormUnfinished, Some(place)) => place.missing.len(),
            _ => 1,
        })
        .sum();
    Fit {
        notes: parts.len(),
        out,
        forms,
    }
}

fn union(a: PitchSet, b: PitchSet) -> PitchSet {
    PitchSet::from_bits(a.bits() | b.bits())
}

fn common(a: PitchSet, b: PitchSet) -> PitchSet {
    PitchSet::from_bits(a.bits() & b.bits())
}

fn without(a: PitchSet, b: PitchSet) -> PitchSet {
    PitchSet::from_bits(a.bits() & !b.bits())
}

/// The pitch classes of `notes`.
fn classes(notes: &[PartNote]) -> PitchSet {
    notes
        .iter()
        .map(|played| played.note.pitch.pitch_class())
        .collect()
}

/// For each group of notes that start together, the pitch classes still
/// sounding there from the groups before.
fn held(groups: &[&[PartNote]]) -> Vec<PitchSet> {
    let ticks: Vec<usize> = groups
        .iter()
        .map(|group| group[0].note.start_tick())
        .collect();
    let mut held = vec![PitchSet::EMPTY; groups.len()];
    for (index, group) in groups.iter().enumerate() {
        for played in group.iter() {
            let end = played.note.end_tick();
            let until = ticks.partition_point(|&tick| tick < end);
            for later in held.iter_mut().take(until).skip(index + 1) {
                later.insert(played.note.pitch.pitch_class());
            }
        }
    }
    held
}

/// The cheapest way found to read the groups up to one of them: what it
/// costs, how many notes are out of place, how many the first and the
/// last forms leave out, how many stretches it has, where its last
/// stretch starts, and that stretch's form, as an index into the forms.
#[derive(Debug, Clone, Copy)]
struct Best {
    cost: u32,
    out: u32,
    unheard: u32,
    stretches: u32,
    from: usize,
    form: usize,
}

/// Reads `notes`, in the order they start, as forms of the set `key` is
/// (see the module documentation), and reports what is out of place.
fn read(notes: &[PartNote], key: Scale, slips: &mut Vec<Slip>, read: &mut Vec<FormStatement>) {
    let groups: Vec<&[PartNote]> = notes
        .chunk_by(|a, b| a.note.start_tick() == b.note.start_tick())
        .collect();
    if groups.is_empty() {
        return;
    }
    let forms: Vec<PitchSet> = forms_by_nearness(key)
        .into_iter()
        .map(|(form, _)| form)
        .collect();
    // A micromode is heard whole; a larger set a few notes at a time.
    let whole = key.pitch_set().len() == 3;
    let new_form = if whole {
        NEW_FORM
    } else {
        NEW_FORM_OF_A_COLLECTION
    };
    let count = groups.len();
    let sounded: Vec<PitchSet> = groups.iter().map(|group| classes(group)).collect();
    // How many notes of each group lie outside each form.
    let outside: Vec<Vec<u32>> = groups
        .iter()
        .map(|group| {
            forms
                .iter()
                .map(|&form| {
                    group
                        .iter()
                        .filter(|played| !form.contains(played.note.pitch.pitch_class()))
                        .count() as u32
                })
                .collect()
        })
        .collect();
    let held = held(&groups);
    // Whether a stretch from group `from` up to group `to` may leave its
    // form unfinished: the first and the last may, as their forms can
    // run on from the pattern before or into the next.
    let may_stop = |from: usize, to: usize| !whole || from == 0 || to == count;

    let mut best: Vec<Option<Best>> = vec![None; count + 1];
    best[0] = Some(Best {
        cost: 0,
        out: 0,
        unheard: 0,
        stretches: 0,
        from: 0,
        form: 0,
    });
    for from in 0..count {
        let Some(before) = best[from] else {
            continue;
        };
        for (index, &form) in forms.iter().enumerate() {
            let mut out = 0;
            let mut heard = common(held[from], form);
            for to in from + 1..=(from + LONGEST).min(count) {
                out += outside[to - 1][index];
                heard = union(heard, common(sounded[to - 1], form));
                let gaps = match whole {
                    true => (form.len() - heard.len()) as u32,
                    false => 0,
                };
                let (missing, unheard) = match may_stop(from, to) {
                    true => (0, before.unheard + gaps),
                    false => (gaps, before.unheard),
                };
                let cost = before.cost + new_form + OUT_OF_PLACE * (out + missing);
                let placed = before.out + out + missing;
                let stretches = before.stretches + 1;
                // Of readings as good, the one whose last form starts
                // latest: a note that fits neither form stays with the
                // form it came in.
                let key = (cost, placed, unheard, stretches, Reverse(from));
                if best[to]
                    .is_none_or(|b| key < (b.cost, b.out, b.unheard, b.stretches, Reverse(b.from)))
                {
                    best[to] = Some(Best {
                        cost,
                        out: placed,
                        unheard,
                        stretches,
                        from,
                        form: index,
                    });
                }
            }
        }
    }

    // Walk back through the table, from the last group.
    let mut spans: Vec<(usize, usize, PitchSet)> = Vec::new();
    let mut to = count;
    while to > 0 {
        let Some(step) = best[to] else {
            break;
        };
        spans.push((step.from, to, forms[step.form]));
        to = step.from;
    }
    spans.reverse();

    for (index, &(from, to, form)) in spans.iter().enumerate() {
        let played: Vec<PartNote> = groups[from..to]
            .iter()
            .flat_map(|group| group.iter().copied())
            .collect();
        let start = played[0].note.start;
        let end = match spans.get(index + 1) {
            Some(&(next, _, _)) => groups[next][0].note.start,
            None => played.iter().map(|p| p.note.end()).max().unwrap_or(start),
        };
        read.push(FormStatement {
            form,
            start,
            end: end.max(start + 1),
            key,
        });
        let place = ColourPlace {
            form,
            heard: PitchSet::EMPTY,
            missing: PitchSet::EMPTY,
        };
        for &note in &played {
            if !form.contains(note.note.pitch.pitch_class()) {
                slips.push(Slip {
                    notes: vec![note],
                    colour: Some(place),
                    ..Slip::of(SlipKind::OutsideColour, key)
                });
            }
        }
        let heard = groups[from..to]
            .iter()
            .fold(common(held[from], form), |heard, group| {
                union(heard, common(classes(group), form))
            });
        let missing = without(form, heard);
        if !may_stop(from, to) && !missing.is_empty() {
            // Marked on the last group, the highest note of it: after it,
            // the next form starts.
            let last = groups[to - 1][0];
            let mut gaps = missing.iter();
            let expected = match (gaps.next(), gaps.next()) {
                (Some(only), None) => nearest(last.note.pitch, only),
                _ => None,
            };
            slips.push(Slip {
                notes: vec![last],
                colour: Some(ColourPlace {
                    heard,
                    missing,
                    ..place
                }),
                expected,
                ..Slip::of(SlipKind::FormUnfinished, key)
            });
        }
    }
}

/// The note of `pitch_class` nearest `note`.
fn nearest(note: Note, pitch_class: PitchClass) -> Option<Note> {
    let up = i32::from(note.pitch_class().semitones_up_to(pitch_class));
    note.transpose(if up > 6 { up - 12 } else { up })
}

/// The notes of `form`, closest together, spelled in `key`: "D F F#".
fn form_notes(key: Scale, form: PitchSet) -> String {
    form.normal_form()
        .into_iter()
        .map(|pitch_class| key.spell(pitch_class).to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

/// A form by its degree in `key` and its notes, "T2I (D F F#)", or by
/// its notes alone when it is no form of the key's set.
fn form_name(key: Scale, form: PitchSet) -> String {
    let notes = form_notes(key, form);
    match Degree::of(key, form) {
        Some(degree) => format!("{degree} ({notes})"),
        None => notes,
    }
}

/// What happens, in a sentence, for the colour check.
pub(super) fn what(slip: &Slip, voice_name: impl Fn(usize) -> String) -> String {
    let key = slip.key;
    let class = |pitch_class: PitchClass| key.spell(pitch_class).to_string();
    match (slip.kind, slip.notes.as_slice(), slip.colour) {
        (SlipKind::OutsideColour, [played], Some(place)) => trf(
            "Outside the colour: {} plays {}, which is not in {}, the form heard there.",
            &[
                &voice_name(played.voice),
                &key.note_name(played.note.pitch),
                &form_name(key, place.form),
            ],
        ),
        (SlipKind::FormUnfinished, [played], Some(place)) => {
            // In the order of the form, its notes closest together.
            let order = place.form.normal_form();
            let named = |set: PitchSet| -> Vec<String> {
                order
                    .iter()
                    .copied()
                    .filter(|&pitch_class| set.contains(pitch_class))
                    .map(class)
                    .collect()
            };
            let missing = named(place.missing);
            let never = match missing.as_slice() {
                [one] => trf("{} never sounds", &[one]),
                some => trf("{} never sound", &[&list(some)]),
            };
            let heard = match named(place.heard).as_slice() {
                [] => tr("no note of it").to_string(),
                some => list(some),
            };
            trf(
                "Form left unfinished: {} stops after {}, with {}'s {}; {} before the next \
                 form starts.",
                &[
                    &form_name(key, place.form),
                    &heard,
                    &voice_name(played.voice),
                    &key.note_name(played.note.pitch),
                    &never,
                ],
            )
        }
        _ => format!("{}.", tr(slip.kind.name())),
    }
}

/// Why music built on a set avoids it.
pub(super) fn why(kind: SlipKind) -> &'static str {
    match kind {
        SlipKind::OutsideColour => {
            "Where the key is a pitch-class set, the set gives the music its colour, as the key \
             does in tonal music: the notes come from forms of the set, the set moved or turned \
             upside down, one form after another. A note outside the form heard around it \
             brings in the sound of another set."
        }
        SlipKind::FormUnfinished => {
            "A micromode is heard whole: its colour is the sound of its three notes, together as \
             a chord, one after another in a line, or shared out among the parts. Two of them \
             alone make an interval that other micromodes have too, and the colour is lost."
        }
        _ => "",
    }
}

/// The set a key is, by name: "Menor 3 (minor 3)", "set 7-35".
fn colour_name(key: Scale) -> String {
    let set = key.pitch_set();
    match Micromode::of(set) {
        Some(micromode) => micromode.full_name(),
        None => trf("set {}", &[&set.set_class().name()]),
    }
}

/// The line summing up the colour check: the set, the forms read in
/// the notes, and how many of each kind of slip.
pub(super) fn summary(check: &VoiceLeading) -> String {
    let Some(first) = check.forms.first() else {
        return tr("Colour: no notes of the set yet.").to_string();
    };
    let labels: Vec<String> = check
        .forms
        .iter()
        .map(|statement| {
            statement.degree().map_or_else(
                || form_notes(statement.key, statement.form),
                |d| d.to_string(),
            )
        })
        .collect();
    let read = match labels.len() {
        1..=8 => labels.join(", "),
        n => trf("{} … ({} forms){}", &[&labels[..8].join(", "), &n, &""]),
    };
    let found = check
        .counts()
        .unwrap_or_else(|| tr("every note in the colour").to_string());
    trf(
        "Colour of {}, {}: {}.",
        &[&colour_name(first.key), &read, &found],
    )
}

#[cfg(test)]
mod tests {
    use super::super::{Rules, beats, check, voices_of};
    use super::*;
    use crate::{NoteEvent, TimeSignature};

    /// Menor 3 on C: C C# E.
    fn menor_3(_: usize) -> Scale {
        Scale::new(
            PitchClass::C,
            ScaleKind::Set(PitchSet::from_semitones(&[0, 1, 4])),
        )
    }

    /// The set of the major scale, 7-35, on C.
    fn diatonic(_: usize) -> Scale {
        Scale::new(
            PitchClass::C,
            ScaleKind::Set(PitchSet::from_semitones(&[0, 2, 4, 5, 7, 9, 11])),
        )
    }

    fn note(name: &str) -> Note {
        let (pitch, rest) = crate::NoteName::read(name).expect("a note name");
        let octave: i32 = rest.parse().expect("an octave");
        Note::new((i32::from(pitch.pitch_class().semitone()) + 12 * (octave + 1)) as u8)
    }

    /// Notes from names such as "E4", each `length` steps long, one
    /// after the other, starting at step `from`: a track. "E4+G4" is a
    /// chord, and "-" rests.
    fn track(text: &str, length: usize, from: usize) -> Vec<NoteEvent> {
        text.split_whitespace()
            .enumerate()
            .filter(|(_, names)| *names != "-")
            .flat_map(|(index, names)| {
                names
                    .split('+')
                    .map(move |name| NoteEvent::new(note(name), from + index * length, length))
            })
            .collect()
    }

    fn check_in(tracks: &[Vec<NoteEvent>], key: impl Fn(usize) -> Scale) -> VoiceLeading {
        let voices: Vec<Voice> = tracks
            .iter()
            .enumerate()
            .flat_map(|(channel, notes)| voices_of(channel, notes))
            .collect();
        let length = tracks
            .iter()
            .flatten()
            .map(NoteEvent::end)
            .max()
            .unwrap_or(0);
        check(voices, &beats(TimeSignature::default(), length), key)
    }

    fn kinds(check: &VoiceLeading) -> Vec<SlipKind> {
        check.slips.iter().map(|slip| slip.kind).collect()
    }

    fn degrees(check: &VoiceLeading) -> Vec<String> {
        check
            .forms
            .iter()
            .map(|form| form.degree().map(|d| d.to_string()).unwrap_or_default())
            .collect()
    }

    fn told(check: &VoiceLeading, kind: SlipKind) -> String {
        let slip = check
            .slips
            .iter()
            .find(|slip| slip.kind == kind)
            .unwrap_or_else(|| panic!("no {kind:?} in {:?}", kinds(check)));
        slip.what(|voice| ["Melody", "Chords", "Bass"][check.voices[voice].channel].into())
    }

    #[test]
    fn a_micromode_heard_whole_keeps_its_colour() {
        // C C# E, then C# E F, a form keeping two of its notes, as a line.
        let check = check_in(&[track("C4 C#4 E4 F4 E4 C#4", 4, 0)], menor_3);
        assert_eq!(check.rules, Rules::Colour);
        assert_eq!(kinds(&check), vec![]);
        assert_eq!(degrees(&check), vec!["T0", "T5I"]);
        assert_eq!((check.forms[0].start, check.forms[0].end), (0, 12));
        assert_eq!((check.forms[1].start, check.forms[1].end), (12, 24));
        assert_eq!(
            check.forms[1].describe(),
            "T5I: the key's form upside down around C, moved up 5 semitones: C# E F."
        );
        assert_eq!(
            check.summary(),
            "Colour of Menor 3 (minor 3), T0, T5I: every note in the colour."
        );
        // The same forms as chords, and shared out among two parts.
        let chords = check_in(&[track("C4+C#4+E4 C#4+E4+F4", 8, 0)], menor_3);
        assert_eq!(kinds(&chords), vec![]);
        assert_eq!(degrees(&chords), vec!["T0", "T5I"]);
        let shared = check_in(
            &[track("C#5 E5 E5 F5", 4, 0), track("C4 C#4", 8, 0)],
            menor_3,
        );
        assert_eq!(kinds(&shared), vec![]);
        assert_eq!(degrees(&shared), vec!["T0", "T5I"]);
    }

    #[test]
    fn a_note_outside_the_form_heard_there_is_marked() {
        let check = check_in(
            &[track("C#5 G4 E5 C5", 4, 0), track("C4+C#4+E4", 16, 0)],
            menor_3,
        );
        assert_eq!(kinds(&check), vec![SlipKind::OutsideColour]);
        assert_eq!(degrees(&check), vec!["T0"]);
        assert_eq!(
            told(&check, SlipKind::OutsideColour),
            "Outside the colour: Melody plays G4, which is not in T0 (C C# E), the form heard \
             there."
        );
        assert_eq!(check.slips[0].step(), 4);
        assert!(check.summary().ends_with("outside the colour (1)."));
    }

    #[test]
    fn a_micromode_form_left_unfinished_is_marked() {
        // C C# E whole, then only D and F of a form, then G G# B whole.
        let check = check_in(&[track("C4 C#4 E4 D4 F4 G4 G#4 B4", 4, 0)], menor_3);
        assert_eq!(kinds(&check), vec![SlipKind::FormUnfinished]);
        assert_eq!(degrees(&check), vec!["T0", "T1", "T7"]);
        assert_eq!(
            told(&check, SlipKind::FormUnfinished),
            "Form left unfinished: T1 (C# D F) stops after D and F, with Melody's F4; C# never \
             sounds before the next form starts."
        );
        // Where the missing note would have gone, by the last note.
        assert_eq!(check.slips[0].expected, Some(note("C#4")));
        // The first and the last forms may run on from the pattern
        // before, or into the next.
        let ends = check_in(&[track("C#4 E4 F4 G4 G#4 B4 C5", 4, 0)], menor_3);
        assert_eq!(kinds(&ends), vec![]);
    }

    #[test]
    fn notes_outside_a_larger_set_are_marked() {
        let check = check_in(&[track("C4 D4 E4 F4 G4 A4 B4 C5", 4, 0)], diatonic);
        assert_eq!(kinds(&check), vec![]);
        assert_eq!(degrees(&check), vec!["T0"]);
        assert!(
            check
                .summary()
                .starts_with("Colour of set 7-35, T0: every note in the colour"),
            "{}",
            check.summary()
        );
        // A chromatic note between two notes a semitone away: no form of
        // the set has all three.
        let check = check_in(
            &[track("C4 D4 E4 F4 G4 G#4 A4 B4 C5 B4 A4 G4 F4 E4", 4, 0)],
            diatonic,
        );
        assert_eq!(kinds(&check), vec![SlipKind::OutsideColour]);
        assert_eq!(check.slips[0].notes[0].note.pitch, note("G#4"));
        assert_eq!(degrees(&check), vec!["T0"]);
    }

    #[test]
    fn a_larger_set_may_move_to_another_form() {
        // C major's notes up and down, then G major's: the set moved up
        // a fifth.
        let check = check_in(
            &[track(
                "C4 D4 E4 F4 G4 A4 B4 C5 B4 A4 G4 F4 E4 D4 C4 \
                 D4 E4 F#4 G4 A4 B4 C5 D5 C5 B4 A4 G4 F#4 E4 D4",
                4,
                0,
            )],
            diatonic,
        );
        assert_eq!(kinds(&check), vec![]);
        assert_eq!(degrees(&check), vec!["T0", "T7"]);
        // A single stray note among the key's own does not make a form of
        // its own: F# between Fs.
        let check = check_in(
            &[track("C4 D4 E4 F4 G4 F#4 G4 A4 F4 E4 D4 C4", 4, 0)],
            diatonic,
        );
        assert_eq!(kinds(&check), vec![SlipKind::OutsideColour]);
        assert_eq!(check.slips[0].notes[0].note.pitch, note("F#4"));
        assert_eq!(degrees(&check), vec!["T0"]);
    }

    #[test]
    fn only_where_the_key_is_a_set() {
        // A key marker into C major halfway: the second half is not read.
        let key = |step: usize| {
            if step < 16 {
                menor_3(step)
            } else {
                Scale::new(PitchClass::C, ScaleKind::Major)
            }
        };
        let check = check_in(&[track("C4 C#4 E4 E4 G4 A4 B4 D4", 4, 0)], key);
        assert_eq!(kinds(&check), vec![]);
        assert_eq!(degrees(&check), vec!["T0"]);
        assert_eq!(check.forms[0].end, 16);
    }
}
