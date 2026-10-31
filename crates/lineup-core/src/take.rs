//! Takes: a voice recorded while the song plays, kept on a track of the
//! playlist, and the notes it sang written into the song.
//!
//! You sing or whistle along with the song; the app records the
//! microphone, and `lineup_listen::voice` hears the notes in the
//! recording, in seconds from its start. This module works out where in
//! the song each of those seconds was, writes the notes there, on the
//! grid, and keeps the recording on a track, where it plays with the song
//! like any other part.
//!
//! **When was each moment sung?** While recording, the app writes down a
//! [`Mark`] every few milliseconds: at this sample of the recording, the
//! song was heard at this position, moving on at this speed. Between two
//! marks the song moves at a steady speed, so any sample's place in the
//! song follows. The marks come from the sound card's own clock (see
//! `lineup_audio::input`): when the sound now coming out of the speakers
//! was made, and when the sound coming into the microphone was heard, so
//! the delays of both are taken out.
//!
//! **Where is that in the playlist?** The song as it plays is not the
//! playlist as drawn: a chorus marked ×2 plays twice, a loop region goes
//! round and round (see [`crate::arrangement`]). So a take is cut where
//! the playlist position jumps, into *runs*. Where two runs cover the same
//! bars (a chorus sung twice, a loop sung round again), the later one is
//! kept: sing it again until it is right.
//!
//! **Which notes?** Each note's pitch is rounded to the nearest semitone,
//! after taking out how sharp or flat the whole take is. A pitch right
//! between two semitones goes to the one in the key. Its start and end go
//! to the nearest point of the grid. How loud it was sung sets how hard
//! it is played (its *velocity*): a note as loud as most of the take is
//! played at 100, louder ones harder, softer ones softer.

use std::ops::Range;
use std::path::PathBuf;

use crate::arrangement::{Arrangement, Pass};
use crate::harmony::PitchSet;
use crate::pattern::{MAX_STEPS, NoteEvent, Pattern, STEP_NOTE};
use crate::pitch::{Note, PitchClass};
use crate::playlist::{Clip, MAX_SONG_STEPS};
use crate::project::{Channel, ChannelKind, Project, SampleSource};

/// One note heard in a take.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SungNote {
    /// When it starts and ends, in seconds from the start of the take.
    pub start: f64,
    pub end: f64,
    /// Where its pitch settled, as a fractional MIDI note number: 69.0
    /// is A4, 69.5 a quarter tone above.
    pub pitch: f32,
    /// Its loudest moment, in dB below full scale.
    pub loudness: f32,
}

impl SungNote {
    pub fn seconds(&self) -> f64 {
        self.end - self.start
    }
}

/// Where the song was heard at one moment of a take.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mark {
    /// The sample of the recording.
    pub frame: usize,
    /// The position of the song heard then, in steps with a fraction: of
    /// the song as it plays in song mode (see [`Arrangement`]), of the
    /// pattern in pattern mode.
    pub position: f64,
    /// How fast the song moved on from there.
    pub steps_per_second: f64,
}

/// What a take was sung along with.
#[derive(Debug, Clone, PartialEq)]
pub enum Along {
    /// The song, as song mode played it.
    Song(Arrangement),
    /// A pattern, looping in pattern mode.
    Pattern { pattern: usize, length: usize },
}

impl Along {
    /// How many steps go by before playback comes round again.
    fn length(&self) -> usize {
        match self {
            Along::Song(arrangement) => arrangement.length,
            Along::Pattern { length, .. } => *length,
        }
    }
}

/// A recording made along with the song.
#[derive(Debug, Clone, PartialEq)]
pub struct Take {
    pub sample_rate: u32,
    /// How many samples it holds.
    pub frames: usize,
    /// Where the song was, in the order they were written down.
    pub marks: Vec<Mark>,
    pub along: Along,
}

/// A stretch of a take where the song played straight on: its samples,
/// and the place in the playlist (or the pattern) of its first and its
/// last, in steps with a fraction.
#[derive(Debug, Clone, PartialEq)]
struct Run {
    frames: Range<usize>,
    from: f64,
    to: f64,
}

/// A stretch of a take to keep: its samples, from the one heard right on
/// step `step` of the playlist (or of the pattern), for `steps` steps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    pub frames: Range<usize>,
    pub step: usize,
    pub steps: usize,
}

/// A note of a take on the grid: from step `start` of the playlist (or
/// of the pattern), for `length` steps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Placed {
    pub start: usize,
    pub length: usize,
    pub pitch: Note,
    pub velocity: u8,
}

impl Placed {
    pub fn end(&self) -> usize {
        self.start + self.length
    }
}

/// What a take sang, ready to write into the song.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Transcription {
    /// The stretches of the recording to keep, in the order they play.
    pub pieces: Vec<Piece>,
    /// The notes, in order, one at a time.
    pub notes: Vec<Placed>,
    /// How sharp the take was sung, in semitones (see `lineup_listen::voice::tuning`).
    pub tuning: f32,
}

impl Transcription {
    /// The steps from the start of the first note to the end of the last:
    /// where the notes there were are replaced.
    pub fn range(&self) -> Option<Range<usize>> {
        let start = self.notes.first()?.start;
        let end = self.notes.iter().map(Placed::end).max()?;
        Some(start..end)
    }

    /// The steps the kept recording covers.
    pub fn recorded(&self) -> Option<Range<usize>> {
        let start = self.pieces.iter().map(|piece| piece.step).min()?;
        let end = self
            .pieces
            .iter()
            .map(|piece| piece.step + piece.steps)
            .max()?;
        Some(start..end)
    }
}

impl Take {
    /// A recording of `frames` samples at `sample_rate` laid on the
    /// playlist from step `step` on: an audio file added to the song. It
    /// goes straight on at the song's tempo, through its tempo changes,
    /// as a sound placed on a track does: the repeats and loops the song
    /// plays are not followed. It ends at the last step a clip can start
    /// on.
    pub fn laid(project: &Project, step: usize, sample_rate: u32, frames: usize) -> Take {
        let tempo = project.tempo_map();
        let step_seconds = |step: usize| f64::from(tempo.tempo_at(step).seconds_per_beat()) / 4.0;
        let rate = f64::from(sample_rate.max(1));
        // A mark every 10 ms.
        let every = (sample_rate / 100).max(1) as usize;
        let mut marks = Vec::new();
        // The step playing, when it started and how long it lasts.
        let (mut at, mut started, mut length) = (step, 0.0, step_seconds(step));
        for frame in (0..frames).step_by(every) {
            let seconds = frame as f64 / rate;
            while seconds >= started + length && at < MAX_SONG_STEPS {
                started += length;
                at += 1;
                length = step_seconds(at);
            }
            if at >= MAX_SONG_STEPS {
                break;
            }
            marks.push(Mark {
                frame,
                position: at as f64 + (seconds - started) / length,
                steps_per_second: 1.0 / length,
            });
        }
        Take {
            sample_rate,
            frames,
            marks,
            along: Along::Song(straight_on()),
        }
    }

