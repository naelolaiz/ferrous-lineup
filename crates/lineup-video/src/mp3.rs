//! Compressing the sound as MP3, with the LAME encoder.
//!
//! LAME takes the samples and gives back MP3 "frames": each holds 1152
//! samples of every channel, and starts with a 4-byte header that says
//! how long it is. The MP4 file keeps each frame as one sample of the
//! sound track, so the frames are cut apart here as they come.
//!
//! MP3 sound comes out a little late: the encoder puts 576 frames of
//! silence first, and a decoder adds 529 more. The MP4 file says so (see
//! [`PRIMING`]), and players skip them, so the sound starts exactly when
//! the pictures do.

use std::mem::MaybeUninit;

use mp3lame_encoder::{Bitrate, Builder, Encoder, FlushGap, InterleavedPcm, Mode, Quality};

/// Frames of a channel in an MP3 frame (MPEG-1 Layer III).
pub const FRAME_LENGTH: u32 = 1152;

/// The silence before the sound: LAME's 576 frames, and a decoder's 529.
pub const PRIMING: u32 = 576 + 529;

/// The bit rate: 256 kbit/s, which sounds the same as the WAV file to
/// almost everyone.
pub const BITRATE: u32 = 256_000;

/// Turns samples into MP3 frames.
pub struct SoundEncoder {
    encoder: Encoder,
    /// Bytes made and not yet cut into frames.
    pending: Vec<u8>,
}

impl SoundEncoder {
    /// An encoder for stereo sound at `sample_rate` frames a second (one
    /// of 32 000, 44 100 and 48 000).
    pub fn new(sample_rate: u32) -> Result<Self, String> {
        let mut builder = Builder::new().ok_or("LAME could not start")?;
        builder.set_num_channels(2).map_err(|err| err.to_string())?;
        builder
            .set_sample_rate(sample_rate)
            .map_err(|err| err.to_string())?;
        builder
            .set_brate(Bitrate::Kbps256)
            .map_err(|err| err.to_string())?;
        builder
            .set_mode(Mode::JointStereo)
            .map_err(|err| err.to_string())?;
        builder
            .set_quality(Quality::NearBest)
            .map_err(|err| err.to_string())?;
        // An MP3 file can start with a frame of information about the
        // file, which LAME writes as silence until the end. In an MP4
        // file it would be heard: a moment of silence, and the sound late.
        builder
            .set_to_write_vbr_tag(false)
            .map_err(|err| err.to_string())?;
        let encoder = builder.build().map_err(|err| err.to_string())?;
        Ok(SoundEncoder {
            encoder,
            pending: Vec::new(),
        })
    }

    /// Compresses `samples`, left and right one after the other, and
    /// returns the MP3 frames finished so far.
    pub fn encode(&mut self, samples: &[f32]) -> Result<Vec<Vec<u8>>, String> {
        let frames = samples.len() / 2;
        self.pending
            .reserve(mp3lame_encoder::max_required_buffer_size(frames));
        let written = self
            .encoder
            .encode(InterleavedPcm(samples), spare(&mut self.pending))
            .map_err(|err| err.to_string())?;
        // SAFETY: LAME wrote `written` bytes into the spare room.
        unsafe { self.pending.set_len(self.pending.len() + written) };
        Ok(self.cut())
    }

    /// Ends the sound, and returns the last frames: the sound still in
    /// the encoder, padded with silence to a whole frame.
    pub fn finish(&mut self) -> Result<Vec<Vec<u8>>, String> {
        self.pending.reserve(7200);
        let written = self
            .encoder
            .flush::<FlushGap>(spare(&mut self.pending))
            .map_err(|err| err.to_string())?;
        // SAFETY: as in `encode`.
        unsafe { self.pending.set_len(self.pending.len() + written) };
        Ok(self.cut())
    }

    /// Takes the whole frames off the front of the bytes made.
    fn cut(&mut self) -> Vec<Vec<u8>> {
        let mut frames = Vec::new();
        let mut at = 0;
        while let Some(length) = self.pending.get(at..).and_then(frame_length) {
            if at + length > self.pending.len() {
                break;
            }
            frames.push(self.pending[at..at + length].to_vec());
            at += length;
        }
        self.pending.drain(..at);
        frames
    }
}

/// The spare room at the end of `bytes`, for LAME to write in.
fn spare(bytes: &mut Vec<u8>) -> &mut [MaybeUninit<u8>] {
    bytes.spare_capacity_mut()
}

/// The length in bytes of the MPEG-1 Layer III frame whose header
/// starts `bytes`, or `None` if they do not start with one.
///
/// The header is 32 bits: 11 bits set to find it, the version and the
/// layer, then the bit rate and the sample rate as numbers in a table,
/// and whether the frame has one byte more (padding).
fn frame_length(bytes: &[u8]) -> Option<usize> {
    let [first, second, third, ..] = *bytes else {
        return None;
    };
    // Sync, MPEG-1 (11), Layer III (01).
    if first != 0xFF || second & 0xFE != 0xFA {
        return None;
    }
    const KBPS: [usize; 15] = [
        0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320,
    ];
    const RATES: [usize; 3] = [44_100, 48_000, 32_000];
    let kbps = *KBPS.get(usize::from(third >> 4))?;
    let rate = *RATES.get(usize::from((third >> 2) & 0b11))?;
    let padding = usize::from((third >> 1) & 1);
    (kbps > 0).then(|| 144 * kbps * 1000 / rate + padding)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_frame_length_from_the_header() {
        // 256 kbit/s at 44.1 kHz: 144 × 256 000 / 44 100 = 835.9, so 835
        // bytes, or 836 with padding.
        assert_eq!(frame_length(&[0xFF, 0xFB, 0xD0, 0x00]), Some(835));
        assert_eq!(frame_length(&[0xFF, 0xFB, 0xD2, 0x00]), Some(836));
        // 48 kHz: exactly 768.
        assert_eq!(frame_length(&[0xFF, 0xFB, 0xD4, 0x00]), Some(768));
        assert_eq!(frame_length(&[0x00, 0xFB, 0xD0]), None, "no sync");
        assert_eq!(frame_length(&[0xFF, 0xFB]), None, "too short");
    }

    #[test]
    fn a_second_of_sound_makes_whole_frames() {
        let rate = 44_100;
        let mut encoder = SoundEncoder::new(rate).unwrap();
        let mut frames = Vec::new();
        // A sine wave at 440 Hz, a tenth of a second at a time.
        for block in 0..10 {
            let samples: Vec<f32> = (0..rate / 10)
                .flat_map(|frame| {
                    let t = f64::from(block * rate / 10 + frame) / f64::from(rate);
                    let sample = (t * 440.0 * std::f64::consts::TAU).sin() as f32 * 0.5;
                    [sample, sample]
                })
                .collect();
            frames.extend(encoder.encode(&samples).unwrap());
        }
        frames.extend(encoder.finish().unwrap());
        // 44 100 frames, and LAME's 576 of silence, in frames of 1152.
        let needed = (44_100 + 576_u32).div_ceil(FRAME_LENGTH) as usize;
        assert!(frames.len() >= needed, "{} frames", frames.len());
        for frame in &frames {
            assert_eq!(frame_length(frame), Some(frame.len()));
        }
    }
}
