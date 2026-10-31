//! The arrangement: what the song really plays, in order, once the
//! repeats of clips and sections are unrolled.
//!
//! The playlist is drawn as a *timeline*: a chorus marked "×2" is drawn
//! once. The song that plays is longer: that chorus is heard twice before
//! the song moves on. So there are two kinds of step:
//!
//! - a **timeline step** is a place in the playlist, as drawn;
//! - a **song step** is a moment in the song as it plays.
//!
//! [`Arrangement`] turns one into the other. Its [`Pass`]es say which
//! stretch of the timeline plays at which moment, and its [`Play`]s are
//! the pieces of patterns the audio thread must play, already in song
//! steps, so the audio thread knows nothing about sections or repeats.
//!
//! ```text
//! timeline:   | Intro | Chorus ×2 | Outro |
//! song:       | Intro | Chorus | Chorus | Outro |
//! ```
//!
//! A band's song form repeats too: its chorus, all the bars written on
//! the chord sheet after the intro, plays as many times as the form says
//! (see [`crate::band::Form`]), with its sections repeating inside each
//! time round.

use crate::playlist::Clip;
use crate::project::Project;

/// A piece of a pattern that the song plays: from song step `start`, for
/// `length` steps, beginning `skip` steps into the pattern, counting
/// every time round.
///
/// The pattern loops for as long as the piece lasts, so a clip that
/// repeats its pattern three times is one `Play` three patterns long.
/// `skip` is not zero when the song jumps into the middle of a clip: when
/// a repeated section starts again, or a loop region starts, partway
/// through it. It is not zero either when the clip is trimmed at the
/// start (see [`Clip::skip`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Play {
    pub pattern: usize,
    pub start: usize,
    pub skip: usize,
    pub length: usize,
}

impl Play {
    /// The step just after its last one.
    pub fn end(&self) -> usize {
        self.start + self.length
    }

    /// The step of the pattern played on song step `step`, if this piece
    /// is playing then. `pattern_length` is the length of its pattern.
    pub fn pattern_step(&self, step: usize, pattern_length: usize) -> Option<usize> {
        if pattern_length == 0 || !(self.start..self.end()).contains(&step) {
            return None;
        }
        Some((self.skip + step - self.start) % pattern_length)
    }

    /// Every song step on which this piece plays step `pattern_step` of
    /// its pattern, in order. A clip that repeats its pattern plays each
    /// of its steps once per repeat.
    pub fn song_steps_of(
        &self,
        pattern_step: usize,
        pattern_length: usize,
    ) -> impl Iterator<Item = usize> + '_ {
        // How far into the piece the pattern step first comes round. The
        // `% pattern_length` on `skip` keeps the subtraction from going
        // below zero; adding `pattern_length` first does the same.
        let first = if pattern_length == 0 || pattern_step >= pattern_length {
            self.length // nothing: start past the end
        } else {
            (pattern_step + pattern_length - self.skip % pattern_length) % pattern_length
        };
        (first..self.length)
            .step_by(pattern_length.max(1))
            .map(move |offset| self.start + offset)
    }
}

/// A stretch of the timeline and the moment it plays in the song.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pass {
    /// The song step it starts on.
    pub start: usize,
    /// The timeline step it starts on.
    pub from: usize,
    /// How many steps it lasts.
    pub length: usize,
    /// The section it belongs to, if any.
    pub section: Option<usize>,
    /// Which time this is that the stretch plays, from 1...
    pub round: usize,
    /// ...out of this many.
    pub rounds: usize,
    /// Which time round the band's chorus is, out of how many, when the
    /// stretch is in a chorus that plays more than once.
    pub chorus: Option<(usize, usize)>,
}

impl Pass {
    pub fn end(&self) -> usize {
        self.start + self.length
    }
}

/// The song as it plays, with every repeat unrolled.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Arrangement {
    /// The pieces of patterns to play, in song steps.
    pub plays: Vec<Play>,
    /// The stretches of the timeline, in the order they play.
    pub passes: Vec<Pass>,
    /// How long the song is, in song steps.
    pub length: usize,
}

impl Arrangement {
    /// The pass playing on song step `step`.
    pub fn pass_at(&self, step: usize) -> Option<&Pass> {
        self.passes
            .iter()
            .find(|pass| (pass.start..pass.end()).contains(&step))
    }

