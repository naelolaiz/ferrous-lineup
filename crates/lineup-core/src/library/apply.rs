//! Writing styles and rhythms into the patterns of a project.
//!
//! The library's tables know nothing about channels. This module decides
//! where their notes go: chords to a synth called "Keys" (or the channel
//! you have selected), the bass line to a synth called "Bass", and each
//! drum to the channel that plays it. Missing channels are added.
//!
//! It also changes the rhythm of a song that is already written: the
//! drums take the new rhythm, and the chords and bass line are played
//! again (re-"comped") in the way that goes with it, keeping the same
//! chords. That needs the chords, which a pattern does not store, so
//! they are worked out from its notes first (see [`super::listen`]).
//!
//! Nothing here writes on a locked channel (see [`crate::lock`]): its
//! notes and its sound stay as they are, and the rest of the style is
//! written around it.

use crate::TimeSignature;
use crate::general_midi;
use crate::harmony::{Chord, ChordQuality, Scale};
use crate::pattern::NoteEvent;
use crate::pitch::PitchClass;
use crate::project::{Channel, ChannelKind, Drum, Project, SampleSource};
use crate::remeter::remeter;

use super::chart::accompany;
use super::comping::{Bass, Comping};
use super::listen::hear_chords;
use super::rhythm::Rhythm;
use super::style::{STYLES, Style};
use super::voicing::{bass_root, voice, voice_progression};

mod parts;
pub use parts::{style_instruments, write_rhythm_parts, write_style_parts};

/// The names of the channels the library writes to.
pub const KEYS: &str = "Keys";
pub const BASS: &str = "Bass";
pub const LEAD: &str = "Lead";

/// Which parts of a library choice replace the song's current settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StyleParts {
    pub rhythm: bool,
    pub instruments: bool,
    pub time_signature: bool,
    pub progression: bool,
    /// Take the preset's usual tempo where the meter stays the same. Off
    /// by default: the song keeps its tempo, so its own notes last as
    /// long as before.
    pub tempo: bool,
    pub swing: bool,
    /// Copy accompaniment patterns from the preset (or a rhythm's matching
    /// style). Off uses plain accents and keeps the current instruments
    /// and chords.
    pub matching_style: bool,
}

impl Default for StyleParts {
    fn default() -> Self {
        Self {
            rhythm: true,
            instruments: true,
            time_signature: true,
            progression: false,
            tempo: false,
            swing: true,
            matching_style: true,
        }
    }
}

impl StyleParts {
    /// Plain accents always keep the current sounds and chords, for every
    /// preset source: they bring no chord progression.
    pub fn for_preset(mut self) -> Self {
        self.instruments &= self.matching_style;
        self.progression &= self.matching_style;
        self
    }

    /// Rhythm-only choices never supply a chord progression.
    pub fn for_rhythm(mut self) -> Self {
        self.progression = false;
        self.for_preset()
    }

    /// A style that follows the song (see [`Style::follows_the_song`])
    /// keeps the song's time signatures and tempo: these parts are left
    /// out when `follows` is true.
    pub fn following(mut self, follows: bool) -> Self {
        if follows {
            self.time_signature = false;
            self.tempo = false;
        }
        self
    }
}

/// How loud new channels start, so the melody stands out and the chords
/// stay behind it.
const KEYS_VOLUME: f32 = 0.4;
const BASS_VOLUME: f32 = 0.7;
pub(super) const LEAD_VOLUME: f32 = 0.55;

/// How loud a new synth channel called `name` starts. The other voices
/// of a round sing as loud as the first.
pub(crate) fn synth_volume(name: &str) -> f32 {
    match name {
        BASS => BASS_VOLUME,
        LEAD => LEAD_VOLUME,
        _ if name.starts_with("Voice ") => LEAD_VOLUME,
        _ => KEYS_VOLUME,
    }
}

/// Whether a channel can play chords and melodies.
fn plays_notes(channel: &Channel) -> bool {
    matches!(
        channel.kind,
        ChannelKind::Synth | ChannelKind::MidiOut { .. }
    )
}

/// The first synth channel called `name`, or the first sent over MIDI:
/// a "Bass" sent to an external synth is still the bass.
pub(super) fn find_synth(project: &Project, name: &str) -> Option<usize> {
    project
        .channels()
        .iter()
        .position(|channel| plays_notes(channel) && channel.name == name)
}

/// A synth channel called `name`: the first one there is, or a new one.
/// Returns `None` only if the rack is full.
pub fn synth_named(project: &mut Project, name: &str) -> Option<usize> {
    if let Some(index) = find_synth(project, name) {
        return Some(index);
    }
    let volume = synth_volume(name);
    // The chords are played on a piano and the bass line on a bass, by
    // a SoundFont or an external synth (the Lineup Osc takes a sound
    // close to them). Other voices keep the Lineup Osc's own sound.
    let program = match name {
        KEYS => Some(general_midi::ACOUSTIC_GRAND_PIANO),
        BASS => Some(general_midi::FINGER_BASS),
        _ => None,
    };
    project.add_channel(Channel {
        name: name.to_string(),
        kind: ChannelKind::Synth,
        volume,
        muted: false,
        program,
        pan: 0.0,
        reverb_send: 1.0,
        sent_on: None,
        effects: Default::default(),
    })
}

