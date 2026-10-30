//! Saving projects to files and loading them back.
//!
//! A project file is plain text in RON ("Rusty Object Notation"), a
//! format that looks like Rust's own struct and enum syntax:
//!
//! ```text
//! (
//!     format: 1,
//!     tempo: 130.0,
//!     scale: (root: C, kind: Major),
//!     channels: [
//!         (name: "Kick", kind: Sampler(Builtin(Kick)), volume: 0.8, muted: false),
//!         ...
//! ```
//!
//! The `serde` crate does the work. `#[derive(Serialize, Deserialize)]`
//! writes the code that turns a struct into text and back, field by
//! field, and the `ron` crate decides what the text looks like.
//!
//! The file has its own struct, [`ProjectFile`], instead of saving
//! [`Project`] directly. `Project` has rules to keep (one row of notes
//! per channel, clips that point at real patterns), and a file could
//! break any of them. So loading reads a `ProjectFile`, which accepts
//! anything shaped right, and then builds the `Project` through its
//! normal methods, which check everything, as if a user had clicked it
//! all in.

use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::NoteNaming;
use crate::i18n::{Language, tr, trf};

use crate::band::{Band, BandFile, Player};
use crate::chord_sheet::{ChordMarker, HarmonyName};
use crate::key_map::KeyMarker;
use crate::meter::TimeMarker;
use crate::pattern::{MAX_STEPS, MAX_STEPS_BEFORE_FORMAT_3, NoteEvent};
use crate::playlist::{Clip, LoopRegion, MAX_TRACKS, Section, Track};
use crate::project::{
    Channel, ChannelKind, MAX_CHANNELS, MAX_LOOSE_TIMING, MAX_PATTERNS, Project, SampleSource,
};
use crate::sources::{PartSource, SongPart, Source};
use crate::swing::MAX_SWING;
use crate::tempo::TempoMarker;
use crate::{Scale, Tempo, TimeSignature};

/// The file name extension of Ferrous Lineup projects, without the dot.
pub const EXTENSION: &str = "lineup";

/// Whether `path` names a project file: one ending in `.lineup`, in
/// capitals or not.
pub fn is_project(path: &Path) -> bool {
    path.extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case(EXTENSION))
}

/// The version of the file layout. A later version of Ferrous Lineup that
/// changes the layout will raise this, so it knows how to read old files
/// and so this version can refuse files it would misread.
///
/// Format 2 (step 7) added channels that play an external MIDI synth.
/// A project without one is still written as format 1, which is exactly
/// the same text, so older versions of Ferrous Lineup can open it. See
/// [`format_for`].
///
/// Format 3 (step 8) added nine built-in drums, patterns longer than
/// four bars, and time signatures. Older versions would not know the
/// drums, would cut the long patterns short without a word, and would
/// draw a waltz in bars of four, so those projects need format 3.
///
/// Format 4 added the song's structure: clips that repeat their pattern,
/// named sections that can repeat, and a loop region. Older versions
/// would play every repeat once, so a song that uses them needs format 4.
///
/// Format 5 added time markers, which change the time signature partway
/// through a song. Older versions would draw every bar in the first time
/// signature, so a song with markers needs format 5.
///
/// Format 6 added two kinds of key for music without one: a twelve-tone
/// row and a pitch-class set. Older versions would not know them and could
/// not open the file, so they are told it is newer instead.
///
/// Format 7 added tempo changes through a song, and loose timing. Older
/// versions would play the whole song at its first tempo, and every
/// note on the grid, so a song that uses either needs format 7.
///
/// Format 8 added key changes through a song. Older versions would show
/// the whole song in its first key, so a song with key markers needs
/// format 8.
///
/// Format 9 added a General MIDI instrument to channels. Older versions
/// would play every Lineup Osc with the same sound and send no program
/// changes, so a project with instruments needs format 9.
///
/// Format 10 added notes that start or end between two steps, such as
/// the triplets of an imported MIDI file (see [`NoteEvent::nudge`]).
/// Older versions would move them onto the grid, so a project with one
/// needs format 10.
///
/// Format 11 added beats grouped in irregular ways, such as 7/8 counted
/// 2+2+3. Older versions would read the 7/8 but draw a beat line on
/// every eighth note, so a project with a grouping needs format 11. Any
/// other time signature, such as 11/8 or 5/16, already fits in format 3.
///
/// Format 12 added the chord sheet: the harmony of the song, bar by
/// bar. Older versions would lose it without a word when saving, so a
/// project with chords on its sheet needs format 12.
///
/// Format 13 added the band that plays the chord sheet, and the patterns
/// it writes. Older versions would keep the band's patterns as plain
/// ones and stop following the sheet, so a song with a band needs
/// format 13.
///
/// Format 14 added the band's song form: an intro, a chorus played more
/// than once, and an ending. Older versions would play the song once,
/// as written, so a song whose band plays a form needs format 14.
///
/// Format 15 added the style's second variation, which the band plays
/// from markers on the chord sheet. Older versions would play the whole
/// song in the first one, so a song with variation markers needs
/// format 15.
///
/// Format 16 added swing: the eighth notes played long-short. Older
/// versions would play them even, so a song with swing needs format 16.
///
/// Format 17 added the band's guitar, strings and organ. Older versions
/// know only a piano, a bass and drums, and could not read their parts,
/// so a song whose band has any of them needs format 17.
///
/// Format 18 added shots, holds, rests and pushes to the chord sheet's
/// harmonies. Older versions would drop them and play the style straight
/// through, so a song with any needs format 18.
///
/// Format 19 added the band's soloist. Older versions know no such
/// player, so a song whose band has one, or has a soloist's part, needs
/// format 19.
/// Format 20 separates a band's rhythm from its instruments and remembers
/// the exact harmony style and the option to play plain accents.
/// Format 21 adds the saved master mixer, solo and stereo reverb.
/// Format 22 embeds user style patterns and instruments with the band.
///
/// Format 23 added locked channels (see [`crate::lock`]). Older versions
/// would let styles and the band write over them, so a song with a
/// locked channel needs format 23.
///
/// Format 24 added a style for part of a song (see
/// [`crate::band::sections`]). Older versions would play the whole song
/// in the band's own style, so a song with a section style needs format
/// 24.
///
/// Format 25 added two chords at once to the chord sheet, written
/// "Eb7|Fb" (see [`HarmonyName::Polychord`]). Older versions cannot read
/// them, so a song with any needs format 25.
///
/// Format 26 added a pan and a reverb send to each channel. Older
/// versions would play every channel in the middle with all of it in
/// the reverb, so a song with any other mix needs format 26.
///
/// Format 27 added sending a built-in drum over MIDI (see
/// [`Channel::sent_on`]). Older versions would play it here, so a song
/// with a drum sent over MIDI needs format 27.
///
/// Format 28 added the horn section (see [`Project::set_horns`]), and the
/// equalizer, compressor and delay of each channel (see
/// [`Channel::effects`]). Older versions know neither, so a song with
/// horns, or with a channel going through any of its effects, needs
/// format 28.
///
/// A file can also say where the parts of the song came from (see
/// [`crate::sources`]), in any format: older versions skip it, and lose
/// nothing they could play. In the same way, a key can be written the
/// other way round, Gb major rather than F# major (see
/// [`Scale::other_spelling`]): older versions name it the usual way and
/// play the same notes.
pub const FORMAT_VERSION: u32 = 28;

