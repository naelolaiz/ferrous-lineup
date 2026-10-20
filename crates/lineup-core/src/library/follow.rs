//! Following a chord: the accompaniment plays whatever chord you hold.
//!
//! Arranger keyboards work like this. A style plays drums, chords and a
//! bass line, and your left hand only tells it which chord to play: hold
//! C and it plays in C, move to A minor and the very same rhythm and
//! bass line carry on in A minor. The style is written once, in one
//! chord, and the keyboard bends its notes into the chord you hold.
//!
//! Ferrous Lineup does the same with the patterns of any project. It first
//! hears the chords each pattern already plays (see [`super::listen`]),
//! once. Then, for each chord you hold, every note of the accompaniment
//! is moved to the matching note of your chord:
//!
//! - **Block chords** (three or more notes starting together) are voiced
//!   again for your chord, as close as possible to where they were (see
//!   [`super::voicing`]), so the hand hardly moves.
//! - **Single notes**, as in arpeggios and bass lines, keep their *role*:
//!   the root goes to your root, the third to your third, the fifth to
//!   your fifth, a seventh to your seventh (or to the root, if your chord
//!   has none). Each lands in the octave nearest to where it was, so a
//!   bass line stays low and a melody of arpeggios keeps its shape.
//! - **Other notes**, such as the passing notes of a walking bass, move
//!   as far as the root does.
//!
//! Nothing here changes the project: the moved notes are sent to the
//! audio thread only, and the written notes come back when you stop.

use crate::harmony::{Chord, ChordQuality, Harmony, analyze};
use crate::pattern::NoteEvent;
use crate::pitch::Note;
use crate::project::Project;

use super::apply::{BASS, find_synth, heard_chords_channel};
use super::listen::{HeardChord, hear_chords};
use super::voicing::voice;

/// The chord the keys `held` ask for, the way arranger keyboards read
/// the left hand: a chord of three notes or more is itself; one key is
/// its major chord; two keys a minor third apart are a minor chord, a
/// major third or a fifth apart a major chord, and a minor seventh apart
/// a dominant seventh, all on the lower key. Anything else asks for no
/// chord, and the accompaniment keeps the last one.
pub fn chord_held(held: &[Note]) -> Option<Chord> {
    match analyze(held.iter().copied()) {
        Harmony::Chord { chord, .. } => Some(chord),
        Harmony::Single(root) => Some(Chord::new(root, ChordQuality::Major)),
        Harmony::Interval { low, semitones, .. } => {
            let quality = match semitones {
                3 => ChordQuality::Minor,
                4 | 7 => ChordQuality::Major,
                10 => ChordQuality::Dominant7,
                _ => return None,
            };
            Some(Chord::new(low, quality))
        }
        // `analyze` never hears two chords at once.
        Harmony::Silence | Harmony::Polychord { .. } | Harmony::Unknown(_) => None,
    }
}

/// What a note does in its chord.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Role {
    Root,
    /// The third, or what stands for it in a "sus" chord.
    Third,
    Fifth,
    /// The seventh, or the sixth of a sixth chord.
    Seventh,
    /// A ninth, eleventh or thirteenth.
    Tension,
}

/// The notes of `chord` as semitones above its root (0 to 11), each with
/// its role.
fn roles(chord: Chord) -> Vec<(u8, Role)> {
    let intervals = chord.quality.intervals();
    let has_third = intervals.iter().any(|&i| i == 3 || i == 4);
    intervals
        .iter()
        .map(|&interval| {
            let role = match interval {
                0 => Role::Root,
                3 | 4 => Role::Third,
                2 | 5 if !has_third => Role::Third,
                6..=8 => Role::Fifth,
                9..=11 => Role::Seventh,
                _ => Role::Tension,
            };
            (interval % 12, role)
        })
        .collect()
}