/// The channel chords go to: the `selected` channel if it plays notes
/// and is not the bass or the lead; otherwise the synth called "Keys";
/// otherwise the first other synth; otherwise a new "Keys" synth.
///
/// A locked channel is not picked, as the chords would be left out, but
/// the Keys synth is: a locked Keys keeps the song's chords, so a style
/// does not add its own on another channel.
pub fn chords_channel(project: &mut Project, selected: Option<usize>) -> Option<usize> {
    let free = |(index, channel): (usize, &Channel)| {
        plays_notes(channel)
            && channel.name != BASS
            && channel.name != LEAD
            && !project.is_locked(index)
    };
    if let Some(index) = selected
        && project
            .channels()
            .get(index)
            .is_some_and(|channel| free((index, channel)))
    {
        return Some(index);
    }
    let other = project
        .channels()
        .iter()
        .enumerate()
        .position(|(index, channel)| channel.kind == ChannelKind::Synth && free((index, channel)));
    find_synth(project, KEYS)
        .or(other)
        .or_else(|| synth_named(project, KEYS))
}

/// The channel that plays a built-in drum, added if there is none.
pub fn drum_channel(project: &mut Project, drum: Drum) -> Option<usize> {
    let existing = project
        .channels()
        .iter()
        .position(|channel| channel.kind == ChannelKind::Sampler(SampleSource::Builtin(drum)));
    existing.or_else(|| project.add_channel(Channel::drum(drum)))
}

/// The channels that play built-in drums.
fn drum_channels(project: &Project) -> Vec<usize> {
    project
        .channels()
        .iter()
        .enumerate()
        .filter(|(_, channel)| {
            matches!(channel.kind, ChannelKind::Sampler(SampleSource::Builtin(_)))
        })
        .map(|(index, _)| index)
        .collect()
}

/// How chords and a bass line are played with a rhythm: the comping
/// and bass of the style that goes with it in the library (the one
/// whose drums it is). A rhythm no style uses borrows the playing of a
/// style of its genre in its time signature, or of any style in its
/// time signature, or else plays steady eighth notes.
pub fn feel(rhythm: &Rhythm) -> (Comping, Bass) {
    let same_time = |style: &&Style| style.time == rhythm.time;
    STYLES
        .iter()
        .find(|style| style.rhythm == rhythm.name)
        .or_else(|| {
            STYLES
                .iter()
                .filter(same_time)
                .find(|style| style.genre == rhythm.genre)
        })
        .or_else(|| STYLES.iter().find(same_time))
        .map_or((Comping::Pulse, Bass::Pulse), |style| {
            (style.comping, style.bass)
        })
}

/// Writes a style into pattern `pattern`, in the key of `tonic`: its
/// chords, its bass line, and its length. The project takes the style's
/// scale and time signature. Returns `false` if the pattern does not
/// exist or the rack is full.
pub fn write_style(
    project: &mut Project,
    pattern: usize,
    style: &Style,
    tonic: PitchClass,
    selected: Option<usize>,
) -> bool {
    write_style_played(project, pattern, style, tonic, selected, None)
}

/// Writes a style as [`write_style`] does, but with its chords and bass
/// played the way `rhythm` goes (see [`feel`]), if there is one, instead
/// of the style's own way.
pub fn write_style_played(
    project: &mut Project,
    pattern: usize,
    style: &Style,
    tonic: PitchClass,
    selected: Option<usize>,
    rhythm: Option<&Rhythm>,
) -> bool {
    if project.pattern(pattern).is_none() {
        return false;
    }
    let (comping, bass_line) = rhythm.map_or((style.comping, style.bass), feel);
    let backing = accompany(&style.spans(), tonic, style.time, comping, bass_line);
    let Some(chords) = chords_channel(project, selected) else {
        return false;
    };
    let bass = match bass_line {
        Bass::None => find_synth(project, BASS),
        _ => synth_named(project, BASS),
    };
    // A song with music of its own keeps its key and time signature, and
    // the pattern gets the style's time signature for itself, so its bars
    // are drawn right. An empty song takes the style's.
    let song_has_music = song_has_music(project, pattern);
    if !song_has_music {
        project.scale = style.scale(tonic);
        project.time_signature = style.time;
    }
    let Some(mut target) = project.style_pattern(pattern) else {
        return false;
    };
    target.set_time(song_has_music.then_some(style.time));
    target.set_length(style.length());
    target.set_notes(chords, backing.chords);
    if let Some(bass) = bass {
        target.set_notes(bass, backing.bass);
    }
    true
}

/// Whether the song plays anything but pattern `pattern`: then what the
/// library writes there must not change the song's key or time signature.
pub fn song_has_music(project: &Project, pattern: usize) -> bool {
    project
        .playlist()
        .clips()
        .iter()
        .any(|clip| clip.pattern != pattern)
}

/// Writes a rhythm into pattern `pattern`, repeated to the pattern's
/// length, in bars of the project's time signature. It replaces whatever
/// the built-in drums played there; drums the rhythm needs are added.
/// Returns `false` if the pattern does not exist, the rack is full or
/// every drum is locked.
pub fn write_rhythm(project: &mut Project, pattern: usize, rhythm: &Rhythm) -> bool {
    let Some(length) = project.pattern(pattern).map(|p| p.length()) else {
        return false;
    };
    let mut rows = Vec::new();
    for drum in rhythm.drums() {
        let Some(channel) = drum_channel(project, drum) else {
            return false;
        };
        rows.push((drum, channel));
    }
    let time = project.pattern_time(pattern);
    let old = drum_channels(project);
    let Some(mut target) = project.style_pattern(pattern) else {
        return false;
    };
    let mut wrote = false;
    for channel in old {
        wrote |= target.set_notes(channel, []);
    }
    for (drum, channel) in rows {
        wrote |= target.set_notes(channel, rhythm.notes_in(drum, time, length));
    }
    wrote
}

