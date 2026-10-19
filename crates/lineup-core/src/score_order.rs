//! Score order: the order a conductor's score lists the instruments in,
//! top to bottom, which the mixer uses to list the channels.
//!
//! The channel rack keeps channels in the order they were added, so a
//! style that brings in a ride cymbal puts it after the piano and the
//! bass, far from the kick and the snare. A score never does that: every
//! instrument has its place, the same in every score, so a reader finds
//! the flutes at the top and the double basses at the bottom without
//! looking. The order comes from the orchestra, families from high
//! winds to low strings:
//!
//! 1. woodwinds, highest first: piccolo, flutes, oboes, clarinets,
//!    saxophones, bassoons;
//! 2. brass: horns, trumpets, trombones, tuba;
//! 3. percussion: timpani, then the tuned percussion (glockenspiel,
//!    xylophone, vibraphone, marimba) and the rest;
//! 4. the drums, kept together: hand percussion first, then the drum kit
//!    in the order the drum staff reads, cymbals at the top and the kick
//!    at the bottom;
//! 5. harp and guitars, with the bass guitar;
//! 6. keyboards: celesta, piano, harpsichord, organ, accordion;
//! 7. synthesizers, which scores put with the keyboards;
//! 8. voices: soloists above the choir;
//! 9. strings: violins, violas, cellos, double basses.
//!
//! This is only how channels are *shown*: their numbers, the patterns'
//! rows and the saved file stay as they are.

use crate::DRUM_CHANNEL;
use crate::project::{Channel, ChannelKind, Drum, SampleSource};

/// A family of instruments, a block of staves in a score.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Section {
    Woodwinds,
    Brass,
    Percussion,
    Drums,
    HarpAndGuitars,
    Keyboards,
    Synths,
    Voices,
    Strings,
}

impl Section {
    /// Every section, top of the score first.
    pub const ALL: [Section; 9] = [
        Section::Woodwinds,
        Section::Brass,
        Section::Percussion,
        Section::Drums,
        Section::HarpAndGuitars,
        Section::Keyboards,
        Section::Synths,
        Section::Voices,
        Section::Strings,
    ];

    /// Its name, as a heading.
    pub fn name(self) -> &'static str {
        match self {
            Section::Woodwinds => "Woodwinds",
            Section::Brass => "Brass",
            Section::Percussion => "Percussion",
            Section::Drums => "Drums",
            Section::HarpAndGuitars => "Harp and guitars",
            Section::Keyboards => "Keyboards",
            Section::Synths => "Synths",
            Section::Voices => "Voices",
            Section::Strings => "Strings",
        }
    }

    /// The General MIDI programs of the section, top of the score first.
    /// Every program is in exactly one section.
    fn programs(self) -> &'static [u8] {
        match self {
            // Piccolo, flute, recorder, pan flute, blown bottle,
            // shakuhachi, whistle, ocarina; oboe, shanai, English horn,
            // clarinet; soprano, alto, tenor and baritone sax; bassoon;
            // bagpipe and harmonica, which blow through reeds too.
            Section::Woodwinds => &[
                72, 73, 74, 75, 76, 77, 78, 79, 68, 111, 69, 71, 64, 65, 66, 67, 70, 109, 22,
            ],
            // French horn, trumpet, muted trumpet, trombone, tuba, then
            // the whole section and the synth brass.
            Section::Brass => &[60, 56, 59, 57, 58, 61, 62, 63],
            // Timpani; glockenspiel, music box, xylophone, vibraphone,
            // marimba, tubular bells, dulcimer, kalimba, steel drums;
            // tinkle bell, agogo, woodblock, taiko, melodic tom, synth
            // drum, reverse cymbal; then the sound effects, which
            // percussionists play in an orchestra.
            Section::Percussion => &[
                47, 9, 10, 13, 11, 12, 14, 15, 108, 114, 112, 113, 115, 116, 117, 118, 119, 120,
                121, 122, 123, 124, 125, 126, 127,
            ],
            // The drums are built-in sounds and the MIDI drum channel, not
            // programs (see `place`).
            Section::Drums => &[],
            // Harp; nylon, steel, jazz, clean, muted, overdriven,
            // distortion guitar and harmonics; banjo, sitar, shamisen,
            // koto; the eight basses.
            Section::HarpAndGuitars => &[
                46, 24, 25, 26, 27, 28, 29, 30, 31, 105, 104, 106, 107, 32, 33, 34, 35, 36, 37, 38,
                39,
            ],
            // Celesta; the six pianos; harpsichord, clavinet; the five
            // organs; accordion, tango accordion (the bandoneon).
            Section::Keyboards => &[8, 0, 1, 2, 3, 4, 5, 6, 7, 16, 17, 18, 19, 20, 21, 23],
            // Synth leads, pads and effects, and the orchestra hit.
            Section::Synths => &[
                80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99,
                100, 101, 102, 103, 55,
            ],
            // A solo voice above the choir.
            Section::Voices => &[53, 52, 54],
            // The string sections first; then violin, fiddle, viola,
            // cello and double bass, so the basses end the score.
            Section::Strings => &[48, 49, 50, 51, 44, 45, 40, 110, 41, 42, 43],
        }
    }
}