/// The oldest format that can hold `project`.
///
/// Writing the lowest version that works, instead of always the newest,
/// means a file only becomes unreadable to an older Ferrous Lineup when it
/// really uses something that version does not know.
pub fn format_for(project: &Project) -> u32 {
    let uses_new_drums = project.channels().iter().any(|channel| {
        matches!(
            channel.kind,
            ChannelKind::Sampler(SampleSource::Builtin(drum)) if drum.is_new_in_format_3()
        )
    });
    let has_long_patterns = project
        .patterns()
        .iter()
        .any(|pattern| pattern.length() > MAX_STEPS_BEFORE_FORMAT_3);
    let uses_midi_out = project
        .channels()
        .iter()
        .any(|channel| matches!(channel.kind, ChannelKind::MidiOut { .. }));
    let has_time_signature = !project.time_signature.is_common()
        || project
            .patterns()
            .iter()
            .any(|pattern| pattern.time.is_some());
    let has_instruments = project
        .channels()
        .iter()
        .any(|channel| channel.program.is_some());
    let playlist = project.playlist();
    let has_structure = !playlist.sections().is_empty()
        || playlist.loop_region().is_some()
        || playlist.clips().iter().any(|clip| clip.repeats != 1);
    let has_notes_off_the_grid = project
        .patterns()
        .iter()
        .any(|pattern| pattern.all_notes().any(|(_, note)| !note.is_on_grid()));
    let has_grouped_beats = project.time_signature.is_grouped()
        || project
            .patterns()
            .iter()
            .any(|pattern| pattern.time.is_some_and(TimeSignature::is_grouped))
        || playlist
            .time_markers()
            .iter()
            .any(|marker| marker.time.is_grouped());
    let has_band = project.band().is_some()
        || project
            .patterns()
            .iter()
            .any(|pattern| pattern.player.is_some());
    let has_form = project.form().is_some_and(|form| !form.is_plain());
    let has_bigger_band = project.patterns().iter().any(|pattern| {
        matches!(
            pattern.player,
            Some(Player::Guitar | Player::Strings | Player::Organ)
        )
    }) || project.band().is_some_and(|band| {
        [Player::Guitar, Player::Strings, Player::Organ]
            .into_iter()
            .any(|player| band.channel(player).is_some())
    });
    let has_breaks = playlist
        .chord_markers()
        .iter()
        .any(|marker| marker.push.is_some() || marker.stop.is_some());
    let has_soloist = project.band().is_some_and(Band::has_soloist)
        || project
            .patterns()
            .iter()
            .any(|pattern| pattern.player == Some(Player::Soloist));
    let has_style_parts = project.band().is_some_and(|band| {
        band.instrument_style != band.style || band.harmony_style.is_some() || band.accents_only
    });
    let has_polychords = playlist
        .chord_markers()
        .iter()
        .any(|marker| matches!(marker.harmony, HarmonyName::Polychord(..)));
    let has_channel_mix = project
        .channels()
        .iter()
        .any(|channel| !channel.has_plain_mix());
    let sends_drums = project
        .channels()
        .iter()
        .any(|channel| channel.sent_on.is_some());
    let has_horns = project.band().is_some_and(|band| {
        band.has_horns()
            || [&band.custom_rhythm, &band.custom_instruments]
                .into_iter()
                .flatten()
                .any(|style| style.players().contains(&Player::Horns))
    }) || project
        .patterns()
        .iter()
        .any(|pattern| pattern.player == Some(Player::Horns));
    let has_effects = project
        .channels()
        .iter()
        .any(|channel| !channel.has_plain_effects());
    if has_horns || has_effects {
        28
    } else if sends_drums {
        27
    } else if has_channel_mix {
        26
    } else if has_polychords {
        25
    } else if !project.section_styles().is_empty() {
        24
    } else if !project.locked().is_empty() {
        23
    } else if project
        .band()
        .is_some_and(|band| band.custom_rhythm.is_some() || band.custom_instruments.is_some())
    {
        22
    } else if !project.mixer.is_default() {
        21
    } else if has_style_parts {
        20
    } else if has_soloist {
        19
    } else if has_breaks {
        18
    } else if has_bigger_band {
        17
    } else if project.swing > 0 {
        16
    } else if !project.variations().is_empty() {
        15
    } else if has_form {
        14
    } else if has_band {
        13
    } else if !playlist.chord_markers().is_empty() {
        12
    } else if has_grouped_beats {
        11
    } else if has_notes_off_the_grid {
        10
    } else if has_instruments {
        9
    } else if !playlist.key_markers().is_empty() {
        8
    } else if !playlist.tempo_markers().is_empty() || project.loose_timing > 0 {
        7
    } else if !project.scale.kind.is_tonal() {
        6
    } else if !playlist.time_markers().is_empty() {
        5
    } else if has_structure {
        4
    } else if uses_new_drums || has_long_patterns || has_time_signature {
        3
    } else if uses_midi_out {
        2
    } else {
        1
    }
}

/// A project as it is written in a file.
#[derive(Debug, Serialize, Deserialize)]
struct ProjectFile {
    format: u32,
    #[serde(default, skip_serializing_if = "crate::Mixer::is_default")]
    mixer: crate::Mixer,
    tempo: Tempo,
    scale: Scale,
    /// Left out for 4/4, so those files read exactly as before. A file
    /// without it is in 4/4: `default` fills in `TimeSignature::default()`.
    #[serde(default, skip_serializing_if = "TimeSignature::is_common")]
    time_signature: TimeSignature,
    channels: Vec<Channel>,
    patterns: Vec<PatternFile>,
    tracks: Vec<Track>,
    clips: Vec<Clip>,
    /// Left out when there are none, like `time_signature`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    sections: Vec<Section>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    loop_region: Option<LoopRegion>,
    /// Left out when the time signature never changes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    time_markers: Vec<TimeMarker>,
    /// Left out when the tempo never changes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    tempo_markers: Vec<TempoMarker>,
    /// Left out when the key never changes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    key_markers: Vec<KeyMarker>,
    /// Left out when every note plays on the grid.
    #[serde(default, skip_serializing_if = "is_zero")]
    loose_timing: u8,
    /// Left out when the eighths are played even.
    #[serde(default, skip_serializing_if = "is_zero")]
    swing: u8,
    /// The chord sheet. Left out when it is empty.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    chord_markers: Vec<ChordMarker>,
    /// The band that plays the sheet. Left out when there is none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    band: Option<BandFile>,
    /// Where the parts of the song came from. Left out when they are
    /// all yours.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    sources: Vec<SourceFile>,
    /// Where the parts that older versions do not know came from: the
    /// drums, the melody and the key (see [`SongPart::FIRST`]). They have
    /// a field of their own, which those versions skip. It is written
    /// whenever a part came from somewhere, even empty, so a file without
    /// it is from before.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    more_sources: Option<Vec<SourceFile>>,
    /// The locked channels, by number. Left out when there are none.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    locked: Vec<usize>,
}

