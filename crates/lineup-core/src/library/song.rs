//! Well-known songs, with their melodies.
//!
//! Almost every song here is old enough to belong to everyone: folk
//! songs nobody owns, and pieces by composers who died long ago. A few
//! newer ones are still under copyright in some countries or in all of
//! them, such as Hit the Road Jack (1960); a comment above each says
//! so. The melodies are written in the notation of
//! [`chart::parse_melody`] and the chords as chord charts, one bar at a
//! time, exactly like a lead sheet. Each song is dressed in a style (a
//! house track, a swing band, lo-fi...) by its comping, bass and rhythm,
//! the same building blocks the harmony styles use.
//!
//! The songs live in one file per genre, in the `song/` folder next to
//! this one, and [`SONGS`] lists them all. Songs marked "Written for
//! Ferrous Lineup" are new tunes, written to show off a genre that has
//! few well-known tunes old enough to be free.

use crate::general_midi as gm;
use crate::harmony::{Scale, ScaleKind};
use crate::meter::TimeSignature;
use crate::pitch::PitchClass;

use super::Genre;
use super::chart::{self, ChordSpan, Melody};
use super::comping::{Bass, Comping};
use super::rhythm::Rhythm;

/// One section of a song, such as a verse or a chorus. Each part
/// becomes one pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Part {
    pub name: &'static str,
    /// The tune, played by the Lead channel.
    pub melody: &'static str,
    /// The chords, accompanied in the song's comping and bass style, and
    /// written on the chord sheet. A chart may name them as numerals,
    /// "i | V7", or by name as a lead sheet does, "C#m | G#7/B#", read in
    /// the part's key (see [`chart::parse_chart_in`]).
    pub chords: &'static str,
    /// Notes for the Keys channel written out in full, for pieces whose
    /// accompaniment is part of the music. When there are any, the
    /// chords are not accompanied.
    pub left: &'static str,
    /// Who sings the melody, and from which bar of the part: one
    /// `(voice, bar)` for each copy. One voice is `[(1, 0)]`. In a round
    /// or a canon, each voice sings the same tune a few bars after the
    /// one before: `[(1, 0), (2, 2), (3, 4)]`. A negative bar is a voice
    /// that began in the part before: only its end is heard. Notes
    /// outside the part are left out. Voice 1 is the Lead channel; every
    /// other voice gets a channel of its own, "Voice 2", "Voice 3"...
    pub voices: &'static [(usize, i32)],
    /// A time signature of its own, for a part in another time than the
    /// rest of the song. The demo puts a time marker where it starts.
    pub time: Option<TimeSignature>,
    /// The time signature of each bar, for music whose meter changes
    /// from bar to bar, as in Mussorgsky's Promenade: 5/4, 6/4, 5/4...
    /// Empty when every bar is in the part's time. The demo puts a time
    /// marker wherever the meter changes, and a pattern for each run of
    /// bars in one time signature, since a pattern has one.
    pub meters: &'static [TimeSignature],
    /// The key this part is in, when it is not the song's. Its chords
    /// are read in this key, so a piece that modulates has a part for
    /// each key it visits.
    pub key: Option<(PitchClass, ScaleKind)>,
    /// The tempo markings of the score inside this part, in order. The
    /// demo puts a tempo marker on the playlist for each, every time the
    /// part is played. After the part, the song carries on at the tempo
    /// it got to.
    pub tempo: &'static [TempoMark],
    /// How this part is accompanied, when it is not the way the rest of
    /// the song is: its comping, its bass line and its rhythm, a name in
    /// [`super::RHYTHMS`] ("" for no drums). A refrain in 3/4 in a song
    /// in 2/4 needs a waltz's comping and a rhythm in three.
    pub played: Option<(Comping, Bass, &'static str)>,
}

/// The General MIDI instruments a song's channels play (see
/// [`crate::general_midi`]): the melody (the Lead channel, and every other
/// voice of a round or a canon), the chords or written-out accompaniment
/// (Keys) and the bass, and a big band's horn section. A piece from the
/// concert hall gets the instruments it was written for, and a song
/// dressed in a style gets that style's band.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Band {
    pub lead: u8,
    pub keys: u8,
    pub bass: u8,
    pub horns: Option<Horns>,
}

/// A horn section behind the melody, on a channel of its own ("Horns"):
/// it plays the song's chords, voiced as the Keys channel voices them,
/// in a comping of its own. A big band's saxophones hold the chords
/// under the trumpet's tune ([`Comping::Held`]), and its brass punch
/// them on the beat and just before beat 3 ([`Comping::Charleston`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Horns {
    pub program: u8,
    pub comping: Comping,
}

impl Band {
    const fn new(lead: u8, keys: u8, bass: u8) -> Band {
        Band {
            lead,
            keys,
            bass,
            horns: None,
        }
    }

    /// One instrument playing everything, such as a piano piece.
    const fn solo(instrument: u8) -> Band {
        Band::new(instrument, instrument, instrument)
    }

    /// The same band with a horn section playing `program` in `comping`.
    const fn with_horns(self, program: u8, comping: Comping) -> Band {
        Band {
            horns: Some(Horns { program, comping }),
            ..self
        }
    }
}

/// A tempo marking in a score, inside a part. `at` counts steps from the
/// start of the part.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TempoMark {
    /// From `at` on, `bpm` beats a minute, at once: a new section with
    /// a new tempo, such as "Più mosso" (more movement), "Adagio" or
    /// "Tempo I" (the first tempo again).
    At { at: usize, bpm: u16 },
    /// From `at`, slowing down or speeding up bit by bit until `bpm`,
    /// reached `over` steps later: "rit." or "rall." (slowing down),
    /// "accel." or "stringendo" (speeding up).
    Glide { at: usize, bpm: u16, over: usize },
    /// A fermata (𝄐), the "bird's eye" over a note or a rest: from `at`,
    /// for `steps` steps, the music waits, lasting twice as long; then
    /// it carries on at the tempo before.
    Fermata { at: usize, steps: usize },
}

