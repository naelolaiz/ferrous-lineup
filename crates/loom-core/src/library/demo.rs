//! Demo projects, arranged from the library.
//!
//! A pattern is a loop; a song is loops arranged in time. These
//! functions do the arranging the way a producer would: the harmony
//! alone first, then the drums come in, a fill every few bars, and a
//! final chord with a crash at the end. Every demo is an ordinary
//! project, to take apart and change.

use crate::playlist::Clip;
use crate::project::Project;
use crate::tempo::Tempo;

use super::apply::{self, LEAD};
use super::chart::accompany;
use super::comping::Bass;
use super::rhythm::{FILLS, Rhythm};
use super::song::{Part, Song};
use super::style::Style;
use crate::pattern::NoteEvent;

/// About how many bars a style demo plays with the drums.
const DEMO_BARS: usize = 32;

/// The playlist tracks the demos use.
const SONG_TRACK: usize = 0;
const DRUM_TRACK: usize = 1;

/// Names the first two playlist tracks.
fn name_tracks(project: &mut Project, first: &str, second: &str) {
    if let Some(track) = project.track_mut(SONG_TRACK) {
        track.name = first.to_string();
    }
    if let Some(track) = project.track_mut(DRUM_TRACK) {
        track.name = second.to_string();
    }
}

/// Adds a pattern called `name`, `length` steps long. Falls back to the
/// first pattern if there is no room, which never happens with the
/// handful of patterns a demo has.
fn new_pattern(project: &mut Project, name: &str, length: usize) -> usize {
    let index = project.add_pattern().unwrap_or(0);
    if let Some(pattern) = project.pattern_mut(index) {
        pattern.name = name.to_string();
        pattern.set_length(length);
    }
    index
}

/// A drum pattern: `rhythm` over `length` steps, ending with its fill
/// in the last bar if `with_fill` and the rhythm has one.
fn drum_pattern(
    project: &mut Project,
    name: &str,
    length: usize,
    rhythm: &Rhythm,
    with_fill: bool,
) -> usize {
    let index = new_pattern(project, name, length);
    apply::write_rhythm(project, index, rhythm);
    let bar = project.time_signature.steps_per_bar();
    if with_fill
        && length >= 2 * bar
        && let Some(fill) = rhythm.fill()
    {
        apply::write_fill(project, index, fill, length - bar);
    }
    index
}

/// The last bar: the home chord held, with a crash if there are drums.
fn ending(project: &mut Project, with_drums: bool) -> usize {
    let bar = project.time_signature.steps_per_bar();
    let index = new_pattern(project, "Ending", bar);
    let chord = apply::home_chord(project.scale);
    apply::write_final_chord(project, index, chord);
    if with_drums && let Some(crash) = FILLS.iter().find(|fill| fill.name == "Crash ending") {
        apply::write_rhythm(project, index, crash);
    }
    index
}

/// A demo song built from a style: its harmony, its rhythm, fills and
/// an ending, in the style's own key, tempo and time signature.
pub fn style_demo(style: &Style) -> Project {
    let tonic = style.home_tonic();
    let mut project = Project::empty();
    project.tempo = Tempo::new(style.tempo);
    project.time_signature = style.time;
    project.scale = style.scale(tonic);
    if let Some(pattern) = project.pattern_mut(0) {
        pattern.name = "Harmony".to_string();
    }
    let harmony = 0;
    apply::write_style(&mut project, harmony, style, tonic, None);
    let length = style.length();
    let bars = style.bars();
    let passes = (DEMO_BARS / bars).clamp(2, 6);

    let rhythm = Rhythm::named(style.rhythm);
    let drums = rhythm.map(|rhythm| {
        let plain = drum_pattern(&mut project, "Drums", length, rhythm, false);
        let fill = drum_pattern(&mut project, "Drums and fill", length, rhythm, true);
        (plain, fill)
    });
    let end = ending(&mut project, drums.is_some());
    name_tracks(&mut project, "Harmony", "Drums");

    let mut at = 0;
    // A short progression plays once alone, so you hear the chords
    // before the drums come in.
    if drums.is_some() && bars <= 4 {
        project.add_clip(Clip::new(harmony, SONG_TRACK, at));
        at += length;
    }
    for pass in 0..passes {
        project.add_clip(Clip::new(harmony, SONG_TRACK, at));
        if let Some((plain, fill)) = drums {
            let last = pass + 1 == passes;
            let pattern = if pass % 2 == 1 || last { fill } else { plain };
            project.add_clip(Clip::new(pattern, DRUM_TRACK, at));
        }
        at += length;
    }
    project.add_clip(Clip::new(end, SONG_TRACK, at));
    project.remove_unused_tracks();
    project
}

/// The notes of a part's melody, one copy per voice, inside the part.
fn melody_notes(part: &Part, bar: usize, length: usize) -> Vec<NoteEvent> {
    let melody = part.melody();
    let mut notes = Vec::new();
    for &voice in part.voices {
        let offset = voice as isize * bar as isize;
        for note in &melody.notes {
            let start = note.start as isize + offset;
            if (0..length as isize).contains(&start) {
                notes.push(NoteEvent {
                    start: start as usize,
                    ..*note
                });
            }
        }
    }
    notes
}

