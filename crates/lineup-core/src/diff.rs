//! Working out what changed between two versions of a project.
//!
//! The audio thread keeps its own copy of everything it plays, and the app
//! tells it about every change. Most edits change one thing, but undo
//! swaps in a whole earlier project at once. [`diff`] compares the project
//! before and after any change and lists the differences as [`Change`]s,
//! in an order the audio thread can follow one by one. The app turns each
//! one into a command.
//!
//! Only what the audio thread plays is compared. Names, the key and the
//! track names make no sound, so changing them gives no `Change`.

use std::borrow::Cow;

use crate::Project;

/// One difference between two versions of a project.
///
/// The numbers are positions in the list *at the moment the change is
/// applied*, after the changes before it in the list. Values (a pattern's
/// length, a channel's volume...) are read from the newer project.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Change {
    /// Pattern `n` was removed; the ones after it move down one place.
    RemovePattern(usize),
    /// A pattern was inserted at position `n`, with no notes yet.
    InsertPattern(usize),
    /// Channel `n` was removed, with its notes in every pattern.
    RemoveChannel(usize),
    /// A channel was inserted at position `n`, with its notes in every pattern.
    InsertChannel(usize),
    /// Channel `n` has another instrument (for example, a new sample, or
    /// another General MIDI program).
    Instrument(usize),
    Volume(usize),
    Muted(usize),
    PatternLength(usize),
    Notes {
        pattern: usize,
        channel: usize,
    },
    Tempo,
    /// How loose the timing is.
    LooseTiming,
    /// The clips that can be heard, the song's length, or the tempo of
    /// its steps.
    Song,
}

/// How the items of an old list line up with the items of a new one.
///
/// The first `start` items are the same in both. After them, `removed`
/// old items were replaced by `inserted` new ones, and the rest are the
/// same again. When both lists are as long, nothing was removed or
/// inserted, and item `i` lines up with item `i` (it may have changed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Alignment {
    start: usize,
    removed: usize,
    inserted: usize,
}

impl Alignment {
    fn new<T: PartialEq>(before: &[T], after: &[T]) -> Self {
        if before.len() == after.len() {
            return Alignment {
                start: 0,
                removed: 0,
                inserted: 0,
            };
        }
        // The same at the start...
        let start = before
            .iter()
            .zip(after)
            .take_while(|(old, new)| old == new)
            .count();
        // ...and the same at the end, without counting an item twice.
        let room = before.len().min(after.len()) - start;
        let end = before[start..]
            .iter()
            .rev()
            .zip(after[start..].iter().rev())
            .take_while(|(old, new)| old == new)
            .count()
            .min(room);
        Alignment {
            start,
            removed: before.len() - start - end,
            inserted: after.len() - start - end,
        }
    }

    /// Where item `index` of the new list was in the old one, or `None`
    /// if it was inserted.
    fn old_index(self, index: usize) -> Option<usize> {
        if index < self.start {
            Some(index)
        } else if index < self.start + self.inserted {
            None
        } else {
            Some(index - self.inserted + self.removed)
        }
    }

    /// The changes that turn the old list into the new one: removals from
    /// the last one backwards, so the positions of the others stay right,
    /// then insertions from the first one forwards.
    fn changes(
        self,
        remove: fn(usize) -> Change,
        insert: fn(usize) -> Change,
    ) -> impl Iterator<Item = Change> {
        let removals = (self.start..self.start + self.removed).rev().map(remove);
        let insertions = (self.start..self.start + self.inserted).map(insert);
        removals.chain(insertions)
    }
}

