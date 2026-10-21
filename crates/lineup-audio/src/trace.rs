//! A trace of what the audio thread plays, to check playback by reading
//! instead of listening.
//!
//! An export you listen to tells you that something sounds wrong, but
//! not what. A trace says what the player did: every note it started, on
//! which frame, how long it held it and why the note ended, and the tempo
//! of every step. It is written down on the audio thread itself, where
//! the notes start, so it is the playback you hear: after swing, tempo
//! changes, nudges and loose timing, the song as the playlist unrolls it,
//! with muted tracks left out. It is not a second account worked out from
//! the project, which could be right while the sound is wrong.
//!
//! The audio thread may not allocate, wait or write files, so a
//! [`Tracer`] hands each [`TraceEvent`] to a ring buffer, the way MIDI
//! messages leave. A [`TraceReader`] on another thread collects them into
//! a [`Trace`], which pairs the start and the end of each note
//! ([`Trace::notes`]) and writes them out as a table ([`write_table`]).
//!
//! The trace follows how long each note is held as the instruments do:
//! the gate it starts with, stretched when the tempo changes under it, and
//! cut short when playback stops or jumps, its track is muted or its clip
//! taken away. It does not follow the sound itself: a drum sample plays
//! to its end whatever the length, and a release fades out after the end.
//! Problems inside a synth still need to be heard, or the samples looked at.

use std::io::{self, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use lineup_core::sequencer::frames_per_step;
use lineup_core::{
    Arrangement, Channel, ChannelKind, KeyMap, Meter, Note, Project, SampleSource, Tempo,
    general_midi,
};

use crate::instrument::{HELD, Origin, stretched};

/// How many events can wait for the reader. The app reads them once a
/// frame; a few seconds of a dense song are a few thousand.
pub const TRACE_QUEUE_SIZE: usize = 1 << 16;

/// How many notes the trace can follow at once. A note started while
/// this many sound is still traced, but its end is not.
const OPEN_NOTES: usize = 4_096;

/// Something the audio thread did, on frame `frame` of its own clock: the
/// frames it has played since it started.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TraceEvent {
    /// Playback started on `step`: of the song, or of pattern `pattern`
    /// when `song` is false. After a count-in, when the count is over.
    Play {
        frame: u64,
        step: usize,
        song: bool,
        pattern: usize,
    },
    /// Playback jumped to `step` while playing.
    Jump { frame: u64, step: usize },
    /// Playback stopped.
    Stop { frame: u64 },
    /// A step started, at `tempo`.
    Step {
        frame: u64,
        step: usize,
        tempo: Tempo,
    },
    /// A note started on channel `channel` (its row in the channel rack),
    /// `late` frames after its step (nudge and loose timing), to be held
    /// for `gate` frames. `id` pairs it with its [`TraceEvent::NoteOff`].
    NoteOn {
        frame: u64,
        id: u32,
        channel: usize,
        origin: Origin,
        pitch: Note,
        velocity: u8,
        gate: u32,
        late: u32,
    },
    /// The note `id` ended, as `end` says.
    NoteOff { frame: u64, id: u32, end: End },
}

/// Why a note ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum End {
    /// Its length was over.
    Gate,
    /// The same note started again on its channel. A MIDI Out and a
    /// SoundFont play one of each note at a time, so the new one ends
    /// the old one.
    Again,
    /// It was stopped early: playback stopped or jumped, its track was
    /// muted, its clip taken away, or its channel removed.
    Cut,
}

impl End {
    fn name(self) -> &'static str {
        match self {
            End::Gate => "gate",
            End::Again => "again",
            End::Cut => "cut",
        }
    }
}

/// A note the trace knows is sounding, and the frame it ends on.
#[derive(Debug, Clone, Copy)]
struct Open {
    id: u32,
    channel: usize,
    pitch: Note,
    origin: Origin,
    end: u64,
}

