# Only the tracks you use

Files: [`crates/loom-core/src/project.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-core/src/project.rs),
[`crates/loom-core/src/file.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-core/src/file.rs)
and [`crates/loom-app/src/playlist.rs`](https://github.com/naelolaiz/ferrous-loom/blob/main/crates/loom-app/src/playlist.rs)

Every playlist used to start with sixteen tracks. A demo with a melody
and drums opened with Track 3 to Track 16 empty under them, which is a
lot of grey to scroll past to learn that nothing is there.

Now a project has only the tracks down to its last clip. File > New has
one track, the demos have the two they use. Below the last track there
is a fainter **spare row**: place a clip on it (or drag one onto it) and
it becomes a track. Its header has a **➕ New track** button too, and
"➕ Track" and the right-click track menu from [the previous
pages](look-and-transport.md) still work.

![The start demo after placing a clip on the spare row and pressing "New track".](../images/step-8-spare-row.png)

## A clip can make its own track

`Project::clip_fits` decides whether a clip may go somewhere. It used to
say "on an existing track". Now one more row is allowed, the one just
below the last track, as long as the playlist is not full:

```rust
let tracks = self.playlist.tracks.len();
clip.pattern < self.patterns.len()
    && (clip.track < tracks || (clip.track == tracks && tracks < MAX_TRACKS))
```

`add_clip` and `move_clip` then call `open_track`, which pushes a
`Track::numbered` when the clip is on that new row. Because the track and
the clip are added by the same edit, one Ctrl+Z takes both away.

The playlist view only had to draw one more row. `Layout` got a `spare`
flag and a `rows()` method, and `cell_at`, which turns the mouse
position into a track and a step, now clamps to `rows()` instead of the
number of tracks. Everything else, placing and dragging, already worked
from `cell_at`.

## Trimming, and closures as rules

The projects are still built on a playlist of sixteen tracks: the demo
code names track 0 and track 1 before it places any clips, and that is
easier when they exist. At the end, `remove_unused_tracks` drops the
empty tracks at the bottom. Old files need almost the same thing, with
one difference: a track someone renamed or muted was kept on purpose,
so loading drops only tracks that are exactly as a new track starts.

Two rules, one loop. The loop takes the rule as a closure:

```rust
fn trim_tracks(&mut self, drop: impl Fn(usize, &Track) -> bool) {
    // ... from the bottom, pop tracks with no clips while `drop` says so
}

pub fn remove_unused_tracks(&mut self) {
    self.trim_tracks(|_, _| true);
}

pub(crate) fn remove_blank_tracks(&mut self) {
    self.trim_tracks(|index, track| *track == Track::numbered(index + 1));
}
```

`impl Fn(usize, &Track) -> bool` means "any function or closure that
takes a row number and a track and answers yes or no". The compiler
makes a copy of `trim_tracks` for each closure, so this costs nothing at
run time. Comparing with `Track::numbered(index + 1)` works because
`Track` derives `PartialEq`: a blank track is equal to a brand new one
for its row.

Trimming only ever removes tracks **below** the last clip. An empty
track in the middle stays, because removing it would move every clip
under it up a row.

## Old files

Files saved before this change have sixteen tracks. They still load:
the format did not change, only what is done after reading it. Loading
also stays strict in one way. When you place a clip by hand, the row
below the tracks makes a new track; in a file, a clip below the file's
own tracks is a mistake, and it is reported as one.
