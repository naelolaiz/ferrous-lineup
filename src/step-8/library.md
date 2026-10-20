# The library: knowledge as data

Files: [`crates/lineup-core/src/library/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates/lineup-core/src/library)
and [`crates/lineup-audio/src/drums.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/drums.rs)

A musician who sits down to play a bossa nova knows four things: the
chords, how the guitar plays them, what the bass does, and the drum
pattern. None of it is code. It is *knowledge*, and the best way to put
knowledge into a program is to write it down as data, in a form a
person can read and check, and to keep the code that uses it small.

Step 6 did this for chord suggestions, with a `const` table. Step 8
does it for a whole library: 78 harmony styles, 49 rhythms, 6 fills
and 16 songs. Each one is a few lines in a table. This chapter goes
through the notations, from chords to songs, and the Rust that reads
them.

## Chords as Roman numerals

`numeral.rs`

A progression written as chord names, "C G Am F", only works in C. The
same progression as numerals, "I V vi IV", works in every key: the
numeral says which note of the key the chord is built on, and its case
says major (upper) or minor (lower). What follows adds the rest: "7",
"maj7", "°", "sus4"...

There is one choice to make: counted along *which* scale? The library
counts along the **major** scale of the tonic, whatever the key, the
way most pop and jazz books do. III is always four semitones above the
tonic, and a flat in front lowers it, so the chords of A minor are
written i, ii°, bIII, iv, v, bVI, bVII. That lets one table hold
progressions in any mode, and "bVII" means the same chord in a minor
key, a Mixolydian rock song and a Dorian jam.

```rust
pub struct RomanChord {
    pub semitones: u8,
    pub quality: ChordQuality,
    pub spelling: Option<Spelling>,
}
```

`parse` reads one numeral. The longest numerals are tried first, so
"III" is not read as "I" followed by "II", and the suffixes come from a
table that says which quality each suffix means after an upper-case and
a lower-case numeral:

```rust
const SUFFIXES: [(&str, Option<ChordQuality>, Option<ChordQuality>); 20] = {
    use ChordQuality::*;
    [
        ("", Some(Major), Some(Minor)),
        ("7", Some(Dominant7), Some(Minor7)),
        ("maj7", Some(Major7), Some(MinorMajor7)),
        // ...
        ("°", None, Some(Diminished)),
        // ...
    ]
};
```

A `const` can be a whole block that ends in a value, as here, and the
`use` inside it only applies inside the block. `None` means "not
allowed": "V°" is not a chord anybody writes.

`spelling` remembers a flat or a sharp in front, so bII7 in C is
written D♭7, not C♯7: same key on the piano, but the name tells a
musician what the chord does.

## Chord charts: chords by the bar

`chart.rs`

A **lead sheet**, the page a working musician reads, has the melody
with chord names above it, bar by bar. The library writes chords the
same way, with `|` between bars:

```text
"I | IV V | vi | ii7 V7"
```

The chords in a bar share it equally, so "IV V" is half a bar each. A
chord that repeats the one before is *held on*, not played again:
"I | I" is two bars of one chord. `parse_chart` turns a chart into
`ChordSpan`s, a chord with where it starts and how long it lasts, in
steps of the style's time signature.

## Comping: how the chords are played

`comping.rs`

The same four chords can be a ballad, a reggae tune or a house track.
What changes is the rhythm of the chords, which jazz musicians call
**comping** (short for accompanying), and what the bass does. Each way
of comping is a short table of *hits*: when a note starts, how long it
lasts, which notes of the chord it plays, and how hard:

```rust
Comping::Pulse => (4, &[hit!(0, 2, All, 96), hit!(2, 2, All, 78)]),
Comping::Offbeat => (4, &[hit!(2, 1, All, 92)]),
Comping::Habanera => (
    8,
    &[
        hit!(0, 3, All, 96),
        hit!(3, 1, Upper, 74),
        hit!(4, 2, All, 86),
        hit!(6, 2, All, 84),
    ],
),
```

The first number is the *period*: the hits repeat every 4 steps for
`Pulse` (eighth notes), every 8 for the habanera's long, short, long,
long. `Offbeat` is the reggae "skank": one short chord on the second
eighth note of every beat. `Upper` plays every note of the chord but
the lowest, like a guitarist's lighter up-stroke.

