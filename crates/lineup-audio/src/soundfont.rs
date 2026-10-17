//! Real instrument sounds from a SoundFont.
//!
//! A SoundFont (a `.sf2` file) is a bank of recorded instruments: a few
//! notes of a real piano, a real bass, real strings, each recorded and
//! cut into samples, with instructions for stretching them over the
//! whole keyboard. A *General MIDI* SoundFont has all 128 instruments of
//! the General MIDI list in the same order, plus a drum kit, so channel
//! program 0 is always a piano and program 33 a fingered bass.
//!
//! The synthesizer that plays a SoundFont is the `rustysynth` crate, a
//! SoundFont player written in plain Rust. It takes MIDI messages (note
//! on, note off, program change) and renders blocks of sound. That is
//! exactly what a [`MidiOut`] channel produces for an external synth, so
//! a [`SoundFontPlayer`] is a MIDI Out whose messages go to a synthesizer
//! inside the app instead of out of a MIDI port: the same bookkeeping of
//! which notes sound and when each one ends, and sound instead of
//! silence.
//!
//! Each channel that plays from the SoundFont gets a synthesizer of its
//! own, so its volume and its mute button work as on any other channel.
//! They all share one copy of the SoundFont: it is loaded once, and an
//! `Arc` hands the same memory to every synthesizer.

use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

use lineup_core::{MidiMessage, Note};
use rustysynth::{Synthesizer, SynthesizerSettings};

pub use rustysynth::SoundFont;

use crate::instrument::{MidiOut, Origin};

/// How many frames the synthesizer renders at a time. A note that starts
/// in the middle of a block starts with the next one, at most this many
/// frames late: 1.3 ms at 48 kHz.
const BLOCK: usize = 64;

/// How many notes one channel's synthesizer can play at once. One SoundFont
/// note can use two voices (a piano's left and right samples), so this is
/// sixteen chords of four notes.
const VOICES: usize = 128;

/// The MIDI channel for drums: channel 10, counted from 0. A General MIDI
/// synthesizer plays its drum kit there.
const DRUM_CHANNEL: u8 = 9;

/// How loud the synthesizer is turned up. Its own level is low, because a
/// whole band shares it; one channel's synthesizer plays one part, so it
/// is turned up to sound about as loud as the Lineup Osc.
const GAIN: f32 = 5.0;

/// The same for a drum, turned up to sound about as loud as the built-in
/// drums, which hit hard.
const DRUM_GAIN: f32 = 7.0;

/// Why a SoundFont could not be read.
#[derive(Debug)]
pub enum SoundFontError {
    Io(std::io::Error),
    /// The file is not a SoundFont, or it is damaged.
    Invalid(String),
}

impl fmt::Display for SoundFontError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SoundFontError::Io(err) => write!(f, "could not read the SoundFont: {err}"),
            SoundFontError::Invalid(why) => write!(f, "not a SoundFont file ({why})"),
        }
    }
}

impl std::error::Error for SoundFontError {}

/// Reads a SoundFont file. A General MIDI SoundFont is tens of megabytes,
/// so this takes a moment: call it away from the UI thread.
pub fn load(path: &Path) -> Result<Arc<SoundFont>, SoundFontError> {
    let file = File::open(path).map_err(SoundFontError::Io)?;
    let mut reader = BufReader::new(file);
    SoundFont::new(&mut reader)
        .map(Arc::new)
        .map_err(|err| SoundFontError::Invalid(err.to_string()))
}

/// A channel's instrument played from a SoundFont.
pub struct SoundFontPlayer {
    /// Which notes sound and until when, as for an external synth; its
    /// messages go to the synthesizer once per block.
    notes: MidiOut,
    /// Boxed so an `Instrument` stays small.
    block: Box<Block>,
}

/// The synthesizer and the block of sound it rendered last.
struct Block {
    synthesizer: Synthesizer,
    sound_font: Arc<SoundFont>,
    /// [`GAIN`] or [`DRUM_GAIN`].
    gain: f32,
    left: [f32; BLOCK],
    right: [f32; BLOCK],
    /// The next frame of `left` and `right` to hand out. At `BLOCK`, a
    /// new block is due.
    read: usize,
}

impl SoundFontPlayer {
    /// A player for General MIDI program `program` (0 to 127): a piano, a
    /// bass, strings. Called on the UI thread, since the synthesizer's
    /// memory is reserved here. Fails only for a sample rate the
    /// synthesizer does not support (below 16 kHz or above 192 kHz).
    pub fn new(sound_font: &Arc<SoundFont>, sample_rate: u32, program: u8) -> Option<Self> {
        let notes = MidiOut::new(0).with_program(Some(program));
        Self::on(sound_font, sample_rate, notes, GAIN)
    }

