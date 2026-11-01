//! Takes: a voice recorded while the song plays, kept on a track of the
//! playlist, and the notes it sang written into the song; or the drums,
//! or the chords, a recording played.
//!
//! You sing or whistle along with the song; the app records the
//! microphone, and `lineup_listen::voice` hears the notes in the
//! recording, in seconds from its start. This module works out where in
//! the song each of those seconds was, writes the notes there, on the
//! grid, and keeps the recording on a track, where it plays with the song
//! like any other part. In the same way the drum hits heard in a take
//! (`lineup_listen::drums`) go on the drum channels, and the chords heard
//! (`lineup_listen::chords`) on the chord sheet.
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
//!
//! **Which drums?** Each hit goes to the nearest point of the grid, on
//! the channel of its drum, as hard as it was hit against that drum's
//! other hits. They take the place of the kick, snare and hi-hats there
//! were, from the first hit to the last; the drums a take does not hear
//! (toms, cymbals, percussion) stay as they were.
//!
//! **Which chords?** Each beat gets the chord heard longest in it, spelled
//! in the key there; a marker goes where the chord changes. They take the
//! place of the chord sheet from the beat of the first chord to the end
//! of the beat of the last, and the chord that played on after that still
//! does. The chord sheet is the song's, so only a take sung along with
//! the song, not with a pattern, writes chords.

use std::ops::Range;
use std::path::PathBuf;

use crate::arrangement::{Arrangement, Pass};
use crate::chord_sheet::{ChordMarker, HarmonyName};
use crate::harmony::{Chord, PitchSet, chord_scale};
use crate::pattern::{MAX_STEPS, NoteEvent, Pattern, STEP_NOTE};
use crate::pitch::{Note, PitchClass};
use crate::playlist::{Clip, MAX_SONG_STEPS};
use crate::project::{Channel, ChannelKind, Drum, Project, SampleSource};

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

/// One drum hit heard in a take.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrumHit {
    /// When it was hit, in seconds from the start of the take.
    pub time: f64,
    pub drum: Drum,
    /// How loud it was, in dB: only how the hits of one drum compare
    /// counts.
    pub loudness: f32,
}

/// One bass note heard in a take.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BassNote {
    /// When it starts and ends, in seconds from the start of the take.
    pub start: f64,
    pub end: f64,
    /// Its pitch, as a fractional MIDI note number: 28.0 is E1.
    pub pitch: f32,
    /// Its loudest moment, in dB below full scale.
    pub loudness: f32,
}

impl BassNote {
    pub fn seconds(&self) -> f64 {
        self.end - self.start
    }
}

/// A chord heard in a take, from `start` to `end` seconds from its start.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HeardChord {
    pub start: f64,
    pub end: f64,
    /// The chord, or `None` where no chord sounds (silence, or drums
    /// alone).
    pub chord: Option<Chord>,
    /// The lowest note, when it is not the chord's root: C/E.
    pub bass: Option<PitchClass>,
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

/// A drum hit of a take on the grid: on step `step` of the playlist (or
/// of the pattern).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlacedHit {
    pub step: usize,
    pub drum: Drum,
    pub velocity: u8,
}

/// The drums a take's hits take the place of: the ones a recording is
/// heard for (see `lineup_listen::drums`).
pub const HEARD_DRUMS: [Drum; 4] = [Drum::Kick, Drum::Snare, Drum::HiHat, Drum::OpenHat];

/// The chords of a take, for the chord sheet: its markers, and the steps
/// whose chords they take the place of.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SheetChords {
    pub markers: Vec<ChordMarker>,
    pub steps: Range<usize>,
}

/// What a take sang, ready to write into the song.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Transcription {
    /// The stretches of the recording to keep, in the order they play.
    pub pieces: Vec<Piece>,
    /// The notes, in order, one at a time.
    pub notes: Vec<Placed>,
    /// The bass line, in order.
    pub bass: Vec<Placed>,
    /// How sharp the take was sung, in semitones (see `lineup_listen::voice::tuning`).
    pub tuning: f32,
    /// The drum hits, in order (see [`transcribe_drums`]).
    pub drums: Vec<PlacedHit>,
    /// The chords, in song mode (see [`transcribe_chords`]).
    pub chords: Option<SheetChords>,
}