/// Where a pitch class `semitones` above the root of `from` goes in `to`,
/// as semitones above the root of `to`.
fn move_tone(semitones: u8, from: Chord, to: Chord) -> u8 {
    let Some(role) = roles(from)
        .into_iter()
        .find(|&(tone, _)| tone == semitones)
        .map(|(_, role)| role)
    else {
        // Not in the chord: it moves with the root.
        return semitones;
    };
    let target = roles(to);
    let find = |role: Role| target.iter().find(|(_, r)| *r == role).map(|&(t, _)| t);
    let fallback = match role {
        Role::Seventh | Role::Tension => Role::Root,
        Role::Third => Role::Fifth,
        _ => Role::Root,
    };
    find(role).or_else(|| find(fallback)).unwrap_or(0)
}

/// The note in pitch class `pitch_class` (0 to 11) nearest to `near`.
fn nearest(pitch_class: u8, near: Note) -> Note {
    let near = i32::from(near.midi());
    let up = (i32::from(pitch_class) - near).rem_euclid(12);
    // Up to a tritone up, or else down.
    let midi = if up <= 6 { near + up } else { near + up - 12 };
    Note::new(midi.clamp(0, 127) as u8)
}

/// `note`, moved from chord `from` into chord `to`.
fn move_note(note: Note, from: Chord, to: Chord) -> Note {
    let above_root = from.root.semitones_up_to(note.pitch_class());
    let tone = move_tone(above_root, from, to);
    nearest((to.root.semitone() + tone) % 12, note)
}

/// Moves the notes of one channel into `to`, stretch by stretch.
fn follow_notes(
    notes: &[NoteEvent],
    heard: &[HeardChord],
    to: Chord,
    blocks: bool,
) -> Vec<NoteEvent> {
    let mut moved = Vec::with_capacity(notes.len());
    for stretch in heard {
        let end = stretch.start + stretch.length;
        let mut inside: Vec<NoteEvent> = notes
            .iter()
            .copied()
            .filter(|note| (stretch.start..end).contains(&note.start))
            .collect();
        let Some(from) = stretch.chord else {
            moved.extend(inside);
            continue;
        };
        inside.sort_by_key(|note| (note.start, note.pitch));
        let mut rest = inside.as_slice();
        while let Some(first) = rest.first() {
            let together = rest.iter().take_while(|n| n.start == first.start).count();
            let (group, after) = rest.split_at(together);
            rest = after;
            if blocks && group.len() >= 3 {
                // Voiced again near where it was, one note per old note.
                let old: Vec<Note> = group.iter().map(|note| note.pitch).collect();
                let new = voice(to, Some(&old));
                for (index, pitch) in new.into_iter().enumerate() {
                    let like = group[index.min(group.len() - 1)];
                    moved.push(NoteEvent { pitch, ..like });
                }
            } else {
                moved.extend(group.iter().map(|note| NoteEvent {
                    pitch: move_note(note.pitch, from, to),
                    ..*note
                }));
            }
        }
    }
    moved.sort_by_key(|note| (note.start, note.pitch));
    moved.dedup_by_key(|note| (note.start, note.pitch));
    moved
}

/// The accompaniment of one pattern, as heard once.
#[derive(Debug, Clone, PartialEq)]
struct Heard {
    pattern: usize,
    /// The channel with its chords, if any.
    chords: Option<usize>,
    /// The "Bass" synth, if it plays in this pattern.
    bass: Option<usize>,
    stretches: Vec<HeardChord>,
}

/// The accompaniment of a whole project, ready to follow any chord.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Accompaniment {
    patterns: Vec<Heard>,
}

/// New notes for one channel of one pattern.
#[derive(Debug, Clone, PartialEq)]
pub struct Followed {
    pub pattern: usize,
    pub channel: usize,
    pub notes: Vec<NoteEvent>,
}

