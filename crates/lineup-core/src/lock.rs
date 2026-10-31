//! Locked channels: the ones styles leave alone.
//!
//! A locked channel keeps its notes and its sound (its instrument) when
//! a style, a rhythm, a saved style, a row or micromode style, the Time
//! box or the band changes the song, and the rest of the song changes as
//! usual. A style treats it as a channel it has no part for. You can
//! still edit a locked channel by hand, and what moves the whole song,
//! such as a new time signature or key, moves its notes with the others,
//! so they still fit the bars.
//!
//! The library writes into a pattern only through a [`StylePattern`],
//! which leaves the rows of locked channels as they are, and changes a
//! channel's sound only through [`Project::set_sound`]. The band leaves
//! a player that plays on a locked channel as it is, keeps the notes of
//! locked drums, and leaves a part to a locked "Keys" or "Bass" of the
//! song's own (see [`crate::band`]).

use std::ops::Deref;

use crate::ChannelKind;
use crate::pattern::{NoteEvent, Pattern};
use crate::project::{MAX_CHANNELS, Project};

/// A set of channels, by number in the rack.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Locked(u32);

// One bit for each channel there can be.
const _: () = assert!(MAX_CHANNELS <= u32::BITS as usize);

impl Locked {
    pub const NONE: Locked = Locked(0);

    pub fn contains(self, channel: usize) -> bool {
        channel < MAX_CHANNELS && self.0 & (1 << channel) != 0
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// The channels in the set, from the top of the rack.
    pub fn iter(self) -> impl Iterator<Item = usize> {
        (0..MAX_CHANNELS).filter(move |&channel| self.contains(channel))
    }

    fn with(self, channel: usize, on: bool) -> Self {
        let bit = 1 << channel;
        Locked(if on { self.0 | bit } else { self.0 & !bit })
    }

    /// The set once channel `index` is removed from the rack: the
    /// channels after it are numbered one lower.
    pub(crate) fn without(self, index: usize) -> Self {
        if index >= MAX_CHANNELS {
            return self;
        }
        let below = self.0 & ((1 << index) - 1);
        let above = self.0.checked_shr(index as u32 + 1).unwrap_or(0) << index;
        Locked(below | above)
    }

    /// The set as a list of channels, as a file writes it.
    pub(crate) fn to_vec(self) -> Vec<usize> {
        self.iter().collect()
    }

    /// The set of `channels`, as a file reads it. Channels the rack does
    /// not have are left out.
    pub(crate) fn of(channels: impl IntoIterator<Item = usize>, count: usize) -> Self {
        channels
            .into_iter()
            .filter(|&channel| channel < count.min(MAX_CHANNELS))
            .fold(Locked::NONE, |set, channel| set.with(channel, true))
    }
}

impl Project {
    /// The locked channels.
    pub fn locked(&self) -> Locked {
        self.locked
    }

    /// Whether channel `channel` is locked.
    pub fn is_locked(&self, channel: usize) -> bool {
        self.locked.contains(channel)
    }

    /// The names of the locked channels, from the top of the rack.
    pub fn locked_names(&self) -> Vec<&str> {
        self.locked
            .iter()
            .filter_map(|channel| self.channels().get(channel))
            .map(|channel| channel.name.as_str())
            .collect()
    }

    /// Locks channel `channel`, or unlocks it. Returns `false`, changing
    /// nothing, if there is no such channel or it already was.
    ///
    /// Locking changes nothing you hear. Unlocking a "Keys" or "Bass" of
    /// the song's own while the band plays has the band take that part
    /// over again, as it did when it came in: the song's own goes quiet.
    pub fn set_locked(&mut self, channel: usize, on: bool) -> bool {
        if channel >= self.channels().len() || self.is_locked(channel) == on {
            return false;
        }
        let handed_back = !on && self.band_leaves_to(channel);
        self.locked = self.locked.with(channel, on);
        if handed_back {
            self.give_to_the_band(channel);
        }
        true
    }

    /// Pattern `index`, for the library to write in. What it writes on a
    /// locked channel is left out.
    pub(crate) fn style_pattern(&mut self, index: usize) -> Option<StylePattern<'_>> {
        let locked = self.locked;
        let pattern = self.pattern_mut(index)?;
        Some(StylePattern { pattern, locked })
    }

