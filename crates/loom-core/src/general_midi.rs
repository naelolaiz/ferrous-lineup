//! General MIDI: the 128 instruments every MIDI synth knows by number.
//!
//! A MIDI file or a MIDI cable carries notes, not sounds. To say *which*
//! sound should play them, MIDI has the *program change* message: "from
//! now on, channel 3 plays program 40". On its own, a number means
//! nothing; General MIDI (1991) fixed what each number is, so that 40 is a
//! violin on every synth that follows it, from a phone to a concert
//! keyboard. Channel 10 is kept for drums (see
//! [`DRUM_CHANNEL`](crate::DRUM_CHANNEL)).
//!
//! The programs come in sixteen families of eight: pianos 0 to 7,
//! chromatic percussion 8 to 15, organs 16 to 23, and so on. Manuals and
//! synths count them from 1, so program 40 here is "41 Violin" there;
//! the bytes, like this module, count from 0.

/// The highest program number.
pub const LAST: u8 = 127;

pub const ACOUSTIC_GRAND_PIANO: u8 = 0;
pub const HONKY_TONK_PIANO: u8 = 3;
pub const ELECTRIC_PIANO: u8 = 4;
pub const HARPSICHORD: u8 = 6;
pub const CLAVINET: u8 = 7;
pub const CELESTA: u8 = 8;
pub const VIBRAPHONE: u8 = 11;
pub const MARIMBA: u8 = 12;
pub const DRAWBAR_ORGAN: u8 = 16;
pub const CHURCH_ORGAN: u8 = 19;
pub const ACCORDION: u8 = 21;
pub const HARMONICA: u8 = 22;
pub const TANGO_ACCORDION: u8 = 23;
pub const NYLON_GUITAR: u8 = 24;
pub const STEEL_GUITAR: u8 = 25;
pub const JAZZ_GUITAR: u8 = 26;
pub const CLEAN_GUITAR: u8 = 27;
pub const OVERDRIVEN_GUITAR: u8 = 29;
pub const DISTORTION_GUITAR: u8 = 30;
pub const ACOUSTIC_BASS: u8 = 32;
pub const FINGER_BASS: u8 = 33;
pub const SLAP_BASS: u8 = 36;
pub const SYNTH_BASS: u8 = 38;
pub const VIOLIN: u8 = 40;
pub const VIOLA: u8 = 41;
pub const CELLO: u8 = 42;
pub const CONTRABASS: u8 = 43;
pub const PIZZICATO_STRINGS: u8 = 45;
pub const HARP: u8 = 46;
pub const STRING_ENSEMBLE: u8 = 48;
pub const CHOIR_AAHS: u8 = 52;
pub const VOICE_OOHS: u8 = 53;
pub const TRUMPET: u8 = 56;
pub const TROMBONE: u8 = 57;
pub const TUBA: u8 = 58;
pub const FRENCH_HORN: u8 = 60;
pub const BRASS_SECTION: u8 = 61;
pub const ALTO_SAX: u8 = 65;
pub const TENOR_SAX: u8 = 66;
pub const OBOE: u8 = 68;
pub const ENGLISH_HORN: u8 = 69;
pub const BASSOON: u8 = 70;
pub const CLARINET: u8 = 71;
pub const PICCOLO: u8 = 72;
pub const FLUTE: u8 = 73;
pub const PAN_FLUTE: u8 = 75;
pub const SQUARE_LEAD: u8 = 80;
pub const SAW_LEAD: u8 = 81;
pub const WARM_PAD: u8 = 89;
pub const BANJO: u8 = 105;
pub const KOTO: u8 = 107;
pub const FIDDLE: u8 = 110;

