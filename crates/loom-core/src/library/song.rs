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
const TWELVE_EIGHT: TimeSignature = TimeSignature::new(12, 8);

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
    rock::DRUNKEN_SAILOR,
    rock::BELLA_CIAO,
    rock::BEETHOVENS_FIFTH,
    rock::YANKEE_DOODLE,
    rock::WHISKEY_IN_THE_JAR,
    rock::BATTLE_HYMN,
    rock::WHEN_JOHNNY,
    rock::TOREADOR_SONG,
    rock::HOUSE_OF_THE_RISING_SUN,
    blues::MIDNIGHT_SHUFFLE,
    blues::ROLLING_BOOGIE,
    blues::MINOR_MOOD,
    blues::CARELESS_LOVE,
    blues::FRANKIE_AND_JOHNNY,
    blues::TROUBLE_IN_MIND,
    blues::ST_JAMES_INFIRMARY,
    blues::SEE_SEE_RIDER,
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
    electronic::ODE_TO_JOY,
    electronic::KOROBEINIKI,
    electronic::SYMPHONY_40,
    electronic::EINE_KLEINE,
    electronic::TOCCATA,
    electronic::TURKISH_MARCH,
    electronic::KALINKA,
    latin::LA_CUCARACHA,
    latin::RIO_AFTERNOON,
    latin::CARNIVAL_MORNING,
    tango::HABANERA,
    tango::TANGO_DEL_TELAR,
    tango::MILONGA_DEL_RIO,
    tango::VALS_DEL_PUERTO,
    tango::EL_CHOCLO,
    tango::LA_CUMPARSITA,
    tango::LA_PALOMA,
    tango::GALLO_CIEGO,
    folklore::EL_CONDOR_PASA,
    folklore::ALMA_LLANERA,
    folklore::DOS_PALOMITAS,
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
            if song.time.steps_per_bar() == 12 {
                assert!(
                    matches!(
                        song.comping,
                        Comping::Waltz | Comping::Sway | Comping::Arp3 | Comping::Held
                    ),
                    "{} is in three but plays {:?}",
                    song.name,
                    song.comping
                );
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
