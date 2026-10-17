//! Demo projects, arranged from the library.
//!
//! A pattern is a loop; a song is loops arranged in time. These
//! functions do the arranging the way a producer would: the harmony
//! alone first, then the drums come in, a fill every few bars, and a
//! final chord with a crash at the end. Every demo is an ordinary
//! project, to take apart and change.

use crate::chord_sheet::{ChordMarker, HarmonyName};
use crate::harmony::{ChordSymbol, Scale};
use crate::key_map::KeyMarker;
use crate::meter::{TimeMarker, TimeSignature};
use crate::playlist::{Clip, Section};
use crate::project::Project;
use crate::tempo::{Tempo, TempoMarker};

use super::apply::{self, LEAD};
use super::chart::{ChordSpan, accompany};
use super::comping::Bass;
use super::interpret::{self, Role};
use super::micromodes::{self, MicromodeStyle};
use super::rhythm::{FILLS, RHYTHMS, Rhythm};
use super::rows::{self, DEMO_SECTIONS, RowStyle};
use super::song::{Part, Song, TempoMark};
use super::style::Style;
use crate::pattern::NoteEvent;
use crate::pitch::PitchClass;

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

/// Writes a chart's chords on the chord sheet, from step `at` on, named
/// in `key`, so a demo opens with the chords it plays written there.
fn write_chart(project: &mut Project, at: usize, spans: &[ChordSpan], key: Scale) {
    let Some(end) = spans.last().map(|span| at + span.start + span.length) else {
        return;
    };
    let markers: Vec<ChordMarker> = spans
        .iter()
        .map(|span| {
            ChordMarker::new(
                at + span.start,
                HarmonyName::Chord(span.chord.symbol_in(key)),
            )
        })
        .collect();
    project.set_chords(at, end, &markers);
}

/// Writes the home chord of `key` on the chord sheet for the ending, a
/// bar of `bar` steps from step `at`.
fn write_ending(project: &mut Project, at: usize, bar: usize, key: Scale) {
    let chord = apply::home_chord(key);
    let symbol = ChordSymbol::new(key.spell_root(chord), chord.quality);
    project.set_chords(
        at,
        at + bar,
        &[ChordMarker::new(at, HarmonyName::Chord(symbol))],
    );
}

/// A demo song built from a style: its harmony, its rhythm, fills and
/// an ending, in the style's own key, tempo and time signature.
pub fn style_demo(style: &Style) -> Project {
    let tonic = style.home_tonic();
    let mut project = Project::empty();
    project.tempo = Tempo::new(style.tempo);
    project.time_signature = style.time;
    project.scale = style.scale(tonic);
    let key = project.scale;
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
        write_chart(&mut project, at, &style.spans(), key);
        at += length;
    }
    for pass in 0..passes {
        project.add_clip(Clip::new(harmony, SONG_TRACK, at));
        write_chart(&mut project, at, &style.spans(), key);
        if let Some((plain, fill)) = drums {
            let last = pass + 1 == passes;
            let pattern = if pass % 2 == 1 || last { fill } else { plain };
            project.add_clip(Clip::new(pattern, DRUM_TRACK, at));
        }
        at += length;
    }
    project.add_clip(Clip::new(end, SONG_TRACK, at));
    write_ending(&mut project, at, style.time.steps_per_bar(), key);
    project.remove_unused_tracks();
    project
}

/// Puts a tempo marking of a part that starts on step `part_start` on
/// the playlist.
fn mark_tempo(project: &mut Project, part_start: usize, mark: TempoMark) {
    match mark {
        TempoMark::At { at, bpm } => {
            project.set_tempo_marker(TempoMarker::new(part_start + at, f32::from(bpm)));
        }
        TempoMark::Glide { at, bpm, over } => {
            project.set_tempo_marker(TempoMarker::gliding(part_start + at, f32::from(bpm), over));
        }
        TempoMark::Fermata { at, steps } => {
            let start = part_start + at;
            let before = project.tempo_map().tempo_at(start).bpm();
            // Twice as long is half as fast, for as long as the held
            // note lasts; then on at the tempo before.
            project.set_tempo_marker(TempoMarker::new(start, before / 2.0));
            project.set_tempo_marker(TempoMarker::new(start + steps, before));
        }
    }
}