/// The name of every program, by number, as the General MIDI list gives it.
pub const NAMES: [&str; 128] = [
    // Piano
    "Acoustic Grand Piano",
    "Bright Acoustic Piano",
    "Electric Grand Piano",
    "Honky-tonk Piano",
    "Electric Piano 1",
    "Electric Piano 2",
    "Harpsichord",
    "Clavinet",
    // Chromatic percussion
    "Celesta",
    "Glockenspiel",
    "Music Box",
    "Vibraphone",
    "Marimba",
    "Xylophone",
    "Tubular Bells",
    "Dulcimer",
    // Organ
    "Drawbar Organ",
    "Percussive Organ",
    "Rock Organ",
    "Church Organ",
    "Reed Organ",
    "Accordion",
    "Harmonica",
    "Tango Accordion",
    // Guitar
    "Acoustic Guitar (nylon)",
    "Acoustic Guitar (steel)",
    "Electric Guitar (jazz)",
    "Electric Guitar (clean)",
    "Electric Guitar (muted)",
    "Overdriven Guitar",
    "Distortion Guitar",
    "Guitar Harmonics",
    // Bass
    "Acoustic Bass",
    "Electric Bass (finger)",
    "Electric Bass (pick)",
    "Fretless Bass",
    "Slap Bass 1",
    "Slap Bass 2",
    "Synth Bass 1",
    "Synth Bass 2",
    // Strings
    "Violin",
    "Viola",
    "Cello",
    "Contrabass",
    "Tremolo Strings",
    "Pizzicato Strings",
    "Orchestral Harp",
    "Timpani",
    // Ensemble
    "String Ensemble 1",
    "String Ensemble 2",
    "Synth Strings 1",
    "Synth Strings 2",
    "Choir Aahs",
    "Voice Oohs",
    "Synth Voice",
    "Orchestra Hit",
    // Brass
    "Trumpet",
    "Trombone",
    "Tuba",
    "Muted Trumpet",
    "French Horn",
    "Brass Section",
    "Synth Brass 1",
    "Synth Brass 2",
    // Reed
    "Soprano Sax",
    "Alto Sax",
    "Tenor Sax",
    "Baritone Sax",
    "Oboe",
    "English Horn",
    "Bassoon",
    "Clarinet",
    // Pipe
    "Piccolo",
    "Flute",
    "Recorder",
    "Pan Flute",
    "Blown Bottle",
    "Shakuhachi",
    "Whistle",
    "Ocarina",
    // Synth lead
    "Lead 1 (square)",
    "Lead 2 (sawtooth)",
    "Lead 3 (calliope)",
    "Lead 4 (chiff)",
    "Lead 5 (charang)",
    "Lead 6 (voice)",
    "Lead 7 (fifths)",
    "Lead 8 (bass + lead)",
    // Synth pad
    "Pad 1 (new age)",
    "Pad 2 (warm)",
    "Pad 3 (polysynth)",
    "Pad 4 (choir)",
    "Pad 5 (bowed)",
    "Pad 6 (metallic)",
    "Pad 7 (halo)",
    "Pad 8 (sweep)",
    // Synth effects
    "FX 1 (rain)",
    "FX 2 (soundtrack)",
    "FX 3 (crystal)",
    "FX 4 (atmosphere)",
    "FX 5 (brightness)",
    "FX 6 (goblins)",
    "FX 7 (echoes)",
    "FX 8 (sci-fi)",
    // Ethnic
    "Sitar",
    "Banjo",
    "Shamisen",
    "Koto",
    "Kalimba",
    "Bagpipe",
    "Fiddle",
    "Shanai",
    // Percussive
    "Tinkle Bell",
    "Agogo",
    "Steel Drums",
    "Woodblock",
    "Taiko Drum",
    "Melodic Tom",
    "Synth Drum",
    "Reverse Cymbal",
    // Sound effects
    "Guitar Fret Noise",
    "Breath Noise",
    "Seashore",
    "Bird Tweet",
    "Telephone Ring",
    "Helicopter",
    "Applause",
    "Gunshot",
];

/// The sixteen families, eight programs each, in order.
pub const FAMILIES: [&str; 16] = [
    "Piano",
    "Chromatic percussion",
    "Organ",
    "Guitar",
    "Bass",
    "Strings",
    "Ensemble",
    "Brass",
    "Reed",
    "Pipe",
    "Synth lead",
    "Synth pad",
    "Synth effects",
    "Ethnic",
    "Percussive",
    "Sound effects",
];

/// The name of program `program`, such as "Violin" for 40.
pub fn name(program: u8) -> &'static str {
    NAMES[usize::from(program.min(LAST))]
}

/// The family program `program` belongs to, 0 (piano) to 15 (sound effects).
pub fn family(program: u8) -> usize {
    usize::from(program.min(LAST)) / 8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_named_programs_are_the_general_midi_ones() {
        assert_eq!(name(ACOUSTIC_GRAND_PIANO), "Acoustic Grand Piano");
        assert_eq!(name(VIOLIN), "Violin");
        assert_eq!(name(CHOIR_AAHS), "Choir Aahs");
        assert_eq!(name(TANGO_ACCORDION), "Tango Accordion");
        assert_eq!(name(NYLON_GUITAR), "Acoustic Guitar (nylon)");
        assert_eq!(name(FLUTE), "Flute");
        assert_eq!(name(PAN_FLUTE), "Pan Flute");
        assert_eq!(name(FIDDLE), "Fiddle");
        assert_eq!(name(KOTO), "Koto");
        assert_eq!(name(ENGLISH_HORN), "English Horn");
        assert_eq!(name(LAST), "Gunshot");
        assert_eq!(FAMILIES[family(VIOLIN)], "Strings");
        assert_eq!(FAMILIES[family(CHOIR_AAHS)], "Ensemble");
    }
}
