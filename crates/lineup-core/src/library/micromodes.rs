//! Micromode styles: music in one of Kröpfl's colours, to try from the
//! library.
//!
//! Like a twelve-tone style, a micromode style has no chord progression.
//! It names one micromode, a [`Walk`] from form to form, and how each
//! instrument plays the forms (see [`crate::micromodal`]). Every
//! instrument plays the same walk, so the chords, the bass and the tune
//! always share their three notes.
//!
//! The styles were written for Ferrous Lineup. Kröpfl's own music, and his
//! students', is still under copyright, so none of it is copied here.

use crate::harmony::{Micromode, PitchSet, Scale};
use crate::meter::TimeSignature;
use crate::micromodal::{MicroWriting, Texture, Walk, bass_note, walk, write_forms};
use crate::pitch::{Note, PitchClass};
use crate::project::Project;
use crate::serial::Register;

use super::apply::{BASS, KEYS, LEAD, RhythmChange, chords_channel, find_synth, synth_named};
use super::rhythm::Rhythm;
use super::rows::take_over_harmony;

/// How one instrument plays the walk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MicroVoice {
    /// The channel: [`KEYS`], [`BASS`] or [`LEAD`].
    pub channel: &'static str,
    pub texture: Texture,
    pub register: Register,
    /// The lowest note, as a MIDI note number.
    pub low: u8,
    pub velocity: u8,
}

const fn voice(channel: &'static str, texture: Texture, low: u8) -> MicroVoice {
    MicroVoice {
        channel,
        texture,
        register: Register::Nearest,
        low,
        velocity: crate::DEFAULT_VELOCITY,
    }
}

impl MicroVoice {
    const fn wide(self) -> Self {
        MicroVoice {
            register: Register::Wide,
            ..self
        }
    }

    const fn soft(self, velocity: u8) -> Self {
        MicroVoice { velocity, ..self }
    }

    pub fn writing(&self) -> MicroWriting {
        MicroWriting {
            texture: self.texture,
            register: self.register,
            low: Note::new(self.low),
            velocity: self.velocity,
        }
    }
}

/// Where a section of a micromode piece starts its walk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Start {
    /// Whether it starts on the micromode upside down.
    pub inverted: bool,
    /// How many semitones above the key note.
    pub up: u8,
}

/// The sections of a micromode demo, with their names: the micromode,
/// upside down, a tritone away, and back.
pub const DEMO_SECTIONS: [(Start, &str); 4] = [
    (
        Start {
            inverted: false,
            up: 0,
        },
        "The micromode",
    ),
    (
        Start {
            inverted: true,
            up: 0,
        },
        "Upside down",
    ),
    (
        Start {
            inverted: false,
            up: 6,
        },
        "A tritone away",
    ),
    (
        Start {
            inverted: true,
            up: 6,
        },
        "Upside down, a tritone away",
    ),
];

/// A way of playing a micromode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MicromodeStyle {
    pub name: &'static str,
    pub about: &'static str,
    /// The micromode's number, 1 to 12 (see [`Micromode::number`]).
    pub micromode: u8,
    pub walk: Walk,
    /// How many steps each form lasts.
    pub steps: usize,
    pub tempo: f32,
    pub time: TimeSignature,
    pub bars: usize,
    pub voices: &'static [MicroVoice],
    /// A rhythm from the library for the drums, or "" for none.
    pub rhythm: &'static str,
}

