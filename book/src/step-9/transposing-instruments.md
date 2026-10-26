# Lead sheets for B♭, E♭ and F instruments

Files: [`crates/lineup-core/src/lead_sheet/transpose.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/lead_sheet/transpose.rs),
[`crates/lineup-core/src/lead_sheet/score.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/lead_sheet/score.rs),
[`crates/lineup-core/src/lead_sheet/layout.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/lead_sheet/layout.rs)
and, in the app, [`crates/lineup-app/src/lead_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/lead_sheet.rs)

Put a [lead sheet](lead-sheet.md) of a song in C in front of a trumpet
player and a pianist, ask them to play the first note together, and
they will not agree. The pianist plays C; the trumpet, reading the same
C, sounds the B♭ a whole tone lower. The trumpet is a **transposing
instrument**: the notes it reads are not the notes it sounds. To play
with the band, the trumpet player needs a sheet written a whole tone
higher, in D.

This chapter adds that sheet. The Lead sheet window gets a **For**
choice, and the same song can be written for B♭, E♭ and F instruments,
each in the key its players read in.

## Why some instruments read in other keys

Wind instruments come in families of different sizes: the saxophones
from soprano to baritone, the clarinets, the horns. Each size sounds
lower or higher, but the fingers are the same: on every saxophone, the
same fingers play the note written C. If the music were written as it
sounds, a player switching from alto to tenor would have to learn new
fingerings for every note. So the music is written for the fingers
instead, and each instrument is named after the note that sounds when
it plays a written C: a trumpet **in B♭**, an alto saxophone **in E♭**,
a horn **in F**.

Music written as it sounds is at **concert pitch**, or "in C". Piano,
guitar, bass, voice, flute, violin and trombone all read it.

| Instruments | A written C sounds | The part is written |
|---|---|---|
| Piano, guitar, bass, voice, flute, violin, trombone | C | as it sounds |
| Trumpet, cornet, flugelhorn, clarinet, soprano saxophone | the B♭ below | a major second higher |
| Tenor saxophone | the B♭ an octave and a second below | a major ninth higher |
| Alto saxophone | the E♭ below | a major sixth higher |
| Baritone saxophone | the E♭ an octave and a sixth below | an octave and a sixth higher |
| Horn, English horn | the F below | a perfect fifth higher |

The part is always written higher than it sounds, and within a family
by the same interval give or take an octave, so a song in C is written
in D for every B♭ instrument, in A for every E♭ instrument and in G for
every F instrument.

## Using it

Open **View > Lead sheet**. Next to **Clef** there is now **For**:

![The Lead sheet window's toolbar with the For menu open: Concert pitch, Bb instruments, Eb instruments and F instruments](../images/step-9-transposing-for-menu.png)

- **Concert pitch** writes the song as it sounds, as before.
- **Bb instruments**: trumpet, clarinet, soprano and tenor saxophone.
- **Eb instruments**: alto and baritone saxophone.
- **F instruments**: horn and English horn.

Hover over a choice to see which instruments it is for and how much
higher it is written. The sheet changes at once: the key signature, the
tune and the chord symbols move together, and the top left corner of
the first page says who the part is for, "B♭ instruments", where
printed parts name their instrument. **Export PDF…** offers the file
name with the instruments in it, `I Got Rhythm (Bb instruments).pdf`,
so the parts of one song don't overwrite each other. The choice is
remembered, like the paper and the zoom: a saxophone player who opens
another song finds it written for them again.

Here is the I Got Rhythm demo for B♭ instruments. The song is in B♭
major, two flats; the part is in C major, with no sharps or flats at
all. This is one reason so many jazz tunes are in flat keys: concert B♭
is C for the trumpet and the tenor saxophone, and concert E♭ is C for
the alto saxophone.

![The lead sheet of I Got Rhythm for Bb instruments: "B♭ instruments" at the top left, no key signature, the chords Am, E7, Am, Dm, then Am6, F7, Am, F, Am6, Fm6](../images/step-9-transposing-b-flat.png)

The demo starts with an introduction on G minor chords (Gm, D7, Cm). On
the B♭ part they are Am, E7 and Dm; on the E♭ part, in G major, Em, B7
and Am; on the F part, in F major, Dm, A7 and Gm.

The song itself, its file and what the band plays are left as they
are. Only the sheet is written differently.

## The key: never more than six sharps or flats

Moving the key is adding semitones: two for B♭ instruments (B♭ up to
C), nine for E♭ instruments (E♭ up to C) and seven for F instruments
(F up to C).

```rust
pub fn semitones(self) -> u8 {
    self.note().pitch_class().semitones_up_to(PitchClass::C)
}

pub fn key(self, key: Scale) -> Scale {
    Scale::new(key.root.transpose(i32::from(self.semitones())), key.kind)
}
```

`note()` is the note a written C sounds, B♭ for `Transposition::BFlat`,
so the instruments' names and their distances come from one place. The
kind of key stays: F minor is G minor for a trumpet, D Dorian is B
Dorian for an alto saxophone.

The new key is a `Scale` like any other, so it is named the way the app
names every key
([Which spelling does a key use?](../step-3/theory.md#which-spelling-does-a-key-use)):
each of the twelve keys has one name with at most six sharps or flats,
and that is the one used. This matters for parts, because counting up
by letters would sometimes give keys nobody writes in:

| Song | B♭ part | E♭ part | F part |
|---|---|---|---|
| C | D | A | G |
| F | G | D | C |
| B♭ | C | G | F |
| E♭ | F | C | B♭ |
| A♭ | B♭ | F | E♭ |
| D♭ | E♭ | B♭ | A♭ |
| G | A | E | D |
| D | E | B | A |
| A | B | F♯ | E |
| E | F♯ | D♭ | B |
| B | D♭ | A♭ | F♯ |
| F♯ | A♭ | E♭ | D♭ |

A song in E for an alto saxophone, a major sixth up, would strictly be
in C♯ major, seven sharps. D♭ major has the same notes with five flats,
and it is what a copyist writes. A song in F♯ for a trumpet would be in
G♯ major, which needs an F double sharp in its key signature; A♭ major,
four flats, has the same notes. A test goes
through all twelve major and minor keys for every instrument and checks
that no part has more than six.

## The chords: moved letters and all

A chord must keep its place in the key: the V7 of the song is the V7 of
the part. [Chord names](chord-names.md) showed how a chart is moved to
another key, counting letters as well as semitones, and the chord sheet
already uses it when the song changes key (`HarmonyName::moved`). The
part uses the same function, from the song's key to the part's:

```rust
pub fn harmony(self, harmony: HarmonyName, key: Scale) -> HarmonyName {
    harmony.moved(key, self.key(key))
}
```

Because it goes from tonic to tonic, it follows the key's spelling too.
From C to D every letter moves up one, so Dm7 G7 Cmaj7 becomes Em7 A7
Dmaj7. From F♯ to A♭ every letter moves up two, so the V7 of F♯, C♯7,
becomes E♭7, the V7 of A♭, and never D♯7. The bossa nova demo Garota de
Ipanema is in D♭ in the app; for an alto saxophone, in B♭, its D♭maj7
E♭7 E♭m7 D7 are B♭maj7 C7 Cm7 B7.

## The notes: every interval keeps its name

The tune moves by the same semitones, but its notes need letters too,
and here a short cut would go wrong. The obvious way is to move each
note up and then spell it again in the new key with the rules of
[the lead sheet](lead-sheet.md#which-letter-spellingrs). Mozart's
Turkish March, in A minor, shows the problem. Its tune leans on E from
the semitone below, D♯–E. In B minor, the trumpet's key, the same
sounds would be spelled again as F–F♯: the rules keep a white key of
the piano plain outside the key, so they write F rather than E♯. But
D♯–E is a minor second, a note leaning on the next, and F–F♯ is an
augmented unison, one letter bent upward. A player reads a different
line.

So the notes are spelled at concert pitch first, where the rules know
the key and the chords the song was written in, and then each name is
moved by the same letters and semitones as the key:

```rust
pub fn name(self, name: NoteName, key: Scale) -> NoteName {
    let written = self.key(key);
    if written == key {
        return name;
    }
    // ... (a row or a set has no letters: its notes take their usual names)
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
```

`NoteName::above` is the same function that moves chord roots, so the
notes and the chords move by exactly the same letters. The D♯–E of the
Turkish March is E♯–F♯ for a trumpet, B♯–C♯ for an alto saxophone and
A♯–B for a horn: a minor second each time, as Mozart wrote it.

The last lines deal with the keys that were renamed. A song in E has a
lowered sixth, C. For an alto saxophone, in D♭ rather than C♯, the
letters move from E to D, and C becomes B double flat (B♭♭), the
lowered sixth of D♭ spelled strictly. Nobody wants to read that, and
it is the same note as A, so the name is made simpler, as chord
symbols are. A double sharp or flat that the new key itself has
stays: the leading tone of C♯ harmonic minor is B♯, and on the trumpet
part, in D♯ harmonic minor, it is C double sharp (C##), the leading
tone of that key.

To check all of this, every demo's tune was written for every
instrument and compared with the concert sheet: every note's pitch was
right, and every name moved by the key's letters except the few that
needed the simpler name. The concert sheets of all 343 demos are
exactly as they were before this change.

## The octave: where the tune sits on the staff

The interval says which notes to write, not in which octave. A tenor
saxophone sounds a ninth below its written notes, an octave lower than
a trumpet, yet in books of lead sheets for B♭ instruments both read the
same page: the tenor player simply sounds it an octave lower. What
matters on the page is that the tune sits on the staff, not on a ladder
of leger lines above it.

So the part is written by the instrument's interval, moved by whole
octaves until the middle note of the tune is as near as it can be to
the middle line of the staff (B4 in the treble clef):

```rust
pub fn shift(self, pitches: &[Note], clef: Clef) -> i32 {
    let interval = i32::from(self.semitones());
    // ... (concert pitch is written as it sounds)
    let middle = i32::from(clef.middle_line().midi());
    let median = i32::from(median.midi());
    (-4..=4)
        .map(|octaves| interval + 12 * octaves)
        .min_by_key(|shift| ((median + shift - middle).abs(), (shift - interval).abs()))
        .unwrap_or(interval)
}
```

The middle note is the **median**, the one in the middle once the
notes are sorted, as for choosing the clef: a few very high or low
notes don't pull it away. For a tune in the range of a voice the
result is the instrument's own interval: a major second up for B♭
instruments, a major sixth up for E♭ instruments, a fifth up for F
instruments, so trumpet, alto saxophone and horn sound the tune where
it is. The Turkish March climbs to C6, and for B♭ instruments it comes
out a minor seventh *lower*, the same letters an octave down, where a
trumpet can play it.

`min_by_key` keeps the candidate with the smallest key, and the key
here is a pair. Pairs compare like words in a dictionary: by the first
item, and by the second only when the first ones are equal. So the
distance to the middle line decides, and when two octaves are equally
near, the one nearer the instrument's own interval wins.

Parts for transposing instruments use the treble clef, which all of
them read; **Clef** can still choose the bass clef, and then the tune
is centred on that staff's middle line, D3.

## What changed where

The lead sheet was already split into steps (see
[From notes to notation](lead-sheet.md#from-notes-to-notation)). One
step is new, and only two of the old ones needed to know about
instruments:

- `transpose.rs` is new: the `Transposition` type with `key`,
  `harmony`, `name` and `shift`.
- `score.rs` asks it for the key of each bar, the chords, the names of
  the notes and the octave. Two closures keep that short:

  ```rust
  let key_at = |step: usize| transposition.key(keys.key_at(step));
  let harmony_of =
      |marker: ChordMarker| transposition.harmony(marker.harmony, keys.key_at(marker.start));
  ```

  A **closure** is a small function written inline. These two borrow
  `keys` and `transposition` from the function around them, so each call
  only says which step or which marker.
- `layout.rs` writes "B♭ instruments" at the top left of the first page,
  with the flat from the music font as in chord symbols. When the title
  is too wide to leave room beside it, the words go on a line under it.
- `melody.rs` and `rhythm.rs` did not change, and `spelling.rs` only
  learned where the middle line of each clef is: the tune is read,
  spelled at concert pitch, rounded and cut into note values the same
  way for every instrument.

In the app, `Transposition` is stored with the paper and the zoom in
the window's settings:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
struct Settings {
    paper: Paper,
    zoom: f32,
    transposition: Transposition,
}
```

`#[serde(default)]` is what lets a new field arrive without breaking
anything: settings saved by an older version have no `transposition`,
and serde fills it from `Settings::default()`, concert pitch.
`Transposition` itself gets its default from an attribute on a variant:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Transposition {
    #[default]
    Concert,
    BFlat,
    EFlat,
    F,
}
```

The project file doesn't change: who reads the sheet is a choice of the
reader, not part of the song.

## How it was tested

Thirteen new tests in `lineup-core` and one in the app:

- every instrument's interval, and the keys of a song in C (D, A, G),
  in minor keys and modes;
- no part key with more than six sharps or flats, for all twelve major
  and minor keys and every instrument; E is D♭ for an alto saxophone, F♯
  is A♭ for a trumpet;
- chords keep their place: ii–V–I in C is Em7 A7 Dmaj7, C♯7 in F♯ is E♭7
  in A♭, a polychord and N.C. are written too;
- notes keep their intervals: D♯–E in A minor becomes E♯–F♯, B♯–C♯ and
  A♯–B; the lowered sixth of E is A in D♭; C## stays in D♯ harmonic
  minor;
- the octave for a tune around G4, a high tune and a bass line, in both
  clefs; concert pitch changes nothing;
- whole sheets: a trumpet part with its D major signature and no extra
  sharps, a key change written in the parts' keys (A major to D♭ major
  for an alto saxophone), the treble clef for parts, and the words at
  the top of the page, beside or under the title;
- the PDF's file name, `Op. 24 (Bb instruments).pdf`.

Then the parts were looked at in the app for I Got Rhythm, the Turkish
March, with its many changes of key, and Garota de Ipanema.

![The Turkish March for Eb instruments: "E♭ instruments" at the top left, F♯ minor with three sharps, C♯ minor with four from bar 4, F♯ minor again from bar 6, and the chords F♯m, C♯m, G♯ and C♯](../images/step-9-transposing-key-changes.png)

## Exercises

1. **The exact octave.** Add a choice that writes every part exactly as
   the instrument plays it: a ninth up for tenor saxophone, an octave
   and a sixth up for baritone. Which function changes? Try it on a
   tune around C5 for the tenor: how many leger lines does it need, and
   why do books of lead sheets not do this?
2. **The E♭ clarinet** sounds a minor third *higher* than it reads, so
   its part is written a minor third lower. Is that a new
   `Transposition`, or the same notes as the alto saxophone's part in
   another octave? What would `shift` need to know to tell them apart?
3. **D and G instruments.** A trumpet in D sounds a whole tone higher
   than written, an alto flute in G a fourth lower. Add them. What do
   `note()` and `semitones()` return, and what else in the window and
   the label has to know about them?
4. **A transposed chord sheet as text.** [Chord sheets as
   text](text-chord-sheets.md) writes the chords for pasting. Give it the
   same choice of instruments. Which single function does the work for
   every chord?
5. **Concert key on the part.** Some parts print the song's own key
   under the instrument, "Concert key: B♭". Where in `layout.rs` would it
   go, and what does `lay_out` need to know that it doesn't now?
