//! The players a band style adds to the piano, the bass and the drums:
//! a guitar, strings and an organ, each where the music has them.
//!
//! A band is more than a rhythm section. A rock band has its guitar, a
//! bossa nova its nylon-string guitar, a tango orchestra its violins and
//! a reggae band its organ "bubble". Each band style lists the players
//! it adds, what they play and on which General MIDI instrument; most
//! styles add one, some none. A player can also wait for variation b,
//! coming in to lift a chorus, as strings often do in pop.

use crate::i18n::{list, tr};
use crate::library::comping::{Comping, Played};
use crate::meter::Bar;
use crate::pattern::NoteEvent;

use super::guitar::GuitarPart;
use super::variation::Variation;

/// What strings or an organ play.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PadPart {
    /// Each chord held for as long as it lasts: a pad under the band.
    Held,
    /// Plain accents in the bar's beat grouping, on the existing sound.
    Accents,
    /// The reggae organ's "bubble": short chords between all the beats
    /// low down, and on beats 2 and 4 an octave higher, with the guitar.
    Bubble,
}

/// A player a style adds: what it plays, on which General MIDI
/// instrument, and whether it waits for variation b.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Extra<Part> {
    pub part: Part,
    pub program: u8,
    pub only_in_b: bool,
}

/// The players a band style adds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Extras {
    pub guitar: Option<Extra<GuitarPart>>,
    pub strings: Option<Extra<PadPart>>,
    pub organ: Option<Extra<PadPart>>,
}

// The General MIDI instruments, numbered from 0.
const NYLON_GUITAR: u8 = 24;
const STEEL_GUITAR: u8 = 25;
const JAZZ_GUITAR: u8 = 26;
const CLEAN_GUITAR: u8 = 27;
const MUTED_GUITAR: u8 = 28;
const OVERDRIVEN_GUITAR: u8 = 29;
const DISTORTION_GUITAR: u8 = 30;
const STRINGS: u8 = 48;
const SLOW_STRINGS: u8 = 49;
const SYNTH_STRINGS: u8 = 50;
const DRAWBAR_ORGAN: u8 = 16;
const PERCUSSIVE_ORGAN: u8 = 17;
const ROCK_ORGAN: u8 = 18;
const CHURCH_ORGAN: u8 = 19;
const BANJO: u8 = 105;

const fn guitar(part: GuitarPart, program: u8) -> Option<Extra<GuitarPart>> {
    Some(Extra {
        part,
        program,
        only_in_b: false,
    })
}

const fn pad(program: u8) -> Option<Extra<PadPart>> {
    Some(Extra {
        part: PadPart::Held,
        program,
        only_in_b: false,
    })
}

const fn pad_in_b(program: u8) -> Option<Extra<PadPart>> {
    Some(Extra {
        part: PadPart::Held,
        program,
        only_in_b: true,
    })
}

const NONE: Extras = Extras {
    guitar: None,
    strings: None,
    organ: None,
};

const fn with_guitar(part: GuitarPart, program: u8) -> Extras {
    Extras {
        guitar: guitar(part, program),
        ..NONE
    }
}

const STRUMMED: GuitarPart = GuitarPart::Strum(Comping::Strum);
const EIGHTHS: GuitarPart = GuitarPart::Strum(Comping::Pulse);
const CHOP: GuitarPart = GuitarPart::Chop;
const SCRATCH: GuitarPart = GuitarPart::Strum(Comping::Funk);
const FOLKLORE: Extras = with_guitar(GuitarPart::StylesRhythm, NYLON_GUITAR);
const TANGO: Extras = Extras {
    strings: pad(STRINGS),
    ..NONE
};
const REGGAE: Extras = Extras {
    guitar: guitar(CHOP, CLEAN_GUITAR),
    organ: Some(Extra {
        part: PadPart::Bubble,
        program: PERCUSSIVE_ORGAN,
        only_in_b: false,
    }),
    ..NONE
};

