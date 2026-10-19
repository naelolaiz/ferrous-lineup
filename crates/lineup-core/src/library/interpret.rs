//! Playing written notes the way a person would: a little louder here,
//! a little softer there.
//!
//! A score written out note by note, every note as loud as the next,
//! sounds like a machine. A musician never plays it that way: the first
//! beat of a bar weighs more than the others, a phrase swells towards its
//! highest note and relaxes at its end, a long note is leaned on, and no
//! two notes are ever quite equal. This module does that to the demos'
//! notes by changing their velocities (how hard each key is struck),
//! subtly, by a few steps out of 127. The notes stay where they are.
//!
//! The changes look random but are not: they come from the notes
//! themselves, so a demo sounds the same every time it is opened.
//!
//! What the score says comes first: the shaping is small next to the
//! difference between a piano and a forte, which it never evens out
//! (see [`perform`]), and the band plays as loud as the tune is marked
//! (see [`follow`]).

use crate::meter::TimeSignature;
use crate::pattern::NoteEvent;

use super::chart::Melody;

/// What a line of notes does in the music, which decides how it is
/// shaped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// The tune: shaped by the bar, by the phrase and by its contour.
    Melody,
    /// The chords or the bass, whose accents the comping already
    /// wrote: they only lose their machine-like evenness.
    Accompaniment,
}

/// The quietest and loudest the shaping ever makes a note, unless it
/// was written softer or louder still: a note marked pianissimo or
/// fortissimo keeps its place at the edge.
const SOFTEST: i32 = 24;
const LOUDEST: i32 = 124;

/// How much of an accent of the tune the band plays with it: half.
const BAND_ACCENT: f32 = 0.5;

/// A rest at least this many steps long ends a phrase: an eighth note,
/// the shortest breath a singer takes.
const BREATH: usize = 2;
/// A phrase is never longer than this many bars, so a tune without
/// rests still breathes.
const LONGEST_PHRASE: usize = 4;

/// Gives `notes` (one channel's notes in a part, in `time`) the small
/// differences in loudness of a real performance. `seed` makes each
/// channel and part come out differently.
///
/// The changes are a few steps either way, from where each note is
/// written: a tune marked piano stays soft and one marked forte loud. A
/// note written softer than [`SOFTEST`] or louder than [`LOUDEST`], such
/// as a fortissimo, may stay there.
pub fn perform(notes: &mut [NoteEvent], time: TimeSignature, role: Role, seed: u64) {
    notes.sort_by_key(|note| (note.start, note.pitch));
    match role {
        Role::Melody => shape_melody(notes, time, seed),
        Role::Accompaniment => {
            for note in notes.iter_mut() {
                // The notes of a chord are struck together, nearly as
                // hard as each other.
                let together = wobble(seed, note.start as u64);
                let apart = wobble(
                    seed ^ 0xA5A5,
                    (note.start as u64) << 8 | u64::from(note.pitch.midi()),
                );
                let change = (2.5 * together + 1.0 * apart).round() as i32;
                note.velocity = clamp(note.velocity, change);
            }
        }
    }
}

/// The accents of a melody: the bar, the phrase, the contour.
fn shape_melody(notes: &mut [NoteEvent], time: TimeSignature, seed: u64) {
    let bar = time.steps_per_bar();
    for phrase in phrases(notes, bar) {
        let notes = &mut notes[phrase];
        let count = notes.len();
        let mean = notes
            .iter()
            .map(|note| f32::from(note.pitch.midi()))
            .sum::<f32>()
            / count as f32;
        for (index, note) in notes.iter_mut().enumerate() {
            let mut change = 0.0;
            // The beat: strong on the first beat of the bar, a little on
            // the other beats, lighter off them.
            let in_bar = note.start % bar;
            change += if in_bar == 0 {
                6.0
            } else if time.is_beat_start(note.start) {
                2.0
            } else if in_bar.is_multiple_of(2) {
                -2.0
            } else {
                -4.0
            };
            // The contour: the higher a note is in its phrase, the more
            // it is sung out, up to half a dozen steps either way.
            change += ((f32::from(note.pitch.midi()) - mean) * 0.8).clamp(-6.0, 6.0);
            // A long note is leaned on.
            if note.length * 2 >= bar {
                change += 2.0;
            }
            // The end of a phrase relaxes.
            if count > 1 && index == count - 1 {
                change -= 5.0;
            }
            // And no two notes are ever quite alike.
            change += 3.0
                * wobble(
                    seed,
                    (note.start as u64) << 8 | u64::from(note.pitch.midi()),
                );
            note.velocity = clamp(note.velocity, change.round() as i32);
        }
    }
}