/// The genres whose demos slow down a little at the very end, as a
/// player does where the music has no beat to keep: songs, hymns and
/// concert pieces. Dance music and anything over a drum machine keeps
/// its tempo to the last beat.
fn ends_broadening(song: &Song) -> bool {
    use super::Genre::*;
    matches!(song.genre, Classical | Folk | Soul | Tango | Folklore)
}

/// The step the last bar of the song's last part starts on, when the
/// song's last part ends on step `end`, and the part has no tempo
/// markings of its own (the score then already says how it ends).
fn last_bar(song: &Song, end: usize) -> Option<usize> {
    let part = song.form.last().and_then(|name| song.part(name))?;
    if !part.tempo.is_empty() {
        return None;
    }
    end.checked_sub(song.part_time(part).steps_per_bar())
}

/// A *poco rit.*, a little slowing down, over the last bar of the music,
/// to nine tenths of the tempo, which the final chord keeps.
fn broaden_the_end(project: &mut Project, song: &Song, bar: usize) {
    if !ends_broadening(song) || bar == 0 {
        return;
    }
    let tempo = project.tempo_map().tempo_at(bar).bpm();
    let length = project.meter().bar_at(bar).length;
    project.set_tempo_marker(TempoMarker::gliding(bar, tempo * 0.9, length));
}

/// A demo of a twelve-tone style: the piece as written (P), backwards
/// (R), upside down (I) and both (RI), each a section of the song, and as
/// written once more to end. With a rhythm, the drums come in after the
/// first section, with a fill before the last.
pub fn row_demo(style: &RowStyle) -> Project {
    let mut project = Project::empty();
    project.tempo = Tempo::new(style.tempo);
    project.time_signature = style.time;
    let length = style.length();
    let names = DEMO_SECTIONS.map(|(_, name)| name);
    let patterns = section_patterns(&mut project, &names, length, |project, index, i| {
        rows::write_section(project, index, style, DEMO_SECTIONS[i].0);
    });
    arrange_sections(&mut project, &patterns, length, style.rhythm(), "Row");
    project
}

/// A demo of a micromode style: the walk from the micromode on the key
/// note, from it upside down, from a tritone away, and from there upside
/// down, each a section of the song, then the first once more. The key
/// note is C. With a rhythm, the drums come in after the first section.
pub fn micromode_demo(style: &MicromodeStyle) -> Project {
    let mut project = Project::empty();
    project.tempo = Tempo::new(style.tempo);
    let length = style.length();
    let names = micromodes::DEMO_SECTIONS.map(|(_, name)| name);
    let patterns = section_patterns(&mut project, &names, length, |project, index, i| {
        let start = micromodes::DEMO_SECTIONS[i].0;
        micromodes::write_micromode_style(project, index, style, PitchClass::C, start, None);
    });
    project.scale = style.scale(PitchClass::C);
    arrange_sections(&mut project, &patterns, length, style.rhythm(), "Micromode");
    project
}

