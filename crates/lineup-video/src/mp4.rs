//! Writing an MP4 file: the pictures and the sound, side by side.
//!
//! An MP4 file is a list of "boxes". Each box starts with its size and
//! a four-letter name, and holds bytes or other boxes. Two matter here:
//!
//! - `mdat` (media data) holds the compressed pictures and sound, one
//!   after the other, as they are made.
//! - `moov` (movie) says where each of them is in `mdat`, how big it is,
//!   how long it lasts, and how to decode it. It can only be written
//!   once everything is in, so it goes at the end.
//!
//! A picture or a piece of sound in `mdat` is a "sample". The tables in
//! `moov` list every sample of a track: its size (`stsz`), where it starts
//! in the file (`stco`), how long it lasts (`stts`), and for pictures
//! which ones can be decoded on their own (`stss`), to jump to.
//!
//! The format is ISO/IEC 14496-12 (the ISO base media file format) and
//! 14496-14 (MP4). Only what a video with one H.264 track and one MP3
//! track needs is written.

use std::fs::File;
use std::io::{self, BufWriter, Seek, SeekFrom, Write};

/// The movie's own clock: milliseconds.
const MOVIE_TIMESCALE: u32 = 1000;

/// What the pictures are.
#[derive(Debug, Clone)]
pub struct VideoFormat {
    pub width: u32,
    pub height: u32,
    /// Ticks per second of the track's clock.
    pub timescale: u32,
    /// The H.264 sequence parameter set: the picture size, the profile.
    pub sps: Vec<u8>,
    /// The H.264 picture parameter set.
    pub pps: Vec<u8>,
}

/// What the sound is.
#[derive(Debug, Clone, Copy)]
pub struct AudioFormat {
    pub sample_rate: u32,
    pub channels: u16,
    /// Bits per second, for players that show it.
    pub bitrate: u32,
    /// How many frames of silence the encoder and decoder put before the
    /// sound, which a player skips.
    pub priming: u32,
    /// How many frames of real sound there are, without the priming and
    /// the padding after the end.
    pub frames: u64,
}

/// The samples of one track, as they were written.
#[derive(Default)]
struct Track {
    sizes: Vec<u32>,
    offsets: Vec<u64>,
    /// How long each sample lasts, in ticks of the track's clock.
    durations: Vec<u32>,
    /// The numbers (from 1) of the samples that can be decoded on their
    /// own: the key frames.
    sync: Vec<u32>,
}

impl Track {
    fn duration(&self) -> u64 {
        self.durations.iter().map(|&ticks| u64::from(ticks)).sum()
    }
}

/// An MP4 file being written.
pub struct Mp4Writer {
    file: BufWriter<File>,
    /// Where the `mdat` box starts, to write its size at the end.
    mdat_start: u64,
    /// Where the next sample goes.
    position: u64,
    video: Track,
    audio: Track,
}

impl Mp4Writer {
    /// Starts the file: its type, then the media data box, still empty.
    pub fn create(file: File) -> io::Result<Self> {
        let mut file = BufWriter::new(file);
        let mut head = Vec::new();
        write_box(&mut head, b"ftyp", |out| {
            out.extend_from_slice(b"isom");
            out.extend_from_slice(&512_u32.to_be_bytes());
            for brand in [b"isom", b"iso2", b"avc1", b"mp41"] {
                out.extend_from_slice(brand);
            }
        });
        let mdat_start = head.len() as u64;
        // A size of 1 says the real size follows as 64 bits, so the file
        // can grow past 4 GB. It is filled in at the end.
        head.extend_from_slice(&1_u32.to_be_bytes());
        head.extend_from_slice(b"mdat");
        head.extend_from_slice(&0_u64.to_be_bytes());
        file.write_all(&head)?;
        Ok(Mp4Writer {
            file,
            mdat_start,
            position: head.len() as u64,
            video: Track::default(),
            audio: Track::default(),
        })
    }