/// A new tempo from `at`, at once.
const fn tempo(at: usize, bpm: u16) -> TempoMark {
    TempoMark::At { at, bpm }
}

/// A gradual change from `at`, reaching `bpm` after `over` steps.
const fn glide(at: usize, bpm: u16, over: usize) -> TempoMark {
    TempoMark::Glide { at, bpm, over }
}

/// A fermata from `at`, `steps` steps long.
const fn fermata(at: usize, steps: usize) -> TempoMark {
    TempoMark::Fermata { at, steps }
}

impl Part {
    /// A part with one voice and no written-out accompaniment.
    const fn new(name: &'static str, melody: &'static str, chords: &'static str) -> Part {
        Part {
            name,
            melody,
            chords,
            left: "",
            voices: &[(1, 0)],
            time: None,
            meters: &[],
            key: None,
            tempo: &[],
            played: None,
        }
    }

    const fn with_left(self, left: &'static str) -> Part {
        Part { left, ..self }
    }

    const fn with_voices(self, voices: &'static [(usize, i32)]) -> Part {
        Part { voices, ..self }
    }

    /// The same part in another time signature than the song's, such as
    /// a verse in 3/4 in a song in 4/4.
    const fn in_time(self, time: TimeSignature) -> Part {
        Part {
            time: Some(time),
            ..self
        }
    }

    /// The same part with a time signature for each bar, for music whose
    /// meter changes from bar to bar.
    const fn in_meters(self, meters: &'static [TimeSignature]) -> Part {
        Part { meters, ..self }
    }

    /// The same part with the tempo markings of the score.
    const fn with_tempo(self, tempo: &'static [TempoMark]) -> Part {
        Part { tempo, ..self }
    }

    /// The same part accompanied another way than the rest of the song.
    const fn played(self, comping: Comping, bass: Bass, rhythm: &'static str) -> Part {
        Part {
            played: Some((comping, bass, rhythm)),
            ..self
        }
    }

    /// The same part in another key: a modulation.
    const fn in_key(self, tonic: PitchClass, mode: ScaleKind) -> Part {
        Part {
            key: Some((tonic, mode)),
            ..self
        }
    }

    pub fn melody(&self) -> Melody {
        chart::parse_melody(self.melody).unwrap_or(Melody {
            notes: Vec::new(),
            bars: Vec::new(),
            length: 0,
        })
    }

    pub fn left(&self) -> Melody {
        chart::parse_melody(self.left).unwrap_or(Melody {
            notes: Vec::new(),
            bars: Vec::new(),
            length: 0,
        })
    }

    /// The chords of the chart, `bar` steps to the bar, in the key whose
    /// tonic is `tonic`.
    pub fn spans(&self, bar: usize, tonic: PitchClass) -> Vec<ChordSpan> {
        chart::parse_chart_in(self.chords, bar, tonic).unwrap_or_default()
    }

    /// How many steps the part lasts: the longest of its melody, its
    /// chords and its written-out accompaniment.
    pub fn length(&self, bar: usize) -> usize {
        let chords = if self.chords.trim().is_empty() {
            0
        } else {
            self.chords.split('|').count() * bar
        };
        self.melody().length.max(self.left().length).max(chords)
    }
}

/// A song, its parts, and how they are put together.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Song {
    pub name: &'static str,
    /// Who wrote it and when, or where it comes from.
    pub origin: &'static str,
    /// The style it is dressed in here, such as "Swing" or "Lo-fi".
    pub feel: &'static str,
    /// The family of that style, which the Demos menu sorts songs by.
    pub genre: Genre,
    pub tonic: PitchClass,
    pub mode: ScaleKind,
    pub time: TimeSignature,
    pub tempo: f32,
    pub comping: Comping,
    pub bass: Bass,
    /// A rhythm from [`super::RHYTHMS`], or "" for no drums.
    pub rhythm: &'static str,
    /// The instruments that play it.
    pub band: Band,
    /// The style in [`super::STYLES`] whose harmony and playing the song
    /// is closest to, or "" for none. The library shows it, and the
    /// rhythm, selected while the demo is open.
    pub style: &'static str,
    pub parts: &'static [Part],
    /// The order the parts are played in, by name.
    pub form: &'static [&'static str],
    /// The drums come in at this entry of `form` (0 for the start).
    pub drums_from: usize,
    pub about: &'static str,
}

impl Song {
    /// How much the song swings (see [`crate::swing`]): as much as its
    /// rhythm does, except in folk and classical music, whose eighths
    /// stay even over any beat: Scarborough Fair keeps its even eighths
    /// over the ride cymbal of a jazz waltz.
    pub fn swing(&self) -> u8 {
        match self.genre {
            Genre::Folk | Genre::Classical => 0,
            _ => Rhythm::named(self.rhythm).map_or(0, |rhythm| rhythm.swing),
        }
    }

    pub fn scale(&self) -> Scale {
        Scale::new(self.tonic, self.mode)
    }

    /// The time signature `part` is in: its own, or else the song's. A
    /// part whose meter changes from bar to bar is in the time of its
    /// first bar.
    pub fn part_time(&self, part: &Part) -> TimeSignature {
        part.meters
            .first()
            .copied()
            .or(part.time)
            .unwrap_or(self.time)
    }

