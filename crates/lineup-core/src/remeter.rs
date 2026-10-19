//! Re-metering: putting a whole song into another time signature.
//!
//! A song in 4/4 played with a waltz rhythm sounds wrong if its bars stay
//! sixteen steps long: the waltz's "one two three" is cut off every bar,
//! and its downbeat drifts against the melody. To play the song as a
//! waltz, every bar has to become a 3/4 bar. [`remeter`] does that: each
//! bar of the song, whatever its time signature, is squeezed or
//! stretched into one bar of the new one, and every note keeps its place
//! in its bar. A note on the half bar stays on the half bar; a note on
//! beat 2 of a 4/4 bar (a quarter of the way in) lands a quarter of the
//! way into the 3/4 bar.
//!
//! ```text
//! 4/4: | C . . . E . . . G . . . E . . . |   16 steps
//! 3/4: | C . . E . . G . . E . . |           12 steps
//! ```
//!
//! This is a mechanical stretch, the way a tape machine would do it, not
//! the way a composer rewrites a tune for a new meter. Where the squeeze
//! lands a note between two steps, it keeps its exact place, measured in
//! ticks (see [`TICKS_PER_STEP`]): four even eighth notes squeezed into
//! three beats stay even.
//!
//! Everything that is placed in the song moves with its bar: clips,
//! sections, the loop, the tempo and key markers, and the chords of the
//! chord sheet, which land on the nearest beat of the new bar. The time
//! markers go, since the whole song is now in one time signature.
//!
//! Each bar also keeps how long it lasts. The tempo counts quarter notes,
//! so a 3/4 bar at the same tempo would be over in three quarters of the
//! time, and the whole song would run faster. Instead the tempo goes down
//! in the same proportion: 4/4 at 112 becomes 3/4 at 84, and the song
//! keeps its pace. Only where the beats fall changes.

use crate::chord_sheet::{snap_to_beat, tidy};
use crate::pattern::{MAX_STEPS, TICKS_PER_STEP};
use crate::{Clip, MAX_SONG_STEPS, Meter, NoteEvent, Project, Tempo, TempoMarker, TimeSignature};

/// Where a moment of a song goes when its bars change length: bar `n` of
/// the old meter becomes bar `n` of the new time signature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Remap {
    old: Meter,
    /// Steps in one new bar.
    bar: usize,
}

impl Remap {
    /// From the bars of `old` to bars of `time`.
    pub fn new(old: Meter, time: TimeSignature) -> Self {
        Remap {
            old,
            bar: time.steps_per_bar(),
        }
    }

    /// Where tick `tick` of the old song is in the new one.
    ///
    /// A bar shorter than its time signature (one cut short by a time
    /// marker that is not on a bar line) still becomes a whole new bar.
    pub fn tick(&self, tick: usize) -> usize {
        let bar = self.old.bar_at(tick / TICKS_PER_STEP);
        let start = bar.start * TICKS_PER_STEP;
        let length = bar.length * TICKS_PER_STEP;
        let new_bar = self.bar * TICKS_PER_STEP;
        // Rounded to the nearest tick.
        bar.number * new_bar + ((tick - start) * new_bar + length / 2) / length
    }

    /// Where step `step` of the old song is in the new one, to the
    /// nearest step.
    pub fn step(&self, step: usize) -> usize {
        (self.tick(step * TICKS_PER_STEP) + TICKS_PER_STEP / 2) / TICKS_PER_STEP
    }
}

/// Whether every bar of the song, and every pattern, is already in
/// `time`.
pub fn is_in(project: &Project, time: TimeSignature) -> bool {
    project.time_signature == time
        && project
            .playlist()
            .time_markers()
            .iter()
            .all(|marker| marker.time == time)
        && (0..project.patterns().len()).all(|pattern| project.pattern_time(pattern) == time)
}

/// Puts the whole song in `time`, bar for bar, as the module explains.
/// Returns `false`, changing nothing, if it is already in `time` or
/// `time` has no room for a bar in a pattern.
///
/// A pattern whose bars no longer fit in [`MAX_STEPS`] (sixteen bars of
/// 4/4 are 384 steps in 12/8) is cut into patterns that do, and each of
/// its clips into clips of those, one after the other.
pub fn remeter(project: &mut Project, time: TimeSignature) -> bool {
    project.moving_the_band(|project| remeter_song(project, time))
}