fn is_zero(value: &u8) -> bool {
    *value == 0
}

fn is_false(value: &bool) -> bool {
    !*value
}

/// Where a part of the song came from, as it is written in a file: and
/// whether it was edited since, because the number that tells
/// ([`Project::part_fingerprint`]) is only good within one run of the app.
#[derive(Debug, Serialize, Deserialize)]
struct SourceFile {
    part: SongPart,
    from: Source,
    #[serde(default, skip_serializing_if = "is_false")]
    edited: bool,
}

/// A pattern as it is written in a file.
#[derive(Debug, Serialize, Deserialize)]
struct PatternFile {
    name: String,
    /// Left out when it is the pattern's place in the list, as it is for
    /// every pattern until one is removed, so most files read exactly as
    /// before. Older versions skip it and colour patterns by place.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    color: Option<usize>,
    /// Left out when the pattern takes the song's time signature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    time: Option<TimeSignature>,
    /// The band's player whose part it is. Left out for your own.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    player: Option<Player>,
    length: usize,
    /// One list of notes per channel, in the order of `channels`.
    notes: Vec<Vec<NoteEvent>>,
}

/// Everything that can go wrong while saving or loading.
#[derive(Debug)]
pub enum FileError {
    /// The file could not be read or written: missing, no permission, disk full...
    Io(io::Error),
    /// The text is not a valid project file. The error says where.
    Parse(ron::error::SpannedError),
    /// The project could not be turned into text (should never happen).
    Write(ron::Error),
    /// The file was saved by a newer version of Ferrous Lineup.
    NewerFormat(u32),
    /// The file is shaped right, but what it says does not make sense,
    /// such as a clip that plays a pattern that does not exist.
    Invalid(String),
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileError::Io(err) => write!(f, "{err}"),
            FileError::Parse(err) => f.write_str(&trf("not a valid project file: {}", &[err])),
            FileError::Write(err) => f.write_str(&trf("could not write the project: {}", &[err])),
            FileError::NewerFormat(version) => f.write_str(&trf(
                "this file uses format {}, made by a newer version of Ferrous Lineup \
                 (this one reads up to format {})",
                &[version, &FORMAT_VERSION],
            )),
            FileError::Invalid(reason) => {
                f.write_str(&trf("the project file is damaged: {}", &[reason]))
            }
        }
    }
}

impl std::error::Error for FileError {
    /// The lower-level error this one wraps, if any.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FileError::Io(err) => Some(err),
            FileError::Parse(err) => Some(err),
            FileError::Write(err) => Some(err),
            FileError::NewerFormat(_) | FileError::Invalid(_) => None,
        }
    }
}

// These `From` implementations let the `?` operator turn the errors of
// `std::fs` and `ron` into a `FileError` on its own.

impl From<io::Error> for FileError {
    fn from(err: io::Error) -> Self {
        FileError::Io(err)
    }
}

impl From<ron::error::SpannedError> for FileError {
    fn from(err: ron::error::SpannedError) -> Self {
        FileError::Parse(err)
    }
}

impl From<ron::Error> for FileError {
    fn from(err: ron::Error) -> Self {
        FileError::Write(err)
    }
}

impl From<&Project> for ProjectFile {
    fn from(project: &Project) -> Self {
        let channels = project.channels().len();
        let origins: Vec<SourceFile> = SongPart::ALL
            .into_iter()
            .filter_map(|part| {
                let origin = project.part_source(part)?;
                Some(SourceFile {
                    part,
                    from: origin.source.clone(),
                    edited: origin.edited,
                })
            })
            .collect();
        let brought = !origins.is_empty();
        let (sources, more_sources): (Vec<_>, Vec<_>) = origins
            .into_iter()
            .partition(|came| SongPart::FIRST.contains(&came.part));
        ProjectFile {
            format: format_for(project),
            mixer: project.mixer,
            tempo: project.tempo,
            scale: project.scale,
            time_signature: project.time_signature,
            channels: project.channels().to_vec(),
            patterns: project
                .patterns()
                .iter()
                .enumerate()
                .map(|(index, pattern)| PatternFile {
                    name: pattern.name.clone(),
                    color: (pattern.color != index).then_some(pattern.color),
                    time: pattern.time,
                    player: pattern.player,
                    length: pattern.length(),
                    notes: (0..channels)
                        .map(|channel| pattern.notes(channel).to_vec())
                        .collect(),
                })
                .collect(),
            tracks: project.playlist().tracks().to_vec(),
            clips: project.playlist().clips().to_vec(),
            sections: project.playlist().sections().to_vec(),
            loop_region: project.playlist().loop_region(),
            time_markers: project.playlist().time_markers().to_vec(),
            tempo_markers: project.playlist().tempo_markers().to_vec(),
            key_markers: project.playlist().key_markers().to_vec(),
            loose_timing: project.loose_timing,
            swing: project.swing,
            chord_markers: project.playlist().chord_markers().to_vec(),
            band: project.band().map(BandFile::from),
            sources,
            more_sources: (!more_sources.is_empty() || brought).then_some(more_sources),
            locked: project.locked().to_vec(),
        }
    }
}

impl TryFrom<ProjectFile> for Project {
    type Error = FileError;

