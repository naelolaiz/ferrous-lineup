# A clearer look, a real transport, and editing tracks

Files: [`crates/lineup-app/src/theme.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-app/src/theme.rs),
[`crates/lineup-app/src/app.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-app/src/app.rs),
[`crates/lineup-core/src/sequencer.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-core/src/sequencer.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-audio/src/engine.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-core/src/project.rs)
and [`crates/lineup-app/src/playlist.rs`](https://github.com/naelolaiz/ferrous-loom/blob/band-in-a-box/crates/lineup-app/src/playlist.rs)

Up to here the app used egui's stock dark style, and it showed: one long
row of flat text buttons, nothing to say what a control does unless you
waited for a tooltip, and no display of where the song is. Worse, the
transport was a single Play/Stop button that always started from the
beginning. You could not pause, could not jump to bar 9 to listen to the
chorus, and could not insert or delete a track. This page fixes all of
that:

- **A theme**: blue-grey panels, raised grey buttons, an orange light for
  whatever is selected or switched on, black displays for numbers.
- **A grouped toolbar** with a tempo display and a BAR:BEAT:STEP display.
- **A hint bar** next to the menus that says what the control under the
  mouse does.
- **Transport buttons**: go to start, back a bar, play/pause, stop,
  forward a bar, record. Click the playlist's bar numbers to move the
  playhead.
- **Track editing**: right-click a track's name to insert, move or delete
  tracks.

![The new toolbar while playing. The hint bar at the top explains the
pause button the mouse is on.](../images/step-8-look.png)

## Using it

| Do this | To |
|---|---|
| ▶ or Space | play from the playhead |
| ❚❚ (while playing) | pause; ▶ carries on from there |
| ⏹ or Space (while playing) | stop and go back to where you last put the playhead |
| ⏮ or Home | go to the start |
| ◀◀ ▶▶ or Page Up, Page Down | move a bar back or forward |
| click the playlist's bar numbers | put the playhead there (dragging still sets the loop) |
| right-click a track's name | insert a track above or below, move it, or delete it |

## A theme is one function

egui draws every widget from a `Style`: colours in `style.visuals`,
sizes and gaps in `style.spacing`. Nothing is painted with a hard-coded
colour unless the code asks for one. So changing the whole look is one
function, called once when the app starts:

```rust
pub fn apply(ctx: &egui::Context) {
    ctx.set_theme(egui::Theme::Dark);
    ctx.style_mut_of(egui::Theme::Dark, |style| {
        let visuals = &mut style.visuals;
        visuals.panel_fill = PANEL;
        visuals.window_fill = WINDOW;
        visuals.selection.bg_fill = ACCENT;      // selected and switched-on things
        visuals.selection.stroke = Stroke::new(1.0, ON_ACCENT);
        // ...
        for (state, fill) in [
            (&mut widgets.inactive, BUTTON),
            (&mut widgets.hovered, BUTTON_HOVER),
            (&mut widgets.active, BUTTON_PRESSED),
            (&mut widgets.open, BUTTON_HOVER),
        ] {
            state.bg_fill = fill;
            state.weak_bg_fill = fill;
            state.bg_stroke = Stroke::new(1.0, OUTLINE);
        }
        style.spacing.button_padding = vec2(7.0, 3.0);
    });
}
```

Three things to notice:

- `style_mut_of(Theme::Dark, ...)` changes the *dark* style only. egui
  keeps one style for dark and one for light, and the app always uses
  dark.
- The `for` loop walks an **array of pairs of mutable references**. Each
  `&mut widgets.inactive` borrows a *different field*, so Rust allows
  several of them at once: the borrow checker tracks fields of a struct
  separately.
- The colours are `const` items: `Color32::from_rgb` is a `const fn`, so
  it runs at compile time and costs nothing when the app runs.

Views still have their own constants for the things only they draw (the
playlist's rows, the piano roll's keys, the step buttons). Those simply
got new values that match the theme.

## Changing the style for a few widgets

The tempo display is a `DragValue`, the same widget as before, but drawn
black with green-blue digits. Its style is changed only inside its frame:

```rust
pub fn lcd_frame(ui: &mut Ui, label: &str, width: f32,
                 add: impl FnOnce(&mut Ui) -> Response) -> Response {
    egui::Frame::new()
        .fill(LCD)
        .show(ui, |ui| {
            // ...
            let visuals = &mut ui.visuals_mut().widgets;
            for state in [&mut visuals.inactive, &mut visuals.hovered, &mut visuals.active] {
                state.bg_fill = LCD;
                state.fg_stroke.color = LCD_TEXT;
            }
            ui.style_mut().override_font_id = Some(FontId::monospace(16.0));
            add(ui)
        })
        .inner
}
```

Every `Ui` has its own copy of the style (an `Arc<Style>`; `style_mut`
makes a private copy the first time it is changed). The frame's
contents get a child `Ui`, so the change stays inside the frame and the
rest of the toolbar is untouched. The tempo field is then just:

```rust
let tempo = theme::lcd_frame(ui, "TEMPO", 62.0, |ui| {
    ui.add(egui::DragValue::new(&mut bpm).range(Tempo::MIN..=Tempo::MAX))
});
```

`add` is a closure that *returns* the widget's `Response`, so the caller
can still ask `tempo.changed()` outside.

## The hint bar: a message from the future

The hint bar sits at the top of the window, so it is drawn *first*. But
the widgets that know what the mouse is on are drawn *later* in the same
frame. In immediate mode there is no going back to repaint the bar.

The fix is to let the bar show what was found on the **previous** frame.
Widgets leave their hint in egui's per-frame memory, `ctx.data`, which is
a map from an `Id` to a value of any type:

```rust
pub fn hint(response: Response, text: impl Into<String>) -> Response {
    let text = text.into();
    if response.hovered() {
        response.ctx.data_mut(|data| data.insert_temp(hint_id(), text.clone()));
    }
    response.on_hover_text(text)
}
```

At the start of the next frame, the bar takes it out (so it does not
linger once the mouse moves away):

```rust
pub fn start_frame(&mut self, ctx: &egui::Context) {
    let text = ctx.data_mut(|data| data.remove_temp::<String>(hint_id()));
    if text != self.text {
        ctx.request_repaint();
    }
    self.text = text;
}
```

`remove_temp::<String>` uses the **turbofish** `::<String>` to say which
type to take out: the map can hold values of many types under the same
`Id`. The `request_repaint` is the subtle part. egui only draws a frame
when something happens. When the mouse moves onto a button, that frame
records the hint, but the bar still shows the old one. Asking for one
more frame shows the new hint straight away, instead of whenever the
mouse next moves.

A frame of lag is invisible at 60 frames a second, and the code stays
simple. It is a common trick in immediate-mode UIs.

## The transport: where does Play start?

Before, `Command::Play` always reset the sequencer to step 0. Pausing and
seeking need one new idea: a **start position**. The sequencer learns to
jump:

```rust
/// Jumps to `step`, so it fires on the next tick. A step past the end
/// wraps around, as playback would.
pub fn start_at(&mut self, step: usize) {
    self.frames_until_next = 0.0;
    self.next_step = step % self.length;
}
```

and the audio thread gets one new command:

```rust
Command::Play => {
    self.sequencer.start_at(self.start);
    self.playing = true;
}
Command::Seek(step) => {
    self.start = step;
    if self.playing {
        for channel in &mut self.channels {
            channel.instrument.release_all();
        }
        self.sequencer.start_at(step);
    }
}
```

While stopped, `Seek` only moves where the next `Play` starts. While
playing, it jumps there, and lets go of every note first: a note that
was sounding would otherwise wait for its end, which playback will now
never reach.

Everything else is bookkeeping in the app, with two numbers:

- `cursor`: where playback starts or carries on from;
- `marker`: where Stop goes back to, the last place you put the playhead.

```rust
fn pause(&mut self) {
    let here = self.playhead().unwrap_or(self.cursor);
    self.halt();
    self.cursor = here;              // carry on from here
    self.send(Command::Seek(here));
}

fn set_playing(&mut self, playing: bool) {
    if playing {
        self.send(Command::Seek(self.cursor));
        self.send(Command::Play);
        self.playing = true;
    } else {
        self.halt();
        self.cursor = self.marker;   // Stop goes back to the marker
        self.send(Command::Seek(self.cursor));
    }
}
```

This is how pattern-based studios behave: pausing keeps your place,
stopping takes you back to where you started listening, so you can
hear the same passage again and again with Space.

### Seeking with repeats

Clicking the bar numbers gives a **timeline step**, but the audio thread
counts **song steps**, where a chorus played twice counts twice (see
[Song structure](song-structure.md)). The app turns one into the other
with the arrangement's passes:

```rust
let song = self.project.playback_arrangement();
let step = song
    .passes
    .iter()
    .find(|pass| (pass.from..pass.from + pass.length).contains(&step))
    .map_or(0, |pass| pass.start + (step - pass.from));
self.seek(step);
```

`find` takes the **first** pass that covers the step, so clicking inside
a repeated chorus goes to its first time round. A range like
`a..b` is a value in Rust, with methods such as `contains`.

The bar ruler already used dragging for the loop region. A click and a
drag are told apart by egui: `Sense::click_and_drag()` reports
`clicked()` only when the mouse did not move far between press and
release, and `drag_started()` only when it did. So both live on the same
strip.

### Moving by bars

`skip_bars` moves to the start of a bar. Going back from the middle of a
bar first goes to that bar's start, as on a tape machine:

```rust
let target = if bars < 0 && !here.is_multiple_of(per_bar) {
    bar + bars + 1
} else {
    bar + bars
};
```

`is_multiple_of` is a newer standard library method on unsigned
integers; clippy suggests it over `here % per_bar != 0`, which says the
same thing less directly.

## Inserting, moving and deleting tracks

A clip names its track by **number**. Insert a track at row 2, and every
clip on row 2 or below must move down one, or the drums would suddenly
sit on the wrong track:

```rust
pub fn insert_track(&mut self, index: usize) -> Option<usize> {
    let tracks = &mut self.playlist.tracks;
    if tracks.len() >= MAX_TRACKS {
        return None;
    }
    let index = index.min(tracks.len());
    tracks.insert(index, Track::numbered(tracks.len() + 1));
    for clip in &mut self.playlist.clips {
        if clip.track >= index {
            clip.track += 1;
        }
    }
    Some(index)
}
```

`remove_track` is the mirror image: drop the track's clips with
`retain`, then move the ones below up one. `swap_tracks` swaps two rows
and the clips on them. This is the same rule as patterns and clips in
step 4: whoever changes the list fixes the numbers that point into it,
which is why the fields are private and only `Project`'s methods change
them. Each of these is one call inside `edit`, so undo brings a deleted
track back with its clips.

![Right-click a track's name.](../images/step-8-track-menu.png)

The menu is egui's `context_menu` on the track's name field. A text
field still opens a menu on a right-click, because typing only needs the
left button.