/// The players each band style adds, by its name. Styles not listed add
/// none.
const BY_STYLE: &[(&str, Extras)] = &[
    // Pop: a strummed acoustic guitar, strings for the chorus.
    (
        "Pop beat",
        Extras {
            guitar: guitar(STRUMMED, STEEL_GUITAR),
            strings: pad_in_b(STRINGS),
            ..NONE
        },
    ),
    (
        "Pop ballad",
        Extras {
            guitar: guitar(GuitarPart::Strum(Comping::ArpUpDown), STEEL_GUITAR),
            strings: pad(SLOW_STRINGS),
            ..NONE
        },
    ),
    (
        "Dance pop",
        Extras {
            strings: pad(SYNTH_STRINGS),
            ..NONE
        },
    ),
    // Rock: power chords on a distorted guitar, an organ for the chorus.
    (
        "Rock beat",
        Extras {
            guitar: guitar(GuitarPart::Power(Comping::Pulse), OVERDRIVEN_GUITAR),
            organ: pad_in_b(ROCK_ORGAN),
            ..NONE
        },
    ),
    (
        "Half-time rock",
        with_guitar(GuitarPart::Power(Comping::Held), DISTORTION_GUITAR),
    ),
    (
        "Punk",
        with_guitar(GuitarPart::Power(Comping::Pulse), DISTORTION_GUITAR),
    ),
    (
        "Rock ride",
        Extras {
            guitar: guitar(EIGHTHS, OVERDRIVEN_GUITAR),
            organ: pad(DRAWBAR_ORGAN),
            ..NONE
        },
    ),
    ("Surf beat", with_guitar(EIGHTHS, CLEAN_GUITAR)),
    (
        "Rock and roll",
        with_guitar(GuitarPart::Boogie, CLEAN_GUITAR),
    ),
    (
        "Metal",
        with_guitar(GuitarPart::Power(Comping::Pulse), DISTORTION_GUITAR),
    ),
    // Blues: the boogie shuffle, the Hammond organ of a slow blues.
    ("Shuffle", with_guitar(GuitarPart::Boogie, CLEAN_GUITAR)),
    (
        "Slow blues",
        Extras {
            organ: pad(DRAWBAR_ORGAN),
            ..NONE
        },
    ),
    // Jazz: the swing band's guitar, a ballad with strings.
    ("Swing", with_guitar(GuitarPart::FourToTheBar, JAZZ_GUITAR)),
    // Freddie Green's guitar in Count Basie's band: four even chords to
    // the bar, felt more than heard.
    (
        "Big band swing",
        with_guitar(GuitarPart::FourToTheBar, JAZZ_GUITAR),
    ),
    (
        "Jazz ballad",
        Extras {
            strings: pad(SLOW_STRINGS),
            ..NONE
        },
    ),
    // A ragtime band's banjo strums across the beat in 3 + 3 + 2.
    (
        "Cakewalk",
        with_guitar(GuitarPart::Strum(Comping::Dembow), BANJO),
    ),
    // Soul: the Motown chop and strings, the funk scratch.
    (
        "Motown",
        Extras {
            guitar: guitar(CHOP, CLEAN_GUITAR),
            strings: pad_in_b(STRINGS),
            ..NONE
        },
    ),
    ("Funk", with_guitar(SCRATCH, MUTED_GUITAR)),
    // Afrobeat: a guitar riffing in short sixteenths, an organ under it.
    (
        "Afrobeat",
        Extras {
            guitar: guitar(SCRATCH, CLEAN_GUITAR),
            organ: pad(DRAWBAR_ORGAN),
            ..NONE
        },
    ),
    (
        "Neo soul",
        Extras {
            organ: pad(DRAWBAR_ORGAN),
            ..NONE
        },
    ),
    (
        "R&B",
        Extras {
            strings: pad(STRINGS),
            ..NONE
        },
    ),
    // Electronic: disco's strings and scratch guitar, synth pads.
    (
        "Disco",
        Extras {
            guitar: guitar(SCRATCH, MUTED_GUITAR),
            strings: pad(STRINGS),
            ..NONE
        },
    ),
    (
        "Trance",
        Extras {
            strings: pad(SYNTH_STRINGS),
            ..NONE
        },
    ),
    (
        "Synthwave",
        Extras {
            strings: pad(SYNTH_STRINGS),
            ..NONE
        },
    ),
    // Latin: the nylon-string guitar of bossa nova and bolero.
    (
        "Bossa nova",
        with_guitar(GuitarPart::Strum(Comping::Bossa), NYLON_GUITAR),
    ),
    ("Samba", with_guitar(GuitarPart::StylesRhythm, NYLON_GUITAR)),
    (
        "Bolero",
        Extras {
            guitar: guitar(GuitarPart::StylesRhythm, NYLON_GUITAR),
            strings: pad(STRINGS),
            ..NONE
        },
    ),
    (
        "Bachata",
        with_guitar(GuitarPart::StylesRhythm, STEEL_GUITAR),
    ),
    // Tango: the orchestra's violins.
    ("Tango marcato", TANGO),
    ("Tango síncopa", TANGO),
    ("Milonga", TANGO),
    ("Habanera", TANGO),
    ("Tango vals", TANGO),
    ("Tango nuevo", TANGO),
    // Folklore: the criolla, a nylon-string guitar, in each dance's
    // rhythm.
    ("Zamba", FOLKLORE),
    ("Chacarera", FOLKLORE),
    ("Gato", FOLKLORE),
    ("Malambo", FOLKLORE),
    ("Chamamé", FOLKLORE),
    ("Cueca", FOLKLORE),
    ("Marinera", FOLKLORE),
    ("Bambuco", FOLKLORE),
    ("Joropo", FOLKLORE),
    ("Vidala", FOLKLORE),
    ("Carnavalito", FOLKLORE),
    ("Huayno", FOLKLORE),
    ("Peruvian waltz", FOLKLORE),
    ("Festejo", FOLKLORE),
    ("Bailecito", FOLKLORE),
    ("Rasguido doble", FOLKLORE),
    (
        "Rumba flamenca",
        with_guitar(GuitarPart::Strum(Comping::Rumba), NYLON_GUITAR),
    ),
    // Reggae: the guitar's chop and the organ's bubble; ska's upstrokes.
    ("One drop", REGGAE),
    ("Steppers", REGGAE),
    ("Rocksteady", REGGAE),
    (
        "Ska",
        with_guitar(GuitarPart::Strum(Comping::Offbeat), CLEAN_GUITAR),
    ),
    // Calypso's strummed guitar.
    ("Calypso", with_guitar(STRUMMED, STEEL_GUITAR)),
    // Folk and country: the steel-string guitar.
    ("Country train", with_guitar(EIGHTHS, STEEL_GUITAR)),
    ("Folk", with_guitar(STRUMMED, STEEL_GUITAR)),
    ("Two-step", with_guitar(CHOP, STEEL_GUITAR)),
    (
        "Waltz",
        with_guitar(GuitarPart::Strum(Comping::Waltz), STEEL_GUITAR),
    ),
    (
        "Six-eight",
        with_guitar(GuitarPart::StylesRhythm, STEEL_GUITAR),
    ),
    ("Bluegrass", with_guitar(CHOP, STEEL_GUITAR)),
    (
        "Western swing",
        with_guitar(GuitarPart::FourToTheBar, JAZZ_GUITAR),
    ),
    ("Honky-tonk shuffle", with_guitar(CHOP, STEEL_GUITAR)),
    (
        "Cajun two-step",
        with_guitar(GuitarPart::Strum(Comping::Afterbeat), STEEL_GUITAR),
    ),
    // The musette's accordion has a guitar strumming the waltz.
    (
        "Musette",
        with_guitar(GuitarPart::Strum(Comping::Waltz), NYLON_GUITAR),
    ),
    // The fado's Portuguese guitar picks above the classical guitar.
    (
        "Fado",
        with_guitar(GuitarPart::Strum(Comping::ArpUpDown), STEEL_GUITAR),
    ),
    // Classical: strings, the church organ of a chorale.
    (
        "Epic toms",
        Extras {
            strings: pad(STRINGS),
            ..NONE
        },
    ),
    (
        "Chorale",
        Extras {
            organ: pad(CHURCH_ORGAN),
            ..NONE
        },
    ),
    (
        "Arpeggios",
        Extras {
            strings: pad_in_b(STRINGS),
            ..NONE
        },
    ),
    // The orchestra's strings hold the chords its horns strike.
    (
        "Orchestra",
        Extras {
            strings: pad(STRINGS),
            ..NONE
        },
    ),
];

