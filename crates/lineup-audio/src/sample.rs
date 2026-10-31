//! Recorded sounds: loading them from WAV files and holding them in memory.

use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use lineup_core::i18n::{tr, trf};

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

/// Whether the audio file at `path` is a recording, to add to the song
/// as a take, rather than a sound for the sampler: a file the sampler cannot play (anything but
/// WAV, such as MP3), or a WAV longer than a sound that rings out (see
/// [`crate::Instrument::release_all`]), such as a whole song.
pub fn is_recording(path: &Path) -> bool {
    let is_wav = path
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("wav"));
    if !is_wav {
        return true;
    }
    hound::WavReader::open(path).is_ok_and(|reader| {
        let rate = u64::from(reader.spec().sample_rate);
        u64::from(reader.duration()) > rate * crate::instrument::LONG_SAMPLE_SECONDS
    })
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
            SampleError::Wav(err) => f.write_str(&trf("could not read the WAV file: {}", &[err])),
            SampleError::Empty => f.write_str(tr("the WAV file contains no audio")),
        }
    }
}

impl std::error::Error for SampleError {}

/// Samples already loaded from files, by path.
///
/// Building an instrument for a sample file reads it from the disk. With
/// undo, the same channel can come and go many times, and an export
/// builds every instrument again, so each file is read once and then
/// shared: an `Arc` lets every sampler that plays it point to the same
/// memory.
#[derive(Debug, Default)]
pub struct SampleCache {
    loaded: HashMap<PathBuf, Arc<SampleData>>,
}

impl SampleCache {
    pub fn new() -> Self {
        SampleCache::default()
    }

    /// The sound in `path`, read from the file the first time and from
    /// memory after that.
    pub fn get(&mut self, path: &Path) -> Result<Arc<SampleData>, SampleError> {
        // A `HashMap<PathBuf, _>` can be searched with a `&Path`, because
        // `PathBuf` implements `Borrow<Path>`: no `PathBuf` is built here.
        if let Some(sample) = self.loaded.get(path) {
            return Ok(Arc::clone(sample));
        }
        self.reload(path)
    }

    /// Reads the file again even if it was read before, for when the
    /// user drops it again (it may have changed). A file that cannot be
    /// read is not remembered, so the next try reads it again.
    pub fn reload(&mut self, path: &Path) -> Result<Arc<SampleData>, SampleError> {
        let sample = Arc::new(SampleData::load_wav(path)?);
        self.loaded.insert(path.to_path_buf(), Arc::clone(&sample));
        Ok(sample)
    }

    /// Forgets every sample, for example when another project is opened.
    pub fn clear(&mut self) {
        self.loaded.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_long_wav_or_another_kind_of_file_is_a_recording() {
        let short = write_test_wav("lineup-short-sound.wav");
        assert!(!is_recording(&short), "a drum hit is a sound");
        let long = std::env::temp_dir().join("lineup-long-recording.wav");
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 8_000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(&long, spec).unwrap();
        for _ in 0..11 * 8_000 {
            writer.write_sample(0_i16).unwrap();
        }
        writer.finalize().unwrap();
        assert!(is_recording(&long), "eleven seconds is a recording");
        assert!(
            is_recording(Path::new("song.mp3")),
            "the sampler reads only WAV"
        );
        std::fs::remove_file(short).ok();
        std::fs::remove_file(long).ok();
    }

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
        let path = write_test_wav("ferrous-lineup-test-stereo.wav");
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

    #[test]
    fn the_cache_reads_each_file_once() {
        let path = write_test_wav("ferrous-lineup-test-cache.wav");
        let mut cache = SampleCache::new();
        let first = cache.get(&path).unwrap();
        let second = cache.get(&path).unwrap();
        let reloaded = cache.reload(&path).unwrap();
        std::fs::remove_file(&path).ok();

        // `Arc::ptr_eq` asks whether two `Arc`s point to the same memory.
        assert!(
            Arc::ptr_eq(&first, &second),
            "the second get shares the first"
        );
        assert!(
            !Arc::ptr_eq(&first, &reloaded),
            "reload reads the file again"
        );
        assert!(Arc::ptr_eq(&reloaded, &cache.get(&path).unwrap()));
    }

    #[test]
    fn the_cache_does_not_remember_failures() {
        let mut cache = SampleCache::new();
        let path = Path::new("/no/such/file.wav");
        assert!(cache.get(path).is_err());
        assert!(cache.loaded.is_empty());
    }
}
