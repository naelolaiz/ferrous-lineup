//! The band: a piano, a bass and drums that play the chord sheet, and in
//! many styles a guitar, strings or an organ.
//!
//! The chord sheet says *what* the harmony is, bar by bar. A
//! [`BandStyle`] says *how* to play it: the rhythm the piano strikes the
//! chords in ([`Comping`]), the line the bass plays ([`Bass`]), the
//! drums ([`Rhythm`]) and what the players it adds play ([`Extras`]).
//! The band writes each player's part from the two,
//! into patterns of its own on the playlist, one track per player, and
//! writes them again whenever the sheet, the key, the time signature or
//! the style changes.
//!
//! A part you edit yourself (a note moved in the piano roll, a clip
//! dragged) is *frozen*: the band leaves it as you left it until you ask
//! it to play that part again ([`Project::play_again`]). The band notices
//! the edit by keeping a fingerprint of what it last wrote for each
//! player and comparing it with what is there now, so it does not matter
//! which tool made the change.
//!
//! A part on a locked channel stays as it is too, and the band keeps the
//! song's own locked parts (see [`locks`]).
//!
//! A part of the song can be played in another style (see [`sections`]).

use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::chord_sheet::{ChordMarker, HarmonyName, read_bar};
use crate::general_midi;
use crate::harmony::{Chord, PitchSet, Scale, ScaleKind};
use crate::i18n::{tr, trf};
use crate::library::apply::{self, BASS, KEYS};
use crate::library::comping::{Bass, BassPart, Comping, Played};
use crate::library::drum_form::{self, DrumPart, PartKind, Then};
use crate::library::interpret::{self, Role};
use crate::library::voicing::bass_line;
use crate::library::{Genre, RHYTHMS, Rhythm, Style};
use crate::meter::{Bar, TimeSignature};
use crate::pattern::{MAX_STEPS, NoteEvent};
use crate::pitch::{Note, PitchClass};
use crate::playlist::{Clip, MAX_SONG_STEPS, Section, Track};
use crate::project::{Channel, ChannelKind, Drum, Project, SampleSource};
use crate::tempo::Tempo;

mod extras;
mod following;
mod form;
mod guitar;
pub mod horns;
pub mod locks;
pub mod sections;
pub mod solo;
pub(crate) mod stops;
mod variation;

pub use extras::{Extra, Extras, PadPart};
pub use form::{ENDING_BARS, Ending, Form, INTRO_BARS};
pub use guitar::GuitarPart;
pub use sections::StyleMarker;
use sections::StyleMarkerFile;
use solo::{NoteRole, Over, Soloist};
pub use variation::{Variation, VariationMarker};
pub(crate) use variation::{tidy as tidy_variations, variation_in};

/// One of the band's players.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Player {
    Piano,
    Bass,
    Drums,
    Guitar,
    Strings,
    Organ,
    /// The soloist: experimental, and only when it is switched on (see
    /// [`Project::set_soloist`]).
    Soloist,
    /// The horn section, when it is switched on (see
    /// [`Project::set_horns`]).
    Horns,
}

impl Player {
    /// How many there are.
    pub const COUNT: usize = 8;

    pub const ALL: [Player; Player::COUNT] = [
        Player::Piano,
        Player::Bass,
        Player::Drums,
        Player::Guitar,
        Player::Strings,
        Player::Organ,
        Player::Soloist,
        Player::Horns,
    ];

    /// Its name, which its playlist track and patterns are called by.
    pub fn name(self) -> &'static str {
        match self {
            Player::Piano => "Piano",
            Player::Bass => "Bass",
            Player::Drums => "Drums",
            Player::Guitar => "Guitar",
            Player::Strings => "Strings",
            Player::Organ => "Organ",
            Player::Soloist => "Soloist",
            Player::Horns => "Horns",
        }
    }

    fn index(self) -> usize {
        self as usize
    }
}

/// A way for the band to play: how the piano comps, what the bass does,
/// and the drums. Unlike a library [`Style`], it has no chords of its
/// own: it plays whatever the chord sheet says.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BandStyle {
    pub name: &'static str,
    pub genre: Genre,
    pub time: TimeSignature,
    /// The tempo it is usually played at, in BPM.
    pub tempo: f32,
    pub comping: Comping,
    pub bass: Bass,
    /// The drums, or `None` for a band without them.
    pub rhythm: Option<&'static Rhythm>,
    pub about: &'static str,
}

/// The band style File > New starts with.
pub const DEFAULT_STYLE: &str = "Pop beat";

/// The band style of an orchestra, which follows the song's meter (see
/// [`BandStyle::follows_the_song`]).
const ORCHESTRA: &str = "Orchestra";

/// The styles with no drums: the ways of playing of the library's
/// classical and ambient styles, which have no rhythm of their own, and
/// of the styles for a solo piano or guitar.
const WITHOUT_DRUMS: [BandStyle; 12] = [
    BandStyle {
        name: "Chorale",
        genre: Genre::Classical,
        time: TimeSignature::COMMON,
        tempo: 72.0,
        comping: Comping::Held,
        bass: Bass::Held,
        rhythm: None,
        about: "No drums: each chord held for as long as it lasts, over its root, as a \
                choir or an organ plays a hymn.",
    },
    BandStyle {
        name: "Alberti bass",
        genre: Genre::Classical,
        time: TimeSignature::COMMON,
        tempo: 100.0,
        comping: Comping::Alberti,
        bass: Bass::Held,
        rhythm: None,
        about: "No drums: the chords broken low, high, middle, high, as in the sonatas of \
                Mozart and Haydn.",
    },
    BandStyle {
        name: "Arpeggios",
        genre: Genre::Classical,
        time: TimeSignature::COMMON,
        tempo: 84.0,
        comping: Comping::ArpUpDown,
        bass: Bass::Held,
        rhythm: None,
        about: "No drums: up and down each chord in eighth notes, over a held bass.",
    },
    BandStyle {
        name: "Sarabande",
        genre: Genre::Classical,
        time: TimeSignature::new(3, 4),
        tempo: 66.0,
        comping: Comping::Sway,
        bass: Bass::Held,
        rhythm: None,
        about: "No drums: a slow dance in 3/4 whose chord falls on beat 2 and is held, the \
                weight of the sarabande and of La folia.",
    },
    BandStyle {
        name: "Ragtime",
        genre: Genre::Jazz,
        time: TimeSignature::COMMON,
        tempo: 96.0,
        comping: Comping::Afterbeat,
        bass: Bass::Alternating,
        rhythm: None,
        about: "No drums: a ragtime pianist alone, the left hand striding from a bass note on \
                each beat to a chord after it.",
    },
    BandStyle {
        name: "Minuet",
        genre: Genre::Classical,
        time: TimeSignature::new(3, 4),
        tempo: 108.0,
        comping: Comping::Minuet,
        bass: Bass::Held,
        rhythm: None,
        about: "No drums: a chord on every beat of a stately 3/4, the first the strongest.",
    },
    BandStyle {
        name: "Mazurka",
        genre: Genre::Classical,
        time: TimeSignature::new(3, 4),
        tempo: 132.0,
        comping: Comping::Mazurka,
        bass: Bass::Waltz,
        rhythm: None,
        about: "No drums: the bass on beat 1 and the weight on beat 3, as in Chopin's mazurkas.",
    },
    BandStyle {
        name: "Polonaise",
        genre: Genre::Classical,
        time: TimeSignature::new(3, 4),
        tempo: 96.0,
        comping: Comping::Polonaise,
        bass: Bass::Held,
        rhythm: None,
        about: "No drums: every bar in the polonaise rhythm, an eighth and two sixteenths, then \
                four eighths.",
    },
    BandStyle {
        name: "Siciliano",
        genre: Genre::Classical,
        time: TimeSignature::new(6, 8),
        tempo: 76.0,
        comping: Comping::Siciliano,
        bass: Bass::Held,
        rhythm: None,
        about: "No drums: a slow 6/8, each beat long, short, short.",
    },
    BandStyle {
        name: "Baroque continuo",
        genre: Genre::Classical,
        time: TimeSignature::COMMON,
        tempo: 80.0,
        comping: Comping::Held,
        bass: Bass::Octaves,
        rhythm: None,
        about: "No drums: a harpsichord's chords over a cello leaping in octaves in eighth notes.",
    },
    BandStyle {
        name: "Fado",
        genre: Genre::Folk,
        time: TimeSignature::COMMON,
        tempo: 76.0,
        comping: Comping::Afterbeat,
        bass: Bass::Alternating,
        rhythm: None,
        about: "No drums: the fado's guitars, the bass on each beat and the chord between, with \
                the Portuguese guitar picking above them.",
    },
    BandStyle {
        name: "Travis picking",
        genre: Genre::Folk,
        time: TimeSignature::COMMON,
        tempo: 100.0,
        comping: Comping::Travis,
        bass: Bass::Alternating,
        rhythm: None,
        about: "No drums: a fingerpicking guitar, the fingers between the thumb's alternating \
                bass notes, which the bass doubles.",
    },
];

/// The band styles whose bass is the piano's own left hand, for the
/// piano styles: ragtime, stride and boogie-woogie.
const LEFT_HAND: [&str; 2] = ["Ragtime", "Stride"];

/// The band styles whose bass is a tuba: a ragtime band, the marches and
/// the pasodoble.
const TUBA: [&str; 4] = ["Cakewalk", "March", "Six-eight march", "Pasodoble"];

/// The band styles whose chords are an accordion's.
const ACCORDION: [&str; 5] = [
    "Musette",
    "Cajun two-step",
    "Vallenato",
    "Baião",
    "Rasguido doble",
];

/// The band styles whose chords are a guitar's.
const GUITAR: [(&str, u8); 5] = [
    ("Fado", general_midi::NYLON_GUITAR),
    ("Twelve-beat compás", general_midi::NYLON_GUITAR),
    ("Travis picking", general_midi::STEEL_GUITAR),
    ("Highlife", general_midi::CLEAN_GUITAR),
    ("Soukous", general_midi::CLEAN_GUITAR),
];

/// The band styles whose chords a choir sings: the street rumba's coro
/// and the murga.
const CHOIR: [&str; 2] = ["Guaguancó", "Murga"];

/// The band styles whose chords are synth pads over a synth bass.
const SYNTHS: [&str; 1] = ["Kizomba"];

/// The band styles that bring their horn section with them: a big band
/// is its horns.
const HORN_BANDS: [&str; 2] = ["Big band swing", "Big band ballad"];

/// Every band style: one for each rhythm of the library, played the way
/// the library's styles with that rhythm play (see [`apply::feel`]), and
/// a few without drums. Sorted by genre, as the library is.
pub fn band_styles() -> &'static [BandStyle] {
    static STYLES: OnceLock<Vec<BandStyle>> = OnceLock::new();
    STYLES.get_or_init(|| {
        let mut styles: Vec<BandStyle> = RHYTHMS
            .iter()
            .map(|rhythm| {
                let (comping, bass) = apply::feel(rhythm);
                BandStyle {
                    name: rhythm.name,
                    genre: rhythm.genre,
                    time: rhythm.time,
                    tempo: rhythm.tempo,
                    comping,
                    bass,
                    rhythm: Some(rhythm),
                    about: rhythm.about,
                }
            })
            .chain(WITHOUT_DRUMS)
            .collect();
        // A stable sort: within a genre, the library's order stays.
        styles.sort_by_key(|style| style.genre);
        styles
    })
}

impl BandStyle {
    /// The players it adds to the piano, the bass and the drums.
    pub fn extras(&self) -> Extras {
        extras::of(self.name)
    }

    /// Who plays in it: the piano and the bass, the drums if it has
    /// them, and the players it adds.
    pub fn players(&self) -> Vec<Player> {
        let extras = self.extras();
        Player::ALL
            .into_iter()
            .filter(|player| match player {
                Player::Piano | Player::Bass => true,
                Player::Drums => self.rhythm.is_some(),
                Player::Guitar => extras.guitar.is_some(),
                Player::Strings => extras.strings.is_some(),
                Player::Organ => extras.organ.is_some(),
                Player::Soloist | Player::Horns => false,
            })
            .collect()
    }

    /// The General MIDI instruments its players play, the drums left out:
    /// what it brings to a song with its instruments, piano first.
    pub fn programs(&self) -> Vec<u8> {
        let extras = self.extras();
        [
            Some(self.piano_program()),
            Some(self.bass_program()),
            extras.guitar.map(|guitar| guitar.program),
            extras.strings.map(|strings| strings.program),
            extras.organ.map(|organ| organ.program),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    /// How much it swings when it is picked (see [`crate::swing`]): as
    /// much as its drums do, and not at all without drums.
    pub fn swing(&self) -> u8 {
        self.rhythm.map_or(0, |rhythm| rhythm.swing)
    }

    /// Whether it follows the song, as [`Style::follows_the_song`] says:
    /// its drums follow the meter, so it plays every bar in that bar's
    /// meter, leaves the song's time signatures and tempo as they are,
    /// and comes in alongside the song's own parts instead of muting them.
    pub fn follows_the_song(&self) -> bool {
        self.rhythm.is_some_and(|rhythm| rhythm.follows_the_meter)
    }

    /// Whether the horn section comes in with the style's instruments:
    /// it does in a big band, and the Band menu can still send it away.
    pub fn brings_horns(&self) -> bool {
        HORN_BANDS.contains(&self.name)
    }

    /// The band style called `name`, if there is one.
    pub fn named(name: &str) -> Option<&'static BandStyle> {
        band_styles().iter().find(|style| style.name == name)
    }

    /// The band style that plays the way a library style does: the one
    /// of its rhythm, or for a style without drums, the one without
    /// drums that comps and plays the bass as it does.
    pub fn of_style(style: &Style) -> Option<&'static BandStyle> {
        let without_drums = |band: &&BandStyle| band.rhythm.is_none();
        BandStyle::named(style.rhythm)
            .or_else(|| {
                band_styles()
                    .iter()
                    .filter(without_drums)
                    .find(|band| band.name == style.name)
            })
            .or_else(|| {
                band_styles().iter().find(|band| {
                    band.rhythm.is_none()
                        && band.comping == style.comping
                        && band.bass == style.bass
                        && band.time == style.time
                })
            })
    }

    /// The General MIDI instrument of the piano: the horns of the
    /// orchestra, an electric piano in the electronic, hip hop and soul
    /// styles, an accordion in the Balkan ones, a grand piano elsewhere.
    /// Some styles have their own: an accordion, a guitar, a choir, the
    /// harpsichord of Baroque music, the honky-tonk piano of country bars,
    /// the synth pads of kizomba.
    pub(crate) fn piano_program(&self) -> u8 {
        let name = self.name;
        if let Some(&(_, guitar)) = GUITAR.iter().find(|(style, _)| *style == name) {
            return guitar;
        }
        match self.genre {
            _ if name == ORCHESTRA => general_midi::FRENCH_HORN,
            _ if ACCORDION.contains(&name) => general_midi::ACCORDION,
            _ if CHOIR.contains(&name) => general_midi::CHOIR_AAHS,
            _ if name == "Baroque continuo" => general_midi::HARPSICHORD,
            _ if name == "Honky-tonk shuffle" => general_midi::HONKY_TONK_PIANO,
            _ if SYNTHS.contains(&name) => general_midi::WARM_PAD,
            Genre::Electronic | Genre::HipHop | Genre::Soul => 4,
            Genre::Balkan => 21,
            _ => 0,
        }
    }

    /// The General MIDI instrument of the bass: the orchestra's timpani,
    /// the pianist's own left hand in the piano styles, a tuba in a
    /// ragtime band and a march, a cello under Baroque music, a double
    /// bass in jazz, tango, folklore and the acoustic styles, a synth
    /// bass in electronic music, hip hop and kizomba, and a bass guitar
    /// elsewhere.
    pub(crate) fn bass_program(&self) -> u8 {
        match self.genre {
            _ if self.name == ORCHESTRA => general_midi::TIMPANI,
            _ if LEFT_HAND.contains(&self.name) => general_midi::ACOUSTIC_GRAND_PIANO,
            _ if TUBA.contains(&self.name) => general_midi::TUBA,
            _ if self.name == "Baroque continuo" => general_midi::CELLO,
            _ if SYNTHS.contains(&self.name) => general_midi::SYNTH_BASS,
            Genre::Jazz
            | Genre::Tango
            | Genre::Folklore
            | Genre::Flamenco
            | Genre::Balkan
            | Genre::Folk
            | Genre::Classical => 32,
            Genre::Electronic | Genre::HipHop => 38,
            _ => 33,
        }
    }
}

/// The names of the song's own channels and tracks that the band muted
/// when it came in, to play them again when it leaves.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Muted {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub channels: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tracks: Vec<String>,
}

/// Whose drums the band plays. They can come from another style than
/// its chord and bass parts, each taken from the Library on its own.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum BandDrums {
    /// Those of the style the band plays the chords and the bass in, or
    /// of the saved style it plays them from.
    #[default]
    Style,
    /// Those of another style.
    Of(&'static BandStyle),
    /// Those of a saved style.
    Saved(Box<crate::library::UserStyle>),
    /// None: the song's own drums play.
    Song,
}

/// The band of a song: its style, how far it plays, and what it knows
/// about the parts it wrote.
#[derive(Debug, Clone, PartialEq)]
pub struct Band {
    pub custom_rhythm: Option<Box<crate::library::UserStyle>>,
    pub custom_instruments: Option<Box<crate::library::UserStyle>>,
    pub style: &'static BandStyle,
    /// The instrumentation can come from a different style than the rhythm.
    pub instrument_style: &'static BandStyle,
    /// The exact library style supplying the comping, when one was picked.
    pub harmony_style: Option<&'static Style>,
    /// The chords and the bass played as plain accents on the beats: how
    /// songs saved so play. The Library no longer chooses it.
    pub accents_only: bool,
    /// Whose drums play, when not those of `style` or `custom_rhythm`.
    pub drums: BandDrums,
    /// The step the band plays up to: the end of the last bar written
    /// on the chord sheet, or of a bar typed after it.
    pub(crate) end: usize,
    /// The channel each player plays on, by [`Player::index`]. The drums
    /// play on the built-in drum channels, so theirs is `None`.
    pub(crate) channels: [Option<usize>; Player::COUNT],
    /// For each player, a fingerprint of what the band last wrote for it,
    /// or `None` if it never has. A part that no longer matches its
    /// fingerprint was edited, and is frozen.
    pub(crate) written: [Option<u64>; Player::COUNT],
    /// A fingerprint of what the parts were last written from: the chord
    /// sheet, the keys, the bars and the style.
    pub(crate) heard: Option<u64>,
    pub(crate) muted: Muted,
    /// How many channels the song had when the band came in: the ones
    /// after them are the band's, and go with it if they play nothing
    /// else.
    pub(crate) since: usize,
    /// The intro, how many times the chorus plays, and the ending.
    pub(crate) form: Form,
    /// Where the band changes between the style's two variations, in
    /// order. Before the first, it plays variation a.
    pub(crate) variations: Vec<VariationMarker>,
    /// Where the band plays another style than its own, in order (see
    /// [`sections`]).
    pub(crate) styles: Vec<StyleMarker>,
    /// The seed of the soloist's solo, while there is a soloist.
    pub(crate) solo: Option<u64>,
    /// Whether the horn section plays.
    pub(crate) horns: bool,
}

impl Band {
    fn new(style: &'static BandStyle) -> Self {
        Band {
            custom_rhythm: None,
            custom_instruments: None,
            style,
            instrument_style: style,
            harmony_style: None,
            accents_only: false,
            drums: BandDrums::Style,
            end: 0,
            channels: [None; Player::COUNT],
            written: [None; Player::COUNT],
            heard: None,
            muted: Muted::default(),
            since: 0,
            form: Form::default(),
            variations: Vec::new(),
            styles: Vec::new(),
            solo: None,
            horns: style.brings_horns(),
        }
    }

    /// Plays the instruments of `style`. A style that brings its horns
    /// (see [`BandStyle::brings_horns`]) has them play; the horns the
    /// instruments before it brought leave with them, and horns switched
    /// on in the Band menu stay, as do horns on a locked channel.
    fn take_instruments(&mut self, style: &'static BandStyle, horns_locked: bool) {
        let brought = self.custom_instruments.is_none() && self.instrument_style.brings_horns();
        let horns = style.brings_horns() || (self.horns && !brought);
        if horns != self.horns && !horns_locked {
            self.horns = horns;
            self.written[Player::Horns.index()] = None;
        }
        self.custom_instruments = None;
        self.instrument_style = style;
    }

    pub fn end(&self) -> usize {
        self.end
    }

    /// The channel `player` plays on: `None` for the drums, and for a
    /// player the band has no part for.
    pub fn channel(&self, player: Player) -> Option<usize> {
        self.channels[player.index()]
    }

    /// The song's own channels and tracks the band muted.
    pub fn muted(&self) -> &Muted {
        &self.muted
    }

    /// Whether a soloist plays with the band.
    pub fn has_soloist(&self) -> bool {
        self.solo.is_some()
    }

