# The Misa Criolla: a Mass to the dances of Argentina

Files: [`crates/lineup-core/src/library/song/misa_criolla.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/misa_criolla.rs),
[`crates/lineup-core/src/library/song.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song.rs),
[`crates/lineup-core/src/library/demo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/demo.rs),
[`crates/lineup-core/src/library/rhythm.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/rhythm.rs)
and [`crates/lineup-core/src/library/style.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/style.rs)

Ariel Ramírez wrote the **Misa Criolla** in 1963 and 1964: a Mass, the
five sung parts of the Catholic service, in Spanish, for two soloists,
a choir and the instruments of Argentine folk music. Each part is set
to a dance or a song of the country. The demos have all five, complete,
every voice and instrument on a channel of its own (PRs #154, #156 and
#159):

| Demo | Its rhythm | Meter and key |
|---|---|---|
| **Misa Criolla: Kyrie** | vidala-baguala | 3/4, A minor, the middle in A major |
| **Misa Criolla: Gloria** | carnavalito, with a yaraví in the middle | 2/4, A minor; the introduction and the yaraví in 3/4 |
| **Misa Criolla: Credo** | chacarera trunca | 3/4 after a one-beat pickup; A minor, then A major, then E major |
| **Misa Criolla: Sanctus** | carnaval cochabambino | 3/4, A major, three bars of 4/4 |
| **Misa Criolla: Agnus Dei** | estilo pampeano | 3/4, A minor, no drums |

The words are not in the demos: the voices sing without them.

## Trying it

1. Open **File > Demos**, type `Misa` in the search box and pick
   **Misa Criolla: Kyrie**. Press Play.
2. Open the **mixer**. Lead, Soloist 2, Choir and Low tom each have a
   strip. Lead is the first soloist wherever the soloists sing, and the
   choir's top voice where they do not; Choir has the other voices.
   Mute Choir and listen to the two soloists alone: mostly a third
   apart, the second under the first.
3. Open **Misa Criolla: Gloria** and watch the time signature in the
   ruler: 3/4 for the introduction, 2/4 for the dance, 3/4 again for
   the slow yaraví from bar 73, and 2/4 to the end.
4. Open **Misa Criolla: Credo** and look at the key in the harmony bar:
   A minor, then A major from bar 78 and E major from bar 122.
5. In the **Library**, the new styles are lit for their demos:
   **Carnavalito (2/4)**, **Yaraví**, **Chacarera trunca**, **Carnaval
   cochabambino** and **Estilo**. Try one on a song of your own.

## The music

### Kyrie: a bass that climbs by semitones

The Kyrie is a **vidala-baguala**, the slow song of the Andean
north-west of Argentina, sung to a hand drum. Here the bombo plays the
first and third beats of every 3/4 bar and nothing else.

The choir sings alone first, softly, six bars that are a small lesson
in harmony. Follow the bass on the piano roll of the Choir channel: A,
B, C, C sharp, D, D sharp, E. It climbs from the tonic to the
dominant, by semitones from C. Over it:

| Bar | Bass | Chord | What it does |
|---|---|---|---|
| 3 | A, then B | A minor, then E major over B | i, and a passing V6/4 |
| 4 | C, then C sharp | A minor over C, then A7 over C sharp | i6, then V7 of D minor: a **secondary dominant** |
| 5 | D | D minor | iv, where A7 was leading |
| 6 | D sharp | F, A, C and D sharp | a **German augmented sixth**: F and D sharp, an augmented sixth apart, open out to the two E's of the next bar |
| 7 | E | A minor over E | the **cadential six-four**, i6/4, which delays the dominant |
| 8 | E | E, A and D | V7 with a **suspended fourth**: A instead of G sharp |
| 9 | E | E, G sharp and D | V7: the suspension resolves, A down to G sharp |

Then the soloists sing. The middle part, the **Christe**, turns to A
major, the **tonic major**: the same tonic, with C sharp, F sharp and G
sharp instead of C, F and G. The Kyrie comes back in A minor, and its
very last chord is A major again, under a fermata: a minor piece that
ends on the major chord of its tonic ends on a **Picardy third**.

### Gloria: the carnival dance, and a lament in the middle

The **carnavalito** is the quick carnival dance of the Quebrada de
Humahuaca, in the far north of Argentina. It is in 2/4: two beats to
the bar. The bombo strikes each beat and answers it with two quick
sixteenths, and the guitars strum **A minor and E major, a beat each**:
i and V, tonic and dominant, every bar. Later the harmony moves to C
major, the **relative major** (the major key with the same notes as A
minor), through F, G and C: IV, V and I of C, or bVI, bVII and bIII
counted from A.

The library had a carnavalito already, written as pairs of 2/4 bars in
one bar of 4/4. **Carnavalito (2/4)** keeps the bars as the Gloria
writes them, so a song in 2/4 can use it.

In the middle the tempo falls from a quarter at 88 to 40, and the meter
turns to 3/4: a **yaraví**, the slow lament of the Andes. The bass holds
one A for bars on end, a **pedal**, the bombo plays soft eighth notes,
and the voices sing over it in A minor, mostly on the notes of the
**minor pentatonic** scale (A, C, D, E and G).

### Credo: a chacarera cut short

The **chacarera** is the dance of Santiago del Estero. The Credo is a
**chacarera trunca**, "cut off", in a quick 3/4, a quarter at 152.
Listen to where the sung phrases stop: in the second bar of each pair,
the voices let go before the third beat, while the bombo keeps going.
The bombo's pattern is two bars long: its head on the first two beats
of one bar and on the third of the next, its rim answering with four
sixteenths. The new rhythm **Chacarera trunca** is that pattern.

The introduction swings between A minor and **D major**. D major's F
sharp is not in A minor: A minor with F sharp is **A Dorian**, the
minor mode with a raised sixth, the mode of much music of the Andes.

The voices start in A minor, turn to **A major** at bar 78, and to **E
major** at bar 122. The end slows down from 152 to a quarter at 46,
swinging between E major and A major over an E in the bass, and closes
on E major. Heard from A, where the Credo began, E is the dominant: the
movement ends open.

### Sanctus: the carnival of Cochabamba

The **carnaval cochabambino** comes from Cochabamba, in Bolivia. The
Sanctus is in A major and a quick 3/4. The guitars and the harpsichord
open on the A major chord, a flute plays above them, and the drums come
in one by one: a wood block (played on the rim here), a snare with a
burst of four loud sixteenths on the second beat, and two drums
answering each other. Three bars in **4/4** broaden the music into a
held chord; then the voices sing alone, softly and then loudly,
slowing down, and the drums come back for the last bars.

### Agnus Dei: the gaucho's guitar

The **estilo** is the slow song of the Pampas, sung to the guitar, with
no drums. The guitar opens on A minor, then plays three chords a beat
each: **D minor, A minor and E seventh**, iv, i and V7. That is the
whole of minor-key harmony in one bar: the **subdominant**, the
**tonic** and the **dominant**. A second guitar plays a falling line
above it, with turns, and the bass walks down, A, D, C, B.

Between the verses the guitar runs in sixteenths, in free time:
listen to how each bar speeds up into its middle and slows down at its
end. The demo's tempo marks do that, two glides in every bar, as the
sequence it comes from plays it. The last bars are very soft, and the
choir closes from D minor to A minor: iv to i, a **plagal close**, the
"amen" ending.

## How it works

### Lines: the other instruments, written out

Until now a demo part had a tune, a chord chart, sometimes a left hand
for the Keys, and rounds for extra voices. The Misa Criolla has
soloists, a choir of four voices, guitars, a harpsichord and a bass at
once. PR #154 gives a [`Part`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song.rs)
a list of **lines**: each instrument's part written out, on a channel
of its own.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line {
    pub name: &'static str,
    pub program: u8,
    pub notes: &'static str,
}

pub const fn line(name: &'static str, program: u8, notes: &'static str) -> Line {
    Line { name, program, notes }
}
```

A line has the name of its channel, the General MIDI **program** that
plays it, and its notes in the same text notation as a tune, where `&`
starts another voice on the same channel. The choir's alto, tenor and
bass are one line with two `&`. Lines with the same name, in any part,
play on the same channel.

`&'static str` is a string that lives as long as the program: text
written in the source code. `const fn` is a function Rust can run while
it compiles, so a whole song can be a constant:

```rust
Part::new("Kyrie", "-:8 !mf E4:4 | E4:6 D4:2 C4:4 | ...", "")
    .with_lines(&[
        line("Soloist 2", gm::VOICE_OOHS, "-:8 !mf C4:4 | C4:6 B3:2 A3:4 | ..."),
        line("Choir", gm::VOICE_OOHS, "!p E4:8 -:4 | ... & ... & ..."),
    ])
```

`with_lines` is a `const fn` too. It copies the part and changes one
field, with Rust's **struct update syntax**: `Part { lines, ..self }`
means "the field `lines` is this, every other field as in `self`".

When a demo is built ([`song_demo`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/demo.rs)),
each line name gets a channel, with the program of its first line, and
each part's lines are performed like the other accompaniment and follow
the score's dynamics. A part with lines is **written out**: the style's
comping and horns stay quiet in it, because the score already says what
everyone plays.

A song whose parts are all written out, with no left hand, never plays
chords on the Keys, so it gets no Keys channel at all:

```rust
let keys_play = song.parts.iter().any(|part| {
    !part.left.is_empty() || (part.lines.is_empty() && !part.chords.trim().is_empty())
});
let keys = keys_play
    .then(|| apply::synth_named(&mut project, apply::KEYS))
    .flatten();
```

`any` asks whether any part needs the Keys. `bool::then` turns `true`
into `Some(...)` of what the closure returns and `false` into `None`;
`synth_named` itself returns an `Option`, so `flatten` turns the
`Option<Option<usize>>` into an `Option<usize>`.

### From a sequence to a demo

The five movements come from a MIDI sequence of the whole Mass, a
track for each voice and instrument. A script read it, bar by bar, and
wrote the parts:

- each note placed on the nearest sixteenth, or the nearest third of
  one where the sequence plays a triplet, since triplets are written
  exactly in a sequence;
- the tempo map read as the score's marks: a deep dip that comes back
  is a **fermata**, a run of small changes a **glide** (*rit.* or
  *accel.*), and anything else a new tempo;
- the velocities read as dynamics, from *pp* to *ff*;
- a **tremolo**, the same chord struck faster than a sixteenth, written
  as one held chord, because the app's grid stops at the sixteenth.

The words were left out on purpose.

### Two things a demo must keep

Two tests shaped the parts.

**A MIDI export must come back note for note.** The importer cuts a
long song into patterns at bar lines, 8 to 16 bars long, choosing a bar
line no note crosses. A note that crosses the cut is cut in two and
struck again. In the Credo the bass ties over almost every bar line, so
between bars 21 and 37 there was no clean bar line, the importer cut a
held note, and the file came back with an extra note. The fix was in
the demo: a part boundary at bar 30, where the soloists come in. A demo
strikes held notes again at its own part boundaries, so those bar lines
are always clean.

**A demo keeps a sensible tempo**: never slower than three tenths of its
starting tempo. The sequence holds the Credo's last chord at a quarter
of 20, an eighth of 152; the demo holds it at 46.

## Checking it

Every movement was played with the [playback trace](playback-trace.md)
and compared with the sequence, note by note, in steps:

```text
ferrous-lineup --trace "Misa Criolla: Credo" --out credo.tsv
```

The trace lists every note the player starts: its step, pitch,
channel and program. All five match the sequence except where the demo
differs on purpose: the tremolos are held chords, unisons sung by two
voices of the choir sound once (they are one channel), and a bass line
the sequence doubles on two tracks is played once.

## Exercises

1. In the Kyrie, mute every channel but Choir and play bars 3 to 9.
   Sing or play the bass alone, A to E. Then play the chords of the
   table above on the chord sheet, one per bar, and listen to the
   German sixth open out to the E.
2. Apply **Estilo** to a song of your own in a minor key, in 3/4. Then
   write a bar of three chords a beat each, iv, i and V7, as in the
   Agnus Dei.
3. Find the bars of the Credo where the voices stop before the third
   beat. How many bars long is each phrase?
4. Rust: write a `const fn` that takes a `Line` and returns it with
   another program, using struct update syntax.