/// The audio thread's end of a trace (see the module's documentation).
///
/// Made on another thread with [`trace_queue`], where its memory is
/// reserved, and handed over with
/// [`Command::SetTrace`](crate::Command::SetTrace).
pub struct Tracer {
    events: rtrb::Producer<TraceEvent>,
    /// How many events did not fit in the queue.
    lost: Arc<AtomicUsize>,
    /// The notes sounding. Reserved with room for [`OPEN_NOTES`].
    open: Vec<Open>,
    next_id: u32,
    /// The first frame an open note ends on, so the frames before it
    /// need no look at the notes.
    next_end: u64,
}

/// A tracer for the audio thread and the reader of what it writes.
pub fn trace_queue() -> (Tracer, TraceReader) {
    let (events, consumer) = rtrb::RingBuffer::new(TRACE_QUEUE_SIZE);
    let lost = Arc::new(AtomicUsize::new(0));
    let tracer = Tracer {
        events,
        lost: Arc::clone(&lost),
        open: Vec::with_capacity(OPEN_NOTES),
        next_id: 0,
        next_end: u64::MAX,
    };
    let reader = TraceReader {
        events: consumer,
        lost,
    };
    (tracer, reader)
}

impl std::fmt::Debug for Tracer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tracer")
            .field("open", &self.open.len())
            .finish_non_exhaustive()
    }
}

impl Tracer {
    fn push(&mut self, event: TraceEvent) {
        if self.events.push(event).is_err() {
            self.lost.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub(crate) fn played(&mut self, frame: u64, step: usize, song: bool, pattern: usize) {
        self.push(TraceEvent::Play {
            frame,
            step,
            song,
            pattern,
        });
    }

    pub(crate) fn jumped(&mut self, frame: u64, step: usize) {
        self.cut(frame, None, |_| false);
        self.push(TraceEvent::Jump { frame, step });
    }

    pub(crate) fn stopped(&mut self, frame: u64) {
        self.cut(frame, None, |_| false);
        self.push(TraceEvent::Stop { frame });
    }

    pub(crate) fn step(&mut self, frame: u64, step: usize, tempo: Tempo) {
        self.push(TraceEvent::Step { frame, step, tempo });
    }

    /// A note of the song started on `frame`. With `one_per_pitch` (a MIDI
    /// Out or a SoundFont), the same note sounding on the channel ends.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn note_on(
        &mut self,
        frame: u64,
        channel: usize,
        origin: Origin,
        pitch: Note,
        velocity: u8,
        gate: u32,
        late: u32,
        one_per_pitch: bool,
    ) {
        if one_per_pitch {
            self.end_where(frame, End::Again, |note| {
                note.channel == channel && note.pitch == pitch
            });
        }
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        self.push(TraceEvent::NoteOn {
            frame,
            id,
            channel,
            origin,
            pitch,
            velocity,
            gate,
            late,
        });
        let end = if gate == HELD {
            u64::MAX
        } else {
            frame + u64::from(gate)
        };
        if self.open.len() < self.open.capacity() {
            self.open.push(Open {
                id,
                channel,
                pitch,
                origin,
                end,
            });
            self.next_end = self.next_end.min(end);
        }
    }

    /// Ends the notes whose length is over by frame `now`. Called once
    /// a frame, before anything else happens on it.
    pub(crate) fn frame(&mut self, now: u64) {
        if now < self.next_end {
            return;
        }
        // The fields are borrowed one by one: `open` to go through, the
        // queue and the counter to write to.
        let events = &mut self.events;
        let lost = &self.lost;
        self.open.retain(|note| {
            if note.end > now {
                return true;
            }
            let event = TraceEvent::NoteOff {
                frame: note.end,
                id: note.id,
                end: End::Gate,
            };
            if events.push(event).is_err() {
                lost.fetch_add(1, Ordering::Relaxed);
            }
            false
        });
        self.find_next_end();
    }

    /// Stretches the lengths of the notes sounding after a change of
    /// tempo, as the instruments do (see
    /// [`Instrument::tempo_changed`](crate::Instrument::tempo_changed)).
    pub(crate) fn tempo_changed(&mut self, now: u64, next_step: f64, stretch: f64) {
        for note in &mut self.open {
            if note.end != u64::MAX {
                let left = note.end.saturating_sub(now) as f64;
                note.end = now + stretched(left, next_step, stretch) as u64;
            }
        }
        self.find_next_end();
    }

    /// Ends the notes of `channel` (of every channel with `None`) that
    /// `keep` turns down, as `release_unless` does.
    pub(crate) fn cut(&mut self, now: u64, channel: Option<usize>, keep: impl Fn(Origin) -> bool) {
        self.end_where(now, End::Cut, |note| {
            channel.is_none_or(|channel| note.channel == channel) && !keep(note.origin)
        });
    }

    /// A channel was added at `index`: the channels from there on move
    /// down one row, and the notes they play with them.
    pub(crate) fn channel_inserted(&mut self, index: usize) {
        for note in &mut self.open {
            if note.channel >= index {
                note.channel += 1;
            }
        }
    }

    /// Channel `index` was taken away: its notes end, and the channels
    /// after it move up one row.
    pub(crate) fn channel_removed(&mut self, now: u64, index: usize) {
        self.cut(now, Some(index), |_| false);
        for note in &mut self.open {
            if note.channel > index {
                note.channel -= 1;
            }
        }
    }

    fn end_where(&mut self, now: u64, end: End, ends: impl Fn(&Open) -> bool) {
        let events = &mut self.events;
        let lost = &self.lost;
        self.open.retain(|note| {
            if !ends(note) {
                return true;
            }
            let event = TraceEvent::NoteOff {
                frame: now,
                id: note.id,
                end,
            };
            if events.push(event).is_err() {
                lost.fetch_add(1, Ordering::Relaxed);
            }
            false
        });
        self.find_next_end();
    }

    fn find_next_end(&mut self) {
        self.next_end = self
            .open
            .iter()
            .map(|note| note.end)
            .min()
            .unwrap_or(u64::MAX);
    }
}

/// The reading end of a trace, for a thread that may take its time.
pub struct TraceReader {
    events: rtrb::Consumer<TraceEvent>,
    lost: Arc<AtomicUsize>,
}

impl TraceReader {
    /// Moves the events waiting into `trace`.
    pub fn read_into(&mut self, trace: &mut Trace) {
        while let Ok(event) = self.events.pop() {
            trace.events.push(event);
        }
        trace.lost = self.lost.load(Ordering::Relaxed);
    }
}

/// What a tracer wrote, in the order it happened.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Trace {
    /// The audio thread's frames per second.
    pub sample_rate: u32,
    pub events: Vec<TraceEvent>,
    /// How many events were lost because the queue was full: the reader
    /// did not read for too long.
    pub lost: usize,
}

