//! The tune a lead sheet writes down: the notes of one channel, one at a
//! time, along the playlist's timeline.

use crate::pattern::TICKS_PER_STEP;
use crate::pitch::Note;
use crate::project::{ChannelKind, Project, SampleSource};

/// A note of the melody, in steps of the timeline (the song as drawn,
/// with repeated sections written once).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MelodyNote {
    pub start: usize,
    pub length: usize,
    pub pitch: Note,
}

impl MelodyNote {
    pub fn end(&self) -> usize {
        self.start + self.length
    }
}

/// The channels that could carry a tune: those that play pitches (not
/// the built-in drums) and have notes in a clip of the song.
pub fn melody_channels(project: &Project) -> Vec<usize> {
    project
        .channels()
        .iter()
        .enumerate()
        .filter(|(_, channel)| {
            !matches!(channel.kind, ChannelKind::Sampler(SampleSource::Builtin(_)))
        })
        .map(|(index, _)| index)
        .filter(|&channel| !raw_notes(project, channel, usize::MAX).is_empty())
        .collect()
}

/// The channel to write by default: the one called "Lead" (the tune of
/// every song demo), or else the channel whose notes are highest on
/// average, which is where a tune usually is.
pub fn default_melody_channel(project: &Project) -> Option<usize> {
    let channels = melody_channels(project);
    let lead = channels
        .iter()
        .copied()
        .find(|&channel| project.channels()[channel].name == crate::library::apply::LEAD);
    lead.or_else(|| {
        channels.into_iter().max_by_key(|&channel| {
            let notes = raw_notes(project, channel, usize::MAX);
            let sum: usize = notes
                .iter()
                .map(|note| usize::from(note.pitch.midi()))
                .sum();
            sum / notes.len().max(1)
        })
    })
}

/// Every note `channel` plays in the song before step `end`, as drawn
/// on the timeline, rounded to the nearest steps.
fn raw_notes(project: &Project, channel: usize, end: usize) -> Vec<MelodyNote> {
    let mut notes = Vec::new();
    for clip in project.playlist().audible_clips() {
        let Some(pattern) = project.pattern(clip.pattern) else {
            continue;
        };
        let length = pattern.length();
        for repeat in 0..clip.repeats {
            let from = clip.start + repeat * length;
            if from >= end {
                break;
            }
            for note in pattern.notes(channel).iter().filter(|n| n.start < length) {
                // In ticks, then rounded to steps: a note played a little
                // early or late, as recorded or imported, is written on
                // the nearest step.
                let start = (from + note.start) * TICKS_PER_STEP + usize::from(note.nudge);
                let stop = (start + note.ticks()).min((from + length) * TICKS_PER_STEP);
                let first = (start + TICKS_PER_STEP / 2) / TICKS_PER_STEP;
                let last = ((stop + TICKS_PER_STEP / 2) / TICKS_PER_STEP).max(first + 1);
                if first < end {
                    notes.push(MelodyNote {
                        start: first,
                        length: last.min(end).max(first + 1) - first,
                        pitch: note.pitch,
                    });
                }
            }
        }
    }
    notes
}

/// The tune `channel` plays before step `end`: one note at a time.
///
/// Where several notes start together, the highest is the tune (the
/// "skyline" rule: a melody harmonized in thirds is read as its top
/// voice). A note is held until the next one starts at the latest, so
/// the notes never overlap.
pub fn melody(project: &Project, channel: usize, end: usize) -> Vec<MelodyNote> {
    let mut notes = raw_notes(project, channel, end);
    notes.sort_by(|a, b| a.start.cmp(&b.start).then(b.pitch.cmp(&a.pitch)));
    notes.dedup_by_key(|note| note.start);
    for i in 1..notes.len() {
        let next = notes[i].start;
        let note = &mut notes[i - 1];
        if note.end() > next {
            note.length = next - note.start;
        }
    }
    notes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::NoteEvent;
    use crate::playlist::Clip;

    fn song(notes: &[NoteEvent]) -> Project {
        let mut project = Project::empty();
        let channel = crate::library::apply::synth_named(&mut project, "Lead").unwrap();
        let pattern = project.pattern_mut(0).unwrap();
        pattern.set_length(16);
        pattern.set_notes(channel, notes.to_vec());
        project.add_clip(Clip::new(0, 0, 0));
        project
    }

    fn pitches(notes: &[MelodyNote]) -> Vec<(usize, usize, u8)> {
        notes
            .iter()
            .map(|n| (n.start, n.length, n.pitch.midi()))
            .collect()
    }

    #[test]
    fn the_top_note_of_a_chord_is_the_tune() {
        let project = song(&[
            NoteEvent::new(Note::new(60), 0, 4),
            NoteEvent::new(Note::new(64), 0, 4),
            NoteEvent::new(Note::new(67), 4, 8),
            NoteEvent::new(Note::new(62), 8, 4),
        ]);
        let channel = default_melody_channel(&project).unwrap();
        let tune = melody(&project, channel, 16);
        // The G is cut where the D starts, so only one note sounds.
        assert_eq!(pitches(&tune), [(0, 4, 64), (4, 4, 67), (8, 4, 62)]);
    }

    #[test]
    fn notes_off_the_grid_are_written_on_the_nearest_step() {
        // An eighth-note triplet: ticks 0, 32 and 64, each 32 long.
        let project = song(&[
            NoteEvent::exact(Note::new(60), 0, 32),
            NoteEvent::exact(Note::new(62), 32, 32),
            NoteEvent::exact(Note::new(64), 64, 32),
        ]);
        let channel = default_melody_channel(&project).unwrap();
        let tune = melody(&project, channel, 16);
        assert_eq!(pitches(&tune), [(0, 1, 60), (1, 2, 62), (3, 1, 64)]);
    }

    #[test]
    fn a_repeated_clip_plays_its_notes_again() {
        let mut project = song(&[NoteEvent::new(Note::new(60), 0, 4)]);
        project.add_clip(Clip::new(0, 0, 16).repeated(2));
        let channel = default_melody_channel(&project).unwrap();
        let tune = melody(&project, channel, 64);
        let starts: Vec<usize> = tune.iter().map(|n| n.start).collect();
        assert_eq!(starts, [0, 16, 32]);
    }
}
