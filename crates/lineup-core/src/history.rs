//! Undo and redo.
//!
//! [`History`] keeps whole copies ("snapshots") of whatever it is given,
//! usually a [`Project`](crate::Project). Before each change, the app
//! hands it a copy of the project as it was. Undo swaps the current
//! project with the last copy; redo swaps it back.
//!
//! Several small changes can make up one step: dragging a note changes it
//! on every frame of the drag, but undo should put it back where the drag
//! started, in one go. So the history keeps a step *open* until
//! [`History::close`] is called, and changes recorded while a step is open
//! join it instead of making new ones. The app closes the step when the
//! mouse button is released.

use std::collections::VecDeque;

/// How many steps are kept by default. Older steps are forgotten.
pub const DEFAULT_LIMIT: usize = 100;

/// One step that can be undone or redone: what it was called, and the
/// state to go back to.
#[derive(Debug, Clone)]
struct Step<T> {
    label: &'static str,
    state: T,
}

/// A list of earlier states to undo to, and later ones to redo to.
///
/// `T` can be any type: the history only moves values around, so it needs
/// nothing from `T` at all (not even `Clone`).
#[derive(Debug, Clone)]
pub struct History<T> {
    /// The oldest step is at the front, the newest at the back. A
    /// `VecDeque` can drop the oldest step from the front cheaply.
    undo: VecDeque<Step<T>>,
    redo: Vec<Step<T>>,
    limit: usize,
    /// Whether the newest undo step still takes in new changes.
    open: bool,
}

impl<T> History<T> {
    /// An empty history that keeps up to `limit` steps (at least one).
    pub fn new(limit: usize) -> Self {
        History {
            undo: VecDeque::new(),
            redo: Vec::new(),
            limit: limit.max(1),
            open: false,
        }
    }

    /// Records that something is about to change, or just changed:
    /// `before` is the state as it was before the change.
    ///
    /// If a step is open, the change joins it and `before` is not needed
    /// (the step already holds the state from before it started).
    /// Otherwise a new step starts. Either way, the redo list is cleared:
    /// after a new change, the old "future" can no longer be reached.
    pub fn record(&mut self, label: &'static str, before: T) {
        self.redo.clear();
        if self.open {
            return;
        }
        self.undo.push_back(Step {
            label,
            state: before,
        });
        if self.undo.len() > self.limit {
            self.undo.pop_front();
        }
        self.open = true;
    }

    /// Whether the newest step still takes in changes.
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Ends the open step, if there is one: the next change starts a new step.
    pub fn close(&mut self) {
        self.open = false;
    }

    /// Opens the newest step again, if it is called `label` and nothing
    /// was undone since, so the next change joins it: taps of a tempo,
    /// each a click of its own, make one step. Returns whether it did.
    pub fn reopen(&mut self, label: &'static str) -> bool {
        if self.redo.is_empty() && self.undo_label() == Some(label) {
            self.open = true;
        }
        self.open
    }

    /// Goes back one step. `current` is replaced with the state from
    /// before that step, and the state it had is kept for redo.
    ///
    /// Returns the step's label, or `None` (changing nothing) if there is
    /// nothing to undo.
    pub fn undo(&mut self, current: &mut T) -> Option<&'static str> {
        let mut step = self.undo.pop_back()?;
        // Now `current` holds the old state and `step` the newer one.
        std::mem::swap(current, &mut step.state);
        let label = step.label;
        self.redo.push(step);
        self.open = false;
        Some(label)
    }

    /// Goes forward again, after an undo. Returns the step's label, or
    /// `None` if there is nothing to redo.
    pub fn redo(&mut self, current: &mut T) -> Option<&'static str> {
        let mut step = self.redo.pop()?;
        std::mem::swap(current, &mut step.state);
        let label = step.label;
        self.undo.push_back(step);
        self.open = false;
        Some(label)
    }

    /// What undo would undo, if anything.
    pub fn undo_label(&self) -> Option<&'static str> {
        self.undo.back().map(|step| step.label)
    }

    /// What redo would redo, if anything.
    pub fn redo_label(&self) -> Option<&'static str> {
        self.redo.last().map(|step| step.label)
    }

    /// Forgets every step, for example after opening another file.
    pub fn clear(&mut self) {
        self.undo.clear();
        self.redo.clear();
        self.open = false;
    }
}

