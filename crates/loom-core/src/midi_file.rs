//! Standard MIDI Files (`.mid`): patterns and songs that any other studio
//! can open.
//!
//! A MIDI file holds notes, not sound: it says "C4 starts here, ends
//! there, on channel 1", and the program that opens it chooses the
//! instruments. That makes it the common language of music software, and
//! the way to take a Ferrous Loom pattern into another studio.
//!
//! The file is a header chunk (`MThd`) followed by one track chunk
//! (`MTrk`) per track. A track is a list of events, each one after a
//! *delta time*: the number of ticks since the event before it. This
//! module writes "format 1" files: the first track holds the tempo and the
//! time signature, and every channel of the project gets a track of its own.

use crate::arrangement::Play;
use crate::meter::{Meter, TimeMarker};
use crate::midi::{DRUM_CHANNEL, MidiMessage};
use crate::project::{Channel, ChannelKind, Drum, Project, SampleSource};
use crate::{Note, STEPS_PER_BEAT};

/// How many ticks one beat (a quarter note) is divided into. 96 is a
/// common choice: it divides by 2, 3, 4, 8, 16 and 32.
pub const TICKS_PER_BEAT: u16 = 96;

/// How many ticks one step (a sixteenth note) lasts: 24.
pub const TICKS_PER_STEP: u32 = TICKS_PER_BEAT as u32 / STEPS_PER_BEAT;

/// The whole song, as the playlist plays it: every clip on a track that
/// is not muted, every channel that is not muted, with every repeat of
/// clips and sections written out.
pub fn song_to_midi(project: &Project) -> Vec<u8> {
    let song = project.arrangement();
    let meter = project.played_meter(&song);
    write_file(project, &meter, &song.plays, song.length)
}

/// One pattern, from its first step, as pattern mode loops it.
pub fn pattern_to_midi(project: &Project, pattern: usize) -> Vec<u8> {
    let length = project
        .pattern(pattern)
        .map_or(0, |pattern| pattern.length());
    let play = Play {
        pattern,
        start: 0,
        skip: 0,
        length,
    };
    let meter = Meter::steady(project.pattern_time(pattern));
    write_file(project, &meter, &[play], length)
}

/// The note General MIDI drum sets play for each built-in drum.
pub fn general_midi_drum(drum: Drum) -> Note {
    Note::new(match drum {
        Drum::Kick => 36,
        Drum::Snare => 38,
        Drum::Clap => 39,
        Drum::HiHat => 42,
        Drum::OpenHat => 46,
        Drum::Rim => 37,
        Drum::LowTom => 45,
        Drum::HighTom => 50,
        Drum::Ride => 51,
        Drum::Crash => 49,
        Drum::Cowbell => 56,
        Drum::Shaker => 70,
        Drum::Conga => 63,
    })
}

/// Writes a file with the pieces of patterns `placed` for every channel,
/// lasting `length` steps, with bars counted in `meter`.
fn write_file(project: &Project, meter: &Meter, placed: &[Play], length: usize) -> Vec<u8> {
    let midi_channels = midi_channels(project.channels());
    let end = length as u32 * TICKS_PER_STEP;

    let mut tracks = vec![tempo_track(project, meter, end)];
    for (index, channel) in project.channels().iter().enumerate() {
        if channel.muted {
            continue;
        }
        let events = channel_events(project, index, midi_channels[index], placed);
        if events.is_empty() {
            continue;
        }
        let mut track = TrackWriter::default();
        track.meta(0, TRACK_NAME, channel.name.as_bytes());
        for (tick, message) in events {
            track.event(tick, &message.bytes());
        }
        tracks.push(track.finish(end));
    }

    let mut file = Vec::new();
    file.extend_from_slice(b"MThd");
    file.extend_from_slice(&6u32.to_be_bytes()); // the header's length
    file.extend_from_slice(&1u16.to_be_bytes()); // format 1
    file.extend_from_slice(&(tracks.len() as u16).to_be_bytes());
    file.extend_from_slice(&TICKS_PER_BEAT.to_be_bytes());
    for track in tracks {
        file.extend(track);
    }
    file
}

/// The MIDI channel (0 to 15) each channel of the project goes out on.
///
/// Built-in drums share channel 10, where General MIDI expects drums.
/// MIDI Out channels keep the channel they were given. Every other
/// channel gets the next free one, skipping 10; after 15 of them, the
/// channels are used again from the start.
fn midi_channels(channels: &[Channel]) -> Vec<u8> {
    let mut melodic = (0..16u8).filter(|&channel| channel != DRUM_CHANNEL).cycle();
    channels
        .iter()
        .map(|channel| match channel.kind {
            ChannelKind::Sampler(SampleSource::Builtin(_)) => DRUM_CHANNEL,
            ChannelKind::MidiOut { channel } => channel,
            _ => melodic.next().unwrap_or(0),
        })
        .collect()
}

