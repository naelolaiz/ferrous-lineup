# Tangos and folk songs from MIDI files

Files: [`crates/lineup-core/src/library/song/tango_played.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/tango_played.rs),
[`crates/lineup-core/src/library/song/tango_arranged.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/tango_arranged.rs),
[`crates/lineup-core/src/library/song/folklore_arranged.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/folklore_arranged.rs)
and [`crates/lineup-core/src/library/song.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song.rs)

A pile of MIDI files of tangos and Argentine folk songs became 28 new
demos, each complete as its file plays it, note for note (PRs #158 and
#164, checked further in #174). The files came in three kinds, and each kind needed its own way
of finding the beat:

| Module | Demos | How the file was made |
|---|---|---|
| `tango_arranged` | Mi noche triste, Sentimiento gaucho, Zorro gris | a band arrangement, sequenced to a click |
| `folklore_arranged` | Cinco siglos igual, La cultura es la sonrisa, El Noi de la Mare, Zamba de mi esperanza, Zamba por vos, Que nadie sepa mi sufrir | a band arrangement, sequenced to a click |
| `folklore_arranged` | Balderrama, Acuarela del río | a band arrangement to a steady beat that does not fall on the file's own grid |
| `tango_played` | Uno, Sur, Soledad, Nostalgias, Pasional, Patotero sentimental, Qué falta que me hacés, Qué me van a hablar de amor, Marionetas, Tiempos viejos, Yira yira, Los mareados, Mano a mano, La casita de mis viejos, Lluvia de estrellas, La pulpera de Santa Lucía, Malena | a pianist playing freely, without a click |

The tune, the line the singer sings, is always on the **Lead** channel.
In the band arrangements every other instrument has a channel of its
own, with its General MIDI program, and the bombo or the percussion
plays on the drums. On the piano, everything under the tune is the left
hand, on the Keys.

The chord sheets were read from the notes by a script. For eleven of
the piano songs the reading was then checked against a lead sheet, and
those songs are cut into their verses and refrains, each in its own key
(PR #174). The other six, and the band arrangements, are still a reading
of the notes alone, so treat their chords as a draft.

## Trying it

1. Open **File > Demos**, type `Uno` and pick **Uno**. Press Play and
   watch the tempo in the transport: it changes almost every bar,
   between 78 and 98 to the quarter, because the pianist slows down
   and pushes on with the words. That is **rubato**, "robbed" time.
2. Open **Malena** and the **mixer**. Lead is the pianist's tune;
   Bandoneón, Guitar, Contrabass and Strings are the arrangement's
   other instruments, each on a strip of its own.
3. Open **Balderrama** and look at the key in the harmony bar: **E
   minor**. Then open the piano roll of the Lead channel at bars 21
   and 22 and find the C sharps.
4. Open **Acuarela del río**. Solo **Double bass**, then **Keys**,
   then the drums, and listen to where each one strikes in the bar.
5. Open **Zamba de mi esperanza** and mute **Lead**. The **Second
   voice** channel sings the lower note wherever the tune is sung in
   two.
6. Open **Yira, yira** and watch the parts go by in the playlist:
   Verse 1a, Verse 1b, Refrain 1, and the song again. The harmony bar
   shows **B minor** in the verses and **D major** in the refrains.

## The music

### Tango in 2/4, written two bars to one

Most of these tangos are written in **2/4**: two beats to the bar. The
app's Tango style is in 4/4, so each pair of 2/4 bars is one bar of
4/4 here. Nothing moves: the beats are where they were, and every
other bar line of the sheet music falls in the middle of a bar of the
demo.

### The vals criollo: 3/8 becomes 3/4

**La pulpera de Santa Lucía** is a **vals criollo**, the Argentine
waltz, quick and light. Its file is in **3/8**: three eighths to the
bar. Here its eighths are the quarters of a bar of 3/4. The music is
the same, read with a longer note as the beat, so the tempo is about
196 to the quarter: three quick beats a bar, felt as one.

### Six eighths, two ways: the zamba and the polca

A bar of six eighths can be grouped in two ways:

```text
6/8:  1 2 3 4 5 6     two beats:   ONE . . TWO . .
3/4:  1 2 3 4 5 6     three beats: ONE . TWO . THREE .
```

The music of Argentina and Paraguay plays with both at once, or one
after the other. That is called **sesquialtera**, "one and a half":
three beats in the time of two.

The **zamba** is the slow dance of the handkerchiefs from the north of
Argentina, written in 6/8. **Zamba de mi esperanza**'s file is written
in 3/4. The demo writes it in 6/8, the zamba's own meter: the bars have
the same six eighths, and only the grouping on paper changes.

**Acuarela del río** is a **polca**, the 6/8 dance of Paraguay and the
Litoral. Listen to its band, bar by bar:

| Instrument | Strikes on eighth | Grouping |
|---|---|---|
| percussion, accented | 1 and 4 | 6/8: two beats |
| double bass | 1, 3 and 5 | 3/4: three beats |
| piano | 2 and 5 | the offbeats that drive the dance |

The bass walks in 3/4 while the drums count 6/8, in the same bar.

### Balderrama: a chromatic note is not a new mode

**Balderrama** is in E minor, the natural minor: E F# G A B **C** D.
Yet its tune sings C sharp 24 times and C natural only 8. Is it in **E
Dorian**, E F# G A B **C#** D, the minor scale with a raised sixth?

Look at where the C sharps are. All of them come in one turn of the
tune, in bars 21 and 22, which comes back five times later in the
song:

```text
G A B | D C# C# B  C# B C# E
```

The tune falls from D to B through C sharp, and then circles round B.
Under it the band plays a **G major** chord: G, B and D. C sharp is not
in that chord, and not in E minor: it is a **chromatic passing note**,
a note from outside the scale that fills the gap between two notes of
it. The chords themselves keep C natural: where A is the root, the
band plays A minor, the **iv** of E minor, never A major.

A mode is the scale the harmony lives in, not the notes a tune touches
on its way. A piece in Dorian would have the raised sixth in its chords
too, as the introduction of the Misa Criolla's Credo does with its D
major chord in A minor ([The Misa Criolla](misa-criolla.md)).
Balderrama keeps its C natural there, so it is in E minor, with a
little chromatic colour in one phrase.

### Acuarela del río: one tonic, two modes

In **Acuarela del río** the flute plays the tune with C, F and G, and
the clarinet with C sharp, F sharp and G sharp. The flute's sections
are in **A minor** and the clarinet's in **A major**, twice over: the
same tonic, two modes. Keys that share a tonic are **parallel** keys. Each part of the demo carries
its own key with `in_key`, so the harmony bar follows the change.

## How it works

The demos were written out by a script in Python, with the
[mido](https://mido.readthedocs.io/) library to read MIDI files. The
script is not part of the app: what it wrote is, as ordinary Rust
constants.

### What a demo looks like

```rust
pub(super) const BALDERRAMA: Song = Song {
    name: "Balderrama",
    origin: "Gustavo Leguizamón, words by Manuel J. Castilla",
    feel: "Zamba",
    genre: Genre::Folklore,
    tonic: E,
    mode: NaturalMinor,
    time: SIX_EIGHT,
    tempo: 98.0,
    // ...
    band: Band::new(gm::TANGO_ACCORDION, gm::NYLON_GUITAR, gm::ACOUSTIC_BASS),
    style: "Zamba",
    parts: &[
        Part::new("Part 1", "-:12 | - | - | ...", "Em | Em | ...")
            .with_lines(&[
                line("Shakuhachi", 77, "-:12 | - | - | -:8 E5:1 F#5 G5 A5 | ..."),
                line("Double bass", gm::ACOUSTIC_BASS, "-:4 E1+E2:2 - E1+E2:4 | ..."),
                // ...
            ])
            .with_drums(&[(Drum::Kick, "..."), (Drum::Rim, "..."), (Drum::LowTom, "...")]),
        // ...
    ],
    // ...
};
```

`Band::new` names the programs of the Lead, the Keys and the Bass. It
is a `const fn`, so it runs while the program compiles and the band
can sit inside a constant. `gm::TANGO_ACCORDION` is General MIDI
program 23, the closest the standard has to a bandoneón; a program
with no constant in [`general_midi.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/general_midi.rs),
such as 77 for the shakuhachi, is written as a plain number.

A part that changes key says so:

```rust
Part::new("Part 2", "...", "...").in_key(A, HarmonicMinor)
```

and a part where the tempo moves carries tempo marks, each at a step
(a sixteenth) from the start of the part:

```rust
.with_tempo(&[tempo(0, 97), tempo(48, 98)])
```

`tempo(at, bpm)` is a `const fn` too, returning a `TempoMark`; `&[...]`
is a borrowed slice, a view of an array that lives in the constant.

### Finding the beat of a free performance

A file played to a click has its bar lines in it: the sequencer's grid.
A pianist playing freely has none, and the file's grid means nothing.
The script finds the beat the way a listener would, after Dan Ellis's
"Beat tracking by dynamic programming" (2007):

1. An **onset envelope**: for every hundredth of a second, how much
   new sound starts there (louder and lower notes count more).
2. The steadiest **period**: the time shift at which the envelope looks
   most like itself (its **autocorrelation**), near a likely tempo.
3. The **beat path**: the chain of beats that lands on the most onsets
   while keeping close to that period from beat to beat, found by
   dynamic programming, the way a satnav finds the cheapest route.

Each bar then gets the tempo it was played at. A new tempo mark is
written wherever the tempo moves by half a beat a minute or more, so
**Uno** has 61 marks and **La pulpera de Santa Lucía** 84.

Where a file's tempo map already follows the player, as in most of
the piano files, its own beats are used.

### A steady beat off the grid

Balderrama and Acuarela del río looked like free performances at
first: their notes fall nowhere near the file's grid, and the beat
tracker gave Balderrama a tempo that jumped from 98 to 130 halfway
through. But the bombo told another story. Its hits came every 293 or
294 ticks, from the first bar to the last. The files were sequenced to
a steady click, 98 and 130 to the quarter, and saved at a tempo of 120,
so the click's beats are a fraction of a tick long and drift across
the file's own bars.

To find that beat exactly, number the hits by the beat they fall on,
`k = 0, 1, 2, ...`, and fit a straight line through them, tick against
`k`. The **least squares** line is the one with the smallest sum of
squared misses, and its slope, the length of the beat, has a formula:

```text
period = Σ (k − k̄)(t − t̄) / Σ (k − k̄)²
```

where `k̄` and `t̄` are the averages. For Balderrama the eighth is
293.966 ticks long, and the hits miss the line by 3.5 ticks typically,
under four thousandths of a second: a machine, not a player.

The line gives the beat but not the bar: which of the six eighths is
the first? The answer came from the choir. Of its long held notes,
208 of 242 start on one and the same eighth of the six, and harmony
changes on the downbeat. That eighth is the first of the bar. The tune then starts
two eighths before a bar line, with an **anacrusis**, a pickup, as a
zamba often does.

### The tune and the key

The **tune** is the top line: of the notes sounding at each moment on
the melody's track, the highest (a **skyline**). Notes under it that
the tune doubles, like the second voice of Zamba de mi esperanza or the
clarinet under Acuarela del río's flute, go on a line of their own.

The **key** comes from how long each of the twelve pitch classes
sounds, compared with the Krumhansl–Kessler **key profiles**: how well
each note fits each key, measured by listeners in the 1980s. The key
whose profile correlates best wins. Where it chose wrong, the key was
set by hand, and where a part changes key it got its own `in_key`.

A test keeps that honest:

```rust
#[test]
fn melodies_are_mostly_in_their_key() {
    for song in SONGS {
        let mut total = 0;
        let mut inside = 0;
        for part in song.parts {
            let scale = song.part_scale(part);
            // ...
            for note in part.melody().notes {
                total += 1;
                if scale.contains(note.pitch.pitch_class()) /* ... */ {
                    inside += 1;
                }
            }
        }
        assert!(inside * 100 >= total * 80, /* ... */);
    }
}
```

At least four notes in five of every tune must be in the key of their
part. `inside * 100 >= total * 80` compares whole numbers, so there is
no rounding to worry about. Balderrama would fail it in E harmonic
minor, the app's usual minor (228 of 288 notes), because its tune sings
D natural, the seventh of the natural minor, far more often than D
sharp. In E natural minor it passes.

### The chords

For every bar, half bar or beat, the script scores each common chord
(major, minor, sevenths, diminished and a few more, on all twelve
roots): the notes inside the chord count for it, the notes outside
against it, and the bass note on the chord's root counts extra. Chords
of the key get a small bonus, so a passing note in a performance does
not turn A minor into something exotic. A bar is split only where the
split scores clearly better.

That is a first reading. Notes alone name every colour as a chord of
its own: a sixth the pianist adds turns Bm into Bm6, a note of the tune
on its way adds a flat ninth to F#7, and a passing note can make a chord
that is not there at all.

### Checking the chords against a lead sheet

A **lead sheet** is the tune with the chord names over it, the page a
singer and a band play from. Its chords are what the song is meant to
have, but its bars do not line up with a free performance: the pianist
adds an introduction, plays a strain twice, stretches a bar. So the
sheet's chords were not laid over the bars one by one. Instead the
sheet gives the reader its **vocabulary**: only the chords the sheet
uses may be chosen.

The sheet is often in another key than the pianist. Uno's sheet is in
G and the pianist plays in C, five semitones higher, so every chord of
the sheet moves up a fourth before it is used: G becomes C, D7 becomes
G7, Em becomes Am. Moving all the chords by the same interval is
**transposing**. It keeps every chord's job in the key, so the song is
the same song.

Yira, yira shows what the vocabulary does. Its sheet uses Bm, F#7, A7,
D, G, B7 and Em. Bars 38 to 40, read from the notes alone, were

```text
Bm6 F#7b9 | Bm F#7 | Bm A7
```

and limited to the sheet's chords they are

```text
Bm F#7 | Bm F#7 | Bm A7
```

The pianist's left hand colours the B minor with a G sharp, its sixth,
and the tune touches G at the end of the bar, a flat ninth over F#7.
Under those colours are the chords the song is built on, the ones the
sheet names: Bm and F#7, **i** and **V7** of B minor, the tonic and the
dominant seventh.

Five songs were also cut where their form changes: Yira, yira, Sur,
Malena, Soledad and Los mareados. Each part has its own key, so the
harmony bar shows the key the music is in, and chords of that key win
close calls. Yira, yira's verses are in B minor and its refrain in D
major, the **relative major**: the same key signature, two sharps, with
D as the tonic instead of B. Sur goes the other way, from a verse in D
major to a refrain in D minor, the **parallel minor**: the same tonic
with a flat third, sixth and seventh. A few bars were fixed by hand,
like the first four bars of Yira, yira, where the left hand comes in
late in each bar and the reading missed it.

For the songs with no lead sheet, the notes were checked one by one.
Tiempos viejos ends on D, F, A and B. The script had called that
**Bm7b5** (B D F A), and it is the same four notes, but the bass plays
D under it and it comes after A7, the dominant of D minor, so it is
**Dm6**: D minor with an added sixth. (The tune's last note, a high E,
adds a ninth on top.) The name follows the bass and what the chord
does, not just the notes.

### Spelling the notes

A MIDI file has numbers, not names. Note 70 is A sharp or B flat, and
the file does not say which. The key names the seven notes of its scale;
the other five need a rule, and the rule is the chord around the note:

- **A note of the chord is spelled as the chord has it.** In F#7 (F#,
  A#, C#, E) note 70 is **A sharp**, the chord's third. In Gm (G, B♭, D)
  it is **B flat**.
- **A chord on a note outside the key is named by where it goes.** A
  dominant seventh resolves down a fifth, so G#7 going to C#m is G
  sharp, because G# is the fifth of C#. A seventh chord that falls a
  semitone onto its target is a **tritone substitute**: Ab7 going to G
  is A flat, not G sharp. A diminished seventh leads up a semitone:
  G#dim7 goes to A.
- **A note outside the chord is spelled by where it goes.** A
  **chromatic passing note** between two notes a whole tone apart is a
  sharp on the way up and a flat on the way down: A, A♯, B and G, G♭, F.
- **No E sharp, B sharp, C flat or F flat for a passing note.** On the
  page they look like typos for F, C, B and E. The key's own names
  still can use them: in B minor the note between E and F sharp, on the
  way up, is E sharp, the raised fourth that leads up to F sharp, the
  dominant.
- **Two notes on one line of the staff get letters of their own.** E
  and E sharp sounding together would sit on the same line, so the
  sharp one becomes F.

## Checking it

Each demo was played with the [playback trace](playback-trace.md):

```text
ferrous-lineup --trace "Balderrama" --out balderrama.tsv
```

and every note of the MIDI file was looked for in the trace, at the
same pitch, within a fifth of a second. Almost all of them are there.
What is missing is what the app plays once on purpose: unisons a file
doubles on one channel (Balderrama's guitars strike many of their
notes twice at the same moment), strums faster than a sixteenth, and
drum hits that land on the same step of one lane. Acuarela del río
lost 10 notes of 2408.

## Exercises

1. Play **Uno** and write down the tempo of its first eight bars from
   the transport. Where does the pianist slow down? What happens in
   the words there?
2. Clap the two groupings of six eighths, 3+3 and 2+2+2, then both at
   once with a friend. Then play **Acuarela del río** and clap along
   with the bass, then with the drums.
3. Write the scales of E natural minor, E harmonic minor and E
   Dorian. Which note differs in each? Then play Balderrama's bars 21
   to 23 and name the chord under the C sharps.
4. Pick one of the six piano songs with no lead sheet yet (Patotero
   sentimental, Qué falta que me hacés, Qué me van a hablar de amor,
   Tiempos viejos, La casita de mis viejos, Lluvia de estrellas), find
   a lead sheet and check one part against it. Which chords did the
   notes alone get wrong?
5. In C major, name the black key between F and G: over a D7 chord; in
   the line G, ?, F; and over an Ab7 going to G. Then check your
   answers against the rules above.
6. Rust: `in_key` and `with_tempo` are `const fn` methods that take
   `self` by value and return a new `Part`. Why does that let them be
   chained inside a constant, and why would `&mut self` not work
   there?
