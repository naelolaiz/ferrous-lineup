//! Lead sheets for transposing instruments: the same tune and chords,
//! written in the key the player reads in.
//!
//! Many wind instruments do not sound the notes they read. A trumpet in
//! B♭ that plays a written C sounds the B♭ a whole tone lower; an alto
//! saxophone in E♭ sounds the E♭ a major sixth lower; a horn in F sounds
//! the F a fifth lower. So that the player's fingers stay the same on
//! every instrument of a family, their music is written higher by just
//! that much: a song in C is written in D for a trumpet, in A for an
//! alto saxophone and in G for a horn.
//!
//! Everything moves by the same distance: the key signature, every note
//! and every chord symbol. The new key is named as the app names keys,
//! with no more than six sharps or flats, as players expect: a song in
//! E for an alto saxophone is written in D♭ (five flats), not C♯ (seven
//! sharps). The notes and chords are spelled as at concert pitch and
//! then moved by the same letters as the key, so every interval keeps
//! its name.

use serde::{Deserialize, Serialize};

use super::spelling::Clef;
use crate::chord_sheet::HarmonyName;
use crate::harmony::Scale;
use crate::pitch::{Letter, Note, NoteName, PitchClass};

/// The instruments a lead sheet is written for, named by the note that
/// sounds when they play a written C.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Transposition {
    /// Written as it sounds: piano, guitar, bass, voice, flute, violin,
    /// trombone and every other instrument "in C".
    #[default]
    Concert,
    /// Trumpet, cornet, flugelhorn, clarinet, soprano and tenor saxophone:
    /// written a major second higher than the song sounds.
    BFlat,
    /// Alto and baritone saxophone: written a major sixth higher.
    EFlat,
    /// Horn and English horn: written a perfect fifth higher.
    F,
}

impl Transposition {
    pub const ALL: [Transposition; 4] = [
        Transposition::Concert,
        Transposition::BFlat,
        Transposition::EFlat,
        Transposition::F,
    ];

    /// The note that sounds when the part says C: the B♭ of a trumpet in
    /// B♭. It names the instruments.
    pub fn note(self) -> NoteName {
        match self {
            Transposition::Concert => NoteName::new(Letter::C, 0),
            Transposition::BFlat => NoteName::new(Letter::B, -1),
            Transposition::EFlat => NoteName::new(Letter::E, -1),
            Transposition::F => NoteName::new(Letter::F, 0),
        }
    }

    /// How many semitones higher than it sounds the music is written,
    /// leaving whole octaves aside: from the instrument's note up to C.
    pub fn semitones(self) -> u8 {
        self.note().pitch_class().semitones_up_to(PitchClass::C)
    }

    /// The key the part is written in where the song is in `key`: in D
    /// for a trumpet when the song is in C, and G minor when it is in F
    /// minor. Keys are named as everywhere in the app (see
    /// [`Scale::spelling`]), so a part never has more than six sharps
    /// or flats: a song in F♯ is written in A♭ for a trumpet, not G♯.
    pub fn key(self, key: Scale) -> Scale {
        Scale::new(key.root.transpose(i32::from(self.semitones())), key.kind)
    }

    /// A chord of the song where it is in `key`, as the part writes it:
    /// moved from the song's key to the part's, letters and all, so the
    /// chord keeps its place in the key. G7 in C is A7 in D, and the
    /// C♯7 of a song in F♯ becomes E♭7 in A♭. Sets, micromodes and row
    /// forms move by the same number of semitones.
    pub fn harmony(self, harmony: HarmonyName, key: Scale) -> HarmonyName {
        harmony.moved(key, self.key(key))
    }

    /// A note of the song spelled `name` where the song is in `key`, as
    /// the part writes it: moved by the same letters and semitones as
    /// the key, so every interval keeps its name. The D♯ that leans on
    /// the E of a song in A minor is E♯ leaning on F♯ in B minor, not F
    /// going to F♯; the G♯ of a song in E is F in D♭.
    ///
    /// A name that would need two sharps or flats takes the simpler one,
    /// as chord symbols do, unless the part's key has it so: the C of a
    /// song in E (its lowered sixth) is A in D♭, not B𝄫, but the B♯ of C♯
    /// harmonic minor is C𝄪, the leading tone of D♯ harmonic minor. A
    /// row or a set has no letters of its own, so its notes take their
    /// usual names in the new key.
    pub fn name(self, name: NoteName, key: Scale) -> NoteName {
        let written = self.key(key);
        if written == key {
            return name;
        }
        if !key.as_key().kind.is_tonal() {
            return written.spell(name.pitch_class().transpose(i32::from(self.semitones())));
        }
        let (from, to) = (key.spell(key.root), written.spell(written.root));
        let moved = name.above(
            from.pitch_class().semitones_up_to(to.pitch_class()),
            from.letter.steps_up_to(to.letter),
        );
        if moved.accidental.abs() >= 2 && written.spell(moved.pitch_class()) != moved {
            moved.simplified()
        } else {
            moved
        }
    }

