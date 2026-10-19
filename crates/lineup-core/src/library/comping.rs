//! Comping and bass lines: how a progression is *played*.
//!
//! The same four chords can be a ballad, a reggae tune or a house track.
//! What changes is the rhythm the chords are played in (jazz musicians
//! call this "comping", short for accompanying) and what the bass does.
//! Each style here is a short table of *hits*: when a note starts, how
//! long it lasts, which notes of the chord it plays, and how hard.

use std::borrow::Cow;

use crate::harmony::Chord;
use crate::meter::{Bar, TimeSignature};
use crate::pattern::NoteEvent;
use crate::pitch::{Note, PitchClass};

use super::voicing::bass_root;

/// Which notes of a chord voicing a hit plays.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Voices {
    /// The whole chord.
    All,
    /// Every note but the lowest: the lighter "up-strokes" of a strum.
    Upper,
    /// One note, counted from the bottom. Counting past the top goes on
    /// in the next octave, so on a three-note chord, 3 is the bottom
    /// note an octave higher: arpeggios climb instead of jumping back.
    Nth(usize),
}

/// One note or chord in a comping or bass pattern.
#[derive(Debug, Clone, Copy)]
struct Hit<T> {
    /// When it starts, in steps from the start of the pattern's period.
    at: usize,
    /// How long it lasts, in steps.
    len: usize,
    what: T,
    velocity: u8,
}

/// A shorter way to write a [`Hit`] in the tables below.
///
/// A macro, not a function: the tables are borrowed as `&'static`
/// slices, and Rust only makes a borrowed value live forever (it
/// "promotes" it) when it is written out as plain values, not as the
/// result of a function call.
macro_rules! hit {
    ($at:expr, $len:expr, $what:expr, $velocity:expr) => {
        Hit {
            at: $at,
            len: $len,
            what: $what,
            velocity: $velocity,
        }
    };
}

/// The ways of playing chords.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Comping {
    /// Each chord held for as long as it lasts: pads and slow ballads.
    Held,
    /// Steady eighth notes: rock and pop piano or guitar.
    Pulse,
    /// Short chords on every beat, the tango's "marcato".
    Quarters,
    /// Short chords between the beats: the reggae and ska "skank".
    Offbeat,
    /// Short chords on beats 2 and 4, with the snare: the guitar's
    /// "chop" in soul and reggae, the "chick" of country's boom-chick.
    Backbeat,
    /// The jazz "Charleston": on beat 1, then just before beat 3.
    Charleston,
    /// Syncopated piano stabs, as in house music.
    Stabs,
    /// A strummed guitar: down, down-up, up-down-up.
    Strum,
    /// The bossa nova guitar's two-bar syncopation.
    Bossa,
    /// 3 + 3 + 2: the tresillo of Cuban music, everywhere in pop now.
    Tresillo,
    /// 3 + 3 + 2 in every half bar: the reggaeton "dembow".
    Dembow,
    /// Chords chopped into sixteenths: the trance "gate".
    Gate,
    /// Short, syncopated sixteenth-note stabs.
    Funk,
    /// On beat 1, then again just before beat 3, held: lazy lo-fi chords.
    Lazy,
    /// The chord on both eighths of each beat, the second one lighter:
    /// with swing, the long-short "chunk-a" of blues and boogie players.
    Shuffle,
    /// Gospel piano: chords on the beats with quick pushes between them.
    Gospel,
    /// A ballad piano: the chord, then its notes one by one.
    Ballad,
    /// Up the chord in eighth notes.
    ArpUp,
    /// Up and down the chord in eighth notes.
    ArpUpDown,
    /// Up and down the chord in sixteenth notes, as synths do.
    Arp16,
    /// Low, high, middle, high: the classical "Alberti bass".
    Alberti,
    /// A salsa piano montuno: a syncopated, repeating figure.
    Montuno,
    /// The flamenco rumba strum, with its quick double up-strokes.
    Rumba,
    /// For 3/4: chords on beats 2 and 3, after the bass on beat 1.
    Waltz,
    /// For 3/4: one chord on beat 2, held to the end of the bar.
    Sway,
    /// For 3/4 and 6/8: up and down the chord in eighth notes, one bar
    /// at a time.
    Arp3,
    /// Long, short, long, long: the Cuban habanera, grandmother of the tango.
    Habanera,
    /// For 12/8: the chord on every eighth, three to a beat, the first of
    /// each three the strongest: the slow blues piano.
    Triplets,
    /// For odd meters, and any other: the "oom-pa" of a dance band in
    /// the beats the bar is felt in (see [`TimeSignature::felt_groups`]).
    /// The bass takes the first note of every beat and the chord the
    /// notes after it, so the long beats get two chords and the short
    /// ones one. In the 2+2+3/8 of a Bulgarian rachenitsa: bass-chord,
    /// bass-chord, bass-chord-chord; in a 3+2+2/8 kalamatianos the long
    /// beat comes first. Turkish and Balkan musicians call these meters
    /// *aksak*, "limping". Play it with [`Bass::Aksak`].
    Aksak,
    /// One chord on each felt beat, with the first accent strongest.
    Accents,
}

impl Comping {
    /// How many steps the pattern lasts before it repeats, and its hits,
    /// in bars of `time`. Only [`Comping::Aksak`] depends on the time
    /// signature: its pattern is a bar long.
    fn hits(self, time: TimeSignature) -> (usize, Cow<'static, [Hit<Voices>]>) {
        match self {
            Comping::Accents => {
                let unit = time.unit_steps();
                let mut at = 0;
                let hits = time
                    .felt_groups()
                    .into_iter()
                    .enumerate()
                    .map(|(index, group)| {
                        let hit = hit!(at, unit, Voices::All, if index == 0 { 104 } else { 88 });
                        at += usize::from(group) * unit;
                        hit
                    })
                    .collect();
                (time.steps_per_bar(), Cow::Owned(hits))
            }
            Comping::Aksak => {
                let unit = time.unit_steps();
                // A chord a little shorter than its note, so the next
                // one is heard to start, but never under a step.
                let length = (unit - unit / 4).max(1);
                let mut hits = Vec::new();
                let mut at = 0;
                for group in time.felt_groups() {
                    for note in 1..usize::from(group) {
                        let velocity = if note == 1 { 84 } else { 74 };
                        hits.push(hit!(at + note * unit, length, Voices::All, velocity));
                    }
                    at += usize::from(group) * unit;
                }
                (time.steps_per_bar(), Cow::Owned(hits))
            }
            _ => {
                let (period, hits) = self.pattern();
                (period, Cow::Borrowed(hits))
            }
        }
    }