    /// A player for one drum of the General MIDI drum kit: every note
    /// plays `drum`, such as note 36 for the kick, whatever note it is
    /// written on, as the built-in drums do.
    pub fn drum(sound_font: &Arc<SoundFont>, sample_rate: u32, drum: Note) -> Option<Self> {
        let notes = MidiOut::new(DRUM_CHANNEL).playing_as(Some(drum));
        Self::on(sound_font, sample_rate, notes, DRUM_GAIN)
    }

    fn on(
        sound_font: &Arc<SoundFont>,
        sample_rate: u32,
        notes: MidiOut,
        gain: f32,
    ) -> Option<Self> {
        let mut settings = SynthesizerSettings::new(i32::try_from(sample_rate).ok()?);
        settings.block_size = BLOCK;
        settings.maximum_polyphony = VOICES;
        // Reverb and chorus would cost as much as the voices, once for
        // every channel. The sounds are recorded with some room already.
        settings.enable_reverb_and_chorus = false;
        let synthesizer = Synthesizer::new(sound_font, &settings).ok()?;
        Some(SoundFontPlayer {
            notes,
            block: Box::new(Block {
                synthesizer,
                sound_font: Arc::clone(sound_font),
                gain,
                left: [0.0; BLOCK],
                right: [0.0; BLOCK],
                read: BLOCK,
            }),
        })
    }

    pub(crate) fn start(&mut self, pitch: Note, velocity: u8, gate: u32, origin: Option<Origin>) {
        self.notes.start(pitch, velocity, gate, origin);
    }

    pub fn note_off(&mut self, pitch: Note) {
        self.notes.note_off(pitch);
    }

    pub fn release_all(&mut self) {
        self.notes.release_all();
    }

    pub fn release_unless(&mut self, keep: impl Fn(Origin) -> bool) {
        self.notes.release_unless(keep);
    }

    pub fn tempo_changed(&mut self, next_step: f64, stretch: f64) {
        self.notes.tempo_changed(next_step, stretch);
    }

    /// The next output sample. At the start of each block, the notes
    /// that started or ended since the last one are passed on, and the
    /// synthesizer renders the block. Its two sides are mixed into one,
    /// as every channel is mono.
    pub fn next_sample(&mut self) -> f32 {
        self.notes.next_sample();
        let block = &mut *self.block;
        if block.read == BLOCK {
            let synthesizer = &mut block.synthesizer;
            self.notes.flush(|message| play(synthesizer, message));
            synthesizer.render(&mut block.left, &mut block.right);
            block.read = 0;
        }
        let sample = (block.left[block.read] + block.right[block.read]) * 0.5 * block.gain;
        block.read += 1;
        sample
    }

    pub(crate) fn notes_sounding(&self) -> usize {
        self.notes.notes_sounding()
    }
}

/// Hands one MIDI message to the synthesizer.
fn play(synthesizer: &mut Synthesizer, message: MidiMessage) {
    match message {
        MidiMessage::NoteOn {
            channel,
            pitch,
            velocity,
        } => synthesizer.note_on(
            i32::from(channel),
            i32::from(pitch.midi()),
            i32::from(velocity),
        ),
        MidiMessage::NoteOff { channel, pitch } => {
            synthesizer.note_off(i32::from(channel), i32::from(pitch.midi()))
        }
        MidiMessage::ProgramChange { channel, program } => {
            synthesizer.process_midi_message(i32::from(channel), 0xC0, i32::from(program), 0)
        }
    }
}

/// A copy starts silent, with a synthesizer of its own on the same
/// SoundFont. Only tests copy instruments.
impl Clone for SoundFontPlayer {
    fn clone(&self) -> Self {
        let block = &self.block;
        let sample_rate = block.synthesizer.get_sample_rate() as u32;
        SoundFontPlayer::on(
            &block.sound_font,
            sample_rate,
            self.notes.clone(),
            block.gain,
        )
        .expect("the sample rate worked once")
    }
}

