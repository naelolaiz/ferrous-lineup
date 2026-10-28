//! The lead sheet bar by bar: what each bar holds, before anything is
//! placed on a page.

use std::ops::Range;

use super::melody::{MelodyNote, melody};
use super::rhythm::{self, Base, Value};
use super::spelling::{self, BarAccidentals, Clef, KeySignature};
use super::transpose::Transposition;
use crate::chord_sheet::{ChordMarker, HarmonyName};
use crate::meter::{Bar, TimeSignature};
use crate::pitch::Note;
use crate::playlist::MAX_SONG_STEPS;
use crate::project::Project;

/// The whole song, ready to lay out.
#[derive(Debug, Clone)]
pub struct Score {
    pub clef: Clef,
    pub bars: Vec<BarScore>,
    /// The tempo at the start, in quarter notes per minute.
    pub tempo: f32,
}

/// One bar.
#[derive(Debug, Clone)]
pub struct BarScore {
    pub bar: Bar,
    pub signature: KeySignature,
    /// The signature of the bar before, when the key changes here.
    pub key_change: Option<KeySignature>,
    /// Whether the time signature is written here: in the first bar and
    /// wherever it changes.
    pub time_change: bool,
    pub content: Content,
    /// The harmonies that start in this bar, at steps from its start.
    pub chords: Vec<(usize, HarmonyName)>,
    /// The name of the section that starts here, if one does.
    pub section: Option<String>,
    /// Whether a repeated section starts here.
    pub repeat_start: bool,
    /// How many times the section is played, when a repeated section
    /// ends with this bar.
    pub repeat_end: Option<usize>,
    /// Whether a section starts right after this bar, which a double bar
    /// line shows.
    pub double_bar: bool,
    /// Whether this is the song's last bar.
    pub last: bool,
}

/// What is written on the staff of a bar.
#[derive(Debug, Clone, PartialEq)]
pub enum Content {
    /// Notes and rests filling the bar, with the beams that join them.
    Notes {
        pieces: Vec<Piece>,
        /// Runs of eighths and sixteenths beamed together, as ranges of
        /// `pieces`.
        beams: Vec<Range<usize>>,
    },
    /// The tune rests the whole bar.
    WholeRest,
    /// No tune: one slash per beat, for the chords to be played.
    Slashes,
}

/// A note or a rest written as one value. A note too long for one
/// value is written as several pieces tied together.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    /// Steps from the start of the bar.
    pub from: usize,
    pub to: usize,
    pub value: Value,
    /// `None` for a rest.
    pub note: Option<Written>,
}

/// A note as written.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Written {
    /// Which note of the tune it is.
    pub index: usize,
    /// The line or space it sits on ([`spelling::staff_step`]).
    pub step: i32,
    /// The accidental printed before it, if any.
    pub accidental: Option<i8>,
    /// Whether it is tied to the next piece of the same note.
    pub tie: bool,
}

/// Where the song ends: after the last clip, section or chord, or the
/// last bar the band plays, on a bar line.
fn song_end(project: &Project) -> usize {
    let meter = project.meter();
    let mut end = project.song_length();
    if let Some(last) = project.playlist().chord_markers().last() {
        end = end.max(meter.round_up_to_bar(last.start + 1));
    }
    if let Some(band) = project.band() {
        end = end.max(meter.round_up_to_bar(band.end()));
    }
    end.min(MAX_SONG_STEPS)
}

