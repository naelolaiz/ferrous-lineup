//! Voice leading: how the parts move from one chord to the next.
//!
//! The chord lane and the harmony keys name each chord. Voice leading is
//! the other half of harmony: how each part, a singer, an instrument or
//! one line of a piano's chords, moves from its note in one chord to its
//! note in the next. Classical part writing, the four-part harmony of
//! chorales that harmony courses teach, keeps every part a line of its
//! own and has a few rules for it. This module checks the best known:
//!
//! - **No parallel fifths or octaves.** Two parts a perfect fifth apart
//!   (or an octave apart, or in unison) that both move the same way into
//!   another fifth (octave, unison) stop sounding like two lines: these
//!   intervals blend so well that for that moment the two parts melt
//!   into one. A fifth plus an octave (a twelfth) counts as a fifth.
//!   Fifths and octaves "by contrary motion", where the parts move apart
//!   from a fifth to a twelfth, are still two fifths in a row, and strict
//!   part writing avoids them too.
//! - **The leading tone rises.** When a dominant chord (V, V7, vii°,
//!   vii°7) goes to the tonic, its leading tone, the note a semitone
//!   under the tonic, pulls up to it. The rule is strict in the top part
//!   and the bass, where the ear follows it; an inner part is freer (it
//!   may drop to the fifth of the tonic chord, so that chord has all its
//!   notes), and is left alone. In a deceptive cadence (V to vi) the
//!   leading tone still rises. A secondary dominant (D7 going to G in C
//!   major) has a leading tone of its own, F#, which rises to G the same
//!   way. A leading tone on its way down a scale (the tonic, the leading
//!   tone, the sixth) is a passing note, not one that must rise.
//! - **A seventh falls.** The seventh of a seventh chord is a dissonance,
//!   and resolves down by step into the next chord. It may stay where it
//!   is when the next chord has the same note, or be held over (a
//!   suspension) and fall a little later. In V⁴₃ going to I⁶ it may rise
//!   a step, alongside the bass rising a step a tenth below it. A note
//!   shorter than a beat that comes and goes by step is a passing or a
//!   neighbour note, even where it makes a seventh, and is left alone.
//!
//! These are the rules of one style, the part writing of Bach to Brahms.
//! Pop and jazz move whole chords in parallel on purpose (Debussy did
//! too), and nobody expects a jazz seventh to resolve: the check is a
//! teacher of the classical style, not a judge of all music.
//!
//! Where the key is a twelve-tone row, the music has no tonic, no
//! leading tone and no sevenths to resolve, and these rules mean
//! nothing. The rules of twelve-tone writing take their place (see
//! [`serial`]): the notes follow the forms of the row in order, no note
//! comes back before the other eleven have sounded, no note is doubled
//! in octaves, and no chord sounds like tonal harmony. Where the key is
//! a pitch-class set, such as one of Kröpfl's micromodes, neither
//! style's rules apply: the check reads which forms of the set the
//! notes make and marks what is out of the set's colour instead (see
//! [`colour`]).
//!
//! ## Which notes make a part
//!
//! A track that plays one note at a time is one part. A track that plays
//! chords, such as a piano, holds several: each new note joins the part
//! nearest to it, so the notes of a chord are followed into the next
//! chord the way a reader of a score follows each voice. A rest of a
//! beat or more ends a part's line: the notes on either side of it are
//! not checked against each other.
//!
//! A line played in octaves (or in unison) all the way through, as when
//! an orchestra's double basses double its cellos, is one line made
//! louder, not two parts in parallel octaves: such a pair is reported as
//! a doubling, and only one of the two is checked.

pub mod colour;
pub mod serial;

pub use colour::{ColourPlace, FormStatement};

use super::hearing::{HeardNote, Listening};
use super::{
    Chord, ChordFamily, ChordQuality, Harmony, PitchSet, RowLabel, Scale, ScaleKind, analyze,
    with_figures,
};
use crate::i18n::{tr, trf};
use crate::{Note, NoteEvent, PitchClass, TICKS_PER_STEP, TimeSignature};

/// A rest this long or longer ends a part's line: a quarter note, in
/// ticks.
const LINE_BREAK: usize = 4 * TICKS_PER_STEP;

/// Two notes of a track that overlap by less than this, in ticks, are
/// one after the other, played legato, not a chord.
const LEGATO: usize = TICKS_PER_STEP;

/// What a new part costs when the notes of a chord are shared out among
/// the parts: more than any distance between notes, so a track makes a
/// new part only when it plays more notes at once than it had parts.
const NEW_PART: u32 = 10_000;

/// One part: a line of notes, one at a time.
#[derive(Debug, Clone, PartialEq)]
pub struct Voice {
    /// The channel (track) it is played on.
    pub channel: usize,
    /// Which of the track's parts it is, from 0 for the highest.
    pub rank: usize,
    /// How many parts the track holds.
    pub of: usize,
    /// Its notes, in the order they start. A note may overlap the next
    /// one a little, when they are played legato.
    pub notes: Vec<NoteEvent>,
}

impl Voice {
    /// The name of the part on the track named `track`: the track's name
    /// when it is the track's only part, otherwise which of its parts it
    /// is, as "Piano (top voice)".
    pub fn name(&self, track: &str) -> String {
        match (self.rank, self.of) {
            (_, 0 | 1) => track.to_string(),
            (0, 2) => trf("{} (upper voice)", &[&track]),
            (_, 2) => trf("{} (lower voice)", &[&track]),
            (0, _) => trf("{} (top voice)", &[&track]),
            (rank, of) if rank + 1 == of => trf("{} (lowest voice)", &[&track]),
            (rank, of) => trf("{} (voice {} of {})", &[&track, &(rank + 1), &of]),
        }
    }

    /// The index of the note the part is on at tick `tick`: the last
    /// one started by then, unless the part has rested a beat or more
    /// since it ended.
    fn current(&self, tick: usize) -> Option<usize> {
        let index = self
            .notes
            .partition_point(|note| note.start_tick() <= tick)
            .checked_sub(1)?;
        (tick < self.notes[index].end_tick() + LINE_BREAK).then_some(index)
    }
}

/// Splits the notes of track `channel` into parts (see the module
/// documentation): one for a track that plays a note at a time, several
/// for a track that plays chords, highest first.
pub fn voices_of(channel: usize, notes: &[NoteEvent]) -> Vec<Voice> {
    let mut notes = notes.to_vec();
    notes.sort_by_key(|note| (note.start_tick(), std::cmp::Reverse(note.pitch)));
    let mut lines: Vec<Vec<NoteEvent>> = Vec::new();
    for chord in notes.chunk_by(|a, b| a.start_tick() == b.start_tick()) {
        let tick = chord[0].start_tick();
        // The parts free to take a note: those whose last note has
        // ended, or nearly (legato), highest first.
        let mut free: Vec<usize> = (0..lines.len())
            .filter(|&line| {
                lines[line]
                    .last()
                    .is_some_and(|last| last.end_tick() <= tick + LEGATO.min(last.ticks() / 2))
            })
            .collect();
        free.sort_by_key(|&line| std::cmp::Reverse(lines[line].last().map(|note| note.pitch)));
        let last_pitches: Vec<Note> = free
            .iter()
            .filter_map(|&line| lines[line].last().map(|note| note.pitch))
            .collect();
        for (note, part) in chord.iter().zip(share_out(chord, &last_pitches)) {
            match part {
                Some(index) => lines[free[index]].push(*note),
                None => lines.push(vec![*note]),
            }
        }
    }
    // Highest first, by the average pitch of each line.
    let average = |line: &Vec<NoteEvent>| {
        line.iter()
            .map(|note| u32::from(note.pitch.midi()))
            .sum::<u32>()
            * 1000
            / line.len().max(1) as u32
    };
    lines.sort_by_key(|line| std::cmp::Reverse(average(line)));
    let of = lines.len();
    lines
        .into_iter()
        .enumerate()
        .map(|(rank, notes)| Voice {
            channel,
            rank,
            of,
            notes,
        })
        .collect()
}

