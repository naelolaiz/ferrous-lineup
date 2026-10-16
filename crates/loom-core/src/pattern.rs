//! Patterns: which steps of which channels are switched on.

/// The longest pattern supported, in steps.
pub const MAX_STEPS: usize = 64;

/// The pattern lengths offered in the UI, in steps (sixteenth notes).
pub const PATTERN_LENGTHS: [usize; 3] = [16, 32, 64];

/// One row of steps, for one channel.
///
/// A fixed-size array instead of a `Vec`: every row has room for
/// [`MAX_STEPS`], so changing the pattern length never needs to allocate.
/// That matters because the audio thread keeps a copy of these rows.
pub type StepRow = [bool; MAX_STEPS];

/// A pattern: a grid of on/off steps, one row per channel.
///
/// Each step is a sixteenth note, so four steps make one beat and a
/// 16-step pattern is one bar of 4/4.
#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    length: usize,
    rows: Vec<StepRow>,
}

impl Pattern {
    /// Creates an empty pattern with `channels` rows and the given length.
    pub fn new(channels: usize, length: usize) -> Self {
        Pattern {
            length: length.clamp(1, MAX_STEPS),
            rows: vec![[false; MAX_STEPS]; channels],
        }
    }

    /// How many steps are played before the pattern loops.
    pub fn length(&self) -> usize {
        self.length
    }

    /// Changes the length. Steps beyond the new length are kept, so
    /// making the pattern longer again brings them back.
    pub fn set_length(&mut self, length: usize) {
        self.length = length.clamp(1, MAX_STEPS);
    }

    /// The number of rows (one per channel).
    pub fn rows(&self) -> usize {
        self.rows.len()
    }

    /// The steps of one channel, or `None` if there is no such row.
    pub fn row(&self, channel: usize) -> Option<&StepRow> {
        self.rows.get(channel)
    }

    /// Whether a step is on. Steps outside the grid count as off.
    pub fn is_on(&self, channel: usize, step: usize) -> bool {
        self.rows
            .get(channel)
            .and_then(|row| row.get(step))
            .copied()
            .unwrap_or(false)
    }

    /// Switches a step on or off. Returns `false` if it is outside the grid.
    pub fn set(&mut self, channel: usize, step: usize, on: bool) -> bool {
        match self.rows.get_mut(channel).and_then(|row| row.get_mut(step)) {
            Some(cell) => {
                *cell = on;
                true
            }
            None => false,
        }
    }

    /// Flips a step and returns its new value, or `None` if it is outside the grid.
    pub fn toggle(&mut self, channel: usize, step: usize) -> Option<bool> {
        let cell = self.rows.get_mut(channel)?.get_mut(step)?;
        *cell = !*cell;
        Some(*cell)
    }

    /// Adds an empty row at the bottom.
    pub(crate) fn push_row(&mut self) {
        self.rows.push([false; MAX_STEPS]);
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
        let pattern = Pattern::new(2, 16);
        assert_eq!(pattern.rows(), 2);
        assert_eq!(pattern.length(), 16);
        assert!(!pattern.is_on(0, 0));
    }

    #[test]
    fn toggling_a_step() {
        let mut pattern = Pattern::new(1, 16);
        assert_eq!(pattern.toggle(0, 3), Some(true));
        assert!(pattern.is_on(0, 3));
        assert_eq!(pattern.toggle(0, 3), Some(false));
        assert!(!pattern.is_on(0, 3));
    }

    #[test]
    fn outside_the_grid_is_ignored() {
        let mut pattern = Pattern::new(1, 16);
        assert_eq!(pattern.toggle(5, 0), None);
        assert_eq!(pattern.toggle(0, MAX_STEPS), None);
        assert!(!pattern.set(1, 0, true));
        assert!(!pattern.is_on(9, 9));
    }

    #[test]
    fn length_is_clamped_and_keeps_hidden_steps() {
        let mut pattern = Pattern::new(1, 32);
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
}
