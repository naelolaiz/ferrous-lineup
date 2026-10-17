# The accompaniment follows your chords

Files: [`crates/lineup-core/src/library/follow.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-core/src/library/follow.rs),
[`crates/lineup-app/src/app/follow.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-app/src/app/follow.rs)
and [`crates/lineup-app/src/app/live.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-app/src/app/live.rs)

An **arranger keyboard** (the home keyboards with a "style" button)
plays drums, chords and a bass line for you, and your left hand only
tells it *which chord*. Hold C and the band plays in C; move to A minor
and the same groove carries on in A minor. It is a fast way to learn
harmony: you choose the chords, and you hear at once what each one does
to the song.

FL Studio has no such mode built in. Its forum has people asking for
one, and the nearest thing it has is the arpeggiator of a channel, which
turns one held chord into an arpeggio. Ferrous Lineup goes further, and
does what the arranger keyboard does with any song.

## How to use it

1. Open the **MIDI** menu and turn on **Accompaniment follows chords**.
   To play with the letter keys instead of a MIDI keyboard, turn on
   **Typing keyboard to piano** too.
2. Play a pattern or the song (a demo, or a style from the Library).
3. Hold a chord. From the next note they play, the chords and the bass
   line move into it, keeping their rhythm. The status bar says which
   chord the accompaniment is in.

![The MIDI menu, with the new option](../images/step-8-follow-menu.png)

![Holding A and C: the accompaniment plays A minor](../images/step-8-follow-status.png)

Like arranger keyboards, Ferrous Lineup does not need the whole chord:

| Keys held | Chord |
|-----------|-------|
| one key, such as G | its major chord: G |
| two keys a minor third apart, such as D and F | the minor chord on the lower one: Dm |
| two keys a major third or a fifth apart | the major chord on the lower one |
| two keys a minor seventh apart, such as G and F | the seventh chord: G7 |
| three keys or more | the chord they make, any of the 28 Ferrous Lineup knows |

The last chord stays when you let go, as the "memory" button of an
arranger keyboard does, so you can play a chord and take your hand
away. Letting go of the keys one by one does not count as playing the
keys left: only a key going *down* asks for a new chord. When you stop,
the song plays its written chords again.

The song itself never changes. Ctrl+Z, saving and the WAV export see
the notes as they were written. The moved notes go to the audio thread
only.

## Moving notes into a chord

`library/follow.rs`

Before it can move anything, Ferrous Lineup needs to know which chord
each note was written for. That is the same problem as [A new rhythm
for a song](new-rhythm.md), and it uses the same answer: `hear_chords`
listens to each pattern and cuts it into stretches, each with a chord.
This takes a moment, so it is done once, when the first chord is held,
and kept in an `Accompaniment` until the project changes:

```rust
pub struct Accompaniment {
    patterns: Vec<Heard>,
}
```

Then every note of a stretch heard as, say, C major is moved into the
held chord, say A minor. Arranger keyboards have a table for this,
written by whoever made the style. Ferrous Lineup works it out from what
each note *does* in its chord, its **role**:

```rust
enum Role {
    Root,
    Third,
    Fifth,
    Seventh,
    Tension,
}
```

- **Block chords**, three notes or more starting together, are voiced
  again with `voice` from `voicing.rs`, the same function the library
  uses to write styles, passing the old notes as the "previous" chord.
  So the new chord sits where the old one was, and the hand hardly
  moves.
- **Single notes**, as in arpeggios and bass lines, keep their role:
  the root of C (C) goes to the root of A minor (A), the third (E) to
  the third (C), the fifth (G) to the fifth (E). A seventh goes to the
  new seventh, or to the root if the new chord has none.
- **Notes outside the chord**, such as the passing notes of a walking
  bass, move as far as the root does: C to A is three semitones down,
  so a passing D becomes a B.

Each moved note lands in the octave nearest to where it was. That is
what keeps a bass line low and an arpeggio the same shape:

```rust
fn nearest(pitch_class: u8, near: Note) -> Note {
    let near = i32::from(near.midi());
    let up = (i32::from(pitch_class) - near).rem_euclid(12);
    // Up to a tritone up, or else down.
    let midi = if up <= 6 { near + up } else { near + up - 12 };
    Note::new(midi.clamp(0, 127) as u8)
}
```

`rem_euclid` is the remainder that is never negative: `-3 % 12` is
`-3` in Rust, but `(-3i32).rem_euclid(12)` is `9`, which is what "how
far up to the next A" means.

## Sending the notes, not changing the song

`app/follow.rs`

The audio thread already has a command to replace the notes of one
channel in one pattern, `Command::SetNotes`. Every edit uses it. Chord
follow uses it too, but without touching `self.project`:

```rust
for moved in followed {
    self.send(Command::SetNotes {
        pattern: moved.pattern,
        channel: moved.channel,
        notes: moved.notes,
    });
}
```

The audio thread swaps the list in between two blocks of sound, so the
next note it starts is already in the new chord. Notes already sounding
ring out as they were.

The app remembers which channels it moved (`moved`). To put them back,
it sends the project's own notes for those channels. That happens when
playback stops, when the option is turned off, and, just before moving
them again, after an edit: `sync_audio` in `edit.rs` sends the edited
notes as written, then calls `follow_project_changed`, which hears the
patterns again and moves them into the chord being followed.

Only a key going down asks for a new chord. `key_played` in `live.rs`
calls `follow.key_down()` on each Note On, and the check reads the flag
with `std::mem::take`, which returns it and leaves `false` behind:

```rust
if !std::mem::take(&mut self.live.follow.key_went_down) {
    return;
}
```

## How it was tested

- Unit tests: keys are read as chords; notes keep their role; for every
  one of the 78 styles, the chord channel plays only notes of the held
  chord (E minor, B♭7 and Fmaj7 were tried); and for every demo song,
  every note start of the chords and bass is still played after
  following a chord.
- In the running app, with the typing keyboard (the test machine has no
  MIDI device, and both keyboards go through `key_played`), the sound
  was recorded and the notes measured: the "Four chords" style went from
  C, G, Am and F to only D, F♯ and A after holding D; to A, C and E
  after holding A and C, and stayed there after letting go; London
  Bridge in song mode went to F♯, A♯ and C♯ after holding F♯; and Stop
  then Play brought the written chords back.

## Exercises

1. Arranger keyboards often have a **split point**: only keys below it
   choose the chord, so the right hand can play a melody. Add one, say
   below C4, as a second option in the MIDI menu.
2. The table for two keys is small. Add two keys a major second apart as
   a sus2 chord. Which tests need a new case?
3. Notes already sounding keep their old pitch until they end, so a
   chord held for a whole bar changes only at the next bar. Arranger
   keyboards play the new chord at once. What would the audio thread
   need to know to do that?