/// The phrases of a melody, as ranges of `notes` (sorted by start): a
/// phrase ends at a rest of [`BREATH`] steps or more, or after
/// [`LONGEST_PHRASE`] bars.
fn phrases(notes: &[NoteEvent], bar: usize) -> Vec<std::ops::Range<usize>> {
    let mut phrases = Vec::new();
    let mut first = 0;
    for index in 1..=notes.len() {
        let ends = match notes.get(index) {
            None => true,
            Some(next) => {
                let previous = &notes[index - 1];
                let rest = next.start.saturating_sub(previous.start + previous.length);
                let long = next.start - notes[first].start >= LONGEST_PHRASE * bar;
                rest >= BREATH || long
            }
        };
        if ends {
            phrases.push(first..index);
            first = index;
        }
    }
    phrases
}

/// A note written at `velocity`, changed by `change`, between
/// [`SOFTEST`] and [`LOUDEST`] or as far as it was written beyond them.
fn clamp(velocity: u8, change: i32) -> u8 {
    let written = i32::from(velocity);
    (written + change).clamp(SOFTEST.min(written), LOUDEST.max(written)) as u8
}

/// Plays a band's `notes` (one channel's notes in a part) as loud as the
/// part's `tune` is marked to be where each starts (see
/// [`Melody::loudness`]): softer under a piano, louder under a forte,
/// growing with a crescendo, as an orchestra follows the singer. A note
/// struck with an accent or a sforzando of the tune is accented too, half
/// as much: the band leans on it with the tune. A tune with no marks
/// leaves the notes as they are.
pub fn follow(notes: &mut [NoteEvent], tune: &Melody) {
    if !tune.is_marked() {
        return;
    }
    for note in notes {
        let loudness = tune
            .loudness
            .get(note.start)
            .or(tune.loudness.last())
            .copied()
            .unwrap_or(1.0);
        let accent = tune
            .accents
            .iter()
            .find(|&&(step, _)| step == note.start)
            .map_or(1.0, |&(_, accent)| 1.0 + (accent - 1.0) * BAND_ACCENT);
        let velocity = f32::from(note.velocity) * loudness * accent;
        note.velocity = velocity.round().clamp(1.0, 127.0) as u8;
    }
}

/// A number from -1 to 1 that looks random but is always the same for
/// the same `seed` and `value` (the "SplitMix64" mixer).
fn wobble(seed: u64, value: u64) -> f32 {
    let mut x = seed ^ value.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^= x >> 31;
    (x >> 40) as f32 / (1u64 << 23) as f32 - 1.0
}

