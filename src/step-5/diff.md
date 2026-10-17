# Telling the audio thread what changed

Files: [`crates/lineup-core/src/diff.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/diff.rs),
[`crates/lineup-app/src/app/edit.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/edit.rs)
and [`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs)

## The problem undo brings

Since step 2, the audio thread has kept its own copy of everything it
plays: each channel's instrument, volume and notes, the pattern lengths,
the song. The UI keeps it up to date with commands. Until now, each kind
of edit sent its own: a click on a step sent `SetNotes` for that channel,
deleting a pattern sent `RemovePattern` and then `SetSong` with the
renumbered clips, and so on.

Undo does not fit that scheme. It replaces the whole project with an
older one, and the older one can differ in anything. There were three
ways to tell the audio thread about it:

1. **Send the whole project again**: `Clear`, then every pattern and
   channel, as when a file is opened. Simple, but every instrument would
   be built again, cutting off every note that is ringing, and a sample
   file would be read again for a one-step undo.
2. **Remember, for each step, the commands that undo it.** That is the
   command pattern from the previous chapter, coming back through the
   side door, with all the hand-written reverses we avoided.
3. **Compare the project before and after**, and send only what differs.

Ferrous Lineup does the third, and then goes one step further: *normal*
edits use the same comparison too. No edit sends its own commands any
more. This is what every change in the app looks like now:

```rust
pub(super) fn edit<R>(
    &mut self,
    label: &'static str,
    change: impl FnOnce(&mut Project) -> R,
) -> R {
    let before = self.project.clone();
    let result = change(&mut self.project);
    if self.project != before {
        self.sync_audio(&before);
        // `before` is moved into the history here, after its last use.
        self.history.record(label, before);
    }
    result
}
```

and undo:

```rust
pub(super) fn undo(&mut self) {
    let before = self.project.clone();
    if self.history.undo(&mut self.project).is_some() {
        self.sync_audio(&before);
    }
}
```

Having one path pays off twice. There is only one piece of code that
translates project changes into commands, so it can be tested thoroughly
on its own. And it is used on every click, so if it had a mistake, you
would hear it while editing, not only in the rare moment you undo
something unusual.

### Reading `edit`

`edit` takes a **closure** that changes the project, and the closure gets
the project as its argument:

```rust
self.edit("move clip", |project| project.move_clip(index, track, start));
```

Why not let the closure reach `self.project` itself? Because `edit`
already holds `&mut self`, and a closure that borrowed `self` too would
be a second borrow of the same value while the first is still in use. The
borrow checker refuses that. Handing the closure `&mut self.project` as
an argument lends it exactly the part it needs, for exactly as long as
the call lasts.

The closure's type is `impl FnOnce(&mut Project) -> R`. **`FnOnce`** is
the most generous of the closure traits: a closure that can be called at
least once. That allows closures that *move* something into the project,
such as `|project| project.add_channel(channel)`, which gives `channel`
away and so could not be called twice. `edit` only calls it once, so it
asks for nothing more.

`R` is whatever the closure returns, and `edit` passes it back to the
caller. Many `Project` methods return something: `add_clip` returns the
new clip's index as an `Option<usize>`, `remove_pattern` the pattern
it removed. Because `R` can be any type, a closure can simply be
the method call, `|project| project.add_clip(clip)`. If `edit` asked for
`impl FnOnce(&mut Project)`, which must return `()`, each of those would
need braces and a semicolon to throw the value away:
`|project| { project.add_clip(clip); }`. No caller uses the value yet,
but the day one needs it, it is there.

`self.project != before` compares the two projects completely (the
derived `PartialEq` compares every field, every list, every note). It
runs once per edit, not once per frame. If nothing changed, such as a
clip dragged back onto the same cell or a note deleted where there was
none, nothing is sent and no undo step is recorded.

## `diff`: the list of changes

The comparison lives in `lineup-core`, which knows projects but nothing
about the audio thread. It describes differences in the project's own
words:

```rust
pub enum Change {
    RemovePattern(usize),
    InsertPattern(usize),
    RemoveChannel(usize),
    InsertChannel(usize),
    Instrument(usize),
    Volume(usize),
    Muted(usize),
    PatternLength(usize),
    Notes { pattern: usize, channel: usize },
    Tempo,
    Song,
}

pub fn diff(before: &Project, after: &Project) -> Vec<Change>
```

The changes are steps to follow **in order**, and each number is a
position at the moment its change is applied. After `RemovePattern(0)`,
the pattern that used to be number 1 is number 0, and any later change
calls it 0. Values (a volume, a list of notes) are not in the `Change`:
they are read from the newer project when the change is turned into a
command.

Only what makes a sound is compared. Pattern names, channel names, track
names and the key give no change at all, because the audio thread never
sees them. Muting a track does give one: it takes that track's clips out
of the song, so the clips the audio thread plays change.

### Lining up two lists

The hard part is the lists. Say the patterns were `[Beat, Chords, Fill]`
and now are `[Beat, Fill]`. Comparing item by item would say pattern 1
changed from Chords to Fill and pattern 2 disappeared, which is true but
clumsy: it would send all of Fill's notes again. What really happened is
"pattern 1 was removed".

`Alignment` finds that. It counts how many items are the same at the
start of both lists, and how many at the end. What is left in the middle
was removed from the old list and inserted in the new one:

```text
  before:  Beat │ Chords │ Fill        start = 1 (Beat is the same)
  after:   Beat │        │ Fill        end   = 1 (Fill is the same)
                                       removed = 3 - 1 - 1 = 1
                                       inserted = 2 - 1 - 1 = 0
```

```rust
fn new<T: PartialEq>(before: &[T], after: &[T]) -> Self {
    if before.len() == after.len() {
        return Alignment { start: 0, removed: 0, inserted: 0 };
    }
    // The same at the start...
    let start = before
        .iter()
        .zip(after)
        .take_while(|(old, new)| old == new)
        .count();
    // ...and the same at the end, without counting an item twice.
    let room = before.len().min(after.len()) - start;
    let end = before[start..]
        .iter()
        .rev()
        .zip(after[start..].iter().rev())
        .take_while(|(old, new)| old == new)
        .count()
        .min(room);
    Alignment {
        start,
        removed: before.len() - start - end,
        inserted: after.len() - start - end,
    }
}
```

The iterator chain reads well once you know the pieces: `zip` walks two
lists side by side, giving pairs; `take_while` keeps pairs while they are
equal and stops at the first that is not; `count` says how many were
kept. `rev` walks the lists backwards, for the end.

`.min(room)` is easy to miss and important. With `[A, A]` becoming `[A]`,
the start finds one equal item, and so does the end, *the same `A`*. Both
counts together would say two items are kept out of a list of one, and
`after.len() - start - end` would be `1 - 1 - 1`, which for a `usize` is
not -1 but a panic ("attempt to subtract with overflow"). Limiting the end
to what is left after the start keeps the arithmetic honest.

When both lists are as long, nothing was added or removed, so items are
simply compared position by position. That covers the most common
edits: a click on a step changes one pattern, in place.

`Alignment` works for any `T: PartialEq`, so the same code lines up
patterns and channels. Two patterns are equal only if their names,
lengths and every note are equal, so a pattern that is the same at both
ends really is the same pattern.

### `fn` pointers: enum variants are functions

Once two lists are lined up, the changes are: remove the middle of the
old list, then insert the middle of the new one.

```rust
fn changes(
    self,
    remove: fn(usize) -> Change,
    insert: fn(usize) -> Change,
) -> impl Iterator<Item = Change> {
    let removals = (self.start..self.start + self.removed).rev().map(remove);
    let insertions = (self.start..self.start + self.inserted).map(insert);
    removals.chain(insertions)
}
```

and it is called like this:

```rust
patterns.changes(Change::RemovePattern, Change::InsertPattern)
```

`Change::RemovePattern` is not a value here, it is a **function**. Every
enum variant that holds data in parentheses is also a function that
builds it: `Change::RemovePattern` has the type `fn(usize) -> Change`. The
lowercase `fn(usize) -> Change` is a **function pointer** type: the
address of a plain function, with no captured variables. A closure that
captures nothing can be one too. `map(remove)` calls it for every
number, building `RemovePattern(2)`, `RemovePattern(1)` and so on.

Removals go from the **last** one backwards. Removing pattern 2 first
leaves patterns 0 and 1 where they were, so `RemovePattern(1)` still
means the right pattern. Going forwards, removing pattern 1 first would
move pattern 2 down to 1, and the second removal would hit the wrong one.
Insertions go forwards for the same reason: once position 1 is filled,
position 2 is right after it.

`chain` joins two iterators into one: first all of `removals`, then all
of `insertions`. The function returns `impl Iterator<Item = Change>`,
"some iterator of changes", without naming its long real type
(`Chain<Map<Rev<Range<usize>>, fn(usize) -> Change>, ...>`).

### The order of everything

```rust
pub fn diff(before: &Project, after: &Project) -> Vec<Change> {
    let patterns = Alignment::new(before.patterns(), after.patterns());
    let channels = Alignment::new(before.channels(), after.channels());

    let mut changes: Vec<Change> = patterns
        .changes(Change::RemovePattern, Change::InsertPattern)
        .chain(channels.changes(Change::RemoveChannel, Change::InsertChannel))
        .collect();
    ...
```

Patterns come first, then channels, then everything inside them. The
order matters because of what the audio thread does with each change:

- An inserted **pattern** arrives in the audio thread with no notes in any
  channel; its notes follow as `Notes` changes.
- An inserted **channel** arrives complete: its instrument, volume, and
  its notes in *every* pattern. So the patterns must already be right
  when it comes, or it would bring notes for the wrong number of patterns.

Then, for every channel that existed before (`old_index` says where it
was, or `None` if it is new), the instrument, volume and mute are
compared:

```rust
for (index, channel) in after.channels().iter().enumerate() {
    let Some(old) = channels.old_index(index).map(|old| &before.channels()[old]) else {
        continue; // a new channel arrives complete
    };
    if old.kind != channel.kind {
        changes.push(Change::Instrument(index));
    }
    ...
}
```

`let ... else` (from step 2) either takes the value out of the `Some`, or
runs the `else` block, which must leave: here `continue` jumps to the
next channel.

Then the notes, for every pattern and every old channel:

```rust
for (index, pattern) in after.patterns().iter().enumerate() {
    let old_pattern = patterns.old_index(index).map(|old| &before.patterns()[old]);
    if let Some(old) = old_pattern
        && old.length() != pattern.length()
    {
        changes.push(Change::PatternLength(index));
    }
    for channel in 0..after.channels().len() {
        let Some(old_channel) = channels.old_index(channel) else {
            continue; // a new channel brought its notes with it
        };
        // A new pattern starts with no notes in the audio thread.
        let old_notes = old_pattern.map_or(&[][..], |old| old.notes(old_channel));
        if old_notes != pattern.notes(channel) {
            changes.push(Change::Notes { pattern: index, channel });
        }
    }
}
```

The `if let ... && ...` is a **let chain** (step 2 again): the block runs
only if the pattern is old *and* its length changed. `&[][..]` is an
empty slice: `[]` is an empty array, and `[..]` takes all of it as a
slice, so both arms of `map_or` give a `&[NoteEvent]`. A new pattern is
compared with "no notes", which is exactly what the audio thread has for
it, so a `Notes` change is sent only for the channels that have notes in
it.

Last come the tempo and the song. The song is compared through the clips
that can be heard:

```rust
let same_clips = before
    .playlist()
    .audible_clips()
    .eq(after.playlist().audible_clips());
```

`audible_clips()` returns an iterator, not a list. **`Iterator::eq`**
compares two iterators item by item and stops at the first difference,
without collecting either into a `Vec` first.

### When the story is ambiguous

Say there are three Lineup Osc channels with the same settings but
different notes, and you delete the middle one. Comparing the *channels*
alone (name, instrument, volume, mute), the old list is `[Osc, Osc, Osc]`
and the new one `[Osc, Osc]`, and the alignment cannot tell which one
went: it finds two equal at the start and says the *last* was removed.

That is fine. The notes of the second channel now differ (it holds what
used to be the third channel's notes), so a `Notes` change follows, and
the audio thread ends up with exactly the right channels and notes. The
diff does not need to guess what you really did. It needs a list of
steps that ends in the same place, and a short one where it can.

## Testing with a model

How do you test a function like `diff`? Checking its exact output for
every case would be brittle, and would not prove the output is *right*.
What we really want to know is: *if the audio thread follows these
changes, does it end up with the new project?*

So the tests build a small **model** of what the audio thread knows: the
pattern lengths, each channel's instrument kind, volume, mute and notes
per pattern, the tempo, and the song. It is simple enough to trust at a
glance:

```rust
struct Model {
    lengths: Vec<usize>,
    channels: Vec<ModelChannel>,
    tempo: Tempo,
    clips: Vec<Clip>,
    song_length: usize,
}
```

`Model::apply` does to the model what the audio thread does for each
change, and `check` puts it together:

```rust
/// Diffs two projects, checks that following the changes turns the
/// first into the second, and returns the changes.
fn check(before: &Project, after: &Project) -> Vec<Change> {
    let changes = diff(before, after);
    let mut model = Model::of(before);
    for &change in &changes {
        model.apply(change, after);
    }
    assert_eq!(model, Model::of(after), "changes: {changes:?}");
    changes
}

/// Checks both directions, as undo and then redo would.
fn check_both_ways(before: &Project, after: &Project) -> Vec<Change> {
    check(after, before);
    check(before, after)
}
```

Every test checks this property first, and only then, where it matters,
the exact changes (that a step click gives a single `Notes`, that undoing
a deleted pattern starts with `InsertPattern(0)`). The last test makes a
mess on purpose, removing a pattern, adding a channel, toggling a step,
changing a volume and the tempo all at once, and also diffs an empty
project against the demo song, as opening a file would.

This kind of test is called **property-based**: it checks a rule that
must hold for any input, rather than one answer for one input. Exercise 5
takes it further, with projects changed at random.

## The app's side: `sync_audio`

`sync_audio` turns each `Change` into a command, reading values from the
current project:

```rust
fn sync_audio(&mut self, before: &Project) {
    let sample_rate = self.sample_rate();
    let mut patterns_moved = false;
    for change in diff(before, &self.project) {
        match change {
            Change::RemovePattern(index) => {
                self.send(Command::RemovePattern(index));
                // Keep the same pattern selected. If it was the one
                // removed, the next one takes its place.
                if index < self.pattern {
                    self.pattern -= 1;
                }
                patterns_moved = true;
            }
            Change::InsertPattern(index) => {
                let length = self.project.patterns()[index].length();
                self.send(Command::InsertPattern { index, length });
                // A new pattern (added, cloned, or brought back by
                // undo) is the one you want to see.
                self.pattern = index;
                patterns_moved = true;
            }
            Change::Notes { pattern, channel } => {
                let notes = self.project.patterns()[pattern].notes(channel).to_vec();
                self.send(Command::SetNotes { pattern, channel, notes });
            }
            ...
        }
    }
    ...
}
```

It also keeps the UI's own numbers pointing at the right things. The
selected pattern is a position, so it moves down when a pattern before it
goes. The piano roll remembers which channel it shows, so it is told
when channels come and go (`channel_removed`, and the new
`channel_inserted`). Because these updates are here, adding, cloning and
deleting patterns now select the right pattern whether you did them or
undo did.

## The audio thread learns to insert

Undoing "delete pattern 0" must put a pattern back at position 0, not at
the end. In step 4 the audio thread could only add patterns and channels
at the end (`AddPattern`, `AddChannel`). Now the commands take a
position:

```rust
/// Add an empty pattern at position `index`; the patterns from there
/// on move up one place. An index past the end adds it at the end.
InsertPattern { index: usize, length: usize },

/// Add a channel at position `index`, with its notes in every pattern.
InsertChannel {
    index: usize,
    instrument: Instrument,
    notes: ChannelNotes,
    volume: f32,
    muted: bool,
},
```

`Vec::insert(index, value)` moves every item from `index` on one place
up, and puts `value` in the gap. Would that allocate on the audio thread?
Only if the `Vec` were full. Step 2 reserved room for `MAX_CHANNELS`
channels and step 4 room for `MAX_PATTERNS` patterns, and both commands
check the limit first, so `insert` always fits in memory that is already
there. Moving items along is just copying, which is allowed in real time.
`ChannelNotes::push_empty` became `insert_empty(pattern)` for the same
reason.

An index past the end is clamped (`index.min(len)`) rather than refused.
The audio thread must never panic, and "insert after the last one" is the
most sensible reading of a too-big number.

The new `index` field made `InsertChannel` the biggest command, at 72
bytes. Every slot of the command queue is as big as the biggest command,
so the test that keeps commands small (from step 3) now allows 80 bytes.
It still holds its real purpose: an instrument's voices are behind a
`Box`, so a command never carries all of them.

## Instruments come back from a cache

Undoing "load sample" gives a channel its old instrument back, and that
might be a sample file. Before this step, building a sampler always read
its file. Reading a file on the UI thread for an undo would make the
window stutter, and the file might even be gone by now.

So the app keeps a **`SampleCache`**: every sample file read so far, by
path.

```rust
pub struct SampleCache {
    loaded: HashMap<PathBuf, Arc<SampleData>>,
}

impl SampleCache {
    pub fn get(&mut self, path: &Path) -> Result<Arc<SampleData>, SampleError> {
        // A `HashMap<PathBuf, _>` can be searched with a `&Path`, because
        // `PathBuf` implements `Borrow<Path>`: no `PathBuf` is built here.
        if let Some(sample) = self.loaded.get(path) {
            return Ok(Arc::clone(sample));
        }
        self.reload(path)
    }

    pub fn reload(&mut self, path: &Path) -> Result<Arc<SampleData>, SampleError> {
        let sample = Arc::new(SampleData::load_wav(path)?);
        self.loaded.insert(path.to_path_buf(), Arc::clone(&sample));
        Ok(sample)
    }
}
```

A **`HashMap`** finds a value by its key in about the same time however
many entries it holds. Its keys are owned `PathBuf`s, but `get` takes a
borrowed `&Path`. That works because of the **`Borrow` trait**: `PathBuf`
implements `Borrow<Path>`, a promise that a `PathBuf` and the `Path` it
holds hash and compare the same way. So the map can look up a `&Path`
directly, without building a `PathBuf` just to search. (`HashMap<String,
_>` can be searched with a `&str` for the same reason.)

The cache hands out `Arc` clones, so the cache and every sampler using
the sound share one copy of it in memory, as in step 2. A test checks
this with **`Arc::ptr_eq`**, which asks whether two `Arc`s point at the
very same memory, not merely at equal values:

```rust
assert!(Arc::ptr_eq(&first, &second), "the second get shares the first");
assert!(!Arc::ptr_eq(&first, &reloaded), "reload reads the file again");
```

Dropping a file on the rack calls `reload`, not `get`: you may have
changed the file in an audio editor since, and dropping it again should
pick that up. A file that cannot be read is not remembered, so the next
attempt reads it again. Opening another project clears the cache.

`Instrument::for_channel` now takes the cache as an argument
(`&mut SampleCache`), instead of reading files itself. The export, in the
next chapter, builds every instrument again, and thanks to the cache it
reads no files either.