/// Replaces the drums of pattern `pattern` from step `at` on with
/// `fill`, which lasts one bar.
pub fn write_fill(project: &mut Project, pattern: usize, fill: &Rhythm, at: usize) -> bool {
    let mut rows = Vec::new();
    for drum in fill.drums() {
        let Some(channel) = drum_channel(project, drum) else {
            return false;
        };
        rows.push((drum, channel));
    }
    let bar = project.pattern_time(pattern).steps_per_bar();
    let old = drum_channels(project);
    let Some(mut target) = project.style_pattern(pattern) else {
        return false;
    };
    let length = target.length();
    for channel in old {
        let before: Vec<NoteEvent> = target
            .notes(channel)
            .iter()
            .copied()
            .filter(|note| note.start < at)
            .collect();
        target.set_notes(channel, before);
    }
    for (drum, channel) in rows {
        let mut notes = target.notes(channel).to_vec();
        notes.extend(
            fill.notes(drum, bar, bar)
                .into_iter()
                .map(|note| NoteEvent {
                    start: note.start + at,
                    ..note
                })
                .filter(|note| note.start < length),
        );
        target.set_notes(channel, notes);
    }
    true
}

/// The channel whose notes in pattern `pattern` are its chords, if
/// any: the synth called "Keys" if it plays there; otherwise the first
/// other channel that plays notes there, is not the bass or the lead,
/// and plays three notes at once somewhere, as chords do.
pub(super) fn heard_chords_channel(project: &Project, pattern: usize) -> Option<usize> {
    let notes = project.pattern(pattern)?;
    let plays = |index: usize| !notes.notes(index).is_empty();
    if let Some(keys) = find_synth(project, KEYS)
        && plays(keys)
    {
        return Some(keys);
    }
    project
        .channels()
        .iter()
        .enumerate()
        .filter(|(_, channel)| plays_notes(channel) && channel.name != BASS && channel.name != LEAD)
        .map(|(index, _)| index)
        .find(|&index| plays(index) && plays_chords(notes.notes(index)))
}

/// Whether some three notes sound at the same time.
fn plays_chords(notes: &[NoteEvent]) -> bool {
    notes.iter().any(|note| {
        notes
            .iter()
            .filter(|other| other.start <= note.start && note.start < other.start + other.length)
            .count()
            >= 3
    })
}

/// Plays the chords of pattern `pattern` again with `comping`, and its
/// bass line with `bass`, keeping the chords themselves. The chords are
/// heard from the notes (see [`hear_chords`]); stretches where no chord
/// is heard keep the notes they had. If `bass` is [`Bass::None`] but the
/// pattern had a bass line, the bass holds the roots instead of going
/// quiet; if it had none, a "Bass" synth is added for it.
///
/// Returns `false`, changing nothing, if the pattern has no chords or
/// both its chords and its bass are locked.
pub fn recomp(project: &mut Project, pattern: usize, comping: Comping, bass: Bass) -> bool {
    let Some(chords_channel) = heard_chords_channel(project, pattern) else {
        return false;
    };
    let Some(target) = project.pattern(pattern) else {
        return false;
    };
    let length = target.length();
    let old_chords = target.notes(chords_channel).to_vec();
    let old_bass = find_synth(project, BASS)
        .map(|channel| target.notes(channel).to_vec())
        .unwrap_or_default();
    let time = project.pattern_time(pattern);
    let heard = hear_chords(&old_chords, &old_bass, length, time);
    let played: Vec<_> = heard
        .iter()
        .filter_map(|stretch| stretch.chord.map(|chord| (chord, stretch)))
        .collect();
    if played.is_empty() {
        return false;
    }

    let bass = match bass {
        Bass::None if !old_bass.is_empty() => Bass::Held,
        bass => bass,
    };
    let bass_channel = match bass {
        Bass::None => find_synth(project, BASS),
        _ => synth_named(project, BASS),
    };
    let voicings = voice_progression(&played.iter().map(|(chord, _)| *chord).collect::<Vec<_>>());
    let mut new_chords = Vec::new();
    let mut new_bass = Vec::new();
    for (index, (chord, stretch)) in played.iter().enumerate() {
        let next = played[(index + 1) % played.len()].0;
        new_chords.extend(comping.play(time, &voicings[index], stretch.start, stretch.length));
        new_bass.extend(bass.play(time, *chord, next, stretch.start, stretch.length));
    }
    // Where no chord is heard, the old notes stay.
    for stretch in heard.iter().filter(|stretch| stretch.chord.is_none()) {
        let inside = |note: &&NoteEvent| {
            (stretch.start..stretch.start + stretch.length).contains(&note.start)
        };
        new_chords.extend(old_chords.iter().filter(inside));
        new_bass.extend(old_bass.iter().filter(inside));
    }

    let Some(mut target) = project.style_pattern(pattern) else {
        return false;
    };
    let chords = target.set_notes(chords_channel, new_chords);
    let bass = bass_channel.is_some_and(|channel| target.set_notes(channel, new_bass));
    chords || bass
}

/// What [`use_rhythm`], [`use_style`] or [`use_fill`] changed.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RhythmChange {
    /// How many patterns got the new drums.
    pub drums: usize,
    /// How many patterns had their chords and bass played again.
    pub harmony: usize,
    /// The time signature the song was put into, bar for bar, to play in
    /// the new rhythm (see [`remeter`](crate::remeter)), if it was in
    /// another.
    pub time: Option<TimeSignature>,
}