    /// The song position heard at sample `frame`, from the last mark at
    /// or before it, or `None` where the song was not heard: before the
    /// first mark, or long after the last (the marks come every few
    /// milliseconds while the song plays).
    fn song_position(&self, frame: usize) -> Option<f64> {
        /// The longest a mark says where the song is: 0.2 s.
        const MARK_SECONDS: f64 = 0.2;
        let index = self
            .marks
            .partition_point(|mark| mark.frame <= frame)
            .checked_sub(1)?;
        let mark = self.marks[index];
        let seconds = (frame - mark.frame) as f64 / f64::from(self.sample_rate.max(1));
        if seconds > MARK_SECONDS {
            return None;
        }
        let length = self.along.length() as f64;
        if length <= 0.0 {
            return None;
        }
        // Past the end, playback has come round to the start again.
        Some((mark.position + seconds * mark.steps_per_second).rem_euclid(length))
    }

    /// Where sample `frame` was heard in the playlist (or the pattern).
    fn place(&self, frame: usize) -> Option<f64> {
        let song = self.song_position(frame)?;
        match &self.along {
            Along::Song(arrangement) => {
                let pass = arrangement.pass_at(song.floor() as usize)?;
                Some(pass.from as f64 + (song - pass.start as f64))
            }
            Along::Pattern { .. } => Some(song),
        }
    }

    /// How many steps go by in `frames` samples, at the speed of the mark
    /// before `frame`.
    fn steps_in(&self, frame: usize, frames: usize) -> f64 {
        let index = self
            .marks
            .partition_point(|mark| mark.frame <= frame)
            .saturating_sub(1);
        let speed = self
            .marks
            .get(index)
            .map_or(0.0, |mark| mark.steps_per_second);
        frames as f64 / f64::from(self.sample_rate.max(1)) * speed
    }

    /// Whether the song went straight on from sample `from`, heard at
    /// `at`, to sample `to`: no jump. The sound card's clocks wobble by a
    /// few milliseconds from block to block, so the song may seem to go
    /// back that much, or on a little further.
    fn goes_on(&self, from: usize, at: f64, to: usize) -> bool {
        /// How far back the song may seem to go: 20 ms.
        const WOBBLE_SECONDS: f64 = 0.02;
        let Some(then) = self.place(to) else {
            return false;
        };
        let expected = self.steps_in(from, to - from);
        let wobble = (WOBBLE_SECONDS * f64::from(self.sample_rate)) as usize;
        let back = self.steps_in(from, wobble).max(0.01);
        let moved = then - at;
        moved >= -back && moved <= expected + 0.25
    }

    /// The stretches of the take where the playlist position goes
    /// straight on, cut wherever it jumps, to the sample.
    fn runs(&self) -> Vec<Run> {
        // Checked every 5 ms; a jump between two checks is found to the
        // sample by halving the gap.
        let hop = (self.sample_rate / 200).max(1) as usize;
        let mut runs = Vec::new();
        let mut current: Option<(usize, f64)> = None; // first frame, its place
        let mut last: Option<(usize, f64)> = None; // last frame checked, its place
        let mut frame = 0;
        while frame < self.frames {
            let place = self.place(frame);
            match (last, place) {
                (Some((previous, at)), Some(_)) if self.goes_on(previous, at, frame) => {}
                (Some((previous, at)), _) => {
                    // The last sample that still goes on from `previous`.
                    let (mut good, mut bad) = (previous, frame);
                    while bad - good > 1 {
                        let middle = (good + bad) / 2;
                        if self.goes_on(previous, at, middle) {
                            good = middle;
                        } else {
                            bad = middle;
                        }
                    }
                    if let Some((first, from)) = current.take() {
                        let to = self.place(good).unwrap_or(from);
                        runs.push(Run {
                            frames: first..good + 1,
                            from,
                            to,
                        });
                    }
                    // The next run starts on the first sample past the
                    // jump, if the song is heard there.
                    current = self.place(bad).map(|place| (bad, place));
                    if bad < frame {
                        last = current;
                        // Check the frames from `bad` to `frame` again.
                        frame = bad + hop.min(frame - bad);
                        continue;
                    }
                }
                (None, Some(place)) => current = Some((frame, place)),
                (None, None) => {}
            }
            last = place.map(|place| (frame, place));
            if place.is_none() {
                current = None;
            }
            frame += hop;
        }
        if let (Some((first, from)), Some((mut end, mut at))) = (current, last) {
            // The last samples, up to the end of the take.
            while end + 1 < self.frames {
                let next = (end + hop).min(self.frames - 1);
                match self.place(next) {
                    Some(then) if self.goes_on(end, at, next) => (end, at) = (next, then),
                    _ => break,
                }
            }
            let to = self.place(end).unwrap_or(from);
            runs.push(Run {
                frames: first..end + 1,
                from,
                to,
            });
        }
        runs.retain(|run| run.to > run.from);
        runs
    }

    /// The first sample of `run` heard at or after playlist position
    /// `place` (the run's end if none is).
    fn frame_at(&self, run: &Run, place: f64) -> usize {
        let (mut low, mut high) = (run.frames.start, run.frames.end);
        while low < high {
            let middle = (low + high) / 2;
            if self.place(middle).is_some_and(|at| at >= place - 1e-9) {
                high = middle;
            } else {
                low = middle + 1;
            }
        }
        low
    }
}

/// The parts of each run that no later run sings again, as places in the
/// playlist: for each run, a list of `(from, to)`.
fn kept(runs: &[Run]) -> Vec<Vec<(f64, f64)>> {
    let mut covered: Vec<(f64, f64)> = Vec::new();
    let mut kept = vec![Vec::new(); runs.len()];
    for (index, run) in runs.iter().enumerate().rev() {
        let mut parts = vec![(run.from, run.to)];
        for &(from, to) in &covered {
            parts = parts
                .into_iter()
                .flat_map(|(start, end)| {
                    let mut left = Vec::new();
                    if start < from.min(end) {
                        left.push((start, from.min(end)));
                    }
                    if to.max(start) < end {
                        left.push((to.max(start), end));
                    }
                    left
                })
                .collect();
        }
        covered.push((run.from, run.to));
        kept[index] = parts;
    }
    kept
}

/// How hard a note sung `loudness` dB loud is played, when most of the
/// take is `usual` dB loud: 100 for the usual, louder notes harder and
/// softer ones softer. A voice's loud and soft lie further apart than an
/// instrument's velocities, so the difference is squeezed: 6 dB louder
/// plays at 127, 20 dB softer at 45, as from forte to piano.
pub fn velocity(loudness: f32, usual: f32) -> u8 {
    let gain = 10.0_f32.powf(0.35 * (loudness - usual) / 20.0);
    (100.0 * gain).round().clamp(20.0, 127.0) as u8
}

