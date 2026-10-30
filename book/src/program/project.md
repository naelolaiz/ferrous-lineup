# The project: patterns, channels, tracks, clips and markers

Files: [`crates/lineup-core/src/pattern.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/pattern.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs),
[`crates/lineup-core/src/playlist.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/playlist.rs)
and [`crates/lineup-core/src/arrangement.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/arrangement.rs)

Everything the user works on is one value of type `Project`: the
channels of the channel rack, the patterns of notes they play, the
playlist that arranges the patterns into a song, and the markers along
it (sections, tempo, time signature, key and chords). It is also exactly
what a project file saves. This chapter is about its shape and the rules
it keeps. The windows that edit it are in [The
editors](editors.md); how to use them is in the user
manual's [Patterns and the
playlist](https://naelolaiz.github.io/ferrous-lineup/manual/patterns-and-playlist.html).

## Notes, steps and patterns

A **pattern** is a short stretch of music, often one bar, with a row of
notes for every channel. A note is a pitch, a start, a length and a
velocity, all counted in **steps** (a sixteenth note each in 4/4):

```rust
pub struct NoteEvent {
    pub pitch: Note,
    /// The step the note starts on, counted from 0.
    pub start: usize,
    /// How many steps the note is held for, at least 1.
    pub length: usize,
    /// How hard the note is played, from 1 to 127 (as in MIDI).
    pub velocity: u8,
    /// How many ticks after the start of its first step the note really
    /// starts, from 0 to [`TICKS_PER_STEP`] - 1.
    pub nudge: u8,
    /// How many ticks before the end of its last step the note really
    /// ends.
    pub trim: u8,
}
```

The channel rack's step buttons and the piano roll edit the same notes.
A step that is "on" is just a note one step long at the default pitch,
so `is_on` asks whether any note starts there:

```rust
pub fn is_on(&self, channel: usize, step: usize) -> bool {
    self.notes(channel).iter().any(|note| note.start == step)
}
```

One kind of data with two views is how pattern-based studios work, and
it means the audio thread has a single kind of thing to play. `nudge`
and `trim` place a note between steps, for music played by a person or
imported from a MIDI file; notes on the grid leave them at 0.

## Channels: enums with data

```rust
pub enum SampleSource {
    /// One of the drums built into Ferrous Lineup.
    Builtin(Drum),
    /// A WAV file on disk.
    File(PathBuf),
}

pub enum ChannelKind {
    /// Plays a recorded or built-in sound from start to end for each
    /// note, faster for higher notes and slower for lower ones.
    Sampler(SampleSource),
    /// The built-in "Lineup Osc" synthesizer, which can play chords.
    Synth,
    /// Makes no sound itself: sends its notes out through MIDI, on
    /// MIDI channel `channel` (0 to 15, shown as 1 to 16).
    MidiOut { channel: u8 },
}
```

A channel has a name, a volume, a mute switch, a General MIDI
instrument number, and a kind. Enums nest: a channel is a sampler, the
Lineup Osc or a MIDI Out channel, and a sampler's sound is either a
built-in drum or a file. Each fact is stored once and every combination
makes sense. `MidiOut { channel: u8 }` shows that a variant can also
have **named fields**, like a struct.

`PathBuf` is the standard type for an owned file path. It is not a
`String`, because file names on some systems are not valid UTF-8 text;
`path.display()` turns one into something printable.

`Drum::ALL` is an associated constant holding every variant, so menus can
loop over the drums instead of listing them by hand.

## The shape of a project

A project has a list of patterns, and a playlist that says where each
one plays:

```rust
pub struct Project {
    pub mixer: crate::Mixer,
    pub tempo: Tempo,
    pub scale: Scale,
    pub time_signature: TimeSignature,
    pub loose_timing: u8,
    pub swing: u8,
    pub(crate) channels: Vec<Channel>,
    pub(crate) patterns: Vec<Pattern>,
    pub(crate) playlist: Playlist,
    pub(crate) band: Option<Band>,
    // ...where each part came from, and the locked channels
}
```

Every pattern has one row of notes per channel. Pattern-based
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

`Project::playback_arrangement` unrolls the repeats into `Play`s before
the song reaches the audio thread (see [Sections, repeats and
loops](#sections-repeats-and-loops) below); the clip is the thing you
draw, and its `repeats` is the little "×2" on it.

`map_or(0, Pattern::length)` says "if there is a pattern, call
`Pattern::length` on it; otherwise use 0". A method can be passed by name,
like a function, when its only argument is `self`.

## Four rules, one gatekeeper

The comment on `Project` lists the rules it keeps:

- every clip in the playlist refers to a pattern and a track that exist;
- sections are sorted and never overlap, and repeats stay in range.

Most of these rules are about two lists at once, so no single list can
keep them. The `Project` owns all of them, keeps the fields private to
`lineup-core`, and changes them only in methods that keep every rule.
Adding a channel adds a row to every pattern:

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
  can ignore it, keep it, or check that the
  removal happened at all.

The test `removing_a_pattern_renumbers_its_followers` checks the whole
story: after removing "Beat", the five Beat clips are gone, "Chords" is
pattern 0, and the fill still plays at bar 8.

For a list of at most 999 patterns (usually far fewer), positions plus
one careful method are the simpler choice. It is the kind of trade-off
with no single right answer; one of the [exercises](exercises.md)
of this part asks you to think about it.

## Handing out `&mut Pattern` safely

The app needs to change notes in a pattern. `Project` could copy every
`Pattern` method (`toggle`, `add_note`, `replace_note`...) as a method of
its own. Instead it hands out the pattern itself:

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

On the app side, every pattern edit goes through one small helper:

```rust
fn edit_pattern(&mut self, label: &'static str, change: impl FnOnce(&mut Pattern)) {
    let index = self.pattern;
    self.edit(label, |project| {
        if let Some(pattern) = project.pattern_mut(index) {
            change(pattern);
        }
    });
}

// For example:
RackAction::ToggleStep { channel, step } => {
    self.edit_pattern("change steps", |pattern| {
        pattern.toggle(channel, step);
    });
}
```

- `impl FnOnce(&mut Pattern)` accepts any closure that takes a pattern.
  `FnOnce` is the most accepting of the three closure traits: the
  closure only has to be callable once, so it may even move values it
  captured.
- `self.edit(label, ...)` is the one function every change to the
  project goes through. It keeps the change for undo and tells the audio
  thread what changed ([Undo and redo](undo.md), [Telling
  the audio thread what changed](sync.md)). `label` is what
  **Edit > Undo** will say.

## Names, and `impl Into<String>`

Patterns have names. The constructor takes the name first:

```rust
pub fn new(name: impl Into<String>, channels: usize, length: usize) -> Self {
    Pattern {
        name: name.into(),
        color: 0,
        time: None,
        player: None,
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
it behind methods. `color`, `time` and `player` say which colour the
pattern is drawn in, the time signature it was written in, and which of
the band's players wrote it, if one did.

Cloning a pattern uses `Pattern`'s `Clone` derive:

```rust
pub fn clone_pattern(&mut self, index: usize) -> Option<usize> {
    let mut copy = self.patterns.get(index)?.clone();
    copy.name = format!("{} copy", copy.name);
    // A copy is yours, not the band's.
    copy.player = None;
    self.push_pattern(copy)
}
```

`.clone()` copies the whole pattern, every row and every note, so the
copy and the original never share anything; the test
`cloned_patterns_are_independent` checks that changing one leaves the
other alone. The `?` after `get(index)` returns `None` from the function
straight away if there is no such pattern.

## No two notes in the same place

`add_note` refuses to add a note in the same place as another.
`replace_note`, used when dragging, must refuse too, or one note could
be dragged exactly on top of another, so it checks every note except
the one being replaced:

```rust
let duplicate = row
    .iter()
    .enumerate()
    .any(|(other_index, other)| other_index != index && other.same_place(&note));
```

So "no two notes of the same pitch start at the same moment" is true
for every pattern, and loading a file can insist on it. `same_place`
compares the pitch, the step and the nudge, so two notes that start a
few ticks apart are still two notes.

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
that repeats at song step 16 is on pattern step 0 there. `audible_clips` skips clips
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

## The starting song

`Project::demo` builds an eight-bar song, which many tests play, with
the same methods the app uses:

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
in bar 8. **File > New beat** starts from `Project::template` instead:
the drum kit and a Lineup Osc, with no notes and one track. **File >
New** starts from `Project::with_band`, an empty chord sheet with a band
([How the band plays a chord sheet](../band/band.md)).

## Sections, repeats and loops

A song has more shape than its clips: an intro, a verse, a chorus heard
twice, a beat that loops for eight bars. The playlist keeps three more
things for it:

- **Sections**: named stretches of the song (Intro, Verse, Chorus…) in a
  strip above the bar numbers. A section can be played several times in a
  row: it is drawn once and marked "×2".
- **Clip repeats**: a clip can play its pattern again and again, like
  stretching a pattern clip in other pattern-based studios.
- **A loop region**: song mode can loop just one part of the song.

Sections do not own clips. A section is only a name and a range of steps:
whatever clips lie under it are part of it. Deleting a section leaves the
clips alone.

### Two kinds of step

Here is the one idea that makes everything else simple. Once a section can
repeat, the playlist as *drawn* and the song as *played* are no longer the
same length:

```text
timeline:   | Intro | Chorus ×2 | Outro |
song:       | Intro | Chorus | Chorus | Outro |
```

So the code talks about two kinds of step:

- a **timeline step** is a place in the playlist, as drawn;
- a **song step** is a moment in the song, as heard.

`Project::song_length` is the length of the timeline. `Arrangement` is
the song as heard:

```rust
pub struct Arrangement {
    /// The pieces of patterns to play, in song steps.
    pub plays: Vec<Play>,
    /// The stretches of the timeline, in the order they play.
    pub passes: Vec<Pass>,
    /// How long the song is, in song steps.
    pub length: usize,
}

pub struct Play {
    pub pattern: usize,
    pub start: usize,  // the song step it starts on
    pub skip: usize,   // how far into the clip it starts
    pub length: usize, // how many steps it lasts
}
```

A `Pass` says "timeline steps 32 to 63 play at song steps 64 to 95, and
this is round 2 of 2 of section 1". A `Play` says "play pattern 3 from song
step 64 for 32 steps, starting 0 steps into it". The pattern loops for as
long as the `Play` lasts, so a clip repeated three times is one `Play`
three patterns long. `skip` is not zero when a section or the loop region
starts halfway through a clip.

`Project::arrangement()` builds it for the whole song (the exports use
it), and `Project::playback_arrangement()` for what song mode plays: the
loop region if there is one, the whole song if not.

#### Building the arrangement

`arrange` in `arrangement.rs` cuts the timeline into pieces at section
edges. Sections play `repeats` times; the gaps between them play once:

```rust
fn pieces(project: &Project, from: usize, to: usize)
    -> Vec<(usize, usize, Option<usize>, usize)>
{
    let mut pieces = Vec::new();
    let mut cursor = from;
    for (index, section) in project.playlist().sections().iter().enumerate() {
        if section.end() <= cursor || section.start >= to {
            continue;
        }
        if section.start > cursor {
            pieces.push((cursor, section.start, None, 1)); // a gap
        }
        let start = section.start.max(cursor);
        let end = section.end().min(to);
        pieces.push((start, end, Some(index), section.repeats));
        cursor = end;
    }
    if cursor < to {
        pieces.push((cursor, to, None, 1));
    }
    pieces
}
```

This only works because sections are kept **sorted and never
overlapping**. That is a rule, so, as with clips, the list is private and
only `Project`'s methods change it. `add_section` and `set_section` check
the rule and refuse a section that would break it; `insert_section` keeps
the order with `partition_point`, a binary search that returns the first
place where the condition stops being true:

```rust
let index = sections.partition_point(|other| other.start < section.start);
sections.insert(index, section);
```

Then, for each piece and each time round, every clip that overlaps the
piece adds a `Play` for the part of it inside the piece. `max` and `min`
clamp the clip to the piece:

```rust
let begin = clip.start.max(piece_from);
let end = clip_end.min(piece_to);
if begin < end {
    arrangement.plays.push(Play {
        pattern: clip.pattern,
        start: start + (begin - piece_from),
        skip: begin - clip.start,
        length: end - begin,
    });
}
```

### The audio thread knows nothing of sections

`Command::SetSong` carries the arrangement's `plays`, and all the audio
thread does with them is a remainder:

```rust
pub fn pattern_step(&self, step: usize, pattern_length: usize) -> Option<usize> {
    if pattern_length == 0 || !(self.start..self.end()).contains(&step) {
        return None;
    }
    Some((self.skip + step - self.start) % pattern_length)
}
```

`%` wraps the step around the pattern, which is all that "repeat" means.
Everything about names, rounds and loops stays in `lineup-core`, where it
can be tested without a sound card. The audio thread stays small, and it
allocates nothing: the `Vec<Play>` is built on the UI thread, and the
old one is sent back to be freed there.

The WAV export and the MIDI export use the same `Arrangement`, so all
three agree on what the song is. The MIDI writer needs the opposite
question: on which song steps does a note at pattern step 2 play? That is
`Play::song_steps_of`, which uses an iterator with `step_by`:

```rust
(first..self.length)
    .step_by(pattern_length.max(1))
    .map(move |offset| self.start + offset)
```

### Back from the song to the playlist

The audio thread counts song steps, so the playhead it reports is a song
step. To draw it, the app turns it back into a timeline step with the
arrangement's passes:

```rust
fn song_position(&self) -> Option<(usize, Pass)> {
    if self.mode != PlayMode::Song {
        return None;
    }
    let step = self.playhead()?;
    let song = self.project.playback_arrangement();
    let pass = *song.pass_at(step)?;
    Some((pass.from + (step - pass.start), pass))
}
```

The `Pass` travels with it to the playlist, which is how a section knows
to show "▶ 2 of 2". The channel rack, the piano roll and the harmony keys
get the timeline step, so they show the right step of the right clip
during a repeat too.

### Saving: format 4

`Clip` and `Section` both have a `repeats` field, and most of the time it
is 1. serde can leave a field out of the file when it has its usual value,
and fill it back in when reading a file that does not have it:

```rust
#[serde(default = "once", skip_serializing_if = "is_once")]
pub repeats: usize,
```

`default` names a function that returns the value to use, and
`skip_serializing_if` a function that says when to leave the field out.
The lists in the file (`sections`, `loop_region`) use the plain
`#[serde(default)]` with `Vec::is_empty` and `Option::is_none`. So a
project that uses none of this is written byte for byte as before, and an
older Ferrous Lineup opens it.

An older version would also open a project that *does* use repeats: it
skips fields it does not know. But it would play every repeat once, which
is a song that sounds wrong without a word of warning. So `format_for`
returns **4** for any project with sections, a loop region or a repeated
clip, and older versions refuse it with a clear message
([Saving](saving.md)).

### Drawing it: a module inside a module

The strips are drawn in a file of their own. A file module can have
children: `mod sections;` inside `playlist.rs` loads
`playlist/sections.rs`. A child module can see its parent's private
items, so `sections.rs` uses `Layout`, `Snap` and `PlaylistAction` from
`playlist.rs` with `use super::{Layout, PlaylistAction, Snap};` without
making them public to the rest of the app.

Each section is its own clickable area, made with `ui.interact` over a
rectangle we choose, which returns a `Response` like any widget:

```rust
let response = ui.interact(rect, ui.scope_id().with(("section", index)),
                           Sense::click_and_drag());
```

The id must be different for each section and the same from one frame to
the next, so it is built from the section's number. The menu is an
`egui::Popup` attached to that response, opened on a click and closed only
by a click outside, because it holds a text field you type into.

Dragging a section does not change the project on every frame, the way
dragging a clip does. Only an outline is drawn, and the project changes
once, when the button comes up. A section moves inside a sorted list: if
one frame's move jumped it past a neighbour, its number would change in
the middle of the drag, and the next frame would move the wrong section.

### Writing out the repeats

**Write out the repeats** in a section's menu turns "Chorus ×3" into
Chorus, Chorus 2 and Chorus 3, one after the other, each played once, so
you can change the third chorus without changing the others. The song
sounds the same before and after; a test checks it by comparing the two
arrangements.

![Before (bottom) and after (top) writing out the repeats of "Chorus
×2".](../images/step-8-write-out.png)

It shares its code with **Duplicate**: `make_room` moves everything after
the section later, then the clips that start inside the section are
copied into the gap. One thing to watch: clips are copied whole. The fill
in the picture starts inside the chorus and runs past its end, so the
original and its copy overlap. Ferrous Lineup cannot cut a clip in two
(a clip has no "start this far into the pattern"); one of the
[exercises](exercises.md) of this part adds it.

![Dragging along the bar numbers loops bars 3 to 6.](../images/step-8-loop.png)