impl Accompaniment {
    /// Hears the chords of every pattern of `project` that has an
    /// accompaniment: a chord channel, a "Bass" synth, or both. A locked
    /// channel keeps its notes, so it is heard but does not follow.
    pub fn hear(project: &Project) -> Self {
        let bass_channel = find_synth(project, BASS);
        let free = |channel: Option<usize>| channel.filter(|&c| !project.is_locked(c));
        let patterns = project
            .patterns()
            .iter()
            .enumerate()
            .filter_map(|(index, pattern)| {
                let chords = heard_chords_channel(project, index);
                let bass = bass_channel.filter(|&b| !pattern.notes(b).is_empty());
                if free(chords).is_none() && free(bass).is_none() {
                    return None;
                }
                let notes = |channel: Option<usize>| {
                    channel.map_or(&[][..], |channel| pattern.notes(channel))
                };
                let stretches = hear_chords(
                    notes(chords),
                    notes(bass),
                    pattern.length(),
                    project.pattern_time(index),
                );
                Some(Heard {
                    pattern: index,
                    chords: free(chords),
                    bass: free(bass),
                    stretches,
                })
            })
            .collect();
        Accompaniment { patterns }
    }

    /// Whether there is anything to follow.
    pub fn is_empty(&self) -> bool {
        self.patterns.is_empty()
    }

    /// The channels that follow, by pattern.
    pub fn channels(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.patterns.iter().flat_map(|heard| {
            [heard.chords, heard.bass]
                .into_iter()
                .flatten()
                .map(move |channel| (heard.pattern, channel))
        })
    }