/// The note `pitch` (a fractional MIDI number, already in tune) is
/// meant as: the nearest, or, right between two, the one in `scale`.
fn meant(pitch: f32, scale: crate::Scale) -> Note {
    let below = pitch.floor();
    let fraction = pitch - below;
    let nearest = pitch.round();
    let midi = if (0.35..=0.65).contains(&fraction) {
        let low = Note::new(below.clamp(0.0, 127.0) as u8);
        let high = Note::new((below + 1.0).clamp(0.0, 127.0) as u8);
        match (
            scale.contains(low.pitch_class()),
            scale.contains(high.pitch_class()),
        ) {
            (true, false) => below,
            (false, true) => below + 1.0,
            _ => nearest,
        }
    } else {
        nearest
    };
    Note::new(midi.clamp(0.0, 127.0) as u8)
}

/// How the pitch of a sung note is written.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Pitches {
    /// The nearest semitone; right between two, the one in the key.
    #[default]
    AsSung,
    /// The nearest note of the key there.
    InKey,
    /// The nearest note of the key as the chord on the chord sheet there
    /// colours it: the chord's own notes, and the notes of the key that
    /// a chord note from outside the key does not replace. Over D7 in C
    /// major F♯ is written, not F. Right between two notes, the chord's.
    WithChords,
}

/// The notes a sung `pitch` may be written as, at `step`, and the ones
/// preferred right between two: `None` for any semitone.
fn allowed_at(
    project: &Project,
    along: &Along,
    step: usize,
    scale: crate::Scale,
    pitches: Pitches,
) -> Option<(PitchSet, PitchSet)> {
    let key: PitchSet = scale.pitch_classes().collect();
    match pitches {
        Pitches::AsSung => None,
        Pitches::InKey => Some((key, key)),
        Pitches::WithChords => {
            let chord = match along {
                Along::Song(_) => project
                    .chord_at(step)
                    .and_then(|marker| marker.harmony.pitch_set()),
                Along::Pattern { .. } => None,
            };
            let Some(chord) = chord else {
                return Some((key, key));
            };
            let mut allowed = chord;
            for note in key.iter() {
                let replaced = chord.iter().any(|tone| {
                    !key.contains(tone) && (tone.transpose(1) == note || tone.transpose(-1) == note)
                });
                if !replaced {
                    allowed.insert(note);
                }
            }
            Some((allowed, chord))
        }
    }
}

/// The note of `allowed` nearest to `pitch` (already in tune); right
/// between two, the one in `preferred`.
fn nearest_in(pitch: f32, allowed: PitchSet, preferred: PitchSet) -> Note {
    if allowed.is_empty() {
        return Note::new(pitch.round().clamp(0.0, 127.0) as u8);
    }
    let around = pitch.round() as i32;
    let mut best: Option<(f32, bool, i32)> = None;
    for midi in (around - 12)..=(around + 12) {
        if !(0..=127).contains(&midi) {
            continue;
        }
        let class = PitchClass::from_semitone((midi % 12) as u8);
        if !allowed.contains(class) {
            continue;
        }
        let distance = (midi as f32 - pitch).abs();
        let liked = preferred.contains(class);
        let better = match best {
            None => true,
            Some((closest, was_liked, _)) => {
                distance + 1e-3 < closest
                    || ((distance - closest).abs() <= 1e-3 && liked && !was_liked)
            }
        };
        if better {
            best = Some((distance, liked, midi));
        }
    }
    best.map_or_else(
        || Note::new(pitch.round().clamp(0.0, 127.0) as u8),
        |(_, _, midi)| Note::new(midi as u8),
    )
}

/// The playlist played straight on from its start, with no repeats or
/// loops, as far as a clip can start: where a take laid on it goes.
fn straight_on() -> Arrangement {
    Arrangement {
        plays: Vec::new(),
        passes: vec![Pass {
            start: 0,
            from: 0,
            length: MAX_SONG_STEPS,
            section: None,
            round: 1,
            rounds: 1,
            chorus: None,
        }],
        length: MAX_SONG_STEPS,
    }
}

/// How long before the song is heard a note may start and still be
/// kept, from where the song starts: 150 ms.
const EARLY_SECONDS: f64 = 0.15;

/// Works out where in the song `take` was sung, and which notes:
/// `sung` are the notes heard in it, `grid` how many steps the notes
/// snap to (1 for every step, 2 for eighth notes, 4 for beats) and
/// `tuning` how sharp it was sung (see `lineup_listen::voice::tuning`).
pub fn transcribe(
    project: &Project,
    take: &Take,
    sung: &[SungNote],
    grid: usize,
    tuning: f32,
) -> Transcription {
    transcribe_with(project, take, sung, grid, tuning, Pitches::AsSung)
}