/// Reads the song: the melody from `channel` (none: chords only) and
/// everything else from the playlist, written for the instruments of
/// `transposition`.
pub fn build(
    project: &Project,
    channel: Option<usize>,
    clef: Option<Clef>,
    transposition: Transposition,
) -> Score {
    let end = song_end(project);
    let meter = project.meter();
    let keys = project.key_map();
    // The key each step is written in, and each harmony as written: for
    // a transposing instrument, moved from the key the song sounds in.
    let key_at = |step: usize| transposition.key(keys.key_at(step));
    let harmony_of =
        |marker: ChordMarker| transposition.harmony(marker.harmony, keys.key_at(marker.start));
    let heard = channel.map_or_else(Vec::new, |channel| melody(project, channel, end));
    let pitches: Vec<Note> = heard.iter().map(|note| note.pitch).collect();
    let clef = clef.unwrap_or_else(|| {
        if transposition != Transposition::Concert {
            return Clef::Treble;
        }
        let mut pitches = pitches.clone();
        pitches.sort();
        pitches
            .get(pitches.len() / 2)
            .map_or(Clef::Treble, |&median| Clef::for_median(median))
    });
    // The tune as the part writes it, higher than it sounds for a
    // transposing instrument.
    let shift = transposition.shift(&pitches, clef);
    let tune: Vec<MelodyNote> = heard
        .iter()
        .map(|&note| MelodyNote {
            pitch: note.pitch.transpose(shift).unwrap_or(note.pitch),
            ..note
        })
        .collect();

    // How each note of the tune is spelled: at concert pitch, from the
    // key and the chord where it starts and the note after it, and then
    // moved to the part's key, so that its intervals keep their names.
    let names: Vec<(i32, crate::pitch::NoteName)> = heard
        .iter()
        .enumerate()
        .map(|(index, note)| {
            let chord = match project.chord_at(note.start).map(|marker| marker.harmony) {
                Some(HarmonyName::Chord(symbol)) => Some(symbol),
                _ => None,
            };
            let next = heard.get(index + 1).map(|next| next.pitch);
            let key = keys.key_at(note.start);
            let name = transposition.name(spelling::spell(key, note.pitch, chord, next), key);
            (spelling::staff_step(tune[index].pitch, name), name)
        })
        .collect();

    let first_note = tune.first().map_or(usize::MAX, |note| note.start);
    let last_note = tune.last().map_or(0, MelodyNote::end);
    let markers = project.playlist().chord_markers();
    let mut bars: Vec<BarScore> = Vec::new();
    let mut next_note = 0;
    for bar in meter.bars().take_while(|bar| bar.start < end) {
        let signature = KeySignature::of(key_at(bar.start));
        let previous = bars.last();
        let key_change = previous
            .filter(|previous| previous.signature != signature)
            .map(|previous| previous.signature);
        let time_change = previous.is_none_or(|previous| previous.bar.time != bar.time);

        // The notes that sound in this bar.
        while tune
            .get(next_note)
            .is_some_and(|note| note.end() <= bar.start)
        {
            next_note += 1;
        }
        let notes: Vec<usize> = (next_note..tune.len())
            .take_while(|&index| tune[index].start < bar.end())
            .collect();

        let chords: Vec<(usize, HarmonyName)> = markers
            .iter()
            .filter(|marker| (bar.start..bar.end()).contains(&marker.start))
            .map(|marker| (marker.start - bar.start, harmony_of(*marker)))
            .collect();
        let sounding = project
            .chord_at(bar.start)
            .map(|marker| marker.harmony)
            .into_iter()
            .chain(chords.iter().map(|(_, harmony)| *harmony))
            .any(|harmony| harmony != HarmonyName::NoChord);

        let content = if !notes.is_empty() {
            notes_of_bar(&bar, &tune, &names, &notes, signature)
        } else if sounding && (bar.end() <= first_note || bar.start >= last_note) {
            // Before the tune starts or after it ends, as in an intro,
            // the slashes say "play the chords here".
            Content::Slashes
        } else {
            Content::WholeRest
        };

        bars.push(BarScore {
            bar,
            signature,
            key_change,
            time_change,
            content,
            chords,
            section: None,
            repeat_start: false,
            repeat_end: None,
            double_bar: false,
            last: false,
        });
    }

    mark_sections(project, &mut bars);
    if let Some(last) = bars.last_mut() {
        last.last = true;
    }
    Score {
        clef,
        bars,
        tempo: project.tempo_map().tempo_at(0).bpm(),
    }
}

/// The notes and rests of a bar, from the notes of the tune that sound
/// in it.
fn notes_of_bar(
    bar: &Bar,
    tune: &[MelodyNote],
    names: &[(i32, crate::pitch::NoteName)],
    notes: &[usize],
    signature: KeySignature,
) -> Content {
    let time = bar.time;
    let mut accidentals = BarAccidentals::new(signature);
    let mut pieces = Vec::new();
    let mut at = 0;
    for &index in notes {
        let note = tune[index];
        let from = note.start.max(bar.start) - bar.start;
        let to = note.end().min(bar.end()) - bar.start;
        let tied_in = note.start < bar.start;
        let tied_out = note.end() > bar.end();
        rests(time, bar.length, at, from, &mut pieces);
        let written = rhythm::note_pieces(time, bar.length, from, to);
        let count = written.len();
        let (step, name) = names[index];
        for (number, (a, b)) in written.into_iter().enumerate() {
            let accidental = accidentals.print(step, name, tied_in || number > 0);
            pieces.push(Piece {
                from: a,
                to: b,
                value: value_of(b - a),
                note: Some(Written {
                    index,
                    step,
                    accidental,
                    tie: number + 1 < count || tied_out,
                }),
            });
        }
        at = to;
    }
    rests(time, bar.length, at, bar.length, &mut pieces);
    let beams = beams_of(time, &pieces);
    Content::Notes { pieces, beams }
}