    /// Whether the pattern counts from the bar line, wherever the chord
    /// starts: the beats of an odd meter are where the bar puts them.
    /// The other patterns start again with each chord.
    fn follows_the_bar(self) -> bool {
        matches!(self, Comping::Aksak | Comping::Accents)
    }

    /// How many steps the pattern lasts before it repeats, and its hits,
    /// for the patterns that are the same in every time signature.
    fn pattern(self) -> (usize, &'static [Hit<Voices>]) {
        use Voices::*;
        match self {
            // Held is handled on its own: it lasts as long as the chord.
            Comping::Held => (16, &[hit!(0, 16, All, 88)]),
            Comping::Pulse => (4, &[hit!(0, 2, All, 96), hit!(2, 2, All, 78)]),
            Comping::Quarters => (4, &[hit!(0, 2, All, 100)]),
            Comping::Offbeat => (4, &[hit!(2, 1, All, 92)]),
            Comping::Backbeat => (8, &[hit!(4, 1, All, 96)]),
            Comping::Charleston => (16, &[hit!(0, 3, All, 96), hit!(6, 2, All, 86)]),
            Comping::Stabs => (
                16,
                &[
                    hit!(0, 2, All, 100),
                    hit!(3, 2, All, 88),
                    hit!(6, 3, All, 96),
                    hit!(10, 2, All, 88),
                    hit!(12, 3, All, 94),
                ],
            ),
            Comping::Strum => (
                16,
                &[
                    hit!(0, 4, All, 100),
                    hit!(4, 2, All, 90),
                    hit!(6, 4, Upper, 74),
                    hit!(10, 2, Upper, 74),
                    hit!(12, 2, All, 90),
                    hit!(14, 2, Upper, 74),
                ],
            ),
            Comping::Bossa => (
                32,
                &[
                    hit!(0, 3, All, 92),
                    hit!(3, 3, All, 80),
                    hit!(6, 4, All, 88),
                    hit!(10, 3, All, 80),
                    hit!(13, 3, All, 86),
                    hit!(18, 4, All, 84),
                    hit!(22, 3, All, 80),
                    hit!(26, 3, All, 86),
                    hit!(29, 3, All, 80),
                ],
            ),
            Comping::Tresillo => (
                16,
                &[
                    hit!(0, 3, All, 100),
                    hit!(6, 3, All, 90),
                    hit!(12, 3, All, 94),
                ],
            ),
            Comping::Dembow => (
                8,
                &[
                    hit!(0, 3, All, 96),
                    hit!(3, 3, All, 84),
                    hit!(6, 2, All, 90),
                ],
            ),
            Comping::Gate => (
                16,
                &[
                    hit!(0, 1, All, 104),
                    hit!(1, 1, All, 80),
                    hit!(3, 1, All, 92),
                    hit!(4, 1, All, 98),
                    hit!(6, 1, All, 92),
                    hit!(7, 1, All, 80),
                    hit!(9, 1, All, 92),
                    hit!(10, 1, All, 98),
                    hit!(12, 1, All, 92),
                    hit!(14, 1, All, 92),
                ],
            ),
            Comping::Funk => (
                16,
                &[
                    hit!(0, 1, All, 106),
                    hit!(3, 1, Upper, 80),
                    hit!(6, 1, All, 96),
                    hit!(7, 1, Upper, 72),
                    hit!(10, 1, All, 96),
                    hit!(12, 1, Upper, 82),
                    hit!(14, 1, All, 92),
                ],
            ),
            Comping::Lazy => (16, &[hit!(0, 6, All, 84), hit!(7, 9, All, 72)]),
            Comping::Shuffle => (4, &[hit!(0, 2, All, 94), hit!(2, 2, All, 70)]),
            Comping::Gospel => (
                16,
                &[
                    hit!(0, 3, All, 100),
                    hit!(3, 1, Upper, 70),
                    hit!(4, 3, All, 90),
                    hit!(7, 1, Upper, 70),
                    hit!(8, 3, All, 96),
                    hit!(11, 1, Upper, 70),
                    hit!(12, 2, All, 90),
                    hit!(14, 2, Upper, 76),
                ],
            ),
            Comping::Ballad => (
                16,
                &[
                    hit!(0, 4, All, 90),
                    hit!(4, 2, Nth(1), 70),
                    hit!(6, 2, Nth(2), 72),
                    hit!(8, 4, All, 84),
                    hit!(12, 2, Nth(1), 70),
                    hit!(14, 2, Nth(2), 72),
                ],
            ),
            Comping::ArpUp => (
                8,
                &[
                    hit!(0, 2, Nth(0), 92),
                    hit!(2, 2, Nth(1), 80),
                    hit!(4, 2, Nth(2), 84),
                    hit!(6, 2, Nth(3), 80),
                ],
            ),
            Comping::ArpUpDown => (
                16,
                &[
                    hit!(0, 2, Nth(0), 92),
                    hit!(2, 2, Nth(1), 78),
                    hit!(4, 2, Nth(2), 84),
                    hit!(6, 2, Nth(3), 78),
                    hit!(8, 2, Nth(4), 88),
                    hit!(10, 2, Nth(3), 78),
                    hit!(12, 2, Nth(2), 84),
                    hit!(14, 2, Nth(1), 78),
                ],
            ),
            Comping::Arp16 => (
                8,
                &[
                    hit!(0, 1, Nth(0), 96),
                    hit!(1, 1, Nth(1), 78),
                    hit!(2, 1, Nth(2), 86),
                    hit!(3, 1, Nth(3), 78),
                    hit!(4, 1, Nth(4), 90),
                    hit!(5, 1, Nth(3), 78),
                    hit!(6, 1, Nth(2), 86),
                    hit!(7, 1, Nth(1), 78),
                ],
            ),
            Comping::Alberti => (
                8,
                &[
                    hit!(0, 2, Nth(0), 88),
                    hit!(2, 2, Nth(2), 74),
                    hit!(4, 2, Nth(1), 80),
                    hit!(6, 2, Nth(2), 74),
                ],
            ),
            Comping::Montuno => (
                16,
                &[
                    hit!(0, 2, Nth(0), 96),
                    hit!(3, 1, Nth(2), 82),
                    hit!(4, 2, Nth(1), 88),
                    hit!(6, 2, Nth(3), 90),
                    hit!(8, 2, Nth(2), 84),
                    hit!(10, 1, Nth(1), 80),
                    hit!(11, 2, Nth(3), 90),
                    hit!(14, 2, Nth(2), 86),
                ],
            ),
            Comping::Rumba => (
                16,
                &[
                    hit!(0, 2, All, 106),
                    hit!(2, 1, Upper, 70),
                    hit!(3, 1, Upper, 70),
                    hit!(4, 2, All, 94),
                    hit!(6, 2, Upper, 76),
                    hit!(8, 2, All, 100),
                    hit!(10, 1, Upper, 70),
                    hit!(11, 1, Upper, 70),
                    hit!(12, 2, All, 94),
                    hit!(14, 2, Upper, 76),
                ],
            ),
            Comping::Waltz => (12, &[hit!(4, 3, All, 80), hit!(8, 3, All, 74)]),
            Comping::Sway => (12, &[hit!(4, 8, All, 76)]),
            Comping::Habanera => (
                8,
                &[
                    hit!(0, 3, All, 96),
                    hit!(3, 1, Upper, 74),
                    hit!(4, 2, All, 86),
                    hit!(6, 2, All, 84),
                ],
            ),
            Comping::Triplets => (
                6,
                &[
                    hit!(0, 2, All, 92),
                    hit!(2, 2, Upper, 66),
                    hit!(4, 2, Upper, 72),
                ],
            ),
            Comping::Arp3 => (
                12,
                &[
                    hit!(0, 2, Nth(0), 86),
                    hit!(2, 2, Nth(1), 72),
                    hit!(4, 2, Nth(2), 78),
                    hit!(6, 2, Nth(3), 74),
                    hit!(8, 2, Nth(2), 76),
                    hit!(10, 2, Nth(1), 72),
                ],
            ),
            // Worked out from the bar in `hits`.
            Comping::Aksak | Comping::Accents => (16, &[]),
        }
    }

