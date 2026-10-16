//! Recorded sounds: loading them from WAV files and holding them in memory.

use std::fmt;
use std::path::Path;

/// A mono sound held in memory, ready to play.
///
/// Instruments hold it inside an `Arc` (a shared, reference-counted
/// pointer), so several voices could play the same sound without copying it.
#[derive(Debug, Clone, PartialEq)]
pub struct SampleData {
    frames: Vec<f32>,
    sample_rate: u32,
}

impl SampleData {
    pub fn new(frames: Vec<f32>, sample_rate: u32) -> Self {
        SampleData {
            frames,
            sample_rate,
        }
    }

    /// The samples, between -1.0 and 1.0.
    pub fn frames(&self) -> &[f32] {
        &self.frames
    }

    /// The sample rate the sound was recorded at.
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Loads a WAV file, mixing stereo (or more channels) down to mono.
    pub fn load_wav(path: &Path) -> Result<Self, SampleError> {
        let mut reader = hound::WavReader::open(path).map_err(SampleError::Wav)?;
        let spec = reader.spec();
        let channels = spec.channels.max(1) as usize;

        // WAV files store samples either as floats or as integers of some
        // bit depth. We turn both into floats between -1.0 and 1.0.
        let interleaved: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Float => reader
                .samples::<f32>()
                .collect::<Result<_, _>>()
                .map_err(SampleError::Wav)?,
            hound::SampleFormat::Int => {
                let full_scale = (1_i64 << (spec.bits_per_sample - 1)) as f32;
                reader
                    .samples::<i32>()
                    .map(|sample| sample.map(|s| s as f32 / full_scale))
                    .collect::<Result<_, _>>()
                    .map_err(SampleError::Wav)?
            }
        };
        if interleaved.is_empty() {
            return Err(SampleError::Empty);
        }

        let mono = interleaved
            .chunks(channels)
            .map(|frame| frame.iter().sum::<f32>() / frame.len() as f32)
            .collect();
        Ok(SampleData::new(mono, spec.sample_rate))
    }
}

/// Why a sample could not be loaded.
#[derive(Debug)]
pub enum SampleError {
    /// The file is missing, unreadable or not a valid WAV file.
    Wav(hound::Error),
    /// The file has no audio in it.
    Empty,
}

impl fmt::Display for SampleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SampleError::Wav(err) => write!(f, "could not read the WAV file: {err}"),
            SampleError::Empty => write!(f, "the WAV file contains no audio"),
        }
    }
}

impl std::error::Error for SampleError {}

#[cfg(test)]
mod tests {
    use super::*;

    /// Writes a small 16-bit stereo WAV file and returns its path.
    fn write_test_wav(name: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(name);
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: 44_100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(&path, spec).unwrap();
        for _ in 0..100 {
            writer.write_sample(i16::MAX).unwrap(); // left: full scale
            writer.write_sample(0_i16).unwrap(); // right: silent
        }
        writer.finalize().unwrap();
        path
    }

    #[test]
    fn loads_and_mixes_to_mono() {
        let path = write_test_wav("ferrous-loom-test-stereo.wav");
        let sample = SampleData::load_wav(&path).unwrap();
        std::fs::remove_file(&path).ok();

        assert_eq!(sample.sample_rate(), 44_100);
        assert_eq!(sample.frames().len(), 100);
        // (1.0 + 0.0) / 2 = 0.5, give or take the 16-bit rounding.
        assert!((sample.frames()[0] - 0.5).abs() < 1e-3);
    }

    #[test]
    fn missing_file_is_an_error() {
        let result = SampleData::load_wav(Path::new("/no/such/file.wav"));
        assert!(matches!(result, Err(SampleError::Wav(_))));
    }
}