    /// Whether the horn section plays with the band.
    pub fn has_horns(&self) -> bool {
        self.horns
    }

    /// The horn section, when it plays (see [`Band::horn_section`]).
    pub fn horns(&self) -> Option<horns::Horns> {
        self.horns.then(|| self.horn_section())
    }

    /// The horn section the band would bring: what it plays in the style
    /// the band plays, on the instrument of the band's instruments.
    pub fn horn_section(&self) -> horns::Horns {
        horns::Horns {
            program: horns::Horns::of(self.instrument_style).program,
            ..horns::Horns::of(self.style)
        }
    }

    /// Who plays: the style's players, and the soloist and the horns
    /// when they are there.
    pub fn players(&self) -> Vec<Player> {
        if let Some(style) = &self.custom_rhythm {
            let mut players = self
                .custom_instruments
                .as_ref()
                .map_or_else(|| self.instrument_style.players(), |style| style.players());
            let drums = self.has_drums() || self.sections_drum();
            players.retain(|player| *player != Player::Drums || drums);
            for player in style.players() {
                if player == Player::Drums || self.accents_only {
                    continue;
                }
                if !players.contains(&player) {
                    players.push(player);
                }
            }
            if drums && !players.contains(&Player::Drums) {
                players.push(Player::Drums);
            }
            if self.has_soloist() && !players.contains(&Player::Soloist) {
                players.push(Player::Soloist);
            }
            if self.has_horns() && !players.contains(&Player::Horns) {
                players.push(Player::Horns);
            }
            return players;
        }
        let mut players = self.instrument_style.players();
        players.retain(|player| *player != Player::Drums);
        if self.has_drums() || self.sections_drum() {
            players.push(Player::Drums);
        }
        if self.has_soloist() {
            players.push(Player::Soloist);
        }
        if self.has_horns() {
            players.push(Player::Horns);
        }
        players
    }

    /// The built-in style whose drums play: its own, or another's.
    /// `None` when a saved style's drums play, or the song's own.
    pub fn drums_style(&self) -> Option<&'static BandStyle> {
        match &self.drums {
            BandDrums::Style if self.custom_rhythm.is_none() => Some(self.style),
            BandDrums::Of(style) => Some(style),
            _ => None,
        }
    }

    /// The saved style whose drum lanes play, if one does.
    pub fn saved_drums(&self) -> Option<&crate::library::UserStyle> {
        match &self.drums {
            BandDrums::Style => self.custom_rhythm.as_deref(),
            BandDrums::Saved(style) => Some(style),
            _ => None,
        }
    }

    /// The name of the style whose drums play, or `None` when the song's
    /// own drums play.
    pub fn drums_name(&self) -> Option<&str> {
        match (self.drums_style(), self.saved_drums()) {
            (Some(style), _) => Some(style.name),
            (None, Some(style)) => Some(&style.name),
            (None, None) => None,
        }
    }

    /// Whose drums play said the simplest way: those of the style its
    /// chord and bass parts come from, when they are.
    fn tidy_drums(&mut self) {
        let same = match &self.drums {
            BandDrums::Of(style) => self.custom_rhythm.is_none() && *style == self.style,
            BandDrums::Saved(style) => self.custom_rhythm.as_ref() == Some(style),
            _ => false,
        };
        if same {
            self.drums = BandDrums::Style;
        }
    }

    /// Whether the band plays drums of its own (sections aside).
    fn has_drums(&self) -> bool {
        self.drums_style()
            .is_some_and(|style| style.rhythm.is_some())
            || self
                .saved_drums()
                .is_some_and(|style| style.lanes.iter().any(|lane| lane.player == Player::Drums))
    }

    /// Whether a section's style has drums.
    fn sections_drum(&self) -> bool {
        self.styles
            .iter()
            .any(|marker| marker.style.is_some_and(|style| style.rhythm.is_some()))
    }

    pub fn name(&self) -> &str {
        if let Some(style) = &self.custom_rhythm {
            return &style.name;
        }
        self.harmony_style
            .map_or(self.style.name, |style| style.name)
    }

    pub fn instrument_name(&self) -> &str {
        self.custom_instruments
            .as_ref()
            .map_or(self.instrument_style.name, |style| &style.name)
    }

    fn extras(&self) -> Extras {
        let mut extras = self.instrument_style.extras();
        let patterns = self.style.extras();
        if let Some(guitar) = &mut extras.guitar {
            guitar.part = if self.accents_only {
                GuitarPart::Strum(Comping::Accents)
            } else {
                patterns
                    .guitar
                    .map_or(GuitarPart::StylesRhythm, |part| part.part)
            };
        }
        for (player, pattern) in [
            (&mut extras.strings, patterns.strings),
            (&mut extras.organ, patterns.organ),
        ] {
            if let Some(player) = player {
                player.part = if self.accents_only {
                    PadPart::Accents
                } else {
                    pattern.map_or(PadPart::Held, |part| part.part)
                };
            }
        }
        extras
    }

    fn comping_in(&self, variation: Variation) -> Comping {
        if self.accents_only {
            Comping::Accents
        } else {
            self.playing_style().comping_in(variation)
        }
    }

    fn bass_in(&self, variation: Variation) -> Bass {
        if self.accents_only {
            Bass::Accents
        } else {
            self.playing_style().bass_in(variation)
        }
    }

    fn playing_style(&self) -> BandStyle {
        self.harmony_style.map_or(*self.style, |style| BandStyle {
            name: style.name,
            genre: style.genre,
            time: style.time,
            tempo: style.tempo,
            comping: style.comping,
            bass: style.bass,
            rhythm: Rhythm::named(style.rhythm),
            about: style.about,
        })
    }
}

/// A band as it is saved in a project file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BandFile {
    pub style: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_rhythm: Option<Box<crate::library::UserStyle>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_instruments: Option<Box<crate::library::UserStyle>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instrument_style: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub harmony_style: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub accents_only: bool,
    /// Whose drums play, when not the style's.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drums: Option<DrumsFile>,
    pub end: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub piano: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bass: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guitar: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strings: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organ: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub soloist: Option<usize>,
    /// The seed of the solo, while there is a soloist.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solo: Option<u64>,
    /// The fingerprints of the piano, the bass and the drums.
    pub written: [Option<u64>; 3],
    /// The fingerprints of the guitar, the strings and the organ, left
    /// out when there are none, so a band of three reads as before.
    #[serde(default, skip_serializing_if = "none_written")]
    pub written_too: [Option<u64>; 3],
    /// The fingerprint of the soloist's part.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub written_solo: Option<u64>,
    /// Whether the horn section plays, its channel and the fingerprint
    /// of its part.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub with_horns: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub horns: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub written_horns: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heard: Option<u64>,
    #[serde(default)]
    pub muted: Muted,
    #[serde(default)]
    pub since: usize,
    #[serde(default, skip_serializing_if = "Form::is_plain")]
    pub form: Form,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variations: Vec<VariationMarker>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub styles: Vec<StyleMarkerFile>,
}

/// Whose drums a band plays, as it is saved, when not its style's.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum DrumsFile {
    /// A built-in style's, by name.
    Of(String),
    Saved(Box<crate::library::UserStyle>),
    /// None: the song's own.
    Song,
}

fn none_written(written: &[Option<u64>; 3]) -> bool {
    written.iter().all(Option::is_none)
}

impl From<&Band> for BandFile {
    fn from(band: &Band) -> Self {
        let [piano, bass, drums, guitar, strings, organ, soloist, horns] = band.written;
        BandFile {
            style: band.style.name.to_string(),
            custom_rhythm: band.custom_rhythm.clone(),
            custom_instruments: band.custom_instruments.clone(),
            instrument_style: (band.instrument_style != band.style)
                .then(|| band.instrument_style.name.to_string()),
            harmony_style: band.harmony_style.map(|style| style.name.to_string()),
            accents_only: band.accents_only,
            drums: match &band.drums {
                BandDrums::Style => None,
                BandDrums::Of(style) => Some(DrumsFile::Of(style.name.to_string())),
                BandDrums::Saved(style) => Some(DrumsFile::Saved(style.clone())),
                BandDrums::Song => Some(DrumsFile::Song),
            },
            end: band.end,
            piano: band.channel(Player::Piano),
            bass: band.channel(Player::Bass),
            guitar: band.channel(Player::Guitar),
            strings: band.channel(Player::Strings),
            organ: band.channel(Player::Organ),
            soloist: band.channel(Player::Soloist),
            solo: band.solo,
            written: [piano, bass, drums],
            written_too: [guitar, strings, organ],
            written_solo: soloist,
            with_horns: band.horns,
            horns: band.channel(Player::Horns),
            written_horns: horns,
            heard: band.heard,
            muted: band.muted.clone(),
            since: band.since,
            form: band.form,
            variations: band.variations.clone(),
            styles: band.styles.iter().map(StyleMarkerFile::from).collect(),
        }
    }
}

impl BandFile {
    /// The band it describes, in a project with `channels` channels.
    pub(crate) fn band(self, channels: usize) -> Result<Band, String> {
        let style = BandStyle::named(&self.style).ok_or_else(|| {
            trf(
                "its band plays \"{}\", a style this version does not know",
                &[&self.style],
            )
        })?;
        let instrument_style = self.instrument_style.as_deref().map_or(Ok(style), |name| {
            BandStyle::named(name).ok_or_else(|| trf("unknown instrument style: {}", &[&name]))
        })?;
        let harmony_style = self
            .harmony_style
            .as_deref()
            .map(|name| {
                Style::named(name).ok_or_else(|| trf("unknown harmony style: {}", &[&name]))
            })
            .transpose()?;
        if self.end > MAX_SONG_STEPS {
            return Err(tr("its band plays past the end of the song").to_string());
        }
        let channel = |index: Option<usize>| match index {
            Some(index) if index >= channels => Err(trf(
                "its band plays on channel {}, which does not exist",
                &[&(index + 1)],
            )),
            index => Ok(index),
        };
        let [piano, bass, drums] = self.written;
        let [guitar, strings, organ] = self.written_too;
        let drums_from = match self.drums {
            None => BandDrums::Style,
            Some(DrumsFile::Of(name)) => {
                BandDrums::Of(BandStyle::named(&name).ok_or_else(|| {
                    trf(
                        "its band plays the drums of \"{}\", a style this version does not know",
                        &[&name],
                    )
                })?)
            }
            Some(DrumsFile::Saved(style)) => {
                style.validate()?;
                BandDrums::Saved(style)
            }
            Some(DrumsFile::Song) => BandDrums::Song,
        };
        for style in [&self.custom_rhythm, &self.custom_instruments]
            .into_iter()
            .flatten()
        {
            style.validate()?;
        }
        Ok(Band {
            custom_rhythm: self.custom_rhythm,
            custom_instruments: self.custom_instruments,
            style,
            instrument_style,
            harmony_style,
            accents_only: self.accents_only,
            drums: drums_from,
            end: self.end,
            channels: [
                channel(self.piano)?,
                channel(self.bass)?,
                None,
                channel(self.guitar)?,
                channel(self.strings)?,
                channel(self.organ)?,
                channel(self.soloist)?,
                channel(self.horns)?,
            ],
            written: [
                piano,
                bass,
                drums,
                guitar,
                strings,
                organ,
                self.written_solo,
                self.written_horns,
            ],
            heard: self.heard,
            muted: self.muted,
            since: self.since.min(channels),
            form: Form {
                intro: self.form.intro.min(self.end),
                choruses: self.form.choruses.clamp(1, crate::playlist::MAX_REPEATS),
                ending: self.form.ending,
            },
            variations: {
                let mut markers = self.variations;
                markers.retain(|marker| marker.start < MAX_SONG_STEPS);
                variation::tidy(&mut markers);
                markers
            },
            styles: {
                let mut markers = self
                    .styles
                    .iter()
                    .map(StyleMarkerFile::marker)
                    .collect::<Result<Vec<_>, _>>()?;
                markers.retain(|marker| marker.start < MAX_SONG_STEPS);
                sections::tidy(&mut markers);
                markers
            },
            solo: self.solo,
            horns: self.with_horns,
        })
    }
}

/// FNV-1a, a simple hash that mixes in one byte at a time. Unlike the
/// standard library's hasher, it is promised to give the same number on
/// every computer and in every version, which a fingerprint saved in a
/// file needs.
struct Fingerprint(u64);

impl Fingerprint {
    fn new() -> Self {
        Fingerprint(0xCBF2_9CE4_8422_2325)
    }

    fn bytes(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.0 ^= u64::from(byte);
            self.0 = self.0.wrapping_mul(0x0100_0000_01B3);
        }
    }

    fn number(&mut self, number: usize) {
        self.bytes(&(number as u64).to_le_bytes());
    }
}

/// How loud a band player's channel starts: the bass under the melody,
/// the piano and the guitar further back, and pads behind them all.
fn player_volume(player: Player) -> f32 {
    match player {
        Player::Piano => 0.45,
        Player::Bass => 0.7,
        Player::Guitar | Player::Horns => 0.4,
        Player::Strings | Player::Organ => 0.3,
        Player::Soloist => 0.6,
        Player::Drums => Channel::drum(Drum::Kick).volume,
    }
}

/// How far above the piano's voicing the strings play: an octave.
const STRINGS_OCTAVE: i32 = 12;

/// What the drums play in the last bar of an ending: the fill of the
/// library that is one hit of the kick and the crash cymbal.
const LAST_HIT: &str = "Crash ending";

/// A stretch of the song that one pattern of each player covers: whole
/// bars of one time signature, inside one section, at most
/// [`MAX_STEPS`] long.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Chunk {
    start: usize,
    length: usize,
    time: TimeSignature,
    /// Whether it starts only because a section does: a player holding
    /// a note over its start plays on in one pattern, so the note is not
    /// struck again (see [`Project::write_part`]).
    soft: bool,
}

impl Chunk {
    fn end(&self) -> usize {
        self.start + self.length
    }
}

/// One pattern's worth of a part: how long, in which time signature,
/// how it swings, and its notes by channel.
#[derive(Debug, Clone, PartialEq)]
struct Content {
    length: usize,
    time: TimeSignature,
    /// The swing of a section's style, if one plays there: the same notes
    /// swung otherwise are another pattern.
    swing: Option<u8>,
    rows: Vec<(usize, Vec<NoteEvent>)>,
}

/// The pitch class a group of notes is played over: the first of its
/// normal form, its most compact arrangement. A set has no root, so this
/// is a choice, but a fixed one: the same set always gets the same bass.
fn group_bass(set: PitchSet) -> Note {
    let first = set.normal_form().first().copied().unwrap_or(PitchClass::C);
    Note::new(36 + first.semitone())
}

/// The notes of a group, from middle C up, each above the last.
fn group_voicing(pitch_classes: impl IntoIterator<Item = PitchClass>) -> Vec<Note> {
    let mut notes: Vec<Note> = Vec::new();
    for pitch_class in pitch_classes {
        let mut midi = 60 + pitch_class.semitone();
        while notes.last().is_some_and(|last| last.midi() >= midi) {
            midi += 12;
        }
        notes.push(Note::new(midi));
    }
    notes
}

/// Two chords at once, each in a layer of its own: the lower chord from
/// the G below middle C up, and the upper chord stacked right above it,
/// so the two never cross, as an orchestra keeps them apart.
fn polychord_voicing(upper: Chord, lower: Chord) -> Vec<Note> {
    let stacked = |chord: Chord, floor: u8| -> Vec<Note> {
        let root = floor + floor_gap(floor, chord.root);
        chord.notes_from(Note::new(root))
    };
    let low = stacked(lower, 55);
    let top = low.last().map_or(60, |note| note.midi() + 1);
    let mut notes = low;
    notes.extend(stacked(upper, top));
    notes
}

/// How many semitones up from `floor` the first `pitch_class` is.
fn floor_gap(floor: u8, pitch_class: PitchClass) -> u8 {
    Note::new(floor).pitch_class().semitones_up_to(pitch_class)
}

/// `played` cut to the steps from `start` to `end`. A harmony cut short
/// at `end` goes on after it, so its bass does not lead anywhere yet.
fn cut(played: &[Played], start: usize, end: usize) -> Vec<Played> {
    played
        .iter()
        .filter(|harmony| harmony.start < end && harmony.end > start)
        .map(|harmony| {
            let mut harmony = harmony.clone();
            if harmony.end > end
                && let BassPart::Chord { note, next, .. } = &mut harmony.bass
            {
                *next = *note;
            }
            harmony.start = harmony.start.max(start);
            harmony.end = harmony.end.min(end);
            harmony
        })
        .collect()
}

/// A stretch of bars the band plays in one variation of one style.
struct Run {
    bars: Vec<Bar>,
    variation: Variation,
}

/// `bars` in runs of one variation each, as `markers` say, starting
/// again where a section's style starts (`styles`), all but the `held`
/// bar, if there is one: the last bar of an ending, played otherwise.
fn runs(
    bars: &[Bar],
    held: Option<Bar>,
    markers: &[VariationMarker],
    styles: &[StyleMarker],
) -> Vec<Run> {
    let mut runs: Vec<Run> = Vec::new();
    let played = bars
        .iter()
        .filter(|bar| held.is_none_or(|held| bar.start < held.start));
    for &bar in played {
        let variation = variation::variation_in(markers, bar.start);
        let restyled = styles.iter().any(|marker| marker.start == bar.start);
        match runs.last_mut() {
            Some(run) if run.variation == variation && !restyled => run.bars.push(bar),
            _ => runs.push(Run {
                bars: vec![bar],
                variation,
            }),
        }
    }
    runs
}

/// What the band plays from: the bars, cut into chunks, and the chord
/// sheet's harmonies, breaks and keys.
struct Score {
    bars: Vec<Bar>,
    chunks: Vec<Chunk>,
    /// The song's sections, written or heard (see
    /// [`Project::form_sections`]).
    form: Vec<Section>,
    /// The end of the chorus.
    end: usize,
    /// The last bar of an ending, struck and held.
    held: Option<Bar>,
    breaks: stops::Breaks,
    harmonies: Vec<Played>,
    keys: crate::key_map::KeyMap,
    runs: Vec<Run>,
    variations: Vec<VariationMarker>,
    /// The time signature of the first bar.
    time: TimeSignature,
}

/// A player's notes, run by run: each run played with `play` in its
/// variation, from the harmonies inside it.
fn played_in_runs(
    runs: &[Run],
    harmonies: &[Played],
    play: impl Fn(Variation, &[Bar], &[Played]) -> Vec<NoteEvent>,
) -> Vec<NoteEvent> {
    let mut notes = Vec::new();
    for run in runs {
        let (Some(first), Some(last)) = (run.bars.first(), run.bars.last()) else {
            continue;
        };
        let played = cut(harmonies, first.start, last.end());
        notes.extend(play(run.variation, &run.bars, &played));
    }
    notes
}

/// Makes the notes that start in variation b louder.
fn louder_in_b(notes: &mut [NoteEvent], markers: &[VariationMarker]) {
    for note in notes {
        if variation::variation_in(markers, note.start) == Variation::B {
            note.velocity = variation::louder(note.velocity);
        }
    }
}

/// The notes of the `held` bar, if there is one, played with `play`.
fn held_bar(
    held: Option<Bar>,
    harmonies: &[Played],
    play: impl Fn(&[Bar], &[Played]) -> Vec<NoteEvent>,
) -> Vec<NoteEvent> {
    held.map_or_else(Vec::new, |bar| {
        play(&[bar], &cut(harmonies, bar.start, bar.end()))
    })
}

/// The bass's note under `played`, struck once and held to its end: the
/// bass's part in a shot, a hold or a push.
pub(crate) fn bass_struck(played: &Played) -> Vec<NoteEvent> {
    match played.bass {
        BassPart::Chord { note, .. } | BassPart::Pedal(note) => {
            vec![NoteEvent::new(
                note,
                played.start,
                played.end - played.start,
            )]
        }
        BassPart::Rest => Vec::new(),
    }
}

impl Project {
    /// The band, if the song has one.
    pub fn band(&self) -> Option<&Band> {
        self.band.as_ref()
    }

    /// The track `player`'s part is on, if the band has written one.
    pub fn player_track(&self, player: Player) -> Option<usize> {
        self.playlist
            .clips
            .iter()
            .find(|clip| self.patterns[clip.pattern].player == Some(player))
            .map(|clip| clip.track)
    }

    /// Whether you play `player`'s part yourself: its track is muted, so
    /// the band leaves it out.
    pub fn you_play(&self, player: Player) -> bool {
        self.player_track(player)
            .is_some_and(|track| self.playlist.tracks[track].muted)
    }

    /// Leaves `player`'s part to you, by muting its track, or gives it
    /// back to the band. The band goes on writing the part, so it is
    /// there, up to date, when you give it back. Returns false if the
    /// band has no part for the player yet.
    pub fn set_you_play(&mut self, player: Player, you: bool) -> bool {
        let Some(track) = self.player_track(player) else {
            return false;
        };
        self.playlist.tracks[track].muted = you;
        true
    }