    /// The notes for a chord voiced as `voicing`, starting at step
    /// `start` and lasting `span` steps, in bars of `time` counted from
    /// step 0. A pattern that follows the bar ([`Comping::Aksak`]) and
    /// has no hit inside so short a chord strikes it once where it
    /// starts, so no chord goes unheard.
    pub fn play(
        self,
        time: TimeSignature,
        voicing: &[Note],
        start: usize,
        span: usize,
    ) -> Vec<NoteEvent> {
        if voicing.is_empty() || span == 0 {
            return Vec::new();
        }
        if self == Comping::Held {
            return voicing
                .iter()
                .map(|&pitch| NoteEvent {
                    velocity: 88,
                    ..NoteEvent::new(pitch, start, span)
                })
                .collect();
        }
        let (period, hits) = self.hits(time);
        let end = start + span;
        let origin = if self.follows_the_bar() {
            start - start % period
        } else {
            start
        };
        let mut notes = Vec::new();
        for offset in (origin..end).step_by(period) {
            for hit in hits.iter() {
                let at = offset + hit.at;
                if at < start || at >= end {
                    continue;
                }
                let length = hit.len.min(end - at);
                for pitch in pick(voicing, hit.what) {
                    notes.push(NoteEvent {
                        velocity: hit.velocity,
                        ..NoteEvent::new(pitch, at, length)
                    });
                }
            }
        }
        if notes.is_empty()
            && self.follows_the_bar()
            && let Some(first) = hits.first()
        {
            for &pitch in voicing {
                notes.push(NoteEvent {
                    velocity: first.velocity,
                    ..NoteEvent::new(pitch, start, first.len.min(span))
                });
            }
        }
        notes
    }
}

/// The notes of `voicing` that `voices` asks for. A voicing with no
/// notes (N.C., no chord) gives none.
fn pick(voicing: &[Note], voices: Voices) -> Vec<Note> {
    match voices {
        Voices::All => voicing.to_vec(),
        Voices::Upper => voicing.iter().skip(1).copied().collect(),
        Voices::Nth(_) if voicing.is_empty() => Vec::new(),
        Voices::Nth(index) => {
            let octave = (index / voicing.len()) as i32;
            voicing[index % voicing.len()]
                .transpose(12 * octave)
                .into_iter()
                .collect()
        }
    }
}

/// A note of the bass line, chosen from the chord.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tone {
    Root,
    /// The fifth above the root.
    Fifth,
    /// The fifth below the root: the low note of "oom-pah".
    LowFifth,
    Octave,
    Third,
    /// The chord's seventh, or its sixth if it has no seventh.
    Seventh,
    /// The next chord's root, played early: an "anticipation".
    Next,
    /// One semitone from the next chord's root, leading into it.
    Approach,
}