    /// Adds a picture lasting `duration` ticks of the video clock.
    /// `key` says whether it can be decoded without the ones before.
    pub fn add_video(&mut self, bytes: &[u8], duration: u32, key: bool) -> io::Result<()> {
        let number = self.video.sizes.len() as u32 + 1;
        if key {
            self.video.sync.push(number);
        }
        Self::add(
            &mut self.file,
            &mut self.position,
            &mut self.video,
            bytes,
            duration,
        )
    }

    /// Makes the last picture last `ticks` longer: the encoder made
    /// nothing for a picture, so the one before stays on screen.
    pub fn extend_last_video(&mut self, ticks: u32) {
        if let Some(last) = self.video.durations.last_mut() {
            *last += ticks;
        }
    }

    /// Adds a piece of sound lasting `duration` frames.
    pub fn add_audio(&mut self, bytes: &[u8], duration: u32) -> io::Result<()> {
        Self::add(
            &mut self.file,
            &mut self.position,
            &mut self.audio,
            bytes,
            duration,
        )
    }

    fn add(
        file: &mut BufWriter<File>,
        position: &mut u64,
        track: &mut Track,
        bytes: &[u8],
        duration: u32,
    ) -> io::Result<()> {
        file.write_all(bytes)?;
        track.sizes.push(bytes.len() as u32);
        track.offsets.push(*position);
        track.durations.push(duration);
        *position += bytes.len() as u64;
        Ok(())
    }

    /// Writes the size of the media data and the movie box, which says
    /// where everything is, and closes the file.
    pub fn finish(mut self, video: &VideoFormat, audio: &AudioFormat) -> io::Result<()> {
        self.file.seek(SeekFrom::Start(self.mdat_start + 8))?;
        self.file
            .write_all(&(self.position - self.mdat_start).to_be_bytes())?;
        self.file.seek(SeekFrom::Start(self.position))?;

        let mut moov = Vec::new();
        self.write_moov(&mut moov, video, audio);
        self.file.write_all(&moov)?;
        self.file.flush()?;
        self.file.get_ref().sync_all()
    }

    fn write_moov(&self, out: &mut Vec<u8>, video: &VideoFormat, audio: &AudioFormat) {
        let video_ms = to_movie_time(self.video.duration(), video.timescale);
        let audio_ms = to_movie_time(audio.frames, audio.sample_rate);
        write_box(out, b"moov", |out| {
            write_full_box(out, b"mvhd", 0, 0, |out| {
                out.extend_from_slice(&[0; 8]); // created, modified
                out.extend_from_slice(&MOVIE_TIMESCALE.to_be_bytes());
                out.extend_from_slice(&(video_ms.max(audio_ms) as u32).to_be_bytes());
                out.extend_from_slice(&0x0001_0000_u32.to_be_bytes()); // rate 1.0
                out.extend_from_slice(&0x0100_u16.to_be_bytes()); // volume 1.0
                out.extend_from_slice(&[0; 10]);
                write_matrix(out);
                out.extend_from_slice(&[0; 24]);
                out.extend_from_slice(&3_u32.to_be_bytes()); // the next track's number
            });
            self.write_video_trak(out, video, video_ms);
            self.write_audio_trak(out, audio, audio_ms);
        });
    }

    fn write_video_trak(&self, out: &mut Vec<u8>, format: &VideoFormat, duration_ms: u64) {
        write_box(out, b"trak", |out| {
            write_tkhd(out, 1, duration_ms, false, format.width, format.height);
            write_box(out, b"mdia", |out| {
                write_mdhd(out, format.timescale, self.video.duration());
                write_hdlr(out, b"vide", "Pictures");
                write_box(out, b"minf", |out| {
                    write_full_box(out, b"vmhd", 0, 1, |out| out.extend_from_slice(&[0; 8]));
                    write_dinf(out);
                    write_box(out, b"stbl", |out| {
                        write_full_box(out, b"stsd", 0, 0, |out| {
                            out.extend_from_slice(&1_u32.to_be_bytes());
                            write_avc1(out, format);
                        });
                        write_tables(out, &self.video, true);
                    });
                });
            });
        });
    }