    fn try_from(file: ProjectFile) -> Result<Self, Self::Error> {
        let invalid = |reason: String| Err(FileError::Invalid(reason));

        let mut project = Project::empty();
        if !file.mixer.valid() {
            return invalid(tr("its mixer settings are outside their ranges").to_string());
        }
        project.mixer = file.mixer;
        project.tempo = file.tempo;
        project.scale = file.scale;
        if file.time_signature.steps_per_bar() > MAX_STEPS {
            return invalid(trf(
                "its time signature {} is too long",
                &[&file.time_signature],
            ));
        }
        project.time_signature = file.time_signature;

        if file
            .mixer
            .solo
            .is_some_and(|solo| solo >= file.channels.len())
        {
            return invalid(tr("its solo channel does not exist").to_string());
        }
        if file.channels.len() > MAX_CHANNELS {
            return invalid(trf("it has more than {} channels", &[&MAX_CHANNELS]));
        }
        let channels = file.channels.len();
        if let Some(channel) = file.channels.iter().find(|channel| !channel.mix_is_valid()) {
            return invalid(trf(
                "channel \"{}\" has its pan or reverb send outside its range",
                &[&channel.name],
            ));
        }
        if let Some(channel) = file
            .channels
            .iter()
            .find(|channel| !channel.effects.is_valid())
        {
            return invalid(trf(
                "channel \"{}\" has an effect set outside its range",
                &[&channel.name],
            ));
        }
        if let Some(channel) = file.channels.iter().find(|channel| {
            channel
                .sent_on
                .is_some_and(|midi_channel| midi_channel > 15)
        }) {
            return invalid(trf(
                "channel \"{}\" is sent over a MIDI channel other than 1 to 16",
                &[&channel.name],
            ));
        }
        for channel in file.channels {
            project.add_channel(channel);
        }

        if file.patterns.is_empty() || file.patterns.len() > MAX_PATTERNS {
            return invalid(trf("it must have 1 to {} patterns", &[&MAX_PATTERNS]));
        }
        for (index, saved) in file.patterns.into_iter().enumerate() {
            // `Project::empty` already has one pattern; add the others.
            if index > 0 {
                project.add_pattern();
            }
            let Some(pattern) = project.pattern_mut(index) else {
                return invalid(trf("pattern {} could not be added", &[&(index + 1)]));
            };
            pattern.name = saved.name;
            pattern.color = saved.color.unwrap_or(index);
            if saved
                .time
                .is_some_and(|time| time.steps_per_bar() > MAX_STEPS)
            {
                return invalid(trf("pattern \"{}\" has too long a bar", &[&pattern.name]));
            }
            pattern.time = saved.time;
            pattern.player = saved.player;
            pattern.set_length(saved.length);
            if saved.notes.len() != channels {
                return invalid(trf(
                    "pattern \"{}\" has notes for {} channels, but there are {}",
                    &[&pattern.name, &saved.notes.len(), &channels],
                ));
            }
            for (channel, notes) in saved.notes.into_iter().enumerate() {
                for note in notes {
                    if pattern.add_note(channel, note).is_none() {
                        return invalid(trf(
                            "pattern \"{}\" has a note that does not fit, or a duplicate, \
                             at step {} on channel {}",
                            &[&pattern.name, &note.start.saturating_add(1), &(channel + 1)],
                        ));
                    }
                }
            }
        }

        if file.tracks.is_empty() || file.tracks.len() > MAX_TRACKS {
            return invalid(trf("it must have 1 to {} playlist tracks", &[&MAX_TRACKS]));
        }
        project.replace_tracks(file.tracks);
        for (index, clip) in file.clips.into_iter().enumerate() {
            // A file names all its tracks: a clip below them is a mistake,
            // not a new track as it is when placed by hand.
            let on_a_track = clip.track < project.playlist().tracks().len();
            if !on_a_track || project.add_clip(clip).is_none() {
                return invalid(trf(
                    "clip {} (pattern {}, track {}) does not fit in the playlist",
                    &[
                        &(index + 1),
                        &clip.pattern.saturating_add(1),
                        &clip.track.saturating_add(1),
                    ],
                ));
            }
        }
        for section in file.sections {
            if project.add_section(section.clone()).is_none() {
                return invalid(trf(
                    "section \"{}\" is empty, too long, repeats too often or overlaps another",
                    &[&section.name],
                ));
            }
        }
        if !project.set_loop_region(file.loop_region) {
            return invalid(tr("its loop region is empty or too long").to_string());
        }
        for marker in file.time_markers {
            if marker.start == 0 || !project.set_time_marker(marker) {
                return invalid(trf(
                    "its change to {} at step {} is too late, too long or on the first step",
                    &[&marker.time, &marker.start.saturating_add(1)],
                ));
            }
        }
        for marker in file.tempo_markers {
            if marker.start == 0 || !project.set_tempo_marker(marker) {
                return invalid(trf(
                    "its change to {} BPM at step {} is too late or on the first step",
                    &[&marker.tempo.bpm(), &marker.start.saturating_add(1)],
                ));
            }
        }
        for marker in file.key_markers {
            if marker.start == 0 || !project.set_key_marker(marker) {
                return invalid(trf(
                    "its change to {} at step {} is too late or on the first step",
                    &[&marker.scale.name(), &marker.start.saturating_add(1)],
                ));
            }
        }
        for marker in file.chord_markers {
            if !project.set_chords(marker.start, marker.start.saturating_add(1), &[marker]) {
                return invalid(trf(
                    "its chord {} at step {} is too late",
                    &[&marker.harmony, &marker.start.saturating_add(1)],
                ));
            }
        }
        if let Some(band) = file.band {
            match band.band(channels) {
                Ok(band) => project.band = Some(band),
                Err(reason) => return invalid(reason),
            }
        }
        project.lock_saved(file.locked);
        project.loose_timing = file.loose_timing.min(MAX_LOOSE_TIMING);
        project.swing = file.swing.min(MAX_SWING);
        project.remove_blank_tracks();
        let older = file.more_sources.is_none();
        for came in file
            .sources
            .into_iter()
            .chain(file.more_sources.into_iter().flatten())
        {
            let print = (!came.edited).then(|| project.part_fingerprint(came.part));
            let source = PartSource {
                source: came.from,
                print,
            };
            project.sources.set(came.part, Some(source));
        }
        if older {
            project.sources_of_an_older_file();
        }
        Ok(project)
    }
}

/// Turns a project into the text of a project file.
pub fn to_text(project: &Project) -> Result<String, FileError> {
    // Pretty-printed, so the file can be read (and fixed) by a person.
    // Things nested deeper than 5 levels go on one line, so each note
    // takes one line instead of one line per field. Lines always end in
    // "\n" (RON would use "\r\n" on Windows), so a project is the same
    // file on every system.
    // Names are written in letters, whichever way the app shows them.
    let style = ron::ser::PrettyConfig::new().depth_limit(5).new_line("\n");
    let file = Language::English.with(|| NoteNaming::Letters.with(|| ProjectFile::from(project)));
    let mut text = NoteNaming::Letters.with(|| ron::ser::to_string_pretty(&file, style))?;
    text.push('\n');
    Ok(text)
}

/// Reads a project from the text of a project file.
///
/// The whole file is read first, and its format number checked after. A
/// file from a newer version may not read at all, because of fields or
/// names this version does not know: then the format number alone says
/// why.
pub fn from_text(text: &str) -> Result<Project, FileError> {
    // A text editor may start the file with a byte-order mark, which is
    // not part of the RON text or of the format number.
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let file: ProjectFile = match NoteNaming::Letters.with(|| ron::from_str(text)) {
        Ok(file) => file,
        Err(err) => {
            return Err(match format_number(text) {
                Some(format) if format > FORMAT_VERSION => FileError::NewerFormat(format),
                _ => err.into(),
            });
        }
    };
    if file.format > FORMAT_VERSION {
        return Err(FileError::NewerFormat(file.format));
    }
    NoteNaming::Letters.with(|| Project::try_from(file))
}