/// The ways of playing the bass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Bass {
    /// No bass line.
    None,
    /// The root, held for as long as the chord lasts.
    Held,
    /// The root in steady eighth notes: rock.
    Pulse,
    /// The root on every beat.
    Quarters,
    /// Root on beat 1, the fifth below on beat 3: country, polka, cumbia.
    RootFifth,
    /// Root and octave in eighth notes: disco.
    Octaves,
    /// The root between the beats: house and trance.
    Offbeat,
    /// A walking jazz line: a note on every beat, leading to the next chord.
    Walking,
    /// The boogie-woogie line up and down the chord: root, third, fifth, sixth, seventh.
    Boogie,
    /// Root and fifth with the bossa nova's pickups.
    Bossa,
    /// The tumbao of son and salsa: nothing on beat 1, the fifth on the
    /// "and" of 2, and on 4 the next chord's root, held over the bar line.
    Tumbao,
    /// A reggae one-drop line.
    Reggae,
    /// The 3 + 3 + 2 of reggaeton.
    Dembow,
    /// Syncopated sixteenths with octave "pops".
    Funk,
    /// Long, deep notes that follow the kick, as an 808 does in trap.
    Sub,
    /// A lazy lo-fi line that leans into the next chord.
    Lazy,
    /// Root and octave in sixteenth notes: synthwave.
    Octaves16,
    /// 3 + 3 + 2 once a bar: the tango's "síncopa" and the milonga.
    Tresillo,
    /// For 3/4: the root on beat 1, and the fifth below it in the next bar.
    Waltz,
    /// The habanera's long, short, long, long.
    Habanera,
    /// For 12/8: the root on every beat, long then short, the short one
    /// on the beat's last eighth leading to the next: the slow blues.
    LongShort,
    /// For odd meters, and any other: the bass of the "oom-pa" (see
    /// [`Comping::Aksak`]), on the first note of every beat the bar is
    /// felt in: the root, then the fifth below and the root by turns.
    /// In 2+2+3/8: root, fifth, root, the last one on the long beat.
    Aksak,
    /// The root on each felt beat, without borrowing a style's bass line.
    Accents,
}

impl Bass {
    /// How long the line's anticipation of the next chord rings on into
    /// it, in steps: the tumbao's root on 4 is held to the next "and" of
    /// 2. A line that starts on its own, at the start of a song or a
    /// part, has nothing held over, so it plays that much of its first
    /// chord on the root.
    pub fn held_over(self) -> usize {
        let (period, hits) = self.pattern();
        hits.iter()
            .filter(|hit| hit.what == Tone::Next)
            .map(|hit| (hit.at + hit.len).saturating_sub(period))
            .max()
            .unwrap_or(0)
    }

    /// How many steps the line lasts before it repeats, and its hits, in
    /// bars of `time`. Only [`Bass::Aksak`] depends on the time
    /// signature: its line is a bar long.
    fn hits(self, time: TimeSignature) -> (usize, Cow<'static, [Hit<Tone>]>) {
        match self {
            Bass::Aksak | Bass::Accents => {
                let unit = time.unit_steps();
                let mut hits = Vec::new();
                let mut at = 0;
                for (index, group) in time.felt_groups().into_iter().enumerate() {
                    let (tone, velocity) = match index {
                        0 => (Tone::Root, 104),
                        _ if self == Bass::Accents => (Tone::Root, 92),
                        _ if index % 2 == 1 => (Tone::LowFifth, 92),
                        _ => (Tone::Root, 96),
                    };
                    hits.push(hit!(at, unit, tone, velocity));
                    at += usize::from(group) * unit;
                }
                (time.steps_per_bar(), Cow::Owned(hits))
            }
            _ => {
                let (period, hits) = self.pattern();
                (period, Cow::Borrowed(hits))
            }
        }
    }

    /// How many steps the line lasts before it repeats, and its hits, for
    /// the lines that are the same in every time signature.
    fn pattern(self) -> (usize, &'static [Hit<Tone>]) {
        use Tone::*;
        match self {
            Bass::None | Bass::Held | Bass::Walking | Bass::Aksak | Bass::Accents => (16, &[]),
            Bass::Pulse => (4, &[hit!(0, 2, Root, 100), hit!(2, 2, Root, 84)]),
            Bass::Quarters => (4, &[hit!(0, 2, Root, 104)]),
            Bass::RootFifth => (16, &[hit!(0, 4, Root, 104), hit!(8, 4, LowFifth, 96)]),
            Bass::Octaves => (4, &[hit!(0, 2, Root, 104), hit!(2, 2, Octave, 92)]),
            Bass::Offbeat => (4, &[hit!(2, 2, Root, 104)]),
            Bass::Boogie => (
                16,
                &[
                    hit!(0, 2, Root, 104),
                    hit!(2, 2, Third, 90),
                    hit!(4, 2, Fifth, 96),
                    hit!(6, 2, Seventh, 90),
                    hit!(8, 2, Octave, 100),
                    hit!(10, 2, Seventh, 90),
                    hit!(12, 2, Fifth, 96),
                    hit!(14, 2, Third, 90),
                ],
            ),
            Bass::Bossa => (
                16,
                &[
                    hit!(0, 6, Root, 104),
                    hit!(6, 2, LowFifth, 86),
                    hit!(8, 6, LowFifth, 96),
                    hit!(14, 2, Root, 86),
                ],
            ),
            // The root on 4 rings on to the next bar's fifth.
            Bass::Tumbao => (16, &[hit!(6, 6, Fifth, 96), hit!(12, 10, Next, 104)]),
            Bass::Reggae => (
                16,
                &[
                    hit!(0, 4, Root, 104),
                    hit!(6, 2, Fifth, 86),
                    hit!(8, 2, Octave, 92),
                    hit!(10, 2, Fifth, 84),
                    hit!(12, 4, Root, 96),
                ],
            ),
            Bass::Dembow => (
                8,
                &[
                    hit!(0, 3, Root, 104),
                    hit!(3, 3, Root, 88),
                    hit!(6, 2, Fifth, 92),
                ],
            ),
            Bass::Funk => (
                16,
                &[
                    hit!(0, 2, Root, 110),
                    hit!(3, 1, Root, 80),
                    hit!(6, 1, Octave, 96),
                    hit!(7, 1, Root, 72),
                    hit!(10, 2, Root, 96),
                    hit!(12, 1, Octave, 92),
                    hit!(14, 1, Seventh, 86),
                ],
            ),
            Bass::Sub => (
                16,
                &[
                    hit!(0, 6, Root, 110),
                    hit!(7, 2, Root, 96),
                    hit!(10, 6, Root, 104),
                ],
            ),
            Bass::Lazy => (
                16,
                &[
                    hit!(0, 7, Root, 96),
                    hit!(10, 4, Fifth, 80),
                    hit!(14, 2, Approach, 76),
                ],
            ),
            Bass::Octaves16 => (2, &[hit!(0, 1, Root, 100), hit!(1, 1, Octave, 82)]),
            Bass::Tresillo => (
                16,
                &[
                    hit!(0, 5, Root, 106),
                    hit!(6, 5, Root, 92),
                    hit!(12, 4, Fifth, 96),
                ],
            ),
            Bass::Waltz => (24, &[hit!(0, 4, Root, 100), hit!(12, 4, LowFifth, 90)]),
            Bass::LongShort => (6, &[hit!(0, 4, Root, 104), hit!(4, 2, Root, 78)]),
            Bass::Habanera => (
                8,
                &[
                    hit!(0, 3, Root, 104),
                    hit!(3, 1, Fifth, 84),
                    hit!(4, 2, Third, 92),
                    hit!(6, 2, Fifth, 90),
                ],
            ),
        }
    }