/// Which of the parts that last played `parts` (highest first) each note
/// of `chord` (highest first) goes on: the nearest, keeping their order,
/// so that the top note of a chord goes to the top part. `None` starts a
/// new part, which happens only when there are more notes than parts.
fn share_out(chord: &[NoteEvent], parts: &[Note]) -> Vec<Option<usize>> {
    let (notes, lines) = (chord.len(), parts.len());
    let distance =
        |note: usize, part: usize| u32::from(chord[note].pitch.midi().abs_diff(parts[part].midi()));
    // cost[i][j]: the least cost of placing the first i notes, using
    // only the first j parts, each part taking one note at most.
    let mut cost = vec![vec![0u32; lines + 1]; notes + 1];
    for i in 1..=notes {
        cost[i][0] = cost[i - 1][0] + NEW_PART;
    }
    for i in 1..=notes {
        for j in 1..=lines {
            let skip_part = cost[i][j - 1];
            let new_part = cost[i - 1][j] + NEW_PART;
            let take = cost[i - 1][j - 1] + distance(i - 1, j - 1);
            cost[i][j] = skip_part.min(new_part).min(take);
        }
    }
    // Walk back through the table to see which choice made each cost.
    let mut placed = vec![None; notes];
    let (mut i, mut j) = (notes, lines);
    while i > 0 {
        if j > 0 && cost[i][j] == cost[i][j - 1] {
            j -= 1;
        } else if j > 0 && cost[i][j] == cost[i - 1][j - 1] + distance(i - 1, j - 1) {
            placed[i - 1] = Some(j - 1);
            i -= 1;
            j -= 1;
        } else {
            i -= 1;
        }
    }
    placed
}

/// What is wrong with a move.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SlipKind {
    /// Two parts move the same way from a fifth to a fifth.
    ParallelFifths,
    /// Two parts move the same way from an octave to an octave.
    ParallelOctaves,
    /// Two parts move together from a unison to a unison.
    ParallelUnisons,
    /// Two parts move apart (or together) from a fifth to a fifth.
    ContraryFifths,
    /// Two parts move apart (or together) from an octave or unison to
    /// another.
    ContraryOctaves,
    /// A leading tone that does not rise to the tonic.
    LeadingTone,
    /// A seventh that does not fall by step.
    Seventh,
    /// In a row: a note that comes before its turn in the row.
    RowOrder,
    /// In a row: a note that comes back before the row has sounded all
    /// twelve.
    EarlyRepeat,
    /// In a row: a form of the row that stops before its twelfth note,
    /// when the next one starts.
    RowUnfinished,
    /// In a row: two parts sounding the same note in octaves or unison.
    OctaveDoubling,
    /// In a row: notes that sound together as a chord of tonal music,
    /// a major or minor triad or a dominant seventh.
    TonalChord,
    /// In a set: a note outside the form of the set heard around it.
    OutsideColour,
    /// In a micromode: a form that stops before its three notes have
    /// sounded, when the next one starts.
    FormUnfinished,
}

impl SlipKind {
    /// The name, as a heading: "Parallel fifths".
    pub fn name(self) -> &'static str {
        match self {
            SlipKind::ParallelFifths => "Parallel fifths",
            SlipKind::ParallelOctaves => "Parallel octaves",
            SlipKind::ParallelUnisons => "Parallel unisons",
            SlipKind::ContraryFifths => "Fifths by contrary motion",
            SlipKind::ContraryOctaves => "Octaves by contrary motion",
            SlipKind::LeadingTone => "Leading tone not rising",
            SlipKind::Seventh => "Seventh not falling",
            SlipKind::RowOrder => "Out of row order",
            SlipKind::EarlyRepeat => "Note repeated early",
            SlipKind::RowUnfinished => "Row left unfinished",
            SlipKind::OctaveDoubling => "Octave doubling",
            SlipKind::TonalChord => "Sounds tonal",
            SlipKind::OutsideColour => "Outside the colour",
            SlipKind::FormUnfinished => "Form left unfinished",
        }
    }

    /// A few letters for a narrow place, such as the chord lane.
    pub fn short(self) -> &'static str {
        match self {
            SlipKind::ParallelFifths | SlipKind::ContraryFifths => "5ths",
            SlipKind::ParallelOctaves | SlipKind::ContraryOctaves => "8ves",
            SlipKind::ParallelUnisons => "1sns",
            SlipKind::LeadingTone => "LT",
            SlipKind::Seventh => "7th",
            SlipKind::RowOrder => "order",
            SlipKind::EarlyRepeat => "rep",
            SlipKind::RowUnfinished | SlipKind::FormUnfinished => "gap",
            SlipKind::OctaveDoubling => "8ve",
            SlipKind::TonalChord => "tonal",
            SlipKind::OutsideColour => "out",
        }
    }
}

/// A part going from one note to the next.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartMove {
    /// The part, as an index into [`VoiceLeading::voices`].
    pub voice: usize,
    pub from: NoteEvent,
    pub to: NoteEvent,
}

/// A note one part plays, for what is about notes rather than moves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartNote {
    /// The part, as an index into [`VoiceLeading::voices`].
    pub voice: usize,
    pub note: NoteEvent,
}

/// Where a note stands in a form of the row, for the twelve-tone rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RowPlace {
    /// The form of the row the notes there are read as.
    pub label: RowLabel,
    /// The note's place in that form, from 1 to 12.
    pub order: usize,
    /// The note the row wanted at that moment, and its place.
    pub wanted: Option<(PitchClass, usize)>,
    /// How many of the form's twelve notes had sounded.
    pub heard: usize,
    /// For a form left unfinished: the notes that never sounded.
    pub missing: PitchSet,
}

/// One form of the row played through, for the twelve-tone rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RowStatement {
    pub label: RowLabel,
    /// From its first note's step to the step after its last note ends.
    pub start: usize,
    pub end: usize,
    /// The track that plays it, or `None` when the row runs through all
    /// the tracks together.
    pub channel: Option<usize>,
    /// For a form the pattern starts in the middle of: how many of its
    /// notes come before, in the pattern before. Otherwise 0.
    pub before: usize,
    /// The key there, the row.
    pub key: Scale,
}

/// The two chords a leading tone or a seventh goes between.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Resolution {
    /// The chord the note is in. A chord named by its sixth that works
    /// as a seventh chord is given as that seventh chord: F6 going to G
    /// is Dm7 with F in the bass, the ii⁶₅ of C major.
    pub from: Chord,
    pub from_bass: PitchClass,
    pub to: Chord,
    pub to_bass: PitchClass,
}

/// Something the parts do that classical part writing avoids.
#[derive(Debug, Clone, PartialEq)]
pub struct Slip {
    pub kind: SlipKind,
    /// The parts that move: two for parallels, one for a leading tone or
    /// a seventh.
    pub moves: Vec<PartMove>,
    /// The key there, which spells the notes.
    pub key: Scale,
    /// For a leading tone or a seventh: the chords it goes between.
    pub chords: Option<Resolution>,
    /// For a leading tone or a seventh: where it should have gone.
    pub expected: Option<Note>,
    /// For a leading tone: whether another part has the same one, which
    /// classical part writing avoids, since both cannot rise without
    /// parallel octaves. For an octave doubling: whether the two parts
    /// double each other all the way through, as one line.
    pub doubled: bool,
    /// For the twelve-tone rules, which are about notes rather than
    /// moves: the notes concerned.
    pub notes: Vec<PartNote>,
    /// For the row's order: where the note stands in the row.
    pub row: Option<RowPlace>,
    /// For a chord that sounds tonal: the chord, and its lowest note.
    pub chord: Option<(Chord, PitchClass)>,
    /// For the colour check: the form of the set heard there.
    pub colour: Option<ColourPlace>,
}

impl Slip {
    /// A slip of `kind` in `key`, with nothing else filled in yet.
    fn of(kind: SlipKind, key: Scale) -> Slip {
        Slip {
            kind,
            moves: Vec::new(),
            key,
            chords: None,
            expected: None,
            doubled: false,
            notes: Vec::new(),
            row: None,
            chord: None,
            colour: None,
        }
    }

    /// The step where the move lands, or where the notes concerned
    /// start: where the chord lane marks it.
    pub fn step(&self) -> usize {
        let moved = self.moves.iter().map(|m| m.to.start);
        let played = self.notes.iter().map(|n| n.note.start);
        moved.chain(played).max().unwrap_or(0)
    }

    /// The steps where the parts arrive, from where the move lands to
    /// where the first of the notes moved to ends: the piano roll marks
    /// them.
    pub fn steps(&self) -> std::ops::Range<usize> {
        let start = self.step();
        let moved = self.moves.iter().map(|m| m.to.end());
        let played = self.notes.iter().map(|n| n.note.end());
        let end = moved.chain(played).min().unwrap_or(start);
        start..end.max(start + 1)
    }

    /// What happens and why it is avoided, in a few sentences.
    /// `voice_name` names each part by its index.
    pub fn describe(&self, voice_name: impl Fn(usize) -> String) -> String {
        format!("{} {}", self.what(voice_name), self.why())
    }

