//! Patterns: the notes each channel plays.
//!
//! Since step 3, a pattern stores *notes* for each channel: a pitch, a
//! start step and a length. The step sequencer and the piano roll are
//! two views of the same notes. A step that is "on" in the channel rack
//! is simply a note one step long, at the default pitch [`STEP_NOTE`].
//!
//! Since step 4, a project has many patterns, and the playlist arranges
//! them into a song. Each pattern still holds notes for every channel.

use serde::{Deserialize, Serialize};

use crate::Note;

/// The longest pattern supported, in steps.
pub const MAX_STEPS: usize = 64;

/// The pattern lengths offered in the UI, in steps (sixteenth notes).
pub const PATTERN_LENGTHS: [usize; 3] = [16, 32, 64];

/// The pitch a step of the step sequencer plays. Samplers play their
/// sound at its original speed at this pitch.
pub const STEP_NOTE: Note = Note::C4;

/// How hard notes are played unless told otherwise, from 1 to 127.
pub const DEFAULT_VELOCITY: u8 = 100;

/// One note of a pattern. Times are counted in steps (sixteenth notes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NoteEvent {
    pub pitch: Note,
    /// The step the note starts on, counted from 0.
    pub start: usize,
    /// How many steps the note is held for, at least 1.
    pub length: usize,
    /// How hard the note is played, from 1 to 127 (as in MIDI).
    pub velocity: u8,
}

impl NoteEvent {
    pub fn new(pitch: Note, start: usize, length: usize) -> Self {
        NoteEvent {
            pitch,
            start,
            length,
            velocity: DEFAULT_VELOCITY,
        }
    }

    /// A step of the step sequencer: one step long, at [`STEP_NOTE`].
    pub fn step(start: usize) -> Self {
        NoteEvent::new(STEP_NOTE, start, 1)
    }

    /// The step just after the note ends.
    pub fn end(&self) -> usize {
        self.start + self.length
    }

    /// Whether the note is still held during `step`.
    pub fn sounds_at(&self, step: usize) -> bool {
        (self.start..self.end()).contains(&step)
    }

    /// Whether this note looks like a plain step-sequencer step.
    pub fn is_step(&self) -> bool {
        self.pitch == STEP_NOTE && self.length == 1
    }

    /// Makes the note fit in the pattern grid: it must start before
    /// [`MAX_STEPS`] and last at least one step, and it is shortened if
    /// it would run past the end. Returns `None` if it cannot fit.
    fn fitted(self) -> Option<NoteEvent> {
        if self.start >= MAX_STEPS || self.length == 0 {
            return None;
        }
        Some(NoteEvent {
            length: self.length.min(MAX_STEPS - self.start),
            ..self
        })
    }
}

/// A pattern: a name, a list of notes for each channel, and a length.
///
/// `name` is public because any name is fine. `length` and `rows` are
/// private, because they have rules to keep: the length stays within
/// 1..=[`MAX_STEPS`], and there is one row per channel of the project.
#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    pub name: String,
    length: usize,
    rows: Vec<Vec<NoteEvent>>,
}

impl Pattern {
    /// Creates an empty pattern with `channels` rows and the given length.
    ///
    /// `impl Into<String>` accepts anything that can become a `String`:
    /// a `&str` such as `"Beat"`, a `String`, or the result of `format!`.
    pub fn new(name: impl Into<String>, channels: usize, length: usize) -> Self {
        Pattern {
            name: name.into(),
            length: length.clamp(1, MAX_STEPS),
            rows: vec![Vec::new(); channels],
        }
    }

    /// How many steps are played before the pattern loops.
    pub fn length(&self) -> usize {
        self.length
    }

    /// Changes the length. Notes beyond the new length are kept, so
    /// making the pattern longer again brings them back.
    pub fn set_length(&mut self, length: usize) {
        self.length = length.clamp(1, MAX_STEPS);
    }

    /// The number of rows (one per channel).
    pub fn rows(&self) -> usize {
        self.rows.len()
    }

    /// The notes of one channel, in the order they were added. A channel
    /// that does not exist has no notes.
    pub fn notes(&self, channel: usize) -> &[NoteEvent] {
        match self.rows.get(channel) {
            Some(row) => row,
            None => &[],
        }
    }

    /// Whether the pattern has no notes at all.
    pub fn is_empty(&self) -> bool {
        self.rows.iter().all(Vec::is_empty)
    }

