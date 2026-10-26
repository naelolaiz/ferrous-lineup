# Before the next release: keys with two names, notes that stop, and damaged files

Files: [`crates/lineup-core/src/harmony/scale.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/scale.rs),
[`crates/lineup-core/src/library/song.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song.rs),
[`crates/lineup-app/src/twelve_tone.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/twelve_tone.rs),
[`crates/lineup-app/src/midi.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/midi.rs),
[`crates/lineup-app/src/app/trace.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/trace.rs)
and [`crates/lineup-core/tests/damaged_files.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/tests/damaged_files.rs)

Before a release is a good time to look for weak spots. The search
turned up three: a few keys had only one of their two names, **Restart
audio** left notes sounding on an external synth, and nothing tested
what a damaged file does to the app. This page covers all three. The
first is about music theory, and the other two are about hardening.

## Keys with two names

On the piano, F# and G♭ are the same black key. That makes **F# major**
and **G♭ major** the same seven keys of the piano, written two ways:

| Key | Its notes | Key signature |
|---|---|---|
| F# major | F# G# A# B C# D# E# | six sharps |
| G♭ major | G♭ A♭ B♭ C♭ D♭ E♭ F | six flats |

They sound the same. The composer picks the name that fits the music
around it. Golliwogg's Cakewalk is in E♭ major, three flats. Its middle
section moves to the key a minor third up, and Debussy writes that key
as G♭ major, adding three more flats. As F# major, the page would switch
from flats to sharps in the middle of the piece, and every chord would
be renamed. Take Five is written in **E♭ minor**, six flats, for the same
reason: its tune moves through G♭, D♭ and B♭.

Until now Ferrous Lineup gave every key one name. Its rule (see
[Notes spelled right](../step-8/bug-hunt.md)) picks flats for the keys
whose major signature has flats and sharps for the rest. That rule chose
F# major and D# minor, so the app showed Golliwogg's middle section in F#
major and Take Five in D# minor.

### Which keys have two names

