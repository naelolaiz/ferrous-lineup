//! Inserting and deleting bars: an empty bar put into the middle of a
//! song, or a bar taken out of it, with everything after it moving along.
//!
//! ```text
//! before:  | C    | F    | G7   | C    |
//! insert a bar after bar 2:
//! after:   | C    | F    | %    | G7   | C    |
//! ```
//!
//! An inserted bar is empty: the harmony going on before it goes on
//! through it, as a bar with nothing written in a chart does, and so do
//! the key, the tempo and the band's variation. It is in the time
//! signature of the bar it is inserted next to, and in that bar's section.
//!
//! Everything placed in the song after it moves later by a bar: clips,
//! sections, the loop, the time, tempo, key and chord markers, and the
//! band's variations and styles. Deleting a bar moves them all earlier,
//! and what was going on at the end of the deleted bar (its harmony,
//! time signature, key, tempo) goes on from where it was.
//!
//! The notes move too. A clip that plays across the place where a bar
//! goes in or comes out is opened up or closed: if no other clip plays
//! its pattern, the pattern itself gets the empty bar, or loses the bar's
//! notes. If other clips play it too, they must not change, so the clip
//! is cut in two there, each half playing a pattern of its own: "Verse
//! (1)" and "Verse (2)".

use crate::band::sections::{self, StyleMarker};
use crate::band::{VariationMarker, tidy_variations, variation_in};
use crate::chord_sheet::{self, ChordMarker};
use crate::pattern::{MAX_STEPS, TICKS_PER_STEP};
use crate::{Clip, KeyMarker, MAX_SONG_STEPS, NoteEvent, Project, TempoMarker, TimeMarker};

impl Project {
    /// Inserts `count` empty bars next to bar `bar` (counted from 0):
    /// after it if `after`, else before it. They are in its time
    /// signature and its section; the rest is as the module explains.
    /// Returns `false`, changing nothing, if the song would get too long
    /// or there is no room for the patterns it needs.
    pub fn insert_bars(&mut self, bar: usize, after: bool, count: usize) -> bool {
        self.moving_the_band(|project| project.open_bars(bar, after, count))
    }

    /// Deletes bars `first` to `last` (counted from 0), as the module
    /// explains. Returns `false`, changing nothing, if there are no such
    /// bars or there is no room for the patterns it needs.
    pub fn delete_bars(&mut self, first: usize, last: usize) -> bool {
        self.moving_the_band(|project| project.close_bars(first, last))
    }

    fn open_bars(&mut self, bar: usize, after: bool, count: usize) -> bool {
        let meter = self.meter();
        let bar = meter.bar(bar);
        let at = if after { bar.end() } else { bar.start };
        let steps = bar.length * count;
        if count == 0 || at >= MAX_SONG_STEPS {
            return false;
        }
        let before = self.clone();
        let section = self
            .playlist
            .sections
            .iter()
            .position(|section| section.contains(bar.start))
            .map(|index| (index, self.playlist.sections[index].clone()));
        if !self.open_clips(at, steps) || !self.make_room(at, steps) {
            *self = before;
            return false;
        }
        // The bars join the section of the bar they are next to.
        if let Some((index, section)) = section {
            let grown = &mut self.playlist.sections[index];
            grown.start = section.start;
            grown.length = section.length + steps;
        }
        // And are in its time signature: a change of time signature on
        // the bar they go before now comes on them.
        self.set_time_marker(TimeMarker::new(at, bar.time));
        self.playlist.time_markers = self.meter().tidy().markers().to_vec();
        true
    }

