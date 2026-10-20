# The concert hall: Boléro, La Mer, The Rite of Spring and Petrushka

Files: [`crates/lineup-core/src/library/song/classical.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/classical.rs),
[`crates/lineup-core/src/library/song.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song.rs),
[`crates/lineup-core/src/general_midi.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/general_midi.rs)
and [`crates/lineup-core/tests/key_finding.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/tests/key_finding.rs)

Until now the classical demos were piano pieces, or a tune with a
band. This round adds four pieces for orchestra from the early
twentieth century. They are written for many instruments, and their
meters and harmony are unlike the songs elsewhere in the library:

- **Boléro** (Maurice Ravel, 1928), complete: 340 bars, one long
  crescendo.
- **La Mer**, its opening (Claude Debussy, 1905): bars 1 to 11, dawn
  on the sea.
- Two dances from **The Rite of Spring** (Igor Stravinsky, 1913):
  **The Augurs of Spring** and the **Ritual of Abduction**.
- **Petrushka** (Igor Stravinsky, 1911, revised 1947), all four
  tableaux, as five songs: **The Shrovetide Fair**, **Russian Dance**,
  **Petrushka's Room**, **The Moor's Room** and **The Shrovetide Fair
  in the Evening**.

## Trying it

Open **File > Demos > Songs > Classical and film > Boléro** and press
Play. The snare drum plays the same two bars, in 3/4, from the
beginning to the end. Count it: **1** (and a triplet) **2** (and a
triplet) **3 and**. Watch the mixer while it plays: one instrument
after another takes the tune, and the snare is joined by more and more
of the orchestra.

Then open **The Rite of Spring, Ritual of Abduction** and watch the
time signature in the ruler. It changes almost every bar: 9/8, 6/8,
7/8, 3/4, and near the end 5/8, 3/8 and 2/8, a bar of a single stroke.

Last, open **Petrushka, Petrushka's Room** and listen for the two
clarinets after the drum roll and the kick into the room. One plays
an arpeggio of C major and the other one of F sharp major, at the same
time.

## The music

### Boléro: one tune, louder and louder

Boléro does not develop its tunes. Two tunes of sixteen bars each come
round again and again over the same snare drum rhythm and the same C
and G plucked in the bass. Everything
else is **orchestration**, the choice of instruments. The flute, the
clarinet, the bassoon high in its range, the little E-flat clarinet,
the oboe d'amore, a muted trumpet, the saxophones: each plays a tune,
and each time a little more of the orchestra joins the rhythm.

The first tune is in **C major**. The second starts on B flat, C
major's seventh flattened, so it is in **C Mixolydian**, and it
borrows D flat, E flat and A flat, the flattened second, third and
sixth of **C Phrygian**. The demo marks its parts C Mixolydian. The
app's key finder hears its first statements as C minor instead,
because E flat and A flat are C minor's notes too.

At one point Ravel mixes colours to make new ones. A horn plays the
tune, the celesta plays it one and two octaves higher, and two
piccolos play it in **G** and in **E**, an octave and a fifth and two
octaves and a third above the horn: the third and fifth notes of the
horn's overtone series. The
piccolos then blend into the horn like the stops of an organ, and you
hear one new instrument instead of three tunes in three keys. Only at the very end does the music leave C: eight bars in **E
major**, then back to C for the crash.

### La Mer: a pedal and a pentatonic scale

The opening of La Mer has no chords in the usual sense. The double
basses and the timpani hold a low **B**, a **pedal**, and above it
the harps and cellos rock between F sharp and G sharp, notes of a
pentatonic scale on B. The key signature is B minor's, but the G
sharp makes it **B Dorian**: the minor mode with a raised sixth. The
chord sheet stays empty, because no chord symbol says what is
happening.

### The Rite of Spring: two chords at once, and accents out of place

**The Augurs of Spring** hammers one chord in even eighth notes, and
that chord is two chords at once, a **polychord**: F flat major (E, G
sharp, B) in the low strings under E flat seventh (E flat, G, B flat, D
flat) above it. The meter stays 2/4, but eight horns strike the chord
on accents that fall where no beat leads the ear to expect them, so
the steady pulse lurches.

The **Ritual of Abduction** changes meter all the time, and it is in
no key at all. About three of its notes in four come from one
**octatonic scale**, a scale of eight notes that goes half step, whole
step, half step, whole step: C, D flat, E flat, E, F sharp, G, A, B
flat.

### Petrushka: two triads a tritone apart

Petrushka is a puppet, and Stravinsky gave him a chord of his own,
the **Petrushka chord**: C major (C, E, G) and F sharp major (F sharp,
A sharp, C sharp) at the same time. The two triads are a **tritone**
apart, the interval of three whole steps that splits the octave in
half, and they share no note. All six notes still lie in one
octatonic scale, the same one as in the Rite: C, D flat (C sharp), E
flat, E, F sharp, G, A, B flat (A sharp). No major or minor key holds
them.

The rest of Petrushka is the fair around his booth, and it is full of
**Russian folk tunes**. These are **modal**: their notes are a major
or minor key's, but they turn around another note. The opening flute
tune is in **D Dorian** (D minor with B natural). The Russian Dance
moves in **parallel chords**, every voice of the chord moving with the
tune, over a bass on G, in **G Mixolydian** (G major with F natural).
The coachmen of the last tableau stamp in **E Mixolydian** over a
**drone** on E.

### Modes over drones, and the key finder