/// Every note-on and note-off of one channel, in time order.
fn channel_events(
    project: &Project,
    channel: usize,
    midi_channel: u8,
    placed: &[Play],
) -> Vec<(u32, MidiMessage)> {
    let drum = match &project.channels()[channel].kind {
        ChannelKind::Sampler(SampleSource::Builtin(drum)) => Some(*drum),
        _ => None,
    };
    let mut events = Vec::new();
    for play in placed {
        let Some(pattern) = project.pattern(play.pattern) else {
            continue;
        };
        let length = pattern.length();
        for note in pattern
            .notes(channel)
            .iter()
            .filter(|note| note.start < length)
        {
            for song_step in play.song_steps_of(note.start, length) {
                // As in the audio thread: a note that runs past the end of
                // its pattern, or of the piece being played, is cut there.
                let held = note
                    .length
                    .min(length - note.start)
                    .min(play.end() - song_step);
                let on = song_step as u32 * TICKS_PER_STEP;
                let off = on + held as u32 * TICKS_PER_STEP;
                let pitch = drum.map_or(note.pitch, general_midi_drum);
                events.push((
                    on,
                    MidiMessage::NoteOn {
                        channel: midi_channel,
                        pitch,
                        velocity: note.velocity,
                    },
                ));
                events.push((
                    off,
                    MidiMessage::NoteOff {
                        channel: midi_channel,
                        pitch,
                    },
                ));
            }
        }
    }
    // At the same tick, notes end before new ones start, so a note
    // repeated straight away is heard twice, not cut off by its own end.
    events.sort_by_key(|&(tick, message)| (tick, matches!(message, MidiMessage::NoteOn { .. })));
    events
}

/// The first track: the song's name, tempo and time signatures.
fn tempo_track(project: &Project, meter: &Meter, end: u32) -> Vec<u8> {
    let mut track = TrackWriter::default();
    track.meta(0, TRACK_NAME, b"Ferrous Loom");
    // The tempo is written as microseconds per beat, in three bytes.
    let micros = (60_000_000.0 / f64::from(project.tempo.bpm())).round() as u32;
    let micros = micros.min(0xFF_FFFF).to_be_bytes();
    track.meta(0, TEMPO, &micros[1..]);
    // The time signature: beats to the bar, then the beat's note as a
    // power of two (2 for a quarter, 2^2 = 4; 3 for an eighth), then the
    // MIDI clocks per metronome click (24 per quarter note, 6 per step),
    // and 8 thirty-seconds per quarter. A song that changes time
    // signature gets one of these at every change.
    let first = TimeMarker::new(0, meter.first());
    for marker in std::iter::once(first).chain(meter.markers().iter().copied()) {
        let tick = marker.start as u32 * TICKS_PER_STEP;
        if tick > end {
            break;
        }
        let signature = marker.time;
        let unit_power = signature.unit.max(1).ilog2() as u8;
        let clocks = (signature.steps_per_beat() * 6).min(255) as u8;
        track.meta(
            tick,
            TIME_SIGNATURE,
            &[signature.beats, unit_power, clocks, 8],
        );
    }
    track.finish(end)
}

/// Meta events: information for the reader, not notes.
const TRACK_NAME: u8 = 0x03;
const END_OF_TRACK: u8 = 0x2F;
const TEMPO: u8 = 0x51;
const TIME_SIGNATURE: u8 = 0x58;

/// Builds one track chunk, keeping the time of the last event so each
/// new one can be written as a delta.
#[derive(Default)]
struct TrackWriter {
    data: Vec<u8>,
    last_tick: u32,
}

impl TrackWriter {
    /// Writes the time since the last event. Events must come in order.
    fn delta(&mut self, tick: u32) {
        let delta = tick.saturating_sub(self.last_tick);
        write_variable_length(&mut self.data, delta);
        self.last_tick = self.last_tick.max(tick);
    }

    fn event(&mut self, tick: u32, bytes: &[u8]) {
        self.delta(tick);
        self.data.extend_from_slice(bytes);
    }

    fn meta(&mut self, tick: u32, kind: u8, bytes: &[u8]) {
        self.delta(tick);
        self.data.extend_from_slice(&[0xFF, kind]);
        write_variable_length(&mut self.data, bytes.len() as u32);
        self.data.extend_from_slice(bytes);
    }

    /// Ends the track at `end` (or at its last event, if that is later)
    /// and wraps it in its chunk header.
    fn finish(mut self, end: u32) -> Vec<u8> {
        self.meta(end.max(self.last_tick), END_OF_TRACK, &[]);
        let mut chunk = Vec::with_capacity(8 + self.data.len());
        chunk.extend_from_slice(b"MTrk");
        chunk.extend_from_slice(&(self.data.len() as u32).to_be_bytes());
        chunk.extend(self.data);
        chunk
    }
}

