# Telling the audio thread what changed

Files: [`crates/lineup-core/src/diff.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/diff.rs),
[`crates/lineup-app/src/app/edit.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/edit.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs)
and [`crates/lineup-audio/src/sample.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/sample.rs)

## One way in for every change

The audio thread keeps its own copy of everything it plays: each
channel's instrument, volume, pan and notes, the pattern lengths, the
song (see [Two threads](threads.md)). The window must keep
that copy up to date, and a change can touch anything. A click on a step
changes one list of notes. Deleting a pattern renumbers every clip after
it. Typing a chord makes the band write new patterns. Undo replaces the
whole project with an older one, which can differ in anything at all.

There are three ways to tell the audio thread about a change:

1. **Send the whole project again**: every pattern and channel, as when
   a file is opened. Simple, but every instrument would be built again,
   cutting off every note that is ringing, and a sample file would be
   read again for a one-step undo.
2. **Write, for each kind of edit, the commands it needs**, and for
   undo, the commands that reverse it. That is a hand-written pair for
   every edit, and every pair is a chance to get the reverse wrong (the
   [next chapter](undo.md) says more about this choice).
3. **Compare the project before and after**, and send only what differs.

Ferrous Lineup does the third, for *every* change, not only for undo.
No edit sends its own commands. Every change goes through one method,
`edit`:

```rust
pub(super) fn edit<R>(
    &mut self,
    label: &'static str,
    change: impl FnOnce(&mut Project) -> R,
) -> R {
    if self.history.is_open() {
        // A drag or a typing session is under way, and its first change
        // made the undo step. Keep changing the project without copying
        // the whole of it every frame; the audio catches up steadily
        // from the last state it was given.
        let (patterns, channels) =
            (self.project.patterns().len(), self.project.channels().len());
        let result = change(&mut self.project);
        self.project.follow_chord_sheet();
        self.file.project_edited();
        self.recovery.project_changed();
        self.step_unsynced = true;
        if self.project.patterns().len() != patterns
            || self.project.channels().len() != channels
        {
            // A pattern or channel came or went: the views and the
            // audio thread's rows have to move now, not in 50 ms.
            self.sync_step();
        } else {
            self.sync_step_if_due();
        }
        return result;
    }
    // A step left open by a direct history call: send it before the
    // audio's state is assumed to be this project.
    self.finish_step_sync();
    let before = self.project.clone();
    let result = change(&mut self.project);
    // The band plays what the chord sheet says now, in the same step.
    self.project.follow_chord_sheet();
    if self.project != before {
        self.sync_audio(&before);
        self.synced = self.project.clone();
        self.step_unsynced = false;
        self.last_sync = Instant::now();
        self.file.project_reverted();
        self.recovery.project_changed();
        // `before` is moved into the history here, after its last use.
        self.history.record(label, before);
    }
    result
}
```

Read the second half first, the ordinary case. `edit` keeps a copy of
the project, lets the closure change it, and if anything changed, it
does four things in one place:

- `sync_audio(&before)` tells the audio thread what differs. The rest
  of this chapter is about it.
- `follow_chord_sheet` lets the band write its patterns again if the
  chord sheet, the key or the bars changed, as part of the same step
  (see [How the band plays a chord sheet](../band/band.md)).
- `file` and `recovery` hear that the project changed: the title bar
  gets its unsaved-changes star, and a recovery copy will be written
  (see [Saving](saving.md)). `project_reverted` works the
  star out again from scratch, because a change made in one go can put
  back exactly what was saved, as a step clicked on and off again does.
- `history.record` keeps `before` as the undo step called `label`
  (see [Undo and redo](undo.md)).

Having one path pays off twice. There is only one piece of code that
turns project changes into commands, so it can be tested thoroughly on
its own. And it runs on every click, so if it had a mistake you would
hear it while editing, not only in the rare moment you undo something
unusual.

### A drag is one step, not one per frame

The first half, the `is_open` branch, is the important performance
detail. A drag changes notes on every frame, sixty times a second, but
only its first frame needs the copy for undo and the full comparison.
While the step is open, the project is changed directly, and `synced`
holds the last state the audio thread was given. `sync_step_if_due`
compares the project with that state and sends the differences at most
every 50 ms:

```rust
/// How often, at most, a drag's changes reach the audio thread. The
/// window still draws every frame; only the sound waits this long, and
/// the whole project is copied only when this comes round.
const SYNC_INTERVAL: Duration = Duration::from_millis(50);

fn sync_step_if_due(&mut self) {
    if self.last_sync.elapsed() >= SYNC_INTERVAL {
        self.sync_step();
    }
}

fn sync_step(&mut self) {
    let before = std::mem::replace(&mut self.synced, Project::empty());
    self.sync_audio(&before);
    self.synced = self.project.clone();
    self.step_unsynced = false;
    self.last_sync = Instant::now();
}
```

`std::mem::replace` takes the old `synced` out of the struct and leaves
an empty project in its place, so `sync_audio` can borrow it as
`before` without a copy (it could not borrow `self.synced` while also
borrowing `self` mutably). When a pattern or a channel comes or goes in
the middle of a step, waiting is not good enough: the views count rows
that the audio thread does not have yet, so `sync_step` runs at once.

When the mouse is released, `finish_step_sync` sends the last missing
changes, and the step closes. One drag is one undo step, one copy and a
handful of messages.

Undo and redo replace the whole project, so they always do the full
comparison:

```rust
pub(super) fn undo(&mut self) {
    self.finish_step_sync();
    let before = self.project.clone();
    if self.history.undo(&mut self.project).is_some() {
        self.sync_audio(&before);
        self.synced = self.project.clone();
        self.step_unsynced = false;
        self.last_sync = Instant::now();
        self.file.project_reverted();
        self.recovery.project_changed();
        self.restore_song_library();
        self.keep_place_in_song(&before);
    }
}
```

The last two lines are for the window, not the sound: the library shows
the style and rhythm this version of the project plays, and if undo
changed the time signature, playback stays on the same bar.

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
the closure trait for closures called exactly once: it lets the closure
move a value into the project, such as a new `channel`, because nothing
will call it a second time. (`Fn` closures may be called many times and
only read what they capture; `FnMut` ones may change it.)

`R` is whatever the closure returns, and `edit` passes it back to the
caller. Many `Project` methods return something: `add_clip` returns the
new clip's index as an `Option<usize>`, `remove_pattern` the pattern it
removed, `insert_bars` whether there was room. Because `R` can be any
type, a closure can simply be the method call,
`|project| project.add_clip(clip)`, and the caller gets the answer. If
`edit` asked for `impl FnOnce(&mut Project)`, which must return `()`,
each of those would need braces and a semicolon to throw the value away.

`edit_pattern` is a small wrapper for the most common case, a change to
the selected pattern:

```rust
fn edit_pattern(&mut self, label: &'static str, change: impl FnOnce(&mut Pattern)) {
    let index = self.pattern;
    self.edit(label, |project| {
        if let Some(pattern) = project.pattern_mut(index) {
            change(pattern);
        }
    });
}
```

`self.project != before` compares the two projects completely: the
derived `PartialEq` compares every field, every list, every note. If
nothing changed, such as a clip dragged back onto the same cell or a
note deleted where there was none, nothing is sent and no undo step is
recorded.

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
    Pan(usize),
    ReverbSend(usize),
    Effects(usize),
    PatternLength(usize),
    Notes { pattern: usize, channel: usize },
    Tempo,
    LooseTiming,
    Mixer,
    Song,
}

pub fn diff(before: &Project, after: &Project) -> Vec<Change>
```

The changes are steps to follow **in order**, and each number is a
position at the moment its change is applied. After `RemovePattern(0)`,
the pattern that used to be number 1 is number 0, and any later change
calls it 0. Values (a volume, a list of notes) are not in the `Change`:
they are read from the newer project when the change is turned into a
command. That keeps `Change` small and `Copy`.

Only what makes a sound is compared. Pattern names, channel names,
track names, section names and the key give no change at all, because
the audio thread never sees them. Muting a track does give one: it takes
that track's clips out of the song, so the clips the audio thread plays
change.

The mixer is audible too. Moving a channel between the speakers gives
`Pan`, changing how much of it goes to the reverb gives `ReverbSend`,
changing its EQ, compressor or delay gives `Effects`, and changing the
master volume or the reverb itself gives `Mixer`. Soloing a channel is a
mixer change *and* changes which channels are heard, so `diff` also
reports `Muted` for every channel whose effective mute changed. Loose
timing gives `LooseTiming`: it decides how far off the grid a note may
start.

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
fn new<T>(before: &[T], after: &[T], same: impl Fn(&T, &T) -> bool) -> Self {
    if before.len() == after.len() {
        return Alignment { start: 0, removed: 0, inserted: 0 };
    }
    // The same at the start...
    let start = before
        .iter()
        .zip(after)
        .take_while(|(old, new)| same(old, new))
        .count();
    // ...and the same at the end, without counting an item twice.
    let room = before.len().min(after.len()) - start;
    let end = before[start..]
        .iter()
        .rev()
        .zip(after[start..].iter().rev())
        .take_while(|(old, new)| same(old, new))
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

`Alignment` works for any `T`, because the caller says what "the same"
means with the `same` closure. For channels that is full equality:
`|old, new| old == new`. For patterns it is **not**, and that is the
subtle part. A pattern's rows change when channels come and go: when the
band joins, every pattern gets a row for each of its channels, so
comparing whole patterns would say every pattern changed. The alignment
would then send every pattern's notes again, and the app would select a
pattern that only looks new.

So patterns are lined up by the things that identify them, not by their
contents:

```rust
/// Whether two patterns are the same pattern, perhaps with other notes.
fn same_pattern(old: &Pattern, new: &Pattern) -> bool {
    old.name == new.name && old.color == new.color && old.player == new.player
}
```

The notes and the length are what the aligner lets change in place. Name,
colour and player say *which* pattern it is; the notes that differ are
sent afterwards as `Notes` and `PatternLength` changes anyway.

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
    let patterns = Alignment::new(before.patterns(), after.patterns(), same_pattern);
    let channels = Alignment::new(before.channels(), after.channels(), |old, new| old == new);

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
was, or `None` if it is new), the instrument and mix are compared:

```rust
for (index, channel) in after.channels().iter().enumerate() {
    let Some(old_index) = channels.old_index(index) else {
        continue; // a new channel arrives complete
    };
    let old = &before.channels()[old_index];
    if old.kind != channel.kind
        || old.program != channel.program
        || old.sent_on != channel.sent_on
    {
        changes.push(Change::Instrument(index));
    }
    if old.volume != channel.volume {
        changes.push(Change::Volume(index));
    }
    if old.pan != channel.pan {
        changes.push(Change::Pan(index));
    }
    if old.reverb_send != channel.reverb_send {
        changes.push(Change::ReverbSend(index));
    }
    if old.effects != channel.effects {
        changes.push(Change::Effects(index));
    }
    if before.channel_muted(old_index) != after.channel_muted(index) {
        changes.push(Change::Muted(index));
    }
}
```

The instrument is built from three fields: the kind (drum, sample file,
Lineup Osc, MIDI Out), the General MIDI program, and `sent_on`, the MIDI
port a drum is sent to instead of playing here. `channel_muted` is not
simply the channel's mute button: it also takes the mixer's solo into
account. That is why a solo change shows up as a `Muted` change on the
other channels, as well as a `Mixer` change.

**`let ... else`** either takes the value out of the `Some`, or runs the
`else` block, which must leave: here `continue` jumps to the next
channel.

Then the notes, for every pattern and every old channel:

```rust
for (index, pattern) in after.patterns().iter().enumerate() {
    let old_index = patterns.old_index(index);
    let old_pattern = old_index.map(|old| &before.patterns()[old]);
    if let Some(old) = old_pattern
        && old.length() != pattern.length()
    {
        changes.push(Change::PatternLength(index));
    }
    for channel in 0..after.channels().len() {
        let Some(old_channel) = channels.old_index(channel) else {
            continue; // a new channel brought its notes with it
        };
        // The notes as they are played, with the song's swing: a
        // new amount, or a pattern now in a time signature that does
        // not swing, plays other notes. A new pattern starts with no
        // notes in the audio thread.
        let old_notes = match old_index {
            Some(old) => before.played_notes(old, old_channel),
            None => Cow::Borrowed(&[][..]),
        };
        if old_notes != after.played_notes(index, channel) {
            changes.push(Change::Notes { pattern: index, channel });
        }
    }
}
```

The `if let ... && ...` is a **let chain**: the block runs only if the
pattern is old *and* its length changed.

The notes compared are `played_notes`, not the written ones. Swing moves
the offbeat notes later without changing what is written (see [Tempo,
meter, swing and feel](../sound/tempo.md)), and the audio thread must
be sent what is *heard*. So turning the swing up changes no written note,
but gives a `Notes` change for every list it moves. `played_notes`
returns a **`Cow`** ("clone on write"): a `Cow::Borrowed` when the
written notes are played as they are, which costs nothing, and a
`Cow::Owned` with a changed copy only when swing moves them. Two `Cow`s
compare by their contents, whichever kind they are.

`&[][..]` is an empty slice: `[]` is an empty array, and `[..]` takes all
of it as a slice. A new pattern is compared with "no notes", which is
exactly what the audio thread has for it, so a `Notes` change is sent
only for the channels that have notes in it.

Last come the tempo, the looseness, the mixer and the song. The song is
compared through the arrangement that song mode actually plays:

```rust
let (old_song, new_song) = (before.playback_arrangement(), after.playback_arrangement());
if old_song.plays != new_song.plays
    || old_song.length != new_song.length
    || before.played_tempos(&old_song) != after.played_tempos(&new_song)
{
    changes.push(Change::Song);
}
```

An `Arrangement` is the clips with their repeats and the sections'
repeats already unrolled, cut to the loop region if there is one (see
[The project](project.md)). Comparing it catches a moved
clip, a mute, a repeat count, a changed loop and a tempo marker in one
`Song` change. Tempo markers are part of the song because the length of
a step changes partway through it.

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
pattern lengths, each channel's instrument, mix, effects and notes per
pattern, the tempo, and the song. It is simple enough to trust at a
glance:

```rust
struct ModelChannel {
    kind: ChannelKind,
    program: Option<u8>,
    volume: f32,
    muted: bool,
    pan: f32,
    reverb_send: f32,
    effects: crate::Effects,
    /// One list per pattern.
    notes: Vec<Vec<NoteEvent>>,
}

struct Model {
    lengths: Vec<usize>,
    channels: Vec<ModelChannel>,
    tempo: Tempo,
    loose_timing: u8,
    plays: Vec<Play>,
    song_length: usize,
    tempos: Vec<Tempo>,
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
the exact changes: that a step click gives a single `Notes`, that names
and the key give nothing, that undoing a deleted pattern starts with
`InsertPattern(0)`, that swing changes the notes played. The last test
makes a mess on purpose, removing a pattern, adding a channel, toggling a
step, changing a volume and the tempo all at once, and also diffs an
empty project against the demo song, as opening a file would.

This kind of test is called **property-based**: it checks a rule that
must hold for any input, rather than one answer for one input. One of
the [exercises](exercises.md) takes it further, with
projects changed at random.

## The app's side: `sync_audio`

`sync_audio` turns each `Change` into a command, reading values from the
current project:

```rust
fn sync_audio(&mut self, before: &Project) {
    let sample_rate = self.sample_rate();
    let mut patterns_moved = false;
    let mut channels_changed = false;
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
                let pattern = &self.project.patterns()[index];
                let (length, bands) = (pattern.length(), pattern.player.is_some());
                self.send(Command::InsertPattern { index, length });
                if bands {
                    // A part the band wrote, as you typed a chord:
                    // the pattern you had stays selected.
                    if index <= self.pattern {
                        self.pattern += 1;
                    }
                } else {
                    // A new pattern (added, cloned, or brought back
                    // by undo) is the one you want to see, from its
                    // start.
                    self.pattern = index;
                    self.rewind_for_new_pattern();
                }
                patterns_moved = true;
            }
            Change::Notes { pattern, channel } => {
                let notes = self.project.played_notes(pattern, channel).into_owned();
                self.send(Command::SetNotes { pattern, channel, notes });
            }
            ...
        }
    }
    ...
}
```

The other arms are just as direct. `InsertChannel` builds the channel's
instrument and sends it with its notes for every pattern. `Instrument`
builds a new instrument and sends `SetInstrument`. `Volume`, `Pan`,
`ReverbSend` and `Effects` send their one value, `Muted` sends the
channel's effective mute, `LooseTiming` sends the new amount, `Mixer`
sends the master volume and the reverb settings, and `Song` sends the
three commands that replace the song, its tempos and its metronome
clicks. `Tempo` sends the tempo at the practice speed, if you slowed the
song down to practise (see [Practice tools](../band/practice.md)).

`into_owned` turns the `Cow` from `played_notes` into a `Vec` the
command can carry: it clones a borrowed list, and simply hands over an
owned one.

`sync_audio` also keeps the window's own numbers pointing at the right
things. The selected pattern is a position, so it moves down when a
pattern before it goes, and up when the band inserts one before it. The
piano roll remembers which channel it shows, so it is told when channels
come and go (`channel_removed` and `channel_inserted`). Because these
updates are here, adding, cloning and deleting patterns select the right
pattern whether you did them or undo did.

## The audio thread inserts in place

Undoing "delete pattern 0" must put a pattern back at position 0, not at
the end. So the audio thread's commands take a position:

```rust
/// Add an empty pattern at position `index`; the patterns from there
/// on move up one place. An index past the end adds it at the end.
InsertPattern { index: usize, length: usize },

