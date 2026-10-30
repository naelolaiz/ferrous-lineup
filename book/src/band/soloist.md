# The soloist

Files: [`crates/lineup-core/src/harmony/chord_scale.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/chord_scale.rs),
[`crates/lineup-core/src/band/solo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/solo.rs),
[`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs),
[`crates/lineup-app/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet.rs)
and [`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs)

The band accompanies: it plays the chords so that someone else can play
the tune or improvise over them. The **soloist** is that someone else,
a player who improvises a line over the chord sheet. It is
experimental and off until you switch it on, because a solo made by
rules is a study, not a performance. But it is a useful study: every
note it plays has a reason, and the piano roll shows the reason, so you
can see how an improvised line is built and try the same rules on your
own instrument.

## Trying it

Press **Ctrl+N** for a new song with a band, and type these bars on the
chord sheet, one chord a bar:

```text
| Dm7 | G7 | Cmaj7 | A7 | Dm7 | G7 | C6 | C6 |
```

In the **Library**, apply **Swing** (under Jazz). Then open the sheet's
**🎷 Band** menu and tick **Soloist (experimental)**. Hover over it
first: the hint says who the soloist is in this style, a tenor sax. A
**Soloist** track appears in the playlist, and the channel rack gets a
Soloist channel with the tenor sax. Press Space: over the band, the sax
plays two-bar phrases in swung eighth notes, with a breath between
them.

**Another solo** in the same menu plays a different one; undo brings
the last one back. Untick the soloist and the track and the channel go
away. Like every band part, the solo is played again when the sheet
changes, and if you edit its notes it stays as you left it until you
press **Play it again** (or **Another solo**).

Now double-click the soloist's clip to open its part in the piano roll.
Its notes are coloured by what they are:

- **green** for chord tones, marked with which one: `R` for the root,
  `3`, `5`, `7` (and `6`, `9` when the chord has them), or `•` for a
  note of a set, which has no root to count from;
- **blue** for the other notes of the chord's scale: `p` a passing note,
  `n` a neighbour note, `s` any other;
- **orange** for chromatic notes, from outside the scale: `a` an
  approach, `cp` a chromatic passing note.

Point at a note, or play the song, and the line above the grid says
what it is, in a sentence:

![The piano roll of "Soloist 1" over bars 1 to 5. The notes are green, blue or orange with marks on them: "7", "p", "5", "3", "R", "B3 3", "D4 5", "cp" and so on. Above the grid: "Bar 3, beat 4, step 3 · Bb3: a chromatic passing note, a half step from the notes either side. Over Cmaj7 the scale is C major (Ionian): C D E F G A B." and the legend "Soloist: chord tones green (R 3 5 7), scale notes blue (p passing, n neighbour, s other), chromatic orange (a approach, cp passing)"](../images/step-9-soloist-piano-roll.png)

In this solo, bar 3 ends with B3 (the 7th of Cmaj7) on beat 4, then
B♭3 on the "and", then A3 on beat 1 of bar 4, the root of A7: three
notes a half step apart, walking down into the next chord. The B♭ is in
neither chord, and that is fine: it is on the weak half of the beat and
it leads somewhere.

## The music: how an improvised line is built

### Chord tones on the beat

Ask a jazz teacher how to start improvising over changes, and the first
rule is nearly always the same: **play chord tones on the strong beats,
and anything that connects them in between**. On the beat the ear
checks the melody against the harmony; between the beats it is moving
and forgives more. A line that lands on a chord tone on each beat
"makes the changes": you can hear the chords in it even without the
band.

The most telling chord tones are the **3rd** and the **7th**, the
**guide tones**. They say whether a chord is major or minor and what
kind of seventh it has, and from one chord to the next of a ii–V–I they
move by half steps: the 7th of Dm7 (C) falls to the 3rd of G7 (B), and
the 7th of G7 (F) falls to the 3rd of Cmaj7 (E). Many solos are guide
tones joined up.

The soloist chooses, for each beat, the chord tone **nearest** to
where it is going: the line moves up or down, and keeps its direction
until it turns at the edge of the instrument's range, or now and then
on its own. One time in five it skips a chord tone and leaps to the
next. That gives lines that mostly move by small steps, as tunes do,
with a leap here and there.

### Chord scales

Between the chord tones go other notes, and which ones depends on the
chord: over Dm7, is the note between A and C a B or a B♭? Jazz teaching
answers with **chord scales**: each chord gets a seven-note scale that
contains it, and the notes between come from that scale.

The scale is not chosen from the chord alone, but from the chord **in
its key**. Over Dm7 in C major, the B is natural, because the music is
in C: the scale is C major from D, D E F G A B C, called **D Dorian**.
The modes of C major are what the chords of C major play:

| Chord in C major | Its scale    | Notes             |
|------------------|--------------|-------------------|
| Cmaj7 (I)        | C Ionian     | C D E F G A B     |
| Dm7 (ii)         | D Dorian     | D E F G A B C     |
| Em7 (iii)        | E Phrygian   | E F G A B C D     |
| Fmaj7 (IV)       | F Lydian     | F G A B C D E     |
| G7 (V)           | G Mixolydian | G A B C D E F     |
| Am7 (vi)         | A Aeolian    | A B C D E F G     |
| Bm7♭5 (vii)      | B Locrian    | B C D E F G A     |

Seven names for the same seven notes. The names matter because they
say which note is the root and where the half steps fall from it: Dorian
is a minor scale with a major 6th (B over D), Lydian a major scale with
a raised 4th (B over F), Mixolydian a major scale with a minor 7th
(F over G). Those are the notes that give each chord its colour in the
key.

Other chords get their scale the way a player thinking in the key would
choose it, in this order:

1. **A chord of the key** plays the key's scale from its root, as in the
   table. In a minor key, a chord that uses the raised 7th (E7 in A
   minor, with G♯) plays the key's **harmonic minor** from its root: A
   harmonic minor from E, E F G♯ A B C D, called **E Phrygian
   dominant**.
2. **A chord borrowed from the parallel key** plays that key's scale:
   Fm in C major comes from C minor, so it plays C minor from F, which
   is F Dorian. B♭7 in C major (from C minor too) plays B♭ Mixolydian.
3. **A secondary dominant** plays the scale of the chord it leads to.
   D7 in C major is the V7 of G, so it plays G major from D, D
   Mixolydian. A7 in C major is the V7 of Dm, a minor chord, so it plays
   D harmonic minor from A: A B♭ C♯ D E F G, A Phrygian dominant. Over
   A7, a B♭ is a scale note, pointing to the D minor chord that
   follows.
4. **Anything else** plays the scale its kind usually gets: **Lydian**
   for a major chord from outside the key (the D♭ of the Neapolitan
   sixth in C), **Lydian dominant** for a dominant seventh that comes
   from outside and leads nowhere, **half-whole diminished** for 7♭9
   and 7♯9, **whole-half diminished** for a diminished seventh,
   **whole tone** for an augmented chord, **Locrian** for a
   half-diminished one, **Dorian** for a minor chord. In a key that is
   not a seven-note scale (a blues scale, a pentatonic one) every chord
   gets the usual scale of its kind, with major chords on plain major
   (Ionian) and dominants on Mixolydian.

One more rule: **the chord's own notes win**. E9 in C major leads to Am,
so it starts from A harmonic minor (E F G♯ A B C D), but its 9th is
F♯, and F and F♯ cannot both be in the scale with the same letter. The
F♯ of the chord replaces the F: E F♯ G♯ A B C D, **Mixolydian ♭6**.

There is a second school, which gives every chord its own scale
without asking about the key (Dorian for every m7, Mixolydian for every
7). It is simpler to teach and often sounds the same. Thinking in the
key is closer to how the tunes were written, and it keeps the notes
that chords of the key have in common the same from chord to chord.

### Notes between the beats

Each kind of note between the chord tones has a name, and most of them
are older than jazz: harmony textbooks call them **non-chord tones**.

- A **passing note** fills the gap between two chord tones a third
  apart, by step, going the same way: C **D** E over C.
- A **neighbour note** (or auxiliary note) steps away from a chord tone
  and comes back: E **F** E.
- A **chromatic passing note** fills a whole step with the half step
  between: D **D♯** E.
- A **chromatic approach** comes into a chord tone from a half step
  below (or above): **F♯** G. When the soloist starts a phrase with a
  note just before the beat, a **pickup**, it is usually one of these.
- Any other scale note, reached or left by a leap, is marked `s`, and a
  note from outside the scale that does none of this is marked `x`. The
  soloist does not play `x` notes, but a part you edit can have them.

A step here is a half step or a whole step, or, in harmonic minor, the
augmented second between its 6th and 7th notes (F and G♯ in A), because
those two are next to each other in the scale.

Chromatic passing notes on the "and" of the beat are part of the sound
of **bebop**, the jazz of Charlie Parker and Dizzy Gillespie: a half
step added between two scale notes keeps the chord tones on the beats
when a line runs in eighth notes. Jazz teachers later wrote this down
as the "bebop scales".

### Phrases and breathing

A singer has to breathe, and a horn player too, and so lines come in
**phrases**. The soloist plays two-bar phrases. Each one may start a
beat late or with a pickup, and leaves a beat or two empty at the end.
A line that never stops sounds like an exercise; the rests are what
make it sound like someone talking. Then the eighth notes are swung as
the band is (see [Swing](../sound/tempo.md#swing)), and the
library's phrasing for melodies shapes its loudness, as it does for the
demos' tunes.

The soloist plays over the chorus, not over the intro or the ending
(see [The form](song-form.md#the-form-an-intro-choruses-and-an-ending)),
and it keeps playing through shots and rests (see [Shots, holds, rests
and pushes](stops.md)): the band stops precisely so that a
soloist can be heard alone. Over "no chord" it is silent. Over a
pitch-class set, a micromode or a row form there is no scale to choose,
and the soloist plays the set's own notes, on the beats and between
them.

### Who plays

Each family of styles has its soloist, as a General MIDI instrument and
the range it plays in. The family is the one of the band's instruments
(its **Instrument lineup**):

| Styles                  | Soloist           | Range     |
|-------------------------|-------------------|-----------|
| Pop                     | alto sax          | G3 to G5  |
| Rock                    | overdriven guitar | A3 to C6  |
| Blues                   | harmonica         | C4 to C6  |
| Jazz                    | tenor sax         | D3 to F5  |
| Soul, funk and R&B      | alto sax          | G3 to G5  |
| Hip hop and lo-fi       | vibraphone        | C4 to C6  |
| Electronic              | saw lead          | C4 to C6  |
| Latin                   | trumpet           | B♭3 to A5 |
| Tango                   | tango accordion   | G3 to C6  |
| Latin American folklore | pan flute         | E4 to E6  |
| Flamenco                | nylon guitar      | E3 to A5  |
| Balkan and Turkish      | clarinet          | D3 to B♭5 |
| Reggae and ska          | trumpet           | B♭3 to A5 |
| Country and folk        | fiddle            | G3 to E6  |
| Classical and film      | flute             | F4 to F6  |

The tango accordion is General MIDI's stand-in for the bandoneon. The
ranges are the comfortable middle of each instrument, not all of it:
solos sit where an instrument sings.

## How it works

### Choosing a chord scale

`chord_scale` in `harmony/chord_scale.rs` follows the four rules above,
one `if` after another:

```rust
pub fn chord_scale(chord: Chord, key: Scale) -> PitchSet {
    if !key.kind.is_tonal() || !key.is_heptatonic() {
        return usual_scale(chord, false);
    }
    let harmonic = |root: PitchClass| Scale::new(root, ScaleKind::HarmonicMinor);
    let source = if key.is_chord_of_key(chord) {
        match key.uses_raised_seventh(chord) {
            true => harmonic(key.root),
            false => key,
        }
    } else if let Some(parallel) = key.borrowed_from(chord) {
        match parallel.is_diatonic(chord) {
            true => parallel,
            false => harmonic(key.root),
        }
    } else if let Some(target) = key.secondary_dominant_target(chord) {
        match target.quality.is_minor() {
            true => harmonic(target.root),
            false => Scale::new(target.root, ScaleKind::Major),
        }
    } else {
        return usual_scale(chord, true);
    };
    with_chord_notes(source, chord)
}
```

Almost all of the music theory is elsewhere: `is_chord_of_key`,
`borrowed_from` and `secondary_dominant_target` are what the chord
sheet uses to name borrowed chords and secondary dominants (see [The
key's helpers](../theory/numerals.md#the-keys-helpers)). This
function only puts them in the order a player would ask the questions.

Three bits of Rust worth noticing:

- An `if` is an **expression** in Rust: `let source = if ... { a } else
  if ... { b } else { ... };` gives `source` the value of whichever
  branch ran. The last branch does not give a value; it `return`s from
  the whole function, which is allowed because `return` has the type
  `!` ("never"), which fits anywhere.
- `else if let Some(parallel) = key.borrowed_from(chord)` both tests
  and unpacks: the branch runs only when there is a parallel key, and
  inside it `parallel` is that key.
- `harmonic` is a **closure**, a small function written in place, used
  three times. `|root: PitchClass| Scale::new(...)` takes a root and
  gives a scale.

`usual_scale` is one `match` on the chord's quality, with a guard for
the chords from outside the key:

```rust
Major | Add9 | Major6 | Six9 | Major7 | Major9 | Major13 if outside => LYDIAN,
Major | Add9 | Major6 | Six9 | Major7 | Major9 | Major13 | Sus2 => IONIAN,
```

and ends by joining the scale and the chord. A `PitchSet` is twelve
bits in a number, one for each pitch class, so joining two sets is a
bitwise or of the numbers:

```rust
PitchSet::from_bits(scale.bits() | chord.pitch_set().bits())
```

### The chord's notes win, letter by letter

```rust
fn with_chord_notes(source: Scale, chord: Chord) -> PitchSet {
    let names = source.chord_notes(chord);
    let mut set = chord.pitch_set();
    for pitch_class in source.pitch_classes() {
        let letter = source.spell(pitch_class).letter;
        let replaced = names.iter().any(|name| {
            name.letter == letter
                && name.pitch_class() != pitch_class
                && !source.contains(name.pitch_class())
        });
        if !replaced {
            set.insert(pitch_class);
        }
    }
    set
}
```

The set starts with the chord's notes, and each note of the source
scale joins it unless a chord note has the **same letter** but a
different pitch, from outside the scale. Letters, not pitches, decide
which note is replaced: the F♯ of E9 replaces F, not G. That is why the
scale is spelled with note names (`NoteName`, a letter and an
accidental) before it is compared, and not just with numbers.

The last condition, `!source.contains(...)`, keeps enharmonic spellings
from doing damage. Fdim7 in C major is spelled F A♭ C♭ E♭♭, and C♭ and
E♭♭ have the letters C and E: without the condition they would knock C
and E out of the scale, although C♭ is B and E♭♭ is D, both in C major
already. The test `every_chord_scale_holds_its_chord` plays every kind
of chord on every root in several keys and checks that each scale holds
its chord and has six to nine notes.

### Naming the scale

The names are a table of slices, the semitones above the root and the
name:

```rust
pub(crate) const NAMES: &[(&[u8], &str)] = &[
    (IONIAN, "major (Ionian)"),
    (DORIAN, "Dorian"),
    (&[0, 1, 3, 5, 7, 8, 10], "Phrygian"),
    // ...
    (&[0, 1, 4, 5, 7, 8, 10], "Phrygian dominant"),
    // ...
];

pub fn chord_scale_name(scale: PitchSet, root: PitchClass) -> Option<&'static str> {
    let relative = scale.relative_to(root);
    NAMES
        .iter()
        .find(|(steps, _)| PitchSet::from_semitones(steps) == relative)
        .map(|(_, name)| *name)
}
```

`relative_to(root)` turns the scale so that the chord's root is 0.
Then it is the same set as one of the table's rows, or none. `&'static
str` says the name lives as long as the program: it is text written in
the code, so the function can hand it out without copying it.

### What each note is

The kinds of notes are an `enum`, and one of them carries data:

```rust
pub enum NoteRole {
    ChordTone(Option<u8>),
    Passing,
    Neighbour,
    ChromaticPassing,
    Approach,
    ScaleTone,
    Outside,
}
```

`ChordTone(Some(3))` is the 3rd; `ChordTone(None)` is a note of a set,
which has no root to count from. `note_roles` reads a line and gives a
role for each note. For a note of the scale it looks at its neighbours,
`before` and `after`, which are `None` at the start of a phrase or
after a rest:

```rust
match (before, after) {
    (Some(before), Some(after))
        if before == after && is_step(before, pitch, over.scale) =>
    {
        NoteRole::Neighbour
    }
    (Some(before), Some(after))
        if (before < pitch) == (pitch < after)
            && before != pitch
            && pitch != after
            && is_step(before, pitch, over.scale)
            && is_step(pitch, after, over.scale) =>
    {
        NoteRole::Passing
    }
    _ => NoteRole::ScaleTone,
}
```

Matching on a **tuple** of two `Option`s checks both at once, and the
`if` after a pattern, a **match guard**, adds a condition. The arms are
tried in order, so the neighbour note (same note before and after) is
found before the passing note. `(before < pitch) == (pitch < after)`
compares two `bool`s: true when the line goes up both times or down
both times, which is what makes a passing note pass.

`note_roles` reads any line the same way, the soloist's or one you
edited, so the piano roll can paint every soloist note from it. Its
colour is a `match` on an `Option<NoteRole>` with guards too:

```rust
let mut fill = match role {
    Some(role) if role.is_chord_tone() => NOTE,
    Some(role) if role.is_chromatic() => SOLO_CHROMATIC,
    Some(_) => SOLO_SCALE,
    None if scale.contains(note.pitch.pitch_class()) => NOTE,
    None => NOTE_OUTSIDE,
};
```

The `None` arms are what every other part gets: green in the key, red
outside it.

### Dice that always roll the same

An improvised solo needs chance: which chord tone, whether to leap,
whether to start late. But the band must play the same notes every
time the song is opened, or the parts you froze would no longer match
the sheet. So the soloist rolls **pseudo-random** dice: a small function
that turns a number, the **seed**, into a stream of numbers that look
random but are always the same from the same seed.

```rust
struct Dice(u64);