    fn close_bars(&mut self, first: usize, last: usize) -> bool {
        let meter = self.meter();
        if first > last {
            return false;
        }
        let start = meter.bar(first).start;
        let end = meter.bar(last).end();
        let steps = end - start;
        let before = self.clone();
        if !self.close_clips(start, end) {
            *self = before;
            return false;
        }
        // What goes on at the end of the bars goes on from their start.
        let time = meter.time_at(end);
        let tempo = self.tempo_map().tempo_at(end);
        let key = self.key_at(end);
        let going_on = self
            .chord_at(end)
            .filter(|marker| (start..end).contains(&marker.start));
        let variation = self
            .band
            .as_ref()
            .map(|band| variation_in(&band.variations, end));
        let style = self
            .band
            .as_ref()
            .map(|band| sections::style_in(&band.styles, end));

        // Where a step of the song goes: the deleted bars close up.
        let place = |step: usize| match step {
            step if step <= start => step,
            step if step >= end => step - steps,
            _ => start,
        };
        let inside = |step: usize| (start..end).contains(&step);
        let playlist = &mut self.playlist;
        for clip in &mut playlist.clips {
            clip.start = place(clip.start);
        }
        for section in &mut playlist.sections {
            let section_end = place(section.end());
            section.start = place(section.start);
            section.length = section_end - section.start;
        }
        playlist.sections.retain(|section| section.length > 0);
        playlist.loop_region = playlist.loop_region.and_then(|region| {
            let (from, to) = (place(region.start), place(region.end));
            (from < to).then_some(crate::LoopRegion::new(from, to))
        });
        playlist.time_markers.retain(|marker| !inside(marker.start));
        for marker in &mut playlist.time_markers {
            marker.start = place(marker.start);
        }
        playlist
            .tempo_markers
            .retain(|marker| !inside(marker.start));
        for marker in &mut playlist.tempo_markers {
            let glide_end = place(marker.start + marker.glide);
            marker.start = place(marker.start);
            marker.glide = glide_end - marker.start;
        }
        playlist.key_markers.retain(|marker| !inside(marker.start));
        for marker in &mut playlist.key_markers {
            marker.start = place(marker.start);
        }
        playlist
            .chord_markers
            .retain(|marker| !inside(marker.start));
        for marker in &mut playlist.chord_markers {
            marker.start = place(marker.start);
        }
        if let Some(band) = &mut self.band {
            band.end = place(band.end);
            band.form.intro = place(band.form.intro);
            band.variations.retain(|marker| !inside(marker.start));
            for marker in &mut band.variations {
                marker.start = place(marker.start);
            }
            band.styles.retain(|marker| !inside(marker.start));
            for marker in &mut band.styles {
                marker.start = place(marker.start);
            }
        }

        self.set_time_marker(TimeMarker::new(start, time));
        self.set_tempo_marker(TempoMarker {
            glide: 0,
            ..TempoMarker::new(start, tempo.bpm())
        });
        if start == 0 {
            self.scale = key;
        } else {
            self.set_key_marker(KeyMarker::new(start, key));
        }
        // A harmony that started in the deleted bars and went on past
        // them now starts where they were, unless it is the one going on
        // already.
        if let Some(marker) = going_on
            && self.playlist.chord_markers.iter().all(|m| m.start != start)
            && start
                .checked_sub(1)
                .and_then(|step| self.chord_at(step))
                .is_none_or(|before| before.harmony != marker.harmony)
        {
            self.playlist
                .chord_markers
                .push(ChordMarker::new(start, marker.harmony));
        }
        chord_sheet::tidy(&mut self.playlist.chord_markers, MAX_SONG_STEPS);
        if let Some(band) = &mut self.band {
            if let Some(variation) = variation {
                band.variations.push(VariationMarker::new(start, variation));
                tidy_variations(&mut band.variations);
            }
            if let Some(style) = style {
                band.styles.push(StyleMarker::new(start, style));
                sections::tidy(&mut band.styles);
            }
        }
        self.playlist.time_markers = self.meter().tidy().markers().to_vec();
        self.playlist.tempo_markers = self.tempo_map().tidy().markers().to_vec();
        self.playlist.key_markers = self.key_map().tidy().markers().to_vec();
        true
    }

    /// Opens every clip that plays across step `at` by `steps` empty
    /// steps there: its pattern gets them if no other clip plays it and
    /// it has room, or else the clip is cut in two at `at` (see
    /// [`Project::cut_clip`]), so that the room made there is empty.
    fn open_clips(&mut self, at: usize, steps: usize) -> bool {
        let mut index = 0;
        while index < self.playlist.clips.len() {
            let clip = self.playlist.clips[index];
            let length = self.patterns[clip.pattern].length();
            let crosses = clip.start < at && at < clip.end(length);
            if crosses && self.only_clip(index) && length + steps <= MAX_STEPS {
                // The step of the pattern played there.
                let inside = at - clip.start + clip.first_step(length);
                let pattern = &mut self.patterns[clip.pattern];
                for channel in 0..pattern.rows() {
                    let notes = opened(pattern.notes(channel), inside, steps);
                    pattern.set_notes(channel, notes);
                }
                pattern.set_length(length + steps);
                index += 1;
                continue;
            }
            let Some(pieces) = self.cut_clip(index, at) else {
                return false;
            };
            index += pieces;
        }
        true
    }