    /// How many semitones higher than they sound the notes `pitches` are
    /// written on a staff in `clef`.
    ///
    /// The part is written by the instrument's interval, moved by whole
    /// octaves so that the middle note of the tune sits as near as it can
    /// to the middle line of the staff: the octave a player reads best.
    /// This is how the books of lead sheets made for these instruments
    /// are written; each player then plays the tune in the octave that
    /// suits the instrument. A tune in the range of a voice is written a
    /// major second up for B♭ instruments: a trumpet sounds it where it
    /// is, and a tenor saxophone, reading the same page, an octave lower.
    /// Of two octaves equally near, the one nearer the instrument's own
    /// interval is taken.
    ///
    /// Concert pitch is written as it sounds, in its own octave.
    pub fn shift(self, pitches: &[Note], clef: Clef) -> i32 {
        let interval = i32::from(self.semitones());
        if self == Transposition::Concert {
            return 0;
        }
        let mut sorted = pitches.to_vec();
        sorted.sort();
        let Some(median) = sorted.get(sorted.len() / 2) else {
            return interval;
        };
        let middle = i32::from(clef.middle_line().midi());
        let median = i32::from(median.midi());
        (-4..=4)
            .map(|octaves| interval + 12 * octaves)
            .min_by_key(|shift| ((median + shift - middle).abs(), (shift - interval).abs()))
            .unwrap_or(interval)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harmony::ScaleKind;
    use PitchClass::*;
    use Transposition::{BFlat, Concert, EFlat};

    fn key_name(transposition: Transposition, root: PitchClass, kind: ScaleKind) -> String {
        transposition.key(Scale::new(root, kind)).name()
    }

    fn chord(transposition: Transposition, text: &str, root: PitchClass) -> String {
        let harmony = HarmonyName::parse(text).unwrap();
        transposition
            .harmony(harmony, Scale::new(root, ScaleKind::Major))
            .to_string()
    }

    #[test]
    fn each_instrument_writes_up_by_its_interval() {
        assert_eq!(Concert.semitones(), 0);
        // Bb up to C is a major second, Eb up to C a major sixth, F up
        // to C a perfect fifth.
        assert_eq!(BFlat.semitones(), 2);
        assert_eq!(EFlat.semitones(), 9);
        assert_eq!(Transposition::F.semitones(), 7);
        assert_eq!(BFlat.note().to_string(), "Bb");
    }

    #[test]
    fn a_song_in_c_is_written_in_d_a_and_g() {
        assert_eq!(key_name(Concert, C, ScaleKind::Major), "C major");
        assert_eq!(key_name(BFlat, C, ScaleKind::Major), "D major");
        assert_eq!(key_name(EFlat, C, ScaleKind::Major), "A major");
        assert_eq!(key_name(Transposition::F, C, ScaleKind::Major), "G major");
        // Minor keys and modes keep their kind.
        assert_eq!(key_name(BFlat, F, ScaleKind::NaturalMinor), "G minor");
        assert_eq!(key_name(EFlat, D, ScaleKind::Dorian), "B Dorian");
    }

    #[test]
    fn written_keys_never_need_more_than_six_sharps_or_flats() {
        // A song in E for an alto saxophone is in Db, not C#; in F# for
        // a trumpet it is in Ab, not G#; in B for a horn, in F#.
        assert_eq!(key_name(EFlat, E, ScaleKind::Major), "Db major");
        assert_eq!(key_name(BFlat, FSharp, ScaleKind::Major), "Ab major");
        assert_eq!(key_name(Transposition::F, B, ScaleKind::Major), "F# major");
        assert_eq!(key_name(BFlat, ASharp, ScaleKind::Major), "C major");
        for transposition in Transposition::ALL {
            for root in 0..12 {
                for kind in [ScaleKind::Major, ScaleKind::NaturalMinor] {
                    let key = transposition.key(Scale::new(PitchClass::from_semitone(root), kind));
                    let (count, _) = key.key_signature().unwrap();
                    assert!(count.abs() <= 6, "{}", key.name());
                }
            }
        }
    }

    #[test]
    fn chords_keep_their_place_in_the_key() {
        // ii V I in C: Dm7 G7 Cmaj7 is Em7 A7 Dmaj7 for a trumpet.
        assert_eq!(chord(BFlat, "Dm7", C), "Em7");
        assert_eq!(chord(BFlat, "G7", C), "A7");
        assert_eq!(chord(EFlat, "Cmaj7", C), "Amaj7");
        assert_eq!(chord(Transposition::F, "G7/B", C), "D7/F#");
        // The V of F# major for a trumpet in Ab major is Eb7, not D#7.
        assert_eq!(chord(BFlat, "C#7", FSharp), "Eb7");
        // In Db major for an alto saxophone, the V7 of a song in E.
        assert_eq!(chord(EFlat, "B7", E), "Ab7");
        // Chords from outside the key keep their place too: the bVII
        // of C (Bb) is the bVII of A (G).
        assert_eq!(chord(EFlat, "Bb", C), "G");
        // Two chords at once, and no chord, are written as well (the
        // key of Eb is the pitch class D#, which the app calls Eb).
        assert_eq!(chord(BFlat, "Eb7|Fb", DSharp), "F7|Gb");
        assert_eq!(chord(BFlat, "N.C.", C), "N.C.");
    }

    #[test]
    fn notes_keep_their_intervals() {
        let name = |transposition: Transposition, text: &str, root, kind| {
            let key = Scale::new(root, kind);
            transposition
                .name(NoteName::parse(text).unwrap(), key)
                .to_string()
        };
        let minor = ScaleKind::NaturalMinor;
        // D# leaning on E in A minor: E# on F# for a trumpet, B# on C#
        // for an alto saxophone, A# on B for a horn.
        assert_eq!(name(BFlat, "D#", A, minor), "E#");
        assert_eq!(name(BFlat, "E", A, minor), "F#");
        assert_eq!(name(EFlat, "D#", A, minor), "B#");
        assert_eq!(name(Transposition::F, "D#", A, minor), "A#");
        // A song in E for an alto saxophone is in Db: its G# is F, its
        // D# is C, and its lowered sixth C is A rather than Bbb.
        assert_eq!(name(EFlat, "G#", E, ScaleKind::Major), "F");
        assert_eq!(name(EFlat, "D#", E, ScaleKind::Major), "C");
        assert_eq!(name(EFlat, "C", E, ScaleKind::Major), "A");
        // The leading tone of C# harmonic minor, B#, is C## in D# minor.
        assert_eq!(name(BFlat, "B#", CSharp, ScaleKind::HarmonicMinor), "C##");
        // A row has no letters: its notes take their usual names.
        let row = Scale::row(crate::harmony::ToneRow::CHROMATIC);
        assert_eq!(
            BFlat.name(NoteName::parse("Eb").unwrap(), row).to_string(),
            "F"
        );
        assert_eq!(
            Concert
                .name(NoteName::parse("D#").unwrap(), row)
                .to_string(),
            "D#"
        );
    }

    #[test]
    fn concert_pitch_changes_nothing() {
        let harmony = HarmonyName::parse("F#m7b5").unwrap();
        let key = Scale::new(E, ScaleKind::NaturalMinor);
        assert_eq!(Concert.harmony(harmony, key), harmony);
        assert_eq!(Concert.key(key), key);
        assert_eq!(Concert.shift(&[Note::new(40)], Clef::Treble), 0);
    }

    #[test]
    fn the_tune_sits_in_the_middle_of_the_staff() {
        let notes = |midi: &[u8]| midi.iter().map(|&m| Note::new(m)).collect::<Vec<_>>();
        // A tune around G4 is written a major second up for a trumpet,
        // a major sixth up for an alto saxophone, a fifth up for a horn:
        // the octave each of them sounds it in.
        let tune = notes(&[60, 64, 67, 69, 72]);
        assert_eq!(BFlat.shift(&tune, Clef::Treble), 2);
        assert_eq!(EFlat.shift(&tune, Clef::Treble), 9);
        assert_eq!(Transposition::F.shift(&tune, Clef::Treble), 7);
        // A high tune (around C6) is written lower: a minor seventh down
        // for a B-flat instrument puts its middle note on D5.
        let high = notes(&[81, 84, 86]);
        assert_eq!(BFlat.shift(&high, Clef::Treble), -10);
        // A low one (a bass line around C3) is written two octaves and a
        // whole tone up on the treble staff, or a whole tone up on the
        // bass staff.
        let low = notes(&[43, 48, 52]);
        assert_eq!(BFlat.shift(&low, Clef::Treble), 26);
        assert_eq!(BFlat.shift(&low, Clef::Bass), 2);
        // With no tune, only the interval.
        assert_eq!(EFlat.shift(&[], Clef::Treble), 9);
    }
}