impl Dice {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut x = self.0;
        x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        x ^ (x >> 31)
    }

    fn chance(&mut self, percent: u64) -> bool {
        self.next() % 100 < percent
    }
}
```

This is **SplitMix64**, a well-known generator: add a large odd
constant, then scramble the bits with shifts, exclusive ors (`^`) and
multiplications. In Rust an ordinary `+` or `*` that overflows stops a
debug build with a panic, because overflow is usually a bug. Here it is
the point, so `wrapping_add` and `wrapping_mul` say so: the result
wraps around past the largest `u64`, as the algorithm expects. The
underscores in `0x9E37_79B9_7F4A_7C15` are only for reading.

`struct Dice(u64)` is a **tuple struct**: a struct with one unnamed
field, reached as `self.0`. Wrapping the number in its own type means
the only things you can do with it are `next`, `chance` and `below`.

The seed is saved in the project (`Band.solo: Option<u64>`, `None` with
no soloist, 1 when you tick it), it is part of the band's fingerprint,
and **Another solo** adds one to it. A new fingerprint means the band
plays again, so a new seed plays a new solo, and undo, which brings the
old seed back, brings the old solo back.

### Improvising

`improvise` cuts the chorus into two-bar phrases with `chunks(2)`, and
each phrase into **pulses**: a pulse is a beat, or a quarter note of a
longer beat (a half note in 2/2 has two), so the soloist moves in
eighth notes whatever the time signature. The line is a struct with
the state a player keeps in their head: which way it is going and the
last note played.

```rust
struct Line {
    soloist: Soloist,
    up: bool,
    last: Option<u8>,
    notes: Vec<NoteEvent>,
}
```

`phrase` first picks a chord tone for every pulse, and only then fills
between them, because what goes between a beat and the next depends on
where the next one lands. It starts by deciding how much of the phrase
to play:

```rust
let late = usize::from(count >= 4 && dice.chance(35));
let rest = match count {
    6.. => 1 + dice.below(2),
    3.. => 1,
    _ => 0,
};
let Some(window) = pulses.get(late..count - rest) else {
    return;
};
```

`usize::from(bool)` turns `true` into 1 and `false` into 0. `6..` and
`3..` are **range patterns**: 6 or more, then 3 or more (the arms are
tried in order). `let ... else` unpacks an `Option` or leaves: `get`
with a range gives a slice of the pulses, or `None` if the range does
not fit, and then the phrase is not played.

The pickup uses a **let chain**: several conditions and a `let` joined
with `&&` in one `if`.

```rust
if index == 0
    && pickup
    && pulse.start >= 2
    && let Some(over) = over_at(overs, pulse.start - 2)
{
    let approach = approach(target, over.scale, dice);
    self.play(approach, pulse.start - 2, 2, BETWEEN);
}
```

It reads as a sentence: if this is the first beat, and a pickup was
rolled, and there is room before it, and a harmony sounds there, play
an approach note an eighth before the beat. Before the 2024 edition of
Rust, the `let` needed an `if` of its own inside the first one.

`fill` chooses what goes between two chord tones: passing notes when
the scale notes between them fit in the time and each is a step from
the next, a neighbour note when the two chord tones are the same, the
half step between them when they are a whole step apart, and, for a
leap too wide to walk, the scale notes just before the target or a half
step below it.

### Joining the band

The soloist is one more `Player`, after the piano, bass, drums, guitar,
strings and organ. A style never lists it among its players
(`Player::Soloist | Player::Horns => false` in `BandStyle::players`);
the band adds it when the seed is there:

```rust
if self.has_soloist() {
    players.push(Player::Soloist);
}
```

Everything the bigger band gives each player comes with it: its own
channel with its General MIDI program, its track and pattern, being
frozen when edited, **You play** to mute it and play the solo yourself.
Two things are different. The soloist's notes go through the library's
**melody** phrasing instead of the accompaniment's, and the shots,
holds and rests do not touch it.

A song with a soloist needs at least format 19. Without one, the new
fields are left out and the file is as it would be without them.

## Testing it

In `harmony/chord_scale.rs`, one test per rule:
`chords_of_the_key_play_its_modes`,
`the_dominant_of_a_minor_key_plays_harmonic_minor`,
`borrowed_chords_play_the_parallel_key`,
`secondary_dominants_play_the_scale_of_their_target` and
`other_chords_play_the_scale_of_their_kind`, each checking the spelled
notes of a scale, as "D E F G A B C"; and
`every_chord_scale_holds_its_chord`, described above.

In `band/solo.rs`, `roles_name_what_each_note_does` reads short lines
by hand and checks every role, and
`a_role_is_described_with_its_chord_and_scale` the sentence the piano
roll shows. Three tests play many solos, over several progressions and
seeds: `chord_tones_on_the_beats_and_nothing_unexplained_between`
checks the rule of chord tones on the beats and that no note is left
unexplained, `a_solo_is_one_line_in_range_that_breathes` that the notes
never overlap, stay within a whole step of the soloist's range, are
silent where there is no chord and leave a rest of an eighth or more in
every two bars, and `the_same_seed_plays_the_same_solo` the dice.

In `band.rs`, `a_soloist_improvises_when_asked`,
`the_soloist_plays_the_chorus_only`,
`over_a_set_the_soloist_plays_its_notes_on_the_beats` and
`a_soloist_is_saved_in_format_nineteen` check the soloist as a member
of the band.
