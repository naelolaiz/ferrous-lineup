//! A horn section the band can bring to any style.
//!
//! Soul and funk records, big bands, salsa orchestras, ska bands and the
//! brass bands of the Balkans all have one: trumpets, saxophones and
//! trombones playing the chords together. Switched on in the Band menu
//! (see [`Project::set_horns`](crate::Project::set_horns)), the horns
//! play the chord sheet with whatever style the band plays, in a part
//! that suits it: long notes under a ballad, the big band's kicks in
//! swing, punches in 3 + 3 + 2 in soul, rock and salsa, and the
//! afterbeats of a brass band in a march, a polka or a Balkan dance.
//!
//! ## Close harmony
//!
//! The horns play each chord in *close position*: its notes as near
//! together as they go, all inside an octave, under the highest horn,
//! the lead. A triad takes three horns, a seventh chord four. A bigger
//! chord leaves out its fifth first, and then its root, which the bass
//! plays: what is left (the third, the seventh and the notes above) is
//! what makes the chord sound as it does. A C13 is played E, Bb, D, A.
//!
//! From chord to chord the horns take the voicing that moves least, each
//! horn to the nearest note it can, and where they can they never move
//! two horns a perfect fifth apart to another perfect fifth: those
//! *parallel fifths* make two parts sound like one. From G7 to C, the
//! lead goes from B up to C and the horn on F down to E, as the leading
//! tone and the seventh of a dominant do. The lead stays between F4 and
//! F5.

use crate::Note;
use crate::general_midi as gm;
use crate::library::Genre;
use crate::library::comping::{BassPart, Comping, Played};
use crate::meter::{Bar, TimeSignature};
use crate::pattern::NoteEvent;

use super::BandStyle;

/// What the horns play.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HornPart {
    /// Long notes: each chord held for as long as it lasts.
    Pads,
    /// Short chords in the rhythm of a way of comping.
    Hits(Comping),
}

/// The horn section of a style: what it plays, and on which General
/// MIDI instrument.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Horns {
    pub part: HornPart,
    pub program: u8,
}

/// The most horns: four, as in a section of trumpet, alto and tenor
/// saxophones and trombone.
const MOST: usize = 4;
/// The lowest and the highest note the lead plays: F4 and F5.
const LEAD_LOW: u8 = 65;
const LEAD_HIGH: u8 = 77;
/// The note the lead starts nearest to: C5.
const LEAD_START: u8 = 72;
/// What each pair of horns moving in parallel fifths costs, in
/// semitones of movement: more than any other way to the next chord.
const PARALLEL_FIFTHS: u32 = 24;
/// How hard the horns hold their long notes: under the band.
const PAD_VELOCITY: u8 = 68;
/// The longest a hit lasts: an eighth note.
const HIT_STEPS: usize = 2;

/// The styles whose horns hold long notes, whatever their genre: the
/// ballads and the slow, spacious grooves.
const HELD: [&str; 13] = [
    "Pop ballad",
    "Jazz ballad",
    "Slow blues",
    "Neo soul",
    "R&B",
    "Lo-fi",
    "Trance",
    "Synthwave",
    "Bossa nova",
    "Bolero",
    "Half-time rock",
    "Epic toms",
    "Big band ballad",
];

