# A second bug hunt: crashes, spelling and the beat

Files: [`crates/lineup-core/src/library/comping.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/comping.rs),
[`crates/lineup-core/src/pitch.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/pitch.rs),
[`crates/lineup-core/src/harmony/scale.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/scale.rs),
[`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-core/src/band/form.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/form.rs),
[`crates/lineup-core/src/ear_training.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/ear_training.rs),
[`crates/lineup-core/src/lead_sheet/rhythm.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/lead_sheet/rhythm.rs),
[`crates/lineup-core/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/chord_sheet.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs),
[`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs)
and [`crates/lineup-app/src/app/practice.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/practice.rs)

With the thirteen items of the plan done, the question was again the
opposite of a feature: *what can be improved, and what is broken?* This
chapter goes through the eighteen bugs that were fixed, one commit
each, and why each was there. As in the [first bug
hunt](../step-8/bug-hunt.md), the interesting part is rarely the fix,
which is often one line, but the reason the line was missing.

## How the hunt was done

The app ran on a virtual screen (`Xvfb`) and was driven with `xdotool`,
as before. Each part of the code got a different question:

- the band: does every style play every kind of bar, including N.C.,
  pushes and odd meters?
- the theory: is every name the app writes (notes, chords, numerals,
  key signatures) the one a musician would write?
- the playback: does every edit made while playing leave the sound and
  the playhead where they should be?
- the windows: does every key go to the window you are looking at?

A bug counted as found when it was **seen**: reproduced in the running
app, or by a test on the code as it was. Each fix then came with a test
that fails without it. That is worth doing even for a one-line fix: the
test is what stops the same bug from coming back the next time the code
around it changes.

## Crashes and lost work

### N.C. closed the app

Type N.C. in a bar with the **Pop ballad** band, and the window closed.
Sixteen styles did it: every one whose piano or bass plays the chord
**one note at a time** (arpeggios, the Alberti bass, salsa's montuno).
Those patterns ask for "the note number *n* of the chord", and go up
an octave and around when *n* is bigger than the chord:

```rust
Voices::Nth(index) => {
    let octave = (index / voicing.len()) as i32;
    voicing[index % voicing.len()]
```

N.C. is a harmony with no notes, so `voicing.len()` is 0, and
`index / 0` is a division by zero. In Rust an integer division by zero is not a
strange number, as it can be elsewhere: the program **panics** (stops
with an error), because there is no right answer to give. The fix is a
*match guard*, an `if` on a pattern, placed before the general case:

```rust
Voices::Nth(_) if voicing.is_empty() => Vec::new(),
```

Match arms are tried in order, so an empty voicing never reaches the
`%`. A test now plays a bar of N.C. in **every** band style and checks
that no piano, bass or guitar note starts in it.

### A double sharp could not be opened again

In G# harmonic minor the seventh degree is F double sharp (the raised
seventh of G# minor: F# raised once more), and the chord on it is
**F##dim7**: F## A# C# E. The app wrote that name when saving, but the
note reader only read **one** sharp or flat, so opening the file read
"F#" and then "#dim7", which is not a chord, and the file would not
open. Nobody types F##, but the app does, so it must read what it
writes. `NoteName::read` now takes up to two signs, as long as they go
the same way:

```rust
let mut accidental = 0;
for _ in 0..2 {
    let mut chars = rest.chars();
    let step = chars.next().map_or(0, sign);
    if step == 0 || (accidental != 0 && step != accidental) {
        break;
    }
    accidental += step;
    rest = chars.as_str();
}
```

`rest` is a `&str`, a view into the text, and `chars.as_str()` is the
part not read yet; moving `rest` forward reads nothing twice and copies
nothing. "F#b" stops after the sharp (a sharp and a flat cancel out,
and nobody writes them together), and "Ebbb" reads Ebb and leaves a
"b". A test now writes every letter with every accidental from bb to
\#\# into a chord name, with slash basses, and reads it back.

### The chord sheet's keys moved the piano roll's notes

With the chord sheet open over the piano roll, Right arrow moved every
note of the piano roll one step, and the selected bar did not move.
Both windows listen to the arrow keys, Delete and Ctrl+A, and the piano
roll decided it had the keyboard by checking whether the click was
inside its own rectangle. A click on the chord sheet *above* it is
inside that rectangle too.

egui draws in **layers**, and can say which one is on top at a point:
`ctx.layer_id_at(pos)`. So the piano roll now asks that:

```rust
if let Some(pos) = press {
    match ctx.layer_id_at(pos) {
        Some(layer) if layer.order > egui::Order::Middle => {}
        layer => {
            self.has_keyboard = layer.is_some_and(|layer| layer.id == Self::id());
        }
    }
}
```

Windows are in the `Middle` order; menus and tooltips are above it, in
`Foreground` and `Tooltip`. A click on a menu changes nothing (the menu
was opened from somewhere, and that window keeps the keyboard), a click
on the piano roll's own layer gives it the keyboard, and a click on any
other window takes it away.

### Deleting a channel froze the band

The band knows which parts you edited by keeping a **fingerprint** of
each part it wrote: a hash of every note, including the number of the
channel it is on. A part whose notes no longer match was edited by you,
and is left alone ("frozen"). But deleting a channel numbers every
channel after it one lower, so every part after it no longer matched,
and the whole band stopped following the chord sheet.

The project already had a way to say "this change moves the band's
notes, but it is not an edit": `moving_the_band`, used when the song
changes key or time signature. It notes which parts were following the
sheet, runs the change, and takes the new fingerprints as the band's
own. Deleting a channel now goes through a cousin of it,
`removing_channel`, which does the same for every part **that played
nothing on the deleted channel**. The part that did play there lost
those notes, so it stays frozen, which is right: you took them away.

The fingerprints are saved in the file, so the other way of fixing it,
hashing something other than the channel number, would have made every
band in every saved song look edited the first time it was opened. The
fix chosen keeps the fingerprints as they were.

### Another song got the bar you were typing in

Typing in a bar of the chord sheet and pressing **Ctrl+N** (or opening
a file, a demo or an import) kept the text field open over the new
song, and Enter then wrote the old text into it. When the project is
replaced, the chord sheet now forgets its selection and what was being
typed (`ChordSheet::new_song`), as the piano roll already did.

## Wrong theory

### F Locrian has six flats

F Locrian is the seventh mode of Gb major: **F Gb Ab Bb Cb Db Eb**. Its
key signature is Gb major's, six flats. The app wrote it with six
sharps, the signature of F# major, because it chose sharps or flats by
the *sound* of the related major key, and F# and Gb are the same keys
of the piano. All the other modes of that key are named with sharps (G#
Dorian, B Lydian, D# minor), so the rule is right for them; F Locrian
is the one mode whose tonic, F, is a natural that only fits among
flats. A test now checks, for every mode on every tonic, that the
sharps and flats the scale's notes are spelled with add up to its key
signature.

### Numerals follow the letter

In C major, C#°7 and Db7 sound on the same piano key, but they are
different chords in the music:

- **C#°7** (C# E G Bb) is a raised tonic, **#i°7**: it leads up a
  semitone to Dm, as a passing chord between I and ii.
- **Db7** (Db F Ab Cb) is **bII7**: the "tritone substitute" of G7,
  sliding down a semitone to C.

The chord sheet's captions read the degree from the sound, so both came
out on the second degree ("bii°7", "bII7"). The degree is a matter of
**letters**: C# is on the letter C, the first degree, raised; Db is on
D, the second, lowered. `Scale::spelled_numeral` now counts letters
from the tonic's name to the chord's written root, and the captions and
tooltips use it. In G# harmonic minor, F##dim7 is vii°7: F is the
seventh letter from G.

### The band's intro is spelled from the first chord

The intro is a turnaround into the first chord, VI, iiø, V7: into Ebm
it is **Cbmaj7 Fm7b5 Bb7**. The app wrote **Bmaj7 E#m7b5 A#7**, the
same sounds spelled in D# minor, because the key's tonic is named by
its usual name (D# minor) and the intro was spelled in it. A song that
starts on Ebm is written in flats, so the intro is now written from the
first chord's letter.

### bVII to IV: the root rises a fifth

The hint for bVII to IV said "the root falls a fifth". From Bb up to F
is five letters (Bb C D E F) and seven semitones: a fifth **up** (or a
fourth down). It is a plagal move, like IV to I (F up to C is also a
fifth), and I, bVII, IV, I is the "double plagal" cadence of a lot of
rock: two plagal steps in a row. The hint now says so.

### Chromatic and back keeps every note

Picking the chromatic scale as the key and then going back to C major
took the sharps and flats out of the song: D7 (D F# A C) became Dm7.
Moving notes from one scale to another takes each note of the old scale
to the same degree of the new one (from C major to C minor, E goes down
to Eb). The chromatic scale has every note, so coming back from it,
every note was "of the old scale", was matched to a degree of C major,
and was pulled into it. The chromatic scale has no degrees to match, so
going into it or out of it now only moves the notes to the new root.

### A row or a set as the key moves no key changes

Making a twelve-tone row or a pitch-class set the key moves no notes
(it is another way of naming them, as the [chapter on sets as a
key](../step-8/key-as-a-set.md) explains). But the song's key markers
did move: a G major section of a C major song went up to B with the
row's first note. The key markers now follow the same rule as the
notes (`notes_follow`): when the notes stay, the key changes stay too.

## Practice and learning

### Ear training gave the next question away

Each question draws its notes from a small random number generator
(SplitMix64) started at a **seed**, and the next question's seed was
the generator's next internal state. So the second question drew the
same numbers as the first, shifted by one: its first draw (the chord's
kind) was the last question's second draw (its root). After C, E or Ab
the next chord was always major. Nothing looked wrong on any one
question; only a test that asked thousands in a row, and counted what
came after a C, showed it.

The next seed is now one of the generator's **outputs**, which are
mixed far from its state:

```rust
pub fn next_seed(seed: u64) -> u64 {
    Random(seed).next()
}
```

The lesson is general: a seed must not be something the generator
itself will reach a step later.

In a minor key, the progression questions also said "the chords after
I". The tonic chord of a minor key is minor, written in lower case,
**i**; the question now names the tonic as the key does.

### Tap tempo counts the clicks

In 6/8 the metronome clicks the **dotted quarter**, two clicks a bar,
but tap tempo took every tap as a quarter note. Tapping along with the
clicks of a song at ♩ = 120 (a dotted quarter at 80 a minute) set the
tempo to 80. Tap tempo now takes the length of the beat the metronome
clicks, in quarter notes:

```rust
let quarters = time.steps_per_bar() as f64 / beats as f64 / f64::from(STEPS_PER_BEAT);
```

and the tempo is `60 × quarters × taps / seconds`. In 6/8 a beat is 1.5
quarters, so taps 0.75 seconds apart give 120. In 3/8 the clicks are
eighths, half a quarter, which used to double the tempo; in 5/8 and 7/8,
whose beats are of two lengths, it is the average beat.

### Rests in 6/8

A bar of 6/8 with an eighth note and then silence for the rest of the
first beat was written "♪ 𝄽": an eighth and a quarter rest. In compound
time a beat is a dotted quarter, three eighths, and the rule of
engraving is that the notation shows where each beat's thirds are. A
quarter rest may cover the **first** two eighths of a beat, but not the
last two, so the right writing is "♪ 𝄾 𝄾", two eighth rests. The lead
sheet now follows that rule in 6/8, 9/8 and 12/8.

## Playback

### The slow blues plays four beats of three

The 12/8 slow blues ("Trouble in Mind", and the Slow blues style) has
four beats a bar, each a dotted quarter split in three eighths: the
triplet feel of the blues ballad. Its bass played six notes a bar, one
every quarter: three notes against every two beats, which crosses the
beats instead of marking them.
The style has two new patterns for it:

- the bass plays **long-short** on each beat, a quarter then an eighth,
  the short note leading to the next beat;
- the piano strikes the chord on **every eighth**, three to a beat, the
  first of each three the strongest.

This was checked by sound: the bass alone was rendered to audio, and
its notes start on each beat and on each beat's last third, eight a
bar.

### Switching pattern and song mode let go of no notes

Switching between pattern and song mode while playing restarted the
sequencer, but sent no note-offs, so a pad or a note sent to a MIDI
synthesizer kept sounding. The engine now lets go of every held note
when the mode changes while it plays, as it already did after a seek.

### Removing a loop kept the playhead's place

While song mode loops bars 5 to 8, the playhead counts from the loop's
start. Removing the loop with its **Looping bars 5–8 ✖** button set the
playhead to that count from bar 1: stopped at bar 7, it went to bar 3.
The practice bar's loop tick box already added the loop's start back;
the playlist's button now goes the same way.

### A pushed N.C. gained a dot

"N.C." has dots in its name, and the dots after a harmony are a stop
(C. is a shot). The reader knew that for N.C., but not for a pushed
**^N.C.**: it read the last dot as a shot, and wrote it back as
"^N.C..". Each copy and paste added a dot. The carets of a push are now
taken off first, and what is left is checked for N.C. before any dots
are read.

## What is left

Some things that were found change what you would hear or see, so
they wait for a decision: a pushed chord at a section change is struck
twice, a text chord sheet stretches a section to the next heading, and
a few smaller ones. They are listed, with the rest of the findings, in
the bug hunt's notes in the project files.

## Exercises

1. `Voices::Nth(n)` wraps with `n % voicing.len()`. Write a function
   `pick(voicing: &[u8], n: usize) -> Option<u8>` that returns `None`
   for an empty voicing instead of panicking, using `slice::get` and
   `checked_rem`.
2. Spell the seventh chord on each degree of G# harmonic minor. Which
   ones need a double sharp?
3. In Db major, what are the Roman numerals of D°7 and of Ebb7? Count
   the letters from Db.
4. Write I, bVII, IV, I, the double plagal cadence, in G major and in
   E major.
5. Tap tempo in 9/8: how many quarters is a beat, and what tempo do
   taps one second apart give?
