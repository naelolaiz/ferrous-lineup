# A bigger band: guitar, strings and organ

Files: [`crates/lineup-core/src/band/guitar.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/guitar.rs),
[`crates/lineup-core/src/band/extras.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/extras.rs),
[`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-core/src/swing.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/swing.rs)
and [`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs)

Until now every band style had the same three players: a piano, a bass
and drums. That is a jazz trio, and the rhythm section of most bands,
but a rock band without its guitar or a tango without its violins does
not sound like itself. Now a style can bring up to three more players:
a **guitar**, **strings** and an **organ**, each playing the way that
music uses them.

![The chord sheet's Band menu, open at Reggae and ska. The hint over "One drop" reads: "Nothing on beat 1: kick and rim fall together on beat 3. 4/4, about 76 BPM. With a guitar on beats 2 and 4 and an organ bubbling." Behind the menu, the "You play" row lists Piano, Bass, Drums, Guitar and Strings](../images/step-9-bigger-band-styles.png)

## Trying it

Press **Ctrl+N** for a new chord sheet (the band plays **Pop beat**),
then **Progression > Pop > Four chords**. Next to the piano, bass and
drums, the playlist has a fourth track, **Guitar**, and the channel
rack a **Guitar** channel on the steel-string acoustic guitar. With
**Sounds > SoundFont instruments** on, it sounds like one.

Hover over a style in the **Band** menu: its hint now ends with who
else plays, as in "With a strummed guitar and strings in variation b".
Strings in variation b means they wait for the chorus: right-click a
bar, pick **Variation b**, and from there the strings come in, holding
the chords an octave above the piano. In the **You play** row, a
player who does not play yet is greyed out.

Try a few styles over the same chords:

| Style | Who comes in | What they play |
|---|---|---|
| Pop beat | steel-string guitar, strings in b | strummed chords, held strings |
| Rock beat | overdriven guitar, rock organ in b | power chords in eighth notes |
| Shuffle, Rock and roll | clean electric guitar | the boogie shuffle |
| Swing | jazz guitar | four to the bar |
| Funk, Disco | muted guitar, and strings in disco | sixteenths, scratched |
| One drop, Steppers, Rocksteady | clean guitar, organ | the chop on 2 and 4, the organ bubble |
| Bossa nova | nylon-string guitar | the bossa rhythm |
| Tangos | strings | held chords |
| Chorale | church organ | held chords |
| Hip hop, most electronic styles | nobody | |

Switch to a style without a guitar and the guitar's track and channel
go away; undo brings them back. A part you edited in the piano roll
stays, as always, until you click **Play it again**. Songs saved with
these players are format 17; a song with only the piano, the bass and
the drums is still saved as format 16, so older versions keep opening
it.

## The music: how a guitarist plays a chord

### Six strings and four fingers

A pianist can play any notes of a chord: ten fingers, every key in
reach. A guitarist has six strings, tuned from the lowest

| String | 6 | 5 | 4 | 3 | 2 | 1 |
|---|---|---|---|---|---|---|
| Open note | E2 | A2 | D3 | G3 | B3 | E4 |
| MIDI | 40 | 45 | 50 | 55 | 59 | 64 |

and a left hand with four fingers to press them against the **frets**,
one semitone each. Each string plays one note at a time, or none. A
string not pressed rings **open**, for free; that is why the keys of E,
A, D and G, whose chords use many open strings, are the guitar's home
keys.

So a guitar chord is a **shape**: a fret (or nothing) for each string.
Chord books write it as six characters from the low E string, with `x`
for a string not played and `0` for an open one:

```text
C    x32010     the A string at fret 3 (C), D at 2 (E), G open (G),
                B at 1 (C), high E open (E): C E G C E
G    320003
Am   x02210
F    133211     the first finger flat across fret 1: a "barre"
```

The lowest string that sounds must be the chord's **bass**, the note
the bass player plays (the root, or under a slash chord the note after
the slash). That is why the C shape leaves out the low E string: an E
there would make it C/E. Chord books do write C/E that way, `032010`.

### Finding the shape

Ferrous Lineup does not store a chord book. It finds the shapes the way
a guitarist learns them, by trying the frets in reach and keeping the
easiest. For each **position** on the neck (the fret the first finger
sits at), it tries every way to put a note of the chord on each string
within four frets, keeps the ones a hand can play, and scores them:

- every **needed** note must be there: the root, the third (or the
  second or fourth of a sus chord), the sixth or seventh, a fifth that
  is not perfect, and the highest extension, which names the chord (the
  13 of a G13);
- a perfect fifth and the lower extensions are **welcome**, and
  shapes with more of them are better;
- shapes low on the neck, with **open strings**, and on more strings
  are better;
- up to four pressed strings are fine. More need a **barre**, one
  finger flat across the lowest fret, which stops every string above
  it, so none of those can be open, and leaves three fingers for the
  rest.

One more rule keeps the sound clear. Two notes close together sound
muddy low down: a third between E2 and G#2 rumbles, while a fifth
there is fine. Arrangers have a table of the lowest note each interval
sounds clear from, the **low interval limits**, and a shape that goes
below them costs a little more. A little, not a lot: the open G,
`320003`, starts with G2 and B2, a third below its limit (B♭2), and
it is still the G every guitarist plays. Its sound is part of the
guitar's sound.

With those rules, the search finds the open chords of the chord books
(C `x32010`, G `320003`, D `xx0232`, Am `x02210`, G7 `320001`, Cmaj7
`x32000`), barre chords in other keys (F `133211`, the open E shape a
fret higher, and B `x24442`, the A shape), and a shape for every chord
type in every key. The tests check all of them.

### Down and up

A guitarist does not strike the strings together. The pick sweeps
across them: on a **down-stroke** from the low strings to the high
ones, on an **up-stroke** back from the high ones, and the hand coming
up rarely reaches the low strings. The rhythms of the library already
mark which hits are the light "up" ones, between the beats. The guitar
plays each down-stroke on every string, low to high, a tick apart
(about 5 ms at 120 BPM, so a six-string strum takes about 25 ms, as a
real one does), and each up-stroke on the top four, high to low. Every
string stops where the hit stops.

### What each style's guitarist plays

- **Strummed chords** (pop, folk, country): the full shape in the
  style's rhythm.
- **Power chords** (rock, punk, metal): only the root, its fifth and
  the octave, `A5` = A2 E3 A3. Distortion adds new notes at the sums
  and differences of the frequencies played. For a fifth, whose
  frequencies are close to 3:2, they fall in line with the root, an
  octave or two below it; for a third they land on notes out of tune
  with the chord, and it buzzes and sours. So rock guitarists leave the
  third to the bass and the singer, and "A5" means "A without its
  third".
- **Four to the bar** (swing): three-note chords, the root, the third
  and the seventh, struck on every beat, the second and fourth a little
  harder. This is how Freddie Green kept time in Count Basie's band
  for almost fifty years. The shapes are the jazz **shells**: from a
  root on the low E string, the seventh comes next and the third an
  octave up (G7 = G2 F3 B3); from the A string, the third and then the
  seventh (C7 = C3 E3 B♭3).
- **The boogie shuffle** (blues, rock and roll): two strings only, the
  root with its fifth on one beat and with its sixth on the next, both
  eighths of each beat: A and E, A and E, A and F♯, A and F♯. It is the
  guitar part of countless blues and early rock and roll records. Over
  a minor chord it stays on the fifth.
- **The chop** (reggae, soul, Motown, bluegrass): a short chord on the
  top four strings on beats 2 and 4, with the snare. The low strings
  are left to the bass. Country calls the same thing the "chick" of
  "boom-chick": the bass plays "boom" on 1 and 3, the guitar "chick"
  on 2 and 4. In **ska** the guitar plays between all the beats instead.
- **Scratching** (funk, disco): sixteenth notes on a muted guitar,
  some accented, most only a percussive "chk".

### Strings and organ

**Strings** hold each chord for as long as it lasts, an octave above
the piano, softly: a **pad**, a cushion of sound under the band. Pop
uses them to lift the chorus, so in Pop beat and Motown they only play
in variation b. Ballads use the slower-starting strings, synth styles
the synthesizer ones.

The **organ** holds chords too, as a church organ in a chorale or a
Hammond in slow blues. In reggae it plays the **bubble**: short chords
between all the beats low down, and on beats 2 and 4 an octave higher,
with the guitar's chop. In a rock beat it waits for variation b, the
way an organ fills out a chorus.

### Which instrument

Every player plays a General MIDI instrument (see
[Every voice on its own channel](../step-8/voices-and-instruments.md)),
chosen by the style. The table numbers them from 1, the MIDI messages
from 0, so the code says 24 for the nylon guitar:

| Program (from 0) | Instrument | Styles |
|---|---|---|
| 24 | Acoustic guitar (nylon) | bossa nova, samba, bolero, folklore, rumba flamenca |
| 25 | Acoustic guitar (steel) | pop, folk, country, bluegrass, waltz |
| 26 | Electric guitar (jazz) | swing |
| 27 | Electric guitar (clean) | shuffle, rock and roll, surf, Motown, reggae, ska |
| 28 | Electric guitar (muted) | funk, disco |
| 29, 30 | Overdriven, distortion guitar | rock, punk, metal |
| 48, 49, 50 | Strings, slow strings, synth strings | pop, tango, ballads, R&B, trance |
| 16, 17, 18, 19 | Drawbar, percussive, rock, church organ | blues, reggae, rock, chorale |

## How it works

### Six players instead of three

`Player` gets three more variants, and the band keeps one channel and
one fingerprint per player in arrays indexed by them:

```rust
pub const COUNT: usize = 6;
pub const ALL: [Player; Player::COUNT] = [
    Player::Piano, Player::Bass, Player::Drums,
    Player::Guitar, Player::Strings, Player::Organ,
];

pub(crate) channels: [Option<usize>; Player::COUNT],
pub(crate) written: [Option<u64>; Player::COUNT],
```

Before, the band had a field each, `piano` and `bass`, and code that
asked "which channel?" matched on the player to pick the field. With
an array, `band.channels[player.index()]` answers for every player,
and a seventh player would only need a bigger `COUNT`. The array's
length is part of its type, `[Option<usize>; 6]`, so the compiler
checks that `ALL` really lists `COUNT` players.

`write_band`, which writes every player's part, already looped over
`Player::ALL`. It gets three more arms in its `match player`, and
everything after it, the human touches of `interpret::perform`, the
"louder in b", the patterns, the tracks and the fingerprints that keep
an edited part frozen, works for the new players unchanged.

### Who plays in which style: a table of constants

Each style's players are a value of this type:

```rust
pub struct Extra<Part> {
    pub part: Part,
    pub program: u8,
    pub only_in_b: bool,
}

pub struct Extras {
    pub guitar: Option<Extra<GuitarPart>>,
    pub strings: Option<Extra<PadPart>>,
    pub organ: Option<Extra<PadPart>>,
}
```

`Extra` is **generic**: `Part` is a type parameter, filled in as
`GuitarPart` for the guitar and `PadPart` for strings and organ. The
three fields "what it plays, on which instrument, does it wait for b"
are written once for both.

The styles are listed in a table that exists when the program starts,
built by the compiler:

```rust
const NONE: Extras = Extras { guitar: None, strings: None, organ: None };

const fn with_guitar(part: GuitarPart, program: u8) -> Extras {
    Extras { guitar: guitar(part, program), ..NONE }
}

const BY_STYLE: &[(&str, Extras)] = &[
    ("Pop beat", Extras {
        guitar: guitar(STRUMMED, STEEL_GUITAR),
        strings: pad_in_b(STRINGS),
        ..NONE
    }),
    ("Bossa nova", with_guitar(GuitarPart::Strum(Comping::Bossa), NYLON_GUITAR)),
    // ...
];
```

A **`const fn`** can be called where Rust needs a value at compile
time, such as in a `const`. `..NONE` is **struct update syntax**: the
fields not written take their values from `NONE`, so each style names
only the players it has. A test checks that every name in the table is
a real band style, so a typo cannot quietly leave a style without its
guitar.

### Enums that carry data

What the guitar plays is an enum whose variants can carry a value:

```rust
pub enum GuitarPart {
    Strum(Comping),   // full chords in this rhythm
    StylesRhythm,     // full chords, in the rhythm of the style's piano
    Chop,             // the top four strings on 2 and 4
    Power(Comping),   // power chords in this rhythm
    FourToTheBar,
    Boogie,
}
```

`Strum(Comping::Bossa)` and `Strum(Comping::Waltz)` are one variant
with different rhythms inside. A `match` can look inside it, and the
`|` patterns can be nested, as in variation b, where held and picked
chords become strummed ones:

```rust
match (variation, self) {
    (Variation::B, GuitarPart::Power(Comping::Held)) => GuitarPart::Power(Comping::Pulse),
    (Variation::B, GuitarPart::Strum(Comping::Held | Comping::ArpUp | Comping::ArpUpDown)) =>
        GuitarPart::Strum(Comping::Strum),
    (_, part) => part,
}
```

### A function as a parameter

Strummed chords, power chords and shells differ only in which notes
each harmony gets. The rest, playing the rhythm and strumming it, is
the same, so `strummed` takes the voicing as a parameter:

```rust
fn strummed(
    rhythm: Comping,
    bars: &[Bar],
    harmonies: &[Played],
    voice: fn(&Played) -> Vec<Note>,
) -> Vec<NoteEvent>
```

`fn(&Played) -> Vec<Note>` is a **function pointer** type: any plain
function with that signature fits, so `play` passes `shape`,
`power_chord`, `shell` or `top_strings` by name. Finding a shape tries
thousands of fret combinations, and a song repeats its chords, so
`strummed` remembers the shapes it found in a `HashMap`:

```rust
let voicing = shapes.entry(key).or_insert_with(|| voice(harmony)).clone();
```

`entry` finds the place for `key` in the map, and `or_insert_with`
calls the closure only if that place is empty. Each different chord is
searched once.

### Trying every fret: recursion with a callback

How many strings a shape uses is not known in advance, so the search
cannot be a fixed number of nested `for` loops. It is a function that
picks a fret for one string and calls itself for the next:

```rust
fn try_frets(
    choices: &[Vec<u8>],
    low: usize,
    top: usize,
    frets: &mut Vec<u8>,
    found: &mut impl FnMut(&[u8]),
) {
    let string = low + frets.len();
    if string > top {
        found(frets);
        return;
    }
    for &fret in &choices[string] {
        frets.push(fret);
        try_frets(choices, low, top, frets, found);
        frets.pop();
    }
}
```

`frets` is one `Vec` shared by every level: a level pushes its string's
fret, lets the deeper levels finish, and pops it again before trying
the next. When every string has a fret, `found` gets the whole shape.
`found` is a closure that scores it and keeps the best one, so it
changes something outside itself: that makes it an **`FnMut`**, and it
is passed as `&mut` so that every level of the recursion can call the
same closure, rather than each taking its own copy.

### Strums and swing

A strum starts its strings a tick apart, so a strum on the second
eighth of a beat starts at ticks 48, 49, 50 and so on. Swing
([Swing](swing.md)) moved only notes exactly on tick 48: it would have
played the first string late and left the others where they were,
tearing the strum apart. Swing now moves every note less than 6 ticks
(a sixty-fourth note) from the offbeat, which also suits notes played
by hand, rarely exactly on time.

### Saving: format 17, only when needed

The band's part of the file kept its old fields and gained optional
ones:

```rust
#[serde(default, skip_serializing_if = "Option::is_none")]
pub guitar: Option<usize>,
// strings, organ the same
pub written: [Option<u64>; 3],          // piano, bass, drums, as before
#[serde(default, skip_serializing_if = "none_written")]
pub written_too: [Option<u64>; 3],      // guitar, strings, organ
```

`skip_serializing_if` leaves a field out of the file when the function
named says so, and `default` fills it in when a file does not have it.
A band of three is therefore written exactly as before, and the file
says format 16. Only a song whose band has a guitar, strings or an
organ is format 17: an older version refuses it with a message instead
of silently dropping the guitar's channel.

## Exercises

1. Make a chord sheet with **C Am F G** and try **Pop beat**, **Folk**,
   **Rock beat** and **Swing**. Click **Piano**, **Bass** and **Drums**
   in the **You play** row, so that only the guitar is left. How does
   each guitarist play the same four chords?
2. Look at the guitar's notes in the piano roll for G and for F in Pop
   beat. Write their shapes as chord books do (`320003`). Why does the
   F shape have no open strings?
3. Which shape would you play for **E7**, and which notes of the chord
   does it double? Check your answer with the test
   `open_chords_are_the_ones_in_chord_books`.
4. In `extras.rs`, change Rock beat's guitar from
   `GuitarPart::Power(Comping::Pulse)` to
   `GuitarPart::Strum(Comping::Pulse)`: full shapes through the
   overdriven guitar. Listen with a SoundFont. Why do rock guitarists
   avoid this?
5. In the boogie shuffle, the sixth over a major chord makes the line
   rock. What would a minor sixth (8 semitones) sound like over a minor
   chord, and why is it risky? (Hint: what chord are A, C, E and F?)
6. (Rust) Give **Lo-fi** a nylon-string guitar picking the chords,
   `GuitarPart::Strum(Comping::ArpUp)`, in `BY_STYLE`. Then misspell
   the style's name: which test fails, and what does it print?
7. (Rust) `try_frets` takes `found: &mut impl FnMut(&[u8])`. Try
   `found: impl FnMut(&[u8])` instead. What does the compiler say about
   the recursive call, and why?
8. (Rust, harder) Add a **second keyboard**: an electric piano (program
   4) playing the style's comping an octave up in the R&B and Neo soul
   styles. Which parts of `band.rs` and `file.rs` must change? Check
   that a song with only three players still saves as format 16.