/// [`transcribe`], with the pitches written as `pitches` says.
pub fn transcribe_with(
    project: &Project,
    take: &Take,
    sung: &[SungNote],
    grid: usize,
    tuning: f32,
    pitches: Pitches,
) -> Transcription {
    let grid = grid.max(1) as f64;
    let runs = take.runs();
    let kept = kept(&runs);
    let rate = f64::from(take.sample_rate.max(1));

    let mut pieces = Vec::new();
    for (run, parts) in runs.iter().zip(&kept) {
        for &(from, to) in parts {
            // The recording starts on the first whole step inside.
            let step = (from - 1e-6).ceil();
            if to - step < 1.0 {
                continue;
            }
            pieces.push(Piece {
                frames: take.frame_at(run, step)..take.frame_at(run, to),
                step: step as usize,
                steps: (to - step).ceil() as usize,
            });
        }
    }
    pieces.sort_by_key(|piece| piece.step);

    // Each note's place, if it was sung in a part that is kept.
    let mut heard: Vec<(f64, f64, f32, f32, f64)> = Vec::new(); // start, end, pitch, loudness, seconds
    // A note that starts a moment before the song is heard (the pitch
    // finder can hear its start a frame or two early) starts with it.
    let early = (EARLY_SECONDS * rate) as usize;
    for note in sung {
        let mut first = (note.start.max(0.0) * rate) as usize;
        let after = (note.end * rate) as usize;
        let index = match runs.iter().position(|run| run.frames.contains(&first)) {
            Some(index) => index,
            None => {
                let Some(index) = runs.iter().position(|run| {
                    run.frames.start > first
                        && run.frames.start < after
                        && run.frames.start - first <= early
                }) else {
                    continue;
                };
                first = runs[index].frames.start;
                index
            }
        };
        let run = &runs[index];
        let Some(start) = take.place(first) else {
            continue;
        };
        if !kept[index]
            .iter()
            .any(|&(from, to)| start >= from - 0.5 && start < to)
        {
            continue;
        }
        let last = ((note.end * rate) as usize).clamp(first, run.frames.end - 1);
        let end = take.place(last).unwrap_or(start).max(start);
        heard.push((start, end, note.pitch, note.loudness, note.seconds()));
    }

    // Most of the take is as loud as its middle note.
    let mut levels: Vec<f32> = heard.iter().map(|h| h.3).collect();
    levels.sort_by(f32::total_cmp);
    let usual = levels.get(levels.len() / 2).copied().unwrap_or(0.0);

    let snap = |place: f64| ((place / grid).round() * grid).max(0.0) as usize;
    let mut notes: Vec<(Placed, f64)> = Vec::new();
    for &(start, end, pitch, loudness, seconds) in &heard {
        let begin = snap(start);
        let finish = snap(end).max(begin + grid as usize);
        let scale = match &take.along {
            Along::Song(_) => project.key_at(begin),
            Along::Pattern { pattern, .. } => project.pattern_key(*pattern),
        };
        let pitch = match allowed_at(project, &take.along, begin, scale, pitches) {
            None => meant(pitch - tuning, scale),
            Some((allowed, preferred)) => nearest_in(pitch - tuning, allowed, preferred),
        };
        notes.push((
            Placed {
                start: begin,
                length: finish - begin,
                pitch,
                velocity: velocity(loudness, usual),
            },
            seconds,
        ));
    }
    // One note at a time: of two on the same step, the longer one sung
    // stays; a note ends where the next one starts.
    notes.sort_by(|a, b| a.0.start.cmp(&b.0.start).then(b.1.total_cmp(&a.1)));
    notes.dedup_by(|later, earlier| later.0.start == earlier.0.start);
    let mut placed: Vec<Placed> = notes.into_iter().map(|(note, _)| note).collect();
    for index in 1..placed.len() {
        let next = placed[index].start;
        let note = &mut placed[index - 1];
        if note.end() > next {
            note.length = next - note.start;
        } else if next - note.end() < grid as usize * 2 && next - note.end() <= 2 {
            // A breath or a consonant before the next note is no rest.
            note.length = next - note.start;
        }
    }
    if let Along::Pattern { length, .. } = take.along {
        placed.retain(|note| note.start < length);
        for note in &mut placed {
            note.length = note.length.min(length - note.start);
        }
    }
    Transcription {
        pieces,
        notes: placed,
        tuning,
    }
}

/// The channel that sings the tune, if any: the first whose name says
/// so, as "Lead", "Melody" or "Voice" (see
/// [`crate::library::apply::plays_the_tune`]), and is not a take.
pub fn tune_channel(project: &Project) -> Option<usize> {
    project.channels().iter().position(|channel| {
        crate::library::apply::plays_the_tune(&channel.name)
            && !matches!(channel.kind, ChannelKind::Sampler(SampleSource::Take(_)))
    })
}

/// Where the notes of a take go.
#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    /// In place of the notes this channel played there.
    Channel(usize),
    /// On a new channel of their own, with this name and General MIDI
    /// instrument, in the take's own clips.
    New { name: String, program: u8 },
    /// Nowhere: only the recording is kept.
    Nowhere,
}

/// What [`Project::add_take`] keeps of a take, and where.
#[derive(Debug, Clone, PartialEq)]
pub struct Keep {
    /// The name of the take, as "Take 1".
    pub name: String,
    /// The name of the clips of notes sung where the channel they go to
    /// has no clip, and of their track, as "Take 1 notes": the notes are
    /// never in the recording's clips, but on a track of their own.
    pub notes: String,
    /// The playlist track its clips go on, in song mode: past the last
    /// track for a new one at the bottom. An empty track with the name
    /// it was given ("Track 5") takes the name of the take.
    pub track: usize,
    /// The recording of each piece of the take, saved as a WAV file, in
    /// the order of [`Transcription::pieces`]; empty to keep no
    /// recording.
    pub recordings: Vec<PathBuf>,
}

/// What [`Project::add_take`] did, for the app to say.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Added {
    /// How many notes were written, and on which channel.
    pub notes: usize,
    pub channel: Option<usize>,
    /// The patterns whose notes changed that play more than once in the
    /// song, with how many times: the change is heard each time.
    pub shared: Vec<(usize, usize)>,
    /// The channels made for the recordings.
    pub recordings: Vec<usize>,
}

impl Project {
    /// Writes a take into the song: its notes on the channel `target`
    /// says, in place of the notes there were there, and its recording
    /// on track `keep.track` (in song mode) or in the pattern (in pattern
    /// mode), each piece a sampler channel of its own.
    ///
    /// In song mode the notes go into the patterns that already hold the
    /// target channel's notes where they were sung, so a tune corrected
    /// once is corrected wherever its pattern plays. Where no pattern
    /// holds them, they go into clips of their own, on a track of their
    /// own right under the recording's ([`Keep::notes`]): the recording's
    /// clips only ever hold the recording.
    pub fn add_take(
        &mut self,
        along: &Along,
        transcription: &Transcription,
        target: &Target,
        keep: &Keep,
    ) -> Added {
        let mut added = Added::default();
        let channel = match target {
            Target::Channel(channel) if *channel < self.channels().len() => Some(*channel),
            Target::New { name, program } => self.add_channel(Channel {
                name: name.clone(),
                program: Some(*program),
                ..Channel::synth()
            }),
            _ => None,
        };
        added.channel = channel;

        let mut recordings = Vec::new();
        for (index, path) in keep.recordings.iter().enumerate() {
            let name = if index == 0 {
                keep.name.clone()
            } else {
                format!("{} ({})", keep.name, index + 1)
            };
            if let Some(channel) = self.add_channel(Channel::take(&name, path.clone())) {
                recordings.push(channel);
            }
        }
        added.recordings.clone_from(&recordings);

        match along {
            Along::Pattern { pattern, .. } => {
                let pattern = *pattern;
                if let (Some(channel), Some(range)) = (channel, transcription.range()) {
                    added.notes =
                        self.replace_in_pattern(pattern, channel, range, 0, &transcription.notes);
                }
                for (piece, &take) in transcription.pieces.iter().zip(&recordings) {
                    if let Some(pattern) = self.pattern_mut(pattern) {
                        pattern.set_notes(take, [NoteEvent::new(STEP_NOTE, piece.step, 1)]);
                    }
                }
            }
            Along::Song(_) => {
                added.notes = self.write_take_in_song(
                    transcription,
                    channel,
                    &recordings,
                    keep,
                    &mut added.shared,
                );
            }
        }
        added
    }

