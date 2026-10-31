//! The microphone: recording a take while the song plays.
//!
//! A take is a voice, sung or whistled along with the song, to be turned
//! into notes (see `lineup_core::take`). For that, every moment of the
//! recording must be placed in the song: this sample was sung while the
//! song was heard at bar 5, beat 2.
//!
//! Two clocks are involved, and both have a delay:
//!
//! - the sound the audio thread makes now comes out of the speakers a
//!   little later (the output's *latency*), so the singer hears bar 5
//!   after the engine played it;
//! - the sound the microphone hears now reaches the program a little
//!   later (the input's latency).
//!
//! The sound card says both delays, with every block it asks for or
//! hands over. So the audio thread writes down, for each block, *when it
//! will be heard* and where the song is then (a [`Heard`], through the
//! [`SongClock`]), and the microphone's callback writes down, for each
//! block, *when it was heard*. Both are moments on the same clock,
//! [`Instant`], so the place in the song of each block of the recording
//! follows: it is a [`Mark`].
//!
//! The microphone's callback runs on a thread of its own, which must not
//! wait any more than the audio thread may: it only copies the samples
//! into a ring buffer. A [`Recorder`] thread empties it every few
//! milliseconds, keeps the samples, and writes the marks.

use std::collections::VecDeque;
use std::fmt;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SampleFormat, SizedSample, StreamConfig};
use lineup_core::i18n::{tr, trf};
use lineup_core::take::Mark;

use crate::engine::{Heard, SongClock};

/// How many seconds of sound can wait for the recorder: it empties the
/// queue every few milliseconds, so only a stalled machine fills it.
const QUEUE_SECONDS: u32 = 4;

/// How many blocks can wait.
const BLOCK_QUEUE: usize = 4096;

/// The longest take: ten minutes, as long as an audio file added to the
/// song (a take is held in memory while it is recorded).
pub const MAX_TAKE_SECONDS: u32 = 10 * 60;

/// How often the recorder empties the queues.
const POLL: Duration = Duration::from_millis(5);

/// How often the live display is fed the latest sound.
const LIVE_EVERY: Duration = Duration::from_millis(40);

/// How many of the latest samples the live display gets: about 60 ms.
const LIVE_SECONDS: f32 = 0.06;

/// The longest gap between what is heard of the song and a block of the
/// recording for the block to count as sung along with it: longer, and
/// the song had stopped.
const LONGEST_GAP: Duration = Duration::from_millis(100);

/// What can go wrong opening the microphone.
#[derive(Debug)]
pub enum InputError {
    /// The system has no microphone, or none enabled.
    NoInputDevice,
    Config(cpal::Error),
    UnsupportedFormat(SampleFormat),
    BuildStream(cpal::Error),
    Play(cpal::Error),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::NoInputDevice => f.write_str(tr("no microphone found")),
            InputError::Config(err) => {
                f.write_str(&trf("could not read the microphone's settings: {}", &[err]))
            }
            InputError::UnsupportedFormat(format) => {
                f.write_str(&trf("unsupported sample format: {}", &[format]))
            }
            InputError::BuildStream(err) | InputError::Play(err) => {
                f.write_str(&trf("could not open the microphone: {}", &[err]))
            }
        }
    }
}

impl std::error::Error for InputError {}

/// One block the microphone handed over: how many samples, and when the
/// first of them was heard. `lost` when the queue had no room for it.
#[derive(Debug, Clone, Copy)]
struct Block {
    frames: usize,
    at: Instant,
    lost: bool,
}

/// The open microphone.
pub struct Microphone {
    // Kept for as long as the microphone is open: dropping it closes it.
    _stream: cpal::Stream,
    samples: rtrb::Consumer<f32>,
    blocks: rtrb::Consumer<Block>,
    sample_rate: u32,
    name: String,
    broken: Arc<AtomicBool>,
}

