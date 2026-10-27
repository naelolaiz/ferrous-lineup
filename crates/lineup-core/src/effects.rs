//! The effects of a mixer channel: an equalizer, a compressor and a
//! delay, as on each channel strip of a mixing desk.
//!
//! The mixer's reverb puts the whole band in one room. These shape each
//! instrument on its own, before it joins the others:
//!
//! - The **equalizer** (EQ) turns three bands of frequencies up or down:
//!   the lows, where a bass and a kick drum have their weight; the
//!   middle, where most instruments have their body; and the highs, the
//!   air and sparkle of cymbals and of a breathy voice. Turning the lows
//!   of a piano down leaves room for the bass; a little more of the
//!   highs brings a guitar forward.
//! - The **compressor** evens out how loud an instrument is: above its
//!   threshold it turns the sound down, the more the louder it gets (its
//!   ratio says how much), so a bass whose notes come out unevenly sits
//!   at one level under the band. Its gain brings the whole part back up
//!   afterwards, since the loud notes came down.
//! - The **delay** plays the sound again a little later, in time with
//!   the song (a dotted eighth note later, say), and again, each echo
//!   quieter than the one before: its feedback says how much of each
//!   echo comes back as the next one.
//!
//! They work in that order, as on most desks: EQ, compressor, delay, and
//! then the channel's volume, its pan and its reverb send.

use serde::{Deserialize, Serialize};

use crate::tempo::Tempo;

/// The effects of one channel. All of them leave the sound as it is when
/// a channel is new.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Effects {
    #[serde(default, skip_serializing_if = "Equalizer::is_flat")]
    pub eq: Equalizer,
    #[serde(default, skip_serializing_if = "Compressor::is_default")]
    pub compressor: Compressor,
    #[serde(default, skip_serializing_if = "Delay::is_default")]
    pub delay: Delay,
}

impl Effects {
    /// Whether they are as on a new channel. Files older than format 28
    /// can only say that.
    pub fn is_plain(&self) -> bool {
        *self == Effects::default()
    }

    /// Whether every setting is within its range.
    pub fn is_valid(&self) -> bool {
        let Effects {
            eq,
            compressor,
            delay,
        } = self;
        let eq_range = -Equalizer::RANGE..=Equalizer::RANGE;
        [eq.low, eq.mid, eq.high]
            .iter()
            .all(|gain| eq_range.contains(gain))
            && (Compressor::LOWEST_THRESHOLD..=0.0).contains(&compressor.threshold)
            && (1.0..=Compressor::MOST_RATIO).contains(&compressor.ratio)
            && (0.0..=Compressor::MOST_GAIN).contains(&compressor.gain)
            && (1..=Delay::MOST_STEPS).contains(&delay.steps)
            && (0.0..=Delay::MOST_FEEDBACK).contains(&delay.feedback)
            && (0.0..=1.0).contains(&delay.mix)
    }

    /// The effects that change the sound, in a few words for the mixer:
    /// "EQ, delay". Empty when none does.
    pub fn summary(&self) -> String {
        let mut on = Vec::new();
        if !self.eq.is_flat() {
            on.push("EQ");
        }
        if self.compressor.is_on() {
            on.push("comp");
        }
        if self.delay.is_on() {
            on.push("delay");
        }
        on.join(", ")
    }
}

/// Three bands of frequencies, each turned up or down by its gain in
/// decibels: 0 leaves the band as it is, +6 makes it about twice as
/// strong, -6 half.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Equalizer {
    /// Below about [`Equalizer::LOW_HZ`]: a low shelf.
    #[serde(default)]
    pub low: f32,
    /// Around [`Equalizer::MID_HZ`]: a bell, about an octave and a half
    /// wide.
    #[serde(default)]
    pub mid: f32,
    /// Above about [`Equalizer::HIGH_HZ`]: a high shelf.
    #[serde(default)]
    pub high: f32,
}

impl Equalizer {
    /// How far a band can go up or down, in decibels.
    pub const RANGE: f32 = 12.0;
    /// Where the low shelf turns, in hertz.
    pub const LOW_HZ: f32 = 200.0;
    /// The middle of the middle band, in hertz.
    pub const MID_HZ: f32 = 1_000.0;
    /// Where the high shelf turns, in hertz.
    pub const HIGH_HZ: f32 = 4_000.0;

    /// Whether it leaves every band as it is.
    pub fn is_flat(&self) -> bool {
        self.low == 0.0 && self.mid == 0.0 && self.high == 0.0
    }
}

/// Turns a channel down when it gets loud.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Compressor {
    /// The level it starts turning down at, in decibels below full scale
    /// (0 dB is the loudest a sample can be).
    pub threshold: f32,
    /// How much it turns down above the threshold: at 4 (written 4:1), a
    /// sound 8 dB over the threshold comes out 2 dB over it. 1 turns
    /// nothing down: the compressor is off.
    pub ratio: f32,
    /// How much it turns the whole channel up afterwards, in decibels:
    /// its "make-up gain".
    #[serde(default)]
    pub gain: f32,
}

impl Default for Compressor {
    /// Off, with its threshold where a compressor on a single instrument
    /// usually starts.
    fn default() -> Self {
        Compressor {
            threshold: -18.0,
            ratio: 1.0,
            gain: 0.0,
        }
    }
}

impl Compressor {
    /// The lowest threshold, in decibels.
    pub const LOWEST_THRESHOLD: f32 = -40.0;
    /// The highest ratio. At 20:1 it is almost a limiter: nothing gets
    /// much over the threshold.
    pub const MOST_RATIO: f32 = 20.0;
    /// The most make-up gain, in decibels.
    pub const MOST_GAIN: f32 = 24.0;
    /// How fast it turns down once a sound is over the threshold.
    pub const ATTACK_SECONDS: f32 = 0.01;
    /// How fast it turns back up once a sound is under it again.
    pub const RELEASE_SECONDS: f32 = 0.15;

