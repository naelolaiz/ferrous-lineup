# lineup-app: the window

Files: [`crates/lineup-app/src/main.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/main.rs)
and [`crates/lineup-app/src/app.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app.rs)

The window is drawn with [egui](https://github.com/emilk/egui), a GUI
library written in Rust, and [eframe](https://docs.rs/eframe), egui's
framework for running it as a native app on Linux, macOS and Windows (and
even in a web browser).

## Immediate mode

Most GUI toolkits are **retained mode**: you create button objects once,
keep them in a tree and attach callbacks to them. egui is
**immediate mode**: on every frame, you call a function that describes
the *whole* UI from your own data, and the library draws it and tells you
what was clicked.

```rust
if ui.button("▶ Play").clicked() {
    self.set_playing(true);
}
```

That line both draws the button and checks whether it was clicked. There
are no button objects to keep in sync with your state. The state is just
the fields of your struct, and the UI is always a picture of it. This fits
Rust nicely, because there are no callbacks holding references to your
data.

## `main.rs`

```rust
mod app;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Ferrous Lineup")
            .with_inner_size([960.0, 600.0])
            .with_min_inner_size([640.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Ferrous Lineup",
        options,
        Box::new(|cc| Ok(Box::new(app::LineupApp::new(cc)))),
    )
}
```

- `main` returns a `Result`. If it returns `Err`, Rust prints the error
  and exits with a failure code.
- `ViewportBuilder::default().with_title(...)` is the **builder pattern**:
  each method takes the builder and returns it modified, so calls chain.
- `..Default::default()` is **struct update syntax**: "every field I did
  not list takes its default value".
- `Box::new(|cc| ...)` passes a closure that eframe calls once to create
  our app. `Box` puts a value on the **heap**. It is needed here because
  eframe accepts *any* type implementing its `App` trait, and to store a
  value whose exact type it does not know, it needs a pointer to it:
  `Box<dyn App>`. The `dyn` keyword means "some type implementing this
  trait, decided at run time" (a **trait object**).

## `LineupApp`: the state

```rust
enum Audio {
    Running(AudioEngine),
    Unavailable(String),
}

pub struct LineupApp {
    audio: Audio,
    playing: bool,
    tempo: Tempo,
    tone_note: Note,
    volume: f32,
}
```

The `Audio` enum captures a real situation: either we have a running
engine, or we have the reason we don't. You cannot have both, and you
cannot have neither, so the type cannot represent an impossible state.
Compare that with the usual alternative, an `Option<AudioEngine>` plus an
`Option<String>`, which allows four combinations of which two make no
sense.

Using it:

```rust
fn send(&mut self, command: Command) {
    if let Audio::Running(engine) = &mut self.audio {
        engine.send(command);
    }
}
```

`if let` is a one-arm `match`: if `self.audio` is `Running`, bind the
engine inside to `engine` and run the block. Matching on `&mut self.audio`
borrows the engine mutably instead of moving it out of the struct.

## Implementing `eframe::App`

```rust
impl eframe::App for LineupApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let typing = ui.ctx().memory(|memory| memory.focused().is_some());
        if !typing && ui.ctx().input(|input| input.key_pressed(egui::Key::Space)) {
            self.set_playing(!self.playing);
        }

        egui::Panel::top("transport").show(ui, |ui| self.transport_bar(ui));
        egui::Panel::bottom("status").show(ui, |ui| self.status_bar(ui));
        egui::CentralPanel::default_margins().show(ui, |ui| self.welcome(ui));
    }
}
```

This is where our type fulfills the trait's promise. eframe calls `ui`
every time the window needs repainting.

- Space toggles playback, as in the original studio, unless a text field
  has keyboard focus (so you can type in the tempo box).
- `ui.ctx().input(|input| ...)` gives a closure temporary access to the
  input state. egui uses this style a lot, so that it can lock its internal
  data only for the duration of your closure.
- The window has three **panels**: a top bar (transport), a bottom bar
  (status) and the central area. Each panel takes a closure that draws
  its contents. Panels must be added before the central panel, which
  takes whatever space is left.
- The closures call methods like `self.transport_bar(ui)` to keep `ui`
  short. They can borrow `self` mutably because each closure runs and
  finishes before the next one starts, which the borrow checker can see.

## The transport bar

```rust
let mut bpm = self.tempo.bpm();
let tempo_field = egui::DragValue::new(&mut bpm)
    .range(Tempo::MIN..=Tempo::MAX)
    .speed(0.5)
    .fixed_decimals(1)
    .suffix(" BPM");
if ui.add(tempo_field).on_hover_text("Tempo").changed() {
    self.tempo = Tempo::new(bpm);
}
```

- `Tempo`'s field is private, so we copy the value into a local `bpm`,
  let the widget edit that, and build a new `Tempo` if it changed. The
  `Tempo` type stays valid no matter what the user types.
- `&mut bpm` lends the variable to the widget mutably, so the widget can
  write the new value straight into it.
- `Tempo::MIN..=Tempo::MAX` is an **inclusive range**. `a..b` excludes
  `b`, `a..=b` includes it.

The test tone slider shows note names instead of numbers:

```rust
let note_slider = egui::Slider::new(&mut midi, TONE_RANGE)
    .text("Test tone")
    .custom_formatter(|n, _| Note::new(n as u8).to_string());
```

The formatter is a closure that turns the slider's number into text using
the `Display` implementation from `lineup-core`. The `_` ignores the second
argument, which we do not need.

When a value changes, the app sends a `Command` to the audio thread, for
example `Command::SetFrequency(self.tone_note.frequency())`. The UI never
waits for an answer; the audio thread applies it on its next block, a few
milliseconds later.

## The status bar

```rust
match &self.audio {
    Audio::Running(engine) => { /* show device name, sample rate, channels */ }
    Audio::Unavailable(reason) => {
        ui.colored_label(ui.visuals().warn_fg_color, format!("Audio unavailable: {reason}"));
    }
}
```

`format!` works like `println!` but returns a `String`. Inside the braces,
`{reason}` prints the variable `reason` directly; `{:.1}` elsewhere in the
file prints a float with one decimal.

If the computer has no working sound card, the app still opens and the
status bar shows why in orange. A failure becomes data we display, rather
than a crash.
