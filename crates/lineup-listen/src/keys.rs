//! Where the key of a recording changes.
//!
//! The work is done by [`lineup_core::harmony::key_finding`], which finds
//! key changes from what sounds in each bar. A recording gives no notes,
//! only the chroma of each beat, so the beats are put together into bars
//! and each bar's chroma stands for the notes in it.

use lineup_core::Scale;
use lineup_core::harmony::key_finding::{self, KeySpan};

use crate::chroma::Frame;

/// How much of the note a fifth below is taken away from each pitch
/// class. A note's third harmonic sounds an octave and a fifth above it,
/// so a recording of C major is full of G, D and B that nobody played,
/// and the key comes out a fifth too high (G for C) more often than
/// not. Measured on the demos rendered to audio, 0.3 helps most.
const FIFTH_OVERTONE: f32 = 0.3;

/// The chroma of a bar: the treble chroma of its beats added up, each
/// beat first lowered by its quietest pitch class, then with the
/// overtones of the note a fifth below taken away. In a log spectrum
/// every pitch class has some energy (noise, the overtones of other
/// notes); taking away the floor leaves the notes that stand out. The
/// bass chroma is left out: the low notes' overtones fill it with fifths,
/// and with it the key was right less often (80% of beats against 84%).
fn bar_chroma(beats: &[Frame]) -> key_finding::Chroma {
    let mut sum = [0.0; 12];
    for beat in beats {
        let floor = beat.treble.iter().copied().fold(f32::MAX, f32::min);
        let chroma = beat.treble.map(|v| v - floor);
        for (pc, s) in sum.iter_mut().enumerate() {
            // The note a fifth below `pc` is five semitones above it.
            *s += (chroma[pc] - FIFTH_OVERTONE * chroma[(pc + 5) % 12]).max(0.0);
        }
    }
    sum
}

/// The stretches in one key, with `start` and `end` in beats, from the
/// chroma of every beat and where the bars fall: `beats_per_bar` beats
/// to a bar, the first full bar starting on beat `first_downbeat`. The
/// beats before it make a bar of their own.
pub fn key_spans(beats: &[Frame], beats_per_bar: usize, first_downbeat: usize) -> Vec<KeySpan> {
    let mut edges = vec![0];
    let mut at = first_downbeat.min(beats.len());
    if at == 0 {
        at = beats_per_bar.max(1);
    }
    while at < beats.len() {
        edges.push(at);
        at += beats_per_bar.max(1);
    }
    edges.push(beats.len());
    let bars: Vec<_> = edges
        .windows(2)
        .map(|bar| bar_chroma(&beats[bar[0]..bar[1]]))
        .collect();
    key_finding::key_spans(&bars)
        .into_iter()
        .map(|span| KeySpan {
            start: edges[span.start],
            end: edges[span.end],
            ..span
        })
        .collect()
}

/// The key of the longest stretch in one key: the song's main key. On
/// the songs of the key changes report it is right more often than the
/// key of the whole recording heard at once (91 songs of 136 against
/// 82), as a modulation or a long introduction in another key pulls
/// the whole recording's notes away from the song's home key.
pub fn main_key(spans: &[KeySpan]) -> Option<Scale> {
    spans
        .iter()
        .max_by_key(|span| span.end - span.start)
        .and_then(KeySpan::key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lineup_core::harmony::key_finding::KeyGuess;
    use lineup_core::{PitchClass, ScaleKind};

    fn span(start: usize, end: usize, tonic: PitchClass) -> KeySpan {
        KeySpan {
            start,
            end,
            guesses: vec![KeyGuess {
                key: Scale::new(tonic, ScaleKind::Major),
                fit: 0.8,
                share: 0.9,
            }],
        }
    }

    #[test]
    fn the_main_key_is_the_longest_stretchs() {
        let spans = [
            span(0, 8, PitchClass::G),
            span(8, 40, PitchClass::C),
            span(40, 56, PitchClass::F),
        ];
        assert_eq!(
            main_key(&spans),
            Some(Scale::new(PitchClass::C, ScaleKind::Major))
        );
        assert_eq!(main_key(&[]), None);
    }
}