/// The styles whose horns play another rhythm than their genre's.
const BY_STYLE: [(&str, Comping); 16] = [
    // Funk horns play tight sixteenth-note stabs.
    ("Funk", Comping::Funk),
    // A New Orleans brass band.
    ("Second line", Comping::Tresillo),
    // The saxophones of early rock and roll swing with the drums.
    ("Rock and roll", Comping::Charleston),
    // Ska horns punch the offbeats with the guitar's skank.
    ("Ska", Comping::Offbeat),
    ("Dancehall", Comping::Tresillo),
    ("Reggaeton", Comping::Dembow),
    ("Moombahton", Comping::Dembow),
    // The afterbeats of a brass band: 2 and 3 in a waltz, 2 in a polka.
    ("Waltz", Comping::Aksak),
    ("Two-four polka", Comping::Aksak),
    // In 2/4 and 6/8, and in a march's or ragtime's 2/4 doubled up, after
    // each beat.
    ("March", Comping::Afterbeat),
    ("Cakewalk", Comping::Afterbeat),
    ("Six-eight march", Comping::Afterbeat),
    ("Pasodoble", Comping::Afterbeat),
    // Klezmer horns lean on the bulgar's 3 + 3 + 2.
    ("Freylekhs", Comping::Tresillo),
    // Mambo brass punches in the clave's rhythm.
    ("Mambo", Comping::Tresillo),
    // A big band riffs as in its shout chorus.
    ("Big band swing", Comping::Shout),
];

impl Horns {
    /// The horn section of `style`: French horns in classical music,
    /// synth brass in electronic music, the saxophones of a big band's
    /// ballad, a brass section elsewhere.
    pub fn of(style: &BandStyle) -> Horns {
        let program = match style.genre {
            _ if style.name == "Big band ballad" => gm::ALTO_SAX,
            Genre::Classical => gm::FRENCH_HORN,
            Genre::Electronic => gm::SYNTH_BRASS,
            _ => gm::BRASS_SECTION,
        };
        Horns {
            part: HornPart::of(style),
            program,
        }
    }

    /// The horns' notes for `harmonies`, voiced by [`voiced`], in `bars`.
    pub(crate) fn play(self, bars: &[Bar], harmonies: &[Played]) -> Vec<NoteEvent> {
        match self.part {
            HornPart::Pads => held(harmonies),
            HornPart::Hits(comping) => {
                let mut notes = Vec::new();
                // A rhythm of four beats does not fit a bar of another
                // meter: there the horns punch the beats the bar is felt
                // in.
                let fits = |bar: &Bar| fits_any_bar(comping) || bar.time == TimeSignature::COMMON;
                for run in bars.chunk_by(|a, b| fits(a) == fits(b)) {
                    let (Some(first), Some(last)) = (run.first(), run.last()) else {
                        continue;
                    };
                    let comping = if fits(first) {
                        comping
                    } else {
                        Comping::Accents
                    };
                    let played = super::cut(harmonies, first.start, last.end());
                    notes.extend(comping.play_in_bars(run, &played));
                }
                for note in &mut notes {
                    note.length = note.length.min(HIT_STEPS);
                }
                notes
            }
        }
    }

    /// The section in words, for the Band menu: "the horns (Brass
    /// Section) play short punches in 3 + 3 + 2".
    pub fn describe(self) -> String {
        crate::i18n::trf(
            "the horns ({}) play {}",
            &[
                &crate::i18n::tr(gm::name(self.program)),
                &crate::i18n::tr(self.part.describe()),
            ],
        )
    }
}

/// Whether a way of comping follows the bar, whatever its meter.
fn fits_any_bar(comping: Comping) -> bool {
    matches!(comping, Comping::Aksak | Comping::Accents)
}

/// The horns holding each of `harmonies`, softly.
pub(crate) fn held(harmonies: &[Played]) -> Vec<NoteEvent> {
    let mut notes = Comping::Held.play_in_bars(&[], harmonies);
    for note in &mut notes {
        note.velocity = PAD_VELOCITY;
    }
    notes
}

