//! Opens the sound card and runs the audio thread.
//!
//! The audio thread is special: the sound card asks it for a new block of
//! samples every few milliseconds, and if it is late you hear a click or a
//! dropout. So the code that runs there must never wait: no locks, no file
//! or network access, no memory allocation, no printing.
//!
//! The UI therefore never touches the audio state directly. It sends
//! [`Command`] messages through a lock-free ring buffer (`rtrb`), and the
//! audio thread applies them at the start of each block.

use std::fmt;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SampleFormat, SizedSample, StreamConfig};

use crate::synth::TestTone;

/// How many commands can wait in the queue before `send` starts failing.
/// The audio thread empties the queue every few milliseconds, so this is
/// far more than the UI will ever send in that time.
const COMMAND_QUEUE_SIZE: usize = 1024;

/// A message from the UI thread to the audio thread.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command {
    /// Switch the test tone on (`true`) or off (`false`).
    SetToneOn(bool),
    /// Set the test tone pitch, in hertz.
    SetFrequency(f32),
    /// Set the test tone volume, from 0.0 to 1.0.
    SetVolume(f32),
}

/// What the engine found when it opened the sound card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineInfo {
    pub device_name: String,
    pub sample_rate: u32,
    pub channels: u16,
}

/// Everything that can go wrong while starting the engine.
#[derive(Debug)]
pub enum EngineError {
    /// The system has no default output device (no sound card, or none enabled).
    NoOutputDevice,
    /// The device would not tell us its preferred settings.
    Config(cpal::Error),
    /// The device uses a sample format this engine does not handle.
    UnsupportedFormat(SampleFormat),
    /// The audio stream could not be created.
    BuildStream(cpal::Error),
    /// The audio stream was created but would not start.
    Play(cpal::Error),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::NoOutputDevice => write!(f, "no audio output device found"),
            EngineError::Config(err) => write!(f, "could not read the device settings: {err}"),
            EngineError::UnsupportedFormat(format) => {
                write!(f, "unsupported sample format: {format}")
            }
            EngineError::BuildStream(err) => write!(f, "could not open the audio stream: {err}"),
            EngineError::Play(err) => write!(f, "could not start the audio stream: {err}"),
        }
    }
}

impl std::error::Error for EngineError {}

/// A running connection to the sound card.
///
/// Sound keeps playing for as long as this value is alive. When it is
/// dropped, the stream is closed and the audio thread stops.
pub struct AudioEngine {
    // The stream is never read, but it must be kept: dropping it stops audio.
    _stream: cpal::Stream,
    commands: rtrb::Producer<Command>,
    info: EngineInfo,
}

impl AudioEngine {
    /// Opens the default output device and starts the audio thread.
    pub fn start() -> Result<Self, EngineError> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or(EngineError::NoOutputDevice)?;
        let supported = device
            .default_output_config()
            .map_err(EngineError::Config)?;

        let sample_format = supported.sample_format();
        let config: StreamConfig = supported.into();
        let info = EngineInfo {
            device_name: device.to_string(),
            sample_rate: config.sample_rate,
            channels: config.channels,
        };

        let (producer, consumer) = rtrb::RingBuffer::new(COMMAND_QUEUE_SIZE);

        // The sound card decides the sample type. We pick the matching
        // version of the generic `build_stream` function for each one.
        let stream = match sample_format {
            SampleFormat::F32 => build_stream::<f32>(&device, config, consumer),
            SampleFormat::F64 => build_stream::<f64>(&device, config, consumer),
            SampleFormat::I16 => build_stream::<i16>(&device, config, consumer),
            SampleFormat::I32 => build_stream::<i32>(&device, config, consumer),
            SampleFormat::U16 => build_stream::<u16>(&device, config, consumer),
            SampleFormat::U8 => build_stream::<u8>(&device, config, consumer),
            other => return Err(EngineError::UnsupportedFormat(other)),
        }?;
        stream.play().map_err(EngineError::Play)?;

        Ok(AudioEngine {
            _stream: stream,
            commands: producer,
            info,
        })
    }

    /// Sends a command to the audio thread.
    ///
    /// Returns `false` if the queue was full and the command was dropped.
    pub fn send(&mut self, command: Command) -> bool {
        self.commands.push(command).is_ok()
    }

    /// Details about the open device.
    pub fn info(&self) -> &EngineInfo {
        &self.info
    }
}

/// Builds an output stream for one sample type `T`.
///
/// `T: SizedSample + FromSample<f32>` means: any sample type cpal supports
/// that can be converted from our `f32` samples.
fn build_stream<T>(
    device: &cpal::Device,
    config: StreamConfig,
    commands: rtrb::Consumer<Command>,
) -> Result<cpal::Stream, EngineError>
where
    T: SizedSample + FromSample<f32>,
{
    let mut audio_thread = AudioThread::new(config.sample_rate, config.channels as usize, commands);

    // This closure is the audio callback. `move` gives it ownership of
    // `audio_thread`, and cpal runs it on the audio thread.
    let data_callback = move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
        audio_thread.process(output);
    };

    // Called by cpal if something goes wrong while the stream runs, for
    // example when the device is unplugged. This one is allowed to print.
    let error_callback = |err: cpal::Error| eprintln!("audio stream error: {err}");

    device
        .build_output_stream(config, data_callback, error_callback, None)
        .map_err(EngineError::BuildStream)
}

/// Everything the audio callback owns: the instruments and the end of the
/// command queue that receives messages from the UI.
///
/// Keeping this in a struct (instead of loose variables inside the
/// closure) lets the tests below run it without a sound card.
struct AudioThread {
    tone: TestTone,
    channels: usize,
    commands: rtrb::Consumer<Command>,
}

impl AudioThread {
    fn new(sample_rate: u32, channels: usize, commands: rtrb::Consumer<Command>) -> Self {
        AudioThread {
            tone: TestTone::new(sample_rate),
            channels,
            commands,
        }
    }

    /// Applies waiting commands, then fills `output` with samples.
    fn process<T>(&mut self, output: &mut [T])
    where
        T: SizedSample + FromSample<f32>,
    {
        while let Ok(command) = self.commands.pop() {
            match command {
                Command::SetToneOn(on) => self.tone.set_on(on),
                Command::SetFrequency(hz) => self.tone.set_frequency(hz),
                Command::SetVolume(volume) => self.tone.set_volume(volume),
            }
        }
        for frame in output.chunks_mut(self.channels) {
            let sample = T::from_sample(self.tone.next_sample());
            frame.fill(sample);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commands_reach_the_audio_thread() {
        let (mut producer, consumer) = rtrb::RingBuffer::new(8);
        let mut audio_thread = AudioThread::new(48_000, 2, consumer);
        let mut buffer = vec![0.0_f32; 1024];

        audio_thread.process(&mut buffer);
        assert!(buffer.iter().all(|&s| s == 0.0), "should start silent");

        producer.push(Command::SetToneOn(true)).unwrap();
        audio_thread.process(&mut buffer);
        assert!(
            buffer.iter().any(|&s| s != 0.0),
            "should play after SetToneOn(true)"
        );
    }

    #[test]
    fn converts_to_integer_samples() {
        let (mut producer, consumer) = rtrb::RingBuffer::new(8);
        let mut audio_thread = AudioThread::new(48_000, 1, consumer);
        producer.push(Command::SetVolume(1.0)).unwrap();
        producer.push(Command::SetToneOn(true)).unwrap();

        let mut buffer = vec![0_i16; 4800];
        audio_thread.process(&mut buffer);
        assert!(
            buffer.iter().any(|&s| s > 10_000),
            "16-bit samples should use the full range"
        );
    }
}