/// One note of a trace.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TracedNote {
    /// Which playback it was played in: 0 for the first `Play` of the
    /// trace, 1 for the next.
    pub run: usize,
    /// The channel's row in the channel rack, counted from 0.
    pub channel: usize,
    /// Its pattern, the step of the pattern, and the step of the song.
    pub origin: Origin,
    pub pitch: Note,
    pub velocity: u8,
    /// When it started, in seconds from the start of its playback.
    pub start: f64,
    /// How long it was held, in seconds, if the trace saw it end.
    pub length: Option<f64>,
    /// The step of the loop it started on (of the song, or of the
    /// pattern in pattern mode), with the fraction of a step it was late
    /// by: 64.25 is a quarter of the way into step 64.
    pub step: f64,
    /// How many steps it was held for, if the trace saw it end.
    pub steps: Option<f64>,
    /// The tempo of the step it started on.
    pub tempo: Tempo,
    /// How late it started after its step, in seconds: its nudge and
    /// loose timing.
    pub late: f64,
    /// Why it ended, if the trace saw it end.
    pub end: Option<End>,
}

/// A change of tempo, on the step it starts.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TempoChange {
    pub run: usize,
    /// In seconds from the start of the playback.
    pub start: f64,
    pub step: usize,
    pub tempo: Tempo,
}

/// A step a playback played: its frame, its step, its tempo, and how many
/// steps were played before it in this playback.
#[derive(Debug, Clone, Copy)]
struct Played {
    frame: u64,
    step: usize,
    tempo: Tempo,
    count: usize,
}

/// One playback: from a `Play` to the next, or to the end of the trace.
struct Run<'a> {
    start: u64,
    steps: Vec<Played>,
    events: &'a [TraceEvent],
}