    /// What happens, in a sentence: "Parallel fifths: Piano (top voice)
    /// and Bass go from G4 over C3 (a twelfth, a fifth plus an octave)
    /// to A4 over D3 (...), both up."
    pub fn what(&self, voice_name: impl Fn(usize) -> String) -> String {
        let key = self.key;
        let note = |note: Note| key.note_name(note);
        match (self.kind, self.moves.as_slice(), self.chords) {
            (_, [a, b], _) => {
                // The upper part first, as on a score.
                let (high, low) = if a.from.pitch >= b.from.pitch {
                    (a, b)
                } else {
                    (b, a)
                };
                let (up, down) = (a.to.pitch > a.from.pitch, b.to.pitch > b.from.pitch);
                let way = match (up, down) {
                    (true, true) => "both up",
                    (false, false) => "both down",
                    _ => "one up and one down",
                };
                trf(
                    "{}: {} and {} go from {} over {} ({}) to {} over {} ({}), {}.",
                    &[
                        &tr(self.kind.name()),
                        &voice_name(high.voice),
                        &voice_name(low.voice),
                        &note(high.from.pitch),
                        &note(low.from.pitch),
                        &perfect_name(high.from.pitch.midi().abs_diff(low.from.pitch.midi())),
                        &note(high.to.pitch),
                        &note(low.to.pitch),
                        &perfect_name(high.to.pitch.midi().abs_diff(low.to.pitch.midi())),
                        &tr(way),
                    ],
                )
            }
            (SlipKind::LeadingTone, [m], Some(chords)) => trf(
                "Leading tone not rising: {} has {}, the leading tone, in {} and {} in {} \
                 instead of rising a semitone to {}.",
                &[
                    &voice_name(m.voice),
                    &chord_note(key, chords.from, m.from.pitch),
                    &chord_label(key, chords.from, chords.from_bass),
                    &went(key, m.from.pitch, m.to.pitch),
                    &chord_label(key, chords.to, chords.to_bass),
                    &self.expected.map(note).unwrap_or_default(),
                ],
            ),
            (SlipKind::Seventh, [m], Some(chords)) => trf(
                "Seventh not falling: {} has {}, the seventh of {}, and {} in {} instead of \
                 falling a step{}.",
                &[
                    &voice_name(m.voice),
                    &chord_note(key, chords.from, m.from.pitch),
                    &chord_label(key, chords.from, chords.from_bass),
                    &went(key, m.from.pitch, m.to.pitch),
                    &chord_label(key, chords.to, chords.to_bass),
                    &self
                        .expected
                        .map(|to| trf(" to {}", &[&note(to)]))
                        .unwrap_or_default(),
                ],
            ),
            (SlipKind::OutsideColour | SlipKind::FormUnfinished, _, _) => {
                colour::what(self, voice_name)
            }
            _ => serial::what(self, voice_name),
        }
    }

    /// Why classical part writing avoids it.
    pub fn why(&self) -> String {
        let mut text = tr(why(self.kind)).to_string();
        if self.doubled {
            text += tr(
                " The leading tone is doubled here: another part has it too, and both \
                        cannot rise without moving in octaves, so classical part writing does \
                        not double it.",
            );
        }
        text
    }
}

/// Why each kind of consecutive interval is avoided.
pub(crate) fn why(kind: SlipKind) -> &'static str {
    match kind {
        SlipKind::ParallelFifths => {
            "Two parts a fifth apart that move the same way into another fifth blend into \
             one sound and stop being heard as two lines, so classical part writing avoids \
             them. Keep a note both chords share, or move the parts the opposite way."
        }
        SlipKind::ParallelOctaves => {
            "Two parts an octave apart that move the same way into another octave double \
             each other: for that moment there is one line fewer. Classical part writing \
             keeps its parts apart, so it avoids them."
        }
        SlipKind::ParallelUnisons => {
            "Two parts on the same note that move together to another shared note are one \
             part for that moment, so classical part writing avoids them."
        }
        SlipKind::ContraryFifths => {
            "The parts move apart, but the ear still hears two fifths in a row, so strict \
             part writing avoids these too."
        }
        SlipKind::ContraryOctaves => {
            "The parts move apart, but the ear still hears two octaves in a row, so strict \
             part writing avoids these too, though they turn up now and then between the top \
             part and the bass at a cadence."
        }
        SlipKind::LeadingTone => {
            "The leading tone, a semitone under the note it leads to, pulls up to it, and in \
             the top part or the bass, where the ear follows it, it should rise."
        }
        SlipKind::Seventh => {
            "A chord's seventh is a dissonance, and classical part writing resolves it down \
             by step into the next chord."
        }
        SlipKind::OutsideColour | SlipKind::FormUnfinished => colour::why(kind),
        kind => serial::why(kind),
    }
}

/// The name of a perfect interval `semitones` wide: "a fifth", "a
/// twelfth", "two octaves".
fn perfect_name(semitones: u8) -> String {
    match semitones {
        0 => tr("a unison").to_string(),
        7 => tr("a fifth").to_string(),
        12 => tr("an octave").to_string(),
        19 => tr("a twelfth, a fifth plus an octave").to_string(),
        s if s % 12 == 0 => trf("{} octaves", &[&(s / 12)]),
        s => trf("a fifth plus {} octaves", &[&(s / 12)]),
    }
}

/// A chord's name and its Roman numeral in `key`: "G7 (V7)", "D7/F#
/// (V⁶₅/V)".
fn chord_label(key: Scale, chord: Chord, bass: PitchClass) -> String {
    let name = Harmony::Chord { chord, bass }
        .label_in(key)
        .unwrap_or_else(|| key.chord_name(chord));
    let numeral = match key.secondary_dominant_target(chord) {
        Some(target) if key.borrowed_from(chord).is_none() => Some(with_figures(
            &format!(
                "V{}/{}",
                chord.quality.numeral_suffix(),
                key.roman_numeral(target).unwrap_or_default()
            ),
            chord.inversion_figures(bass),
        )),
        _ => key.figured_numeral(chord, bass),
    };
    match numeral {
        Some(numeral) => format!("{name} ({numeral})"),
        None => name,
    }
}

/// `note` named as the note of `chord` it is, in `key`: the seventh of
/// B diminished seventh is A flat, never G sharp.
fn chord_note(key: Scale, chord: Chord, note: Note) -> String {
    match chord.spell_note(key.spell_root(chord), note.pitch_class()) {
        Some(name) => note.name_as(name),
        None => key.note_name(note),
    }
}

/// Where a part went from `from` to `to`: "falls to G3", "leaps down
/// to D4", "rises to G4".
fn went(key: Scale, from: Note, to: Note) -> String {
    let name = key.note_name(to);
    let size = from.midi().abs_diff(to.midi());
    match (to > from, size) {
        (true, 1 | 2) => trf("rises to {}", &[&name]),
        (true, _) => trf("leaps up to {}", &[&name]),
        (false, 1 | 2) => trf("falls to {}", &[&name]),
        (false, _) => trf("leaps down to {}", &[&name]),
    }
}

/// The voice leading of some parts: the parts, what they do that
/// classical part writing avoids, and which of them double each other.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VoiceLeading {
    pub voices: Vec<Voice>,
    /// In the order they happen.
    pub slips: Vec<Slip>,
    /// Pairs of parts that play one line in octaves or unison: the part
    /// checked, then the one that doubles it, which is not checked.
    pub doublings: Vec<(usize, usize)>,
    /// Which rules were checked: those of the key at the start.
    pub rules: Rules,
    /// Where the key is a row: the forms of the row played, in order.
    pub rows: Vec<RowStatement>,
    /// Where the key is a set: the forms of the set the notes make, in
    /// order.
    pub forms: Vec<FormStatement>,
}

/// Which style's rules the check follows, from the key.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Rules {
    /// In a key of tonal music: classical part writing.
    #[default]
    Classical,
    /// In a twelve-tone row: twelve-tone writing.
    TwelveTone,
    /// In a pitch-class set: whether the notes keep to the set's
    /// colour.
    Colour,
}

impl Rules {
    /// The rules for music in `key`.
    pub fn of(key: Scale) -> Rules {
        match key.kind {
            ScaleKind::Row(_) => Rules::TwelveTone,
            ScaleKind::Set(_) => Rules::Colour,
            _ => Rules::Classical,
        }
    }
}

impl VoiceLeading {
    /// The slips whose move lands on `step`.
    pub fn slips_at(&self, step: usize) -> impl Iterator<Item = &Slip> {
        self.slips
            .iter()
            .filter(move |slip| slip.steps().contains(&step))
    }

