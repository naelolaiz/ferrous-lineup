//! The rules of twelve-tone writing, checked where the key is a row.
//!
//! In twelve-tone music a row of the twelve notes takes the place of the
//! key. Schoenberg, who devised the method, and the composers who took
//! it up wrote by a few rules, which composition courses still teach:
//!
//! - **The row sets the order.** The notes follow one of the row's 48
//!   forms (P, I, R or RI, on any of the twelve notes) one after another.
//!   Notes next to each other in the row may sound together as a chord,
//!   in any order from the bottom up, and a note may be repeated
//!   straight away, or trilled with the note before it. A form may
//!   begin on the note, or the chord, the form before ended on, so that
//!   the two share it (an elision).
//! - **No note comes back early.** Before a note returns, the other
//!   eleven sound. The row was made for that: no note is heard more
//!   often than the others, so none becomes a tonic.
//! - **A form is played whole.** A form that stops short, when the next
//!   one starts, leaves a gap in the twelve.
//! - **No octave doublings.** A doubled note stands out, and a note that
//!   stands out can be heard as a root, or even a tonic.
//! - **No chords of tonal harmony.** A major or minor triad or a
//!   dominant seventh reminds the ear of tonal music, and it expects the
//!   music to go on as tonal music would. Berg, unlike Schoenberg and
//!   Webern, used them on purpose.
//!
//! ## Reading the row
//!
//! Which form of the row is played, and where one form ends and the next
//! begins, is not written down: the check finds it, as an analyst would.
//! Notes that start together make a group, whose notes may come in any
//! order. The groups are shared out among forms of the row so that as
//! few notes as possible are out of place, counting the notes out of
//! order, the notes repeated early and the notes a form leaves out. The
//! best way is found by dynamic programming: every form of the row is
//! tried on every stretch of groups, and the cheapest way to reach each
//! group is kept.
//!
//! A pattern is read on its own, but a row may run on from one pattern
//! into the next: the first form may begin in the pattern before, if
//! all its notes in this pattern are in order, and the last may go on in
//! the next, so neither counts as short of notes. The forms are read
//! from their first note whenever that costs no more.
//!
//! The row may run through all the tracks together (a tune and the
//! chords under it sharing one form), or each track may play forms of
//! its own (a canon). Both readings are tried, and the one with fewer
//! notes out of place is kept.

use super::{PartNote, RowPlace, RowStatement, Slip, SlipKind, Voice, VoiceLeading};
use crate::harmony::{
    ChordQuality, Harmony, PitchSet, RowForm, RowLabel, Scale, ScaleKind, ToneRow, analyze,
};
use crate::{Note, NoteEvent, PitchClass};

/// The most groups of notes one form of the row is tried on: twelve, and
/// room for notes repeated.
const LONGEST_FORM: usize = 40;

/// A stretch of the pattern in one twelve-tone key.
struct InRow {
    start: usize,
    end: usize,
    key: Scale,
    row: ToneRow,
}

/// The stretches of `beats` where the key is a row.
fn stretches(beats: &[(usize, usize)], key_at: &impl Fn(usize) -> Scale) -> Vec<InRow> {
    let mut stretches: Vec<InRow> = Vec::new();
    for &(start, end) in beats {
        let key = key_at(start);
        let ScaleKind::Row(row) = key.kind else {
            continue;
        };
        match stretches.last_mut() {
            Some(last) if last.end == start && last.key == key => last.end = end,
            _ => stretches.push(InRow {
                start,
                end,
                key,
                row,
            }),
        }
    }
    stretches
}

/// Checks the twelve-tone rules where the key is a row: the slips, and
/// the forms of the row played, in order. `doublings` are the pairs of
/// parts that play one line in octaves (see `doublings`).
pub(super) fn check(
    voices: &[Voice],
    doublings: &[(usize, usize)],
    beats: &[(usize, usize)],
    key_at: &impl Fn(usize) -> Scale,
) -> (Vec<Slip>, Vec<RowStatement>) {
    let mut slips = Vec::new();
    let mut rows = Vec::new();
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
        notes.sort_by_key(|played| {
            (
                played.note.start_tick(),
                std::cmp::Reverse(played.note.pitch),
            )
        });
        let doubling = octaves(voices, doublings, &notes, stretch.key, &mut slips);
        tonal_chords(voices, &notes, stretch.key, &mut slips);
        read_rows(voices, &notes, &doubling, &stretch, &mut slips, &mut rows);
    }
    rows.sort_by_key(|row| (row.start, row.channel));
    (slips, rows)
}

/// The note part `voice` has sounding at `tick`, if any.
fn sounding(voice: &Voice, tick: usize) -> Option<NoteEvent> {
    let note = voice.notes[voice.current(tick)?];
    (tick < note.end_tick()).then_some(note)
}