impl Run<'_> {
    /// Where frame `frame` is: the step played then and the fraction
    /// of it gone by, and the same counted in steps played since the
    /// start, which goes on growing past a loop or a jump.
    fn position(&self, frame: u64, sample_rate: u32) -> Option<(f64, f64, Tempo)> {
        let index = self.steps.partition_point(|played| played.frame <= frame);
        let played = self.steps.get(index.checked_sub(1)?)?;
        let length = match self.steps.get(index) {
            Some(next) => (next.frame - played.frame) as f64,
            None => frames_per_step(played.tempo, sample_rate),
        };
        let fraction = (frame - played.frame) as f64 / length.max(1.0);
        Some((
            played.step as f64 + fraction,
            played.count as f64 + fraction,
            played.tempo,
        ))
    }
}

impl Trace {
    /// An empty trace, for an audio thread running at `sample_rate`.
    pub fn new(sample_rate: u32) -> Self {
        Trace {
            sample_rate,
            ..Trace::default()
        }
    }

    fn seconds(&self, frames: u64) -> f64 {
        frames as f64 / f64::from(self.sample_rate.max(1))
    }

    /// The playbacks, each from its `Play`. Events before the first
    /// `Play` (there are none, unless the trace started while playing)
    /// count from the first event.
    fn runs(&self) -> Vec<Run<'_>> {
        let mut starts: Vec<usize> = self
            .events
            .iter()
            .enumerate()
            .filter(|(_, event)| matches!(event, TraceEvent::Play { .. }))
            .map(|(index, _)| index)
            .collect();
        if starts.first() != Some(&0) && !self.events.is_empty() {
            starts.insert(0, 0);
        }
        let ends = starts.iter().skip(1).copied().chain([self.events.len()]);
        starts
            .iter()
            .zip(ends)
            .map(|(&from, to)| {
                let events = &self.events[from..to];
                let start = events.first().map_or(0, frame_of);
                let mut steps = Vec::new();
                for event in events {
                    if let &TraceEvent::Step { frame, step, tempo } = event {
                        let count = steps.len();
                        steps.push(Played {
                            frame,
                            step,
                            tempo,
                            count,
                        });
                    }
                }
                Run {
                    start,
                    steps,
                    events,
                }
            })
            .collect()
    }

    /// Every note in the trace, in the order they started.
    pub fn notes(&self) -> Vec<TracedNote> {
        let mut notes = Vec::new();
        for (run_index, run) in self.runs().iter().enumerate() {
            // Where each note is in `notes`, by id, until it ends.
            let mut sounding: Vec<(u32, usize, u64)> = Vec::new();
            for event in run.events {
                match *event {
                    TraceEvent::NoteOn {
                        frame,
                        id,
                        channel,
                        origin,
                        pitch,
                        velocity,
                        late,
                        ..
                    } => {
                        let (step, _, tempo) = run.position(frame, self.sample_rate).unwrap_or((
                            origin.song_step as f64,
                            0.0,
                            Tempo::default(),
                        ));
                        sounding.push((id, notes.len(), frame));
                        notes.push(TracedNote {
                            run: run_index,
                            channel,
                            origin,
                            pitch,
                            velocity,
                            start: self.seconds(frame - run.start),
                            length: None,
                            step,
                            steps: None,
                            tempo,
                            late: self.seconds(u64::from(late)),
                            end: None,
                        });
                    }
                    TraceEvent::NoteOff { frame, id, end } => {
                        let Some(at) = sounding.iter().position(|&(open, ..)| open == id) else {
                            continue;
                        };
                        let (_, index, started) = sounding.swap_remove(at);
                        let note = &mut notes[index];
                        note.length = Some(self.seconds(frame.saturating_sub(started)));
                        note.end = Some(end);
                        let counted = |frame| {
                            run.position(frame, self.sample_rate)
                                .map(|(_, count, _)| count)
                        };
                        note.steps = counted(started)
                            .zip(counted(frame))
                            .map(|(from, to)| to - from);
                    }
                    _ => {}
                }
            }
        }
        notes
    }

    /// The tempo of the first step of each playback, and every change of
    /// tempo after it.
    pub fn tempo_changes(&self) -> Vec<TempoChange> {
        let mut changes = Vec::new();
        for (run_index, run) in self.runs().iter().enumerate() {
            let mut last = None;
            for played in &run.steps {
                if last != Some(played.tempo) {
                    changes.push(TempoChange {
                        run: run_index,
                        start: self.seconds(played.frame - run.start),
                        step: played.step,
                        tempo: played.tempo,
                    });
                    last = Some(played.tempo);
                }
            }
        }
        changes
    }

    /// The playbacks that are over, each as a trace of its own, taken out
    /// of this one, which keeps the playback still going, if any. A
    /// playback is over when it stops, or when another starts: switching
    /// between song and pattern mode while playing starts another.
    pub fn take_finished(&mut self) -> Vec<Trace> {
        let is_play = |event: &TraceEvent| matches!(event, TraceEvent::Play { .. });
        let last_play = self.events.iter().rposition(is_play);
        let stopped = self.events[last_play.unwrap_or(0)..]
            .iter()
            .any(|event| matches!(event, TraceEvent::Stop { .. }));
        let over = match (stopped, last_play) {
            (true, _) => self.events.len(),
            (false, Some(last_play)) => last_play,
            (false, None) => 0,
        };
        let mut traces: Vec<Trace> = Vec::new();
        for event in self.events.drain(..over) {
            if is_play(&event) || traces.is_empty() {
                traces.push(Trace::new(self.sample_rate));
            }
            if let Some(trace) = traces.last_mut() {
                trace.events.push(event);
            }
        }
        traces
    }

    /// The first `Play` of the trace: whether it played the song, and
    /// which pattern otherwise.
    pub fn played(&self) -> Option<(bool, usize)> {
        self.events.iter().find_map(|event| match *event {
            TraceEvent::Play { song, pattern, .. } => Some((song, pattern)),
            _ => None,
        })
    }
}

