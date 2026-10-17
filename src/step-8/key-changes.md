# Key changes partway through, and the whole Turkish March

Files: [`crates/loom-core/src/key_map.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-core/src/key_map.rs),
[`crates/loom-core/src/project.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-core/src/project.rs),
[`crates/loom-core/src/library/demo.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-core/src/library/demo.rs),
[`crates/loom-core/src/library/song/electronic.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-core/src/library/song/electronic.rs)
and [`crates/loom-app/src/playlist/sections.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-app/src/playlist/sections.rs)

The question that started this: *"I don't see the modulation of the key
signature in the Turkish March demo. Isn't there a modulation in the
original?"* There is. Mozart's Rondo alla Turca, the last movement of
his Piano Sonata No. 11 (K. 331), changes its key signature three times:

| Bars | Key signature | Key |
|---|---|---|
| 1–24 | none | A minor (bars 9–16 pass through C major) |
| 25–32 | three sharps | A major: the loud "Turkish" march |
| 33–40 | three sharps | F♯ minor, ending on C♯ minor |
| 41–48 | three sharps | A major scales |
| 49–56 | three sharps | F♯ minor |
| 57–64 | three sharps | A major: the march again |
| 65–88 | none | A minor: the theme again |
| 89–128 | three sharps | A major: the finale and the coda |

The demo already played its march in A major, but you could not see it:
a project had one key, `Project::scale`, and the key shown never moved.
A part written with `.in_key(A, Major)` only changed how its chord
numerals were read. Now a song can change key, and the app shows it.

## Why the same signature can mean two keys

Three sharps (F♯, C♯, G♯) is the key signature of A major, and also of
F♯ minor, its *relative minor*: the minor key that starts on the sixth
note of the major scale and uses the same notes. So bars 25–64 keep one
signature while the music moves between A major and F♯ minor. You hear
the difference by where the music comes home. In bars 33–40 the bass
keeps returning to F♯, and the melody leans on E♯ (the raised seventh of
F♯ harmonic minor, the note just under F♯). The demo marks the key the
music is in, not just the signature, so the ruler says *F# harmonic
minor* from bar 33.

The C major bars in the theme (9–16) are different: the music visits C
major for a few bars without a new signature and without settling there.
That is called a *tonicization* rather than a modulation, so the demo
keeps A minor there and writes the chords as ♭III and ♭VII of A minor.

## A key marker

A key change is one more marker on the playlist, next to the time
signature and tempo markers:

```rust
pub struct KeyMarker {
    pub start: usize, // the step it starts on
    pub scale: Scale, // the key from there on
}
```

`KeyMap` answers "what key is step 300 in?" the way `Meter` answers
"which bar is it in?": the project's own key until the first marker,
then each marker's key in turn. `Project::key_at(step)` asks it, and
`Project::pattern_key(pattern)` gives the key where a pattern's first
clip starts, as `pattern_time` does for the time signature.

A key marker never moves a note. That is the difference from the Key
menus (see [Changing key](changing-key.md)), which transpose the whole
song. Changing the song's key now also moves every key marker by the same
interval, and each pattern moves from the key it is played in to where
that key went. Move the Turkish March from A minor up to C minor and the
march becomes C major and the episode A minor, as a transposed score
would read.

A song exported as a MIDI file gets a key signature event at the start
and at every change, so a notation program opening it shows the three
sharps appearing at bar 25.

Project files keep the markers in a `key_markers` list. A song with any
is saved as format 8; one without stays at the older format, so older
versions of Ferrous Loom can still open it.

## Seeing it in the app

- The ruler writes the key after the bar number at bar 1 and wherever it
  changes, in blue.
- Right-click the bar numbers: under the time signatures there is now
  **Key from bar N**, with a root and a scale to pick, and **Remove this
  key change**. Bar 1 is the song's own key, which the Key menus change.
- In song mode, the harmony keys follow the key at the playhead (or
  where Play will start). When it is not the key the song starts in,
  *Now in A major* appears under the Key menus, and the seven chord keys
  are that key's chords.
- The piano roll highlights a pattern's notes in the key where it plays,
  and says *This pattern: A major* beside the Key menus.

![The Turkish March at bar 27, in the march: the ruler says A major at bar 26 and F# harmonic minor at bar 34, and the harmony keys are A major's.](../images/step-8-key-change-march.png)

![Right-clicking the bar numbers: the time signature, then the key from that bar on.](../images/step-8-key-menu.png)

## The whole piece

The old demo played only the first 32 bars, and skipped the C major
bars. It now follows the score, transcribed from the Humdrum edition of
K. 331 (a free, machine-readable score): every section and every
repeat, written out in order, the first and second endings included, up
to the coda's last A major chords. Each bar of the demo holds two bars
of the score, which is in 2/4, so score bar 25 (the march) first comes
at demo bar 26. The right hand's top line is the melody, and the left
hand is read as chords. A few chords need a word:

- **Bar 20, F A D♯** is an *augmented sixth* chord: F and D♯ are an
  augmented sixth apart and pull outwards to E. It sounds like F7, which
  is how the chord chart writes it (♭VI7).
- **Bar 27, D♯ F♯ A** is a diminished chord on the raised fourth degree
  (♯iv°), leading to E: the march's bass climbs D, D♯, E.
- **Bars 38–40** end the F♯ minor episode in C♯ minor, its fifth degree
  made minor (v), through G♯7 (II7).

Grace notes (the quick notes before a beat) are left out, and the
trill in bar 23 is a plain note.
