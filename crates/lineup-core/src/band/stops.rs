//! Where the band leaves its style: shots, holds, rests and pushes.
//!
//! A style plays the same groove bar after bar. Real arrangements break
//! it where the music asks: the whole band hits one chord together and
//! stops (a **shot**), holds a chord out (a **hold**), drops out (a
//! **rest**), or comes in with the next chord a little before the beat
//! (a **push**). The chord sheet marks them on a harmony (see
//! [`Stop`] and [`Push`]); this module rewrites each player's part
//! around them, after the style has written it. The demo songs' chord
//! charts mark them the same way (see
//! [`crate::library::chart::parse_chart`]), and their bands play them so.

use crate::chord_sheet::{ChordMarker, Push, Stop};
use crate::library::comping::Played;
use crate::pattern::{NoteEvent, STEP_NOTE, TICKS_PER_STEP};

/// How long a shot sounds: an eighth note.
const SHOT_STEPS: usize = 2;

/// How hard the band strikes a shot, a hold or a push: a little harder
/// than it plays the style, so it stands out.
const ACCENT: u8 = 100;

/// A stretch of the song where the band stops playing its style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StopSpan {
    /// The step the harmony is written on.
    written: usize,
    /// Where the band plays it from, earlier when it is pushed too.
    start: usize,
    /// Where the next harmony comes in.
    end: usize,
    stop: Stop,
}

/// A harmony the band plays early.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PushSpan {
    /// The step the harmony is written on.
    written: usize,
    /// Where the band plays it from.
    start: usize,
    /// Where the next harmony comes in.
    end: usize,
}

/// The shots, holds, rests and pushes of a song.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Breaks {
    stops: Vec<StopSpan>,
    pushes: Vec<PushSpan>,
}

impl Breaks {
    /// The breaks marked on `markers`, in order, the last harmony lasting
    /// up to step `end`. A push never reaches back to the harmony before
    /// it: it starts at least one step after it.
    pub(crate) fn of(markers: &[ChordMarker], end: usize) -> Breaks {
        let marks: Vec<(usize, Option<Push>, Option<Stop>)> = markers
            .iter()
            .map(|marker| (marker.start, marker.push, marker.stop))
            .collect();
        Breaks::marked(&marks, end)
    }

    /// The breaks of harmonies written on the steps of `marks`, in order,
    /// each with its push and its stop, as [`Breaks::of`] reads them from
    /// the chord sheet's markers.
    pub(crate) fn marked(marks: &[(usize, Option<Push>, Option<Stop>)], end: usize) -> Breaks {
        let starts: Vec<usize> = marks
            .iter()
            .enumerate()
            .map(|(index, &(written, push, _))| {
                let earliest = match index {
                    0 => 0,
                    _ => marks[index - 1].0 + 1,
                };
                let early = written.saturating_sub(push.map_or(0, Push::steps));
                early.max(earliest).min(written)
            })
            .collect();
        let mut breaks = Breaks::default();
        for (index, &(written, _, stop)) in marks.iter().enumerate() {
            let start = starts[index];
            let next = starts.get(index + 1).copied().unwrap_or(end).max(start);
            match stop {
                Some(stop) => breaks.stops.push(StopSpan {
                    written,
                    start,
                    end: next,
                    stop,
                }),
                None if start < written => breaks.pushes.push(PushSpan {
                    written,
                    start,
                    end: next,
                }),
                None => {}
            }
        }
        breaks
    }

    /// Whether the song has none.
    pub(crate) fn is_empty(&self) -> bool {
        self.stops.is_empty() && self.pushes.is_empty()
    }

    /// A player's `notes` with the breaks played: the style's notes
    /// make way, and `strike` gives what the player strikes instead, the
    /// harmony held from its `start` to its `end`.
    ///
    /// A push strikes the chord early and holds it over the beat it is
    /// written on, up to the player's next note. A shot strikes it for
    /// an eighth note, a hold until the next harmony, and a rest leaves
    /// the player silent.
    pub(crate) fn apply(
        &self,
        mut notes: Vec<NoteEvent>,
        harmonies: &[Played],
        strike: impl Fn(&Played) -> Vec<NoteEvent>,
    ) -> Vec<NoteEvent> {
        if self.is_empty() {
            return notes;
        }
        let harmony_at = |step: usize| harmonies.iter().find(|harmony| harmony.start == step);
        let starts: Vec<usize> = notes.iter().map(|note| note.start).collect();
        for push in &self.pushes {
            let Some(harmony) = harmony_at(push.written) else {
                continue;
            };
            // Held over into the beat, up to where the style goes on.
            let until = starts
                .iter()
                .copied()
                .filter(|&start| start > push.written)
                .min()
                .unwrap_or(push.end)
                .min(push.end)
                .max(push.written + 1);
            clear(&mut notes, push.start, push.written + 1);
            notes.extend(accented(strike(&Played {
                start: push.start,
                end: until,
                ..harmony.clone()
            })));
        }
        for stop in &self.stops {
            clear(&mut notes, stop.start, stop.end);
            let Some(harmony) = harmony_at(stop.written) else {
                continue;
            };
            match stop.stop {
                Stop::Shot => notes.extend(accented(strike(&Played {
                    start: stop.start,
                    end: (stop.start + SHOT_STEPS).min(stop.end),
                    ..harmony.clone()
                }))),
                // A row form is played as trichords one after another:
                // each is struck as it comes, and held.
                Stop::Hold => {
                    let parts = harmonies
                        .iter()
                        .filter(|part| (stop.written..stop.end).contains(&part.start));
                    for part in parts {
                        let start = if part.start == stop.written {
                            stop.start
                        } else {
                            part.start
                        };
                        notes.extend(accented(strike(&Played {
                            start,
                            end: part.end.min(stop.end),
                            ..part.clone()
                        })));
                    }
                }
                Stop::Rest => {}
            }
        }
        notes.sort_by_key(|note| (note.start_tick(), note.pitch));
        notes
    }

