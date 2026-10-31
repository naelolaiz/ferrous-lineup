//! Harmony styles: a chord progression, and the way a genre plays it.
//!
//! A style is three things: a progression written in Roman numerals (so
//! it works in any key), a way of playing the chords ([`Comping`]) and a
//! bass line ([`Bass`]). Playing "I V vi IV" as held piano chords makes a
//! ballad; playing it as offbeat stabs over a bass on the offbeats makes
//! house music. The tables below are the library: 215 styles,
//! each with a sentence on what makes it sound the way it does.

use crate::harmony::{Chord, Scale, ScaleKind};
use crate::meter::TimeSignature;
use crate::pitch::PitchClass;

use super::Genre;
use super::chart::{self, Backing, ChordSpan};
use super::comping::{Bass, Comping};

/// One harmony style.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub name: &'static str,
    pub genre: Genre,
    /// The mode the chords are written in: the project's scale becomes
    /// this mode when the style is used.
    pub mode: ScaleKind,
    pub time: TimeSignature,
    /// The progression, as a chord chart (see [`chart::parse_chart`]):
    /// numerals counted from the tonic, bars separated by `|`.
    pub chart: &'static str,
    pub comping: Comping,
    pub bass: Bass,
    /// The tempo the style is usually played at, in BPM.
    pub tempo: f32,
    /// The name of the rhythm in [`super::RHYTHMS`] that goes with it, or
    /// "" for none.
    pub rhythm: &'static str,
    /// One or two sentences on what to listen for.
    pub about: &'static str,
}

impl Style {
    /// The chords with their times, in bars of this style's time signature.
    pub fn spans(&self) -> Vec<ChordSpan> {
        chart::parse_chart(self.chart, self.time.steps_per_bar()).unwrap_or_default()
    }

    /// How many bars the progression lasts.
    pub fn bars(&self) -> usize {
        self.chart.split('|').count()
    }

    /// How many steps the progression lasts.
    pub fn length(&self) -> usize {
        self.bars() * self.time.steps_per_bar()
    }

    /// The scale the style is in, when its tonic is `tonic`.
    pub fn scale(&self, tonic: PitchClass) -> Scale {
        Scale::new(tonic, self.mode)
    }

    /// The chords of the progression in a key, one per change.
    pub fn chords_in(&self, tonic: PitchClass) -> Vec<Chord> {
        self.spans()
            .iter()
            .map(|span| span.chord.in_key(tonic))
            .collect()
    }

    /// The progression's chord names in a key, such as "Am – F – C – G",
    /// spelled as the key writes them (see [`Scale::chord_name`]):
    /// chords written with a flat, like bVII, are spelled with flats, and
    /// the V of D minor is A, with its C#.
    pub fn chord_names(&self, tonic: PitchClass) -> String {
        let scale = self.scale(tonic);
        self.spans()
            .iter()
            .map(|span| scale.chord_name(span.chord.in_key(tonic)))
            .collect::<Vec<_>>()
            .join(" – ")
    }

    /// The chords and the bass line, played in the key of `tonic`.
    pub fn render(&self, tonic: PitchClass) -> Backing {
        chart::accompany(&self.spans(), tonic, self.time, self.comping, self.bass)
    }

    /// A key that suits the style, for the demos: A for the minor modes,
    /// E for Phrygian (as on the Spanish guitar), C for the rest.
    pub fn home_tonic(&self) -> PitchClass {
        match self.mode {
            ScaleKind::NaturalMinor
            | ScaleKind::HarmonicMinor
            | ScaleKind::MelodicMinor
            | ScaleKind::Dorian
            | ScaleKind::MinorPentatonic
            | ScaleKind::Blues => PitchClass::A,
            ScaleKind::Phrygian | ScaleKind::Locrian => PitchClass::E,
            _ => PitchClass::C,
        }
    }

    /// The style called `name`, if there is one.
    pub fn named(name: &str) -> Option<&'static Style> {
        STYLES.iter().find(|style| style.name == name)
    }

    /// Whether it follows the song (its drums follow the meter, see
    /// [`super::Rhythm::follows_the_meter`]): it plays in whatever meter
    /// each bar is in, so the song keeps its time signatures and tempo,
    /// and a band in it plays along with the song's own parts.
    pub fn follows_the_song(&self) -> bool {
        super::Rhythm::named(self.rhythm).is_some_and(|rhythm| rhythm.follows_the_meter)
    }

    /// The same style in another time signature.
    const fn in_time(self, time: TimeSignature) -> Style {
        Style { time, ..self }
    }
}

/// A shorter way to write a 4/4 style in the table below.
#[allow(clippy::too_many_arguments)]
const fn style(
    name: &'static str,
    genre: Genre,
    mode: ScaleKind,
    chart: &'static str,
    comping: Comping,
    bass: Bass,
    tempo: f32,
    rhythm: &'static str,
    about: &'static str,
) -> Style {
    Style {
        name,
        genre,
        mode,
        time: TimeSignature::COMMON,
        chart,
        comping,
        bass,
        tempo,
        rhythm,
        about,
    }
}

const TWO_FOUR: TimeSignature = TimeSignature::new(2, 4);
const THREE_FOUR: TimeSignature = TimeSignature::new(3, 4);
const SIX_EIGHT: TimeSignature = TimeSignature::new(6, 8);
const TWELVE_EIGHT: TimeSignature = TimeSignature::new(12, 8);
/// Flamenco's twelve beats, counted from 12: 12 1 2, 3 4 5, 6 7, 8 9, 10 11.
const COMPAS: TimeSignature = TimeSignature::of_groups(&[3, 3, 2, 2, 2], 8);

use Genre::*;
use ScaleKind::{Dorian, HarmonicMinor, Lydian, Major, Mixolydian, NaturalMinor, Phrygian};