    fn write_audio_trak(&self, out: &mut Vec<u8>, format: &AudioFormat, duration_ms: u64) {
        write_box(out, b"trak", |out| {
            write_tkhd(out, 2, duration_ms, true, 0, 0);
            // The edit list: play the track from `priming` on, for as long
            // as the real sound lasts. The silence the encoder put first,
            // and the padding after the last frame, are skipped.
            write_box(out, b"edts", |out| {
                write_full_box(out, b"elst", 0, 0, |out| {
                    out.extend_from_slice(&1_u32.to_be_bytes());
                    out.extend_from_slice(&(duration_ms as u32).to_be_bytes());
                    out.extend_from_slice(&format.priming.to_be_bytes());
                    out.extend_from_slice(&1_u16.to_be_bytes()); // at normal speed
                    out.extend_from_slice(&0_u16.to_be_bytes());
                });
            });
            write_box(out, b"mdia", |out| {
                write_mdhd(out, format.sample_rate, self.audio.duration());
                write_hdlr(out, b"soun", "Sound");
                write_box(out, b"minf", |out| {
                    write_full_box(out, b"smhd", 0, 0, |out| out.extend_from_slice(&[0; 4]));
                    write_dinf(out);
                    write_box(out, b"stbl", |out| {
                        write_full_box(out, b"stsd", 0, 0, |out| {
                            out.extend_from_slice(&1_u32.to_be_bytes());
                            write_mp4a(out, format);
                        });
                        write_tables(out, &self.audio, false);
                    });
                });
            });
        });
    }
}

/// `ticks` of a clock running at `timescale` ticks a second, in
/// milliseconds.
fn to_movie_time(ticks: u64, timescale: u32) -> u64 {
    ticks * u64::from(MOVIE_TIMESCALE) / u64::from(timescale.max(1))
}

/// Writes a box: its size, its name, then whatever `contents` writes.
/// The size is only known afterwards, so it is filled in at the end.
fn write_box(out: &mut Vec<u8>, name: &[u8; 4], contents: impl FnOnce(&mut Vec<u8>)) {
    let start = out.len();
    out.extend_from_slice(&[0; 4]);
    out.extend_from_slice(name);
    contents(out);
    let size = (out.len() - start) as u32;
    out[start..start + 4].copy_from_slice(&size.to_be_bytes());
}

/// A "full" box: a box that starts with a version and 24 bits of flags.
fn write_full_box(
    out: &mut Vec<u8>,
    name: &[u8; 4],
    version: u8,
    flags: u32,
    contents: impl FnOnce(&mut Vec<u8>),
) {
    write_box(out, name, |out| {
        out.push(version);
        out.extend_from_slice(&flags.to_be_bytes()[1..]);
        contents(out);
    });
}

/// The identity matrix, in the 16.16 and 2.30 fixed-point numbers MP4
/// uses: the picture is shown as it is, not turned or stretched.
fn write_matrix(out: &mut Vec<u8>) {
    for value in [0x0001_0000_u32, 0, 0, 0, 0x0001_0000, 0, 0, 0, 0x4000_0000] {
        out.extend_from_slice(&value.to_be_bytes());
    }
}

/// The track header. Flags 3: the track is on and part of the movie.
fn write_tkhd(out: &mut Vec<u8>, id: u32, duration_ms: u64, sound: bool, width: u32, height: u32) {
    write_full_box(out, b"tkhd", 0, 3, |out| {
        out.extend_from_slice(&[0; 8]); // created, modified
        out.extend_from_slice(&id.to_be_bytes());
        out.extend_from_slice(&[0; 4]);
        out.extend_from_slice(&(duration_ms as u32).to_be_bytes());
        out.extend_from_slice(&[0; 8]);
        out.extend_from_slice(&[0; 4]); // layer, alternate group
        let volume: u16 = if sound { 0x0100 } else { 0 };
        out.extend_from_slice(&volume.to_be_bytes());
        out.extend_from_slice(&[0; 2]);
        write_matrix(out);
        out.extend_from_slice(&(width << 16).to_be_bytes());
        out.extend_from_slice(&(height << 16).to_be_bytes());
    });
}