    /// `part` cut where its meter changes: one part for each run of bars
    /// in one time signature, each with its own time and its own share
    /// of the melody, the chords and the written-out accompaniment. A
    /// part in one time signature is a single run, itself. The pieces
    /// keep the part's name, voices and key, and its tempo markings go
    /// with the first.
    pub fn part_runs(&self, part: &Part) -> Vec<Part> {
        if part.meters.is_empty() {
            return vec![*part];
        }
        let mut runs: Vec<(usize, usize, TimeSignature)> = Vec::new();
        for (bar, &time) in part.meters.iter().enumerate() {
            match runs.last_mut() {
                Some((_, bars, last)) if *last == time => *bars += 1,
                _ => runs.push((bar, 1, time)),
            }
        }
        runs.iter()
            .enumerate()
            .map(|(index, &(first, bars, time))| Part {
                melody: bars_of(part.melody, first, bars),
                chords: bars_of(part.chords, first, bars),
                left: bars_of(part.left, first, bars),
                time: Some(time),
                meters: &[],
                tempo: if index == 0 { part.tempo } else { &[] },
                ..*part
            })
            .collect()
    }

    /// How `part` is accompanied: its comping, bass line and rhythm, its
    /// own or else the song's.
    pub fn part_playing(&self, part: &Part) -> (Comping, Bass, &'static str) {
        part.played
            .unwrap_or((self.comping, self.bass, self.rhythm))
    }

    /// The key `part` is in: its own, if it modulates, or the song's.
    pub fn part_scale(&self, part: &Part) -> Scale {
        match part.key {
            Some((tonic, mode)) => Scale::new(tonic, mode),
            None => self.scale(),
        }
    }

    /// How many voices sing the melody at most: 1, or more in a round.
    pub fn voices(&self) -> usize {
        self.parts
            .iter()
            .flat_map(|part| part.voices)
            .map(|&(voice, _)| voice)
            .max()
            .unwrap_or(1)
    }

    pub fn part(&self, name: &str) -> Option<&'static Part> {
        self.parts.iter().find(|part| part.name == name)
    }

    /// The song called `name`, if there is one.
    pub fn named(name: &str) -> Option<&'static Song> {
        SONGS.iter().find(|song| song.name == name)
    }
}