    /// A line summing up the check: how many parts, and how many of each
    /// kind of slip.
    pub fn summary(&self) -> String {
        match self.rules {
            Rules::Classical => {}
            Rules::TwelveTone => return serial::summary(self),
            Rules::Colour => return colour::summary(self),
        }
        let parts = self.voices.len() - self.doublings.len();
        if parts < 2 {
            return tr(
                "Voice leading: it takes two parts or more; show the other tracks \
                 (Ghost notes) or write chords.",
            )
            .to_string();
        }
        let found = self
            .counts()
            .unwrap_or_else(|| tr("nothing classical part writing avoids").to_string());
        let doubled = match self.doublings.len() {
            0 => String::new(),
            1 => tr(", one line doubled in octaves").to_string(),
            n => trf(", {} lines doubled in octaves", &[&n]),
        };
        trf(
            "Voice leading, {} parts{}: {}.",
            &[&parts, &doubled, &found],
        )
    }

    /// How many of each kind of slip there are, as "parallel fifths (2),
    /// seventh not falling (1)", or `None` when there are none.
    fn counts(&self) -> Option<String> {
        let mut counts: Vec<(SlipKind, usize)> = Vec::new();
        for slip in &self.slips {
            match counts.iter_mut().find(|(kind, _)| *kind == slip.kind) {
                Some((_, count)) => *count += 1,
                None => counts.push((slip.kind, 1)),
            }
        }
        counts.sort_by_key(|(kind, _)| *kind as usize);
        (!counts.is_empty()).then(|| {
            counts
                .iter()
                .map(|(kind, count)| format!("{} ({count})", tr(kind.name()).to_lowercase()))
                .collect::<Vec<_>>()
                .join(", ")
        })
    }
}

/// The beats of `length` steps of music in `time`, each as its first
/// step and the step after it.
pub fn beats(time: TimeSignature, length: usize) -> Vec<(usize, usize)> {
    let bar = time.steps_per_bar().max(1);
    let starts = time.beat_starts();
    let mut beats = Vec::new();
    for bar_start in (0..length).step_by(bar) {
        for (index, &start) in starts.iter().enumerate() {
            let end = starts.get(index + 1).copied().unwrap_or(bar);
            let (start, end) = (bar_start + start, (bar_start + end).min(length));
            if start < end {
                beats.push((start, end));
            }
        }
    }
    beats
}

/// Checks the voice leading of `voices`. The chords are heard beat by
/// beat, `beats` as [`beats`] gives them, and `key_at` says the key on
/// each step.
pub fn check(
    voices: Vec<Voice>,
    beats: &[(usize, usize)],
    key_at: impl Fn(usize) -> Scale,
) -> VoiceLeading {
    let doublings = doublings(&voices);
    let checked: Vec<usize> = (0..voices.len())
        .filter(|&voice| !doublings.iter().any(|&(_, doubling)| doubling == voice))
        .collect();
    let mut slips = Vec::new();
    // The classical rules hold only where the key is one of tonal music.
    let classical = beats
        .iter()
        .any(|&(start, _)| Rules::of(key_at(start)) == Rules::Classical);
    let pairs = if classical { checked.as_slice() } else { &[] };
    for (index, &a) in pairs.iter().enumerate() {
        for &b in &pairs[index + 1..] {
            for (kind, first, second) in consecutive(&voices[a], &voices[b]) {
                let mut slip = Slip {
                    moves: vec![
                        PartMove {
                            voice: a,
                            from: first.0,
                            to: first.1,
                        },
                        PartMove {
                            voice: b,
                            from: second.0,
                            to: second.1,
                        },
                    ],
                    ..Slip::of(kind, Scale::default())
                };
                slip.key = key_at(slip.step());
                slips.push(slip);
            }
        }
    }
    if classical {
        slips.extend(resolutions(&voices, &checked, beats, &key_at));
    }
    slips.retain(|slip| Rules::of(slip.key) == Rules::Classical);
    let (row_slips, rows) = serial::check(&voices, &doublings, beats, &key_at);
    slips.extend(row_slips);
    let (colour_slips, forms) = colour::check(&voices, beats, &key_at);
    slips.extend(colour_slips);
    slips.sort_by_key(|slip| (slip.step(), slip.kind as usize));
    VoiceLeading {
        voices,
        slips,
        doublings,
        rules: Rules::of(key_at(0)),
        rows,
        forms,
    }
}

/// The notes two parts play together: at every note either of them
/// starts, the note each is on, as indices, or `None` where one of them
/// is not playing.
fn together(a: &Voice, b: &Voice) -> Vec<Option<(usize, usize)>> {
    let mut onsets: Vec<usize> = a
        .notes
        .iter()
        .chain(&b.notes)
        .map(NoteEvent::start_tick)
        .collect();
    onsets.sort_unstable();
    onsets.dedup();
    onsets
        .into_iter()
        .map(|tick| a.current(tick).zip(b.current(tick)))
        .collect()
}

/// The pairs of parts that play one line in octaves or unison: at least
/// four notes together, and seven in eight of them an octave or a unison
/// apart. Each pair is the part kept, then the one doubling it.
fn doublings(voices: &[Voice]) -> Vec<(usize, usize)> {
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    for b in 0..voices.len() {
        if pairs.iter().any(|&(_, doubling)| doubling == b) {
            continue;
        }
        for a in 0..b {
            if pairs.iter().any(|&(_, doubling)| doubling == a) {
                continue;
            }
            let both: Vec<(usize, usize)> = together(&voices[a], &voices[b])
                .into_iter()
                .flatten()
                .collect();
            let octaves = both
                .iter()
                .filter(|&&(i, j)| {
                    let (x, y) = (voices[a].notes[i].pitch, voices[b].notes[j].pitch);
                    x.midi().abs_diff(y.midi()) % 12 == 0
                })
                .count();
            if both.len() >= 4 && octaves * 8 >= both.len() * 7 {
                pairs.push((a, b));
                break;
            }
        }
    }
    pairs
}

/// The kinds of perfect interval, for consecutive fifths and octaves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Perfect {
    Unison,
    Octave,
    Fifth,
}

fn perfect(semitones: u8) -> Option<Perfect> {
    match semitones {
        0 => Some(Perfect::Unison),
        s if s % 12 == 0 => Some(Perfect::Octave),
        s if s % 12 == 7 => Some(Perfect::Fifth),
        _ => None,
    }
}

/// One part's move: the note it leaves and the note it goes to.
type Hop = (NoteEvent, NoteEvent);

/// The consecutive fifths, octaves and unisons between parts `a` and
/// `b`: each with the move of `a`, then of `b`.
fn consecutive(a: &Voice, b: &Voice) -> Vec<(SlipKind, Hop, Hop)> {
    let mut found = Vec::new();
    let mut before: Option<(usize, usize)> = None;
    for now in together(a, b) {
        if let (Some((i0, j0)), Some((i1, j1))) = (before, now) {
            let (x0, x1, y0, y1) = (a.notes[i0], a.notes[i1], b.notes[j0], b.notes[j1]);
            if let Some(kind) = kind_of_move((x0, x1), (y0, y1)) {
                found.push((kind, (x0, x1), (y0, y1)));
            }
        }
        before = now;
    }
    found
}

/// What two parts' moves from one interval to the next make, if it is
/// something to avoid. Both parts must move, each without a rest of a
/// beat or more on the way.
fn kind_of_move(x: Hop, y: Hop) -> Option<SlipKind> {
    let moves = |(from, to): Hop| {
        let rested = to.start_tick() >= from.end_tick() + LINE_BREAK;
        (from.pitch != to.pitch && !rested).then(|| to.pitch > from.pitch)
    };
    let (up_x, up_y) = (moves(x)?, moves(y)?);
    let first = perfect(x.0.pitch.midi().abs_diff(y.0.pitch.midi()))?;
    let second = perfect(x.1.pitch.midi().abs_diff(y.1.pitch.midi()))?;
    let same_way = up_x == up_y;
    match (first, second) {
        (Perfect::Fifth, Perfect::Fifth) if same_way => Some(SlipKind::ParallelFifths),
        (Perfect::Fifth, Perfect::Fifth) => Some(SlipKind::ContraryFifths),
        (Perfect::Unison, Perfect::Unison) => Some(SlipKind::ParallelUnisons),
        (Perfect::Fifth, _) | (_, Perfect::Fifth) => None,
        _ if same_way => Some(SlipKind::ParallelOctaves),
        _ => Some(SlipKind::ContraryOctaves),
    }
}

/// A stretch of beats heard as one chord, or as no chord.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Span {
    start: usize,
    end: usize,
    chord: Option<Chord>,
}