/// The synthesizer would print the whole SoundFont.
impl fmt::Debug for SoundFontPlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SoundFontPlayer")
            .field("notes", &self.notes)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instrument::{HELD, Instrument};
    use lineup_core::{Channel, ChannelKind, Drum, SampleSource};

    const RATE: u32 = 48_000;

    /// The pitch of the test SoundFont's one sample: A4, 440 Hz.
    const A4: u8 = 69;

    /// One chunk of a RIFF file: its name, its size and its bytes, padded
    /// to an even length.
    fn chunk(id: &[u8; 4], body: &[u8]) -> Vec<u8> {
        let mut bytes = id.to_vec();
        bytes.extend((body.len() as u32).to_le_bytes());
        bytes.extend(body);
        if body.len() % 2 == 1 {
            bytes.push(0);
        }
        bytes
    }

    /// A list of chunks: a chunk whose bytes are a type and other chunks.
    fn list(kind: &[u8; 4], chunks: &[Vec<u8>]) -> Vec<u8> {
        let mut body = kind.to_vec();
        for each in chunks {
            body.extend(each);
        }
        chunk(b"LIST", &body)
    }

    fn name(text: &str, length: usize) -> Vec<u8> {
        let mut bytes = text.as_bytes().to_vec();
        bytes.resize(length, 0);
        bytes
    }

    fn u16s(values: &[u16]) -> Vec<u8> {
        values
            .iter()
            .flat_map(|value| value.to_le_bytes())
            .collect()
    }

    /// Generators: (number, amount) pairs, ending with the terminator.
    fn generators(list: &[(u16, u16)]) -> Vec<u8> {
        let mut values: Vec<u16> = list
            .iter()
            .flat_map(|&(kind, amount)| [kind, amount])
            .collect();
        values.extend([0, 0]);
        u16s(&values)
    }

    /// Bags: the index of each zone's first generator, ending with the
    /// terminator. No zone has modulators.
    fn bags(starts: &[u16]) -> Vec<u8> {
        u16s(
            &starts
                .iter()
                .flat_map(|&start| [start, 0])
                .collect::<Vec<_>>(),
        )
    }

    const INSTRUMENT: u16 = 41;
    const SAMPLE_ID: u16 = 53;
    const SAMPLE_MODES: u16 = 54;
    const OVERRIDING_ROOT_KEY: u16 = 58;

    /// A tiny SoundFont, written byte by byte so the tests need no file:
    /// one sample, a sine wave at 440 Hz that loops, and two instruments
    /// that play it, one as recorded and one an octave higher. Program 0
    /// plays the first, program 33 the second, and the drum kit (bank
    /// 128) the first on every key, so a drum's key sets its pitch.
    fn tiny_sound_font() -> Arc<SoundFont> {
        const SAMPLE_RATE: u32 = 44_000; // 100 frames per period of 440 Hz
        const LENGTH: u32 = 4_400;
        let mut wave: Vec<u8> = (0..LENGTH)
            .flat_map(|frame| {
                let phase = std::f32::consts::TAU * frame as f32 / 100.0;
                ((phase.sin() * 16_000.0) as i16).to_le_bytes()
            })
            .collect();
        // A SoundFont leaves 46 frames of silence after each sample.
        wave.extend([0; 2 * 46]);

        let mut sample = name("Sine", 20);
        for value in [0, LENGTH, 100, LENGTH - 100, SAMPLE_RATE] {
            sample.extend(value.to_le_bytes());
        }
        sample.extend([A4, 0, 0, 0, 1, 0]); // pitch, correction, link, mono
        sample.extend(name("EOS", 46));

        let mut presets = Vec::new();
        for (preset, program, bank, bag) in [
            ("Sine", 0u16, 0u16, 0u16),
            ("Sine up", 33, 0, 1),
            ("Drums", 0, 128, 2),
            ("EOP", 0, 0, 3),
        ] {
            presets.extend(name(preset, 20));
            presets.extend(u16s(&[program, bank, bag]));
            presets.extend([0; 12]);
        }
        let mut instruments = Vec::new();
        for (instrument, bag) in [("Sine", 0u16), ("Sine up", 1), ("EOI", 2)] {
            instruments.extend(name(instrument, 20));
            instruments.extend(u16s(&[bag]));
        }
        let no_modulators = vec![0; 10];

        let file = list(
            b"sfbk",
            &[
                list(
                    b"INFO",
                    &[
                        chunk(b"ifil", &u16s(&[2, 1])),
                        chunk(b"INAM", &name("Test", 6)),
                    ],
                ),
                list(b"sdta", &[chunk(b"smpl", &wave)]),
                list(
                    b"pdta",
                    &[
                        chunk(b"phdr", &presets),
                        chunk(b"pbag", &bags(&[0, 1, 2, 3])),
                        chunk(b"pmod", &no_modulators),
                        chunk(
                            b"pgen",
                            &generators(&[(INSTRUMENT, 0), (INSTRUMENT, 1), (INSTRUMENT, 0)]),
                        ),
                        chunk(b"inst", &instruments),
                        chunk(b"ibag", &bags(&[0, 2, 5])),
                        chunk(b"imod", &no_modulators),
                        chunk(
                            b"igen",
                            &generators(&[
                                (SAMPLE_MODES, 1),
                                (SAMPLE_ID, 0),
                                (SAMPLE_MODES, 1),
                                (OVERRIDING_ROOT_KEY, u16::from(A4 - 12)),
                                (SAMPLE_ID, 0),
                            ]),
                        ),
                        chunk(b"shdr", &sample),
                    ],
                ),
            ],
        );
        // A RIFF file is one chunk called RIFF, not LIST.
        let mut riff = b"RIFF".to_vec();
        riff.extend(&file[4..]);
        Arc::new(SoundFont::new(&mut riff.as_slice()).expect("the test SoundFont is valid"))
    }

    fn render(player: &mut SoundFontPlayer, frames: usize) -> Vec<f32> {
        (0..frames).map(|_| player.next_sample()).collect()
    }

    /// How many times the wave crosses zero, which for a sine wave of
    /// one second is twice its frequency.
    fn crossings(samples: &[f32]) -> usize {
        samples
            .windows(2)
            .filter(|pair| (pair[0] < 0.0) != (pair[1] < 0.0))
            .count()
    }

    /// The frequency of a sine wave, from a quarter of a second of it.
    fn frequency(samples: &[f32]) -> f32 {
        let quarter = &samples[samples.len() - RATE as usize / 4..];
        crossings(quarter) as f32 * 2.0
    }

    fn peak(samples: &[f32]) -> f32 {
        samples
            .iter()
            .fold(0.0, |peak, sample| peak.max(sample.abs()))
    }

    #[test]
    fn silent_until_a_note_starts() {
        let mut player = SoundFontPlayer::new(&tiny_sound_font(), RATE, 0).unwrap();
        assert_eq!(peak(&render(&mut player, 1000)), 0.0);
    }

    #[test]
    fn plays_the_note_at_its_pitch() {
        let mut player = SoundFontPlayer::new(&tiny_sound_font(), RATE, 0).unwrap();
        player.start(Note::new(A4), 100, HELD, None);
        let sound = render(&mut player, RATE as usize / 2);
        assert!(
            (frequency(&sound) - 440.0).abs() <= 8.0,
            "{}",
            frequency(&sound)
        );

        let mut player = SoundFontPlayer::new(&tiny_sound_font(), RATE, 0).unwrap();
        player.start(Note::new(A4 - 12), 100, HELD, None);
        let sound = render(&mut player, RATE as usize / 2);
        assert!(
            (frequency(&sound) - 220.0).abs() <= 8.0,
            "{}",
            frequency(&sound)
        );
    }

    #[test]
    fn the_program_picks_the_instrument() {
        let mut player = SoundFontPlayer::new(&tiny_sound_font(), RATE, 33).unwrap();
        player.start(Note::new(A4), 100, HELD, None);
        let sound = render(&mut player, RATE as usize / 2);
        assert!(
            (frequency(&sound) - 880.0).abs() <= 8.0,
            "{}",
            frequency(&sound)
        );
    }

    #[test]
    fn a_note_starts_within_one_block() {
        let mut player = SoundFontPlayer::new(&tiny_sound_font(), RATE, 0).unwrap();
        render(&mut player, 10);
        player.start(Note::new(A4), 100, HELD, None);
        let sound = render(&mut player, BLOCK + 50);
        let first = sound.iter().position(|sample| *sample != 0.0).unwrap();
        assert!(first <= BLOCK, "{first}");
    }

    #[test]
    fn a_note_ends_after_its_gate() {
        let mut player = SoundFontPlayer::new(&tiny_sound_font(), RATE, 0).unwrap();
        player.start(Note::new(A4), 100, RATE / 10, None);
        let sound = render(&mut player, RATE as usize / 2);
        assert!(peak(&sound[..RATE as usize / 10]) > 0.1);
        assert_eq!(peak(&sound[RATE as usize / 4..]), 0.0);
        assert_eq!(player.notes_sounding(), 0);
    }

    #[test]
    fn held_notes_end_with_note_off_and_release_all() {
        let mut player = SoundFontPlayer::new(&tiny_sound_font(), RATE, 0).unwrap();
        player.start(Note::new(A4), 100, HELD, None);
        player.start(Note::new(A4 + 7), 100, HELD, None);
        render(&mut player, RATE as usize / 4);
        assert_eq!(player.notes_sounding(), 2);
        player.note_off(Note::new(A4));
        assert_eq!(player.notes_sounding(), 1);
        player.release_all();
        assert_eq!(player.notes_sounding(), 0);
        let sound = render(&mut player, RATE as usize / 2);
        assert_eq!(peak(&sound[RATE as usize / 4..]), 0.0);
    }

    #[test]
    fn louder_notes_are_louder() {
        let level = |velocity| {
            let mut player = SoundFontPlayer::new(&tiny_sound_font(), RATE, 0).unwrap();
            player.start(Note::new(A4), velocity, HELD, None);
            peak(&render(&mut player, RATE as usize / 10))
        };
        assert!(level(40) < level(100));
    }

    #[test]
    fn a_drum_plays_its_drum_whatever_note_it_is_on() {
        let kick = Note::new(A4 - 24);
        for written in [60, 72] {
            let mut player = SoundFontPlayer::drum(&tiny_sound_font(), RATE, kick).unwrap();
            player.start(Note::new(written), 100, RATE / 2, None);
            let sound = render(&mut player, RATE as usize / 2);
            assert!(
                (frequency(&sound) - 110.0).abs() <= 8.0,
                "{}",
                frequency(&sound)
            );
        }
    }

    #[test]
    fn a_copy_plays_on_its_own() {
        let mut player = SoundFontPlayer::new(&tiny_sound_font(), RATE, 0).unwrap();
        let mut copy = player.clone();
        player.start(Note::new(A4), 100, HELD, None);
        assert!(peak(&render(&mut player, 1000)) > 0.0);
        assert_eq!(peak(&render(&mut copy, 1000)), 0.0);
    }

    #[test]
    fn loading_a_file_that_is_not_a_sound_font_fails() {
        let missing = Path::new("/no/such/file.sf2");
        assert!(matches!(load(missing), Err(SoundFontError::Io(_))));
        let path = std::env::temp_dir().join("lineup-not-a-sound-font.sf2");
        std::fs::write(&path, b"RIFF\x04\x00\x00\x00WAVE").unwrap();
        assert!(matches!(load(&path), Err(SoundFontError::Invalid(_))));
        std::fs::remove_file(&path).unwrap();
    }

    fn channel(kind: ChannelKind, program: Option<u8>) -> Channel {
        Channel {
            name: "Test".to_string(),
            kind,
            volume: 1.0,
            muted: false,
            program,
        }
    }

    #[test]
    fn channels_with_a_program_play_it() {
        let sound_font = tiny_sound_font();
        let piano = channel(ChannelKind::Synth, Some(0));
        let instrument = Instrument::from_sound_font(&piano, RATE, &sound_font, false);
        assert!(matches!(instrument, Some(Instrument::SoundFont(_))));
    }

    #[test]
    fn channels_without_a_sound_in_the_sound_font_keep_theirs() {
        let sound_font = tiny_sound_font();
        let kick = ChannelKind::Sampler(SampleSource::Builtin(Drum::Kick));
        let file = ChannelKind::Sampler(SampleSource::File("kick.wav".into()));
        for (kind, program, drums) in [
            (ChannelKind::Synth, None, true),
            (kick.clone(), None, false),
            (file, None, true),
            (ChannelKind::MidiOut { channel: 2 }, Some(0), true),
        ] {
            let state = channel(kind, program);
            assert!(Instrument::from_sound_font(&state, RATE, &sound_font, drums).is_none());
        }
        let drum = channel(kick, None);
        let instrument = Instrument::from_sound_font(&drum, RATE, &sound_font, true);
        assert!(matches!(instrument, Some(Instrument::SoundFont(_))));
    }

    #[test]
    fn a_sound_font_channel_can_go_to_the_midi_port() {
        let sound_font = tiny_sound_font();
        let piano = channel(ChannelKind::Synth, Some(0));
        let instrument = Instrument::from_sound_font(&piano, RATE, &sound_font, false).unwrap();
        let mut sent = instrument.to_midi(4, None, Some(0));
        sent.note_on(Note::new(60), 100, HELD, RATE);
        let mut messages = Vec::new();
        sent.send_midi(|message| messages.push(message));
        assert_eq!(messages.len(), 2, "{messages:?}"); // the program, the note
        assert_eq!(sent.next_sample(), 0.0);
    }
}