/// Writes a number in MIDI's "variable-length quantity" format: seven
/// bits per byte, the most significant first, with the top bit set on
/// every byte except the last. Small numbers, the most common delta
/// times, take a single byte.
fn write_variable_length(out: &mut Vec<u8>, value: u32) {
    let mut groups = [0u8; 5];
    let mut count = 0;
    let mut rest = value;
    loop {
        groups[count] = (rest & 0x7F) as u8;
        count += 1;
        rest >>= 7;
        if rest == 0 {
            break;
        }
    }
    for index in (0..count).rev() {
        let more = if index > 0 { 0x80 } else { 0 };
        out.push(groups[index] | more);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::playlist::{Clip, Section};
    use crate::{NoteEvent, Tempo};

    fn variable_length(value: u32) -> Vec<u8> {
        let mut out = Vec::new();
        write_variable_length(&mut out, value);
        out
    }

    #[test]
    fn variable_length_numbers() {
        // The examples from the MIDI file specification.
        assert_eq!(variable_length(0), [0x00]);
        assert_eq!(variable_length(0x40), [0x40]);
        assert_eq!(variable_length(0x7F), [0x7F]);
        assert_eq!(variable_length(0x80), [0x81, 0x00]);
        assert_eq!(variable_length(0x2000), [0xC0, 0x00]);
        assert_eq!(variable_length(0x3FFF), [0xFF, 0x7F]);
        assert_eq!(variable_length(0x4000), [0x81, 0x80, 0x00]);
        assert_eq!(variable_length(0x0FFF_FFFF), [0xFF, 0xFF, 0xFF, 0x7F]);
    }

    /// Reads a file back into tracks of (tick, bytes) events, the way any
    /// MIDI reader would, to check what was written.
    fn read(file: &[u8]) -> Vec<Vec<(u32, Vec<u8>)>> {
        assert_eq!(&file[..4], b"MThd");
        assert_eq!(&file[8..10], [0, 1], "format 1");
        let count = u16::from_be_bytes([file[10], file[11]]) as usize;
        assert_eq!(u16::from_be_bytes([file[12], file[13]]), TICKS_PER_BEAT);
        let mut at = 14;
        let mut tracks = Vec::new();
        for _ in 0..count {
            assert_eq!(&file[at..at + 4], b"MTrk");
            let length = u32::from_be_bytes(file[at + 4..at + 8].try_into().unwrap()) as usize;
            let data = &file[at + 8..at + 8 + length];
            at += 8 + length;
            let mut events = Vec::new();
            let (mut i, mut tick) = (0, 0);
            while i < data.len() {
                let mut delta = 0;
                loop {
                    let byte = data[i];
                    i += 1;
                    delta = (delta << 7) | u32::from(byte & 0x7F);
                    if byte & 0x80 == 0 {
                        break;
                    }
                }
                tick += delta;
                let size = if data[i] == 0xFF {
                    3 + data[i + 2] as usize // FF, kind, length (one byte here), data
                } else {
                    3
                };
                events.push((tick, data[i..i + size].to_vec()));
                i += size;
            }
            tracks.push(events);
        }
        assert_eq!(at, file.len(), "nothing after the last track");
        tracks
    }

    /// A project with a Loom Osc and a kick, at 120 BPM, and one bar-long
    /// pattern.
    fn project() -> Project {
        let mut project = Project::empty();
        project.tempo = Tempo::new(120.0);
        project.add_channel(Channel::synth());
        project.add_channel(Channel::drum(Drum::Kick));
        let pattern = project.pattern_mut(0).unwrap();
        pattern.add_note(0, NoteEvent::new(Note::C4, 0, 4));
        pattern.add_note(0, NoteEvent::new(Note::A4, 14, 8)); // runs past the end
        pattern.add_note(1, NoteEvent::step(0));
        pattern.add_note(1, NoteEvent::step(8));
        project
    }

    #[test]
    fn a_pattern_becomes_three_tracks() {
        let tracks = read(&pattern_to_midi(&project(), 0));
        assert_eq!(tracks.len(), 3, "tempo, synth, kick");

        // 120 BPM is 500,000 microseconds per beat: 07 A1 20.
        assert!(tracks[0].contains(&(0, vec![0xFF, 0x51, 3, 0x07, 0xA1, 0x20])));
        // Every track ends where the pattern does: 16 steps of 24 ticks.
        for track in &tracks {
            assert_eq!(track.last(), Some(&(16 * 24, vec![0xFF, 0x2F, 0])));
        }

        let synth: Vec<_> = tracks[1].iter().filter(|(_, e)| e[0] != 0xFF).collect();
        assert_eq!(
            synth,
            [
                &(0, vec![0x90, 60, 100]),
                &(96, vec![0x80, 60, 64]),
                &(14 * 24, vec![0x90, 69, 100]),
                // Cut at the end of the pattern, as it is played.
                &(16 * 24, vec![0x80, 69, 64]),
            ]
        );
    }

    #[test]
    fn the_time_signature_is_written() {
        let mut project = project();
        let tracks = read(&pattern_to_midi(&project, 0));
        assert!(
            tracks[0].contains(&(0, vec![0xFF, 0x58, 4, 4, 2, 24, 8])),
            "4/4"
        );
        // 6/8: six eighths (2^3), a click every dotted quarter (36 clocks).
        project.time_signature = crate::TimeSignature::new(6, 8);
        let tracks = read(&pattern_to_midi(&project, 0));
        assert!(
            tracks[0].contains(&(0, vec![0xFF, 0x58, 4, 6, 3, 36, 8])),
            "6/8"
        );
    }

    #[test]
    fn every_change_of_time_signature_is_written() {
        let mut project = project();
        project.add_clip(crate::Clip::new(0, 0, 0).repeated(4));
        project.set_time_marker(TimeMarker::new(32, crate::TimeSignature::new(3, 4)));
        let tracks = read(&song_to_midi(&project));
        let signatures: Vec<&(u32, Vec<u8>)> = tracks[0]
            .iter()
            .filter(|(_, event)| event[..2] == [0xFF, 0x58])
            .collect();
        assert_eq!(
            signatures,
            [
                &(0, vec![0xFF, 0x58, 4, 4, 2, 24, 8]),
                &(32 * 24, vec![0xFF, 0x58, 4, 3, 2, 24, 8])
            ]
        );
    }

    #[test]
    fn drums_use_channel_ten_and_general_midi_notes() {
        let tracks = read(&pattern_to_midi(&project(), 0));
        let kicks: Vec<_> = tracks[2]
            .iter()
            .filter(|(_, e)| e[0] == 0x99)
            .map(|(tick, e)| (*tick, e[1]))
            .collect();
        assert_eq!(kicks, [(0, 36), (8 * 24, 36)]);
    }

    #[test]
    fn a_song_places_each_clip() {
        let mut project = project();
        project.add_clip(Clip::new(0, 0, 0));
        project.add_clip(Clip::new(0, 0, 32));
        let tracks = read(&song_to_midi(&project));
        let starts: Vec<u32> = tracks[1]
            .iter()
            .filter(|(_, e)| e[0] == 0x90 && e[1] == 60)
            .map(|(tick, _)| *tick)
            .collect();
        assert_eq!(starts, [0, 32 * 24]);

        // A muted channel is left out, as it is when the song plays.
        project.channel_mut(1).unwrap().muted = true;
        assert_eq!(read(&song_to_midi(&project)).len(), 2);
    }

    #[test]
    fn repeats_are_written_out() {
        let mut project = project();
        project.add_clip(Clip::new(0, 0, 0).repeated(2));
        let mut twice = Section::new("Twice", 0, 32);
        twice.repeats = 2;
        project.add_section(twice);
        let tracks = read(&song_to_midi(&project));
        let starts: Vec<u32> = tracks[1]
            .iter()
            .filter(|(_, e)| e[0] == 0x90 && e[1] == 60)
            .map(|(tick, _)| *tick)
            .collect();
        // The clip plays its pattern twice, and the section plays twice.
        assert_eq!(starts, [0, 16 * 24, 32 * 24, 48 * 24]);
    }

    #[test]
    fn a_repeated_note_ends_before_it_starts_again() {
        let mut project = Project::empty();
        project.add_channel(Channel::synth());
        let pattern = project.pattern_mut(0).unwrap();
        pattern.add_note(0, NoteEvent::new(Note::C4, 0, 2));
        pattern.add_note(0, NoteEvent::new(Note::C4, 2, 2));
        let tracks = read(&pattern_to_midi(&project, 0));
        let at_two_steps: Vec<u8> = tracks[1]
            .iter()
            .filter(|(tick, _)| *tick == 48)
            .map(|(_, e)| e[0])
            .collect();
        assert_eq!(at_two_steps, [0x80, 0x90]);
    }

    #[test]
    fn midi_out_channels_keep_their_channel() {
        let channels = [
            Channel::synth(),
            Channel::drum(Drum::Snare),
            Channel::midi_out(4),
            Channel::synth(),
        ];
        assert_eq!(midi_channels(&channels), [0, DRUM_CHANNEL, 4, 1]);
    }
}
