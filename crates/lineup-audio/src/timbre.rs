//! The sounds of the Loom Osc: one for each kind of General MIDI
//! instrument.
//!
//! The Loom Osc is a very small synth: one oscillator per note, a soft
//! low-pass filter and a volume envelope. It cannot sound like a violin,
//! but it can sound more like one than like a piano. Three things do most
//! of the work:
//!
//! - The *wave*: a sawtooth is bright and buzzy, like bowed strings and
//!   brass; a square is hollow, like a clarinet or an organ; a triangle is
//!   soft, like a flute or a voice; a sine is pure, like a bell or a
//!   vibraphone.
//! - The *envelope*: a piano, a guitar or a plucked string starts at once
//!   and dies away even while the key is held (no sustain); strings, brass
//!   and voices swell in (a slow attack) and keep going.
//! - The *filter*: a low cutoff makes a dark sound (a bass), a high one a
//!   bright sound (a harpsichord).
//!
//! A channel with no General MIDI program keeps the Loom Osc's own sound.

/// The shape of one cycle of the oscillator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wave {
    Saw,
    Square,
    Triangle,
    Sine,
}

impl Wave {
    /// The wave's value at `phase`, from 0 to 1 through one cycle.
    pub fn at(self, phase: f32) -> f32 {
        match self {
            // Ramps from -1 to 1.
            Wave::Saw => phase * 2.0 - 1.0,
            // 1 for the first half of the cycle, -1 for the second.
            Wave::Square => {
                if phase < 0.5 {
                    1.0
                } else {
                    -1.0
                }
            }
            // Up from -1 to 1 and back down.
            Wave::Triangle => 1.0 - 4.0 * (phase - 0.5).abs(),
            Wave::Sine => (phase * std::f32::consts::TAU).sin(),
        }
    }

    /// How much to turn the wave up or down so that all of them sound
    /// about as loud: a square is the loudest, a filtered triangle the
    /// softest.
    fn gain(self) -> f32 {
        match self {
            Wave::Saw => 1.0,
            Wave::Square => 0.55,
            Wave::Triangle => 1.1,
            Wave::Sine => 0.9,
        }
    }
}

/// Everything that makes one sound of the Loom Osc different from another.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Timbre {
    pub wave: Wave,
    /// Seconds to reach full volume.
    pub attack: f32,
    /// How quickly the volume falls to the sustain level (time constant,
    /// in seconds).
    pub decay: f32,
    /// The volume a held note settles at, 0 to 1. 0 for a sound that dies
    /// away by itself, like a piano or a plucked string.
    pub sustain: f32,
    /// How quickly a note fades after it is let go (time constant).
    pub release: f32,
    /// The low-pass filter's cutoff, in Hz: lower is darker.
    pub cutoff: f32,
    /// How loud, compared with the Loom Osc's own sound.
    pub gain: f32,
}

impl Timbre {
    /// The Loom Osc's own sound: a bright, short-decaying sawtooth.
    pub const LOOM_OSC: Timbre = Timbre {
        wave: Wave::Saw,
        attack: 0.004,
        decay: 0.3,
        sustain: 0.45,
        release: 0.08,
        cutoff: 2_500.0,
        gain: 1.0,
    };

    const fn new(wave: Wave, attack: f32, decay: f32, sustain: f32, release: f32) -> Timbre {
        Timbre {
            wave,
            attack,
            decay,
            sustain,
            release,
            cutoff: 2_500.0,
            gain: 1.0,
        }
    }

    const fn cutoff(self, cutoff: f32) -> Timbre {
        Timbre { cutoff, ..self }
    }

    /// Struck or plucked: starts at once and dies away over `decay`.
    const fn struck(wave: Wave, decay: f32, release: f32) -> Timbre {
        Timbre::new(wave, 0.003, decay, 0.0, release)
    }

    /// Bowed, blown or sung: swells in over `attack` and keeps going.
    const fn held(wave: Wave, attack: f32, release: f32) -> Timbre {
        Timbre::new(wave, attack, 0.3, 0.85, release)
    }