/// Bars `first` to `first + count` of a melody or a chart: the text
/// between its bar lines. Text that is a slice of `'static` text lives
/// as long, so the pieces of a part are parts too.
fn bars_of(text: &'static str, first: usize, count: usize) -> &'static str {
    if text.trim().is_empty() {
        return text;
    }
    // Where each bar starts and ends, in bytes.
    let mut cuts = vec![0];
    cuts.extend(text.match_indices('|').map(|(at, _)| at));
    cuts.push(text.len());
    let start = cuts[first.min(cuts.len() - 1)];
    let end = cuts[(first + count).min(cuts.len() - 1)];
    // Leave out the bar line the piece starts with.
    text[start..end].trim_start_matches('|').trim()
}

// Every genre file starts with `use super::*;`, which brings in all the
// names below: `Song`, `Part`, the time signatures, and the notes and
// scales without their type in front, so a song can say `tonic: A`.
use PitchClass::*;
use ScaleKind::*;

const COMMON: TimeSignature = TimeSignature::COMMON;
const TWO_FOUR: TimeSignature = TimeSignature::new(2, 4);
const THREE_FOUR: TimeSignature = TimeSignature::new(3, 4);
const SIX_EIGHT: TimeSignature = TimeSignature::new(6, 8);
const NINE_EIGHT: TimeSignature = TimeSignature::new(9, 8);
const TWELVE_EIGHT: TimeSignature = TimeSignature::new(12, 8);
// Six quarter notes, each a beat, as in Mussorgsky's Promenade.
const SIX_FOUR: TimeSignature = TimeSignature::new(6, 4);
// Odd meters, with the beats they are felt in: 3 + 2 quarters, 2 + 3
// eighths...
const FIVE_FOUR: TimeSignature = TimeSignature::of_groups(&[3, 2], 4);
// Five quarter notes felt the other way round, 2 + 3: Tchaikovsky's waltz in five.
const FIVE_FOUR_TWO_THREE: TimeSignature = TimeSignature::of_groups(&[2, 3], 4);
const FIVE_EIGHT: TimeSignature = TimeSignature::of_groups(&[2, 3], 8);
// 5/8 the other way round, a long beat and then a short one, 3 + 2.
const FIVE_EIGHT_THREE_TWO: TimeSignature = TimeSignature::of_groups(&[3, 2], 8);
const THREE_TWO_TWO: TimeSignature = TimeSignature::of_groups(&[3, 2, 2], 8);
const TWO_TWO_THREE: TimeSignature = TimeSignature::of_groups(&[2, 2, 3], 8);
const NINE_EIGHT_AKSAK: TimeSignature = TimeSignature::of_groups(&[2, 2, 2, 3], 8);
const RAKS_AKSAGI: TimeSignature = TimeSignature::of_groups(&[2, 3, 2, 2], 8);
const CURCUNA: TimeSignature = TimeSignature::of_groups(&[3, 2, 2, 3], 8);
const ELEVEN_EIGHT: TimeSignature = TimeSignature::of_groups(&[2, 2, 3, 2, 2], 8);
const FIFTEEN_EIGHT: TimeSignature = TimeSignature::of_groups(&[2, 2, 2, 2, 3, 2, 2], 8);

// The bands the songs are dressed in, named after their style. Each is
// melody, chords, bass. A piece from the concert hall names its own
// instruments instead.
const PIANO: Band = Band::solo(gm::ACOUSTIC_GRAND_PIANO);
const CHOIR: Band = Band::solo(gm::CHOIR_AAHS);
const SYNTHS: Band = Band::new(gm::SAW_LEAD, gm::WARM_PAD, gm::SYNTH_BASS);
const DISCO: Band = Band::new(gm::STRING_ENSEMBLE, gm::ELECTRIC_PIANO, gm::FINGER_BASS);
const BLUES_HARP: Band = Band::new(gm::HARMONICA, gm::ACOUSTIC_GRAND_PIANO, gm::FINGER_BASS);
const BLUES_GUITAR: Band = Band::new(gm::JAZZ_GUITAR, gm::ACOUSTIC_GRAND_PIANO, gm::ACOUSTIC_BASS);
const SLOW_BLUES: Band = Band::new(gm::CLEAN_GUITAR, gm::ACOUSTIC_GRAND_PIANO, gm::FINGER_BASS);
const GUITARS: Band = Band::new(gm::NYLON_GUITAR, gm::NYLON_GUITAR, gm::ACOUSTIC_BASS);
const SONG_AND_GUITAR: Band = Band::new(gm::VOICE_OOHS, gm::NYLON_GUITAR, gm::ACOUSTIC_BASS);
const CAROL: Band = Band::new(gm::CHOIR_AAHS, gm::NYLON_GUITAR, gm::ACOUSTIC_BASS);
const FLUTE_AND_GUITAR: Band = Band::new(gm::FLUTE, gm::NYLON_GUITAR, gm::ACOUSTIC_BASS);
const COUNTRY: Band = Band::new(gm::FIDDLE, gm::STEEL_GUITAR, gm::ACOUSTIC_BASS);
const BLUEGRASS: Band = Band::new(gm::FIDDLE, gm::BANJO, gm::ACOUSTIC_BASS);
const FOLK: Band = Band::new(gm::FLUTE, gm::STEEL_GUITAR, gm::ACOUSTIC_BASS);
const JIG: Band = Band::new(gm::FIDDLE, gm::STEEL_GUITAR, gm::ACOUSTIC_BASS);
const ACCORDION_BAND: Band = Band::new(gm::ACCORDION, gm::NYLON_GUITAR, gm::ACOUSTIC_BASS);
const BALLAD: Band = Band::new(gm::VIOLIN, gm::ACOUSTIC_GRAND_PIANO, gm::CELLO);
const KLEZMER: Band = Band::new(gm::CLARINET, gm::ACCORDION, gm::ACOUSTIC_BASS);
const BALKAN: Band = Band::new(gm::CLARINET, gm::ACCORDION, gm::ACOUSTIC_BASS);
const KAVAL: Band = Band::new(gm::FLUTE, gm::ACCORDION, gm::ACOUSTIC_BASS);
const GREEK: Band = Band::new(gm::VIOLIN, gm::STEEL_GUITAR, gm::ACOUSTIC_BASS);
// The kanun, the Turkish zither, is played by a koto, the nearest General MIDI sound.
const TURKISH: Band = Band::new(gm::CLARINET, gm::KOTO, gm::ACOUSTIC_BASS);
// The kemençe of the Black Sea coast, a small fiddle played upright, by a fiddle.
const KEMENCE: Band = Band::new(gm::FIDDLE, gm::KOTO, gm::ACOUSTIC_BASS);
const POLKA: Band = Band::new(gm::VIOLIN, gm::ACCORDION, gm::TUBA);
// A Bavarian dance band: clarinet, the button accordion and the tuba.
const BAVARIAN: Band = Band::new(gm::CLARINET, gm::ACCORDION, gm::TUBA);
// A Romanian band of lăutari: the violin leads, over accordion and double bass.
const TARAF: Band = Band::new(gm::VIOLIN, gm::ACCORDION, gm::ACOUSTIC_BASS);
const ANDEAN: Band = Band::new(gm::PAN_FLUTE, gm::NYLON_GUITAR, gm::ACOUSTIC_BASS);
const LLANERO: Band = Band::new(gm::HARP, gm::NYLON_GUITAR, gm::ACOUSTIC_BASS);
const FIDDLE_AND_GUITAR: Band = Band::new(gm::VIOLIN, gm::NYLON_GUITAR, gm::ACOUSTIC_BASS);
const CANDOMBE: Band = Band::new(gm::NYLON_GUITAR, gm::ACOUSTIC_GRAND_PIANO, gm::FINGER_BASS);
const LO_FI: Band = Band::new(gm::ELECTRIC_PIANO, gm::ELECTRIC_PIANO, gm::FINGER_BASS);
const BOOM_BAP: Band = Band::new(
    gm::ACOUSTIC_GRAND_PIANO,
    gm::ACOUSTIC_GRAND_PIANO,
    gm::ACOUSTIC_BASS,
);
const TRAP: Band = Band::new(gm::CHOIR_AAHS, gm::STRING_ENSEMBLE, gm::SYNTH_BASS);
const SWING: Band = Band::new(gm::ALTO_SAX, gm::ACOUSTIC_GRAND_PIANO, gm::ACOUSTIC_BASS);
// A big band: a trumpet with the tune over the saxophones holding the
// chords; the saxophones' tune with the brass punching the chords; and a
// tenor saxophone's ballad over the trombones.
const BIG_BAND: Band = Band::new(gm::TRUMPET, gm::ACOUSTIC_GRAND_PIANO, gm::ACOUSTIC_BASS)
    .with_horns(gm::TENOR_SAX, Comping::Held);
const SAX_SECTION: Band = Band::new(gm::ALTO_SAX, gm::ACOUSTIC_GRAND_PIANO, gm::ACOUSTIC_BASS)
    .with_horns(gm::BRASS_SECTION, Comping::Charleston);
const BALLAD_BAND: Band = Band::new(gm::TENOR_SAX, gm::ACOUSTIC_GRAND_PIANO, gm::ACOUSTIC_BASS)
    .with_horns(gm::TROMBONE, Comping::Held);
const DIXIELAND: Band = Band::new(gm::TRUMPET, gm::ACOUSTIC_GRAND_PIANO, gm::TUBA);
const JAZZ_WALTZ: Band = Band::new(gm::VIBRAPHONE, gm::ACOUSTIC_GRAND_PIANO, gm::ACOUSTIC_BASS);
const GYPSY_JAZZ: Band = Band::new(gm::VIOLIN, gm::JAZZ_GUITAR, gm::ACOUSTIC_BASS);
const SAMBA: Band = Band::new(gm::FLUTE, gm::NYLON_GUITAR, gm::FINGER_BASS);
const AFROBEATS: Band = Band::new(gm::MARIMBA, gm::ELECTRIC_PIANO, gm::SYNTH_BASS);
const POP: Band = Band::new(gm::SQUARE_LEAD, gm::ACOUSTIC_GRAND_PIANO, gm::FINGER_BASS);
const SKA: Band = Band::new(gm::TRUMPET, gm::CLEAN_GUITAR, gm::FINGER_BASS);
const REGGAE: Band = Band::new(gm::CLEAN_GUITAR, gm::DRAWBAR_ORGAN, gm::FINGER_BASS);
const ROCK: Band = Band::new(
    gm::OVERDRIVEN_GUITAR,
    gm::DISTORTION_GUITAR,
    gm::FINGER_BASS,
);
const ROCK_AND_ROLL: Band = Band::new(gm::TENOR_SAX, gm::ACOUSTIC_GRAND_PIANO, gm::FINGER_BASS);
const BRASS_BAND: Band = Band::new(gm::TRUMPET, gm::BRASS_SECTION, gm::TUBA);
const GOSPEL: Band = Band::new(gm::CHOIR_AAHS, gm::DRAWBAR_ORGAN, gm::FINGER_BASS);
const SOUL: Band = Band::new(gm::VOICE_OOHS, gm::DRAWBAR_ORGAN, gm::FINGER_BASS);
const MOTOWN: Band = Band::new(gm::VOICE_OOHS, gm::ACOUSTIC_GRAND_PIANO, gm::FINGER_BASS);
const FUNK: Band = Band::new(gm::BRASS_SECTION, gm::CLAVINET, gm::SLAP_BASS);
const NEO_SOUL: Band = Band::new(gm::VOICE_OOHS, gm::ELECTRIC_PIANO, gm::FINGER_BASS);
const SPIRITUAL: Band = Band::new(gm::VOICE_OOHS, gm::ACOUSTIC_GRAND_PIANO, gm::ACOUSTIC_BASS);
const TANGO: Band = Band::new(
    gm::TANGO_ACCORDION,
    gm::ACOUSTIC_GRAND_PIANO,
    gm::ACOUSTIC_BASS,
);
const SALON: Band = Band::new(gm::VIOLIN, gm::ACOUSTIC_GRAND_PIANO, gm::ACOUSTIC_BASS);
/// Trumpet, the steel strings of the Cuban tres, and the bass.
const SON: Band = Band::new(gm::TRUMPET, gm::STEEL_GUITAR, gm::ACOUSTIC_BASS);
/// Trumpets, the vihuela and guitars, and the guitarrón.
const MARIACHI: Band = Band::new(gm::TRUMPET, gm::NYLON_GUITAR, gm::ACOUSTIC_BASS);

mod balkan;
mod blues;
mod classical;
mod electronic;
mod flamenco;
mod folk;
mod folklore;
mod hiphop;
mod jazz;
mod latin;
mod pop;
mod reggae;
mod rock;
mod soul;
mod tango;

/// All the songs, sorted by genre in the order of [`Genre::ALL`].
pub static SONGS: &[Song] = &[
    pop::AULD_LANG_SYNE,
    pop::LONDON_BRIDGE,
    pop::OLD_MACDONALD,
    pop::DECK_THE_HALLS,
    pop::THREE_BLIND_MICE,
    pop::SKIP_TO_MY_LOU,
    pop::POP_GOES_THE_WEASEL,
    pop::FUNICULI_FUNICULA,
    pop::AU_CLAIR_DE_LA_LUNE,
    pop::ALOUETTE,
    pop::VOGELFAENGER,
    pop::SURPRISE_SYMPHONY,
    rock::DRUNKEN_SAILOR,
    rock::BELLA_CIAO,
    rock::BEETHOVENS_FIFTH,
    rock::YANKEE_DOODLE,
    rock::WHISKEY_IN_THE_JAR,
    rock::BATTLE_HYMN,
    rock::WHEN_JOHNNY,
    rock::TOREADOR_SONG,
    rock::HOUSE_OF_THE_RISING_SUN,
    rock::WILLIAM_TELL,
    blues::MIDNIGHT_SHUFFLE,
    blues::ROLLING_BOOGIE,
    blues::MINOR_MOOD,
    blues::CARELESS_LOVE,
    blues::FRANKIE_AND_JOHNNY,
    blues::TROUBLE_IN_MIND,
    blues::ST_JAMES_INFIRMARY,
    blues::SEE_SEE_RIDER,
    blues::HIT_THE_ROAD_JACK,
    blues::ST_LOUIS_BLUES,
    jazz::JINGLE_BELLS,
    jazz::SAINTS,
    jazz::ENTERTAINER,
    jazz::HAPPY_BIRTHDAY,
    jazz::TWO_FIVE_WALK,
    jazz::GOLLIWOGGS_CAKEWALK,
    jazz::DARK_EYES,
    jazz::MAPLE_LEAF_RAG,
    jazz::I_GOT_RHYTHM,
    jazz::BILL_BAILEY,
    jazz::SWANEE,
    jazz::AVALON,
    jazz::BALLIN_THE_JACK,
    jazz::TWELFTH_STREET_RAG,
    jazz::DILL_PICKLES,
    jazz::PINE_APPLE_RAG,
    jazz::ELITE_SYNCOPATIONS,
    jazz::TEA_FOR_TWO,
    jazz::LADY_BE_GOOD,
    jazz::HONEYSUCKLE_ROSE,
    jazz::SOMETIMES_IM_HAPPY,
    jazz::MY_BLUE_HEAVEN,
    jazz::LIMEHOUSE_BLUES,
    jazz::JAPANESE_SANDMAN,
    jazz::LIZA,
    jazz::STRIKE_UP_THE_BAND,
    jazz::S_WONDERFUL,
    jazz::IN_THE_MOOD,
    jazz::SING_SING_SING,
    jazz::TAKE_THE_A_TRAIN,
    jazz::FASCINATING_RHYTHM,
    jazz::LOVE_ME_OR_LEAVE_ME,
    jazz::MAKIN_WHOOPEE,
    jazz::WITHOUT_A_SONG,
    jazz::EMBRACEABLE_YOU,
    jazz::BUT_NOT_FOR_ME,
    jazz::SWEET_GEORGIA_BROWN,
    jazz::AINT_MISBEHAVIN,
    jazz::IT_HAD_TO_BE_YOU,
    jazz::TIGER_RAG,
    jazz::AFTER_YOUVE_GONE,
    jazz::KING_PORTER_STOMP,
    jazz::INDIANA,
    jazz::ROSE_ROOM,
    jazz::CHARLESTON,
    jazz::GREAT_DAY,
    jazz::IF_I_COULD_BE_WITH_YOU,
    jazz::CAROLINA_IN_THE_MORNING,
    jazz::TAKE_FIVE,
    jazz::MISSION_IMPOSSIBLE,
    soul::AMAZING_GRACE,
    soul::JOY_TO_THE_WORLD,
    soul::DORIAN_STRUT,
    soul::SUNDAY_SEVENTH,
    soul::WADE_IN_THE_WATER,
    soul::SWING_LOW,
    soul::DOWN_BY_THE_RIVERSIDE,
    soul::GO_DOWN_MOSES,
    soul::JOSHUA,
    soul::MOTHERLESS_CHILD,
    soul::NOBODY_KNOWS,
    soul::MICHAEL_ROW,
    soul::HIS_EYE_IS_ON_THE_SPARROW,
    soul::DEEP_RIVER,
    soul::CLOSER_WALK,
    soul::WILL_THE_CIRCLE,
    soul::O_HAPPY_DAY,
    soul::STEAL_AWAY,
    soul::WERE_YOU_THERE,
    soul::ROLL_CALLED_UP_YONDER,
    soul::OLD_TIME_RELIGION,
    soul::NEED_OF_PRAYER,
    hiphop::TWINKLE,
    hiphop::GYMNOPEDIE,
    hiphop::PRELUDE_IN_C,
    hiphop::SAKURA,
    hiphop::DIES_IRAE,
    hiphop::REVERIE,
    hiphop::GNOSSIENNE,
    hiphop::PRELUDE_IN_E_MINOR,
    electronic::ODE_TO_JOY,
    electronic::KOROBEINIKI,
    electronic::SYMPHONY_40,
    electronic::EINE_KLEINE,
    electronic::TOCCATA,
    electronic::TURKISH_MARCH,
    electronic::KALINKA,
    electronic::CELLO_SUITE_PRELUDE,
    electronic::BADINERIE,
    electronic::SPRING,
    electronic::CAN_CAN,
    latin::RIO_AFTERNOON,
    latin::CARNIVAL_MORNING,
    latin::TICO_TICO,
    latin::EL_MANISERO,
    latin::CIELITO_LINDO,
    latin::QUIEREME_MUCHO,
    latin::SIBONEY,
    latin::APANHEI_TE_CAVAQUINHO,
    latin::ATRAENTE,
    latin::CARIOCA,
    latin::SONS_DE_CARRILHOES,
    latin::O_ABRE_ALAS,
    latin::AQUELLOS_OJOS_VERDES,
    latin::MARIA_ELENA,
    latin::TE_QUIERO_DIJISTE,
    tango::HABANERA,
    tango::TANGO_DEL_TELAR,
    tango::MILONGA_DEL_RIO,
    tango::VALS_DEL_PUERTO,
    tango::EL_CHOCLO,
    tango::LA_CUMPARSITA,
    tango::LA_PALOMA,
    tango::GALLO_CIEGO,
    tango::LAGRIMAS_Y_SONRISAS,
    tango::A_MEDIA_LUZ,
    tango::ADIOS_MUCHACHOS,
    tango::CAMINITO,
    tango::BOEDO,
    tango::ALMA_EN_PENA,
    tango::EL_AMANECER,
    tango::A_LA_GRAN_MUNECA,
    tango::POR_UNA_CABEZA,
    tango::EL_DIA_QUE_ME_QUIERAS,
    tango::VOLVER,
    folklore::EL_CONDOR_PASA,
    folklore::ALMA_LLANERA,
    folklore::DOS_PALOMITAS,
    folklore::PAPEL_DE_PLATA,
    folklore::LA_QUEBRADA_DE_HUMAHUACA,
    folklore::HUAYTA_HUAYTUCHA,
    folklore::KICHARIRILLAWAY,
    folklore::A_LAS_ORILLAS_DEL_TITICACA,
    folklore::CHUNGUINADA,
    folklore::JESUSITA_EN_CHIHUAHUA,
    folklore::DE_COLORES,
    folklore::SOBRE_LAS_OLAS,
    folklore::LA_GOLONDRINA,
    folklore::ESTRELLITA,
    folklore::JARABE_TAPATIO,
    folklore::LA_RASPA,
    folklore::LA_BAMBA,
    folklore::LA_CUCARACHA,
    folklore::DIANA_TRISTE,
    folklore::LA_GATA_GOLOSA,
    folklore::PEZINHO,
    folklore::BALAIO,
    folklore::LUAR_DO_SERTAO,
    folklore::LA_LLORONA,
    folklore::LUNA_DE_XELAJU,
    folklore::LOS_MACHETES,
    folklore::ZAMBA_DEL_TELAR,
    folklore::CHACARERA_DEL_PATIO,
    folklore::GATO_DEL_ARROYO,
    folklore::CHAMAME_DEL_ESTERO,
    folklore::CUECA_DE_LA_CALETA,
    folklore::MARINERA_DEL_CALLEJON,
    folklore::BAMBUCO_DEL_PARAMO,
    folklore::MALAMBO_DEL_FOGON,
    folklore::VIDALA_DE_LA_QUEBRADA,
    folklore::CARNAVALITO_DE_LA_QUEBRADA,
    folklore::CANDOMBE_DEL_BARRIO,
    flamenco::RUMBA_DEL_SUR,
    flamenco::NOCHE_FRIGIA,
    flamenco::ASTURIAS,
    balkan::AJDE_JANO,
    balkan::JOVANO_JOVANKE,
    balkan::SAMIOTISSA,
    balkan::YERAKINA,
    balkan::MILO_MOU_KOKKINO,
    balkan::TSAKONIKOS,
    balkan::PAIDUSHKO_HORO,
    balkan::SVATBARSKA_RACHENITSA,
    balkan::DAICHOVO_HORO,
    balkan::GANKINO_HORO,
    balkan::BUCIMIS,
    balkan::GEAMPARAUA_DE_LA_MURFATLAR,
    balkan::MANDRELE,
    balkan::ENTARISI,
    balkan::SARI_GELIN,
    balkan::IZMIRIN_KAVAKLARI,
    balkan::CAYELINDEN_OTEYE,
    reggae::MARY_HAD_A_LITTLE_LAMB,
    reggae::THIS_OLD_MAN,
    reggae::VALLEY_DUB,
    folk::FRERE_JACQUES,
    folk::GREENSLEEVES,
    folk::OH_SUSANNA,
    folk::SCARBOROUGH_FAIR,
    folk::CLEMENTINE,
    folk::CAMPTOWN_RACES,
    folk::ROW_YOUR_BOAT,
    folk::SILENT_NIGHT,
    folk::WE_WISH_YOU,
    folk::WILD_ROVER,
    folk::SHENANDOAH,
    folk::RED_RIVER_VALLEY,
    folk::HOME_ON_THE_RANGE,
    folk::DANNY_BOY,
    folk::LOCH_LOMOND,
    folk::WAYFARING_STRANGER,
    folk::COTTON_EYED_JOE,
    folk::OLD_JOE_CLARK,
    folk::SOLDIERS_JOY,
    folk::TURKEY_IN_THE_STRAW,
    folk::IRISH_WASHERWOMAN,
    folk::WALTZING_MATILDA,
    folk::MOLLY_MALONE,
    folk::HAVA_NAGILA,
    folk::SANTA_LUCIA,
    folk::TARANTELLA_NAPOLETANA,
    folk::TORNA_A_SURRIENTO,
    folk::ARIRANG,
    folk::TUMBALALAIKA,
    folk::MO_LI_HUA,
    folk::HUNGARIAN_DANCE,
    folk::SUSSERL,
    classical::CANON,
    classical::FUR_ELISE,
    classical::MOUNTAIN_KING,
    classical::MINUET,
    classical::BLUE_DANUBE,
    classical::BRAHMS_LULLABY,
    classical::MORNING_MOOD,
    classical::NEW_WORLD,
    classical::SHCHEDRYK,
    classical::FUNERAL_MARCH,
    classical::LA_DONNA_E_MOBILE,
    classical::JUPITER_HYMN,
    classical::AVE_MARIA,
    classical::PRELUDE_IN_C_SHARP_MINOR,
    classical::WEDDING_MARCH,
    classical::HALLELUJAH_CHORUS,
    classical::GREAT_GATE_OF_KIEV,
    classical::ANITRAS_DANCE,
    classical::DIES_IRAE_MOZART,
    classical::PATHETIQUE,
    classical::MOONLIGHT_SONATA,
    classical::AIR_ON_THE_G_STRING,
    classical::DANSE_MACABRE,
    classical::HUMORESQUE,
    classical::TRAUMEREI,
    classical::SYMPHONY_7_ALLEGRETTO,
    classical::RADETZKY_MARCH,
    classical::PROMENADE,
    classical::PATHETIQUE_WALTZ,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::RHYTHMS;
    use crate::pattern::MAX_STEPS;
    use std::collections::HashSet;

    #[test]
    fn every_bar_of_every_part_adds_up() {
        for song in SONGS {
            for whole in song.parts {
                if !whole.meters.is_empty() {
                    let bars = chart::parse_melody(whole.melody).unwrap().bars.len();
                    assert_eq!(bars, whole.meters.len(), "{}: a meter per bar", song.name);
                    assert_eq!(whole.voices, [(1, 0)], "{}: one voice", song.name);
                }
            }
            for part in song.parts.iter().flat_map(|part| song.part_runs(part)) {
                let part = &part;
                let bar = song.part_time(part).steps_per_bar();
                let what = format!("{}, {}", song.name, part.name);
                for text in [part.melody, part.left] {
                    if text.is_empty() {
                        continue;
                    }
                    let melody = chart::parse_melody(text)
                        .unwrap_or_else(|| panic!("{what}: cannot read {text}"));
                    for (index, length) in melody.bars.iter().enumerate() {
                        assert_eq!(*length, bar, "{what}: bar {} has {length} steps", index + 1);
                    }
                }
                let tonic = song.part_scale(part).root;
                assert!(
                    chart::parse_chart_in(part.chords, bar, tonic).is_some(),
                    "{what}: cannot read the chords"
                );
                let length = part.length(bar);
                assert!(length > 0 && length <= MAX_STEPS, "{what}: {length} steps");
                assert_eq!(part.melody().length, length, "{what}: melody");
                if !part.chords.trim().is_empty() {
                    assert_eq!(
                        part.chords.split('|').count() * bar,
                        length,
                        "{what}: chords"
                    );
                }
                if !part.left.is_empty() {
                    assert_eq!(part.left().length, length, "{what}: left hand");
                }
            }
        }
    }

    #[test]
    fn forms_name_real_parts_and_rhythms_exist() {
        let mut names = HashSet::new();
        for song in SONGS {
            assert!(names.insert(song.name), "{} twice", song.name);
            for name in song.form {
                assert!(song.part(name).is_some(), "{}: no part {name}", song.name);
            }
            assert!(song.drums_from < song.form.len().max(1), "{}", song.name);
            if !song.rhythm.is_empty() {
                let rhythm = RHYTHMS.iter().find(|r| r.name == song.rhythm);
                let rhythm =
                    rhythm.unwrap_or_else(|| panic!("{}: no rhythm {}", song.name, song.rhythm));
                assert_eq!(
                    rhythm.time, song.time,
                    "{}: the drums are in {}",
                    song.name, rhythm.name
                );
            }
        }
        assert!(SONGS.len() >= 60);
    }

    /// A comping written for four beats lands across the beats of a bar
    /// of three, so a song in 3/4 or 6/8 plays one written for three, as
    /// the styles do.
    /// The style a song names is in the library and in the song's time
    /// signature, so selecting it never changes the bars.
    #[test]
    fn every_song_names_a_real_style() {
        for song in SONGS {
            if song.style.is_empty() {
                continue;
            }
            let style = crate::library::Style::named(song.style)
                .unwrap_or_else(|| panic!("{}: no style called {}", song.name, song.style));
            assert_eq!(
                style.time, song.time,
                "{}: {} is in another time",
                song.name, song.style
            );
        }
    }

    #[test]
    fn songs_in_three_play_in_three() {
        for song in SONGS {
            for part in song.parts {
                let (comping, _, rhythm) = song.part_playing(part);
                let time = song.part_time(part);
                if time.steps_per_bar() == 12 {
                    assert!(
                        matches!(
                            comping,
                            Comping::Waltz | Comping::Sway | Comping::Arp3 | Comping::Held
                        ),
                        "{}, {}: in three but plays {comping:?}",
                        song.name,
                        part.name,
                    );
                }
                if part.played.is_some() && !rhythm.is_empty() {
                    let rhythm = RHYTHMS.iter().find(|r| r.name == rhythm);
                    assert_eq!(
                        rhythm.map(|r| r.time),
                        Some(time),
                        "{}, {}: its rhythm is in another time",
                        song.name,
                        part.name
                    );
                }
            }
        }
    }

    #[test]
    fn every_genre_has_songs_and_they_are_in_order() {
        for genre in Genre::ALL {
            let count = SONGS.iter().filter(|song| song.genre == genre).count();
            assert!(count >= 2, "{}: only {count} songs", genre.name());
        }
        assert!(
            SONGS.windows(2).all(|pair| pair[0].genre <= pair[1].genre),
            "SONGS is sorted by genre"
        );
    }

    #[test]
    fn melodies_are_mostly_in_their_key() {
        // A few notes from outside the key are normal (passing notes,
        // borrowed leading notes), but a song with many is more likely
        // a typo, such as F written for F#. The Phrygian of flamenco and
        // of the Turkish makam hicaz raises its third (E F G# A B C D):
        // that G# counts as in the key.
        for song in SONGS {
            let mut total = 0;
            let mut inside = 0;
            for part in song.parts {
                let scale = song.part_scale(part);
                let raised_third =
                    (scale.kind == ScaleKind::Phrygian).then(|| scale.root.transpose(4));
                for note in part.melody().notes {
                    total += 1;
                    let pitch = note.pitch.pitch_class();
                    if scale.contains(pitch) || Some(pitch) == raised_third {
                        inside += 1;
                    }
                }
            }
            assert!(
                inside * 100 >= total * 80,
                "{}: only {inside} of {total} notes in {}",
                song.name,
                song.scale().name()
            );
        }
    }

    #[test]
    fn the_first_notes_are_right() {
        let first = |song: &str| {
            let song = Song::named(song).unwrap();
            let part = song.part(song.form[0]).unwrap();
            let notes: Vec<u8> = part
                .melody()
                .notes
                .iter()
                .take(4)
                .map(|n| n.pitch.midi())
                .collect();
            notes
        };
        assert_eq!(first("Ode to Joy"), [78, 78, 79, 81], "F# F# G A");
        assert_eq!(
            first("Twinkle, Twinkle, Little Star"),
            [72, 72, 79, 79],
            "C C G G"
        );
        assert_eq!(first("Korobeiniki"), [76, 71, 72, 74], "E B C D");
    }
}