impl Transcription {
    /// The steps from the start of the first note to the end of the last:
    /// where the notes there were are replaced.
    pub fn range(&self) -> Option<Range<usize>> {
        let start = self.notes.first()?.start;
        let end = self.notes.iter().map(Placed::end).max()?;
        Some(start..end)
    }

    /// The steps from the first drum hit to the last: where the kick,
    /// snare and hi-hats there were are replaced.
    pub fn drum_range(&self) -> Option<Range<usize>> {
        let first = self.drums.iter().map(|hit| hit.step).min()?;
        let last = self.drums.iter().map(|hit| hit.step).max()?;
        Some(first..last + 1)
    }

    /// The steps from the first bass note to the last.
    pub fn bass_range(&self) -> Option<Range<usize>> {
        let start = self.bass.iter().map(|note| note.start).min()?;
        let end = self.bass.iter().map(Placed::end).max()?;
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
    /// The nearest note of the scale a soloist plays over the chord on
    /// the chord sheet there, in the key (see [`chord_scale`]): over D7
    /// in C major, G major, so F♯ is written and not F. Right between two
    /// notes, the chord's.
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
            let harmony = match along {
                Along::Song(_) => project.chord_at(step).map(|marker| marker.harmony),
                Along::Pattern { .. } => None,
            };
            let Some(chord) = harmony.and_then(HarmonyName::pitch_set) else {
                return Some((key, key));
            };
            let allowed = match harmony {
                Some(HarmonyName::Chord(symbol)) => {
                    let mut allowed = chord_scale(symbol.chord(), scale);
                    allowed.insert(symbol.bass_pitch_class());
                    allowed
                }
                // A micromode is the scale there.
                Some(HarmonyName::Micromode(set)) => set,
                // A set, or two chords at once: the key with their notes.
                _ => PitchSet::from_bits(key.bits() | chord.bits()),
            };
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

/// Where a sound heard from sample `first` of `take` to sample `after`
/// starts in the playlist (or the pattern), if in a part of the take that
/// is kept: the run it starts in, its first sample there and its place. A
/// sound that starts a moment before the song is heard (a pitch or an
/// onset can be heard a frame or two early) starts with it.
fn kept_start(
    take: &Take,
    runs: &[Run],
    kept: &[Vec<(f64, f64)>],
    first: usize,
    after: usize,
) -> Option<(usize, usize, f64)> {
    let early = (EARLY_SECONDS * f64::from(take.sample_rate.max(1))) as usize;
    let (index, first) = match runs.iter().position(|run| run.frames.contains(&first)) {
        Some(index) => (index, first),
        None => {
            let index = runs.iter().position(|run| {
                run.frames.start > first
                    && run.frames.start < after
                    && run.frames.start - first <= early
            })?;
            (index, runs[index].frames.start)
        }
    };
    let start = take.place(first)?;
    kept[index]
        .iter()
        .any(|&(from, to)| start >= from - 0.5 && start < to)
        .then_some((index, first, start))
}

/// The stretches of `take` to keep, in the order they play.
fn pieces(take: &Take, runs: &[Run], kept: &[Vec<(f64, f64)>]) -> Vec<Piece> {
    let mut pieces = Vec::new();
    for (run, parts) in runs.iter().zip(kept) {
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
    pieces
}

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
    let pieces = pieces(take, &runs, &kept);

    // Each note's place, if it was sung in a part that is kept.
    let mut heard: Vec<(f64, f64, f32, f32, f64)> = Vec::new(); // start, end, pitch, loudness, seconds
    for note in sung {
        let first = (note.start.max(0.0) * rate) as usize;
        let after = (note.end * rate) as usize;
        let Some((index, first, start)) = kept_start(take, &runs, &kept, first, after) else {
            continue;
        };
        let run = &runs[index];
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
        ..Transcription::default()
    }
}

/// Works out where in the song (or pattern) each of the bass `notes`
/// heard in `take` was played, on a grid of `grid` steps. Of two notes
/// on one step, the longer stays; one note ends where the next starts.
/// Bass pitches stay as heard, including chromatic notes outside the key.
pub fn transcribe_bass(take: &Take, notes: &[BassNote], grid: usize) -> Vec<Placed> {
    let grid = grid.max(1) as f64;
    let runs = take.runs();
    let kept = kept(&runs);
    let rate = f64::from(take.sample_rate.max(1));
    let mut heard = Vec::new();
    for note in notes {
        let first = (note.start.max(0.0) * rate) as usize;
        let after = (note.end * rate) as usize;
        let Some((index, first, start)) = kept_start(take, &runs, &kept, first, after) else {
            continue;
        };
        let run = &runs[index];
        let last = ((note.end * rate) as usize).clamp(first, run.frames.end - 1);
        let end = take.place(last).unwrap_or(start).max(start);
        heard.push((start, end, note.pitch, note.loudness, note.seconds()));
    }
    if heard.is_empty() {
        return Vec::new();
    }
    let mut levels: Vec<f32> = heard.iter().map(|note| note.3).collect();
    levels.sort_by(f32::total_cmp);
    let usual = levels[levels.len() / 2];
    let snap = |place: f64| ((place / grid).round() * grid).max(0.0) as usize;
    let mut placed: Vec<(Placed, f64)> = heard
        .iter()
        .map(|&(start, end, pitch, loudness, seconds)| {
            let begin = snap(start);
            let finish = snap(end).max(begin + grid as usize);
            (
                Placed {
                    start: begin,
                    length: finish - begin,
                    pitch: Note::new(pitch.round().clamp(0.0, 127.0) as u8),
                    velocity: velocity(loudness, usual),
                },
                seconds,
            )
        })
        .collect();
    placed.sort_by(|a, b| a.0.start.cmp(&b.0.start).then(b.1.total_cmp(&a.1)));
    placed.dedup_by(|later, earlier| later.0.start == earlier.0.start);
    let mut placed: Vec<Placed> = placed.into_iter().map(|(note, _)| note).collect();
    for index in 1..placed.len() {
        let next = placed[index].start;
        let note = &mut placed[index - 1];
        if note.end() > next {
            note.length = next - note.start;
        }
    }
    if let Along::Pattern { length, .. } = take.along {
        placed.retain(|note| note.start < length);
        for note in &mut placed {
            note.length = note.length.min(length - note.start);
        }
    }
    placed
}

/// Works out where in the song (or the pattern) each of the drum `hits`
/// heard in `take` was played, on a grid of `grid` steps. Each is played
/// as hard as it was hit against the other hits of its drum; of two hits
/// of a drum on one step, the harder stays.
pub fn transcribe_drums(take: &Take, hits: &[DrumHit], grid: usize) -> Vec<PlacedHit> {
    let grid = grid.max(1) as f64;
    let runs = take.runs();
    let kept = kept(&runs);
    let rate = f64::from(take.sample_rate.max(1));
    // Each drum's usual loudness: its middle hit's.
    let usual = |drum: Drum| {
        let mut levels: Vec<f32> = hits
            .iter()
            .filter(|hit| hit.drum == drum)
            .map(|hit| hit.loudness)
            .collect();
        levels.sort_by(f32::total_cmp);
        levels.get(levels.len() / 2).copied().unwrap_or(0.0)
    };
    let usual: Vec<(Drum, f32)> = Drum::ALL.iter().map(|&drum| (drum, usual(drum))).collect();
    let length = match take.along {
        Along::Pattern { length, .. } => Some(length.max(1)),
        Along::Song(_) => None,
    };
    let mut placed: Vec<PlacedHit> = Vec::new();
    for hit in hits {
        let first = (hit.time.max(0.0) * rate) as usize;
        let early = (EARLY_SECONDS * rate) as usize;
        let Some((_, _, place)) = kept_start(take, &runs, &kept, first, first + early + 1) else {
            continue;
        };
        let mut step = ((place / grid).round() * grid).max(0.0) as usize;
        // On the last step of a pattern's round, a hit snapped on is the
        // first of the next.
        if let Some(length) = length {
            step %= length;
        }
        let usual = usual
            .iter()
            .find(|(drum, _)| *drum == hit.drum)
            .map_or(0.0, |(_, level)| *level);
        placed.push(PlacedHit {
            step,
            drum: hit.drum,
            velocity: velocity(hit.loudness, usual),
        });
    }
    placed.sort_by(|a, b| a.step.cmp(&b.step).then(b.velocity.cmp(&a.velocity)));
    let mut kept_hits: Vec<PlacedHit> = Vec::new();
    for hit in placed {
        let twice = kept_hits
            .iter()
            .rev()
            .take_while(|other| other.step == hit.step)
            .any(|other| other.drum == hit.drum);
        if !twice {
            kept_hits.push(hit);
        }
    }
    kept_hits
}

/// How often the chord heard is looked at, to name each beat's: every
/// 10 ms.
const CHORD_LOOK_SECONDS: f64 = 0.01;

/// Works out the chords of a take sung along with the song, for the
/// chord sheet: each beat whose time was heard for at least half gets
/// the chord heard there longest, spelled in the key there; a marker goes
/// where it changes. Silence before the first chord and after the last
/// changes nothing; in between it is "no chord". `None` for a take sung
/// along with a pattern, or where no chord was heard.
pub fn transcribe_chords(
    project: &Project,
    take: &Take,
    chords: &[HeardChord],
) -> Option<SheetChords> {
    if !matches!(take.along, Along::Song(_)) {
        return None;
    }
    type Heard = (Option<Chord>, Option<PitchClass>);
    let runs = take.runs();
    let kept = kept(&runs);
    let rate = f64::from(take.sample_rate.max(1));
    let look = ((CHORD_LOOK_SECONDS * rate) as usize).max(1);
    let meter = project.meter();
    // For each beat, by its first step: how often each harmony was heard
    // there, and which of its steps were heard (a bit each).
    let mut beats: std::collections::BTreeMap<usize, (Vec<(Heard, usize)>, u64)> =
        std::collections::BTreeMap::new();
    for (run, parts) in runs.iter().zip(&kept) {
        for frame in run.frames.clone().step_by(look) {
            let Some(place) = take.place(frame) else {
                continue;
            };
            if !parts.iter().any(|&(from, to)| place >= from && place < to) {
                continue;
            }
            let seconds = frame as f64 / rate;
            let Some(chord) = chords
                .partition_point(|chord| chord.start <= seconds)
                .checked_sub(1)
                .map(|index| chords[index])
                .filter(|chord| seconds < chord.end)
            else {
                continue;
            };
            let step = place.floor() as usize;
            let beat = meter.floor_to_beat(step);
            let (votes, steps) = beats.entry(beat).or_default();
            *steps |= 1 << (step - beat).min(63);
            let heard = (chord.chord, chord.bass);
            match votes.iter_mut().find(|(other, _)| *other == heard) {
                Some((_, count)) => *count += 1,
                None => votes.push((heard, 1)),
            }
        }
    }
    let mut named: Vec<(usize, usize, Option<HarmonyName>)> = Vec::new();
    for (&beat, (votes, steps)) in &beats {
        let end = meter.beat_end(beat);
        if (steps.count_ones() as usize) * 2 < end - beat {
            continue;
        }
        let Some(&((chord, bass), _)) = votes.iter().max_by_key(|(_, count)| *count) else {
            continue;
        };
        let harmony = chord.map(|chord| {
            let bass = bass.unwrap_or(chord.root);
            HarmonyName::Chord(crate::project::spelled(chord, bass, project.key_at(beat)))
        });
        named.push((beat, end, harmony));
    }
    let first = named.iter().position(|(_, _, harmony)| harmony.is_some())?;
    let last = named
        .iter()
        .rposition(|(_, _, harmony)| harmony.is_some())?;
    let named = &named[first..=last];
    let mut markers: Vec<ChordMarker> = Vec::new();
    for &(beat, _, harmony) in named {
        let harmony = harmony.unwrap_or(HarmonyName::NoChord);
        if markers
            .last()
            .is_none_or(|marker| marker.harmony != harmony)
        {
            markers.push(ChordMarker::new(beat, harmony));
        }
    }
    Some(SheetChords {
        markers,
        steps: named[0].0..named[named.len() - 1].1,
    })
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
    /// The track of the take's recording when it is already in the song,
    /// for a take transcribed again: the notes with no home go on the
    /// track right under it, as when the take was first written.
    pub under: Option<usize>,
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
    /// How many drum hits were written.
    pub drum_hits: usize,
    /// How many bass notes were written.
    pub bass_notes: usize,
    /// How many chords were written on the chord sheet.
    pub chords: usize,
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
    /// clips only ever hold the recording. Its drum hits go on the drum
    /// channels the same way, in place of the kick, snare and hi-hats
    /// there were, and in song mode its chords on the chord sheet.
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
        let drums = self.drum_lines(transcription);
        let drum_range = transcription.drum_range();
        let bass = self.bass_line(transcription);
        let bass_range = transcription.bass_range();

        match along {
            Along::Pattern { pattern, .. } => {
                let pattern = *pattern;
                if let (Some(channel), Some(range)) = (channel, transcription.range()) {
                    added.notes =
                        self.replace_in_pattern(pattern, channel, range, 0, &transcription.notes);
                }
                if let Some(range) = drum_range {
                    for (channel, hits) in &drums {
                        added.drum_hits +=
                            self.replace_in_pattern(pattern, *channel, range.clone(), 0, hits);
                    }
                }
                if let (Some((channel, notes)), Some(range)) = (&bass, bass_range) {
                    added.bass_notes += self.replace_in_pattern(pattern, *channel, range, 0, notes);
                }
                for (piece, &take) in transcription.pieces.iter().zip(&recordings) {
                    if let Some(pattern) = self.pattern_mut(pattern) {
                        pattern.set_notes(take, [NoteEvent::new(STEP_NOTE, piece.step, 1)]);
                    }
                }
            }
            Along::Song(_) => {
                let mut lines = Vec::new();
                if let (Some(channel), Some(range)) = (channel, transcription.range()) {
                    lines.push((channel, transcription.notes.clone(), range));
                }
                let melody = lines.len();
                if let Some(range) = drum_range {
                    for (channel, hits) in drums {
                        lines.push((channel, hits, range.clone()));
                    }
                }
                let drum_end = lines.len();
                if let (Some((channel, notes)), Some(range)) = (bass, bass_range) {
                    lines.push((channel, notes, range));
                }
                let written = self.write_take_in_song(
                    transcription,
                    &lines,
                    &recordings,
                    keep,
                    &mut added.shared,
                );
                added.notes = written[..melody].iter().sum();
                added.drum_hits = written[melody..drum_end].iter().sum();
                added.bass_notes = written[drum_end..].iter().sum();
                if let Some(chords) = &transcription.chords {
                    added.chords = self.write_take_chords(chords);
                }
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

    /// [`Project::add_take`] in song mode: each of `lines` (a channel,
    /// its notes, and the steps where they take the place of that
    /// channel's) goes into the patterns that hold the channel's notes
    /// there, or into clips of their own under the recording's. Returns
    /// how many notes of each line were written.
    fn write_take_in_song(
        &mut self,
        transcription: &Transcription,
        lines: &[(usize, Vec<Placed>, Range<usize>)],
        recordings: &[usize],
        keep: &Keep,
        shared: &mut Vec<(usize, usize)>,
    ) -> Vec<usize> {
        let mut written = vec![0; lines.len()];
        let mut homeless: Vec<Vec<Placed>> = Vec::with_capacity(lines.len());
        for (index, (channel, notes, range)) in lines.iter().enumerate() {
            let (count, left) = self.write_in_homes(*channel, notes, range.clone(), shared);
            written[index] = count;
            homeless.push(left);
        }

        // The recording's clips, on the take's track; the notes with no
        // home on a track of their own, right under it.
        let notes_span = homeless
            .iter()
            .flatten()
            .map(|note| note.start..note.end())
            .reduce(|a, b| a.start.min(b.start)..a.end.max(b.end));
        let recorded = transcription.recorded().filter(|_| !recordings.is_empty());
        let mut recording_track = keep.under;
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
            recording_track = Some(track);
        }
        let notes_track = match (recording_track, &notes_span) {
            (Some(track), Some(span)) => {
                Some(self.notes_track_under(track, &keep.notes, span.clone()))
            }
            _ => None,
        };
        if let Some(span) = notes_span {
            let track = notes_track.unwrap_or_else(|| self.take_track(keep.track, &keep.notes));
            for (pattern, start, end) in self.bar_clips(span, track, &keep.notes) {
                for (index, (channel, _, _)) in lines.iter().enumerate() {
                    let notes: Vec<Placed> = homeless[index]
                        .iter()
                        .copied()
                        .filter(|note| (start..end).contains(&note.start))
                        .collect();
                    if !notes.is_empty() {
                        written[index] +=
                            self.replace_in_pattern(pattern, *channel, start..end, start, &notes);
                    }
                }
            }
        }
        written
    }

    /// `notes` written on `channel` into the patterns that already hold
    /// that channel's notes in `range`, in place of those: a pattern
    /// corrected once is corrected wherever it plays (the patterns that
    /// play more than once are added to `shared`). Returns how many were
    /// written, and the notes no pattern holds.
    fn write_in_homes(
        &mut self,
        channel: usize,
        notes: &[Placed],
        range: Range<usize>,
        shared: &mut Vec<(usize, usize)>,
    ) -> (usize, Vec<Placed>) {
        // The clips that hold the channel's notes where the take was sung.
        let homes: Vec<Clip> = self
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
            .collect();

        // The notes that find a home, by clip.
        let mut written = 0;
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
            // Every place this clip plays the pattern inside the range,
            // and where the pattern's first step is that time round. A
            // clip trimmed at an edge leaves the steps it hides alone.
            let rounds: Vec<(usize, Range<usize>)> = clip
                .rounds(length)
                .filter_map(|round| {
                    let offset = round.at.checked_sub(round.steps.start)?;
                    Some((offset, round.at..round.end()))
                })
                .collect();
            for (offset, heard) in rounds {
                let here = range.start.max(heard.start)..range.end.min(heard.end);
                if here.start < here.end {
                    let notes: Vec<Placed> = notes
                        .iter()
                        .copied()
                        .filter(|note| {
                            here.contains(&note.start)
                                && first_home(&homes, note.start, self) == Some(*clip)
                        })
                        .collect();
                    written += self.replace_in_pattern(clip.pattern, channel, here, offset, &notes);
                }
            }
            let uses = self
                .playlist()
                .clips()
                .iter()
                .filter(|other| other.pattern == clip.pattern)
                .map(|other| other.repeats)
                .sum::<usize>();
            if uses > 1 && !shared.iter().any(|(pattern, _)| *pattern == clip.pattern) {
                shared.push((clip.pattern, uses));
            }
        }
        let homeless = notes
            .iter()
            .copied()
            .filter(|note| first_home(&homes, note.start, self).is_none())
            .collect();
        (written, homeless)
    }

    /// The drum hits of `transcription`, as notes on the channel of each
    /// drum a take is heard for: one for each drum hit, made if the song
    /// has none, and one for each drum not hit whose channel the song
    /// has, so its notes there go.
    fn drum_lines(&mut self, transcription: &Transcription) -> Vec<(usize, Vec<Placed>)> {
        let mut lines = Vec::new();
        for drum in HEARD_DRUMS {
            let notes: Vec<Placed> = transcription
                .drums
                .iter()
                .filter(|hit| hit.drum == drum)
                .map(|hit| Placed {
                    start: hit.step,
                    length: 1,
                    pitch: STEP_NOTE,
                    velocity: hit.velocity,
                })
                .collect();
            let existing = self.channels().iter().position(|channel| {
                channel.kind == ChannelKind::Sampler(SampleSource::Builtin(drum))
            });
            let channel = match existing {
                Some(channel) => Some(channel),
                None if !notes.is_empty() => crate::library::apply::drum_channel(self, drum),
                None => None,
            };
            if let Some(channel) = channel {
                lines.push((channel, notes));
            }
        }
        lines
    }

    /// The bass line as notes on the song's bass channel, or a new
    /// acoustic-bass channel when the song has none.
    fn bass_line(&mut self, transcription: &Transcription) -> Option<(usize, Vec<Placed>)> {
        if transcription.bass.is_empty() {
            return None;
        }
        let channel = self
            .channels()
            .iter()
            .position(crate::library::apply::plays_the_bass)
            .or_else(|| {
                self.add_channel(Channel {
                    name: "Bass".to_string(),
                    program: Some(crate::general_midi::ACOUSTIC_BASS),
                    ..Channel::synth()
                })
            })?;
        Some((channel, transcription.bass.clone()))
    }

    /// Writes `chords` on the chord sheet, in place of the chords there
    /// were on their steps; the chord that played on after them still
    /// does. Returns how many markers were written.
    fn write_take_chords(&mut self, chords: &SheetChords) -> usize {
        let Range { start, end } = chords.steps;
        let mut markers = chords.markers.clone();
        let mut until = end;
        if let Some(after) = self.chord_at(end)
            && after.start < end
            && end < self.song_length()
        {
            markers.push(ChordMarker::new(end, after.harmony));
            until = end + 1;
        }
        if self.set_chords(start, until, &markers) {
            chords.markers.len()
        } else {
            0
        }
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
    /// The track right under `track` for the notes of a take sung over
    /// `span`, named `name`: the one already there if it has that name
    /// and room there, else a new one (`track` itself if no track can be
    /// added).
    fn notes_track_under(&mut self, track: usize, name: &str, span: Range<usize>) -> usize {
        let below = track + 1;
        let free = !self.playlist().clips().iter().any(|clip| {
            clip.track == below
                && clip.start < span.end
                && clip.start + self.clip_length(clip) > span.start
        });
        if free
            && self
                .playlist()
                .tracks()
                .get(below)
                .is_some_and(|t| t.name == name)
        {
            return below;
        }
        match self.insert_track(below) {
            Some(below) => self.take_track(below, name),
            None => track,
        }
    }

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
            under: None,
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
    fn bass_notes_land_on_the_grid_and_keep_chromatic_pitches() {
        let project = song();
        let take = take(Along::Song(project.playback_arrangement()), 16.0, 2.0);
        let placed = transcribe_bass(
            &take,
            &[
                BassNote {
                    start: 0.0,
                    end: 0.45,
                    pitch: 28.0,
                    loudness: -12.0,
                },
                BassNote {
                    start: 0.5,
                    end: 0.95,
                    pitch: 29.0,
                    loudness: -12.0,
                },
            ],
            1,
        );
        assert_eq!(
            placed
                .iter()
                .map(|note| (note.start, note.length, note.pitch.midi(), note.velocity))
                .collect::<Vec<_>>(),
            [(16, 4, 28, 100), (20, 4, 29, 100)]
        );
    }

    #[test]
    fn a_take_writes_its_bass_notes_into_the_bass_channel() {
        let mut project = song();
        let bass = project
            .add_channel(Channel {
                name: "Bass".into(),
                program: Some(crate::general_midi::ACOUSTIC_BASS),
                ..Channel::synth()
            })
            .unwrap();
        project
            .pattern_mut(0)
            .unwrap()
            .set_notes(bass, [NoteEvent::new(Note::new(36), 0, 4)]);
        let along = Along::Song(project.playback_arrangement());
        let transcription = Transcription {
            bass: [Placed {
                start: 16,
                length: 4,
                pitch: Note::new(33),
                velocity: 100,
            }]
            .into(),
            ..Transcription::default()
        };
        let keep = Keep {
            recordings: Vec::new(),
            ..keep(&project, 0)
        };
        let added = project.add_take(&along, &transcription, &Target::Nowhere, &keep);
        assert_eq!(added.bass_notes, 1);
        assert_eq!(
            project.patterns()[0]
                .notes(bass)
                .iter()
                .find(|note| note.start == 16)
                .map(|note| (note.pitch.midi(), note.velocity)),
            Some((33, 100))
        );
        assert!(project.channels()[bass].name == "Bass");
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
            under: None,
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
    fn a_take_transcribed_again_replaces_its_notes_and_keeps_one_recording() {
        let mut project = song();
        let along = Along::Song(project.playback_arrangement());
        let take = take(along.clone(), 16.0, 2.0);
        let first = transcribe(&project, &take, &[sung(0.0, 0.45, 67.0, -12.0)], 1, 0.0);
        let target = Target::New {
            name: "Sung".into(),
            program: 53,
        };
        let added = project.add_take(&along, &first, &target, &keep(&project, 1));
        let sung_on = added.channel.unwrap();
        let channels = project.channels().len();
        let tracks = project.playlist().tracks().len();
        let clips = project.playlist().clips().len();

        // Heard again, a different note, written to the same channel with
        // no recording: it takes the old note's place, in the same clip.
        let again = transcribe(&project, &take, &[sung(0.0, 0.45, 69.0, -12.0)], 1, 0.0);
        let keep = Keep {
            recordings: Vec::new(),
            under: Some(1),
            ..keep(&project, 0)
        };
        let added = project.add_take(&along, &again, &Target::Channel(sung_on), &keep);
        assert_eq!(added.notes, 1);
        assert_eq!(project.channels().len(), channels, "no second recording");
        assert_eq!(project.playlist().tracks().len(), tracks);
        assert_eq!(project.playlist().clips().len(), clips);
        let notes: Vec<_> = project
            .playlist()
            .clips()
            .iter()
            .filter(|clip| clip.track == 2)
            .flat_map(|clip| project.patterns()[clip.pattern].notes(sung_on).to_vec())
            .collect();
        assert_eq!(notes, [NoteEvent::new(Note::new(69), 0, 4)]);

        // To a new channel this time: its notes need a track of their own,
        // still right under the recording.
        let added = project.add_take(&along, &again, &target, &keep);
        let newer = added.channel.unwrap();
        let tracks = project.playlist().tracks();
        assert_eq!(tracks[1].name, "Take 1");
        assert_eq!(tracks[2].name, "Take 1 notes");
        let below: Vec<_> = project
            .playlist()
            .clips()
            .iter()
            .filter(|clip| clip.track == 2)
            .map(|clip| &project.patterns()[clip.pattern])
            .collect();
        assert!(
            below.iter().any(|p| !p.notes(newer).is_empty()),
            "the new channel's notes are right under the recording"
        );
        assert!(below.iter().all(|p| p.notes(sung_on).is_empty()));
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
        // D7 over bar 2: its F♯ takes the place of the key's F, and only
        // of F: G, a semitone above it too, stays.
        let d7 = ChordMarker::new(16, HarmonyName::parse("D7").unwrap());
        assert!(project.set_chords(16, 32, &[d7]));
        let along = Along::Song(project.playback_arrangement());
        let take = take(along, 16.0, 2.0);
        // A little sharp of C♯, of F and of G.
        let notes = [
            sung(0.0, 0.45, 61.1, -12.0),
            sung(0.5, 0.95, 65.3, -12.0),
            sung(1.0, 1.45, 67.3, -12.0),
        ];
        let written = |pitches| -> Vec<u8> {
            transcribe_with(&project, &take, &notes, 1, 0.0, pitches)
                .notes
                .iter()
                .map(|note| note.pitch.midi())
                .collect()
        };
        assert_eq!(written(Pitches::AsSung), [61, 65, 67], "as sung");
        assert_eq!(
            written(Pitches::InKey),
            [62, 65, 67],
            "C♯ is not in C major"
        );
        assert_eq!(
            written(Pitches::WithChords),
            [62, 66, 67],
            "F♯ over D7, G stays"
        );
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
