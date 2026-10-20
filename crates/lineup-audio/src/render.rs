//! Playing a project without a sound card, to save it as a WAV file.
//!
//! The audio thread does not care who asks it for samples. With a sound
//! card, cpal asks a few hundred at a time, as fast as they are played.
//! Here a loop asks for them as fast as the computer can make them, and
//! writes them to a file instead. It is the same code making the same
//! sound, so an export sounds exactly like playback.
//!
//! A renderer can write a trace of what it plays, too ([`Renderer::trace`]):
//! the notes of an export, to read instead of listening to them.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize};

use crate::engine::{
    AudioThread, Command, Garbage, NO_POSITION, NO_STEP, TRASH_QUEUE_SIZE, load_position,
};
use crate::trace::{Trace, TraceReader, trace_queue};

/// How many frames are made and written at a time.
const BLOCK_FRAMES: usize = 4_096;

/// An audio thread that runs on whichever thread calls it.
pub struct Renderer {
    audio: AudioThread,
    sample_rate: u32,
    channels: usize,
    /// The audio thread is built with a command queue, but the renderer
    /// hands it commands directly. Keeping this end of the queue alive
    /// keeps the queue valid.
    _commands: rtrb::Producer<Command>,
    trash: rtrb::Consumer<Garbage>,
    /// Where playback is, which the audio thread stores after every block.
    position: Arc<AtomicU64>,
    /// The trace being written, and its reader, once [`Renderer::trace`]
    /// has started one.
    trace: Option<(TraceReader, Trace)>,
}

impl Renderer {
    /// A renderer making `channels` channels of sound (2 for stereo) at
    /// `sample_rate` frames per second.
    pub fn new(sample_rate: u32, channels: usize) -> Self {
        let (commands, command_consumer) = rtrb::RingBuffer::new(1);
        let (trash_producer, trash) = rtrb::RingBuffer::new(TRASH_QUEUE_SIZE);
        let playhead = Arc::new(AtomicUsize::new(NO_STEP));
        let position = Arc::new(AtomicU64::new(NO_POSITION));
        let channels = channels.max(1);
        Renderer {
            audio: AudioThread::new(
                sample_rate,
                channels,
                command_consumer,
                trash_producer,
                playhead,
                Arc::clone(&position),
            ),
            sample_rate,
            channels,
            _commands: commands,
            trash,
            position,
            trace: None,
        }
    }

    /// Starts writing down every note played from now on (see
    /// [`crate::trace`]). Send it before [`Command::Play`], so the trace
    /// starts with the playback.
    pub fn trace(&mut self) {
        let (tracer, reader) = trace_queue();
        self.audio.apply(Command::SetTrace(Some(Box::new(tracer))));
        self.empty_trash();
        self.trace = Some((reader, Trace::new(self.sample_rate)));
    }

    /// What the trace has written so far, which it then starts again
    /// from empty; or `None` if no trace was started.
    pub fn take_trace(&mut self) -> Option<Trace> {
        self.read_trace();
        let (_, trace) = self.trace.as_mut()?;
        let sample_rate = trace.sample_rate;
        Some(std::mem::replace(trace, Trace::new(sample_rate)))
    }

