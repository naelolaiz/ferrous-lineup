//! How loud each channel and the master are: measured on the audio
//! thread, read by the mixer's meters.
//!
//! The audio thread keeps the loudest sample of each channel over a block
//! and then raises the shared peak to it, if it is higher; the UI takes
//! the peaks (setting them back to silence) once a frame. So no peak is
//! missed, however the blocks and the frames line up, and neither side
//! ever waits for the other.

use std::sync::atomic::{AtomicU32, Ordering};

use lineup_core::MAX_CHANNELS;

/// The peaks since the UI last took them, as the bits of `f32`s.
///
/// For numbers that are not negative, comparing their bits compares the
/// numbers, so `fetch_max` on the bits keeps the louder peak.
pub struct Levels {
    channels: [AtomicU32; MAX_CHANNELS],
    master: [AtomicU32; 2],
}

impl Default for Levels {
    fn default() -> Self {
        Levels {
            channels: std::array::from_fn(|_| AtomicU32::new(0)),
            master: std::array::from_fn(|_| AtomicU32::new(0)),
        }
    }
}

fn raise(slot: &AtomicU32, peak: f32) {
    // `peak > 0.0` is false for NaN too, which must not get in.
    if peak > 0.0 {
        slot.fetch_max(peak.to_bits(), Ordering::Relaxed);
    }
}

fn take(slot: &AtomicU32) -> f32 {
    f32::from_bits(slot.swap(0, Ordering::Relaxed))
}

impl Levels {
    /// Raises the peak of channel `index` to `peak`, if it is louder.
    pub fn raise_channel(&self, index: usize, peak: f32) {
        if let Some(slot) = self.channels.get(index) {
            raise(slot, peak);
        }
    }

    /// Raises the master's peaks, left and right.
    pub fn raise_master(&self, peaks: [f32; 2]) {
        for (slot, peak) in self.master.iter().zip(peaks) {
            raise(slot, peak);
        }
    }

    /// The loudest sample of channel `index` since the last call, after
    /// its volume and before its pan: 1.0 is full scale.
    pub fn take_channel(&self, index: usize) -> f32 {
        self.channels.get(index).map_or(0.0, take)
    }

    /// The loudest sample of the master since the last call, left and
    /// right, after the master volume: above 1.0 the output is rounded
    /// off (see the end of the audio thread's mix).
    pub fn take_master(&self) -> [f32; 2] {
        [take(&self.master[0]), take(&self.master[1])]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_loudest_peak_is_kept_until_taken() {
        let levels = Levels::default();
        levels.raise_channel(3, 0.25);
        levels.raise_channel(3, 0.5);
        levels.raise_channel(3, 0.125);
        levels.raise_channel(3, f32::NAN);
        levels.raise_channel(MAX_CHANNELS, 1.0);
        assert_eq!(levels.take_channel(3), 0.5);
        assert_eq!(levels.take_channel(3), 0.0);
        assert_eq!(levels.take_channel(MAX_CHANNELS), 0.0);
        levels.raise_master([0.25, 1.5]);
        levels.raise_master([0.75, 0.5]);
        assert_eq!(levels.take_master(), [0.75, 1.5]);
        assert_eq!(levels.take_master(), [0.0, 0.0]);
    }
}