### `macro_rules!`, and why not a function

`hit!` is a small *declarative macro*:

```rust
macro_rules! hit {
    ($at:expr, $len:expr, $what:expr, $velocity:expr) => {
        Hit {
            at: $at,
            len: $len,
            what: $what,
            velocity: $velocity,
        }
    };
}
```

A macro is code that writes code. `macro_rules!` matches what it is
given against a pattern (here four expressions, `$at:expr` and so on)
and pastes them into the template on the right. `hit!(0, 2, All, 96)`
becomes `Hit { at: 0, len: 2, what: All, velocity: 96 }` before the
compiler checks anything.

Why not a `const fn hit(...) -> Hit`? Look at the type the match
returns: `&'static [Hit<Voices>]`, a borrowed slice that lives for the
whole program. Writing `&[ ... ]` of plain values gives exactly that:
the compiler sees an expression made only of constants, puts it in the
program's read-only data, and lets the borrow live for ever. That is
called **constant promotion**. But it only happens for plain values: a
*function call* inside the brackets, even a `const fn`, is not
promoted, and the compiler complains that the borrowed temporary is
dropped at the end of the statement. The macro keeps the table short
to write and still made of plain values.

The bass works the same way, with `Tone`s instead of `Voices`: the
root, the fifth, the next chord's root played early (an
"anticipation"), or the note a semitone away from it, leading in.

## Styles

`style.rs`

A style puts it together:

```rust
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
```

The name, the genre, the mode the chords are in, the chart, the
comping, the bass, the usual tempo, the rhythm that goes with it, and
a sentence on what to listen for. `style` is a `const fn` that fills in
4/4; the few styles in another time signature add `.in_time(THREE_FOUR)`,
another `const fn`, which uses *struct update syntax*:

```rust
const fn in_time(self, time: TimeSignature) -> Style {
    Style { time, ..self }
}
```

`..self` means "every other field as in `self`".

The table itself is a `static`:

```rust
pub static STYLES: &[Style] = &[ /* ... */ ];
```

A `const` is pasted into every place that uses it, like a macro; a
`static` is one value at one address in memory for the whole program.
For a big table that is used in many places, `static` keeps one copy.