/// The drum notes of pattern `pattern` from step `from` for `steps`
/// steps, by channel, starting from 0.
pub(super) fn drums_in(
    project: &Project,
    pattern: usize,
    from: usize,
    steps: usize,
) -> Vec<(usize, NoteEvent)> {
    let Some(target) = project.pattern(pattern) else {
        return Vec::new();
    };
    let mut notes: Vec<(usize, NoteEvent)> = drum_channels(project)
        .into_iter()
        .flat_map(|channel| {
            target
                .notes(channel)
                .iter()
                .filter(|note| (from..from + steps).contains(&note.start))
                .map(move |note| {
                    (
                        channel,
                        NoteEvent {
                            start: note.start - from,
                            ..*note
                        },
                    )
                })
        })
        .collect();
    notes.sort_by_key(|(channel, note)| (*channel, note.start, note.pitch));
    notes
}

/// Changes the rhythm of the whole song to `rhythm`, the way a band
/// changes the feel of a song it knows: every pattern with drums gets the
/// new rhythm (with the rhythm's fill where the old drums had a fill in
/// the last bar), and every pattern with chords has them played the way
/// the rhythm goes (see [`feel`] and [`recomp`]), keeping its chords.
/// If no pattern has drums, the drums go with the chords.
///
/// A pattern whose notes all start on its first step, such as a final
/// held chord with a crash, is an ending, and is left as it is. So is the
/// melody.
///
/// A rhythm in another time signature first puts the whole song in it,
/// bar for bar ([`remeter`]): a waltz cut into 4/4 bars would lose its
/// "one" every bar, so a 4/4 song played as a waltz becomes a song in
/// 3/4, each bar's notes keeping their place in the bar.
pub fn use_rhythm(project: &mut Project, rhythm: &Rhythm) -> RhythmChange {
    use_rhythm_parts(project, rhythm, StyleParts::default())
}

pub fn use_rhythm_parts(project: &mut Project, rhythm: &Rhythm, parts: StyleParts) -> RhythmChange {
    let parts = parts.following(rhythm.follows_the_meter);
    let (comping, bass) = if parts.matching_style {
        feel(rhythm)
    } else {
        (Comping::Accents, Bass::Accents)
    };
    let change = play_song(project, Some(rhythm), rhythm.time, comping, bass, parts);
    if parts.instruments
        && parts.matching_style
        && let Some(style) = STYLES.iter().find(|style| style.rhythm == rhythm.name)
    {
        style_instruments(project, style, None);
    }
    change
}

/// Plays the whole song the way `style` goes: its rhythm on the drums,
/// and its way of playing the chords and the bass. The song keeps its own
/// chords, melody and key, as with [`use_rhythm`], and takes the style's
/// time signature the same way. A style without a rhythm leaves the drums
/// as they are.
pub fn use_style(project: &mut Project, style: &Style) -> RhythmChange {
    use_style_parts(project, style, StyleParts::default())
}

pub fn use_style_parts(project: &mut Project, style: &Style, parts: StyleParts) -> RhythmChange {
    let parts = parts.for_preset().following(style.follows_the_song());
    let (comping, bass) = if parts.matching_style {
        (style.comping, style.bass)
    } else {
        (Comping::Accents, Bass::Accents)
    };
    let mut change = play_song(
        project,
        Rhythm::named(style.rhythm),
        style.time,
        comping,
        bass,
        parts,
    );
    if parts.progression {
        for index in 0..project.patterns().len() {
            if !is_the_bands(project, index) && parts::progression(project, index, style) {
                change.harmony += 1;
            }
        }
        project.write_heard_chords();
    }
    if parts.instruments {
        style_instruments(project, style, None);
    }
    change
}

/// Puts `fill` in the last bar of every pattern with drums that is at
/// least two bars long, in place of the drums there. If no pattern has
/// drums, the fill goes with the chords, as with [`use_rhythm`]. Endings
/// are left as they are. Only [`RhythmChange::drums`] is counted.
pub fn use_fill(project: &mut Project, fill: &Rhythm) -> RhythmChange {
    let patterns = project.patterns().len();
    let length =
        |project: &Project, pattern: usize| project.pattern(pattern).map_or(0, |p| p.length());
    let has_drums = |project: &Project, pattern: usize| {
        !drums_in(project, pattern, 0, length(project, pattern)).is_empty()
    };
    let any_drums = (0..patterns).any(|pattern| has_drums(project, pattern));
    let mut change = RhythmChange::default();
    for pattern in 0..patterns {
        let length = length(project, pattern);
        let bar = project.pattern_time(pattern).steps_per_bar();
        if length < 2 * bar || is_ending(project, pattern) || is_the_bands(project, pattern) {
            continue;
        }
        let takes_it = if any_drums {
            has_drums(project, pattern)
        } else {
            heard_chords_channel(project, pattern).is_some()
        };
        if !takes_it {
            continue;
        }
        let before = project.pattern(pattern).cloned();
        if write_fill(project, pattern, fill, length - bar)
            && project.pattern(pattern) != before.as_ref()
        {
            change.drums += 1;
        }
    }
    change
}

/// Whether the band wrote pattern `pattern` (see [`crate::band`]).
fn is_the_bands(project: &Project, pattern: usize) -> bool {
    project
        .pattern(pattern)
        .is_some_and(|pattern| pattern.player.is_some())
}