/// The media header: the track's clock and length, and its language
/// ("und", undetermined, packed into three 5-bit letters).
fn write_mdhd(out: &mut Vec<u8>, timescale: u32, duration: u64) {
    write_full_box(out, b"mdhd", 0, 0, |out| {
        out.extend_from_slice(&[0; 8]);
        out.extend_from_slice(&timescale.to_be_bytes());
        out.extend_from_slice(&(duration as u32).to_be_bytes());
        let language = b"und".iter().fold(0_u16, |packed, &letter| {
            (packed << 5) | u16::from(letter - 0x60)
        });
        out.extend_from_slice(&language.to_be_bytes());
        out.extend_from_slice(&[0; 2]);
    });
}

/// What kind of track this is: `vide` or `soun`.
fn write_hdlr(out: &mut Vec<u8>, kind: &[u8; 4], name: &str) {
    write_full_box(out, b"hdlr", 0, 0, |out| {
        out.extend_from_slice(&[0; 4]);
        out.extend_from_slice(kind);
        out.extend_from_slice(&[0; 12]);
        out.extend_from_slice(name.as_bytes());
        out.push(0);
    });
}

/// Where the samples are: in this file (flag 1), not another one.
fn write_dinf(out: &mut Vec<u8>) {
    write_box(out, b"dinf", |out| {
        write_full_box(out, b"dref", 0, 0, |out| {
            out.extend_from_slice(&1_u32.to_be_bytes());
            write_full_box(out, b"url ", 0, 1, |_| {});
        });
    });
}

/// How H.264 pictures are decoded: their size, and the parameter sets
/// the decoder needs before the first picture.
fn write_avc1(out: &mut Vec<u8>, format: &VideoFormat) {
    write_box(out, b"avc1", |out| {
        out.extend_from_slice(&[0; 6]);
        out.extend_from_slice(&1_u16.to_be_bytes()); // the first `dref` entry
        out.extend_from_slice(&[0; 16]);
        out.extend_from_slice(&(format.width as u16).to_be_bytes());
        out.extend_from_slice(&(format.height as u16).to_be_bytes());
        out.extend_from_slice(&0x0048_0000_u32.to_be_bytes()); // 72 dpi
        out.extend_from_slice(&0x0048_0000_u32.to_be_bytes());
        out.extend_from_slice(&[0; 4]);
        out.extend_from_slice(&1_u16.to_be_bytes()); // one picture per sample
        out.extend_from_slice(&[0; 32]); // no compressor name
        out.extend_from_slice(&0x0018_u16.to_be_bytes()); // colour, no alpha
        out.extend_from_slice(&(-1_i16).to_be_bytes());
        write_box(out, b"avcC", |out| {
            out.push(1);
            // The profile, its compatibility flags and the level, as the
            // sequence parameter set says.
            out.extend_from_slice(&format.sps[1..4]);
            // 6 bits set, then 3: each piece of a sample starts with its
            // length in 4 bytes.
            out.push(0xFF);
            out.push(0xE1); // 3 bits set, then one sequence parameter set
            out.extend_from_slice(&(format.sps.len() as u16).to_be_bytes());
            out.extend_from_slice(&format.sps);
            out.push(1);
            out.extend_from_slice(&(format.pps.len() as u16).to_be_bytes());
            out.extend_from_slice(&format.pps);
        });
        // The colours, as the sequence parameter set also says them, for
        // players that only look here: sRGB's primaries and transfer
        // (the same as HD video's, BT.709), BT.601's matrix, which is how
        // the pictures were turned from RGB, and the video range.
        write_box(out, b"colr", |out| {
            out.extend_from_slice(b"nclx");
            out.extend_from_slice(&1_u16.to_be_bytes());
            out.extend_from_slice(&1_u16.to_be_bytes());
            out.extend_from_slice(&6_u16.to_be_bytes());
            out.push(0);
        });
    });
}

