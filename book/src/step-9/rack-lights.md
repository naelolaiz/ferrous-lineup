# Step lights in the channel rack

Files: [`crates/lineup-app/src/widgets.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/widgets.rs)
and [`crates/lineup-app/src/channel_rack.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/channel_rack.rs)

A hardware step sequencer (a drum machine) has a row of small lights,
one over each step, and the light over the step being played runs
along with the music. The channel rack already knew which step was
playing, but it only drew a thin orange outline around that step's
buttons, which was easy to miss.

Now the rack has that row of lights over its steps. The light over the
step that is sounding is lit orange with a soft glow, and the whole
column under it is tinted orange too, down every row: the drum steps
and the small previews of melodies and chords alike.

![The channel rack of the starting song while the song plays: over the steps, a row of small grey lights, with the eighth lit orange; under it, the eighth step of Kick, Clap, Hat, Snare and Lineup Osc tinted orange.](../images/step-9-rack-lights-song.png)

## Trying it

1. Open the starting song and show the rack with **F6**. It shows the
   **Beat** pattern.
2. Press **L** for pattern mode (**PAT** lights up) and **Space**. The
   light runs along the sixteen steps, again and again.
3. Press **Space** to stop, **L** for song mode (**SONG**) and
   **Space** again. Bars 1 and 2 have no Beat clip, so no light is lit;
   from bar 3 the light runs along with the Beat clips in the playlist.
4. Pick **Chords** in the rack's pattern list. Its Lineup Osc row is a
   preview of the chords, and the playing column glows over it too.

![The rack playing the Beat pattern on its own: the third light is lit and the third column glows.](../images/step-9-rack-lights-pattern.png)

When the rack shows one pattern and the song is playing another, the
rack stays on the pattern you chose, and its lights stay dark until
that pattern plays. Unlit, the light on the first step of each bar is
a little brighter than the others, and the light on the first step of
each beat a little brighter than the steps in between, so you can count
the row as you count the steps under it. In 3/4 or 6/8 the beats fall
where that time signature puts them.

## How it works

### Which step is playing

Nothing in playback changed. The app already had
`LineupApp::pattern_playhead`: in pattern mode it is the step the
audio engine reports; in song mode it asks the project which step of
*this* pattern the song is on, with `Project::pattern_step_in_song`,
which looks for a clip of the pattern on an unmuted track that covers
the song's current step. If none does, it returns `None`, and that is
why the lights stay dark while the pattern isn't sounding.

### Painting the lights after the rows

The lights must sit exactly over the steps, but the steps start after
the mute light, the knob, the lock and the channel name, and their
widths change with the rack's zoom. Rather than adding those widths
up (and getting it wrong the next time a control is added to a row),
the rack lets the rows tell it:

```rust
let (lights, _) = ui.allocate_exact_size(
    egui::vec2(1.0, widgets::step_lights_height(ui)),
    egui::Sense::hover(),
);
let mut steps_left = None;
for (index, channel) in project.channels().iter().enumerate() {
    let row = ui.horizontal(|ui| {
        // ... mute light, knob, lock, name ...
        steps_left.get_or_insert(ui.cursor().left());
        // ... the steps ...
    });
}
if let Some(left) = steps_left {
    widgets::step_lights(ui, lights, left, &marks, playhead);
}
```

`allocate_exact_size` reserves an empty strip at the top, so the rows
go under it. While the first row is laid out, `ui.cursor().left()` is
where its first step is about to go; `get_or_insert` keeps the first
value and ignores the rows after it. Once the rows are done,
`step_lights` paints into the reserved strip. egui draws everything at
the end of the frame, so painting into space reserved earlier is fine.

### The lights

`step_lights` takes a slice of `StepMark`, a small enum that says
whether each step starts a bar, starts a beat, or neither:

```rust
pub enum StepMark {
    Bar,
    Beat,
    Plain,
}
```

The rack works it out from the pattern's time signature: a step starts
a bar when `step % steps_per_bar == 0`, and a beat when
`time.beat_at(step).start` is that step's place in the bar. Each light
is placed one step width (the button plus the gap after it) after the
one before, the same arithmetic `mini_roll` uses, so lights, buttons
and previews line up at any zoom.

### The glowing column

`step_button` already got `is_playhead`. Now it also mixes some orange
into the button's colour with `Color32::lerp_to_gamma`, a little less
on a step that is on, so a white step stays white. `mini_roll` fills
the playing column with a see-through orange (`gamma_multiply(0.25)`
makes the colour a quarter as strong) under its outline.

## Exercises

1. Make the lit light green instead of orange. Where is the colour
   defined, and what else uses it?
2. Make the lights on the first step of each bar a different colour
   from the beat lights, instead of only brighter.
3. While the song plays a different pattern than the rack shows, the
   lights stay dark. Add a faint hint in the rack, such as "Not playing
   now", for that case. Which values in `channel_rack::show` tell you
   the song is playing?