impl MicromodeStyle {
    pub fn named(name: &str) -> Option<&'static MicromodeStyle> {
        MICROMODE_STYLES.iter().find(|style| style.name == name)
    }

    pub fn micromode(&self) -> &'static Micromode {
        Micromode::number(self.micromode).unwrap_or(&Micromode::all()[0])
    }

    /// How long one pass is, in steps.
    pub fn length(&self) -> usize {
        self.bars * self.time.steps_per_bar()
    }

    pub fn rhythm(&self) -> Option<&'static Rhythm> {
        Rhythm::named(self.rhythm)
    }

    /// The first form: the micromode's prime form, or its inversion,
    /// starting on `key` moved up `start.up` semitones.
    pub fn first_form(&self, key: PitchClass, start: Start) -> PitchSet {
        let prime = self.micromode().prime;
        let form = if start.inverted {
            prime.invert()
        } else {
            prime
        };
        let from = form.normal_form().first().copied().unwrap_or(PitchClass::C);
        let note = key.transpose(i32::from(start.up));
        form.transpose(from.semitones_up_to(note))
    }

    /// The forms one pass plays, from `first`.
    pub fn forms(&self, first: PitchSet, length: usize) -> Vec<PitchSet> {
        let count = length.div_ceil(self.steps.max(1)).max(1);
        walk(self.micromode(), first, self.walk, count)
    }

    /// The key a piece in this style is in: the micromode itself, as a
    /// pitch-class set, on the note the first form starts on (or, for a
    /// chord, on its root).
    pub fn scale(&self, key: PitchClass) -> Scale {
        let first = self.first_form(key, DEMO_SECTIONS[0].0);
        Scale::set(bass_note(first), first)
    }
}

const THREE_FOUR: TimeSignature = TimeSignature::new(3, 4);

/// The micromode styles of the library.
pub static MICROMODE_STYLES: [MicromodeStyle; 7] = [
    MicromodeStyle {
        name: "Menor 3 colour",
        about: "One colour all through: Menor 3, a semitone and two thirds. The chords keep \
                two notes and move one, and the tune is made of the same three notes. This is \
                the idea of Kröpfl's micromodes: one group of three notes is the harmony and \
                the melody at once.",
        micromode: 3,
        walk: Walk::CommonTones,
        steps: 8,
        tempo: 76.0,
        time: TimeSignature::COMMON,
        bars: 4,
        voices: &[
            voice(KEYS, Texture::Chords, 55),
            voice(LEAD, Texture::Line(2), 67),
            voice(BASS, Texture::Bass, 36),
        ],
        rhythm: "",
    },
    MicromodeStyle {
        name: "Viennese points",
        about: "Menor 5, the \"Viennese trichord\" of Schoenberg's school (a semitone, a fourth \
                and a tritone): single notes leaping over three octaves, as in Webern, over \
                soft chords of the same three notes.",
        micromode: 5,
        walk: Walk::CommonTones,
        steps: 8,
        tempo: 66.0,
        time: TimeSignature::COMMON,
        bars: 4,
        voices: &[
            voice(LEAD, Texture::Line(2), 55).wide(),
            voice(KEYS, Texture::Chords, 48).soft(64),
        ],
        rhythm: "",
    },
    MicromodeStyle {
        name: "Cluster pulse",
        about: "Menor 1, three notes a semitone apart, struck as clusters that creep up a \
                semitone at a time, over a techno beat.",
        micromode: 1,
        walk: Walk::CommonTones,
        steps: 4,
        tempo: 124.0,
        time: TimeSignature::COMMON,
        bars: 2,
        voices: &[
            voice(KEYS, Texture::Pulse(2), 52),
            voice(BASS, Texture::Bass, 36),
        ],
        rhythm: "Techno",
    },
    MicromodeStyle {
        name: "Whole-tone haze",
        about: "Mayor 1, two whole tones, moved up a whole tone every bar. It never leaves one \
                whole-tone scale, the sound of Debussy's Voiles.",
        micromode: 6,
        walk: Walk::Transpose(2),
        steps: 12,
        tempo: 60.0,
        time: THREE_FOUR,
        bars: 6,
        voices: &[
            voice(KEYS, Texture::Arpeggio(2), 60),
            voice(BASS, Texture::Bass, 40).soft(80),
        ],
        rhythm: "",
    },
    MicromodeStyle {
        name: "Fourths groove",
        about: "Mayor 4, a whole tone and two fourths: open sus chords that keep two notes and \
                move one, over a neo-soul beat.",
        micromode: 9,
        walk: Walk::CommonTones,
        steps: 8,
        tempo: 84.0,
        time: TimeSignature::COMMON,
        bars: 4,
        voices: &[
            voice(KEYS, Texture::Chords, 55),
            voice(BASS, Texture::Bass, 36),
        ],
        rhythm: "Neo soul",
    },
    MicromodeStyle {
        name: "Triads, two notes kept",
        about: "Acorde menor-mayor: major and minor triads, each keeping two notes of the one \
                before. These are the three moves of neo-Riemannian theory (P, L and R), the \
                chromatic chord changes of Wagner and Liszt.",
        micromode: 11,
        walk: Walk::CommonTones,
        steps: 8,
        tempo: 80.0,
        time: TimeSignature::COMMON,
        bars: 4,
        voices: &[
            voice(KEYS, Texture::Chords, 55),
            voice(LEAD, Texture::Line(4), 67),
            voice(BASS, Texture::Bass, 36),
        ],
        rhythm: "",
    },
    MicromodeStyle {
        name: "Augmented drift",
        about: "Acorde aumentado, three major thirds, sliding down a semitone at a time. There \
                are only four augmented triads, so the fifth chord has the notes of the first.",
        micromode: 12,
        walk: Walk::Transpose(11),
        steps: 8,
        tempo: 63.0,
        time: TimeSignature::COMMON,
        bars: 4,
        voices: &[
            voice(KEYS, Texture::Chords, 55),
            voice(LEAD, Texture::Line(4), 72),
        ],
        rhythm: "",
    },
];

