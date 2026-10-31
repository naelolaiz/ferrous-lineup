//! Find the key: an experimental option that works out the key from the
//! music itself, and shows it under the harmony keys with the closest
//! other keys beside it, fainter, and what the chord playing is in each.
//!
//! There are two sources, from the more to the less immediate:
//!
//! 1. **What you play** on a MIDI or typing keyboard (or the harmony
//!    keys), with the bars played before as context: a
//!    [`KeyListener`]. It counts while you have played in the last few
//!    bars.
//! 2. **The notes** of the song, or of the selected pattern in pattern
//!    mode: [`Project::heard_keys`].
//!
//! Under the key, the colour of the passage: the micromode its notes
//! fit and the set they make ([`Project::heard_colours`]), read eight
//! bars at a time between the song's key markers
//! ([`Project::colour_passage`]) and over the whole of a pattern. Only
//! where the key is a set, a micromode, a row or the chromatic scale
//! ([`shows_colours`]), and not while you play: three notes are too few.
//!
//! Nothing here changes the project. The work is done in
//! [`lineup_core::harmony::key_finding`] and
//! [`lineup_core::harmony::colour_finding`].

use lineup_core::harmony::colour_finding::{Colours, find_colours};
use lineup_core::harmony::key_finding::{KeyListener, KeySpan, chromas_of_notes, rank_keys};
use lineup_core::{NoteEvent, Project, Scale, ScaleKind, Tempo};

use super::LineupApp;
use crate::harmony_keys::{ColoursHeard, KeyHeard};
use crate::i18n::{tr, trf};

/// What you play counts for this many bars after you last played.
const LIVE_BARS: usize = 4;

/// What Find the key remembers.
#[derive(Default)]
pub(super) struct KeyFinder {
    /// The key changes in the song's notes, worked out once and kept
    /// until the project changes.
    heard: Option<Vec<KeySpan>>,
    /// The ranking of the selected pattern's notes, and which pattern,
    /// with the colours of its notes.
    pattern: Option<(usize, KeySpan, Colours)>,
    /// The colours of the passage last read in the song, and where it
    /// starts and ends, in timeline steps.
    passage: Option<((usize, usize), Colours)>,
    /// What you play.
    listener: KeyListener,
    /// How far into the current bar the listener's clock is, in steps.
    clock: f32,
    /// Bars since you last played, or `None` before you play at all.
    bars_since_played: Option<usize>,
}

impl KeyFinder {
    /// Forgets what was worked out from the notes, after an edit.
    pub(super) fn project_changed(&mut self) {
        self.heard = None;
        self.pattern = None;
        self.passage = None;
    }

    /// Forgets everything, for another project.
    pub(super) fn new_project(&mut self, project: &Project) {
        *self = KeyFinder {
            listener: KeyListener::starting_in(Some(project.scale)),
            ..KeyFinder::default()
        };
    }

    /// Whether you played in the last few bars.
    fn is_live(&self) -> bool {
        self.bars_since_played.is_some_and(|bars| bars < LIVE_BARS)
    }

    /// Counts `seconds` of what you hold, at `tempo`, with bars of
    /// `steps_per_bar` steps. The bars of what you play are the bars of
    /// the clock, whether the song plays or not.
    fn listen(
        &mut self,
        held: &[lineup_core::Note],
        seconds: f32,
        tempo: Tempo,
        steps_per_bar: usize,
    ) {
        let steps = seconds / tempo.seconds_per_beat() * 4.0;
        if !held.is_empty() {
            self.listener.hear(held.iter().copied(), steps);
            self.bars_since_played = Some(0);
        }
        self.clock += steps;
        let bar = steps_per_bar.max(1) as f32;
        while self.clock >= bar {
            self.clock -= bar;
            self.listener.end_bar();
            if let Some(bars) = &mut self.bars_since_played {
                *bars += 1;
            }
        }
    }
}

/// The key a span shows, with its alternatives, from `source`.
fn shown(span: &KeySpan, from: &'static str, can_mark: bool) -> Option<KeyHeard> {
    Some(KeyHeard {
        best: *span.guesses.first()?,
        alternatives: span.alternatives().to_vec(),
        from,
        can_mark,
        colours: None,
    })
}

impl LineupApp {
    /// Listens to the keys held, once per frame.
    pub(super) fn find_key_live(&mut self, seconds: f32) {
        if !self.harmony_keys.find_key {
            return;
        }
        let held = self.live.held_pitches();
        // When you start playing, what you play is heard in the light of
        // the key the music around it is in: the notes here, or the key
        // of the project.
        if !held.is_empty() && !self.key_finder.is_live() {
            let around = self.key_heard().map(|heard| heard.best.key);
            self.key_finder.listener = KeyListener::starting_in(around.or(Some(self.key_here())));
            self.key_finder.clock = 0.0;
        }
        let steps_per_bar = self.project.time_signature.steps_per_bar();
        self.key_finder
            .listen(&held, seconds, self.project.tempo, steps_per_bar);
    }

