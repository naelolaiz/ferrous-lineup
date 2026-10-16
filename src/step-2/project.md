# Patterns and the project

Files: [`crates/loom-core/src/pattern.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-core/src/pattern.rs)
and [`crates/loom-core/src/project.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-core/src/project.rs)

## The music

A **pattern** is a short loop, usually one bar. In 4/4 time a bar has four
**beats**, and the step sequencer splits each beat into four **steps**
(sixteenth notes), so a 16-step pattern is one bar. The demo beat is the
classic dance groove:

```text
step   1 . . . 2 . . . 3 . . . 4 . . .
Kick   X . . . X . . . X . . . X . . .   "four on the floor"
Clap   . . . . X . . . . . . . X . . .   beats 2 and 4 (the backbeat)
Hat    . . X . . . X . . . X . . . X .   between the beats (off-beats)
```

The step buttons alternate colors every four steps so you can see the
beats.

## Fixed-size arrays and type aliases

```rust
pub const MAX_STEPS: usize = 64;
pub type StepRow = [bool; MAX_STEPS];
```

`[bool; 64]` is an **array**: exactly 64 values, stored inline, with the
length part of the type. Unlike a `Vec`, it never allocates and cannot
grow. Every row has room for 64 steps even when the pattern is 16 long,
so changing the length never allocates, which matters because the audio
thread keeps copies of these rows. Arrays of `Copy` values are `Copy`
themselves, so sending a row to the audio thread just copies 64 bytes.

`type StepRow = ...` is a **type alias**: a second name for an existing
type, to make signatures easier to read.

## `Option` combinators

```rust
pub fn is_on(&self, channel: usize, step: usize) -> bool {
    self.rows
        .get(channel)
        .and_then(|row| row.get(step))
        .copied()
        .unwrap_or(false)
}
```

Indexing with `rows[channel]` **panics** if the index is out of range.
`.get(channel)` returns an `Option` instead, and the chain handles the
missing case without any `if`:

| Method | Does |
|--------|------|
| `get(i)` | `Some(&item)` or `None` if out of range |
| `and_then(f)` | if `Some(x)`, call `f(x)` which itself returns an `Option` |
| `copied()` | turn `Option<&bool>` into `Option<bool>` |
| `unwrap_or(v)` | the value, or `v` if `None` |

`toggle` shows the same idea with the `?` operator, which works on
`Option` too: `None` returns `None` from the function early.

```rust
pub fn toggle(&mut self, channel: usize, step: usize) -> Option<bool> {
    let cell = self.rows.get_mut(channel)?.get_mut(step)?;
    *cell = !*cell;
    Some(*cell)
}
```

`get_mut` gives a mutable reference, `&mut bool`, and `*cell` reads or
writes the value it points to (the `*` **dereferences** it).

## Enums with data model the instruments

```rust
pub enum Drum { Kick, Snare, Clap, HiHat }

pub enum SampleSource {
    Builtin(Drum),
    File(PathBuf),
}

pub enum ChannelKind {
    Sampler(SampleSource),
    Synth { note: Note },
}
```

Enums nest. A channel is either a sampler or a synth; a sampler's sound
is either a built-in drum or a file. Each fact is stored once and every
combination makes sense. `Synth { note: Note }` shows that a variant can
also have **named fields**, like a struct.

`PathBuf` is the standard type for an owned file path. It is not a
`String`, because file names on some systems are not valid UTF-8 text;
`path.display()` turns one into something printable.

`Drum::ALL` is an associated constant holding every variant, so menus can
loop over the drums instead of listing them by hand.

## Keeping an invariant with privacy

```rust
pub struct Project {
    pub tempo: Tempo,
    channels: Vec<Channel>,
    pattern: Pattern,
}
```

The project must keep one pattern row per channel, in the same order.
If both fields were public, any code could push a channel and forget the
row. So they are private, and the only ways to change them keep the rule:

```rust
pub fn add_channel(&mut self, channel: Channel) -> Option<usize> {
    if self.channels.len() >= MAX_CHANNELS {
        return None;
    }
    self.channels.push(channel);
    self.pattern.push_row();
    Some(self.channels.len() - 1)
}
```

A rule that is always true about a value is called an **invariant**, and
making the fields private is how Rust code protects one. `tempo` stays
public because every `Tempo` is valid by construction, so nothing can
break it.

`Pattern::push_row` is declared `pub(crate)`: visible everywhere inside
`loom-core`, but not to other crates. Only `Project` should add rows,
and other crates cannot get around it.

`channels()` returns `&[Channel]`, a **slice**, rather than `&Vec<Channel>`.
Callers can read and loop, but not push, and the `Vec` could be swapped
for another storage later without changing them.