/// Reports every note of `notes` that doubles, in octaves or unison, a
/// note another part is sounding, and returns the notes that start
/// together with the note they double, each with that note: such a note
/// is one note of the row, played twice, and read once. A note doubling
/// one held from before is a note of the row in its own right. A pair
/// of parts that plays one line in octaves is reported once, where it
/// starts.
fn octaves(
    voices: &[Voice],
    doublings: &[(usize, usize)],
    notes: &[PartNote],
    key: Scale,
    slips: &mut Vec<Slip>,
) -> Vec<(PartNote, PartNote)> {
    let mut doubling = Vec::new();
    let mut lines_told: Vec<(usize, usize)> = Vec::new();
    for &played in notes {
        let tick = played.note.start_tick();
        let pitch_class = played.note.pitch.pitch_class();
        // The other note, sounding already or starting with this one in
        // a part before it, so that each doubling is found once.
        let other = voices.iter().enumerate().find_map(|(voice, part)| {
            let note = sounding(part, tick)?;
            let before = note.start_tick() < tick || voice < played.voice;
            (voice != played.voice && before && note.pitch.pitch_class() == pitch_class)
                .then_some(PartNote { voice, note })
        });
        let Some(other) = other else {
            continue;
        };
        if other.note.start_tick() == tick {
            doubling.push((played, other));
        }
        let pair = (other.voice.min(played.voice), other.voice.max(played.voice));
        let line = doublings.iter().any(|&(a, b)| (a.min(b), a.max(b)) == pair);
        if line {
            if lines_told.contains(&pair) {
                continue;
            }
            lines_told.push(pair);
        }
        slips.push(Slip {
            notes: vec![other, played],
            doubled: line,
            ..Slip::of(SlipKind::OctaveDoubling, key)
        });
    }
    doubling
}

/// Reports the moments where the notes sounding together make a chord
/// of tonal harmony: all of them, three or more, a major or minor triad
/// or a dominant seventh. A chord held or played again is told once.
fn tonal_chords(voices: &[Voice], notes: &[PartNote], key: Scale, slips: &mut Vec<Slip>) {
    let mut ticks: Vec<usize> = notes
        .iter()
        .map(|played| played.note.start_tick())
        .collect();
    ticks.dedup();
    let mut last: Option<PitchSet> = None;
    for tick in ticks {
        let together: Vec<PartNote> = voices
            .iter()
            .enumerate()
            .filter_map(|(voice, part)| {
                Some(PartNote {
                    voice,
                    note: sounding(part, tick)?,
                })
            })
            .collect();
        let set: PitchSet = together
            .iter()
            .map(|played| played.note.pitch.pitch_class())
            .collect();
        let tonal = match analyze(together.iter().map(|played| played.note.pitch)) {
            Harmony::Chord { chord, bass }
                if set.len() >= 3
                    && chord.pitch_set() == set
                    && matches!(
                        chord.quality,
                        ChordQuality::Major | ChordQuality::Minor | ChordQuality::Dominant7
                    ) =>
            {
                Some((chord, bass))
            }
            _ => None,
        };
        if tonal.is_some() && last != Some(set) {
            slips.push(Slip {
                notes: together,
                chord: tonal,
                ..Slip::of(SlipKind::TonalChord, key)
            });
        }
        last = tonal.map(|_| set);
    }
}

/// One of the forms of a row, with the place of each note in it.
struct Form {
    label: RowLabel,
    row: ToneRow,
    /// The place of each pitch class in the form, from 0, by semitones
    /// above C.
    place: [usize; 12],
}

/// The forms of `row`: 48, or fewer for a row some of whose forms are
/// the same (a row that is its own retrograde a tritone up, say).
fn forms_of(row: ToneRow) -> Vec<Form> {
    let mut forms: Vec<Form> = Vec::new();
    for form in RowForm::ALL {
        for number in 0..12 {
            let label = RowLabel::new(form, number);
            let notes = row.form(label);
            if forms.iter().any(|known| known.row == notes) {
                continue;
            }
            let mut place = [0; 12];
            for (index, semitone) in notes.semitones().into_iter().enumerate() {
                place[usize::from(semitone)] = index;
            }
            forms.push(Form {
                label,
                row: notes,
                place,
            });
        }
    }
    forms
}

/// Notes that start together.
struct Group {
    notes: Vec<PartNote>,
    /// Their pitch classes.
    set: PitchSet,
    /// Those that may only repeat a note just played (see [`groups`]).
    repeats: PitchSet,
}

/// The groups of `played` (sorted by when they start). A note repeated
/// straight away may be no new note of the row: one whose pitch class is
/// in the group before. Nor may a note trilled with the note before it:
/// a single note, after a single note, that is the single note before
/// that. Such a note is free: it brings the next note of the row when it
/// is one, as when a chord shares a note with the chord before, and
/// otherwise it is only a note played again.
fn groups(played: &[PartNote]) -> Vec<Group> {
    let mut groups = Vec::new();
    let mut recent = [PitchSet::EMPTY; 2];
    for chunk in played.chunk_by(|a, b| a.note.start_tick() == b.note.start_tick()) {
        let set: PitchSet = chunk
            .iter()
            .map(|played| played.note.pitch.pitch_class())
            .collect();
        let trill = set.len() == 1 && recent[1].len() == 1 && recent[0] == set;
        let repeats = match trill {
            true => set,
            false => set
                .iter()
                .filter(|&pitch_class| recent[1].contains(pitch_class))
                .collect(),
        };
        recent = [recent[1], set];
        groups.push(Group {
            notes: chunk.to_vec(),
            set,
            repeats,
        });
    }
    groups
}

/// A note out of place in a form of the row.
struct Misplaced {
    kind: SlipKind,
    played: PartNote,
    /// Its place in the form, from 0.
    place: usize,
    /// The place the form wanted next, from 0.
    wanted: Option<usize>,
    /// How many of the form's notes had sounded.
    heard: usize,
}