    /// The notes of `channel` in pattern `pattern` that start in `range`
    /// (steps of the pattern, `offset` steps before the steps `notes` are
    /// placed on) replaced by the notes in it; a note from before the
    /// range that sounds into it ends where it starts. Returns how many
    /// notes were written.
    fn replace_in_pattern(
        &mut self,
        pattern: usize,
        channel: usize,
        range: Range<usize>,
        offset: usize,
        notes: &[Placed],
    ) -> usize {
        let Some(target) = self.pattern_mut(pattern) else {
            return 0;
        };
        let length = target.length();
        let local =
            range.start.saturating_sub(offset)..range.end.saturating_sub(offset).min(length);
        let mut kept: Vec<NoteEvent> = target
            .notes(channel)
            .iter()
            .filter(|note| !local.contains(&note.start))
            .map(|note| {
                let mut note = *note;
                if note.start < local.start && note.end() > local.start {
                    note.length = local.start - note.start;
                }
                note
            })
            .collect();
        let mut written = 0;
        for note in notes {
            let Some(start) = note.start.checked_sub(offset) else {
                continue;
            };
            if !local.contains(&start) {
                continue;
            }
            kept.push(NoteEvent {
                velocity: note.velocity,
                ..NoteEvent::new(note.pitch, start, note.length.min(length - start))
            });
            written += 1;
        }
        kept.sort_by_key(|note| (note.start, note.pitch));
        target.set_notes(channel, kept);
        written
    }

    /// [`Project::add_take`] in song mode.
    fn write_take_in_song(
        &mut self,
        transcription: &Transcription,
        channel: Option<usize>,
        recordings: &[usize],
        keep: &Keep,
        shared: &mut Vec<(usize, usize)>,
    ) -> usize {
        let range = transcription.range();
        // The clips that hold the channel's notes where the take was sung.
        let homes: Vec<Clip> = match (channel, &range) {
            (Some(channel), Some(range)) => self
                .playlist()
                .clips()
                .iter()
                .copied()
                .filter(|clip| {
                    let end = clip.start + self.clip_length(clip);
                    clip.start < range.end
                        && end > range.start
                        && self
                            .pattern(clip.pattern)
                            .is_some_and(|p| !p.notes(channel).is_empty())
                })
                .collect(),
            _ => Vec::new(),
        };

        // The notes that find a home, by clip; the rest go into clips of
        // their own.
        let mut homeless: Vec<Placed> = Vec::new();
        let mut written = 0;
        if let (Some(channel), Some(range)) = (channel, &range) {
            let mut done: Vec<usize> = Vec::new();
            for clip in &homes {
                if done.contains(&clip.pattern) {
                    continue;
                }
                done.push(clip.pattern);
                let length = self.pattern(clip.pattern).map_or(0, Pattern::length);
                if length == 0 {
                    continue;
                }
                // Every place this clip plays the pattern inside the range.
                let mut offset = clip.start;
                while offset < clip.start + self.clip_length(clip) {
                    let here = range.start.max(offset)..range.end.min(offset + length);
                    if here.start < here.end {
                        let notes: Vec<Placed> = transcription
                            .notes
                            .iter()
                            .copied()
                            .filter(|note| {
                                here.contains(&note.start)
                                    && first_home(&homes, note.start, self) == Some(*clip)
                            })
                            .collect();
                        written +=
                            self.replace_in_pattern(clip.pattern, channel, here, offset, &notes);
                    }
                    offset += length;
                }
                let uses = self
                    .playlist()
                    .clips()
                    .iter()
                    .filter(|other| other.pattern == clip.pattern)
                    .map(|other| other.repeats)
                    .sum::<usize>();
                if uses > 1 {
                    shared.push((clip.pattern, uses));
                }
            }
            homeless = transcription
                .notes
                .iter()
                .copied()
                .filter(|note| first_home(&homes, note.start, self).is_none())
                .collect();
        }

        // The recording's clips, on the take's track; the notes with no
        // home on a track of their own, right under it.
        let notes_span = homeless.first().map(|first| {
            first.start
                ..homeless
                    .iter()
                    .map(Placed::end)
                    .max()
                    .unwrap_or(first.end())
        });
        let recorded = transcription.recorded().filter(|_| !recordings.is_empty());
        let mut notes_track = None;
        if let Some(span) = recorded {
            let track = self.take_track(keep.track, &keep.name);
            for (pattern, start, end) in self.bar_clips(span, track, &keep.name) {
                for (piece, &take) in transcription.pieces.iter().zip(recordings) {
                    if (start..end).contains(&piece.step)
                        && let Some(p) = self.pattern_mut(pattern)
                    {
                        p.set_notes(take, [NoteEvent::new(STEP_NOTE, piece.step - start, 1)]);
                    }
                }
            }
            if notes_span.is_some() {
                notes_track = Some(match self.insert_track(track + 1) {
                    Some(below) => self.take_track(below, &keep.notes),
                    None => track,
                });
            }
        }
        if let (Some(channel), Some(span)) = (channel, notes_span) {
            let track = notes_track.unwrap_or_else(|| self.take_track(keep.track, &keep.notes));
            for (pattern, start, end) in self.bar_clips(span, track, &keep.notes) {
                let notes: Vec<Placed> = homeless
                    .iter()
                    .copied()
                    .filter(|note| (start..end).contains(&note.start))
                    .collect();
                written += self.replace_in_pattern(pattern, channel, start..end, start, &notes);
            }
        }
        written
    }

    /// New empty clips on `track` covering `span`: from the bar where it
    /// starts to the bar where it ends, in patterns of at most
    /// [`MAX_STEPS`] steps that start on bar lines, named `name`, then
    /// "`name` (2)" and so on. Returns each one's pattern, first step and
    /// end.
    fn bar_clips(
        &mut self,
        span: Range<usize>,
        track: usize,
        name: &str,
    ) -> Vec<(usize, usize, usize)> {
        let meter = self.meter();
        let mut clips = Vec::new();
        let mut start = meter.floor_to_bar(span.start);
        while start < span.end && start < MAX_SONG_STEPS {
            // Whole bars, as many as fit in a pattern.
            let mut end = start;
            while end < span.end {
                let bar = meter.bar_at(end);
                if bar.end() - start > MAX_STEPS {
                    break;
                }
                end = bar.end();
            }
            if end == start {
                end = (start + MAX_STEPS).min(span.end);
            }
            let named = if clips.is_empty() {
                name.to_string()
            } else {
                format!("{name} ({})", clips.len() + 1)
            };
            let Some(pattern) =
                self.push_pattern(Pattern::new(named, self.channels().len(), end - start))
            else {
                break;
            };
            self.add_clip(Clip::new(pattern, track, start));
            clips.push((pattern, start, end));
            start = end;
        }
        clips
    }
}

impl Project {
    /// The track a take's clips go on: `track`, or past the last one a
    /// new one at the bottom. An empty track still called as it was made
    /// ("Track 5") is named `name`.
    fn take_track(&mut self, track: usize, name: &str) -> usize {
        let tracks = self.playlist().tracks().len();
        let track = if track < tracks {
            track
        } else {
            self.add_track().unwrap_or(tracks.saturating_sub(1))
        };
        let empty = !self
            .playlist()
            .clips()
            .iter()
            .any(|clip| clip.track == track);
        if let Some(named) = self.track_mut(track)
            && empty
            && named.name.strip_prefix("Track ").is_some_and(|number| {
                !number.is_empty() && number.chars().all(|c| c.is_ascii_digit())
            })
        {
            named.name = name.to_string();
        }
        track
    }
}