fn remeter_song(project: &mut Project, time: TimeSignature) -> bool {
    let bar = time.steps_per_bar();
    if is_in(project, time) || bar > MAX_STEPS {
        return false;
    }
    let song = Remap::new(project.meter(), time);

    // Each pattern is stretched the way the song is where it first
    // plays, or, if it is not in the song, the way its own bars go.
    let mut stretched = Vec::new();
    for (index, pattern) in project.patterns().iter().enumerate() {
        let first_clip = project
            .playlist()
            .clips()
            .iter()
            .filter(|clip| clip.pattern == index)
            .map(|clip| clip.start)
            .min();
        let (remap, origin) = match first_clip {
            Some(start) => (song.clone(), start * TICKS_PER_STEP),
            None => (
                Remap::new(Meter::steady(project.pattern_time(index)), time),
                0,
            ),
        };
        let from = remap.tick(origin);
        let place = |tick: usize| remap.tick(origin + tick) - from;
        let length = place(pattern.length() * TICKS_PER_STEP).div_ceil(TICKS_PER_STEP);
        let rows: Vec<Vec<NoteEvent>> = (0..pattern.rows())
            .map(|channel| {
                pattern
                    .notes(channel)
                    .iter()
                    .map(|note| {
                        let start = place(note.start_tick());
                        let end = place(note.end_tick()).max(start + 1);
                        NoteEvent {
                            velocity: note.velocity,
                            ..NoteEvent::exact(note.pitch, start, end - start)
                        }
                    })
                    .collect()
            })
            .collect();
        stretched.push((length.max(1), rows));
    }

    // The playlist, while the old meter still holds.
    let playlist = project.playlist().clone();

    // The patterns, cut where they grew too long.
    let chunk = MAX_STEPS / bar * bar;
    let mut pieces: Vec<Vec<usize>> = Vec::new();
    for (index, (length, rows)) in stretched.into_iter().enumerate() {
        let count = length.div_ceil(chunk);
        let mut indices = vec![index];
        for piece in 1..count {
            let Some(copy) = project.clone_pattern(index) else {
                break;
            };
            let name = &project.patterns()[index].name;
            project.patterns[copy].name = format!("{name} ({})", piece + 1);
            // A piece of the band's part is still the band's.
            project.patterns[copy].player = project.patterns[index].player;
            indices.push(copy);
        }
        for (piece, &target) in indices.iter().enumerate() {
            let from = piece * chunk;
            let until = if piece + 1 == indices.len() {
                MAX_STEPS.max(length)
            } else {
                from + chunk
            };
            let pattern = &mut project.patterns[target];
            pattern.time = None;
            pattern.set_length((length - from).min(chunk));
            for (channel, notes) in rows.iter().enumerate() {
                pattern.set_notes(
                    channel,
                    notes
                        .iter()
                        .filter(|note| (from..until).contains(&note.start))
                        .map(|note| NoteEvent {
                            start: note.start - from,
                            ..*note
                        }),
                );
            }
        }
        pieces.push(indices);
    }

    // Every bar keeps how long it lasts: the tempo changes as much as
    // the bar's length in steps does, wherever the time signature
    // changes too.
    let meter = project.meter();
    let tempos = project.tempo_map();
    let ratio = |step: usize| bar as f32 / meter.time_at(step).steps_per_bar() as f32;
    let mut markers: Vec<TempoMarker> = playlist
        .tempo_markers()
        .iter()
        .map(|marker| TempoMarker {
            tempo: Tempo::new(marker.tempo.bpm() * ratio(marker.start)),
            ..*marker
        })
        .collect();
    for change in meter.markers() {
        let step = change.start;
        let already = markers.iter().any(|marker| marker.start == step);
        if !already && ratio(step) != ratio(step - 1) {
            let bpm = tempos.tempo_at(step).bpm() * ratio(step);
            markers.push(TempoMarker::new(step, bpm));
        }
    }
    markers.sort_by_key(|marker| marker.start);
    project.tempo = Tempo::new(project.tempo.bpm() * ratio(0));
    project.playlist.tempo_markers = markers;

    // Everything on the playlist moves with its bar.
    let clips = &mut project.playlist.clips;
    clips.clear();
    for clip in playlist.clips() {
        let indices = &pieces[clip.pattern];
        let start = song.step(clip.start);
        if indices.len() == 1 {
            clips.push(Clip { start, ..*clip });
            continue;
        }
        let length = project.patterns[indices[0]].length() * (indices.len() - 1)
            + project.patterns[indices[indices.len() - 1]].length();
        for repeat in 0..clip.repeats {
            for (piece, &pattern) in indices.iter().enumerate() {
                clips.push(Clip::new(
                    pattern,
                    clip.track,
                    start + repeat * length + piece * chunk,
                ));
            }
        }
    }
    let playlist_mut = &mut project.playlist;
    for section in &mut playlist_mut.sections {
        let end = song.step(section.end());
        section.start = song.step(section.start);
        section.length = end.saturating_sub(section.start).max(1);
    }
    if let Some(region) = &mut playlist_mut.loop_region {
        region.start = song.step(region.start);
        region.end = song.step(region.end);
    }
    for marker in &mut playlist_mut.tempo_markers {
        let end = song.step(marker.start + marker.glide);
        marker.start = song.step(marker.start);
        marker.glide = end - marker.start;
    }
    for marker in &mut playlist_mut.key_markers {
        marker.start = song.step(marker.start);
    }
    if let Some(band) = &mut project.band {
        band.end = song.step(band.end);
        band.form.intro = song.step(band.form.intro);
        for marker in &mut band.variations {
            marker.start = song.step(marker.start);
        }
        crate::band::tidy_variations(&mut band.variations);
        for marker in &mut band.styles {
            marker.start = song.step(marker.start);
        }
        crate::band::sections::tidy(&mut band.styles);
    }
    // A chord changes on a beat, so it goes to the nearest one.
    for marker in &mut playlist_mut.chord_markers {
        marker.start = snap_to_beat(song.step(marker.start), time);
    }
    tidy(&mut playlist_mut.chord_markers, MAX_SONG_STEPS);
    // Two markers squeezed onto one step: the later one wins.
    playlist_mut.tempo_markers.dedup_by(|later, earlier| {
        later.start == earlier.start && {
            *earlier = *later;
            true
        }
    });
    playlist_mut.key_markers.dedup_by(|later, earlier| {
        later.start == earlier.start && {
            *earlier = *later;
            true
        }
    });
    playlist_mut.time_markers.clear();
    project.time_signature = time;
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{KeyMarker, Note, Pattern, Scale, Section, TempoMarker, TimeMarker};

    const THREE_FOUR: TimeSignature = TimeSignature::new(3, 4);
    const SIX_EIGHT: TimeSignature = TimeSignature::new(6, 8);
    const TWELVE_EIGHT: TimeSignature = TimeSignature::new(12, 8);

    /// One pattern of `bars` bars of 4/4, a note on every beat, played
    /// from the start of the song.
    fn quarter_notes(bars: usize) -> Project {
        let mut project = Project::empty();
        project.add_channel(crate::Channel::synth());
        let pattern = project.pattern_mut(0).unwrap();
        pattern.set_length(bars * 16);
        pattern.set_notes(
            0,
            (0..bars * 4).map(|beat| NoteEvent::new(Note::C4, beat * 4, 4)),
        );
        project.add_clip(Clip::new(0, 0, 0)).unwrap();
        project
    }

    #[test]
    fn every_note_keeps_its_place_in_the_bar() {
        let mut project = quarter_notes(2);
        assert!(remeter(&mut project, THREE_FOUR));
        assert_eq!(project.time_signature, THREE_FOUR);
        let pattern = project.pattern(0).unwrap();
        assert_eq!(pattern.length(), 24);
        // Beats 1 to 4 of each bar land on steps 0, 3, 6 and 9 of it:
        // a quarter of the bar, half of it, three quarters of it.
        let starts: Vec<usize> = pattern.notes(0).iter().map(|n| n.start).collect();
        assert_eq!(starts, [0, 3, 6, 9, 12, 15, 18, 21]);
        // Each is three steps long now, and still on the grid.
        assert!(
            pattern
                .notes(0)
                .iter()
                .all(|n| n.length == 3 && n.is_on_grid())
        );
    }

    #[test]
    fn notes_between_steps_keep_their_exact_place() {
        let mut project = quarter_notes(1);
        // Eighth notes: every two steps.
        project
            .pattern_mut(0)
            .unwrap()
            .set_notes(0, (0..8).map(|n| NoteEvent::new(Note::C4, n * 2, 2)));
        assert!(remeter(&mut project, THREE_FOUR));
        let notes = project.pattern(0).unwrap().notes(0);
        // 1.5 steps apart: every other one is 12 ticks after a step.
        let ticks: Vec<usize> = notes.iter().map(NoteEvent::start_tick).collect();
        assert_eq!(ticks, [0, 36, 72, 108, 144, 180, 216, 252]);
        assert!(notes.iter().all(|n| n.ticks() == 36));
    }

    #[test]
    fn the_same_number_of_steps_only_changes_the_signature() {
        let mut project = quarter_notes(1);
        project.time_signature = THREE_FOUR;
        project.pattern_mut(0).unwrap().set_length(12);
        let notes = project.pattern(0).unwrap().notes(0).to_vec();
        assert!(remeter(&mut project, SIX_EIGHT));
        assert_eq!(project.time_signature, SIX_EIGHT);
        assert_eq!(project.pattern(0).unwrap().notes(0), &notes[..]);
        assert!(!remeter(&mut project, SIX_EIGHT), "already in 6/8");
    }

    #[test]
    fn the_playlist_moves_with_the_bars() {
        let mut project = quarter_notes(2);
        project.add_clip(Clip::new(0, 0, 32).repeated(2)).unwrap();
        project.add_section(Section::new("Verse", 32, 64)).unwrap();
        project.set_loop_region(Some(crate::LoopRegion::new(16, 48)));
        assert!(project.set_tempo_marker(TempoMarker::gliding(64, 90.0, 16)));
        assert!(project.set_key_marker(KeyMarker::new(
            48,
            Scale::new(crate::PitchClass::G, crate::ScaleKind::Major)
        )));
        assert!(remeter(&mut project, THREE_FOUR));
        let starts: Vec<usize> = project.playlist().clips().iter().map(|c| c.start).collect();
        assert_eq!(starts, [0, 24]);
        assert_eq!(project.playlist().sections()[0].start, 24);
        assert_eq!(project.playlist().sections()[0].length, 48);
        assert_eq!(
            project.playlist().loop_region(),
            Some(crate::LoopRegion::new(12, 36))
        );
        let tempo = project.playlist().tempo_markers()[0];
        assert_eq!((tempo.start, tempo.glide), (48, 12));
        assert_eq!(project.playlist().key_markers()[0].start, 36);
        assert_eq!(project.song_length(), 24 * 3);
    }

    #[test]
    fn chords_land_on_the_nearest_beat() {
        let mut project = quarter_notes(2);
        let name = |text| crate::HarmonyName::parse(text).unwrap();
        let chords = [
            crate::ChordMarker::new(0, name("C")),
            crate::ChordMarker::new(8, name("G")),
            crate::ChordMarker::new(28, name("F")),
        ];
        assert!(project.set_chords(0, 32, &chords));
        assert!(remeter(&mut project, THREE_FOUR));
        let starts: Vec<usize> = project
            .playlist()
            .chord_markers()
            .iter()
            .map(|marker| marker.start)
            .collect();
        // Half a bar of 4/4 is a beat and a half of 3/4: G goes on to
        // the third beat, and F, on the last beat, to the third too.
        assert_eq!(starts, [0, 8, 20]);
    }

    #[test]
    fn a_change_of_time_signature_goes_away() {
        // Two bars of 4/4, then two of 3/4: four bars of 6/8.
        let mut project = quarter_notes(2);
        let mut waltz = Pattern::new("Waltz", 1, 24);
        waltz.set_notes(0, [NoteEvent::new(Note::A4, 12, 12)]);
        project.patterns.push(waltz);
        project.add_clip(Clip::new(1, 0, 32)).unwrap();
        assert!(project.set_time_marker(TimeMarker::new(32, THREE_FOUR)));
        assert!(remeter(&mut project, SIX_EIGHT));
        assert!(project.playlist().time_markers().is_empty());
        assert_eq!(project.playlist().clips()[1].start, 24);
        // The 3/4 bars have twelve steps, as 6/8 bars do: unchanged.
        assert_eq!(
            project.pattern(1).unwrap().notes(0),
            [NoteEvent::new(Note::A4, 12, 12)]
        );
        assert_eq!(project.song_length(), 48);
    }

    #[test]
    fn every_bar_lasts_as_long_as_before() {
        let mut project = quarter_notes(4);
        project.tempo = Tempo::new(112.0);
        assert!(project.set_tempo_marker(TempoMarker::gliding(32, 100.0, 16)));
        let seconds = |p: &Project| p.played_seconds(&p.arrangement());
        let before = seconds(&project);
        assert!(remeter(&mut project, THREE_FOUR));
        assert_eq!(project.tempo.bpm(), 84.0);
        assert_eq!(project.playlist().tempo_markers()[0].tempo.bpm(), 75.0);
        assert!(
            (seconds(&project) - before).abs() < 2e-3 * before,
            "{} vs {before}",
            seconds(&project)
        );
    }

    #[test]
    fn a_change_of_time_signature_keeps_its_pace_too() {
        // Two bars of 4/4 at 120, two of 3/4: in 6/8 the 4/4 bars slow
        // down to 90, and the 3/4 bars, already twelve steps, stay at 120.
        let mut project = quarter_notes(2);
        project.tempo = Tempo::new(120.0);
        let mut waltz = Pattern::new("Waltz", 1, 24);
        waltz.set_notes(0, [NoteEvent::new(Note::A4, 0, 12)]);
        project.patterns.push(waltz);
        project.add_clip(Clip::new(1, 0, 32)).unwrap();
        assert!(project.set_time_marker(TimeMarker::new(32, THREE_FOUR)));
        let seconds = |p: &Project| p.played_seconds(&p.arrangement());
        let before = seconds(&project);
        assert!(remeter(&mut project, SIX_EIGHT));
        assert_eq!(project.tempo.bpm(), 90.0);
        let marker = project.playlist().tempo_markers()[0];
        assert_eq!((marker.start, marker.tempo.bpm()), (24, 120.0));
        assert!(
            (seconds(&project) - before).abs() < 2e-3 * before,
            "{} vs {before}",
            seconds(&project)
        );
    }

    #[test]
    fn a_pattern_too_long_for_the_new_bars_is_cut() {
        // Sixteen bars of 4/4 are sixteen bars of 12/8: 384 steps. Ten
        // fit in a pattern.
        let mut project = quarter_notes(16);
        project.add_clip(Clip::new(0, 1, 256)).unwrap();
        assert!(remeter(&mut project, TWELVE_EIGHT));
        assert_eq!(project.patterns().len(), 2);
        assert_eq!(project.pattern(0).unwrap().length(), 240);
        assert_eq!(project.pattern(1).unwrap().length(), 144);
        assert_eq!(project.pattern(1).unwrap().name, "Pattern 1 (2)");
        let clips: Vec<(usize, usize, usize)> = project
            .playlist()
            .clips()
            .iter()
            .map(|c| (c.pattern, c.track, c.start))
            .collect();
        assert_eq!(clips, [(0, 0, 0), (1, 0, 240), (0, 1, 384), (1, 1, 624)]);
        // Every beat is still there: 64 of them.
        let notes =
            project.pattern(0).unwrap().notes(0).len() + project.pattern(1).unwrap().notes(0).len();
        assert_eq!(notes, 64);
        // Beat 1 of bar 11 opens the second piece.
        assert_eq!(project.pattern(1).unwrap().notes(0)[0].start, 0);
    }
}