/// The chords of `voices`, heard beat by beat, with the beats of one
/// chord joined.
fn spans(voices: &[Voice], beats: &[(usize, usize)]) -> Vec<Span> {
    let step = TICKS_PER_STEP as f32;
    let heard: Vec<HeardNote> = voices
        .iter()
        .flat_map(|voice| {
            voice.notes.iter().map(|note| HeardNote {
                pitch: note.pitch,
                start: note.start_tick() as f32 / step,
                end: note.end_tick() as f32 / step,
                part: voice.channel,
            })
        })
        .collect();
    let listening = Listening::new(&heard);
    let mut spans: Vec<Span> = Vec::new();
    for &(start, end) in beats {
        let chord = match listening.hear(start as f32, end as f32).best() {
            Harmony::Chord { chord, .. } => Some(chord),
            _ => None,
        };
        match spans.last_mut() {
            Some(last) if last.chord == chord && last.end == start => last.end = end,
            _ => spans.push(Span { start, end, chord }),
        }
    }
    spans
}

/// How a chord leads to the chord a semitone above its leading tone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Leads {
    /// A major chord or dominant seventh a fifth above: V, V7.
    Dominant,
    /// A diminished chord on the leading tone: vii°, viiø7, vii°7.
    LeadingToneChord,
}

/// How `chord` leads to a chord on `target`, if it is its dominant or
/// its leading-tone chord.
fn leads_to(chord: Chord, target: PitchClass) -> Option<Leads> {
    use ChordQuality::*;
    let leading = target.transpose(-1);
    let dominant = matches!(chord.quality, Major | Augmented) || chord.quality.is_dominant();
    if dominant && chord.root == target.transpose(7) {
        return Some(Leads::Dominant);
    }
    let on_leading = match chord.quality {
        Diminished | HalfDiminished7 => chord.root == leading,
        // Any of a diminished seventh chord's notes can be its root: it
        // is the one a semitone under where it goes.
        Diminished7 => (0..4).any(|third| chord.root.transpose(3 * third) == leading),
        // B D F A with D lowest is named Dm6: it is viiø⁶₅.
        Minor6 => chord.root.transpose(9) == leading,
        _ => false,
    };
    on_leading.then_some(Leads::LeadingToneChord)
}

/// The leading tone of `from` going to `to` in `key`, if `to` is where
/// it leads: the tonic, the vi of a deceptive cadence, or the chord a
/// secondary dominant leads to.
fn leading_tone(key: Scale, from: Chord, to: Chord) -> Option<PitchClass> {
    use ChordQuality::*;
    if !key.kind.is_tonal() || !key.is_heptatonic() {
        return None;
    }
    // Only a major or minor chord is somewhere to arrive: a dominant
    // seventh on the tonic (C7 in C major) leads on, to F.
    let arrives = !to.quality.is_dominant()
        && !matches!(
            to.quality,
            Sus2 | Sus4 | Augmented | AugmentedMajor7 | Dominant7Sus4
        )
        && matches!(to.quality.family(), ChordFamily::Major | ChordFamily::Minor);
    if !arrives {
        return None;
    }
    let tonic = key.root;
    let leading = tonic.transpose(-1);
    if to.root == tonic && leads_to(from, tonic).is_some() {
        return Some(leading);
    }
    // The deceptive cadence: V to vi in a major key, V to VI in a minor
    // one. The leading tone still rises, to the tonic, the third of vi.
    let deceptive = (to.root == tonic.transpose(9) && to.quality.family() == ChordFamily::Minor)
        || (to.root == tonic.transpose(8) && to.quality.family() == ChordFamily::Major);
    if deceptive && leads_to(from, tonic) == Some(Leads::Dominant) {
        return Some(leading);
    }
    // A secondary dominant or leading-tone chord, from outside the key,
    // going to the chord it leads to: D7 to G in C major. Its leading
    // tone is a note from outside the key, F#, that pulls up like the
    // key's own. (C7 going to F in C major leads with E, a note of the
    // key, which keeps the freedom it has in C.)
    let secondary = to.root.transpose(-1);
    if to.root != tonic
        && !key.is_chord_of_key(from)
        && !key.contains(secondary)
        && leads_to(from, to.root).is_some()
    {
        return Some(secondary);
    }
    None
}

/// The seventh of `from` going to `to` in `key`, with `from` as the
/// seventh chord it works as: a chord named by its sixth may be a
/// seventh chord with its third lowest (F6 going to G is Dm7, ii⁶₅ in
/// C), and a diminished seventh chord's root is the note under where it
/// goes.
fn seventh_of(key: Scale, from: Chord, to: Chord) -> Option<(Chord, PitchClass)> {
    use ChordQuality::*;
    if from.root == to.root {
        return None;
    }
    let sixth = from.root.transpose(9);
    let reading = match from.quality {
        Diminished7 => {
            let roots = (0..4).map(|third| from.root.transpose(3 * third));
            let leading = key.root.transpose(-1);
            let root = roots
                .clone()
                .find(|root| root.transpose(1) == to.root)
                .or_else(|| roots.clone().find(|&root| root == leading))?;
            Chord::new(root, Diminished7)
        }
        Major6 if to.root == sixth.transpose(5) => Chord::new(sixth, Minor7),
        Minor6 if to.root == sixth.transpose(1) || to.root == sixth.transpose(5) => {
            Chord::new(sixth, HalfDiminished7)
        }
        // Only the plain seventh chords: a ninth, an eleventh or an
        // altered chord heard in a beat is most often a seventh chord or
        // a triad with passing notes.
        Dominant7 | Major7 | Minor7 | HalfDiminished7 => from,
        _ => return None,
    };
    let quality = reading.quality;
    let place = quality.tones().iter().position(|&tone| tone == 7)?;
    let seventh = reading
        .root
        .transpose(i32::from(quality.intervals()[place]));
    (reading.root != to.root).then_some((reading, seventh))
}