    /// The bass notes for `chord`, from step `start` for `span` steps, in
    /// bars of `time` counted from step 0. `next` is the chord after it,
    /// which some lines lead into.
    pub fn play(
        self,
        time: TimeSignature,
        chord: Chord,
        next: Chord,
        start: usize,
        span: usize,
    ) -> Vec<NoteEvent> {
        self.play_over(time, chord, bass_root(chord), next.root, start, span)
    }

    /// The bass notes for `chord` with `bass` as its lowest note, as in
    /// a slash chord: G/B has B in the bass. The line plays `bass`
    /// wherever it would play the root, and finds the chord's other
    /// notes (its third, fifth and seventh) above it, or its fifth just
    /// below. `next` is the bass note of the chord after it, which some
    /// lines lead into.
    ///
    /// The bars of `time` are counted from step 0, which only matters to
    /// a line that follows the bar ([`Bass::Aksak`]).
    pub fn play_over(
        self,
        time: TimeSignature,
        chord: Chord,
        bass: Note,
        next: PitchClass,
        start: usize,
        span: usize,
    ) -> Vec<NoteEvent> {
        let bar = time.steps_per_bar();
        self.play_from(time, start - start % bar, chord, bass, next, start, span)
    }

    /// [`Bass::play_over`], with a bar of `time` starting on step `bar`,
    /// at or before `start`.
    #[allow(clippy::too_many_arguments)]
    fn play_from(
        self,
        time: TimeSignature,
        bar: usize,
        chord: Chord,
        bass: Note,
        next: PitchClass,
        start: usize,
        span: usize,
    ) -> Vec<NoteEvent> {
        let note = |tone: Tone, at: usize, length: usize, velocity: u8| NoteEvent {
            velocity,
            ..NoteEvent::new(tone_note(tone, chord, bass, next), start + at, length)
        };
        match self {
            Bass::None => Vec::new(),
            Bass::Held => vec![note(Tone::Root, 0, span, 100)],
            Bass::Walking => walk(span)
                .into_iter()
                .map(|(at, tone)| note(tone, at, 4.min(span - at), if at == 0 { 104 } else { 92 }))
                .collect(),
            Bass::Aksak | Bass::Accents => {
                // The line keeps its place in the bar, wherever the chord
                // starts; a chord that starts between two of its notes
                // gets its root at once, as the bass would play it.
                let (period, hits) = self.hits(time);
                let mut notes = Vec::new();
                for offset in (bar..start + span).step_by(period) {
                    for hit in hits.iter() {
                        let at = offset + hit.at;
                        if at < start || at >= start + span {
                            continue;
                        }
                        // The first note of a chord is its root.
                        let tone = if at == start { Tone::Root } else { hit.what };
                        notes.push(note(
                            tone,
                            at - start,
                            hit.len.min(start + span - at),
                            hit.velocity,
                        ));
                    }
                }
                if notes.first().is_none_or(|first| first.start > start) {
                    let length = hits.first().map_or(span, |hit| hit.len).min(span);
                    notes.insert(0, note(Tone::Root, 0, length, 100));
                }
                notes
            }
            _ => {
                let (period, hits) = self.pattern();
                let mut notes = Vec::new();
                for offset in (0..span).step_by(period) {
                    // The next chord is only anticipated in the last period
                    // of this one; before that, it is this chord's root.
                    let last = offset + period >= span;
                    // The anticipation comes as long before the next chord
                    // as the pattern has it before the end of its period,
                    // even under a chord shorter than the period, and the
                    // line holds it from there.
                    let early = hits
                        .iter()
                        .find(|hit| hit.what == Tone::Next)
                        .and_then(|hit| span.checked_sub(period - hit.at))
                        .filter(|_| last);
                    for hit in hits {
                        let at = match (hit.what, early) {
                            (Tone::Next, Some(early)) => early,
                            _ => offset + hit.at,
                        };
                        if at >= span || early.is_some_and(|early| at > early) {
                            continue;
                        }
                        let tone = match hit.what {
                            Tone::Next | Tone::Approach if !last => Tone::Root,
                            tone => tone,
                        };
                        // The next chord's root, played early, belongs to
                        // that chord: it may ring on into it.
                        let length = match tone {
                            Tone::Next => hit.len,
                            _ => hit.len.min(span - at),
                        };
                        notes.push(note(tone, at, length, hit.velocity));
                    }
                }
                notes.sort_by_key(|note| note.start);
                notes
            }
        }
    }
}