impl Microphone {
    /// Opens the system's default input device.
    pub fn open() -> Result<Self, InputError> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or(InputError::NoInputDevice)?;
        let supported = device.default_input_config().map_err(InputError::Config)?;
        let format = supported.sample_format();
        let config: StreamConfig = supported.into();
        let sample_rate = config.sample_rate;
        let (sample_in, samples) = rtrb::RingBuffer::new((sample_rate * QUEUE_SECONDS) as usize);
        let (block_in, blocks) = rtrb::RingBuffer::new(BLOCK_QUEUE);
        let broken = Arc::new(AtomicBool::new(false));
        let ends = Ends {
            samples: sample_in,
            blocks: block_in,
            channels: usize::from(config.channels.max(1)),
        };
        let stream = match format {
            SampleFormat::F32 => build::<f32>(&device, config, ends, Arc::clone(&broken)),
            SampleFormat::F64 => build::<f64>(&device, config, ends, Arc::clone(&broken)),
            SampleFormat::I16 => build::<i16>(&device, config, ends, Arc::clone(&broken)),
            SampleFormat::I32 => build::<i32>(&device, config, ends, Arc::clone(&broken)),
            SampleFormat::U16 => build::<u16>(&device, config, ends, Arc::clone(&broken)),
            SampleFormat::U8 => build::<u8>(&device, config, ends, Arc::clone(&broken)),
            other => return Err(InputError::UnsupportedFormat(other)),
        }?;
        stream.play().map_err(InputError::Play)?;
        Ok(Microphone {
            _stream: stream,
            samples,
            blocks,
            sample_rate,
            name: device.to_string(),
            broken,
        })
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// The device's name, as the system gives it.
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// The microphone callback's ends of the queues.
struct Ends {
    samples: rtrb::Producer<f32>,
    blocks: rtrb::Producer<Block>,
    channels: usize,
}

/// Opens the input stream for one sample type `T`.
fn build<T>(
    device: &cpal::Device,
    config: StreamConfig,
    mut ends: Ends,
    broken: Arc<AtomicBool>,
) -> Result<cpal::Stream, InputError>
where
    T: SizedSample,
    f32: FromSample<T>,
{
    let data_callback = move |data: &[T], info: &cpal::InputCallbackInfo| {
        // When the first sample of the block was heard: now, less the
        // time the sound card says it took to get here.
        let stamp = info.timestamp();
        let delay = stamp.callback.duration_since(stamp.capture);
        let at = Instant::now()
            .checked_sub(delay)
            .unwrap_or_else(Instant::now);
        let frames = data.len() / ends.channels;
        let lost = ends.samples.slots() < frames;
        if !lost {
            // The channels mixed into one.
            for frame in data.chunks(ends.channels) {
                let sum: f32 = frame
                    .iter()
                    .map(|&s| <f32 as FromSample<T>>::from_sample_(s))
                    .sum();
                let _ = ends.samples.push(sum / ends.channels as f32);
            }
        }
        let _ = ends.blocks.push(Block { frames, at, lost });
    };
    let error_callback = move |err: cpal::Error| {
        broken.store(true, Ordering::Relaxed);
        eprintln!("microphone error: {err}");
    };
    device
        .build_input_stream(config, data_callback, error_callback, None)
        .map_err(InputError::BuildStream)
}

/// A take as recorded: its samples, and where the song was heard while
/// it was sung.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Recording {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub marks: Vec<Mark>,
    /// How many samples were lost (silence in their place), when the
    /// machine was too busy to keep up.
    pub lost: usize,
}

/// What the recorder shows while it records.
#[derive(Debug, Default)]
struct Live {
    /// The loudest sample of the last moment, as the bits of an `f32`.
    level: AtomicU32,
    /// How many samples are recorded.
    frames: AtomicUsize,
}

/// Records a take on a thread of its own.
pub struct Recorder {
    worker: Option<JoinHandle<(Recording, SongClock)>>,
    stop: Arc<AtomicBool>,
    live: Arc<Live>,
    sample_rate: u32,
    name: String,
}

/// Something that wants the latest sound while recording, as a display of
/// the pitch sung: called every few hundredths of a second with the last
/// moment of the take and its sample rate.
pub type LiveView = Box<dyn FnMut(&[f32], u32) + Send>;

impl Recorder {
    /// Starts recording from `microphone`, placing the take in the song
    /// with `clock`. `nudge` moves every moment of the take later in the
    /// song (earlier if negative), in seconds, for a sound card whose
    /// delays are not what it says.
    pub fn start(
        microphone: Microphone,
        mut clock: SongClock,
        nudge: f64,
        mut view: LiveView,
    ) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let live = Arc::new(Live::default());
        let sample_rate = microphone.sample_rate;
        let name = microphone.name.clone();
        let worker = {
            let stop = Arc::clone(&stop);
            let live = Arc::clone(&live);
            thread::spawn(move || {
                clock.listen(true);
                let mut microphone = microphone;
                let mut recording = Recording {
                    sample_rate,
                    ..Recording::default()
                };
                let mut heard: VecDeque<Heard> = VecDeque::new();
                let mut last_view = Instant::now();
                let most = (sample_rate * MAX_TAKE_SECONDS) as usize;
                while !stop.load(Ordering::Relaxed)
                    && !microphone.broken.load(Ordering::Relaxed)
                    && recording.samples.len() < most
                {
                    // What was heard of the song first, so each block of
                    // the take finds the moments heard before it.
                    while let Some(moment) = clock.pop() {
                        heard.push_back(moment);
                    }
                    while let Ok(block) = microphone.blocks.pop() {
                        take_block(&mut microphone, &mut recording, &heard, block, nudge);
                    }
                    // Only the last two seconds are needed.
                    while heard
                        .front()
                        .is_some_and(|moment| moment.at.elapsed() > Duration::from_secs(2))
                    {
                        heard.pop_front();
                    }
                    let recent = &recording.samples[recording
                        .samples
                        .len()
                        .saturating_sub(sample_rate as usize / 20)..];
                    let level = recent.iter().fold(0.0_f32, |peak, s| peak.max(s.abs()));
                    live.level.store(level.to_bits(), Ordering::Relaxed);
                    live.frames
                        .store(recording.samples.len(), Ordering::Relaxed);
                    if last_view.elapsed() >= LIVE_EVERY {
                        last_view = Instant::now();
                        let keep = (sample_rate as f32 * LIVE_SECONDS) as usize;
                        let from = recording.samples.len().saturating_sub(keep);
                        view(&recording.samples[from..], sample_rate);
                    }
                    thread::sleep(POLL);
                }
                clock.listen(false);
                // What came in since the last look.
                while let Ok(block) = microphone.blocks.pop() {
                    take_block(&mut microphone, &mut recording, &heard, block, nudge);
                }
                (recording, clock)
            })
        };
        Recorder {
            worker: Some(worker),
            stop,
            live,
            sample_rate,
            name,
        }
    }

    /// The loudest sample of the last moment, from 0 to 1.
    pub fn level(&self) -> f32 {
        f32::from_bits(self.live.level.load(Ordering::Relaxed))
    }

    /// How long the take is so far, in seconds.
    pub fn seconds(&self) -> f64 {
        self.live.frames.load(Ordering::Relaxed) as f64 / f64::from(self.sample_rate.max(1))
    }

    /// The microphone's name.
    pub fn device(&self) -> &str {
        &self.name
    }

    /// Whether the recorder has stopped by itself: the take reached
    /// [`MAX_TAKE_SECONDS`], or the microphone went away.
    pub fn has_stopped(&self) -> bool {
        self.worker.as_ref().is_none_or(JoinHandle::is_finished)
    }

    /// Stops recording, and gives back the take and the song clock.
    /// `None` if the recorder's thread failed.
    pub fn finish(mut self) -> Option<(Recording, SongClock)> {
        self.stop.store(true, Ordering::Relaxed);
        self.worker.take()?.join().ok()
    }
}