    /// Every note of every channel, with the channel it belongs to.
    pub fn all_notes(&self) -> impl Iterator<Item = (usize, &NoteEvent)> {
        self.rows
            .iter()
            .enumerate()
            .flat_map(|(channel, row)| row.iter().map(move |note| (channel, note)))
    }

    /// Whether every note of the channel is a plain step, so the channel
    /// rack can show the row as step buttons.
    pub fn is_step_row(&self, channel: usize) -> bool {
        self.notes(channel).iter().all(NoteEvent::is_step)
    }

    /// Whether a note starts on this step (a step is "on").
    pub fn is_on(&self, channel: usize, step: usize) -> bool {
        self.notes(channel).iter().any(|note| note.start == step)
    }

    /// Switches a step on (adds a step note) or off (removes every note
    /// starting there). Returns `false` if it is outside the grid.
    pub fn set(&mut self, channel: usize, step: usize, on: bool) -> bool {
        if step >= MAX_STEPS {
            return false;
        }
        let Some(row) = self.rows.get_mut(channel) else {
            return false;
        };
        row.retain(|note| note.start != step);
        if on {
            row.push(NoteEvent::step(step));
        }
        true
    }

    /// Flips a step and returns its new value, or `None` if it is outside the grid.
    pub fn toggle(&mut self, channel: usize, step: usize) -> Option<bool> {
        let on = !self.is_on(channel, step);
        self.set(channel, step, on).then_some(on)
    }

    /// Adds a note and returns its index in [`Pattern::notes`]. Returns
    /// `None` if the channel does not exist, the note does not fit, or
    /// the same pitch already starts on the same step.
    pub fn add_note(&mut self, channel: usize, note: NoteEvent) -> Option<usize> {
        let note = note.fitted()?;
        let row = self.rows.get_mut(channel)?;
        let duplicate = row
            .iter()
            .any(|other| other.pitch == note.pitch && other.start == note.start);
        if duplicate {
            return None;
        }
        row.push(note);
        Some(row.len() - 1)
    }

    /// Removes a note by its index and returns it.
    pub fn remove_note(&mut self, channel: usize, index: usize) -> Option<NoteEvent> {
        let row = self.rows.get_mut(channel)?;
        (index < row.len()).then(|| row.remove(index))
    }

    /// Replaces a note, for example after it was moved or resized.
    /// Returns `false` if there is no such note, the new one does not fit,
    /// or another note of the same pitch already starts on the same step.
    pub fn replace_note(&mut self, channel: usize, index: usize, note: NoteEvent) -> bool {
        let Some(note) = note.fitted() else {
            return false;
        };
        let Some(row) = self.rows.get_mut(channel) else {
            return false;
        };
        let duplicate = row.iter().enumerate().any(|(other_index, other)| {
            other_index != index && other.pitch == note.pitch && other.start == note.start
        });
        match row.get_mut(index) {
            Some(slot) if !duplicate => {
                *slot = note;
                true
            }
            _ => false,
        }
    }