impl HornPart {
    /// What the horns of `style` play.
    fn of(style: &BandStyle) -> HornPart {
        if style.rhythm.is_none() || HELD.contains(&style.name) {
            return HornPart::Pads;
        }
        if let Some(&(_, comping)) = BY_STYLE.iter().find(|(name, _)| *name == style.name) {
            return HornPart::Hits(comping);
        }
        let time = style.time;
        if time != TimeSignature::COMMON {
            let groups = time.felt_groups();
            let even = groups.windows(2).all(|pair| pair[0] == pair[1]);
            return match style.genre {
                // The "kontra" of a Balkan brass band: the horns after
                // the first note of each beat, the tuba on it.
                Genre::Balkan => HornPart::Hits(Comping::Aksak),
                // In 3/4 and 6/8, long notes.
                _ if even => HornPart::Pads,
                // In 5/4 and other odd meters, a punch on each beat.
                _ => HornPart::Hits(Comping::Accents),
            };
        }
        match style.genre {
            Genre::Jazz | Genre::Blues => HornPart::Hits(Comping::Charleston),
            Genre::Electronic => HornPart::Hits(Comping::Stabs),
            Genre::Tango | Genre::Folklore | Genre::Folk | Genre::Classical | Genre::Reggae => {
                HornPart::Pads
            }
            Genre::Pop
            | Genre::Rock
            | Genre::Soul
            | Genre::HipHop
            | Genre::Latin
            | Genre::Flamenco
            | Genre::Balkan => HornPart::Hits(Comping::Tresillo),
        }
    }

    /// What it plays, in words.
    pub fn describe(self) -> &'static str {
        match self {
            HornPart::Pads => "long notes held under the band",
            HornPart::Hits(Comping::Charleston) => {
                "big-band kicks on beat 1 and just before beat 3"
            }
            HornPart::Hits(Comping::Tresillo) => "short punches in 3 + 3 + 2",
            HornPart::Hits(Comping::Funk) => "tight funk stabs in sixteenth notes",
            HornPart::Hits(Comping::Stabs) => "syncopated stabs, as in house music",
            HornPart::Hits(Comping::Dembow) => "short chords in the dembow's 3 + 3 + 2",
            HornPart::Hits(Comping::Offbeat) => "short chords between the beats",
            HornPart::Hits(Comping::Afterbeat) => "afterbeats, the \"pah\" after each beat",
            HornPart::Hits(Comping::Shout) => "two-bar riffs, as in a big band's shout chorus",
            HornPart::Hits(Comping::Aksak) => "afterbeats, the \"pah\" of a brass band's oom-pah",
            HornPart::Hits(Comping::Accents) => "a punch on each beat the bar is felt in",
            HornPart::Hits(_) => "short chords",
        }
    }
}

/// The horns' voicings of `harmonies`, in close position under the lead,
/// each moving as little as it can from the one before (see the module
/// documentation). A harmony with no notes stays silent.
pub(crate) fn voiced(harmonies: &[Played]) -> Vec<Played> {
    let mut last: Option<Vec<u8>> = None;
    harmonies
        .iter()
        .map(|played| {
            let voicing = choose(&chord_tones(played), last.as_deref());
            if !voicing.is_empty() {
                last = Some(voicing.clone());
            }
            Played {
                voicing: voicing.into_iter().map(Note::new).collect(),
                ..played.clone()
            }
        })
        .collect()
}

/// The pitch classes the horns play of `played`, as semitones above C,
/// from the top of the piano's voicing down: at most four, leaving out
/// the fifth and then the root of a bigger chord.
fn chord_tones(played: &Played) -> Vec<u8> {
    let mut classes: Vec<u8> = Vec::new();
    for note in played.voicing.iter().rev() {
        let class = note.midi() % 12;
        if !classes.contains(&class) {
            classes.push(class);
        }
    }
    let root = match played.bass {
        BassPart::Chord { chord, .. } => Some(chord.root.semitone()),
        BassPart::Pedal(note) => Some(note.midi() % 12),
        BassPart::Rest => None,
    };
    if let Some(root) = root {
        for left_out in [(root + 7) % 12, root] {
            if classes.len() > MOST {
                classes.retain(|&class| class != left_out);
            }
        }
    }
    classes.truncate(MOST);
    classes
}