/// Lists what the audio thread must change to go from `before` to `after`.
///
/// The order matters: patterns are added and removed first, so that a
/// channel added afterwards comes with notes for the right patterns; then
/// channels; then everything that changed inside them.
pub fn diff(before: &Project, after: &Project) -> Vec<Change> {
    let patterns = Alignment::new(before.patterns(), after.patterns());
    let channels = Alignment::new(before.channels(), after.channels());

    let mut changes: Vec<Change> = patterns
        .changes(Change::RemovePattern, Change::InsertPattern)
        .chain(channels.changes(Change::RemoveChannel, Change::InsertChannel))
        .collect();

    // Channels that were there before: their instrument, volume and mute.
    for (index, channel) in after.channels().iter().enumerate() {
        let Some(old) = channels.old_index(index).map(|old| &before.channels()[old]) else {
            continue; // a new channel arrives complete
        };
        if old.kind != channel.kind || old.program != channel.program {
            changes.push(Change::Instrument(index));
        }
        if old.volume != channel.volume {
            changes.push(Change::Volume(index));
        }
        if old.muted != channel.muted {
            changes.push(Change::Muted(index));
        }
    }

    // Pattern lengths, and the notes of every channel in every pattern.
    for (index, pattern) in after.patterns().iter().enumerate() {
        let old_index = patterns.old_index(index);
        let old_pattern = old_index.map(|old| &before.patterns()[old]);
        if let Some(old) = old_pattern
            && old.length() != pattern.length()
        {
            changes.push(Change::PatternLength(index));
        }
        for channel in 0..after.channels().len() {
            let Some(old_channel) = channels.old_index(channel) else {
                continue; // a new channel brought its notes with it
            };
            // The notes as they are played, with the song's swing: a
            // new amount, or a pattern now in a time signature that does
            // not swing, plays other notes. A new pattern starts with no
            // notes in the audio thread.
            let old_notes = match old_index {
                Some(old) => before.played_notes(old, old_channel),
                None => Cow::Borrowed(&[][..]),
            };
            if old_notes != after.played_notes(index, channel) {
                changes.push(Change::Notes {
                    pattern: index,
                    channel,
                });
            }
        }
    }

    if before.tempo != after.tempo {
        changes.push(Change::Tempo);
    }
    if before.loose_timing != after.loose_timing {
        changes.push(Change::LooseTiming);
    }
    // What song mode plays: the clips, with their repeats and the
    // sections' repeats unrolled, cut to the loop region if there is one.
    // Only what the audio thread is sent counts, so renaming a section
    // changes nothing.
    let (old_song, new_song) = (before.playback_arrangement(), after.playback_arrangement());
    if old_song.plays != new_song.plays
        || old_song.length != new_song.length
        || before.played_tempos(&old_song) != after.played_tempos(&new_song)
    {
        changes.push(Change::Song);
    }
    changes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Channel, ChannelKind, Drum, LoopRegion, Note, NoteEvent, Play, SampleSource, Section, Tempo,
    };

    /// What the audio thread knows about one channel.
    #[derive(Debug, Clone, PartialEq)]
    struct ModelChannel {
        kind: ChannelKind,
        program: Option<u8>,
        volume: f32,
        muted: bool,
        /// One list per pattern.
        notes: Vec<Vec<NoteEvent>>,
    }

    /// A stand-in for the audio thread's copy of a project, to check that
    /// following the changes really turns `before` into `after`.
    #[derive(Debug, Clone, PartialEq)]
    struct Model {
        lengths: Vec<usize>,
        channels: Vec<ModelChannel>,
        tempo: Tempo,
        loose_timing: u8,
        plays: Vec<Play>,
        song_length: usize,
        tempos: Vec<Tempo>,
    }

    impl Model {
        fn channel(project: &Project, index: usize) -> ModelChannel {
            let channel = &project.channels()[index];
            ModelChannel {
                kind: channel.kind.clone(),
                program: channel.program,
                volume: channel.volume,
                muted: channel.muted,
                notes: (0..project.patterns().len())
                    .map(|pattern| project.played_notes(pattern, index).into_owned())
                    .collect(),
            }
        }

        fn of(project: &Project) -> Self {
            Model {
                lengths: project.patterns().iter().map(|p| p.length()).collect(),
                channels: (0..project.channels().len())
                    .map(|index| Model::channel(project, index))
                    .collect(),
                tempo: project.tempo,
                loose_timing: project.loose_timing,
                plays: project.playback_arrangement().plays,
                song_length: project.playback_arrangement().length,
                tempos: project.played_tempos(&project.playback_arrangement()),
            }
        }

        /// Does what the audio thread does when it is told about `change`.
        fn apply(&mut self, change: Change, after: &Project) {
            match change {
                Change::RemovePattern(index) => {
                    self.lengths.remove(index);
                    for channel in &mut self.channels {
                        channel.notes.remove(index);
                    }
                }
                Change::InsertPattern(index) => {
                    self.lengths.insert(index, after.patterns()[index].length());
                    for channel in &mut self.channels {
                        channel.notes.insert(index, Vec::new());
                    }
                }
                Change::RemoveChannel(index) => {
                    self.channels.remove(index);
                }
                Change::InsertChannel(index) => {
                    self.channels.insert(index, Model::channel(after, index));
                }
                Change::Instrument(index) => {
                    self.channels[index].kind = after.channels()[index].kind.clone();
                    self.channels[index].program = after.channels()[index].program;
                }
                Change::Volume(index) => {
                    self.channels[index].volume = after.channels()[index].volume
                }
                Change::Muted(index) => self.channels[index].muted = after.channels()[index].muted,
                Change::PatternLength(index) => {
                    self.lengths[index] = after.patterns()[index].length();
                }
                Change::Notes { pattern, channel } => {
                    self.channels[channel].notes[pattern] =
                        after.played_notes(pattern, channel).into_owned();
                }
                Change::Tempo => self.tempo = after.tempo,
                Change::LooseTiming => self.loose_timing = after.loose_timing,
                Change::Song => {
                    let song = after.playback_arrangement();
                    self.tempos = after.played_tempos(&song);
                    self.plays = song.plays;
                    self.song_length = song.length;
                }
            }
        }
    }

    /// Diffs two projects, checks that following the changes turns the
    /// first into the second, and returns the changes.
    fn check(before: &Project, after: &Project) -> Vec<Change> {
        let changes = diff(before, after);
        let mut model = Model::of(before);
        for &change in &changes {
            model.apply(change, after);
        }
        assert_eq!(model, Model::of(after), "changes: {changes:?}");
        changes
    }

    /// Checks both directions, as undo and then redo would.
    fn check_both_ways(before: &Project, after: &Project) -> Vec<Change> {
        check(after, before);
        check(before, after)
    }

    fn synth_with_note(project: &mut Project, midi: u8) -> usize {
        let channel = project.add_channel(Channel::synth()).unwrap();
        project
            .pattern_mut(0)
            .unwrap()
            .add_note(channel, NoteEvent::new(Note::new(midi), 0, 4));
        channel
    }

    #[test]
    fn no_change_no_changes() {
        let project = Project::demo();
        assert_eq!(check(&project, &project.clone()), vec![]);
    }

    #[test]
    fn a_step_changes_one_list_of_notes() {
        let before = Project::demo();
        let mut after = before.clone();
        after.pattern_mut(0).unwrap().toggle(3, 5);
        assert_eq!(
            check_both_ways(&before, &after),
            vec![Change::Notes {
                pattern: 0,
                channel: 3
            }]
        );
    }

    #[test]
    fn names_and_the_key_make_no_sound() {
        let before = Project::demo();
        let mut after = before.clone();
        after.pattern_mut(1).unwrap().name = "Verse".to_string();
        after.track_mut(0).unwrap().name = "Keys".to_string();
        after.channel_mut(0).unwrap().name = "Boom".to_string();
        after.scale = crate::Scale::new(crate::PitchClass::A, crate::ScaleKind::NaturalMinor);
        assert_eq!(check(&before, &after), vec![]);
    }

    #[test]
    fn adding_a_pattern_inserts_it_at_the_end() {
        let before = Project::demo();
        let mut after = before.clone();
        after.add_pattern();
        assert_eq!(
            check_both_ways(&before, &after),
            vec![Change::InsertPattern(3)]
        );
    }

    #[test]
    fn a_cloned_pattern_brings_its_notes() {
        let before = Project::demo();
        let mut after = before.clone();
        after.clone_pattern(0);
        let changes = check(&before, &after);
        assert_eq!(changes[0], Change::InsertPattern(3));
        // Kick, clap and hat have notes in the beat; snare and synth do not.
        assert_eq!(changes.len(), 4);
    }

    #[test]
    fn removing_a_pattern_and_putting_it_back() {
        let before = Project::demo();
        let mut after = before.clone();
        after.remove_pattern(0);
        // The clips of the other patterns were renumbered: the song changed.
        assert_eq!(
            check(&before, &after),
            vec![Change::RemovePattern(0), Change::Song]
        );
        // Undoing it puts the pattern back where it was, with its notes.
        let undo = check(&after, &before);
        assert_eq!(undo[0], Change::InsertPattern(0));
        assert_eq!(undo.last(), Some(&Change::Song));
    }

    #[test]
    fn removing_a_channel_and_putting_it_back() {
        let before = Project::demo();
        let mut after = before.clone();
        after.remove_channel(2);
        assert_eq!(check(&before, &after), vec![Change::RemoveChannel(2)]);
        assert_eq!(check(&after, &before), vec![Change::InsertChannel(2)]);
    }

    #[test]
    fn identical_channels_are_told_apart_by_their_notes() {
        // Three Lineup Osc channels with different notes. Removing the
        // middle one looks, by the channels alone, like removing the last.
        let mut before = Project::template();
        for midi in [60, 64, 67] {
            synth_with_note(&mut before, midi);
        }
        let mut after = before.clone();
        after.remove_channel(6);
        let changes = check_both_ways(&before, &after);
        assert!(changes.contains(&Change::RemoveChannel(7)));
        assert!(changes.contains(&Change::Notes {
            pattern: 0,
            channel: 6
        }));
    }

    #[test]
    fn volume_mute_and_instrument() {
        let before = Project::demo();
        let mut after = before.clone();
        after.channel_mut(0).unwrap().volume = 0.3;
        after.channel_mut(1).unwrap().muted = true;
        after.channel_mut(2).unwrap().kind =
            ChannelKind::Sampler(SampleSource::Builtin(Drum::Snare));
        after.channel_mut(3).unwrap().program = Some(crate::general_midi::VIOLIN);
        assert_eq!(
            check_both_ways(&before, &after),
            vec![
                Change::Volume(0),
                Change::Muted(1),
                Change::Instrument(2),
                Change::Instrument(3)
            ]
        );
    }

    #[test]
    fn tempo_changes_and_loose_timing() {
        let before = Project::demo();
        let mut after = before.clone();
        after.set_tempo_marker(crate::TempoMarker::gliding(64, 90.0, 32));
        assert_eq!(check_both_ways(&before, &after), vec![Change::Song]);

        // With a tempo change, the tempo box changes the tempo of the
        // song's steps too.
        let mut faster = after.clone();
        faster.scale_tempo(Tempo::new(150.0));
        assert_eq!(
            check_both_ways(&after, &faster),
            vec![Change::Tempo, Change::Song]
        );

        let mut loose = before.clone();
        loose.loose_timing = 10;
        assert_eq!(check_both_ways(&before, &loose), vec![Change::LooseTiming]);
    }

    #[test]
    fn swing_plays_other_notes() {
        let before = Project::demo();
        let mut after = before.clone();
        after.swing = 60;
        let changes = check_both_ways(&before, &after);
        assert!(!changes.is_empty());
        assert!(
            changes
                .iter()
                .all(|change| matches!(change, Change::Notes { .. })),
            "{changes:?}"
        );
        // Only the channels with notes on the second eighth of a beat.
        let offbeats = |pattern: usize, channel: usize| {
            before.patterns()[pattern]
                .notes(channel)
                .iter()
                .any(|note| note.start % 4 == 2 || note.end() % 4 == 2)
        };
        for change in changes {
            if let Change::Notes { pattern, channel } = change {
                assert!(offbeats(pattern, channel), "{pattern}, {channel}");
            }
        }
    }

    #[test]
    fn tempo_and_song() {
        let before = Project::demo();
        let mut after = before.clone();
        after.tempo = Tempo::new(90.0);
        after.move_clip(0, 1, 16 * 20);
        assert_eq!(
            check_both_ways(&before, &after),
            vec![Change::Tempo, Change::Song]
        );

        // A muted track takes its clips out of the song.
        let mut muted = before.clone();
        muted.track_mut(1).unwrap().muted = true;
        assert_eq!(check(&before, &muted), vec![Change::Song]);
    }

    #[test]
    fn song_structure_changes_the_song() {
        let before = Project::demo();
        let mut after = before.clone();
        let mut chorus = Section::new("Chorus", 32, 64);
        chorus.repeats = 2;
        after.add_section(chorus);
        assert_eq!(check_both_ways(&before, &after), vec![Change::Song]);

        // A section that plays once, and its name, change nothing heard.
        let mut named = before.clone();
        named.add_section(Section::new("Intro", 0, 32));
        assert!(check_both_ways(&before, &named).is_empty());

        let mut repeated = before.clone();
        repeated.set_clip_repeats(0, 2);
        assert_eq!(check_both_ways(&before, &repeated), vec![Change::Song]);

        let mut looped = before.clone();
        looped.set_loop_region(Some(LoopRegion::new(32, 64)));
        assert_eq!(check_both_ways(&before, &looped), vec![Change::Song]);
    }

    #[test]
    fn a_longer_pattern_makes_a_longer_song() {
        let before = Project::demo();
        let mut after = before.clone();
        // The fill is the last clip; making it longer makes the song longer.
        after.pattern_mut(2).unwrap().set_length(32);
        assert_eq!(
            check_both_ways(&before, &after),
            vec![Change::PatternLength(2), Change::Song]
        );
    }

    #[test]
    fn many_changes_at_once() {
        // What one undo step might hold if several things changed in it.
        let before = Project::demo();
        let mut after = before.clone();
        after.remove_pattern(1);
        after.add_channel(Channel::drum(Drum::Snare));
        after.pattern_mut(0).unwrap().toggle(5, 3);
        after.channel_mut(4).unwrap().volume = 0.1;
        after.tempo = Tempo::new(140.0);
        check_both_ways(&before, &after);

        // And starting from scratch, as when a file is opened.
        check_both_ways(&Project::empty(), &Project::demo());
    }
}