/// Makes a bass line play one note at a time: a note held over into
/// the next chord stops where the next note starts, or at `end`, where
/// the chords end.
pub(crate) fn one_at_a_time(notes: &mut [NoteEvent], end: usize) {
    notes.sort_by_key(|note| note.start);
    let starts: Vec<usize> = notes.iter().map(|note| note.start).collect();
    for (index, note) in notes.iter_mut().enumerate() {
        let until = starts[index + 1..]
            .iter()
            .find(|&&start| start > note.start)
            .map_or(end, |&start| start.min(end));
        note.length = note.length.min(until.saturating_sub(note.start));
    }
}

/// The beats of a walking bass line over `span` steps: the root first,
/// a note leading into the next chord last, and chord notes between.
/// A bar of three walks root, fifth, then the lead-in: the third would
/// often be the lead-in itself (C to F: E, then E again).
fn walk(span: usize) -> Vec<(usize, Tone)> {
    const MIDDLE: [Tone; 6] = [
        Tone::Third,
        Tone::Fifth,
        Tone::Seventh,
        Tone::Octave,
        Tone::Fifth,
        Tone::Third,
    ];
    let beats = span.div_ceil(4);
    (0..beats)
        .map(|beat| {
            let tone = if beat == 0 {
                Tone::Root
            } else if beat == beats - 1 {
                Tone::Approach
            } else if beats == 3 {
                Tone::Fifth
            } else {
                MIDDLE[(beat - 1) % MIDDLE.len()]
            };
            (beat * 4, tone)
        })
        .collect()
}

/// The note a bass tone stands for, over `bass`.
fn tone_note(tone: Tone, chord: Chord, bass: Note, next: PitchClass) -> Note {
    let intervals = chord.quality.intervals();
    let has = |interval: u8| intervals.contains(&interval);
    // The note `interval` semitones above the chord's root, in the
    // octave from the bass note up. When the root is in the bass, that
    // is simply the root plus `interval`.
    let up = |interval: u8| {
        let pitch_class = chord.root.transpose(i32::from(interval));
        Note::new(bass.midi() + bass.pitch_class().semitones_up_to(pitch_class))
    };
    // The chord's own fifth: flat in a diminished chord (Bdim: F, not
    // F#), sharp in an augmented one.
    let fifth = if has(6) {
        6
    } else if has(8) {
        8
    } else {
        7
    };
    match tone {
        Tone::Root => bass,
        Tone::Fifth => up(fifth),
        // The same fifth, an octave lower: under the bass note.
        Tone::LowFifth => Note::new(up(fifth).midi().saturating_sub(12)),
        Tone::Octave => Note::new(bass.midi() + 12),
        Tone::Third => up(if has(3) {
            3
        } else if has(4) {
            4
        } else if has(5) {
            5
        } else {
            2
        }),
        // A chord's own seventh or sixth comes first: the double-flat
        // seventh of a diminished seventh chord, the sixth of a minor
        // sixth chord. Only a plain minor chord borrows the flat seventh.
        Tone::Seventh => up(if has(10) {
            10
        } else if has(11) {
            11
        } else if has(9) {
            9
        } else if has(3) {
            10
        } else {
            9
        }),
        Tone::Next => nearest(bass, next),
        Tone::Approach => {
            let target = nearest(bass, next);
            if target > bass {
                Note::new(target.midi() - 1)
            } else {
                Note::new(target.midi() + 1)
            }
        }
    }
}

/// The note of `pitch_class` nearest to `from`: at most six semitones
/// away, and below it when both ways are six.
fn nearest(from: Note, pitch_class: PitchClass) -> Note {
    let up = from.pitch_class().semitones_up_to(pitch_class);
    if up < 6 {
        Note::new(from.midi() + up)
    } else {
        Note::new(from.midi().saturating_sub(12 - up))
    }
}

/// One harmony along a song, for [`Comping::play_in_bars`] and
/// [`Bass::play_in_bars`]: from step `start` to step `end`, played with
/// these notes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Played {
    pub start: usize,
    pub end: usize,
    /// The notes the chords are played with, from low to high. Empty for
    /// no chord: nothing plays.
    pub voicing: Vec<Note>,
    /// What the bass plays.
    pub bass: BassPart,
}

/// What the bass plays under a harmony.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BassPart {
    /// A chord: its line goes over the chord's notes from `note` (its
    /// root, or a slash chord's bass), and may lead into `next`.
    Chord {
        chord: Chord,
        note: Note,
        next: PitchClass,
    },
    /// A group of notes with no root, such as a pitch-class set: the
    /// bass keeps the rhythm of its line on this one note and its octave.
    Pedal(Note),
    /// Nothing.
    Rest,
}

/// Where the hits of a pattern `period` steps long start in `bar`: at
/// the bar's start if the pattern fits in a bar, so it starts again
/// with every bar, as a player counts "one" again; a pattern longer than
/// a bar (the bossa nova's two bars) goes on from the bar before, counted
/// from bar 1.
fn period_start(bar: &Bar, period: usize) -> usize {
    if period <= bar.length {
        return bar.start;
    }
    let bars = period.div_ceil(bar.length.max(1));
    // Where the meter changes from bar to bar, the bars before this one
    // can be shorter than it: the pattern never starts before the song.
    bar.start.saturating_sub((bar.number % bars) * bar.length)
}