/// The voicing of `classes` that moves least from `last`, low to high,
/// as MIDI notes. Empty if there are no classes.
fn choose(classes: &[u8], last: Option<&[u8]>) -> Vec<u8> {
    (LEAD_LOW..=LEAD_HIGH)
        .filter(|lead| classes.contains(&(lead % 12)))
        .map(|lead| close(lead, classes))
        .min_by_key(|voicing| cost(voicing, last))
        .unwrap_or_default()
}

/// `classes` in close position under `lead`: from the top down, each
/// horn on the nearest note below the one above it that no horn plays
/// yet. Low to high.
fn close(lead: u8, classes: &[u8]) -> Vec<u8> {
    let mut notes = vec![lead];
    let mut above = lead;
    while notes.len() < classes.len() {
        let Some(below) = (1..12)
            .filter_map(|down| above.checked_sub(down))
            .find(|note| {
                classes.contains(&(note % 12)) && notes.iter().all(|other| other % 12 != note % 12)
            })
        else {
            break;
        };
        notes.push(below);
        above = below;
    }
    notes.reverse();
    notes
}

/// How far `voicing` moves from `last`, in semitones, with each pair of
/// horns that moves in parallel fifths costing [`PARALLEL_FIFTHS`] more.
/// The horns are matched from the top down; a horn `last` did not have
/// comes in from the lowest one. With no `last`, how far the lead is
/// from [`LEAD_START`].
fn cost(voicing: &[u8], last: Option<&[u8]>) -> u32 {
    let Some(last) = last.filter(|last| !last.is_empty()) else {
        return voicing
            .last()
            .map_or(0, |lead| u32::from(lead.abs_diff(LEAD_START)));
    };
    let moved: u32 = (0..voicing.len())
        .filter_map(|index| {
            let to = horn(voicing, index)?;
            let from = horn(last, index).unwrap_or(last[0]);
            Some(u32::from(to.abs_diff(from)))
        })
        .sum();
    moved + PARALLEL_FIFTHS * parallel_fifths(last, voicing)
}

/// The note of horn `index`, counted from the top, in `notes`.
fn horn(notes: &[u8], index: usize) -> Option<u8> {
    notes.len().checked_sub(index + 1).map(|at| notes[at])
}