/// One pattern per section, named after it and `length` steps long, the
/// first being the project's first pattern; `write` fills section `i`
/// into pattern `index`.
fn section_patterns(
    project: &mut Project,
    names: &[&'static str],
    length: usize,
    mut write: impl FnMut(&mut Project, usize, usize),
) -> Vec<(usize, &'static str)> {
    let mut patterns = Vec::new();
    for (i, name) in names.iter().enumerate() {
        let index = if i == 0 {
            if let Some(pattern) = project.pattern_mut(0) {
                pattern.name = name.to_string();
            }
            0
        } else {
            new_pattern(project, name, length)
        };
        write(project, index, i);
        patterns.push((index, *name));
    }
    patterns
}

/// Arranges four section patterns as a song: each in turn, then the
/// first again, each a section of the playlist. With a rhythm, the drums
/// come in after the first section, with a fill before the last.
fn arrange_sections(
    project: &mut Project,
    patterns: &[(usize, &'static str)],
    length: usize,
    rhythm: Option<&'static Rhythm>,
    track: &str,
) {
    let drums = rhythm.map(|rhythm| {
        let plain = drum_pattern(project, "Drums", length, rhythm, false);
        let fill = drum_pattern(project, "Drums and fill", length, rhythm, true);
        (plain, fill)
    });
    name_tracks(project, track, "Drums");
    let order = [0, 1, 2, 3, 0];
    for (place, &which) in order.iter().enumerate() {
        let (pattern, name) = patterns[which];
        let at = place * length;
        project.add_clip(Clip::new(pattern, SONG_TRACK, at));
        project.add_section(Section::new(name, at, length));
        if let Some((plain, fill)) = drums
            && place > 0
        {
            let drums = if place + 2 == order.len() {
                fill
            } else {
                plain
            };
            project.add_clip(Clip::new(drums, DRUM_TRACK, at));
        }
    }
    project.remove_unused_tracks();
}

/// The notes of a part's melody for each voice that sings it, inside the
/// part, played with the accents of a performance (see [`interpret`]):
/// the first list is voice 1's, the second voice 2's, and so on, for
/// `voices` voices. A round has a list per voice; a song, one.
fn melody_notes(
    song: &Song,
    part: &Part,
    time: TimeSignature,
    length: usize,
    voices: usize,
) -> Vec<Vec<NoteEvent>> {
    let bar = time.steps_per_bar();
    let mut sung = vec![Vec::new(); voices];
    for &(voice, start_bar) in part.voices {
        let Some(notes) = voice.checked_sub(1).and_then(|index| sung.get_mut(index)) else {
            continue;
        };
        // Each voice gets accents of its own, as each singer would.
        let mut melody = part.melody();
        let seed = interpret::seed(song.name, part.name, &voice_channel(voice));
        interpret::perform(&mut melody.notes, time, Role::Melody, seed);
        let offset = start_bar as isize * bar as isize;
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
    for notes in &mut sung {
        notes.sort_by_key(|note| (note.start, note.pitch));
    }
    sung
}

/// The channel voice `voice` of a melody sings on: the Lead channel for
/// the first, "Voice 2", "Voice 3"... for the others.
fn voice_channel(voice: usize) -> String {
    if voice <= 1 {
        LEAD.to_string()
    } else {
        format!("Voice {voice}")
    }
}

/// A demo project of a song: one pattern per part with the melody, the
/// chords and the bass, drum patterns, and the parts arranged in order.
/// Each voice of a round or a canon sings on a channel of its own, and
/// every channel plays the General MIDI instrument of the song's band.
/// A part in a time signature of its own gets a time marker where it
/// starts, and one back where the song's time comes again.
pub fn song_demo(song: &Song) -> Project {
    let mut project = Project::empty();
    project.tempo = Tempo::new(song.tempo);
    project.time_signature = song.time;
    project.scale = song.scale();

    let voices: Vec<Option<usize>> = (1..=song.voices())
        .map(|voice| apply::synth_named(&mut project, &voice_channel(voice)))
        .collect();
    let keys = apply::synth_named(&mut project, apply::KEYS);
    let bass = match song.bass {
        Bass::None => None,
        _ => apply::synth_named(&mut project, apply::BASS),
    };
    let band = song.band;
    for (channel, program) in voices
        .iter()
        .map(|&voice| (voice, band.lead))
        .chain([(keys, band.keys), (bass, band.bass)])
    {
        if let Some(state) = channel.and_then(|channel| project.channel_mut(channel)) {
            state.program = Some(program);
        }
    }

    // One pattern per part. `Project::empty` starts with one pattern,
    // which the first part uses.
    let mut patterns = Vec::new();
    for (number, part) in song.parts.iter().enumerate() {
        let time = song.part_time(part);
        let bar = time.steps_per_bar();
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
        let melody = melody_notes(song, part, time, length, voices.len());
        let (mut chords, mut bass_line) = if part.left.is_empty() {
            let tonic = song.part_scale(part).root;
            let backing = accompany(&part.spans(bar), tonic, song.comping, song.bass);
            (backing.chords, backing.bass)
        } else {
            (part.left().notes, Vec::new())
        };
        let seed = |channel| interpret::seed(song.name, part.name, channel);
        interpret::perform(&mut chords, time, Role::Accompaniment, seed(apply::KEYS));
        interpret::perform(&mut bass_line, time, Role::Accompaniment, seed(apply::BASS));
        if let Some(pattern) = project.pattern_mut(index) {
            for (&channel, notes) in voices.iter().zip(melody) {
                if let Some(channel) = channel {
                    pattern.set_notes(channel, notes);
                }
            }
            if let Some(keys) = keys {
                pattern.set_notes(keys, chords);
            }
            if let Some(bass) = bass {
                pattern.set_notes(bass, bass_line);
            }
        }
        patterns.push((part.name, index, length, time));
    }

    // A drum pattern for each part, with a fill at the end of long ones.
    // Patterns without clips are counted in the project's time signature,
    // so it is the part's own while its drums are written.
    let rhythm = Rhythm::named(song.rhythm);
    let mut drums = Vec::new();
    if let Some(rhythm) = rhythm {
        for &(name, _, length, time) in &patterns {
            let Some(rhythm) = rhythm_in(rhythm, time) else {
                continue;
            };
            project.time_signature = time;
            let long = length >= 4 * time.steps_per_bar();
            let index = drum_pattern(&mut project, &format!("{name} drums"), length, rhythm, long);
            drums.push((name, index));
        }
    }
    // The ending is a bar of the last part's time, on the home chord of
    // the last part's key: a song that ends after a modulation ends in
    // the new key.
    let last_part = song.form.last().and_then(|name| song.part(name));
    project.time_signature = last_part.map_or(song.time, |part| song.part_time(part));
    project.scale = last_part.map_or(song.scale(), |part| song.part_scale(part));
    let end = ending(&mut project, rhythm.is_some());
    project.time_signature = song.time;
    project.scale = song.scale();
    name_tracks(&mut project, "Song", "Drums");

    let mut at = 0;
    let mut time_here = song.time;
    for (entry, name) in song.form.iter().enumerate() {
        let Some(&(_, pattern, length, time)) = patterns.iter().find(|(part, ..)| part == name)
        else {
            continue;
        };
        if time != time_here {
            project.set_time_marker(TimeMarker::new(at, time));
            time_here = time;
        }
        if let Some(part) = song.part(name) {
            // A part in a key of its own is a modulation: the key changes
            // where it starts, and changes back after it.
            let key = song.part_scale(part);
            if key != project.key_at(at) {
                project.set_key_marker(KeyMarker::new(at, key));
            }
            for &mark in part.tempo {
                mark_tempo(&mut project, at, mark);
            }
            // A part with its accompaniment written out has no chart:
            // its chords are only heard (see `Project::heard_chords`).
            write_chart(&mut project, at, &part.spans(time.steps_per_bar()), key);
        }
        project.add_clip(Clip::new(pattern, SONG_TRACK, at));
        if entry >= song.drums_from
            && let Some(&(_, drum)) = drums.iter().find(|(part, _)| part == name)
        {
            project.add_clip(Clip::new(drum, DRUM_TRACK, at));
        }
        at += length;
    }
    project.add_clip(Clip::new(end, SONG_TRACK, at));
    if !project.playlist().chord_markers().is_empty() {
        let (time, key) = last_part.map_or((song.time, song.scale()), |part| {
            (song.part_time(part), song.part_scale(part))
        });
        write_ending(&mut project, at, time.steps_per_bar(), key);
    }
    if let Some(bar) = last_bar(song, at) {
        broaden_the_end(&mut project, song, bar);
    }
    project.remove_unused_tracks();
    project
}

/// The rhythm to play in a part in `time`: the song's own if it is in
/// that time, or else one of the same genre that is, or any that is.
fn rhythm_in(rhythm: &'static Rhythm, time: TimeSignature) -> Option<&'static Rhythm> {
    if rhythm.time == time {
        return Some(rhythm);
    }
    let in_time = || RHYTHMS.iter().filter(move |other| other.time == time);
    in_time()
        .find(|other| other.genre == rhythm.genre)
        .or_else(|| in_time().next())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file::{from_text, to_text};
    use crate::library::song::Part;
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
    fn every_row_style_makes_a_demo() {
        for style in &crate::library::ROW_STYLES {
            let project = row_demo(style);
            check(&project, style.name);
            assert_eq!(project.playlist().sections().len(), 5, "{}", style.name);
            assert!(!project.scale.kind.is_tonal());
            // A row key needs format 6, and a piano on the Keys channel 9.
            let piano = project.channels().iter().any(|c| c.program.is_some());
            let format = if piano { 9 } else { 6 };
            assert_eq!(crate::file::format_for(&project), format);
        }
    }

    #[test]
    fn every_micromode_style_makes_a_demo() {
        for style in &crate::library::MICROMODE_STYLES {
            let project = micromode_demo(style);
            check(&project, style.name);
            assert_eq!(project.playlist().sections().len(), 5, "{}", style.name);
            assert_eq!(project.time_signature, style.time);
            let micromode = style.micromode();
            assert_eq!(
                crate::harmony::Micromode::of(project.scale.pitch_set()),
                Some(micromode)
            );
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
    fn a_part_in_its_own_time_gets_time_markers() {
        // A song in 4/4 with one waltz part: three bars of 3/4.
        let song = SONGS
            .iter()
            .find(|song| song.time == TimeSignature::COMMON && !song.rhythm.is_empty())
            .unwrap();
        let waltz = TimeSignature::new(3, 4);
        let waltz_part = Part {
            name: "Waltz",
            melody: "C5:12 | E5:12 | G5:12",
            chords: "I | IV | V",
            left: "",
            voices: &[(1, 0)],
            time: Some(waltz),
            key: None,
            tempo: &[],
        };
        let first = song.form[0];
        let parts: &'static [Part] =
            Box::leak(vec![*song.part(first).unwrap(), waltz_part].into_boxed_slice());
        let form: &'static [&'static str] =
            Box::leak(vec![first, "Waltz", first].into_boxed_slice());
        let song = Song {
            parts,
            form,
            drums_from: 0,
            ..*song
        };
        let project = song_demo(&song);
        let first_length = song.part(first).unwrap().length(16);
        let markers: Vec<(usize, TimeSignature)> = project
            .playlist()
            .time_markers()
            .iter()
            .map(|marker| (marker.start, marker.time))
            .collect();
        assert_eq!(
            markers,
            [
                (first_length, waltz),
                (first_length + 36, TimeSignature::COMMON)
            ]
        );
        let drums = project
            .patterns()
            .iter()
            .position(|pattern| pattern.name == "Waltz drums")
            .expect("the waltz has drums");
        assert_eq!(project.pattern_time(drums), waltz);
        assert_eq!(project.patterns()[drums].length(), 36);
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
    fn a_song_demo_opens_with_its_chart_on_the_chord_sheet() {
        let project = song_demo(Song::named("El Choclo").unwrap());
        // Bar `bar` (from 1), beat `beat` (from 1), in 4/4.
        let chord = |bar: usize, beat: usize| {
            let step = (bar - 1) * 16 + (beat - 1) * 4;
            project.chord_at(step).unwrap().harmony.to_string()
        };
        // The pickup and the first strain, in A minor: i | i | i V7.
        assert_eq!(chord(1, 1), "Am");
        assert_eq!(chord(3, 3), "E7");
        assert_eq!(chord(7, 1), "A7");
        assert_eq!(chord(7, 3), "Dm");
        // The second strain leans on C: bVII7 bIII is G7 C.
        assert_eq!(chord(10, 1), "G7");
        assert_eq!(chord(10, 3), "C");
        // The third strain is in A major: VI7 ii | II7 V7 is F#7 Bm | B7 E7.
        assert_eq!(chord(18, 1), "A");
        assert_eq!(chord(20, 1), "F#7");
        assert_eq!(chord(20, 3), "Bm");
        assert_eq!(chord(21, 1), "B7");
        assert_eq!(chord(21, 3), "E7");
        // The ending holds the home chord of A major.
        let end = project.song_length() - 1;
        assert_eq!(project.chord_at(end).unwrap().harmony.to_string(), "A");
    }

    #[test]
    fn a_style_demo_has_its_chart_on_the_chord_sheet_every_time_round() {
        let style = STYLES
            .iter()
            .find(|s| s.name == "Twelve-bar blues")
            .unwrap();
        let project = style_demo(style);
        let bar = style.time.steps_per_bar();
        let spans = style.spans();
        let key = project.scale;
        let mut at = 0;
        while at + style.length() <= project.song_length() - bar {
            for span in &spans {
                let marker = project.chord_at(at + span.start).unwrap();
                assert_eq!(
                    marker.harmony,
                    HarmonyName::Chord(span.chord.symbol_in(key))
                );
            }
            at += style.length();
        }
        assert!(at > style.length(), "the chart is written more than once");
    }

    #[test]
    fn a_piece_with_its_accompaniment_written_out_has_no_chart_to_write() {
        // Its chords are heard when it is opened instead.
        let project = song_demo(Song::named("Für Elise").unwrap());
        assert!(project.playlist().chord_markers().is_empty());
    }

    #[test]
    fn a_song_that_ends_after_a_modulation_ends_in_the_new_key() {
        // Mozart's Dies irae, D minor, stops in A minor here.
        let project = song_demo(Song::named("Dies irae (Mozart)").unwrap());
        let end = project.song_length() - 16;
        assert_eq!(project.key_at(end).root, PitchClass::A);
        let ending = project
            .patterns()
            .iter()
            .find(|p| p.name == "Ending")
            .unwrap();
        let mut heard: Vec<PitchClass> = (0..16)
            .flat_map(|channel| ending.notes(channel))
            .map(|note| note.pitch.pitch_class())
            .collect();
        heard.sort();
        heard.dedup();
        assert_eq!(heard, [PitchClass::C, PitchClass::E, PitchClass::A]);
    }

    #[test]
    fn tempo_markings_go_on_the_playlist() {
        let mut project = Project::empty();
        project.tempo = Tempo::new(100.0);
        mark_tempo(&mut project, 32, TempoMark::At { at: 0, bpm: 120 });
        mark_tempo(&mut project, 32, TempoMark::Fermata { at: 8, steps: 4 });
        mark_tempo(
            &mut project,
            32,
            TempoMark::Glide {
                at: 16,
                bpm: 60,
                over: 8,
            },
        );
        let map = project.tempo_map();
        let bpm = |step| map.tempo_at(step).bpm();
        assert_eq!(bpm(31), 100.0);
        assert_eq!(bpm(32), 120.0);
        // The fermata: half as fast for the four steps it holds.
        assert_eq!((bpm(40), bpm(43), bpm(44)), (60.0, 60.0, 120.0));
        assert_eq!(bpm(55), 60.0);
    }

    #[test]
    fn songs_and_hymns_slow_down_a_little_at_the_end() {
        let ends_at = |name: &str| {
            let song = Song::named(name).unwrap();
            let project = song_demo(song);
            let map = project.tempo_map();
            let end = project.song_length();
            (song.tempo, map.tempo_at(end - 1).bpm())
        };
        // A hymn broadens to nine tenths of its tempo...
        let (tempo, last) = ends_at("Amazing Grace");
        assert!((last - tempo * 0.9).abs() < 0.01, "{tempo} {last}");
        // ...a dance track does not.
        let (tempo, last) = ends_at("Ode to Joy");
        assert_eq!(last, tempo);
    }

    #[test]
    fn every_demo_keeps_a_sensible_tempo() {
        for song in SONGS {
            let project = song_demo(song);
            let map = project.tempo_map();
            for marker in map.markers() {
                let ratio = marker.tempo.bpm() / song.tempo;
                assert!(
                    (0.3..=2.5).contains(&ratio),
                    "{}: {} BPM on step {}",
                    song.name,
                    marker.tempo.bpm(),
                    marker.start
                );
                assert!(marker.start < project.song_length(), "{}", song.name);
            }
        }
    }

    #[test]
    fn a_round_has_its_voices_in_the_right_places() {
        let song = Song::named("Frère Jacques").unwrap();
        assert_eq!(song.voices(), 4);
        let round = song.part("Round").unwrap();
        let voices = melody_notes(song, round, TimeSignature::COMMON, 128, 4);
        // Four voices, each starting on C5, two bars apart, each on its own.
        for (voice, notes) in voices.iter().enumerate() {
            let first = notes.first().unwrap();
            assert_eq!((first.start, first.pitch.midi()), (voice * 32, 72));
        }
        // In the last part, the first voice has finished; the others
        // sing their ends, voice 4 the longest.
        let last = song.part("Last voices").unwrap();
        let voices = melody_notes(song, last, TimeSignature::COMMON, 128, 4);
        assert!(voices[0].is_empty());
        let ends: Vec<usize> = voices[1..]
            .iter()
            .map(|notes| notes.iter().map(|note| note.start).max().unwrap())
            .collect();
        assert!(
            ends[0] < ends[1] && ends[1] < ends[2] && ends[2] < 96,
            "{ends:?}"
        );
    }

    #[test]
    fn every_voice_of_a_round_has_a_channel_and_every_channel_an_instrument() {
        let canon = song_demo(Song::named("Canon in D").unwrap());
        let names: Vec<(&str, Option<u8>)> = canon
            .channels()
            .iter()
            .map(|channel| (channel.name.as_str(), channel.program))
            .collect();
        use crate::general_midi::{CELLO, HARPSICHORD, VIOLIN};
        assert_eq!(
            names,
            [
                (LEAD, Some(VIOLIN)),
                ("Voice 2", Some(VIOLIN)),
                ("Voice 3", Some(VIOLIN)),
                (apply::KEYS, Some(HARPSICHORD)),
                (apply::BASS, Some(CELLO)),
            ]
        );
        // Each violin plays the tune once it has come in: the second two
        // bars after the first, the third two bars after that.
        let entries = canon
            .patterns()
            .iter()
            .position(|pattern| pattern.name == "Entries")
            .unwrap();
        let first_note = |channel: usize| canon.patterns()[entries].notes(channel)[0].start;
        assert_eq!((first_note(0), first_note(1), first_note(2)), (0, 32, 64));

        for song in SONGS {
            let project = song_demo(song);
            let voices = project
                .channels()
                .iter()
                .filter(|channel| channel.name.starts_with("Voice "))
                .count();
            assert_eq!(voices + 1, song.voices(), "{}", song.name);
            for channel in project.channels() {
                if channel.kind == crate::ChannelKind::Synth {
                    assert!(channel.program.is_some(), "{}: {}", song.name, channel.name);
                }
            }
        }
    }
}
