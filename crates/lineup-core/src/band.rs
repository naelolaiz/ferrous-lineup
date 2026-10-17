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

use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::chord_sheet::{ChordMarker, HarmonyName, read_bar};
use crate::harmony::{Chord, PitchSet, Scale, ScaleKind};
use crate::library::apply::{self, BASS, KEYS};
use crate::library::comping::{Bass, BassPart, Comping, Played};
use crate::library::interpret::{self, Role};
use crate::library::voicing::{bass_line, voice_progression};
use crate::library::{Genre, RHYTHMS, Rhythm, Style};
use crate::meter::{Bar, TimeSignature};
use crate::pattern::{MAX_STEPS, NoteEvent};
use crate::pitch::{Note, PitchClass};
use crate::playlist::{Clip, MAX_SONG_STEPS, Track};
use crate::project::{Channel, ChannelKind, Drum, Project, SampleSource};
use crate::tempo::Tempo;

mod extras;
mod form;
mod guitar;
mod stops;
mod variation;

pub use extras::{Extra, Extras, PadPart};
pub use form::{ENDING_BARS, Ending, Form, INTRO_BARS};
pub use guitar::GuitarPart;
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
}

impl Player {
    /// How many there are.
    pub const COUNT: usize = 6;

    pub const ALL: [Player; Player::COUNT] = [
        Player::Piano,
        Player::Bass,
        Player::Drums,
        Player::Guitar,
        Player::Strings,
        Player::Organ,
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

/// The styles with no drums: the ways of playing of the library's
/// classical and ambient styles, which have no rhythm of their own.
const WITHOUT_DRUMS: [BandStyle; 4] = [
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
];

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
            })
            .collect()
    }

    /// How much it swings when it is picked (see [`crate::swing`]): as
    /// much as its drums do, and not at all without drums.
    pub fn swing(&self) -> u8 {
        self.rhythm.map_or(0, |rhythm| rhythm.swing)
    }

    /// The band style called `name`, if there is one.
    pub fn named(name: &str) -> Option<&'static BandStyle> {
        band_styles().iter().find(|style| style.name == name)
    }

    /// The band style that plays the way a library style does: the one
    /// of its rhythm, or for a style without drums, the one without
    /// drums that comps and plays the bass as it does.
    pub fn of_style(style: &Style) -> Option<&'static BandStyle> {
        BandStyle::named(style.rhythm).or_else(|| {
            band_styles().iter().find(|band| {
                band.rhythm.is_none()
                    && band.comping == style.comping
                    && band.bass == style.bass
                    && band.time == style.time
            })
        })
    }

    /// The General MIDI instrument of the piano: an electric piano in
    /// the electronic, hip hop and soul styles, a grand piano elsewhere.
    fn piano_program(&self) -> u8 {
        match self.genre {
            Genre::Electronic | Genre::HipHop | Genre::Soul => 4,
            _ => 0,
        }
    }

    /// The General MIDI instrument of the bass: a double bass in jazz,
    /// tango, folklore and the acoustic styles, a synth bass in
    /// electronic music and hip hop, and a bass guitar elsewhere.
    fn bass_program(&self) -> u8 {
        match self.genre {
            Genre::Jazz
            | Genre::Tango
            | Genre::Folklore
            | Genre::Flamenco
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

/// The band of a song: its style, how far it plays, and what it knows
/// about the parts it wrote.
#[derive(Debug, Clone, PartialEq)]
pub struct Band {
    pub style: &'static BandStyle,
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
}

impl Band {
    fn new(style: &'static BandStyle) -> Self {
        Band {
            style,
            end: 0,
            channels: [None; Player::COUNT],
            written: [None; Player::COUNT],
            heard: None,
            muted: Muted::default(),
            since: 0,
            form: Form::default(),
            variations: Vec::new(),
        }
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
}

/// A band as it is saved in a project file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BandFile {
    pub style: String,
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
    /// The fingerprints of the piano, the bass and the drums.
    pub written: [Option<u64>; 3],
    /// The fingerprints of the guitar, the strings and the organ, left
    /// out when there are none, so a band of three reads as before.
    #[serde(default, skip_serializing_if = "none_written")]
    pub written_too: [Option<u64>; 3],
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
}

fn none_written(written: &[Option<u64>; 3]) -> bool {
    written.iter().all(Option::is_none)
}

impl From<&Band> for BandFile {
    fn from(band: &Band) -> Self {
        let [piano, bass, drums, guitar, strings, organ] = band.written;
        BandFile {
            style: band.style.name.to_string(),
            end: band.end,
            piano: band.channel(Player::Piano),
            bass: band.channel(Player::Bass),
            guitar: band.channel(Player::Guitar),
            strings: band.channel(Player::Strings),
            organ: band.channel(Player::Organ),
            written: [piano, bass, drums],
            written_too: [guitar, strings, organ],
            heard: band.heard,
            muted: band.muted.clone(),
            since: band.since,
            form: band.form,
            variations: band.variations.clone(),
        }
    }
}

impl BandFile {
    /// The band it describes, in a project with `channels` channels.
    pub(crate) fn band(self, channels: usize) -> Result<Band, String> {
        let style = BandStyle::named(&self.style).ok_or_else(|| {
            format!(
                "its band plays \"{}\", a style this version does not know",
                self.style
            )
        })?;
        if self.end > MAX_SONG_STEPS {
            return Err("its band plays past the end of the song".to_string());
        }
        let channel = |index: Option<usize>| match index {
            Some(index) if index >= channels => Err(format!(
                "its band plays on channel {}, which does not exist",
                index + 1
            )),
            index => Ok(index),
        };
        let [piano, bass, drums] = self.written;
        let [guitar, strings, organ] = self.written_too;
        Ok(Band {
            style,
            end: self.end,
            channels: [
                channel(self.piano)?,
                channel(self.bass)?,
                None,
                channel(self.guitar)?,
                channel(self.strings)?,
                channel(self.organ)?,
            ],
            written: [piano, bass, drums, guitar, strings, organ],
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

/// The volumes of the band's channels: the bass under the melody, the
/// piano and the guitar further back, and pads behind them all.
const PIANO_VOLUME: f32 = 0.45;
const BASS_VOLUME: f32 = 0.7;
const GUITAR_VOLUME: f32 = 0.4;
const PAD_VOLUME: f32 = 0.3;

/// How far above the piano's voicing the strings play: an octave.
const STRINGS_OCTAVE: i32 = 12;

/// How often the drums play a fill when nothing else says where: in the
/// last bar of every eight, as most songs are built.
const PHRASE_BARS: usize = 8;

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
}

impl Chunk {
    fn end(&self) -> usize {
        self.start + self.length
    }
}

/// One pattern's worth of a part: how long, in which time signature,
/// and its notes by channel.
#[derive(Debug, Clone, PartialEq)]
struct Content {
    length: usize,
    time: TimeSignature,
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
                *next = note.pitch_class();
            }
            harmony.start = harmony.start.max(start);
            harmony.end = harmony.end.min(end);
            harmony
        })
        .collect()
}

/// A stretch of bars the band plays in one variation.
struct Run {
    bars: Vec<Bar>,
    variation: Variation,
}

/// `bars` in runs of one variation each, as `markers` say, all but the
/// `held` bar, if there is one: the last bar of an ending, played
/// otherwise.
fn runs(bars: &[Bar], held: Option<Bar>, markers: &[VariationMarker]) -> Vec<Run> {
    let mut runs: Vec<Run> = Vec::new();
    let played = bars
        .iter()
        .filter(|bar| held.is_none_or(|held| bar.start < held.start));
    for &bar in played {
        let variation = variation::variation_in(markers, bar.start);
        match runs.last_mut() {
            Some(run) if run.variation == variation => run.bars.push(bar),
            _ => runs.push(Run {
                bars: vec![bar],
                variation,
            }),
        }
    }
    runs
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
fn bass_struck(played: &Played) -> Vec<NoteEvent> {
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
        if self.band.as_ref().is_some_and(|band| band.style == style) {
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
            if !self.playlist.chord_markers.is_empty() {
                band.muted = self.mute_the_songs_own();
            }
            self.band = Some(band);
        } else if let Some(band) = &mut self.band {
            band.style = style;
        }
        let empty = self.playlist.chord_markers.is_empty() && self.song_length() == 0;
        if self.meter().first() != style.time || !self.playlist.time_markers.is_empty() {
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
        self.band_channels();
        self.write_band();
    }

    /// Stops the band: its patterns go, with their clips and the tracks
    /// they leave empty, and the song's own channels and tracks it muted
    /// play again. The chord sheet stays.
    pub fn stop_band(&mut self) {
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
            if self
                .patterns
                .iter()
                .all(|pattern| pattern.notes(channel).is_empty())
            {
                self.remove_channel(channel);
            }
        }
        for name in &band.muted.channels {
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
    /// in place of what was edited. Returns `false` if there is no band.
    pub fn play_again(&mut self, player: Player) -> bool {
        let Some(band) = &mut self.band else {
            return false;
        };
        band.written[player.index()] = None;
        self.band_channels();
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
        self.band_channels();
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

    /// Makes sure the band has its channels: the piano and the bass, the
    /// guitar, strings and organ its style adds (each in the instrument
    /// of its style), and the drums its rhythm plays.
    fn band_channels(&mut self) {
        let Some(band) = self.band.clone() else {
            return;
        };
        let style = band.style;
        let extras = style.extras();
        let mut channels = band.channels;
        for player in Player::ALL {
            let sound = match player {
                Player::Piano => Some((KEYS, PIANO_VOLUME, style.piano_program())),
                Player::Bass => Some((BASS, BASS_VOLUME, style.bass_program())),
                Player::Drums => None,
                Player::Guitar => extras
                    .guitar
                    .map(|guitar| (player.name(), GUITAR_VOLUME, guitar.program)),
                Player::Strings => extras
                    .strings
                    .map(|strings| (player.name(), PAD_VOLUME, strings.program)),
                Player::Organ => extras
                    .organ
                    .map(|organ| (player.name(), PAD_VOLUME, organ.program)),
            };
            if let Some((other, volume, program)) = sound {
                channels[player.index()] =
                    self.band_channel(band.channel(player), player.name(), other, volume, program);
            }
        }
        if let Some(rhythm) = style.rhythm {
            for drum in rhythm.drums() {
                apply::drum_channel(self, drum);
            }
        }
        if let Some(band) = &mut self.band {
            band.channels = channels;
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
        let players = band.style.players();
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
    ) -> Option<usize> {
        let plays_notes = |channel: &Channel| {
            matches!(
                channel.kind,
                ChannelKind::Synth | ChannelKind::MidiOut { .. }
            )
        };
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
                    program: None,
                })?
            }
        };
        if let Some(channel) = self.channels.get_mut(index) {
            channel.program = Some(program);
        }
        Some(index)
    }

    /// Mutes the chords, bass and drums the song plays without the band:
    /// the channels the library writes chords and the bass on, and the
    /// tracks that hold only drums. Returns what it muted.
    fn mute_the_songs_own(&mut self) -> Muted {
        let mut muted = Muted::default();
        let plays = |project: &Project, channel: usize| {
            project
                .patterns
                .iter()
                .any(|pattern| pattern.player.is_none() && !pattern.notes(channel).is_empty())
        };
        for index in 0..self.channels.len() {
            let channel = &self.channels[index];
            let accompanies = channel.kind == ChannelKind::Synth
                && (channel.name == KEYS || channel.name == BASS);
            if accompanies && !channel.muted && plays(self, index) {
                muted.channels.push(channel.name.clone());
                self.channels[index].muted = true;
            }
        }
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
                muted.tracks.push(state.name.clone());
            }
        }
        muted
    }

    /// A fingerprint of what the band's parts are played from.
    fn band_inputs(&self) -> u64 {
        let mut print = Fingerprint::new();
        if let Some(band) = &self.band {
            print.bytes(band.style.name.as_bytes());
            print.number(band.end);
            print.bytes(
                format!("{:?}{:?}{:?}", band.form, self.ending(), band.variations).as_bytes(),
            );
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
        let following: Vec<Player> = Player::ALL
            .into_iter()
            .filter(|&player| {
                self.band
                    .as_ref()
                    .is_some_and(|band| band.written[player.index()].is_some())
                    && !self.is_frozen(player)
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
        let chunks = self.chunks(&bars);
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
        let harmonies = self.played_harmonies(&markers, parts_end);
        let breaks = stops::Breaks::of(&markers, parts_end);
        // The last bar of an ending is one chord, struck and held.
        let held = ending.and_then(|_| bars.last().copied());
        let variations = band.variations.clone();
        let runs = runs(&bars, held, &variations);
        let style = band.style;
        let extras = style.extras();
        let seed = |player: Player| interpret::seed("band", style.name, player.name());
        let time = meter.first();
        // The variation the last bar of an ending is played in.
        let held_in = held.map(|bar| variation::variation_in(&variations, bar.start));
        let variation_at = |step: usize| variation::variation_in(&variations, step);
        let mut rewritten = Vec::new();
        for player in Player::ALL {
            if self.is_frozen(player) {
                continue;
            }
            let notes: Vec<NoteEvent> = match player {
                Player::Piano => {
                    let mut notes = played_in_runs(&runs, &harmonies, |variation, bars, played| {
                        style.comping_in(variation).play_in_bars(bars, played)
                    });
                    notes.extend(held_bar(held, &harmonies, |bars, played| {
                        Comping::Held.play_in_bars(bars, played)
                    }));
                    breaks.apply(notes, &harmonies, |played| {
                        Comping::Held.play_in_bars(&bars, std::slice::from_ref(played))
                    })
                }
                Player::Bass => {
                    let mut notes = played_in_runs(&runs, &harmonies, |variation, bars, played| {
                        style.bass_in(variation).play_in_bars(bars, played)
                    });
                    notes.extend(held_bar(held, &harmonies, |bars, played| {
                        Bass::Held.play_in_bars(bars, played)
                    }));
                    breaks.apply(notes, &harmonies, bass_struck)
                }
                Player::Drums => Vec::new(),
                Player::Guitar => {
                    let mut notes = played_in_runs(&runs, &harmonies, |variation, bars, played| {
                        extras.guitar_in(variation).map_or_else(Vec::new, |guitar| {
                            guitar::play(guitar.part, style.comping_in(variation), bars, played)
                        })
                    });
                    if held_in
                        .and_then(|variation| extras.guitar_in(variation))
                        .is_some()
                    {
                        notes.extend(held_bar(held, &harmonies, |bars, played| {
                            let ring = GuitarPart::Strum(Comping::Held);
                            guitar::play(ring, Comping::Held, bars, played)
                        }));
                    }
                    breaks.apply(notes, &harmonies, |played| {
                        match extras.guitar_in(variation_at(played.start)) {
                            Some(_) => guitar::play(
                                GuitarPart::Strum(Comping::Held),
                                Comping::Held,
                                &bars,
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
                    let mut notes = played_in_runs(&runs, &harmonies, |variation, bars, played| {
                        pad_in(variation)
                            .map_or_else(Vec::new, |pad| pad.part.play(octave, bars, played))
                    });
                    if held_in.and_then(pad_in).is_some() {
                        notes.extend(held_bar(held, &harmonies, |bars, played| {
                            PadPart::Held.play(octave, bars, played)
                        }));
                    }
                    breaks.apply(notes, &harmonies, |played| {
                        match pad_in(variation_at(played.start)) {
                            Some(_) => {
                                PadPart::Held.play(octave, &bars, std::slice::from_ref(played))
                            }
                            None => Vec::new(),
                        }
                    })
                }
            };
            let notes = match (player, band.channel(player)) {
                (Player::Drums, _) => self.drum_part(style, &bars, &chunks, end, held, &breaks),
                (_, Some(channel)) => {
                    let mut notes = notes;
                    interpret::perform(&mut notes, time, Role::Accompaniment, seed(player));
                    louder_in_b(&mut notes, &variations);
                    notes.into_iter().map(|note| (channel, note)).collect()
                }
                (_, None) => Vec::new(),
            };
            self.write_part(player, &chunks, notes);
            rewritten.push(player);
        }
        self.drop_unused_players(&rewritten);
        // Fingerprints last: dropping a channel renumbers the ones after it.
        // A player the style does not have has no part to keep.
        let players = style.players();
        let mut written = band.written;
        for player in rewritten {
            written[player.index()] = players.contains(&player).then(|| self.part_print(player));
        }
        let heard = self.band_inputs();
        if let Some(band) = &mut self.band {
            band.written = written;
            band.heard = Some(heard);
        }
    }

    /// The stretches of the song one pattern of each player covers: the
    /// bars cut where a section, the chorus, the ending or a variation
    /// starts or the time signature changes, and wherever a pattern would
    /// grow too long.
    fn chunks(&self, bars: &[Bar]) -> Vec<Chunk> {
        let mut edges: Vec<usize> = self
            .playlist
            .sections
            .iter()
            .flat_map(|section| [section.start, section.end()])
            .collect();
        if let Some((start, end)) = self.chorus() {
            edges.extend([start, end]);
        }
        edges.extend(self.variations().iter().map(|marker| marker.start));
        let mut chunks: Vec<Chunk> = Vec::new();
        for bar in bars {
            let joins = chunks.last().is_some_and(|chunk| {
                chunk.time == bar.time
                    && chunk.length + bar.length <= MAX_STEPS
                    && !edges.contains(&bar.start)
            });
            match chunks.last_mut() {
                Some(chunk) if joins => chunk.length += bar.length,
                _ => chunks.push(Chunk {
                    start: bar.start,
                    length: bar.length,
                    time: bar.time,
                }),
            }
        }
        chunks
    }

    /// The harmonies of `markers`, the last lasting up to step `end`,
    /// with the notes the piano plays them with and what the bass plays
    /// under them.
    ///
    /// Chords are voiced each from the one before (see
    /// [`voice_progression`]), and their bass notes make a line (see
    /// [`bass_line`]). A pitch-class set or a micromode is played from
    /// middle C in its normal form, over its first note. A row form is a
    /// line of twelve notes, not a chord: its four groups of three notes,
    /// in the row's order, are played one after another, each over its
    /// first note. No chord plays nothing.
    fn played_harmonies(&self, markers: &[ChordMarker], end: usize) -> Vec<Played> {
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
        let voicings =
            voice_progression(&chords.iter().map(|(chord, ..)| *chord).collect::<Vec<_>>());
        let line = bass_line(
            &chords
                .iter()
                .map(|&(_, bass, slash)| (bass, slash))
                .collect::<Vec<_>>(),
        );
        // The note each harmony has in the bass, to lead into it.
        let bass_of = |harmony: HarmonyName, start: usize| -> Option<PitchClass> {
            match harmony {
                HarmonyName::Chord(symbol) => Some(symbol.bass_pitch_class()),
                HarmonyName::Set(set) | HarmonyName::Micromode(set) => {
                    Some(group_bass(set).pitch_class())
                }
                HarmonyName::Row(label) => match keys.key_at(start).kind {
                    ScaleKind::Row(row) => Some(row.form(label).first()),
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
                        .and_then(|&(next_start, _, next)| bass_of(next, next_start))
                        .unwrap_or(note.pitch_class());
                    played.push(Played {
                        start,
                        end: stop,
                        voicing: voicings[chord_index].clone(),
                        bass: BassPart::Chord { chord, note, next },
                    });
                    chord_index += 1;
                }
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

    /// The drums: the style's rhythm in every bar, started again at each
    /// stretch, with its fill in the last bar of every eight, the last
    /// bar before a section or a change of variation, and the last bar of
    /// the intro and of the chorus, which ends at `end`. A change of
    /// variation starts with a crash, and variation b plays some drums on
    /// others, louder. With an ending, its `held` bar is one last hit of
    /// the kick and the crash cymbal, as are the shots, holds and pushes
    /// of the chord sheet, its `breaks`.
    fn drum_part(
        &mut self,
        style: &BandStyle,
        bars: &[Bar],
        chunks: &[Chunk],
        end: usize,
        held: Option<Bar>,
        breaks: &stops::Breaks,
    ) -> Vec<(usize, NoteEvent)> {
        let Some(rhythm) = style.rhythm else {
            return Vec::new();
        };
        let fill = rhythm.fill();
        let last_hit = held.and_then(|_| Rhythm::named(LAST_HIT));
        // A change of variation: the drums crash into it, and variation b
        // plays some of the rhythm's drums on others (see
        // `BandStyle::drum_in`).
        let markers = self.variations().to_vec();
        let changes: Vec<usize> = markers
            .iter()
            .map(|marker| marker.start)
            .filter(|&start| start > 0 && start < end)
            .collect();
        let crash = (!changes.is_empty())
            .then(|| Rhythm::named(LAST_HIT))
            .flatten();
        let in_b = markers
            .iter()
            .any(|marker| marker.variation == Variation::B);
        let swapped = rhythm
            .drums()
            .map(|drum| style.drum_in(drum, Variation::B))
            .filter(|_| in_b);
        let mut drums = Vec::new();
        for drum in rhythm
            .drums()
            .chain(swapped)
            .chain(fill.into_iter().flat_map(|fill| fill.drums()))
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
        // Where a part of the song starts: a section, the chorus after
        // the intro, or a change of variation. Phrases of eight bars are
        // counted from there.
        let mut starts: Vec<usize> = self
            .playlist
            .sections
            .iter()
            .map(|section| section.start)
            .collect();
        starts.extend(self.chorus().map(|(start, _)| start));
        starts.extend(&changes);
        starts.retain(|&start| start > 0);
        let phrase_start = |bar: &Bar| {
            starts
                .iter()
                .filter(|&&start| start <= bar.start)
                .max()
                .map_or(0, |&start| self.meter().bar_at(start).number)
        };
        let fills: Vec<&Bar> = bars
            .iter()
            .filter(|bar| {
                let phrase = bar.number - phrase_start(bar) + 1;
                fill.is_some()
                    && bars.len() >= 4
                    && bar.start < end
                    && (phrase.is_multiple_of(PHRASE_BARS)
                        || starts.contains(&bar.end())
                        || bar.end() >= end)
            })
            .collect();
        let in_a_fill = |at: usize| {
            fills
                .iter()
                .copied()
                .chain(held.as_ref())
                .any(|bar| (bar.start..bar.end()).contains(&at))
        };
        let mut notes = Vec::new();
        for chunk in chunks {
            let bar = chunk.time.steps_per_bar();
            let variation = variation::variation_in(&markers, chunk.start);
            for &(drum, _) in &drums {
                let Some(channel) = channel_of(style.drum_in(drum, variation)) else {
                    continue;
                };
                for note in rhythm.notes(drum, bar, chunk.length) {
                    let at = chunk.start + note.start;
                    if !in_a_fill(at) {
                        let velocity = match variation {
                            Variation::A => note.velocity,
                            Variation::B => variation::louder(note.velocity),
                        };
                        notes.push((
                            channel,
                            NoteEvent {
                                start: at,
                                velocity,
                                ..note
                            },
                        ));
                    }
                }
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
        let fills = fill
            .into_iter()
            .flat_map(|fill| fills.iter().map(move |&bar| (fill, bar)));
        for (played, bar) in fills.chain(last_hit.zip(held.as_ref())) {
            for &(drum, channel) in &drums {
                let steps = bar.time.steps_per_bar();
                for note in played.notes(drum, steps, bar.length) {
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
        let mut contents: Vec<Content> = Vec::new();
        let mut placed: Vec<(usize, usize)> = Vec::new();
        for chunk in chunks {
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
    fn write(project: &mut Project, bars: &[&str]) {
        let meter = project.meter();
        for (number, text) in bars.iter().enumerate() {
            let bar = meter.bar(number);
            let markers = read_bar(text, &bar, project.key_at(bar.start)).unwrap();
            assert!(project.set_chords(bar.start, bar.end(), &markers));
        }
        project.follow_chord_sheet();
    }

    /// The notes `player` plays over the whole song, on the timeline.
    fn part(project: &Project, player: Player) -> Vec<(usize, usize, u8)> {
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
    fn a_style_in_three_puts_the_song_in_three() {
        let mut project = Project::with_band();
        write(&mut project, &["C", "G7", "C", "C"]);
        project.set_band_style(BandStyle::named("Jazz waltz"));
        assert_eq!(project.time_signature, TimeSignature::new(3, 4));
        assert_eq!(project.song_length(), 48, "four bars of three beats");
        assert_eq!(pitch_classes_at(&project, Player::Bass, 12), [7]);
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
    fn band(style: &str, bars: &[&str]) -> Project {
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
}