/// How many pairs of horns go from a perfect fifth apart in `last` to
/// another perfect fifth in `voicing`, moving.
fn parallel_fifths(last: &[u8], voicing: &[u8]) -> u32 {
    let shared = voicing.len().min(last.len());
    let mut fifths = 0;
    for upper in 0..shared {
        for lower in upper + 1..shared {
            let pair = |notes: &[u8]| Some((horn(notes, upper)?, horn(notes, lower)?));
            let (Some((a, b)), Some((c, d))) = (pair(last), pair(voicing)) else {
                continue;
            };
            if a - b == 7 && c - d == 7 && a != c {
                fifths += 1;
            }
        }
    }
    fifths
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PitchClass;
    use crate::harmony::{Chord, ChordQuality};

    /// A harmony of `chord`, voiced as a piano would, from its root up.
    fn played(chord: Chord, start: usize) -> Played {
        let root = 48 + chord.root.semitone();
        let voicing: Vec<Note> = chord
            .quality
            .intervals()
            .iter()
            .map(|&interval| Note::new(root + interval))
            .collect();
        Played {
            start,
            end: start + 16,
            voicing,
            bass: BassPart::Chord {
                chord,
                note: Note::new(36 + chord.root.semitone()),
                next: Note::new(36 + chord.root.semitone()),
            },
        }
    }

    fn notes(played: &Played) -> Vec<String> {
        played.voicing.iter().map(Note::to_string).collect()
    }

    #[test]
    fn the_horns_move_as_little_as_they_can() {
        use ChordQuality::*;
        let chords = [
            Chord::new(PitchClass::C, Major),
            Chord::new(PitchClass::GSharp, Major),
            Chord::new(PitchClass::G, Dominant7),
            Chord::new(PitchClass::C, Major),
        ];
        let harmonies: Vec<Played> = chords
            .iter()
            .enumerate()
            .map(|(index, &chord)| played(chord, index * 16))
            .collect();
        let horns = voiced(&harmonies);
        // C with the lead on C5, then Ab: E and G go down and up a
        // semitone to Eb and Ab over the held C.
        assert_eq!(notes(&horns[0]), ["E4", "G4", "C5"]);
        assert_eq!(notes(&horns[1]), ["D#4", "G#4", "C5"]);
        // G7 takes four horns, and resolves to C: B up to C, F down to E.
        assert_eq!(horns[2].voicing.len(), 4);
        let g7: Vec<String> = notes(&horns[2]);
        assert_eq!(g7, ["D4", "F4", "G4", "B4"]);
        assert_eq!(notes(&horns[3]), ["E4", "G4", "C5"]);
    }

    #[test]
    fn the_horns_never_move_in_parallel_fifths_when_they_can_help_it() {
        use ChordQuality::*;
        // C to D minor in close position, root position to root position,
        // would be C-G to D-A.
        let harmonies = [
            played(Chord::new(PitchClass::C, Major), 0),
            played(Chord::new(PitchClass::D, Minor), 16),
            played(Chord::new(PitchClass::E, Minor), 32),
            played(Chord::new(PitchClass::F, Major), 48),
        ];
        let horns = voiced(&harmonies);
        for pair in horns.windows(2) {
            let (from, to) = (&pair[0].voicing, &pair[1].voicing);
            let midi = |notes: &[Note]| notes.iter().map(|note| note.midi()).collect::<Vec<_>>();
            assert_eq!(
                parallel_fifths(&midi(from), &midi(to)),
                0,
                "{from:?} to {to:?}"
            );
        }
    }

    #[test]
    fn a_big_chord_leaves_out_its_fifth_and_root() {
        // C13: C E G Bb D A. The horns play E, Bb, D and A.
        let voicing = [48, 52, 55, 58, 62, 69].map(Note::new).to_vec();
        let chord = Chord::new(PitchClass::C, ChordQuality::Dominant7);
        let played = Played {
            start: 0,
            end: 16,
            voicing,
            bass: BassPart::Chord {
                chord,
                note: Note::new(36),
                next: Note::new(36),
            },
        };
        let mut classes = chord_tones(&played);
        classes.sort_unstable();
        assert_eq!(classes, [2, 4, 9, 10]);
    }

    #[test]
    fn every_style_has_horns_that_fit_it() {
        let part = |name: &str| HornPart::of(BandStyle::named(name).unwrap());
        assert_eq!(part("Swing"), HornPart::Hits(Comping::Charleston));
        assert_eq!(part("Motown"), HornPart::Hits(Comping::Tresillo));
        assert_eq!(part("Jazz ballad"), HornPart::Pads);
        assert_eq!(part("Rachenitsa"), HornPart::Hits(Comping::Aksak));
        assert_eq!(part("Five-four rock"), HornPart::Hits(Comping::Accents));
        assert_eq!(part("Zamba"), HornPart::Pads);
        assert_eq!(part("Chorale"), HornPart::Pads);
        assert_eq!(part("Big band swing"), HornPart::Hits(Comping::Shout));
        assert_eq!(part("Big band ballad"), HornPart::Pads);
        let program = |name: &str| Horns::of(BandStyle::named(name).unwrap()).program;
        assert_eq!(program("Chorale"), gm::FRENCH_HORN);
        assert_eq!(program("Techno"), gm::SYNTH_BRASS);
        assert_eq!(program("Funk"), gm::BRASS_SECTION);
        assert_eq!(program("Big band ballad"), gm::ALTO_SAX);
        for name in BY_STYLE.iter().map(|(name, _)| *name).chain(HELD) {
            assert!(BandStyle::named(name).is_some(), "{name}");
        }
    }
}