    /// The drums' `notes`, each on its channel, with the breaks played:
    /// the groove makes way, and a shot, a hold or a push gets one hit of
    /// the kick and the crash cymbal, on the channels `kick` and `crash`.
    /// A rest is silent.
    pub(crate) fn apply_to_drums(
        &self,
        mut notes: Vec<(usize, NoteEvent)>,
        kick: Option<usize>,
        crash: Option<usize>,
    ) -> Vec<(usize, NoteEvent)> {
        if self.is_empty() {
            return notes;
        }
        let mut hits = Vec::new();
        for push in &self.pushes {
            notes.retain(|(_, note)| !(push.start..=push.written).contains(&note.start));
            hits.push(push.start);
        }
        for stop in &self.stops {
            notes.retain(|(_, note)| !(stop.start..stop.end).contains(&note.start));
            if stop.stop != Stop::Rest {
                hits.push(stop.start);
            }
        }
        for at in hits {
            for channel in kick.into_iter().chain(crash) {
                notes.push((
                    channel,
                    NoteEvent {
                        velocity: ACCENT,
                        ..NoteEvent::new(STEP_NOTE, at, 1)
                    },
                ));
            }
        }
        notes
    }
}

/// Takes out of `notes` those that start from step `from` up to `to`,
/// and stops at `from` those that would still sound there.
fn clear(notes: &mut Vec<NoteEvent>, from: usize, to: usize) {
    let cut = from * TICKS_PER_STEP;
    notes.retain(|note| !(from..to).contains(&note.start));
    for note in notes.iter_mut() {
        if note.start_tick() < cut && note.end_tick() > cut {
            *note = NoteEvent {
                velocity: note.velocity,
                ..NoteEvent::exact(note.pitch, note.start_tick(), cut - note.start_tick())
            };
        }
    }
}