    /// Clears the notes of pattern `index` on every channel that is not
    /// locked, for a style to be tried there afresh. Returns `false` if
    /// there is no such pattern.
    pub fn clear_unlocked(&mut self, index: usize) -> bool {
        self.style_pattern(index)
            .map(|mut pattern| pattern.clear())
            .is_some()
    }

    /// Adds a pattern to try a library style in, while pattern `from` is
    /// selected. If locked channels play in `from`, the new pattern
    /// starts as a copy of it with only their notes, as long and in the
    /// same time, so the style is tried around them: the bass then
    /// follows the chords of a locked Keys. Otherwise it is a new empty
    /// pattern, as [`Project::add_pattern`] makes. Returns its index, or
    /// `None` if there is no room.
    pub fn add_pattern_around_locked(&mut self, from: usize) -> Option<usize> {
        let plays_locked = self
            .pattern(from)
            .is_some_and(|pattern| self.locked.iter().any(|c| !pattern.notes(c).is_empty()));
        if !plays_locked {
            return self.add_pattern();
        }
        let index = self.clone_pattern(from)?;
        self.clear_unlocked(index);
        let name = format!("Pattern {}", index + 1);
        if let Some(pattern) = self.pattern_mut(index) {
            pattern.name = name;
        }
        Some(index)
    }

    /// Gives channel `index` the instrument `program`, and the kind of
    /// sound `kind` if there is one, unless it is locked. Returns whether
    /// it did.
    pub(crate) fn set_sound(
        &mut self,
        index: usize,
        kind: Option<ChannelKind>,
        program: Option<u8>,
    ) -> bool {
        if self.is_locked(index) {
            return false;
        }
        let Some(channel) = self.channel_mut(index) else {
            return false;
        };
        if let Some(kind) = kind {
            channel.kind = kind;
        }
        channel.program = program;
        true
    }

    /// Renumbers the locked channels once channel `index` is removed.
    pub(crate) fn unlock_removed(&mut self, index: usize) {
        self.locked = self.locked.without(index);
    }

    /// Restores the locked channels a file saved.
    pub(crate) fn lock_saved(&mut self, channels: impl IntoIterator<Item = usize>) {
        self.locked = Locked::of(channels, self.channels().len());
    }
}

/// A pattern the library writes in, which leaves the rows of locked
/// channels as they are. It reads like a [`Pattern`].
pub(crate) struct StylePattern<'a> {
    pattern: &'a mut Pattern,
    locked: Locked,
}

impl StylePattern<'_> {
    /// Replaces the notes of `channel`, unless it is locked. Returns
    /// whether it did.
    pub(crate) fn set_notes(
        &mut self,
        channel: usize,
        notes: impl IntoIterator<Item = NoteEvent>,
    ) -> bool {
        !self.locked.contains(channel) && self.pattern.set_notes(channel, notes)
    }

    pub(crate) fn set_length(&mut self, length: usize) {
        self.pattern.set_length(length);
    }

    pub(crate) fn set_time(&mut self, time: Option<crate::TimeSignature>) {
        self.pattern.time = time;
    }

    /// Clears every row that is not locked.
    pub(crate) fn clear(&mut self) {
        for channel in 0..self.pattern.rows() {
            self.set_notes(channel, []);
        }
    }
}

