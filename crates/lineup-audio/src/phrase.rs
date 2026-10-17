//! A phrase: a few notes played apart from the song, on instruments of
//! its own, such as a question of the ear training.
//!
//! The UI thread builds the whole phrase, instruments and notes, and
//! sends it to the audio thread in one [`Command::PlayPhrase`]. There the
//! phrase only starts each note when its time comes and adds its
//! instruments to the mix, so its timing is as exact as the song's. When
//! it has rung out, or another phrase takes its place, it goes back to
//! the UI thread to be freed.
//!
//! [`Command::PlayPhrase`]: crate::Command::PlayPhrase

use lineup_core::Note;

use crate::instrument::Instrument;

/// How loud a phrase is: as loud as a channel at the rack's usual volume.
const VOLUME: f32 = 0.8;

/// How long a phrase goes on after its last note is let go, in seconds,
/// for the sound to ring out.
const RING_OUT: f64 = 2.0;

/// One note of a phrase.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhraseNote {
    /// Which of the phrase's instruments plays it.
    pub instrument: usize,
    /// When it starts, in seconds from the start of the phrase.
    pub start: f64,
    /// How long it is held, in seconds.
    pub length: f64,
    pub pitch: Note,
    pub velocity: u8,
}

/// A note with its times in frames, ready for the audio thread.
#[derive(Debug, Clone, Copy)]
struct Timed {
    instrument: usize,
    start: u64,
    gate: u32,
    pitch: Note,
    velocity: u8,
}

/// Notes and the instruments that play them.
#[derive(Debug)]
pub struct Phrase {
    instruments: Vec<Instrument>,
    /// In the order they start.
    notes: Vec<Timed>,
    /// The next note to start.
    next: usize,
    /// Frames played since the phrase started.
    clock: u64,
    /// When the phrase is over, in frames: it has rung out.
    end: u64,
    sample_rate: u32,
}

impl Phrase {
    /// A phrase of `notes` played by `instruments`, at `sample_rate`.
    /// Notes for an instrument that is not there are left out.
    pub fn new(instruments: Vec<Instrument>, notes: &[PhraseNote], sample_rate: u32) -> Self {
        let frames = |seconds: f64| (seconds.max(0.0) * f64::from(sample_rate)).round();
        let mut notes: Vec<Timed> = notes
            .iter()
            .filter(|note| note.instrument < instruments.len())
            .map(|note| Timed {
                instrument: note.instrument,
                start: frames(note.start) as u64,
                gate: frames(note.length).clamp(1.0, f64::from(u32::MAX - 1)) as u32,
                pitch: note.pitch,
                velocity: note.velocity,
            })
            .collect();
        notes.sort_by_key(|note| note.start);
        let last = notes
            .iter()
            .map(|note| note.start + u64::from(note.gate))
            .max()
            .unwrap_or(0);
        Phrase {
            instruments,
            notes,
            next: 0,
            clock: 0,
            end: last + frames(RING_OUT) as u64,
            sample_rate,
        }
    }

    /// The next sample of the phrase, starting the notes due now.
    pub fn next_sample(&mut self) -> f32 {
        while let Some(note) = self.notes.get(self.next)
            && note.start <= self.clock
        {
            if let Some(instrument) = self.instruments.get_mut(note.instrument) {
                instrument.note_on(note.pitch, note.velocity, note.gate, self.sample_rate);
            }
            self.next += 1;
        }
        self.clock += 1;
        let mix: f32 = self
            .instruments
            .iter_mut()
            .map(Instrument::next_sample)
            .sum();
        mix * VOLUME
    }

    /// Lets go of every note and drops the ones not started yet: the
    /// phrase rings out and ends.
    pub fn stop(&mut self) {
        for instrument in &mut self.instruments {
            instrument.release_all();
        }
        self.next = self.notes.len();
        self.end = self
            .end
            .min(self.clock + (RING_OUT * f64::from(self.sample_rate)) as u64);
    }

    /// Whether it has finished and rung out.
    pub fn is_over(&self) -> bool {
        self.clock >= self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instrument::Synth;

    const RATE: u32 = 1000;

    fn note(start: f64, pitch: u8) -> PhraseNote {
        PhraseNote {
            instrument: 0,
            start,
            length: 0.1,
            pitch: Note::new(pitch),
            velocity: 100,
        }
    }

    fn phrase(notes: &[PhraseNote]) -> Phrase {
        Phrase::new(vec![Instrument::Synth(Synth::new(RATE))], notes, RATE)
    }

    /// The first frame that is not silent, from `from` on.
    fn first_sound(phrase: &mut Phrase, from: usize, frames: usize) -> Option<usize> {
        (0..frames)
            .map(|_| phrase.next_sample())
            .enumerate()
            .skip(from)
            .find(|(_, sample)| sample.abs() > 1e-4)
            .map(|(frame, _)| frame)
    }

    #[test]
    fn notes_start_on_time_whatever_order_they_come_in() {
        let mut later = phrase(&[note(0.5, 60), note(0.25, 64)]);
        let start = first_sound(&mut later, 0, 400).unwrap();
        assert!((250..=252).contains(&start), "{start}");
        assert_eq!(later.next, 1);
    }

    #[test]
    fn a_phrase_is_over_once_it_has_rung_out() {
        let mut short = phrase(&[note(0.0, 60)]);
        for _ in 0..2050 {
            short.next_sample();
        }
        assert!(!short.is_over());
        for _ in 0..100 {
            short.next_sample();
        }
        assert!(short.is_over());
    }

    #[test]
    fn stopping_drops_the_notes_still_to_come() {
        let mut stopped = phrase(&[note(0.0, 60), note(1.0, 64)]);
        for _ in 0..100 {
            stopped.next_sample();
        }
        stopped.stop();
        for _ in 0..1500 {
            stopped.next_sample();
        }
        assert_eq!(stopped.next, 2);
        let rest: f32 = (0..400).map(|_| stopped.next_sample().abs()).sum();
        assert!(rest < 1e-3, "{rest}");
        // Two seconds after it was stopped, it is over.
        assert!(!stopped.is_over());
        for _ in 0..100 {
            stopped.next_sample();
        }
        assert!(stopped.is_over());
    }

    #[test]
    fn notes_for_missing_instruments_are_left_out() {
        let lost = PhraseNote {
            instrument: 3,
            ..note(0.0, 60)
        };
        let mut silent = phrase(&[lost]);
        assert_eq!(first_sound(&mut silent, 0, 500), None);
    }
}
