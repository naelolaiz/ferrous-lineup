//! Applying library components to a pattern without replacing unrelated parts.

use super::*;
use crate::library::chart::parse_chart;
use crate::pattern::{MAX_STEPS, TICKS_PER_STEP};
use crate::{BandStyle, Meter, Note};

fn meter(project: &mut Project, index: usize, time: TimeSignature) {
    let old = project.pattern_time(index);
    if old == time {
        return;
    }
    let map = crate::remeter::Remap::new(Meter::new(old, []), time);
    let own_song = !song_has_music(project, index);
    let Some(pattern) = project.pattern_mut(index) else {
        return;
    };
    let notes: Vec<Vec<_>> = (0..pattern.rows())
        .map(|channel| {
            pattern
                .notes(channel)
                .iter()
                .map(|note| {
                    let start = map.tick(note.start_tick());
                    NoteEvent {
                        velocity: note.velocity,
                        ..NoteEvent::exact(
                            note.pitch,
                            start,
                            map.tick(note.end_tick()).saturating_sub(start).max(1),
                        )
                    }
                })
                .collect()
        })
        .collect();
    pattern.set_length(map.step(pattern.length()).clamp(1, MAX_STEPS));
    pattern.time = Some(time);
    for (channel, notes) in notes.into_iter().enumerate() {
        pattern.set_notes(channel, notes);
    }
    if own_song {
        project.time_signature = time;
    }
}

/// Change the sounds of the existing chord and bass voices. Locked
/// channels keep theirs, and a locked `selected` channel counts as none.
pub fn style_instruments(project: &mut Project, style: &Style, selected: Option<usize>) {
    let Some(band) = BandStyle::of_style(style) else {
        return;
    };
    let chords = selected
        .filter(|&index| !project.is_locked(index))
        .or_else(|| find_synth(project, KEYS));
    if let Some(index) = chords
        && project.channels().get(index).is_some_and(plays_notes)
    {
        project.set_sound(index, None, Some(band.piano_program()));
    }
    if let Some(index) = find_synth(project, BASS) {
        project.set_sound(index, None, Some(band.bass_program()));
    }
}

/// Put new chords under an existing performance, keeping its attacks,
/// velocities and channels. Held notes split where the new harmony changes.
/// Each pattern repeats the chart.
///
/// A pattern whose chords or bass line are locked keeps its chords: the
/// other part could not take new ones without clashing with it.
pub(super) fn progression(project: &mut Project, index: usize, style: &Style) -> bool {
    let Some(chords) = heard_chords_channel(project, index) else {
        return false;
    };
    let bass = find_synth(project, BASS);
    let plays_locked = |channel: usize| {
        project.is_locked(channel)
            && project
                .pattern(index)
                .is_some_and(|pattern| !pattern.notes(channel).is_empty())
    };
    if plays_locked(chords) || bass.is_some_and(plays_locked) {
        return false;
    }
    let time = project.pattern_time(index);
    let Some(spans) = parse_chart(style.chart, time.steps_per_bar()) else {
        return false;
    };
    let length = style.bars() * time.steps_per_bar();
    if length == 0 {
        return false;
    }
    let tonic = project.scale.root;
    let Some(mut pattern) = project.style_pattern(index) else {
        return false;
    };
    for channel in std::iter::once(chords).chain(bass) {
        let original = pattern.notes(channel).to_vec();
        let mut notes = Vec::new();
        for note in &original {
            let simultaneous: Vec<_> = original
                .iter()
                .filter(|other| other.start_tick() == note.start_tick())
                .collect();
            let rank = simultaneous
                .iter()
                .filter(|other| other.pitch < note.pitch)
                .count();
            let mut start = note.start_tick();
            let end = note.end_tick().min(pattern.length() * TICKS_PER_STEP);
            while start < end {
                let at = start / TICKS_PER_STEP % length;
                let Some(span) = spans
                    .iter()
                    .find(|span| (span.start..span.start + span.length).contains(&at))
                else {
                    break;
                };
                let chord = span.chord.in_key(tonic);
                let voiced = voice(chord, None);
                let pitch = if Some(channel) == bass {
                    bass_root(chord)
                } else if simultaneous.len() == 1 {
                    (0..=127)
                        .map(Note::new)
                        .filter(|pitch| {
                            voiced
                                .iter()
                                .any(|tone| tone.pitch_class() == pitch.pitch_class())
                        })
                        .min_by_key(|pitch| pitch.midi().abs_diff(note.pitch.midi()))
                        .unwrap()
                } else {
                    let pc = voiced[rank % voiced.len()].pitch_class();
                    (0..=127)
                        .map(Note::new)
                        .filter(|pitch| pitch.pitch_class() == pc)
                        .min_by_key(|pitch| pitch.midi().abs_diff(note.pitch.midi()))
                        .unwrap()
                };
                let cycle = start / (length * TICKS_PER_STEP);
                let stop = end.min((cycle * length + span.start + span.length) * TICKS_PER_STEP);
                notes.push(NoteEvent {
                    velocity: note.velocity,
                    ..NoteEvent::exact(pitch, start, stop - start)
                });
                start = stop;
            }
        }
        pattern.set_notes(channel, notes);
    }
    true
}

