//! Saving sound as a WAV file, as takes are kept.

use std::path::Path;

/// Writes `samples`, one channel at `sample_rate`, to a WAV file at
/// `path`, as 32-bit floating point numbers: nothing is lost of what was
/// recorded, and every sampler reads it.
pub fn write(samples: &[f32], sample_rate: u32, path: &Path) -> Result<(), hound::Error> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = hound::WavWriter::create(path, spec)?;
    for &sample in samples {
        writer.write_sample(sample)?;
    }
    writer.finalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_take_written_reads_back_the_same() {
        let path = std::env::temp_dir().join(format!("lineup-wav-test-{}.wav", std::process::id()));
        let samples: Vec<f32> = (0..480).map(|i| (i as f32 * 0.05).sin() * 0.5).collect();
        write(&samples, 48_000, &path).unwrap();
        let read = crate::decode_file(&path).unwrap();
        std::fs::remove_file(&path).unwrap();
        assert_eq!(read.sample_rate, 48_000);
        assert_eq!(read.samples.len(), samples.len());
        for (a, b) in read.samples.iter().zip(&samples) {
            assert!((a - b).abs() < 1e-6);
        }
    }
}