    /// The pitches of a channel that are held during `step`.
    ///
    /// This returns an iterator instead of a `Vec`: nothing is collected
    /// or allocated, and the caller decides what to do with the pitches.
    /// The `'_` says the iterator borrows from `self`.
    pub fn sounding_at(&self, channel: usize, step: usize) -> impl Iterator<Item = Note> + '_ {
        self.notes(channel)
            .iter()
            .filter(move |note| note.sounds_at(step))
            .map(|note| note.pitch)
    }

    /// Adds an empty row at the bottom.
    pub(crate) fn push_row(&mut self) {
        self.rows.push(Vec::new());
    }

    /// Removes a row, moving the rows below it up.
    pub(crate) fn remove_row(&mut self, channel: usize) {
        if channel < self.rows.len() {
            self.rows.remove(channel);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_empty() {
        let pattern = Pattern::new("Test", 2, 16);
        assert_eq!(pattern.rows(), 2);
        assert_eq!(pattern.length(), 16);
        assert!(!pattern.is_on(0, 0));
        assert!(pattern.notes(0).is_empty());
        assert!(pattern.is_empty());
        assert!(
            pattern.notes(9).is_empty(),
            "missing channels have no notes"
        );
    }

    #[test]
    fn toggling_a_step() {
        let mut pattern = Pattern::new("Test", 1, 16);
        assert_eq!(pattern.toggle(0, 3), Some(true));
        assert!(pattern.is_on(0, 3));
        assert_eq!(pattern.notes(0), &[NoteEvent::step(3)]);
        assert_eq!(pattern.toggle(0, 3), Some(false));
        assert!(!pattern.is_on(0, 3));
        assert!(pattern.notes(0).is_empty());
    }

    #[test]
    fn outside_the_grid_is_ignored() {
        let mut pattern = Pattern::new("Test", 1, 16);
        assert_eq!(pattern.toggle(5, 0), None);
        assert_eq!(pattern.toggle(0, MAX_STEPS), None);
        assert!(!pattern.set(1, 0, true));
        assert!(!pattern.is_on(9, 9));
    }

    #[test]
    fn length_is_clamped_and_keeps_hidden_notes() {
        let mut pattern = Pattern::new("Test", 1, 32);
        pattern.set(0, 20, true);
        pattern.set_length(16);
        assert_eq!(pattern.length(), 16);
        pattern.set_length(32);
        assert!(pattern.is_on(0, 20), "step 20 should survive shortening");

        pattern.set_length(0);
        assert_eq!(pattern.length(), 1);
        pattern.set_length(1000);
        assert_eq!(pattern.length(), MAX_STEPS);
    }

    #[test]
    fn steps_and_piano_roll_notes() {
        let mut pattern = Pattern::new("Test", 1, 16);
        pattern.set(0, 0, true);
        assert!(pattern.is_step_row(0));
        let a4 = NoteEvent::new(Note::A4, 4, 2);
        assert_eq!(pattern.add_note(0, a4), Some(1));
        assert!(
            !pattern.is_step_row(0),
            "a real melody is no longer a step row"
        );
    }

    #[test]
    fn adding_notes() {
        let mut pattern = Pattern::new("Test", 1, 16);
        let note = NoteEvent::new(Note::C4, 2, 4);
        assert_eq!(pattern.add_note(0, note), Some(0));
        assert_eq!(pattern.add_note(0, note), None, "no duplicates");
        assert_eq!(pattern.add_note(1, note), None, "no such channel");
        assert_eq!(pattern.add_note(0, NoteEvent::new(Note::C4, 3, 0)), None);
        // A note that would run past the end is shortened.
        let long = NoteEvent::new(Note::A4, MAX_STEPS - 2, 10);
        let index = pattern.add_note(0, long).unwrap();
        assert_eq!(pattern.notes(0)[index].length, 2);
    }

    #[test]
    fn moving_and_removing_notes() {
        let mut pattern = Pattern::new("Test", 1, 16);
        pattern.add_note(0, NoteEvent::new(Note::C4, 0, 4));
        let moved = NoteEvent::new(Note::A4, 8, 2);
        assert!(pattern.replace_note(0, 0, moved));
        assert_eq!(pattern.notes(0), &[moved]);
        assert!(!pattern.replace_note(0, 5, moved));
        // A note cannot be moved on top of an identical one.
        pattern.add_note(0, NoteEvent::new(Note::C4, 0, 4));
        assert!(
            !pattern.replace_note(0, 1, moved),
            "A4 already starts on step 8"
        );
        pattern.remove_note(0, 1);
        assert_eq!(pattern.remove_note(0, 0), Some(moved));
        assert_eq!(pattern.remove_note(0, 0), None);
    }

    #[test]
    fn all_notes_lists_every_channel() {
        let mut pattern = Pattern::new("Test", 3, 16);
        pattern.set(0, 0, true);
        pattern.set(2, 4, true);
        let found: Vec<(usize, usize)> = pattern
            .all_notes()
            .map(|(channel, note)| (channel, note.start))
            .collect();
        assert_eq!(found, vec![(0, 0), (2, 4)]);
        assert!(!pattern.is_empty());
    }

    #[test]
    fn which_notes_sound_at_a_step() {
        let mut pattern = Pattern::new("Test", 1, 16);
        pattern.add_note(0, NoteEvent::new(Note::new(60), 0, 4));
        pattern.add_note(0, NoteEvent::new(Note::new(64), 2, 4));
        let at = |step| {
            pattern
                .sounding_at(0, step)
                .map(|n| n.midi())
                .collect::<Vec<_>>()
        };
        assert_eq!(at(0), vec![60]);
        assert_eq!(at(3), vec![60, 64]);
        assert_eq!(at(4), vec![64], "a note ends just before its end step");
        assert!(at(6).is_empty());
    }
}