/// The format number at the start of a project file, `(format: 25, ...`,
/// read without the RON reader.
///
/// RON could read it as a struct with just that field, skipping the
/// others, but to skip a number it looks ahead through all the text
/// after it. Doing that for every number of a long song took over a
/// minute.
fn format_number(text: &str) -> Option<u32> {
    let rest = text.trim_start().strip_prefix('(')?.trim_start();
    let rest = rest.strip_prefix("format")?.trim_start();
    let rest = rest.strip_prefix(':')?.trim_start();
    let digits = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
    rest[..digits].parse().ok()
}

/// Saves a project to a file, replacing the file if it exists.
///
/// The text is first written to a temporary file next to it, which is
/// then renamed. Renaming is a single step for the operating system, so
/// if the computer crashes halfway, the old file is still intact.
pub fn save(project: &Project, path: &Path) -> Result<(), FileError> {
    let text = to_text(project)?;
    let temporary = path.with_extension(format!("{EXTENSION}.tmp"));
    if let Err(err) = fs::write(&temporary, text) {
        let _ = fs::remove_file(&temporary);
        return Err(err.into());
    }
    if let Err(err) = fs::rename(&temporary, path) {
        let _ = fs::remove_file(&temporary);
        return Err(err.into());
    }
    Ok(())
}