/// Apply only the checked components to the selected audition pattern.
pub fn write_style_parts(
    project: &mut Project,
    index: usize,
    style: &Style,
    selected: Option<usize>,
    parts: StyleParts,
) {
    let parts = parts.following(style.follows_the_song());
    let empty = project
        .pattern(index)
        .is_some_and(|pattern| pattern.is_empty());
    let time = if parts.time_signature {
        style.time
    } else {
        project.pattern_time(index)
    };
    if empty && (parts.matching_style || parts.progression) {
        // An empty audition needs some harmony to demonstrate the style:
        // its chart, played its way, or held without its chord parts.
        let scale = project.scale;
        let old_time = project.time_signature;
        let seed = match parts.matching_style {
            true => Style { time, ..*style },
            false => Style {
                time,
                comping: Comping::Held,
                bass: Bass::Held,
                ..*style
            },
        };
        write_style(project, index, &seed, scale.root, selected);
        project.scale = scale;
        if !parts.time_signature {
            project.time_signature = old_time;
        }
    } else if empty && parts.rhythm {
        // The drums alone, as long as the style.
        if !song_has_music(project, index) {
            project.time_signature = time;
        }
        if let Some(pattern) = project.pattern_mut(index) {
            pattern.set_length((style.bars() * time.steps_per_bar()).min(MAX_STEPS));
            pattern.time = Some(time);
        }
    } else {
        if parts.time_signature {
            meter(project, index, style.time);
        }
        if parts.matching_style {
            recomp(project, index, style.comping, style.bass);
        }
        if parts.progression {
            progression(project, index, style);
        }
    }
    if parts.rhythm
        && let Some(rhythm) = Rhythm::named(style.rhythm)
    {
        write_rhythm(project, index, rhythm);
    }
    if parts.instruments {
        style_instruments(project, style, selected);
    }
    if parts.swing {
        project.swing = Rhythm::named(style.rhythm).map_or(0, |rhythm| rhythm.swing);
    }
}