impl Deref for StylePattern<'_> {
    type Target = Pattern;

    fn deref(&self) -> &Pattern {
        self.pattern
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::apply::{self, StyleParts};
    use crate::library::demo::{micromode_demo, row_demo, song_demo, style_demo};
    use crate::library::{
        FILLS, MICROMODE_STYLES, RHYTHMS, ROW_STYLES, SONGS, STYLES, micromodes, rows,
    };
    use crate::remeter::remeter;
    use crate::{Note, TimeSignature};

    /// Checks that every channel locked in `before` is the same in
    /// `after`, its sound and its notes in every pattern, but for the
    /// song being put in `time` bar for bar, which moves every note. A
    /// pattern added since has no notes on it.
    fn assert_kept(before: &Project, after: &Project, time: Option<TimeSignature>, what: &str) {
        let mut expected = before.clone();
        if let Some(time) = time {
            remeter(&mut expected, time);
        }
        assert_eq!(after.locked(), before.locked(), "{what}");
        for channel in before.locked().iter() {
            assert_eq!(
                after.channels()[channel],
                expected.channels()[channel],
                "{what}: the sound of channel {channel}"
            );
            for (index, pattern) in after.patterns().iter().enumerate() {
                let kept = expected
                    .pattern(index)
                    .map_or(&[][..], |pattern| pattern.notes(channel));
                assert_eq!(
                    pattern.notes(channel),
                    kept,
                    "{what}: pattern {index}, channel {channel}"
                );
            }
        }
    }

    /// Checks that the channels not locked in `locked` play in it as they
    /// do in `free`, where nothing was locked: a lock leaves the rest of
    /// the style as it was.
    fn assert_rest_as_without(locked: &Project, free: &Project, what: &str) {
        assert_eq!(locked.channels().len(), free.channels().len(), "{what}");
        for channel in (0..free.channels().len()).filter(|&c| !locked.is_locked(c)) {
            assert_eq!(
                locked.channels()[channel],
                free.channels()[channel],
                "{what}: channel {channel}"
            );
            for (index, pattern) in free.patterns().iter().enumerate() {
                assert_eq!(
                    locked.pattern(index).unwrap().notes(channel),
                    pattern.notes(channel),
                    "{what}: pattern {index}, channel {channel}"
                );
            }
        }
    }

    #[test]
    fn no_library_choice_writes_a_locked_channel() {
        // Every style, rhythm and fill is tried, on the short demos of the
        // styles and, now and then, on a whole song: a debug build would
        // take minutes over every song.
        let songs: Vec<Project> = SONGS.iter().step_by(8).map(song_demo).collect();
        let demos: Vec<Project> = ROW_STYLES
            .iter()
            .map(row_demo)
            .chain(MICROMODE_STYLES.iter().map(micromode_demo))
            .chain(STYLES.iter().map(style_demo))
            .collect();
        let in_pattern = StyleParts {
            time_signature: false,
            ..StyleParts::default()
        };
        let with_chords = StyleParts {
            progression: true,
            ..StyleParts::default()
        };
        let tries = demos.len().max(RHYTHMS.len());
        for index in 0..tries {
            let demo = match index % 8 {
                0 => &songs[index / 8 % songs.len()],
                _ => &demos[index % demos.len()],
            };
            let style = &STYLES[index % STYLES.len()];
            let rhythm = &RHYTHMS[index % RHYTHMS.len()];
            let fill = &FILLS[index % FILLS.len()];
            let row = &ROW_STYLES[index % ROW_STYLES.len()];
            let micromode = &MICROMODE_STYLES[index % MICROMODE_STYLES.len()];
            let channels = demo.channels().len();
            let one = index % channels;
            // Every channel locked, now and then.
            for all in [false, true]
                .into_iter()
                .take(1 + usize::from(index % 4 == 1))
            {
                let mut before = demo.clone();
                for channel in (0..channels).filter(|&c| all || c == one) {
                    before.set_locked(channel, true);
                }
                let what = |action: &str| format!("{action} on demo {index}, all locked: {all}");
                let mut project = before.clone();
                let change = apply::use_style_parts(&mut project, style, StyleParts::default());
                assert_kept(&before, &project, change.time, &what(style.name));
                // Whether the rest plays as without the lock, and the
                // chords of a style, are checked now and then: they take
                // the longest.
                let sometimes = index % 4 == 2;
                if sometimes && !all {
                    let mut free = demo.clone();
                    apply::use_style_parts(&mut free, style, StyleParts::default());
                    assert_rest_as_without(&project, &free, &what(style.name));
                }
                if sometimes {
                    let mut project = before.clone();
                    let change = apply::use_style_parts(&mut project, style, with_chords);
                    assert_kept(&before, &project, change.time, &what("chords"));
                }

                let parts = StyleParts::default().for_rhythm();
                let mut project = before.clone();
                let change = apply::use_rhythm_parts(&mut project, rhythm, parts);
                assert_kept(&before, &project, change.time, &what(rhythm.name));
                if sometimes && !all {
                    let mut free = demo.clone();
                    apply::use_rhythm_parts(&mut free, rhythm, parts);
                    assert_rest_as_without(&project, &free, &what(rhythm.name));
                }

                let mut project = before.clone();
                apply::use_fill(&mut project, fill);
                assert_kept(&before, &project, None, &what(fill.name));

                // A micromode style or a twelve-tone style in a song is the
                // band's: see `band::micromode` and `band::row` for how
                // they keep locked parts.

                // Tried in a pattern of the song, with the locked channel
                // selected, and in a new one.
                let mut project = before.clone();
                apply::write_style_parts(&mut project, 0, style, Some(one), in_pattern);
                assert_kept(&before, &project, None, &what(style.name));
                let mut project = before.clone();
                apply::write_rhythm_parts(&mut project, 0, rhythm, in_pattern);
                assert_kept(&before, &project, None, &what(rhythm.name));
                let mut project = before.clone();
                let key = project.scale.root;
                let on = row.row_in(project.scale, false);
                rows::write_row_style(&mut project, 0, row, on, Some(one));
                assert_kept(&before, &project, None, &what(row.name));
                let start = micromodes::DEMO_SECTIONS[0].0;
                let mut project = before.clone();
                micromodes::write_micromode_style(&mut project, 0, micromode, key, start, None);
                assert_kept(&before, &project, None, &what(micromode.name));
                let mut project = before.clone();
                if let Some(new) = project.add_pattern() {
                    let parts = StyleParts::default();
                    apply::write_style_parts(&mut project, new, style, Some(one), parts);
                    assert_kept(&before, &project, None, &what(style.name));
                }
                let mut project = before.clone();
                if let Some(new) = project.add_pattern_around_locked(0) {
                    apply::write_style_parts(&mut project, new, style, None, in_pattern);
                    let trial = project.pattern(new).unwrap();
                    for channel in before.locked().iter() {
                        assert_eq!(
                            trial.notes(channel),
                            before.pattern(0).unwrap().notes(channel),
                            "{}",
                            what(style.name)
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn removing_a_channel_renumbers_the_ones_after_it() {
        let locked = Locked::NONE.with(1, true).with(3, true).with(31, true);
        assert_eq!(locked.without(0).to_vec(), [0, 2, 30]);
        assert_eq!(locked.without(1).to_vec(), [2, 30]);
        assert_eq!(locked.without(2).to_vec(), [1, 2, 30]);
        assert_eq!(locked.without(31).to_vec(), [1, 3]);
        assert_eq!(Locked::of([0, 5, 40], 4).to_vec(), [0]);
    }

    #[test]
    fn a_locked_row_keeps_its_notes_in_a_style_pattern() {
        let mut project = Project::demo();
        let note = NoteEvent::new(Note::new(60), 0, 4);
        assert!(project.set_locked(1, true));
        assert!(!project.set_locked(1, true));
        let before = project.pattern(0).unwrap().notes(1).to_vec();
        let mut pattern = project.style_pattern(0).unwrap();
        assert!(!pattern.set_notes(1, [note]));
        assert!(pattern.set_notes(0, [note]));
        pattern.clear();
        assert_eq!(project.pattern(0).unwrap().notes(1), before);
        assert!(project.pattern(0).unwrap().notes(0).is_empty());
    }

    #[test]
    fn a_locked_channel_follows_its_row_when_another_is_removed() {
        let mut project = Project::demo();
        let last = project.channels().len() - 1;
        let name = project.channels()[last].name.clone();
        project.set_locked(last, true);
        project.remove_channel(0);
        assert_eq!(project.locked_names(), [name.as_str()]);
        project.remove_channel(last - 1);
        assert!(project.locked().is_empty());
    }

    #[test]
    fn a_trial_starts_from_the_locked_notes() {
        let mut project = Project::demo();
        let empty = project.add_pattern_around_locked(0).unwrap();
        assert!(project.pattern(empty).unwrap().is_empty());
        project.set_locked(1, true);
        let around = project.add_pattern_around_locked(0).unwrap();
        let (from, trial) = (
            project.pattern(0).unwrap(),
            project.pattern(around).unwrap(),
        );
        assert_eq!(trial.notes(1), from.notes(1));
        assert_eq!(trial.length(), from.length());
        assert!(trial.player.is_none());
        assert!(
            (0..trial.rows())
                .filter(|&channel| channel != 1)
                .all(|channel| trial.notes(channel).is_empty())
        );
        assert_eq!(trial.name, format!("Pattern {}", around + 1));
    }
}
