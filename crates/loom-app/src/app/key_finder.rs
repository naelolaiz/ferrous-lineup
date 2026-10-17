//! Find the key: an experimental option that works out the key from the
//! music itself, and shows it under the harmony keys with the closest
//! other keys beside it, fainter, and what the chord playing is in each.
//!
//! There are three sources, from the most to the least immediate:
//!
//! 1. **What you play** on a MIDI or typing keyboard (or the harmony
//!    keys), with the bars played before as context: a
//!    [`KeyListener`]. It counts while you have played in the last few
//!    bars.
//! 2. **The recording**, for a project made by File > Import recording:
//!    the key changes heard in the audio, kept from the import.
//! 3. **The notes** of the song, or of the selected pattern in pattern
//!    mode: [`Project::heard_keys`].
//!
//! Nothing here changes the project. The work is done in
//! [`loom_core::harmony::key_finding`].

use loom_core::harmony::key_finding::{KeyListener, KeySpan, chromas_of_notes, rank_keys};
use loom_core::{Project, Tempo};

use super::LoomApp;
use crate::harmony_keys::KeyHeard;

/// What you play counts for this many bars after you last played.
const LIVE_BARS: usize = 4;

/// What Find the key remembers.
#[derive(Default)]
pub(super) struct KeyFinder {
    /// The key changes in the song's notes, worked out once and kept
    /// until the project changes.
    heard: Option<Vec<KeySpan>>,
    /// The ranking of the selected pattern's notes, and which pattern.
    pattern: Option<(usize, KeySpan)>,
    /// The key changes heard in an imported recording, in song steps.
    recording: Vec<KeySpan>,
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
    }

    /// Forgets everything, for another project. `recording` is the key
    /// changes heard in it, if it was made from a recording.
    pub(super) fn new_project(&mut self, project: &Project, recording: Vec<KeySpan>) {
        *self = KeyFinder {
            recording,
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
        held: &[loom_core::Note],
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
    })
}

impl LoomApp {
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

    /// The key changes heard in the whole song: in the recording it was
    /// made from, or else in its notes.
    pub(super) fn song_keys_heard(&mut self) -> Vec<KeySpan> {
        let finder = &mut self.key_finder;
        if !finder.recording.is_empty() {
            return finder.recording.clone();
        }
        let project = &self.project;
        finder
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
            });
        }
        match position {
            Some((step, _)) => {
                let (spans, from) = if finder.recording.is_empty() {
                    let project = &self.project;
                    let spans = finder.heard.get_or_insert_with(|| project.heard_keys());
                    (&*spans, "in the notes here")
                } else {
                    (&finder.recording, "in the recording here")
                };
                let span = spans
                    .iter()
                    .find(|span| (span.start..span.end).contains(&step))
                    .or_else(|| spans.last())?;
                shown(span, from, true)
            }
            None => {
                let index = self.pattern;
                if finder.pattern.as_ref().is_none_or(|(at, _)| *at != index) {
                    let pattern = self.project.pattern(index)?;
                    let notes = self
                        .project
                        .channels()
                        .iter()
                        .enumerate()
                        .filter(|(_, channel)| {
                            matches!(channel.kind, loom_core::ChannelKind::Synth)
                                || matches!(channel.kind, loom_core::ChannelKind::MidiOut { channel } if channel != loom_core::DRUM_CHANNEL)
                        })
                        .filter(|(_, channel)| !channel.muted)
                        .flat_map(|(channel, _)| pattern.notes(channel))
                        .map(|note| (note.pitch, note.start, note.length));
                    let chroma = chromas_of_notes(notes, &[0, pattern.length().max(1)]);
                    let span = KeySpan {
                        start: 0,
                        end: pattern.length(),
                        guesses: rank_keys(&chroma[0]),
                    };
                    finder.pattern = Some((index, span));
                }
                let (_, span) = finder.pattern.as_ref()?;
                shown(span, "in this pattern", false)
            }
        }
    }
}