impl Misplaced {
    fn slip(self, form: &Form, key: Scale) -> Slip {
        let wanted = self
            .wanted
            .map(|place| (form.row.pitch_classes()[place], place + 1));
        Slip {
            notes: vec![self.played],
            expected: wanted
                .and_then(|(pitch_class, _)| nearest(self.played.note.pitch, pitch_class)),
            row: Some(RowPlace {
                label: form.label,
                order: self.place + 1,
                wanted,
                heard: self.heard,
                missing: PitchSet::EMPTY,
            }),
            ..Slip::of(self.kind, key)
        }
    }
}

/// The note of `pitch_class` nearest to `note`.
fn nearest(note: Note, pitch_class: PitchClass) -> Option<Note> {
    let up = i32::from(note.pitch_class().semitones_up_to(pitch_class));
    note.transpose(if up > 6 { up - 12 } else { up })
}

/// Going through a form of the row group by group: which of its places
/// have sounded, and how many notes were out of place.
#[derive(Debug, Clone, Copy)]
struct Walk {
    /// One bit for each place in the form, from the first.
    used: u16,
    /// The place after the furthest one sounded.
    next: usize,
    errors: usize,
}

impl Walk {
    /// A walk through a form from place `from`, the places before it
    /// sounded already: in the pattern before, or as the last notes of
    /// the form before, shared by the two (an elision).
    fn from(from: usize) -> Walk {
        Walk {
            used: (1 << from) - 1,
            next: from,
            errors: 0,
        }
    }

    fn heard(self) -> usize {
        self.used.count_ones() as usize
    }

    fn has(self, place: usize) -> bool {
        self.used & (1 << place) != 0
    }

    /// The first `count` places of `places` not sounded yet, as bits.
    fn open(self, places: std::ops::Range<usize>, count: usize) -> u16 {
        places
            .filter(|&place| !self.has(place))
            .take(count)
            .fold(0, |bits, place| bits | 1 << place)
    }

    /// The places `count` long that a group bringing the places `taken`
    /// should fill: the first places still open, which catches up a
    /// note skipped just before, or the places after the furthest one
    /// sounded, which goes on after a skip, whichever holds more of them.
    fn window(self, taken: u16, count: usize) -> u16 {
        let first_open = self.open(0..12, count);
        let ahead = self.open(self.next..12, count);
        match (taken & ahead).count_ones() > (taken & first_open).count_ones() {
            true => ahead,
            false => first_open,
        }
    }

    /// Takes in a group of notes, telling `tell` about each note out of
    /// place. A group of n new notes is in order when they are the next
    /// n places of the form, in any order among themselves (see
    /// [`Walk::window`]). So a note left out, or two notes swapped, is
    /// one note out of place, not all the notes after it. A note that
    /// only repeats the one just played is never out of place.
    fn take(&mut self, form: &Form, group: &Group, mut tell: impl FnMut(Misplaced)) {
        let place_of =
            |played: &PartNote| form.place[usize::from(played.note.pitch.pitch_class().semitone())];
        let repeat = |played: &PartNote| group.repeats.contains(played.note.pitch.pitch_class());
        let open = |repeats: bool| {
            group
                .notes
                .iter()
                .filter(|played| repeat(played) == repeats)
                .map(place_of)
                .filter(|&place| !self.has(place))
                .fold(0u16, |bits, place| bits | 1 << place)
        };
        let required = open(false);
        let optional = open(true) & !required;
        let all = required | optional;
        let around = self.window(all, all.count_ones() as usize);
        let taken = required | (optional & around);
        let window = self.window(taken, taken.count_ones() as usize);
        let fresh = required | (optional & window);
        let wanted = (0..12)
            .find(|&place| window & (1 << place) != 0 && fresh & (1 << place) == 0)
            .or_else(|| {
                (self.next..12)
                    .chain(0..12)
                    .find(|&place| !self.has(place) && fresh & (1 << place) == 0)
            });
        for &played in &group.notes {
            let place = place_of(&played);
            let kind = if repeat(&played) {
                continue;
            } else if self.has(place) {
                SlipKind::EarlyRepeat
            } else if window & (1 << place) == 0 {
                SlipKind::RowOrder
            } else {
                continue;
            };
            self.errors += 1;
            tell(Misplaced {
                kind,
                played,
                place,
                wanted,
                heard: self.heard(),
            });
        }
        self.used |= fresh;
        if fresh != 0 {
            self.next = self.next.max(16 - fresh.leading_zeros() as usize);
        }
    }
}

/// A stretch of groups read as one form of the row, from place `from`
/// (see [`Walk::from`]).
#[derive(Debug, Clone, Copy)]
struct Stretch {
    start: usize,
    end: usize,
    form: usize,
    from: usize,
}

/// The best reading of a stream of notes as forms of the row.
struct Reading {
    /// How many notes are out of place, repeated early, or missing.
    cost: usize,
    stretches: Vec<Stretch>,
    groups: Vec<Group>,
}