/// The library of harmony styles, sorted by genre.
pub static STYLES: &[Style] = &[
    // ----- Pop -----
    style(
        "Four chords",
        Pop,
        Major,
        "I | V | vi | IV",
        Comping::Pulse,
        Bass::Pulse,
        110.0,
        "Pop beat",
        "The four chords behind hundreds of hits: home, the dominant, the sad relative minor, \
         and the subdominant.",
    ),
    style(
        "Emotional pop",
        Pop,
        Major,
        "vi | IV | I | V",
        Comping::Ballad,
        Bass::Held,
        76.0,
        "Pop ballad",
        "The same four chords starting on the minor one, which makes them wistful instead of bright.",
    ),
    style(
        "Doo-wop",
        Pop,
        Major,
        "I | vi | IV | V",
        Comping::Shuffle,
        Bass::Quarters,
        92.0,
        "Shuffle",
        "The progression of the 1950s, in a slow swing with the bass on every beat.",
    ),
    style(
        "Royal road",
        Pop,
        Major,
        "IVmaj7 | V7 | iii7 | vi7",
        Comping::Stabs,
        Bass::Octaves,
        124.0,
        "Dance pop",
        "IV V iii vi, Japan's favourite progression, heard in countless anime and J-pop songs.",
    ),
    style(
        "Pop minor",
        Pop,
        NaturalMinor,
        "i | bVII | bVI | bVII",
        Comping::Pulse,
        Bass::Pulse,
        104.0,
        "Pop beat",
        "A minor key rocking between the tonic and the two major chords just below it.",
    ),
    style(
        "Pop punk",
        Pop,
        Major,
        "I | IV | vi | V",
        Comping::Pulse,
        Bass::Pulse,
        170.0,
        "Punk",
        "Fast, loud and bright: chords on every eighth note with the bass right under them.",
    ),
    style(
        "Piano ballad",
        Pop,
        Major,
        "I | V | vi | iii | IV | I | ii | V",
        Comping::Ballad,
        Bass::Held,
        70.0,
        "Pop ballad",
        "The first six chords are those of Pachelbel's Canon: again and again, the root \
         falls a fourth, then climbs a step.",
    ),
    style(
        "Dance pop",
        Pop,
        Major,
        "IV | I | V | vi",
        Comping::Stabs,
        Bass::Offbeat,
        122.0,
        "Dance pop",
        "Starting on IV keeps the music floating: home only arrives in the second bar.",
    ),
    style(
        "Bittersweet",
        Pop,
        Major,
        "I | III | IV | iv",
        Comping::Ballad,
        Bass::Held,
        84.0,
        "Pop ballad",
        "A major III and a minor iv, both from outside the key: hopeful and sad at once.",
    ),
    style(
        "Afropop",
        Pop,
        Major,
        "I | iii7 | vi7 | IV",
        Comping::Tresillo,
        Bass::Dembow,
        105.0,
        "Afrobeats",
        "Bright chords pushed by the 3 + 3 + 2 rhythm that West African pop shares with Latin music.",
    ),
    style(
        "Six-eight ballad",
        Pop,
        Major,
        "I | vi | IV | V",
        Comping::Arp3,
        Bass::Held,
        90.0,
        "Six-eight",
        "Two beats to the bar, each split in three: the lilt of 1950s and 1960s slow dances.",
    )
    .in_time(SIX_EIGHT),
    style(
        "Plagal pop",
        Pop,
        Major,
        "I | IV | I | IV",
        Comping::Strum,
        Bass::RootFifth,
        100.0,
        "Pop beat",
        "Home and the IV chord, back and forth: the gentle \"amen\" cadence that ends hymns, stretched into a whole song.",
    ),
    style(
        "Climbing pop",
        Pop,
        Major,
        "I | ii | iii | IV",
        Comping::Pulse,
        Bass::Pulse,
        116.0,
        "Pop beat",
        "One step up the scale every bar, I ii iii IV: the music rises and asks for more.",
    ),
    style(
        "Augmented climb",
        Pop,
        Major,
        "I | I+ | I6 | I7",
        Comping::Ballad,
        Bass::Held,
        80.0,
        "Pop ballad",
        "One note climbs while the others stay: G, G sharp, A, B flat over C. The last chord, C7, leads on to F.",
    ),
    style(
        "Minor four",
        Pop,
        Major,
        "I | IV | iv | I",
        Comping::Ballad,
        Bass::Held,
        72.0,
        "Pop ballad",
        "The IV chord turns minor for a bar before coming home: a chord borrowed from the minor key, full of longing.",
    ),
    style(
        "City pop",
        Pop,
        Major,
        "IVmaj7 | III7 | vi7 | v7 I7",
        Comping::Stabs,
        Bass::Octaves,
        112.0,
        "Dance pop",
        "Japan's 1980s city pop: major sevenths and borrowed dominants. III7 leads to vi7, and v7 I7 leads back to IVmaj7.",
    ),
    style(
        "Highlife",
        Pop,
        Major,
        "I | IV | I | V7",
        Comping::ArpUp,
        Bass::Tresillo,
        116.0,
        "Highlife",
        "Ghana's highlife, from the dance bands and guitar bands of the 1950s: picked guitar \
         lines ringing over I, IV and V7, the bell's time line, and a bass in 3 + 3 + 2.",
    ),
    style(
        "Soukous",
        Pop,
        Major,
        "I | IV | V | IV",
        Comping::Arp16,
        Bass::Octaves,
        132.0,
        "Soukous",
        "Congolese soukous, the rumba of Kinshasa sped up: in the sebene, its dance section, \
         the lead guitar picks bright sixteenth notes over I, IV and V while the snare gallops.",
    ),
    style(
        "Kizomba",
        Pop,
        NaturalMinor,
        "i | bVI | bIII | bVII",
        Comping::Held,
        Bass::Tresillo,
        92.0,
        "Kizomba",
        "Angola's slow, close dance, grown in the 1980s out of semba and the zouk of the \
         French Antilles: soft synth pads hold a minor-key loop, i, flat VI, flat III and flat \
         VII, while the synth bass rolls in 3 + 3 + 2.",
    ),
    // ----- Rock -----
    style(
        "Mixolydian rock",
        Rock,
        Mixolydian,
        "I | bVII | IV | I",
        Comping::Pulse,
        Bass::Pulse,
        120.0,
        "Rock beat",
        "The flat seventh chord of the Mixolydian mode: classic rock's favourite way out of a key.",
    ),
    style(
        "Three-chord rock",
        Rock,
        Major,
        "I | IV | V | IV",
        Comping::Strum,
        Bass::Pulse,
        132.0,
        "Rock beat",
        "I, IV and V, the three major chords of a key: enough for a lifetime of rock and roll.",
    ),
    style(
        "Aeolian climb",
        Rock,
        NaturalMinor,
        "bVI | bVII | i | i",
        Comping::Pulse,
        Bass::Pulse,
        128.0,
        "Half-time rock",
        "Two major chords climbing to the minor tonic: the sound of stadium rock.",
    ),
    style(
        "Dark rock",
        Rock,
        NaturalMinor,
        "i | bIII | bVII | iv",
        Comping::Pulse,
        Bass::Pulse,
        136.0,
        "Rock ride",
        "A minor progression that circles without ever resting on the dominant.",
    ),
    style(
        "Heroic cadence",
        Rock,
        Major,
        "bVI | bVII | I | I",
        Comping::Strum,
        Bass::Pulse,
        120.0,
        "Half-time rock",
        "Two chords borrowed from the minor key land on a bright major tonic: a hero arriving.",
    ),
    style(
        "Surf rock",
        Rock,
        Major,
        "I | I | IV | IV | V | IV | I | V",
        Comping::Pulse,
        Bass::Pulse,
        160.0,
        "Surf beat",
        "Fast eighths, rumbling toms and a progression borrowed from the blues: California in the early 1960s.",
    ),
    style(
        "Rock and roll",
        Rock,
        Major,
        "I | I | I | I | IV | IV | I | I | V | IV | I | V",
        Comping::Shuffle,
        Bass::Boogie,
        168.0,
        "Rock and roll",
        "The twelve-bar blues sped up, with a boogie bass under it: the 1950s.",
    ),
    style(
        "Metal",
        Rock,
        Phrygian,
        "i | bII | i | bVII",
        Comping::Pulse,
        Bass::Pulse,
        180.0,
        "Metal",
        "The Phrygian flat II, a semitone above the root, played fast and heavy over a double bass drum.",
    ),
    style(
        "Power chords",
        Rock,
        Major,
        "I | bIII | IV | bVI bVII",
        Comping::Pulse,
        Bass::Pulse,
        140.0,
        "Rock beat",
        "Major chords borrowed from the minor key, bIII, bVI and bVII: the sound of garage rock and grunge.",
    ),
    style(
        "Five-four rock",
        Rock,
        NaturalMinor,
        "i | bVI | bVII | i",
        Comping::Aksak,
        Bass::Aksak,
        120.0,
        "Five-four rock",
        "Rock in 5/4, counted 3 + 2: the bass on 1 and 4, the chords on the beats \
         between, climbing from bVI through bVII back to i.",
    )
    .in_time(TimeSignature::of_groups(&[3, 2], 4)),
    // ----- Blues -----
    style(
        "Twelve-bar blues",
        Blues,
        Mixolydian,
        "I7 | I7 | I7 | I7 | IV7 | IV7 | I7 | I7 | V7 | IV7 | I7 | V7",
        Comping::Shuffle,
        Bass::Boogie,
        100.0,
        "Shuffle",
        "The form of the blues: twelve bars, three chords, all of them dominant sevenths, over the \
         boogie-woogie's bass, eight to the bar.",
    ),
    style(
        "Quick-change blues",
        Blues,
        Mixolydian,
        "I7 | IV7 | I7 | I7 | IV7 | IV7 | I7 | I7 | V7 | IV7 | I7 | V7",
        Comping::Charleston,
        Bass::Walking,
        120.0,
        "Swing",
        "The twelve-bar blues with a quick visit to IV in the second bar.",
    ),
    style(
        "Minor blues",
        Blues,
        Dorian,
        "i7 | i7 | i7 | i7 | iv7 | iv7 | i7 | i7 | bVI7 | V7 | i7 | V7",
        Comping::Charleston,
        Bass::Walking,
        112.0,
        "Swing",
        "Twelve bars in a minor key, with a dark flat-six chord before the turnaround.",
    ),
    style(
        "Eight-bar blues",
        Blues,
        Mixolydian,
        "I7 | V7 | IV7 | IV7 | I7 | V7 | I7 | V7",
        Comping::Shuffle,
        Bass::Walking,
        88.0,
        "Shuffle",
        "A shorter blues that already goes to the dominant in its second bar.",
    ),
    style(
        "Slow blues",
        Blues,
        Mixolydian,
        "I7 | IV7 | I7 | VI7 | ii7 | V7 | I7 | V7",
        Comping::Triplets,
        Bass::LongShort,
        66.0,
        "Slow blues",
        "Eight bars in 12/8: four slow beats, each split in three. VI7 ii7 V7 is a turnaround borrowed from jazz.",
    )
    .in_time(TWELVE_EIGHT),
    style(
        "Blues rock",
        Blues,
        Mixolydian,
        "I7 | I7 | I7 | I7 | IV7 | IV7 | I7 | I7 | V7 | IV7 | I7 | V7",
        Comping::Pulse,
        Bass::Boogie,
        132.0,
        "Rock beat",
        "The twelve-bar blues with straight rock eighths and a boogie bass, the way British bands played it in the 1960s.",
    ),
    style(
        "Jump blues",
        Blues,
        Major,
        "I6 | IV7 | I6 | I7 | IV7 | IV7 | I6 | VI7 | ii7 | V7 | I6 | V7",
        Comping::Charleston,
        Bass::Walking,
        160.0,
        "Swing",
        "The blues as a 1940s swing band played it: sixth chords, and a turnaround through VI7, ii7 and V7.",
    ),
    // ----- Jazz -----
    style(
        "ii–V–I",
        Jazz,
        Major,
        "ii7 | V7 | Imaj7 | Imaj7",
        Comping::Charleston,
        Bass::Walking,
        120.0,
        "Swing",
        "The building block of jazz: a minor seventh, a dominant seventh, and home.",
    ),
    style(
        "Minor ii–V–i",
        Jazz,
        HarmonicMinor,
        "iiø7 | V7b9 | i6 | i6",
        Comping::Charleston,
        Bass::Walking,
        116.0,
        "Swing",
        "The same move in a minor key: half-diminished, a dominant with a flat nine, and a \
         minor sixth chord.",
    ),
    style(
        "Turnaround",
        Jazz,
        Major,
        "Imaj7 | vi7 | ii7 | V7",
        Comping::Charleston,
        Bass::Walking,
        132.0,
        "Swing",
        "I vi ii V: the progression that ends so many standards and leads back to the top.",
    ),
    style(
        "Rhythm changes",
        Jazz,
        Major,
        "I VI7 | ii7 V7 | I VI7 | ii7 V7 | I I7 | IV iv | I V7 | I",
        Comping::Charleston,
        Bass::Walking,
        168.0,
        "Swing",
        "Eight bars from a 1930 show tune that jazz musicians wrote hundreds of new melodies over.",
    ),
    style(
        "Diatonic cycle",
        Jazz,
        Major,
        "Imaj7 | IVmaj7 | viiø7 | iii7 | vi7 | ii7 | V7 | Imaj7",
        Comping::Charleston,
        Bass::Walking,
        126.0,
        "Swing",
        "Every chord of the key, each a fifth below the last: the circle of fifths inside one key.",
    ),
    style(
        "Jazz ballad",
        Jazz,
        Major,
        "Imaj7 | vi7 | ii7 | V7 | iii7 | VI7 | ii7 | V7",
        Comping::Ballad,
        Bass::RootFifth,
        68.0,
        "Jazz ballad",
        "A slow progression where a secondary dominant (VI7) pulls toward ii.",
    ),
    style(
        "Modal jazz",
        Jazz,
        Dorian,
        "i7 | i7 | i7 | i7 | bii7 | bii7 | i7 | i7",
        Comping::Charleston,
        Bass::Walking,
        136.0,
        "Swing",
        "Two chords a semitone apart, each held for bars: in modal jazz, the scale matters more \
         than the changes.",
    ),
    style(
        "Jazz waltz",
        Jazz,
        Major,
        "Imaj7 | vi7 | ii7 | V7",
        Comping::Waltz,
        Bass::Walking,
        150.0,
        "Jazz waltz",
        "Jazz in three: a turnaround with a walking bass, three steps to the bar.",
    )
    .in_time(THREE_FOUR),
    style(
        "Tritone sub",
        Jazz,
        Major,
        "ii7 | bII7 | Imaj7 | Imaj7",
        Comping::Charleston,
        Bass::Walking,
        124.0,
        "Swing",
        "G7 swapped for D flat 7: both hold the same tritone, F and B, and now the bass slides down by semitones, D, D flat, C.",
    ),
    style(
        "Backdoor ii–V",
        Jazz,
        Major,
        "iv7 | bVII7 | Imaj7 | Imaj7",
        Comping::Charleston,
        Bass::Walking,
        120.0,
        "Swing",
        "Home reached by the back door: Fm7 and B flat 7, borrowed from C minor, instead of Dm7 and G7.",
    ),
    style(
        "Gypsy jazz",
        Jazz,
        HarmonicMinor,
        "i6 | i6 | V7 | V7 | V7 | V7 | i6 | i6",
        Comping::Quarters,
        Bass::RootFifth,
        180.0,
        "Swing",
        "Swing from 1930s Paris: minor sixth chords and a guitar chopping out every beat, \"la pompe\".",
    ),
    style(
        "Major-minor cycle",
        Jazz,
        Major,
        "ii7 | V7 | Imaj7 | IVmaj7 | viiø7 | III7 | vi7 | vi7",
        Comping::Charleston,
        Bass::Walking,
        132.0,
        "Swing",
        "Down the circle of fifths in the major key, then a ii–V into the relative minor: the pattern behind countless standards.",
    ),
    style(
        "Three tonics",
        Jazz,
        Major,
        "Imaj7 bIII7 | bVImaj7 VII7 | IIImaj7 V7 | Imaj7",
        Comping::Charleston,
        Bass::Walking,
        150.0,
        "Swing",
        "Three keys a major third apart, C, A flat and E, each reached by its own V7: the cycle John Coltrane made famous in 1959.",
    ),
    style(
        "New Orleans",
        Jazz,
        Major,
        "I | I | I | I7 | IV | iv | I V7 | I",
        Comping::Quarters,
        Bass::RootFifth,
        100.0,
        "Second line",
        "A brass band in the street: two beats to the bar on the tuba, and the IV chord turning minor before coming home.",
    ),
    style(
        "Minor jazz waltz",
        Jazz,
        NaturalMinor,
        "i7 | iv7 | bVII7 | bIIImaj7 | bVImaj7 | iiø7 | V7 | i7",
        Comping::Waltz,
        Bass::Walking,
        160.0,
        "Jazz waltz",
        "The whole circle of fifths in a minor key, one chord a bar, in three.",
    )
    .in_time(THREE_FOUR),
    style(
        "Five-four swing",
        Jazz,
        NaturalMinor,
        "i7 | v7 | i7 | v7 | iv7 | v7 | bVImaj7 | V7",
        Comping::Aksak,
        Bass::Aksak,
        170.0,
        "Five-four swing",
        "Jazz in 5/4, counted 3 + 2: a minor vamp from i7 to v7, then iv7 and \
         bVImaj7 lead to a V7 with the leading note, back to i7.",
    )
    .in_time(TimeSignature::of_groups(&[3, 2], 4)),
    style(
        "Five-four spy",
        Jazz,
        NaturalMinor,
        "i | i | iv | i | i | i | bVI | V",
        Comping::Aksak,
        Bass::Aksak,
        160.0,
        "Five-four spy",
        "A 1960s spy theme in 5/4: a minor chord held over the beat, moving to iv as \
         a blues does, then bVI and the V with the leading note bring it home.",
    )
    .in_time(TimeSignature::of_groups(&[3, 2], 4)),
    style(
        "Ragtime",
        Jazz,
        Major,
        "I | V7 | V7 | I | I7 | IV iv | I VI7 | II7 V7",
        Comping::Afterbeat,
        Bass::Alternating,
        96.0,
        "",
        "Scott Joplin's piano music, straight and not fast: the left hand plays a bass note on \
         every beat, the root and the fifth below by turns, and the chord between them. I7 \
         turns towards IV, the minor iv is borrowed from the minor key, and the strain comes \
         home round the circle of dominants, VI7, II7, V7. Ragtime is written in 2/4; each bar \
         here lasts two.",
    ),
    style(
        "Ragtime band",
        Jazz,
        Major,
        "I | III7 | VI7 | VI7 | II7 | V7 | I | V7",
        Comping::Afterbeat,
        Bass::Alternating,
        104.0,
        "Cakewalk",
        "The ragtime progression, a chain of dominant sevenths round the circle of fifths, \
         E7, A7, D7, G7 and home to C, each the V7 of the next. A ragtime band plays it: the \
         tuba on the beats, the piano's chords between them, and a banjo strumming across \
         them. Ragtime is written in 2/4; each bar here lasts two.",
    ),
    style(
        "Stride piano",
        Jazz,
        Major,
        "I | I7 | IV | #iv°7 | I | VI7 | II7 | V7",
        Comping::Stride,
        Bass::RootFifth,
        176.0,
        "Stride",
        "Harlem stride, the piano of James P. Johnson and Fats Waller: fast and swung, the left \
         hand leaping from a bass note on 1 and 3 to a chord on 2 and 4. I7 leads to IV, the \
         diminished seventh on the raised fourth (F sharp diminished in C) climbs back to I, \
         and the dominants circle home.",
    ),
    style(
        "Big band",
        Jazz,
        Major,
        "I7 | IV7 | I7 | I7 | IV7 | #iv°7 | I7 | VI7 | ii7 | V7 | I7 VI7 | ii7 V7",
        Comping::Charleston,
        Bass::Walking,
        160.0,
        "Big band swing",
        "A swing orchestra of the 1930s and 1940s, such as Count Basie's: a walking bass, the \
         guitar strumming four to the bar, a sparse piano, and the horns (trumpets, saxophones \
         and trombones) riffing together over a jazz blues. The diminished seventh on the \
         raised fourth (F sharp diminished in C) leads from IV back to I, and VI7, ii7 and V7 \
         turn the form around.",
    ),
    style(
        "Big band ballad",
        Jazz,
        Major,
        "Imaj7 | #i°7 | ii7 | V7 | iii7 | biii°7 | ii7 | V7",
        Comping::Ballad,
        Bass::RootFifth,
        66.0,
        "Big band ballad",
        "A slow number for a big band: the saxophones hold the chords under the melody, and \
         two diminished sevenths fill in the bass's steps by semitones, C sharp between C and \
         D on the way up, E flat between E and D on the way down.",
    ),
    // ----- Soul, funk and R&B -----
    style(
        "Neo-soul",
        Soul,
        Major,
        "IVmaj7 | III7 | vi7 | I7",
        Comping::Lazy,
        Bass::Lazy,
        84.0,
        "Neo soul",
        "Lush seventh chords with two secondary dominants (III7 and I7) that keep the music moving.",
    ),
    style(
        "Soul",
        Soul,
        Major,
        "I | I7 | IV | iv",
        Comping::Gospel,
        Bass::Quarters,
        76.0,
        "R&B",
        "From the major IV to the minor iv: the bittersweet move of soul ballads.",
    ),
    style(
        "Gospel",
        Soul,
        Major,
        "I | I7 | IV | #iv°7 | I | VI7 | ii7 | V7",
        Comping::Gospel,
        Bass::Walking,
        96.0,
        "Shuffle",
        "The passing diminished chord (#iv°7) climbs from IV back to I, as church pianists do.",
    ),
    style(
        "Motown",
        Soul,
        Major,
        "I | iii | IV | V",
        Comping::Quarters,
        Bass::Walking,
        112.0,
        "Motown",
        "Piano chords on every beat, a busy bass and a snare on all four beats: 1960s Detroit.",
    ),
    style(
        "Funk vamp",
        Soul,
        Dorian,
        "i7 | i7 | IV7 | IV7",
        Comping::Funk,
        Bass::Funk,
        102.0,
        "Funk",
        "Two chords and a lot of rhythm: in funk, the groove is the song.",
    ),
    style(
        "Funk dominant",
        Soul,
        Mixolydian,
        "I9 | I9 | IV9 | I9",
        Comping::Funk,
        Bass::Funk,
        106.0,
        "Funk",
        "A dominant ninth chord as home: no tension to resolve, only groove.",
    ),
    style(
        "R&B minor",
        Soul,
        NaturalMinor,
        "i9 | bVImaj7 | iv9 | V7b9",
        Comping::Held,
        Bass::Sub,
        88.0,
        "R&B",
        "Ninth chords and a dominant with a flat nine: smooth, late-night R&B.",
    ),
    style(
        "New jack swing",
        Soul,
        Dorian,
        "i9 | i9 | iv9 | V7#9",
        Comping::Funk,
        Bass::Funk,
        104.0,
        "New jack swing",
        "Late-1980s R&B: jazzy ninth chords over hip hop drums and a skipping hi-hat.",
    ),
    style(
        "Philly soul",
        Soul,
        Major,
        "ii7 | V7 | iii7 | vi7",
        Comping::Offbeat,
        Bass::Octaves,
        112.0,
        "Disco",
        "1970s Philadelphia soul, the road to disco: ii7 V7 that lands on iii7 instead of home, and keeps going.",
    ),
    style(
        "Quiet storm",
        Soul,
        Major,
        "Imaj9 | IVmaj9 | iii7 | vi9",
        Comping::Held,
        Bass::Lazy,
        70.0,
        "R&B",
        "Late-night soul: soft ninth chords held long, and a bass that leans into each change.",
    ),
    style(
        "Southern soul",
        Soul,
        Major,
        "I | IV | I | V7",
        Comping::Gospel,
        Bass::Quarters,
        92.0,
        "R&B",
        "Church piano, three chords and a lot of feeling: Memphis in the 1960s.",
    ),
    style(
        "Afrobeat",
        Soul,
        Dorian,
        "i7 | i7 | i7 | IV7",
        Comping::Funk,
        Bass::Funk,
        108.0,
        "Afrobeat",
        "Fela Kuti's Afrobeat, from 1970s Lagos: a long Dorian vamp on i7 and IV7, guitars \
         locked in short repeating riffs, a funky bass, horns, and Tony Allen's drums.",
    ),
    // ----- Hip hop and lo-fi -----
    style(
        "Lo-fi",
        HipHop,
        Major,
        "ii9 | V13 | Imaj9 | vi9",
        Comping::Lazy,
        Bass::Lazy,
        75.0,
        "Lo-fi",
        "The ii V I of jazz with ninths and thirteenths, played lazily behind the beat.",
    ),
    style(
        "Lo-fi descending",
        HipHop,
        Major,
        "IVmaj7 | iii7 | ii7 | Imaj7",
        Comping::Lazy,
        Bass::Lazy,
        72.0,
        "Lo-fi",
        "Seventh chords stepping down the scale to home, one bar each.",
    ),
    style(
        "Boom bap minor",
        HipHop,
        NaturalMinor,
        "i7 | i7 | bVImaj7 | v7",
        Comping::Lazy,
        Bass::Sub,
        90.0,
        "Boom bap",
        "A dark loop in the style of 1990s hip hop, where the beat is the star.",
    ),
    style(
        "Trap minor",
        HipHop,
        HarmonicMinor,
        "i | bVI | iv | V",
        Comping::ArpUp,
        Bass::Sub,
        140.0,
        "Trap",
        "Harmonic minor's major V gives trap its dark pull. The beat is half-time, so 140 BPM \
         feels like 70.",
    ),
    style(
        "Drill",
        HipHop,
        HarmonicMinor,
        "i | bVI | i | V",
        Comping::Held,
        Bass::Sub,
        142.0,
        "Drill",
        "Dark piano chords from the harmonic minor, over half-time drums with a hi-hat in 3 + 3 + 2.",
    ),
    style(
        "G-funk",
        HipHop,
        Dorian,
        "i7 | IV9 | i7 | IV9",
        Comping::Lazy,
        Bass::Funk,
        92.0,
        "G-funk",
        "1990s West Coast hip hop: a two-chord Dorian vamp from funk records, with a funky bass and a lazy beat.",
    ),
    style(
        "Jazz rap",
        HipHop,
        Major,
        "iii7 | VI7 | ii7 | V7",
        Comping::Lazy,
        Bass::Walking,
        92.0,
        "Boom bap",
        "Early-1990s hip hop sampled jazz records: a turnaround of seventh chords and a walking bass over boom bap.",
    ),
    style(
        "Cloud rap",
        HipHop,
        Major,
        "IVmaj7 | V | vi7 | vi7",
        Comping::Held,
        Bass::Sub,
        140.0,
        "Trap",
        "Dreamy held chords that never quite come home, over a half-time trap beat.",
    ),
    // ----- Electronic -----
    style(
        "House piano",
        Electronic,
        Major,
        "vi | IV | I | V",
        Comping::Stabs,
        Bass::Offbeat,
        124.0,
        "Four on the floor",
        "Syncopated piano stabs over a kick on every beat: the sound of house.",
    ),
    style(
        "Deep house",
        Electronic,
        NaturalMinor,
        "i9 | bVImaj7 | bIIImaj7 | bVII",
        Comping::Stabs,
        Bass::Offbeat,
        122.0,
        "Four on the floor",
        "Minor ninth and major seventh chords: deep house is soft, warm and minor.",
    ),
    style(
        "Trance anthem",
        Electronic,
        NaturalMinor,
        "i | bVI | bIII | bVII",
        Comping::Gate,
        Bass::Offbeat,
        138.0,
        "Trance",
        "The minor four-chord loop, chopped into sixteenths by a gate.",
    ),
    style(
        "Synthwave",
        Electronic,
        NaturalMinor,
        "i | bVI | bVII | v",
        Comping::Arp16,
        Bass::Octaves16,
        100.0,
        "Synthwave",
        "Arpeggios and an octave bass in sixteenths, straight out of the 1980s.",
    ),
    style(
        "EDM drop",
        Electronic,
        Major,
        "IV | V | vi | vi",
        Comping::Stabs,
        Bass::Octaves,
        128.0,
        "Four on the floor",
        "Rising from IV to the minor vi, the chords build tension for the drop.",
    ),
    style(
        "Future bass",
        Electronic,
        Major,
        "IVmaj9 | V | vi9 | iii7",
        Comping::Gate,
        Bass::Sub,
        150.0,
        "Dubstep",
        "Big ninth chords chopped in rhythm over a half-time beat.",
    ),
    style(
        "Disco",
        Electronic,
        Major,
        "vi7 | ii7 | V7 | Imaj7",
        Comping::Offbeat,
        Bass::Octaves,
        118.0,
        "Disco",
        "Seventh chords around the circle of fifths, with the octave bass and open hats of the 1970s.",
    ),
    style(
        "Drum and bass",
        Electronic,
        NaturalMinor,
        "i | i | bVI | bVII",
        Comping::Held,
        Bass::Sub,
        174.0,
        "Drum and bass",
        "Long pad chords over a fast broken beat and a deep bass.",
    ),
    style(
        "Techno minor",
        Electronic,
        Phrygian,
        "i | i | bII | i",
        Comping::Stabs,
        Bass::Offbeat,
        130.0,
        "Techno",
        "The Phrygian flat II, a semitone above the tonic, gives techno its dark edge.",
    ),
    style(
        "UK garage",
        Electronic,
        NaturalMinor,
        "i9 | iv9 | i9 | bVII",
        Comping::Stabs,
        Bass::Sub,
        132.0,
        "UK garage",
        "Skippy two-step drums under smooth minor ninth chords.",
    ),
    style(
        "Moombahton",
        Electronic,
        NaturalMinor,
        "i | bVI | bIII | bVII",
        Comping::Dembow,
        Bass::Dembow,
        108.0,
        "Moombahton",
        "House music slowed down to reggaeton's tempo: four on the floor under a dembow snare.",
    ),
    style(
        "Electro",
        Electronic,
        Phrygian,
        "i | i | bII | bVII",
        Comping::Stabs,
        Bass::Octaves16,
        128.0,
        "Electro",
        "Early-1980s electro: drum machines, a sequenced bass in sixteenths and the dark Phrygian flat II.",
    ),
    style(
        "Ambient",
        Electronic,
        Major,
        "Imaj9 | IVmaj9 | Imaj9 | IVmaj9",
        Comping::Held,
        Bass::Held,
        70.0,
        "",
        "No drums at all: two major ninth chords held for as long as they last, music to float in.",
    ),
    style(
        "Lo-fi house",
        Electronic,
        Major,
        "ii9 | iii7 | IVmaj7 | V",
        Comping::Stabs,
        Bass::Offbeat,
        120.0,
        "Four on the floor",
        "Warm jazz chords climbing the scale over a house beat.",
    ),
    style(
        "Eurodance",
        Electronic,
        NaturalMinor,
        "i | bVI | bVII | i",
        Comping::Stabs,
        Bass::Octaves,
        136.0,
        "Dance pop",
        "1990s dance music: a minor loop, an octave bass and stabs on top.",
    ),
    style(
        "Dubstep",
        Electronic,
        NaturalMinor,
        "i | i | bVI | bVII",
        Comping::Gate,
        Bass::Sub,
        140.0,
        "Dubstep",
        "Half-time drums with a big snare on beat 3, and chords chopped up over a deep sub bass.",
    ),
    // ----- Latin -----
    style(
        "Bossa nova",
        Latin,
        Major,
        "Imaj7 | Imaj7 | II7 | II7 | ii7 | bII7 | Imaj7 | bII7",
        Comping::Bossa,
        Bass::Bossa,
        130.0,
        "Bossa nova",
        "Brazil's quiet revolution: jazz chords, a guitar rhythm across two bars, and a flat II7 \
         standing in for V7.",
    ),
    style(
        "Bossa minor",
        Latin,
        NaturalMinor,
        "i7 | iv7 | bVII7 | bIIImaj7 | bVImaj7 | iiø7 | V7b9 | i7",
        Comping::Bossa,
        Bass::Bossa,
        124.0,
        "Bossa nova",
        "A minor key going round the circle of fifths, as many bossa novas and standards do.",
    ),
    style(
        "Samba",
        Latin,
        Major,
        "Imaj7 | VI7 | ii7 | V7",
        Comping::Bossa,
        Bass::Bossa,
        100.0,
        "Samba",
        "Bossa nova's louder parent: the same kind of chords, with a whole percussion section.",
    ),
    style(
        "Salsa montuno",
        Latin,
        Major,
        "I | IV | V | IV",
        Comping::Montuno,
        Bass::Tumbao,
        95.0,
        "Salsa",
        "A piano montuno and a tumbao bass that plays each chord before it arrives, over the son \
         clave. Each bar here lasts two bars of salsa.",
    ),
    style(
        "Cumbia",
        Latin,
        Major,
        "I | IV | V | I",
        Comping::Offbeat,
        Bass::RootFifth,
        95.0,
        "Cumbia",
        "Colombian cumbia: chords on the offbeats, bass on the beats, and a scraper in eighths.",
    ),
    style(
        "Reggaeton",
        Latin,
        NaturalMinor,
        "i | iv | bVII | bIII",
        Comping::Dembow,
        Bass::Dembow,
        95.0,
        "Reggaeton",
        "The dembow rhythm, 3 + 3 + 2 in every half bar, under a minor loop.",
    ),
    style(
        "Bolero",
        Latin,
        HarmonicMinor,
        "i | iv | V7 | i",
        Comping::ArpUpDown,
        Bass::RootFifth,
        80.0,
        "Bolero",
        "The slow Latin love song: arpeggios on the guitar, and the harmonic minor's V7 for the drama.",
    ),
    style(
        "Bachata",
        Latin,
        HarmonicMinor,
        "i | iv | V7 | V7",
        Comping::ArpUp,
        Bass::Tresillo,
        128.0,
        "Bachata",
        "From the Dominican Republic: a picked guitar, bongos, and the güira scraping every sixteenth.",
    ),
    style(
        "Merengue",
        Latin,
        Major,
        "I | V7 | V7 | I",
        Comping::Pulse,
        Bass::Tresillo,
        132.0,
        "Merengue",
        "The fast Dominican dance: only two chords, and all the energy in the drums.",
    ),
    style(
        "Cha-cha-chá",
        Latin,
        Major,
        "Imaj7 | vi7 | ii7 | V7",
        Comping::Stabs,
        Bass::Tumbao,
        116.0,
        "Cha-cha-chá",
        "1950s Cuba: a piano montuno, a tumbao bass and the \"cha-cha-chá\" at the end of every bar.",
    ),
    style(
        "Son montuno minor",
        Latin,
        HarmonicMinor,
        "i | iv | V7 | i",
        Comping::Montuno,
        Bass::Tresillo,
        98.0,
        "Salsa",
        "The salsa montuno in a minor key, over the son clave.",
    ),
    style(
        "Son",
        Latin,
        Major,
        "I | IV | V7 | IV",
        Comping::Montuno,
        Bass::Tumbao,
        192.0,
        "Son",
        "Cuba in the 1920s: a tres guitar plays a repeating figure over the clave, and the bass \
         plays no note on beat 1, only on the \"and\" of 2 and on 4, where it already plays the \
         next chord's root.",
    ),
    style(
        "Choro",
        Latin,
        HarmonicMinor,
        "i | V7 | V7 | i | iv | i | II7 V7 | i",
        Comping::Habanera,
        Bass::Habanera,
        100.0,
        "Choro",
        "Rio de Janeiro around 1900: flute, guitars and pandeiro, a tune running in sixteenths, and \
         the bass in the habanera's long, short, long, long. Each bar here lasts two bars of 2/4.",
    ),
    style(
        "Maxixe",
        Latin,
        HarmonicMinor,
        "i | i | V7 | V7 | iv | i | V7 | i",
        Comping::Habanera,
        Bass::Habanera,
        90.0,
        "Maxixe",
        "Rio de Janeiro's dance of the 1900s, which piano composers called \"Brazilian tango\": \
         the habanera's long, short, long, long in the left hand under a syncopated tune. Each \
         bar here lasts two bars of 2/4.",
    ),
    style(
        "Marchinha",
        Latin,
        Major,
        "I | I | V7 | V7 | V7 | V7 | I | I",
        Comping::Offbeat,
        Bass::RootFifth,
        116.0,
        "Marchinha",
        "Rio's carnival march, sung by the crowds in the street since 1899: a brass band, the \
         tonic and the dominant, the bass on the beats and the chords between them. Each bar \
         here lasts two bars of 2/4.",
    ),
    style(
        "Ranchera",
        Latin,
        Major,
        "I | I | V7 | V7 | V7 | V7 | I | I",
        Comping::Waltz,
        Bass::Waltz,
        150.0,
        "Ranchera",
        "The Mexican mariachi waltz: the bass on beat 1, the guitars on 2 and 3, and the trumpets \
         and violins singing over them.",
    )
    .in_time(THREE_FOUR),
    style(
        "Mambo",
        Latin,
        Major,
        "I | IV | V7 | IV",
        Comping::Montuno,
        Bass::Tumbao,
        104.0,
        "Mambo",
        "The big-band mambo of Pérez Prado and Tito Puente, grown from the son's last and \
         wildest section: the piano repeats its montuno and the bass its tumbao over the \
         son's vamp, I, IV, V7, IV, while the horns riff.",
    ),
    style(
        "Guaguancó",
        Latin,
        Major,
        "I | V7 | V7 | I",
        Comping::Held,
        Bass::Held,
        100.0,
        "Guaguancó",
        "The Cuban street rumba, for voices and drums: the rumba clave and three conga drums \
         talking to each other, the lowest keeping time and the highest, the quinto, \
         improvising. Its chorus, the coro, sings over I and V7, held here.",
    ),
    style(
        "Afro-Cuban 6/8",
        Latin,
        Dorian,
        "i7 ii7 | i7 ii7 | bVIImaj7 | i7",
        Comping::Arp3,
        Bass::LongShort,
        150.0,
        "Afro-Cuban 6/8",
        "The 6/8 of Cuba's drums of African origin, which Afro-Cuban jazz took up: the bell's \
         seven strokes over twelve eighths and four slow beats, under a Dorian minor rocking \
         between i7 and ii7 before the major chord a tone below the tonic.",
    )
    .in_time(TWELVE_EIGHT),
    style(
        "Baião",
        Latin,
        Major,
        "I | IV | I | V7 | I | IV | V7 | I",
        Comping::Dembow,
        Bass::Baiao,
        104.0,
        "Baião",
        "The baião of Brazil's northeast, Luiz Gonzaga's dance: the accordion and the bass in \
         3 + 3 + 2 sixteenths, the zabumba drum's long-short on the beat, and a triangle. The \
         baião is in 2/4; each bar here lasts two.",
    ),
    style(
        "Vallenato",
        Latin,
        Major,
        "I | V7 | V7 | I | I | IV | V7 | I",
        Comping::Afterbeat,
        Bass::Alternating,
        100.0,
        "Vallenato",
        "Colombia's vallenato, from the Caribbean coast, in its paseo rhythm: the accordion \
         over the caja and the guacharaca, three chords, the bass on the beats. The paseo is \
         in 2/4; each bar here lasts two.",
    ),
    // ----- Tango -----
    style(
        "Tango",
        Tango,
        HarmonicMinor,
        "i | V7 | V7 | i | iv | i | V7 | i",
        Comping::Quarters,
        Bass::Quarters,
        118.0,
        "Tango marcato",
        "The marcato: every beat played short and hard, piano and bass together, with the \
         harmonic minor's major V7.",
    ),
    style(
        "Tango síncopa",
        Tango,
        HarmonicMinor,
        "i | iv | V7 | i",
        Comping::Tresillo,
        Bass::Tresillo,
        118.0,
        "Tango síncopa",
        "Accents on 1, the \"and\" of 2, and 4: the 3 + 3 + 2 that tango inherited from the habanera.",
    ),
    style(
        "Milonga",
        Tango,
        Major,
        "I | I | V7 | V7 | V7 | V7 | I | I",
        Comping::Tresillo,
        Bass::Tresillo,
        110.0,
        "Milonga",
        "Tango's quicker, happier cousin from the River Plate, with 3 + 3 + 2 in every bar.",
    ),
    style(
        "Habanera",
        Tango,
        HarmonicMinor,
        "i | V7 | V7 | i",
        Comping::Habanera,
        Bass::Habanera,
        76.0,
        "Habanera",
        "The Cuban dance that travelled to Spain, Argentina and the opera stage: long, short, \
         long, long.",
    ),
    style(
        "Tango vals",
        Tango,
        Major,
        "I | I | V7 | V7 | V7 | V7 | I | I",
        Comping::Waltz,
        Bass::Waltz,
        180.0,
        "Tango vals",
        "The tango orchestras' fast waltz: one strong beat to the bar, felt in one.",
    )
    .in_time(THREE_FOUR),
    style(
        "Tango de salón",
        Tango,
        HarmonicMinor,
        "i | iv | bVII7 | bIIImaj7 | bVImaj7 | iiø7 | V7 | i",
        Comping::Tresillo,
        Bass::Tresillo,
        112.0,
        "Tango síncopa",
        "The ballroom tango of the 1940s, walking the whole circle of fifths in a minor key before V7 brings it home.",
    ),
    style(
        "Tango nuevo",
        Tango,
        HarmonicMinor,
        "i | imaj7 | i7 | i6",
        Comping::Tresillo,
        Bass::Tresillo,
        132.0,
        "Tango nuevo",
        "The new tango of the 1950s: one voice slides down A, G sharp, G, F sharp under a held A minor chord, a line cliché.",
    ),
    style(
        "Vals in minor",
        Tango,
        HarmonicMinor,
        "i | i | V7 | V7 | V7 | V7 | i | i",
        Comping::Waltz,
        Bass::Waltz,
        176.0,
        "Tango vals",
        "The tango waltz in a minor key: fast, felt in one, and wistful.",
    )
    .in_time(THREE_FOUR),
    // ----- Latin American folklore -----
    style(
        "Zamba",
        Folklore,
        HarmonicMinor,
        "i | iv | V7 | i | bIII | bVII | V7 | i",
        Comping::Arp3,
        Bass::Waltz,
        84.0,
        "Zamba",
        "The slow Argentine dance of the handkerchiefs, in 6/8: a minor key that leans on \
         its relative major for two bars before V7 brings it home.",
    )
    .in_time(SIX_EIGHT),
    style(
        "Chacarera",
        Folklore,
        HarmonicMinor,
        "i | V7 | i | V7 | iv | i | V7 | i",
        Comping::Sway,
        Bass::RootFifth,
        132.0,
        "Chacarera",
        "Santiago del Estero's dance: eight bars of i and V7 with one iv, strummed in 6/8 \
         while the bombo plays 3/4 against it.",
    )
    .in_time(SIX_EIGHT),
    style(
        "Gato",
        Folklore,
        Major,
        "I | V7 | V7 | I | I | V7 | V7 | I",
        Comping::Waltz,
        Bass::RootFifth,
        126.0,
        "Gato",
        "The quickest of the old dances of the Pampas: two chords, bright and major, in a \
         6/8 that keeps tipping into 3/4.",
    )
    .in_time(SIX_EIGHT),
    style(
        "Malambo",
        Folklore,
        Major,
        "I | V7 | I | V7",
        Comping::Waltz,
        Bass::Quarters,
        144.0,
        "Malambo",
        "The gaucho's dance of stamping boots: one chord and its dominant, the bass in \
         3/4, and all the interest in the feet.",
    )
    .in_time(SIX_EIGHT),
    style(
        "Chamamé",
        Folklore,
        Major,
        "I | V7 | V7 | I | IV | I | V7 | I",
        Comping::Sway,
        Bass::Waltz,
        120.0,
        "Chamamé",
        "From Corrientes: the guitar's bass and chords move in 3/4 while the accordion and \
         the drums play 6/8, so every bar is three against two.",
    )
    .in_time(SIX_EIGHT),
    style(
        "Cueca",
        Folklore,
        Major,
        "I | V7 | V7 | I | I | IV | V7 | I",
        Comping::Waltz,
        Bass::Waltz,
        126.0,
        "Cueca",
        "Chile's national dance, a cousin of the zamba: faster, in a major key, with hand \
         claps between the beats.",
    )
    .in_time(SIX_EIGHT),
    style(
        "Marinera",
        Folklore,
        HarmonicMinor,
        "i | iv | V7 | i | bIII | bVII | V7 | i",
        Comping::Arp3,
        Bass::Held,
        120.0,
        "Marinera",
        "Peru's courtship dance for guitar and cajón: the zamba's minor-key chords at a \
         brisker pace, with the cajón slapping across the beat.",
    )
    .in_time(SIX_EIGHT),
    style(
        "Bambuco",
        Folklore,
        HarmonicMinor,
        "i | V7 | i | bVI | bIII | V7 | i | i",
        Comping::Arp3,
        Bass::Waltz,
        108.0,
        "Bambuco",
        "Colombia's Andean song: a minor key that visits bVI and bIII, over the tiple's 6/8 \
         and a bass that avoids the downbeat.",
    )
    .in_time(SIX_EIGHT),
    style(
        "Joropo",
        Folklore,
        Major,
        "I | IV | V7 | I",
        Comping::Arp3,
        Bass::Waltz,
        176.0,
        "Joropo",
        "The music of the Venezuelan and Colombian plains: harp arpeggios in a fast 3/4, \
         three chords, and maracas that never stop.",
    )
    .in_time(THREE_FOUR),
    style(
        "Vidala",
        Folklore,
        NaturalMinor,
        "i | bVII | bIII | i",
        Comping::Sway,
        Bass::Held,
        72.0,
        "Vidala",
        "A slow song of Argentina's Andean north-west, sung to a hand drum: natural minor, \
         with bVII and bIII instead of a dominant.",
    )
    .in_time(THREE_FOUR),
    style(
        "Carnavalito",
        Folklore,
        NaturalMinor,
        "i | bVII | bIII | V7",
        Comping::Strum,
        Bass::RootFifth,
        116.0,
        "Carnavalito",
        "The carnival dance of the Quebrada de Humahuaca: a pentatonic tune over a minor \
         loop, with quena, charango and bombo.",
    ),
    style(
        "Carnavalito (2/4)",
        Folklore,
        HarmonicMinor,
        "i V | i V | i V | i V | bVI bVII | bIII bVI | bVI bVII | bIII",
        Comping::Strum,
        Bass::RootFifth,
        88.0,
        "Carnavalito (2/4)",
        "The carnavalito as the Gloria of the Misa Criolla plays it, a bar of 2/4 for each \
         two beats: the minor chord and its dominant, a beat each, then the relative major's \
         IV, V and I (bVI, bVII and bIII of the minor key).",
    )
    .in_time(TWO_FOUR),
    style(
        "Yaraví",
        Folklore,
        NaturalMinor,
        "i | i | bIII | bIII | iv | iv | i | i",
        Comping::Arp3,
        Bass::Held,
        44.0,
        "Yaraví",
        "The yaraví, the slow, sad song of the Andes, in 3/4: a minor pentatonic tune over \
         long chords, the bass held, the guitar picking the chord's notes.",
    )
    .in_time(THREE_FOUR),
    style(
        "Chacarera trunca",
        Folklore,
        HarmonicMinor,
        "i | V7 | i | V7 | iv | i | V7 | i",
        Comping::Sway,
        Bass::RootFifth,
        152.0,
        "Chacarera trunca",
        "The chacarera trunca, its phrases cut short before the end of their second bar, in \
         a quick 3/4: the minor tonic and its dominant seventh, with one iv.",
    )
    .in_time(THREE_FOUR),
    style(
        "Carnaval cochabambino",
        Folklore,
        Major,
        "I | I | V7 | V7 | V7 | V7 | I | I",
        Comping::Waltz,
        Bass::RootFifth,
        152.0,
        "Carnaval cochabambino",
        "The carnival of Cochabamba, in a quick 3/4, in a major key: the tonic and its \
         dominant seventh, two bars at a time, under drums, a wood block and a snare.",
    )
    .in_time(THREE_FOUR),
    style(
        "Estilo",
        Folklore,
        HarmonicMinor,
        "i | iv | i | V7 | i | iv | V7 | i",
        Comping::Sway,
        Bass::Held,
        44.0,
        "",
        "The estilo of the Pampas, the gaucho's slow song to the guitar, in 3/4, with no \
         drums: the guitar's chords, the minor tonic, its iv and its V7, under a voice that \
         takes its time.",
    )
    .in_time(THREE_FOUR),
    style(
        "Huayno",
        Folklore,
        NaturalMinor,
        "i | bIII | i | V7",
        Comping::Strum,
        Bass::RootFifth,
        100.0,
        "Huayno",
        "The Andean huayno of Peru and Bolivia: minor and its relative major, in a 2/4 with \
         a long-short-short lilt.",
    ),
    style(
        "Candombe",
        Folklore,
        Major,
        "I | IV | V7 | I",
        Comping::Bloques,
        Bass::Tresillo,
        110.0,
        "Candombe",
        "From Montevideo, across the river from Buenos Aires: the drums that the tango and \
         the milonga grew up with, under three plain chords.",
    ),
    style(
        "Candombe Ansina",
        Folklore,
        Major,
        "I | IV | V7 | I",
        Comping::Bloques,
        Bass::Tresillo,
        110.0,
        "Candombe Ansina",
        "The candombe as the drums of Montevideo's Palermo neighbourhood play it, the Ansina \
         toque, under the same three plain chords.",
    ),
    style(
        "Polca",
        Folklore,
        Major,
        "I | V7 | V7 | I",
        Comping::Backbeat,
        Bass::RootFifth,
        120.0,
        "Polca",
        "The polka of Mexican corridos and of Rio's choro players: two chords, the bass on \
         beat 1 and the guitars on beat 2. Each bar here lasts two bars of 2/4.",
    ),
    style(
        "Pasillo",
        Folklore,
        HarmonicMinor,
        "i | i | iv | iv | V7 | V7 | i | i",
        Comping::Waltz,
        Bass::Waltz,
        190.0,
        "Pasillo",
        "The Andean waltz of Colombia and Ecuador, quick and often in minor: the bass on 1, \
         the strings on the beats after it.",
    )
    .in_time(THREE_FOUR),
    style(
        "Son jalisciense",
        Folklore,
        Major,
        "I | I | V7 | V7 | V7 | V7 | I | I",
        Comping::Arp3,
        Bass::Waltz,
        180.0,
        "Son jalisciense",
        "The mariachi son: two chords, I and V7, with violins and trumpets over guitars that \
         swing between 6/8 and 3/4.",
    )
    .in_time(SIX_EIGHT),
    style(
        "Peruvian waltz",
        Folklore,
        HarmonicMinor,
        "i | V7 | V7 | i | I7 | iv | V7 | i",
        Comping::Waltz,
        Bass::Waltz,
        156.0,
        "Peruvian waltz",
        "The vals criollo of Lima, quick and syncopated, for two guitars and a cajón. In a \
         minor key the tonic turns into a dominant seventh (A7 in A minor) to lead to iv, as \
         Latin American waltzes love to do.",
    )
    .in_time(THREE_FOUR),
    style(
        "Festejo",
        Folklore,
        Major,
        "I | V7 | I | V7 | IV | I | V7 | I",
        Comping::Waltz,
        Bass::Waltz,
        165.0,
        "Festejo",
        "The Afro-Peruvian festejo, a fast dance of the coast: the cajón plays 6/8 while the \
         guitar's chords fall in 3/4, three against two, over I, IV and V7.",
    )
    .in_time(SIX_EIGHT),
    style(
        "Rasguido doble",
        Folklore,
        Major,
        "I | V7 | V7 | I | IV | I | V7 | I",
        Comping::Habanera,
        Bass::Habanera,
        100.0,
        "Rasguido doble",
        "The rasguido doble of Corrientes and Chaco, made popular by \"El rancho 'e la \
         Cambicha\": accordion and guitar in a lively two, the guitar strumming the \
         habanera's long, short, long, long that it shares with the milonga. Each bar here \
         lasts two bars of 2/4.",
    ),
    style(
        "Murga",
        Folklore,
        Major,
        "I | V7 | V7 | I | IV | I | V7 | I",
        Comping::Held,
        Bass::Held,
        116.0,
        "Murga",
        "The murga of Montevideo's carnival: a choir singing in parts, often in satire, over \
         the marcha camión of bombo, platillo and redoblante.",
    ),
    // ----- Flamenco -----
    style(
        "Andalusian cadence",
        Flamenco,
        NaturalMinor,
        "i | bVII | bVI | V",
        Comping::Rumba,
        Bass::RootFifth,
        105.0,
        "Rumba flamenca",
        "Four chords stepping down to the dominant, the heart of flamenco: Am G F E.",
    ),
    style(
        "Phrygian flamenco",
        Flamenco,
        Phrygian,
        "I | bII | bIII | bII",
        Comping::Rumba,
        Bass::RootFifth,
        110.0,
        "Rumba flamenca",
        "The Phrygian mode with a major chord on its tonic, E F G F: the Spanish guitar.",
    ),
    style(
        "Sevillanas",
        Flamenco,
        Major,
        "I | V7 | V7 | I | IV | I | V7 | I",
        Comping::Waltz,
        Bass::Waltz,
        144.0,
        "Waltz",
        "The dance of the Seville fair, in three: a bright major key, danced in pairs.",
    )
    .in_time(THREE_FOUR),
    style(
        "Tangos flamencos",
        Flamenco,
        Phrygian,
        "I | bII | I | bII",
        Comping::Rumba,
        Bass::RootFifth,
        118.0,
        "Rumba flamenca",
        "Flamenco's tangos, no relation to the Argentine one: E and F, back and forth, in 4/4.",
    ),
    style(
        "Flamenco minor",
        Flamenco,
        HarmonicMinor,
        "i | iv | V7 | i",
        Comping::Rumba,
        Bass::RootFifth,
        110.0,
        "Rumba flamenca",
        "The harmonic minor's E7 makes A minor sound Spanish: its G sharp and F sit a step and a half apart.",
    ),
    style(
        "Pasodoble",
        Flamenco,
        HarmonicMinor,
        "i | bVII | bVI | V7 | i | iv | V7 | i",
        Comping::Afterbeat,
        Bass::Alternating,
        120.0,
        "Pasodoble",
        "The pasodoble of bullfights and Spanish village bands: a march in 2/4, the tuba on \
         the beats and the band's chords between them, in a minor key that walks down the \
         Andalusian cadence, Am G F E7.",
    )
    .in_time(TWO_FOUR),
    style(
        "Bulerías",
        Flamenco,
        Phrygian,
        "I | bII | bIII bII | I",
        Comping::Accents,
        Bass::Accents,
        116.0,
        "Twelve-beat compás",
        "Bulerías, the fastest and most playful of the twelve-beat forms: the guitar strikes \
         the compás's accents, 12, 3, 6, 8 and 10, and its chords step down to the Phrygian \
         tonic, G, F, E.",
    )
    .in_time(COMPAS),
    style(
        "Soleá",
        Flamenco,
        Phrygian,
        "iv | bIII | bII | I",
        Comping::Accents,
        Bass::Accents,
        66.0,
        "Twelve-beat compás",
        "Soleá, the mother of flamenco's songs: slow and solemn, in the twelve-beat compás, \
         a compás for each chord of the Andalusian cadence down to the Phrygian tonic, A minor, \
         G, F, E.",
    )
    .in_time(COMPAS),
    style(
        "Alegrías",
        Flamenco,
        Major,
        "I | V7 | V7 | I",
        Comping::Accents,
        Bass::Accents,
        90.0,
        "Twelve-beat compás",
        "The alegrías of Cádiz, the bright one among the twelve-beat forms: a major key, I and \
         V7 a compás each, at a lively pace.",
    )
    .in_time(COMPAS),
    // ----- Balkan and Turkish -----
    // One chord to the bar: the bass on each beat, short or long, and
    // the chord on the eighths between (see `Comping::Aksak`).
    style(
        "Paidushko",
        Balkan,
        HarmonicMinor,
        "i | i | V7 | i | iv | i | V7 | i",
        Comping::Aksak,
        Bass::Aksak,
        200.0,
        "Paidushko",
        "A limping 5/8, 2 + 3: the bass on the short beat and on the long one, \
         the accordion's chord after each, in a minor key with its V7.",
    )
    .in_time(TimeSignature::of_groups(&[2, 3], 8)),
    style(
        "Kalamatianos",
        Balkan,
        HarmonicMinor,
        "i | i | iv | iv | V7 | V7 | i | i",
        Comping::Aksak,
        Bass::Aksak,
        180.0,
        "Kalamatianos",
        "The Greek 7/8, 3 + 2 + 2, slow, quick, quick: i, iv and V7 of a minor \
         key, two bars each.",
    )
    .in_time(TimeSignature::of_groups(&[3, 2, 2], 8)),
    style(
        "Lesnoto",
        Balkan,
        NaturalMinor,
        "i | bVII | i | bVII | iv | i | V | i",
        Comping::Aksak,
        Bass::Aksak,
        140.0,
        "Lesnoto",
        "Macedonia's 3 + 2 + 2, unhurried: a minor key rocking to the major chord a step \
         below, bVII, before iv and a major V bring it home.",
    )
    .in_time(TimeSignature::of_groups(&[3, 2, 2], 8)),
    style(
        "Rachenitsa",
        Balkan,
        Major,
        "I | I | V7 | V7 | V7 | V7 | I | I",
        Comping::Aksak,
        Bass::Aksak,
        220.0,
        "Rachenitsa",
        "The Bulgarian 7/8, 2 + 2 + 3, quick, quick, slow: two chords, I and V7, at a \
         dancing pace.",
    )
    .in_time(TimeSignature::of_groups(&[2, 2, 3], 8)),
    style(
        "Daichovo",
        Balkan,
        Major,
        "I | IV | V7 | I | I | IV | V7 | I",
        Comping::Aksak,
        Bass::Aksak,
        220.0,
        "Daichovo",
        "A Bulgarian 9/8 counted 2 + 2 + 2 + 3: three short beats and a long one, \
         over I, IV and V7.",
    )
    .in_time(TimeSignature::of_groups(&[2, 2, 2, 3], 8)),
    style(
        "Karşılama",
        Balkan,
        Phrygian,
        "I | bII | I | bII | bvii | bII | I | I",
        Comping::Aksak,
        Bass::Aksak,
        150.0,
        "Karşılama",
        "The Turkish 9/8, 2 + 2 + 2 + 3, with the sound of makam hicaz: a major chord \
         on the tonic, the major chord a semitone above it, and the minor one a \
         tone below.",
    )
    .in_time(TimeSignature::of_groups(&[2, 2, 2, 3], 8)),
    style(
        "Raks aksağı",
        Balkan,
        Phrygian,
        "iv | I | iv | I | bII | I | bvii | I",
        Comping::Aksak,
        Bass::Aksak,
        200.0,
        "Raks aksağı",
        "A Turkish 9/8 counted 2 + 3 + 2 + 2, in makam hicaz: A, B flat, C sharp, D, E, \
         F, G, a major chord on the tonic with D minor, B flat and G minor around it.",
    )
    .in_time(TimeSignature::of_groups(&[2, 3, 2, 2], 8)),
    style(
        "Kopanitsa",
        Balkan,
        HarmonicMinor,
        "i | i | iv | iv | V7 | V7 | i | i",
        Comping::Aksak,
        Bass::Aksak,
        220.0,
        "Kopanitsa",
        "The 11/8 of Gankino horo, 2 + 2 + 3 + 2 + 2: five beats a bar, the long \
         one in the middle, over i, iv and V7.",
    )
    .in_time(TimeSignature::of_groups(&[2, 2, 3, 2, 2], 8)),
    style(
        "Bučimiš",
        Balkan,
        Major,
        "I | I | V7 | V7 | V7 | V7 | I | I",
        Comping::Aksak,
        Bass::Aksak,
        260.0,
        "Bučimiš",
        "Fifteen eighths to the bar, 2 + 2 + 2 + 2 + 3 + 2 + 2: seven beats, the long \
         one fifth, over two chords.",
    )
    .in_time(TimeSignature::of_groups(&[2, 2, 2, 2, 3, 2, 2], 8)),
    style(
        "Tsakonikos",
        Balkan,
        NaturalMinor,
        "i | i | i | iv | iv | i | iv | i",
        Comping::Aksak,
        Bass::Aksak,
        140.0,
        "Tsakonikos",
        "The 5/8 of Tsakonia, 3 + 2, slow, quick, the paidushko's beats the other way \
         round: the bass on the long beat and on the short one, over a minor key's i and iv.",
    )
    .in_time(TimeSignature::of_groups(&[3, 2], 8)),
    style(
        "Five-four",
        Balkan,
        Major,
        "I | I | V7 | I",
        Comping::Aksak,
        Bass::Aksak,
        192.0,
        "Five-four",
        "A folk dance in 5/4, counted 3 + 2: the bass on 1 and 4, the accordion's chord \
         on the beats after each, over I and V7.",
    )
    .in_time(TimeSignature::of_groups(&[3, 2], 4)),
    style(
        "Aksak",
        Balkan,
        Phrygian,
        "bVI | iv | bVI | i | bVI | bIII | bVI | i",
        Comping::Aksak,
        Bass::Aksak,
        90.0,
        "Aksak",
        "The Turkish 9/8, 2 + 2 + 2 + 3, in makam segâh: B, C, D, E, F sharp, G, A, \
         the Phrygian mode, over G major, E minor and D major, coming back to B minor.",
    )
    .in_time(TimeSignature::of_groups(&[2, 2, 2, 3], 8)),
    style(
        "Devr-i hindi",
        Balkan,
        Phrygian,
        "iv | I | iv | bII | bII | I | iv | I",
        Comping::Aksak,
        Bass::Aksak,
        120.0,
        "Devr-i hindi",
        "A Turkish 7/8 counted 3 + 2 + 2, in makam hicaz: A, B flat, C sharp, D, E, F, \
         G, over D minor, B flat and A major.",
    )
    .in_time(TimeSignature::of_groups(&[3, 2, 2], 8)),
    style(
        "Curcuna",
        Balkan,
        Dorian,
        "i | v7 | v7 | IV7 | i | IV7 | v7 | i",
        Comping::Aksak,
        Bass::Aksak,
        96.0,
        "Curcuna",
        "A slow Turkish 10/8 counted 3 + 2 + 2 + 3, in makam hüseyni: A, B, C, D, E, \
         F sharp, G, the Dorian mode, over A minor, E minor 7 and D7.",
    )
    .in_time(TimeSignature::of_groups(&[3, 2, 2, 3], 8)),
    style(
        "Freylekhs",
        Balkan,
        Phrygian,
        "I | iv | I | bvii | I | iv | bII | I",
        Comping::Tresillo,
        Bass::Tresillo,
        136.0,
        "Freylekhs",
        "The freylekhs, the joyful circle dance of klezmer weddings, in freygish, the mode \
         of the Ahava Rabbah prayer: Phrygian with a major third, so the tonic chord is major \
         and the chord a semitone above it (F over E) leads home. The band leans on the \
         bulgar's 3 + 3 + 2.",
    ),
    // ----- Reggae and ska -----
    style(
        "Reggae",
        Reggae,
        Major,
        "I | IV | V | IV",
        Comping::Offbeat,
        Bass::Reggae,
        76.0,
        "One drop",
        "The skank: short chords on every offbeat, while the drums leave beat one empty.",
    ),
    style(
        "Roots minor",
        Reggae,
        Dorian,
        "i7 | IV7 | i7 | IV7",
        Comping::Offbeat,
        Bass::Reggae,
        72.0,
        "Steppers",
        "Two chords of the Dorian mode, the minor key with a major IV, for heavy roots reggae.",
    ),
    style(
        "Ska",
        Reggae,
        Major,
        "I | vi | ii | V",
        Comping::Offbeat,
        Bass::Walking,
        160.0,
        "Ska",
        "Jamaica before reggae: fast, with offbeat chords and a walking bass.",
    ),
    style(
        "Rocksteady",
        Reggae,
        Major,
        "I | vi | IV | V",
        Comping::Offbeat,
        Bass::Walking,
        80.0,
        "Rocksteady",
        "Between ska and reggae, in the mid-1960s: slower than ska, and sweet.",
    ),
    style(
        "Dancehall",
        Reggae,
        NaturalMinor,
        "i | bVII | bVI | bVII",
        Comping::Offbeat,
        Bass::Dembow,
        96.0,
        "Dancehall",
        "Digital Jamaican dance music of the 1980s and after: a minor loop over a kick in 3 + 3 + 2.",
    ),
    style(
        "Lovers rock",
        Reggae,
        Major,
        "Imaj7 | iii7 | IVmaj7 | V",
        Comping::Offbeat,
        Bass::Reggae,
        74.0,
        "One drop",
        "Romantic reggae from 1970s London: soft major seventh chords on the offbeats.",
    ),
    style(
        "Calypso",
        Reggae,
        Major,
        "I | IV | V7 | I",
        Comping::Strum,
        Bass::Tresillo,
        108.0,
        "Calypso",
        "Trinidad's calypso, the song of Lord Kitchener and the Mighty Sparrow: three bright \
         chords, a strummed guitar, and a bass in 3 + 3 + 2 bouncing between root and fifth.",
    ),
    style(
        "Soca",
        Reggae,
        Major,
        "I | IV | I | V",
        Comping::Stabs,
        Bass::Octaves,
        128.0,
        "Soca",
        "Soca, the \"soul of calypso\" from 1970s Trinidad: calypso with the drive of soul and \
         disco, a kick on every beat, octaves in the bass and stabbed chords, for carnival.",
    ),
    // ----- Country and folk -----
    style(
        "Country",
        Folk,
        Major,
        "I | IV | I | V",
        Comping::Strum,
        Bass::RootFifth,
        112.0,
        "Country train",
        "Strummed chords and a bass alternating root and fifth: boom-chick.",
    ),
    style(
        "Folk",
        Folk,
        Major,
        "I | V | IV | I",
        Comping::Strum,
        Bass::Held,
        100.0,
        "Folk",
        "A strummed guitar going out to V and IV and back home.",
    ),
    style(
        "Celtic",
        Folk,
        Dorian,
        "i | bVII | i | bVII",
        Comping::ArpUpDown,
        Bass::Held,
        112.0,
        "Folk",
        "The Dorian mode of Irish and Scottish tunes, rocking between the minor tonic and the \
         major chord a tone below.",
    ),
    style(
        "Polka",
        Folk,
        Major,
        "I | V7 | V7 | I",
        Comping::Offbeat,
        Bass::RootFifth,
        120.0,
        "Two-step",
        "Oom-pah: the bass on the beats, the chords between them.",
    ),
    style(
        "Country waltz",
        Folk,
        Major,
        "I | I | IV | I | I | V7 | V7 | I",
        Comping::Waltz,
        Bass::Waltz,
        120.0,
        "Waltz",
        "Oom-pah-pah: three beats to the bar, the bass on one and the chords on two and three.",
    )
    .in_time(THREE_FOUR),
    style(
        "Two-four polka",
        Folk,
        Major,
        "I | I | V7 | V7 | V7 | V7 | I | I",
        Comping::Backbeat,
        Bass::RootFifth,
        120.0,
        "Two-four polka",
        "The polka in 2/4, two beats to the bar: the bass on 1, the root and the fifth \
         below it by turns from bar to bar, and the chord on 2.",
    )
    .in_time(TWO_FOUR),
    style(
        "Jig",
        Folk,
        Major,
        "I | IV | I | V",
        Comping::Arp3,
        Bass::Held,
        150.0,
        "Six-eight",
        "Six eighth notes to the bar in two groups of three: the bounce of Irish jigs.",
    )
    .in_time(SIX_EIGHT),
    style(
        "Bluegrass",
        Folk,
        Major,
        "I | I | IV | IV | I | V | I | I",
        Comping::Strum,
        Bass::RootFifth,
        180.0,
        "Bluegrass",
        "Banjo, fiddle and mandolin, played fast: three chords and the bass on beats 1 and 3.",
    ),
    style(
        "Tarantella",
        Folk,
        HarmonicMinor,
        "i | V7 | V7 | i",
        Comping::Arp3,
        Bass::Held,
        160.0,
        "Tarantella",
        "A whirling southern Italian dance in fast 6/8, with a tambourine.",
    )
    .in_time(SIX_EIGHT),
    style(
        "Musette waltz",
        Folk,
        Major,
        "I | #i°7 | ii | V7 | I | VI7 | ii | V7",
        Comping::Waltz,
        Bass::Waltz,
        184.0,
        "Musette",
        "The valse musette of the Paris dance halls, on the accordion: a fast waltz felt in \
         one, its chords linked by a chromatic passing chord, C, C sharp diminished, D minor.",
    )
    .in_time(THREE_FOUR),
    style(
        "Fado",
        Folk,
        HarmonicMinor,
        "i | V7 | V7 | i | iv | i | V7 | i",
        Comping::Afterbeat,
        Bass::Alternating,
        76.0,
        "",
        "Lisbon's fado, the song of saudade, in the fado menor: a minor key with only i, iv and \
         V7. The classical guitar (the viola) plays the bass on each beat and the chord between, \
         and the Portuguese guitar answers the singer. Each bar here lasts two bars of 2/4.",
    ),
    style(
        "Western swing",
        Folk,
        Major,
        "I6 | I6 | VI7 | VI7 | II7 | II7 | V7 | V7",
        Comping::Stride,
        Bass::RootFifth,
        176.0,
        "Western swing",
        "The Texas dance-hall swing of Bob Wills and his Texas Playboys: a two-beat bass, a \
         swing guitar, and sixth chords that come home through the dominants, VI7, II7, V7.",
    ),
    style(
        "Honky-tonk",
        Folk,
        Major,
        "I | I7 | IV | I | I | V7 | V7 | I",
        Comping::Shuffle,
        Bass::Walking,
        116.0,
        "Honky-tonk shuffle",
        "The Texas shuffle of 1950s honky-tonk bars, the one Ray Price's band set with \"Crazy \
         Arms\": a shuffling piano over a walking bass, and three chords, I7 turning towards IV.",
    ),
    style(
        "Travis picking",
        Folk,
        Major,
        "I | I | V7 | V7 | V7 | V7 | I | I | III7 | III7 | IV | IV | I | V7 | I | I",
        Comping::Travis,
        Bass::Alternating,
        100.0,
        "",
        "Fingerpicking as Merle Travis played it: the thumb keeps an alternating bass, root \
         and fifth by turns on every beat (here on the bass), while the fingers pick chord \
         notes between them. III7, E7 in C, a dominant that does not go to its own tonic, \
         moves to IV instead.",
    ),
    style(
        "Cajun two-step",
        Folk,
        Major,
        "I | IV | I | V7 | I | IV | V7 | I",
        Comping::Afterbeat,
        Bass::Alternating,
        120.0,
        "Cajun two-step",
        "The Cajun two-step of Louisiana: an accordion and a fiddle over the triangle, the bass \
         on the beats and the chords between, I, IV and V7 as the accordion's buttons give \
         them. The dance is in 2/4; each bar here lasts two.",
    ),
    style(
        "Circle of fifths",
        Classical,
        Major,
        "I | IV | vii° | iii | vi | ii | V | I",
        Comping::Alberti,
        Bass::Held,
        96.0,
        "",
        "The Baroque sequence: every chord of the key, each a fifth below the last.",
    ),
    style(
        "Classical cadence",
        Classical,
        Major,
        "I | IV | V7 | I | vi | ii | V7 | I",
        Comping::Alberti,
        Bass::Held,
        108.0,
        "",
        "Tonic, subdominant, dominant, tonic, with the Alberti bass of Mozart's time.",
    ),
    style(
        "Hymn",
        Classical,
        Major,
        "I | vi | IV | V | I | ii | V | I",
        Comping::Held,
        Bass::Held,
        72.0,
        "",
        "Long chords in four parts, as a church choir sings them.",
    ),
    style(
        "Waltz",
        Classical,
        Major,
        "I | IV | V7 | I | I | IV | V7 | I",
        Comping::Waltz,
        Bass::Waltz,
        150.0,
        "Waltz",
        "The Viennese ballroom: bass on one, chords on two and three.",
    )
    .in_time(THREE_FOUR),
    style(
        "Epic minor",
        Classical,
        NaturalMinor,
        "i | bVI | bIII | bVII | i | bVI | iv | V",
        Comping::Pulse,
        Bass::Pulse,
        90.0,
        "Epic toms",
        "Driving strings and big drums: the film-trailer progression ends on a major V borrowed \
         from harmonic minor.",
    ),
    style(
        "Lydian wonder",
        Classical,
        Lydian,
        "I | II | I | II",
        Comping::ArpUpDown,
        Bass::Held,
        100.0,
        "",
        "The Lydian mode's major II chord: the sound of flying and wonder in film scores.",
    ),
    style(
        "Romanesca",
        Classical,
        NaturalMinor,
        "bIII | bVII | i | V | bIII | bVII | i V | i",
        Comping::Held,
        Bass::Held,
        72.0,
        "",
        "A Renaissance ground: the bass falls a fourth and rises a second, over and over, under endless variations.",
    ),
    style(
        "La folia",
        Classical,
        HarmonicMinor,
        "i | V | i | bVII | bIII | bVII | i | V",
        Comping::Sway,
        Bass::Held,
        90.0,
        "",
        "An old Iberian dance whose chords Corelli, Vivaldi, Bach and Rachmaninoff all wrote variations on.",
    )
    .in_time(THREE_FOUR),
    style(
        "Picardy third",
        Classical,
        HarmonicMinor,
        "i | iv | V7 | I",
        Comping::Held,
        Bass::Held,
        72.0,
        "",
        "A minor piece that ends on a major chord, the \"Picardy third\": a Baroque habit that still sounds like the sun coming out.",
    ),
    style(
        "Neapolitan",
        Classical,
        HarmonicMinor,
        "i | bII | V7 | i",
        Comping::Alberti,
        Bass::Held,
        96.0,
        "",
        "The flat II, a major chord a semitone above the tonic, just before V7: the \"Neapolitan sixth\" of Classical music.",
    ),
    style(
        "Chromatic mediants",
        Classical,
        Major,
        "I | bVI | I | III",
        Comping::Held,
        Bass::Held,
        80.0,
        "Epic toms",
        "Major chords a third apart that share one note: C, A flat, C, E. The awe-struck sound of film scores.",
    ),
    style(
        "Minor plagal",
        Classical,
        Major,
        "I | iv | I | iv",
        Comping::Held,
        Bass::Held,
        70.0,
        "",
        "The minor iv borrowed from the minor key, like a sigh: film music's favourite way to say \"long ago\".",
    ),
    style(
        "March",
        Classical,
        Major,
        "I | V7 | I | IV | I | II7 | V7 | I",
        Comping::Afterbeat,
        Bass::Alternating,
        116.0,
        "March",
        "A military quick march: the tuba on every step, root and fifth in turn, the horns' \
         \"pah\" between them. II7, the dominant of the dominant, pushes the cadence. A march is \
         written in 2/4 or cut time; each bar here lasts two.",
    ),
    style(
        "Six-eight march",
        Classical,
        Major,
        "I | V7 | V7 | I | IV | I | II7 V7 | I",
        Comping::Afterbeat,
        Bass::Alternating,
        180.0,
        "Six-eight march",
        "A march in 6/8, the meter of Sousa's Washington Post and of the two-step it was \
         danced to: two beats to the bar, the tuba on each and the horns' afterbeat on the \
         last eighth of each, in a long-short lilt.",
    )
    .in_time(SIX_EIGHT),
    style(
        "Minuet",
        Classical,
        Major,
        "I | IV | I | V | I | IV | V7 | I",
        Comping::Minuet,
        Bass::Held,
        108.0,
        "",
        "The court dance of the 1700s, stately in 3/4, a chord on every beat. Its phrases come \
         in fours: the first ends on V, a half cadence like a question, and the second answers \
         with V7 to I.",
    )
    .in_time(THREE_FOUR),
    style(
        "Mazurka",
        Classical,
        HarmonicMinor,
        "i | iv | V7 | i | VI | iv | V7 | i",
        Comping::Mazurka,
        Bass::Waltz,
        132.0,
        "",
        "The Polish dance Chopin wrote dozens of: in 3/4, the bass on beat 1 but the weight on \
         beat 3, where the chord is held. A minor key turns to VI, its brightest chord, before \
         iv and V7 bring it home.",
    )
    .in_time(THREE_FOUR),
    style(
        "Polonaise",
        Classical,
        Major,
        "I | IV | I | V7 | I | vi | V7 | I",
        Comping::Polonaise,
        Bass::Held,
        96.0,
        "",
        "The proud Polish processional dance in 3/4, as in Chopin's Military Polonaise: every \
         bar keeps the polonaise rhythm, an eighth and two sixteenths, then four eighths.",
    )
    .in_time(THREE_FOUR),
    style(
        "Siciliano",
        Classical,
        HarmonicMinor,
        "i | iv | V7 | i | VI | bII | V7 | i",
        Comping::Siciliano,
        Bass::Held,
        76.0,
        "",
        "A slow, rocking Baroque dance in 6/8, loved by Bach and Handel: each beat long, short, \
         short. In a minor key, with the Neapolitan chord, the major chord on the flat second \
         (B flat in A minor), on its way to V7.",
    )
    .in_time(SIX_EIGHT),
    style(
        "Baroque continuo",
        Classical,
        Major,
        "I | vi | ii | V7 | I | III7 | vi | II7 V7",
        Comping::Held,
        Bass::Octaves,
        80.0,
        "",
        "The basso continuo of Baroque music: a harpsichord fills in the chords, in parts that \
         never move in parallel fifths or octaves, over a cello leaping in octaves in eighth \
         notes, as in Bach's Air. III7 and II7 are secondary dominants, the V7 of vi and of V.",
    ),
    style(
        "Orchestra",
        Classical,
        Mixolydian,
        "I | bVII | I | bVII | IV | I | bVII | I",
        Comping::Accents,
        Bass::Accents,
        100.0,
        "Orchestra",
        "An orchestra that plays along with the song in whatever meter each bar is in, for music whose meter changes from bar to bar, as in The Rite of Spring: on each beat the bar is felt in, the horns strike the chord and the timpani its root, the strings hold it, and the bass drum marks every bar. The song keeps its meters, its tempo and its own parts. Its chords swing between I and the flat VII of the Mixolydian mode.",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::RHYTHMS;
    use crate::pattern::MAX_STEPS;
    use std::collections::HashSet;

    #[test]
    fn every_style_parses_and_fits_in_a_pattern() {
        for style in STYLES {
            let bar = style.time.steps_per_bar();
            assert!(
                chart::parse_chart(style.chart, bar).is_some(),
                "{}: {}",
                style.name,
                style.chart
            );
            assert!(style.length() <= MAX_STEPS, "{} is too long", style.name);
            let spans = style.spans();
            let end = spans.last().map(|span| span.start + span.length);
            assert_eq!(end, Some(style.length()), "{}", style.name);
        }
    }

    #[test]
    fn names_are_unique_and_rhythms_exist() {
        let mut names = HashSet::new();
        for style in STYLES {
            assert!(names.insert(style.name), "{} twice", style.name);
            assert!(
                style.rhythm.is_empty() || RHYTHMS.iter().any(|r| r.name == style.rhythm),
                "{}: no rhythm called {}",
                style.name,
                style.rhythm
            );
            // Tempos count quarter notes: the fast dances counted in
            // eighths, such as a rachenitsa, go past 200.
            let fastest = if style.time.unit >= 8 { 280.0 } else { 200.0 };
            assert!((40.0..=fastest).contains(&style.tempo), "{}", style.name);
            assert!(!style.about.is_empty());
        }
        assert!(STYLES.len() >= 60, "a large library");
    }

    #[test]
    fn every_genre_has_styles() {
        for genre in Genre::ALL {
            assert!(STYLES.iter().any(|style| style.genre == genre), "{genre:?}");
        }
    }

    #[test]
    fn styles_play_in_any_key() {
        let style = Style::named("Andalusian cadence").unwrap();
        assert_eq!(style.chord_names(PitchClass::A), "Am – G – F – E");
        assert_eq!(style.chord_names(PitchClass::E), "Em – D – C – B");
        // The flat II of the bossa nova is a D flat in C, not a C sharp.
        let bossa = Style::named("Bossa nova").unwrap();
        assert!(bossa.chord_names(PitchClass::C).ends_with("Db7"));
        // Spelled in the key: Db7, not C#7, in C; in Db major the flat II is D.
        let four = Style::named("Four chords").unwrap();
        assert_eq!(four.chord_names(PitchClass::FSharp), "F# – C# – D#m – B");
        assert_eq!(four.chord_names(PitchClass::CSharp), "Db – Ab – Bbm – Gb");
        let backing = style.render(PitchClass::A);
        assert!(!backing.chords.is_empty() && !backing.bass.is_empty());
        let end = backing.chords.iter().map(|note| note.end()).max();
        assert_eq!(end, Some(style.length()), "nothing sounds past the end");
    }

    #[test]
    fn the_candombes_play_their_chords_in_the_clave() {
        for name in ["Candombe", "Candombe Ansina"] {
            let style = Style::named(name).unwrap();
            assert_eq!(
                (style.comping, style.bass),
                (Comping::Bloques, Bass::Tresillo)
            );
            let backing = style.render(PitchClass::C);
            let mut starts: Vec<usize> = backing
                .chords
                .iter()
                .filter(|note| note.start < 16)
                .map(|note| note.start)
                .collect();
            starts.dedup();
            assert_eq!(starts, [0, 3, 6, 10, 12], "{name}");
        }
    }

    #[test]
    fn three_four_styles_have_bars_of_twelve() {
        let waltz = Style::named("Waltz").unwrap();
        assert_eq!(waltz.length(), 8 * 12);
        let starts: Vec<usize> = waltz.spans().iter().map(|span| span.start).collect();
        assert_eq!(starts[..3], [0, 12, 24]);
    }
}