    /// Moves what the audio thread traced into the trace. The queue has
    /// room for far more than a block makes, so it is read after each.
    fn read_trace(&mut self) {
        if let Some((reader, trace)) = &mut self.trace {
            reader.read_into(trace);
        }
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn channels(&self) -> usize {
        self.channels
    }

    /// Where playback is after the frames made so far, in steps (4.5 is
    /// halfway through step 4), or `None` before the first frame and
    /// while stopped. A video asks after each picture's sound, so the
    /// next picture shows where the music is.
    pub fn position(&self) -> Option<f64> {
        load_position(&self.position)
    }

    /// Applies a command at once: there is no other thread to hand it to.
    /// Volume changes take effect at once too, without smoothing: the
    /// commands come before the first sample is made.
    pub fn send(&mut self, command: Command) {
        self.audio.apply(command);
        self.audio.settle_volumes();
        self.empty_trash();
        self.read_trace();
    }

    /// Fills `output` with the next frames, one sample per channel each.
    pub fn render(&mut self, output: &mut [f32]) {
        self.audio.process(output);
        self.empty_trash();
        self.read_trace();
    }

    /// Plays `frames` frames and throws the sound away: for a trace,
    /// which only needs what was played.
    pub fn skip(&mut self, frames: usize) {
        let mut block = vec![0.0_f32; BLOCK_FRAMES * self.channels];
        let mut done = 0;
        while done < frames {
            let count = (frames - done).min(BLOCK_FRAMES);
            self.render(&mut block[..count * self.channels]);
            done += count;
        }
    }

    /// Frees what the audio thread threw away. There is no real-time
    /// thread here, so it can be done straight away.
    fn empty_trash(&mut self) {
        while self.trash.pop().is_ok() {}
    }
}

/// Turns a sample between -1.0 and 1.0 into a 16-bit integer sample.
fn to_i16(sample: f32) -> i16 {
    (sample.clamp(-1.0, 1.0) * f32::from(i16::MAX)).round() as i16
}

/// Plays `frames` frames from `renderer` into a 16-bit WAV file at `path`.
///
/// `progress` is called after each block with the number of frames done
/// so far. The sound goes to a temporary file first, which is renamed to
/// `path` at the end, so a failed export never leaves half a file in
/// place of an older one.
pub fn export_wav(
    renderer: &mut Renderer,
    path: &Path,
    frames: usize,
    mut progress: impl FnMut(usize),
) -> Result<(), hound::Error> {
    let spec = hound::WavSpec {
        channels: renderer.channels() as u16,
        sample_rate: renderer.sample_rate(),
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut temporary = PathBuf::from(path);
    temporary.as_mut_os_string().push(".tmp");

    let mut writer = hound::WavWriter::create(&temporary, spec)?;
    let mut block = vec![0.0_f32; BLOCK_FRAMES * renderer.channels()];
    let mut done = 0;
    while done < frames {
        let count = (frames - done).min(BLOCK_FRAMES);
        let samples = &mut block[..count * renderer.channels()];
        renderer.render(samples);
        for &sample in samples.iter() {
            writer.write_sample(to_i16(sample))?;
        }
        done += count;
        progress(done);
    }
    writer.finalize()?;
    fs::rename(&temporary, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ChannelNotes;
    use crate::instrument::{Instrument, Sampler};
    use crate::sample::SampleData;
    use lineup_core::{NoteEvent, Tempo};

    const RATE: u32 = 48_000;

    /// A renderer with one pattern and a click on its first step.
    fn clicking() -> Renderer {
        let mut renderer = Renderer::new(RATE, 2);
        renderer.send(Command::InsertPattern {
            index: 0,
            length: 16,
        });
        renderer.send(Command::SetTempo(Tempo::new(120.0)));
        renderer.send(Command::InsertChannel {
            index: 0,
            instrument: Instrument::Sampler(Sampler::new(Arc::new(SampleData::new(
                vec![0.5; 10],
                RATE,
            )))),
            // Full velocity, so the click is exactly 0.5.
            notes: ChannelNotes::new([vec![NoteEvent {
                velocity: 127,
                ..NoteEvent::step(0)
            }]]),
            volume: 1.0,
            muted: false,
        });
        renderer.send(Command::SetMasterVolume(1.0));
        renderer.send(Command::Play);
        renderer
    }

    #[test]
    fn renders_like_the_audio_thread() {
        let mut renderer = clicking();
        let mut out = vec![0.0; 2 * 100];
        renderer.render(&mut out);
        // tanh(0.5): the click, the same on both channels.
        assert!((out[0] - 0.5_f32.tanh()).abs() < 1e-6);
        assert_eq!(out[0], out[1]);
        assert_eq!(out[2 * 20], 0.0, "the click is 10 frames long");
    }

    #[test]
    fn a_panned_channel_plays_more_on_its_side() {
        // In the middle both speakers play it in full; moving it one way
        // turns the other speaker down.
        for (pan, gains) in [
            (-1.0, [1.0, 0.0]),
            (-0.5, [1.0, 0.5]),
            (0.0, [1.0, 1.0]),
            (0.5, [0.5, 1.0]),
            (1.0, [0.0, 1.0]),
        ] {
            let mut renderer = clicking();
            renderer.send(Command::SetPan { channel: 0, pan });
            let mut out = [0.0; 2];
            renderer.render(&mut out);
            assert_eq!(out, gains.map(|gain: f32| (0.5 * gain).tanh()), "{pan}");
        }
    }

    #[test]
    fn a_channel_sent_nowhere_plays_dry_in_a_room() {
        let room = lineup_core::ReverbSettings {
            enabled: true,
            mix: 0.5,
            ..Default::default()
        };
        // The first frame, before the room answers, and how much sound
        // there is after the click.
        let play = |send: f32| {
            let mut renderer = clicking();
            renderer.send(Command::SetReverb(room));
            renderer.send(Command::SetReverbSend { channel: 0, send });
            let mut out = vec![0.0; RATE as usize];
            renderer.render(&mut out);
            let after: f32 = out[2 * 20..].iter().map(|sample| sample.abs()).sum();
            (out[0], after)
        };
        let (first, after) = play(0.0);
        assert_eq!(first, 0.5_f32.tanh(), "as loud as with the reverb off");
        assert_eq!(after, 0.0, "and no room after the click");
        let (first, all) = play(1.0);
        assert!(
            (first - 0.25_f32.tanh()).abs() < 1e-6,
            "half went into the room"
        );
        assert!(all > 0.01);
        let (first, half) = play(0.5);
        assert!((first - 0.375_f32.tanh()).abs() < 1e-6);
        assert!((half / all - 0.5).abs() < 0.01, "{half} {all}");
    }

    #[test]
    fn the_meters_see_each_channel_and_the_master() {
        let mut renderer = clicking();
        renderer.send(Command::SetVolume {
            channel: 0,
            volume: 0.5,
        });
        renderer.send(Command::SetPan {
            channel: 0,
            pan: -1.0,
        });
        renderer.send(Command::SetMasterVolume(0.8));
        let levels = Arc::clone(renderer.audio.levels());
        assert_eq!(levels.take_channel(0), 0.0, "nothing played yet");
        let mut out = vec![0.0; 2 * 100];
        renderer.render(&mut out);
        // After the channel's volume, before its pan.
        assert_eq!(levels.take_channel(0), 0.25);
        // After the master volume, all on the left.
        let [left, right] = levels.take_master();
        assert!((left - 0.2).abs() < 1e-6 && right == 0.0, "{left} {right}");
        renderer.render(&mut out);
        assert_eq!(levels.take_channel(0), 0.0, "the click is over");
    }

    #[test]
    fn knows_where_playback_is() {
        let mut renderer = clicking();
        assert_eq!(renderer.position(), None, "nothing played yet");
        // At 120 BPM a step (a sixteenth note) lasts an eighth of a
        // second: 6000 frames.
        let mut out = vec![0.0; 2 * 9000];
        renderer.render(&mut out);
        let position = renderer.position().unwrap();
        assert!((position - 1.5).abs() < 1e-6, "{position}");
    }

    #[test]
    fn samples_become_16_bit_integers() {
        assert_eq!(to_i16(0.0), 0);
        assert_eq!(to_i16(1.0), i16::MAX);
        assert_eq!(to_i16(-1.0), -i16::MAX);
        assert_eq!(to_i16(3.0), i16::MAX, "too loud is clamped");
        assert_eq!(to_i16(0.5), 16_384);
    }

    #[test]
    fn exports_a_wav_file() {
        let path =
            std::env::temp_dir().join(format!("ferrous-lineup-export-{}.wav", std::process::id()));
        let mut renderer = clicking();
        let mut reported = Vec::new();
        let frames = BLOCK_FRAMES + 100;
        export_wav(&mut renderer, &path, frames, |done| reported.push(done)).unwrap();

        let mut reader = hound::WavReader::open(&path).unwrap();
        let spec = reader.spec();
        let samples: Vec<i16> = reader.samples::<i16>().map(Result::unwrap).collect();
        fs::remove_file(&path).ok();

        assert_eq!((spec.channels, spec.sample_rate), (2, RATE));
        assert_eq!(samples.len(), 2 * frames);
        assert_eq!(samples[0], to_i16(0.5_f32.tanh()));
        assert_eq!(reported, vec![BLOCK_FRAMES, frames]);
        let mut temporary = path.into_os_string();
        temporary.push(".tmp");
        assert!(
            !Path::new(&temporary).exists(),
            "the temporary file is gone"
        );
    }
}