Going round the circle of fifths, each key adds one sharp or one flat.
Seven sharps (C# major) and seven flats (C♭ major) are the most a key
signature holds, so there are **fifteen** major keys, from seven flats
to seven sharps, on twelve keys of the piano. Three keys of the piano
are therefore counted twice:

| Usual name | Other name | Signatures |
|---|---|---|
| B major | C♭ major | 5 sharps, or 7 flats |
| F# major | G♭ major | 6 sharps, or 6 flats |
| D♭ major | C# major | 5 flats, or 7 sharps |
| G# minor | A♭ minor | 5 sharps, or 7 flats |
| D# minor | E♭ minor | 6 sharps, or 6 flats |
| B♭ minor | A# minor | 5 flats, or 7 sharps |

The modes go with the major key whose signature they share: G# Dorian
has the notes of F# major, so it can also be A♭ Dorian. Every other
key would need eight sharps or flats or more. D# major would need nine
sharps, with F## and C## in it, so it is always written E♭ major.

### In the app

The key menus in the piano roll and under the harmony keys now list the
fifteen keys for the kind of scale you chose: for major, C, C#, D♭, D,
E♭, E, F, F#, G♭, G, A♭, A, B♭, B and C♭. For minor, the list has D# and
E♭, and A# and B♭.

![The key menu for major, in two halves: C, C#, Db, D, Eb, E, F, F#, then Gb, G, Ab, A, Bb, B and Cb.](../images/step-9-fifteen-keys.png)

Picking the other name of the same key **renames it and moves no
note**. The chord sheet is written again in the new name: F#, D#m7, B
and C#7 in F# major become G♭, E♭m7, C♭ and D♭7 in G♭ major. The
key-change menu on the playlist's bar numbers offers both names too.

Golliwogg's Cakewalk now shows its middle section in G♭ major, with the
chords of that key:

![The piano roll on Golliwogg's Cakewalk's "G flat" pattern: "This pattern: Gb major, Gb Ab Bb Cb Db Eb F", and the chords in Gb major: I Gb, ii Abm, iii Bbm, IV Cb, V Db, vi Ebm, vii° Fdim.](../images/step-9-g-flat-major.png)

The other places a key comes from keep its name as well:

- **MIDI files.** A key signature of six flats opens as G♭ major, and
  six sharps as F# major. Saving writes the signature from the key, so
  G♭ major goes out with six flats.
- **Chord sheets as text.** `Key: Gb major` stays G♭ major, and the text
  the app writes says `Key: Gb major` again. A name no key signature
  has, such as `Gb minor` (it would need a double flat), is read as the
  usual key, F# minor.
- **A key shown as a pitch-class set** ([A key as a pitch-class
  set](../step-8/key-as-a-set.md)) keeps its flats: G♭ major as the set
  7-35 still writes G♭ A♭ B♭ C♭ D♭ E♭ F.

### A field only the scale can set

The key is a `Scale`. It gets one more field:

```rust
pub struct Scale {
    pub root: PitchClass,
    pub kind: ScaleKind,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    respelled: bool,
}
```

`respelled` says the key is written the other way round from its usual
name. It is the only field without `pub`. In Rust, privacy is per
**module**, not per type, so code outside `scale.rs` can still read
`scale.root` but can no longer write `Scale { root, kind, respelled:
true }`. It has to ask the scale instead:

```rust
impl Scale {
    /// The same key written the other way round, if it can be.
    pub fn other_spelling(self) -> Option<Scale> { /* ... */ }

    /// The key written with sharps or with flats, when it can be either.
    pub fn with_spelling(self, spelling: Spelling) -> Self { /* ... */ }

    /// The key on `tonic` as it is written: Gb major, or F# major.
    pub fn written(tonic: NoteName, kind: ScaleKind) -> Self { /* ... */ }
}
```

All three check first that the key has a second name. A "respelled C
major" therefore cannot exist, and two scales that are equal really are
the same key with the same name. `other_spelling` returns an `Option`
because most keys have no other name, and the type makes every caller
deal with that.

`Scale::new` became a `const fn`, so constants such as
`const C_MAJOR: Scale = Scale::new(PitchClass::C, ScaleKind::Major);`
work even though the field is private.

The tonic's name comes from the next letter: F# becomes G♭, B becomes
C♭, D♭ becomes C#. The rest of the scale needs no new code. The note
spelling from [Notes spelled right](../step-8/bug-hunt.md) already gives
each note the letter of its place in the scale, counting from the
tonic's letter. With G as the tonic's letter, the fourth note is a C,
so B becomes C♭.

The serde attributes keep the files as they were. `default` reads a
file without the field as `false`. `skip_serializing_if` leaves the
field out when it is `false`, so a key under its usual name is saved
exactly as before, as `(root: FSharp, kind: Major)`. A G♭ major song
saves `(root: FSharp, kind: Major, respelled: true)`. An older Ferrous
Lineup skips the field it does not know, names the key F# major and
plays the same notes. Nothing it could play is lost, so this needs no
new format number ([Project file format](../appendix-project-format.md)).

### The demos follow their tunes

Each demo song names its key as a pitch class: `tonic: DSharp, mode:
NaturalMinor` for Take Five. Adding a spelling field to every song would
touch hundreds of them. The songs already say how they are written,
though: the tunes are typed in from the scores, flats and all
(`Eb4:12 F4:1 Gb4 F4 Eb4 Db4:4`). So the library counts them:

```rust
fn written_as_tune<'a>(key: Scale, parts: impl IntoIterator<Item = &'a Part>) -> Scale {
    let (sharps, flats) = parts
        .into_iter()
        .map(|part| accidentals(part.melody))
        .fold((0, 0), |(sharps, flats), (s, f)| (sharps + s, flats + f));
    match sharps.cmp(&flats) {
        Ordering::Greater => key.with_spelling(Spelling::Sharps),
        Ordering::Less => key.with_spelling(Spelling::Flats),
        Ordering::Equal => key,
    }
}
```

`impl IntoIterator<Item = &'a Part>` lets the same function take one
part (`[part]`, an array) or all the parts in the song's key (an
iterator with a `filter`). `accidentals` reads each note name with
`NoteName::read` and counts its sharps or flats. It only reads letters
at the start of a name, so the `b` in `Bb4` is not counted as a note B.

The count only matters for keys with two names. It changed three
demos: Golliwogg's Cakewalk (the middle section, now G♭ major), Take
Five (E♭ minor) and the E♭ minor part of Desarma y sangra. Every other
demo keeps its name: the stretto of In the Hall of the Mountain King
stays in F# major, Maple Leaf Rag's trio in D♭ major and the Moor's
waltz in Petrushka in B major, because their tunes are written that way.

### One test for names and signatures together

A key's name and its key signature must agree, and so must the spelled
notes. One test checks all of them together. For each of the seven
diatonic scales (major, natural minor and the modes) on each of the
twelve roots, and under both names where there are two, it adds up the
accidentals of the spelled notes. The sum must equal the key signature:

```rust
let (sharps, _) = key.key_signature().unwrap();
let written: i32 = key
    .pitch_classes()
    .map(|p| i32::from(key.spell(p).accidental))
    .sum();
assert_eq!(written, i32::from(sharps), "{}", key.name());
```

G♭ major spells G♭ A♭ B♭ C♭ D♭ E♭ F, six flats, and its signature
is −6. E# Locrian, the other name of F Locrian, spells six sharps and has
the signature +6. If the name, the letters or the signature were wrong
anywhere, the sums would disagree.

## Restart audio: notes that stop, and a trace that goes on

**Edit > Settings… > Audio > Restart audio** opens the sound card again
([Losing nothing](recovery.md#audio-that-comes-back)). Two things went
wrong when it did.

**Notes kept sounding on an external synth.** A note sent over MIDI Out
gets its note-off from the audio thread that played it. Restarting
drops that audio thread, and the note-offs it still owed went with it.
A long note on a hardware synth would ring until you stopped it there.

The messages go out through a small thread, `midi-out`, that empties the
audio thread's queue every millisecond. The fix keeps that thread's
`JoinHandle` instead of a `bool`. When its audio thread is gone (the
queue is "abandoned"), the thread's loop ends, and before it finishes
it sends **All Notes Off** (controller 123) on all sixteen channels:

```rust
let sender = move || {
    while !messages.is_abandoned() {
        // ... send what the audio thread queued ...
    }
    // The engine is gone, and with it the note offs of notes still
    // sounding on the synth.
    if let Some(connection) = lock(&output).as_mut() {
        for channel in 0..16 {
            let _ = connection.send(&all_notes_off(channel));
        }
    }
};
```

The new audio thread gets a new sending thread. It must not start
before the old one has sent its All Notes Off, or that message could
cut off the first notes of the new engine. So `reconnect_audio` waits
for the old thread to finish, polling `JoinHandle::is_finished`. It
waits at most 100 ms, and in practice about a millisecond, because the
old engine is already gone. It does not call `join()`, which could wait
for ever on a MIDI port that hangs. The window must never freeze over a
port.

**The playback trace stopped.** Edit > Settings… > Playback > Trace writes the
notes the audio thread plays ([A playback trace](playback-trace.md)).
The audio thread hands them over through a queue, and the new audio
thread had none. Now `resume_trace` writes what the old one left,
makes a new queue, and sends it to the new audio thread. The trace goes
on in the same file.

## Damaged files never crash the app

A project file can be cut short by a full disk, edited by hand, or be
something else under a `.lineup` name. MIDI files come from every
program there is. Reading a file must end in a project or in an error
message, never in a crash. A new test file checks this the way a
**fuzzer** does: it damages good files in many small ways and reads each
result.

The damage comes from a tiny random number generator, xorshift, written
in the test:

```rust
struct Damage(u64);

impl Damage {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}
```

It starts from a fixed seed, so every run damages the files the same
way, and a failure shows up again on the next run. It needs no new
dependency. For project files and chord sheets, it replaces numbers
with extreme ones (`-1`, `65535`, `18446744073709551615`, `NaN`), cuts
pieces out, and repeats pieces. For MIDI files, it changes, cuts and
repeats bytes. Each test reads 600 damaged copies. A project that reads
must also save and read back.

The test catches a panic with `std::panic::catch_unwind`, so a failure
prints the damaged file that caused it:

```rust
fn must_not_panic<T: std::fmt::Debug>(input: T, read: impl FnOnce(&T)) {
    if panic::catch_unwind(AssertUnwindSafe(|| read(&input))).is_err() {
        panic!("reading this damaged file panicked:\n{input:?}");
    }
}
```

It found one bug. A file whose key change sat at step
18446744073709551615, the largest `usize`, failed validation as it
should. Writing the error message then computed `marker.start + 1` to
say "step …", and that overflowed. Rust treats overflow differently in
the two build profiles. In a **debug** build, `+` panics on overflow
("attempt to add with overflow"). In a **release** build it wraps
around to 0. A first run in release mode over 15,000 damaged files saw
nothing. The same files in debug mode crashed. The messages now use
`saturating_add(1)`, which stops at the largest number instead of
overflowing. This is also why CI runs the tests in debug mode ([Tests
and continuous integration](../step-1/ci.md)).

## Trying it

1. Open the **Golliwogg's Cakewalk** demo, pick the **G flat** pattern
   and open the piano roll (F7). It says "This pattern: Gb major" and
   lists the chords G♭, A♭m, B♭m, C♭, D♭, E♭m and Fdim.
2. Open **Take Five**. The key menus say E♭ minor.
3. In a new song, open the key menu under the harmony keys and pick
   **F#**. The song moves up to F# major. Now pick **G♭**: no note moves,
   but the harmony keys say G♭, A♭m, B♭m, C♭…, and any chords on the
   chord sheet are renamed.
4. Save the song with **File > Export song as MIDI…** and open the file
   with **File > Import MIDI file…**: it is still in G♭ major.
5. Turn on **Edit > Settings… > Playback > Trace**, play a song, and press
   **Restart audio** while it plays. The trace file goes on after the
   restart. With an external synth on MIDI Out, a long note stops when
   you restart (this needs real MIDI hardware).
6. Run the damaged-file tests:

   ```sh
   cargo test -p lineup-core --test damaged_files
   ```

## Exercises

1. Pick **A# minor** and then **harmonic minor** in the key menu. The
   raised seventh is written **G##**, a double sharp, and the harmony
   keys show its chord as G##dim. Why does B♭
   harmonic minor, on the same keys of the piano, need no double sharp
   or flat? Which name would you write a piece in?
2. `written_as_tune` keeps the usual name when a tune has as many sharps
   as flats. Write a test with a made-up part in D♭ major whose tune is
   `C#4 Db4`. Which name does it get? Where would you change the rule
   to prefer the song's own key?
3. The damaged-file test replaces numbers with `NaN`. Find what happens
   when a project file says `tempo: NaN`. Is it read, and where does the
   tempo end up?