/// `notes`, struck at least as hard as [`ACCENT`].
fn accented(mut notes: Vec<NoteEvent>) -> Vec<NoteEvent> {
    for note in &mut notes {
        note.velocity = note.velocity.max(ACCENT);
    }
    notes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HarmonyName;
    use crate::harmony::{Chord, ChordQuality};
    use crate::library::comping::BassPart;
    use crate::pitch::{Note, PitchClass};

    fn marker(start: usize, name: &str) -> ChordMarker {
        ChordMarker::new(start, HarmonyName::parse(name).unwrap())
    }

    fn played(start: usize, end: usize, root: u8) -> Played {
        Played {
            start,
            end,
            voicing: vec![Note::new(root), Note::new(root + 4), Note::new(root + 7)],
            bass: BassPart::Chord {
                chord: Chord::new(PitchClass::from_semitone(root), ChordQuality::Major),
                note: Note::new(root - 24),
                next: Note::new(36),
            },
        }
    }

    /// A chord on every beat, a quarter note each.
    fn quarters(harmonies: &[Played]) -> Vec<NoteEvent> {
        harmonies
            .iter()
            .flat_map(|harmony| {
                (harmony.start..harmony.end).step_by(4).flat_map(move |at| {
                    harmony
                        .voicing
                        .iter()
                        .map(move |&pitch| NoteEvent::new(pitch, at, 4))
                })
            })
            .collect()
    }

    fn held(harmony: &Played) -> Vec<NoteEvent> {
        harmony
            .voicing
            .iter()
            .map(|&pitch| NoteEvent::new(pitch, harmony.start, harmony.end - harmony.start))
            .collect()
    }

    fn starts_and_lengths(notes: &[NoteEvent], pitch: u8) -> Vec<(usize, usize)> {
        notes
            .iter()
            .filter(|note| note.pitch.midi() == pitch)
            .map(|note| (note.start, note.length))
            .collect()
    }

    #[test]
    fn marks_become_stretches_up_to_the_next_harmony() {
        let markers = [
            marker(0, "C").stopped(Some(Stop::Shot)),
            marker(16, "F").pushed(Some(Push::Eighth)),
            marker(32, "G").stopped(Some(Stop::Hold)),
        ];
        let breaks = Breaks::of(&markers, 48);
        // The shot lasts until F comes in, an eighth early.
        assert_eq!(
            breaks.stops,
            [
                StopSpan {
                    written: 0,
                    start: 0,
                    end: 14,
                    stop: Stop::Shot
                },
                StopSpan {
                    written: 32,
                    start: 32,
                    end: 48,
                    stop: Stop::Hold
                },
            ]
        );
        assert_eq!(
            breaks.pushes,
            [PushSpan {
                written: 16,
                start: 14,
                end: 32
            }]
        );
    }

    #[test]
    fn a_push_never_reaches_back_past_the_harmony_before() {
        let markers = [marker(0, "C"), marker(1, "F").pushed(Some(Push::Eighth))];
        let breaks = Breaks::of(&markers, 16);
        // F is written one step after C: it cannot come in any earlier.
        assert!(breaks.pushes.is_empty());
    }

    #[test]
    fn a_shot_strikes_once_and_the_band_stops() {
        let harmonies = [played(0, 16, 60), played(16, 32, 65)];
        let markers = [marker(0, "C").stopped(Some(Stop::Shot)), marker(16, "F")];
        let notes = Breaks::of(&markers, 32).apply(quarters(&harmonies), &harmonies, held);
        // C: one eighth note, struck hard; then F on every beat again.
        assert_eq!(starts_and_lengths(&notes, 60), [(0, 2)]);
        assert!(
            notes
                .iter()
                .filter(|n| n.start == 0)
                .all(|n| n.velocity >= ACCENT)
        );
        assert_eq!(
            starts_and_lengths(&notes, 65),
            [(16, 4), (20, 4), (24, 4), (28, 4)]
        );
    }

    #[test]
    fn a_hold_lasts_until_the_next_harmony_and_a_rest_is_silent() {
        let harmonies = [played(0, 16, 60), played(16, 32, 65)];
        let markers = [
            marker(0, "C").stopped(Some(Stop::Hold)),
            marker(16, "F").stopped(Some(Stop::Rest)),
        ];
        let notes = Breaks::of(&markers, 32).apply(quarters(&harmonies), &harmonies, held);
        assert_eq!(starts_and_lengths(&notes, 60), [(0, 16)]);
        assert!(starts_and_lengths(&notes, 65).is_empty());
    }

    #[test]
    fn a_hold_over_a_row_form_holds_each_trichord_as_it_comes() {
        // A row form in one bar: four trichords, a beat each.
        let row = [
            played(0, 4, 60),
            played(4, 8, 63),
            played(8, 12, 66),
            played(12, 16, 69),
        ];
        let markers = [marker(0, "C").stopped(Some(Stop::Hold))];
        let notes = Breaks::of(&markers, 16).apply(quarters(&row), &row, held);
        for (beat, root) in [60, 63, 66, 69].into_iter().enumerate() {
            assert_eq!(starts_and_lengths(&notes, root), [(beat * 4, 4)]);
        }
        assert!(notes.iter().all(|note| note.velocity >= ACCENT));
    }

    #[test]
    fn a_push_comes_in_early_and_is_held_over_the_beat() {
        let harmonies = [played(0, 16, 60), played(16, 32, 65)];
        let markers = [marker(0, "C"), marker(16, "F").pushed(Some(Push::Eighth))];
        let notes = Breaks::of(&markers, 32).apply(quarters(&harmonies), &harmonies, held);
        // C's last beat is cut short at the push...
        assert_eq!(
            starts_and_lengths(&notes, 60),
            [(0, 4), (4, 4), (8, 4), (12, 2)]
        );
        // ...and F comes in on the "and" of 4, held up to beat 2.
        assert_eq!(
            starts_and_lengths(&notes, 65),
            [(14, 6), (20, 4), (24, 4), (28, 4)]
        );
    }

    #[test]
    fn the_drums_stop_and_hit_with_the_band() {
        let markers = [
            marker(0, "C"),
            marker(16, "F").stopped(Some(Stop::Shot)),
            marker(32, "G").pushed(Some(Push::Sixteenth)),
        ];
        let hat = |at| (2, NoteEvent::new(STEP_NOTE, at, 1));
        let groove: Vec<(usize, NoteEvent)> = (0..48).step_by(2).map(hat).collect();
        let notes = Breaks::of(&markers, 48).apply_to_drums(groove, Some(0), Some(1));
        let on = |channel| -> Vec<usize> {
            let mut starts: Vec<usize> = notes
                .iter()
                .filter(|(other, _)| *other == channel)
                .map(|(_, note)| note.start)
                .collect();
            starts.sort_unstable();
            starts
        };
        // The kick and the crash on the shot and on the push.
        assert_eq!(on(0), [16, 31]);
        assert_eq!(on(1), [16, 31]);
        // The hi-hat stops for the shot, and makes way for the push.
        let hats = on(2);
        assert!(hats.iter().all(|&at| !(16..31).contains(&at) && at != 32));
        assert!(hats.contains(&14) && hats.contains(&34));
    }
}
