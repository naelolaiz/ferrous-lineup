# Song structure: sections, repeats and loops

Files: [`crates/lineup-core/src/playlist.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-core/src/playlist.rs),
[`crates/lineup-core/src/arrangement.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-core/src/arrangement.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-core/src/project.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-audio/src/engine.rs)
and [`crates/lineup-app/src/playlist/sections.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-app/src/playlist/sections.rs)

Until now the playlist only knew clips. A song has more shape than that:
an intro, a verse, a chorus heard twice, a beat that loops for eight
bars. To get any of it you placed every clip by hand, and nothing on the
screen said "this is the chorus". This page adds three things:

- **Sections**: named stretches of the song (Intro, Verse, Chorus…) in a
  strip above the bar numbers. A section can be played several times in a
  row: it is drawn once and marked "×2".
- **Clip repeats**: drag a clip's right edge and it plays its pattern again
  and again, like stretching a pattern clip in FL Studio.
- **A loop region**: drag along the bar numbers, and song mode loops just
  that part while you work on it.

![The chorus is marked "×2". While the song plays it the second time, the
section says "▶ 2 of 2".](../images/step-8-sections-playing.png)

## Using it

- Drag along the dark strip marked **Sections** to mark a section. It gets
  the first common name not used yet: Intro, then Verse, Chorus, Bridge,
  Outro. **➕ Section** in the playlist toolbar adds one after the last.
- Drag a section to move it, or its right edge to resize it. It snaps like
  clips do (to bars, beats or steps).
- Click a section (left or right button) for its menu: type a name or pick
  one, choose how many times it plays with **−** and **+**, duplicate it,
  write out its repeats, or delete it. **Loop this section** is ticked
  while song mode loops just that section; tick it to loop the section,
  untick it to play the whole song again. Without a loop, song mode
  plays the whole song and then starts again from bar 1.
- Drag a clip by its right edge (the pointer turns into a double arrow) to
  repeat its pattern.
- Drag along the bar numbers to loop that part. The green **Looping bars…**
  button in the toolbar shows it; click it to play the whole song again.

Sections do not own clips. A section is only a name and a range of steps:
whatever clips lie under it are part of it. Deleting a section leaves the
clips alone.

![A section's menu.](../images/step-8-section-menu.png)

## Two kinds of step

Here is the one idea that makes everything else simple. Once a section can
repeat, the playlist as *drawn* and the song as *played* are no longer the
same length:

```text
timeline:   | Intro | Chorus ×2 | Outro |
song:       | Intro | Chorus | Chorus | Outro |
```

So the code now talks about two kinds of step:

- a **timeline step** is a place in the playlist, as drawn;
- a **song step** is a moment in the song, as heard.

`Project::song_length` stays what it was, the length of the timeline. A new
type, `Arrangement`, is the song as heard:

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

### Building the arrangement

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

## The audio thread did not learn about sections

The audio thread used to get the clips and play each pattern from the
clip's start. Now `Command::SetSong` carries `plays` instead of `clips`,
and the only new thing the audio thread does is a remainder:

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
still allocates nothing: the new `Vec<Play>` is built on the UI thread and
the old one is sent back to be freed there, exactly as the clip list was.

The WAV export and the MIDI export use the same `Arrangement`, so all
three agree on what the song is. The MIDI writer needs the opposite
question: on which song steps does a note at pattern step 2 play? That is
`Play::song_steps_of`, which uses an iterator with `step_by`:

```rust
(first..self.length)
    .step_by(pattern_length.max(1))
    .map(move |offset| self.start + offset)
```

## Back from the song to the playlist

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

## Saving: format 4

`Clip` and `Section` both have a `repeats` field, and most of the time it
is 1. serde can leave a field out of the file when it has its usual value,
and fill it back in when reading a file that does not have it:

```rust
#[serde(default = "once", skip_serializing_if = "is_once")]
pub repeats: usize,
```

`default` names a function that returns the value to use, and
`skip_serializing_if` a function that says when to leave the field out.
The new lists in the file (`sections`, `loop_region`) use the plain
`#[serde(default)]` with `Vec::is_empty` and `Option::is_none`. So a
project that uses none of this is written byte for byte as before, and an
older Ferrous Lineup opens it.

An older version would also open a project that *does* use repeats: it
skips fields it does not know. But it would play every repeat once, which
is a song that sounds wrong without a word of warning. So, following the
rule from step 7, `format_for` returns **4** for any project with
sections, a loop region or a repeated clip, and older versions refuse it
with a clear message.

## Drawing it: a module inside a module

`playlist.rs` was already long, so the strips went into their own file.
A file module can have children: `mod sections;` inside `playlist.rs`
loads `playlist/sections.rs`. A child module can see its parent's private
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

## Writing out the repeats

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
original and its copy overlap. Ferrous Lineup cannot cut a clip in two yet
(a clip has no "start this far into the pattern"), which is exercise 3
below.

![Dragging along the bar numbers loops bars 3 to 6.](../images/step-8-loop.png)

## Exercises

1. Add a **Delete with clips** entry to the section menu that removes the
   section, the clips that start inside it, and closes the gap.
   `make_room` shows how to move everything after a point.
2. Show the song's real length, with every repeat, next to the tempo:
   "8 bars drawn, 12 played, 0:22". `Project::arrangement().length` and
   `Tempo` have all you need.
3. Give `Clip` an `offset` (how many steps into its pattern it starts) so
   a clip can be cut in two. `Play::skip` already plays a clip from the
   middle; the work is in the playlist and in `duplicate_section`.
4. FL Studio can also *mark* a point in time without a range, for example
   to jump to it. Add markers to the strip, and a key that moves the
   playhead to the next one.
