//! Video files for Ferrous Lineup: the song's sound with pictures of the
//! app, as an MP4 file any player or website takes.
//!
//! - [`canvas`] paints egui's shapes into a picture in memory, so the
//!   app's views can be drawn without a window or a graphics card.
//! - [`h264`] compresses the pictures as H.264, with OpenH264.
//! - [`mp3`] compresses the sound as MP3, with LAME.
//! - [`mp4`] puts both in an MP4 file.
//!
//! Both encoders are built from their C sources with the app, so making
//! a video needs nothing installed, on any system.
//!
//! [`VideoWriter`] does it all: give it a picture, then the sound that
//! plays while it is shown, and so on to the end.

pub mod canvas;
pub mod h264;
pub mod mp3;
pub mod mp4;

use std::fs::{self, File};
use std::path::{Path, PathBuf};

pub use canvas::Canvas;

use h264::PictureEncoder;
use mp3::SoundEncoder;
use mp4::{AudioFormat, Mp4Writer, VideoFormat};

/// The pictures' clock: 90 000 ticks a second, as video uses, which
/// divides evenly into frames at 24, 25, 30 or 60 a second.
const VIDEO_TIMESCALE: u32 = 90_000;

/// The size and speed of a video.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VideoSettings {
    /// In pixels; both even, as H.264 needs.
    pub width: usize,
    pub height: usize,
    /// Pictures a second.
    pub frame_rate: u32,
    /// Sound frames a second (stereo).
    pub sample_rate: u32,
}

/// A video file being written.
pub struct VideoWriter {
    settings: VideoSettings,
    path: PathBuf,
    temporary: PathBuf,
    file: Mp4Writer,
    pictures: PictureEncoder,
    sound: SoundEncoder,
    /// How many frames of sound went in, to say how long it lasts.
    sound_frames: u64,
}

impl VideoWriter {
    /// Starts writing a video to `path`. It goes to a temporary file
    /// first, renamed at the end, so a failed export never leaves half a
    /// video in place of an older one.
    pub fn create(path: &Path, settings: VideoSettings) -> Result<Self, String> {
        if !settings.width.is_multiple_of(2)
            || !settings.height.is_multiple_of(2)
            || settings.frame_rate == 0
        {
            return Err("the picture size must be even".into());
        }
        let mut temporary = PathBuf::from(path);
        temporary.as_mut_os_string().push(".tmp");
        let file = File::create(&temporary).map_err(|err| err.to_string())?;
        Ok(VideoWriter {
            settings,
            path: path.to_path_buf(),
            file: Mp4Writer::create(file).map_err(|err| err.to_string())?,
            temporary,
            pictures: PictureEncoder::new(settings.width, settings.height, settings.frame_rate)?,
            sound: SoundEncoder::new(settings.sample_rate)?,
            sound_frames: 0,
        })
    }

    /// Adds the next picture, RGBA, four bytes a pixel.
    pub fn add_picture(&mut self, rgba: &[u8]) -> Result<(), String> {
        let ticks = VIDEO_TIMESCALE / self.settings.frame_rate;
        match self.pictures.encode(rgba)? {
            Some(picture) => self
                .file
                .add_video(&picture.bytes, ticks, picture.key)
                .map_err(|err| err.to_string()),
            None => {
                self.file.extend_last_video(ticks);
                Ok(())
            }
        }
    }

    /// Shows the last picture for one picture longer: nothing changed.
    pub fn repeat_picture(&mut self) {
        self.file
            .extend_last_video(VIDEO_TIMESCALE / self.settings.frame_rate);
    }

    /// Adds the next stretch of sound: left and right samples, one after
    /// the other, between -1 and 1.
    pub fn add_sound(&mut self, samples: &[f32]) -> Result<(), String> {
        self.sound_frames += samples.len() as u64 / 2;
        for frame in self.sound.encode(samples)? {
            self.file
                .add_audio(&frame, mp3::FRAME_LENGTH)
                .map_err(|err| err.to_string())?;
        }
        Ok(())
    }

    /// Finishes the file and puts it in place.
    pub fn finish(mut self) -> Result<(), String> {
        for frame in self.sound.finish()? {
            self.file
                .add_audio(&frame, mp3::FRAME_LENGTH)
                .map_err(|err| err.to_string())?;
        }
        let (Some(sps), Some(pps)) = (self.pictures.sps.take(), self.pictures.pps.take()) else {
            return Err("there were no pictures".into());
        };
        let video = VideoFormat {
            width: self.settings.width as u32,
            height: self.settings.height as u32,
            timescale: VIDEO_TIMESCALE,
            sps,
            pps,
        };
        let audio = AudioFormat {
            sample_rate: self.settings.sample_rate,
            channels: 2,
            bitrate: mp3::BITRATE,
            priming: mp3::PRIMING,
            frames: self.sound_frames,
        };
        self.file
            .finish(&video, &audio)
            .map_err(|err| err.to_string())?;
        fs::rename(&self.temporary, &self.path).map_err(|err| err.to_string())
    }

    /// Stops writing, and removes the unfinished file.
    pub fn cancel(self) {
        let temporary = self.temporary.clone();
        drop(self);
        fs::remove_file(temporary).ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_a_video() {
        let path = std::env::temp_dir().join(format!("lineup-video-{}.mp4", std::process::id()));
        let settings = VideoSettings {
            width: 64,
            height: 48,
            frame_rate: 30,
            sample_rate: 44_100,
        };
        let mut writer = VideoWriter::create(&path, settings).unwrap();
        let samples_per_picture = 44_100 / 30 * 2;
        for frame in 0..30_u8 {
            if frame % 2 == 0 {
                let rgba = vec![frame * 8; 64 * 48 * 4];
                writer.add_picture(&rgba).unwrap();
            } else {
                writer.repeat_picture();
            }
            writer.add_sound(&vec![0.0; samples_per_picture]).unwrap();
        }
        writer.finish().unwrap();
        let bytes = fs::read(&path).unwrap();
        fs::remove_file(&path).ok();
        assert_eq!(&bytes[4..8], b"ftyp");
        assert!(bytes.windows(4).any(|name| name == b"moov"));
    }

    #[test]
    fn odd_sizes_are_refused() {
        let settings = VideoSettings {
            width: 63,
            height: 48,
            frame_rate: 30,
            sample_rate: 44_100,
        };
        let path = std::env::temp_dir().join("never-written.mp4");
        assert!(VideoWriter::create(&path, settings).is_err());
    }
}