/// Whether every note of pattern `pattern` starts on its first step, as
/// in a final held chord with a crash.
pub(super) fn is_ending(project: &Project, pattern: usize) -> bool {
    project.pattern(pattern).is_some_and(|target| {
        (0..target.rows())
            .flat_map(|channel| target.notes(channel))
            .all(|note| note.start == 0)
    })
}

/// Puts the song in `time`, then gives every pattern of the song the
/// drums of `rhythm` (if any) and its swing, and plays its chords and
/// bass with `comping` and `bass`: the work of [`use_rhythm`] and
/// [`use_style`].
fn play_song(
    project: &mut Project,
    rhythm: Option<&Rhythm>,
    time: TimeSignature,
    comping: Comping,
    bass: Bass,
    parts: StyleParts,
) -> RhythmChange {
    let mut change = RhythmChange::default();
    if parts.time_signature && remeter(project, time) {
        change.time = Some(time);
    }
    let has_drums = |project: &Project, pattern: usize| {
        let length = project.pattern(pattern).map_or(0, |p| p.length());
        !drums_in(project, pattern, 0, length).is_empty()
    };
    let patterns = project.patterns().len();
    let any_drums = (0..patterns).any(|pattern| has_drums(project, pattern));
    for pattern in 0..patterns {
        let Some(length) = project.pattern(pattern).map(|p| p.length()) else {
            continue;
        };
        let bar = project.pattern_time(pattern).steps_per_bar();
        // The band's parts follow the chord sheet and the band's style.
        if is_ending(project, pattern) || is_the_bands(project, pattern) {
            continue;
        }
        let drums = has_drums(project, pattern);
        // A fill is a last bar that plays like no bar before it.
        let fill = length >= 2 * bar && {
            let last = drums_in(project, pattern, length - bar, bar);
            (0..length - bar)
                .step_by(bar)
                .all(|start| drums_in(project, pattern, start, bar) != last)
        };
        let has_chords = heard_chords_channel(project, pattern).is_some();
        if parts.rhythm && recomp(project, pattern, comping, bass) {
            change.harmony += 1;
        } else if !drums && !has_chords {
            continue;
        }
        if let Some(rhythm) = rhythm
            && parts.rhythm
            && (drums || !any_drums)
            && write_rhythm(project, pattern, rhythm)
        {
            if fill && let Some(fill) = rhythm.fill() {
                write_fill(project, pattern, fill, length - bar);
            }
            change.drums += 1;
        }
    }
    // The song now swings as much as the rhythm it plays.
    if let Some(rhythm) = rhythm
        && parts.swing
    {
        project.swing = rhythm.swing;
    }
    change
}

/// The chord a piece ends on: the triad on the tonic of `scale`, or a
/// major chord for scales without seven notes.
pub fn home_chord(scale: Scale) -> Chord {
    scale
        .triad(0)
        .unwrap_or(Chord::new(scale.root, ChordQuality::Major))
}