/// The clip of `homes` that a note on step `step` goes into: the first
/// one (top track first) that plays there.
fn first_home(homes: &[Clip], step: usize, project: &Project) -> Option<Clip> {
    homes
        .iter()
        .filter(|clip| (clip.start..clip.start + project.clip_length(clip)).contains(&step))
        .min_by_key(|clip| (clip.track, clip.start))
        .copied()
}

impl Channel {
    /// A sampler channel playing a take saved at `path`.
    pub fn take(name: &str, path: PathBuf) -> Self {
        Channel {
            name: name.to_string(),
            kind: ChannelKind::Sampler(SampleSource::Take(path)),
            volume: 0.8,
            ..Channel::synth()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::playlist::Section;

    /// Samples a second, kept small so the tests run fast.
    const RATE: u32 = 1000;
    /// 120 beats a minute, four steps a beat.
    const SPEED: f64 = 8.0;

    /// A take `seconds` long, sung along with `along` from position
    /// `from` at a steady speed, with a mark every 50 ms as the audio
    /// thread writes them.
    fn take(along: Along, from: f64, seconds: f64) -> Take {
        let frames = (seconds * f64::from(RATE)) as usize;
        let length = along.length() as f64;
        let marks = (0..frames)
            .step_by(50)
            .map(|frame| Mark {
                frame,
                position: (from + frame as f64 / f64::from(RATE) * SPEED).rem_euclid(length),
                steps_per_second: SPEED,
            })
            .collect();
        Take {
            sample_rate: RATE,
            frames,
            marks,
            along,
        }
    }

    fn sung(start: f64, end: f64, pitch: f32, loudness: f32) -> SungNote {
        SungNote {
            start,
            end,
            pitch,
            loudness,
        }
    }

    /// A song in C major with a Lead playing a four-bar tune of quarter
    /// notes, C D E F over and over, twice in a row on track 1.
    fn song() -> Project {
        let mut project = Project::empty();
        let lead = project
            .add_channel(Channel {
                name: "Lead".into(),
                ..Channel::synth()
            })
            .unwrap();
        let pattern = project.pattern_mut(0).unwrap();
        pattern.name = "Tune".into();
        pattern.set_length(64);
        let tune = [60, 62, 64, 65];
        pattern.set_notes(
            lead,
            (0..16).map(|beat| NoteEvent::new(Note::new(tune[beat % 4]), beat * 4, 4)),
        );
        project.add_clip(Clip::new(0, 0, 0)).unwrap();
        project.add_clip(Clip::new(0, 0, 64)).unwrap();
        project
    }

    fn keep(project: &Project, pieces: usize) -> Keep {
        Keep {
            name: "Take 1".into(),
            notes: "Take 1 notes".into(),
            track: 1.min(project.playlist().tracks().len() - 1),
            recordings: (0..pieces)
                .map(|i| PathBuf::from(format!("take-{i}.wav")))
                .collect(),
        }
    }

    #[test]
    fn notes_land_on_the_steps_they_were_sung_on() {
        let project = song();
        let along = Along::Song(project.playback_arrangement());
        // From bar 2 (step 16) on, for four seconds.
        let take = take(along, 16.0, 4.0);
        let notes = [
            sung(0.0, 0.45, 64.1, -12.0),
            sung(0.5, 0.95, 66.9, -12.0),
            // Right between C5 and C#5: C5 is in C major.
            sung(1.0, 1.9, 72.45, -6.0),
            sung(2.0, 2.4, 69.0, -30.0),
        ];
        let heard = transcribe(&project, &take, &notes, 1, 0.0);
        let placed: Vec<(usize, usize, u8, u8)> = heard
            .notes
            .iter()
            .map(|n| (n.start, n.length, n.pitch.midi(), n.velocity))
            .collect();
        assert_eq!(
            placed,
            [
                (16, 4, 64, 100),
                (20, 4, 67, 100),
                (24, 8, 72, 127),
                (32, 3, 69, 48)
            ]
        );
        assert_eq!(heard.range(), Some(16..35));
        assert_eq!(heard.pieces.len(), 1);
        assert_eq!(heard.pieces[0].step, 16);
        assert_eq!(heard.pieces[0].steps, 32);
        assert!(heard.pieces[0].frames.start <= 1, "{:?}", heard.pieces);
    }

    #[test]
    fn a_take_replaces_the_tune_where_it_was_sung_and_stays_on_its_track() {
        let mut project = song();
        let along = Along::Song(project.playback_arrangement());
        let take = take(along.clone(), 16.0, 4.0);
        let notes = [sung(0.0, 0.45, 67.0, -12.0), sung(0.5, 0.95, 69.0, -12.0)];
        let heard = transcribe(&project, &take, &notes, 1, 0.0);
        let keep = keep(&project, heard.pieces.len());
        let added = project.add_take(&along, &heard, &Target::Channel(0), &keep);

        assert_eq!(added.notes, 2);
        assert_eq!(added.channel, Some(0));
        // The Tune pattern plays twice: the change is heard both times.
        assert_eq!(added.shared, [(0, 2)]);
        let tune: Vec<(usize, u8)> = project.patterns()[0]
            .notes(0)
            .iter()
            .map(|n| (n.start, n.pitch.midi()))
            .collect();
        // Steps 16 to 24 hold the take's G and A; the rest is as before.
        assert_eq!(&tune[3..7], [(12, 65), (16, 67), (20, 69), (24, 64)]);

        // The recording: a take channel, and a clip on track 2 from bar 2.
        let take_channel = added.recordings[0];
        assert!(project.channels()[take_channel].is_take());
        assert_eq!(project.channels()[take_channel].name, "Take 1");
        let clip = *project.playlist().clips().last().unwrap();
        assert_eq!((clip.track, clip.start), (1, 16));
        let pattern = &project.patterns()[clip.pattern];
        assert_eq!(pattern.name, "Take 1");
        assert_eq!(pattern.length(), 32);
        assert_eq!(
            pattern.notes(take_channel),
            [NoteEvent::new(STEP_NOTE, 0, 1)]
        );
        assert!(pattern.notes(0).is_empty(), "the tune's notes found a home");
        // Track 2 was empty and had the name it was made with.
        assert_eq!(project.playlist().tracks()[1].name, "Take 1");

        // Saved and opened again, the take is still a take, in a format
        // older versions know they cannot read.
        assert_eq!(crate::file::format_for(&project), 32);
        let text = crate::file::to_text(&project).unwrap();
        let opened = crate::file::from_text(&text).unwrap();
        assert!(opened.channels()[take_channel].is_take());
        assert_eq!(opened.playlist().clips(), project.playlist().clips());
    }

    #[test]
    fn a_take_can_go_on_a_new_track_and_changes_only_the_parts_it_sings() {
        use crate::sources::SongPart;
        let mut project = song();
        let parts = |project: &Project| SongPart::ALL.map(|part| project.part_fingerprint(part));
        let before = parts(&project);
        let tracks = project.playlist().tracks().len();
        let along = Along::Song(project.playback_arrangement());
        let take = take(along.clone(), 16.0, 2.0);
        let heard = transcribe(&project, &take, &[sung(0.0, 0.45, 67.0, -12.0)], 1, 0.0);
        let keep = Keep {
            track: tracks,
            ..keep(&project, heard.pieces.len())
        };
        project.add_take(&along, &heard, &Target::Nowhere, &keep);
        assert_eq!(project.playlist().tracks().len(), tracks + 1);
        assert_eq!(project.playlist().tracks()[tracks].name, "Take 1");
        assert_eq!(project.playlist().clips().last().unwrap().track, tracks);
        // Only the recording came: no part of the song changed.
        assert_eq!(parts(&project), before);

        // Its notes on the Lead change the melody, and only that.
        let keep = Keep {
            name: "Take 2".into(),
            notes: "Take 2 notes".into(),
            track: tracks + 1,
            recordings: Vec::new(),
        };
        project.add_take(&along, &heard, &Target::Channel(0), &keep);
        let after = parts(&project);
        for (index, part) in SongPart::ALL.iter().enumerate() {
            assert_eq!(
                after[index] != before[index],
                *part == SongPart::Melody,
                "{part:?}"
            );
        }
    }

    #[test]
    fn a_take_on_a_new_channel_keeps_its_notes_on_a_track_under_the_recording() {
        let mut project = song();
        let along = Along::Song(project.playback_arrangement());
        let take = take(along.clone(), 16.0, 2.0);
        let notes = [sung(0.0, 0.45, 67.0, -12.0)];
        let heard = transcribe(&project, &take, &notes, 1, 0.0);
        let keep = keep(&project, heard.pieces.len());
        let target = Target::New {
            name: "Sung".into(),
            program: 53,
        };
        let added = project.add_take(&along, &heard, &target, &keep);
        let sung = added.channel.unwrap();
        assert_eq!(project.channels()[sung].name, "Sung");
        assert_eq!(project.channels()[sung].program, Some(53));
        assert_eq!(project.patterns()[0].notes(0).len(), 16, "the Lead stays");

        // The recording on its track, the notes on the next one, each in
        // clips of their own.
        let tracks = project.playlist().tracks();
        assert_eq!(tracks[1].name, "Take 1");
        assert_eq!(tracks[2].name, "Take 1 notes");
        let on = |track: usize| -> Vec<&Pattern> {
            project
                .playlist()
                .clips()
                .iter()
                .filter(|clip| clip.track == track)
                .map(|clip| &project.patterns()[clip.pattern])
                .collect()
        };
        let recording = on(1);
        assert!(!recording.is_empty());
        for pattern in &recording {
            assert!(
                pattern.notes(sung).is_empty(),
                "no notes with the recording"
            );
            assert!(
                added
                    .recordings
                    .iter()
                    .any(|&take| !pattern.notes(take).is_empty())
            );
        }
        let notes = on(2);
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].name, "Take 1 notes");
        assert_eq!(notes[0].notes(sung), [NoteEvent::new(Note::new(67), 0, 4)]);
        assert!(
            added
                .recordings
                .iter()
                .all(|&take| notes[0].notes(take).is_empty())
        );
    }