    /// Where in the playlist song step `step` is, to draw the playhead.
    pub fn timeline_step(&self, step: usize) -> Option<usize> {
        self.pass_at(step)
            .map(|pass| pass.from + (step - pass.start))
    }
}

/// The arrangement of the timeline from step `from` up to `to`.
///
/// The stretch is cut into pieces at section edges. A section plays as
/// many times as its `repeats` say; the gaps between sections play once.
/// Each time a piece plays, every audible clip under it adds a [`Play`]
/// for the part of the clip inside the piece.
///
/// A band's chorus that plays more than once is played again as a whole,
/// pieces and all, if the stretch takes in all of it: a loop region
/// inside it loops just the bars it has.
///
/// Where the song carries straight on across a section edge (into a
/// section, or out of one after its last time round), a clip under the
/// edge does not jump either: its part after the edge only makes its
/// `Play` before the edge longer. So a note held across the edge sounds
/// to its end, as it would with no section there. Only a real jump back,
/// to play a section again, starts a new `Play` and cuts the notes.
pub(crate) fn arrange(project: &Project, from: usize, to: usize) -> Arrangement {
    let mut arrangement = Arrangement::default();
    let clips: Vec<Clip> = project.playlist().audible_clips().collect();
    // For each clip, the index of its last `Play` so far.
    let mut last_play: Vec<Option<usize>> = vec![None; clips.len()];
    for (piece, chorus) in played_pieces(project, from, to) {
        let (piece_from, piece_to, section, rounds) = piece;
        for round in 1..=rounds {
            let start = arrangement.length;
            arrangement.passes.push(Pass {
                start,
                from: piece_from,
                length: piece_to - piece_from,
                section,
                round,
                rounds,
                chorus,
            });
            for (index, clip) in clips.iter().enumerate() {
                let clip_end = clip.start + project.clip_length(clip);
                // The part of the clip inside the piece, if any.
                let begin = clip.start.max(piece_from);
                let end = clip_end.min(piece_to);
                if begin >= end {
                    continue;
                }
                // A clip trimmed at the start starts inside its pattern.
                let first = project
                    .pattern(clip.pattern)
                    .map_or(0, |p| clip.first_step(p.length()));
                let play = Play {
                    pattern: clip.pattern,
                    start: start + (begin - piece_from),
                    skip: first + (begin - clip.start),
                    length: end - begin,
                };
                // Does it carry on where the clip's last piece ended, both
                // in the song and in the clip?
                let carries_on = last_play[index].filter(|&previous| {
                    let previous = arrangement.plays[previous];
                    previous.end() == play.start && previous.skip + previous.length == play.skip
                });
                match carries_on {
                    Some(previous) => arrangement.plays[previous].length += play.length,
                    None => {
                        last_play[index] = Some(arrangement.plays.len());
                        arrangement.plays.push(play);
                    }
                }
            }
            arrangement.length += piece_to - piece_from;
        }
    }
    arrangement
}

/// A piece of the timeline: (start, end, section, how many times it
/// plays).
type Piece = (usize, usize, Option<usize>, usize);

/// The pieces of the timeline from `from` up to `to`, in the order they
/// play, each with the time round of the band's chorus it is in, when
/// the chorus plays more than once and the stretch takes in all of it.
fn played_pieces(
    project: &Project,
    from: usize,
    to: usize,
) -> Vec<(Piece, Option<(usize, usize)>)> {
    let once = |from, to| {
        pieces(project, from, to)
            .into_iter()
            .filter(|&(start, end, ..)| start < end)
            .map(|piece| (piece, None))
    };
    let Some((start, end, choruses)) = project
        .repeated_chorus()
        .filter(|&(start, end, _)| from <= start && end <= to)
    else {
        return once(from, to).collect();
    };
    let mut played: Vec<_> = once(from, start).collect();
    for chorus in 1..=choruses {
        played.extend(
            pieces(project, start, end)
                .into_iter()
                .map(|piece| (piece, Some((chorus, choruses)))),
        );
    }
    played.extend(once(end, to));
    played
}