/// Writes a final chord into pattern `pattern`: `chord` held for the
/// whole pattern on the chords channel, its root on the bass.
pub fn write_final_chord(project: &mut Project, pattern: usize, chord: Chord) -> bool {
    let Some(chords) = chords_channel(project, None) else {
        return false;
    };
    let bass = find_synth(project, BASS);
    let Some(mut target) = project.style_pattern(pattern) else {
        return false;
    };
    let length = target.length();
    let held = |pitch| NoteEvent {
        velocity: 90,
        ..NoteEvent::new(pitch, 0, length)
    };
    target.set_notes(chords, voice(chord, None).into_iter().map(held));
    if let Some(bass) = bass {
        target.set_notes(bass, [held(bass_root(chord))]);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::{FILLS, RHYTHMS, STYLES};

    #[test]
    fn a_song_with_music_keeps_its_key_and_time() {
        use crate::TimeSignature;
        let waltz = STYLES.iter().find(|style| style.time.beats == 3).unwrap();
        // The demo song plays its chords and drums: a waltz tried next to
        // it is in 3/4 on its own, and the song stays as it was.
        let mut project = Project::demo();
        let before = (project.scale, project.time_signature);
        let tried = project.add_pattern().unwrap();
        assert!(write_style(&mut project, tried, waltz, PitchClass::C, None));
        assert_eq!((project.scale, project.time_signature), before);
        assert_eq!(project.pattern(tried).unwrap().time, Some(waltz.time));
        assert_eq!(project.pattern_time(tried), waltz.time);
        assert_eq!(project.pattern_time(0), TimeSignature::default());
        // An empty song takes the style's key and time signature.
        let mut empty = Project::empty();
        assert!(write_style(&mut empty, 0, waltz, PitchClass::C, None));
        assert_eq!(empty.time_signature, waltz.time);
        assert_eq!(empty.pattern(0).unwrap().time, None);
        assert_eq!(empty.scale, waltz.scale(PitchClass::C));
    }

    #[test]
    fn styles_go_to_keys_and_bass() {
        let mut project = Project::template();
        let style = Style::named("Four chords").unwrap();
        assert!(write_style(&mut project, 0, style, PitchClass::G, Some(0)));
        // The kick (channel 0) cannot play chords, so the template's
        // Lineup Osc gets them, and a Bass synth is added.
        let names: Vec<&str> = project.channels().iter().map(|c| c.name.as_str()).collect();
        assert_eq!(
            names,
            ["Kick", "Clap", "Hat", "Snare", "Lineup Osc", "Bass"]
        );
        let pattern = project.pattern(0).unwrap();
        assert_eq!(pattern.length(), 64);
        assert!(!pattern.notes(4).is_empty() && !pattern.notes(5).is_empty());
        assert_eq!(
            project.scale,
            Scale::new(PitchClass::G, crate::ScaleKind::Major)
        );

        // Applying it again finds the same channels.
        assert!(write_style(&mut project, 0, style, PitchClass::C, Some(5)));
        assert_eq!(
            project.channels().len(),
            6,
            "the bass is not used for chords"
        );

        // A bass and a kick sent over MIDI are still the bass and the
        // kick: the style plays on them, not on new channels.
        project.channel_mut(5).unwrap().send_over_midi(1);
        project.channel_mut(0).unwrap().send_over_midi(9);
        assert!(write_style(&mut project, 0, style, PitchClass::C, Some(4)));
        assert_eq!(project.channels().len(), 6);
        assert_eq!(
            project.channels()[5].kind,
            ChannelKind::MidiOut { channel: 1 }
        );
        assert_eq!(project.channels()[0].sent_over_midi(), Some(9));
        assert!(!project.pattern(0).unwrap().notes(5).is_empty());
        let beat = Rhythm::named("Pop beat").unwrap();
        assert!(write_rhythm(&mut project, 0, beat));
        assert_eq!(project.channels().len(), 6);
        assert!(!project.pattern(0).unwrap().notes(0).is_empty());
    }

    #[test]
    fn chords_go_around_a_locked_channel() {
        // The demo's only synth plays its chords. Locked and selected, it
        // keeps them, and a style tried in a new pattern plays its chords
        // on a new Keys.
        let mut project = Project::demo();
        let synth = 4;
        let style = &STYLES[0];
        project.set_locked(synth, true);
        let tried = project.add_pattern().unwrap();
        assert!(write_style(
            &mut project,
            tried,
            style,
            PitchClass::C,
            Some(synth)
        ));
        let keys = find_synth(&project, KEYS).unwrap();
        assert_ne!(keys, synth);
        assert!(!project.pattern(tried).unwrap().notes(keys).is_empty());
        assert!(project.pattern(tried).unwrap().notes(synth).is_empty());
        // A locked Keys is where the chords go, and keeps the song's.
        project.set_locked(keys, true);
        let before = project.pattern(tried).unwrap().notes(keys).to_vec();
        assert!(write_style(
            &mut project,
            tried,
            &STYLES[1],
            PitchClass::C,
            None
        ));
        assert_eq!(chords_channel(&mut project, None), Some(keys));
        assert_eq!(project.pattern(tried).unwrap().notes(keys), before);
    }

    #[test]
    fn rhythms_replace_the_drums_and_add_missing_ones() {
        let mut project = Project::demo();
        let salsa = Rhythm::named("Salsa").unwrap();
        assert!(write_rhythm(&mut project, 0, salsa));
        let pattern = project.pattern(0).unwrap();
        assert!(pattern.notes(0).is_empty(), "the old kick is gone");
        let conga = drum_channel(&mut project, Drum::Conga).unwrap();
        assert_eq!(conga, 7, "rim, cowbell and conga were added");
        assert!(!project.pattern(0).unwrap().notes(conga).is_empty());
    }

    #[test]
    fn fills_replace_the_last_bar() {
        let mut project = Project::template();
        project.pattern_mut(0).unwrap().set_length(32);
        let beat = Rhythm::named("Pop beat").unwrap();
        write_rhythm(&mut project, 0, beat);
        write_fill(&mut project, 0, beat.fill().unwrap(), 16);
        let snare = drum_channel(&mut project, Drum::Snare).unwrap();
        let starts: Vec<usize> = project
            .pattern(0)
            .unwrap()
            .notes(snare)
            .iter()
            .map(|n| n.start)
            .collect();
        assert_eq!(starts, [4, 12, 18, 20, 22, 24, 25, 26, 27, 28, 29, 30, 31]);
    }

    #[test]
    fn every_style_and_rhythm_can_be_written() {
        let mut project = Project::template();
        for style in STYLES {
            assert!(
                write_style(&mut project, 0, style, PitchClass::E, None),
                "{}",
                style.name
            );
        }
        for rhythm in RHYTHMS {
            assert!(write_rhythm(&mut project, 0, rhythm), "{}", rhythm.name);
        }
        assert!(project.channels().len() <= crate::MAX_CHANNELS);
    }

    #[test]
    fn rhythms_bring_the_playing_of_their_style() {
        let feel_of = |name| feel(Rhythm::named(name).unwrap());
        assert_eq!(feel_of("Salsa"), (Comping::Montuno, Bass::Tumbao));
        assert_eq!(feel_of("One drop").1, Bass::Reggae);
        // No style uses the march: it plays like a classical style in 4/4.
        let march = STYLES
            .iter()
            .find(|s| s.genre == crate::library::Genre::Classical && s.time.is_common())
            .unwrap();
        assert_eq!(feel_of("March"), (march.comping, march.bass));
        for rhythm in RHYTHMS {
            let (comping, _) = feel(rhythm);
            let three = rhythm.time.steps_per_bar() == 12;
            assert!(
                !three
                    || matches!(
                        comping,
                        Comping::Waltz | Comping::Sway | Comping::Arp3 | Comping::Held
                    ),
                "{} is in three but plays {comping:?}",
                rhythm.name
            );
        }
    }

    #[test]
    fn a_style_can_be_played_the_way_a_rhythm_goes() {
        let style = Style::named("Four chords").unwrap();
        let salsa = Rhythm::named("Salsa").unwrap();
        let mut project = Project::template();
        assert!(write_style_played(
            &mut project,
            0,
            style,
            PitchClass::C,
            None,
            Some(salsa)
        ));
        let keys = chords_channel(&mut project, None).unwrap();
        let expected = accompany(
            &style.spans(),
            PitchClass::C,
            style.time,
            Comping::Montuno,
            Bass::Tumbao,
        );
        assert_eq!(project.pattern(0).unwrap().notes(keys), expected.chords);
    }

    /// Chords written in one way and played again in another come out
    /// as if they had been written that way: the same chords, voiced the
    /// same, in the new rhythm.
    #[test]
    fn recomping_keeps_the_chords() {
        let style = Style::named("Pop punk").unwrap();
        let mut project = Project::template();
        write_style(&mut project, 0, style, PitchClass::D, None);
        let keys = chords_channel(&mut project, None).unwrap();
        let bass = find_synth(&project, BASS).unwrap();
        assert!(recomp(&mut project, 0, Comping::Bossa, Bass::Bossa));
        let expected = accompany(
            &style.spans(),
            PitchClass::D,
            style.time,
            Comping::Bossa,
            Bass::Bossa,
        );
        let mut chords = project.pattern(0).unwrap().notes(keys).to_vec();
        let mut wanted = expected.chords;
        let key = |n: &NoteEvent| (n.start, n.pitch);
        chords.sort_by_key(key);
        wanted.sort_by_key(key);
        assert_eq!(chords, wanted);
        assert_eq!(project.pattern(0).unwrap().notes(bass), expected.bass);
    }

    #[test]
    fn patterns_without_chords_are_not_recomped() {
        let mut project = Project::template();
        let before = project.clone();
        assert!(!recomp(&mut project, 0, Comping::Pulse, Bass::Pulse));
        assert_eq!(project, before);
    }

    #[test]
    fn a_whole_song_takes_a_new_rhythm() {
        let song = crate::library::SONGS
            .iter()
            .find(|song| !song.rhythm.is_empty() && song.parts.iter().all(|p| p.left.is_empty()))
            .unwrap();
        let mut project = super::super::demo::song_demo(song);
        let before = project.clone();
        let lead = find_synth(&project, LEAD).unwrap();
        let keys = chords_channel(&mut project, None).unwrap();
        let rhythm = RHYTHMS
            .iter()
            .find(|r| {
                r.time == project.time_signature && r.name != song.rhythm && r.fill().is_some()
            })
            .unwrap();
        let change = use_rhythm(&mut project, rhythm);
        assert!(change.drums > 0 && change.harmony > 0, "{change:?}");
        let (comping, _) = feel(rhythm);
        for (index, (old, new)) in before.patterns().iter().zip(project.patterns()).enumerate() {
            assert_eq!(old.notes(lead), new.notes(lead), "the melody stays");
            assert_eq!(old.length(), new.length());
            let ending = (0..old.rows())
                .flat_map(|channel| old.notes(channel))
                .all(|note| note.start == 0);
            if ending {
                for channel in 0..new.rows() {
                    assert_eq!(old.notes(channel), new.notes(channel), "the ending stays");
                }
            } else if !old.notes(keys).is_empty() && comping != song.comping {
                assert_ne!(old.notes(keys), new.notes(keys), "pattern {index}");
            }
        }
        assert_eq!(project.time_signature, before.time_signature);
    }

    #[test]
    fn songs_without_drums_get_them_with_the_chords() {
        let mut project = Project::template();
        write_style(
            &mut project,
            0,
            Style::named("Four chords").unwrap(),
            PitchClass::C,
            None,
        );
        let pop = Rhythm::named("Pop beat").unwrap();
        let change = use_rhythm(&mut project, pop);
        assert_eq!(
            change,
            RhythmChange {
                drums: 1,
                harmony: 1,
                time: None,
            }
        );
        let snare = drum_channel(&mut project, Drum::Snare).unwrap();
        assert!(!project.pattern(0).unwrap().notes(snare).is_empty());
    }

    #[test]
    fn a_whole_song_takes_a_style() {
        let song = crate::library::SONGS
            .iter()
            .find(|song| song.name.starts_with("London Bridge"))
            .unwrap();
        let mut project = super::super::demo::song_demo(song);
        let before = project.clone();
        let lead = find_synth(&project, LEAD).unwrap();
        let style = Style::named("House piano").unwrap();
        let change = use_style(&mut project, style);
        assert!(change.drums > 0 && change.harmony > 0, "{change:?}");
        let kick = drum_channel(&mut project, Drum::Kick).unwrap();
        for (old, new) in before.patterns().iter().zip(project.patterns()) {
            assert_eq!(old.notes(lead), new.notes(lead), "the melody stays");
            assert_eq!(old.length(), new.length());
        }
        // Four on the floor: a kick on every beat of the drum patterns.
        let drums = project.pattern(1).unwrap();
        let beats: Vec<usize> = drums.notes(kick).iter().map(|note| note.start).collect();
        assert!(
            (0..drums.length())
                .step_by(4)
                .all(|beat| beats.contains(&beat))
        );
        assert_eq!(project.time_signature, before.time_signature);
        assert_eq!(project.scale, before.scale);
    }

    #[test]
    fn a_fill_goes_in_the_last_bar_of_the_drums() {
        let mut project = super::super::demo::style_demo(Style::named("Four chords").unwrap());
        let before = project.clone();
        let fill = FILLS[0];
        let change = use_fill(&mut project, &fill);
        assert!(change.drums > 0 && change.harmony == 0, "{change:?}");
        let bar = project.time_signature.steps_per_bar();
        for (old, new) in before.patterns().iter().zip(project.patterns()) {
            for channel in 0..old.rows() {
                let early = |notes: &[NoteEvent]| {
                    notes
                        .iter()
                        .filter(|note| note.start < old.length().saturating_sub(bar))
                        .copied()
                        .collect::<Vec<_>>()
                };
                assert_eq!(early(old.notes(channel)), early(new.notes(channel)));
            }
        }
    }

    /// While the song plays, a click on any style, rhythm or fill in the
    /// library changes the song. So each of them must find something to
    /// change in every demo; otherwise the click would do nothing.
    #[test]
    fn every_demo_takes_every_kind_of_library_click() {
        let demos: Vec<Project> = crate::library::SONGS
            .iter()
            .map(super::super::demo::song_demo)
            .chain(STYLES.iter().map(super::super::demo::style_demo))
            .collect();
        // Every style, rhythm and fill is tried on at least one demo, and
        // every demo gets one of each.
        let tries = demos.len().max(STYLES.len()).max(RHYTHMS.len());
        for index in 0..tries {
            let demo = &demos[index % demos.len()];
            let style = &STYLES[index % STYLES.len()];
            let rhythm = &RHYTHMS[index % RHYTHMS.len()];
            let fill = &FILLS[index % FILLS.len()];
            let mut project = demo.clone();
            assert_ne!(
                use_style(&mut project, style),
                RhythmChange::default(),
                "{}",
                style.name
            );
            let mut project = demo.clone();
            assert_ne!(
                use_rhythm(&mut project, rhythm),
                RhythmChange::default(),
                "{}",
                rhythm.name
            );
            // A fill that is already in every place a fill can go changes
            // nothing, so a second fill is tried before giving up.
            let other = &FILLS[(index + 1) % FILLS.len()];
            let fills =
                |fill: &Rhythm| use_fill(&mut demo.clone(), fill) != RhythmChange::default();
            assert!(
                fills(fill) || fills(other),
                "{} or {}",
                fill.name,
                other.name
            );
        }
    }

    /// A 4/4 song played with a waltz rhythm becomes a song in 3/4: as
    /// many bars as before, each with the same melody notes in the same
    /// order, and the waltz's own drums, whole, in every bar.
    #[test]
    fn a_waltz_puts_the_song_in_three_four() {
        use crate::TimeSignature;
        let song = crate::library::SONGS
            .iter()
            .find(|song| song.name == "Ode to Joy")
            .unwrap();
        let demo = super::super::demo::song_demo(song);
        assert_eq!(demo.time_signature, TimeSignature::COMMON);
        let waltz = Rhythm::named("Jazz waltz").unwrap();
        let mut project = demo.clone();
        let change = use_rhythm(&mut project, waltz);
        let three_four = TimeSignature::new(3, 4);
        assert_eq!(change.time, Some(three_four));
        assert!(change.drums > 0 && change.harmony > 0, "{change:?}");
        assert_eq!(project.time_signature, three_four);
        assert_eq!(project.song_length() / 12, demo.song_length() / 16);
        // Every bar lasts as long as before: the song keeps its pace.
        let seconds = |p: &Project| p.played_seconds(&p.arrangement());
        assert_eq!(project.tempo.bpm(), demo.tempo.bpm() * 0.75);
        assert!((seconds(&project) - seconds(&demo)).abs() < 2e-3 * seconds(&demo));
        let melody = |project: &Project, bar: usize, steps: usize| {
            let mut notes: Vec<(usize, crate::Note)> = Vec::new();
            for clip in project.playlist().clips() {
                let pattern = project.pattern(clip.pattern).unwrap();
                let channel = (0..pattern.rows()).find(|&c| project.channels()[c].name == LEAD);
                let Some(channel) = channel else { continue };
                for note in pattern.notes(channel) {
                    let at = clip.start + note.start;
                    if (bar * steps..(bar + 1) * steps).contains(&at) {
                        notes.push((note.start_tick() * 16 / steps, note.pitch));
                    }
                }
            }
            notes.sort();
            notes
        };
        assert!(!melody(&demo, 0, 16).is_empty());
        for bar in 0..demo.song_length() / 16 {
            assert_eq!(
                melody(&demo, bar, 16),
                melody(&project, bar, 12),
                "bar {bar}"
            );
        }
        // Back to a 4/4 rhythm, the bars are sixteen steps again.
        let change = use_rhythm(&mut project, Rhythm::named("Pop beat").unwrap());
        assert_eq!(change.time, Some(TimeSignature::COMMON));
        assert_eq!(project.tempo.bpm(), demo.tempo.bpm());
        for bar in 0..demo.song_length() / 16 {
            assert_eq!(
                melody(&demo, bar, 16),
                melody(&project, bar, 16),
                "bar {bar}"
            );
        }
    }

    #[test]
    fn a_rhythm_change_is_undone_in_one_step() {
        let mut project = super::super::demo::style_demo(Style::named("Four chords").unwrap());
        let mut history = crate::History::new(10);
        let before = project.clone();
        use_rhythm(&mut project, Rhythm::named("Reggaeton").unwrap());
        assert_ne!(project, before);
        history.record("change the rhythm", before.clone());
        history.undo(&mut project);
        assert_eq!(project, before);
    }
}