    /// The channel `player` plays on, for every player but the drums:
    /// the one to play yourself when you take over the part.
    pub fn player_channel(&self, player: Player) -> Option<usize> {
        self.band.as_ref()?.channel(player)
    }

    /// What each note of the soloist's part in `pattern` is (see
    /// [`solo::note_roles`]), with the harmony it is over, in the order
    /// of the pattern's notes on the soloist's channel; `None` for a note
    /// where no harmony sounds. `None` if `pattern` is not the soloist's.
    pub fn solo_roles(&self, pattern: usize) -> Option<Vec<Option<(NoteRole, Over)>>> {
        let band = self.band.as_ref()?;
        let channel = band.channel(Player::Soloist)?;
        let solo = self
            .patterns
            .get(pattern)
            .filter(|solo| solo.player == Some(Player::Soloist))?;
        let at = self
            .playlist
            .clips
            .iter()
            .find(|clip| clip.pattern == pattern)?
            .start;
        let markers: Vec<ChordMarker> = self
            .playlist
            .chord_markers
            .iter()
            .filter(|marker| marker.start < band.end)
            .copied()
            .collect();
        let keys = self.key_map();
        let overs: Vec<Over> = self
            .played_harmonies(&markers, band.end, band.style.comping)
            .iter()
            .filter_map(|played| Over::of(played, keys.key_at(played.start)))
            .collect();
        let notes: Vec<NoteEvent> = solo
            .notes(channel)
            .iter()
            .map(|note| NoteEvent {
                start: at + note.start,
                ..*note
            })
            .collect();
        let roles = solo::note_roles(&notes, &overs);
        let with_harmony = roles.into_iter().zip(&notes).map(|(role, note)| {
            let over = overs
                .iter()
                .find(|over| (over.start..over.end).contains(&note.start))?;
            Some((role?, *over))
        });
        Some(with_harmony.collect())
    }

    /// File > New: an empty chord sheet, and a band ready to play it in
    /// the [`DEFAULT_STYLE`].
    pub fn with_band() -> Self {
        let mut project = Project::empty();
        project.set_band_style(BandStyle::named(DEFAULT_STYLE));
        project.remove_unused_tracks();
        project
    }

    /// Has the band play in `style`, or stops it with `None`.
    ///
    /// A band coming into a song with music of its own first writes the
    /// chords the song plays on the chord sheet, if it is empty, and
    /// mutes the song's own chords, bass and drums, so the two do not
    /// play over each other (see [`Project::stop_band`] for leaving).
    ///
    /// A style in another time signature puts the song in it bar for
    /// bar, as a rhythm from the library does (see
    /// [`remeter`](crate::remeter::remeter)). While the sheet is empty,
    /// the song also takes the style's tempo. The song always takes the
    /// style's swing, as it is a part of how the style is played.
    pub fn set_band_style(&mut self, style: Option<&'static BandStyle>) {
        let Some(style) = style else {
            self.stop_band();
            return;
        };
        if self.band.as_ref().is_some_and(|band| {
            band.style == style
                && band.instrument_style == style
                && band.harmony_style.is_none()
                && !band.accents_only
                && band.drums == BandDrums::Style
                && band.custom_rhythm.is_none()
                && band.custom_instruments.is_none()
        }) {
            return;
        }
        let arriving = self.band.is_none();
        if arriving {
            let song_end = self.song_length();
            if self.playlist.chord_markers.is_empty() && song_end > 0 {
                self.write_heard_chords();
            }
            let mut band = Band::new(style);
            band.end = song_end;
            band.since = self.channels.len();
            if !self.playlist.chord_markers.is_empty() && !style.follows_the_song() {
                band.muted = self.mute_the_songs_own(true);
            }
            self.band = Some(band);
        } else {
            let horns_locked = self.plays_locked(Player::Horns);
            let Some(band) = &mut self.band else { return };
            band.custom_rhythm = None;
            band.style = style;
            band.take_instruments(style, horns_locked);
            band.harmony_style = None;
            band.accents_only = false;
            band.drums = BandDrums::Style;
        }
        let empty = self.playlist.chord_markers.is_empty() && self.song_length() == 0;
        let other_meter =
            self.meter().first() != style.time || !self.playlist.time_markers.is_empty();
        if other_meter && !style.follows_the_song() {
            if empty {
                self.time_signature = style.time;
                self.playlist.time_markers.clear();
            } else {
                crate::remeter::remeter(self, style.time);
            }
        }
        if self.playlist.chord_markers.is_empty() {
            self.tempo = Tempo::new(style.tempo);
        }
        self.swing = style.swing();
        self.band_channels(true);
        self.write_band();
    }

    /// Applies the selected components, each on its own: the drums, the
    /// chord and bass parts, the sounds, the meter, the tempo, the swing
    /// and the progression. What is left out stays as the song has it.
    pub fn apply_band_parts(
        &mut self,
        style: &'static BandStyle,
        harmony: Option<&'static Style>,
        parts: apply::StyleParts,
    ) {
        let parts = parts.following(style.follows_the_song());
        // The band comes in to play the style's chord and bass parts with
        // its instruments, on tracks of its own; without the style's
        // drums, the song's own go on playing. A style that follows the
        // song comes in alongside it with its patterns or its
        // instruments, either, and plays as it does: it never plays the
        // song's own parts.
        let follows = style.follows_the_song();
        let joins = if follows {
            parts.rhythm || parts.matching_style || parts.instruments
        } else {
            parts.instruments && parts.matching_style
        };
        if self.band.is_none() && joins {
            let mut band = Band::new(style);
            if !parts.rhythm && !follows {
                band.drums = BandDrums::Song;
            }
            band.end = self.song_length();
            band.since = self.channels.len();
            if self.playlist.chord_markers.is_empty() && band.end > 0 {
                self.write_heard_chords();
            }
            if !self.playlist.chord_markers.is_empty() && !style.follows_the_song() {
                band.muted = self.mute_the_songs_own(parts.rhythm);
            }
            self.band = Some(band);
        }
        let horns_locked = self.plays_locked(Player::Horns);
        let Some(band) = &mut self.band else { return };
        if parts.matching_style {
            // The drums the band played stay.
            if !parts.rhythm && band.drums == BandDrums::Style {
                band.drums = match band.custom_rhythm.take() {
                    Some(saved) => BandDrums::Saved(saved),
                    None => BandDrums::Of(band.style),
                };
            }
            band.custom_rhythm = None;
            band.style = style;
            band.harmony_style = harmony;
            band.accents_only = false;
        }
        let mut drums_come = false;
        if parts.rhythm {
            drums_come = band.drums == BandDrums::Song;
            band.drums = BandDrums::Of(style);
        }
        band.tidy_drums();
        if parts.instruments {
            band.take_instruments(style, horns_locked);
        }
        self.drums_come(drums_come);
        let time = self.meter();
        if parts.time_signature {
            crate::remeter::remeter(self, harmony.map_or(style.time, |style| style.time));
        }
        if parts.tempo && time == self.meter() {
            self.take_tempo(Tempo::new(harmony.map_or(style.tempo, |style| style.tempo)));
        }
        if parts.swing {
            self.swing = style.swing();
        }
        if parts.progression
            && let Some(harmony) = harmony
        {
            self.replace_progression(harmony);
        }
        self.band_channels(parts.instruments);
        self.write_band();
    }

    /// Apply a portable style while keeping unchecked components intact.
    pub fn apply_user_style(
        &mut self,
        style: &crate::library::UserStyle,
        parts: apply::StyleParts,
    ) -> Result<(), String> {
        style.validate()?;
        // The chart is read into the song's bars, which can fail halfway
        // in another meter, after the band and the tempo changed: so it
        // is applied to a copy, and an error changes nothing.
        let mut project = self.clone();
        project.apply_valid_user_style(style, parts)?;
        *self = project;
        Ok(())
    }

    fn apply_valid_user_style(
        &mut self,
        style: &crate::library::UserStyle,
        parts: apply::StyleParts,
    ) -> Result<(), String> {
        // Only the style's chord and bass parts need a band to play them:
        // its tempo, meter, swing or chords alone change the song as it
        // is. Without the style's drums, the song's own go on playing.
        if self.band.is_none() && parts.matching_style {
            // The players the style has no lanes for play as the band
            // usually does in its time signature (a waltz for 3/4).
            let Some(base) =
                BandStyle::named(crate::library::Rhythm::for_time(style.time, None).name)
                    .or_else(|| BandStyle::named("Pop beat"))
            else {
                return Err(trf(
                    "there is no band style to play a song in {}",
                    &[&style.time],
                ));
            };
            let mut band = Band::new(base);
            if !parts.rhythm {
                band.drums = BandDrums::Song;
            }
            band.end = self.song_length();
            band.since = self.channels.len();
            if self.playlist.chord_markers.is_empty() && band.end > 0 {
                self.write_heard_chords();
            }
            if !self.playlist.chord_markers.is_empty() {
                band.muted = self.mute_the_songs_own(parts.rhythm);
            }
            self.band = Some(band);
        }
        if parts.time_signature {
            crate::remeter::remeter(self, style.time);
        }
        if parts.tempo {
            self.take_tempo(style.tempo);
        }
        if parts.swing {
            self.swing = style.swing;
        }
        if parts.progression {
            let meter = self.meter();
            let end = self
                .band
                .as_ref()
                .map_or(self.song_length(), |band| band.end)
                .max(
                    self.playlist
                        .chord_markers
                        .last()
                        .map_or(0, |marker| meter.bar_at(marker.start).end()),
                );
            let from = self
                .band
                .as_ref()
                .map_or(0, |band| meter.bar_at(band.form.intro).number);
            let chart: Vec<_> = style.chart.split('|').collect();
            let count = if end == 0 {
                from + chart.len()
            } else {
                meter.bar_at(end - 1).number + 1
            };
            let keys = self.key_map();
            for index in from..count {
                let bar = meter.bar(index);
                let markers = read_bar(
                    chart[(index - from) % chart.len()].trim(),
                    &bar,
                    keys.key_at(bar.start),
                )?;
                self.set_chords(bar.start, bar.end(), &markers);
            }
        }
        let mut drums_come = false;
        if let Some(band) = &mut self.band {
            if parts.matching_style {
                // The drums the band played stay.
                if !parts.rhythm && band.drums == BandDrums::Style {
                    band.drums = match band.custom_rhythm.take() {
                        Some(saved) => BandDrums::Saved(saved),
                        None => BandDrums::Of(band.style),
                    };
                }
                band.custom_rhythm = Some(Box::new(style.clone()));
                band.accents_only = false;
                band.harmony_style = None;
            }
            if parts.rhythm {
                drums_come = band.drums == BandDrums::Song;
                band.drums = BandDrums::Saved(Box::new(style.clone()));
            }
            band.tidy_drums();
            if parts.instruments {
                band.custom_instruments = Some(Box::new(style.clone()));
            }
        }
        self.drums_come(drums_come);
        self.band_channels(parts.instruments);
        self.write_band();
        Ok(())
    }

    pub fn write_user_progression(
        &mut self,
        style: &crate::library::UserStyle,
        from: usize,
    ) -> Result<usize, String> {
        style.validate()?;
        if self.playlist.chord_markers.is_empty() && self.song_length() == 0 {
            self.scale.kind = style.mode;
        }
        let meter = self.meter();
        let keys = self.key_map();
        let mut count = 0;
        for (index, text) in style.chart.split('|').enumerate() {
            let bar = meter.bar(from + index);
            if bar.end() > MAX_SONG_STEPS {
                break;
            }
            let markers = read_bar(text.trim(), &bar, keys.key_at(bar.start))?;
            self.set_chords(bar.start, bar.end(), &markers);
            count += 1;
        }
        Ok(count)
    }

    fn user_style_channel(
        &mut self,
        lane: &crate::library::StyleLane,
        sounds: bool,
    ) -> Option<usize> {
        if lane.player == Player::Drums {
            return self
                .channels
                .iter()
                .position(|channel| channel.kind == lane.channel.kind)
                .or_else(|| self.add_channel(lane.channel.clone()));
        }
        let had = self
            .band
            .as_ref()?
            .channel(lane.player)
            .filter(|index| *index < self.channels.len());
        let index = had.or_else(|| {
            let mut channel = if sounds {
                lane.channel.clone()
            } else {
                Channel::synth()
            };
            channel.name = lane.player.name().to_string();
            self.add_channel(channel)
        })?;
        if sounds {
            self.set_sound(index, Some(lane.channel.kind.clone()), lane.channel.program);
        }
        self.band.as_mut()?.channels[lane.player.index()] = Some(index);
        Some(index)
    }

    /// Repeats a style's chart over the existing song, or writes it once on
    /// an empty sheet. Keep the key and stop at the song's current last bar.
    pub fn replace_progression(&mut self, style: &Style) {
        let meter = self.meter();
        let end = self
            .band
            .as_ref()
            .map_or(self.song_length(), |band| band.end)
            .max(
                self.playlist
                    .chord_markers
                    .last()
                    .map_or(0, |marker| meter.bar_at(marker.start).end()),
            );
        let from = self
            .band
            .as_ref()
            .map_or(0, |band| meter.bar_at(band.form.intro).number);
        let bars = if end == 0 {
            from + style.bars()
        } else {
            meter.bar_at(end - 1).number + 1
        };
        let chart: Vec<_> = style.chart.split('|').collect();
        if chart.is_empty() {
            return;
        }
        let keys = self.key_map();
        for index in from..bars {
            let bar = meter.bar(index);
            if let Ok(markers) = read_bar(
                chart[(index - from) % chart.len()].trim(),
                &bar,
                keys.key_at(bar.start),
            ) {
                self.set_chords(bar.start, bar.end(), &markers);
            }
        }
    }

    /// Stops the band: its patterns go, with their clips and the tracks
    /// they leave empty, and the song's own channels and tracks it muted
    /// play again. The chord sheet stays, and so do the parts on locked
    /// channels, as the song's own; a song's Keys or Bass that a kept
    /// piano or bass replaces stays muted.
    pub fn stop_band(&mut self) {
        // A locked piano or bass of the band stays as the song's own, so
        // the song's first Keys or Bass, which it played over, stays
        // quiet: the two would double.
        let replaced: Vec<&str> = [(Player::Piano, KEYS), (Player::Bass, BASS)]
            .into_iter()
            .filter(|&(player, _)| self.plays_locked(player))
            .map(|(_, name)| name)
            .collect();
        self.keep_locked_parts();
        let Some(band) = self.band.take() else {
            return;
        };
        let tracks: Vec<usize> = self
            .playlist
            .clips
            .iter()
            .filter(|clip| self.patterns[clip.pattern].player.is_some())
            .map(|clip| clip.track)
            .collect();
        for index in (0..self.patterns.len()).rev() {
            if self.patterns[index].player.is_some() && self.remove_pattern(index).is_none() {
                // The last pattern stays, empty and the song's own.
                let pattern = &mut self.patterns[index];
                pattern.player = None;
                for channel in 0..pattern.rows() {
                    pattern.set_notes(channel, []);
                }
            }
        }
        let mut tracks: Vec<usize> = tracks
            .into_iter()
            .filter(|&track| self.playlist.clips.iter().all(|clip| clip.track != track))
            .collect();
        tracks.sort_unstable();
        tracks.dedup();
        for track in tracks.into_iter().rev() {
            self.remove_track(track);
        }
        for channel in (band.since..self.channels.len()).rev() {
            if !self.is_locked(channel)
                && self
                    .patterns
                    .iter()
                    .all(|pattern| pattern.notes(channel).is_empty())
            {
                self.remove_channel(channel);
            }
        }
        for name in &band.muted.channels {
            if replaced.contains(&name.as_str()) {
                continue;
            }
            if let Some(channel) = self.channels.iter_mut().find(|c| c.name == *name) {
                channel.muted = false;
            }
        }
        for name in &band.muted.tracks {
            if let Some(track) = self.playlist.tracks.iter_mut().find(|t| t.name == *name) {
                track.muted = false;
            }
        }
    }

    /// Whether `player`'s part was edited since the band wrote it, so the
    /// band leaves it as it is.
    pub fn is_frozen(&self, player: Player) -> bool {
        self.band.as_ref().is_some_and(|band| {
            band.written[player.index()].is_some_and(|written| written != self.part_print(player))
        })
    }

    /// Has the band play `player`'s part again, from the chord sheet,
    /// in place of what was edited. Returns `false` if there is no band,
    /// or the part is locked.
    pub fn play_again(&mut self, player: Player) -> bool {
        if self.plays_locked(player) {
            return false;
        }
        let Some(band) = &mut self.band else {
            return false;
        };
        band.written[player.index()] = None;
        self.band_channels(false);
        self.write_band();
        true
    }

    /// Has a soloist improvise over the chorus, or stops it. The soloist
    /// is experimental. Returns `false` if there is no band, or the
    /// soloist's channel is locked.
    pub fn set_soloist(&mut self, on: bool) -> bool {
        if self.plays_locked(Player::Soloist) {
            return false;
        }
        let Some(band) = &mut self.band else {
            return false;
        };
        if band.has_soloist() == on {
            return true;
        }
        band.solo = on.then_some(1);
        band.written[Player::Soloist.index()] = None;
        self.band_channels(false);
        self.write_band();
        true
    }

    /// Has the soloist play another solo, in place of the one it plays
    /// (or one you edited). Returns `false` if there is no soloist, or
    /// its channel is locked.
    pub fn another_solo(&mut self) -> bool {
        if self.plays_locked(Player::Soloist) {
            return false;
        }
        let Some(band) = &mut self.band else {
            return false;
        };
        let Some(solo) = band.solo else {
            return false;
        };
        band.solo = Some(solo.wrapping_add(1));
        band.written[Player::Soloist.index()] = None;
        self.band_channels(false);
        self.write_band();
        true
    }

    /// Has the horn section play with the band, or stops it. Returns
    /// `false` if there is no band, or the horns' channel is locked.
    pub fn set_horns(&mut self, on: bool) -> bool {
        if self.plays_locked(Player::Horns) {
            return false;
        }
        let Some(band) = &mut self.band else {
            return false;
        };
        if band.horns == on {
            return true;
        }
        band.horns = on;
        band.written[Player::Horns.index()] = None;
        self.band_channels(false);
        self.write_band();
        true
    }

    /// Writes the band's parts again if what they are played from (the
    /// chord sheet, the keys, the bars or the style) changed since they
    /// were written. The app calls this after every change, so the band
    /// always plays what the sheet says. Frozen parts stay as they are.
    /// Returns whether anything was written.
    pub fn follow_chord_sheet(&mut self) -> bool {
        let Some(band) = &self.band else {
            return false;
        };
        if band.heard == Some(self.band_inputs()) {
            return false;
        }
        self.band_channels(false);
        self.write_band();
        true
    }

    /// Ends the song with bar `bar`: the band stops at its end, and the
    /// chords written after it are cleared. Returns `false` if there is
    /// no band.
    pub fn end_song_at(&mut self, bar: usize) -> bool {
        if self.band.is_none() {
            return false;
        }
        let end = self.meter().bar(bar).end();
        self.playlist
            .chord_markers
            .retain(|marker| marker.start < end);
        if let Some(band) = &mut self.band {
            band.end = end;
            band.variations.retain(|marker| marker.start < end);
            band.styles.retain(|marker| marker.start < end);
        }
        self.write_band();
        true
    }

    /// Writes the chords of a library style's progression on the chord
    /// sheet, from bar `from` on, one bar of its chart in each bar, and
    /// returns how many bars were written. The chords are named in the
    /// key where they land: "V7" in Bb major is F7. A chord that is
    /// already sounding is not written again, so the sheet reads like a
    /// chart: "I | I | IV" writes C, then nothing, then F.
    ///
    /// On an empty sheet the song takes the progression's mode, on the
    /// same key note: "i bVII bVI" is a minor progression, Am G F in A
    /// minor.
    pub fn write_progression(&mut self, style: &Style, from: usize) -> usize {
        if self.playlist.chord_markers.is_empty() && self.scale.kind.is_tonal() {
            let scale = Scale::new(self.scale.root, style.mode);
            if self.song_length() == 0 {
                self.scale = scale;
            } else {
                self.change_key(scale);
            }
        }
        let meter = self.meter();
        let keys = self.key_map();
        let mut written = 0;
        for (index, text) in style.chart.split('|').enumerate() {
            let bar = meter.bar(from + index);
            if bar.end() > MAX_SONG_STEPS {
                break;
            }
            let Ok(mut markers) = read_bar(text.trim(), &bar, keys.key_at(bar.start)) else {
                break;
            };
            let sounding = bar
                .start
                .checked_sub(1)
                .and_then(|step| self.chord_at(step));
            if let (Some(first), Some(sounding)) = (markers.first(), sounding)
                && first.start == bar.start
                && first.harmony == sounding.harmony
            {
                markers.remove(0);
            }
            self.set_chords(bar.start, bar.end(), &markers);
            written += 1;
        }
        written
    }

    // ----- Writing the parts -----