    #[test]
    fn a_take_kept_without_its_recording_has_only_its_notes_track() {
        let mut project = song();
        let along = Along::Song(project.playback_arrangement());
        let take = take(along.clone(), 16.0, 2.0);
        let notes = [sung(0.0, 0.45, 67.0, -12.0)];
        let heard = transcribe(&project, &take, &notes, 1, 0.0);
        let keep = Keep {
            recordings: Vec::new(),
            track: 1,
            ..keep(&project, 0)
        };
        let target = Target::New {
            name: "Sung".into(),
            program: 53,
        };
        let tracks = project.playlist().tracks().len();
        project.add_take(&along, &heard, &target, &keep);
        assert_eq!(project.playlist().tracks().len(), tracks);
        assert_eq!(project.playlist().tracks()[1].name, "Take 1 notes");
    }

    #[test]
    fn a_loop_sung_twice_keeps_the_second_time() {
        let mut project = song();
        project.pattern_mut(0).unwrap().set_length(16);
        let along = Along::Pattern {
            pattern: 0,
            length: 16,
        };
        // Two times round a two-second pattern.
        let take = take(along.clone(), 0.0, 4.0);
        let notes = [sung(0.0, 0.45, 64.0, -12.0), sung(2.0, 2.45, 67.0, -12.0)];
        let heard = transcribe(&project, &take, &notes, 1, 0.0);
        let placed: Vec<(usize, u8)> = heard
            .notes
            .iter()
            .map(|n| (n.start, n.pitch.midi()))
            .collect();
        assert_eq!(placed, [(0, 67)]);
        assert_eq!(heard.pieces.len(), 1);
        let piece = &heard.pieces[0];
        assert_eq!((piece.step, piece.steps), (0, 16));
        assert!(piece.frames.start.abs_diff(2000) <= 1, "{piece:?}");

        let keep = keep(&project, 1);
        let added = project.add_take(&along, &heard, &Target::Channel(0), &keep);
        let take = added.recordings[0];
        assert_eq!(
            project.patterns()[0].notes(take),
            [NoteEvent::new(STEP_NOTE, 0, 1)]
        );
        assert_eq!(project.patterns()[0].notes(0)[0].pitch, Note::new(67));
    }

    #[test]
    fn a_chorus_sung_twice_keeps_the_second_time() {
        let mut project = song();
        let mut chorus = Section::new("Chorus", 0, 16);
        chorus.repeats = 2;
        project.add_section(chorus).unwrap();
        let along = Along::Song(project.playback_arrangement());
        let take = take(along, 0.0, 4.0);
        let notes = [sung(0.0, 0.45, 64.0, -12.0), sung(2.0, 2.45, 67.0, -12.0)];
        let heard = transcribe(&project, &take, &notes, 1, 0.0);
        let placed: Vec<(usize, u8)> = heard
            .notes
            .iter()
            .map(|n| (n.start, n.pitch.midi()))
            .collect();
        assert_eq!(placed, [(0, 67)]);
        assert_eq!(heard.pieces.len(), 1);
        assert_eq!(heard.pieces[0].step, 0);
    }