/// Reads `played` as forms of the row (see the module documentation).
/// The first form may have begun in the pattern before, when the notes
/// read that way are all in order, and the last may go on in the next,
/// so neither is short of notes.
fn read(forms: &[Form], played: &[PartNote]) -> Reading {
    let groups = groups(played);
    let n = groups.len();
    // best[end]: the fewest notes out of place in the first `end`
    // groups, and the stretch that ends there on the way to it.
    let mut best = vec![usize::MAX; n + 1];
    let mut came_from = vec![
        Stretch {
            start: 0,
            end: 0,
            form: 0,
            from: 0,
        };
        n + 1
    ];
    best[0] = 0;
    for start in 0..n {
        // The notes that ended the form before, which may also start
        // the next: one note, or a chord played again.
        let shared = start
            .checked_sub(1)
            .map_or(PitchSet::EMPTY, |before| groups[before].set);
        let reach = (start + LONGEST_FORM).min(n);
        // Forms from their first note are tried first, so that they win
        // over a reading that starts in the middle at the same cost.
        for from in 0..12 {
            for (index, form) in forms.iter().enumerate() {
                let begins = from == 0
                    || start == 0
                    || (from <= shared.len()
                        && form.row.pitch_classes()[..from]
                            .iter()
                            .all(|&pitch_class| shared.contains(pitch_class)));
                if !begins {
                    continue;
                }
                let mut walk = Walk::from(from);
                for end in start + 1..=reach {
                    walk.take(form, &groups[end - 1], |_| {});
                    // A form begun before is read only while its notes
                    // are all in order, or it could explain any start.
                    // And a form with more notes out of place than the
                    // row has is no reading of them at all.
                    if (start == 0 && from > 0 && walk.errors > 0) || walk.errors > 12 {
                        break;
                    }
                    let missing = if end == n { 0 } else { 12 - walk.heard() };
                    let cost = best[start] + walk.errors + missing;
                    if cost < best[end] {
                        best[end] = cost;
                        came_from[end] = Stretch {
                            start,
                            end,
                            form: index,
                            from,
                        };
                    }
                }
            }
        }
    }
    let mut stretches = Vec::new();
    let mut end = n;
    while end > 0 {
        let stretch = came_from[end];
        stretches.push(stretch);
        end = stretch.start;
    }
    stretches.reverse();
    Reading {
        cost: best[n],
        stretches,
        groups,
    }
}

impl Reading {
    /// Reports what is out of place in the reading, and the forms it
    /// reads, played by track `channel` or by all of them.
    fn report(
        &self,
        forms: &[Form],
        channel: Option<usize>,
        key: Scale,
        slips: &mut Vec<Slip>,
        rows: &mut Vec<RowStatement>,
    ) {
        for (index, stretch) in self.stretches.iter().enumerate() {
            let form = &forms[stretch.form];
            let groups = &self.groups[stretch.start..stretch.end];
            let mut walk = Walk::from(stretch.from);
            for group in groups {
                walk.take(form, group, |misplaced| {
                    slips.push(misplaced.slip(form, key))
                });
            }
            let notes = || groups.iter().flat_map(|group| &group.notes);
            rows.push(RowStatement {
                label: form.label,
                start: notes().map(|played| played.note.start).min().unwrap_or(0),
                end: notes().map(|played| played.note.end()).max().unwrap_or(0),
                channel,
                before: if stretch.start == 0 { stretch.from } else { 0 },
                key,
            });
            let last = groups.last().and_then(|group| group.notes.last()).copied();
            if index + 1 < self.stretches.len()
                && walk.heard() < 12
                && let Some(last) = last
            {
                let missing = form
                    .row
                    .pitch_classes()
                    .into_iter()
                    .enumerate()
                    .filter(|&(place, _)| !walk.has(place))
                    .map(|(_, pitch_class)| pitch_class)
                    .collect();
                slips.push(Slip {
                    notes: vec![last],
                    row: Some(RowPlace {
                        label: form.label,
                        order: form.place[usize::from(last.note.pitch.pitch_class().semitone())]
                            + 1,
                        wanted: None,
                        heard: walk.heard(),
                        missing,
                    }),
                    ..Slip::of(SlipKind::RowUnfinished, key)
                });
            }
        }
    }
}

/// Reads the row in `notes`, through all the tracks together and
/// track by track, keeps the reading with fewer notes out of place, and
/// reports it. Each note of `doubling` doubles the note paired with it,
/// and is left out where that note is read too.
fn read_rows(
    voices: &[Voice],
    notes: &[PartNote],
    doubling: &[(PartNote, PartNote)],
    stretch: &InRow,
    slips: &mut Vec<Slip>,
    rows: &mut Vec<RowStatement>,
) {
    let channel = |played: &PartNote| voices[played.voice].channel;
    let forms = forms_of(stretch.row);
    let played: Vec<PartNote> = notes
        .iter()
        .filter(|played| !doubling.iter().any(|(doubler, _)| doubler == *played))
        .copied()
        .collect();
    let together = read(&forms, &played);
    let mut channels: Vec<usize> = notes.iter().map(channel).collect();
    channels.sort_unstable();
    channels.dedup();
    if channels.len() > 1 {
        let apart: Vec<(usize, Reading)> = channels
            .iter()
            .map(|&track| {
                let own: Vec<PartNote> = notes
                    .iter()
                    .filter(|played| channel(played) == track)
                    .filter(|played| {
                        !doubling
                            .iter()
                            .any(|(doubler, other)| doubler == *played && channel(other) == track)
                    })
                    .copied()
                    .collect();
                (track, read(&forms, &own))
            })
            .collect();
        if apart.iter().map(|(_, reading)| reading.cost).sum::<usize>() < together.cost {
            for (track, reading) in &apart {
                reading.report(&forms, Some(*track), stretch.key, slips, rows);
            }
            return;
        }
    }
    together.report(&forms, None, stretch.key, slips, rows);
}