/// How the sound is decoded: MP3 (MPEG-1 audio, object type 0x6B), with
/// its rate and channels.
fn write_mp4a(out: &mut Vec<u8>, format: &AudioFormat) {
    write_box(out, b"mp4a", |out| {
        out.extend_from_slice(&[0; 6]);
        out.extend_from_slice(&1_u16.to_be_bytes());
        out.extend_from_slice(&[0; 8]);
        out.extend_from_slice(&format.channels.to_be_bytes());
        out.extend_from_slice(&16_u16.to_be_bytes());
        out.extend_from_slice(&[0; 4]);
        out.extend_from_slice(&(format.sample_rate << 16).to_be_bytes());
        // The elementary stream descriptor, in MPEG-4 Systems' own
        // tag-length-contents form.
        write_full_box(out, b"esds", 0, 0, |out| {
            let mut config = vec![0x6B, (0x05 << 2) | 1];
            config.extend_from_slice(&[0; 3]); // buffer size: unknown
            config.extend_from_slice(&format.bitrate.to_be_bytes()); // highest
            config.extend_from_slice(&format.bitrate.to_be_bytes()); // average
            let mut stream = vec![0, 0, 0]; // stream number, flags
            descriptor(&mut stream, 0x04, &config);
            descriptor(&mut stream, 0x06, &[0x02]); // as MP4 files have it
            descriptor(out, 0x03, &stream);
        });
    });
}

/// An MPEG-4 descriptor: a tag, the length, the contents.
fn descriptor(out: &mut Vec<u8>, tag: u8, contents: &[u8]) {
    out.push(tag);
    out.push(contents.len() as u8);
    out.extend_from_slice(contents);
}