impl Comping {
    /// Plays a song's harmonies this way, along its `bars`.
    ///
    /// [`Comping::play`] starts the pattern again at every chord, which
    /// is right for a chart where chords change on the bar line. A song
    /// whose chords change on beat 3, or on the "and" of 4, would then
    /// shift the rhythm every time. Here the rhythm stays in step with
    /// the bars, the way a band keeps its groove whatever the chords do,
    /// and each hit plays the harmony sounding when it falls. A harmony
    /// so short that no hit falls in it is struck once where it starts,
    /// so no chord is skipped.
    pub fn play_in_bars(self, bars: &[Bar], harmonies: &[Played]) -> Vec<NoteEvent> {
        let mut notes = Vec::new();
        if self == Comping::Held {
            for harmony in harmonies {
                notes.extend(self.play(
                    TimeSignature::COMMON,
                    &harmony.voicing,
                    harmony.start,
                    harmony.end - harmony.start,
                ));
            }
            return notes;
        }
        let mut struck = vec![false; harmonies.len()];
        for bar in bars {
            let (period, hits) = self.hits(bar.time);
            let from = if self.follows_the_bar() {
                bar.start
            } else {
                period_start(bar, period)
            };
            for offset in (0..bar.end() - from).step_by(period) {
                for hit in hits.iter() {
                    let at = from + offset + hit.at;
                    if at < bar.start || at >= bar.end() {
                        continue;
                    }
                    let index = harmonies.partition_point(|harmony| harmony.start <= at);
                    let Some(index) = index.checked_sub(1) else {
                        continue;
                    };
                    let harmony = &harmonies[index];
                    if at >= harmony.end {
                        continue;
                    }
                    struck[index] = true;
                    let length = hit.len.min(harmony.end - at).min(bar.end() - at);
                    for pitch in pick(&harmony.voicing, hit.what) {
                        notes.push(NoteEvent {
                            velocity: hit.velocity,
                            ..NoteEvent::new(pitch, at, length)
                        });
                    }
                }
            }
        }
        let time = bars.first().map_or(TimeSignature::COMMON, |bar| bar.time);
        if let Some(first) = self.hits(time).1.first() {
            for (harmony, _) in harmonies
                .iter()
                .zip(&struck)
                .filter(|(_, struck)| !**struck)
            {
                let length = first.len.min(harmony.end - harmony.start);
                for &pitch in &harmony.voicing {
                    notes.push(NoteEvent {
                        velocity: first.velocity,
                        ..NoteEvent::new(pitch, harmony.start, length)
                    });
                }
            }
        }
        notes.sort_by_key(|note| (note.start, note.pitch));
        notes
    }
}

impl Bass {
    /// Plays the bass under a song's harmonies, along its `bars`.
    ///
    /// The line starts again where each harmony starts, so the root is
    /// played on the change, and again at each bar line, or every other
    /// one for a line two bars long (the waltz's root, then fifth), so
    /// a chord held for many bars keeps the line in step with the bars.
    pub fn play_in_bars(self, bars: &[Bar], harmonies: &[Played]) -> Vec<NoteEvent> {
        let (period, _) = self.pattern();
        // A held or walking bass has no pattern of its own to keep: it
        // starts again with every bar, whatever the bar's length. So does
        // the line of an odd meter, which is a bar long.
        let period = match self {
            Bass::None | Bass::Held | Bass::Walking | Bass::Aksak | Bass::Accents => 0,
            _ => period,
        };
        let mut notes = Vec::new();
        for harmony in harmonies {
            // Where the line starts again: the harmony's start, and the
            // bar lines inside it where a period of the line begins.
            let mut cuts = vec![harmony.start];
            for bar in bars {
                if bar.start > harmony.start
                    && bar.start < harmony.end
                    && period_start(bar, period.max(bar.length)) == bar.start
                {
                    cuts.push(bar.start);
                }
            }
            cuts.push(harmony.end);
            for (index, piece) in cuts.windows(2).enumerate() {
                let (start, span) = (piece[0], piece[1] - piece[0]);
                let last = index + 2 == cuts.len();
                // The bar the piece is in, for a line that follows it.
                let (time, bar) = bars
                    .iter()
                    .rfind(|bar| bar.start <= start)
                    .map_or((TimeSignature::COMMON, start), |bar| (bar.time, bar.start));
                notes.extend(match harmony.bass {
                    BassPart::Chord { chord, note, next } => {
                        // Only the last piece leads into the next chord.
                        let next = if last { next } else { note.pitch_class() };
                        self.play_from(time, bar, chord, note, next, start, span)
                    }
                    BassPart::Pedal(note) => self.pedal(time, bar, note, start, span),
                    BassPart::Rest => Vec::new(),
                });
            }
        }
        // Nothing is held over into the first harmony: it starts on its
        // root (see `held_over`).
        if let Some(first) = harmonies.first()
            && let BassPart::Chord { note, .. } = first.bass
            && self.held_over() > 0
        {
            let length = self.held_over().min(first.end - first.start);
            notes.push(NoteEvent::new(note, first.start, length));
        }
        let end = harmonies.last().map_or(0, |harmony| harmony.end);
        one_at_a_time(&mut notes, end);
        notes
    }