/// Loads a project from a file.
pub fn load(path: &Path) -> Result<Project, FileError> {
    let text = fs::read_to_string(path)?;
    from_text(&text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Note;
    use crate::project::Drum;

    #[test]
    fn mixer_round_trip_and_legacy_defaults() {
        let mut project = Project::demo();
        let legacy = to_text(&project).unwrap();
        assert!(!legacy.contains("mixer:"));
        assert_eq!(from_text(&legacy).unwrap().mixer, crate::Mixer::default());
        project.mixer.master = 0.42;
        project.mixer.solo = Some(2);
        project.mixer.reverb.enabled = true;
        project.mixer.reverb.decay = 3.2;
        assert_eq!(format_for(&project), 21);
        let text = to_text(&project).unwrap();
        assert_eq!(from_text(&text).unwrap(), project);
        assert!(from_text(&text.replace("master: 0.42", "master: NaN")).is_err());
        project.remove_channel(0);
        assert_eq!(project.mixer.solo, Some(1));
        project.remove_channel(1);
        assert_eq!(project.mixer.solo, None);
    }

    /// The demo song with nothing that needs a newer format than 1: its
    /// chords on the Lineup Osc's own sound instead of a piano.
    fn plain_demo() -> Project {
        let mut project = Project::demo();
        for index in 0..project.channels().len() {
            if let Some(channel) = project.channel_mut(index) {
                channel.program = None;
            }
        }
        project
    }

    #[test]
    fn the_demo_survives_a_round_trip() {
        let demo = Project::demo();
        let text = to_text(&demo).unwrap();
        assert!(text.contains("tempo: 130.0"), "{text}");
        assert!(text.contains("kind: Sampler(Builtin(Kick))"), "{text}");
        let loaded = from_text(&text).unwrap();
        assert_eq!(loaded, demo);
    }

    #[test]
    fn saving_and_loading_a_file() {
        let mut project = Project::template();
        project.tempo = Tempo::new(97.5);
        project
            .pattern_mut(0)
            .unwrap()
            .add_note(4, NoteEvent::new(Note::A4, 2, 3));
        let path = std::env::temp_dir().join(format!("lineup-test-{}.lineup", std::process::id()));
        save(&project, &path).unwrap();
        let loaded = load(&path);
        std::fs::remove_file(&path).unwrap();
        assert_eq!(loaded.unwrap(), project);
    }

    #[test]
    fn patterns_keep_their_colours_in_a_file() {
        let mut project = plain_demo();
        let text = to_text(&project).unwrap();
        assert!(
            !text.contains("color"),
            "colours in their places are left out"
        );
        project.remove_pattern(0);
        let text = to_text(&project).unwrap();
        assert_eq!(from_text(&text).unwrap(), project);
    }

    #[test]
    fn a_missing_file_is_an_io_error() {
        let result = load(Path::new("/this/file/does/not/exist.lineup"));
        assert!(matches!(result, Err(FileError::Io(_))));
    }

    #[test]
    fn rubbish_is_a_parse_error() {
        let error = from_text("this is not a project").unwrap_err();
        assert!(matches!(error, FileError::Parse(_)));
        assert!(error.to_string().starts_with("not a valid project file"));
    }

    #[test]
    fn files_from_the_future_are_refused() {
        // A future format might have fields we have never heard of: the
        // format number alone says why the file does not read.
        let error =
            from_text(&format!("(format: {}, sparkles: true)", FORMAT_VERSION + 1)).unwrap_err();
        assert!(
            matches!(error, FileError::NewerFormat(version) if version == FORMAT_VERSION+1),
            "{error}"
        );
        // Or it might read, and still mean something else.
        let text = to_text(&Project::template()).unwrap();
        let newer = text.replacen(
            &format!("format: {}", format_for(&Project::template())),
            &format!("format: {}", FORMAT_VERSION + 1),
            1,
        );
        assert!(matches!(
            from_text(&newer),
            Err(FileError::NewerFormat(version)) if version == FORMAT_VERSION + 1
        ));
    }

    #[test]
    fn the_format_number_is_read_from_the_start_of_the_file() {
        assert_eq!(format_number("(format: 25, tempo: 120.0)"), Some(25));
        assert_eq!(format_number("  (\n    format:3,\n"), Some(3));
        assert_eq!(format_number("(tempo: 120.0, format: 25)"), None);
        assert_eq!(format_number("this is not a project"), None);
    }

    #[test]
    fn a_long_song_opens_quickly() {
        // Boléro is the longest demo: over two megabytes of text. Reading
        // its format number with RON alone took 85 seconds, even
        // optimized; the whole file now reads in a fraction of one.
        let bolero = crate::library::SONGS
            .iter()
            .find(|song| song.name == "Boléro")
            .unwrap();
        let project = crate::library::demo::song_demo(bolero);
        let text = to_text(&project).unwrap();
        let start = std::time::Instant::now();
        assert_eq!(from_text(&text).unwrap(), project);
        let seconds = start.elapsed().as_secs_f64();
        assert!(seconds < 10.0, "Boléro took {seconds:.1} s to read");
    }

    #[test]
    fn lines_end_the_same_way_on_every_system() {
        let text = to_text(&Project::template()).unwrap();
        assert!(!text.contains('\r'));
    }

    #[test]
    fn midi_out_channels_need_format_two() {
        let mut project = Project::template();
        assert!(to_text(&project).unwrap().contains("format: 1,"));

        project.add_channel(Channel::midi_out(4)).unwrap();
        let text = to_text(&project).unwrap();
        assert!(text.contains("format: 2,"), "{text}");
        assert!(
            text.contains("MidiOut(\n                channel: 4,"),
            "{text}"
        );
        assert_eq!(from_text(&text).unwrap(), project);
    }

    #[test]
    fn new_drums_and_long_patterns_need_format_three() {
        let mut project = Project::template();
        project.add_channel(Channel::midi_out(0)).unwrap();
        assert!(to_text(&project).unwrap().contains("format: 2,"));

        let mut drums = project.clone();
        drums.add_channel(Channel::drum(Drum::Conga)).unwrap();
        let text = to_text(&drums).unwrap();
        assert!(text.contains("format: 3,"), "{text}");
        assert_eq!(from_text(&text).unwrap(), drums);

        let mut long = project;
        long.pattern_mut(0).unwrap().set_length(192);
        long.pattern_mut(0)
            .unwrap()
            .add_note(4, NoteEvent::new(Note::C4, 180, 12));
        let text = to_text(&long).unwrap();
        assert!(text.contains("format: 3,"), "{text}");
        assert_eq!(from_text(&text).unwrap(), long);
    }

    #[test]
    fn time_signatures_other_than_four_four_need_format_three() {
        let mut project = Project::template();
        let text = to_text(&project).unwrap();
        assert!(!text.contains("time_signature"), "4/4 is left out: {text}");
        assert_eq!(
            from_text(&text).unwrap().time_signature,
            TimeSignature::COMMON
        );

        project.time_signature = TimeSignature::new(3, 4);
        let text = to_text(&project).unwrap();
        assert!(text.contains("format: 3,"), "{text}");
        assert!(text.contains("beats: 3"), "{text}");
        assert_eq!(from_text(&text).unwrap(), project);
    }

    #[test]
    fn grouped_beats_need_format_eleven() {
        let mut project = Project::template();
        project.time_signature = "11/8".parse().unwrap();
        let text = to_text(&project).unwrap();
        assert!(
            text.contains("format: 3,"),
            "any numbers fit in format 3: {text}"
        );

        project.time_signature = "2+2+3/8".parse().unwrap();
        assert!(project.set_time_marker(TimeMarker::new(28, "3+3+2+3/8".parse().unwrap())));
        let text = to_text(&project).unwrap();
        assert!(text.contains("format: 11,"), "{text}");
        assert!(text.contains("groups: ["), "{text}");
        assert_eq!(from_text(&text).unwrap(), project);
    }

    #[test]
    fn song_structure_needs_format_four() {
        let mut project = plain_demo();
        let text = to_text(&project).unwrap();
        assert!(text.contains("format: 1,"), "{text}");
        assert!(
            !text.contains("repeats") && !text.contains("sections"),
            "{text}"
        );

        let mut chorus = Section::new("Chorus", 32, 64);
        chorus.repeats = 2;
        project.add_section(chorus);
        project.add_section(Section::new("Outro", 96, 32));
        project.set_clip_repeats(0, 2);
        project.set_loop_region(Some(LoopRegion::new(32, 96)));
        let text = to_text(&project).unwrap();
        assert!(text.contains("format: 4,"), "{text}");
        assert!(text.contains("name: \"Chorus\""), "{text}");
        assert!(text.contains("repeats: 2"), "{text}");
        assert!(text.contains("loop_region"), "{text}");
        assert_eq!(from_text(&text).unwrap(), project);
    }

    #[test]
    fn time_markers_need_format_five() {
        let mut project = plain_demo();
        assert!(project.set_time_marker(TimeMarker::new(64, TimeSignature::new(3, 4))));
        let text = to_text(&project).unwrap();
        assert!(text.contains("format: 5,"), "{text}");
        assert!(text.contains("time_markers"), "{text}");
        let loaded = from_text(&text).unwrap();
        assert_eq!(loaded.meter(), project.meter());

        // A marker on step 0 is the project's own time signature.
        let broken = text.replace(
            "start: 64,\n            time",
            "start: 0,\n            time",
        );
        assert_ne!(broken, text, "{text}");
        assert!(from_text(&broken).is_err());
    }

    #[test]
    fn tempo_changes_and_loose_timing_need_format_seven() {
        let mut project = plain_demo();
        assert!(project.set_tempo_marker(TempoMarker::gliding(96, 90.0, 16)));
        assert!(project.set_tempo_marker(TempoMarker::new(120, 130.0)));
        let text = to_text(&project).unwrap();
        assert!(text.contains("format: 7,"), "{text}");
        assert!(text.contains("tempo_markers"), "{text}");
        assert!(text.contains("glide: 16"), "{text}");
        assert!(!text.contains("loose_timing"), "{text}");
        let loaded = from_text(&text).unwrap();
        assert_eq!(loaded, project);
        assert_eq!(loaded.tempo_map(), project.tempo_map());

        let mut loose = plain_demo();
        loose.loose_timing = 12;
        let text = to_text(&loose).unwrap();
        assert!(text.contains("format: 7,"), "{text}");
        assert_eq!(from_text(&text).unwrap(), loose);
    }

    #[test]
    fn songs_keep_where_their_parts_came_from() {
        use crate::{SongPart, Source, SourceKind};
        let mut project = Project::demo();
        let text = to_text(&project).unwrap();
        assert!(!text.contains("sources"), "{text}");
        let format = format_for(&project);
        project.bring_in_all(&Source::new(SourceKind::Demo, "El Choclo"));
        assert_eq!(format_for(&project), format, "older versions skip it");
        project.swing = 30;
        let read = from_text(&to_text(&project).unwrap()).unwrap();
        for part in SongPart::ALL {
            let origin = read.part_source(part).unwrap();
            assert_eq!(origin.source.name, "El Choclo");
            assert_eq!(origin.edited, part == SongPart::Swing, "{part:?}");
        }
        // Still edited after it is set back: the file only knew it changed.
        let mut read = read;
        read.swing = 0;
        assert!(read.part_source(SongPart::Swing).unwrap().edited);
    }

    #[test]
    fn a_file_from_before_has_its_drums_melody_and_key_come_with_the_other_parts() {
        use crate::{SongPart, Source, SourceKind};
        let mut project = Project::demo();
        project.bring_in_all(&Source::new(SourceKind::Demo, "El Choclo"));
        project.swing = 30;
        let mut file = ProjectFile::from(&project);
        let more = file.more_sources.take().unwrap();
        assert_eq!(more.len(), 3, "the drums, the melody and the key");
        assert_eq!(file.sources.len(), 6);
        let older = ron::ser::to_string(&file).unwrap();
        let read = from_text(&older).unwrap();
        for part in SongPart::ALL {
            let origin = read.part_source(part).unwrap();
            assert_eq!(origin.source.name, "El Choclo", "{part:?}");
            assert_eq!(origin.edited, part == SongPart::Swing, "{part:?}");
        }
        // A style's patterns bring no melody, and the key comes with the
        // chords.
        let style = Source::new(SourceKind::Style, "Bossa nova");
        let mut project = Project::demo();
        project.bring_in(
            &style,
            crate::sources::PartSet::of(&[SongPart::Patterns]),
            crate::sources::PartSet::NONE,
            |_| (),
        );
        let mut file = ProjectFile::from(&project);
        file.more_sources = None;
        let read = from_text(&ron::ser::to_string(&file).unwrap()).unwrap();
        assert_eq!(read.part_source(SongPart::Drums).unwrap().source, &style);
        assert!(read.part_source(SongPart::Melody).is_none());
        assert!(read.part_source(SongPart::Key).is_none());
    }

    #[test]
    fn swing_needs_format_sixteen() {
        let mut project = plain_demo();
        let text = to_text(&project).unwrap();
        assert!(!text.contains("swing"), "{text}");
        project.swing = 60;
        let text = to_text(&project).unwrap();
        assert!(text.contains("format: 16,"), "{text}");
        assert!(text.contains("swing: 60"), "{text}");
        assert_eq!(from_text(&text).unwrap(), project);
        // Too much swing from a file edited by hand is a triplet.
        let text = text.replace("swing: 60", "swing: 250");
        assert_eq!(from_text(&text).unwrap().swing, MAX_SWING);
    }

    #[test]
    fn key_changes_need_format_eight() {
        let mut project = plain_demo();
        let g_major = Scale::new(crate::PitchClass::G, crate::ScaleKind::Major);
        assert!(project.set_key_marker(KeyMarker::new(64, g_major)));
        let text = to_text(&project).unwrap();
        assert!(text.contains("format: 8,"), "{text}");
        assert!(text.contains("key_markers"), "{text}");
        let loaded = from_text(&text).unwrap();
        assert_eq!(loaded, project);
        assert_eq!(loaded.key_at(64), g_major);
    }

    #[test]
    fn the_chord_sheet_needs_format_twelve() {
        let mut project = plain_demo();
        let name = |text| crate::HarmonyName::parse(text).unwrap();
        let chords = [
            ChordMarker::new(0, name("C")),
            ChordMarker::new(16, name("G7/B")),
            ChordMarker::new(24, name("3-11 T2")),
            ChordMarker::new(32, name("Menor 3 T2I")),
            ChordMarker::new(48, name("N.C.")),
        ];
        assert!(project.set_chords(0, 64, &chords));
        let text = to_text(&project).unwrap();
        assert!(text.contains("format: 12,"), "{text}");
        assert!(text.contains("harmony: \"G7/B\""), "{text}");
        let loaded = from_text(&text).unwrap();
        assert_eq!(loaded, project);
        assert_eq!(loaded.playlist().chord_markers(), chords);

        let bad = text.replace("G7/B", "H7");
        assert!(from_text(&bad).is_err());
    }

    #[test]
    fn two_chords_at_once_need_format_twenty_five() {
        let mut project = plain_demo();
        let name = |text| crate::HarmonyName::parse(text).unwrap();
        let chords = [
            ChordMarker::new(0, name("Eb7|Fb")),
            ChordMarker::new(16, name("C|F#/C#")),
        ];
        assert!(project.set_chords(0, 32, &chords));
        let text = to_text(&project).unwrap();
        assert!(text.contains("format: 25,"), "{text}");
        assert!(text.contains("harmony: \"Eb7|Fb\""), "{text}");
        let loaded = from_text(&text).unwrap();
        assert_eq!(loaded.playlist().chord_markers(), chords);
    }

    #[test]
    fn chords_with_two_sharps_open_again() {
        // In G# harmonic minor the leading-tone chord is F##dim7, written
        // by the sheet itself from "vii°7".
        let mut project = plain_demo();
        project.scale = crate::Scale::new(
            crate::PitchClass::GSharp,
            crate::harmony::ScaleKind::HarmonicMinor,
        );
        let bar = project.meter().bar(0);
        let chords = crate::chord_sheet::read_bar("vii°7 D#/F##", &bar, project.scale).unwrap();
        assert!(project.set_chords(0, 16, &chords));
        let text = to_text(&project).unwrap();
        assert!(text.contains("harmony: \"F##dim7\""), "{text}");
        assert!(text.contains("harmony: \"D#/F##\""), "{text}");
        assert_eq!(from_text(&text).unwrap(), project);
    }

    #[test]
    fn shots_holds_rests_and_pushes_need_format_eighteen() {
        use crate::chord_sheet::{Push, Stop};
        let mut project = plain_demo();
        let name = |text| crate::HarmonyName::parse(text).unwrap();
        let chords = [
            ChordMarker::new(0, name("C")).stopped(Some(Stop::Shot)),
            ChordMarker::new(16, name("F")).pushed(Some(Push::Eighth)),
            ChordMarker::new(32, name("G7")),
        ];
        assert!(project.set_chords(0, 48, &chords));
        let text = to_text(&project).unwrap();
        assert!(text.contains("format: 18,"), "{text}");
        assert!(text.contains("stop: Some(Shot)"), "{text}");
        assert!(text.contains("push: Some(Eighth)"), "{text}");
        // A harmony without marks is written as before.
        assert_eq!(text.matches("stop:").count(), 1, "{text}");
        assert_eq!(text.matches("push:").count(), 1, "{text}");
        let loaded = from_text(&text).unwrap();
        assert_eq!(loaded.playlist().chord_markers(), chords);
    }

    #[test]
    fn notes_off_the_grid_need_format_ten() {
        let mut project = plain_demo();
        let synth = 4;
        let text = to_text(&project).unwrap();
        assert!(!text.contains("nudge"), "notes on the grid read as before");
        // A triplet note, 8 ticks late, held 32 ticks.
        let triplet = crate::NoteEvent::exact(crate::Note::C4, 32, 32);
        project.pattern_mut(0).unwrap().add_note(synth, triplet);
        let text = to_text(&project).unwrap();
        assert!(text.contains("format: 10,"), "{text}");
        assert!(text.contains("nudge: 8"), "{text}");
        assert!(text.contains("trim: 8"), "{text}");
        assert_eq!(from_text(&text).unwrap(), project);
    }

    #[test]
    fn pan_and_reverb_sends_need_format_twenty_six() {
        let mut project = plain_demo();
        let text = to_text(&project).unwrap();
        assert!(
            !text.contains("pan") && !text.contains("reverb_send"),
            "{text}"
        );
        project.channel_mut(0).unwrap().pan = -0.5;
        project.channel_mut(1).unwrap().reverb_send = 0.25;
        let text = to_text(&project).unwrap();
        assert!(text.contains("format: 26,"), "{text}");
        assert!(text.contains("pan: -0.5"), "{text}");
        assert!(text.contains("reverb_send: 0.25"), "{text}");
        assert_eq!(from_text(&text).unwrap(), project);
        // A file edited by hand cannot pan past the speakers.
        let text = text.replace("pan: -0.5", "pan: -3.0");
        assert!(matches!(from_text(&text), Err(FileError::Invalid(_))));
    }

    #[test]
    fn drums_sent_over_midi_need_format_twenty_seven() {
        let mut project = plain_demo();
        let kick = project
            .channels()
            .iter()
            .position(|channel| {
                channel.kind == ChannelKind::Sampler(SampleSource::Builtin(Drum::Kick))
            })
            .unwrap();
        project.channel_mut(kick).unwrap().send_over_midi(9);
        let text = to_text(&project).unwrap();
        assert!(text.contains("format: 27,"), "{text}");
        assert!(text.contains("sent_on: Some(9)"), "{text}");
        assert_eq!(from_text(&text).unwrap(), project);
        // A file edited by hand cannot send it past MIDI channel 16.
        let text = text.replace("sent_on: Some(9)", "sent_on: Some(16)");
        assert!(matches!(from_text(&text), Err(FileError::Invalid(_))));
    }

    #[test]
    fn effects_need_format_twenty_eight() {
        let mut project = plain_demo();
        let text = to_text(&project).unwrap();
        assert!(!text.contains("effects"), "{text}");
        let channel = project.channel_mut(0).unwrap();
        channel.effects.eq.low = -6.0;
        channel.effects.delay.mix = 0.3;
        let text = to_text(&project).unwrap();
        assert!(text.contains("format: 28,"), "{text}");
        assert!(text.contains("low: -6.0"), "{text}");
        // The compressor, off, is left out.
        assert!(!text.contains("compressor"), "{text}");
        assert_eq!(from_text(&text).unwrap(), project);
        // A file edited by hand cannot have echoes that never die away.
        let text = text.replace("feedback: 0.35", "feedback: 1.5");
        assert!(matches!(from_text(&text), Err(FileError::Invalid(_))));
    }

    #[test]
    fn instruments_need_format_nine() {
        let mut project = plain_demo();
        let text = to_text(&project).unwrap();
        assert!(!text.contains("program"), "{text}");
        project.channel_mut(0).unwrap().program = Some(crate::general_midi::VIOLIN);
        let text = to_text(&project).unwrap();
        assert!(text.contains("format: 9,"), "{text}");
        assert!(text.contains("program: Some(40)"), "{text}");
        assert_eq!(from_text(&text).unwrap(), project);
    }

    #[test]
    fn rows_and_sets_need_format_six() {
        let mut project = plain_demo();
        project.scale = Scale::row(crate::harmony::ROW_PRESETS[0].row);
        let text = to_text(&project).unwrap();
        assert!(text.contains("format: 6,"), "{text}");
        assert!(
            text.contains("kind: Row(\"0 1 3 9 2 11 4 10 7 8 5 6\")"),
            "{text}"
        );
        assert_eq!(from_text(&text).unwrap(), project);

        project.scale = Scale::set(
            crate::PitchClass::D,
            crate::PitchSet::from_semitones(&[2, 5, 8, 11]),
        );
        let text = to_text(&project).unwrap();
        assert!(text.contains("format: 6,"), "{text}");
        assert!(text.contains("kind: Set(\"0 3 6 9\")"), "{text}");
        assert_eq!(from_text(&text).unwrap(), project);

        // A row that repeats a note is not a row.
        let broken = text.replace("Set(\"0 3 6 9\")", "Row(\"0 0 2 3 4 5 6 7 8 9 10 11\")");
        assert!(from_text(&broken).is_err());
    }

    #[test]
    fn overlapping_sections_are_reported() {
        let mut project = plain_demo();
        project.add_section(Section::new("Verse", 0, 32));
        project.add_section(Section::new("Chorus", 32, 32));
        let text = to_text(&project).unwrap();
        let broken = text.replace(
            "start: 32,\n            length: 32",
            "start: 16,\n            length: 32",
        );
        assert_ne!(broken, text, "{text}");
        let error = from_text(&broken).unwrap_err();
        assert!(error.to_string().contains("\"Chorus\""), "{error}");
    }

    #[test]
    fn a_byte_order_mark_is_not_part_of_the_text() {
        let project = plain_demo();
        let text = to_text(&project).unwrap();
        assert_eq!(from_text(&format!("\u{feff}{text}")).unwrap(), project);
        assert!(matches!(
            from_text("\u{feff}(format: 99)"),
            Err(FileError::NewerFormat(99))
        ));
    }

    #[test]
    fn values_out_of_range_are_clamped() {
        let text = to_text(&Project::empty()).unwrap();
        let text = text.replace("tempo: 130.0", "tempo: 9000.0");
        let project = from_text(&text).unwrap();
        assert_eq!(project.tempo.bpm(), Tempo::MAX);
    }

    #[test]
    fn broken_rules_are_reported() {
        let text = to_text(&plain_demo()).unwrap();

        // A clip that plays pattern 99.
        let broken = text.replacen("pattern: 0,", "pattern: 98,", 1);
        let error = from_text(&broken).unwrap_err();
        assert!(matches!(error, FileError::Invalid(_)), "{error}");
        assert!(error.to_string().contains("pattern 99"), "{error}");

        // A pattern with notes for fewer channels than the project has.
        let mut file: ProjectFile = ron::from_str(&text).unwrap();
        file.patterns[0].notes.pop();
        let error = Project::try_from(file).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("\"Beat\" has notes for 4 channels")
        );
    }

    #[test]
    fn old_files_open_with_only_the_tracks_they_use() {
        // Files used to be saved with sixteen tracks, most of them empty.
        let mut file = ProjectFile::from(&plain_demo());
        file.tracks
            .extend((3..=16).map(crate::playlist::Track::numbered));
        // A track someone renamed is kept, and so are the ones above it.
        file.tracks[5].name = "Keep me".to_string();
        let project = Project::try_from(file).unwrap();
        let names: Vec<&str> = project
            .playlist()
            .tracks()
            .iter()
            .map(|track| track.name.as_str())
            .collect();
        assert_eq!(
            names,
            [
                "Chords", "Drums", "Track 3", "Track 4", "Track 5", "Keep me"
            ]
        );
    }

    #[test]
    fn project_files_end_in_lineup() {
        assert!(is_project(Path::new("song.lineup")));
        assert!(is_project(Path::new("/music/Song.LINEUP")));
        assert!(!is_project(Path::new("song.mid")));
        assert!(!is_project(Path::new("lineup")));
    }

    #[test]
    fn clips_below_the_tracks_are_reported() {
        let mut file = ProjectFile::from(&plain_demo());
        file.clips[0].track = file.tracks.len();
        let error = Project::try_from(file).unwrap_err();
        assert!(error.to_string().contains("does not fit"), "{error}");
    }
}
