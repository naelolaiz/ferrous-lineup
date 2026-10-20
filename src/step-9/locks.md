# Locking a track: styles play around it

Files: [`crates/lineup-core/src/lock.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/lock.rs),
[`crates/lineup-core/src/band/locks.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/locks.rs),
[`crates/lineup-core/src/library/apply.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/apply.rs)
and [`crates/lineup-app/src/channel_rack.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/channel_rack.rs)

Say you played a piano part yourself, or picked a nylon guitar for the
chords, and now want to hear the song as a bossa nova. Before, picking
the style rewrote every track it plays: your piano part was replaced
by the style's comping, and your guitar went back to the style's
instrument. Now you can **lock** a track first. A locked track keeps
its notes and its instrument, and the style plays the rest of the
song around it.

![The channel rack: a small padlock between each channel's knob and its name. Kick's padlock is lit and closed; Clap, Hat, Snare and Lineup Osc have faint, open ones. The hint reads "Lock: styles keep its notes and sound".](../images/step-9-lock-rack.png)

## Trying it

1. Open a demo, for example **Ode to Joy**, and show the rack with
   **F6**.
2. Click the small padlock between **Keys**'s knob and its name. It
   closes and turns orange. Right-clicking the name has **Lock** and
   **Unlock** too, and **Ctrl+Z** undoes it.
3. In the **Library**, apply another style, say **Bossa nova**. The
   drums and the bass change; the Keys part is the one you had. The
   notice ends with **🔒 Kept: Keys.**, and the Library's list of where
   each part came from has a **Locked** row.
4. Try styles one after another in pattern mode. Each trial copies the
   locked rows into the trial pattern, so the new bass follows the
   locked chords.
5. Unlock Keys again. If the band was playing, it takes the piano part
   back, and the notice says your Keys is muted so the two do not play
   over each other.

What moves the *whole* song still moves a locked track with it: a new
time signature puts its notes in the new bars, and a key change moves
them to the new key. A lock protects a part from styles, not from the
song changing shape. You can always edit a locked track by hand.

## How it works

### One bit per channel

The rack has at most 32 channels, so the set of locked ones fits in
one `u32`, one bit for each channel:

```rust
pub struct Locked(u32);

// One bit for each channel there can be.
const _: () = assert!(MAX_CHANNELS <= u32::BITS as usize);

impl Locked {
    pub fn contains(self, channel: usize) -> bool {
        channel < MAX_CHANNELS && self.0 & (1 << channel) != 0
    }
}
```

`Locked(u32)` is a **tuple struct**: a new type wrapped around a `u32`,
reached with `.0`. Wrapping it means nobody can add two sets by
mistake or pass a channel count where a set is meant. The `const _`
line is a check the compiler runs: if someone raised `MAX_CHANNELS`
above 32, the build would fail right there instead of a lock silently
landing on the wrong channel.

Removing a channel renumbers the ones after it, so the set must shift
too. `without` keeps the bits below the removed channel and moves the
bits above it down by one:

```rust
let below = self.0 & ((1 << index) - 1);
let above = self.0.checked_shr(index as u32 + 1).unwrap_or(0) << index;
Locked(below | above)
```

`checked_shr` returns `None` instead of overflowing when the shift is
32 or more, which happens when the last channel is removed.

### One door for every style

Styles write into patterns from many places: a whole style, a rhythm,
a saved style, a row or micromode style, a fill, the final chord. If
each of them had to remember to skip locked channels, one would
forget. So the library no longer gets a `&mut Pattern` at all. It gets
a `StylePattern`, which only writes rows that are not locked:

```rust
pub(crate) struct StylePattern<'a> {
    pattern: &'a mut Pattern,
    locked: Locked,
}

impl StylePattern<'_> {
    pub(crate) fn set_notes(&mut self, channel: usize, notes: impl IntoIterator<Item = NoteEvent>) -> bool {
        !self.locked.contains(channel) && self.pattern.set_notes(channel, notes)
    }
}

impl Deref for StylePattern<'_> {
    type Target = Pattern;
    fn deref(&self) -> &Pattern {
        self.pattern
    }
}
```

`Deref` lets a `StylePattern` be read exactly like a `Pattern`
(`pattern.length()`, `pattern.notes(channel)`), while writing only
goes through its own methods. This is a common Rust shape: a
**wrapper** that hands out reading for free and guards writing. The
instrument has the same kind of door, `Project::set_sound`, which
refuses a locked channel. A test applies every style, rhythm, saved
style, row and micromode style with each channel locked in turn, and
checks that the locked one never changes.

### The band plays around a lock

The band writes its own patterns, a part per player, so it has its own
rules (in `band/locks.rs`):

- a player whose channel is locked is skipped: its part stays as it is;
- locked drum channels keep their notes while the other drums change;
- if your own Keys or Bass is locked and plays, the band's piano or
  bass stays silent, so the song's part is not doubled;
- when the band leaves (**No band**), a locked band part stays in the
  song as your own.

The band keeps a fingerprint of what it plays from, to know when to
write again. The lock counts in that fingerprint only when something is
locked, so songs saved before this change sound exactly as they did.
A song with a locked channel is saved as format 23.

## Exercises

1. Lock **Keys** in Ode to Joy and apply **Doo-wop**. Which tracks
   changed? Open Keys in the piano roll: is anything different?
2. Lock **Kick** in the starting song and apply **Bossa nova**. Look at
   the drums: what kept its old rhythm, and what follows the bossa
   nova?
3. (Rust) `Locked` derives `Copy`. Why can `contains` take `self` by
   value instead of `&self`, and why is that cheap here?
4. (Rust) `StylePattern` holds `&'a mut Pattern`. What would the borrow
   checker say if the library kept a `StylePattern` and then called
   another `Project` method that changes patterns?