    /// Takes the steps from `start` to `end` out of every clip, with the
    /// notes in them: a clip that plays only there goes, and the clips
    /// after them stay where they are, for the room to close up
    /// afterwards.
    fn close_clips(&mut self, start: usize, end: usize) -> bool {
        let made = self.patterns.len();
        let mut index = 0;
        while index < self.playlist.clips.len() {
            let clip = self.playlist.clips[index];
            let length = self.patterns[clip.pattern].length();
            let clip_end = clip.end(length);
            if clip_end <= start || clip.start >= end {
                index += 1;
                continue;
            }
            // The steps of the pattern in them.
            let first = clip.first_step(length);
            let from = start.max(clip.start) - clip.start + first;
            let to = end.min(clip_end) - clip.start + first;
            if self.only_clip(index) && to - from < clip.length(length) {
                let pattern = &mut self.patterns[clip.pattern];
                for channel in 0..pattern.rows() {
                    let notes = closed(pattern.notes(channel), from, to);
                    pattern.set_notes(channel, notes);
                }
                pattern.set_length(length - (to - from));
                // A clip that started inside them starts where they end,
                // and so where they were once they close up.
                if clip.start >= start {
                    self.playlist.clips[index].start = end;
                }
                index += 1;
                continue;
            }
            // Cut where they start, and the piece after that where they
            // end.
            let Some(pieces) = self.cut_clip(index, start) else {
                return false;
            };
            let last = index + pieces - 1;
            let Some(more) = self.cut_clip(last, end) else {
                return false;
            };
            index = last + more;
        }
        self.playlist
            .clips
            .retain(|clip| !(start..end).contains(&clip.start));
        // A half cut off only to go is not kept.
        for pattern in (made..self.patterns.len()).rev() {
            if self
                .playlist
                .clips
                .iter()
                .all(|clip| clip.pattern != pattern)
            {
                self.remove_pattern(pattern);
            }
        }
        true
    }

    /// Whether clip `index` plays its pattern once and no other clip
    /// plays it, so the pattern can change with it.
    fn only_clip(&self, index: usize) -> bool {
        let clip = self.playlist.clips[index];
        clip.repeats == 1
            && self
                .playlist
                .clips
                .iter()
                .enumerate()
                .all(|(other, playing)| other == index || playing.pattern != clip.pattern)
    }

    /// Cuts clip `index` in two if it plays across step `at`, one clip
    /// ending there and one starting there, and returns how many clips
    /// it is now, in its place in the list. A cut between two repeats of
    /// its pattern makes two clips of it; a cut inside one plays that
    /// repeat's two halves from patterns of their own (see
    /// [`Project::cut_pattern`]). A trimmed clip's pieces leave out what
    /// it left out. Returns `None`, changing nothing, if there are too
    /// many patterns for them.
    fn cut_clip(&mut self, index: usize, at: usize) -> Option<usize> {
        let clip = self.playlist.clips[index];
        let length = self.patterns[clip.pattern].length();
        if !(clip.start < at && at < clip.end(length)) {
            return Some(1);
        }
        let skip = clip.first_step(length);
        // Counted from where the pattern starts its first time round.
        let offset = at - clip.start + skip;
        let (whole, part) = (offset / length, offset % length);
        // Where time round `round` starts, or would start untrimmed.
        let round_start = |round: usize| clip.start + round * length - skip;
        let last = whole + 1 == clip.repeats;
        let mut pieces = Vec::new();
        if whole > 0 {
            pieces.push(Clip {
                repeats: whole,
                cut: 0,
                ..clip
            });
        }
        let mut rest = clip.repeats - whole;
        if part > 0 {
            let alone = self.only_clip(index);
            let (head, tail) = self.cut_pattern(clip.pattern, part, alone)?;
            pieces.push(match whole {
                0 => Clip {
                    skip,
                    ..Clip::new(head, clip.track, clip.start)
                },
                _ => Clip::new(head, clip.track, round_start(whole)),
            });
            pieces.push(Clip {
                cut: if last { clip.cut } else { 0 },
                ..Clip::new(tail, clip.track, at)
            });
            rest -= 1;
        }
        if rest > 0 {
            pieces.push(Clip {
                start: round_start(clip.repeats - rest),
                repeats: rest,
                skip: 0,
                ..clip
            });
        }
        let count = pieces.len();
        self.playlist.clips.splice(index..=index, pieces);
        Some(count)
    }