impl RowStatement {
    /// The form played, and its notes: "P4: the row starting on E: E F G
    /// C# F# Eb Ab D B C A Bb."
    pub fn describe(&self) -> String {
        let ScaleKind::Row(row) = self.key.kind else {
            return self.label.to_string();
        };
        let notes: Vec<String> = row
            .form(self.label)
            .pitch_classes()
            .into_iter()
            .map(|pitch_class| self.key.spell(pitch_class).to_string())
            .collect();
        let start = self.key.spell(PitchClass::from_semitone(self.label.number));
        let number = self.label.number;
        let what = match self.label.form {
            RowForm::Prime => format!("the row starting on {start}"),
            RowForm::Inversion => format!("the row upside down, starting on {start}"),
            RowForm::Retrograde => format!("P{number} backwards"),
            RowForm::RetrogradeInversion => format!("I{number} backwards"),
        };
        let carried = match self.before {
            0 => String::new(),
            before => format!(
                " The pattern starts at its note {}: the notes before it come in the pattern \
                 before.",
                before + 1
            ),
        };
        format!("{}: {what}: {}.{carried}", self.label, notes.join(" "))
    }
}

/// What happens, in a sentence, for the twelve-tone rules.
pub(super) fn what(slip: &Slip, voice_name: impl Fn(usize) -> String) -> String {
    let key = slip.key;
    let note = |note: Note| key.note_name(note);
    let class = |pitch_class: PitchClass| key.spell(pitch_class).to_string();
    match (slip.kind, slip.notes.as_slice(), slip.row) {
        (SlipKind::RowOrder, [played], Some(place)) => {
            let wanted = place
                .wanted
                .map(|(pitch_class, order)| {
                    format!(
                        " where {} wants {}, its note {order}",
                        place.label,
                        class(pitch_class)
                    )
                })
                .unwrap_or_default();
            format!(
                "Out of row order: {} plays {}{wanted}; {} is note {} of {}.",
                voice_name(played.voice),
                note(played.note.pitch),
                class(played.note.pitch.pitch_class()),
                place.order,
                place.label,
            )
        }
        (SlipKind::EarlyRepeat, [played], Some(place)) => {
            let wanted = place
                .wanted
                .map(|(pitch_class, order)| {
                    format!("; the row wants {}, its note {order}", class(pitch_class))
                })
                .unwrap_or_default();
            format!(
                "Note repeated early: {} plays {} again before {} has sounded all twelve notes \
                 ({} so far){wanted}.",
                voice_name(played.voice),
                note(played.note.pitch),
                place.label,
                place.heard,
            )
        }
        (SlipKind::RowUnfinished, [played], Some(place)) => {
            // In the order the form wanted them.
            let order: Vec<PitchClass> = match key.kind {
                ScaleKind::Row(row) => row.form(place.label).pitch_classes().to_vec(),
                _ => place.missing.iter().collect(),
            };
            let missing: Vec<String> = order
                .into_iter()
                .filter(|&pitch_class| place.missing.contains(pitch_class))
                .map(class)
                .collect();
            let never = match missing.as_slice() {
                [one] => format!("{one} never sounds"),
                [some @ .., last] => format!("{} and {last} never sound", some.join(", ")),
                [] => String::new(),
            };
            format!(
                "Row left unfinished: {} stops after {} of its 12 notes, with {}'s {}; {never} \
                 before the next form starts.",
                place.label,
                place.heard,
                voice_name(played.voice),
                note(played.note.pitch),
            )
        }
        (SlipKind::OctaveDoubling, [first, second], _) => {
            let (a, b) = (first.note.pitch, second.note.pitch);
            let notes = if a == b {
                format!("{} in both", note(a))
            } else {
                format!("{} and {}", note(a.max(b)), note(a.min(b)))
            };
            if slip.doubled {
                format!(
                    "Octave doubling: {} and {} play one line in octaves, from {notes} on.",
                    voice_name(first.voice),
                    voice_name(second.voice),
                )
            } else if first.note.start_tick() < second.note.start_tick() {
                format!(
                    "Octave doubling: {} plays {} while {} still holds {}.",
                    voice_name(second.voice),
                    note(b),
                    voice_name(first.voice),
                    note(a),
                )
            } else {
                format!(
                    "Octave doubling: {} and {} both play {} ({notes}).",
                    voice_name(first.voice),
                    voice_name(second.voice),
                    class(a.pitch_class()),
                )
            }
        }
        (SlipKind::TonalChord, notes, _) => {
            // The notes spelled as the chord they make: G# B D#, not
            // Ab B Eb.
            let spelled = slip
                .chord
                .map(|(chord, _)| key.chord_notes(chord))
                .unwrap_or_default();
            let in_chord = |pitch: Note| {
                spelled
                    .iter()
                    .find(|name| name.pitch_class() == pitch.pitch_class())
                    .map_or_else(|| note(pitch), |&name| pitch.name_as(name))
            };
            let mut pitches: Vec<Note> = notes.iter().map(|played| played.note.pitch).collect();
            pitches.sort();
            let names: Vec<String> = pitches.into_iter().map(in_chord).collect();
            let chord = slip.chord.map(|(chord, bass)| {
                let name = Harmony::Chord { chord, bass }
                    .label_in(key)
                    .unwrap_or_else(|| key.chord_name(chord));
                let kind = match chord.quality {
                    ChordQuality::Major => "a major triad",
                    ChordQuality::Minor => "a minor triad",
                    _ => "a dominant seventh",
                };
                format!("{name}, {kind}")
            });
            format!(
                "Sounds tonal: {} make {}, a chord of tonal harmony.",
                names.join(" "),
                chord.unwrap_or_default(),
            )
        }
        _ => format!("{}.", slip.kind.name()),
    }
}

