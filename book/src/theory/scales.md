# Scales, modes and keys

Files: [`crates/lineup-core/src/harmony/scale.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/scale.rs),
[`crates/lineup-core/src/key_map.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/key_map.rs),
[`crates/lineup-core/src/tools.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/tools.rs)
and [`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs)

A key is what most of the app looks at before it shows anything: the
piano roll lights up the key's rows, the harmony keys play its chords,
the chord sheet names chords in it, the band plays in it, and every
note's name depends on it. This chapter is about the `Scale` type that
holds a key, how a key writes each of its notes, and what happens when
the song changes key, all at once or partway through.

## The music, briefly

A **scale** is a choice of some of the twelve pitch classes, counted up
from a **root**. The major scale is the white keys from C, C D E F G A
B: as semitones above the root, 0 2 4 5 7 9 11. The same pattern
started on another note gives the major scale of that note: from D, it
is D E F♯ G A B C♯. A scale on a particular root, used as the home of a
piece, is a **key**: D major.

The scales Ferrous Lineup knows, as semitones above the root:

| Scale | Semitones |
|---|---|
| major | 0 2 4 5 7 9 11 |
| minor (natural) | 0 2 3 5 7 8 10 |
| harmonic minor | 0 2 3 5 7 8 11 |
| melodic minor | 0 2 3 5 7 9 11 |
| Dorian | 0 2 3 5 7 9 10 |
| Phrygian | 0 1 3 5 7 8 10 |
| Lydian | 0 2 4 6 7 9 11 |
| Mixolydian | 0 2 4 5 7 9 10 |
| Locrian | 0 1 3 5 6 8 10 |
| major pentatonic | 0 2 4 7 9 |
| minor pentatonic | 0 3 5 7 10 |
| blues | 0 3 5 6 7 10 |
| chromatic | all twelve |

Harmonic minor raises the seventh note of the minor scale so that it
leans on the root a semitone above it; melodic minor raises the sixth
too. (Classical melodic minor falls back to the natural minor on the
way down; the app uses the rising form both ways, as jazz musicians
do.) The **modes**, Dorian to Locrian, are the major scale started on
another of its notes: D Dorian is the white keys from D. So each mode
shares its notes, and its key signature, with one major key: its
**relative major**. A minor's relative major is C major, three
semitones up.

