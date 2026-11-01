//! Hearing the bass line of a recording: each note's start, end, pitch
//! and loudness.
//!
//! The pitch finder is pYIN (as for a voice; see [`crate::voice`]), with
//! a window long enough to hear the low E of a bass guitar, and the sound
//! filtered to the bass register before it is heard. Notes shorter than
//! 80 ms are left out: in a mix, these are more often a kick drum or a
//! transient than a bass note.

use lineup_core::take::BassNote;

use crate::voice;

/// The low and high notes of a bass guitar, with a little room either
/// side for a five-string or a synth bass.
const LOWEST_HZ: f32 = 28.0;
const HIGHEST_HZ: f32 = 300.0;
/// Two periods of the lowest note, to hear its pitch without mistaking
/// the first harmonic for the fundamental.
const WINDOW_SECONDS: f64 = 0.075;
const HIGH_PASS_HZ: f32 = 20.0;
/// Keep the bass fundamental and its first harmonics, while taking out
/// most of the other instruments before the pitch finder.
const LOW_PASS_HZ: f32 = 500.0;
const SHORTEST_SECONDS: f64 = 0.08;

/// The bass notes of `samples`, at `sample_rate`, one after the other
/// from the start of the recording to its end.
pub fn notes(samples: &[f32], sample_rate: u32) -> Vec<BassNote> {
    let track = voice::pitch_track_with(
        samples,
        sample_rate,
        LOWEST_HZ,
        HIGHEST_HZ,
        WINDOW_SECONDS,
        HIGH_PASS_HZ,
        Some(LOW_PASS_HZ),
    );
    voice::notes(&track)
        .into_iter()
        .filter(|note| note.seconds() >= SHORTEST_SECONDS)
        .map(|note| BassNote {
            start: note.start,
            end: note.end,
            pitch: note.pitch,
            loudness: note.loudness,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::voice::midi_of;

    const RATE: u32 = 16_000;

    fn note(midi: f32, time: f32) -> f32 {
        let fundamental = 440.0 * 2.0_f32.powf((midi - 69.0) / 12.0);
        (1..=6)
            .map(|harmonic| {
                let harmonic = harmonic as f32;
                (std::f32::consts::TAU * fundamental * harmonic * time).sin() / harmonic
            })
            .sum::<f32>()
            * 0.2
    }

    fn phrase(notes: &[f32]) -> Vec<f32> {
        let mut samples = Vec::new();
        for &midi in notes {
            let length = RATE as usize * 3 / 5;
            samples.extend((0..length).map(|frame| note(midi, frame as f32 / RATE as f32)));
            samples.resize(samples.len() + RATE as usize / 10, 0.0);
        }
        samples
    }

    #[test]
    fn hears_low_bass_notes_and_the_changes_between_them() {
        let heard = notes(&phrase(&[28.0, 31.0, 33.0]), RATE);
        for (index, wanted) in [28.0, 31.0, 33.0].into_iter().enumerate() {
            let middle = 0.7 * index as f64 + 0.3;
            let note = heard
                .iter()
                .find(|note| note.start <= middle && middle < note.end)
                .unwrap_or_else(|| panic!("no note near {middle}: {heard:?}"));
            assert!(
                (note.pitch - wanted).abs() < 0.5,
                "wanted {wanted}, heard {} ({})",
                note.pitch,
                midi_of(440.0 * 2.0_f32.powf((note.pitch - 69.0) / 12.0)),
            );
        }
    }

    #[test]
    fn silence_has_no_bass_notes() {
        assert!(notes(&vec![0.0; RATE as usize], RATE).is_empty());
        assert!(notes(&[], RATE).is_empty());
    }
}