/// The frame an event happened on.
fn frame_of(event: &TraceEvent) -> u64 {
    match *event {
        TraceEvent::Play { frame, .. }
        | TraceEvent::Jump { frame, .. }
        | TraceEvent::Stop { frame }
        | TraceEvent::Step { frame, .. }
        | TraceEvent::NoteOn { frame, .. }
        | TraceEvent::NoteOff { frame, .. } => frame,
    }
}

/// What a trace table names: the song, the project's channels and
/// patterns, and the bars and keys of the steps played.
pub struct Labels<'a> {
    pub title: &'a str,
    pub project: &'a Project,
    /// The time signatures, in the steps of the loop played.
    pub meter: Meter,
    /// The keys, in the same steps, to spell the notes.
    pub keys: KeyMap,
}

impl<'a> Labels<'a> {
    /// For a trace of song mode playing `arrangement`.
    pub fn song(title: &'a str, project: &'a Project, arrangement: &Arrangement) -> Self {
        Labels {
            title,
            project,
            meter: project.played_meter(arrangement),
            keys: project.played_keys(arrangement),
        }
    }

    /// For a trace of pattern mode looping pattern `pattern`.
    pub fn pattern(title: &'a str, project: &'a Project, pattern: usize) -> Self {
        Labels {
            title,
            project,
            meter: Meter::steady(project.pattern_time(pattern)),
            keys: KeyMap::new(project.pattern_key(pattern), []),
        }
    }
}

/// What a channel plays, for a trace: its General MIDI program's name,
/// or its drum, sample file or MIDI channel.
fn instrument_name(channel: &Channel) -> String {
    match (&channel.kind, channel.program) {
        (ChannelKind::Synth, Some(program)) => general_midi::name(program).to_string(),
        (ChannelKind::Synth, None) => "Lineup Osc".to_string(),
        (ChannelKind::Sampler(SampleSource::Builtin(drum)), _) => match channel.sent_on {
            Some(number) => format!("drum {} on MIDI channel {}", drum.name(), number + 1),
            None => format!("drum {}", drum.name()),
        },
        (ChannelKind::Sampler(SampleSource::File(path)), _) => format!(
            "sample {}",
            path.file_name()
                .map_or_else(String::new, |name| name.to_string_lossy().into_owned())
        ),
        (ChannelKind::MidiOut { channel: number }, _) => {
            format!("MIDI channel {}", number + 1)
        }
    }
}

