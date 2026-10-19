//! Swing: eighth notes played long-short.
//!
//! Jazz, blues and shuffle players do not split a beat into two equal
//! eighth notes. The first one is longer and the second one shorter and
//! later, "doo-ba doo-ba" instead of "da-da da-da". Sheet music still
//! writes plain eighths and puts "Swing" over the first bar: how long
//! the long one is, is left to the players.
//!
//! A project has one swing amount ([`Project::swing`]), from 0 (the
//! eighths even, "straight") to [`MAX_SWING`] (the first two thirds and
//! the last third of a triplet, 2:1). Halfway, 50, is a light swing of
//! 7:5. The notes stay where they are written; only what is played moves
//! (see [`Project::played_notes`]), so the piano roll and the step
//! buttons keep their grid, and the amount can change at any time.

use std::borrow::Cow;

use crate::Project;
use crate::meter::TimeSignature;
use crate::pattern::{NoteEvent, TICKS_PER_STEP};

/// The most swing: the eighths of each beat as the first two thirds and
/// the last third of a triplet.
pub const MAX_SWING: u8 = 100;

/// A quarter note, in ticks.
const BEAT: usize = 4 * TICKS_PER_STEP;

/// The tick of a beat that its second eighth note starts on, straight.
const OFFBEAT: usize = BEAT / 2;

/// How much later the second eighth starts at full swing: on the third
/// note of a triplet, two thirds of the way through the beat.
const FULL_DELAY: usize = BEAT * 2 / 3 - OFFBEAT;

/// How close to the second eighth a note must start to be played as it:
/// less than a sixty-fourth note (6 ticks) either way. The strings of a
/// strummed chord follow each other a tick apart, and a note played by
/// hand is rarely exactly on time.
const NEAR: usize = BEAT / 16;

/// Whether eighth notes swing in `time`: in time signatures counted in
/// quarter notes or longer ones, such as 4/4, 3/4 and 2/2. Where the
/// eighth note is the beat (5/8, 7/8), or a beat is already three of
/// them (6/8, 12/8), there are no pairs of eighths to swing.
pub fn swings_in(time: TimeSignature) -> bool {
    time.unit <= 4
}

/// How many ticks late the second eighth of a beat starts with `swing`.
pub fn delay(swing: u8) -> usize {
    let swing = usize::from(swing.min(MAX_SWING));
    (FULL_DELAY * swing + usize::from(MAX_SWING) / 2) / usize::from(MAX_SWING)
}

/// `notes` as they are played with `swing`. A note that starts on the
/// second eighth of a beat (or very near it) starts later, and a note
/// that ends there ends later, so the eighth before it grows as it
/// shrinks. Beats are counted from the start of the pattern. Every other
/// note stays as it is, sixteenths and dotted rhythms included.
///
/// Drum `hits` sound to their end however long they are held, so only
/// their starts move.
pub fn swung(notes: &[NoteEvent], swing: u8, hits: bool) -> Vec<NoteEvent> {
    let late = delay(swing);
    let played = |tick: usize| match tick % BEAT {
        offset if offset.abs_diff(OFFBEAT) < NEAR => tick + late,
        _ => tick,
    };
    notes
        .iter()
        .map(|note| {
            let start = played(note.start_tick());
            let end = match hits {
                true => note.end_tick(),
                false => played(note.end_tick()),
            };
            NoteEvent {
                velocity: note.velocity,
                ..NoteEvent::exact(note.pitch, start, end.saturating_sub(start))
            }
        })
        .collect()
}

impl Project {
    /// How much pattern `pattern` swings: the song's [`swing`](Self::swing),
    /// or for a band's pattern in a section played in another style, that
    /// style's (see [`crate::band::sections`]); 0 if it is in a time
    /// signature whose eighths do not swing (see [`swings_in`]).
    pub fn pattern_swing(&self, pattern: usize) -> u8 {
        let swing = self.section_swing(pattern).unwrap_or(self.swing);
        if swing > 0 && swings_in(self.pattern_time(pattern)) {
            swing
        } else {
            0
        }
    }

