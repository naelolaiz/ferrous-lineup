//! Well-known songs in the public domain, with their melodies.
//!
//! Every song here is old enough to belong to everyone: folk songs
//! nobody owns, and pieces by composers who died long ago. The melodies
//! are written in the notation of [`chart::parse_melody`] and the chords
//! as chord charts, one bar at a time, exactly like a lead sheet. Each
//! song is dressed in a style (a house track, a swing band, lo-fi...)
//! by its comping, bass and rhythm, the same building blocks the
//! harmony styles use.
//!
//! The songs live in one file per genre, in the `song/` folder next to
//! this one, and [`SONGS`] lists them all. Songs marked "Written for
//! Ferrous Loom" are new tunes, written to show off a genre that has
//! few well-known tunes old enough to be free.

use crate::harmony::{Scale, ScaleKind};
use crate::meter::TimeSignature;
use crate::pitch::PitchClass;

use super::Genre;
use super::chart::{self, ChordSpan, Melody};
use super::comping::{Bass, Comping};

/// One section of a song, such as a verse or a chorus. Each part
/// becomes one pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Part {
    pub name: &'static str,
    /// The tune, played by the Lead channel.
    pub melody: &'static str,
    /// The chords, accompanied in the song's comping and bass style.
    pub chords: &'static str,
    /// Notes for the Keys channel written out in full, for pieces whose
    /// accompaniment is part of the music. When there are any, the
    /// chords are not accompanied.
    pub left: &'static str,
    /// Where copies of the melody start, in bars from the start of the
    /// part. One voice is `[0]`. In a round, each voice is another copy
    /// a few bars later. A negative start is a voice that began in the
    /// part before: only its end is heard. Notes outside the part are
    /// left out.
    pub voices: &'static [i32],
    /// A time signature of its own, for a part in another time than the
    /// rest of the song. The demo puts a time marker where it starts.
    pub time: Option<TimeSignature>,
    /// The key this part is in, when it is not the song's. Its chords
    /// are read in this key, so a piece that modulates has a part for
    /// each key it visits.
    pub key: Option<(PitchClass, ScaleKind)>,
}

impl Part {
    /// A part with one voice and no written-out accompaniment.
    const fn new(name: &'static str, melody: &'static str, chords: &'static str) -> Part {
        Part {
            name,
            melody,
            chords,
            left: "",
            voices: &[0],
            time: None,
            key: None,
        }
    }

    const fn with_left(self, left: &'static str) -> Part {
        Part { left, ..self }
    }

    const fn with_voices(self, voices: &'static [i32]) -> Part {
        Part { voices, ..self }
    }

    /// The same part in another time signature than the song's, such as
    /// a verse in 3/4 in a song in 4/4.
    #[allow(dead_code)] // until a song changes time signature
    const fn in_time(self, time: TimeSignature) -> Part {
        Part {
            time: Some(time),
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

    pub fn spans(&self, bar: usize) -> Vec<ChordSpan> {
        chart::parse_chart(self.chords, bar).unwrap_or_default()
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
    pub parts: &'static [Part],
    /// The order the parts are played in, by name.
    pub form: &'static [&'static str],
    /// The drums come in at this entry of `form` (0 for the start).
    pub drums_from: usize,
    pub about: &'static str,
}

impl Song {
    pub fn scale(&self) -> Scale {
        Scale::new(self.tonic, self.mode)
    }

    /// The time signature `part` is in: its own, or else the song's.
    pub fn part_time(&self, part: &Part) -> TimeSignature {
        part.time.unwrap_or(self.time)
    }

    /// The key `part` is in: its own, if it modulates, or the song's.
    pub fn part_scale(&self, part: &Part) -> Scale {
        match part.key {
            Some((tonic, mode)) => Scale::new(tonic, mode),
            None => self.scale(),
        }
    }

    pub fn part(&self, name: &str) -> Option<&'static Part> {
        self.parts.iter().find(|part| part.name == name)
    }

    /// The song called `name`, if there is one.
    pub fn named(name: &str) -> Option<&'static Song> {
        SONGS.iter().find(|song| song.name == name)
    }
}

// Every genre file starts with `use super::*;`, which brings in all the
// names below: `Song`, `Part`, the time signatures, and the notes and
// scales without their type in front, so a song can say `tonic: A`.
use PitchClass::*;
use ScaleKind::*;

const COMMON: TimeSignature = TimeSignature::COMMON;
const THREE_FOUR: TimeSignature = TimeSignature::new(3, 4);
const THREE_EIGHT: TimeSignature = TimeSignature::new(3, 8);
const SIX_EIGHT: TimeSignature = TimeSignature::new(6, 8);

mod blues;
mod classical;
mod electronic;
mod flamenco;
mod folk;
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
    rock::DRUNKEN_SAILOR,
    rock::BELLA_CIAO,
    rock::BEETHOVENS_FIFTH,
    rock::YANKEE_DOODLE,
    blues::MIDNIGHT_SHUFFLE,
    blues::ROLLING_BOOGIE,
    blues::MINOR_MOOD,
    jazz::JINGLE_BELLS,
    jazz::SAINTS,
    jazz::ENTERTAINER,
    jazz::HAPPY_BIRTHDAY,
    jazz::TWO_FIVE_WALK,
    soul::AMAZING_GRACE,
    soul::JOY_TO_THE_WORLD,
    soul::DORIAN_STRUT,
    soul::SUNDAY_SEVENTH,
    hiphop::TWINKLE,
    hiphop::GYMNOPEDIE,
    hiphop::PRELUDE_IN_C,
    hiphop::SAKURA,
    hiphop::DIES_IRAE,
    electronic::ODE_TO_JOY,
    electronic::KOROBEINIKI,
    electronic::SYMPHONY_40,
    electronic::EINE_KLEINE,
    electronic::TOCCATA,
    electronic::TURKISH_MARCH,
    latin::LA_CUCARACHA,
    latin::RIO_AFTERNOON,
    latin::CARNIVAL_MORNING,
    tango::HABANERA,
    tango::TANGO_DEL_TELAR,
    tango::MILONGA_DEL_RIO,
    tango::VALS_DEL_PUERTO,
    flamenco::RUMBA_DEL_SUR,
    flamenco::NOCHE_FRIGIA,
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
            for part in song.parts {
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
                assert!(
                    chart::parse_chart(part.chords, bar).is_some(),
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
        // a typo, such as F written for F#.
        for song in SONGS {
            let mut total = 0;
            let mut inside = 0;
            for part in song.parts {
                let scale = song.part_scale(part);
                for note in part.melody().notes {
                    total += 1;
                    if scale.contains(note.pitch.pitch_class()) {
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
