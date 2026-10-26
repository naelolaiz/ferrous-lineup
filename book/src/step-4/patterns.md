# Many patterns, and a playlist to arrange them

Files: [`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs),
[`crates/lineup-core/src/playlist.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/playlist.rs)
and [`crates/lineup-core/src/pattern.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/pattern.rs)

## The shape of a project

In step 3 a project had one pattern. Now it has a list of them, and a
playlist that says where each one plays:

```rust
pub struct Project {
    pub tempo: Tempo,
    pub scale: Scale,
    channels: Vec<Channel>,
    patterns: Vec<Pattern>,
    playlist: Playlist,
    // ...time signature, mixer, band and the later chapters' fields
}
```

Every pattern still has one row of notes per channel. Pattern-based
studios work this way: a pattern is "what every instrument plays during
these bars", so the drums pattern simply has empty rows for the synth.

The playlist is two lists:

```rust
pub struct Track {
    pub name: String,
    pub muted: bool,
    // ...a display height, which older files leave out
}

pub struct Clip {
    pub pattern: usize,
    pub track: usize,
    pub start: usize,
    /// How many times the pattern plays, one after the other.
    pub repeats: usize,
}

pub struct Playlist {
    pub(crate) tracks: Vec<Track>,
    pub(crate) clips: Vec<Clip>,
    // ...sections, a loop region, and time, tempo, key and chord markers
}
```

A clip is three numbers and a repeat count: which pattern, on which
track, starting at which step of the song, and how many times the pattern
plays. Notice what is *not* there: the clip's length. It is the pattern's
length times the number of repeats, so it is worked out when needed:

```rust
pub fn clip_length(&self, clip: &Clip) -> usize {
    self.patterns.get(clip.pattern).map_or(0, Pattern::length) * clip.repeats
}

impl Clip {
    /// A clip that plays its pattern once.
    pub fn new(pattern: usize, track: usize, start: usize) -> Self {
        Clip { pattern, track, start, repeats: 1 }
    }
}
```

Storing the length too would mean two places that must agree. Make the
pattern longer and every clip's stored length would be wrong. A value
that can be computed from others is usually better computed than stored.
The same goes for the end of a repeated clip: it is `start +
clip_length`, not another field.

Step 4's clips always played their pattern once, and the audio thread
received the clips themselves. Step 8 added repeats, and now
`Project::playback_arrangement` unrolls them into `Play`s before the song
reaches the audio thread. The chapter on [playing a
song](song-playback.md#starting-notes-in-song-mode) has the current
snippet; the clip is still the thing you draw, and its `repeats` is the
little "×2" on it.

`map_or(0, Pattern::length)` says "if there is a pattern, call
`Pattern::length` on it; otherwise use 0". A method can be passed by name,
like a function, when its only argument is `self`.

## Three rules, one gatekeeper

The comment on `Project` lists the rules it keeps:

- every pattern has one row of notes per channel, in the same order;
- there is always at least one pattern;
- every clip in the playlist refers to a pattern and a track that exist.

Each rule is about two lists at once, so no single list can keep it. The
`Project` owns all of them, keeps the fields private, and changes them
only in methods that keep every rule. Adding a channel adds a row to
every pattern:

```rust
pub fn add_channel(&mut self, channel: Channel) -> Option<usize> {
    if self.channels.len() >= MAX_CHANNELS {
        return None;
    }
    self.channels.push(channel);
    for pattern in &mut self.patterns {
        pattern.push_row();
    }
    Some(self.channels.len() - 1)
}
```

Removing a pattern is the interesting one. Clips refer to patterns by
their position in the list, so when pattern 1 goes, the clips of pattern
1 must go too, and the clips of patterns 2, 3, 4... must each move down
by one so they keep playing the same music:

```rust
pub fn remove_pattern(&mut self, index: usize) -> Option<Pattern> {
    if index >= self.patterns.len() || self.patterns.len() == 1 {
        return None;
    }
    self.playlist.clips.retain(|clip| clip.pattern != index);
    for clip in &mut self.playlist.clips {
        if clip.pattern > index {
            clip.pattern -= 1;
        }
    }
    Some(self.patterns.remove(index))
}
```

- `retain` keeps the items for which the closure returns `true`, and
  drops the others, in one pass.
- `for clip in &mut self.playlist.clips` gives a mutable reference to
  each clip in turn, so `clip.pattern -= 1` changes the clip in the list.
- The method returns the removed pattern, wrapped in `Some`. The caller
  can ignore it, keep it for an undo feature later, or check that the
  removal happened at all.

The test `removing_a_pattern_renumbers_its_followers` checks the whole
story: after removing "Beat", the five Beat clips are gone, "Chords" is
pattern 0, and the fill still plays at bar 8.

Why use positions at all, instead of giving each pattern a permanent id?
Ids would make removal simpler (nothing to renumber) but everything else
harder: every lookup would search for an id instead of indexing a `Vec`.
For a list of at most 999 patterns (usually far fewer), positions plus
one careful method are the simpler choice. It is the kind of trade-off
with no single right answer; the
[last exercise](exercises.md#7-think-about-it) of this step asks you to
think about it.

## Handing out `&mut Pattern` safely

The app needs to change notes in a pattern. `Project` could copy every
`Pattern` method (`toggle`, `add_note`, `replace_note`...) as a method of
its own, as step 3 did with the single pattern. Instead it hands out the
pattern itself:

```rust
pub fn pattern_mut(&mut self, index: usize) -> Option<&mut Pattern> {
    self.patterns.get_mut(index)
}
```

Is that safe? With `&mut Pattern`, could the app add a row and break the
"one row per channel" rule? No, because of how the row methods are
declared in `pattern.rs`:

```rust
pub(crate) fn push_row(&mut self) { ... }
pub(crate) fn remove_row(&mut self, channel: usize) { ... }
```

`pub(crate)` means "public inside this crate only". `project.rs` can call
them, because it is in `lineup-core`; `lineup-app` cannot, even with a
`&mut Pattern` in its hands. Every method `lineup-app` *can* call keeps the
row count as it is. Privacy in Rust is about modules and crates, not
about who holds a reference, and that lets one type give out a
reference to another without giving away its rules.

On the app side, every note edit goes through one small helper:

```rust
fn edit_notes(&mut self, channel: usize, edit: impl FnOnce(&mut Pattern) -> bool) {
    let changed = self.project.pattern_mut(self.pattern).is_some_and(edit);
    if changed {
        self.send_notes(channel);
    }
}

// For example:
RackAction::ToggleStep { channel, step } => {
    self.edit_notes(channel, |pattern| pattern.toggle(channel, step).is_some());
}
```

- `impl FnOnce(&mut Pattern) -> bool` accepts any closure that takes a
  pattern and says whether it changed something. `FnOnce` is the most
  accepting of the three closure traits: the closure only has to be
  callable once, so it may even move values it captured (the piano roll's
  `AddNotes` closure moves its `notes` vector into the loop).
- `is_some_and(edit)` calls `edit` on the pattern if there is one, and
  gives `false` if there is not.

## Names, and `impl Into<String>`

Patterns now have names. The constructor takes the name first:

```rust
pub fn new(name: impl Into<String>, channels: usize, length: usize) -> Self {
    Pattern {
        name: name.into(),
        length: length.clamp(1, MAX_STEPS),
        rows: vec![Vec::new(); channels],
    }
}
```

`impl Into<String>` accepts anything that can be turned into a `String`:
`Pattern::new("Beat", 5, 16)` with a `&str`, or
`Pattern::new(format!("Pattern {n}"), 5, 16)` with a `String`, without
the caller writing `.to_string()`. Inside, `name.into()` does the
conversion (for a `String`, it just moves it).

`name` is a public field while `length` and `rows` are private. Mixing is
fine in Rust: `name` has no rule to keep, so there is no reason to hide
it behind methods.

Cloning a pattern uses the `Clone` derive that `Pattern` has had since
step 2:

```rust
pub fn clone_pattern(&mut self, index: usize) -> Option<usize> {
    let mut copy = self.patterns.get(index)?.clone();
    copy.name = format!("{} copy", copy.name);
    self.push_pattern(copy)
}
```

`.clone()` copies the whole pattern, every row and every note, so the
copy and the original never share anything; the test
`cloned_patterns_are_independent` checks that changing one leaves the
other alone. The `?` after `get(index)` returns `None` from the function
straight away if there is no such pattern.

## One fewer duplicate

Saving files (two chapters on) found a hole in step 3's rules. `add_note`
refused to add a note in the same place as another, but `replace_note`,
used when dragging, did not check, so you could drag one note exactly on
top of another. The new `replace_note` checks every note except the one
being replaced:

```rust
let duplicate = row
    .iter()
    .enumerate()
    .any(|(other_index, other)| other_index != index && other.same_place(&note));
```

Now "no two notes of the same pitch start at the same moment" is true for
every pattern, and loading a file can insist on it. Step 8 later gave
notes a `nudge` inside their step, for imported and loose playing;
`same_place` compares the pitch, the step and the nudge, so two notes that
start a few ticks apart are still two notes.

## How long is the song?

The song ends where its last clip ends, rounded up to a whole bar:

```rust
pub fn song_length(&self) -> usize {
    let clips = self
        .playlist
        .clips
        .iter()
        .map(|clip| clip.start + self.clip_length(clip));
    let sections = self.playlist.sections.iter().map(Section::end);
    let end = clips.chain(sections).max().unwrap_or(0);
    self.meter().round_up_to_bar(end)
}
```

`max` returns an `Option`, because an empty list has no maximum;
`unwrap_or(0)` turns "no clips" into a length of 0. `chain` joins the two
iterators, because a song can end with a clip or with a section that has
no clip at its very end. `self.meter()` knows the time signature, and its
`round_up_to_bar` is division that rounds up: in 4/4, 17 steps is
`17.div_ceil(16) = 2` bars.

Step 8 added sections and time signatures; in step 4 the last line was a
plain `round_up_to_bar(end)` over the clips alone. The rule is the same:
the song ends where its last piece ends, on a bar line.

## Where is a pattern, while the song plays?

In song mode the audio thread reports a step of the *song*. The channel
rack and the piano roll show one pattern, so they need to know which step
of *that* pattern is playing, if any:

```rust
pub fn pattern_step_in_song(&self, pattern: usize, song_step: usize) -> Option<usize> {
    self.playlist
        .audible_clips()
        .filter(|clip| clip.pattern == pattern)
        .find_map(|clip| self.step_in_clip(&clip, song_step))
}

/// The step of its pattern the clip plays on song step `step`.
fn step_in_clip(&self, clip: &Clip, step: usize) -> Option<usize> {
    let pattern_length = self.patterns.get(clip.pattern).map_or(0, Pattern::length);
    let end = clip.start + pattern_length * clip.repeats;
    (clip.start..end)
        .contains(&step)
        .then(|| (step - clip.start) % pattern_length)
}
```

Read it as a sentence: of the clips that can be heard, take those of this
pattern, find the first one that covers this song step, and tell me how
far into the clip that step is. The `% pattern_length` is what makes a
repeated clip start its pattern again each time round: a 16-step clip
that repeats at song step 16 is on pattern step 0 there. In step 4 every
clip played once, so the `%` was not there. `audible_clips` skips clips
on muted tracks:

```rust
pub fn audible_clips(&self) -> impl Iterator<Item = Clip> + '_ {
    self.clips
        .iter()
        .copied()
        .filter(|clip| self.tracks.get(clip.track).is_some_and(|track| !track.muted))
}
```

`.copied()` turns an iterator of `&Clip` into one of `Clip`. That works
because `Clip` is `Copy` (three numbers), and it means callers get plain
values instead of references into the playlist.

## The demo song

`Project::demo` builds the song with the same methods the app uses:

```rust
for bar in [0, 2, 4, 6] {
    project.add_clip(Clip::new(chords, chord_track, bar * 16));
}
for bar in 2..7 {
    project.add_clip(Clip::new(beat, drum_track, bar * 16));
}
project.add_clip(Clip::new(fill, drum_track, 7 * 16));
```

`2..7` is a range from 2 up to, but not including, 7: bars 3 to 7 when
counting from 1, as the ruler does. The fill takes the place of the beat
in bar 8. A new project (File > New) starts from `Project::template`
instead: the same five channels, one empty pattern and an empty playlist.
