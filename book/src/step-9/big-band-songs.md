# Swing and big band songs, with a horn section

Files: [`crates/lineup-core/src/library/song/jazz.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/jazz.rs),
[`crates/lineup-core/src/library/song/blues.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/blues.rs),
[`crates/lineup-core/src/library/song.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song.rs)
and [`crates/lineup-core/src/library/demo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/demo.rs)

The **File > Demos** menu had seventeen jazz songs, most of them ragtime, and
several played only part of their piece: one strain of The Entertainer,
the chorus of Bill Bailey without its proper ending. This change adds
**33 swing, big band and early jazz songs**, each played in full, and
completes twelve of the demos that were there. The big band songs also
get something no demo had before: a **horn section** behind the tune.

## Trying it

Open **File > Demos > Songs > Jazz > Tea for Two** (or type "tea" in
the search box at the top of the Demos menu). Besides Lead, Keys and Bass, the
channel rack now has a fourth channel, **Horns**. With **Edit >
Settings… > Audio > SoundFont instruments** on, the tune is a trumpet
and the Horns channel is the saxophones, holding each chord for as long
as it lasts. Mute
Horns (click its light in the rack) and play again: the band sounds
like a small group. Unmute it, and it is a big band.

Now open **Oh, Lady Be Good!**. Here the alto saxophone has the tune,
and the horns are the brass, punching each chord on beat 1 and just
before beat 3. Then **Embraceable You**, a ballad: a tenor saxophone
over trombones.

The new songs:

| Songs | From |
|---|---|
| Tea for Two, Sometimes I'm Happy, Great Day, Without a Song | Vincent Youmans, 1924–1929 |
| Oh, Lady Be Good!, Fascinating Rhythm, 'S Wonderful, Strike Up the Band, Liza, Embraceable You, But Not for Me | George Gershwin, 1924–1930 |
| My Blue Heaven, Makin' Whoopee, Love Me or Leave Me, Carolina in the Morning | Walter Donaldson, 1922–1928 |
| Honeysuckle Rose | Fats Waller, 1929 |
| Charleston, If I Could Be with You | James P. Johnson, 1923–1926 |
| King Porter Stomp | Jelly Roll Morton, 1924 |
| Limehouse Blues, Japanese Sandman, Indiana, Rose Room | 1917–1922 |
| Sweet Georgia Brown, Ain't Misbehavin', It Had to Be You, Tiger Rag, After You've Gone, St. Louis Blues | 1914–1929 |
| In the Mood, Sing, Sing, Sing, Take the "A" Train, Hit the Road Jack | 1936–1960 |