fn rests(time: TimeSignature, bar: usize, from: usize, to: usize, pieces: &mut Vec<Piece>) {
    for (a, b) in rhythm::rest_pieces(time, bar, from, to) {
        pieces.push(Piece {
            from: a,
            to: b,
            value: value_of(b - a),
            note: None,
        });
    }
}

/// The written value of a piece. The rhythm module only makes pieces
/// one value can show; a sixteenth stands in should that ever fail.
fn value_of(steps: usize) -> Value {
    Value::of(steps).unwrap_or(Value {
        base: Base::Sixteenth,
        dotted: false,
    })
}

/// Runs of two or more eighths and sixteenths inside the same beam
/// group. A rest, or a longer note, ends a run.
fn beams_of(time: TimeSignature, pieces: &[Piece]) -> Vec<Range<usize>> {
    let groups = rhythm::beam_groups(time);
    let group = |offset: usize| groups.partition_point(|&start| start <= offset);
    let mut beams = Vec::new();
    let mut run: Option<Range<usize>> = None;
    for (index, piece) in pieces.iter().enumerate() {
        let beamable = piece.note.is_some()
            && piece.value.beams() > 0
            && group(piece.from) == group(piece.to - 1);
        let joins = run
            .as_ref()
            .is_some_and(|run| beamable && group(pieces[run.start].from) == group(piece.from));
        if joins {
            if let Some(run) = &mut run {
                run.end = index + 1;
            }
        } else {
            if let Some(done) = run.take().filter(|run| run.len() >= 2) {
                beams.push(done);
            }
            if beamable {
                run = Some(index..index + 1);
            }
        }
    }
    if let Some(done) = run.filter(|run| run.len() >= 2) {
        beams.push(done);
    }
    beams
}