    /// Puts the mix back as it starts: the master level, the reverb and no
    /// solo, each channel as loud as a new one of its kind (a band
    /// player's as the band sets it), in the middle with all of it in the
    /// reverb and no effects of its own, and nothing muted but the song's
    /// own parts the band plays instead.
    pub fn reset_mix(&mut self) {
        self.mixer = crate::Mixer::default();
        let band = self.band.as_ref();
        for (index, channel) in self.channels.iter_mut().enumerate() {
            let player = band.and_then(|band| {
                Player::ALL
                    .into_iter()
                    .find(|&player| band.channel(player) == Some(index))
            });
            let named = [KEYS, BASS, apply::LEAD].contains(&channel.name.as_str())
                || channel.name.starts_with("Voice ");
            channel.volume = match (player, &channel.kind) {
                (Some(player), _) => player_volume(player),
                (None, ChannelKind::Synth) if named => apply::synth_volume(&channel.name),
                (None, ChannelKind::Synth) => Channel::synth().volume,
                (None, _) => Channel::drum(Drum::Kick).volume,
            };
            channel.muted = band.is_some_and(|band| band.muted().channels.contains(&channel.name));
            channel.pan = 0.0;
            channel.reverb_send = 1.0;
            channel.effects = crate::Effects::default();
        }
    }

    /// Makes sure the band has its channels: the piano and the bass, the
    /// guitar, strings and organ its style adds (each in the instrument
    /// of its style), and the drums its rhythm plays.
    fn band_channels(&mut self, use_instruments: bool) {
        let Some(band) = self.band.clone() else {
            return;
        };
        let style = band.instrument_style;
        let extras = style.extras();
        let mut channels = band.channels;
        for player in Player::ALL {
            let sound = match player {
                Player::Piano => Some((KEYS, style.piano_program())),
                Player::Bass => Some((BASS, style.bass_program())),
                Player::Drums => None,
                Player::Guitar => extras.guitar.map(|guitar| (player.name(), guitar.program)),
                Player::Strings => extras
                    .strings
                    .map(|strings| (player.name(), strings.program)),
                Player::Organ => extras.organ.map(|organ| (player.name(), organ.program)),
                Player::Soloist => band
                    .solo
                    .map(|_| (player.name(), Soloist::of(style.genre).program)),
                Player::Horns => band.horns().map(|horns| (player.name(), horns.program)),
            };
            if let Some((other, program)) = sound {
                let volume = player_volume(player);
                channels[player.index()] = self.band_channel(
                    band.channel(player),
                    player.name(),
                    other,
                    volume,
                    program,
                    use_instruments,
                );
            }
        }
        let drums = match band.drums {
            BandDrums::Of(style) => Some(style),
            BandDrums::Song => None,
            _ => Some(band.style),
        };
        if let Some(rhythm) = drums.and_then(|style| style.rhythm) {
            for drum in rhythm.drums() {
                apply::drum_channel(self, drum);
            }
        }
        if let Some(band) = &mut self.band {
            band.channels = channels;
        }
        let saved_drums = match &band.drums {
            BandDrums::Saved(style) => Some(style.as_ref()),
            _ => None,
        };
        // The lanes each style plays: a saved style's drums only when the
        // band plays them.
        for (style, sounds, its_drums, the_rest) in [
            (
                band.custom_instruments.as_deref(),
                use_instruments,
                true,
                true,
            ),
            (
                band.custom_rhythm.as_deref(),
                false,
                band.drums == BandDrums::Style,
                !band.accents_only,
            ),
            (saved_drums, false, true, false),
        ] {
            if let Some(style) = style {
                for lane in &style.lanes {
                    let plays = match lane.player {
                        Player::Drums => its_drums,
                        _ => the_rest || sounds,
                    };
                    if plays {
                        self.user_style_channel(lane, sounds);
                    }
                }
            }
        }
    }

    /// Removes the channels and tracks of the players the band's style
    /// does not have, of those just `rewritten`, once they play nothing:
    /// after a change to a style without strings, the strings go. A part
    /// you edited was not rewritten, and stays on its channel and track.
    fn drop_unused_players(&mut self, rewritten: &[Player]) {
        let Some(band) = self.band.clone() else {
            return;
        };
        let players = band.players();
        let gone: Vec<Player> = rewritten
            .iter()
            .copied()
            .filter(|player| !players.contains(player))
            .collect();
        let mut channels: Vec<usize> = gone
            .iter()
            .filter_map(|&player| band.channel(player))
            .filter(|&channel| {
                channel >= band.since
                    && !self.is_locked(channel)
                    && self
                        .patterns
                        .iter()
                        .all(|pattern| pattern.notes(channel).is_empty())
            })
            .collect();
        channels.sort_unstable();
        for channel in channels.into_iter().rev() {
            self.remove_channel(channel);
        }
        for player in gone {
            let empty = (0..self.playlist.tracks.len()).rev().find(|&track| {
                self.playlist.tracks[track].name == player.name()
                    && self.playlist.clips.iter().all(|clip| clip.track != track)
            });
            if let Some(track) = empty {
                self.remove_track(track);
            }
        }
    }

    /// The channel a player plays on: the one it has, if it still plays
    /// notes, or a new synth called `name` (or "Band" and `name`, if the
    /// song already has a channel called that, or the library's `other`
    /// name). Its instrument follows the style.
    fn band_channel(
        &mut self,
        had: Option<usize>,
        name: &str,
        other: &str,
        volume: f32,
        program: u8,
        use_instruments: bool,
    ) -> Option<usize> {
        let plays_notes = |channel: &Channel| channel.is_pitched();
        let index = match had.filter(|&index| self.channels.get(index).is_some_and(plays_notes)) {
            Some(index) => index,
            None => {
                let taken = self
                    .channels
                    .iter()
                    .any(|c| c.name == name || c.name == other);
                let name = if taken {
                    format!("Band {}", name.to_lowercase())
                } else {
                    name.to_string()
                };
                self.add_channel(Channel {
                    name,
                    kind: ChannelKind::Synth,
                    volume,
                    muted: false,
                    program: Some(program),
                    pan: 0.0,
                    reverb_send: 1.0,
                    sent_on: None,
                    effects: Default::default(),
                })?
            }
        };
        if use_instruments {
            self.set_sound(index, None, Some(program));
        }
        Some(index)
    }

    /// Mutes the chords, bass and drums the song plays without the band:
    /// the channels the library writes chords and the bass on, unless
    /// they are locked, and the tracks that hold only drums. Returns what
    /// it muted.
    fn mute_the_songs_own(&mut self, drums: bool) -> Muted {
        let mut muted = Muted::default();
        for index in 0..self.channels.len() {
            if self.accompanies(index) && !self.channels[index].muted && !self.is_locked(index) {
                muted.channels.push(self.channels[index].name.clone());
                self.channels[index].muted = true;
            }
        }
        if drums {
            muted.tracks = self.mute_the_songs_drums();
        }
        muted
    }

    /// Mutes the song's own drum tracks when the band's drums `come` in
    /// their place.
    fn drums_come(&mut self, come: bool) {
        if !come {
            return;
        }
        let tracks = self.mute_the_songs_drums();
        if let Some(band) = &mut self.band {
            band.muted.tracks.extend(tracks);
        }
    }

    /// Mutes the song's own tracks that play only drums, for the band's
    /// drums to play in their place. Returns their names.
    fn mute_the_songs_drums(&mut self) -> Vec<String> {
        let mut muted = Vec::new();
        let is_drum = |channel: &Channel| {
            matches!(channel.kind, ChannelKind::Sampler(SampleSource::Builtin(_)))
        };
        for track in 0..self.playlist.tracks.len() {
            let mut clips = self
                .playlist
                .clips
                .iter()
                .filter(|clip| clip.track == track);
            let only_drums = clips.clone().next().is_some()
                && clips.all(|clip| {
                    let pattern = &self.patterns[clip.pattern];
                    !pattern.is_empty()
                        && pattern
                            .all_notes()
                            .all(|(channel, _)| is_drum(&self.channels[channel]))
                });
            let state = &mut self.playlist.tracks[track];
            if only_drums && !state.muted {
                state.muted = true;
                muted.push(state.name.clone());
            }
        }
        muted
    }

    /// A fingerprint of what the band's parts are played from.
    fn band_inputs(&self) -> u64 {
        let mut print = Fingerprint::new();
        if let Some(band) = &self.band {
            print.bytes(format!("{:?}", band.custom_rhythm).as_bytes());
            print.bytes(band.style.name.as_bytes());
            print.bytes(
                format!(
                    "{:?}{:?}{}",
                    band.instrument_style.name,
                    band.harmony_style.map(|style| style.name),
                    band.accents_only
                )
                .as_bytes(),
            );
            print.number(band.end);
            print.bytes(
                format!(
                    "{:?}{:?}{:?}{:?}",
                    band.form,
                    self.ending(),
                    band.variations,
                    band.solo
                )
                .as_bytes(),
            );
            // Only bands with horns count them, so a song saved before
            // there were horns plays as it was written.
            if band.horns {
                print.bytes(b"horns");
            }
            // Only bands whose drums come from elsewhere count them.
            if band.drums != BandDrums::Style {
                print.bytes(b"drums");
                print.bytes(format!("{:?}", band.drums_name()).as_bytes());
                if let BandDrums::Saved(style) = &band.drums {
                    print.bytes(format!("{style:?}").as_bytes());
                }
            }
            // Only songs with section styles count them, so a song saved
            // before there were any plays as it was written.
            for marker in &band.styles {
                print.number(marker.start);
                print.bytes(marker.style.map_or("", |style| style.name).as_bytes());
            }
        }
        // What the band plays from only has to be compared within one
        // run of the app (a change means "write again"), so the debug
        // text of these values is precise enough.
        let text = format!(
            "{:?}{:?}{:?}{:?}",
            self.meter(),
            self.key_map(),
            self.playlist.chord_markers,
            self.playlist
                .sections
                .iter()
                .map(|section| (section.start, section.length))
                .collect::<Vec<_>>()
        );
        print.bytes(text.as_bytes());
        // Only songs with locks count them, so a song saved before there
        // were locks plays as it was written.
        if !self.locked().is_empty() {
            print.bytes(format!("{:?}", self.locked()).as_bytes());
            // The horns came later: leaving them out keeps the songs
            // saved before them as they were.
            for player in Player::ALL.into_iter().filter(|&p| p != Player::Horns) {
                print.number(usize::from(self.song_keeps(player)));
            }
        }
        print.0
    }

    /// A fingerprint of `player`'s part as it is now: where its clips
    /// are, and every note of their patterns. Which track the clips are
    /// on, and which number their patterns have, do not count, so moving
    /// a track or removing another pattern does not freeze the part.
    fn part_print(&self, player: Player) -> u64 {
        let mut clips: Vec<&Clip> = self
            .playlist
            .clips
            .iter()
            .filter(|clip| self.patterns[clip.pattern].player == Some(player))
            .collect();
        clips.sort_by_key(|clip| (clip.start, clip.track, clip.pattern));
        let mut print = Fingerprint::new();
        for clip in clips {
            let pattern = &self.patterns[clip.pattern];
            print.number(clip.start);
            print.number(clip.repeats);
            print.number(pattern.length());
            for (channel, note) in pattern.all_notes() {
                print.number(channel);
                print.number(usize::from(note.pitch.midi()));
                print.number(note.start);
                print.number(note.length);
                print.number(usize::from(note.velocity));
                print.number(usize::from(note.nudge));
                print.number(usize::from(note.trim));
            }
        }
        print.0
    }

    /// Runs `change`, a tool that moves the whole song (into another key
    /// or time signature, or a section copied), and keeps the band's
    /// parts following the sheet: the band's notes moved with everything
    /// else, which is not an edit of yours.
    pub(crate) fn moving_the_band<R>(&mut self, change: impl FnOnce(&mut Project) -> R) -> R {
        self.keeping_parts(|_, _| true, change)
    }

    /// Runs `change`, which removes channel `index`, and keeps the band's
    /// parts that play nothing on it following the sheet: the channels
    /// after it are numbered one lower, which is not an edit of yours. A
    /// part that played on it lost those notes, and is frozen.
    pub(crate) fn removing_channel<R>(
        &mut self,
        index: usize,
        change: impl FnOnce(&mut Project) -> R,
    ) -> R {
        let plays_on_it = |project: &Project, player: Player| {
            project
                .patterns
                .iter()
                .any(|pattern| pattern.player == Some(player) && !pattern.notes(index).is_empty())
        };
        self.keeping_parts(|project, player| !plays_on_it(project, player), change)
    }

    /// Runs `change`, and afterwards takes what the parts that followed
    /// the sheet before it, and that `keep` picks, are as the band's own.
    fn keeping_parts<R>(
        &mut self,
        keep: impl Fn(&Project, Player) -> bool,
        change: impl FnOnce(&mut Project) -> R,
    ) -> R {
        let following: Vec<Player> = Player::ALL
            .into_iter()
            .filter(|&player| {
                self.band
                    .as_ref()
                    .is_some_and(|band| band.written[player.index()].is_some())
                    && !self.is_frozen(player)
                    && keep(self, player)
            })
            .collect();
        let result = change(self);
        let prints: Vec<(Player, u64)> = following
            .into_iter()
            .map(|player| (player, self.part_print(player)))
            .collect();
        if let Some(band) = &mut self.band {
            for (player, print) in prints {
                band.written[player.index()] = Some(print);
            }
        }
        result
    }

    /// Writes every part that is not frozen from the chord sheet: the
    /// intro, the chorus, and the ending after it, if the band plays one.
    fn write_band(&mut self) {
        let Some(band) = self.band.clone() else {
            return;
        };
        let meter = self.meter();
        let last_written = self
            .playlist
            .chord_markers
            .last()
            .map_or(0, |last| meter.bar_at(last.start).end());
        // The end of the chorus: of the last bar written.
        let end = band.end.max(last_written).min(MAX_SONG_STEPS);
        if let Some(band) = &mut self.band {
            band.end = end;
        }
        let ending = self.ending();
        let parts_end = ending.map_or(end, |ending| ending.end);
        let bars: Vec<Bar> = meter
            .bars()
            .take_while(|bar| bar.start < parts_end)
            .collect();
        // The band plays by the song's sections, heard where none are
        // written: its patterns start where they do, and the drums lay
        // the rhythm out over them.
        let form = self.form_sections();
        let chunks = self.chunks(&bars, &form);
        let mut markers: Vec<ChordMarker> = self
            .playlist
            .chord_markers
            .iter()
            .filter(|marker| marker.start < end)
            .copied()
            .collect();
        if let Some(ending) = ending {
            markers.push(ChordMarker::new(ending.start, ending.harmony));
        }
        let harmonies = self.played_harmonies(&markers, parts_end, band.style.comping);
        let keys = self.key_map();
        let breaks = stops::Breaks::of(&markers, parts_end);
        // The last bar of an ending is one chord, struck and held.
        let held = ending.and_then(|_| bars.last().copied());
        let variations = band.variations.clone();
        let runs = runs(&bars, held, &variations, &band.styles);
        let score = Score {
            bars,
            chunks,
            form,
            end,
            held,
            breaks,
            harmonies,
            keys,
            runs,
            variations,
            time: meter.first(),
        };
        // A style that follows the song plays along with it: it listens
        // to the song first.
        let follows = sections::styles_played(&band.styles)
            .into_iter()
            .any(|style| band.playing(style).style.follows_the_song());
        let following = follows
            .then(|| following::Following::hear(self, &score.bars))
            .flatten();
        let mut rewritten = Vec::new();
        for player in Player::ALL {
            if self.is_frozen(player) || self.plays_locked(player) {
                continue;
            }
            // Each style that plays somewhere plays the whole song, and
            // each of its notes is kept where that style plays. The
            // soloist improvises over the chorus whatever the style.
            let notes = match player {
                Player::Soloist => self.band_notes(&band, player, &score),
                _ => {
                    let mut notes = Vec::new();
                    for style in sections::styles_played(&band.styles) {
                        let playing = band.playing(style);
                        let mut part = self.band_notes(&playing, player, &score);
                        if let Some(following) = &following
                            && playing.style.follows_the_song()
                        {
                            part = self.played_along(following, &playing, player, part, &score);
                        }
                        notes.extend(sections::where_it_plays(part, &band.styles, style));
                    }
                    notes
                }
            };
            let notes = match player {
                Player::Drums => self.keeping_locked_drums(notes, &band.muted),
                _ if self.song_keeps(player) => Vec::new(),
                _ => notes,
            };
            self.write_part(player, &score.chunks, notes);
            rewritten.push(player);
        }
        // A part the band left as it is (a locked one) and that is as the
        // band wrote it stays so: dropping a channel renumbers its notes,
        // which is not an edit.
        let left: Vec<Player> = Player::ALL
            .into_iter()
            .filter(|player| {
                !rewritten.contains(player)
                    && band.written[player.index()].is_some()
                    && !self.is_frozen(*player)
            })
            .collect();
        self.drop_unused_players(&rewritten);
        // Fingerprints last: dropping a channel renumbers the ones after it.
        // A player the band does not have has no part to keep.
        let players = band.players();
        let mut written = band.written;
        for player in rewritten {
            written[player.index()] = players.contains(&player).then(|| self.part_print(player));
        }
        for player in left {
            written[player.index()] = Some(self.part_print(player));
        }
        let heard = self.band_inputs();
        if let Some(band) = &mut self.band {
            band.written = written;
            band.heard = Some(heard);
        }
    }

    /// What `player` of `band` plays along with the song, from `part`, the
    /// notes its style wrote (see [`following::Following::play_along`]).
    /// The horns and the timpani strike the chords and the bass notes
    /// held through each harmony.
    fn played_along(
        &self,
        following: &following::Following,
        band: &Band,
        player: Player,
        part: Vec<(usize, NoteEvent)>,
        score: &Score,
    ) -> Vec<(usize, NoteEvent)> {
        let held = match player {
            Player::Piano => Comping::Held.play_in_bars(&score.bars, &score.harmonies),
            Player::Bass => Bass::Held.play_in_bars(&score.bars, &score.harmonies),
            _ => Vec::new(),
        };
        let kick = self.channels.iter().position(|channel| {
            channel.kind == ChannelKind::Sampler(SampleSource::Builtin(Drum::Kick))
        });
        following.play_along(player, part, &held, band.channel(player), kick)
    }