/// Writes the voices of `style` into pattern `pattern`, starting the
/// walk at `start` above `key`. Chords go to `keys` (or a channel
/// [`chords_channel`] picks). Returns `false` if the pattern does not
/// exist or the rack is full.
fn write_voices(
    project: &mut Project,
    pattern: usize,
    style: &MicromodeStyle,
    key: PitchClass,
    start: Start,
    keys: Option<usize>,
) -> bool {
    let Some(length) = project.pattern(pattern).map(|p| p.length()) else {
        return false;
    };
    let forms = style.forms(style.first_form(key, start), length);
    let mut writes = Vec::new();
    for voice in style.voices {
        let channel = match voice.channel {
            KEYS => chords_channel(project, keys),
            name => synth_named(project, name),
        };
        let Some(channel) = channel else {
            return false;
        };
        writes.push((
            channel,
            write_forms(&forms, style.steps, &voice.writing(), length),
        ));
    }
    let Some(target) = project.pattern_mut(pattern) else {
        return false;
    };
    for (channel, notes) in writes {
        target.set_notes(channel, notes);
    }
    true
}

/// Writes `style` into pattern `pattern` on `key`, starting as `start`
/// says: its voices, its length, its time signature, and the key
/// becomes the micromode. Lead, Keys and Bass channels the style does not
/// use go quiet in the pattern. Returns `false` if the pattern does not
/// exist or the rack is full.
pub fn write_micromode_style(
    project: &mut Project,
    pattern: usize,
    style: &MicromodeStyle,
    key: PitchClass,
    start: Start,
    selected: Option<usize>,
) -> bool {
    let Some(target) = project.pattern_mut(pattern) else {
        return false;
    };
    target.set_length(style.length());
    project.time_signature = style.time;
    project.scale = style.scale(key);
    let unused: Vec<usize> = [KEYS, BASS, LEAD]
        .into_iter()
        .filter(|name| !style.voices.iter().any(|voice| voice.channel == *name))
        .filter_map(|name| find_synth(project, name))
        .collect();
    if let Some(target) = project.pattern_mut(pattern) {
        for channel in unused {
            target.set_notes(channel, []);
        }
    }
    write_voices(project, pattern, style, key, start, selected)
}

