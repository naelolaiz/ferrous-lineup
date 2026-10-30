# How a demo is written

Files: [`crates/lineup-core/src/library/song.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song.rs),
[`crates/lineup-core/src/library/song/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates/lineup-core/src/library/song),
[`crates/lineup-core/src/library/chart.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/chart.rs),
[`crates/lineup-core/src/library/demo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/demo.rs),
[`crates/lineup-core/src/library/interpret.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/interpret.rs)
and [`crates/lineup-app/src/app/demo_menu.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/demo_menu.rs)

**File > Demos** opens 343 songs, from nursery rhymes to the Rite of
Spring, and a short demo of every style, twelve-tone style and
micromode style. Click one and it opens, its chords on the chord sheet,
and plays. What to listen for in them is in the manual's [listening
guide](https://naelolaiz.github.io/ferrous-lineup/manual/finding-a-demo.html).
This chapter is about what a demo *is*.

A song demo is not a file. It is a Rust constant, compiled into the
program: the tune and the chords written as text, the way a lead sheet
writes them, and a few fields that say how the song is played.
`song_demo` turns the constant into an ordinary `Project` each time you
open it. That has three good sides:

- **The tests read every song.** A bar one sixteenth too long, a chord
  that cannot be read, a form that names a part that does not exist:
  each fails `cargo test` before anyone hears it.
- **A demo is the same everywhere.** It cannot be missing, damaged or
  out of date; a new version of the app brings its own copy.
- **What the songs need, the library learns.** A piece that changes
  key, meter or tempo, a round, an orchestra handing its tune from the
  flute to the clarinet: each became a feature of `Part`, and every
  song can use it.

A demo is read-only for the same reason: there is no file to write your
changes back to, so **Save** asks for a new one (see [The title
bar](../program/saving.md#the-title-bar)).

## A song is a constant

Here is a whole song, the round Frère Jacques:

```rust
pub(super) const FRERE_JACQUES: Song = Song {
    name: "Frère Jacques",
    origin: "French round, 18th century",
    feel: "Round",
    genre: Genre::Folk,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 104.0,
    comping: Comping::ArpUpDown,
    bass: Bass::RootFifth,
    rhythm: "Folk",
    band: SONG_AND_GUITAR,
    style: "Folk",
    parts: &[
        Part::new("Alone", FRERE_JACQUES_TUNE, FRERE_JACQUES_CHORDS),
        Part::new("Round", FRERE_JACQUES_TUNE, FRERE_JACQUES_CHORDS).with_voices(&[
            (1, 0),
            (2, 2),
            (3, 4),
            (4, 6),
        ]),
        // ...
    ],
    form: &["Alone", "Round", "Round again", "Last voices"],
    drums_from: 0,
    about: "A round: four voices sing the same tune, each two bars after the last. It works \
            because every two bars of the tune fit the same chord.",
};
```

The tune and the chords are constants of their own above it, because
four parts use them:

```rust
const FRERE_JACQUES_TUNE: &str = "C5:4 D5 E5 C5 | C5 D5 E5 C5 | E5 F5 G5:8 | E5:4 F5 G5:8 | \
     G5:2 A5 G5 F5 E5:4 C5 | G5:2 A5 G5 F5 E5:4 C5 | C5 G4 C5:8 | C5:4 G4 C5:8";
const FRERE_JACQUES_CHORDS: &str = "I | I | I | I | I | I | I | I";
```

The fields read top to bottom like the heading of a lead sheet: what
it is and where it comes from, its key, meter and tempo; then how the
band dresses it, a way of comping and a bass line from the library (see
[Comping: how the chords are
played](../band/library.md#comping-how-the-chords-are-played)), a
rhythm by name, the instruments (`band`) and the style it is closest
to, which the Library shows selected while the demo is open. Then the
music: the **parts**, each a section such as a verse or a chorus, and
the **form**, the order they are played in, by name. A part played
three times is written once and named three times. `drums_from` is the
entry of the form where the drums come in, 0 for the start, 1 when the
song opens with a pickup or an introduction the drums leave alone.

### One file per genre

343 songs would make one file of tens of thousands of lines. So
`song.rs` holds the types, the shared constants, the list and the
tests, and the songs live next to it, one file per genre:

```text
library/
  song.rs          Song, Part, Band, SONGS, the tests
  song/
    balkan.rs
    blues.rs
    classical.rs
    ...
    tango_played.rs
```

`mod tango;` inside `song.rs` makes Rust look for `song/tango.rs`: each
genre file is a **child module** of `song`. A song is
`pub(super) const`, visible to its parent and no further: nothing
outside the library needs the name `HABANERA`, only the song, which it
finds in the list.

Each genre file starts with one line:

```rust
use super::*;
```

`super` is the parent module and `*` brings in every name it can see,
its private ones too: in Rust an item private to a module is visible to
that module's children. So a song can use `COMMON`, `SIX_EIGHT`, the
bands and every other constant `song.rs` keeps to itself. Two of the
parent's own `use` lines matter most:

```rust
use PitchClass::*;
use ScaleKind::*;
```

They let a song write `tonic: A` and `mode: Dorian` rather than
`PitchClass::A` and `ScaleKind::Dorian`. `Genre` is not imported that
way, on purpose: `Genre::Blues` and `ScaleKind::Blues` would both be
`Blues`, and two glob imports with the same name are an error as soon
as the name is used. So a genre is always written in full,
`genre: Genre::Blues`, and `mode: Blues` is the scale.

### The list

```rust
pub static SONGS: &[Song] = &[
    pop::AULD_LANG_SYNE,
    pop::LONDON_BRIDGE,
    pop::OLD_MACDONALD,
```

Every song is listed by name. Rust cannot join slices together while
it compiles, so the genre files cannot each bring a list of their own;
naming each `const` is the plain way. A `const` is copied in wherever
it is used, so the list holds the songs themselves, and being a
`static`, it has one place in memory for the whole run: the menu and
the Library can hand out `&'static Song` references into it. The list
is grouped by genre, in the order of `Genre::ALL`, and a test checks it
with `windows(2)`, which gives every pair of neighbours:

```rust
SONGS.windows(2).all(|pair| pair[0].genre <= pair[1].genre),
```

`<=` works on genres because `Genre` derives `PartialOrd`, which orders
an enum's variants as they are written. The menu sorts the songs by
title on its own (see [The Demos menu](#the-demos-menu)), so a new song
goes at the end of its genre's block, and two changes adding songs at
once do not fight over the same lines.

### Where the notes come from

Each song carries `origin`, who wrote it and when, and a `// Source:`
comment above it naming the written score, lead sheet, engraving or
recording its notes were taken from, and what was simplified. The
Promenade's comment, for instance, names the LilyPond engraving it
was read from and explains how a chord was chosen for each beat. Almost
every song is old enough to belong to everyone; the few that are still
under copyright somewhere say so in their comment. Twenty-five are new
tunes for genres with few free ones, written the way those genres write
theirs, and their `origin` says "Written for Ferrous Lineup". How a
performance or a score becomes a constant is the next chapter, [From a
MIDI performance or a score to a
demo](from-midi-and-scores.md).

## The tune, as text

A melody is written in a small notation, read by `chart::parse_melody`:

| Text | Means |
|---|---|
| `C4`, `F#5`, `Bb3` | a note: its letter, `#` or `b`, and its octave (`C4` is middle C) |
| `:4` | its length in steps (4 is a quarter note); without it, as long as the note before |
| `:1/2`, `:1/4` | shorter than a step: a thirty-second, a sixty-fourth, for grace notes and runs |
| `-:8` | a rest, here half a bar of 4/4 |
| `_:8` | a tie: the note before goes on, across a bar line too |
| `\|` | a bar line: it changes nothing, but each bar is checked |
| `A3+C#4+E4:4` | a chord, its notes struck together |
| `&` | another voice, from the start, at the same time: `E5:4 D5 C5 B4 & C3:16` |
| `(3` | the next three notes or rests are a triplet, three in the time of two |
| `!p`, `!f`, `!sfz` | a dynamic, from `!pp` to `!ff`, or a sforzando on the next note |
| `!<`, `!>` | a crescendo or a diminuendo, up to the next dynamic |
| `>A4:2`, `D5:2.` | an accent before a note; a staccato dot after it |

The marks take no time. A triplet's notes start between the steps of
the grid, so they are placed to the tick (see
[Notes between steps](../sound/midi.md#notes-between-steps)). The verse of
Greensleeves reads like this:

```text
C5:8 D5:4 | E5:6 F#5:2 E5:4 | D5:8 B4:4 | G4:6 A4:2 B4:4 | ...
```

A tune that begins before its first bar, as the "In a" of
"Clementine", has a **pickup**. It is written as a part of its own, a
bar that is nearly all rest:

```rust
Part::new("Pickup", "-:8 C5:3 C5:1", "I"),
```

That is one bar of 3/4: two beats of rest, then "In a", a dotted
eighth and a sixteenth. The first strong beat of
the tune then falls on the first beat of the verse, where the drums and
the chords expect it.

## The chords, as a chart

Under the tune goes a chord chart, bar by bar (see [Chord charts:
chords by the bar](../band/library.md#chord-charts-chords-by-the-bar)):
numerals such as `i | bIII | bVII | v`, or names as a lead sheet writes
them, `C#m | G#7/B#`, read in the part's key. The marks of the chord
sheet work too: a chart can push a chord or stop the band (see [Shots,
holds, rests and pushes](../band/stops.md)). The chart is what the
band plays, with the song's comping and bass line, and what the chord
sheet shows.

Some music's accompaniment is part of the piece: a rag's left hand, a
Bach chorale, a choir. Then the part writes it out, and its chords are
not accompanied:

- `with_left(...)` writes notes for the **Keys** channel, in the
  notation of tunes, `&` and all;
- `with_lines(&[...])` writes **other instruments**, each a `Line` with
  the name of its channel, a General MIDI program and its notes. Lines
  with the same name in any part play on the same channel, so the
  Misa Criolla's choir is one line, "Choir", with its alto, tenor and
  bass joined by `&`.

A written-out part may still have a chart: then it names what the
music plays, for the chord sheet, and the band leaves it alone.

## Parts that change something

`Part::new(name, melody, chords)` makes a plain part. The others are
the same part with one field changed, by a `const fn` that uses
**struct update syntax**:

```rust
const fn in_key(self, tonic: PitchClass, mode: ScaleKind) -> Part {
    Part {
        key: Some((tonic, mode)),
        ..self
    }
}
```

`..self` means "every other field as in `self`". Because it only moves
values around, it can be `const`, and a song stays a constant with
`.in_key(FSharp, Major)` at the end of one of its parts. The rest work
the same way:

| Method | The part... | For example |
|---|---|---|
| `in_key(tonic, mode)` | is in another key: a modulation | the Mountain King climbs to F♯ major |
| `in_time(time)` | is in another time signature | a refrain in 3/4 in a song in 2/4 |
| `in_meters(&[...])` | has a time signature for each bar | the Promenade's 5/4 and 6/4 |
| `with_tempo(&[...])` | has the score's tempo markings | "Più vivo", a ritardando, a fermata |
| `played(comping, bass, rhythm)` | is accompanied its own way | the refrain in 3/4 needs a waltz's comping |
| `with_voices(&[...])` | is sung by several voices | a round |
| `played_by(&[...])` | hands its tune to other instruments | Boléro, from the flute to the clarinet |
| `marked(dynamic)` | is played this loud | Boléro's pianissimo start |
| `with_drums(&[...])` | has its drums written out | Boléro's snare drum |
| `with_left`, `with_lines` | writes out its accompaniment | a rag's left hand, a choir |

In the Hall of the Mountain King uses two of them:

```rust
Part::new("Higher", MOUNTAIN_HIGHER, MOUNTAIN_HIGHER_CHORDS).in_key(FSharp, Major),
// "Più vivo": faster at once, at Grieg's own metronome mark.
Part::new("Più vivo", MOUNTAIN_THEME, MOUNTAIN_CHORDS).with_tempo(&[tempo(0, 154)]),
```

### A key of its own

A part in another key reads its numerals in that key, so `I` in the F♯
major part is an F♯ chord, and the demo puts a key marker where it
starts and another where the music goes back. The song answers which
key a part is in:

```rust
pub fn part_scale(&self, part: &Part) -> Scale {
    match part.key {
        Some((tonic, mode)) => written_as_tune(Scale::new(tonic, mode), [part]),
        None => self.scale(),
    }
}
```

`written_as_tune` decides the key's *name*. A `PitchClass` has one
variant for each of the twelve notes, with sharps (`ASharp`, never a B
flat), but the Promenade is in B♭ major, not A♯ major, and Take Five in
E♭ minor, not D♯ minor. Rather than make every song spell its key, the
library counts the accidentals of the tune and names the key the way
the tune is written:

```rust
fn written_as_tune<'a>(key: Scale, parts: impl IntoIterator<Item = &'a Part>) -> Scale {
    let (sharps, flats) = parts
        .into_iter()
        .map(|part| accidentals(part.melody))
        .fold((0, 0), |(sharps, flats), (s, f)| (sharps + s, flats + f));
    match sharps.cmp(&flats) {
        Ordering::Greater => key.with_spelling(Spelling::Sharps),
        Ordering::Less => key.with_spelling(Spelling::Flats),
        Ordering::Equal => key,
    }
}
```

`fold` carries a pair through the parts, adding up both counts at
once, and `cmp` returns an `Ordering`, which the `match` reads as
"more sharps", "more flats" or "as many". A tune with as many of each,
or none, leaves the key its usual name. How a key is spelled is in
[Sharps or flats: how a key is
written](../theory/scales.md#sharps-or-flats-how-a-key-is-written)

Music in no key at all, where all twelve notes are at home, says so
with the chromatic scale: the Ritual of Abduction from start to end,
with `mode: Chromatic`, and parts of Petrushka with
`.in_key(C, Chromatic)`. Such a part has no key signature, and every
note it plays counts as in its key.

### A meter for each bar

Most parts are in the song's time signature or one of their own. Some
music changes meter nearly every bar: the Promenade alternates 5/4 and
6/4, the Rite of Spring goes through 2/8, 3/8, 3/4, 5/8 and more. A
part with `in_meters(&[...])` has one time signature per bar. A
pattern has one time signature, so `Song::part_runs` cuts such a part
into **runs** of bars in one meter, each a part of its own with its
share of the tune and the chords:

```rust
let mut runs: Vec<(usize, usize, TimeSignature)> = Vec::new();
for (bar, &time) in part.meters.iter().enumerate() {
    match runs.last_mut() {
        Some((_, bars, last)) if *last == time => *bars += 1,
        _ => runs.push((bar, 1, time)),
    }
}
```

`last_mut` gives the last run, if there is one, to change in place: a
bar in the same meter makes it one bar longer, and any other bar starts
a new run. The pieces of the text come from `bars_of`, which returns a
slice of the part's own `&'static str` between two bar lines, so a run
is a `Part` like any other and costs no copy. The demo names the
pattern of each later run after the bar it starts on, "Call, bar 2",
and puts a time marker wherever the meter changes. The Rite of Spring
would need more patterns than a project allows, so a run whose notes
are the same as an earlier run's, in the same meter, plays that
earlier pattern again, as a producer reuses a loop.

### The score's tempo

A `TempoMark` is one of three things a score writes:

```rust
pub enum TempoMark {
    /// From `at` on, `bpm` beats a minute, at once: a new section with
    /// a new tempo, such as "Più mosso" (more movement), "Adagio" or
    /// "Tempo I" (the first tempo again).
    At { at: usize, bpm: u16 },
    /// From `at`, slowing down or speeding up bit by bit until `bpm`,
    /// reached `over` steps later: "rit." or "rall." (slowing down),
    /// "accel." or "stringendo" (speeding up).
    Glide { at: usize, bpm: u16, over: usize },
    /// A fermata (𝄐), the "bird's eye" over a note or a rest: from `at`,
    /// for `steps` steps, the music waits, lasting twice as long; then
    /// it carries on at the tempo before.
    Fermata { at: usize, steps: usize },
}
```

`tempo(at, bpm)`, `glide(at, bpm, over)` and `fermata(at, steps)` are
`const fn`s that build them, counting steps from the start of the part.
The demo turns each into tempo markers on the playlist, every time the
part is played. A fermata becomes two: half the tempo for as long as
the held note lasts, then the tempo before. The project has no fermata
of its own; half as fast is exactly "twice as long" (see [Tempo
changes](../sound/tempo.md#tempo-changes)).

### Rounds and canons

`with_voices` says who sings the tune, and from which bar of the part:
one `(voice, bar)` for each copy. Frère Jacques' round is four voices,
each two bars after the one before:

```rust
Part::new("Round", FRERE_JACQUES_TUNE, FRERE_JACQUES_CHORDS).with_voices(&[
    (1, 0),
    (2, 2),
    (3, 4),
    (4, 6),
]),
```

A negative bar is a voice that began in the part before, so only its
end is heard: "Round again" lets voices 2 to 4 finish their last bars
while they start again. Voice 1 is the Lead channel, and every other
voice gets a channel of its own, "Voice 2", "Voice 3" and so on.

### Other instruments, louder and softer

`played_by` takes a list of `(program, semitones)`: each instrument
plays the tune, that many semitones higher, on a channel named after
it. Boléro hands its tune from the flute to the clarinet, the bassoon
and on; its E♭ clarinet plays it an octave up, `(gm::CLARINET, 12)`.
`marked` sets how loud the part is played, from pianissimo to
fortissimo:

```rust
pub fn scale(self) -> f32 {
    match self {
        Dynamic::Pianissimo => 0.4,
        Dynamic::Piano => 0.55,
        Dynamic::MezzoPiano => 0.75,
        Dynamic::MezzoForte => 1.0,
        Dynamic::Forte => 1.15,
        Dynamic::Fortissimo => 1.3,
    }
}
```

`with_drums` writes a drum part out, in the notation of tunes, with `x`
for a stroke and `X` for an accented one. Rather than a second parser,
`drum_line` turns each `x` into a C4 and each `X` into a D4, reads the
line as a melody, then makes every note a one-step hit, accented if it
was a D4, as loud as the line's dynamics say: triplets, rests and
dynamics come for free.

## The band of a song

`band` names the General MIDI programs of the Lead, the Keys and the
Bass channels, and maybe a horn section:

```rust
pub struct Band {
    pub lead: u8,
    pub keys: u8,
    pub bass: u8,
    pub horns: Option<Horns>,
}
```

Most bands are constants shared by the songs of a genre:

```rust
const BOSSA: Band = Band::new(gm::TENOR_SAX, gm::NYLON_GUITAR, gm::ACOUSTIC_BASS);
```

`Band::new` and `with_horns` are `const fn`s, so a band can be built
inside a constant. A horn section is a program and a way of comping:
a big band's saxophones hold the chords (`Comping::Held`) and its brass
punches them on the beat and just before beat 3
(`Comping::Charleston`). A tango orchestra's violins are a "horn
section" too, with a string program: the demo calls the channel
"Strings" for programs 40 to 51 and "Horns" for anything else. A song
with a band of horns is a different thing from the band style's
[horn section](../band/band.md#a-horn-section), which any song can
switch on.

## From a song to a project

`song_demo(song)` builds the project, in four steps.

**Channels.** The Lead, a channel for each extra voice, the Keys only if
some part plays chords or a left hand, the Bass unless the song has no
bass line, the horns, a channel for each instrument a tune is handed
to, and one for each line's name. Each gets its program.

**A pattern for each part,** or for each run of a part whose meter
changes. Into it go the tune on its voices' channels, the chart
accompanied with the part's comping and bass (`accompany_in_bars`,
which keeps the comping in step with the bars, so a chord a beat long is
still struck), the horns' part, the written-out left hand and lines,
and the written-out drums. Then two passes make them sound played (see
the next section) and the part's dynamic scales them.

**A drum pattern for each part,** from the song's rhythm, or the part's
own, in the part's meter, with a fill at the end of parts four bars or
longer, unless the part writes its drums out.

**The arrangement.** The form is walked once, each entry placing its
part's patterns one after another on the Song track and its drums on
the Drums track from `drums_from` on, with key, tempo and time markers
where they change and the chart written on the chord sheet. A last bar
on the home chord, with a crash, ends the song, in the key of the last
part, so a song that modulates ends in the new key. A piece whose last
part is written out ends where its score ends, and so does a chart that
ends on a break, as a tango's "chan-chan": the score already says how
the music stops.

Two small touches finish it. Songs, hymns, tangos and concert pieces
slow down a little in their very last bar, as a player does where there
is no drum machine to keep up with:

```rust
fn ends_broadening(song: &Song) -> bool {
    use super::Genre::*;
    matches!(song.genre, Classical | Folk | Soul | Tango | Folklore)
}
```

That is a *poco ritardando* to nine tenths of the tempo, unless the
score already marks how the last part's tempo goes. And the swing is
the song's rhythm's, except in folk and classical music, whose eighths
stay even over any beat:

```rust
pub fn swing(&self) -> u8 {
    match self.genre {
        Genre::Folk | Genre::Classical => 0,
        _ => Rhythm::named(self.rhythm).map_or(0, |rhythm| rhythm.swing),
    }
}
```

The result is an ordinary project, to take apart and change: nothing in
it is marked as a demo.

## Playing like a person

A score written out note by note, every note as hard as the next,
sounds like a machine. `interpret.rs` changes the velocities, a few
steps out of 127, the way a player would; the notes stay where they
are. For a tune, `perform` shapes each phrase:

- the first beat of a bar a little harder, the other beats less, the
  notes off the beat lighter;
- the higher a note in its phrase, the more it is sung out;
- a long note leaned on, the last note of a phrase relaxed;
- and no two notes quite alike.

A phrase ends at a rest of an eighth note or more, the shortest breath a
singer takes, or after four bars. The chords and the bass, whose
accents the comping already wrote, only lose their evenness: the notes
of a chord a little apart from each other, the chords a little more.
Then `follow` makes the band as loud as the tune is marked where each of
its notes starts, so a crescendo in the melody carries the band with
it, and an accent in the tune is half an accent in the band.

"No two quite alike" must not mean "different every time": a demo has
to sound the same each time it is opened, and a test that checks its
notes must see the same notes. So the small differences come from a
function that looks random but is not:

```rust
fn wobble(seed: u64, value: u64) -> f32 {
    let mut x = seed ^ value.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^= x >> 31;
    (x >> 40) as f32 / (1u64 << 23) as f32 - 1.0
}
```

This is the mixing step of **SplitMix64**: multiplications and shifts
that scatter the bits of the input so thoroughly that neighbouring
inputs give unrelated outputs. The input is the note's step and pitch;
the seed is made from the song's, the part's and the channel's names
with FNV-1a (see [FNV-1a](../band/band.md#fnv-1a)). The top 24 bits
of the result, divided by 2²³, give a number from 0 to 2, and the
minus one makes it run from −1 to 1. `wrapping_mul` multiplies and
throws away what does not fit in 64 bits, which plain `*` would treat as
an overflow. The shaping is kept small next to the difference between
piano and forte: a velocity changed by `clamp` stays between 24 and
124, or where it was written if that is further out, so a pianissimo
stays soft.

## The Demos menu

The menu lists the demos by kind (**Songs**, **Styles**,
**Twelve-tone**, **Micromodes**), songs and styles by genre, and every
list in alphabetical order. The library's own lists keep the order they
were written in; the menu sorts a copy, built the first time it is
shown, in a `LazyLock`:

```rust
static DEMOS: LazyLock<Demos> = LazyLock::new(|| Language::English.with(all_demos));
static DEMOS_ES: LazyLock<Demos> = LazyLock::new(|| Language::Spanish.with(all_demos));
```

A `static` must be known when the program is compiled; a `LazyLock`
is a `static` whose value is worked out by the closure the first time
anyone reads it, and kept. There are two, because the titles are
sorted as they are shown, and Spanish shows many in Spanish.

### Alphabetical, for people

Comparing text letter by letter, by Unicode number, puts every capital
before every small letter and "Ábalos" after "Zamba". So the menu sorts
by a **folded** copy of each title: small letters, no accents, no
punctuation, a hyphen as a space. "Für Elise" sorts as "fur elise",
right after "Funeral March":

```rust
fn by_title(a: &str, b: &str) -> Ordering {
    fold(a).cmp(&fold(b)).then_with(|| a.cmp(b))
}
```

`then_with` breaks a tie, two titles that fold the same, by the
titles themselves, so the order never depends on chance. `fold` is one
`match` over the characters: each accented letter becomes its plain
one, `♯` and `♭` become `#` and `b`, letters, digits, `#`, `/` and `+`
stay (so "F#", "3/4" and "2+2+3/8" can be searched), and any run of
spaces and marks between words becomes one space.

### Search

A search box sits above the submenus and has the keyboard as soon as
the menu opens. Each demo has three folded texts: its title (in both
languages, so "Noche de paz" finds Silent Night in either), its details
(who wrote it, its feel, genre, key and time signature) and the notes
about it. Every word typed must start a word in one of them: `bach`
finds Bach, not Offenbach:

```rust
fn starts_a_word(text: &str, words: &str) -> bool {
    text.match_indices(words)
        .any(|(at, _)| at == 0 || text[..at].ends_with(' '))
}
```

The matches are ranked, best first:

```rust
enum Rank {
    TitleStart,
    TitleWords,
    Details,
    Words,
    About,
}
```

The title starting with what was typed comes first, then every word
found in the title, then the phrase found in the details, as "d minor"
in a song in D minor, then the words found apart, and last a word found
only in the notes. `Rank` derives `Ord`, so the variants compare in the
order they are written, and a **stable** sort by rank keeps the
alphabetical order among equals. The two weakest ranks are shown
fainter, so the matches you most likely meant stand out.

## Testing it

The songs are data, and the tests read all of it. In `song.rs`:
`every_bar_of_every_part_adds_up` (every bar of every tune and left hand
is as long as its time signature, every chart can be read, a part with
a meter per bar has as many meters as bars),
`forms_name_real_parts_and_rhythms_exist`, `every_song_names_a_real_style`,
`songs_in_three_play_in_three`,
`songs_that_change_meter_have_a_style_that_follows_it`,
`every_genre_has_songs_and_they_are_in_order`,
`keys_are_written_as_their_tunes_are` (Take Five in E♭ minor, the
Maple Leaf Rag's trio in D♭ major), `melodies_are_mostly_in_their_key`
(at least 80% of a song's notes in its part's key, which catches an F
written for an F♯) and `the_first_notes_are_right` (Ode to Joy starts
F♯ F♯ G A).

In `demo.rs`, `every_song_makes_a_demo` builds all 343, and others
check one thing each: `a_round_has_its_voices_in_the_right_places`,
`a_part_in_its_own_time_gets_time_markers`,
`runs_of_the_same_bars_share_a_pattern`,
`tempo_markings_go_on_the_playlist`,
`songs_and_hymns_slow_down_a_little_at_the_end`,
`a_song_that_ends_after_a_modulation_ends_in_the_new_key`,
`a_piece_written_out_in_full_ends_where_its_score_ends`,
`a_big_band_has_its_horns`,
`the_band_follows_the_dynamics_of_the_tune_and_the_breaks_of_the_chart`
and `every_demo_keeps_a_sensible_tempo` (every tempo marker between
three tenths and two and a half times the song's tempo). `interpret.rs` checks that
`piano_stays_soft_and_fortissimo_loud` and that
`a_tune_gets_its_accents_but_stays_close_to_how_it_was_written`.
`demo_menu.rs` checks the folding, the order and the ranks, such as
`matches_only_in_the_notes_come_last`.

Beyond the unit tests, any demo can be played headless with the
[playback trace](../sound/trace.md), `ferrous-lineup --trace "Frère
Jacques" --out round.tsv`, and `ferrous-lineup --demos` lists the
names it takes.