/// The tables listing a track's samples.
fn write_tables(out: &mut Vec<u8>, track: &Track, pictures: bool) {
    // How long the samples last, as runs of samples of the same length.
    let mut runs: Vec<(u32, u32)> = Vec::new();
    for &duration in &track.durations {
        match runs.last_mut() {
            Some((count, last)) if *last == duration => *count += 1,
            _ => runs.push((1, duration)),
        }
    }
    write_full_box(out, b"stts", 0, 0, |out| {
        out.extend_from_slice(&(runs.len() as u32).to_be_bytes());
        for (count, duration) in runs {
            out.extend_from_slice(&count.to_be_bytes());
            out.extend_from_slice(&duration.to_be_bytes());
        }
    });
    if pictures {
        write_full_box(out, b"stss", 0, 0, |out| {
            out.extend_from_slice(&(track.sync.len() as u32).to_be_bytes());
            for number in &track.sync {
                out.extend_from_slice(&number.to_be_bytes());
            }
        });
    }
    // Every sample is a "chunk" of its own: one entry says so for all.
    write_full_box(out, b"stsc", 0, 0, |out| {
        out.extend_from_slice(&1_u32.to_be_bytes());
        for value in [1_u32, 1, 1] {
            out.extend_from_slice(&value.to_be_bytes());
        }
    });
    write_full_box(out, b"stsz", 0, 0, |out| {
        out.extend_from_slice(&0_u32.to_be_bytes()); // sizes differ
        out.extend_from_slice(&(track.sizes.len() as u32).to_be_bytes());
        for size in &track.sizes {
            out.extend_from_slice(&size.to_be_bytes());
        }
    });
    // Where each sample starts, in 32 bits while the file is small
    // enough, which every player reads, or else in 64.
    let small = track
        .offsets
        .iter()
        .all(|&offset| offset <= u64::from(u32::MAX));
    write_full_box(out, if small { b"stco" } else { b"co64" }, 0, 0, |out| {
        out.extend_from_slice(&(track.offsets.len() as u32).to_be_bytes());
        for &offset in &track.offsets {
            if small {
                out.extend_from_slice(&(offset as u32).to_be_bytes());
            } else {
                out.extend_from_slice(&offset.to_be_bytes());
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The boxes in `bytes`, by name, with their contents.
    fn boxes(bytes: &[u8]) -> Vec<(String, &[u8])> {
        let mut found = Vec::new();
        let mut at = 0;
        while at + 8 <= bytes.len() {
            let size = u32::from_be_bytes(bytes[at..at + 4].try_into().unwrap()) as usize;
            let name = String::from_utf8_lossy(&bytes[at + 4..at + 8]).into_owned();
            found.push((name, &bytes[at + 8..at + size]));
            at += size;
        }
        found
    }

    #[test]
    fn a_box_starts_with_its_size_and_name() {
        let mut out = Vec::new();
        write_box(&mut out, b"test", |out| out.extend_from_slice(&[1, 2, 3]));
        assert_eq!(out, [0, 0, 0, 11, b't', b'e', b's', b't', 1, 2, 3]);
    }

    #[test]
    fn the_language_is_undetermined() {
        let mut out = Vec::new();
        write_mdhd(&mut out, 1000, 0);
        // 'u' 'n' 'd' as 5-bit letters: 0x55C4, as every "und" file has.
        assert_eq!(&out[28..30], &[0x55, 0xC4]);
    }

    #[test]
    fn samples_of_the_same_length_are_listed_as_runs() {
        let track = Track {
            sizes: vec![10, 20, 30, 40],
            offsets: vec![48, 58, 78, 108],
            durations: vec![3000, 3000, 6000, 3000],
            sync: vec![1],
        };
        let mut out = Vec::new();
        write_tables(&mut out, &track, true);
        let tables = boxes(&out);
        let names: Vec<&str> = tables.iter().map(|(name, _)| name.as_str()).collect();
        assert_eq!(names, ["stts", "stss", "stsc", "stsz", "stco"]);
        let stts = tables[0].1;
        let words: Vec<u32> = stts[4..]
            .chunks(4)
            .map(|word| u32::from_be_bytes(word.try_into().unwrap()))
            .collect();
        assert_eq!(words, [3, 2, 3000, 1, 6000, 1, 3000]);
    }

    #[test]
    fn writes_a_movie_the_tables_describe() {
        let path = std::env::temp_dir().join(format!("lineup-mp4-{}.mp4", std::process::id()));
        let mut writer = Mp4Writer::create(File::create(&path).unwrap()).unwrap();
        writer.add_video(&[1; 100], 3000, true).unwrap();
        writer.add_audio(&[2; 50], 1152).unwrap();
        writer.add_video(&[3; 10], 3000, false).unwrap();
        writer.extend_last_video(3000);
        let video = VideoFormat {
            width: 64,
            height: 48,
            timescale: 90_000,
            sps: vec![0x67, 0x42, 0xC0, 0x1E, 0xAB],
            pps: vec![0x68, 0xCE],
        };
        let audio = AudioFormat {
            sample_rate: 44_100,
            channels: 2,
            bitrate: 256_000,
            priming: 1105,
            frames: 47,
        };
        writer.finish(&video, &audio).unwrap();
        let bytes = std::fs::read(&path).unwrap();
        std::fs::remove_file(&path).ok();

        // The mdat box has a 64-bit size, which `boxes` does not read, so
        // it is checked by hand.
        assert_eq!(&bytes[4..8], b"ftyp");
        let mdat = 32;
        assert_eq!(&bytes[mdat + 4..mdat + 8], b"mdat");
        let mdat_size = u64::from_be_bytes(bytes[mdat + 8..mdat + 16].try_into().unwrap());
        assert_eq!(mdat_size, 16 + 160);
        let moov = &bytes[mdat + mdat_size as usize..];
        assert_eq!(&moov[4..8], b"moov");
        // The second picture starts after the first and the sound.
        let samples = &bytes[mdat + 16..mdat + mdat_size as usize];
        assert_eq!(samples[0], 1);
        assert_eq!(samples[100], 2);
        assert_eq!(samples[150], 3);
        // 9000 ticks of a 90 kHz clock, and 47 frames at 44.1 kHz: the
        // movie lasts as long as its longer track, 100 ms.
        let mvhd = boxes(&moov[8..]);
        assert_eq!(mvhd[0].0, "mvhd");
        let duration = u32::from_be_bytes(mvhd[0].1[16..20].try_into().unwrap());
        assert_eq!(duration, 100);
    }
}