/// Text for a table cell: no tabs or line breaks, which would split it.
fn cell(text: &str) -> String {
    text.replace(['\t', '\n', '\r'], " ")
}

/// The columns of the table [`write_table`] writes.
pub const COLUMNS: [&str; 19] = [
    "start",
    "length",
    "bar",
    "beat",
    "into_beat",
    "step",
    "steps",
    "bpm",
    "time",
    "channel",
    "name",
    "program",
    "instrument",
    "pitch",
    "note",
    "velocity",
    "late_ms",
    "end",
    "pattern",
];

/// Writes `trace` as a table, one note per line in the order they start,
/// with tabs between the columns (see [`COLUMNS`]), after a few lines
/// starting with `#` that say what the table holds and list the tempo
/// and time signature changes. Spreadsheets, `awk` and `grep` read it.
pub fn write_table(out: &mut impl Write, trace: &Trace, labels: &Labels) -> io::Result<()> {
    let notes = trace.notes();
    writeln!(
        out,
        "# Ferrous Lineup playback trace: {}",
        cell(labels.title)
    )?;
    writeln!(
        out,
        "# Every note the player started, in the order they start. Times are in seconds from the start of playback;"
    )?;
    writeln!(
        out,
        "# steps are sixteenth notes of the loop played (the song, or the pattern in pattern mode), counted from 0."
    )?;
    writeln!(out, "# sample rate: {} Hz", trace.sample_rate)?;
    for change in trace.tempo_changes() {
        let (bar, ..) = labels.meter.position(change.step);
        writeln!(
            out,
            "# tempo: {:.4} s, step {}, bar {bar}: {:.2} BPM",
            change.start,
            change.step,
            change.tempo.bpm()
        )?;
    }
    let first = std::iter::once((0, labels.meter.first()));
    let markers = labels
        .meter
        .markers()
        .iter()
        .map(|marker| (marker.start, marker.time));
    for (step, time) in first.chain(markers) {
        let (bar, ..) = labels.meter.position(step);
        writeln!(out, "# time: step {step}, bar {bar}: {time}")?;
    }
    let unfinished = notes.iter().filter(|note| note.end.is_none()).count();
    if unfinished > 0 {
        writeln!(
            out,
            "# {unfinished} notes were still sounding when the trace ended: their length is \"?\"."
        )?;
    }
    if trace.lost > 0 {
        writeln!(
            out,
            "# {} events were lost: the trace was not read for too long.",
            trace.lost
        )?;
    }
    writeln!(out, "{}", COLUMNS.join("\t"))?;

    let channels = labels.project.channels();
    let patterns = labels.project.patterns();
    for note in &notes {
        let at = note.step.floor() as usize;
        let bar = labels.meter.bar_at(at);
        let beat = bar.beat_at(at);
        let length = note
            .length
            .map_or_else(|| "?".to_string(), |length| format!("{length:.4}"));
        let steps = note
            .steps
            .map_or_else(|| "?".to_string(), |steps| format!("{steps:.3}"));
        let channel = channels.get(note.channel);
        let name = channel.map_or_else(String::new, |channel| cell(&channel.name));
        let program = channel
            .and_then(|channel| channel.program)
            .map_or_else(|| "-".to_string(), |program| program.to_string());
        let instrument = channel.map_or_else(String::new, instrument_name);
        let spelling = labels.keys.key_at(at).spelling();
        let pattern = patterns.get(note.origin.pattern).map_or_else(
            || format!("{}", note.origin.pattern + 1),
            |pattern| cell(&pattern.name),
        );
        let over = if note.origin.over_song { " (over)" } else { "" };
        let end = note.end.map_or("?", End::name);
        writeln!(
            out,
            "{:.4}\t{length}\t{}\t{}\t{:.3}\t{:.3}\t{steps}\t{:.2}\t{}\t{}\t{name}\t{program}\t{instrument}\t{}\t{}\t{}\t{:.1}\t{end}\t{pattern}:{}{over}",
            note.start,
            bar.number + 1,
            beat.index + 1,
            note.step - beat.start as f64,
            note.step,
            note.tempo.bpm(),
            bar.time,
            note.channel + 1,
            note.pitch.midi(),
            note.pitch.name(spelling),
            note.velocity,
            note.late * 1000.0,
            note.origin.pattern_step,
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instrument::{Instrument, MidiOut, Synth};
    use crate::{ChannelNotes, Command, PlayMode, Renderer};
    use lineup_core::{NoteEvent, Play, TICKS_PER_STEP};

    const RATE: u32 = 48_000;
    /// At 120 BPM and 48 kHz, one step lasts 6000 frames: an eighth of
    /// a second.
    const STEP: usize = 6_000;
    const STEP_SECONDS: f64 = 0.125;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    /// A renderer with one 16-step pattern at 120 BPM, a channel playing
    /// `notes` on `instrument`, and a trace on.
    fn tracing(instrument: Instrument, notes: Vec<NoteEvent>) -> Renderer {
        let mut renderer = Renderer::new(RATE, 1);
        renderer.send(Command::InsertPattern {
            index: 0,
            length: 16,
        });
        renderer.send(Command::SetTempo(Tempo::new(120.0)));
        renderer.send(Command::InsertChannel {
            index: 0,
            instrument,
            notes: ChannelNotes::new([notes]),
            volume: 1.0,
            muted: false,
        });
        renderer.trace();
        renderer
    }

    fn synth() -> Instrument {
        Instrument::Synth(Synth::new(RATE))
    }

    #[test]
    fn notes_start_on_their_step_and_last_their_length() {
        let mut renderer = tracing(
            synth(),
            vec![
                NoteEvent::new(Note::C4, 0, 4),
                NoteEvent {
                    velocity: 64,
                    ..NoteEvent::new(Note::A4, 4, 2)
                },
            ],
        );
        renderer.send(Command::Play);
        renderer.skip(8 * STEP);
        let notes = renderer.take_trace().unwrap().notes();
        assert_eq!(notes.len(), 2);
        let (first, second) = (notes[0], notes[1]);
        assert_eq!((first.pitch, first.step), (Note::C4, 0.0));
        assert!(close(first.start, 0.0));
        assert!(close(first.length.unwrap(), 4.0 * STEP_SECONDS));
        assert!(close(first.steps.unwrap(), 4.0));
        assert_eq!(first.end, Some(End::Gate));
        assert_eq!((second.pitch, second.velocity), (Note::A4, 64));
        assert!(close(second.start, 4.0 * STEP_SECONDS));
        assert!(close(second.steps.unwrap(), 2.0));
        assert_eq!(second.tempo, Tempo::new(120.0));
    }

    #[test]
    fn a_nudged_note_starts_late_by_its_nudge() {
        let half = TICKS_PER_STEP as u8 / 2;
        let mut renderer = tracing(
            synth(),
            vec![NoteEvent {
                nudge: half,
                ..NoteEvent::new(Note::C4, 2, 1)
            }],
        );
        renderer.send(Command::Play);
        renderer.skip(4 * STEP);
        let note = renderer.take_trace().unwrap().notes()[0];
        assert!(close(note.start, 2.5 * STEP_SECONDS));
        assert!(close(note.late, 0.5 * STEP_SECONDS));
        assert!(close(note.step, 2.5), "{}", note.step);
        // It ends on the next step, where it would have ended unnudged.
        assert!(close(note.steps.unwrap(), 0.5));
    }

    #[test]
    fn a_slower_tempo_stretches_a_held_note() {
        // Two steps at 120 BPM, then 60 BPM: twice as long a step.
        let mut renderer = tracing(synth(), vec![NoteEvent::new(Note::C4, 0, 4)]);
        let mut tempos = vec![Tempo::new(120.0); 2];
        tempos.extend([Tempo::new(60.0); 14]);
        renderer.send(Command::SetSong {
            plays: vec![Play {
                pattern: 0,
                start: 0,
                skip: 0,
                length: 16,
            }],
            length: 16,
        });
        renderer.send(Command::SetSongTempos(tempos));
        renderer.send(Command::SetMode(PlayMode::Song));
        renderer.send(Command::Play);
        renderer.skip(8 * STEP);
        let trace = renderer.take_trace().unwrap();
        let note = trace.notes()[0];
        assert!(close(
            note.length.unwrap(),
            2.0 * STEP_SECONDS + 4.0 * STEP_SECONDS
        ));
        assert!(close(note.steps.unwrap(), 4.0));
        let changes: Vec<(usize, f32)> = trace
            .tempo_changes()
            .iter()
            .map(|change| (change.step, change.tempo.bpm()))
            .collect();
        assert_eq!(changes, [(0, 120.0), (2, 60.0)]);
        assert!(close(trace.tempo_changes()[1].start, 2.0 * STEP_SECONDS));
    }

    #[test]
    fn stopping_and_muting_cut_the_notes_sounding() {
        let mut renderer = tracing(synth(), vec![NoteEvent::new(Note::C4, 0, 8)]);
        renderer.send(Command::Play);
        renderer.skip(STEP);
        renderer.send(Command::Stop);
        renderer.send(Command::Play);
        renderer.skip(2 * STEP);
        renderer.send(Command::SetMuted {
            channel: 0,
            muted: true,
        });
        renderer.skip(16 * STEP);
        let notes = renderer.take_trace().unwrap().notes();
        // The muted channel starts nothing when the pattern comes round.
        assert_eq!(notes.len(), 2);
        assert_eq!((notes[0].run, notes[1].run), (0, 1));
        assert_eq!(notes[0].end, Some(End::Cut));
        assert!(close(notes[0].steps.unwrap(), 1.0));
        assert_eq!(notes[1].end, Some(End::Cut));
        assert!(close(notes[1].start, 0.0), "times count from each Play");
        assert!(close(notes[1].length.unwrap(), 2.0 * STEP_SECONDS));
    }

    #[test]
    fn a_midi_out_ends_a_note_that_starts_again() {
        let mut renderer = tracing(
            Instrument::MidiOut(MidiOut::new(0)),
            vec![
                NoteEvent::new(Note::C4, 0, 4),
                NoteEvent::new(Note::C4, 2, 1),
            ],
        );
        renderer.send(Command::Play);
        renderer.skip(4 * STEP);
        let notes = renderer.take_trace().unwrap().notes();
        assert_eq!(notes[0].end, Some(End::Again));
        assert!(close(notes[0].steps.unwrap(), 2.0));
        assert_eq!(notes[1].end, Some(End::Gate));
    }

    #[test]
    fn a_looping_pattern_counts_its_steps_on() {
        // Held over the end of the pattern, a note is cut there, on the
        // grid; the step count goes on past the loop.
        let mut renderer = tracing(synth(), vec![NoteEvent::new(Note::C4, 15, 1)]);
        renderer.send(Command::Play);
        renderer.skip(33 * STEP);
        let notes = renderer.take_trace().unwrap().notes();
        assert_eq!(notes.len(), 2);
        assert!(close(notes[1].start, 31.0 * STEP_SECONDS));
        assert!(close(notes[1].step, 15.0));
    }

    #[test]
    fn finished_playbacks_are_taken_out_one_by_one() {
        let mut renderer = tracing(synth(), vec![NoteEvent::new(Note::C4, 0, 1)]);
        renderer.send(Command::Play);
        renderer.skip(STEP);
        renderer.send(Command::Stop);
        renderer.send(Command::Play);
        renderer.skip(STEP);
        renderer.send(Command::Stop);
        renderer.send(Command::Play);
        renderer.skip(STEP);
        renderer.send(Command::SetMode(PlayMode::Song));
        renderer.skip(STEP);
        let mut trace = renderer.take_trace().unwrap();
        let finished = trace.take_finished();
        assert_eq!(finished.len(), 3, "two stopped, one went on in song mode");
        assert!(finished.iter().all(|trace| trace.notes().len() == 1));
        assert_eq!(finished[2].notes()[0].end, Some(End::Cut));
        assert_eq!(trace.played(), Some((true, 0)), "the song still plays");
        assert!(trace.take_finished().is_empty());
    }
}