/// Why twelve-tone writing avoids it.
pub(super) fn why(kind: SlipKind) -> &'static str {
    match kind {
        SlipKind::RowOrder => {
            "In twelve-tone music the row takes the place of the key: each form of the row is \
             played note after note, in its order, so that everything is heard to come from the \
             row. Notes next to each other in the row may sound together as a chord, in any \
             order, and a note may be repeated straight away."
        }
        SlipKind::EarlyRepeat => {
            "The row keeps each note from coming back before the other eleven have sounded, so \
             that no note is heard more than the others and none becomes a tonic. A note may be \
             repeated straight away, or trilled with the note before it, but not brought back \
             later."
        }
        SlipKind::RowUnfinished => {
            "A form of the row is played whole: when it stops short and the next one starts, \
             the notes it leaves out are missing from the twelve, and others come back before \
             their turn."
        }
        SlipKind::OctaveDoubling => {
            "Twelve-tone writing avoids doubling a note in octaves. As Schoenberg explained, a \
             doubled note gets weight, and a note with weight can be heard as a root, or even as \
             a tonic, which this music does without."
        }
        SlipKind::TonalChord => {
            "Schoenberg and Webern avoided the chords of tonal harmony, major and minor triads \
             and dominant sevenths: even a reminder of tonal harmony makes the ear expect the \
             music to go on as tonal music would. Berg used them on purpose: the row of his \
             Violin Concerto is built of triads."
        }
        _ => "",
    }
}

/// The line summing up the check in a row: the forms played, and how
/// many of each kind of slip.
pub(super) fn summary(check: &VoiceLeading) -> String {
    let labels: Vec<String> = check
        .rows
        .iter()
        .map(|row| match row.before {
            0 => row.label.to_string(),
            before => format!("{} from note {}", row.label, before + 1),
        })
        .collect();
    let mut channels: Vec<usize> = check.voices.iter().map(|voice| voice.channel).collect();
    channels.sort_unstable();
    channels.dedup();
    let how = if check.rows.iter().any(|row| row.channel.is_some()) {
        ", each track playing forms of its own"
    } else if channels.len() > 1 {
        ", through all the tracks together"
    } else {
        ""
    };
    let read = match labels.len() {
        0 => "no notes of the row yet".to_string(),
        1..=8 => format!("{}{how}", labels.join(", ")),
        n => format!("{} … ({n} forms){how}", labels[..8].join(", ")),
    };
    let found = check
        .counts()
        .unwrap_or_else(|| "nothing twelve-tone writing avoids".to_string());
    format!("Twelve-tone rules, {read}: {found}.")
}

#[cfg(test)]
mod tests {
    use super::super::Rules;
    use super::super::{beats, check, voices_of};
    use super::*;
    use crate::TimeSignature;

    /// The row of Schoenberg's Suite for piano, Op. 25, on E: its form
    /// P4 is E F G D♭ G♭ E♭ A♭ D B C A B♭.
    fn suite(_: usize) -> Scale {
        let row = ToneRow::parse("0 1 3 9 2 11 4 10 7 8 5 6").expect("a row");
        Scale::new(PitchClass::E, ScaleKind::Row(row))
    }

    const P4: &str = "E4 F4 G4 Db5 Gb4 Eb4 Ab4 D5 B4 C5 A4 Bb4";
    const P10: &str = "Bb4 B4 Db5 G4 C5 A4 D5 Ab4 F4 Gb4 Eb4 E4";

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

    fn twelve_tone(tracks: &[Vec<NoteEvent>]) -> VoiceLeading {
        check_in(tracks, suite)
    }

    fn kinds(check: &VoiceLeading) -> Vec<SlipKind> {
        check.slips.iter().map(|slip| slip.kind).collect()
    }

