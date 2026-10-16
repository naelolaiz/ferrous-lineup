//! Writing styles and rhythms into the patterns of a project.
//!
//! The library's tables know nothing about channels. This module decides
//! where their notes go: chords to a synth called "Keys" (or the channel
//! you have selected), the bass line to a synth called "Bass", and each
//! drum to the channel that plays it. Missing channels are added.

use crate::harmony::{Chord, ChordQuality, Scale};
use crate::pattern::NoteEvent;
use crate::pitch::PitchClass;
use crate::project::{Channel, ChannelKind, Drum, Project, SampleSource};

use super::comping::Bass;
use super::rhythm::Rhythm;
use super::style::Style;
use super::voicing::{bass_root, voice};

/// The names of the channels the library writes to.
pub const KEYS: &str = "Keys";
pub const BASS: &str = "Bass";
pub const LEAD: &str = "Lead";

/// How loud new channels start, so the melody stands out and the chords
/// stay behind it.
const KEYS_VOLUME: f32 = 0.4;
const BASS_VOLUME: f32 = 0.7;
const LEAD_VOLUME: f32 = 0.55;

/// Whether a channel can play chords and melodies.
fn plays_notes(channel: &Channel) -> bool {
    matches!(
        channel.kind,
        ChannelKind::Synth | ChannelKind::MidiOut { .. }
    )
}

/// The first synth channel called `name`.
fn find_synth(project: &Project, name: &str) -> Option<usize> {
    project
        .channels()
        .iter()
        .position(|channel| channel.kind == ChannelKind::Synth && channel.name == name)
}

/// A synth channel called `name`: the first one there is, or a new one.
/// Returns `None` only if the rack is full.
pub fn synth_named(project: &mut Project, name: &str) -> Option<usize> {
    if let Some(index) = find_synth(project, name) {
        return Some(index);
    }
    let volume = match name {
        BASS => BASS_VOLUME,
        LEAD => LEAD_VOLUME,
        _ => KEYS_VOLUME,
    };
    project.add_channel(Channel {
        name: name.to_string(),
        kind: ChannelKind::Synth,
        volume,
        muted: false,
    })
}

/// The channel chords go to: the `selected` channel if it plays notes
/// and is not the bass or the lead; otherwise the synth called "Keys";
/// otherwise the first other synth; otherwise a new "Keys" synth.
pub fn chords_channel(project: &mut Project, selected: Option<usize>) -> Option<usize> {
    let free =
        |channel: &Channel| plays_notes(channel) && channel.name != BASS && channel.name != LEAD;
    if let Some(index) = selected
        && project.channels().get(index).is_some_and(free)
    {
        return Some(index);
    }
    find_synth(project, KEYS)
        .or_else(|| {
            project
                .channels()
                .iter()
                .position(|channel| channel.kind == ChannelKind::Synth && free(channel))
        })
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
    if project.pattern(pattern).is_none() {
        return false;
    }
    let backing = style.render(tonic);
    let Some(chords) = chords_channel(project, selected) else {
        return false;
    };
    let bass = match style.bass {
        Bass::None => find_synth(project, BASS),
        _ => synth_named(project, BASS),
    };
    project.scale = style.scale(tonic);
    project.time_signature = style.time;
    let Some(target) = project.pattern_mut(pattern) else {
        return false;
    };
    target.set_length(style.length());
    target.set_notes(chords, backing.chords);
    if let Some(bass) = bass {
        target.set_notes(bass, backing.bass);
    }
    true
}

/// Writes a rhythm into pattern `pattern`, repeated to the pattern's
/// length, in bars of the project's time signature. It replaces whatever
/// the built-in drums played there; drums the rhythm needs are added.
/// Returns `false` if the pattern does not exist or the rack is full.
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
    let bar = project.time_signature.steps_per_bar();
    let old = drum_channels(project);
    let Some(target) = project.pattern_mut(pattern) else {
        return false;
    };
    for channel in old {
        target.set_notes(channel, []);
    }
    for (drum, channel) in rows {
        target.set_notes(channel, rhythm.notes(drum, bar, length));
    }
    true
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
    let bar = project.time_signature.steps_per_bar();
    let old = drum_channels(project);
    let Some(target) = project.pattern_mut(pattern) else {
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
    let Some(target) = project.pattern_mut(pattern) else {
        return false;
    };
    let length = target.length();
    let held = |pitch| NoteEvent {
        pitch,
        start: 0,
        length,
        velocity: 90,
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
    use crate::library::{RHYTHMS, STYLES};

    #[test]
    fn styles_go_to_keys_and_bass() {
        let mut project = Project::template();
        let style = Style::named("Four chords").unwrap();
        assert!(write_style(&mut project, 0, style, PitchClass::G, Some(0)));
        // The kick (channel 0) cannot play chords, so the template's
        // Loom Osc gets them, and a Bass synth is added.
        let names: Vec<&str> = project.channels().iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, ["Kick", "Clap", "Hat", "Snare", "Loom Osc", "Bass"]);
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
}
