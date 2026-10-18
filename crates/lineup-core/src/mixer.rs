//! Mixer settings shared by live playback, project files and WAV export.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ReverbSettings {
    pub enabled: bool,
    /// Wet fraction, from 0 to 1.
    pub mix: f32,
    /// Approximate time to decay by 60 dB, in seconds.
    pub decay: f32,
    /// High frequency absorption, from 0 to 1.
    pub damping: f32,
    pub pre_delay_ms: f32,
}

impl Default for ReverbSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            mix: 0.2,
            decay: 1.8,
            damping: 0.5,
            pre_delay_ms: 15.0,
        }
    }
}

impl ReverbSettings {
    pub fn valid(&self) -> bool {
        (0.0..=1.0).contains(&self.mix)
            && (0.1..=10.0).contains(&self.decay)
            && (0.0..=1.0).contains(&self.damping)
            && (0.0..=200.0).contains(&self.pre_delay_ms)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Mixer {
    pub master: f32,
    pub reverb: ReverbSettings,
    pub solo: Option<usize>,
}

impl Default for Mixer {
    fn default() -> Self {
        Self {
            master: 0.8,
            reverb: ReverbSettings::default(),
            solo: None,
        }
    }
}

impl Mixer {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
    pub fn valid(&self) -> bool {
        (0.0..=1.0).contains(&self.master) && self.reverb.valid()
    }
}