/// The built-in drums in score order: the hand percussion first, then the
/// drum kit as the drum staff reads from top to bottom.
const DRUMS: [Drum; 13] = [
    Drum::Cowbell,
    Drum::Shaker,
    Drum::Clap,
    Drum::Conga,
    Drum::Crash,
    Drum::HiHat,
    Drum::OpenHat,
    Drum::Ride,
    Drum::HighTom,
    Drum::Snare,
    Drum::Rim,
    Drum::LowTom,
    Drum::Kick,
];

/// Where `channel` goes in the score: its section, and its place in it
/// (lower is higher up).
pub fn place(channel: &Channel) -> (Section, usize) {
    match &channel.kind {
        ChannelKind::Sampler(SampleSource::Builtin(drum)) => {
            let rank = DRUMS.iter().position(|d| d == drum).unwrap_or(DRUMS.len());
            (Section::Drums, rank)
        }
        // A sound file usually takes a drum's row in the rack.
        ChannelKind::Sampler(SampleSource::File(_)) => (Section::Drums, DRUMS.len() + 1),
        ChannelKind::MidiOut {
            channel: DRUM_CHANNEL,
        } => (Section::Drums, DRUMS.len()),
        ChannelKind::Synth | ChannelKind::MidiOut { .. } => match channel.program {
            Some(program) => program_place(program),
            // The Lineup Osc, or an outside synth with no instrument named.
            None => (Section::Synths, usize::MAX),
        },
    }
}

/// Where General MIDI program `program` goes in the score.
fn program_place(program: u8) -> (Section, usize) {
    Section::ALL
        .into_iter()
        .find_map(|section| {
            let rank = section.programs().iter().position(|&p| p == program)?;
            Some((section, rank))
        })
        .unwrap_or((Section::Synths, usize::MAX))
}