/// The players the band style called `name` adds.
pub(crate) fn of(name: &str) -> Extras {
    BY_STYLE
        .iter()
        .find(|(style, _)| *style == name)
        .map_or(NONE, |&(_, extras)| extras)
}

impl PadPart {
    /// The notes of strings or an organ playing `harmonies` in `bars`,
    /// an `octave` (in semitones) above the piano.
    pub(crate) fn play(self, octave: i32, bars: &[Bar], harmonies: &[Played]) -> Vec<NoteEvent> {
        let raised = |by: i32| -> Vec<Played> {
            harmonies
                .iter()
                .map(|harmony| Played {
                    voicing: harmony
                        .voicing
                        .iter()
                        .filter_map(|note| note.transpose(by))
                        .collect(),
                    ..harmony.clone()
                })
                .collect()
        };
        match self {
            PadPart::Held | PadPart::Accents => {
                let comping = if self == PadPart::Accents {
                    Comping::Accents
                } else {
                    Comping::Held
                };
                let mut notes = comping.play_in_bars(bars, &raised(octave));
                for note in &mut notes {
                    note.velocity = PAD_VELOCITY;
                }
                notes
            }
            PadPart::Bubble => {
                let mut notes = Comping::Offbeat.play_in_bars(bars, &raised(octave));
                notes.extend(Comping::Backbeat.play_in_bars(bars, &raised(octave + 12)));
                notes.sort_by_key(|note| (note.start, note.pitch));
                notes
            }
        }
    }