The app's key finder (the **Find the key** option under the harmony
keys) looks at which notes are played and how long, and picks the
major or minor key that fits them best. Modal tunes are hard for it: E Mixolydian has the same notes as
A major, and the finder usually picks A major. The written E major
tonic only shows in the drone on E. In the same way it hears the
merchant's white-note **D Dorian** in Petrushka as C major, and La
Mer's **B Dorian** as B major. This is why the test that measures the
finder on the library allows for a few more wrong bars each time music
like this comes in.

## The code

### An orchestra on fifteen channels

A demo's melody, its left hand and its drums each get channels of
their own. A part can say which **General MIDI** instruments play its
tune with `played_by`, and how loud with `marked`:

```rust
Part::new(
    "Petrushka's chord",
    "-:6 (3 -:1 C4 E4 | C5:8 | ...",
    "",
)
.with_left("-:6 (3 -:1 Bb3 C#4 | Bb4:8 | ...")
.in_key(C, Chromatic)
.played_by(&[(gm::CLARINET, 0)])
.marked(Dynamic::Piano),
```

Here two clarinets play the Petrushka chord. One instrument plays the
C major arpeggio, and the left hand, on the Keys channel, plays the F
sharp major one. `played_by` takes a list of `(program, semitones)`
pairs: each instrument plays the tune, shifted by that many semitones,
so a list with several entries doubles the tune at the octave. Every
new instrument gets a channel of its own in the demo, and a demo can
have at most fifteen pitched channels. Two programs were added to
`general_midi.rs` for Boléro: `MUTED_TRUMPET` and `SOPRANO_SAX`.

### A key no scale holds

`in_key(C, Chromatic)` marks a part as having no key: all twelve
notes are its scale. The Ritual of Abduction, the bars on Petrushka's
chord and the Moor's fight are marked like that. The key-finding test
skips chromatic bars, so they are not counted as wrong.

### Meters that change every bar

`in_meters` gives a part one meter for each bar:

```rust
.in_meters(&[
    TWO_EIGHT,
    SIX_EIGHT,
    THREE_FOUR,
    THREE_FOUR,
    SIX_EIGHT,
    // ...
])
```

The demo cuts the part into runs of bars in one meter, each with a
time marker on the ruler (see [Odd meters](odd-meters.md)). These
songs needed meters the library did not have yet, so `song.rs` gained
constants for 2/8, 3/8, 4/8, 5/8, 7/8, 1/4, 2/2, 3/2 and 13/16.

The Rite also found a bug. A written-out left hand can have several
voices, joined with `&`. When a part with meters that change was cut
into runs of bars, `bars_of` counted the bar lines of all the voices
as one, so the second voice's bars were taken as more bars of the
first. Now it cuts each voice on its own and joins the pieces again
with `&`. A unit test in `song.rs` covers it.

### A test for polytonal songs

The key-finding test measures how many bars of the demos the finder
hears in their written key. The Augurs of Spring would make that figure
meaningless: it is in two keys at once, so whatever key the finder
names, half the music disagrees. Such songs go in a list of their own:

```rust
const POLYTONAL: &[&str] = &["The Rite of Spring, The Augurs of Spring"];
```

The main test leaves them out, and a second test checks them on their
own: the written key must be among the keys heard, but not the only
one:

```rust
assert!(result.right_bars > 0, ...);
assert!(
    result.right_bars < result.bars,
    "{name} is heard in one key: drop it from POLYTONAL"
);
```

The second assert keeps the list honest. If a change to the finder
ever made it hear the Augurs in one key, the song would no longer
belong in the list.

### How the pieces were written down

Reading handwritten orchestral scores note by note is slow, so these
demos come from machine-readable versions:

- **Boléro** comes from a LilyPond engraving of the full score (the
  `nsceaux/Ravel_Bolero` project, public domain), compiled to MIDI with
  LilyPond and checked against Ravel's autograph.
- **La Mer**'s opening was read from Debussy's autograph.
- **The Rite** and **Petrushka** come from MIDI sequences of the scores.
  The tempos, fermatas, dynamics and meters were checked against the
  1921 full score of the Rite. For Petrushka, every meter of the
  sequence is kept, and each part's comment gives the bars it covers.

The scripts that read those files keep the tune in front, follow it
from instrument to instrument with `played_by`, and write everything
else that matters on the Keys channel. Lines doubled at the unison or
the octave are written once. Grace notes, trills, tremolos and
glissandos are simplified, and each song's `// Source:` comment says
how.

### Copyright

The Rite of Spring and Stravinsky's 1947 revision of Petrushka are
still under copyright in many countries, as Stravinsky died in 1971.
They are included at the project owner's request, and a comment above
each of these songs says so. Ravel's Boléro and Debussy's La Mer are
older, and their composers died long enough ago for them to be in the
public domain in most countries.

## Exercises

1. Play **Boléro** and write down, for each statement of the tune, which
   instrument plays it. How many statements are there before the music
   leaves C major?
2. Open **The Rite of Spring, The Augurs of Spring** and, on the
   keyboard, play E, G sharp, B with the left hand and E flat, G, B
   flat, D flat with the right. Which notes of the two chords lie a
   half step apart?
3. Play C, E, G and F sharp, A sharp, C sharp together: the Petrushka
   chord. Now play the octatonic scale C, D flat, E flat, E, F sharp,
   G, A, B flat. Which of the scale's notes are not in the chord?
4. Open **Petrushka, The Shrovetide Fair in the Evening**, turn on
   **Find the key** and play the Coachmen's Dance. Which key does the
   finder show, and why is E Mixolydian heard that way?
5. Print the key-finding report with
   `cargo test -p lineup-core --test key_finding -- --ignored --nocapture`
   and find Petrushka's last tableau in it. Which keys does the score
   give, and which keys does the finder hear in their place?