Completed: The Entertainer, Maple Leaf Rag, Elite Syncopations and Pine
Apple Rag (every strain, with the trio in its own key), Twelfth Street
Rag and Dill Pickles (their later strains), Golliwogg's Cakewalk (the
whole piece, with the middle section in G flat, which the app calls F
sharp major: the same keys of the piano), I Got Rhythm (its
verse), Bill Bailey, Swanee and Avalon (each chorus twice, with the
score's first and last endings), and the chords of Ballin' the Jack.

## The music

### A big band

A swing band of the 1930s had four **sections**: about five
saxophones, four trumpets, four trombones, and the **rhythm section**
(piano, guitar, bass and drums). The arranger rarely lets everyone
play the tune. One section has it, and another plays a **background**
behind it. Two backgrounds are everywhere:

- **Pads**: the saxophones hold the chords, quietly, under a trumpet's
  tune. A pad changes when the chord changes and does nothing else, so
  you hear the harmony move without anything new to follow.
- **Punches** (or "hits"): the brass play short chords on the beat and
  just before beat 3, the same Charleston rhythm the piano comps in.
  Punches push the band forward; they are the "swing" in a swing
  arrangement as much as the ride cymbal is.

That is what the three new bands of the demos do: **BIG_BAND** is a
trumpet over saxophone pads, **SAX_SECTION** an alto saxophone over
brass punches, and **BALLAD_BAND** a tenor saxophone over trombone pads.

### Verse and refrain

Most of these songs were written for Broadway shows or for sheet music,
and have two parts: a **verse**, which sets the scene, and a
**refrain** (or chorus), the part everybody knows. Bands kept the
refrain and dropped the verse, and many lead sheets do too. Where a
written verse could be found, the demo plays it once (I Got Rhythm,
Liza, It Had to Be You, If I Could Be with You); then the refrain
twice, the way a band plays the "head" again after a chorus of solos.
Nothing was filled in from memory, so songs whose verse has no written
source we could read start at the refrain.

### Thirty-two bars, two shapes

Almost every refrain is **32 bars**, four phrases of eight. They come
in two shapes:

- **AABA**: a phrase, the same phrase again, a contrasting **bridge**,
  and the phrase a last time. Oh, Lady Be Good!, Honeysuckle Rose,
  Ain't Misbehavin', Take the "A" Train, Sing, Sing, Sing and I Got
  Rhythm are AABA. Look at the playlist of one: the A section is one
  pattern, used three times, and the ending of the second A is a
  pattern of its own.
- **ABAC**: two halves that start alike and end differently, the
  first half open, the second closing on the tonic. Indiana, Rose
  Room, Embraceable You, But Not for Me and Carolina in the Morning are
  ABAC.

Two older forms are here too. **The blues**, twelve bars, is the riff
strain of In the Mood and two of the three strains of St. Louis Blues.
**The rag**, from marches: several strains of 16 bars, each repeated,
in the order A A B B A C C D D, with the C and D strains, the
**trio**, in another key.

### Key changes you can see

Every key change of these scores is a key marker on the ruler (see
[Key changes](../step-8/key-changes.md)), so you can watch the harmony
keys move:

- **Tea for Two** is in A flat, and its second phrase is the first one
  a major third higher, in C. The last chord of that phrase, E flat 7,
  is the V7 of A flat, and leads straight back home.
- **'S Wonderful** spends the first half of its bridge in G, a major
  third above its E flat; the second half comes back through G7, C7,
  F7 and B flat 7, a chain of dominants.
- **The rags and King Porter Stomp** move to the **subdominant**, the
  key a fourth higher (a fifth lower), for the trio: The Entertainer
  from C to F, Maple Leaf Rag and King Porter Stomp from A flat to D
  flat, Elite Syncopations and Pine Apple Rag from F to B flat. This
  comes from the march, where the trio is the softer, singing part; a
  rag often ends in that new key.
- **Tiger Rag** climbs twice: from B flat to E flat, then to A flat for
  the "Hold that tiger" strain, each a fourth up.
- **St. Louis Blues** keeps its tonic and changes its mode: the middle
  strain is in G minor, over a habanera rhythm, between blues in G
  major.

### Chains of dominants and other turns

Listen for these:

- **Sweet Georgia Brown** opens on D7, G7 and C7, four bars each,
  before the first F chord, in its thirteenth bar: each chord is the V
  of the next, so the song is home only when it reaches F.
- **I Got Rhythm**'s bridge (D7, G7, C7, F7, two bars each) is the same
  idea, two bars at a time, and its A section turns I-vi-ii-V. Hundreds
  of tunes were written on these "rhythm changes".
- **Hit the Road Jack** is four chords over and over, i, bVII, bVI, V7,
  falling step by step in the bass: the **Andalusian cadence** (it is in
  the Library under that name).
- **Love Me or Leave Me** is in minor, with a raised sixth: the tune
  leans on F sharp, and the lead sheet writes its home chord as A minor
  6 (A, C, E, F sharp). So the demo's key is A **Dorian**, the minor
  scale with that sixth, rather than natural minor.

## The code

### A horn section in the band

A song's `Band` was three General MIDI instruments, one for each
channel of a demo. It gets a fourth field:

```rust
pub struct Band {
    pub lead: u8,
    pub keys: u8,
    pub bass: u8,
    pub horns: Option<Horns>,
}

pub struct Horns {
    pub program: u8,
    pub comping: Comping,
}
```

`Option` says the horn section may be missing: every older song keeps
`horns: None` and sounds as before. A horn section is an instrument and
a [`Comping`](../step-8/new-rhythm.md), the same rhythm patterns the
piano uses: `Comping::Held` for pads, `Comping::Charleston` for punches.

The bands are `const` values, built at compile time. `Band::new` sets
`horns: None`, and a second `const fn` adds a section:

```rust
const fn with_horns(self, program: u8, comping: Comping) -> Band {
    Band {
        horns: Some(Horns { program, comping }),
        ..self
    }
}

const BIG_BAND: Band = Band::new(gm::TRUMPET, gm::ACOUSTIC_GRAND_PIANO, gm::ACOUSTIC_BASS)
    .with_horns(gm::TENOR_SAX, Comping::Held);
```

`..self` is **struct update syntax**: "every other field as in
`self`". A `const fn` can be called where Rust needs a value at compile
time, such as the right-hand side of a `const`, which is why the builder
style works here.

### Writing the horns' part

`song_demo` already turned each part's chord chart into the Keys and
Bass notes with `accompany(spans, tonic, comping, bass)`. The horns are
one more call to the same function, with the horns' comping and no bass
line:

```rust
let mut horn_notes = match horns {
    Some((_, horns)) if part.left.is_empty() => {
        accompany(&part.spans(bar), tonic, horns.comping, Bass::None).chords
    }
    _ => Vec::new(),
};
```

The **match guard** `if part.left.is_empty()` skips parts whose
accompaniment is written out note by note (a rag's left hand), which
have no chart for the horns to play. Because `accompany` voices the
chords the same way for both calls, the horns play the piano's voicing,
which sits around E flat above middle C: the range of saxophones and
trombones, under the tune.

The test `a_big_band_has_its_horns` opens every song with a horn
section and checks that its demo has a Horns channel on the right
instrument, with notes in every part that has a chart.

### How the songs were written down

Each song has a `// Source:` comment with the written transcription its
notes come from. Most are **lead sheets** in MusicXML from OpenEWLD,
the public-domain part of the Wikifonia archive, turned into this
app's notation by a small converter; the chords were checked against
the chord charts jazz musicians use, and where a lead sheet's chord
clashed with its own melody, the more common chord was used, with a
note in the comment. Triplets became the nearest sixteenths, the way
the [lead sheet](lead-sheet.md) rounds them.

### Copyright

Most of these songs were published in 1930 or earlier by composers who
died before 1956: they are in the public domain everywhere. Six are
free in the United States (published before 1931) but still protected
in countries where copyright lasts 70 years after the author's death
(Sweet Georgia Brown, Ain't Misbehavin', It Had to Be You, Tiger Rag,
After You've Gone, St. Louis Blues), and four are still under copyright
everywhere (In the Mood, Sing, Sing, Sing, Take the "A" Train, Hit the
Road Jack). A comment above each says which case it is.

## Exercises

1. Open **Tea for Two** and find the key marker where the second
   phrase starts. Which chord of A flat is C major? (Hint: count up
   from A flat: C is the third, so C major is III, a chord the key of A
   flat does not contain.)
2. In **Oh, Lady Be Good!**, mute the Lead channel and play along on
   the piano roll's keyboard. Which bars of the A section change chord
   twice?
3. Give **Rose Room** a horn section: change its `band:` from
   `SAX_SECTION` to `BIG_BAND` and run it. What do pads change about how
   the bridge sounds?
4. Write a band of your own in `song.rs`, `Band::new(gm::CLARINET, ...)`
   with `.with_horns(gm::BRASS_SECTION, Comping::Backbeat)`, and give it
   to **Charleston**. Does a backbeat suit a two-beat song?