    /// A short description, for the chord sheet's style menu.
    pub fn describe(self, organ: bool) -> &'static str {
        match (self, organ) {
            (PadPart::Held | PadPart::Accents, false) => "strings",
            (PadPart::Held | PadPart::Accents, true) => "an organ",
            (PadPart::Bubble, _) => "an organ bubbling",
        }
    }
}

/// How hard pads play: softly, under the band.
const PAD_VELOCITY: u8 = 72;

impl Extras {
    /// The guitar in `variation`, if it plays there.
    pub fn guitar_in(&self, variation: Variation) -> Option<Extra<GuitarPart>> {
        self.guitar
            .filter(|guitar| plays_in(guitar.only_in_b, variation))
            .map(|guitar| Extra {
                part: guitar.part.in_variation(variation),
                ..guitar
            })
    }

    /// The strings in `variation`, if they play there.
    pub fn strings_in(&self, variation: Variation) -> Option<Extra<PadPart>> {
        self.strings
            .filter(|strings| plays_in(strings.only_in_b, variation))
    }

    /// The organ in `variation`, if it plays there.
    pub fn organ_in(&self, variation: Variation) -> Option<Extra<PadPart>> {
        self.organ
            .filter(|organ| plays_in(organ.only_in_b, variation))
    }

    /// The players, in words: "a strummed guitar, and strings in
    /// variation b". Empty if there are none.
    pub fn describe(&self) -> String {
        let in_b = |only_in_b: bool| match only_in_b {
            true => tr(" in variation b"),
            false => "",
        };
        let mut players: Vec<String> = Vec::new();
        if let Some(guitar) = self.guitar {
            players.push(format!(
                "{}{}",
                tr(guitar.part.describe()),
                in_b(guitar.only_in_b)
            ));
        }
        if let Some(strings) = self.strings {
            players.push(format!(
                "{}{}",
                tr(strings.part.describe(false)),
                in_b(strings.only_in_b)
            ));
        }
        if let Some(organ) = self.organ {
            players.push(format!(
                "{}{}",
                tr(organ.part.describe(true)),
                in_b(organ.only_in_b)
            ));
        }
        list(&players)
    }
}

fn plays_in(only_in_b: bool, variation: Variation) -> bool {
    !only_in_b || variation == Variation::B
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::band::BandStyle;

    #[test]
    fn every_style_listed_is_a_band_style() {
        for (name, _) in BY_STYLE {
            assert!(BandStyle::named(name).is_some(), "{name}");
        }
    }

    #[test]
    fn styles_say_who_plays() {
        assert_eq!(
            of("Pop beat").describe(),
            "a strummed guitar and strings in variation b"
        );
        assert_eq!(
            of("One drop").describe(),
            "a guitar on beats 2 and 4 and an organ bubbling"
        );
        assert_eq!(of("Techno").describe(), "");
    }

    #[test]
    fn a_player_for_the_chorus_waits_for_variation_b() {
        let pop = of("Pop beat");
        assert!(pop.strings_in(Variation::A).is_none());
        assert!(pop.strings_in(Variation::B).is_some());
        assert!(pop.guitar_in(Variation::A).is_some());
        // Held power chords are played as eighths in variation b.
        let rock = of("Half-time rock");
        assert_eq!(
            rock.guitar_in(Variation::B).map(|guitar| guitar.part),
            Some(GuitarPart::Power(Comping::Pulse))
        );
    }
}