    fn labels(check: &VoiceLeading) -> Vec<String> {
        check.rows.iter().map(|row| row.label.to_string()).collect()
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
    fn a_row_played_in_order_has_nothing_to_avoid() {
        let check = twelve_tone(&[track(P4, 4, 0)]);
        assert_eq!(check.rules, Rules::TwelveTone);
        assert_eq!(kinds(&check), vec![]);
        assert_eq!(labels(&check), vec!["P4"]);
        assert_eq!((check.rows[0].start, check.rows[0].end), (0, 48));
        assert_eq!(
            check.summary(),
            "Twelve-tone rules, P4: nothing twelve-tone writing avoids."
        );
    }

    #[test]
    fn notes_next_to_each_other_in_the_row_may_sound_together() {
        // The first three notes as a chord, in any order from the bottom,
        // then D♭ G♭ E♭ as another, then the rest one by one.
        let check = twelve_tone(&[track("G3+E4+F4 Eb4+Db5+Gb4 Ab4 D5 B4 C5 A4 Bb4", 4, 0)]);
        assert_eq!(kinds(&check), vec![]);
        assert_eq!(labels(&check), vec!["P4"]);
    }

    #[test]
    fn two_notes_swapped_are_one_note_out_of_order() {
        let check = twelve_tone(&[track("E4 F4 Db5 G4 Gb4 Eb4 Ab4 D5 B4 C5 A4 Bb4", 4, 0)]);
        assert_eq!(kinds(&check), vec![SlipKind::RowOrder]);
        assert_eq!(check.slips[0].step(), 8);
        assert_eq!(
            told(&check, SlipKind::RowOrder),
            "Out of row order: Melody plays C#5 where P4 wants G, its note 3; C# is note 4 \
             of P4."
        );
    }

    #[test]
    fn a_note_left_out_is_one_note_out_of_order_not_all_the_rest() {
        // D♭ is left out: G♭ comes where the row wants it, and the
        // notes after G♭ are in order again.
        let check = twelve_tone(&[track("E4 F4 G4 Gb4 Eb4 Ab4 D5 B4 C5 A4 Bb4", 4, 0)]);
        assert_eq!(kinds(&check), vec![SlipKind::RowOrder]);
        assert!(
            told(&check, SlipKind::RowOrder).contains("plays F#4 where P4 wants C#, its note 4"),
            "{}",
            told(&check, SlipKind::RowOrder)
        );
    }

    #[test]
    fn a_note_may_be_repeated_or_trilled_but_not_brought_back_later() {
        // E F E G: E trills with F, the note after it.
        let trill = twelve_tone(&[track("E4 F4 E4 G4 Db5 Gb4 Eb4 Ab4 D5 B4 C5 A4 Bb4", 4, 0)]);
        assert_eq!(kinds(&trill), vec![]);
        let repeated = twelve_tone(&[track("E4 E4 F4 G4 Db5 Gb4 Eb4 Ab4 D5 B4 C5 A4 Bb4", 4, 0)]);
        assert_eq!(kinds(&repeated), vec![]);
        // E F G D♭ E: E comes back after four notes.
        let early = twelve_tone(&[track("E4 F4 G4 Db5 E4 Gb4 Eb4 Ab4 D5 B4 C5 A4 Bb4", 4, 0)]);
        assert_eq!(kinds(&early), vec![SlipKind::EarlyRepeat]);
        assert_eq!(
            told(&early, SlipKind::EarlyRepeat),
            "Note repeated early: Melody plays E4 again before P4 has sounded all twelve notes \
             (4 so far); the row wants F#, its note 5."
        );
    }

    #[test]
    fn one_form_may_end_on_the_note_the_next_begins_with() {
        // P4 ends on B♭, and P10 starts on it: the note is shared.
        let elided = format!("{P4} {}", &P10[4..]);
        let check = twelve_tone(&[track(&elided, 4, 0)]);
        assert_eq!(kinds(&check), vec![]);
        assert_eq!(labels(&check), vec!["P4", "P10"]);
        // Played twice, it is a note repeated straight away.
        let check = twelve_tone(&[track(&format!("{P4} {P10}"), 4, 0)]);
        assert_eq!(kinds(&check), vec![]);
        assert_eq!(labels(&check), vec!["P4", "P10"]);
    }

    #[test]
    fn a_form_that_stops_short_leaves_a_gap() {
        let check = twelve_tone(&[track(&format!("E4 F4 G4 Db5 Gb4 Eb4 Ab4 D5 {P10}"), 4, 0)]);
        let all: Vec<String> = check
            .slips
            .iter()
            .map(|slip| slip.what(|_| "M".into()))
            .collect();
        assert_eq!(
            kinds(&check),
            vec![SlipKind::RowUnfinished],
            "{all:?} {:?}",
            labels(&check)
        );
        assert_eq!(labels(&check), vec!["P4", "P10"]);
        assert_eq!(check.slips[0].step(), 28);
        assert_eq!(
            told(&check, SlipKind::RowUnfinished),
            "Row left unfinished: P4 stops after 8 of its 12 notes, with Melody's D5; B, C, A \
             and Bb never sound before the next form starts."
        );
        // The last form may go on in the next pattern, and the first may
        // have begun in the pattern before.
        let check = twelve_tone(&[track("Gb4 Eb4 Ab4 D5 B4 C5 A4 Bb4 B4 Db5 G4", 4, 0)]);
        assert_eq!(kinds(&check), vec![]);
        assert_eq!(labels(&check), vec!["P4", "P10"]);
        assert_eq!(check.rows[0].before, 4);
        assert_eq!(
            check.rows[0].describe(),
            "P4: the row starting on E: E F G C# F# Eb Ab D B C A Bb. The pattern starts at its \
             note 5: the notes before it come in the pattern before."
        );
        assert_eq!(
            check.rows[1].describe(),
            "P10: the row starting on Bb: Bb B C# G C A D Ab F F# Eb E."
        );
        assert_eq!(
            check.summary(),
            "Twelve-tone rules, P4 from note 5, P10: nothing twelve-tone writing avoids."
        );
    }

    #[test]
    fn a_tune_and_its_chords_may_share_one_form() {
        // The melody plays notes 1-3 and 7-9, the chords 4-6 and 10-12.
        let check = twelve_tone(&[
            track("E5 F5 G5 - Ab5 D5 B5", 4, 0),
            track("Db4+Gb4+Eb4 C4+A3+Bb3", 16, 12),
        ]);
        assert_eq!(kinds(&check), vec![]);
        assert_eq!(labels(&check), vec!["P4"]);
        assert_eq!(check.rows[0].channel, None);
        assert_eq!(
            check.summary(),
            "Twelve-tone rules, P4, through all the tracks together: nothing twelve-tone \
             writing avoids."
        );
    }

    #[test]
    fn each_track_may_play_forms_of_its_own() {
        // A canon: the melody plays P4, and the chords' track plays P10
        // a note later, an octave and more lower.
        let check = twelve_tone(&[
            track(P4, 4, 0),
            track("Bb2 B2 Db3 G2 C3 A2 D3 Ab2 F2 Gb2 Eb2 E2", 4, 2),
        ]);
        let found = kinds(&check);
        assert!(!found.contains(&SlipKind::RowOrder), "{found:?}");
        assert!(!found.contains(&SlipKind::EarlyRepeat), "{found:?}");
        assert_eq!(labels(&check), vec!["P4", "P10"]);
        assert_eq!(check.rows[0].channel, Some(0));
        assert_eq!(check.rows[1].channel, Some(1));
        assert!(
            check
                .summary()
                .contains("P4, P10, each track playing forms of its own"),
            "{}",
            check.summary()
        );
    }

    #[test]
    fn a_note_doubled_in_octaves_stands_out() {
        // A chord with E in two octaves: one note of the row, doubled.
        let check = twelve_tone(&[track("E3+E4 F4 G4 Db5 Gb4 Eb4 Ab4 D5 B4 C5 A4 Bb4", 4, 0)]);
        assert_eq!(kinds(&check), vec![SlipKind::OctaveDoubling]);
        assert_eq!(labels(&check), vec!["P4"]);
        assert_eq!(
            told(&check, SlipKind::OctaveDoubling),
            "Octave doubling: Melody and Melody both play E (E4 and E3)."
        );
        // Two tracks playing the row in octaves are one line, told once.
        let check = twelve_tone(&[track(P4, 4, 0), track(&P4.replace('4', "3"), 4, 0)]);
        assert_eq!(kinds(&check), vec![SlipKind::OctaveDoubling]);
        assert!(check.slips[0].doubled);
        assert_eq!(labels(&check), vec!["P4"]);
        assert_eq!(
            told(&check, SlipKind::OctaveDoubling),
            "Octave doubling: Melody and Chords play one line in octaves, from E4 and E3 on."
        );
        // A note against one held from before.
        let check = twelve_tone(&[
            vec![NoteEvent::new(note("E4"), 0, 16)],
            vec![NoteEvent::new(note("E5"), 8, 4)],
        ]);
        assert_eq!(kinds(&check), vec![SlipKind::OctaveDoubling]);
        assert_eq!(
            told(&check, SlipKind::OctaveDoubling),
            "Octave doubling: Chords plays E5 while Melody still holds E4."
        );
    }

    #[test]
    fn a_triad_sounding_on_its_own_sounds_tonal() {
        // The Suite's row has no triad in it; C E G played together is
        // one, whatever the order.
        let check = twelve_tone(&[track("E4+C4+G4", 4, 0)]);
        assert!(
            kinds(&check).contains(&SlipKind::TonalChord),
            "{:?}",
            kinds(&check)
        );
        assert_eq!(
            told(&check, SlipKind::TonalChord),
            "Sounds tonal: C4 E4 G4 make C, a major triad, a chord of tonal harmony."
        );
        let check = twelve_tone(&[track("F3+B3+D4+G4", 4, 0)]);
        assert!(
            told(&check, SlipKind::TonalChord).contains("a dominant seventh"),
            "{}",
            told(&check, SlipKind::TonalChord)
        );
        // Three notes of the row as a chord that is no triad: E F G.
        let check = twelve_tone(&[track("E4+F4+G4", 4, 0)]);
        assert!(!kinds(&check).contains(&SlipKind::TonalChord));
    }

    #[test]
    fn in_a_row_the_classical_rules_do_not_apply() {
        // Parallel fifths and a falling leading tone, written in a row.
        let check = twelve_tone(&[track("G4 A4", 4, 0), track("C3 D3", 4, 0)]);
        let found = kinds(&check);
        assert!(!found.contains(&SlipKind::ParallelFifths), "{found:?}");
    }

    #[test]
    fn a_pitch_class_set_has_no_rules_of_its_own() {
        let set = |_| {
            Scale::new(
                PitchClass::C,
                ScaleKind::Set(PitchSet::from_semitones(&[0, 1, 4])),
            )
        };
        let check = check_in(&[track("G4 A4", 4, 0), track("C3 D3", 4, 0)], set);
        assert_eq!(check.rules, Rules::Neither);
        assert_eq!(kinds(&check), vec![]);
        assert!(check.summary().contains("a pitch-class set has no rules"));
    }
}