/// Add a channel at position `index`, with its notes in every pattern.
/// The channels from there on move down one row. An index past the
/// end adds it at the bottom.
InsertChannel {
    index: usize,
    instrument: Instrument,
    notes: ChannelNotes,
    volume: f32,
    muted: bool,
},
```

`Vec::insert(index, value)` moves every item from `index` on one place
up, and puts `value` in the gap. Would that allocate on the audio
thread? Only if the `Vec` were full. Room for `MAX_CHANNELS` channels
and `MAX_PATTERNS` patterns is reserved when the audio thread is made
(see [Room reserved in advance](threads.md#room-reserved-in-advance)),
and both commands check the limit first, so `insert` always fits in
memory that is already there. Moving items along is just copying, which
is allowed in real time:

```rust
fn insert_pattern(&mut self, index: usize, length: usize) {
    if self.pattern_lengths.len() < MAX_PATTERNS {
        let index = index.min(self.pattern_lengths.len());
        self.pattern_lengths.insert(index, length);
        for channel in &mut self.channels {
            channel.notes.insert_empty(index);
        }
        ...
    }
}
```

An index past the end is clamped (`index.min(len)`) rather than refused.
The audio thread must never panic, and "insert after the last one" is the
most sensible reading of a too-big number.

`InsertChannel` is one of the biggest commands. Every slot of the
command queue is as big as the biggest command, so a test keeps commands
at 80 bytes at most. An instrument's voices are behind a `Box`, so a
command carries a pointer to them, never all of them.

## Instruments come back from a cache

Undoing "load sample" gives a channel its old instrument back, and that
might be a sample file. Reading a file on the window's thread for an
undo would make the window stutter, and the file might even be gone by
now.

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
the sound share one copy of it in memory. A test checks this with
**`Arc::ptr_eq`**, which asks whether two `Arc`s point at the very same
memory, not merely at equal values:

```rust
assert!(Arc::ptr_eq(&first, &second), "the second get shares the first");
assert!(!Arc::ptr_eq(&first, &reloaded), "reload reads the file again");
```

Dropping a file on the rack calls `reload`, not `get`: you may have
changed the file in an audio editor since, and dropping it again should
pick that up. A file that cannot be read is not remembered, so the next
attempt reads it again. Opening another project clears the cache.

`Instrument::for_channel` takes the cache as an argument
(`&mut SampleCache`) instead of reading files itself. Exporting builds
every instrument again (see [Exporting](../sound/export.md)), and
thanks to the cache it reads no files either.