    /// The line's rhythm on one note: every tone of the line is `note`,
    /// but its octave stays an octave. For a group of notes with no root,
    /// whose third or fifth would not be in it.
    fn pedal(
        self,
        time: TimeSignature,
        bar: usize,
        note: Note,
        start: usize,
        span: usize,
    ) -> Vec<NoteEvent> {
        let chord = Chord::new(note.pitch_class(), crate::harmony::ChordQuality::Major);
        self.play_from(time, bar, chord, note, note.pitch_class(), start, span)
            .into_iter()
            .map(|played| {
                let octave = played.pitch.midi() == note.midi() + 12;
                NoteEvent {
                    pitch: if octave { played.pitch } else { note },
                    ..played
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harmony::ChordQuality::*;
    use crate::pitch::PitchClass::*;

    const FOUR: TimeSignature = TimeSignature::COMMON;

    fn c_major() -> Vec<Note> {
        vec![Note::new(60), Note::new(64), Note::new(67)]
    }

    fn starts(notes: &[NoteEvent]) -> Vec<usize> {
        let mut starts: Vec<usize> = notes.iter().map(|note| note.start).collect();
        starts.dedup();
        starts
    }

    #[test]
    fn plain_accents_follow_the_grouping_without_a_styled_bass_line() {
        let time = TimeSignature::of_groups(&[2, 2, 3], 8);
        let notes = Comping::Accents.play(time, &c_major(), 0, 28);
        assert_eq!(starts(&notes), [0, 4, 8, 14, 18, 22]);
        let chord = Chord::new(C, Major);
        let bass = Bass::Accents.play(time, chord, chord, 0, 28);
        assert_eq!(starts(&bass), [0, 4, 8, 14, 18, 22]);
        assert!(bass.iter().all(|note| note.pitch.pitch_class() == C));
        assert!(notes[0].velocity > notes[3].velocity);
    }

    #[test]
    fn held_chords_last_the_whole_span() {
        let notes = Comping::Held.play(FOUR, &c_major(), 16, 32);
        assert_eq!(notes.len(), 3);
        assert!(
            notes
                .iter()
                .all(|note| note.start == 16 && note.length == 32)
        );
    }

    #[test]
    fn patterns_repeat_and_stop_at_the_end_of_the_chord() {
        let notes = Comping::Charleston.play(FOUR, &c_major(), 0, 24);
        // Beat 1 and the "and" of 2 in the first bar, then only beat 1
        // and the "and" of 2 of the half bar that is left.
        assert_eq!(starts(&notes), [0, 6, 16, 22]);
        let last = notes.last().unwrap();
        assert_eq!(last.start + last.length, 24);
    }

    #[test]
    fn arpeggios_climb_into_the_next_octave() {
        let notes = Comping::ArpUp.play(FOUR, &c_major(), 0, 8);
        let pitches: Vec<u8> = notes.iter().map(|note| note.pitch.midi()).collect();
        assert_eq!(pitches, [60, 64, 67, 72]);
    }

    #[test]
    fn a_walking_line_leads_into_the_next_chord() {
        let c = Chord::new(C, Dominant7);
        let f = Chord::new(F, Dominant7);
        let notes = Bass::Walking.play(FOUR, c, f, 0, 16);
        let pitches: Vec<u8> = notes.iter().map(|note| note.pitch.midi()).collect();
        // C2, E2, G2, then E2 a semitone under F2.
        assert_eq!(pitches, [36, 40, 43, 40]);
    }

    #[test]
    fn the_tumbao_anticipates_only_at_the_end() {
        let c = Chord::new(C, Major);
        let g = Chord::new(G, Major);
        let notes = Bass::Tumbao.play(FOUR, c, g, 0, 32);
        let pitches: Vec<u8> = notes.iter().map(|note| note.pitch.midi()).collect();
        // Fifth, root; fifth, then G (the next root, nearest to C2: G1).
        assert_eq!(pitches, [43, 36, 43, 31]);
        // The anticipated G rings over the bar line, up to the next fifth.
        let last = notes.last().unwrap();
        assert_eq!((last.start, last.length), (28, 10));
    }

    #[test]
    fn the_low_fifth_is_the_chords_own_fifth() {
        // Bm7b5: the fifth under B is F, not F#.
        let b = Chord::new(B, HalfDiminished7);
        let notes = Bass::RootFifth.play(FOUR, b, b, 0, 16);
        let pitches: Vec<u8> = notes.iter().map(|note| note.pitch.midi()).collect();
        assert_eq!(pitches, [47, 41]);
        // C: G under C.
        let c = Chord::new(C, Major);
        let notes = Bass::RootFifth.play(FOUR, c, c, 0, 16);
        let pitches: Vec<u8> = notes.iter().map(|note| note.pitch.midi()).collect();
        assert_eq!(pitches, [36, 31]);
    }

    #[test]
    fn the_seventh_is_one_the_chord_has() {
        let seventh = |chord: Chord| {
            tone_note(Tone::Seventh, chord, bass_root(chord), chord.root).midi()
                - bass_root(chord).midi()
        };
        // Am6 has F#, not G.
        assert_eq!(seventh(Chord::new(A, Minor6)), 9);
        // Cdim7 has its double-flat seventh, Bbb.
        assert_eq!(seventh(Chord::new(C, Diminished7)), 9);
        // A plain minor chord borrows the flat seventh.
        assert_eq!(seventh(Chord::new(A, Minor)), 10);
        assert_eq!(seventh(Chord::new(C, Major7)), 11);
    }

    #[test]
    fn boogie_walks_up_the_chord() {
        let c = Chord::new(C, Dominant7);
        let notes = Bass::Boogie.play(FOUR, c, c, 0, 16);
        let pitches: Vec<u8> = notes.iter().map(|note| note.pitch.midi()).collect();
        assert_eq!(pitches, [36, 40, 43, 46, 48, 46, 43, 40]);
    }

    #[test]
    fn a_slash_chord_puts_its_bass_note_where_the_root_was() {
        let c = Chord::new(C, Major);
        let pitches = |notes: Vec<NoteEvent>| -> Vec<u8> {
            notes.iter().map(|note| note.pitch.midi()).collect()
        };
        // C/E with the bass on E2: root and fifth become E2 and the G
        // below it.
        let e2 = Note::new(40);
        assert_eq!(
            pitches(Bass::RootFifth.play_over(FOUR, c, e2, F, 0, 16)),
            [40, 31]
        );
        assert_eq!(
            pitches(Bass::Octaves.play_over(FOUR, c, e2, F, 0, 4)),
            [40, 52]
        );
        // A walking line over C/E starts on E and leads into the next
        // bass note, D, from a semitone above.
        let walk = pitches(Bass::Walking.play_over(FOUR, c, e2, D, 0, 16));
        assert_eq!(walk.first(), Some(&40));
        assert_eq!(walk.last(), Some(&39), "Eb2 leads down into D2");
        // In a bar of three: C2, G2, then E2 into F, not C2 E2 E2.
        let c2 = Note::new(36);
        let walk = pitches(Bass::Walking.play_over(FOUR, c, c2, F, 0, 12));
        assert_eq!(walk, [36, 43, 40]);
        // With the root in the bass, play_over is play.
        let g = Chord::new(G, Dominant7);
        for bass in [Bass::Walking, Bass::Boogie, Bass::Tumbao, Bass::Reggae] {
            assert_eq!(
                bass.play_over(FOUR, c, bass_root(c), G, 0, 32),
                bass.play(FOUR, c, g, 0, 32),
                "{bass:?}"
            );
        }
    }
}