    /// Whether it changes the sound: a ratio over 1, or some gain.
    pub fn is_on(&self) -> bool {
        self.ratio > 1.0 || self.gain != 0.0
    }

    fn is_default(&self) -> bool {
        *self == Compressor::default()
    }

    /// How many decibels it turns a sound at `level` decibels down by:
    /// nothing at or under the threshold, and above it all of the excess
    /// but one part in `ratio`.
    pub fn reduction(&self, level: f32) -> f32 {
        let over = level - self.threshold;
        if over <= 0.0 || self.ratio <= 1.0 {
            0.0
        } else {
            over * (1.0 - 1.0 / self.ratio)
        }
    }
}

/// Echoes of a channel, in time with the song.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Delay {
    /// How long after the sound each echo comes, in steps (sixteenth
    /// notes): 3 is a dotted eighth note.
    pub steps: u8,
    /// How loud each echo is next to the one before it, from 0.0 (one
    /// echo only) to [`Delay::MOST_FEEDBACK`].
    pub feedback: f32,
    /// How loud the echoes are, from 0.0 (none: the delay is off) to 1.0
    /// (the first as loud as the sound itself).
    pub mix: f32,
}

impl Default for Delay {
    /// Off, set to a dotted eighth note with a few echoes, the delay
    /// heard on so many guitars and voices.
    fn default() -> Self {
        Delay {
            steps: 3,
            feedback: 0.35,
            mix: 0.0,
        }
    }
}

impl Delay {
    /// The longest time between echoes, in steps: a half note.
    pub const MOST_STEPS: u8 = 8;
    /// The most feedback: under 1, so the echoes always die away.
    pub const MOST_FEEDBACK: f32 = 0.9;
    /// The longest time between echoes, in seconds, however slow the
    /// song: the audio thread keeps this much of each delayed channel.
    pub const MOST_SECONDS: f32 = 2.0;
    /// The times the mixer offers, in steps, with their names.
    pub const TIMES: [(u8, &'static str); 6] = [
        (1, "1/16"),
        (2, "1/8"),
        (3, "dotted 1/8"),
        (4, "1/4"),
        (6, "dotted 1/4"),
        (8, "1/2"),
    ];

    /// Whether it plays any echoes.
    pub fn is_on(&self) -> bool {
        self.mix > 0.0
    }

    fn is_default(&self) -> bool {
        *self == Delay::default()
    }

    /// The name of its time: "dotted 1/8".
    pub fn time_name(&self) -> String {
        Delay::TIMES
            .iter()
            .find(|(steps, _)| *steps == self.steps)
            .map_or_else(
                || format!("{} steps", self.steps),
                |(_, name)| name.to_string(),
            )
    }

    /// The time between echoes at `tempo`, in seconds: its steps, but
    /// never more than [`Delay::MOST_SECONDS`].
    pub fn seconds(&self, tempo: Tempo) -> f32 {
        let step = tempo.seconds_per_beat() / crate::STEPS_PER_BEAT as f32;
        (f32::from(self.steps) * step).min(Delay::MOST_SECONDS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_channel_has_plain_effects() {
        let effects = Effects::default();
        assert!(effects.is_plain());
        assert!(effects.is_valid());
        assert_eq!(effects.summary(), "");
        assert!(!effects.compressor.is_on());
        assert!(!effects.delay.is_on());
        // Saved, it leaves nothing behind.
        assert_eq!(ron::to_string(&effects).unwrap(), "()");
    }

    #[test]
    fn the_compressor_turns_down_what_goes_over_its_threshold() {
        let compressor = Compressor {
            threshold: -20.0,
            ratio: 4.0,
            gain: 0.0,
        };
        assert_eq!(compressor.reduction(-30.0), 0.0);
        // 8 dB over comes out 2 dB over: 6 dB down.
        assert_eq!(compressor.reduction(-12.0), 6.0);
        let off = Compressor::default();
        assert_eq!(off.reduction(0.0), 0.0);
    }

    #[test]
    fn the_delay_keeps_time_with_the_song() {
        let delay = Delay {
            mix: 0.4,
            ..Delay::default()
        };
        // A dotted eighth at 120 BPM: three sixteenths of 0.125 s.
        assert!((delay.seconds(Tempo::new(120.0)) - 0.375).abs() < 1e-6);
        assert_eq!(delay.time_name(), "dotted 1/8");
        // A half note at 20 BPM would be 6 s: it stops at 2.
        let long = Delay { steps: 8, ..delay };
        assert_eq!(long.seconds(Tempo::new(20.0)), Delay::MOST_SECONDS);
        let effects = Effects {
            delay,
            ..Effects::default()
        };
        assert_eq!(effects.summary(), "delay");
        assert!(!effects.is_plain());
    }

    #[test]
    fn settings_out_of_range_are_not_valid() {
        let mut effects = Effects::default();
        effects.eq.low = 13.0;
        assert!(!effects.is_valid());
        effects.eq.low = -3.0;
        effects.delay.feedback = 1.0;
        assert!(!effects.is_valid());
        effects.delay.feedback = 0.5;
        effects.compressor.ratio = 0.5;
        assert!(!effects.is_valid());
        effects.compressor.ratio = 3.0;
        assert!(effects.is_valid());
        assert_eq!(effects.summary(), "EQ, comp");
    }
}