pub fn write_rhythm_parts(project: &mut Project, index: usize, rhythm: &Rhythm, parts: StyleParts) {
    let parts = parts.following(rhythm.follows_the_meter);
    if project
        .pattern(index)
        .is_some_and(|pattern| pattern.is_empty())
        && parts.rhythm
    {
        let time = if parts.time_signature {
            rhythm.time
        } else {
            project.pattern_time(index)
        };
        if let Some(pattern) = project.pattern_mut(index) {
            pattern.set_length((rhythm.bars() * time.steps_per_bar()).min(MAX_STEPS));
            pattern.time = Some(time);
        }
    }
    if parts.time_signature {
        meter(project, index, rhythm.time);
    }
    if parts.matching_style {
        let (comping, bass) = feel(rhythm);
        recomp(project, index, comping, bass);
    }
    if parts.rhythm {
        write_rhythm(project, index, rhythm);
    }
    if parts.instruments
        && let Some(style) = STYLES.iter().find(|style| style.rhythm == rhythm.name)
    {
        style_instruments(project, style, None);
    }
    if parts.swing {
        project.swing = rhythm.swing;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HarmonyName;

    fn no_parts() -> StyleParts {
        StyleParts {
            rhythm: false,
            instruments: false,
            time_signature: false,
            progression: false,
            tempo: false,
            swing: false,
            matching_style: false,
        }
    }

    fn audition() -> Project {
        let mut project = Project::empty();
        write_style(
            &mut project,
            0,
            Style::named("Four chords").unwrap(),
            PitchClass::C,
            None,
        );
        write_rhythm(&mut project, 0, Rhythm::named("Pop beat").unwrap());
        project
    }

    #[test]
    fn unchecked_components_leave_the_audition_unchanged() {
        let mut project = audition();
        let before = project.clone();
        write_style_parts(
            &mut project,
            0,
            Style::named("Emotional pop").unwrap(),
            None,
            no_parts(),
        );
        assert_eq!(project, before);
    }

    #[test]
    fn a_rhythms_drums_alone_keep_the_chords_sounds_and_meter() {
        let mut project = audition();
        let keys = find_synth(&project, KEYS).unwrap();
        project.channel_mut(keys).unwrap().program = Some(80);
        let before = project.clone();
        write_rhythm_parts(
            &mut project,
            0,
            Rhythm::named("Rachenitsa").unwrap(),
            StyleParts {
                rhythm: true,
                ..no_parts()
            },
        );
        assert_eq!(project.time_signature, before.time_signature);
        assert_eq!(project.pattern_time(0), before.pattern_time(0));
        assert_eq!(project.scale, before.scale);
        assert_eq!(project.channels()[keys].program, Some(80));
        assert_eq!(
            project.pattern(0).unwrap().notes(keys),
            before.pattern(0).unwrap().notes(keys)
        );
        assert_ne!(project.pattern(0), before.pattern(0), "the drums came");
    }

    #[test]
    fn a_styles_drums_alone_keep_the_patterns_chords_and_sounds() {
        for empty in [false, true] {
            let mut project = audition();
            let keys = find_synth(&project, KEYS).unwrap();
            project.channel_mut(keys).unwrap().program = Some(80);
            if empty {
                let pattern = project.pattern_mut(0).unwrap();
                for channel in 0..pattern.rows() {
                    pattern.set_notes(channel, []);
                }
            }
            let before = project.clone();
            let style = Style::named("Emotional pop").unwrap();
            write_style_parts(
                &mut project,
                0,
                style,
                None,
                StyleParts {
                    rhythm: true,
                    ..no_parts()
                },
            );
            assert_eq!(project.channels()[keys].program, Some(80));
            assert_eq!(
                project.pattern(0).unwrap().notes(keys),
                before.pattern(0).unwrap().notes(keys)
            );
            let drums = drums_in(&project, 0, 0, project.pattern(0).unwrap().length());
            assert!(!drums.is_empty());
            if empty {
                assert_eq!(project.pattern(0).unwrap().length(), style.length());
            }
        }
    }

    #[test]
    fn an_empty_pattern_takes_a_styles_drums_alone_in_its_meter() {
        use crate::{ChordMarker, Clip};
        // A song of its own in 4/4, and an empty pattern to try a waltz.
        let mut project = Project::empty();
        project.pattern_mut(0).unwrap().set_length(32);
        project.add_clip(Clip::new(0, 0, 0));
        let chord = |text| HarmonyName::parse(text).unwrap();
        project.set_chords(0, 32, &[ChordMarker::new(0, chord("C"))]);
        let trial = project.add_pattern().unwrap();
        let before = project.playlist().chord_markers().to_vec();
        write_style_parts(
            &mut project,
            trial,
            Style::named("Jazz waltz").unwrap(),
            None,
            StyleParts {
                rhythm: true,
                time_signature: true,
                ..no_parts()
            },
        );
        assert_eq!(project.playlist().chord_markers(), before);
        assert_eq!(project.time_signature, TimeSignature::new(4, 4));
        let pattern = project.pattern(trial).unwrap();
        assert_eq!(pattern.time, Some(TimeSignature::new(3, 4)));
        assert_eq!(pattern.length(), 48);
        assert!(heard_chords_channel(&project, trial).is_none());
        assert!(!drums_in(&project, trial, 0, 48).is_empty());
    }

    #[test]
    fn a_progression_changes_pitches_while_keeping_the_rhythmic_pattern() {
        let mut project = audition();
        let before = project.clone();
        write_style_parts(
            &mut project,
            0,
            Style::named("Emotional pop").unwrap(),
            None,
            StyleParts {
                progression: true,
                ..no_parts()
            },
        );
        let keys = find_synth(&project, KEYS).unwrap();
        let attacks = |project: &Project| {
            project
                .pattern(0)
                .unwrap()
                .notes(keys)
                .iter()
                .map(|n| (n.start, n.length, n.velocity, n.nudge, n.trim))
                .collect::<Vec<_>>()
        };
        assert_eq!(attacks(&project), attacks(&before));
        assert_ne!(
            project.pattern(0).unwrap().notes(keys),
            before.pattern(0).unwrap().notes(keys)
        );
        assert_eq!(project.channels(), before.channels());
        assert_eq!(project.time_signature, before.time_signature);
    }

    #[test]
    fn swing_can_be_applied_without_replacing_any_notes() {
        let mut project = audition();
        let before = project.clone();
        write_rhythm_parts(
            &mut project,
            0,
            Rhythm::named("Shuffle").unwrap(),
            StyleParts {
                swing: true,
                ..no_parts()
            },
        );
        assert_eq!(project.patterns(), before.patterns());
        assert_eq!(project.channels(), before.channels());
        assert_eq!(project.swing, 100);
    }

    #[test]
    fn a_held_chord_changes_where_the_new_progression_changes() {
        let mut project = audition();
        let keys = find_synth(&project, KEYS).unwrap();
        let bass = find_synth(&project, BASS).unwrap();
        project.pattern_mut(0).unwrap().set_notes(
            keys,
            [60, 64, 67].map(|pitch| NoteEvent::new(Note::new(pitch), 0, 64)),
        );
        project
            .pattern_mut(0)
            .unwrap()
            .set_notes(bass, [NoteEvent::new(Note::new(36), 0, 64)]);
        write_style_parts(
            &mut project,
            0,
            Style::named("Four chords").unwrap(),
            None,
            StyleParts {
                progression: true,
                ..no_parts()
            },
        );
        let bass = project.pattern(0).unwrap().notes(bass);
        assert_eq!(
            bass.iter()
                .map(|note| (note.start, note.pitch.pitch_class()))
                .collect::<Vec<_>>(),
            [
                (0, PitchClass::C),
                (16, PitchClass::G),
                (32, PitchClass::A),
                (48, PitchClass::F)
            ]
        );
        assert!(bass.iter().all(|note| note.length == 16));
    }
}
