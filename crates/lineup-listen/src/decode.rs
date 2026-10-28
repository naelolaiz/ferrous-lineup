//! Turning an audio file (MP3, WAV, FLAC or Ogg Vorbis) into samples.
//!
//! The work is done by Symphonia, a pure-Rust decoder: no C library to
//! install on any system. A file holds a *container* (WAV, MP3, Ogg...)
//! with one or more *tracks*, each compressed with a *codec*. Decoding
//! is: probe the container, pick its audio track, then feed its
//! *packets* one by one to the codec's decoder, which gives back blocks
//! of samples.

use std::fmt;
use std::fs::File;
use std::path::Path;

use lineup_core::i18n::{tr, trf};
use symphonia::core::codecs::audio::AudioDecoderOptions;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::{FormatOptions, TrackType};
use symphonia::core::io::{MediaSourceStream, MediaSourceStreamOptions};
use symphonia::core::meta::MetadataOptions;

/// The longest recording the importer listens to. Analysis holds the
/// recording, and a copy of it, in memory while it works, so a longer
/// one would need more than a small machine can spare.
pub const MAX_SECONDS: f64 = 10.0 * 60.0;

/// A recording, mixed down to one channel, between -1.0 and 1.0.
#[derive(Debug, Clone, PartialEq)]
pub struct Recording {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
}

impl Recording {
    /// How long the recording lasts, in seconds.
    pub fn seconds(&self) -> f64 {
        self.samples.len() as f64 / f64::from(self.sample_rate.max(1))
    }
}

/// Why a file could not be decoded.
#[derive(Debug)]
pub enum DecodeError {
    /// The file could not be opened.
    Io(std::io::Error),
    /// Symphonia does not know the format, or the file is damaged.
    Format(String),
    /// The file has no audio in it.
    Empty,
    /// The recording is longer than [`MAX_SECONDS`].
    TooLong,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::Io(err) => f.write_str(&trf("could not open the file: {}", &[err])),
            DecodeError::Format(why) => f.write_str(&trf("could not decode the file: {}", &[why])),
            DecodeError::Empty => f.write_str(tr("the file contains no audio")),
            DecodeError::TooLong => f.write_str(&trf(
                "the recording is longer than {} minutes",
                &[&(MAX_SECONDS / 60.0)],
            )),
        }
    }
}

impl std::error::Error for DecodeError {}

/// Reads and decodes a whole audio file, mixing all its channels to one.
pub fn decode_file(path: &Path) -> Result<Recording, DecodeError> {
    let file = File::open(path).map_err(DecodeError::Io)?;
    let stream = MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());

    // The extension is only a hint: Symphonia looks at the bytes too.
    let mut hint = Hint::new();
    if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(extension);
    }
    let format_error = |err: SymphoniaError| DecodeError::Format(err.to_string());
    let mut format = symphonia::default::get_probe()
        .probe(
            &hint,
            stream,
            FormatOptions::default(),
            MetadataOptions::default(),
        )
        .map_err(format_error)?;

    let track = format
        .default_track(TrackType::Audio)
        .ok_or_else(|| DecodeError::Format("no audio track".to_string()))?;
    let track_id = track.id;
    let params = track
        .codec_params
        .as_ref()
        .and_then(|params| params.audio())
        .ok_or_else(|| DecodeError::Format("unknown audio codec".to_string()))?
        .clone();
    let mut decoder = symphonia::default::get_codecs()
        .make_audio_decoder(&params, &AudioDecoderOptions::default())
        .map_err(format_error)?;

    let mut samples = Vec::new();
    let mut interleaved: Vec<f32> = Vec::new();
    let mut sample_rate = 0;
    loop {
        let packet = match format.next_packet() {
            Ok(Some(packet)) => packet,
            Ok(None) => break,
            // A file cut short ends with an I/O error: keep what we have.
            Err(SymphoniaError::IoError(_)) => break,
            Err(err) => return Err(format_error(err)),
        };
        if packet.track_id != track_id {
            continue;
        }
        let block = match decoder.decode(&packet) {
            Ok(block) => block,
            // One damaged packet is a click, not a reason to give up.
            Err(SymphoniaError::DecodeError(_)) => continue,
            Err(err) => return Err(format_error(err)),
        };
        let channels = block.spec().channels().count().max(1);
        sample_rate = block.spec().rate();
        block.copy_to_vec_interleaved(&mut interleaved);
        samples.extend(
            interleaved
                .chunks(channels)
                .map(|frame| frame.iter().sum::<f32>() / channels as f32),
        );
        if too_long(samples.len(), sample_rate) {
            return Err(DecodeError::TooLong);
        }
    }
    if samples.is_empty() || sample_rate == 0 {
        return Err(DecodeError::Empty);
    }
    Ok(Recording {
        samples,
        sample_rate,
    })
}

/// Whether `samples` at `sample_rate` last longer than [`MAX_SECONDS`].
fn too_long(samples: usize, sample_rate: u32) -> bool {
    sample_rate > 0 && samples as f64 / f64::from(sample_rate) > MAX_SECONDS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_recording_longer_than_the_limit_is_refused() {
        let rate = 44_100;
        let limit = (f64::from(rate) * MAX_SECONDS) as usize;
        assert!(!too_long(limit, rate));
        assert!(too_long(limit + 1, rate));
        assert_eq!(
            DecodeError::TooLong.to_string(),
            "the recording is longer than 10 minutes"
        );
    }
}