    /// The key changes heard in the whole song's notes.
    pub(super) fn song_keys_heard(&mut self) -> Vec<KeySpan> {
        let project = &self.project;
        self.key_finder
            .heard
            .get_or_insert_with(|| project.heard_keys())
            .clone()
    }

    /// The key to show under the harmony keys, if the option is on.
    pub(super) fn key_heard(&mut self) -> Option<KeyHeard> {
        if !self.harmony_keys.find_key {
            return None;
        }
        let position = self.song_position();
        let finder = &mut self.key_finder;
        if finder.is_live() {
            let guesses = finder.listener.guesses();
            return Some(KeyHeard {
                best: *guesses.first()?,
                alternatives: finder.listener.alternatives(),
                from: "in what you play",
                can_mark: false,
                colours: None,
            });
        }
        match position {
            Some((step, _)) => {
                let project = &self.project;
                let spans = finder.heard.get_or_insert_with(|| project.heard_keys());
                let span = spans
                    .iter()
                    .find(|span| (span.start..span.end).contains(&step))
                    .or_else(|| spans.last())?;
                let mut heard = shown(span, "in the notes here", true)?;
                let project = &self.project;
                let passage = project.colour_passage(step);
                let key = project.key_at(passage.0);
                if !shows_colours(key) {
                    return Some(heard);
                }
                if finder.passage.as_ref().is_none_or(|(at, _)| *at != passage) {
                    let (start, end) = passage;
                    let colours = project.heard_colours(start, end, Some(heard.best.key));
                    finder.passage = Some((passage, colours));
                }
                let (_, colours) = finder.passage.as_ref()?;
                let meter = project.meter();
                let first = meter.position(passage.0).0;
                let last = meter.position(passage.1.saturating_sub(1)).0;
                heard.colours = Some(ColoursHeard {
                    colours: colours.clone(),
                    from: trf("in bars {} to {}", &[&first, &last]),
                    passage: Some(passage),
                    key,
                });
                Some(heard)
            }
            None => {
                let index = self.pattern;
                if finder.pattern.as_ref().is_none_or(|(at, ..)| *at != index) {
                    let pattern = self.project.pattern(index)?;
                    let notes = self
                        .project
                        .channels()
                        .iter()
                        .enumerate()
                        .filter(|(_, channel)| {
                            matches!(channel.kind, lineup_core::ChannelKind::Synth)
                                || matches!(channel.kind, lineup_core::ChannelKind::MidiOut { channel } if channel != lineup_core::DRUM_CHANNEL)
                        })
                        .filter(|(_, channel)| !channel.muted)
                        .flat_map(|(channel, _)| pattern.notes(channel))
                        .filter(|note| note.start < pattern.length())
                        .copied()
                        .collect::<Vec<NoteEvent>>();
                    let chroma = chromas_of_notes(
                        notes
                            .iter()
                            .map(|note| (note.pitch, note.start, note.length)),
                        &[0, pattern.length().max(1)],
                    );
                    let span = KeySpan {
                        start: 0,
                        end: pattern.length(),
                        guesses: rank_keys(&chroma[0]),
                    };
                    let found = span.guesses.first().map(|guess| guess.key);
                    finder.pattern = Some((index, span, find_colours(&notes, found)));
                }
                let (_, span, colours) = finder.pattern.as_ref()?;
                let mut heard = shown(span, "in this pattern", false)?;
                let key = self.project.pattern_key(index);
                if !shows_colours(key) {
                    return Some(heard);
                }
                heard.colours = Some(ColoursHeard {
                    colours: colours.clone(),
                    from: tr("in this pattern").to_string(),
                    passage: None,
                    key,
                });
                Some(heard)
            }
        }
    }
}

/// Whether Find the key shows the colour of a passage whose key is
/// `key`: in a set, a micromode, a row or the chromatic scale. A song in
/// a major or minor key or a mode is shown in chords only, as the chord
/// sheet and the piano roll show it: tonal music is made of two
/// micromodes, the major and minor triads and the diminished one, and
/// naming them as micromodes says nothing new. To see the colour of a
/// passage in such a song, make the key chromatic first.
fn shows_colours(key: Scale) -> bool {
    !key.kind.is_tonal() || key.kind == ScaleKind::Chromatic
}
