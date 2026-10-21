//! The mixer's level meters: how loud each channel and the master are.
//!
//! The audio thread keeps the loudest sample of each channel and of the
//! master (see [`lineup_audio::Levels`]); once a frame the mixer takes
//! them. A meter jumps up to a new peak at once and then falls slowly,
//! so the eye can follow it.

use eframe::egui::{self, Rect, Sense, Ui, Vec2, vec2};
use lineup_audio::Levels;

use crate::theme;

/// The quietest level a meter shows, in decibels below full scale.
const FLOOR_DB: f32 = -48.0;

/// How fast a meter falls after a peak, in decibels a second.
const FALL_DB_PER_SECOND: f32 = 24.0;

/// From here up a meter turns amber: close to full scale.
const HOT_DB: f32 = -6.0;

/// What the meters show: 1.0 is full scale.
#[derive(Default)]
pub struct Meters {
    channels: Vec<f32>,
    master: [f32; 2],
}

impl Meters {
    /// Takes the peaks from `levels` for `channels` channels, `seconds`
    /// after the last time.
    pub fn update(&mut self, levels: &Levels, channels: usize, seconds: f32) {
        let fall = gain(-FALL_DB_PER_SECOND * seconds);
        self.channels.resize(channels, 0.0);
        for (index, shown) in self.channels.iter_mut().enumerate() {
            *shown = levels.take_channel(index).max(*shown * fall);
        }
        for (shown, peak) in self.master.iter_mut().zip(levels.take_master()) {
            *shown = peak.max(*shown * fall);
        }
    }

    /// The level of channel `index`.
    pub fn channel(&self, index: usize) -> f32 {
        self.channels.get(index).copied().unwrap_or(0.0)
    }

    /// The level of the master, left and right.
    pub fn master(&self) -> [f32; 2] {
        self.master
    }

    /// Whether any meter still shows something, so it must be drawn
    /// again as it falls.
    pub fn moving(&self) -> bool {
        self.channels
            .iter()
            .chain(&self.master)
            .any(|&level| lit(level) > 0.0)
    }
}

/// The gain of `decibels`: 0 dB is 1.0, -6 dB about a half.
fn gain(decibels: f32) -> f32 {
    10.0_f32.powf(decibels / 20.0)
}

fn decibels(level: f32) -> f32 {
    20.0 * level.log10()
}

/// How much of a meter is lit for `level`, from 0 (at the floor or
/// below) to 1 (full scale or more). Decibels, because the ear hears
/// loudness by ratios: every 6 dB down halves the level, wherever on the
/// meter it is.
fn lit(level: f32) -> f32 {
    if level > 0.0 {
        ((decibels(level) - FLOOR_DB) / -FLOOR_DB).clamp(0.0, 1.0)
    } else {
        0.0
    }
}

/// A meter for `level`, filling a bar `size` big from the left.
pub fn meter(ui: &mut Ui, level: f32, size: Vec2) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        painter.rect_filled(rect, 1.0, theme::LCD);
        let lit = lit(level);
        if lit > 0.0 {
            let color = if level >= 1.0 {
                theme::RED
            } else if decibels(level) >= HOT_DB {
                theme::ACCENT
            } else {
                theme::GREEN
            };
            let bar = Rect::from_min_size(rect.min, vec2(rect.width() * lit, rect.height()));
            painter.rect_filled(bar, 1.0, color);
        }
    }
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meters_jump_up_and_fall_slowly() {
        let levels = Levels::default();
        let mut meters = Meters::default();
        assert!(!meters.moving());
        levels.raise_channel(1, 0.5);
        meters.update(&levels, 2, 0.02);
        assert_eq!((meters.channel(0), meters.channel(1)), (0.0, 0.5));
        assert!(meters.moving());
        // Half a second later it has fallen 12 dB: to a quarter.
        meters.update(&levels, 2, 0.5);
        assert!((meters.channel(1) - 0.125).abs() < 0.001);
        // A louder peak shows at once.
        levels.raise_master([0.25, 1.0]);
        meters.update(&levels, 2, 0.02);
        assert_eq!(meters.master(), [0.25, 1.0]);
        // Once below the floor nothing is lit any more.
        meters.update(&levels, 2, 3.0);
        assert!(!meters.moving());
    }

    #[test]
    fn a_meter_is_lit_in_decibels() {
        assert_eq!(lit(0.0), 0.0);
        assert_eq!(lit(1.0), 1.0);
        assert_eq!(lit(2.0), 1.0);
        // -24 dB is halfway to the -48 dB floor.
        assert!((lit(gain(-24.0)) - 0.5).abs() < 1e-5);
    }
}