    /// The two halves of pattern `index`, cut at step `at`, as patterns,
    /// named after it with (1) and (2): the first is the pattern itself,
    /// cut short and keeping its name, if `alone` (no clip but the one
    /// being cut plays it). The band keeps its parts' halves as its own.
    fn cut_pattern(&mut self, index: usize, at: usize, alone: bool) -> Option<(usize, usize)> {
        let pattern = self.patterns[index].clone();
        let at_tick = at * TICKS_PER_STEP;
        let half = |name: String, length: usize, keep: &dyn Fn(&NoteEvent) -> Option<NoteEvent>| {
            let mut half = pattern.clone();
            half.name = name;
            half.set_length(length);
            for channel in 0..pattern.rows() {
                let notes: Vec<NoteEvent> =
                    pattern.notes(channel).iter().filter_map(keep).collect();
                half.set_notes(channel, notes);
            }
            half
        };
        let head = half(
            match alone {
                true => pattern.name.clone(),
                false => format!("{} (1)", pattern.name),
            },
            at,
            &|note| (note.start_tick() < at_tick).then(|| cut_short(note, at_tick)),
        );
        let tail = half(
            format!("{} (2)", pattern.name),
            pattern.length() - at,
            &|note| (note.start_tick() >= at_tick).then(|| moved(note, at_tick)),
        );
        let new = if alone { 1 } else { 2 };
        if self.patterns.len() + new > crate::MAX_PATTERNS {
            return None;
        }
        let head = match alone {
            true => {
                self.patterns[index] = head;
                index
            }
            false => self.push_pattern(head)?,
        };
        let tail = self.push_pattern(tail)?;
        // A new pattern takes the next colour; the halves keep the
        // pattern's own, so the clips look as they did.
        for half in [head, tail] {
            self.patterns[half].color = pattern.color;
        }
        Some((head, tail))
    }
}

/// `note` ending by tick `end` at the latest.
fn cut_short(note: &NoteEvent, end: usize) -> NoteEvent {
    let start = note.start_tick();
    NoteEvent {
        velocity: note.velocity,
        ..NoteEvent::exact(note.pitch, start, note.end_tick().min(end) - start)
    }
}

/// `note` moved `ticks` earlier.
fn moved(note: &NoteEvent, ticks: usize) -> NoteEvent {
    NoteEvent {
        velocity: note.velocity,
        ..NoteEvent::exact(note.pitch, note.start_tick() - ticks, note.ticks())
    }
}

/// `notes` with `steps` empty steps put in at step `at`: the notes from
/// there on move later, and a note held across it ends there.
fn opened(notes: &[NoteEvent], at: usize, steps: usize) -> Vec<NoteEvent> {
    let at_tick = at * TICKS_PER_STEP;
    notes
        .iter()
        .map(|note| match note.start_tick() < at_tick {
            true => cut_short(note, at_tick),
            false => NoteEvent {
                start: note.start + steps,
                ..*note
            },
        })
        .collect()
}

