//! Small signal-processing building blocks shared by the instruments.

/// A value that glides toward its target instead of jumping.
///
/// Jumping a gain from one value to another in a single sample makes an
/// audible click. Moving a small fraction of the remaining distance on
/// every sample (a "one-pole smoother") removes it.
#[derive(Debug, Clone, Copy)]
pub struct Smoothed {
    current: f32,
    target: f32,
    coefficient: f32,
}

impl Smoothed {
    /// Starts at `value`, settling within roughly `seconds` after each change.
    pub fn new(value: f32, seconds: f32, sample_rate: u32) -> Self {
        Smoothed {
            current: value,
            target: value,
            coefficient: 1.0 - (-1.0 / (seconds * sample_rate as f32)).exp(),
        }
    }

    pub fn set(&mut self, target: f32) {
        self.target = target;
    }

    pub fn target(&self) -> f32 {
        self.target
    }

    /// Jumps straight to the target, with no smoothing.
    pub fn settle(&mut self) {
        self.current = self.target;
    }

    /// Moves one sample toward the target and returns the new value.
    pub fn next_value(&mut self) -> f32 {
        self.current += (self.target - self.current) * self.coefficient;
        // Close enough to hear no difference (-120 dB): there it stops,
        // instead of creeping on through numbers so small (denormals)
        // that many processors handle them very slowly.
        if (self.target - self.current).abs() < 1e-6 {
            self.current = self.target;
        }
        self.current
    }
}

/// A fast, repeatable source of white noise (the "xorshift" algorithm).
///
/// We do not need cryptographic randomness for a hi-hat, just numbers
/// that sound random, and this needs no allocation and no extra crate.
#[derive(Debug, Clone)]
pub struct Noise {
    state: u32,
}

impl Noise {
    pub fn new(seed: u32) -> Self {
        // Xorshift gets stuck at zero forever, so never start there.
        Noise { state: seed.max(1) }
    }

    /// A random sample between -1.0 and 1.0.
    pub fn next_sample(&mut self) -> f32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        (x as f32 / u32::MAX as f32) * 2.0 - 1.0
    }
}

/// A simple filter that removes low frequencies (a one-pole high-pass).
#[derive(Debug, Clone)]
pub struct HighPass {
    coefficient: f32,
    previous_input: f32,
    previous_output: f32,
}

impl HighPass {
    pub fn new(cutoff_hz: f32, sample_rate: u32) -> Self {
        let rc = 1.0 / (std::f32::consts::TAU * cutoff_hz);
        let dt = 1.0 / sample_rate as f32;
        HighPass {
            coefficient: rc / (rc + dt),
            previous_input: 0.0,
            previous_output: 0.0,
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let output = self.coefficient * (self.previous_output + input - self.previous_input);
        self.previous_input = input;
        self.previous_output = output;
        output
    }
}

/// A simple filter that removes high frequencies (a one-pole low-pass).
#[derive(Debug, Clone)]
pub struct LowPass {
    coefficient: f32,
    state: f32,
}

impl LowPass {
    pub fn new(cutoff_hz: f32, sample_rate: u32) -> Self {
        LowPass {
            coefficient: 1.0 - (-std::f32::consts::TAU * cutoff_hz / sample_rate as f32).exp(),
            state: 0.0,
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        self.state += (input - self.state) * self.coefficient;
        self.state
    }

    pub fn reset(&mut self) {
        self.state = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoothed_reaches_its_target() {
        let mut gain = Smoothed::new(0.0, 0.005, 48_000);
        gain.set(1.0);
        let first = gain.next_value();
        assert!(
            first > 0.0 && first < 0.01,
            "should start slowly, got {first}"
        );
        for _ in 0..4_800 {
            gain.next_value();
        }
        assert!((gain.next_value() - 1.0).abs() < 1e-3);
    }

    #[test]
    fn noise_stays_in_range_and_is_repeatable() {
        let mut a = Noise::new(42);
        let mut b = Noise::new(42);
        for _ in 0..10_000 {
            let x = a.next_sample();
            assert!((-1.0..=1.0).contains(&x));
            assert_eq!(x, b.next_sample());
        }
    }

    #[test]
    fn high_pass_blocks_a_constant_signal() {
        let mut filter = HighPass::new(100.0, 48_000);
        let mut out = 1.0;
        for _ in 0..48_000 {
            out = filter.process(1.0);
        }
        assert!(out.abs() < 1e-3, "DC should be removed, got {out}");
    }

    #[test]
    fn low_pass_lets_a_constant_signal_through() {
        let mut filter = LowPass::new(100.0, 48_000);
        let mut out = 0.0;
        for _ in 0..48_000 {
            out = filter.process(1.0);
        }
        assert!((out - 1.0).abs() < 1e-3);
    }
}