`Style::render` turns a style into notes in any key: the chart becomes
spans, each span's numeral becomes a chord in the key, the chords are
voiced smoothly one after another (see the [previous
chapter](harmony-keys.md#playing-a-chord-voice-leading)), and the
comping and the bass play them.

## Rhythms, one line per drum

`rhythm.rs`

Drummers think of a beat as a grid: one row per drum, one column per
step. The library writes exactly that, as text:

```rust
rhythm(
    "Pop beat",
    Pop,
    110.0,
    &[
        (Kick, "X... .... X.x. ...."),
        (Snare, ".... X... .... X..."),
        (HiHat, "x.x. x.x. x.x. x.x."),
    ],
    "Snare roll",
    "The backbeat: kick on 1 and 3, snare on 2 and 4, hi-hat on every eighth note.",
),
```

`X` is a loud hit, `x` a normal one, `o` a soft "ghost note" and `.`
silence. `h` and `l` play the drum five semitones higher or lower, for
the open and muted tones of a conga or two toms. Spaces only make the
beats easy to see, and `|` separates bars, for rhythms longer than one.
A rhythm can name a *fill*, a one-bar break that the demos put at the
end of a section.

When a rhythm goes into a pattern, it is laid out bar by bar in the
project's time signature. A 4/4 beat in a 3/4 pattern loses its last
beat in each bar, instead of drifting across the bar lines:

```rust
for start in (0..length).step_by(bar) {
    let steps = &bars[(start / bar) % bars.len()];
    for (step, hit) in steps.iter().enumerate().take(bar) {
        // ...
    }
}
```

`step_by(bar)` gives the start of every bar, and `take(bar)` stops a
long bar at the bar line.

### Nine new drums

The rhythms needed more than the kick, snare, clap and hi-hat of step
2: an open hi-hat, a rim click, two toms, a ride and a crash cymbal, a
cowbell, a shaker and a conga. They are synthesized like the first
four, from sine waves, noise and filters.

The cymbals use a trick from the analog drum machines of the early
1980s. Six square waves at frequencies that are not multiples of each
other are added together:

```rust
const METAL: [f32; 6] = [205.3, 304.4, 369.6, 522.7, 540.0, 800.0];

fn metal(t: f32, ratio: f32) -> f32 {
    METAL.iter().map(|&f| square(t, f * ratio)).sum::<f32>() / METAL.len() as f32
}
```

Notes whose frequencies are simple multiples of each other blend into
a chord. These do not, so together they clash and shimmer like metal.
A high-pass filter keeps the bright part, and the envelope decides
whether it is a closed hat, an open hat, a ride or a crash. The cowbell
is just two of the same square waves, a little more than a fifth
apart.

## Songs

`song.rs`

The demos needed songs everybody knows, and they had to be free to
use. A song is free when it is in the **public domain**: folk songs
nobody owns, and music by composers who died long ago. *Ode to Joy*
(Beethoven, 1824), *Für Elise*, *Canon in D*, *Greensleeves*, *Frère
Jacques*, *Korobeiniki* and the others all are. Some famous old tunes
are not, or not everywhere, because a newer arrangement or the words
are still protected; those were left out.

A melody is written note by note:

```text
"F#5:4 F#5 G5 A5 | A5 G5 F#5 E5 | D5 D5 E5 F#5 | F#5:6 E5:2 E5:8"
```

`F#5` is the F sharp in the fifth octave, and `:4` its length in steps
(a quarter note). A note without a length is as long as the one before,
which keeps the line short. `-` is a rest, `_` ties the note before it
over a bar line, and `|` is a bar line, which changes nothing but lets
a test check that every bar adds up to the time signature.

A song has **parts** (a verse, a chorus, an "A" and a "B" section),
each with its melody and its chord chart, and a **form**: the order the
parts are played in. Each song is dressed in a style by its comping,
bass and rhythm: *Ode to Joy* as a house track, *Twinkle, Twinkle* as
lo-fi, *When the Saints* as Dixieland, *Drunken Sailor* as folk punk.

### Rounds

*Frère Jacques* is a **round**: the same tune sung by several voices,
each starting two bars after the one before. Writing out every voice
would be long and easy to get wrong, so a part can say where its
voices start, in bars:

```rust
Part::new("Round", FRERE_JACQUES_TUNE, FRERE_JACQUES_CHORDS).with_voices(&[0, 2, 4, 6]),
Part::new("Round again", FRERE_JACQUES_TUNE, FRERE_JACQUES_CHORDS)
    .with_voices(&[0, -6, 2, -4, 4, -2, 6]),
```

In "Round again", the voices that started in the part before carry on:
a voice at -6 started six bars earlier, so in this part you hear its
last two bars. Notes that fall outside the part are dropped:

```rust
for &voice in part.voices {
    let offset = voice as isize * bar as isize;
    for note in &melody.notes {
        let start = note.start as isize + offset;
        if (0..length as isize).contains(&start) {
            notes.push(NoteEvent {
                start: start as usize,
                ..*note
            });
        }
    }
}
```

The offsets can be negative, so the sums are done in `isize`, a signed
integer the size of `usize`. Only starts inside the part,
`0..length`, are kept, so the conversion back to `usize` cannot go
wrong. `..*note` is struct update syntax again: a copy of the note
with a new start. *Canon in D* uses the same trick for its three
violins.

## Is it right?

Data can be wrong as easily as code, and a wrong note in a table does
not crash anything. So the tests read the whole library:

- every chart, rhythm line and melody parses,
- every bar of every melody lasts exactly one bar of its time
  signature, and every chord chart divides its bars evenly,
- every style's rhythm and every song's parts exist,
- every style name is unique, and every genre has styles,
- the first notes of each song are the ones people know,
- at least 80% of each melody's notes are in the song's key. The rest
  are passing notes and chromatic turns, like the D♯ of *Für Elise*;
  a melody with fewer notes in its key is almost certainly written in
  the wrong key.

A first version of that last test checked that every note was "within
a semitone of the scale". It always passed: in a scale of seven notes,
*every* note is within a semitone of the scale. A test that cannot fail
tests nothing, so it was replaced. When you write a test, try breaking
the code on purpose once, and check that the test notices.