/// `notes` with the steps from `from` to `to` taken out: the notes in
/// them go, the notes after them move earlier, and a note held into them
/// ends where they start.
fn closed(notes: &[NoteEvent], from: usize, to: usize) -> Vec<NoteEvent> {
    let (from_tick, to_tick) = (from * TICKS_PER_STEP, to * TICKS_PER_STEP);
    notes
        .iter()
        .filter_map(|note| {
            let start = note.start_tick();
            if start < from_tick {
                Some(cut_short(note, from_tick))
            } else if start >= to_tick {
                Some(moved(note, to_tick - from_tick))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Channel, HarmonyName, LoopRegion, Note, Player, Scale, Section, TimeSignature};

    fn chord(start: usize, name: &str) -> ChordMarker {
        ChordMarker::new(start, HarmonyName::parse(name).unwrap())
    }

    fn chords(project: &Project) -> Vec<(usize, String)> {
        project
            .playlist()
            .chord_markers()
            .iter()
            .map(|marker| (marker.start, marker.harmony.to_string()))
            .collect()
    }

    fn starts(pattern: &crate::Pattern) -> Vec<(usize, usize)> {
        pattern
            .notes(0)
            .iter()
            .map(|note| (note.start, note.length))
            .collect()
    }

    /// Four bars of 4/4: C F G7 C, a melody of a note on each bar's
    /// first beat and one held from bar 2 into bar 3, played by one
    /// clip, a verse over all of it, a loop over bars 2 and 3, the key
    /// of G from bar 3 and 90 BPM from bar 4.
    fn song() -> Project {
        let mut project = Project::empty();
        project.add_channel(Channel::synth());
        let pattern = project.pattern_mut(0).unwrap();
        pattern.set_length(64);
        pattern.set_notes(
            0,
            [
                NoteEvent::new(Note::new(60), 0, 4),
                NoteEvent::new(Note::new(65), 16, 4),
                NoteEvent::new(Note::new(69), 24, 16),
                NoteEvent::new(Note::new(67), 32, 4),
                NoteEvent::new(Note::new(72), 48, 16),
            ],
        );
        project.add_clip(Clip::new(0, 0, 0)).unwrap();
        project.set_chords(
            0,
            64,
            &[
                chord(0, "C"),
                chord(16, "F"),
                chord(32, "G7"),
                chord(48, "C"),
            ],
        );
        project.add_section(Section::new("Verse", 0, 64)).unwrap();
        assert!(project.set_loop_region(Some(LoopRegion::new(16, 48))));
        project.set_key_marker(KeyMarker::new(
            32,
            Scale::new(crate::PitchClass::G, crate::ScaleKind::Major),
        ));
        project.set_tempo_marker(TempoMarker::new(48, 90.0));
        project
    }

    #[test]
    fn an_inserted_bar_moves_everything_after_it() {
        let mut project = song();
        // After bar 2, the F.
        assert!(project.insert_bars(1, true, 1));
        let melody = &project.patterns()[0];
        assert_eq!(melody.length(), 80);
        // The note held into the new bar ends where it starts; the notes
        // after it move a bar later.
        assert_eq!(
            starts(melody),
            [(0, 4), (16, 4), (24, 8), (48, 4), (64, 16)]
        );
        assert_eq!(project.playlist().clips().len(), 1);
        // The new bar is empty: the F goes on through it.
        assert_eq!(
            chords(&project),
            [
                (0, "C".into()),
                (16, "F".into()),
                (48, "G7".into()),
                (64, "C".into())
            ]
        );
        assert_eq!(project.key_at(32), Scale::default());
        assert_eq!(
            project.key_at(48),
            Scale::new(crate::PitchClass::G, crate::ScaleKind::Major)
        );
        assert_eq!(project.tempo_map().tempo_at(63).bpm(), project.tempo.bpm());
        assert_eq!(project.tempo_map().tempo_at(64).bpm(), 90.0);
        let verse = &project.playlist().sections()[0];
        assert_eq!((verse.start, verse.length), (0, 80));
        assert_eq!(
            project.playlist().loop_region(),
            Some(LoopRegion::new(16, 64))
        );
        assert_eq!(project.song_length(), 80);
    }

    #[test]
    fn a_bar_inserted_before_a_change_of_time_takes_the_new_time() {
        let waltz = TimeSignature::new(3, 4);
        let mut project = song();
        project.set_time_marker(TimeMarker::new(32, waltz));
        // Bar 3 is the first waltz bar: a bar before it is a waltz bar
        // too, twelve steps long.
        assert!(project.insert_bars(2, false, 1));
        let meter = project.meter();
        assert_eq!(meter.bar(2).start, 32);
        assert_eq!(meter.bar(2).time, waltz);
        assert_eq!(meter.bar(3).start, 44);
        assert_eq!(project.playlist().time_markers().len(), 1);
        // The G7 waits for the bar it was written in.
        assert_eq!(project.chord_at(43).map(|m| m.start), Some(16));
        assert_eq!(project.chord_at(44).map(|m| m.start), Some(44));
    }

    #[test]
    fn a_clip_of_a_pattern_played_elsewhere_too_is_cut_in_two() {
        let mut project = Project::empty();
        project.add_channel(Channel::synth());
        let riff = project.pattern_mut(0).unwrap();
        riff.name = "Riff".into();
        riff.set_length(32);
        riff.set_notes(
            0,
            [
                NoteEvent::new(Note::new(60), 0, 4),
                NoteEvent::new(Note::new(62), 16, 4),
            ],
        );
        project.add_clip(Clip::new(0, 0, 0)).unwrap();
        project.add_clip(Clip::new(0, 0, 64)).unwrap();
        let riff = project.patterns()[0].clone();
        assert!(project.insert_bars(0, true, 1));
        // The riff the other clip plays is as it was.
        assert_eq!(project.patterns()[0], riff);
        let clips: Vec<(String, usize)> = project
            .playlist()
            .clips()
            .iter()
            .map(|clip| (project.patterns()[clip.pattern].name.clone(), clip.start))
            .collect();
        assert_eq!(
            clips,
            [
                ("Riff (1)".into(), 0),
                ("Riff (2)".into(), 32),
                ("Riff".into(), 80)
            ]
        );
        let halves = &project.patterns()[1..];
        assert_eq!(starts(&halves[0]), [(0, 4)]);
        assert_eq!(starts(&halves[1]), [(0, 4)]);
        assert_eq!(halves[1].length(), 16);
    }

    #[test]
    fn a_trimmed_clip_cut_in_two_still_leaves_out_what_it_did() {
        let mut project = Project::empty();
        project.add_channel(Channel::synth());
        let riff = project.pattern_mut(0).unwrap();
        riff.set_length(32);
        riff.set_notes(
            0,
            [
                NoteEvent::new(Note::new(60), 0, 4),
                NoteEvent::new(Note::new(62), 16, 4),
                NoteEvent::new(Note::new(64), 28, 4),
            ],
        );
        project.add_clip(Clip::new(0, 0, 0)).unwrap();
        project.add_clip(Clip::new(0, 0, 64)).unwrap();
        // Without its first note, and with its last one cut short.
        let clip = project.playlist().clips()[0];
        let trimmed = clip
            .trimmed_from(4, 32)
            .unwrap()
            .trimmed_to(30, 32)
            .unwrap();
        assert!(project.trim_clip(0, trimmed));
        let heard = |project: &Project, end: usize| -> Vec<(f32, f32, u8)> {
            let mut notes: Vec<_> = project
                .heard_notes(0, end)
                .iter()
                .map(|note| (note.start, note.end, note.pitch.midi()))
                .collect();
            notes.sort_by(|a, b| a.0.total_cmp(&b.0));
            notes
        };
        assert_eq!(heard(&project, 64), [(16.0, 20.0, 62), (28.0, 30.0, 64)]);
        // A bar put in after the first: the clip is cut where it goes.
        assert!(project.insert_bars(0, true, 1));
        assert_eq!(heard(&project, 80), [(32.0, 36.0, 62), (44.0, 46.0, 64)]);
        assert!(project.delete_bars(1, 1));
        assert_eq!(heard(&project, 64), [(16.0, 20.0, 62), (28.0, 30.0, 64)]);
    }

    #[test]
    fn a_deleted_bar_leaves_no_pattern_behind() {
        let mut project = Project::empty();
        project.add_channel(Channel::synth());
        let riff = project.pattern_mut(0).unwrap();
        riff.name = "Riff".into();
        riff.set_length(32);
        riff.set_notes(
            0,
            [
                NoteEvent::new(Note::new(60), 0, 4),
                NoteEvent::new(Note::new(62), 16, 4),
            ],
        );
        project.add_clip(Clip::new(0, 0, 0)).unwrap();
        project.add_clip(Clip::new(0, 0, 64)).unwrap();
        // The second bar of the first riff: its first half stays.
        assert!(project.delete_bars(1, 1));
        let names: Vec<&str> = project
            .patterns()
            .iter()
            .map(|pattern| pattern.name.as_str())
            .collect();
        assert_eq!(names, ["Riff", "Riff (1)"]);
        assert_eq!(
            project.playlist().clips(),
            [Clip::new(1, 0, 0), Clip::new(0, 0, 48)]
        );
        // A riff played once, from bar 2, loses its first bar.
        let mut project = Project::empty();
        project.add_channel(Channel::synth());
        let riff = project.pattern_mut(0).unwrap();
        riff.set_length(32);
        riff.set_notes(
            0,
            [
                NoteEvent::new(Note::new(60), 0, 4),
                NoteEvent::new(Note::new(62), 16, 4),
            ],
        );
        project.add_clip(Clip::new(0, 0, 16)).unwrap();
        assert!(project.delete_bars(1, 1));
        assert_eq!(project.playlist().clips(), [Clip::new(0, 0, 16)]);
        assert_eq!(project.patterns()[0].length(), 16);
        assert_eq!(starts(&project.patterns()[0]), [(0, 4)]);
        assert_eq!(project.patterns()[0].notes(0)[0].pitch, Note::new(62));
    }

    #[test]
    fn a_deleted_bar_closes_up() {
        let mut project = song();
        // Bar 3, the G7.
        assert!(project.delete_bars(2, 2));
        let melody = &project.patterns()[0];
        assert_eq!(melody.length(), 48);
        assert_eq!(starts(melody), [(0, 4), (16, 4), (24, 8), (32, 16)]);
        assert_eq!(project.playlist().clips(), [Clip::new(0, 0, 0)]);
        assert_eq!(
            chords(&project),
            [(0, "C".into()), (16, "F".into()), (32, "C".into())]
        );
        // The key bar 4 was in goes on from where bar 3 was.
        assert_eq!(
            project.key_at(32),
            Scale::new(crate::PitchClass::G, crate::ScaleKind::Major)
        );
        assert_eq!(project.tempo_map().tempo_at(32).bpm(), 90.0);
        let verse = &project.playlist().sections()[0];
        assert_eq!((verse.start, verse.length), (0, 48));
        assert_eq!(
            project.playlist().loop_region(),
            Some(LoopRegion::new(16, 32))
        );
    }

    #[test]
    fn a_harmony_from_a_deleted_bar_goes_on_where_it_was() {
        let mut project = song();
        // C, F, then G7 held through bar 4.
        project.set_chords(48, 64, &[]);
        assert!(project.delete_bars(2, 2));
        assert_eq!(
            chords(&project),
            [(0, "C".into()), (16, "F".into()), (32, "G7".into())]
        );
        // Deleting the F's bar when the bar before is an F too leaves
        // nothing to write.
        let mut project = song();
        project.set_chords(0, 16, &[chord(0, "F")]);
        assert!(project.delete_bars(1, 1));
        assert_eq!(
            chords(&project),
            [(0, "F".into()), (16, "G7".into()), (32, "C".into())]
        );
    }

    #[test]
    fn a_repeated_clip_loses_the_repeats_in_the_deleted_bars() {
        let mut project = Project::empty();
        project.add_channel(Channel::synth());
        project.pattern_mut(0).unwrap().set_length(16);
        project.add_clip(Clip::new(0, 0, 0).repeated(4)).unwrap();
        assert!(project.delete_bars(1, 1));
        let clips: Vec<(usize, usize)> = project
            .playlist()
            .clips()
            .iter()
            .map(|clip| (clip.start, clip.repeats))
            .collect();
        assert_eq!(clips, [(0, 1), (16, 2)]);
        assert_eq!(project.patterns().len(), 1);
    }

    #[test]
    fn the_band_follows_the_bars() {
        let mut project = Project::with_band();
        project.write_progression(crate::library::Style::named("Four chords").unwrap(), 0);
        project.follow_chord_sheet();
        let bars = project.meter().bar_at(project.song_length() - 1).number + 1;
        assert!(project.insert_bars(1, true, 1));
        project.follow_chord_sheet();
        assert_eq!(
            project.meter().bar_at(project.song_length() - 1).number + 1,
            bars + 1
        );
        for player in [Player::Piano, Player::Bass, Player::Drums] {
            assert!(!project.is_frozen(player), "{player:?}");
        }
        assert!(project.delete_bars(0, 1));
        project.follow_chord_sheet();
        assert_eq!(
            project.meter().bar_at(project.song_length() - 1).number + 1,
            bars - 1
        );
        for player in [Player::Piano, Player::Bass, Player::Drums] {
            assert!(!project.is_frozen(player), "{player:?}");
        }
    }
}
