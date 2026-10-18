//! A stereo Schroeder reverb. All delay memory is allocated before audio
//! starts; changing controls and processing samples never allocates.

use crate::dsp::Smoothed;
use lineup_core::ReverbSettings;

struct Delay {
    samples: Vec<f32>,
    cursor: usize,
}
impl Delay {
    fn new(length: usize) -> Self {
        Self {
            samples: vec![0.0; length.max(1)],
            cursor: 0,
        }
    }
    fn read(&self) -> f32 {
        self.samples[self.cursor]
    }
    fn write(&mut self, value: f32) {
        self.samples[self.cursor] = if value.abs() < 1e-20 { 0.0 } else { value };
        self.cursor = (self.cursor + 1) % self.samples.len();
    }
    fn clear(&mut self) {
        self.samples.fill(0.0);
        self.cursor = 0;
    }
}

struct Comb {
    delay: Delay,
    feedback: f32,
    filtered: f32,
}
impl Comb {
    fn next(&mut self, input: f32, damping: f32) -> f32 {
        let out = self.delay.read();
        self.filtered = out * (1.0 - damping) + self.filtered * damping;
        self.delay.write(input + self.filtered * self.feedback);
        out
    }
}

struct Side {
    combs: [Comb; 4],
    diffusers: [Delay; 3],
}
impl Side {
    fn new(rate: u32, spread: usize) -> Self {
        let length =
            |samples| ((samples + spread) as f32 * rate as f32 / 48_000.0).round() as usize;
        Self {
            combs: [1493, 1601, 1747, 1867].map(|samples| Comb {
                delay: Delay::new(length(samples)),
                feedback: 0.0,
                filtered: 0.0,
            }),
            diffusers: [241, 503, 719].map(|samples| Delay::new(length(samples))),
        }
    }
    fn configure(&mut self, rate: u32, decay: f32) {
        for comb in &mut self.combs {
            comb.feedback =
                10.0_f32.powf(-3.0 * comb.delay.samples.len() as f32 / (rate as f32 * decay));
        }
    }
    fn next(&mut self, input: f32, damping: f32) -> f32 {
        let mut sample = self
            .combs
            .iter_mut()
            .map(|comb| comb.next(input, damping))
            .sum::<f32>()
            * 0.25;
        for delay in &mut self.diffusers {
            let delayed = delay.read();
            let out = delayed - 0.5 * sample;
            delay.write(sample + 0.5 * out);
            sample = out;
        }
        sample
    }
    fn clear(&mut self) {
        for comb in &mut self.combs {
            comb.delay.clear();
            comb.filtered = 0.0;
        }
        for delay in &mut self.diffusers {
            delay.clear();
        }
    }
}

pub(crate) struct Reverb {
    rate: u32,
    settings: ReverbSettings,
    wet: Smoothed,
    pre_delay: Delay,
    pre_frames: usize,
    sides: [Side; 2],
}
impl Reverb {
    pub fn new(rate: u32) -> Self {
        let mut reverb = Self {
            rate,
            settings: ReverbSettings::default(),
            wet: Smoothed::new(0.0, 0.02, rate),
            pre_delay: Delay::new((rate as usize / 5) + 1),
            pre_frames: 0,
            sides: [Side::new(rate, 0), Side::new(rate, 37)],
        };
        reverb.configure(ReverbSettings::default());
        reverb
    }
    pub fn configure(&mut self, settings: ReverbSettings) {
        if !settings.valid() {
            return;
        }
        self.settings = settings;
        self.wet
            .set(if settings.enabled { settings.mix } else { 0.0 });
        self.pre_frames = (settings.pre_delay_ms * self.rate as f32 / 1000.0) as usize;
        for side in &mut self.sides {
            side.configure(self.rate, settings.decay);
        }
    }
    pub fn settle(&mut self) {
        self.wet.settle();
    }
    pub fn clear(&mut self) {
        self.pre_delay.clear();
        for side in &mut self.sides {
            side.clear();
        }
    }
    pub fn next(&mut self, dry: f32) -> [f32; 2] {
        let wet = self.wet.next_value();
        let size = self.pre_delay.samples.len();
        let delayed = if self.pre_frames == 0 {
            dry
        } else {
            self.pre_delay.samples
                [(self.pre_delay.cursor + size - self.pre_frames.min(size - 1)) % size]
        };
        self.pre_delay
            .write(if self.settings.enabled { dry } else { 0.0 });
        self.sides
            .each_mut()
            .map(|side| dry * (1.0 - wet) + side.next(delayed, self.settings.damping * 0.9) * wet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bypass_preserves_samples_exactly() {
        let mut reverb = Reverb::new(48_000);
        for dry in [0.0, 0.2, -0.3, 1.0] {
            assert_eq!(reverb.next(dry), [dry; 2]);
        }
    }
    #[test]
    fn impulse_has_a_stereo_tail_that_decays_and_can_be_cleared() {
        for rate in [22_050, 48_000, 96_000] {
            let mut reverb = Reverb::new(rate);
            reverb.configure(ReverbSettings {
                enabled: true,
                mix: 1.0,
                decay: 0.5,
                damping: 0.0,
                pre_delay_ms: 20.0,
            });
            reverb.settle();
            assert_eq!(reverb.next(1.0), [0.0; 2]);
            let mut early = 0.0;
            let mut late = 0.0;
            let mut stereo = false;
            for frame in 1..rate * 2 {
                let out = reverb.next(0.0);
                assert!(out.iter().all(|x| x.is_finite() && x.abs() < 1.0));
                if frame < rate / 100 {
                    assert_eq!(out, [0.0; 2]);
                }
                let energy = out.iter().map(|x| x * x).sum::<f32>();
                if frame < rate / 2 {
                    early += energy;
                } else if frame > rate {
                    late += energy;
                }
                stereo |= out[0] != out[1];
            }
            assert!(stereo && early > 0.01 && late < early * 0.001);
            reverb.clear();
            for _ in 0..rate {
                assert_eq!(reverb.next(0.0), [0.0; 2]);
            }
        }
    }
}
