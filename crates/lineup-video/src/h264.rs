//! Compressing pictures as H.264, the video format every player and
//! every website reads, with Cisco's OpenH264 encoder.
//!
//! A picture comes in as RGB, three bytes a pixel. The encoder wants it
//! as YUV: brightness (Y) for every pixel, and two colour differences (U
//! and V) for every square of four. The eye sees detail in brightness
//! much better than in colour, so that alone halves the size. The
//! encoder then keeps only what changed from the picture before, which
//! for a window where little moves is very little.
//!
//! What comes out is a list of NAL units ("network abstraction layer"):
//! the parameter sets, which describe the stream, then the pictures.
//! An MP4 file keeps the parameter sets once, in its `avcC` box, and the
//! pictures in its samples, each unit after its length.

use openh264::OpenH264API;
use openh264::encoder::{
    BitRate, Complexity, Encoder, EncoderConfig, FrameRate, IntraFramePeriod, MatrixCoefficients,
    Profile, RateControlMode, UsageType, VuiConfig,
};
use openh264::formats::{RgbaSliceU8, YUVBuffer};

/// The kinds of NAL unit that matter here.
const SEQUENCE_PARAMETERS: u8 = 7;
const PICTURE_PARAMETERS: u8 = 8;
const KEY_PICTURE: u8 = 5;

/// A compressed picture, ready for the MP4 file.
pub struct Picture {
    /// The picture's NAL units, each after its length in 4 bytes.
    pub bytes: Vec<u8>,
    /// Whether it can be decoded on its own: where a player can start.
    pub key: bool,
}

/// Turns pictures into H.264.
pub struct PictureEncoder {
    encoder: Encoder,
    yuv: YUVBuffer,
    width: usize,
    height: usize,
    /// The parameter sets, from the first picture.
    pub sps: Option<Vec<u8>>,
    pub pps: Option<Vec<u8>>,
}

impl PictureEncoder {
    /// An encoder for pictures `width` by `height` pixels (both even),
    /// `frame_rate` of them a second.
    pub fn new(width: usize, height: usize, frame_rate: u32) -> Result<Self, String> {
        let config = EncoderConfig::new()
            // A screen recording: sharp edges, flat colours, text.
            .usage_type(UsageType::ScreenContentRealTime)
            .profile(Profile::Baseline)
            .rate_control_mode(RateControlMode::Bitrate)
            .bitrate(BitRate::from_bps(bitrate(width, height)))
            .max_frame_rate(FrameRate::from_hz(frame_rate as f32))
            // Every picture is kept: dropping one to save room would
            // make the playhead stutter.
            .skip_frames(false)
            .complexity(Complexity::High)
            // A key picture every two seconds, so a player can jump.
            .intra_frame_period(IntraFramePeriod::from_num_frames(frame_rate * 2))
            .vui(VuiConfig::bt709().matrix_coefficients(MatrixCoefficients::Smpte170M));
        let encoder = Encoder::with_api_config(OpenH264API::from_source(), config)
            .map_err(|err| err.to_string())?;
        Ok(PictureEncoder {
            encoder,
            yuv: YUVBuffer::new(width, height),
            width,
            height,
            sps: None,
            pps: None,
        })
    }

    /// Compresses one picture, given as RGBA, four bytes a pixel, row
    /// after row. `None` if the encoder made nothing for it.
    pub fn encode(&mut self, rgba: &[u8]) -> Result<Option<Picture>, String> {
        // OpenH264's conversion uses BT.601's colour matrix, which the
        // stream says (see `new`).
        self.yuv
            .read_rgb8(RgbaSliceU8::new(rgba, (self.width, self.height)));
        let stream = self
            .encoder
            .encode(&self.yuv)
            .map_err(|err| err.to_string())?;
        let mut picture = Picture {
            bytes: Vec::new(),
            key: false,
        };
        for index in 0..stream.num_layers() {
            let Some(layer) = stream.layer(index) else {
                continue;
            };
            for nal in (0..layer.nal_count()).filter_map(|n| layer.nal_unit(n)) {
                let nal = without_start_code(nal);
                let Some(&header) = nal.first() else {
                    continue;
                };
                match header & 0x1F {
                    SEQUENCE_PARAMETERS => self.sps = Some(nal.to_vec()),
                    PICTURE_PARAMETERS => self.pps = Some(nal.to_vec()),
                    kind => {
                        picture.key |= kind == KEY_PICTURE;
                        picture
                            .bytes
                            .extend_from_slice(&(nal.len() as u32).to_be_bytes());
                        picture.bytes.extend_from_slice(nal);
                    }
                }
            }
        }
        Ok((!picture.bytes.is_empty()).then_some(picture))
    }
}

/// Bits per second for pictures of this size: about 4 Mbit/s at
/// 1280×720, 9 at 1920×1080. Text stays sharp, and the file small.
fn bitrate(width: usize, height: usize) -> u32 {
    (width * height * 9 / 2).clamp(500_000, 20_000_000) as u32
}

/// A NAL unit without the 0 0 1 or 0 0 0 1 the encoder puts before it to
/// mark where it starts.
fn without_start_code(nal: &[u8]) -> &[u8] {
    match nal {
        [0, 0, 0, 1, rest @ ..] | [0, 0, 1, rest @ ..] => rest,
        _ => nal,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_codes_are_taken_off() {
        assert_eq!(without_start_code(&[0, 0, 0, 1, 0x67, 9]), &[0x67, 9]);
        assert_eq!(without_start_code(&[0, 0, 1, 0x68]), &[0x68]);
        assert_eq!(without_start_code(&[0x65, 1]), &[0x65, 1]);
    }

    #[test]
    fn the_first_picture_is_a_key_picture_with_parameter_sets() {
        let (width, height) = (64, 48);
        let mut encoder = PictureEncoder::new(width, height, 30).unwrap();
        let mut rgba = vec![255; width * height * 4];
        let first = encoder.encode(&rgba).unwrap().unwrap();
        assert!(first.key);
        let sps = encoder.sps.clone().unwrap();
        assert_eq!(sps[0] & 0x1F, SEQUENCE_PARAMETERS);
        assert!(encoder.pps.is_some());
        // Each unit is after its length: the first length covers the
        // first unit, which is the key picture itself.
        let length = u32::from_be_bytes(first.bytes[..4].try_into().unwrap()) as usize;
        assert!(4 + length <= first.bytes.len());
        assert_eq!(first.bytes[4] & 0x1F, KEY_PICTURE);

        rgba[0] = 0;
        let second = encoder.encode(&rgba).unwrap().unwrap();
        assert!(!second.key, "the next one only says what changed");
        assert!(second.bytes.len() < first.bytes.len());
    }
}