    /// The notes every following channel plays when `chord` is held.
    pub fn follow(&self, project: &Project, chord: Chord) -> Vec<Followed> {
        let mut followed = Vec::new();
        for heard in &self.patterns {
            let Some(pattern) = project.pattern(heard.pattern) else {
                continue;
            };
            for (channel, blocks) in [(heard.chords, true), (heard.bass, false)] {
                let Some(channel) = channel else { continue };
                followed.push(Followed {
                    pattern: heard.pattern,
                    channel,
                    notes: follow_notes(pattern.notes(channel), &heard.stretches, chord, blocks),
                });
            }
        }
        followed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harmony::ChordQuality::*;
    use crate::library::apply::write_style;
    use crate::library::style::STYLES;
    use crate::pitch::PitchClass::*;

    fn n(midi: u8) -> Note {
        Note::new(midi)
    }

    #[test]
    fn keys_ask_for_chords() {
        assert_eq!(
            chord_held(&[n(57), n(60), n(64)]),
            Some(Chord::new(A, Minor))
        );
        assert_eq!(chord_held(&[n(55)]), Some(Chord::new(G, Major)));
        assert_eq!(chord_held(&[n(62), n(65)]), Some(Chord::new(D, Minor)));
        assert_eq!(chord_held(&[n(55), n(65)]), Some(Chord::new(G, Dominant7)));
        assert_eq!(chord_held(&[n(60), n(61)]), None);
        assert_eq!(chord_held(&[]), None);
    }

    #[test]
    fn notes_keep_their_role() {
        let c = Chord::new(C, Major);
        let am = Chord::new(A, Minor);
        // Root, third and fifth of C go to those of A minor, nearby.
        assert_eq!(move_note(n(48), c, am), n(45)); // C3 -> A2
        assert_eq!(move_note(n(64), c, am), n(60)); // E4 -> C4
        assert_eq!(move_note(n(67), c, am), n(64)); // G4 -> E4
        // A passing D moves as the root does: to B.
        assert_eq!(move_note(n(62), c, am), n(59));
    }

    #[test]
    fn a_seventh_goes_to_the_root_of_a_triad() {
        let g7 = Chord::new(G, Dominant7);
        let f = Chord::new(F, Major);
        assert_eq!(move_note(n(65), g7, f), n(65)); // the F of G7 -> F
        let fm7 = Chord::new(F, Minor7);
        assert_eq!(move_note(n(65), g7, fm7), n(63)); // -> Eb, the seventh
    }

    #[test]
    fn a_style_follows_the_held_chord() {
        let mut project = Project::empty();
        let style = STYLES
            .iter()
            .find(|s| s.name == "Pop ballad")
            .unwrap_or(&STYLES[0]);
        assert!(write_style(&mut project, 0, style, C, None));
        let accompaniment = Accompaniment::hear(&project);
        assert!(!accompaniment.is_empty());
        let d_minor = Chord::new(D, Minor);
        let followed = accompaniment.follow(&project, d_minor);
        assert!(!followed.is_empty());
        for channel in &followed {
            assert!(!channel.notes.is_empty());
            for note in &channel.notes {
                assert!(
                    d_minor.pitch_set().contains(note.pitch.pitch_class())
                        || channel.channel == find_synth(&project, BASS).unwrap(),
                    "{:?} is not in D minor",
                    note.pitch
                );
            }
            // The rhythm stays: the same starts as before.
            let before = project
                .pattern(channel.pattern)
                .unwrap()
                .notes(channel.channel);
            let starts = |notes: &[NoteEvent]| {
                let mut s: Vec<usize> = notes.iter().map(|n| n.start).collect();
                s.dedup();
                s
            };
            let mut old = before.to_vec();
            old.sort_by_key(|n| n.start);
            assert_eq!(starts(&old), starts(&channel.notes));
        }
    }

    #[test]
    fn a_locked_channel_does_not_follow() {
        let mut project = Project::empty();
        assert!(write_style(&mut project, 0, &STYLES[0], C, None));
        let bass = find_synth(&project, BASS).unwrap();
        let free = Accompaniment::hear(&project).follow(&project, Chord::new(D, Minor));
        project.set_locked(bass, true);
        let accompaniment = Accompaniment::hear(&project);
        assert!(accompaniment.channels().all(|(_, channel)| channel != bass));
        let followed = accompaniment.follow(&project, Chord::new(D, Minor));
        let others: Vec<_> = free.into_iter().filter(|f| f.channel != bass).collect();
        assert!(!others.is_empty());
        assert_eq!(followed, others);
    }

    #[test]
    fn every_style_follows_any_chord_in_its_chords() {
        let chords = [
            Chord::new(E, Minor),
            Chord::new(ASharp, Dominant7),
            Chord::new(F, Major7),
        ];
        for style in STYLES {
            let mut project = Project::empty();
            assert!(write_style(&mut project, 0, style, C, None));
            let accompaniment = Accompaniment::hear(&project);
            for to in chords {
                for channel in accompaniment.follow(&project, to) {
                    let bass = find_synth(&project, BASS) == Some(channel.channel);
                    if bass {
                        continue;
                    }
                    for note in &channel.notes {
                        assert!(
                            to.pitch_set().contains(note.pitch.pitch_class()),
                            "{}: {:?} is not in {:?}",
                            style.name,
                            note.pitch,
                            to
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn every_demo_song_follows_a_chord() {
        use crate::library::SONGS;
        use crate::library::demo::song_demo;
        let g = Chord::new(G, Major);
        for song in SONGS {
            let project = song_demo(song);
            let accompaniment = Accompaniment::hear(&project);
            assert!(
                !accompaniment.is_empty(),
                "{} has no accompaniment",
                song.name
            );
            for channel in accompaniment.follow(&project, g) {
                let before = project
                    .pattern(channel.pattern)
                    .unwrap()
                    .notes(channel.channel);
                // Nothing is lost: block chords may gain or lose a voice,
                // but every start is still played.
                let starts = |notes: &[NoteEvent]| {
                    let mut s: Vec<usize> = notes.iter().map(|n| n.start).collect();
                    s.sort();
                    s.dedup();
                    s
                };
                assert_eq!(starts(before), starts(&channel.notes), "{}", song.name);
            }
        }
    }
}