    /// What `player` plays in `band`'s style all through `score`, on the
    /// song's timeline, by channel.
    fn band_notes(
        &mut self,
        band: &Band,
        player: Player,
        score: &Score,
    ) -> Vec<(usize, NoteEvent)> {
        let extras = band.extras();
        let seed = |player: Player| interpret::seed("band", band.style.name, player.name());
        // The variation the last bar of an ending is played in.
        let held_in = score
            .held
            .map(|bar| variation::variation_in(&score.variations, bar.start));
        let variation_at = |step: usize| variation::variation_in(&score.variations, step);
        let notes: Vec<NoteEvent> = match player {
            Player::Piano => {
                let mut notes =
                    played_in_runs(&score.runs, &score.harmonies, |variation, bars, played| {
                        band.comping_in(variation).play_in_bars(bars, played)
                    });
                notes.extend(held_bar(score.held, &score.harmonies, |bars, played| {
                    Comping::Held.play_in_bars(bars, played)
                }));
                score.breaks.apply(notes, &score.harmonies, |played| {
                    Comping::Held.play_in_bars(&score.bars, std::slice::from_ref(played))
                })
            }
            Player::Bass => {
                let mut notes =
                    played_in_runs(&score.runs, &score.harmonies, |variation, bars, played| {
                        band.bass_in(variation).play_in_bars(bars, played)
                    });
                notes.extend(held_bar(score.held, &score.harmonies, |bars, played| {
                    Bass::Held.play_in_bars(bars, played)
                }));
                score.breaks.apply(notes, &score.harmonies, bass_struck)
            }
            Player::Drums => Vec::new(),
            Player::Guitar => {
                let mut notes =
                    played_in_runs(&score.runs, &score.harmonies, |variation, bars, played| {
                        extras.guitar_in(variation).map_or_else(Vec::new, |guitar| {
                            let comping = band.comping_in(variation);
                            let part = if band.accents_only {
                                GuitarPart::Strum(comping)
                            } else {
                                guitar.part
                            };
                            guitar::play(part, comping, bars, played)
                        })
                    });
                if held_in
                    .and_then(|variation| extras.guitar_in(variation))
                    .is_some()
                {
                    notes.extend(held_bar(score.held, &score.harmonies, |bars, played| {
                        let ring = GuitarPart::Strum(Comping::Held);
                        guitar::play(ring, Comping::Held, bars, played)
                    }));
                }
                score.breaks.apply(notes, &score.harmonies, |played| {
                    match extras.guitar_in(variation_at(played.start)) {
                        Some(_) => guitar::play(
                            GuitarPart::Strum(Comping::Held),
                            Comping::Held,
                            &score.bars,
                            std::slice::from_ref(played),
                        ),
                        None => Vec::new(),
                    }
                })
            }
            Player::Strings | Player::Organ => {
                let octave = match player {
                    Player::Strings => STRINGS_OCTAVE,
                    _ => 0,
                };
                let pad_in = |variation| match player {
                    Player::Strings => extras.strings_in(variation),
                    _ => extras.organ_in(variation),
                };
                let mut notes =
                    played_in_runs(&score.runs, &score.harmonies, |variation, bars, played| {
                        pad_in(variation)
                            .map_or_else(Vec::new, |pad| pad.part.play(octave, bars, played))
                    });
                if held_in.and_then(pad_in).is_some() {
                    notes.extend(held_bar(score.held, &score.harmonies, |bars, played| {
                        PadPart::Held.play(octave, bars, played)
                    }));
                }
                score.breaks.apply(notes, &score.harmonies, |played| {
                    match pad_in(variation_at(played.start)) {
                        Some(_) => {
                            PadPart::Held.play(octave, &score.bars, std::slice::from_ref(played))
                        }
                        None => Vec::new(),
                    }
                })
            }
            // The horns play the chords in close harmony, each voiced
            // from the one before all through the song (see [`horns`]),
            // and hold the last chord and the shots and holds.
            Player::Horns => match band.horns() {
                Some(horns) => {
                    let voiced = horns::voiced(&score.harmonies);
                    let mut notes = played_in_runs(&score.runs, &voiced, |_, bars, played| {
                        horns.play(bars, played)
                    });
                    notes.extend(held_bar(score.held, &voiced, |_, played| {
                        horns::held(played)
                    }));
                    score.breaks.apply(notes, &voiced, |played| {
                        horns::held(std::slice::from_ref(played))
                    })
                }
                None => Vec::new(),
            },
            // The soloist plays over the chorus, not the intro or the
            // ending, and goes on through shots and rests: the gaps
            // are where a soloist is heard.
            Player::Soloist => match band.solo {
                Some(solo) => {
                    let overs: Vec<Over> = score
                        .harmonies
                        .iter()
                        .filter_map(|played| Over::of(played, score.keys.key_at(played.start)))
                        .collect();
                    let (from, to) = self.chorus().unwrap_or((0, score.end));
                    let soloist = Soloist::of(band.instrument_style.genre);
                    solo::improvise(&score.bars, &overs, from, to, soloist, solo)
                }
                None => Vec::new(),
            },
        };
        // A saved style with only drums (a rhythm's copy) leaves the
        // harmony to the band, as a built-in rhythm does, and the
        // soloist and the horns play unless the style has a lane for
        // them.
        let in_style = |style: &crate::library::UserStyle| {
            player == Player::Drums
                || style.lanes.iter().any(|lane| lane.player == player)
                || (!matches!(player, Player::Soloist | Player::Horns)
                    && style.lanes.iter().any(|lane| lane.player != Player::Drums))
        };
        // The drums can come from another style than the rest.
        let saved = match player {
            Player::Drums => band.saved_drums(),
            _ => band.custom_rhythm.as_deref().filter(|_| !band.accents_only),
        };
        if let Some(style) = saved
            && in_style(style)
        {
            let mut notes = Vec::new();
            for lane in style.lanes.iter().filter(|lane| lane.player == player) {
                if let Some(channel) = self.user_style_channel(lane, false) {
                    let mut part =
                        style.play_lane(lane, &score.bars, &score.harmonies, &score.keys);
                    if player != Player::Drums {
                        part = score.breaks.apply(part, &score.harmonies, |played| {
                            if player == Player::Bass {
                                bass_struck(played)
                            } else {
                                Comping::Held
                                    .play_in_bars(&score.bars, std::slice::from_ref(played))
                            }
                        });
                        louder_in_b(&mut part, &score.variations);
                    }
                    notes.extend(part.into_iter().map(|note| (channel, note)));
                }
            }
            if player == Player::Drums {
                let drum_channel = |drum| {
                    self.channels.iter().position(|channel| {
                        channel.kind == ChannelKind::Sampler(SampleSource::Builtin(drum))
                    })
                };
                score.breaks.apply_to_drums(
                    notes,
                    drum_channel(Drum::Kick),
                    drum_channel(Drum::Crash),
                )
            } else {
                notes
            }
        } else {
            match (player, band.channel(player)) {
                (Player::Drums, _) => match band.drums_style() {
                    Some(style) => self.drum_part(style, score),
                    None => Vec::new(),
                },
                (_, Some(channel)) => {
                    let mut notes = notes;
                    let role = match player {
                        Player::Soloist => Role::Melody,
                        _ => Role::Accompaniment,
                    };
                    interpret::perform(&mut notes, score.time, role, seed(player));
                    louder_in_b(&mut notes, &score.variations);
                    notes.into_iter().map(|note| (channel, note)).collect()
                }
                (_, None) => Vec::new(),
            }
        }
    }

    /// The stretches of the song one pattern of each player covers: the
    /// bars cut where a section of `form` (written or heard, see
    /// [`Project::form_sections`]), the chorus, the ending, a variation or
    /// a section style starts or the time signature changes, and wherever
    /// a pattern would grow too long.
    fn chunks(&self, bars: &[Bar], form: &[Section]) -> Vec<Chunk> {
        let sections: Vec<usize> = self
            .playlist
            .sections
            .iter()
            .chain(form)
            .flat_map(|section| [section.start, section.end()])
            .collect();
        let mut edges: Vec<usize> = Vec::new();
        if let Some((start, end)) = self.chorus() {
            edges.extend([start, end]);
        }
        edges.extend(self.variations().iter().map(|marker| marker.start));
        edges.extend(self.section_styles().iter().map(|marker| marker.start));
        let mut chunks: Vec<Chunk> = Vec::new();
        for bar in bars {
            let fits = chunks.last().is_some_and(|chunk| {
                chunk.time == bar.time && chunk.length + bar.length <= MAX_STEPS
            });
            let section = sections.contains(&bar.start);
            match chunks.last_mut() {
                Some(chunk) if fits && !section && !edges.contains(&bar.start) => {
                    chunk.length += bar.length;
                }
                _ => chunks.push(Chunk {
                    start: bar.start,
                    length: bar.length,
                    time: bar.time,
                    soft: fits && section && !edges.contains(&bar.start),
                }),
            }
        }
        chunks
    }

    /// The harmonies of `markers`, the last lasting up to step `end`,
    /// with the notes the piano plays them with and what the bass plays
    /// under them.
    ///
    /// Chords are voiced each from the one before, as the band comps
    /// (see [`Comping::voicings`]), over the line their bass notes make
    /// (see [`bass_line`]). A pitch-class set or a micromode is played from
    /// middle C in its normal form, over its first note. A row form is a
    /// line of twelve notes, not a chord: its four groups of three notes,
    /// in the row's order, are played one after another, each over its
    /// first note. No chord plays nothing.
    fn played_harmonies(
        &self,
        markers: &[ChordMarker],
        end: usize,
        comping: Comping,
    ) -> Vec<Played> {
        let keys = self.key_map();
        let spans: Vec<(usize, usize, HarmonyName)> = markers
            .iter()
            .enumerate()
            .map(|(index, marker)| {
                let stop = markers.get(index + 1).map_or(end, |next| next.start);
                (marker.start, stop, marker.harmony)
            })
            .collect();
        let chords: Vec<(Chord, PitchClass, bool)> = spans
            .iter()
            .filter_map(|(_, _, harmony)| match harmony {
                HarmonyName::Chord(symbol) => Some((
                    symbol.chord(),
                    symbol.bass_pitch_class(),
                    symbol.bass.is_some(),
                )),
                _ => None,
            })
            .collect();
        let line = bass_line(
            &chords
                .iter()
                .map(|&(_, bass, slash)| (bass, slash))
                .collect::<Vec<_>>(),
        );
        let voicings = comping.voicings(
            &chords.iter().map(|(chord, ..)| *chord).collect::<Vec<_>>(),
            &line,
        );
        // The note each harmony has in the bass, as the bass plays it, to
        // lead into it. A chord's is the next one on the line.
        let bass_of = |harmony: HarmonyName, start: usize, chord_index: usize| -> Option<Note> {
            match harmony {
                HarmonyName::Chord(_) => line.get(chord_index).copied(),
                HarmonyName::Polychord(_, lower) => {
                    Some(Note::new(36 + lower.bass_pitch_class().semitone()))
                }
                HarmonyName::Set(set) | HarmonyName::Micromode(set) => Some(group_bass(set)),
                HarmonyName::Row(label) => match keys.key_at(start).kind {
                    ScaleKind::Row(row) => Some(Note::new(36 + row.form(label).first().semitone())),
                    _ => None,
                },
                HarmonyName::NoChord => None,
            }
        };
        let mut played = Vec::new();
        let mut chord_index = 0;
        for (index, &(start, stop, harmony)) in spans.iter().enumerate() {
            match harmony {
                HarmonyName::Chord(_) => {
                    let (chord, ..) = chords[chord_index];
                    let note = line[chord_index];
                    let next = spans
                        .get(index + 1)
                        .filter(|(next_start, ..)| *next_start == stop)
                        .and_then(|&(next_start, _, next)| {
                            bass_of(next, next_start, chord_index + 1)
                        })
                        .unwrap_or(note);
                    played.push(Played {
                        start,
                        end: stop,
                        voicing: voicings[chord_index].clone(),
                        bass: BassPart::Chord { chord, note, next },
                    });
                    chord_index += 1;
                }
                HarmonyName::Polychord(upper, lower) => played.push(Played {
                    start,
                    end: stop,
                    voicing: polychord_voicing(upper.chord(), lower.chord()),
                    bass: BassPart::Pedal(Note::new(36 + lower.bass_pitch_class().semitone())),
                }),
                HarmonyName::Set(set) | HarmonyName::Micromode(set) => played.push(Played {
                    start,
                    end: stop,
                    voicing: group_voicing(set.normal_form()),
                    bass: BassPart::Pedal(group_bass(set)),
                }),
                HarmonyName::Row(label) => {
                    let ScaleKind::Row(row) = keys.key_at(start).kind else {
                        continue;
                    };
                    let notes = row.form(label).pitch_classes();
                    let groups = (stop - start).min(4);
                    for (number, three) in notes.chunks(3).take(groups).enumerate() {
                        let from = start + (stop - start) * number / groups;
                        let to = start + (stop - start) * (number + 1) / groups;
                        played.push(Played {
                            start: from,
                            end: to,
                            voicing: group_voicing(three.iter().copied()),
                            bass: BassPart::Pedal(Note::new(36 + three[0].semitone())),
                        });
                    }
                }
                HarmonyName::NoChord => played.push(Played {
                    start,
                    end: stop,
                    voicing: Vec::new(),
                    bass: BassPart::Rest,
                }),
            }
        }
        played
    }

    /// The parts of the song the drums play (see
    /// [`drum_form::form_parts`]): the stretches between the song's
    /// sections, written or heard, the chorus after the band's intro, and
    /// the changes of variation and style in `changes`.
    fn drum_parts(
        &self,
        bars: &[Bar],
        form: &[Section],
        end: usize,
        changes: &[usize],
    ) -> Vec<(std::ops::Range<usize>, PartKind)> {
        let chorus = self.chorus();
        let cuts: Vec<usize> = chorus
            .into_iter()
            .flat_map(|(start, end)| [start, end])
            .chain(changes.iter().copied())
            .collect();
        let intro_end = chorus.map_or(0, |(start, _)| start);
        drum_form::form_parts(bars, form, end, &cuts, intro_end)
    }

    /// The drums: the style's rhythm laid out over each part of the song
    /// (see [`crate::library::drum_form`]): a fill at the end of each
    /// part, a groove of another kind in parts of another kind, and up
    /// to the end of the chorus. A change of variation or style starts
    /// with a crash, and variation b plays some drums on others, louder
    /// (a song with variations of its own plays them, not the contrasting
    /// grooves). With an ending, its held bar is one last hit of the kick
    /// and the crash cymbal, as are the shots, holds and pushes of the
    /// chord sheet, its breaks.
    fn drum_part(&mut self, style: &BandStyle, score: &Score) -> Vec<(usize, NoteEvent)> {
        let Some(rhythm) = style.rhythm else {
            return Vec::new();
        };
        let (bars, end, held, breaks) = (&score.bars, score.end, score.held, &score.breaks);
        let last_hit = held.and_then(|_| Rhythm::named(LAST_HIT));
        // A change of variation: the drums crash into it, and variation b
        // plays some of the rhythm's drums on others (see
        // `BandStyle::drum_in`).
        let markers = self.variations().to_vec();
        let mut changes: Vec<usize> = markers
            .iter()
            .map(|marker| marker.start)
            .chain(self.section_styles().iter().map(|marker| marker.start))
            .filter(|&start| start > 0 && start < end)
            .collect();
        changes.sort_unstable();
        changes.dedup();
        let crash = (!changes.is_empty())
            .then(|| Rhythm::named(LAST_HIT))
            .flatten();
        let in_b = markers
            .iter()
            .any(|marker| marker.variation == Variation::B);
        let used = drum_form::drums_used(rhythm);
        let swapped: Vec<Drum> = used
            .iter()
            .map(|&drum| style.drum_in(drum, Variation::B))
            .filter(|_| in_b)
            .collect();
        let mut drums = Vec::new();
        for drum in used
            .iter()
            .copied()
            .chain(swapped)
            .chain(last_hit.into_iter().flat_map(|hit| hit.drums()))
            .chain(crash.map(|_| Drum::Crash))
            .chain(
                (!breaks.is_empty())
                    .then_some([Drum::Kick, Drum::Crash])
                    .into_iter()
                    .flatten(),
            )
        {
            if let Some(channel) = apply::drum_channel(self, drum)
                && !drums.contains(&(drum, channel))
            {
                drums.push((drum, channel));
            }
        }
        let channel_of = |drum: Drum| {
            drums
                .iter()
                .find(|(known, _)| *known == drum)
                .map(|&(_, channel)| channel)
        };
        let in_held = |at: usize| held.is_some_and(|bar| (bar.start..bar.end()).contains(&at));
        let mut parts = self.drum_parts(bars, &score.form, end, &changes);
        let last = parts.len().saturating_sub(1);
        // An ending's first bars go on in the groove of the last part, up
        // to its held bar.
        let to_end = bars.partition_point(|bar| bar.start < end);
        let held_from = held.map_or(bars.len(), |held| {
            bars.partition_point(|bar| bar.start < held.start)
        });
        if to_end < held_from {
            let groove = parts.last().map_or(0, |(_, kind)| kind.groove);
            parts.push((
                to_end..held_from,
                PartKind {
                    groove,
                    opens: false,
                    lead_in: false,
                },
            ));
        }
        // A chorus played more than once plays its patterns again: its
        // last part goes on into the next time round, not into the end.
        let repeats = self.repeated_chorus().is_some();
        let mut fills = 0;
        let mut lands = false;
        let mut turns = drum_form::Turns::default();
        let mut notes = Vec::new();
        for (index, (range, kind)) in parts.iter().enumerate() {
            let part = DrumPart {
                bars: &bars[range.clone()],
                opens: kind.opens,
                groove: kind.groove,
                contrast: markers.is_empty(),
                then: if index == last && !repeats {
                    Then::Ending
                } else {
                    Then::Part
                },
                lead_in: kind.lead_in,
                crash: lands,
                turn: turns.of(kind),
            };
            lands = drum_form::ends_with_fill(rhythm, &part);
            turns.played(kind, part.bars.len());
            for (drum, note) in drum_form::part_drums(rhythm, &part, &mut fills) {
                if in_held(note.start) {
                    continue;
                }
                let variation = variation::variation_in(&markers, note.start);
                let Some(channel) = channel_of(style.drum_in(drum, variation)) else {
                    continue;
                };
                let velocity = match variation {
                    Variation::A => note.velocity,
                    Variation::B => variation::louder(note.velocity),
                };
                notes.push((channel, NoteEvent { velocity, ..note }));
            }
        }
        // The crash on the first beat of each change.
        if let (Some(crash), Some(channel)) = (crash, channel_of(Drum::Crash)) {
            let meter = self.meter();
            for &start in &changes {
                let bar = meter.bar_at(start);
                for note in crash.notes(Drum::Crash, bar.time.steps_per_bar(), bar.length) {
                    let at = bar.start + note.start;
                    // A style that crashes there already crashes once.
                    if !notes
                        .iter()
                        .any(|(other, played)| *other == channel && played.start == at)
                    {
                        notes.push((channel, NoteEvent { start: at, ..note }));
                    }
                }
            }
        }
        if let (Some(hit), Some(bar)) = (last_hit, held) {
            for &(drum, channel) in &drums {
                let steps = bar.time.steps_per_bar();
                for note in hit.notes(drum, steps, bar.length) {
                    notes.push((
                        channel,
                        NoteEvent {
                            start: bar.start + note.start,
                            ..note
                        },
                    ));
                }
            }
        }
        breaks.apply_to_drums(notes, channel_of(Drum::Kick), channel_of(Drum::Crash))
    }

    /// Puts `player`'s notes (on the song's timeline) into patterns, one
    /// for each chunk, and clips of them on the player's track. Chunks
    /// that play the same notes share a pattern, as a producer would
    /// copy a clip, and the patterns the player already had are written
    /// over, so their numbers stay the same while the sheet is edited.
    /// A note that lasts past the end of its chunk goes on at the start
    /// of the next.
    fn write_part(&mut self, player: Player, chunks: &[Chunk], notes: Vec<(usize, NoteEvent)>) {
        // A note held over the start of a section plays on: the pattern
        // before it goes on into the section, where it can.
        let mut joined: Vec<Chunk> = Vec::new();
        for &chunk in chunks {
            let held_over = || {
                notes.iter().any(|(_, note)| {
                    note.start < chunk.start && note.start + note.length > chunk.start
                })
            };
            match joined.last_mut() {
                Some(last)
                    if chunk.soft && last.length + chunk.length <= MAX_STEPS && held_over() =>
                {
                    last.length += chunk.length;
                }
                _ => joined.push(chunk),
            }
        }
        let mut contents: Vec<Content> = Vec::new();
        let mut placed: Vec<(usize, usize)> = Vec::new();
        for chunk in &joined {
            let mut rows: Vec<(usize, Vec<NoteEvent>)> = Vec::new();
            for &(channel, note) in &notes {
                let (start, end) = (note.start, note.start + note.length);
                if end <= chunk.start || start >= chunk.end() {
                    continue;
                }
                let from = start.max(chunk.start);
                // A note cut at a chunk's edge starts or ends right on it.
                let inside = NoteEvent {
                    start: from - chunk.start,
                    length: end.min(chunk.end()) - from,
                    nudge: if from == start { note.nudge } else { 0 },
                    trim: if end <= chunk.end() { note.trim } else { 0 },
                    ..note
                };
                match rows.iter_mut().find(|(row, _)| *row == channel) {
                    Some((_, row)) => row.push(inside),
                    None => rows.push((channel, vec![inside])),
                }
            }
            if rows.is_empty() {
                continue;
            }
            rows.sort_by_key(|(channel, _)| *channel);
            for (_, row) in &mut rows {
                row.sort_by_key(|note| (note.start, note.pitch));
            }
            let content = Content {
                length: chunk.length,
                time: chunk.time,
                swing: self.section_style_at(chunk.start).map(BandStyle::swing),
                rows,
            };
            let index = match contents.iter().position(|known| *known == content) {
                Some(index) => index,
                None => {
                    contents.push(content);
                    contents.len() - 1
                }
            };
            placed.push((chunk.start, index));
        }

        let old: Vec<usize> = (0..self.patterns.len())
            .filter(|&index| self.patterns[index].player == Some(player))
            .collect();
        let had_track = self
            .playlist
            .clips
            .iter()
            .find(|clip| old.contains(&clip.pattern))
            .map(|clip| clip.track);
        self.playlist
            .clips
            .retain(|clip| !old.contains(&clip.pattern));
        let reused = old.len().min(contents.len());
        for &index in old[reused..].iter().rev() {
            if self.remove_pattern(index).is_none() {
                self.patterns[index].player = None;
            }
        }
        let mut patterns = Vec::new();
        for (number, content) in contents.iter().enumerate() {
            let index = match old.get(number) {
                Some(&index) if number < reused => index,
                _ => match self.add_pattern() {
                    Some(index) => index,
                    None => break,
                },
            };
            let pattern = &mut self.patterns[index];
            pattern.name = format!("{} {}", player.name(), number + 1);
            pattern.player = Some(player);
            pattern.time = Some(content.time);
            pattern.set_length(content.length);
            for channel in 0..pattern.rows() {
                pattern.set_notes(channel, []);
            }
            for (channel, row) in &content.rows {
                pattern.set_notes(*channel, row.iter().copied());
            }
            patterns.push(index);
        }
        if placed.is_empty() {
            return;
        }
        let Some(track) = had_track.or_else(|| self.band_track(player)) else {
            return;
        };
        if let Some(state) = self.playlist.tracks.get_mut(track) {
            state.name = player.name().to_string();
        }
        for (start, content) in placed {
            if let Some(&pattern) = patterns.get(content) {
                self.add_clip(Clip::new(pattern, track, start));
            }
        }
    }