impl<T> Default for History<T> {
    fn default() -> Self {
        History::new(DEFAULT_LIMIT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Changes `value` to `new`, recording it as one step, as the app does.
    fn change(history: &mut History<i32>, value: &mut i32, new: i32) {
        history.record("change", *value);
        *value = new;
        history.close();
    }

    #[test]
    fn a_step_can_be_reopened_by_its_name() {
        let mut history = History::new(10);
        let mut tempo = 120;
        history.record("tap tempo", tempo);
        tempo = 100;
        history.close();
        assert!(history.reopen("tap tempo"));
        history.record("tap tempo", tempo);
        tempo = 96;
        history.close();
        assert!(!history.reopen("move clip"), "another step is not reopened");
        history.undo(&mut tempo);
        assert_eq!(tempo, 120, "both taps undo together");
        assert!(!history.reopen("tap tempo"), "nor one that was undone");
    }

    #[test]
    fn undo_and_redo_walk_through_the_steps() {
        let mut history = History::default();
        let mut value = 1;
        change(&mut history, &mut value, 2);
        change(&mut history, &mut value, 3);

        assert_eq!(history.undo(&mut value), Some("change"));
        assert_eq!(value, 2);
        assert_eq!(history.undo(&mut value), Some("change"));
        assert_eq!(value, 1);
        assert_eq!(history.undo(&mut value), None, "nothing left to undo");
        assert_eq!(value, 1, "a failed undo changes nothing");

        assert_eq!(history.redo(&mut value), Some("change"));
        assert_eq!(value, 2);
        assert_eq!(history.redo(&mut value), Some("change"));
        assert_eq!(value, 3);
        assert_eq!(history.redo(&mut value), None);
    }

    #[test]
    fn changes_join_the_open_step() {
        let mut history = History::default();
        let mut value = 0;
        // A drag: three changes before the mouse button is released.
        for new in [1, 2, 3] {
            history.record("drag", value);
            value = new;
        }
        history.close();
        history.undo(&mut value);
        assert_eq!(value, 0, "one undo goes back to before the drag");
        assert_eq!(history.undo_label(), None);
    }

    #[test]
    fn a_new_change_clears_redo() {
        let mut history = History::default();
        let mut value = 1;
        change(&mut history, &mut value, 2);
        history.undo(&mut value);
        assert_eq!(history.redo_label(), Some("change"));
        change(&mut history, &mut value, 5);
        assert_eq!(history.redo_label(), None);
        assert_eq!(history.redo(&mut value), None);
        assert_eq!(value, 5);
    }

    #[test]
    fn undo_closes_the_open_step() {
        let mut history = History::default();
        let mut value = 1;
        history.record("first", value);
        value = 2;
        // No close here: the step is still open when undo is pressed.
        history.undo(&mut value);
        history.record("second", value);
        value = 3;
        history.close();
        assert_eq!(history.undo_label(), Some("second"));
        history.undo(&mut value);
        assert_eq!(value, 1);
    }

    #[test]
    fn the_oldest_steps_are_forgotten() {
        let mut history = History::new(2);
        let mut value = 0;
        for new in 1..=5 {
            change(&mut history, &mut value, new);
        }
        while history.undo(&mut value).is_some() {}
        assert_eq!(value, 3, "only the last two steps were kept");
    }

    #[test]
    fn labels_name_the_next_step() {
        let mut history = History::default();
        let mut value = 0;
        history.record("Add note", value);
        value = 1;
        history.close();
        history.record("Move clip", value);
        value = 2;
        history.close();
        assert_eq!(history.undo_label(), Some("Move clip"));
        history.undo(&mut value);
        assert_eq!(history.undo_label(), Some("Add note"));
        assert_eq!(history.redo_label(), Some("Move clip"));
        assert_eq!(value, 1);
        history.clear();
        assert_eq!((history.undo_label(), history.redo_label()), (None, None));
    }
}