/// Marks where sections start and end: their names, repeat signs and
/// the double bar lines before them.
fn mark_sections(project: &Project, bars: &mut [BarScore]) {
    let bar_of = |step: usize, bars: &[BarScore]| {
        bars.iter()
            .position(|bar| (bar.bar.start..bar.bar.end()).contains(&step))
    };
    for section in project.playlist().sections() {
        let Some(first) = bar_of(section.start, bars) else {
            continue;
        };
        bars[first].section = Some(crate::i18n::tr_name(&section.name).into_owned());
        if first > 0 {
            bars[first - 1].double_bar = true;
        }
        if section.repeats >= 2 {
            bars[first].repeat_start = true;
            if let Some(last) = bar_of(section.end().saturating_sub(1), bars) {
                bars[last].repeat_end = Some(section.repeats);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chord_sheet::ChordMarker;
    use crate::harmony::{Scale, ScaleKind};
    use crate::key_map::KeyMarker;
    use crate::pattern::NoteEvent;
    use crate::pitch::PitchClass;
    use crate::playlist::{Clip, Section};

    /// A song of `bars` bars of 4/4 whose tune is `notes`.
    fn song(bars: usize, notes: &[(u8, usize, usize)]) -> (Project, usize) {
        let mut project = Project::empty();
        let channel = crate::library::apply::synth_named(&mut project, "Lead").unwrap();
        let pattern = project.pattern_mut(0).unwrap();
        pattern.set_length(16 * bars);
        pattern.set_notes(
            channel,
            notes
                .iter()
                .map(|&(midi, start, length)| NoteEvent::new(Note::new(midi), start, length))
                .collect::<Vec<_>>(),
        );
        project.add_clip(Clip::new(0, 0, 0));
        (project, channel)
    }

    fn pieces(bar: &BarScore) -> Vec<(usize, usize, Option<i32>)> {
        match &bar.content {
            Content::Notes { pieces, .. } => pieces
                .iter()
                .map(|piece| (piece.from, piece.to, piece.note.map(|note| note.step)))
                .collect(),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn a_note_over_the_bar_line_is_tied() {
        // C5 from beat 4 of bar 1 to beat 2 of bar 2.
        let (project, channel) = song(2, &[(72, 12, 8)]);
        let score = build(&project, Some(channel), None, Transposition::Concert);
        assert_eq!(score.bars.len(), 2);
        assert_eq!(
            pieces(&score.bars[0]),
            [(0, 8, None), (8, 12, None), (12, 16, Some(35))]
        );
        assert_eq!(
            pieces(&score.bars[1]),
            [(0, 4, Some(35)), (4, 8, None), (8, 16, None)]
        );
        let Content::Notes { pieces, .. } = &score.bars[0].content else {
            unreachable!()
        };
        assert!(pieces[2].note.unwrap().tie);
        assert_eq!(score.clef, Clef::Treble);
    }

    #[test]
    fn accidentals_are_printed_once_a_bar() {
        // F# F# G F# in C major, then F# again in the next bar.
        let (project, channel) = song(
            2,
            &[(66, 0, 4), (66, 4, 4), (67, 8, 4), (66, 12, 4), (66, 16, 4)],
        );
        let score = build(&project, Some(channel), None, Transposition::Concert);
        let printed = |bar: &BarScore| -> Vec<Option<i8>> {
            let Content::Notes { pieces, .. } = &bar.content else {
                unreachable!()
            };
            pieces
                .iter()
                .filter_map(|piece| piece.note.map(|note| note.accidental))
                .collect()
        };
        assert_eq!(printed(&score.bars[0]), [Some(1), None, None, None]);
        assert_eq!(printed(&score.bars[1])[0], Some(1));
    }

    #[test]
    fn eighths_are_beamed_in_beats() {
        let notes: Vec<(u8, usize, usize)> = (0..8).map(|i| (60 + i as u8, i * 2, 2)).collect();
        let (project, channel) = song(1, &notes);
        let score = build(&project, Some(channel), None, Transposition::Concert);
        let Content::Notes { beams, .. } = &score.bars[0].content else {
            unreachable!()
        };
        assert_eq!(beams, &[0..2, 2..4, 4..6, 6..8]);
    }

    #[test]
    fn low_tunes_are_written_in_the_bass_clef() {
        let (project, channel) = song(1, &[(43, 0, 8), (48, 8, 8)]);
        assert_eq!(
            build(&project, Some(channel), None, Transposition::Concert).clef,
            Clef::Bass
        );
        assert_eq!(
            build(
                &project,
                Some(channel),
                Some(Clef::Treble),
                Transposition::Concert
            )
            .clef,
            Clef::Treble
        );
    }

    #[test]
    fn bars_without_a_tune_show_slashes_or_rests() {
        // Bar 1 has chords and no tune yet: slashes. Bar 2 has the tune.
        // Bar 3 rests in the middle of the tune; bar 4 ends it.
        let (mut project, channel) = song(4, &[(72, 16, 16), (72, 48, 16)]);
        let c = HarmonyName::parse("C").unwrap();
        assert!(project.set_chords(0, 64, &[ChordMarker::new(0, c)]));
        let score = build(&project, Some(channel), None, Transposition::Concert);
        assert_eq!(score.bars[0].content, Content::Slashes);
        assert_eq!(score.bars[2].content, Content::WholeRest);
        assert_eq!(score.bars[0].chords, [(0, c)]);
        // With no tune at all, every bar with a chord gets slashes.
        let chords_only = build(&project, None, None, Transposition::Concert);
        assert!(
            chords_only
                .bars
                .iter()
                .all(|bar| bar.content == Content::Slashes)
        );
    }

    #[test]
    fn sections_bring_rehearsal_marks_and_repeats() {
        let (mut project, channel) = song(4, &[(72, 0, 64)]);
        project.add_section(Section::new("A", 0, 32)).unwrap();
        let mut b = Section::new("B", 32, 32);
        b.repeats = 3;
        project.add_section(b).unwrap();
        let score = build(&project, Some(channel), None, Transposition::Concert);
        assert_eq!(score.bars[0].section.as_deref(), Some("A"));
        assert_eq!(score.bars[2].section.as_deref(), Some("B"));
        assert!(score.bars[1].double_bar);
        assert!(score.bars[2].repeat_start);
        assert_eq!(score.bars[3].repeat_end, Some(3));
        assert!(score.bars[3].last);
    }

    #[test]
    fn a_trumpet_part_is_written_a_whole_tone_higher() {
        // C5 B4 C5 over C, G7 and C in C major: for a trumpet in Bb,
        // D5 C#5 D5 over D, A7 and D in D major.
        let (mut project, channel) = song(2, &[(72, 0, 8), (71, 8, 8), (72, 16, 16)]);
        let chord = |start, name| ChordMarker::new(start, HarmonyName::parse(name).unwrap());
        assert!(project.set_chords(0, 32, &[chord(0, "C"), chord(8, "G7"), chord(16, "C")]));
        let score = build(&project, Some(channel), None, Transposition::BFlat);
        assert_eq!(score.clef, Clef::Treble);
        assert_eq!(score.bars[0].signature, KeySignature(2));
        let names: Vec<String> = score.bars[0]
            .chords
            .iter()
            .map(|(_, harmony)| harmony.to_string())
            .collect();
        assert_eq!(names, ["D", "A7"]);
        // D5 sits a step above C5 (35), and C#5 on C5's space with no
        // sharp printed: the key signature has it.
        assert_eq!(
            pieces(&score.bars[0]),
            [(0, 8, Some(36)), (8, 16, Some(35))]
        );
        let Content::Notes { pieces, .. } = &score.bars[0].content else {
            unreachable!()
        };
        assert!(
            pieces
                .iter()
                .all(|piece| piece.note.unwrap().accidental.is_none())
        );
    }

    #[test]
    fn a_part_keeps_the_intervals_of_the_tune() {
        // E5 D#5 E5 in A minor: the D# leans on E, a semitone below it.
        // For a trumpet in B minor it is E# leaning on F#, with a sharp,
        // never F natural going to F#.
        let (mut project, channel) = song(1, &[(76, 0, 4), (75, 4, 4), (76, 8, 8)]);
        project.scale = Scale::new(PitchClass::A, ScaleKind::NaturalMinor);
        let score = build(&project, Some(channel), None, Transposition::BFlat);
        assert_eq!(score.bars[0].signature, KeySignature(2));
        // An octave lower than it would sound, nearer the middle of the
        // staff: F#4 sits on step 31, E#4 on E4's line (30).
        assert_eq!(
            pieces(&score.bars[0]),
            [(0, 4, Some(31)), (4, 8, Some(30)), (8, 16, Some(31))]
        );
        let Content::Notes { pieces, .. } = &score.bars[0].content else {
            unreachable!()
        };
        let printed: Vec<Option<i8>> = pieces
            .iter()
            .map(|piece| piece.note.unwrap().accidental)
            .collect();
        assert_eq!(printed, [None, Some(1), None]);
    }

    #[test]
    fn a_key_change_is_written_in_the_parts_keys() {
        // C major, then E major from bar 2: for an alto saxophone, A
        // major (three sharps) and then Db major (five flats), with
        // the E major V7 (B7) written as Ab7 and its G# as F.
        let (mut project, channel) = song(2, &[(72, 0, 16), (68, 16, 16)]);
        project.set_key_marker(KeyMarker::new(
            16,
            Scale::new(PitchClass::E, ScaleKind::Major),
        ));
        let b7 = HarmonyName::parse("B7").unwrap();
        assert!(project.set_chords(16, 32, &[ChordMarker::new(16, b7)]));
        let score = build(&project, Some(channel), None, Transposition::EFlat);
        assert_eq!(score.bars[0].signature, KeySignature(3));
        assert_eq!(score.bars[1].signature, KeySignature(-5));
        assert_eq!(score.bars[1].key_change, Some(KeySignature(3)));
        assert_eq!(score.bars[1].chords[0].1.to_string(), "Ab7");
        // C5 and G#4 are written a minor third lower, A4 and F4 (staff
        // steps 33 and 31): the octave nearest the middle of the staff.
        assert_eq!(pieces(&score.bars[0]), [(0, 16, Some(33))]);
        assert_eq!(pieces(&score.bars[1]), [(0, 16, Some(31))]);
    }

    #[test]
    fn parts_are_in_the_treble_clef_unless_told() {
        // A bass line: concert pitch takes the bass clef, a part for a
        // B-flat instrument the treble clef, high enough to read on it.
        let (project, channel) = song(1, &[(40, 0, 8), (43, 8, 8)]);
        let concert = build(&project, Some(channel), None, Transposition::Concert);
        assert_eq!(concert.clef, Clef::Bass);
        let part = build(&project, Some(channel), None, Transposition::BFlat);
        assert_eq!(part.clef, Clef::Treble);
        let told = build(
            &project,
            Some(channel),
            Some(Clef::Bass),
            Transposition::BFlat,
        );
        assert_eq!(told.clef, Clef::Bass);
        // On the bass staff, F#2 and A2 (a whole tone up) are steps 17 and 19.
        assert_eq!(pieces(&told.bars[0]), [(0, 8, Some(17)), (8, 16, Some(19))]);
    }

    #[test]
    fn key_changes_are_marked() {
        let (mut project, channel) = song(2, &[(72, 0, 32)]);
        project.set_key_marker(KeyMarker::new(
            16,
            Scale::new(PitchClass::D, ScaleKind::Major),
        ));
        let score = build(&project, Some(channel), None, Transposition::Concert);
        assert_eq!(score.bars[0].key_change, None);
        assert_eq!(score.bars[1].key_change, Some(KeySignature(0)));
        assert_eq!(score.bars[1].signature, KeySignature(2));
        assert!(score.bars[0].time_change);
        assert!(!score.bars[1].time_change);
    }
}