    /// A track for `player`'s clips: an empty one called after it, or an
    /// empty one nobody has named yet, or a new one at the bottom.
    fn band_track(&mut self, player: Player) -> Option<usize> {
        let used = |project: &Project, track: usize| {
            project
                .playlist
                .clips
                .iter()
                .any(|clip| clip.track == track)
        };
        let tracks = &self.playlist.tracks;
        let named = (0..tracks.len())
            .find(|&track| tracks[track].name == player.name() && !used(self, track));
        let blank = || {
            (0..tracks.len())
                .find(|&track| tracks[track] == Track::numbered(track + 1) && !used(self, track))
        };
        named.or_else(blank).or_else(|| self.add_track())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chord_sheet::read_bar;
    use crate::harmony::ScaleKind;
    use crate::library::SONGS;
    use crate::library::demo::song_demo;

    /// Writes `bars`, one text per bar, on the sheet from bar 1.
    pub(super) fn write(project: &mut Project, bars: &[&str]) {
        let meter = project.meter();
        for (number, text) in bars.iter().enumerate() {
            let bar = meter.bar(number);
            let markers = read_bar(text, &bar, project.key_at(bar.start)).unwrap();
            assert!(project.set_chords(bar.start, bar.end(), &markers));
        }
        project.follow_chord_sheet();
    }

    /// The notes `player` plays over the whole song, on the timeline.
    pub(super) fn part(project: &Project, player: Player) -> Vec<(usize, usize, u8)> {
        let mut notes = Vec::new();
        for clip in project.playlist().clips() {
            let pattern = project.pattern(clip.pattern).unwrap();
            if pattern.player != Some(player) {
                continue;
            }
            for (channel, note) in pattern.all_notes() {
                notes.push((clip.start + note.start, channel, note.pitch.midi()));
            }
        }
        notes.sort_unstable();
        notes
    }

    fn pitch_classes_at(project: &Project, player: Player, step: usize) -> Vec<u8> {
        let mut classes: Vec<u8> = part(project, player)
            .into_iter()
            .filter(|&(at, ..)| at == step)
            .map(|(_, _, midi)| midi % 12)
            .collect();
        classes.sort_unstable();
        classes.dedup();
        classes
    }

    #[test]
    fn every_rhythm_is_a_band_style_and_names_are_unique() {
        let styles = band_styles();
        assert_eq!(styles.len(), RHYTHMS.len() + WITHOUT_DRUMS.len());
        for (index, style) in styles.iter().enumerate() {
            assert!(
                styles[..index].iter().all(|other| other.name != style.name),
                "{} twice",
                style.name
            );
        }
        assert!(BandStyle::named(DEFAULT_STYLE).is_some());
        // The genres stay in the library's order.
        assert!(styles.windows(2).all(|pair| pair[0].genre <= pair[1].genre));
        // Every library style has a band style that plays like it.
        for style in crate::library::STYLES {
            assert!(BandStyle::of_style(style).is_some(), "{}", style.name);
        }
    }

    #[test]
    fn a_new_song_is_an_empty_sheet_with_a_band() {
        let project = Project::with_band();
        let band = project.band().unwrap();
        assert_eq!(band.style.name, DEFAULT_STYLE);
        assert!(project.playlist().chord_markers().is_empty());
        assert_eq!(project.song_length(), 0, "nothing plays yet");
        assert_eq!(project.tempo.bpm(), band.style.tempo);
        let names: Vec<&str> = project.channels().iter().map(|c| c.name.as_str()).collect();
        assert_eq!(&names[..2], ["Piano", "Bass"]);
        assert!(names.contains(&"Kick") && names.contains(&"Snare"));
    }

    #[test]
    fn the_band_plays_what_is_typed() {
        let mut project = Project::with_band();
        write(&mut project, &["C", "Am", "Dm7 G7", "C"]);
        assert_eq!(project.song_length(), 64, "four bars");
        // The piano plays C E G on bar 1, A C E on bar 2, D F A C on
        // beat 1 of bar 3 and G B D F on beat 3.
        assert_eq!(pitch_classes_at(&project, Player::Piano, 0), [0, 4, 7]);
        assert_eq!(pitch_classes_at(&project, Player::Piano, 16), [0, 4, 9]);
        assert_eq!(pitch_classes_at(&project, Player::Piano, 32), [0, 2, 5, 9]);
        assert_eq!(pitch_classes_at(&project, Player::Piano, 40), [2, 5, 7, 11]);
        // The bass plays each root on the change.
        assert_eq!(pitch_classes_at(&project, Player::Bass, 0), [0]);
        assert_eq!(pitch_classes_at(&project, Player::Bass, 16), [9]);
        assert_eq!(pitch_classes_at(&project, Player::Bass, 32), [2]);
        assert_eq!(pitch_classes_at(&project, Player::Bass, 40), [7]);
        assert!(!part(&project, Player::Drums).is_empty());
        // Pop beat adds a strummed guitar, and strings only in variation b.
        assert_eq!(pitch_classes_at(&project, Player::Guitar, 16), [0, 4, 9]);
        assert!(part(&project, Player::Strings).is_empty());
        // Each player on a track of its own, called after it.
        let tracks: Vec<&str> = project
            .playlist()
            .tracks()
            .iter()
            .map(|track| track.name.as_str())
            .collect();
        assert_eq!(tracks, ["Piano", "Bass", "Drums", "Guitar"]);
    }

    #[test]
    fn the_band_stops_and_pushes_where_the_sheet_says() {
        let mut project = Project::with_band();
        write(&mut project, &["C", "^F", "G7.", "Am...", "C"]);
        let starts = |player| -> Vec<usize> {
            let mut starts: Vec<usize> = part(&project, player)
                .into_iter()
                .map(|(at, ..)| at)
                .collect();
            starts.dedup();
            starts
        };
        for player in [Player::Piano, Player::Bass, Player::Guitar, Player::Drums] {
            let starts = starts(player);
            // F comes in on the "and" of 4 in bar 1, and nobody plays on
            // the beat it is written on: it is held over.
            assert!(starts.contains(&14), "{player:?} pushes F: {starts:?}");
            assert!(!starts.contains(&16), "{player:?} holds F over: {starts:?}");
            // G7 is one shot on bar 3, and bar 4 rests: nothing else
            // starts until bar 5.
            assert!(starts.contains(&32), "{player:?} plays the shot");
            assert!(
                starts.iter().all(|&at| !(33..64).contains(&at)),
                "{player:?} stops: {starts:?}"
            );
            assert!(starts.contains(&64), "{player:?} comes back");
        }
        // The shot is an eighth note, the whole chord.
        let shot: Vec<(usize, usize, u8)> = part(&project, Player::Piano)
            .into_iter()
            .filter(|&(at, ..)| at == 32)
            .collect();
        assert_eq!(shot.len(), 4, "G B D F");
        assert_eq!(pitch_classes_at(&project, Player::Piano, 14), [0, 5, 9]);
    }

    #[test]
    fn you_can_take_over_a_part() {
        let mut project = Project::with_band();
        assert!(!project.set_you_play(Player::Bass, true), "no part yet");
        write(&mut project, &["C", "F", "G7", "C"]);
        assert!(!project.you_play(Player::Bass));
        assert!(project.set_you_play(Player::Bass, true));
        assert!(project.you_play(Player::Bass));
        // Its clips are left out of the song; the others still play.
        let players: Vec<Option<Player>> = project
            .playback_arrangement()
            .plays
            .iter()
            .map(|play| project.patterns()[play.pattern].player)
            .collect();
        assert!(!players.contains(&Some(Player::Bass)));
        assert!(players.contains(&Some(Player::Piano)));
        // The band goes on writing the part, which is not frozen.
        write(&mut project, &["C", "F", "G7", "Am"]);
        assert_eq!(pitch_classes_at(&project, Player::Bass, 48), [9]);
        assert!(!project.is_frozen(Player::Bass));
        assert!(project.you_play(Player::Bass));
        assert!(project.set_you_play(Player::Bass, false));
        assert!(!project.you_play(Player::Bass));
        assert_eq!(
            project.player_channel(Player::Bass),
            project.channels().iter().position(|c| c.name == "Bass")
        );
        assert_eq!(project.player_channel(Player::Drums), None);
    }

    #[test]
    fn the_band_follows_every_change_of_the_sheet() {
        let mut project = Project::with_band();
        write(&mut project, &["C", "F", "G", "C"]);
        let bar = project.meter().bar(1);
        let markers = read_bar("Fm", &bar, project.scale).unwrap();
        project.set_chords(bar.start, bar.end(), &markers);
        assert!(project.follow_chord_sheet());
        assert_eq!(pitch_classes_at(&project, Player::Piano, 16), [0, 5, 8]);
        assert!(!project.follow_chord_sheet(), "nothing new to play");
        // Into D major: the band plays the sheet as it moved.
        project.change_key(Scale::new(PitchClass::D, ScaleKind::Major));
        assert!(project.follow_chord_sheet());
        assert_eq!(pitch_classes_at(&project, Player::Piano, 0), [2, 6, 9]);
        assert!(Player::ALL.iter().all(|&player| !project.is_frozen(player)));
    }

    #[test]
    fn an_edited_part_stays_until_it_is_played_again() {
        let mut project = Project::with_band();
        write(&mut project, &["C", "F", "G", "C"]);
        // Take a note out of the bass, as the piano roll would.
        let pattern = project
            .patterns()
            .iter()
            .position(|p| p.player == Some(Player::Bass))
            .unwrap();
        let bass = project.player_channel(Player::Bass).unwrap();
        project.pattern_mut(pattern).unwrap().remove_note(bass, 0);
        let edited = part(&project, Player::Bass);
        assert!(project.is_frozen(Player::Bass));
        assert!(!project.is_frozen(Player::Piano));
        // A new chord: the piano follows, the bass stays as it was left.
        let bar = project.meter().bar(1);
        let markers = read_bar("Dm", &bar, project.scale).unwrap();
        project.set_chords(bar.start, bar.end(), &markers);
        assert!(project.follow_chord_sheet());
        assert_eq!(part(&project, Player::Bass), edited);
        assert_eq!(pitch_classes_at(&project, Player::Piano, 16), [2, 5, 9]);
        // Played again, it follows the sheet once more.
        assert!(project.play_again(Player::Bass));
        assert!(!project.is_frozen(Player::Bass));
        assert_eq!(pitch_classes_at(&project, Player::Bass, 16), [2]);
    }

    #[test]
    fn a_frozen_part_stays_frozen_in_the_file() {
        let mut project = Project::with_band();
        write(&mut project, &["C", "G"]);
        let pattern = project
            .patterns()
            .iter()
            .position(|p| p.player == Some(Player::Piano))
            .unwrap();
        let piano = project.player_channel(Player::Piano).unwrap();
        project.pattern_mut(pattern).unwrap().remove_note(piano, 0);
        let text = crate::file::to_text(&project).unwrap();
        let loaded = crate::file::from_text(&text).unwrap();
        assert_eq!(loaded, project);
        assert!(loaded.is_frozen(Player::Piano));
        assert!(!loaded.is_frozen(Player::Bass));
    }

    #[test]
    fn sets_micromodes_and_rows_are_played_by_their_notes() {
        let mut project = Project::with_band();
        write(&mut project, &["3-11 T2", "Menor 3", "N.C."]);
        // D F A over D.
        assert_eq!(pitch_classes_at(&project, Player::Piano, 0), [2, 5, 9]);
        assert_eq!(pitch_classes_at(&project, Player::Bass, 0), [2]);
        // Menor 3 is C C# E.
        assert_eq!(pitch_classes_at(&project, Player::Piano, 16), [0, 1, 4]);
        // No chord: the piano and the bass stop.
        assert!(
            part(&project, Player::Piano)
                .iter()
                .all(|&(at, ..)| at < 32)
        );
        assert!(part(&project, Player::Bass).iter().all(|&(at, ..)| at < 32));

        // A row form plays its four groups of three, one a beat.
        let mut project = Project::with_band();
        let row = crate::harmony::ToneRow::CHROMATIC;
        project.scale = Scale::row(row);
        write(&mut project, &["P0"]);
        assert_eq!(pitch_classes_at(&project, Player::Piano, 0), [0, 1, 2]);
        assert_eq!(pitch_classes_at(&project, Player::Piano, 4), [3, 4, 5]);
        assert_eq!(pitch_classes_at(&project, Player::Bass, 12), [9]);
    }

    #[test]
    fn no_chord_silences_every_style() {
        // Styles that play a chord note by note (arpeggios, Alberti bass,
        // montunos) once divided by the number of notes of N.C.: none.
        for style in band_styles() {
            let mut project = Project::with_band();
            project.set_band_style(Some(style));
            write(&mut project, &["C", "N.C.", "G7", "C"]);
            let bar = project.meter().bar(1);
            for player in [Player::Piano, Player::Bass, Player::Guitar] {
                assert!(
                    part(&project, player)
                        .iter()
                        .all(|&(at, ..)| !(bar.start..bar.end()).contains(&at)),
                    "{} {player:?} plays in a bar of N.C.",
                    style.name
                );
            }
        }
    }

    #[test]
    fn the_slow_blues_plays_four_beats_of_three() {
        // 12/8: four dotted-quarter beats a bar, at steps 0, 6, 12 and
        // 18. The bass is on every beat and nowhere off the eighths, and
        // the piano plays eighths, three to a beat.
        let mut project = Project::with_band();
        project.set_band_style(BandStyle::named("Slow blues"));
        write(&mut project, &["C7", "F7"]);
        assert_eq!(project.time_signature, TimeSignature::new(12, 8));
        let bass: Vec<usize> = part(&project, Player::Bass)
            .iter()
            .map(|&(at, ..)| at)
            .filter(|&at| at < 24)
            .collect();
        for beat in [0, 6, 12, 18] {
            assert!(bass.contains(&beat), "no bass on step {beat}: {bass:?}");
        }
        assert!(bass.iter().all(|at| at % 2 == 0), "{bass:?}");
        let piano: Vec<usize> = part(&project, Player::Piano)
            .iter()
            .map(|&(at, ..)| at)
            .filter(|&at| at < 24)
            .collect();
        for eighth in (0..24).step_by(2) {
            assert!(piano.contains(&eighth), "no chord on step {eighth}");
        }
    }

    #[test]
    fn a_style_in_three_puts_the_song_in_three() {
        let mut project = Project::with_band();
        write(&mut project, &["C", "G7", "C", "C"]);
        project.set_band_style(BandStyle::named("Jazz waltz"));
        assert_eq!(project.time_signature, TimeSignature::new(3, 4));
        assert_eq!(project.song_length(), 48, "four bars of three beats");
        assert_eq!(pitch_classes_at(&project, Player::Bass, 12), [7]);
    }

    #[test]
    fn a_walking_bass_leads_into_each_chord_by_a_semitone() {
        for (style, bar) in [("Swing", 16), ("Jazz waltz", 12)] {
            let mut project = Project::with_band();
            project.set_band_style(BandStyle::named(style));
            write(&mut project, &["C", "Am", "F", "G7", "C"]);
            let bass: Vec<(usize, u8)> = part(&project, Player::Bass)
                .into_iter()
                .map(|(at, _, midi)| (at, midi))
                .collect();
            for change in 1..5 {
                let at = change * bar;
                let before = bass.iter().rev().find(|(start, _)| *start < at).unwrap();
                let root = bass.iter().find(|(start, _)| *start == at).unwrap();
                assert_eq!(
                    before.1.abs_diff(root.1),
                    1,
                    "{style}, bar {}: {bass:?}",
                    change + 1
                );
            }
        }
    }

    #[test]
    fn a_band_in_a_demo_mutes_the_songs_own_accompaniment() {
        let song = SONGS.iter().find(|song| song.name == "Ode to Joy").unwrap();
        let mut project = song_demo(song);
        let before = project.clone();
        project.set_band_style(BandStyle::named("Bossa nova"));
        let band = project.band().unwrap();
        assert!(
            !project.playlist().chord_markers().is_empty(),
            "the chords it heard"
        );
        assert_eq!(band.muted.channels, [KEYS, BASS]);
        assert_eq!(band.muted.tracks, ["Drums"]);
        assert!(!part(&project, Player::Piano).is_empty());
        // The melody is not touched.
        let lead = project
            .channels()
            .iter()
            .position(|c| c.name == apply::LEAD)
            .unwrap();
        assert!(!project.channels()[lead].muted);
        // Stopping the band puts the song back as it was, but for the
        // chords it wrote on the sheet.
        project.stop_band();
        assert_eq!(project.channels(), before.channels());
        assert_eq!(project.patterns(), before.patterns());
        assert_eq!(project.playlist().clips(), before.playlist().clips());
        assert_eq!(project.playlist().tracks(), before.playlist().tracks());
    }

    #[test]
    fn deleting_a_channel_of_the_song_leaves_the_band_following() {
        let song = SONGS.iter().find(|song| song.name == "Ode to Joy").unwrap();
        let mut project = song_demo(song);
        let before = project.clone();
        project.set_band_style(BandStyle::named("Bossa nova"));
        // The song's own keys, before the band's channels: every band
        // channel moves down one.
        let keys = project
            .channels()
            .iter()
            .position(|c| c.name == KEYS)
            .unwrap();
        let since = project.band().unwrap().since;
        assert!(keys < since);
        let piano = project.player_channel(Player::Piano).unwrap();
        project.remove_channel(keys);
        assert_eq!(project.player_channel(Player::Piano), Some(piano - 1));
        assert!(Player::ALL.iter().all(|&player| !project.is_frozen(player)));
        // The band still plays what is typed.
        let bar = project.meter().bar(1);
        let markers = read_bar("Ab", &bar, project.scale).unwrap();
        project.set_chords(bar.start, bar.end(), &markers);
        assert!(project.follow_chord_sheet());
        let in_bar = |player| {
            let mut classes: Vec<u8> = part(&project, player)
                .into_iter()
                .filter(|&(at, ..)| (bar.start..bar.end()).contains(&at))
                .map(|(_, _, midi)| midi % 12)
                .collect();
            classes.sort_unstable();
            classes.dedup();
            classes
        };
        assert_eq!(in_bar(Player::Piano), [0, 3, 8], "Ab C Eb");
        assert!(in_bar(Player::Bass).contains(&8));
        // And takes all its channels with it when it stops.
        project.stop_band();
        let mut channels = before.channels().to_vec();
        channels.remove(keys);
        assert_eq!(project.channels(), channels);
    }

    #[test]
    fn deleting_a_players_channel_freezes_only_that_part() {
        let mut project = Project::with_band();
        write(&mut project, &["C", "F", "G", "C"]);
        let piano = project.player_channel(Player::Piano).unwrap();
        project.remove_channel(piano);
        assert!(project.is_frozen(Player::Piano));
        assert!(!project.is_frozen(Player::Bass));
        assert!(!project.is_frozen(Player::Drums));
        assert!(!project.is_frozen(Player::Guitar));
    }

    #[test]
    fn a_progression_is_written_bar_by_bar_in_the_key() {
        let mut project = Project::with_band();
        project.scale = Scale::new(PitchClass::A, ScaleKind::Major);
        let style = crate::library::Style::named("Andalusian cadence").unwrap();
        let bars = project.write_progression(style, 0);
        assert_eq!(bars, style.bars());
        let names: Vec<String> = project
            .playlist()
            .chord_markers()
            .iter()
            .map(|marker| marker.harmony.to_string())
            .collect();
        // A minor progression on A: the key becomes A minor.
        assert_eq!(project.scale.kind, style.mode);
        assert_eq!(names.first().map(String::as_str), Some("Am"));
        // Held chords are not written again.
        let mut project = Project::with_band();
        let four = crate::library::Style::named("Four chords").unwrap();
        project.write_progression(four, 0);
        project.write_progression(four, 0);
        let names: Vec<String> = project
            .playlist()
            .chord_markers()
            .iter()
            .map(|marker| marker.harmony.to_string())
            .collect();
        assert_eq!(names, ["C", "G", "Am", "F"]);
    }

    #[test]
    fn the_song_ends_where_it_is_told() {
        let mut project = Project::with_band();
        write(&mut project, &["C", "F", "G", "C"]);
        assert!(project.end_song_at(1));
        assert_eq!(project.song_length(), 32);
        assert_eq!(project.playlist().chord_markers().len(), 2);
        // Typing in a bar after the end plays to it, even a bar that
        // only goes on with the chord before.
        let bar = project.meter().bar(5);
        project.set_chords(bar.start, bar.end(), &[]);
        project.follow_chord_sheet();
        assert_eq!(project.song_length(), 96);
    }

    #[test]
    fn long_songs_are_cut_into_patterns_and_repeats_share_one() {
        let mut project = Project::with_band();
        let bars: Vec<&str> = ["C", "Am", "F", "G"]
            .iter()
            .copied()
            .cycle()
            .take(40)
            .collect();
        write(&mut project, &bars);
        for pattern in project.patterns() {
            assert!(pattern.length() <= MAX_STEPS);
        }
        let drums = project
            .patterns()
            .iter()
            .filter(|p| p.player == Some(Player::Drums))
            .count();
        assert!(drums <= 3, "{drums} drum patterns");
        assert_eq!(project.song_length(), 40 * 16);
    }

    /// A band in `style` playing `bars`.
    pub(super) fn band(style: &str, bars: &[&str]) -> Project {
        let mut project = Project::with_band();
        project.set_band_style(BandStyle::named(style));
        write(&mut project, bars);
        project
    }

    fn channel_names(project: &Project) -> Vec<&str> {
        project
            .channels()
            .iter()
            .map(|channel| channel.name.as_str())
            .collect()
    }

    #[test]
    fn styles_bring_their_guitar_strings_and_organ() {
        let pop = band("Pop ballad", &["C", "Am", "F", "G7"]);
        assert_eq!(
            pop.band().unwrap().style.players(),
            [
                Player::Piano,
                Player::Bass,
                Player::Drums,
                Player::Guitar,
                Player::Strings
            ]
        );
        // Each on a channel of its own, in its General MIDI instrument:
        // a steel-string guitar and slow strings.
        let program = |player| {
            let channel = pop.player_channel(player).unwrap();
            pop.channels()[channel].program
        };
        assert_eq!(program(Player::Guitar), Some(25));
        assert_eq!(program(Player::Strings), Some(49));
        assert!(pop.player_channel(Player::Organ).is_none());
        // The strings hold each chord an octave above the piano.
        let piano = part(&pop, Player::Piano);
        let strings = part(&pop, Player::Strings);
        let at = |notes: &[(usize, usize, u8)], step| -> Vec<u8> {
            notes
                .iter()
                .filter(|&&(at, ..)| at == step)
                .map(|&(_, _, midi)| midi)
                .collect()
        };
        let raised: Vec<u8> = at(&piano, 16).iter().map(|midi| midi + 12).collect();
        assert_eq!(at(&strings, 16), raised);
        assert_eq!(strings.iter().filter(|&&(at, ..)| at == 20).count(), 0);
        // The guitar picks the chord one string at a time.
        assert!(
            part(&pop, Player::Guitar)
                .iter()
                .filter(|&&(at, ..)| at == 16)
                .count()
                == 1
        );
    }

    #[test]
    fn the_reggae_organ_bubbles() {
        let reggae = band("One drop", &["C"]);
        let organ = part(&reggae, Player::Organ);
        let starts: Vec<usize> = {
            let mut starts: Vec<usize> = organ.iter().map(|&(at, ..)| at).collect();
            starts.dedup();
            starts
        };
        // Between every beat, and on beats 2 and 4.
        assert_eq!(starts, [2, 4, 6, 10, 12, 14]);
        let lowest_at = |step| {
            organ
                .iter()
                .filter(|&&(at, ..)| at == step)
                .map(|&(_, _, midi)| midi)
                .min()
                .unwrap()
        };
        assert_eq!(lowest_at(4), lowest_at(2) + 12, "an octave higher");
        // The guitar chops on 2 and 4.
        let mut guitar: Vec<usize> = part(&reggae, Player::Guitar)
            .iter()
            .map(|&(at, ..)| at)
            .collect();
        guitar.dedup();
        assert_eq!(guitar, [4, 12]);
    }

    #[test]
    fn players_for_the_chorus_wait_for_variation_b() {
        let mut pop = band("Pop beat", &["C", "Am", "F", "G7"]);
        assert!(part(&pop, Player::Strings).is_empty());
        pop.set_variation(32, None, Variation::B);
        let strings = part(&pop, Player::Strings);
        assert!(!strings.is_empty());
        assert!(strings.iter().all(|&(at, ..)| at >= 32));
    }

    #[test]
    fn a_style_without_a_guitar_lets_it_go_unless_you_edited_it() {
        let mut project = band("Pop beat", &["C", "G"]);
        assert!(channel_names(&project).contains(&"Guitar"));
        project.set_band_style(BandStyle::named("Boom bap"));
        assert!(part(&project, Player::Guitar).is_empty());
        assert!(!channel_names(&project).contains(&"Guitar"));
        assert!(
            project
                .playlist()
                .tracks()
                .iter()
                .all(|track| track.name != "Guitar")
        );
        // The other parts were not edited by the change.
        assert!(Player::ALL.iter().all(|&player| !project.is_frozen(player)));

        // An edited guitar part stays, with its channel.
        let mut project = band("Pop beat", &["C", "G"]);
        let guitar = project.player_channel(Player::Guitar).unwrap();
        let pattern = project
            .patterns()
            .iter()
            .position(|pattern| pattern.player == Some(Player::Guitar))
            .unwrap();
        project.pattern_mut(pattern).unwrap().remove_note(guitar, 0);
        assert!(project.is_frozen(Player::Guitar));
        project.set_band_style(BandStyle::named("Boom bap"));
        assert!(!part(&project, Player::Guitar).is_empty());
        assert!(channel_names(&project).contains(&"Guitar"));
    }

    #[test]
    fn a_soloist_improvises_when_asked() {
        let mut project = band("Swing", &["Dm7", "G7", "Cmaj7", "A7"]);
        assert!(part(&project, Player::Soloist).is_empty());
        assert!(!channel_names(&project).contains(&"Soloist"));
        assert!(project.set_soloist(true));
        let solo = part(&project, Player::Soloist);
        assert!(solo.len() > 8, "{solo:?}");
        let channel = project.player_channel(Player::Soloist).unwrap();
        assert_eq!(
            project.channels()[channel].program,
            Some(crate::general_midi::TENOR_SAX)
        );
        assert!(project.player_track(Player::Soloist).is_some());
        // On every beat, a note of the chord of its bar.
        let chords = ["Dm7", "G7", "Cmaj7", "A7"].map(|name| {
            crate::harmony::ChordSymbol::parse(name)
                .unwrap()
                .chord()
                .pitch_set()
        });
        for &(at, _, midi) in &solo {
            if at % 4 == 0 {
                let pitch_class = PitchClass::from_semitone(midi % 12);
                assert!(chords[at / 16].contains(pitch_class), "{midi} at {at}");
            }
        }
        // Another solo, also after an edit.
        let pattern = project
            .patterns()
            .iter()
            .position(|pattern| pattern.player == Some(Player::Soloist))
            .unwrap();
        project
            .pattern_mut(pattern)
            .unwrap()
            .remove_note(channel, 0);
        assert!(project.is_frozen(Player::Soloist));
        assert!(project.another_solo());
        assert!(!project.is_frozen(Player::Soloist));
        assert_ne!(part(&project, Player::Soloist), solo);
        // And it goes, with its channel and track.
        assert!(project.set_soloist(false));
        assert!(part(&project, Player::Soloist).is_empty());
        assert!(!channel_names(&project).contains(&"Soloist"));
        assert!(project.player_track(Player::Soloist).is_none());
    }

    #[test]
    fn the_soloist_plays_the_chorus_only() {
        let mut project = band("Swing", &["C6", "A7", "Dm7", "G7"]);
        assert!(project.set_intro(2));
        assert!(project.set_ending(true));
        assert!(project.set_soloist(true));
        let (start, end) = project.chorus().unwrap();
        let solo = part(&project, Player::Soloist);
        assert!(!solo.is_empty());
        assert!(
            solo.iter().all(|&(at, ..)| (start..end).contains(&at)),
            "{solo:?}"
        );
    }

    #[test]
    fn over_a_set_the_soloist_plays_its_notes_on_the_beats() {
        let mut project = Project::with_band();
        write(&mut project, &["3-11 T2", "Menor 3", "N.C.", "3-11 T2"]);
        assert!(project.set_soloist(true));
        let solo = part(&project, Player::Soloist);
        assert!(!solo.is_empty());
        let sets = [vec![2, 5, 9], vec![0, 1, 4]];
        for &(at, _, midi) in &solo {
            assert!(!(32..48).contains(&at), "no solo without a chord: {at}");
            if at % 4 == 0 && at < 32 {
                assert!(sets[at / 16].contains(&(midi % 12)), "{midi} at {at}");
            }
        }
        // There is no scale to name over a set, but its notes are
        // still the solo's chord tones.
        let pattern = project
            .patterns()
            .iter()
            .position(|pattern| pattern.player == Some(Player::Soloist))
            .unwrap();
        let channel = project.player_channel(Player::Soloist).unwrap();
        let notes = project.pattern(pattern).unwrap().notes(channel);
        let roles = project.solo_roles(pattern).unwrap();
        assert_eq!(roles.len(), notes.len());
        for (note, role) in notes.iter().zip(&roles) {
            let (role, over) = role.expect("every note is over a set");
            assert!(over.chord.is_none());
            if note.start % 4 == 0 {
                assert_eq!(role, NoteRole::ChordTone(None), "at {}", note.start);
            }
        }
    }

    #[test]
    fn horns_come_to_any_style_when_asked() {
        let chords = ["C", "Ab", "G7", "C"];
        for (style, program) in [
            ("Motown", general_midi::BRASS_SECTION),
            ("Swing", general_midi::BRASS_SECTION),
            ("Chorale", general_midi::FRENCH_HORN),
            ("Techno", general_midi::SYNTH_BRASS),
            ("Zamba", general_midi::BRASS_SECTION),
            ("Rachenitsa", general_midi::BRASS_SECTION),
        ] {
            let mut project = band(style, &chords);
            assert!(part(&project, Player::Horns).is_empty());
            assert!(!channel_names(&project).contains(&"Horns"));
            assert!(project.set_horns(true));
            let channel = project.player_channel(Player::Horns).unwrap();
            assert_eq!(
                project.channels()[channel].program,
                Some(program),
                "{style}"
            );
            let horns = part(&project, Player::Horns);
            assert!(!horns.is_empty(), "{style}");
            let bar = project.time_signature.steps_per_bar();
            let sets = chords.map(|name| {
                crate::harmony::ChordSymbol::parse(name)
                    .unwrap()
                    .chord()
                    .pitch_set()
            });
            let mut starts: Vec<usize> = horns.iter().map(|&(at, ..)| at).collect();
            starts.dedup();
            for start in starts {
                let notes: Vec<u8> = horns
                    .iter()
                    .filter(|&&(at, ..)| at == start)
                    .map(|&(.., midi)| midi)
                    .collect();
                // Notes of the chord of the bar, in close position under
                // a lead from F4 to F5.
                for &midi in &notes {
                    let class = PitchClass::from_semitone(midi % 12);
                    assert!(
                        sets[start / bar].contains(class),
                        "{style}: {midi} at {start}"
                    );
                }
                let (low, high) = (notes[0], notes[notes.len() - 1]);
                assert!(high - low < 12, "{style}: {notes:?} at {start}");
                assert!((65..=77).contains(&high), "{style}: {notes:?} at {start}");
            }
            assert!(project.set_horns(false));
            assert!(part(&project, Player::Horns).is_empty());
            assert!(!channel_names(&project).contains(&"Horns"));
            assert!(project.player_track(Player::Horns).is_none());
        }
    }

    #[test]
    fn the_horns_play_in_the_style() {
        let starts = |style: &str| {
            let mut project = band(style, &["C", "F", "G7", "C"]);
            assert!(project.set_horns(true));
            let mut starts: Vec<usize> = part(&project, Player::Horns)
                .into_iter()
                .map(|(at, ..)| at)
                .collect();
            starts.dedup();
            starts
        };
        // Long notes in a ballad, one for each chord.
        assert_eq!(starts("Pop ballad"), [0, 16, 32, 48]);
        // The big band's kicks: beat 1 and the "and" of 2.
        assert_eq!(&starts("Swing")[..4], [0, 6, 16, 22]);
        // A march's afterbeats, on 2 and 4.
        assert_eq!(&starts("March")[..4], [2, 6, 10, 14]);
        // A big band's two-bar riff: 1 and the "and" of 2, then 2, its
        // "and" and 4.
        assert_eq!(&starts("Big band swing")[..5], [0, 6, 20, 22, 28]);
    }

    #[test]
    fn a_big_band_brings_its_horns() {
        let chords = ["C7", "F7", "C7", "G7"];
        let horns = |project: &Project| {
            project
                .player_channel(Player::Horns)
                .and_then(|channel| project.channels()[channel].program)
        };
        // Whether the band arrives in the style or changes to it.
        let mut arriving = Project::empty();
        arriving.set_band_style(BandStyle::named("Big band swing"));
        assert!(arriving.band().unwrap().has_horns());
        let mut project = band("Big band swing", &chords);
        assert!(!part(&project, Player::Horns).is_empty());
        assert_eq!(horns(&project), Some(general_midi::BRASS_SECTION));
        // Its saxophones in a ballad.
        project.set_band_style(BandStyle::named("Big band ballad"));
        assert_eq!(horns(&project), Some(general_midi::ALTO_SAX));
        // They leave with the big band's instruments.
        project.set_band_style(BandStyle::named("Swing"));
        assert!(part(&project, Player::Horns).is_empty());
        assert_eq!(horns(&project), None);
        // Horns asked for in the Band menu stay.
        assert!(project.set_horns(true));
        project.set_band_style(BandStyle::named("Pop ballad"));
        assert!(!part(&project, Player::Horns).is_empty());
        // And the big band's can be sent away.
        project.set_band_style(BandStyle::named("Big band swing"));
        assert!(project.set_horns(false));
        assert!(part(&project, Player::Horns).is_empty());
        // Its drums alone, without its instruments, bring no horns.
        let mut project = band("Swing", &chords);
        let parts = apply::StyleParts {
            instruments: false,
            ..apply::StyleParts::default()
        };
        project.apply_band_parts(BandStyle::named("Big band swing").unwrap(), None, parts);
        assert!(!project.band().unwrap().has_horns());
        project.apply_band_parts(
            BandStyle::named("Big band swing").unwrap(),
            None,
            apply::StyleParts::default(),
        );
        assert!(project.band().unwrap().has_horns());
    }

    #[test]
    fn horns_are_saved_in_format_twenty_eight() {
        let mut project = band("Funk", &["Em7", "A7", "Em7", "A7"]);
        let without = crate::file::format_for(&project);
        assert!(project.set_horns(true));
        assert_eq!(crate::file::format_for(&project), 28);
        let text = crate::file::to_text(&project).unwrap();
        let loaded = crate::file::from_text(&text).unwrap();
        assert_eq!(loaded, project);
        assert!(loaded.band().unwrap().has_horns());
        assert!(!loaded.is_frozen(Player::Horns));
        assert!(project.set_horns(false));
        assert_eq!(crate::file::format_for(&project), without);
    }

    #[test]
    fn a_soloist_is_saved_in_format_nineteen() {
        let mut project = band("Pop beat", &["C", "Am", "F", "G"]);
        let without = crate::file::format_for(&project);
        assert!(project.set_soloist(true));
        assert_eq!(crate::file::format_for(&project), 19);
        let text = crate::file::to_text(&project).unwrap();
        let loaded = crate::file::from_text(&text).unwrap();
        assert_eq!(loaded, project);
        assert!(loaded.band().unwrap().has_soloist());
        assert!(!loaded.is_frozen(Player::Soloist));
        assert!(project.set_soloist(false));
        assert_eq!(crate::file::format_for(&project), without);
    }

    #[test]
    fn a_bigger_band_is_saved() {
        let mut project = band("Rock ride", &["E", "A", "B7", "E"]);
        assert_eq!(crate::file::format_for(&project), 17);
        let text = crate::file::to_text(&project).unwrap();
        let loaded = crate::file::from_text(&text).unwrap();
        assert_eq!(loaded, project);
        assert!(Player::ALL.iter().all(|&player| !loaded.is_frozen(player)));
        // A band of three is written as before.
        project.set_band_style(BandStyle::named("Boom bap"));
        let text = crate::file::to_text(&project).unwrap();
        assert!(!text.contains("written_too"), "{text}");
        assert_eq!(crate::file::format_for(&project), 13);
    }

    fn no_parts() -> apply::StyleParts {
        apply::StyleParts {
            rhythm: false,
            instruments: false,
            time_signature: false,
            progression: false,
            tempo: false,
            swing: false,
            matching_style: false,
        }
    }

    #[test]
    fn a_rhythm_can_keep_the_instruments_and_chords() {
        let mut project = band("Rock ride", &["C", "Am", "F", "G"]);
        let keys = project.player_channel(Player::Piano).unwrap();
        project.channel_mut(keys).unwrap().program = Some(80);
        let sounds = project.channels().to_vec();
        let chords = project.playlist().chord_markers().to_vec();
        project.apply_band_parts(
            BandStyle::named("Rachenitsa").unwrap(),
            None,
            apply::StyleParts {
                rhythm: true,
                time_signature: true,
                matching_style: false,
                ..no_parts()
            },
        );
        // Its drums alone: the band plays them with its own chord and
        // bass parts.
        assert_eq!(project.band().unwrap().style.name, "Rock ride");
        assert_eq!(project.band().unwrap().drums_name(), Some("Rachenitsa"));
        assert_eq!(project.band().unwrap().instrument_style.name, "Rock ride");
        assert!(!project.band().unwrap().accents_only);
        for (index, channel) in sounds.iter().enumerate() {
            assert_eq!(&project.channels()[index], channel);
        }
        assert_eq!(
            project
                .playlist()
                .chord_markers()
                .iter()
                .map(|m| m.harmony)
                .collect::<Vec<_>>(),
            chords.iter().map(|m| m.harmony).collect::<Vec<_>>()
        );
        project.follow_chord_sheet();
        assert_eq!(project.channels()[keys].program, Some(80));
    }

    #[test]
    fn a_saved_rhythm_copy_leaves_the_harmony_to_the_band() {
        let copy = crate::library::UserStyle::from_rhythm(
            crate::library::Rhythm::named("Bossa nova").unwrap(),
        );
        assert!(copy.chart.is_empty(), "a rhythm brings no chords");
        let mut project = band("Pop beat", &["C", "Am", "F", "G"]);
        assert!(project.set_soloist(true));
        project
            .apply_user_style(&copy, apply::StyleParts::default())
            .unwrap();
        for player in [Player::Drums, Player::Piano, Player::Bass, Player::Soloist] {
            assert!(!part(&project, player).is_empty(), "{player:?} plays");
        }
        // A song with no band yet gets one that plays as usual in the
        // copy's time.
        let waltz =
            crate::library::UserStyle::from_rhythm(crate::library::Rhythm::named("Waltz").unwrap());
        let mut project = Project::with_band();
        project.band = None;
        crate::remeter::remeter(&mut project, waltz.time);
        write(&mut project, &["C", "F", "G", "C"]);
        project
            .apply_user_style(&waltz, apply::StyleParts::default())
            .unwrap();
        assert_eq!(project.band().unwrap().style.time, waltz.time);
        for player in [Player::Piano, Player::Bass] {
            assert!(!part(&project, player).is_empty(), "{player:?} plays");
        }
    }

    #[test]
    fn drums_and_chord_parts_come_on_their_own_from_builtin_and_saved_styles() {
        let style = Style::named("Bossa nova").unwrap();
        let drums_only = apply::StyleParts {
            rhythm: true,
            ..no_parts()
        };
        let chords_only = apply::StyleParts {
            matching_style: true,
            ..no_parts()
        };
        for custom in [false, true] {
            let mut project = band("Rock ride", &["C", "Am", "F", "G"]);
            let keys = project.player_channel(Player::Piano).unwrap();
            project.channel_mut(keys).unwrap().program = Some(80);
            let chords = project.playlist.chord_markers.clone();
            let sounds = project.channels().to_vec();
            let piano = part(&project, Player::Piano);
            let drums = part(&project, Player::Drums);
            let saved = crate::library::UserStyle::from_builtin(style);
            let apply = |project: &mut Project, parts| {
                if custom {
                    project.apply_user_style(&saved, parts).unwrap();
                } else {
                    let band_style = BandStyle::of_style(style).unwrap();
                    project.apply_band_parts(band_style, Some(style), parts);
                }
            };
            // The style's drums, the band's own chord and bass parts.
            apply(&mut project, drums_only);
            assert!(!project.band().unwrap().accents_only);
            assert_eq!(project.band().unwrap().instrument_style.name, "Rock ride");
            // The new drums may need channels of their own.
            assert_eq!(project.channels()[..sounds.len()], sounds[..]);
            assert_eq!(project.playlist.chord_markers, chords);
            assert_eq!(part(&project, Player::Piano), piano);
            let new_drums = part(&project, Player::Drums);
            assert_ne!(new_drums, drums);
            let text = crate::file::to_text(&project).unwrap();
            let mut reopened = crate::file::from_text(&text).unwrap();
            assert_eq!(reopened, project);
            reopened.follow_chord_sheet();
            assert_eq!(part(&reopened, Player::Piano), piano);
            assert_eq!(part(&reopened, Player::Drums), new_drums);
            // Then its chord and bass parts: the drums stay.
            apply(&mut project, chords_only);
            assert_ne!(part(&project, Player::Piano), piano);
            assert_eq!(part(&project, Player::Drums), new_drums);
            assert_eq!(project.band().unwrap().drums, BandDrums::Style);
            assert_eq!(
                crate::file::format_for(&project),
                if custom { 22 } else { 20 }
            );
        }
    }

    #[test]
    fn a_styles_chord_parts_can_come_without_its_drums() {
        let mut project = band("Rock ride", &["C", "Am", "F", "G"]);
        let drums = part(&project, Player::Drums);
        let piano = part(&project, Player::Piano);
        project.apply_band_parts(
            BandStyle::named("Bossa nova").unwrap(),
            None,
            apply::StyleParts {
                matching_style: true,
                ..no_parts()
            },
        );
        let band = project.band().unwrap();
        assert_eq!(band.style.name, "Bossa nova");
        assert_eq!(band.drums_name(), Some("Rock ride"));
        assert_eq!(part(&project, Player::Drums), drums);
        assert_ne!(part(&project, Player::Piano), piano);
        assert_eq!(crate::file::format_for(&project), 29);
        let text = crate::file::to_text(&project).unwrap();
        assert_eq!(crate::file::from_text(&text).unwrap(), project);
        // The drums of the style the chord parts come from: as one again.
        project.apply_band_parts(
            BandStyle::named("Bossa nova").unwrap(),
            None,
            apply::StyleParts {
                rhythm: true,
                ..no_parts()
            },
        );
        assert_eq!(project.band().unwrap().drums, BandDrums::Style);
    }

    #[test]
    fn instruments_can_change_without_changing_the_performance() {
        let mut project = band("Boom bap", &["C", "Am", "F", "G"]);
        let piano = part(&project, Player::Piano);
        let bass = part(&project, Player::Bass);
        let drums = part(&project, Player::Drums);
        let time = project.meter();
        let tempo = project.tempo;
        project.apply_band_parts(
            BandStyle::named("Pop beat").unwrap(),
            None,
            apply::StyleParts {
                instruments: true,
                ..no_parts()
            },
        );
        assert_eq!(project.band().unwrap().style.name, "Boom bap");
        assert_eq!(project.band().unwrap().instrument_style.name, "Pop beat");
        assert_eq!(part(&project, Player::Piano), piano);
        assert_eq!(part(&project, Player::Bass), bass);
        assert_eq!(part(&project, Player::Drums), drums);
        assert_eq!(project.meter(), time);
        assert_eq!(project.tempo, tempo);
    }

    #[test]
    fn the_progression_is_independent_of_the_rhythm_and_sounds() {
        let mut project = band("Boom bap", &["C", "C", "C", "C"]);
        let rhythm = project.band().unwrap().style;
        let channels = project.channels().to_vec();
        let drums = part(&project, Player::Drums);
        project.apply_band_parts(
            BandStyle::named("Pop beat").unwrap(),
            Style::named("Four chords"),
            apply::StyleParts {
                progression: true,
                ..no_parts()
            },
        );
        assert_eq!(project.band().unwrap().style, rhythm);
        assert_eq!(project.channels(), channels);
        assert_eq!(part(&project, Player::Drums), drums);
        assert_eq!(project.playlist().chord_markers().len(), 4);
        assert_eq!(
            project.chord_at(16).unwrap().harmony.caption(project.scale),
            Some("V".to_string())
        );
    }

    #[test]
    fn separate_style_components_survive_saving_and_undo() {
        let mut project = band("Rock ride", &["C", "Am", "F", "G"]);
        let before = project.clone();
        let mut history = crate::History::default();
        project.apply_band_parts(
            BandStyle::named("Rachenitsa").unwrap(),
            None,
            apply::StyleParts {
                rhythm: true,
                matching_style: false,
                ..no_parts()
            },
        );
        history.record("change rhythm", before.clone());
        let changed = project.clone();
        let text = crate::file::to_text(&project).unwrap();
        assert_eq!(crate::file::format_for(&project), 29);
        let mut loaded = crate::file::from_text(&text).unwrap();
        assert_eq!(loaded, changed);
        assert!(!loaded.follow_chord_sheet());
        history.undo(&mut project).unwrap();
        assert_eq!(project, before);
        history.redo(&mut project).unwrap();
        assert_eq!(project, changed);
    }

    #[test]
    fn starting_a_band_from_a_preset_respects_unchecked_timing_and_progression() {
        let mut project = Project::empty();
        project.time_signature = TimeSignature::new(3, 4);
        project.tempo = Tempo::new(73.0);
        project.swing = 17;
        let lead = apply::synth_named(&mut project, apply::LEAD).unwrap();
        let melody = vec![NoteEvent::new(Note::new(63), 0, 12)];
        project.pattern_mut(0).unwrap().set_length(48);
        project
            .pattern_mut(0)
            .unwrap()
            .set_notes(lead, melody.clone());
        project.add_clip(Clip::new(0, 0, 0));
        project.set_chords(
            0,
            48,
            &[
                ChordMarker::new(0, HarmonyName::parse("Cm").unwrap()),
                ChordMarker::new(24, HarmonyName::parse("G7").unwrap()),
            ],
        );
        let before = project.clone();
        let style = BandStyle::named("Rock ride").unwrap();
        project.apply_band_parts(
            style,
            None,
            apply::StyleParts {
                rhythm: true,
                instruments: true,
                matching_style: true,
                ..no_parts()
            },
        );
        assert_eq!(project.meter(), before.meter());
        assert_eq!(project.tempo, before.tempo);
        assert_eq!(project.swing, before.swing);
        assert_eq!(
            project.playlist.chord_markers,
            before.playlist.chord_markers
        );
        assert_eq!(project.pattern(0).unwrap().notes(lead), melody);
        for player in style
            .players()
            .into_iter()
            .filter(|player| *player != Player::Drums)
        {
            assert!(project.player_channel(player).is_some(), "{player:?}");
            assert!(!part(&project, player).is_empty(), "{player:?}");
        }
    }

    #[test]
    fn every_builtin_preset_can_bring_in_its_full_instrument_lineup() {
        for harmony in crate::library::STYLES {
            let mut project = Project::empty();
            project.set_chords(
                0,
                16,
                &[ChordMarker::new(0, HarmonyName::parse("Cm").unwrap())],
            );
            let style = BandStyle::of_style(harmony).unwrap();
            project.apply_band_parts(
                style,
                Some(harmony),
                apply::StyleParts {
                    rhythm: true,
                    instruments: true,
                    matching_style: true,
                    ..no_parts()
                },
            );
            let band = project.band().unwrap();
            assert_eq!(band.instrument_style, style, "{}", harmony.name);
            for player in style
                .players()
                .into_iter()
                .filter(|player| *player != Player::Drums)
            {
                assert!(
                    project.player_channel(player).is_some(),
                    "{}: {player:?}",
                    harmony.name
                );
            }
        }
    }

    #[test]
    fn a_band_style_names_the_instruments_it_brings() {
        for harmony in crate::library::STYLES {
            let mut project = Project::empty();
            project.set_chords(
                0,
                16,
                &[ChordMarker::new(0, HarmonyName::parse("Cm").unwrap())],
            );
            let style = BandStyle::of_style(harmony).unwrap();
            project.apply_band_parts(
                style,
                Some(harmony),
                apply::StyleParts {
                    rhythm: true,
                    instruments: true,
                    matching_style: true,
                    ..no_parts()
                },
            );
            let brought: Vec<u8> = style
                .players()
                .into_iter()
                .filter_map(|player| project.player_channel(player))
                .filter_map(|channel| project.channels()[channel].program)
                .collect();
            assert_eq!(brought, style.programs(), "{}", harmony.name);
        }
        let tango = BandStyle::of_style(Style::named("Tango").unwrap()).unwrap();
        assert!(tango.programs().contains(&32), "a double bass");
    }

    /// Where each beat a bar is felt in starts, bar by bar up to `end`,
    /// and whether it is the bar's first.
    fn felt_beats(project: &Project, end: usize) -> Vec<(usize, bool)> {
        let mut beats = Vec::new();
        for bar in project.meter().bars().take_while(|bar| bar.start < end) {
            let mut at = bar.start;
            for (index, group) in bar.time.felt_groups().into_iter().enumerate() {
                beats.push((at, index == 0));
                at += usize::from(group) * bar.time.unit_steps();
            }
        }
        beats
    }

    #[test]
    fn an_orchestra_plays_every_bar_of_the_rite_of_spring_in_its_own_meter() {
        let name = "The Rite of Spring, Ritual of Abduction";
        let song = SONGS.iter().find(|song| song.name == name).unwrap();
        let mut project = song_demo(song);
        project.write_heard_chords();
        assert!(
            project.playlist().time_markers().len() > 20,
            "a new meter every bar or two"
        );
        let before = project.clone();
        let harmony = Style::named(song.style).unwrap();
        let style = BandStyle::of_style(harmony).unwrap();
        assert!(style.follows_the_song());
        project.apply_band_parts(style, Some(harmony), apply::StyleParts::default());
        // The song keeps its meters, its tempo and its own parts, which
        // the orchestra plays along with.
        assert_eq!(project.meter(), before.meter());
        assert_eq!(project.tempo, before.tempo);
        assert_eq!(
            project.playlist().tempo_markers(),
            before.playlist().tempo_markers()
        );
        for (channel, was) in before.channels().iter().enumerate() {
            assert_eq!(project.channels()[channel].muted, was.muted, "{}", was.name);
        }
        assert!(project.band().unwrap().muted.channels.is_empty());
        // The horns strike the chords, the timpani play their roots and
        // the strings hold them.
        let program = |player| project.channels()[project.player_channel(player).unwrap()].program;
        assert_eq!(program(Player::Piano), Some(general_midi::FRENCH_HORN));
        assert_eq!(program(Player::Bass), Some(general_midi::TIMPANI));
        assert_eq!(
            program(Player::Strings),
            Some(general_midi::STRING_ENSEMBLE)
        );
        // The bass drum strikes with the score's accents, and on the first
        // beat of loud bars, and nothing else: not every bar.
        let end = project.band().unwrap().end();
        let beats = felt_beats(&project, end);
        let bars: Vec<Bar> = project
            .meter()
            .bars()
            .take_while(|bar| bar.start < end)
            .collect();
        let heard = following::Following::hear(&before, &bars).unwrap();
        let accents: Vec<usize> = heard.accents().iter().map(|accent| accent.at).collect();
        assert!(!accents.is_empty());
        let mut drums = Vec::new();
        for clip in project.playlist().clips() {
            let pattern = project.pattern(clip.pattern).unwrap();
            if pattern.player == Some(Player::Drums) {
                drums.extend(pattern.all_notes().map(|(_, note)| clip.start + note.start));
            }
        }
        drums.sort_unstable();
        drums.dedup();
        assert!(!drums.is_empty());
        assert!(drums.len() < bars.len(), "a bass drum on every bar");
        for at in &drums {
            assert!(
                accents.contains(at) || beats.contains(&(*at, true)),
                "bass drum at step {at}"
            );
        }
        // The horns and the timpani play on the felt beats, where a chord
        // changes between two of them, or with the score's accents.
        let changes: Vec<usize> = project
            .playlist()
            .chord_markers()
            .iter()
            .map(|marker| marker.start)
            .collect();
        for player in [Player::Piano, Player::Bass] {
            let notes = part(&project, player);
            assert!(!notes.is_empty(), "{player:?}");
            for (at, ..) in notes {
                assert!(
                    beats.iter().any(|&(beat, _)| beat == at)
                        || changes.contains(&at)
                        || accents.contains(&at),
                    "{player:?} at step {at}, bar {}",
                    project.meter().bar_at(at).number + 1
                );
            }
        }
    }

    /// The notes of the song's own patterns, the first `count`, by
    /// channel.
    fn own_notes(project: &Project, count: usize) -> Vec<Vec<(usize, NoteEvent)>> {
        project.patterns()[..count]
            .iter()
            .map(|pattern| {
                pattern
                    .all_notes()
                    .map(|(channel, note)| (channel, *note))
                    .collect()
            })
            .collect()
    }

    /// Where `player` starts notes, and how hard, from step `from` to `to`.
    #[test]
    fn the_band_starts_its_patterns_where_the_parts_start() {
        // Candombe para Gardel's parts are 12, 8, 8, 4, 7 and 12 bars
        // long: the band's patterns start with them, not every 16 bars.
        let song = SONGS
            .iter()
            .find(|song| song.name == "Candombe para Gardel")
            .unwrap();
        let mut project = song_demo(song);
        let starts: Vec<usize> = project
            .playlist()
            .sections()
            .iter()
            .map(|section| section.start)
            .collect();
        let song_starts: Vec<usize> = project
            .playlist()
            .clips()
            .iter()
            .filter(|clip| clip.track == 0)
            .map(|clip| clip.start)
            .collect();
        assert_eq!(starts, song_starts);
        let style = Style::named("Candombe").unwrap();
        let band = BandStyle::of_style(style).unwrap();
        project.apply_band_parts(band, Some(style), apply::StyleParts::default());
        for player in [Player::Piano, Player::Bass, Player::Drums] {
            let mut clips: Vec<usize> = project
                .playlist()
                .clips()
                .iter()
                .filter(|clip| project.pattern(clip.pattern).unwrap().player == Some(player))
                .map(|clip| clip.start)
                .collect();
            clips.sort_unstable();
            assert_eq!(clips, starts, "{player:?}");
        }
    }

    #[test]
    fn a_chord_held_over_a_section_start_is_not_struck_again() {
        let mut project = band("Pop ballad", &["C", "Am", "", "", "G", "C"]);
        // F from bar 3 to the end of bar 4, and a section from bar 4.
        let meter = project.meter();
        let (three, four) = (meter.bar(2), meter.bar(3));
        let f = ChordMarker::new(three.start, HarmonyName::parse("F").unwrap());
        assert!(project.set_chords(three.start, four.end(), &[f]));
        assert!(
            project
                .add_section(Section::new("B", four.start, 3 * four.length))
                .is_some()
        );
        project.follow_chord_sheet();
        // The strings hold the chord on: nothing strikes it again where
        // the section starts.
        assert!(!struck(&project, Player::Strings, three.start, four.start).is_empty());
        assert_eq!(
            struck(&project, Player::Strings, four.start, four.start + 1),
            []
        );
    }

    fn struck(project: &Project, player: Player, from: usize, to: usize) -> Vec<(usize, u8)> {
        let mut notes = Vec::new();
        for clip in project.playlist().clips() {
            let pattern = project.pattern(clip.pattern).unwrap();
            if pattern.player != Some(player) {
                continue;
            }
            for (_, note) in pattern.all_notes() {
                let at = clip.start + note.start;
                if (from..to).contains(&at) {
                    notes.push((at, note.velocity));
                }
            }
        }
        notes.sort_unstable();
        notes
    }

    #[test]
    fn the_orchestra_strikes_with_the_accents_of_the_augurs_of_spring() {
        let name = "The Rite of Spring, The Augurs of Spring";
        let song = SONGS.iter().find(|song| song.name == name).unwrap();
        let mut project = song_demo(song);
        project.write_heard_chords();
        let before = project.clone();
        let harmony = Style::named(ORCHESTRA).unwrap();
        let style = BandStyle::of_style(harmony).unwrap();
        // Only the voices: an orchestra comes in with its patterns anyway.
        let parts = apply::StyleParts {
            rhythm: false,
            ..apply::StyleParts::default()
        };
        project.apply_band_parts(style, Some(harmony), parts.following(true));
        assert!(!project.band().unwrap().accents_only);
        assert_eq!(
            project.channels()[..before.channels().len()],
            *before.channels()
        );
        let count = before.patterns().len();
        assert_eq!(own_notes(&project, count), own_notes(&before, count));
        // Bars 1 to 8 of the demo, in 2/4: the strings stamp the same
        // chord on every eighth, and the horns accent it off the beat.
        let bar = |number: usize| project.meter().bars().nth(number - 1).unwrap();
        let eight = bar(8).end();
        let accents = [18, 22, 34, 40, 48, 58];
        let steps = |player| -> Vec<usize> {
            let mut steps: Vec<usize> = struck(&project, player, 0, eight)
                .into_iter()
                .map(|(at, _)| at)
                .collect();
            steps.dedup();
            steps
        };
        for player in [Player::Piano, Player::Bass, Player::Drums] {
            assert_eq!(steps(player), accents, "{player:?}");
        }
        // Bars 9 to 12 are soft: the horns and the bass drum rest, and the
        // strings play quietly.
        let (from, to) = (bar(9).start, bar(12).end());
        for player in [Player::Piano, Player::Drums] {
            assert_eq!(struck(&project, player, from, to), [], "{player:?}");
        }
        let strings = struck(&project, Player::Strings, from, to);
        assert!(!strings.is_empty());
        assert!(
            strings.iter().all(|&(_, velocity)| velocity < 60),
            "{strings:?}"
        );
    }

    #[test]
    fn a_tune_or_a_held_chord_is_not_an_accent() {
        // Carol of the Bells: four quarter and eighth notes over a chord
        // held for four bars, all at the same strength.
        let name = "Shchedryk (Carol of the Bells)";
        let song = SONGS.iter().find(|song| song.name == name).unwrap();
        let mut project = song_demo(song);
        project.write_heard_chords();
        let bars: Vec<Bar> = project
            .meter()
            .bars()
            .take_while(|bar| bar.start < project.song_length())
            .collect();
        let heard = following::Following::hear(&project, &bars).unwrap();
        assert_eq!(heard.accents(), []);
        // So the orchestra plays every bar on its beat, from the start.
        let harmony = Style::named(ORCHESTRA).unwrap();
        let style = BandStyle::of_style(harmony).unwrap();
        project.apply_band_parts(
            style,
            Some(harmony),
            apply::StyleParts::default().following(true),
        );
        let horns: Vec<usize> = struck(&project, Player::Piano, 0, bars[8].start)
            .into_iter()
            .map(|(at, _)| at)
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect();
        assert_eq!(horns, (0..8).map(|bar| bars[bar].start).collect::<Vec<_>>());
    }

    #[test]
    fn with_no_score_to_follow_the_orchestra_plays_as_written() {
        // Chords on the chord sheet and no notes: nothing to listen to.
        let mut project = Project::with_band();
        project.set_chords(
            0,
            32,
            &[ChordMarker::new(0, HarmonyName::parse("C").unwrap())],
        );
        let harmony = Style::named(ORCHESTRA).unwrap();
        let style = BandStyle::of_style(harmony).unwrap();
        project.apply_band_parts(style, Some(harmony), apply::StyleParts::default());
        for player in [Player::Piano, Player::Bass, Player::Drums, Player::Strings] {
            assert!(!part(&project, player).is_empty(), "{player:?}");
        }
    }

    #[test]
    fn a_band_brought_without_drums_leaves_them_to_the_song() {
        let song = SONGS.iter().find(|song| song.name == "El Choclo").unwrap();
        let mut project = song_demo(song);
        let before = project.clone();
        let harmony = Style::named("Emotional pop").unwrap();
        let style = BandStyle::of_style(harmony).unwrap();
        let parts = apply::StyleParts {
            rhythm: false,
            time_signature: false,
            tempo: false,
            ..apply::StyleParts::default()
        };
        project.apply_band_parts(style, Some(harmony), parts);
        let band = project.band().expect("the band comes in");
        assert_eq!(band.drums, BandDrums::Song);
        assert!(!band.accents_only);
        assert!(!part(&project, Player::Piano).is_empty());
        assert!(part(&project, Player::Drums).is_empty());
        assert!(project.band().unwrap().muted().tracks.is_empty());
        for (channel, was) in before.channels().iter().enumerate() {
            let now = &project.channels()[channel];
            assert_eq!((&now.name, now.program), (&was.name, was.program));
        }
        let count = before.patterns().len();
        assert_eq!(own_notes(&project, count), own_notes(&before, count));
        // The drums come later, in place of the song's.
        project.apply_band_parts(
            style,
            Some(harmony),
            apply::StyleParts {
                rhythm: true,
                ..no_parts()
            },
        );
        assert_eq!(project.band().unwrap().drums, BandDrums::Style);
        assert!(!part(&project, Player::Drums).is_empty());
    }

    #[test]
    fn copying_only_the_voices_brings_no_band() {
        let song = SONGS.iter().find(|song| song.name == "El Choclo").unwrap();
        let mut project = song_demo(song);
        let before = project.clone();
        let harmony = Style::named("Emotional pop").unwrap();
        let style = BandStyle::of_style(harmony).unwrap();
        project.apply_band_parts(
            style,
            Some(harmony),
            apply::StyleParts {
                instruments: true,
                ..no_parts()
            },
        );
        assert_eq!(project, before);
    }

    #[test]
    fn picking_the_orchestra_keeps_the_meters_of_the_song() {
        let name = "The Rite of Spring, The Augurs of Spring";
        let song = SONGS.iter().find(|song| song.name == name).unwrap();
        let mut project = song_demo(song);
        project.write_heard_chords();
        let before = project.clone();
        project.set_band_style(BandStyle::named(ORCHESTRA));
        assert_eq!(project.meter(), before.meter());
        assert_eq!(project.time_signature, TimeSignature::new(2, 4));
        assert!(project.band().unwrap().muted.channels.is_empty());
        // Another style puts the song in its own time, as before.
        project.set_band_style(BandStyle::named("Pop beat"));
        assert!(project.playlist().time_markers().is_empty());
        assert_eq!(project.time_signature, TimeSignature::COMMON);
    }
}
