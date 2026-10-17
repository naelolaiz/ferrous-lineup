//! Playing a project without a sound card, to save it as a WAV file.
//!
//! The audio thread does not care who asks it for samples. With a sound
//! card, cpal asks a few hundred at a time, as fast as they are played.
//! Here a loop asks for them as fast as the computer can make them, and
//! writes them to a file instead. It is the same code making the same
//! sound, so an export sounds exactly like playback.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize};

use crate::engine::{AudioThread, Command, Garbage, NO_POSITION, NO_STEP, TRASH_QUEUE_SIZE};

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
                position,
            ),
            sample_rate,
            channels,
            _commands: commands,
            trash,
        }
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn channels(&self) -> usize {
        self.channels
    }

    /// Applies a command at once: there is no other thread to hand it to.
    /// Volume changes take effect at once too, without smoothing: the
    /// commands come before the first sample is made.
    pub fn send(&mut self, command: Command) {
        self.audio.apply(command);
        self.audio.settle_volumes();
        self.empty_trash();
    }

    /// Fills `output` with the next frames, one sample per channel each.
    pub fn render(&mut self, output: &mut [f32]) {
        self.audio.process(output);
        self.empty_trash();
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