impl Drop for Recorder {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

/// Moves one block from the microphone's queue into the take, with a mark
/// placing it in the song if the song was heard then.
fn take_block(
    microphone: &mut Microphone,
    recording: &mut Recording,
    heard: &VecDeque<Heard>,
    block: Block,
    nudge: f64,
) {
    let frame = recording.samples.len();
    if block.lost {
        recording
            .samples
            .extend(std::iter::repeat_n(0.0, block.frames));
        recording.lost += block.frames;
    } else {
        for _ in 0..block.frames {
            recording
                .samples
                .push(microphone.samples.pop().unwrap_or(0.0));
        }
    }
    if let Some(mark) = mark_at(heard, block.at, frame, nudge) {
        recording.marks.push(mark);
    }
}

/// The mark for sample `frame` of a take, heard by the microphone at
/// `at`: where the song was heard then, from the last moment heard at or
/// before it. `None` if the song was not heard then.
fn mark_at(heard: &VecDeque<Heard>, at: Instant, frame: usize, nudge: f64) -> Option<Mark> {
    // A positive nudge places the take later in the song: as if the
    // microphone had heard it that much later.
    let at = if nudge >= 0.0 {
        at + Duration::from_secs_f64(nudge)
    } else {
        at.checked_sub(Duration::from_secs_f64(-nudge))?
    };
    let moment = heard.iter().rev().find(|moment| moment.at <= at)?;
    let since = at.duration_since(moment.at);
    if since > LONGEST_GAP {
        return None;
    }
    Some(Mark {
        frame,
        position: moment.position + since.as_secs_f64() * moment.steps_per_second,
        steps_per_second: moment.steps_per_second,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_block_is_placed_where_the_song_was_heard_then() {
        let start = Instant::now();
        let heard: VecDeque<Heard> = [0, 10, 20]
            .iter()
            .map(|&ms| Heard {
                at: start + Duration::from_millis(ms),
                position: 4.0 + ms as f64 / 1000.0 * 8.0,
                steps_per_second: 8.0,
            })
            .collect();
        // Heard by the microphone 15 ms in: 5 ms after the second moment.
        let mark = mark_at(&heard, start + Duration::from_millis(15), 480, 0.0).unwrap();
        assert_eq!(mark.frame, 480);
        assert!((mark.position - 4.12).abs() < 1e-9, "{mark:?}");
        // Before the song was heard: no mark.
        assert_eq!(
            mark_at(&heard, start - Duration::from_millis(1), 0, 0.0),
            None
        );
        // Long after the last moment: the song had stopped.
        assert_eq!(
            mark_at(&heard, start + Duration::from_millis(500), 0, 0.0),
            None
        );
    }

    #[test]
    fn a_nudge_moves_the_take_later_in_the_song() {
        let start = Instant::now();
        let heard: VecDeque<Heard> = (0..10)
            .map(|i| Heard {
                at: start + Duration::from_millis(i * 10),
                position: i as f64 * 0.08,
                steps_per_second: 8.0,
            })
            .collect();
        let at = start + Duration::from_millis(50);
        let plain = mark_at(&heard, at, 0, 0.0).unwrap().position;
        let nudged = mark_at(&heard, at, 0, 0.02).unwrap().position;
        assert!((nudged - plain - 0.16).abs() < 1e-6, "{plain} {nudged}");
    }
}