    #[test]
    fn what_is_sung_before_the_song_starts_is_left_out() {
        let project = song();
        let along = Along::Song(project.playback_arrangement());
        let mut take = take(along, 16.0, 3.0);
        // The song only started a second in, after a count-in.
        take.marks.retain(|mark| mark.frame >= 1000);
        for mark in &mut take.marks {
            mark.position -= SPEED;
        }
        let notes = [sung(0.2, 0.6, 60.0, -12.0), sung(1.0, 1.4, 64.0, -12.0)];
        let heard = transcribe(&project, &take, &notes, 1, 0.0);
        let placed: Vec<(usize, u8)> = heard
            .notes
            .iter()
            .map(|n| (n.start, n.pitch.midi()))
            .collect();
        assert_eq!(placed, [(16, 64)]);
        assert_eq!(heard.pieces[0].frames.start, 1000);
    }

    #[test]
    fn a_sound_card_whose_clock_wobbles_does_not_cut_the_take() {
        let project = song();
        let along = Along::Song(project.playback_arrangement());
        let mut take = take(along, 16.0, 4.0);
        // Every so often, a mark 4 ms late, and the next one on time: the
        // song seems to go back 4 ms.
        for (index, mark) in take.marks.iter_mut().enumerate() {
            if index % 97 == 50 {
                mark.position += 0.004 * SPEED;
            }
        }
        let heard = transcribe(&project, &take, &[sung(0.0, 3.5, 67.0, -12.0)], 1, 0.0);
        assert_eq!(heard.pieces.len(), 1, "{:?}", heard.pieces);
        assert_eq!(heard.pieces[0].step, 16);
    }

    #[test]
    fn a_note_heard_a_moment_before_the_song_starts_with_it() {
        let project = song();
        let along = Along::Song(project.playback_arrangement());
        let mut take = take(along, 0.0, 2.0);
        // The song is heard from 0.1 s into the take on.
        let delay = RATE as usize / 10;
        take.frames += delay;
        for mark in &mut take.marks {
            mark.frame += delay;
        }
        let notes = [sung(0.09, 0.55, 66.0, -12.0), sung(0.0, 0.05, 60.0, -12.0)];
        let heard = transcribe(&project, &take, &notes, 1, 0.0);
        let placed: Vec<(usize, u8)> = heard
            .notes
            .iter()
            .map(|n| (n.start, n.pitch.midi()))
            .collect();
        // The first starts with the song; the second ends before it.
        assert_eq!(placed, [(0, 66)]);
    }

    #[test]
    fn a_recording_laid_on_the_playlist_goes_straight_on_at_the_songs_tempo() {
        let mut project = song();
        project.tempo = crate::tempo::Tempo::new(120.0);
        // Bar 3 on is twice as fast: 240 BPM, 16 steps a second.
        assert!(project.set_tempo_marker(crate::tempo::TempoMarker::new(32, 240.0)));
        let rate = 1000;
        // From bar 2: 16 steps at 120 BPM take 2 s, then 16 a second.
        let take = Take::laid(&project, 16, rate, 4 * rate as usize);
        let notes = [
            sung(0.0, 0.45, 67.0, -12.0),
            sung(1.5, 1.9, 69.0, -12.0),
            sung(2.5, 2.7, 71.0, -12.0),
        ];
        let heard = transcribe(&project, &take, &notes, 1, 0.0);
        let placed: Vec<(usize, usize, u8)> = heard
            .notes
            .iter()
            .map(|n| (n.start, n.length, n.pitch.midi()))
            .collect();
        assert_eq!(placed, [(16, 4, 67), (28, 3, 69), (40, 3, 71)]);
        assert_eq!(heard.pieces.len(), 1);
        assert_eq!(
            (heard.pieces[0].step, heard.pieces[0].frames.start),
            (16, 0)
        );
        // Past the end of the song, where the song itself would start
        // again: a sound on a track goes on.
        let late = Take::laid(&project, 120, rate, 4 * rate as usize);
        let heard = transcribe(&project, &late, &[sung(2.0, 2.4, 67.0, -12.0)], 1, 0.0);
        assert_eq!(heard.notes[0].start, 152);
    }

    #[test]
    fn a_take_sung_sharp_is_brought_in_tune() {
        let project = song();
        let along = Along::Song(project.playback_arrangement());
        let take = take(along, 0.0, 2.0);
        let notes = [sung(0.0, 0.4, 60.6, -12.0), sung(0.5, 0.9, 62.6, -12.0)];
        let heard = transcribe(&project, &take, &notes, 1, 0.6);
        let pitches: Vec<u8> = heard.notes.iter().map(|n| n.pitch.midi()).collect();
        assert_eq!(pitches, [60, 62]);
    }

    #[test]
    fn notes_snap_to_the_grid_asked_for() {
        let project = song();
        let along = Along::Song(project.playback_arrangement());
        let take = take(along, 0.0, 2.0);
        // Sung a little late, on step 1.25.
        let notes = [sung(0.15, 0.6, 60.0, -12.0)];
        let on_steps = transcribe(&project, &take, &notes, 1, 0.0);
        assert_eq!(on_steps.notes[0].start, 1);
        let on_beats = transcribe(&project, &take, &notes, 4, 0.0);
        assert_eq!((on_beats.notes[0].start, on_beats.notes[0].length), (0, 4));
    }

    #[test]
    fn loud_notes_are_played_harder() {
        assert_eq!(velocity(-12.0, -12.0), 100);
        assert_eq!(velocity(-6.0, -12.0), 127);
        assert!(velocity(-32.0, -12.0) < 50);
        assert!(velocity(-60.0, -12.0) >= 20);
    }

    #[test]
    fn pitches_can_keep_to_the_key_or_to_the_chords() {
        use crate::chord_sheet::{ChordMarker, HarmonyName};
        let mut project = song();
        // D7 over bar 2: its F♯ takes the place of the key's F.
        let d7 = ChordMarker::new(16, HarmonyName::parse("D7").unwrap());
        assert!(project.set_chords(16, 32, &[d7]));
        let along = Along::Song(project.playback_arrangement());
        let take = take(along, 16.0, 2.0);
        // A little sharp of C♯, and a little sharp of F.
        let notes = [sung(0.0, 0.45, 61.1, -12.0), sung(0.5, 0.95, 65.3, -12.0)];
        let written = |pitches| -> Vec<u8> {
            transcribe_with(&project, &take, &notes, 1, 0.0, pitches)
                .notes
                .iter()
                .map(|note| note.pitch.midi())
                .collect()
        };
        assert_eq!(written(Pitches::AsSung), [61, 65], "as sung");
        assert_eq!(written(Pitches::InKey), [62, 65], "C♯ is not in C major");
        assert_eq!(written(Pitches::WithChords), [62, 66], "F♯ over D7");
    }

    #[test]
    fn the_tune_channel_is_the_lead() {
        let mut project = song();
        project.add_channel(Channel::take("Take 1", "t.wav".into()));
        assert_eq!(tune_channel(&project), Some(0));
        let mut drums = Project::template();
        drums.add_channel(Channel::take("Voice take", "t.wav".into()));
        assert_eq!(tune_channel(&drums), None);
    }
}