/// Cuts the timeline from `from` up to `to` at section edges: (start,
/// end, section, how many times it plays) for each piece, in order.
fn pieces(project: &Project, from: usize, to: usize) -> Vec<Piece> {
    let mut pieces = Vec::new();
    let mut cursor = from;
    for (index, section) in project.playlist().sections().iter().enumerate() {
        if section.end() <= cursor || section.start >= to {
            continue;
        }
        if section.start > cursor {
            pieces.push((cursor, section.start, None, 1));
        }
        let start = section.start.max(cursor);
        let end = section.end().min(to);
        pieces.push((start, end, Some(index), section.repeats));
        cursor = end;
    }
    if cursor < to {
        pieces.push((cursor, to, None, 1));
    }
    pieces
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::playlist::{LoopRegion, Section};

    #[test]
    fn a_play_loops_its_pattern() {
        // Three times a 16-step pattern, starting at song step 32.
        let play = Play {
            pattern: 0,
            start: 32,
            skip: 0,
            length: 48,
        };
        assert_eq!(play.pattern_step(31, 16), None);
        assert_eq!(play.pattern_step(32, 16), Some(0));
        assert_eq!(play.pattern_step(50, 16), Some(2));
        assert_eq!(play.pattern_step(79, 16), Some(15));
        assert_eq!(play.pattern_step(80, 16), None);
        let steps: Vec<usize> = play.song_steps_of(2, 16).collect();
        assert_eq!(steps, [34, 50, 66]);
    }

    #[test]
    fn a_play_can_start_inside_its_pattern() {
        // The last 4 steps of the first round, then a whole round.
        let play = Play {
            pattern: 0,
            start: 0,
            skip: 12,
            length: 20,
        };
        assert_eq!(play.pattern_step(0, 16), Some(12));
        assert_eq!(play.pattern_step(4, 16), Some(0));
        let steps: Vec<usize> = play.song_steps_of(13, 16).collect();
        assert_eq!(steps, [1, 17]);
        let steps: Vec<usize> = play.song_steps_of(0, 16).collect();
        assert_eq!(steps, [4]);
        assert_eq!(play.song_steps_of(16, 16).count(), 0, "no step 16");
    }

    #[test]
    fn without_sections_the_song_plays_as_drawn() {
        let project = Project::demo();
        let song = project.arrangement();
        assert_eq!(song.length, project.song_length());
        assert_eq!(song.passes.len(), 1);
        assert_eq!(song.timeline_step(50), Some(50));
        assert_eq!(song.plays.len(), project.playlist().clips().len());
    }

    #[test]
    fn a_repeated_clip_is_one_long_play() {
        let mut project = Project::template();
        project.add_clip(Clip::new(0, 0, 16).repeated(3));
        assert_eq!(project.song_length(), 64);
        let song = project.arrangement();
        assert_eq!(
            song.plays,
            [Play {
                pattern: 0,
                start: 16,
                skip: 0,
                length: 48
            }]
        );
    }

    #[test]
    fn a_trimmed_clip_plays_only_what_it_shows() {
        // A 16-step pattern from step 16, three times round, without its
        // first 4 steps and the last 2 of its last time round.
        let mut project = Project::template();
        assert!(project.add_clip(Clip::new(0, 0, 16).repeated(3)).is_some());
        let clip = project.playlist().clips()[0];
        let trimmed = clip
            .trimmed_from(20, 16)
            .unwrap()
            .trimmed_to(62, 16)
            .unwrap();
        assert!(project.trim_clip(0, trimmed));
        let song = project.arrangement();
        assert_eq!(
            song.plays,
            [Play {
                pattern: 0,
                start: 20,
                skip: 4,
                length: 42
            }]
        );
        assert_eq!(song.plays[0].pattern_step(20, 16), Some(4));
        assert_eq!(song.plays[0].pattern_step(61, 16), Some(13));
        assert_eq!(song.plays[0].pattern_step(62, 16), None);
    }

    #[test]
    fn a_repeated_section_plays_again_before_the_song_moves_on() {
        // The demo: chords from bar 1, beat from bar 3, a fill in bar 8.
        let mut project = Project::demo();
        project.add_section(Section::new("Intro", 0, 32));
        let mut chorus = Section::new("Chorus", 32, 64);
        chorus.repeats = 2;
        project.add_section(chorus);

        let song = project.arrangement();
        // 8 bars drawn, 12 played: the 4 chorus bars twice.
        assert_eq!(project.song_length(), 128);
        assert_eq!(song.length, 192);
        let rounds: Vec<(Option<usize>, usize, usize)> = song
            .passes
            .iter()
            .map(|pass| (pass.section, pass.round, pass.from))
            .collect();
        assert_eq!(
            rounds,
            [
                (Some(0), 1, 0),
                (Some(1), 1, 32),
                (Some(1), 2, 32),
                (None, 1, 96)
            ]
        );
        // Song step 100 is 4 steps into the second chorus.
        assert_eq!(song.timeline_step(100), Some(36));
        assert_eq!(song.pass_at(100).map(|pass| pass.round), Some(2));
        // The fill (pattern 2, bar 8) plays once, after both choruses.
        let fills: Vec<usize> = song
            .plays
            .iter()
            .filter(|play| play.pattern == 2)
            .map(|play| play.start)
            .collect();
        assert_eq!(fills, [176]);
    }

    #[test]
    fn a_section_can_cut_a_clip_in_two() {
        // One clip, four bars long; a section over its last two bars.
        let mut project = Project::template();
        project.pattern_mut(0).unwrap().set_length(64);
        project.add_clip(Clip::new(0, 0, 0));
        let mut end = Section::new("End", 32, 32);
        end.repeats = 2;
        project.add_section(end);
        let song = project.arrangement();
        let plays: Vec<(usize, usize, usize)> = song
            .plays
            .iter()
            .map(|play| (play.start, play.skip, play.length))
            .collect();
        // The first time round, the song plays straight into the section,
        // so the clip is not cut there: it is cut where the song jumps
        // back to the start of the section, at song step 64.
        assert_eq!(plays, [(0, 0, 64), (64, 32, 32)]);
    }

    #[test]
    fn a_section_played_once_does_not_cut_a_clip() {
        // A note held across bar 3 of a four-bar clip, and sections over
        // its first two bars and its last bar, each played once: naming
        // parts of the song changes nothing that is heard.
        let mut project = Project::template();
        project.pattern_mut(0).unwrap().set_length(64);
        project.add_clip(Clip::new(0, 0, 0));
        let plain = project.arrangement().plays;
        project.add_section(Section::new("Intro", 0, 32));
        project.add_section(Section::new("Outro", 48, 16));
        let song = project.arrangement();
        assert_eq!(song.passes.len(), 3);
        assert_eq!(song.plays, plain);
        assert_eq!(
            song.plays,
            [Play {
                pattern: 0,
                start: 0,
                skip: 0,
                length: 64
            }]
        );
    }

    #[test]
    fn a_repeated_section_keeps_its_time_signature() {
        use crate::{TimeMarker, TimeSignature};
        // Two bars of 4/4, then a section of two bars of 3/4 played
        // twice, then 4/4 again.
        let waltz = TimeSignature::new(3, 4);
        let mut project = Project::template();
        project.add_clip(Clip::new(0, 0, 0).repeated(2));
        project.set_time_marker(TimeMarker::new(32, waltz));
        project.set_time_marker(TimeMarker::new(56, TimeSignature::COMMON));
        let mut waltzes = Section::new("Waltz", 32, 24);
        waltzes.repeats = 2;
        project.add_section(waltzes);
        project.add_clip(Clip::new(0, 0, 56));

        assert_eq!(project.song_length(), 72);
        let song = project.arrangement();
        assert_eq!(song.length, 96);
        let played = project.played_meter(&song);
        let bars: Vec<(usize, TimeSignature)> = played
            .bars()
            .take(7)
            .map(|bar| (bar.start, bar.time))
            .collect();
        let common = TimeSignature::COMMON;
        assert_eq!(
            bars,
            [
                (0, common),
                (16, common),
                (32, waltz),
                (44, waltz),
                (56, waltz),
                (68, waltz),
                (80, common)
            ]
        );
    }

    #[test]
    fn a_loop_region_plays_only_its_part() {
        let mut project = Project::demo();
        project.set_loop_region(Some(LoopRegion::new(48, 80)));
        let full = project.arrangement();
        assert_eq!(full.length, 128, "the full song ignores the loop");
        let looped = project.playback_arrangement();
        assert_eq!(looped.length, 32);
        assert_eq!(looped.timeline_step(0), Some(48));
        // The chords clip of bars 3 and 4 is cut in half: its second bar.
        assert!(looped.plays.contains(&Play {
            pattern: 1,
            start: 0,
            skip: 16,
            length: 16
        }));
    }
}
