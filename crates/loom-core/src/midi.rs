//! MIDI messages: the bytes keyboards, synths and studios send each other.
//!
//! MIDI (Musical Instrument Digital Interface) is from 1983, and it is
//! still how every keyboard talks to every computer. A message is a
//! *status* byte, which says what kind of message it is and on which of
//! 16 channels, followed by one or two *data* bytes. Status bytes have
//! their top bit set (128 and up); data bytes do not (0 to 127). That is
//! why a MIDI note or velocity is never more than 127.
//!
//! Ferrous Loom only needs notes, so only note messages are understood.
//! Everything else (knobs, pedals, pitch bends, clock) is ignored.

use crate::Note;

/// The MIDI channel that General MIDI keeps for drums: channel 10, which
/// is 9 when counted from 0 as the bytes do.
pub const DRUM_CHANNEL: u8 = 9;

/// A note message. `channel` is 0 to 15 (shown to people as 1 to 16).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MidiMessage {
    NoteOn {
        channel: u8,
        pitch: Note,
        /// How hard the key was struck, 1 to 127.
        velocity: u8,
    },
    NoteOff {
        channel: u8,
        pitch: Note,
    },
}

/// The kinds of message, as the top four bits of the status byte.
const NOTE_OFF: u8 = 0x80;
const NOTE_ON: u8 = 0x90;
const CONTROL_CHANGE: u8 = 0xB0;

/// The bytes that tell every note on `channel` to stop: control change
/// 123, "All Notes Off". Sent on all 16 channels before a port is let go,
/// in case a note-off was lost on the way.
pub fn all_notes_off(channel: u8) -> [u8; 3] {
    [CONTROL_CHANGE | (channel & 0x0F), 123, 0]
}

impl MidiMessage {
    /// Reads a message from its bytes, or `None` for anything that is not
    /// a complete note message.
    ///
    /// A note-on with velocity 0 is a note-off. Many keyboards send that
    /// instead of a real note-off, because a stream of note-ons can leave
    /// out the repeated status byte ("running status") and so save time
    /// on the slow cable of 1983.
    pub fn parse(bytes: &[u8]) -> Option<MidiMessage> {
        let [status, pitch, velocity, ..] = *bytes else {
            return None;
        };
        // Data bytes have the top bit clear; anything else is malformed.
        if pitch > 127 || velocity > 127 {
            return None;
        }
        let channel = status & 0x0F;
        let pitch = Note::new(pitch);
        match status & 0xF0 {
            NOTE_ON if velocity > 0 => Some(MidiMessage::NoteOn {
                channel,
                pitch,
                velocity,
            }),
            NOTE_ON | NOTE_OFF => Some(MidiMessage::NoteOff { channel, pitch }),
            _ => None,
        }
    }

    /// The three bytes of the message, ready to send.
    pub fn bytes(self) -> [u8; 3] {
        match self {
            MidiMessage::NoteOn {
                channel,
                pitch,
                velocity,
            } => [
                NOTE_ON | (channel & 0x0F),
                pitch.midi(),
                velocity.clamp(1, 127),
            ],
            // Velocity 64 is the usual "no particular speed" for a release.
            MidiMessage::NoteOff { channel, pitch } => {
                [NOTE_OFF | (channel & 0x0F), pitch.midi(), 64]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_on_and_off() {
        assert_eq!(
            MidiMessage::parse(&[0x90, 60, 100]),
            Some(MidiMessage::NoteOn {
                channel: 0,
                pitch: Note::C4,
                velocity: 100
            })
        );
        assert_eq!(
            MidiMessage::parse(&[0x83, 69, 40]),
            Some(MidiMessage::NoteOff {
                channel: 3,
                pitch: Note::A4
            })
        );
    }

    #[test]
    fn a_silent_note_on_is_a_note_off() {
        assert_eq!(
            MidiMessage::parse(&[0x9F, 60, 0]),
            Some(MidiMessage::NoteOff {
                channel: 15,
                pitch: Note::C4
            })
        );
    }

    #[test]
    fn other_messages_are_ignored() {
        assert_eq!(MidiMessage::parse(&[0xB0, 64, 127]), None, "sustain pedal");
        assert_eq!(MidiMessage::parse(&[0xE0, 0, 64]), None, "pitch bend");
        assert_eq!(MidiMessage::parse(&[0xF8]), None, "clock");
        assert_eq!(MidiMessage::parse(&[0x90, 60]), None, "too short");
        assert_eq!(MidiMessage::parse(&[0x90, 200, 1]), None, "not a data byte");
    }

    #[test]
    fn bytes_round_trip() {
        for message in [
            MidiMessage::NoteOn {
                channel: 9,
                pitch: Note::new(36),
                velocity: 127,
            },
            MidiMessage::NoteOff {
                channel: 2,
                pitch: Note::new(72),
            },
        ] {
            assert_eq!(MidiMessage::parse(&message.bytes()), Some(message));
        }
        let off = MidiMessage::NoteOff {
            channel: 0,
            pitch: Note::C4,
        };
        assert_eq!(off.bytes(), [0x80, 60, 64]);
        assert_eq!(all_notes_off(15), [0xBF, 123, 0]);
    }
}