/// The channels in score order, grouped by section: each section that has
/// channels, with their indices. Channels in the same place keep the
/// rack's order, so "Voice 2" stays under "Lead".
pub fn score_order(channels: &[Channel]) -> Vec<(Section, Vec<usize>)> {
    let mut order: Vec<usize> = (0..channels.len()).collect();
    order.sort_by_key(|&index| place(&channels[index]));
    let mut sections: Vec<(Section, Vec<usize>)> = Vec::new();
    for index in order {
        let section = place(&channels[index]).0;
        match sections.last_mut() {
            Some((last, indices)) if *last == section => indices.push(index),
            _ => sections.push((section, vec![index])),
        }
    }
    sections
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::general_midi::{self as gm, LAST};

    fn synth(name: &str, program: Option<u8>) -> Channel {
        Channel {
            name: name.to_string(),
            program,
            ..Channel::synth()
        }
    }

    fn names(channels: &[Channel]) -> Vec<(&'static str, Vec<&str>)> {
        score_order(channels)
            .into_iter()
            .map(|(section, indices)| {
                let names = indices.iter().map(|&i| channels[i].name.as_str()).collect();
                (section.name(), names)
            })
            .collect()
    }

    #[test]
    fn every_program_has_one_place_in_the_score() {
        for program in 0..=LAST {
            let sections: Vec<Section> = Section::ALL
                .into_iter()
                .filter(|section| section.programs().contains(&program))
                .collect();
            assert_eq!(sections.len(), 1, "{}: {sections:?}", gm::name(program));
        }
    }

    #[test]
    fn every_built_in_drum_has_a_place_in_the_kit() {
        for drum in Drum::ALL {
            assert_eq!(DRUMS.iter().filter(|&&d| d == drum).count(), 1, "{drum:?}");
        }
    }

    #[test]
    fn drums_added_after_the_band_join_the_other_drums() {
        let channels = [
            Channel::drum(Drum::Kick),
            Channel::drum(Drum::Clap),
            Channel::drum(Drum::HiHat),
            Channel::drum(Drum::Snare),
            synth("Lineup Osc", None),
            synth("Keys", Some(gm::ACOUSTIC_GRAND_PIANO)),
            synth("Bass", Some(gm::FINGER_BASS)),
            Channel::drum(Drum::Ride),
            Channel::drum(Drum::Crash),
            synth("Lead", Some(gm::ALTO_SAX)),
        ];
        assert_eq!(
            names(&channels),
            [
                ("Woodwinds", vec!["Lead"]),
                (
                    "Drums",
                    vec!["Clap", "Crash", "Hat", "Ride", "Snare", "Kick"]
                ),
                ("Harp and guitars", vec!["Bass"]),
                ("Keyboards", vec!["Keys"]),
                ("Synths", vec!["Lineup Osc"]),
            ]
        );
    }

    #[test]
    fn an_orchestra_reads_from_the_flutes_down_to_the_basses() {
        let channels = [
            synth("Basses", Some(gm::CONTRABASS)),
            synth("Violins", Some(gm::VIOLIN)),
            synth("Choir", Some(gm::CHOIR_AAHS)),
            synth("Timpani", Some(47)),
            synth("Horn", Some(gm::FRENCH_HORN)),
            synth("Trumpet", Some(gm::TRUMPET)),
            synth("Oboe", Some(gm::OBOE)),
            synth("Flute", Some(gm::FLUTE)),
            synth("Harp", Some(gm::HARP)),
            synth("Celesta", Some(gm::CELESTA)),
            synth("Cellos", Some(gm::CELLO)),
            synth("Violas", Some(gm::VIOLA)),
            Channel::drum(Drum::Snare),
            synth("Bassoon", Some(gm::BASSOON)),
        ];
        let order: Vec<&str> = names(&channels)
            .into_iter()
            .flat_map(|(_, names)| names)
            .collect();
        assert_eq!(
            order,
            [
                "Flute", "Oboe", "Bassoon", "Horn", "Trumpet", "Timpani", "Snare", "Harp",
                "Celesta", "Choir", "Violins", "Violas", "Cellos", "Basses"
            ]
        );
    }

    #[test]
    fn the_midi_drum_channel_and_sound_files_are_drums_too() {
        let mut sample = Channel::drum(Drum::Kick);
        sample.name = "my-kick.wav".to_string();
        sample.kind = ChannelKind::Sampler(SampleSource::File("my-kick.wav".into()));
        let channels = [
            sample,
            Channel::midi_out(0),
            Channel::midi_out(DRUM_CHANNEL),
            Channel::drum(Drum::Snare),
        ];
        assert_eq!(
            names(&channels),
            [
                ("Drums", vec!["Snare", "MIDI Out 10", "my-kick.wav"]),
                ("Synths", vec!["MIDI Out 1"]),
            ]
        );
    }

    #[test]
    fn channels_in_the_same_place_keep_the_racks_order() {
        let channels = [
            synth("Lead", Some(gm::VOICE_OOHS)),
            synth("Voice 2", Some(gm::VOICE_OOHS)),
            synth("Voice 3", Some(gm::VOICE_OOHS)),
        ];
        assert_eq!(
            names(&channels),
            [("Voices", vec!["Lead", "Voice 2", "Voice 3"])]
        );
    }
}