/// The leading tones and sevenths of `voices` (only those `checked`)
/// that do not go where they should, where one chord goes to another.
fn resolutions(
    voices: &[Voice],
    checked: &[usize],
    beats: &[(usize, usize)],
    key_at: &impl Fn(usize) -> Scale,
) -> Vec<Slip> {
    let mut slips = Vec::new();
    let spans = spans(voices, beats);
    let on_beat = |tick: usize| {
        beats
            .iter()
            .any(|&(start, _)| start * TICKS_PER_STEP == tick)
    };
    for (index, pair) in spans.windows(2).enumerate() {
        let (before, after) = (pair[0], pair[1]);
        let (Some(heard_from), Some(heard_to)) = (before.chord, after.chord) else {
            continue;
        };
        if before.end != after.start || heard_from == heard_to {
            continue;
        }
        // The chord just before the one the parts leave.
        let earlier = index
            .checked_sub(1)
            .map(|earlier| spans[earlier])
            .filter(|earlier| earlier.end == before.start)
            .and_then(|earlier| earlier.chord);
        let key = key_at(before.end - 1);
        let last = before.end * TICKS_PER_STEP - 1;
        let first = after.start * TICKS_PER_STEP;
        // The notes sounding at the end of the first chord and at the
        // start of the second, in every part, to find the bass and the
        // outer parts.
        let sounding = |tick: usize| -> Vec<Note> {
            voices
                .iter()
                .filter_map(|voice| voice.current(tick).map(|i| voice.notes[i].pitch))
                .collect()
        };
        let (end_notes, start_notes) = (sounding(last), sounding(first));
        // A bass passing by under the end of the first chord makes a
        // chord of the moment, not a change of harmony to check: G E D C
        // in the bass under a held C major passes through Em7 with D
        // lowest.
        let bass_passes = voices.iter().any(|voice| {
            voice.current(last).is_some_and(|index| {
                Some(&voice.notes[index].pitch) == end_notes.iter().min() && passes_by(voice, index)
            })
        });
        if bass_passes {
            continue;
        }
        let (Some(&low), Some(&high), Some(&next_low)) = (
            end_notes.iter().min(),
            end_notes.iter().max(),
            start_notes.iter().min(),
        ) else {
            continue;
        };
        // The chord the parts arrive on is what sounds just after the
        // move, when those notes make a chord, or else the chord heard
        // in that beat. The chord they leave must be both what sounds
        // just before the move and the chord heard in its beat: when the
        // two differ, passing notes make one of them, and nothing is
        // certain enough to flag.
        let at_move = |notes: &[Note]| match analyze(notes.iter().copied()) {
            Harmony::Chord { chord, .. } => Some(chord),
            _ => None,
        };
        let to = at_move(&start_notes).unwrap_or(heard_to);
        let from = at_move(&end_notes).unwrap_or(heard_from);
        if from == to {
            continue;
        }
        let agree = |a: Option<PitchClass>, b: Option<PitchClass>| a.filter(|_| a == b);
        // A six-four chord after V is no arrival: the parts move on
        // before the leading tone has anywhere to go.
        let six_four = to.tone_of(next_low.pitch_class()) == Some(5);
        let leading = agree(
            leading_tone(key, from, to),
            leading_tone(key, heard_from, to),
        )
        .filter(|_| !six_four);
        let seventh = seventh_of(key, from, to).filter(|&(_, tone)| {
            seventh_of(key, heard_from, to).is_some_and(|(_, heard)| heard == tone)
        });
        // The next chord has the note when either reading of it does.
        let to_has = |pitch_class: PitchClass| {
            to.pitch_set().contains(pitch_class) || heard_to.pitch_set().contains(pitch_class)
        };
        let resolution = Resolution {
            from: seventh.map_or(from, |(reading, _)| reading),
            from_bass: low.pitch_class(),
            to,
            to_bass: next_low.pitch_class(),
        };
        let leading_count = leading.map_or(0, |tone| {
            checked
                .iter()
                .filter(|&&v| {
                    voices[v]
                        .current(last)
                        .is_some_and(|i| voices[v].notes[i].pitch.pitch_class() == tone)
                })
                .count()
        });
        for &v in checked {
            let voice = &voices[v];
            let Some(index) = voice.current(last) else {
                continue;
            };
            let note = voice.notes[index];
            // A note that ended before the chord started is not in it.
            if note.end_tick() <= before.start * TICKS_PER_STEP {
                continue;
            }
            let pitch_class = note.pitch.pitch_class();
            let arrives_on_it = voice
                .current(first)
                .is_some_and(|i| voice.notes[i].pitch == note.pitch);
            let Some(next) = next_note(voice, index, after.end * TICKS_PER_STEP) else {
                continue;
            };
            let up = i32::from(next.pitch.midi()) - i32::from(note.pitch.midi());
            let outer = note.pitch == low || note.pitch == high;
            // The bass may take its note in another octave: B2 to C2.
            let in_bass = note.pitch == low;
            let goes_to = |semitones: i32| {
                next.pitch.pitch_class() == pitch_class.transpose(semitones)
                    && (up == semitones || in_bass)
            };
            // How the part came to the note, in semitones up.
            let came = previous_note(voice, index)
                .map(|before| i32::from(note.pitch.midi()) - i32::from(before.pitch.midi()));
            let by_step = |semitones: i32| (1..=2).contains(&semitones.abs());
            let slip = |kind, expected: Option<Note>| Slip {
                moves: vec![PartMove {
                    voice: v,
                    from: note,
                    to: next,
                }],
                chords: Some(resolution),
                expected,
                doubled: kind == SlipKind::LeadingTone && leading_count > 1,
                ..Slip::of(kind, key)
            };
            if let Some(tone) = leading
                && tone == pitch_class
                && outer
            {
                let common = arrives_on_it && to_has(pitch_class);
                // Down a scale from the tonic: a passing note.
                let passing_down =
                    came.is_some_and(|came| came < 0 && by_step(came)) && up < 0 && by_step(up);
                if !goes_to(1) && !common && !passing_down {
                    slips.push(slip(SlipKind::LeadingTone, note.pitch.transpose(1)));
                }
            }
            // A note added off the beat, shorter than a beat, while the
            // other parts hold theirs, is a note between chords, not a
            // chord's seventh: B F# G in the tenor under G going to C.
            let added = note.ticks() < LINE_BREAK
                && !on_beat(note.start_tick())
                && !voices.iter().enumerate().any(|(other, part)| {
                    other != v
                        && part
                            .notes
                            .iter()
                            .any(|played| played.start_tick() == note.start_tick())
                });
            // A passing chord between two forms of one chord, its seventh
            // a passing note on the way: Bb C D in the bass from Bb to
            // Bb with D lowest.
            let passing_through = earlier.is_some_and(|chord| chord == to || chord == heard_to)
                && came.is_some_and(|came| by_step(came) && came.signum() == up.signum())
                && by_step(up);
            if let Some((reading, tone)) = seventh
                && tone == pitch_class
                && !added
            {
                let falls = goes_to(-1) || goes_to(-2);
                let ok = if to_has(pitch_class) {
                    // The next chord has the note too: it stops being a
                    // dissonance there, and may stay.
                    true
                } else if arrives_on_it {
                    // Held over (a suspension): it falls a little later.
                    falls
                } else {
                    falls
                        || passes_by(voice, index)
                        || passing_through
                        || v43_to_i6(reading, low, next_low, to, up)
                        || (up == 1 && leads_on(to, next.pitch.pitch_class()))
                };
                if !ok {
                    slips.push(slip(
                        SlipKind::Seventh,
                        resolution_below(key, note.pitch, to),
                    ));
                }
            }
        }
    }
    slips
}

/// The note after `voice.notes[index]` with another pitch, starting
/// before `until` (in ticks), unless the part rests a beat or more on
/// the way.
fn next_note(voice: &Voice, index: usize, until: usize) -> Option<NoteEvent> {
    let mut previous = voice.notes[index];
    for &note in &voice.notes[index + 1..] {
        if note.start_tick() >= until || note.start_tick() >= previous.end_tick() + LINE_BREAK {
            return None;
        }
        if note.pitch != previous.pitch {
            return Some(note);
        }
        previous = note;
    }
    None
}

/// The note before `voice.notes[index]`, if the part played it without
/// resting a beat or more since.
fn previous_note(voice: &Voice, index: usize) -> Option<NoteEvent> {
    let note = voice.notes[index];
    let before = voice.notes[..index]
        .iter()
        .rev()
        .find(|before| before.pitch != note.pitch)?;
    (note.start_tick() < before.end_tick() + LINE_BREAK).then_some(*before)
}

/// Whether `pitch_class` is a note that leads on in `chord`: the third
/// of a major chord, the root of a diminished chord, or any note of a
/// dominant or a diminished seventh, chords that move on themselves. A
/// seventh raised a semitone to such a note (G to G# as Am7 goes to E7)
/// is chromatic voice leading: the line goes on rising, and the next
/// chord resolves it.
fn leads_on(chord: Chord, pitch_class: PitchClass) -> bool {
    use ChordQuality::*;
    match chord.quality {
        Diminished7 => chord.pitch_set().contains(pitch_class),
        quality if quality.is_dominant() => chord.pitch_set().contains(pitch_class),
        Diminished | HalfDiminished7 => chord.root == pitch_class,
        Major => chord.root.transpose(4) == pitch_class,
        _ => false,
    }
}

/// Whether `voice.notes[index]` is a passing or a neighbour note:
/// shorter than a beat, coming and going by step.
fn passes_by(voice: &Voice, index: usize) -> bool {
    let note = voice.notes[index];
    let step = |other: NoteEvent| (1..=2).contains(&other.pitch.midi().abs_diff(note.pitch.midi()));
    note.ticks() < LINE_BREAK
        && previous_note(voice, index).is_some_and(step)
        && next_note(voice, index, usize::MAX).is_some_and(step)
}

/// Whether a seventh rising a step (`up` semitones) is the one
/// exception: V⁴₃ going to I⁶, the seventh rising with the bass a tenth
/// below it, from the fifth of V to the third of I.
fn v43_to_i6(reading: Chord, bass: Note, next_bass: Note, to: Chord, up: i32) -> bool {
    let bass_up = i32::from(next_bass.midi()) - i32::from(bass.midi());
    reading.quality.is_dominant()
        && bass.pitch_class() == reading.root.transpose(7)
        && to.tone_of(next_bass.pitch_class()) == Some(3)
        && (1..=2).contains(&bass_up)
        && (1..=2).contains(&up)
}