The **key signature** is the sharps or flats written at the start of
every line of a score. Each major key has its own, and going up a
fifth adds a sharp: C major has none, G major one, D major two. Going
down a fifth adds a flat: F major has one, B♭ major two. That circle of
keys a fifth apart is the **circle of fifths**. The manual's [Keys and
modes](https://naelolaiz.github.io/ferrous-lineup/manual/keys-and-modes.html)
tells all of this with sounds to try.

## Kinds of scale: an enum with data

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScaleKind {
    Major,
    NaturalMinor,
    HarmonicMinor,
    MelodicMinor,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Locrian,
    MajorPentatonic,
    MinorPentatonic,
    Blues,
    Chromatic,
    Row(ToneRow),
    Set(PitchSet),
}
```

The thirteen scales of tonal music are plain variants. The last two
carry their notes with them: a twelve-tone row and a pitch-class set
can be the "key" of a song too, for music that has no key in the usual
sense. They have chapters of their own ([Twelve-tone rows and their
rules](twelve-tone.md) and [Pitch-class sets, and one
harmony with four names](sets.md)); here they mostly show
up as the cases that are different. One method sorts them out:

```rust
pub fn is_tonal(self) -> bool {
    !matches!(self, ScaleKind::Row(_) | ScaleKind::Set(_))
}
```

`Row(_)` matches a `Row` whatever row it holds.

The notes of a scale are listed as semitones above the root. For the
tonal scales those lists never change, so they are written into the
program and borrowed as `&'static [u8]`, slices that can have any
length, five for a pentatonic scale, seven for the major scale:

```rust
fn tonal_intervals(self) -> &'static [u8] {
    match self {
        ScaleKind::Major => &[0, 2, 4, 5, 7, 9, 11],
        ScaleKind::NaturalMinor => &[0, 2, 3, 5, 7, 8, 10],
        // ...
        ScaleKind::Chromatic | ScaleKind::Row(_) | ScaleKind::Set(_) => {
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
        }
    }
}
```

A row or a set has its own notes, which exist only inside the value, so
nothing `'static` can point at them. The public `intervals` therefore
returns an owned `Vec<u8>`, made from the row, the set or the table:

```rust
pub fn intervals(self) -> Vec<u8> {
    match self {
        ScaleKind::Row(row) => row.semitones().to_vec(),
        ScaleKind::Set(set) => set.semitones(),
        kind => kind.tonal_intervals().to_vec(),
    }
}
```

The last arm's pattern is a plain name, `kind`, which matches anything
and binds it, so every tonal scale goes there.

## A key: a root and a kind

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Scale {
    pub root: PitchClass,
    pub kind: ScaleKind,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    respelled: bool,
}
```

`respelled` is for the few keys that can be written two ways, below.
Most of the methods on `Scale` are short iterator chains. The pitch
classes of the scale are the root moved up by each interval:

```rust
pub fn pitch_classes(self) -> impl Iterator<Item = PitchClass> {
    self.kind
        .intervals()
        .into_iter()
        .map(move |interval| self.root.transpose(interval as i32))
}
```

`impl Iterator<Item = PitchClass>` says "some iterator of pitch
classes" without naming its real type, which involves a closure, and
closures have types nobody can write. `into_iter()` on the `Vec` gives
its numbers by value, and `move` gives the closure its own copy of
`self` (a `Scale` is `Copy`), since the iterator outlives the call.

The **degree** of a note is its place in the scale. `position` finds it,
counting from 0, where musicians count from 1:

```rust
pub fn degree_of(self, pitch_class: PitchClass) -> Option<usize> {
    self.pitch_classes().position(|p| p == pitch_class)
}
```

In A minor, E is degree 4 (musicians say "the fifth"), and F♯ has no
degree. `pitch_set` collects the same pitch classes into a
[`PitchSet`](pitch.md#a-set-of-pitch-classes-in-twelve-bits),
so the test that every mode shares its notes with its relative major is
a comparison of three numbers:

```rust
let c_major = Scale::default().pitch_set();
assert_eq!(Scale::new(D, ScaleKind::Dorian).pitch_set(), c_major);
assert_eq!(Scale::new(G, ScaleKind::Mixolydian).pitch_set(), c_major);
assert_eq!(Scale::new(A, ScaleKind::NaturalMinor).pitch_set(), c_major);
```

`Scale::default()` is C major, and `is_heptatonic` says whether a scale
has seven notes. Chords stacked in thirds and Roman numerals only make
sense in those, so many methods start by asking it ([Chords: building,
naming and inversions](chords.md)).

## Sharps or flats: how a key is written

Every minor key and mode shares its key signature with its relative
major, so the question "sharps or flats?" is answered by that major
key. `relative_major` says how far up it is:

```rust
pub(crate) fn relative_major(self) -> u8 {
    match self {
        ScaleKind::Major
        | ScaleKind::MajorPentatonic
        | ScaleKind::Chromatic
        | ScaleKind::Row(_)
        | ScaleKind::Set(_) => 0,
        ScaleKind::NaturalMinor
        | ScaleKind::HarmonicMinor
        | ScaleKind::MelodicMinor
        | ScaleKind::MinorPentatonic
        | ScaleKind::Blues => 3,
        ScaleKind::Dorian => 10,
        ScaleKind::Phrygian => 8,
        ScaleKind::Lydian => 7,
        ScaleKind::Mixolydian => 5,
        ScaleKind::Locrian => 1,
    }
}
```

D Dorian is the white keys from D, and C is ten semitones above D; E
Phrygian's C is eight above E, and so on. `pub(crate)` makes the method
visible inside `lineup-core` only: the rest of the program asks the
questions built on it.

The usual spelling then depends on that major key. The keys whose
signature has flats are F, B♭, E♭, A♭ and D♭ major, and their minors and
modes:

```rust
fn usual_spelling(self) -> Spelling {
    use PitchClass::*;
    match self.root.transpose(self.kind.relative_major() as i32) {
        F | ASharp | DSharp | GSharp | CSharp => Spelling::Flats,
        // F# major and Gb major are the same keys of the piano, and
        // the tonic's name decides between them. Their modes are
        // named with sharps (G# Dorian, B Lydian), except F Locrian,
        // whose notes are those of Gb major: F Gb Ab Bb Cb Db Eb.
        FSharp if self.root == F => Spelling::Flats,
        _ => Spelling::Sharps,
    }
}
```

The middle arm has a **match guard**, `if self.root == F`: the arm is
taken only if the pattern matches and the condition holds. F Locrian is
the one mode on those keys whose root is a white key with its own name;
written with sharps it would be E♯ Locrian, so it takes the flats of
G♭ major.

### The key signature, round the circle of fifths

A key signature is a number: how many sharps, or how many flats as a
negative number. Going round the circle of fifths, the major key `n`
fifths above C has `n` sharps. `key_signature` finds `n` by trying
every step round the circle until it lands on the relative major:

```rust
pub fn key_signature(self) -> Option<(i8, bool)> {
    if !self.kind.is_tonal() || self.kind == ScaleKind::Chromatic {
        return None;
    }
    let major = self.root.transpose(self.kind.relative_major() as i32);
    // Going round the circle of fifths: each fifth up adds a sharp.
    let fifths = (0..12i8)
        .find(|&n| PitchClass::C.transpose(7 * i32::from(n)) == major)
        .unwrap_or(0);
    let sharps = match self.spelling() {
        Spelling::Flats => fifths - 12,
        Spelling::Sharps => fifths,
    };
    let minor = self.kind.relative_major() == 3;
    Some((sharps, minor))
}
```

Twelve fifths up come back to C, so a key with flats is the same number
of fifths counted the other way: F major is eleven fifths up, which is
one fifth down, so 11 - 12 = -1, one flat. D♭ major is seven fifths up
and five down: five flats. The second value says whether the key is
minor, which a MIDI file's key signature event stores too ([MIDI:
messages, devices, recording and files](../sound/midi.md)). A row, a
set or the chromatic scale has no key signature, hence the `Option`.

The `0..12i8` range has its type written on the number, `12i8`, so `n`
is an `i8` like the result. `find` stops at the first `n` that fits;
one always does, and `unwrap_or(0)` only keeps the compiler from asking
what happens if not.

### Keys that can be written two ways

The keys with six sharps or six flats are the same keys of the piano:
F♯ major and G♭ major. Around them, C♯ major (seven sharps) is D♭ major
(five flats), and B major (five sharps) is C♭ major (seven flats).
Composers choose the name that fits the music around it; Debussy writes
the middle of Golliwogg's Cakewalk in G♭ major, a key of flats like the
E♭ major it comes from. Any other key would need eight signs or more
the other way round: D♯ major needs nine sharps, so it is always E♭
major.

So a `Scale` can be **respelled**: the same notes, the other name.
`other_spelling` gives that key when there is one:

```rust
pub fn other_spelling(self) -> Option<Scale> {
    use PitchClass::*;
    let key = self.as_key();
    let major = key.root.transpose(key.kind.relative_major() as i32);
    let either_way = key.kind.is_tonal()
        && key.kind != ScaleKind::Chromatic
        && matches!(major, FSharp | CSharp | B);
    either_way.then_some(Scale {
        respelled: !self.respelled,
        ..self
    })
}
```

`Scale { respelled: !self.respelled, ..self }` is **struct update
syntax**: a copy of `self` with one field changed. `spelling()` turns
the usual answer round when the key is respelled, and the tonic takes
the next letter: F♯ becomes G♭, B becomes C♭.

The field is private, so the only ways to set it are the methods, which
check that the key can be written the other way. `Scale::written(tonic,
kind)` builds a key from how its tonic is written, which is what the
Key menus and the key line of a typed chord sheet use: G♭ with "major" gives the
respelled F♯ major, and G♭ with "minor", a key that would need a double
flat in its signature, gives the usual F♯ minor.

In a file, the attribute `#[serde(default, skip_serializing_if =
"std::ops::Not::not")]` leaves the field out when it is `false`, which
is almost always, and reads a missing one as `false`. `skip_serializing_if`
takes the path of a function that receives `&bool` and returns `true`
to skip; `std::ops::Not::not` is the `!` operator as a function, so a
field is skipped when `!respelled`. A file saved with a respelled key
and opened by a version of the app that does not know the field gets
the key under its usual name, with the same notes.

## A letter for every note

`spelling()` only says how the black keys of the tonic are written.
A key's notes need more: D harmonic minor has a B♭ and a C♯, and F♯
major has an E♯, which is neither the sharp nor the flat name of any
black key. The rule is the one from [Pitch classes, notes and
spelling](pitch.md#letters-and-names): a seven-note scale
uses each letter once. `spell` gives a pitch class the letter of its
place in the scale, counted up from the tonic's letter, and lets
`NoteName::with_letter` find the sharps or flats:

```rust
pub fn spell(self, pitch_class: PitchClass) -> NoteName {
    if let Some(scale) = self.set_as_scale() {
        return scale.spell(pitch_class);
    }
    if !self.kind.is_tonal() {
        return NoteName::chromatic(pitch_class);
    }
    let name = NoteName::with_letter(
        pitch_class,
        self.tonic_name()
            .letter
            .up(i32::from(self.letters_above_tonic(pitch_class))),
    );
    if self.has_letter_for(pitch_class) {
        name
    } else {
        name.simplified()
    }
}
```

How many letters above the tonic a note goes is its place in the scale
for a seven-note scale. The pentatonic and blues scales leave letters
out, so they have a table of their own:

```rust
fn letter_steps(self) -> &'static [u8] {
    match self {
        ScaleKind::MajorPentatonic => &[0, 1, 2, 4, 5],
        ScaleKind::MinorPentatonic => &[0, 2, 3, 4, 6],
        ScaleKind::Blues => &[0, 2, 3, 4, 4, 6],
        ScaleKind::Chromatic | ScaleKind::Row(_) | ScaleKind::Set(_) => &CHROMATIC_LETTERS,
        _ => &[0, 1, 2, 3, 4, 5, 6],
    }
}
```

The blues scale's two fourth-letter entries are its "blue note", a flat
fifth next to the fifth: A blues is A C D E♭ E G, not A C D D♯ E G.

### Notes from outside the key

A note outside the scale has no place in it. It is written the way
musicians most often meet it, counted from the tonic: a flat second,
third, sixth or seventh, a sharp fourth, and the major sixth and
seventh, which in a minor key are the raised notes of melodic and
harmonic minor. That is a table of letters by semitones above the
tonic:

```rust
const CHROMATIC_LETTERS: [u8; 12] = [0, 1, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6];
```

One semitone above the tonic is one letter up (a flat second), six
semitones is three letters up (a sharp fourth), eleven is six letters
up (the major seventh). So in C major the black keys outside the key
are D♭, E♭, F♯, A♭ and B♭, and in D minor C♯ is C♯.

`letters_above_tonic` chooses between the scale's place and this table:

```rust
fn letters_above_tonic(self, pitch_class: PitchClass) -> u8 {
    let semitones = self.root.semitones_up_to(pitch_class);
    if self.has_letter_for(pitch_class)
        && let Some(place) = self
            .kind
            .tonal_intervals()
            .iter()
            .position(|&interval| interval == semitones)
    {
        return self.kind.letter_steps()[place];
    }
    CHROMATIC_LETTERS[semitones as usize]
}
```

The `if` joins a condition and a `let` with `&&`. This is a **let
chain**: the block runs only if the condition is true and the pattern
matches, and `place` is bound inside it. Without it, the code would
need an `if` inside an `if`, or a `match` on a tuple.

The table can ask for two signs. In D♭ major the note a semitone above
the tonic, D, is a flat second, the letter E: E𝄫. No musician writes
that for a note outside the key, so `spell` takes the simpler name, D,
for any note the scale has no letter for. Inside the scale two signs
stay, because the key needs them: G♯ harmonic minor's seventh note is
F𝄪.

A pitch-class set whose notes are exactly a tonal scale on its root is
spelled as that scale (`set_as_scale`), so E major shown as a set
keeps its G♯, C♯ and D♯. A row or any other set has no letters, so its
notes take the chromatic names.

### Names with octaves

`note_name` puts the spelled name together with the right octave
number, through [`Note::name_as`](pitch.md#the-octave-of-a-spelled-note):

```rust
pub fn note_name(self, note: Note) -> String {
    note.name_as(self.spell(note.pitch_class()))
}
```

C♯ harmonic minor raises its seventh, B, to B♯, so in that key MIDI
note 60, middle C, is "B♯3". The voice-leading check and the part
motion view name the notes they talk about this way ([Voice leading and
part motion](voice-leading.md)).

## Changing key: moving the song

The **Key** menus, left of the harmony keys and at the top of the piano
roll, set the key of the whole song, the one the harmony keys, the
hints and the band use. So they move the song too, the way a band plays
the same song in another key when asked: pick D and the chords, the
bass and the melody all go up a tone; pick minor and every note moves
to the same place in the minor scale. The drums stay as they are, and
**Ctrl+Z** puts everything back in one go.

As with every [editing tool](tools.md), the musical part is
a plain function that changes nothing itself. It answers one question:
where does this note go when the key changes from `from` to `to`?

```rust
pub fn change_key(note: Note, from: Scale, to: Scale) -> Note {
    let from = from_key(from, to);
    if !from.kind.notes_follow(to.kind) {
        return note;
    }
    let up = from.root.semitones_up_to(to.root) as i32;
    let shift = if up > 6 { up - 12 } else { up };
    let Some(moved) = note.transpose(shift) else {
        return note;
    };
    // The chromatic scale holds every note, so it has no degrees to
    // match with a key's: into it or out of it, notes only take the new
    // root. (Out of it, every note would otherwise be "of the old scale"
    // and be pulled into the new key, losing every sharp and flat.)
    if from.kind == ScaleKind::Chromatic || to.kind == ScaleKind::Chromatic {
        return moved;
    }
    // The old scale moved to the new root, so `moved` can be compared
    // with it degree by degree.
    let old = Scale::new(to.root, from.kind);
    let Some(degree) = old.degree_of(moved.pitch_class()) else {
        return moved;
    };
    let (old_steps, new_steps) = (from.kind.intervals(), to.kind.intervals());
    if old_steps.len() == new_steps.len() {
        let offset = new_steps[degree] as i32 - old_steps[degree] as i32;
        moved.transpose(offset).unwrap_or(moved)
    } else {
        nearest_in_key(moved, to)
    }
}
```

It works in two moves.

1. **A new root moves every note by the same distance.** C to D is two
   semitones up. C to G could be seven up or five down; `shift` takes
   the nearer one, so the song stays roughly where it was on the
   keyboard instead of jumping up a fifth.
2. **A new mode moves each note to the same degree.** After the first
   move the note is compared with the *old* kind of scale on the *new*
   root. If it is the third note of D major (F♯), it becomes the third
   note of D minor (F). The difference between the two interval lists
   at the same place is how far that degree moves:

   | degree        | 1 | 2 | 3  | 4 | 5 | 6  | 7  |
   |---------------|---|---|----|---|---|----|----|
   | major         | 0 | 2 | 4  | 5 | 7 | 9  | 11 |
   | natural minor | 0 | 2 | 3  | 5 | 7 | 8  | 10 |
   | moves by      | 0 | 0 | -1 | 0 | 0 | -1 | -1 |

   So C major to C minor lowers E, A and B, and leaves C, D, F and G.

Some notes need a rule of their own:

- A **note outside the old scale**, such as a note of a borrowed chord
  or of a part written in another key, has no degree, so the `let ...
  else` returns it after the first move only. It keeps its distance
  from the root, and a modulation stays a modulation.
- **Scales of different sizes**, like major (seven notes) and major
  pentatonic (five), have no matching degrees. A note that is not in
  the new scale goes to the nearest one that is, with `nearest_in_key`
  ([Editing tools as plain functions](tools.md)).
- The **chromatic scale** has every note, so going into it or out of it
  only moves the root.
- Between a key and a **row** or a **set**, there is nothing to move
  by, and `notes_follow` says so: only what the piano roll highlights
  changes. A set whose notes are exactly a key, such as C major shown as
  the set 7-35, moves on as that key (`from_key`).

`let Some(moved) = ... else { return note; };` is a **let-else**: if
the pattern does not match, the `else` block must leave the function.
Here it covers "the note would leave the MIDI range" and "the note is
not in the scale".

### The whole project

`Project::change_key` runs that function over every note that should
move. Its core loop:

```rust
let pitched: Vec<bool> = self.channels.iter().map(Channel::is_pitched).collect();
for (index, pattern) in self.patterns.iter_mut().enumerate() {
    let (old, new) = (before[index], after[index]);
    for (channel, _) in pitched.iter().enumerate().filter(|(_, p)| **p) {
        if pattern.is_step_row(channel) {
            continue;
        }
        let notes: Vec<NoteEvent> = pattern
            .notes(channel)
            .iter()
            .map(|note| NoteEvent {
                pitch: tools::change_key(note.pitch, old, new),
                ..*note
            })
            .collect();
        pattern.set_notes(channel, notes);
    }
}
```

- `pitched` is collected before the loop because the loop borrows
  `self.patterns` mutably, and Rust will not let the code look at
  `self.channels` through `self` at the same time. A small `Vec<bool>`
  read first ends that borrow early.
- `Channel::is_pitched` is false for the built-in drums and for a MIDI
  Out channel on the General MIDI drum channel: drums do not move.
- A row of step-sequencer steps (every note a one-step C4) is a sound
  played on the beat, not a tune, so it stays as it is.
- `before[index]` and `after[index]` are the keys each pattern is
  played in before and after the change, which are not always the
  song's key, as the next section shows.

The app calls it through `edit`, which keeps the project as it was
before ([Undo and redo](../program/undo.md)), so one **Ctrl+Z**
restores the key and every note:

```rust
self.edit("change key", |project| project.change_key(scale));
```

The chord sheet and the band's parts move with the notes; they are in
[The chord sheet: model, layout and text](../band/chord-sheet.md) and
[How the band plays a chord sheet](../band/band.md).

## Key changes partway through: key markers

Many pieces do not stay in one key. Mozart's Rondo alla Turca starts in
A minor, turns to A major at bar 25, where the key signature gets three
sharps, and goes back to A minor at bar 65. Moving to a new key is a
**modulation**. Three sharps are also the signature of F♯ minor, A
major's relative minor, and the A major part passes through F♯ minor
without changing its signature; you hear which key the music is in by
where it comes home, not by the signature.

A key change is one more marker on the playlist, next to the time
signature and tempo markers:

```rust
pub struct KeyMarker {
    /// The step it starts on, counted from the start of the song.
    pub start: usize,
    pub scale: Scale,
}
```

A key marker never moves a note. It changes what the key is called,
which rows the piano roll lights up, which chords the harmony keys
offer and which chords count as "in the key". That is the difference
from the Key menus, which move the song.

`KeyMap` answers "what key is step 300 in?": the song's own key until
the first marker, then each marker's key in turn. Building one puts the
markers in order and keeps one per step:

```rust
pub fn new(first: Scale, markers: impl IntoIterator<Item = KeyMarker>) -> Self {
    let mut markers: Vec<KeyMarker> = markers
        .into_iter()
        .filter(|marker| marker.start > 0)
        .collect();
    markers.sort_by_key(|marker| marker.start);
    markers.dedup_by_key(|marker| marker.start);
    KeyMap { first, markers }
}
```

`dedup_by_key` removes an item when it has the same key as the one
before it, so after sorting it keeps the first marker on each step.
Looking a step up takes the last marker that starts at or before it:

```rust
pub fn marker_at(&self, step: usize) -> Option<&KeyMarker> {
    self.markers
        .iter()
        .take_while(|marker| marker.start <= step)
        .last()
}
```

`take_while` stops at the first marker after `step`, and `last` gives
the one before it, or `None` if the step comes before every marker. A
`Project` builds its map with `key_map()`, and `key_at(step)` asks it.

Adding a marker keeps the list sorted with `binary_search_by_key`,
which gives `Ok(index)` if a marker already starts on that step and
`Err(index)`, where it would go, if not:

```rust
let markers = &mut self.playlist.key_markers;
match markers.binary_search_by_key(&marker.start, |m| m.start) {
    Ok(index) => markers[index] = marker,
    Err(index) => markers.insert(index, marker),
}
```

A marker on step 0 is not added at all: it changes the key the song
starts in.

### The key of a pattern

A pattern is written once and played wherever its clips are, and a song
can play it in different keys. The piano roll needs one key to light up
its notes, so a pattern takes the key where its first clip starts:

```rust
pub fn pattern_key_at(&self, pattern: usize, step: usize) -> Scale {
    self.playlist
        .clips
        .iter()
        .filter(|clip| clip.pattern == pattern)
        .map(|clip| clip.start)
        .min()
        .map_or(self.scale, |start| self.key_at(start + step))
}
```

`pattern_key(p)` is this at step 0. The piano roll says *This pattern:
A major* beside the Key menus, and the harmony keys say *Now in A
major* when the playhead is in a key that is not the song's own.

### When the song moves, the markers move

When the Key menus move a song with key changes, every key change must
move by the same interval, or the march of the Rondo would stay in A
major while the rest went to C minor. `moved_key` moves a key the way
the song's key moved:

```rust
pub fn moved_key(key: Scale, from: Scale, to: Scale) -> Scale {
    let from = crate::tools::from_key(from, to);
    if !from.kind.notes_follow(to.kind) {
        return key;
    }
    let up = from.root.semitones_up_to(to.root);
    let root = key.root.transpose(up as i32);
    let kind = if key.kind == from.kind && from.kind.is_tonal() && to.kind.is_tonal() {
        to.kind
    } else {
        key.kind
    };
    if root == key.root && kind == key.kind {
        return key;
    }
    Scale::new(root, kind)
}
```

The root moves by the same semitones. A key in the song's old mode
takes the new mode: a song in C major with a passage in G major, moved
to C Dorian, has that passage in G Dorian. A key in another mode keeps
its own: in a song in A minor moved up a minor third to C minor, the A
major march becomes C major and the F♯ minor episode A minor, as a
transposed score would read. Then `change_key` moves each pattern from
the key it was played in to the key it is played in now, which is what
`before` and `after` held above.

A key change can also be a micromode, or a set or a row the song is
in: see [Micromodes](micromodes.md#key-changes-in-any-colour).

MIDI files store key changes too, as key signature events; see [MIDI:
messages, devices, recording and files](../sound/midi.md). A marker
can also be written by the app itself, from the keys it hears in the notes
([Hearing harmony: chords and keys from
notes](hearing.md)).

## Snap to scale

The piano roll's **Snap to scale** button makes a note you draw or drag
land on the nearest note of the key, and its right-click menu adds
**Snap notes you play**, for the typing keyboard and a MIDI keyboard.
Both come down to `tools::nearest_in_key(note, scale)`, in [Editing
tools as plain functions](tools.md); the key it snaps to is
the key at that point of the song, after key markers. How a MIDI
keyboard's keys are snapped on the MIDI thread is in [MIDI: messages,
devices, recording and
files](../sound/midi.md#snapping-keys-to-the-key).

## Testing it

The tests in `scale.rs` state the rules for every key at once, not just
for the keys someone thought of. Every seven-note scale, on every root,
written either way, uses seven different letters, and each name still
names the right key of the piano:

```rust
let mut letters: Vec<_> = scale
    .pitch_classes()
    .map(|p| scale.spell(p).letter)
    .collect();
letters.sort();
letters.dedup();
assert_eq!(letters.len(), 7, "{}", scale.name());
```

The signs a key's notes are written with add up to its key signature,
for every mode on every root, so a staff never needs an accidental for
a note of the key:

```rust
let (signature, _) = scale.key_signature().unwrap();
let written: i8 = scale
    .pitch_classes()
    .map(|note| scale.spell(note).accidental)
    .sum();
assert_eq!(written, signature, "{}", scale.name());
```

And the signatures of all twelve major keys, from C upwards, are the
circle of fifths:

```rust
// Sharps count up, flats count down, round the circle of fifths:
// C, Db, D, Eb, E, F, F#, G, Ab, A, Bb, B.
let major = [0, -5, 2, -3, 4, -1, 6, 1, -4, 3, -2, 5];
```

`tools.rs` checks the moves of a key change: C to G goes down a fourth,
C major to C minor lowers E, A and B, a note outside the scale only
takes the new root, and a D7 taken to the chromatic scale and back is
still a D7. `key_map.rs` checks that the Rondo's march and episode land
in C major and A minor when the song goes up a minor third.

Exercises on keys are in [Exercises: music theory in
code](exercises.md).
