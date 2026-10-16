//! Turning tempo into step timing, one audio frame at a time.

use crate::Tempo;

/// How many steps make one beat. Steps are sixteenth notes and a beat is
/// a quarter note, so there are four.
pub const STEPS_PER_BEAT: u32 = 4;

/// Counts audio frames and says when each step of a pattern starts.
///
/// The audio thread calls [`Sequencer::tick`] once per frame. The
/// sequencer keeps a fractional frame count, so even when a step lasts
/// a non-whole number of frames (5512.5 at 130 BPM and 44.1 kHz, say)
/// the timing never drifts: the remainder is carried to the next step.
#[derive(Debug, Clone)]
pub struct Sequencer {
    sample_rate: u32,
    frames_per_step: f64,
    /// Frames left until the next step starts. When it reaches zero or
    /// less, the next step fires.
    frames_until_next: f64,
    next_step: usize,
    length: usize,
}

impl Sequencer {
    /// Creates a sequencer that will fire step 0 on the first tick.
    pub fn new(tempo: Tempo, sample_rate: u32, length: usize) -> Self {
        Sequencer {
            sample_rate,
            frames_per_step: frames_per_step(tempo, sample_rate),
            frames_until_next: 0.0,
            next_step: 0,
            length: length.max(1),
        }
    }

    /// Changes the tempo. The step that is already playing keeps the
    /// time it has left, measured in frames; the new tempo applies from
    /// the next step on.
    pub fn set_tempo(&mut self, tempo: Tempo) {
        self.frames_per_step = frames_per_step(tempo, self.sample_rate);
    }

    /// Changes the pattern length. If the next step is now past the end,
    /// playback wraps to step 0.
    pub fn set_length(&mut self, length: usize) {
        self.length = length.max(1);
        if self.next_step >= self.length {
            self.next_step = 0;
        }
    }

    /// How many frames one step lasts at the current tempo.
    pub fn step_frames(&self) -> f64 {
        self.frames_per_step
    }

    /// How many steps are played before the pattern loops.
    pub fn length(&self) -> usize {
        self.length
    }

    /// Where playback is, in steps, with the fraction of the step that
    /// has passed: 4.5 is halfway through step 4. Recording uses it to
    /// place a key played between two steps on the nearer one.
    ///
    /// Before the first tick, and right after a reset, this is the start
    /// of the step that is about to fire.
    pub fn position(&self) -> f64 {
        if self.frames_until_next <= 0.0 {
            return self.next_step as f64;
        }
        let current = (self.next_step + self.length - 1) % self.length;
        let passed = 1.0 - self.frames_until_next / self.frames_per_step;
        current as f64 + passed.clamp(0.0, 1.0)
    }

    /// Goes back to the start, so step 0 fires on the next tick.
    pub fn reset(&mut self) {
        self.frames_until_next = 0.0;
        self.next_step = 0;
    }

    /// Jumps to `step`, so it fires on the next tick. A step past the end
    /// wraps around, as playback would.
    pub fn start_at(&mut self, step: usize) {
        self.frames_until_next = 0.0;
        self.next_step = step % self.length;
    }

    /// Advances by one frame. Returns `Some(step)` if `step` starts on
    /// this frame, `None` otherwise.
    pub fn tick(&mut self) -> Option<usize> {
        let fired = if self.frames_until_next <= 0.0 {
            let step = self.next_step;
            self.next_step = (self.next_step + 1) % self.length;
            self.frames_until_next += self.frames_per_step;
            Some(step)
        } else {
            None
        };
        self.frames_until_next -= 1.0;
        fired
    }
}

/// How many audio frames one step lasts.
pub fn frames_per_step(tempo: Tempo, sample_rate: u32) -> f64 {
    tempo.samples_per_beat(sample_rate) / STEPS_PER_BEAT as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ticks `frames` times and returns `(frame, step)` for every step fired.
    fn run(sequencer: &mut Sequencer, frames: usize) -> Vec<(usize, usize)> {
        (0..frames)
            .filter_map(|frame| sequencer.tick().map(|step| (frame, step)))
            .collect()
    }

    #[test]
    fn step_length_at_120_bpm() {
        // 120 BPM = 2 beats per second = 8 steps per second.
        assert_eq!(frames_per_step(Tempo::new(120.0), 48_000), 6_000.0);
    }

    #[test]
    fn fires_steps_on_time_and_loops() {
        let mut sequencer = Sequencer::new(Tempo::new(120.0), 48_000, 4);
        let fired = run(&mut sequencer, 30_000);
        assert_eq!(
            fired,
            vec![(0, 0), (6_000, 1), (12_000, 2), (18_000, 3), (24_000, 0)]
        );
    }

    #[test]
    fn fractional_step_lengths_do_not_drift() {
        // 130 BPM at 44.1 kHz: 5088.46... frames per step.
        let mut sequencer = Sequencer::new(Tempo::new(130.0), 44_100, 16);
        let fired = run(&mut sequencer, 44_100 * 60);
        // In one minute at 130 BPM there are exactly 130 * 4 = 520 steps.
        assert_eq!(fired.len(), 520);
        let (last_frame, _) = *fired.last().unwrap();
        let expected = 519.0 * frames_per_step(Tempo::new(130.0), 44_100);
        assert!((last_frame as f64 - expected).abs() <= 1.0);
    }

    #[test]
    fn reset_starts_again_at_step_zero() {
        let mut sequencer = Sequencer::new(Tempo::new(120.0), 48_000, 16);
        run(&mut sequencer, 10_000);
        sequencer.reset();
        assert_eq!(sequencer.tick(), Some(0));
    }

    #[test]
    fn start_at_jumps_and_wraps() {
        let mut sequencer = Sequencer::new(Tempo::new(120.0), 48_000, 16);
        run(&mut sequencer, 10_000);
        sequencer.start_at(5);
        assert_eq!(sequencer.tick(), Some(5));
        sequencer.start_at(18);
        assert_eq!(sequencer.tick(), Some(2));
    }

    #[test]
    fn position_counts_fractions_of_a_step() {
        let mut sequencer = Sequencer::new(Tempo::new(120.0), 48_000, 4);
        assert_eq!(sequencer.position(), 0.0);
        run(&mut sequencer, 1_500); // a quarter of step 0
        assert_eq!(sequencer.position(), 0.25);
        run(&mut sequencer, 6_000 * 3); // a quarter of step 3
        assert_eq!(sequencer.position(), 3.25);
        run(&mut sequencer, 4_500); // just before step 0 comes round again
        assert_eq!(sequencer.position(), 0.0);
        run(&mut sequencer, 3_000);
        assert_eq!(sequencer.position(), 0.5);
    }

    #[test]
    fn shortening_the_pattern_wraps() {
        let mut sequencer = Sequencer::new(Tempo::new(120.0), 48_000, 16);
        run(&mut sequencer, 6_000 * 10 + 1); // steps 0..=10 fired, next is 11
        sequencer.set_length(8);
        let fired = run(&mut sequencer, 6_000);
        assert_eq!(fired.last().map(|&(_, step)| step), Some(0));
    }
}
