# From a MIDI performance or a score to a demo

Files: [`crates/lineup-core/src/library/song/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates/lineup-core/src/library/song),
[`crates/lineup-core/src/midi_file/import.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/midi_file/import.rs)
and [`crates/lineup-core/tests/key_finding.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/tests/key_finding.rs)

A demo is a constant of text (see [How a demo is
written](demos.md)). Short tunes are typed in by hand.
Longer pieces are read from a version a computer can read, a MIDI file,
an engraving or a lead sheet, by small scripts written for each kind of
source. The scripts are not part of the app: what they write is, as
ordinary Rust constants, and the `// Source:` comment above each song
names what it was read from and what was left out. This chapter is
about the methods: how a beat, a tune, a key and chords are read from
notes, how the notes get their names, and what a demo keeps of a score.
Most of it is music theory put to work. What to listen for in the
pieces is in the manual's [listening
guide](https://naelolaiz.github.io/ferrous-lineup/manual/finding-a-demo.html).

## Kinds of source

| Source | What it holds | For |
|---|---|---|
| **ABC** (the collections of John Chambers and Henrik Norbeck, The Traditional Tune Archive) | a tune written as letters, often with chord names | folk songs and dances, the Balkans |
| **MusicXML lead sheets** (OpenEWLD, the public-domain part of the Wikifonia archive) | a tune with the chord names over it | songs of most genres: swing standards, blues, Latin, soul, pop |
| **LilyPond engravings** (the Mutopia Project, OpenScore) | the whole score, every voice, as text | classical pieces |
| **MIDI sequences of a score** | every voice, played to a click | the Misa Criolla, The Rite of Spring, Petrushka, tango and folk arrangements |
| **MIDI performances** | a pianist playing freely, with no click | tangos played on the piano |
| **A transcription in scale degrees** (Hooktheory's TheoryTab) | the tune and the chords as numbers in a key | The Big Bang Theory theme |
| **SymbTr** | Turkish makam music, note by note | the Turkish songs |
| **Page images** of a printed score | what the eye reads, bar by bar | a few tangos and songs |

The text formats are already close to the app's notation, so their
scripts mostly translate. A lead sheet's triplets become the nearest
sixteenths, the way the [lead sheet](../band/lead-sheet.md) itself
rounds them, and where a lead sheet's chord clashes with its own
melody the chord jazz musicians usually play there is used, with a word
in the comment. Where a source gives no chords, the comment says the
chords are a plain harmonization. Many Balkan dances are written in
sixteenths (7/16, 11/16); here they are written in eighths, every note
twice as long, which sounds the same at the tempo given.

## Reading a score

### An engraving, through MIDI

**LilyPond** is a language for printing music from text. Here are the
first notes of the first violins in Handel's "For Unto Us a Child Is
Born":

```text
re'''4 sol''8 sol'' do''' do''' do''' do''' |
```

The note names are Italian (`re` is D, `sol` is G, `do` is C), each
`'` raises the note an octave, and the number is the length (`4` a
quarter, `8` an eighth). LilyPond writes a score as a MIDI file as well
as a page, and the script reads that file bar by bar:

- **Dynamics.** LilyPond plays each dynamic mark at a set velocity
  (*p* 69, *mf* 86, *f* 95), so those velocities read back as the
  marks, `!p`, `!mf`, `!f`.
- **Staccato.** LilyPond plays a staccato note half as long as it is
  written. To tell a staccato quarter from an eighth, the score is
  compiled twice, once without its staccato dots: a note that is
  shorter in the first file is staccato, and gets a `.` in the demo.
- **Thirty-second notes.** Bach's written-out trills are faster than a
  step. The notation can write half a step, `D5:1/2`.

Some scores leave things out, and the comment says what was added:
Messiah's edition prints its fortes in the first violins only, so a
forte holds for all the strings. Editions also have mistakes. Die
Forelle's engraving writes D natural in bars 43 and 44, against a D flat
in the left hand. Bars 19 and 20 have the same music with D flat, and
so does a second edition, OpenScore's: the demo plays D flat.

### A pickup of its own length

Alfonsina y el mar starts one eighth note before its first full bar.
Rather than a bar of 6/8 that is nearly all rest, as "Clementine"
writes its pickup, the upbeat can be a bar of its own length, since a
part can have its own time signature:

```rust
const ONE_EIGHT: TimeSignature = TimeSignature::new(1, 8);
```

```rust
Part::new("Pickup", "!mf F#4:2", "")
    .in_time(ONE_EIGHT)
```

`TimeSignature::new` is a `const fn`, so a time signature can be a
constant too.

### A choir is three channels, not one

Two voices of a choir often sing the same note. Say the altos start an
E in sixteenths while the tenors hold an E for a whole beat: on one
channel those are two notes with the same pitch on the same step, and a pattern never keeps two such notes (see
[No two notes in the same
place](../program/project.md#no-two-notes-in-the-same-place)): the
first wins, and the tenors' held E would be lost. So the chorus has its
altos, tenors and basses on three lines, three channels, as they are on
three staves of the score. The Misa Criolla's choir is one line,
"Choir", its voices joined by `&`: where two of them meet on one note,
that note sounds once.

### Figured bass: chords written as numbers

Baroque composers did not write out the keyboard player's chords. Under
the bass line they wrote small numbers, the **figures**, each an
interval above the bass note, and the player filled in the chords.
Chrysander's edition of Messiah prints figures over some of the bass,
and the Handel demo's chart follows them where they are given:

| Figure | Means | In this chorus |
|---|---|---|
| none | a root-position triad | most bars |
| 6 | a triad in first inversion: the third in the bass | bar 1: B with 6 is `G/B` |
| 6/5 | a seventh chord in first inversion | bar 1: F♯ with 6/5 is `D7/F#` |
| 4/2 | a seventh chord in third inversion: the seventh in the bass | bar 28: G with 4/2 is `A7/G`; bar 43: C with 4/2 is `D7/C` |
| 6 with a slash | first inversion, the sixth raised a semitone | bar 26: E with a raised 6 is `C#dim/E` |
| + on its own | the third raised | bar 22: A with C♯, `A` |
| a natural sign | the third made natural | bar 58: D with F natural, `Dm` |

A seventh wants to fall a step, so a 4/2 chord, with the seventh in the
bass, is nearly always followed by a 6 on the note below: `A7/G` goes to
`D/F#` in bar 29, `D7/C` to `G/B` in bar 44. How the chord sheet writes
inversions is in the manual's [Slash chords and
inversions](https://naelolaiz.github.io/ferrous-lineup/manual/chords.html#slash-chords-and-inversions).

## Reading a sequence

A **sequence** is a MIDI file made in a sequencer, every voice on its
own track, to a click. The Misa Criolla, the Rite of Spring and
Petrushka come from sequences of their scores. Bar lines are in the
file; what the script has to do is read the performance back as a
score:

- each note goes on the nearest sixteenth, or the nearest third of one
  where the sequence plays a triplet, which then lands exactly on the
  tick (see [Notes between
  steps](../sound/midi.md#notes-between-steps));
- the tempo map is read as the score's marks: a deep dip that comes back
  is a **fermata**, a run of small changes a **glide** (*rit.* or
  *accel.*), and anything else a new tempo;
- the velocities are read as dynamics, from *pp* to *ff*;
- a **tremolo**, the same chord struck faster than a sixteenth, is
  written as one held chord.

An orchestral score has more lines than a band has channels. The
script keeps the tune in front and follows it from instrument to
instrument with `played_by`, gives the lines that matter their own
channels, and writes the rest on the Keys channel. Lines doubled at the
unison or the octave are written once. Grace notes, trills and
glissandos are simplified, and the comment says how.

## Reading a performance

A pianist playing freely, with no click, leaves a MIDI file with no
bar lines in it: the file's own grid means nothing. The tangos in
`tango_played.rs` come from such files. Everything a score would say
has to be heard.

### Finding the beat

The beat is found the way a listener finds it, with the method of Dan
Ellis's "Beat tracking by dynamic programming" (2007), the same one the
app uses on a recording (see [Beats: onsets, autocorrelation and
dynamic
programming](../theory/listen.md#beats-onsets-autocorrelation-and-dynamic-programming)):
how much new sound starts at each moment, the steadiest period in it,
then the chain of beats that lands on the most onsets while keeping
close to that period. Each bar then gets the tempo it was played at,
and a tempo marker is written wherever the tempo moves by half a beat a
minute or more. Uno has 61 of them.

### A steady beat off the grid

Some files are sequenced to a steady click but saved at another tempo,
so the click's beats are a fraction of a tick long and drift across the
file's own bars. Balderrama's notes fall nowhere near the file's grid,
but its bombo strikes every 293 or 294 ticks from the first bar to the
last: a steady beat. To find it exactly, number the strokes by the beat
they fall on, `k = 0, 1, 2, ...`, and fit a straight line through them,
tick against `k`. The **least squares** line is the one with the
smallest sum of squared misses, and its slope, the length of the beat,
has a formula:

```text
period = Σ (k − k̄)(t − t̄) / Σ (k − k̄)²
```

where `k̄` and `t̄` are the averages. For Balderrama the eighth note is
293.966 ticks long, and the strokes miss the line by a few ticks,
thousandths of a second: a machine, not a player.

The line gives the beat but not the bar: which of the six eighths is
the first? Harmony changes on the downbeat, and of the choir's long
held notes most start on one and the same eighth of the six. That
eighth is the first of the bar, and the tune turns out to start two
eighths before it, with a pickup, as a zamba often does.

### The tune and the key

The **tune** is the top line: of the notes sounding at each moment on
the melody's track, the highest (the **skyline**, as the [lead
sheet](../band/lead-sheet.md) reads a tune too). Notes under it that
double the tune, such as a second voice in thirds, go on a line of
their own.

The **key** comes from how long each of the twelve pitch classes
sounds, compared with key profiles, the way the app's own key finder
does it (see [Comparing: the Krumhansl-Schmuckler
method](../theory/hearing.md#comparing-the-krumhansl-schmuckler-method)).
Where it chooses wrong the key is set by hand, and a part that changes
key gets its own `in_key`. The test that at least four notes in five are
in their part's key (`melodies_are_mostly_in_their_key`) catches a
wrong choice of minor, too: Balderrama's tune sings D natural far more
often than D sharp, so in E harmonic minor only 228 of its 288 notes
are in the key and the test fails, while in E natural minor 257 are.

### The chords

For every bar, half bar or beat, the script scores each common chord
on all twelve roots: the notes inside the chord count for it, the notes
outside against it, and a bass note on the chord's root counts extra.
Chords of the key get a small bonus, so a passing note does not turn A
minor into something exotic, and a bar is split only where the split
scores clearly better. That is the same idea as the app's own [chord
hearing](../theory/hearing.md), and it has the same weakness: notes
alone name every colour as a chord of its own. A sixth the pianist adds
turns Bm into Bm6, and a note of the tune on its way adds a flat ninth
to F♯7.

A **lead sheet** of the song fixes that. Its bars do not line up with a
free performance (the pianist adds an introduction, repeats a strain,
stretches a bar), so its chords are not laid over the bars one by one.
Instead the sheet gives the reading its **vocabulary**: only the chords
the sheet uses may be chosen. A sheet in another key is first
**transposed**, every chord moved by the same interval, which keeps
each chord's job in the key: Uno's sheet is in G and the pianist plays
in C, so G becomes C, D7 becomes G7 and Em becomes Am.

Yira, yira's bars 38 to 40, read from the notes alone, are
`Bm6 F#7b9 | Bm F#7 | Bm A7`. Limited to the sheet's chords they are

```text
Bm F#7 | Bm F#7 | Bm A7
```

The left hand colours the B minor with its sixth, G♯, and the tune
touches G, a flat ninth over F♯7. Under the colours are the chords the
song is built on, i and V7 of B minor.

With no lead sheet, the chords are checked by ear and by what they do.
Tiempos viejos ends on D, F, A and B. Those four notes are also B D F A,
**Bm7♭5**, but the bass plays D, and the chord comes after A7, the
dominant of D minor: it is **Dm6**, D minor with an added sixth. A
chord's name follows its bass and its job, not only its notes.

### Spelling the notes

A MIDI file has numbers, not names. Note 70 is A sharp or B flat, and
the file does not say which. The key names the seven notes of its
scale; the other five follow the chord around them:

- **A note of the chord is spelled as the chord has it.** In F♯7 (F♯,
  A♯, C♯, E) note 70 is A sharp, the chord's third; in Gm (G, B♭, D) it
  is B flat.
- **A chord on a note outside the key is named by where it goes.** A
  dominant seventh falls a fifth, so the chord before C♯m is G♯7. A
  seventh chord that falls a semitone is a **tritone substitute**: the
  chord before G is A♭7, not G♯7. A diminished seventh rises a
  semitone: G♯dim7 goes to A.
- **A note outside the chord is spelled by where it goes.** A
  chromatic passing note between two notes a whole tone apart is a
  sharp on the way up and a flat on the way down: A, A♯, B and G, G♭,
  F.
- **No E sharp, B sharp, C flat or F flat for a passing note**: on the
  page they look like mistakes for F, C, B and E. The key's own notes
  still can be one: in B minor the raised fourth that leads up to the
  dominant, F sharp, is E sharp.

The app's own spelling of notes outside the key, which needs no chord,
is in [Notes from outside the
key](../theory/scales.md#notes-from-outside-the-key); the lead
sheet's, which looks at the chord as these rules do, in [Which
letter](../band/lead-sheet.md#which-letter-spellingrs).

## A transcription in scale degrees

The Big Bang Theory theme comes from a **transcription** on
Hooktheory's TheoryTab site, where songs are written down the way a
theory class reads them: every note a **scale degree** in the key (1
is the tonic, 5 the fifth), every chord a Roman numeral. The Sheet Sage
dataset (Chris Donahue and others, 2022) gathers about 26,000 of them in
one file. A note there looks like this:

```json
{"sd": "6", "octave": 0, "beat": 1, "duration": 1}
```

Scale degree 6, in the octave that starts on the tonic, on beat 1, a
beat long. The theme is in G, and with the tonic as G4 the tune runs
from D4 to G5, the range a lead sheet writes it in:

| Written there | In G | Here |
|---|---|---|
| sd 6, octave 0, 1 beat | E, above the tonic | `E5:4` |
| sd 5, octave 0, ½ beat | D | `D5:2` |
| sd 1, octave 0, ¼ beat | G, the tonic | `G4:1` |
| sd 5, octave −1, ½ beat | D, below the tonic | `D4:2` |

The transcription is also **aligned** to the recording: the verse's beat
1 is at 7.6 seconds and its beat 33 at 19.66 seconds. 32 beats in 12.06
seconds is 32 × 60 / 12.06 = 159.2 beats a minute, so the verse carries
`.with_tempo(&[tempo(0, 159)])` after an intro at 130.

## What a demo must keep

Two tests over every song shape how a piece is cut into parts and how
its tempo is written.

**A MIDI export must come back note for note.**
`every_demo_comes_back_note_for_note` exports each demo as a MIDI file,
imports it, exports it again and compares the two files. The importer
cuts a long song into patterns at bar lines no note crosses (see
[Cutting the song into
patterns](../sound/midi.md#cutting-the-song-into-patterns)); where
every bar line for many bars is crossed by a held note, it has to cut
one, and the cut note is struck again. A demo strikes held notes again
at its own part boundaries anyway, so a boundary at the right bar line
makes a clean cut: the Misa Criolla's Credo, whose bass ties over
almost every bar line, has one at bar 30, where the soloists come in.

**A demo keeps a sensible tempo.** `every_demo_keeps_a_sensible_tempo`
keeps every tempo marker between three tenths and two and a half times
the song's tempo. A sequence's last chord is often held by slowing the
click right down: the Credo's sequence holds its last chord at 20 beats
a minute, and the demo at 46.

One more test reads the songs in a way that needs care. The key
finder's test measures how many bars of the demos it hears in their
written key, and a piece in two keys at once would make that figure
meaningless: whichever key it names, half the music disagrees. Such
songs go on a list of their own:

```rust
const POLYTONAL: &[&str] = &["The Rite of Spring, The Augurs of Spring"];
```

The main test leaves them out, and a second one checks that the written
key is among the keys heard, but not the only one:

```rust
assert!(
    result.right_bars < result.bars,
    "{name} is heard in one key: drop it from POLYTONAL"
);
```

The second assert keeps the list honest: if the finder ever heard the
Augurs in one key, the song would no longer belong there.

## Checking a demo against its source

The last step is to play the demo with the [playback
trace](../sound/trace.md):

```text
ferrous-lineup --trace "Balderrama" --out balderrama.tsv
```

and look for every note of the source in the trace: for a score at the
same step and pitch, on the right channel and with its length; for a
performance at the same pitch within a fifth of a second. What is
missing must be what a demo leaves out on purpose:

- unisons on one channel, which sound once (two hands on the Keys
  channel, two guitars striking the same note, the voices of a choir on
  one line);
- tremolos and strums faster than a sixteenth, written as held chords;
- drum strokes that land on the same step of one lane;
- notes the comment names, such as an edition's mistake put right.
