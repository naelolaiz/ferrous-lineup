# Patterns and the playlist

A song in Ferrous Lineup is built from **patterns**, short blocks of
music a bar or a few bars long, laid out in time on the **playlist**.
A pattern is "what every channel plays during these bars": a drum beat,
four bars of chords, a fill. The playlist says where each pattern plays,
and how many times. The same pattern can play many times: change it
once, and every place it plays changes with it.

The song the app opens with shows the idea. It has three patterns:
**Chords** (two bars of C, Am, F and G), **Beat** (one bar of drums)
and **Fill** (a drum fill). On the playlist, Chords plays four times on
the first track, and Beat five times on the second from bar 3, with
Fill at the end.

![The starting song's playlist with the Patterns tab open on the left (Beat, Chords and Fill, each with its colour), bars 3 and 4 looped on the bar numbers, and the right-click menu of the Drums track: Track height, Reset track height, Insert track above, Insert track below, Move up, Move down and Delete track](images/playlist.png)

## Patterns

The **Patterns** tab, next to the Library on the left of the window,
lists the song's patterns, each with its colour. Click one to select
it. The **selected pattern** is the one the channel rack and the piano
roll show, and the one a click on the playlist places.

The **PATTERN** controls in the toolbar work on the selected pattern:

- the list picks another pattern;
- **➕** adds a new, empty pattern;
- **⋯** has the pattern's **Name**, **Clone** (a copy to change on its
  own) and **Delete** (which takes its clips off the playlist too);
- the last list sets its length: 1, 2, 3, 4, 6, 8, 12 or 16 bars.

A song with a band has a pattern for each player, written from the
chord sheet: you don't need to touch them to hear the band, and the
band writes them again when the chords change (see [Locking a track,
and playing your part](locks.md)).

## Pattern mode and song mode

**PAT** and **SONG**, at the left of the toolbar, choose what ▶ plays:

- **Pattern mode** plays the selected pattern over and over. Use it
  while you write a beat or a part.
- **Song mode** plays the playlist from the playhead to the end.

**L** switches between them.

## Placing and moving clips

Each block on the playlist is a **clip**: one pattern, at one place on
one track. The playlist's toolbar line says it in short: "Click to
place "Beat". Double-click a clip to edit it, drag to move it, drag its
right edge to repeat it, right-click it for its menu."

- **Click** an empty place on a track to put a clip of the selected
  pattern there.
- **Drag** a clip to move it, to another bar or another track.
- **Drag its right edge** to repeat it: the clip grows by whole
  patterns, and plays the pattern that many times in a row.
- **Double-click** a clip to edit it: melodies and chords open in the
  piano roll, drums in the channel rack.
- **Right-click** a clip for its menu: **Edit its pattern**, as a
  double-click does, and **Delete**, which deletes every selected clip
  when the clip is one of them. On a recording, **Transcribe again…**
  takes the place of Edit (see [Takes](takes.md#transcribing-a-take-again)).

**Snap**, in the playlist's toolbar, says where clips land: on a
**Bar**, a **Beat** or a **Step** (a sixteenth note).

To work with several clips at once, **Ctrl+click** them one by one, or
**Ctrl+drag** a box around them on an empty part of the playlist; drag
one of them and they all move together. **Ctrl+click** a section on the
Sections strip, or pick **Select clips in this section** in its menu,
to select the clips under it. Then:

| Key | What it does |
|---|---|
| **Ctrl+C**, **Ctrl+X**, **Ctrl+V** | copy, cut and paste the clips |
| **Ctrl+D** | duplicate them, right after themselves |
| **Delete** | delete them |
| **Ctrl+A** | select every clip |
| **Escape** | select nothing |

**Ctrl+click** an empty place to choose where **Ctrl+V** pastes, without
placing a clip.

## Tracks

Tracks are rows to arrange clips on; any pattern can go on any track.
A song has as many tracks as it uses, and under the last one is a
fainter row: place a clip on it, or click its **➕ New track**, and it
becomes a track. **➕ Track** in the playlist's toolbar adds one too.

- The **light** at the left of a track mutes it and unmutes it.
- **S**, at its right, plays that track alone: every other track is
  muted. Click it again to hear them all.
- **Click its name** to rename it.
- **Drag its lower edge** to make it taller or shorter;
  **double-click** the edge to give it back its usual height.
- **Right-click its name** for **Track height**, **Reset track height**,
  **Insert track above**, **Insert track below**, **Add a recording…**
  (see [Takes](takes.md#adding-a-recording-made-before)), **Move up**,
  **Move down** and **Delete track**, which deletes its clips too.

## The strips above the tracks

- **Sections**: drag along it to mark a part of the song, such as a
  verse or a chorus. See [Song form and sections](song-form.md).
- **Tempo**: the tempo as a line, with each change marked. See [Tempo,
  time signatures and key changes](tempo-meter-key.md).
- **Bars**: the bar numbers, with the time signature and the key at
  bar 1 and wherever they change. **Click** a number to move the
  playhead there. **Drag** along them to loop that part of the song:
  song mode then plays it over and over, and the playlist's toolbar
  shows **🔁 Looping bars 3–4 ✖**; click it to play the whole song
  again. **Right-click** a number to insert or delete bars, or to
  change the time signature or the key from there.

## Moving around

| Key | What it does |
|---|---|
| **Space** | play or stop |
| **Home**, **End** | go to the start, or to the end |
| **Page Up**, **Page Down** | back or forward a bar |
| **Ctrl+wheel**, **Ctrl++**, **Ctrl+−** | bars wider or narrower, around the pointer |
| **Alt+wheel**, **Alt++**, **Alt+−** | tracks taller or shorter |
| **Ctrl+0** | back to 100% |

The two zooms are also at the right of the status bar: **↔** for the
bars and **↕** for the tracks. **Follow playhead**, in the toolbar,
keeps the bar that plays in sight; turn it off to scroll freely while
the song plays.