    /// The notes channel `channel` plays in pattern `pattern`: the ones
    /// written there, with the song's swing. Without swing, that is the
    /// written notes themselves, borrowed rather than copied.
    pub fn played_notes(&self, pattern: usize, channel: usize) -> Cow<'_, [NoteEvent]> {
        let Some(written) = self.pattern(pattern).map(|p| p.notes(channel)) else {
            return Cow::Borrowed(&[]);
        };
        match self.pattern_swing(pattern) {
            0 => Cow::Borrowed(written),
            swing => Cow::Owned(swung(written, swing, self.plays_hits(channel))),
        }
    }

    /// Whether channel `channel` plays drum hits (see [`swung`]).
    pub fn plays_hits(&self, channel: usize) -> bool {
        self.channels
            .get(channel)
            .is_some_and(|channel| !channel.is_pitched())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Channel, Clip, Note, TimeMarker};

    fn c4() -> Note {
        Note::new(60)
    }

    /// Eighth notes through a bar of 4/4.
    fn eighths() -> Vec<NoteEvent> {
        (0..8).map(|i| NoteEvent::new(c4(), i * 2, 2)).collect()
    }

    /// Where each note starts and ends, in ticks.
    fn ticks(notes: &[NoteEvent]) -> Vec<(usize, usize)> {
        notes
            .iter()
            .map(|note| (note.start_tick(), note.end_tick()))
            .collect()
    }

    #[test]
    fn full_swing_plays_the_eighths_as_triplets() {
        assert_eq!(delay(0), 0);
        assert_eq!(delay(50), 8);
        assert_eq!(delay(MAX_SWING), 16);
        assert_eq!(delay(200), 16, "no more than a triplet");
        let played = ticks(&swung(&eighths(), MAX_SWING, false));
        // Each beat is 96 ticks: a long eighth of 64 and a short one of 32.
        assert_eq!(&played[..4], &[(0, 64), (64, 96), (96, 160), (160, 192)]);
    }

    #[test]
    fn half_swing_is_seven_to_five() {
        let played = ticks(&swung(&eighths(), 50, false));
        assert_eq!(&played[..2], &[(0, 56), (56, 96)]);
    }

    #[test]
    fn notes_off_the_eighths_stay() {
        let notes = [
            // A quarter note, a sixteenth before the beat, a dotted pair.
            NoteEvent::new(c4(), 0, 4),
            NoteEvent::new(c4(), 7, 1),
            NoteEvent::new(c4(), 8, 3),
            NoteEvent::new(c4(), 11, 1),
        ];
        assert_eq!(swung(&notes, MAX_SWING, false), notes);
    }

    #[test]
    fn a_strum_on_the_offbeat_moves_as_a_whole() {
        // Three strings struck a tick apart, from the second eighth.
        let strum: Vec<NoteEvent> = (0..3)
            .map(|string| NoteEvent::exact(Note::new(60 + string * 4), 48 + string as usize, 48))
            .collect();
        let starts: Vec<usize> = swung(&strum, MAX_SWING, false)
            .iter()
            .map(NoteEvent::start_tick)
            .collect();
        assert_eq!(starts, [64, 65, 66]);
        // A note a sixty-fourth or more from the offbeat is not on it.
        let off = [NoteEvent::exact(c4(), 40, 2)];
        assert_eq!(swung(&off, MAX_SWING, false), off);
    }

    #[test]
    fn a_swung_note_keeps_its_velocity_and_a_step_to_start_in() {
        let mut note = NoteEvent::new(c4(), 2, 1);
        note.velocity = 70;
        let played = swung(&[note], MAX_SWING, false)[0];
        assert_eq!((played.start, played.nudge, played.length), (2, 16, 1));
        assert_eq!(played.velocity, 70);
        assert_eq!(played.ticks(), 8);
    }

    #[test]
    fn a_note_held_into_the_offbeat_is_held_longer() {
        // A sixteenth on the second step of the beat, then the offbeat.
        let notes = [NoteEvent::new(c4(), 1, 1), NoteEvent::new(c4(), 2, 2)];
        let played = ticks(&swung(&notes, MAX_SWING, false));
        assert_eq!(played, [(24, 64), (64, 96)]);
        // A drum hit sounds to its end anyway: only its start moves.
        let played = ticks(&swung(&notes, MAX_SWING, true));
        assert_eq!(played, [(24, 48), (64, 96)]);
    }

    #[test]
    fn only_songs_with_swing_and_quarter_beats_swing() {
        let mut project = Project::template();
        let channel = 0;
        project
            .pattern_mut(0)
            .unwrap()
            .set_notes(channel, eighths());
        assert!(matches!(project.played_notes(0, channel), Cow::Borrowed(_)));
        project.swing = MAX_SWING;
        assert_eq!(project.played_notes(0, channel)[1].nudge, 16);

        // In 6/8 the eighths are beats: nothing swings.
        project.time_signature = TimeSignature::new(6, 8);
        assert_eq!(project.pattern_swing(0), 0);
        assert_eq!(project.played_notes(0, channel)[1].nudge, 0);

        // A pattern takes the time signature where its clip starts.
        project.time_signature = TimeSignature::new(3, 4);
        project.add_clip(Clip::new(0, 0, 0));
        assert!(project.set_time_marker(TimeMarker::new(0, TimeSignature::new(12, 8))));
        assert_eq!(project.pattern_swing(0), 0);
        assert!(project.set_time_marker(TimeMarker::new(0, TimeSignature::new(2, 2))));
        assert_eq!(project.pattern_swing(0), MAX_SWING);
    }

    #[test]
    fn a_midi_file_holds_the_swing() {
        use crate::midi_file::read::{EventKind, read};

        let mut project = Project::template();
        let synth = project
            .channels()
            .iter()
            .position(Channel::is_pitched)
            .unwrap();
        project.pattern_mut(0).unwrap().set_notes(synth, eighths());
        project.add_clip(Clip::new(0, 0, 0));
        project.swing = MAX_SWING;
        let file = read(&crate::song_to_midi(&project)).unwrap();
        let ons: Vec<u64> = file
            .tracks
            .iter()
            .flat_map(|track| &track.events)
            .filter(|event| matches!(event.kind, EventKind::NoteOn { .. }))
            .map(|event| event.tick)
            .collect();
        assert_eq!(&ons[..4], &[0, 64, 96, 160]);
    }

    #[test]
    fn styles_and_rhythms_bring_their_swing() {
        use crate::BandStyle;
        use crate::library::demo::{song_demo, style_demo};
        use crate::library::{Rhythm, Song, Style, apply};

        let mut project = Project::with_band();
        assert_eq!(project.swing, 0);
        project.set_band_style(BandStyle::named("Shuffle"));
        assert_eq!(project.swing, MAX_SWING);
        project.set_band_style(BandStyle::named("Rock and roll"));
        assert_eq!(project.swing, 50);
        project.set_band_style(BandStyle::named("Pop beat"));
        assert_eq!(project.swing, 0);

        let mut song = Project::demo();
        apply::use_rhythm(&mut song, Rhythm::named("Swing").unwrap());
        assert_eq!(song.swing, MAX_SWING);
        apply::use_rhythm(&mut song, Rhythm::named("Pop beat").unwrap());
        assert_eq!(song.swing, 0);

        let demo = |name| song_demo(Song::named(name).unwrap()).swing;
        assert_eq!(demo("Midnight Shuffle"), MAX_SWING);
        assert_eq!(demo("Jingle Bells"), MAX_SWING);
        assert_eq!(demo("Für Elise"), 0);
        // A folk song keeps its eighths even over a jazz waltz.
        assert_eq!(demo("Scarborough Fair"), 0);
        assert_eq!(style_demo(Style::named("ii–V–I").unwrap()).swing, MAX_SWING);
    }
}