/// A seed for a channel's notes in a part of a song, made from their
/// names.
pub fn seed(song: &str, part: &str, channel: &str) -> u64 {
    // FNV-1a: a simple hash that mixes in one byte at a time.
    let mut hash: u64 = 0xCBF2_9CE4_8422_2325;
    for byte in song
        .bytes()
        .chain([0])
        .chain(part.bytes())
        .chain([0])
        .chain(channel.bytes())
    {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0100_0000_01B3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DEFAULT_VELOCITY;
    use crate::pitch::Note;

    fn note(midi: u8, start: usize, length: usize) -> NoteEvent {
        NoteEvent::new(Note::new(midi), start, length)
    }

    #[test]
    fn a_tune_gets_its_accents_but_stays_close_to_how_it_was_written() {
        // Twinkle, twinkle: C C G G A A G-, then F F E E D D C-.
        let mut notes = vec![
            note(60, 0, 4),
            note(60, 4, 4),
            note(67, 8, 4),
            note(67, 12, 4),
            note(69, 16, 4),
            note(69, 20, 4),
            note(67, 24, 8),
            note(65, 32, 4),
            note(65, 36, 4),
            note(64, 40, 4),
            note(64, 44, 4),
            note(62, 48, 4),
            note(62, 52, 4),
            note(60, 56, 8),
        ];
        let time = TimeSignature::COMMON;
        perform(&mut notes, time, Role::Melody, seed("Twinkle", "A", "Lead"));
        let velocities: Vec<i32> = notes.iter().map(|n| i32::from(n.velocity)).collect();
        // Subtle: never far from what was written.
        for &velocity in &velocities {
            assert!(
                (DEFAULT_VELOCITY as i32 - velocity).abs() <= 16,
                "{velocities:?}"
            );
        }
        // Not all the same any more.
        assert!(
            velocities.iter().any(|&v| v != velocities[0]),
            "{velocities:?}"
        );
        // The top of the first phrase, A on the downbeat of bar 2, is
        // louder than its first note, the low C.
        assert!(velocities[4] > velocities[0], "{velocities:?}");
        // The same every time.
        let mut again = notes.clone();
        for note in &mut again {
            note.velocity = DEFAULT_VELOCITY;
        }
        perform(&mut again, time, Role::Melody, seed("Twinkle", "A", "Lead"));
        assert_eq!(again, notes);
    }

    #[test]
    fn a_rest_ends_a_phrase() {
        let notes = vec![
            note(60, 0, 4),
            note(62, 4, 2),
            note(64, 8, 4),
            note(65, 12, 4),
        ];
        assert_eq!(phrases(&notes, 16), vec![0..2, 2..4]);
    }

    #[test]
    fn chords_keep_their_accents_give_or_take_a_little() {
        let mut chord = vec![note(48, 0, 16), note(52, 0, 16), note(55, 0, 16)];
        for note in &mut chord {
            note.velocity = 88;
        }
        perform(&mut chord, TimeSignature::COMMON, Role::Accompaniment, 7);
        for note in &chord {
            assert!((84..=92).contains(&note.velocity), "{chord:?}");
        }
    }

    #[test]
    fn piano_stays_soft_and_fortissimo_loud() {
        let tune = |marks: &str| -> Vec<u8> {
            let text = format!("{marks} C5:4 D5 E5 F5 | G5:8 E5 | D5:4 C5 B4 D5 | C5:16");
            let mut notes = crate::library::chart::parse_melody(&text).unwrap().notes;
            perform(&mut notes, TimeSignature::COMMON, Role::Melody, 7);
            notes.iter().map(|note| note.velocity).collect()
        };
        let (piano, forte, fortissimo) = (tune("!p"), tune("!f"), tune("!ff"));
        // Every note of the piano softer than every note of the forte.
        assert!(
            piano.iter().max() < forte.iter().min(),
            "{piano:?} {forte:?}"
        );
        // Fortissimo may go past the usual loudest, as it is written.
        assert!(
            fortissimo.iter().any(|&v| i32::from(v) > LOUDEST),
            "{fortissimo:?}"
        );
        let mean = |v: &[u8]| v.iter().map(|&v| f32::from(v)).sum::<f32>() / v.len() as f32;
        assert!(mean(&fortissimo) > mean(&forte) + 5.0);
    }

    #[test]
    fn the_band_follows_the_marks_of_the_tune() {
        let tune = crate::library::chart::parse_melody("!p C5:8 !< D5:8 | !f >E5:16").unwrap();
        let mut chords: Vec<NoteEvent> = [0, 8, 12, 16, 20]
            .iter()
            .map(|&at| NoteEvent {
                velocity: 80,
                ..note(60, at, 4)
            })
            .collect();
        follow(&mut chords, &tune);
        let velocities: Vec<u8> = chords.iter().map(|note| note.velocity).collect();
        // Piano, growing to forte, the accent leaned on half as hard as the
        // tune does, then forte.
        assert_eq!(velocities, [44, 44, 68, 104, 92]);
        // A tune without marks leaves the band alone.
        let plain = crate::library::chart::parse_melody("C5:16").unwrap();
        let mut again = vec![note(60, 0, 4)];
        follow(&mut again, &plain);
        assert_eq!(again[0].velocity, DEFAULT_VELOCITY);
    }
}