/// Plays the whole song in `style`, as [`super::rows::use_row_style`]
/// does with a row: the style's walk replaces the song's chords and
/// bass, what it does not play goes quiet, and the melody stays unless
/// the style has one. The patterns start in turn from the four places of
/// [`DEMO_SECTIONS`], so the song does not repeat itself. The key becomes
/// the micromode, on the song's key note.
pub fn use_micromode_style(project: &mut Project, style: &MicromodeStyle) -> RhythmChange {
    let key = project.scale.root;
    let uses = |name: &str| style.voices.iter().any(|voice| voice.channel == name);
    let change = take_over_harmony(
        project,
        &uses,
        style.rhythm(),
        |project, pattern, n, keys| {
            let start = DEMO_SECTIONS[n % DEMO_SECTIONS.len()].0;
            write_voices(project, pattern, style, key, start, keys);
        },
    );
    if change.harmony > 0 {
        project.scale = style.scale(key);
    }
    change
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::apply::heard_chords_channel;

    #[test]
    fn every_style_names_a_micromode_and_a_rhythm() {
        for style in &MICROMODE_STYLES {
            assert!(
                Micromode::number(style.micromode).is_some(),
                "{}",
                style.name
            );
            assert!(
                style.rhythm.is_empty() || style.rhythm().is_some(),
                "{}",
                style.name
            );
            if let Some(rhythm) = style.rhythm() {
                assert_eq!(rhythm.time, style.time, "{}", style.name);
            }
            assert!(!style.voices.is_empty());
            assert_eq!(style.length() % style.steps, 0, "{}", style.name);
        }
    }

    #[test]
    fn every_style_plays_only_its_micromode_in_any_key() {
        for style in &MICROMODE_STYLES {
            for key in [PitchClass::C, PitchClass::FSharp] {
                for (start, _) in DEMO_SECTIONS {
                    let mut project = Project::empty();
                    assert!(
                        write_micromode_style(&mut project, 0, style, key, start, None),
                        "{}",
                        style.name
                    );
                    let pattern = project.pattern(0).unwrap();
                    assert_eq!(pattern.length(), style.length());
                    let forms = style.forms(style.first_form(key, start), pattern.length());
                    for voice in style.voices {
                        let channel = find_synth(&project, voice.channel).unwrap();
                        let notes = pattern.notes(channel);
                        assert!(!notes.is_empty(), "{} {}", style.name, voice.channel);
                        for note in notes {
                            assert!(note.end() <= pattern.length());
                            let form = forms[note.start / style.steps];
                            assert_eq!(Micromode::of(form), Some(style.micromode()));
                            assert!(form.contains(note.pitch.pitch_class()), "{}", style.name);
                        }
                    }
                    // The key is the micromode.
                    let scale = project.scale.pitch_set();
                    assert_eq!(
                        Micromode::of(scale),
                        Some(style.micromode()),
                        "{}",
                        style.name
                    );
                }
            }
        }
    }

    #[test]
    fn the_first_form_starts_on_the_key_note() {
        let style = MicromodeStyle::named("Menor 3 colour").unwrap();
        let start = |inverted, up| Start { inverted, up };
        // C C# E, and upside down A C C#: the inversion, packed, from C.
        let prime = style.first_form(PitchClass::C, start(false, 0));
        assert_eq!(prime, PitchSet::from_semitones(&[0, 1, 4]));
        let inverted = style.first_form(PitchClass::C, start(true, 0));
        assert_eq!(inverted, PitchSet::from_semitones(&[0, 3, 4]));
        assert_eq!(inverted.normal_form()[0], PitchClass::C);
        let away = style.first_form(PitchClass::D, start(false, 6));
        assert_eq!(away.normal_form()[0], PitchClass::GSharp);
        // Triads: the key's own chord.
        let triads = MicromodeStyle::named("Triads, two notes kept").unwrap();
        assert_eq!(triads.scale(PitchClass::A).root, PitchClass::A);
    }

    #[test]
    fn a_song_takes_a_micromode_style() {
        let original = Project::demo();
        let pattern = (0..original.patterns().len())
            .find(|&p| heard_chords_channel(&original, p).is_some())
            .unwrap();
        let chords = heard_chords_channel(&original, pattern).unwrap();
        let mut project = original.clone();
        let style = MicromodeStyle::named("Fourths groove").unwrap();
        let change = use_micromode_style(&mut project, style);
        assert!(change.harmony > 0 && change.drums > 0, "{change:?}");
        assert_eq!(
            Micromode::of(project.scale.pitch_set()),
            Some(style.micromode())
        );
        // Its chords replace the song's, on the same channel.
        let notes = project.pattern(pattern).unwrap().notes(chords);
        assert!(!notes.is_empty());
        let heard: PitchSet = notes
            .iter()
            .filter(|n| n.start == 0)
            .map(|n| n.pitch.pitch_class())
            .collect();
        assert_eq!(Micromode::of(heard), Some(style.micromode()));
    }
}