/// A demo project of a song: one pattern per part with the melody, the
/// chords and the bass, drum patterns, and the parts arranged in order.
pub fn song_demo(song: &Song) -> Project {
    let mut project = Project::empty();
    project.tempo = Tempo::new(song.tempo);
    project.time_signature = song.time;
    project.scale = song.scale();
    let bar = song.time.steps_per_bar();

    let lead = apply::synth_named(&mut project, LEAD);
    let keys = apply::synth_named(&mut project, apply::KEYS);
    let bass = match song.bass {
        Bass::None => None,
        _ => apply::synth_named(&mut project, apply::BASS),
    };

    // One pattern per part. `Project::empty` starts with one pattern,
    // which the first part uses.
    let mut patterns = Vec::new();
    for (number, part) in song.parts.iter().enumerate() {
        let length = part.length(bar);
        let index = if number == 0 {
            if let Some(pattern) = project.pattern_mut(0) {
                pattern.name = part.name.to_string();
                pattern.set_length(length);
            }
            0
        } else {
            new_pattern(&mut project, part.name, length)
        };
        let melody = melody_notes(part, bar, length);
        let (chords, bass_line) = if part.left.is_empty() {
            let backing = accompany(&part.spans(bar), song.tonic, song.comping, song.bass);
            (backing.chords, backing.bass)
        } else {
            (part.left().notes, Vec::new())
        };
        if let Some(pattern) = project.pattern_mut(index) {
            if let Some(lead) = lead {
                pattern.set_notes(lead, melody);
            }
            if let Some(keys) = keys {
                pattern.set_notes(keys, chords);
            }
            if let Some(bass) = bass {
                pattern.set_notes(bass, bass_line);
            }
        }
        patterns.push((part.name, index, length));
    }

    // A drum pattern for each part, with a fill at the end of long ones.
    let rhythm = Rhythm::named(song.rhythm);
    let mut drums = Vec::new();
    if let Some(rhythm) = rhythm {
        for &(name, _, length) in &patterns {
            let long = length >= 4 * bar;
            let index = drum_pattern(&mut project, &format!("{name} drums"), length, rhythm, long);
            drums.push((name, index));
        }
    }
    let end = ending(&mut project, rhythm.is_some());
    name_tracks(&mut project, "Song", "Drums");

    let mut at = 0;
    for (entry, name) in song.form.iter().enumerate() {
        let Some(&(_, pattern, length)) = patterns.iter().find(|(part, ..)| part == name) else {
            continue;
        };
        project.add_clip(Clip::new(pattern, SONG_TRACK, at));
        if entry >= song.drums_from
            && let Some(&(_, drum)) = drums.iter().find(|(part, _)| part == name)
        {
            project.add_clip(Clip::new(drum, DRUM_TRACK, at));
        }
        at += length;
    }
    project.add_clip(Clip::new(end, SONG_TRACK, at));
    project.remove_unused_tracks();
    project
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file::{from_text, to_text};
    use crate::library::{SONGS, STYLES};
    use crate::pattern::MAX_STEPS;

    fn check(project: &Project, name: &str) {
        assert!(project.song_length() > 0, "{name}");
        for pattern in project.patterns() {
            assert!(pattern.length() <= MAX_STEPS, "{name}: {}", pattern.name);
            assert!(!pattern.is_empty(), "{name}: {} is empty", pattern.name);
        }
        let text = to_text(project).unwrap();
        assert_eq!(
            &from_text(&text).unwrap(),
            project,
            "{name} survives saving"
        );
    }

    #[test]
    fn every_style_makes_a_demo() {
        for style in STYLES {
            let project = style_demo(style);
            check(&project, style.name);
            assert_eq!(project.time_signature, style.time);
        }
    }

    #[test]
    fn every_song_makes_a_demo() {
        for song in SONGS {
            let project = song_demo(song);
            check(&project, song.name);
            let lead = project
                .channels()
                .iter()
                .position(|c| c.name == LEAD)
                .unwrap();
            let notes: usize = project.patterns().iter().map(|p| p.notes(lead).len()).sum();
            assert!(notes >= 8, "{}: the melody is there", song.name);
        }
    }

    #[test]
    fn a_style_demo_starts_alone_then_brings_in_the_drums() {
        let project = style_demo(Style::named("Four chords").unwrap());
        let clips = project.playlist().clips();
        let at = |start| {
            let mut tracks: Vec<usize> = clips
                .iter()
                .filter(|c| c.start == start)
                .map(|c| c.track)
                .collect();
            tracks.sort();
            tracks
        };
        assert_eq!(at(0), [SONG_TRACK], "the harmony alone");
        assert_eq!(at(64), [SONG_TRACK, DRUM_TRACK]);
        assert_eq!(
            project.playlist().tracks().len(),
            2,
            "only the tracks it uses"
        );
        let names: Vec<&str> = project.patterns().iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, ["Harmony", "Drums", "Drums and fill", "Ending"]);
    }

    #[test]
    fn a_round_has_its_voices_in_the_right_places() {
        let song = Song::named("Frère Jacques").unwrap();
        let round = song.part("Round").unwrap();
        let notes = melody_notes(round, 16, 128);
        // Four voices, each starting on C5, two bars apart.
        let entries: Vec<usize> = notes
            .iter()
            .filter(|note| note.pitch.midi() == 72 && note.start % 32 == 0)
            .map(|note| note.start)
            .collect();
        assert!(entries.starts_with(&[0]) && entries.contains(&32) && entries.contains(&96));
        let last = song.part("Last voices").unwrap();
        assert!(
            melody_notes(last, 16, 128)
                .iter()
                .all(|note| note.start < 96)
        );
    }
}