/// Where a seventh `seventh` should go into chord `to`: the note a
/// semitone or a tone below it that is in the chord, or failing that in
/// the key.
fn resolution_below(key: Scale, seventh: Note, to: Chord) -> Option<Note> {
    let below = [1, 2].map(|step| seventh.transpose(-step));
    below
        .iter()
        .flatten()
        .find(|note| to.pitch_set().contains(note.pitch_class()))
        .or_else(|| {
            below
                .iter()
                .flatten()
                .find(|note| key.contains(note.pitch_class()))
        })
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harmony::ScaleKind;

    /// Notes from names such as "C4", each `length` steps long, one after
    /// the other: a part. "-" rests.
    fn part(text: &str, length: usize) -> Vec<NoteEvent> {
        text.split_whitespace()
            .enumerate()
            .filter(|(_, name)| *name != "-")
            .map(|(index, name)| NoteEvent::new(note(name), index * length, length))
            .collect()
    }

    fn note(name: &str) -> Note {
        let (pitch, rest) = crate::NoteName::read(name).expect("a note name");
        let octave: i32 = rest.parse().expect("an octave");
        Note::new((i32::from(pitch.pitch_class().semitone()) + 12 * (octave + 1)) as u8)
    }

    fn c_major(_: usize) -> Scale {
        Scale::new(PitchClass::C, ScaleKind::Major)
    }

    /// Checks four parts, soprano first, each a note per beat of 4/4.
    fn satb(parts: [&str; 4]) -> VoiceLeading {
        satb_in(parts, c_major)
    }

    fn satb_in(parts: [&str; 4], key: impl Fn(usize) -> Scale) -> VoiceLeading {
        let voices: Vec<Voice> = parts
            .iter()
            .enumerate()
            .flat_map(|(channel, text)| voices_of(channel, &part(text, 4)))
            .collect();
        let length = voices
            .iter()
            .flat_map(|voice| voice.notes.iter().map(NoteEvent::end))
            .max()
            .unwrap_or(0);
        let time = TimeSignature::default();
        check(voices, &beats(time, length), key)
    }

    fn kinds(check: &VoiceLeading) -> Vec<SlipKind> {
        check.slips.iter().map(|slip| slip.kind).collect()
    }

    #[test]
    fn a_textbook_cadence_has_nothing_to_avoid() {
        // I IV V7 I in C major, as harmony books write it.
        let check = satb(["C5 C5 B4 C5", "G4 A4 G4 G4", "E4 F4 F4 E4", "C3 F3 G3 C3"]);
        assert_eq!(kinds(&check), vec![]);
        assert_eq!(
            check.summary(),
            "Voice leading, 4 parts: nothing classical part writing avoids."
        );
    }

    #[test]
    fn root_position_triads_a_step_apart_move_in_parallel_fifths_and_octaves() {
        // C to D minor, with every part moving up a step.
        let check = satb(["G4 A4", "E4 F4", "C4 D4", "C3 D3"]);
        let found = kinds(&check);
        assert!(found.contains(&SlipKind::ParallelFifths), "{found:?}");
        assert!(found.contains(&SlipKind::ParallelOctaves), "{found:?}");
        let fifths = check
            .slips
            .iter()
            .find(|slip| slip.kind == SlipKind::ParallelFifths)
            .unwrap();
        let text = fifths.describe(|voice| ["Soprano", "Alto", "Tenor", "Bass"][voice].into());
        assert!(
            text.starts_with(
                "Parallel fifths: Soprano and Tenor go from G4 over C4 (a fifth) to A4 over \
                 D4 (a fifth), both up."
            ),
            "{text}"
        );
        assert_eq!(check.slips[0].step(), 4);
    }

    #[test]
    fn a_twelfth_is_a_fifth_and_contrary_motion_still_counts() {
        // Soprano G4 over bass C3 is a twelfth; to A4 over D3 both rise.
        let check = satb(["G4 A4", "E4 F4", "C4 A3", "C3 D3"]);
        assert!(kinds(&check).contains(&SlipKind::ParallelFifths));
        // C4 over F3 (a fifth) to G4 over C3 (a twelfth), moving apart.
        let check = satb(["C5 E5", "A4 G4", "C4 G4", "F3 C3"]);
        assert!(
            kinds(&check).contains(&SlipKind::ContraryFifths),
            "{:?}",
            kinds(&check)
        );
    }

    #[test]
    fn a_held_or_repeated_note_is_no_parallel() {
        // Soprano and bass both on C, then the bass moves: oblique motion.
        let check = satb(["C5 C5", "G4 A4", "E4 F4", "C3 F3"]);
        assert!(!kinds(&check).contains(&SlipKind::ParallelOctaves));
    }

    #[test]
    fn a_rest_of_a_beat_ends_a_line() {
        // The same fifths as above, but the tenor rests a beat between
        // its two notes.
        let voices: Vec<Voice> = [("G4 - A4", 4), ("C4 - D4", 4)]
            .iter()
            .enumerate()
            .flat_map(|(channel, (text, length))| voices_of(channel, &part(text, *length)))
            .collect();
        let check = check(voices, &beats(TimeSignature::default(), 12), c_major);
        assert!(!kinds(&check).contains(&SlipKind::ParallelFifths));
    }

    #[test]
    fn a_line_doubled_in_octaves_is_one_line() {
        // The bass doubled an octave up by the tenor, all the way.
        let check = satb(["E5 F5 D5 E5", "G4 A4 B4 G4", "C4 F4 G4 C4", "C3 F3 G3 C3"]);
        assert_eq!(check.doublings, vec![(2, 3)]);
        assert!(!kinds(&check).contains(&SlipKind::ParallelOctaves));
        assert!(check.summary().contains("one line doubled in octaves"));
    }

    #[test]
    fn the_leading_tone_rises_in_the_outer_parts() {
        // V to I with the leading tone B falling to G in the soprano.
        let check = satb(["B4 G4", "G4 E4", "D4 C4", "G3 C3"]);
        let slip = check
            .slips
            .iter()
            .find(|slip| slip.kind == SlipKind::LeadingTone)
            .expect("the leading tone falls");
        assert_eq!(slip.expected, Some(note("C5")));
        let text = slip.describe(|voice| ["Soprano", "Alto", "Tenor", "Bass"][voice].into());
        assert!(
            text.starts_with(
                "Leading tone not rising: Soprano has B4, the leading tone, in G (V) and \
                 leaps down to G4 in C (I) instead of rising a semitone to C5. The leading \
                 tone, a semitone under the note it leads to, pulls up to it, and in the top \
                 part or the bass, where the ear follows it, it should rise."
            ),
            "{text}"
        );
    }

    #[test]
    fn an_inner_leading_tone_is_freer() {
        // V to I, the alto's B dropping to G so C major is complete.
        let check = satb(["D5 C5", "B4 G4", "G4 E4", "G3 C3"]);
        assert!(
            !kinds(&check).contains(&SlipKind::LeadingTone),
            "{:?}",
            kinds(&check)
        );
        // In the bass it must rise: V⁶ to vi with B going down to A.
        let check = satb(["D5 C5", "G4 E4", "D4 C4", "B2 A2"]);
        assert!(
            kinds(&check).contains(&SlipKind::LeadingTone),
            "{:?}",
            kinds(&check)
        );
    }

    #[test]
    fn a_leading_tone_going_down_a_scale_is_a_passing_note() {
        // The soprano goes C B A over C, G and Am: the B passes by.
        let check = satb(["C5 B4 A4", "G4 G4 E4", "E4 D4 C4", "C3 G2 A2"]);
        assert!(
            !kinds(&check).contains(&SlipKind::LeadingTone),
            "{:?}",
            kinds(&check)
        );
    }

    #[test]
    fn the_leading_tone_rises_in_a_deceptive_cadence_and_in_minor() {
        // V to vi in C: B must still go to C.
        let check = satb(["B4 C5", "D4 C4", "G3 E3", "G2 A2"]);
        assert!(
            !kinds(&check).contains(&SlipKind::LeadingTone),
            "{:?}",
            kinds(&check)
        );
        let check = satb(["B4 A4", "D4 C4", "G3 E3", "G2 A2"]);
        assert!(
            kinds(&check).contains(&SlipKind::LeadingTone),
            "{:?}",
            kinds(&check)
        );
        // In A minor, E major's G# rises to A.
        let a_minor = |_: usize| Scale::new(PitchClass::A, ScaleKind::NaturalMinor);
        let check = satb_in(["G#4 E4", "E4 C4", "B3 A3", "E3 A2"], a_minor);
        let slip = check
            .slips
            .iter()
            .find(|slip| slip.kind == SlipKind::LeadingTone)
            .expect("G# falls");
        let text = slip.describe(|voice| ["Soprano", "Alto", "Tenor", "Bass"][voice].into());
        assert!(text.contains("G#4, the leading tone, in E (V)"), "{text}");
    }

    #[test]
    fn a_secondary_dominant_has_its_own_leading_tone() {
        // D7 to G in C major: F# rises to G. Here it falls to D.
        let check = satb(["F#5 D5", "C5 B4", "A4 G4", "D3 G2"]);
        let slip = check
            .slips
            .iter()
            .find(|slip| slip.kind == SlipKind::LeadingTone)
            .expect("F# leaps away");
        let text = slip.describe(|voice| ["Soprano", "Alto", "Tenor", "Bass"][voice].into());
        assert!(text.contains("in D7 (V7/V)"), "{text}");
        // A plain I going to IV is no secondary dominant: E may go anywhere.
        let check = satb(["E5 C5", "C5 A4", "G4 F4", "C3 F3"]);
        assert!(!kinds(&check).contains(&SlipKind::LeadingTone));
    }

    #[test]
    fn a_seventh_falls_a_step() {
        // V7 to I with the seventh F rising to G.
        let check = satb(["F5 G5", "B4 C5", "D4 E4", "G3 C3"]);
        let slip = check
            .slips
            .iter()
            .find(|slip| slip.kind == SlipKind::Seventh)
            .expect("the seventh rises");
        assert_eq!(slip.expected, Some(note("E5")));
        let text = slip.describe(|voice| ["Soprano", "Alto", "Tenor", "Bass"][voice].into());
        assert!(
            text.starts_with(
                "Seventh not falling: Soprano has F5, the seventh of G7 (V7), and rises to G5 \
                 in C (I) instead of falling a step to E5."
            ),
            "{text}"
        );
    }

    #[test]
    fn a_seventh_rising_in_v43_to_i6_is_allowed() {
        // G7 with D lowest to C with E lowest: the seventh F rises to G
        // a tenth above the bass going from D to E.
        let check = satb(["F5 G5", "B4 C5", "G4 G4", "D3 E3"]);
        assert!(
            !kinds(&check).contains(&SlipKind::Seventh),
            "{:?}",
            kinds(&check)
        );
        // From root position the same rise is wrong.
        let check = satb(["F5 G5", "B4 C5", "D4 E4", "G3 C3"]);
        assert!(kinds(&check).contains(&SlipKind::Seventh));
    }

    #[test]
    fn ii65_is_heard_as_a_seventh_chord() {
        // F A C D going to G is ii⁶₅ to V: its seventh C should fall to B.
        let check = satb(["D5 D5", "C5 G4", "A4 B4", "F3 G3"]);
        let slip = check
            .slips
            .iter()
            .find(|slip| slip.kind == SlipKind::Seventh)
            .expect("the seventh rises");
        let text = slip.describe(|voice| ["Soprano", "Alto", "Tenor", "Bass"][voice].into());
        assert!(text.contains("the seventh of Dm7/F (ii⁶₅)"), "{text}");
        let check = satb(["D5 D5", "C5 B4", "A4 G4", "F3 G3"]);
        assert!(
            !kinds(&check).contains(&SlipKind::Seventh),
            "{:?}",
            kinds(&check)
        );
    }

    #[test]
    fn a_seventh_rising_chromatically_leads_on() {
        // Am7 with G lowest to E7 with G# lowest in A minor: the seventh
        // G rises a semitone to G#, the new chord's leading tone.
        let a_minor = |_: usize| Scale::new(PitchClass::A, ScaleKind::NaturalMinor);
        let check = satb_in(["A4 B4", "E4 E4", "C4 D4", "G3 G#3"], a_minor);
        assert_eq!(kinds(&check), vec![]);
    }

    #[test]
    fn a_passing_chord_may_lead_its_seventh_on() {
        // C, then the bass passing through D to C with E lowest: the
        // chord over D is Em7, its seventh a passing note on the way.
        let check = satb(["E5 E5 E5", "G4 G4 G4", "C4 B3 C4", "C3 D3 E3"]);
        assert_eq!(kinds(&check), vec![]);
    }

    #[test]
    fn a_note_added_off_the_beat_is_no_seventh() {
        // C going to F, the tenor touching B3 for the second half of the
        // beat before C4: a note between the chords.
        let parts = [
            part("E5 F5", 4),
            part("G4 A4", 4),
            vec![
                NoteEvent::new(note("E4"), 0, 2),
                NoteEvent::new(note("B3"), 2, 2),
                NoteEvent::new(note("C4"), 4, 4),
            ],
            part("C3 F3", 4),
        ];
        let voices: Vec<Voice> = parts
            .iter()
            .enumerate()
            .flat_map(|(channel, notes)| voices_of(channel, notes))
            .collect();
        let check = check(voices, &beats(TimeSignature::default(), 8), c_major);
        assert!(
            !kinds(&check).contains(&SlipKind::Seventh),
            "{:?}",
            kinds(&check)
        );
        // Struck with the chord, B is Cmaj7's seventh, and rises.
        let check = satb(["E5 F5", "G4 A4", "B3 C4", "C3 F3"]);
        assert!(
            kinds(&check).contains(&SlipKind::Seventh),
            "{:?}",
            kinds(&check)
        );
    }

    #[test]
    fn the_bass_may_take_its_note_in_another_octave() {
        // G with B lowest to C: the bass leaps down a seventh from B2 to
        // C2, the leading tone going to the tonic an octave lower.
        let check = satb(["D5 E5", "G4 G4", "D4 C4", "B2 C2"]);
        assert_eq!(kinds(&check), vec![]);
        // Going to A2 instead, it leaves the leading tone hanging.
        let check = satb(["D5 E5", "G4 G4", "D4 C4", "B2 A2"]);
        assert!(
            kinds(&check).contains(&SlipKind::LeadingTone),
            "{:?}",
            kinds(&check)
        );
    }

    #[test]
    fn a_six_four_chord_is_no_arrival() {
        // G to C with G lowest: the parts move on before the leading
        // tone has anywhere to go.
        let check = satb(["B4 G4", "G4 E4", "D4 C4", "G2 G2"]);
        assert!(
            !kinds(&check).contains(&SlipKind::LeadingTone),
            "{:?}",
            kinds(&check)
        );
    }

    /// Dm7 for two beats, then G for two, with the alto holding the
    /// seventh C an eighth into G and then going to `then`.
    fn held_seventh(then: &str) -> VoiceLeading {
        let parts = [
            vec![
                NoteEvent::new(note("F5"), 0, 8),
                NoteEvent::new(note("D5"), 8, 8),
            ],
            vec![
                NoteEvent::new(note("C5"), 0, 10),
                NoteEvent::new(note(then), 10, 6),
            ],
            vec![
                NoteEvent::new(note("A4"), 0, 8),
                NoteEvent::new(note("B4"), 8, 8),
            ],
            vec![
                NoteEvent::new(note("D3"), 0, 8),
                NoteEvent::new(note("G3"), 8, 8),
            ],
        ];
        let voices = parts
            .iter()
            .enumerate()
            .flat_map(|(channel, notes)| voices_of(channel, notes))
            .collect();
        check(voices, &beats(TimeSignature::default(), 16), c_major)
    }

    #[test]
    fn a_held_seventh_may_fall_a_little_later() {
        // A suspension: the seventh C held into G, then falling to B.
        assert!(!kinds(&held_seventh("B4")).contains(&SlipKind::Seventh));
        // Held, then rising to D, it is still a seventh not falling.
        assert!(kinds(&held_seventh("D5")).contains(&SlipKind::Seventh));
    }

    #[test]
    fn a_track_of_chords_is_split_into_parts() {
        // Three-note chords in one track: C E G, then D F A.
        let mut notes = part("C4 D4", 4);
        notes.extend(part("E4 F4", 4));
        notes.extend(part("G4 A4", 4));
        let voices = voices_of(0, &notes);
        assert_eq!(voices.len(), 3);
        let pitches = |voice: &Voice| {
            voice
                .notes
                .iter()
                .map(|note| key_name(note.pitch))
                .collect::<Vec<_>>()
        };
        assert_eq!(pitches(&voices[0]), ["G4", "A4"]);
        assert_eq!(pitches(&voices[2]), ["C4", "D4"]);
        assert_eq!(voices[0].name("Piano"), "Piano (top voice)");
        assert_eq!(voices[1].name("Piano"), "Piano (voice 2 of 3)");
        assert_eq!(voices[2].name("Piano"), "Piano (lowest voice)");
        // A chord of four notes after one of three gets a new part for
        // the note that is furthest from all.
        let mut notes = part("C4 B3", 4);
        notes.extend(part("E4 F4", 4));
        notes.extend(part("G4 G4", 4));
        notes.push(NoteEvent::new(note("D4"), 4, 4));
        let voices = voices_of(0, &notes);
        assert_eq!(voices.len(), 4);
        assert!(voices.iter().any(|voice| pitches(voice) == ["D4"]));
    }

    #[test]
    fn a_tune_played_legato_is_one_part() {
        // Each note overlaps the next by a few ticks.
        let notes: Vec<NoteEvent> = ["C4", "E4", "G4", "C5"]
            .iter()
            .enumerate()
            .map(|(index, name)| NoteEvent::exact(note(name), index * 96, 100))
            .collect();
        assert_eq!(voices_of(0, &notes).len(), 1);
    }

    fn key_name(note: Note) -> String {
        c_major(0).note_name(note)
    }
}