    /// The sound closest to General MIDI program `program`, or the Loom
    /// Osc's own for `None`.
    pub fn for_program(program: Option<u8>) -> Timbre {
        let Some(program) = program else {
            return Timbre::LOOM_OSC;
        };
        let timbre = match program.min(127) {
            // Pianos.
            0..=3 => Timbre::struck(Wave::Saw, 0.9, 0.25).cutoff(1_800.0),
            4 | 5 => Timbre::struck(Wave::Sine, 1.2, 0.3),
            6 => Timbre::struck(Wave::Saw, 0.6, 0.1).cutoff(4_000.0),
            7 => Timbre::new(Wave::Square, 0.003, 0.4, 0.1, 0.05).cutoff(3_000.0),
            // Chromatic percussion: celesta, vibraphone, marimba, bells.
            8..=15 => Timbre::struck(Wave::Sine, 0.6, 0.4),
            // Organs, and the reed organs: accordion and harmonica.
            16..=20 => Timbre::new(Wave::Square, 0.01, 0.1, 1.0, 0.05).cutoff(1_800.0),
            21..=23 => Timbre::held(Wave::Square, 0.03, 0.08).cutoff(2_600.0),
            // Guitars, the electric ones with sustain.
            24..=28 | 31 => Timbre::struck(Wave::Saw, 0.7, 0.15).cutoff(2_200.0),
            29 | 30 => Timbre::new(Wave::Square, 0.003, 0.4, 0.6, 0.1).cutoff(3_000.0),
            // Basses: the double bass plucked, the others held.
            32 => Timbre::struck(Wave::Saw, 0.5, 0.1).cutoff(700.0),
            33..=37 => Timbre::new(Wave::Saw, 0.005, 0.4, 0.4, 0.08).cutoff(700.0),
            38 | 39 => Timbre::new(Wave::Saw, 0.005, 0.3, 0.7, 0.08).cutoff(1_200.0),
            // Solo strings, then plucked strings and the harp.
            40..=44 => Timbre::held(Wave::Saw, 0.08, 0.25).cutoff(3_000.0),
            45 => Timbre::struck(Wave::Triangle, 0.25, 0.1),
            46 => Timbre::struck(Wave::Triangle, 0.9, 0.5),
            47 => Timbre::struck(Wave::Sine, 0.6, 0.3),
            // Ensembles: strings, then voices.
            48..=51 => Timbre::held(Wave::Saw, 0.2, 0.4),
            52..=54 => Timbre::held(Wave::Triangle, 0.15, 0.3).cutoff(2_000.0),
            55 => Timbre::struck(Wave::Saw, 0.3, 0.1),
            // Brass.
            56..=63 => Timbre::held(Wave::Saw, 0.04, 0.1).cutoff(2_000.0),
            // Reeds: saxophones, oboe, bassoon, clarinet.
            64..=71 => Timbre::held(Wave::Square, 0.03, 0.08).cutoff(1_800.0),
            // Pipes: flutes, recorder, ocarina.
            72..=79 => Timbre::held(Wave::Triangle, 0.05, 0.1).cutoff(3_000.0),
            // Synth leads: the square one is a square.
            80 => Timbre {
                wave: Wave::Square,
                ..Timbre::LOOM_OSC
            },
            81..=87 => Timbre::LOOM_OSC,
            // Pads and atmospheres: slow in, slow out.
            88..=103 => Timbre::held(Wave::Saw, 0.4, 0.6).cutoff(1_500.0),
            // From other lands: bagpipe, fiddle and shanai are held, the
            // rest (sitar, banjo, shamisen, koto, kalimba) plucked.
            109 | 111 => Timbre::held(Wave::Square, 0.03, 0.08).cutoff(2_200.0),
            110 => Timbre::held(Wave::Saw, 0.05, 0.2).cutoff(3_000.0),
            104..=108 => Timbre::struck(Wave::Saw, 0.6, 0.15).cutoff(3_000.0),
            // Percussive sounds with a pitch: bells, steel drums, toms.
            112..=119 => Timbre::struck(Wave::Sine, 0.3, 0.1),
            // Sound effects have no tune to play; the Loom Osc stands in.
            _ => Timbre::LOOM_OSC,
        };
        Timbre {
            gain: timbre.gain * timbre.wave.gain(),
            ..timbre
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lineup_core::general_midi;

    #[test]
    fn no_program_keeps_the_loom_osc() {
        assert_eq!(Timbre::for_program(None), Timbre::LOOM_OSC);
    }

    #[test]
    fn plucked_instruments_die_away_and_bowed_ones_keep_going() {
        for program in [
            general_midi::ACOUSTIC_GRAND_PIANO,
            general_midi::NYLON_GUITAR,
            general_midi::HARP,
            general_midi::PIZZICATO_STRINGS,
        ] {
            assert_eq!(Timbre::for_program(Some(program)).sustain, 0.0, "{program}");
        }
        for program in [
            general_midi::VIOLIN,
            general_midi::STRING_ENSEMBLE,
            general_midi::CHOIR_AAHS,
            general_midi::FLUTE,
            general_midi::CHURCH_ORGAN,
        ] {
            assert!(
                Timbre::for_program(Some(program)).sustain > 0.5,
                "{program}"
            );
        }
        // Strings and voices swell in more slowly than a piano.
        let attack = |program| Timbre::for_program(Some(program)).attack;
        assert!(attack(general_midi::VIOLIN) > 10.0 * attack(general_midi::ACOUSTIC_GRAND_PIANO));
    }

    #[test]
    fn every_wave_stays_between_minus_one_and_one() {
        for wave in [Wave::Saw, Wave::Square, Wave::Triangle, Wave::Sine] {
            for i in 0..100 {
                let value = wave.at(i as f32 / 100.0);
                assert!((-1.0..=1.0).contains(&value), "{wave:?} {value}");
            }
        }
        assert_eq!(Wave::Triangle.at(0.5), 1.0);
        assert_eq!(Wave::Triangle.at(0.0), -1.0);
    }
}
