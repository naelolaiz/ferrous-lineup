//! Exercise the actual app windows and action routing without a sound card.

use super::*;
use eframe::App;
use eframe::egui::{
    Event, FullOutput, Key, Modifiers, PointerButton, Pos2, RawInput, Rect, Shape, vec2,
};

struct Screen {
    ctx: egui::Context,
    app: LineupApp,
    frame: eframe::Frame,
    size: egui::Vec2,
    settings_only: bool,
}

impl Screen {
    fn new() -> Self {
        let ctx = egui::Context::default();
        theme::apply(&ctx);
        ctx.all_styles_mut(|style| style.animation_time = 0.0);
        let mut app = LineupApp::with_audio(&ctx, None, Audio::Unavailable("test".into()));
        app.harmony_keys.open = false;
        app.playlist.browser_open = false;
        // Never read or remove a real recovery copy while testing.
        app.recovery = super::recovery::Recovery::new(None);
        Self {
            ctx,
            app,
            frame: eframe::Frame::_new_kittest(),
            size: vec2(1400.0, 1000.0),
            settings_only: false,
        }
    }

    fn draw(&mut self, events: Vec<Event>) -> FullOutput {
        let raw = RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, self.size)),
            time: Some(self.ctx.cumulative_frame_nr() as f64 * 0.2),
            events,
            ..Default::default()
        };
        self.run(raw)
    }

    /// A frame in which the window is asked to close, as its title bar's
    /// button does.
    fn request_close(&mut self) -> FullOutput {
        let mut raw = RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, self.size)),
            time: Some(self.ctx.cumulative_frame_nr() as f64 * 0.2),
            ..Default::default()
        };
        raw.viewports = std::iter::once((
            egui::ViewportId::ROOT,
            egui::ViewportInfo {
                events: vec![egui::ViewportEvent::Close],
                ..Default::default()
            },
        ))
        .collect();
        self.run(raw)
    }

    fn run(&mut self, raw: RawInput) -> FullOutput {
        let mut output = self.ctx.run_pass(raw, |ctx| {
            if self.settings_only {
                self.app.settings_window(ctx);
                return;
            }
            let mut ui = egui::Ui::new(
                ctx.clone(),
                egui::Id::unique("test-root"),
                egui::UiBuilder::new().max_rect(ctx.content_rect()),
            );
            self.app.ui(&mut ui, &mut self.frame);
        });
        output.textures_delta.clear();
        output
    }

    /// Starts the app again, with what egui keeps of its state between
    /// runs (saved and read back as eframe does), as when it is closed
    /// and opened.
    fn restart(&mut self) {
        let saved = self.ctx.memory(|memory| ron::to_string(memory).unwrap());
        self.ctx
            .memory_mut(|memory| *memory = ron::from_str(&saved).unwrap());
        // The styles are not saved: the app sets them as it starts.
        theme::apply(&self.ctx);
        self.ctx.all_styles_mut(|style| style.animation_time = 0.0);
        self.app = LineupApp::with_audio(&self.ctx, None, Audio::Unavailable("test".into()));
        self.app.harmony_keys.open = false;
        self.app.playlist.browser_open = false;
        self.app.recovery = super::recovery::Recovery::new(None);
    }

    fn click(&mut self, label: &str) {
        self.draw(vec![]);
        let output = self.draw(vec![]);
        let pos = text_rect(&output, label)
            .map(|rect| rect.center())
            .unwrap_or_else(|| panic!("Missing control: {label}"));
        self.click_at(pos, Modifiers::NONE);
    }

    /// Clicks the picture button named `name` (see [`crate::icons`]).
    fn click_icon(&mut self, name: &str) {
        self.draw(vec![]);
        self.draw(vec![]);
        let pos = icon_rect(self, name).center();
        self.click_at(pos, Modifiers::NONE);
    }

    fn click_at(&mut self, pos: Pos2, modifiers: Modifiers) {
        self.draw(vec![
            Event::ModifiersChanged(modifiers),
            Event::PointerMoved(pos),
            Event::PointerButton {
                pos,
                button: PointerButton::Primary,
                pressed: true,
                modifiers,
            },
        ]);
        self.draw(vec![Event::PointerButton {
            pos,
            button: PointerButton::Primary,
            pressed: false,
            modifiers,
        }]);
    }

    fn key(&mut self, key: Key) {
        self.key_with_modifiers(key, Modifiers::NONE);
    }

    fn key_with_modifiers(&mut self, key: Key, modifiers: Modifiers) {
        self.draw(vec![
            Event::ModifiersChanged(modifiers),
            Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers,
            },
        ]);
        self.draw(vec![Event::Key {
            key,
            physical_key: None,
            pressed: false,
            repeat: false,
            modifiers,
        }]);
        self.draw(vec![Event::ModifiersChanged(Modifiers::NONE)]);
    }

    fn scroll_to(&mut self, label: &str) {
        let pos = self.ctx.content_rect().center() + vec2(80.0, 0.0);
        self.scroll_to_at(label, pos);
    }

    fn scroll_to_at(&mut self, label: &str, pos: Pos2) {
        for _ in 0..12 {
            let output = self.draw(vec![]);
            if text_rect(&output, label).is_some() {
                return;
            }
            self.draw(vec![
                Event::PointerMoved(pos),
                Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta: vec2(0.0, -160.0),
                    phase: egui::TouchPhase::Move,
                    source: egui::MouseWheelSource::Wheel,
                    modifiers: Modifiers::NONE,
                },
            ]);
        }
        panic!("Could not scroll to {label}");
    }
}

fn text_rect(output: &FullOutput, label: &str) -> Option<Rect> {
    output
        .shapes
        .iter()
        .rev()
        .find_map(|shape| match &shape.shape {
            Shape::Text(text) if text.galley.text() == label => {
                let rect = Rect::from_min_size(text.pos, text.galley.size());
                shape.clip_rect.contains_rect(rect).then_some(rect)
            }
            _ => None,
        })
}

/// Where the picture button named `name` was in the last frame drawn.
fn icon_rect(screen: &Screen, name: &str) -> Rect {
    screen
        .ctx
        .read_response(crate::icons::id(name))
        .unwrap_or_else(|| panic!("Missing picture button: {name}"))
        .rect
}

fn icon_lit(output: &FullOutput, screen: &Screen, name: &str) -> bool {
    let center = icon_rect(screen, name).center();
    output.shapes.iter().any(|shape| {
        matches!(&shape.shape,
            Shape::Rect(rect) if rect.fill == theme::ACCENT && rect.rect.contains(center)
        )
    })
}

fn button_lit(output: &FullOutput, label: &str) -> bool {
    let label = text_rect(output, label).expect("Toggle is visible");
    output.shapes.iter().any(|shape| {
        matches!(&shape.shape,
            Shape::Rect(rect) if rect.fill == theme::ACCENT && rect.rect.contains(label.center())
        )
    })
}

fn checkbox_checked(output: &FullOutput, label: &str) -> bool {
    let rect = text_rect(output, label).expect("Checkbox label is visible");
    output.shapes.iter().any(|shape| {
        matches!(&shape.shape,
            Shape::Path(path) if path.points.len() == 3 && path.points.iter().all(|point|
                point.x < rect.left() && point.x > rect.left() - 36.0
                    && point.y > rect.top() - 5.0 && point.y < rect.bottom() + 5.0
            )
        )
    })
}

#[test]
fn songs_open_on_the_chord_sheet_and_beats_on_the_rack() {
    let top = |screen: &Screen| {
        screen
            .ctx
            .memory(|memory| memory.areas().top_layer_id(egui::Order::Middle))
            .map(|layer| layer.id)
    };
    let mut screen = Screen::new();
    assert!(screen.app.rack_open);
    screen.app.open_demo(library::Demo::Song(
        lineup_core::library::Song::named("El Choclo").unwrap(),
    ));
    screen.draw(vec![]);
    let output = screen.draw(vec![]);
    assert!(screen.app.chord_sheet.open);
    assert!(!screen.app.rack_open);
    assert_eq!(top(&screen), Some(crate::chord_sheet::window_id()));
    assert!(text_rect(&output, "Chord sheet").is_some());

    screen.app.set_playing(false);
    screen
        .app
        .request(&screen.ctx.clone(), files::Pending::NewBeat);
    screen.draw(vec![]);
    screen.draw(vec![]);
    assert!(screen.app.rack_open);
    assert!(!screen.app.chord_sheet.open);
    assert_eq!(top(&screen), Some(rack_id()));

    screen.app.request(&screen.ctx.clone(), files::Pending::New);
    screen.draw(vec![]);
    screen.draw(vec![]);
    assert!(screen.app.chord_sheet.open);
    assert!(!screen.app.rack_open);
    assert_eq!(top(&screen), Some(crate::chord_sheet::window_id()));
}

#[test]
fn the_rack_fits_its_rows_again_when_the_app_starts() {
    let mut screen = Screen::new();
    assert!(screen.app.rack_open);
    let rack = |screen: &mut Screen| {
        screen.draw(vec![]);
        screen.draw(vec![]);
        screen
            .ctx
            .memory(|memory| memory.area_rect(rack_id()))
            .unwrap()
    };
    let fitted = rack(&mut screen);
    // A longer pattern widens the rack, and egui remembers that size
    // for the next time the app starts.
    screen
        .app
        .edit_pattern("longer", |pattern| pattern.set_length(64));
    let wide = rack(&mut screen);
    assert!(wide.width() > fitted.width() + 100.0);
    // Started again on the short pattern, it fits its rows, not the
    // size remembered.
    screen.restart();
    let restarted = rack(&mut screen);
    assert!(
        (restarted.width() - fitted.width()).abs() < 2.0,
        "{restarted:?} after starting again, {fitted:?} the first time"
    );
    // The same when the rows change while the rack is closed.
    screen.app.rack_open = false;
    screen.draw(vec![]);
    screen
        .app
        .edit_pattern("longer", |pattern| pattern.set_length(64));
    screen.draw(vec![]);
    screen.app.rack_open = true;
    let reopened = rack(&mut screen);
    assert!((reopened.width() - wide.width()).abs() < 2.0);
}

/// Where the window `id` was in the last frame drawn, two frames after
/// whatever changed.
fn window_rect(screen: &mut Screen, id: egui::Id) -> Rect {
    screen.draw(vec![]);
    screen.draw(vec![]);
    screen.ctx.memory(|memory| memory.area_rect(id)).unwrap()
}

/// Has egui remember the window `id` as `size` big, as if it had been
/// dragged that big while open.
fn remember_size(screen: &mut Screen, id: egui::Id, size: egui::Vec2) {
    let mut output = screen.ctx.run_pass(
        RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, screen.size)),
            ..Default::default()
        },
        |ctx| {
            egui::Window::new("")
                .id(id)
                .fixed_size(size)
                .show(ctx, |_| {});
        },
    );
    output.textures_delta.clear();
    // A frame without it, so it opens again in the next.
    screen.draw(vec![]);
}

#[test]
fn the_mixer_is_as_big_as_its_channels() {
    let mut screen = Screen::new();
    screen.app.rack_open = false;
    screen.app.mixer_open = true;
    let id = egui::Id::unique("mixer");
    let mixer = window_rect(&mut screen, id);
    let output = screen.draw(vec![]);
    let solo = text_rect(&output, "Solo").unwrap();
    let last = text_rect(&output, "Lineup Osc").unwrap();
    assert!(mixer.contains_rect(solo) && mixer.contains_rect(last));
    // Only a margin past its last column and its last channel.
    assert!(
        mixer.right() - solo.right() < 30.0,
        "{mixer:?}, Solo at {solo:?}"
    );
    assert!(
        mixer.bottom() - last.bottom() < 30.0,
        "{mixer:?}, last channel at {last:?}"
    );
    // Dragged bigger, closed and opened again, it fits them again.
    screen.app.mixer_open = false;
    screen.draw(vec![]);
    remember_size(&mut screen, id, vec2(1300.0, 900.0));
    screen.app.mixer_open = true;
    let again = window_rect(&mut screen, id);
    assert!(
        (again.size() - mixer.size()).length() < 2.0,
        "{again:?}, {mixer:?} at first"
    );
}

#[test]
fn a_click_on_an_instrument_in_the_mixer_changes_it() {
    use lineup_core::general_midi::{ACOUSTIC_GRAND_PIANO, VIOLIN};
    let mut screen = Screen::new();
    screen.app.rack_open = false;
    screen.app.mixer_open = true;
    let channels = |screen: &Screen| screen.app.project.channels().to_vec();
    let osc = channels(&screen)
        .iter()
        .position(|channel| channel.name == "Lineup Osc")
        .unwrap();
    assert_eq!(channels(&screen)[osc].program, Some(ACOUSTIC_GRAND_PIANO));
    let id = egui::Id::unique("mixer");
    let piano = window_rect(&mut screen, id);
    screen.click("Acoustic Grand Piano");
    screen.click("Strings");
    screen.click("41 Violin");
    assert_eq!(channels(&screen)[osc].program, Some(VIOLIN));
    let output = screen.draw(vec![]);
    assert!(text_rect(&output, "Violin").is_some());
    // A shorter name: the mixer is fitted to its narrower column.
    let violin = window_rect(&mut screen, id);
    assert!(
        violin.width() < piano.width() - 40.0,
        "{violin:?}, {piano:?} with the piano"
    );
    // One step to undo, as in the rack.
    assert_eq!(screen.app.history.undo_label(), Some("change instrument"));
    screen.app.undo();
    assert_eq!(channels(&screen)[osc].program, Some(ACOUSTIC_GRAND_PIANO));
    // A drum plays its own sound, so it has no menu.
    screen.click("Built-in drum");
    let output = screen.draw(vec![]);
    assert!(text_rect(&output, "Guitar").is_none());
}

#[test]
fn an_instrument_menu_opened_low_in_the_window_opens_no_submenu_by_itself() {
    use lineup_core::general_midi;
    let mut screen = Screen::new();
    // Too low for the menu to fit under its button: it opens above it,
    // and scrolls.
    screen.size = vec2(1280.0, 560.0);
    screen.app.rack_open = false;
    screen.app.mixer_open = true;
    screen.click("Acoustic Grand Piano");
    let output = screen.draw(vec![]);
    let button = text_rect(&output, "Acoustic Grand Piano").unwrap();
    // The menu is above the button, its last families hidden over it.
    assert!(text_rect(&output, "Organ").unwrap().bottom() < button.top());
    assert!(text_rect(&output, "Synth effects").is_none());
    // The pointer is still on the button, where a family is hidden:
    // no family opens.
    let programs = (0..=127u8)
        .filter(|&program| {
            let name = format!("{} {}", u16::from(program) + 1, general_midi::name(program));
            text_rect(&output, &name).is_some()
        })
        .count();
    assert_eq!(programs, 0);
    // Over a family in sight, its instruments open as before.
    let brass = text_rect(&output, "Brass").unwrap().center();
    screen.draw(vec![Event::PointerMoved(brass)]);
    let output = screen.draw(vec![]);
    assert!(text_rect(&output, "57 Trumpet").is_some());
}

/// Where `label` was drawn on the row at height `y`.
fn text_rect_in_row(output: &FullOutput, label: &str, y: f32) -> Option<Rect> {
    output.shapes.iter().find_map(|shape| match &shape.shape {
        Shape::Text(text) if text.galley.text() == label => {
            let rect = Rect::from_min_size(text.pos, text.galley.size());
            (rect.top() - 4.0..=rect.bottom() + 4.0)
                .contains(&y)
                .then_some(rect)
        }
        _ => None,
    })
}

#[test]
fn the_mixer_pans_a_channel_and_sets_its_reverb_send() {
    let mut screen = Screen::new();
    screen.app.rack_open = false;
    screen.app.mixer_open = true;
    let osc = screen
        .app
        .project
        .channels()
        .iter()
        .position(|channel| channel.name == "Lineup Osc")
        .unwrap();
    let channel = |screen: &Screen| screen.app.project.channels()[osc].clone();
    assert_eq!(
        (channel(&screen).pan, channel(&screen).reverb_send),
        (0.0, 1.0)
    );
    screen.draw(vec![]);
    let output = screen.draw(vec![]);
    let row = text_rect(&output, "Acoustic Grand Piano")
        .unwrap()
        .center()
        .y;
    // The left end of each slider, under the start of its heading: all
    // the way left, and nothing sent.
    // "L100" is wider than "C", so the columns are found again after each.
    for heading in ["Pan", "Reverb"] {
        screen.draw(vec![]);
        let output = screen.draw(vec![]);
        let left = text_rect(&output, heading).expect(heading).left();
        screen.click_at(egui::pos2(left + 1.0, row), Modifiers::NONE);
    }
    assert_eq!(
        (channel(&screen).pan, channel(&screen).reverb_send),
        (-1.0, 0.0)
    );
    let output = screen.draw(vec![]);
    assert!(text_rect_in_row(&output, "L100", row).is_some());
    assert!(text_rect_in_row(&output, "0%", row).is_some());
    assert_eq!(screen.app.history.undo_label(), Some("mixer"));
    // Reset mix puts it back in the middle, all of it in the reverb.
    screen.click("Reset mix");
    assert_eq!(
        (channel(&screen).pan, channel(&screen).reverb_send),
        (0.0, 1.0)
    );
    screen.app.undo();
    assert_eq!(
        (channel(&screen).pan, channel(&screen).reverb_send),
        (-1.0, 0.0)
    );
}

#[test]
fn the_mixer_sets_a_channels_effects() {
    let mut screen = Screen::new();
    screen.app.rack_open = false;
    screen.app.mixer_open = true;
    let osc = screen
        .app
        .project
        .channels()
        .iter()
        .position(|channel| channel.name == "Lineup Osc")
        .unwrap();
    let effects = |screen: &Screen| screen.app.project.channels()[osc].effects;
    assert!(effects(&screen).is_plain());
    screen.draw(vec![]);
    let output = screen.draw(vec![]);
    let row = text_rect(&output, "Acoustic Grand Piano")
        .unwrap()
        .center()
        .y;
    let none = text_rect_in_row(&output, "none", row).unwrap();
    screen.click_at(none.center(), Modifiers::NONE);
    // The menu has the equalizer, the compressor and the delay.
    let output = screen.draw(vec![]);
    for heading in ["Equalizer", "Compressor", "Delay"] {
        assert!(text_rect(&output, heading).is_some(), "{heading}");
    }
    screen.click("dotted 1/4");
    assert_eq!(effects(&screen).delay.steps, 6);
    assert_eq!(screen.app.history.undo_label(), Some("mixer"));
    screen.click("No effects");
    assert!(effects(&screen).is_plain());
}

#[test]
fn an_exported_song_plays_each_channels_effects() {
    const RATE: u32 = 8000;
    let mut screen = Screen::new();
    let loudness = |app: &mut LineupApp| {
        let mut render = app.song_render(RATE);
        let mut out = vec![0.0; 2 * 2 * RATE as usize];
        render.renderer.render(&mut out);
        out.iter().map(|sample| sample.abs()).sum::<f32>()
    };
    let plain = loudness(&mut screen.app);
    // Every band of every channel turned all the way down.
    screen.app.edit("mixer", |project| {
        for index in 0..project.channels().len() {
            let eq = &mut project.channel_mut(index).unwrap().effects.eq;
            eq.low = -12.0;
            eq.mid = -12.0;
            eq.high = -12.0;
        }
    });
    let cut = loudness(&mut screen.app);
    assert!(cut < 0.6 * plain, "{cut} {plain}");
}

#[test]
fn an_exported_song_keeps_each_channel_where_the_mixer_put_it() {
    const RATE: u32 = 8000;
    let mut screen = Screen::new();
    // How much sound the first two seconds of the export have, left and right.
    let sides = |app: &mut LineupApp| {
        let mut render = app.song_render(RATE);
        let mut out = vec![0.0; 2 * 2 * RATE as usize];
        render.renderer.render(&mut out);
        out.chunks(2).fold([0.0_f32; 2], |sum, frame| {
            [sum[0] + frame[0].abs(), sum[1] + frame[1].abs()]
        })
    };
    let [left, right] = sides(&mut screen.app);
    assert!(
        left > 1.0 && (left - right).abs() < 1e-3 * left,
        "{left} {right}"
    );
    screen.app.edit("mixer", |project| {
        project.mixer.reverb.enabled = false;
        for index in 0..project.channels().len() {
            project.channel_mut(index).unwrap().pan = -1.0;
        }
    });
    let [left, right] = sides(&mut screen.app);
    assert!(left > 1.0 && right == 0.0, "{left} {right}");
}

#[test]
fn the_mixer_and_the_rack_send_a_drum_over_midi_and_back() {
    use lineup_core::{Drum, SampleSource};
    let mut screen = Screen::new();
    screen.app.rack_open = false;
    screen.app.mixer_open = true;
    let drum = |screen: &Screen, drum: Drum| {
        let index = screen
            .app
            .project
            .channels()
            .iter()
            .position(|channel| channel.kind == ChannelKind::Sampler(SampleSource::Builtin(drum)))
            .unwrap();
        (index, screen.app.project.channels()[index].clone())
    };
    // The kick's own row in the mixer.
    screen.draw(vec![]);
    let output = screen.draw(vec![]);
    let row = text_rect(&output, "Kick").unwrap().center().y;
    let built_in = text_rect_in_row(&output, "Built-in drum", row).unwrap();
    screen.click_at(built_in.center(), Modifiers::NONE);
    screen.click("Send over MIDI");
    screen.click("10");
    let (kick, sent) = drum(&screen, Drum::Kick);
    assert_eq!(sent.sent_over_midi(), Some(9));
    assert_eq!(sent.name, "Kick", "a drum keeps its name");
    assert_eq!(screen.app.history.undo_label(), Some("send over MIDI"));
    // The mixer says which drum of a General MIDI set it plays.
    let output = screen.draw(vec![]);
    assert!(text_rect(&output, "Bass Drum 1 · MIDI 10").is_some());
    screen.click("Bass Drum 1 · MIDI 10");
    screen.click("Play it here instead");
    assert_eq!(drum(&screen, Drum::Kick).1.sent_over_midi(), None);

    // The rack offers it on a drum's right-click menu too.
    screen.app.mixer_open = false;
    screen.app.rack_open = true;
    screen.draw(vec![]);
    let output = screen.draw(vec![]);
    let name = text_rect(&output, "Snare").unwrap().center();
    screen.draw(vec![
        Event::PointerMoved(name),
        Event::PointerButton {
            pos: name,
            button: PointerButton::Secondary,
            pressed: true,
            modifiers: Modifiers::NONE,
        },
    ]);
    screen.draw(vec![Event::PointerButton {
        pos: name,
        button: PointerButton::Secondary,
        pressed: false,
        modifiers: Modifiers::NONE,
    }]);
    screen.click("Send over MIDI");
    screen.click("4");
    assert_eq!(drum(&screen, Drum::Snare).1.sent_over_midi(), Some(3));
    assert_ne!(drum(&screen, Drum::Snare).0, kick);
}

#[test]
fn the_mixer_sends_a_channel_over_midi_and_back() {
    use lineup_core::general_midi::ACOUSTIC_GRAND_PIANO;
    let mut screen = Screen::new();
    screen.app.rack_open = false;
    screen.app.mixer_open = true;
    let osc = screen
        .app
        .project
        .channels()
        .iter()
        .position(|channel| channel.name == "Lineup Osc")
        .unwrap();
    let channel = |screen: &Screen| screen.app.project.channels()[osc].clone();
    screen.click("Acoustic Grand Piano");
    screen.click("Send over MIDI");
    let output = screen.draw(vec![]);
    // The outputs this machine has, if any (Windows always has one).
    assert!(text_rect(&output, "Output, for every channel sent over MIDI").is_some());
    let ports = screen.app.live.midi.outputs();
    assert_eq!(
        text_rect(&output, "No outputs found (the MIDI menu looks again)").is_some(),
        ports.is_empty()
    );
    for port in ports {
        assert!(text_rect(&output, &port.name).is_some(), "{}", port.name);
    }
    screen.click("3");
    assert_eq!(channel(&screen).kind, ChannelKind::MidiOut { channel: 2 });
    assert_eq!(channel(&screen).program, Some(ACOUSTIC_GRAND_PIANO));
    assert_eq!(channel(&screen).name, "MIDI Out 3");
    assert_eq!(screen.app.history.undo_label(), Some("send over MIDI"));
    // The mixer says where it goes; the menu, which channel.
    screen.click("Acoustic Grand Piano · MIDI 3");
    screen.click("● Sent over MIDI, channel 3");
    screen.click("10");
    assert_eq!(channel(&screen).kind, ChannelKind::MidiOut { channel: 9 });
    assert_eq!(channel(&screen).name, "MIDI Out 10");
    assert_eq!(screen.app.history.undo_label(), Some("change MIDI channel"));
    screen.click("Acoustic Grand Piano · MIDI 10");
    screen.click("Play it here instead");
    assert_eq!(channel(&screen).kind, ChannelKind::Synth);
    assert_eq!(channel(&screen).name, "Lineup Osc");
    assert_eq!(channel(&screen).program, Some(ACOUSTIC_GRAND_PIANO));
    let output = screen.draw(vec![]);
    assert!(text_rect(&output, "Acoustic Grand Piano").is_some());
}

#[test]
fn windows_open_at_their_own_size_not_one_remembered_from_before() {
    type Open = fn(&mut LineupApp, bool);
    let windows: [(&str, egui::Id, Open); 7] = [
        ("mixer", egui::Id::unique("mixer"), |app, open| {
            app.mixer_open = open;
        }),
        (
            "keyboard shortcuts",
            egui::Id::unique("keyboard-shortcuts"),
            |app, open| app.shortcuts_open = open,
        ),
        (
            "ear training",
            crate::ear_training::window_id(),
            |app, open| app.ear_training.open = open,
        ),
        (
            "chord sheet",
            crate::chord_sheet::window_id(),
            |app, open| app.chord_sheet.open = open,
        ),
        ("lead sheet", crate::lead_sheet::window_id(), |app, open| {
            app.lead_sheet.open = open
        }),
        ("piano roll", PianoRoll::id(), |app, open| {
            app.piano_roll.open = open;
        }),
        (
            "settings",
            egui::Id::unique("application-settings"),
            |app, open| app.settings_open = open,
        ),
    ];
    let mut screen = Screen::new();
    screen.app.rack_open = false;
    screen.app.chord_sheet.open = false;
    for (name, id, open) in windows {
        open(&mut screen.app, true);
        let first = window_rect(&mut screen, id);
        open(&mut screen.app, false);
        screen.draw(vec![]);
        // egui remembers the size it was dragged to, also when the app
        // starts again.
        remember_size(&mut screen, id, vec2(1300.0, 900.0));
        screen.restart();
        screen.app.rack_open = false;
        screen.app.chord_sheet.open = false;
        open(&mut screen.app, true);
        let again = window_rect(&mut screen, id);
        assert!(
            (again.size() - first.size()).length() < 2.0,
            "the {name}: {again:?} after starting again, {first:?} at first"
        );
        open(&mut screen.app, false);
        screen.draw(vec![]);
    }
}

#[test]
fn applying_a_library_preset_to_a_demo_creates_all_its_players() {
    use lineup_core::{Player, library::apply::StyleParts};
    for (copy_progression, playing) in [(false, false), (false, true), (true, true)] {
        let mut screen = Screen::new();
        screen.app.open_demo(library::Demo::Song(
            lineup_core::library::Song::named("El Choclo").unwrap(),
        ));
        screen.app.set_playing(playing);
        screen.app.rack_open = false;
        screen.app.playlist.browser_open = true;
        *screen.app.playlist.library_parts_mut() = StyleParts {
            rhythm: false,
            instruments: false,
            progression: false,
            time_signature: false,
            tempo: false,
            swing: false,
            matching_style: true,
        };
        screen.click("📚 Library");
        screen.click("🔍 Search: bossa, 6/8, tango…");
        screen.draw(vec![Event::Text("Emotional pop".into())]);
        let before = screen.app.project.clone();
        screen.click("Emotional pop");
        for label in [
            "Playing patterns",
            "Voices / instruments",
            "Time signature",
            "Tempo",
            "Swing",
        ] {
            screen.click(label);
        }
        if copy_progression {
            screen.click("Chord progression");
        }
        screen.click("Apply");
        let changed = screen.app.project.clone();
        assert!(
            changed.band().is_some(),
            "The Library must bring in the full band"
        );
        for player in [Player::Piano, Player::Bass, Player::Guitar, Player::Strings] {
            let channel = changed
                .player_channel(player)
                .expect("The preset's player has a channel");
            assert!(!changed.channel_muted(channel), "Muted {player:?}");
            assert!(
                !band_notes(&changed, player).is_empty(),
                "Silent {player:?}"
            );
        }
        if !copy_progression {
            assert_eq!(
                changed.playlist().chord_markers(),
                before.playlist().chord_markers()
            );
        } else {
            assert_ne!(
                changed.playlist().chord_markers(),
                before.playlist().chord_markers()
            );
        }
        let lead = before
            .channels()
            .iter()
            .position(|channel| channel.name == "Lead")
            .unwrap();
        for (index, original) in before.patterns().iter().enumerate() {
            assert_eq!(
                changed.pattern(index).unwrap().notes(lead),
                original.notes(lead)
            );
        }
        assert_eq!(screen.app.mode, PlayMode::Song);
        // Removing the generated band restores the original accompaniment.
        let mut without_band = changed.clone();
        without_band.stop_band();
        assert!(without_band.band().is_none());
        for (channel, original) in before.channels().iter().enumerate() {
            assert_eq!(without_band.channels()[channel], *original);
        }
        let text = lineup_core::file::to_text(&changed).unwrap();
        assert_eq!(lineup_core::file::from_text(&text).unwrap(), changed);
        screen.app.undo();
        assert_eq!(screen.app.project, before);
        screen.app.redo();
        assert_eq!(screen.app.project, changed);
    }
}

#[test]
fn copying_only_the_voices_or_the_patterns_brings_the_band_and_leaves_the_song_alone() {
    use lineup_core::{Player, library::apply::StyleParts};
    let cases = [
        (
            "The Rite of Spring, The Augurs of Spring",
            "Orchestra  (any meter)",
            "Voices / instruments",
        ),
        (
            "The Rite of Spring, Ritual of Abduction",
            "Orchestra  (any meter)",
            "Playing patterns",
        ),
        (
            "Petrushka, Petrushka's Room",
            "Orchestra  (any meter)",
            "Voices / instruments",
        ),
        (
            "Shchedryk (Carol of the Bells)",
            "Orchestra  (any meter)",
            "Voices / instruments",
        ),
        ("El Choclo", "Emotional pop", "Voices / instruments"),
    ];
    for (song, preset, part) in cases {
        let mut screen = Screen::new();
        screen.app.open_demo(library::Demo::Song(
            lineup_core::library::Song::named(song).unwrap(),
        ));
        screen.app.rack_open = false;
        screen.app.playlist.browser_open = true;
        *screen.app.playlist.library_parts_mut() = StyleParts {
            rhythm: false,
            instruments: false,
            progression: false,
            time_signature: false,
            tempo: false,
            swing: false,
            matching_style: true,
        };
        screen.click("📚 Library");
        screen.click("🔍 Search: bossa, 6/8, tango…");
        screen.draw(vec![Event::Text(preset.split("  ").next().unwrap().into())]);
        let before = screen.app.project.clone();
        screen.click(preset);
        screen.click(part);
        screen.click("Apply");
        let changed = &screen.app.project;
        let band = changed.band().expect("the band comes in");
        assert_eq!(
            band.accents_only,
            !preset.contains("any meter") && part != "Playing patterns",
            "{song}"
        );
        // The band plays on tracks of its own ...
        for player in [Player::Piano, Player::Bass] {
            assert!(
                !band_notes(changed, player).is_empty(),
                "{song}: silent {player:?}"
            );
        }
        // ... and the song keeps its instruments and its notes.
        for (channel, original) in before.channels().iter().enumerate() {
            let now = &changed.channels()[channel];
            assert_eq!(
                (&now.name, now.program, &now.kind),
                (&original.name, original.program, &original.kind),
                "{song}"
            );
        }
        for (index, original) in before.patterns().iter().enumerate() {
            let notes = |pattern: &lineup_core::Pattern| -> Vec<_> {
                pattern
                    .all_notes()
                    .map(|(channel, note)| (channel, *note))
                    .collect()
            };
            assert_eq!(
                notes(changed.pattern(index).unwrap()),
                notes(original),
                "{song}"
            );
        }
    }
}

#[test]
fn lagrimas_demo_keeps_its_written_chord_qualities_on_open_and_style_change() {
    use ChordQuality::{Dominant7, Major, Minor, Minor7};
    use PitchClass::{C, D, F, G};
    use lineup_core::{
        ChordQuality, PitchClass,
        library::{Song, Style},
    };
    let mut screen = Screen::new();
    screen.app.open_demo(library::Demo::Song(
        Song::named("Lágrimas y sonrisas").unwrap(),
    ));
    screen.app.rack_open = false;
    let before = screen.app.project.clone();
    screen.draw(vec![]);
    screen.draw(vec![]);
    assert_eq!(screen.app.project, before);
    // Chord qualities from the demo's source chart, independent of its key markers.
    let first = [(C, Minor); 6]
        .into_iter()
        .chain([(G, Dominant7); 8])
        .chain([(C, Minor); 2])
        .collect::<Vec<_>>();
    let second = [(C, Minor); 5]
        .into_iter()
        .chain([(C, Dominant7)])
        .chain([(F, Minor); 4])
        .chain([(C, Minor); 2])
        .chain([(G, Dominant7); 2])
        .chain([(C, Minor); 2])
        .collect::<Vec<_>>();
    let trio_first = [(C, Major); 6]
        .into_iter()
        .chain([(G, Dominant7); 8])
        .chain([(C, Major); 2])
        .collect::<Vec<_>>();
    let trio_second = [(C, Major); 6]
        .into_iter()
        .chain([(F, Major); 3])
        .chain([(D, Minor7)])
        .chain([(C, Major); 2])
        .chain([(G, Dominant7); 2])
        .chain([(C, Major); 2])
        .collect::<Vec<_>>();
    let expected = [
        &first,
        &second,
        &first,
        &second,
        &first,
        &second,
        &first,
        &second,
        &trio_first,
        &trio_second,
        &trio_first,
        &trio_second,
        &first,
        &second,
        &first,
        &second,
    ];
    let check_chart = |project: &Project| {
        for (bar, &(root, quality)) in expected.iter().flat_map(|part| part.iter()).enumerate() {
            let marker = project.chord_at(bar * 12).unwrap();
            let lineup_core::HarmonyName::Chord(symbol) = marker.harmony else {
                panic!("Not a chord")
            };
            assert_eq!(
                symbol.chord(),
                lineup_core::Chord::new(root, quality),
                "Bar {}",
                bar + 1
            );
        }
    };
    let check_played_chords = |project: &Project, channel: usize| {
        let chords: Vec<_> = expected
            .iter()
            .flat_map(|part| part.iter())
            .map(|&(root, quality)| lineup_core::Chord::new(root, quality))
            .collect();
        let mut played = vec![lineup_core::PitchSet::EMPTY; chords.len()];
        for clip in project.playlist().clips() {
            if project.playlist().tracks()[clip.track].muted {
                continue;
            }
            let pattern = project.pattern(clip.pattern).unwrap();
            for repeat in 0..clip.repeats {
                for note in pattern.notes(channel) {
                    let bar = (clip.start + repeat * pattern.length() + note.start) / 12;
                    if let Some(notes) = played.get_mut(bar) {
                        notes.insert(note.pitch.pitch_class());
                    }
                }
            }
        }
        for (bar, (notes, chord)) in played.iter().zip(chords).enumerate() {
            assert_eq!(*notes, chord.pitch_set(), "Played chord in bar {}", bar + 1);
        }
    };
    check_chart(&before);
    let keys = before
        .channels()
        .iter()
        .position(|channel| channel.name == "Keys")
        .unwrap();
    check_played_chords(&before, keys);
    // Only the groove and voices change; the source harmony must remain exact.
    screen.app.playlist.library_parts_mut().progression = false;
    screen
        .app
        .try_from_library(crate::library_browser::LibraryAction::Style(
            Style::named("Vals in minor").unwrap(),
        ));
    check_chart(&screen.app.project);
    assert_eq!(
        screen.app.project.playlist().chord_markers(),
        before.playlist().chord_markers()
    );
    check_played_chords(
        &screen.app.project,
        screen
            .app
            .project
            .player_channel(lineup_core::Player::Piano)
            .unwrap(),
    );
    screen.app.replace_project(before.clone(), None);
    screen.app.set_mode(PlayMode::Song);
    screen.app.set_playing(true);
    screen.app.playlist.library_parts_mut().matching_style = false;
    screen
        .app
        .try_from_library(crate::library_browser::LibraryAction::Style(
            Style::named("Vals in minor").unwrap(),
        ));
    check_chart(&screen.app.project);
    check_played_chords(&screen.app.project, keys);
}

#[test]
fn trying_row_and_micromode_styles_rewrites_one_pattern_while_it_is_unchanged() {
    use crate::library_browser::LibraryAction;
    use lineup_core::library::micromodes::MICROMODE_STYLES;
    use lineup_core::library::rows::ROW_STYLES;
    let mut screen = Screen::new();
    screen
        .app
        .replace_project(lineup_core::Project::empty(), None);
    let before = screen.app.project.clone();
    let patterns = before.patterns().len();
    screen
        .app
        .try_from_library(LibraryAction::RowStyle(&ROW_STYLES[0]));
    let first = screen.app.pattern;
    let name = |screen: &Screen| screen.app.project.pattern(first).unwrap().name.clone();
    assert_eq!(screen.app.project.patterns().len(), patterns + 1);
    assert_eq!(name(&screen), ROW_STYLES[0].name);
    assert_eq!(screen.app.mode, PlayMode::Pattern);
    assert!(screen.app.playing);
    // While the pattern is as the library wrote it, the next style
    // writes over it, whichever kind it is.
    screen
        .app
        .try_from_library(LibraryAction::RowStyle(&ROW_STYLES[1]));
    assert_eq!(name(&screen), ROW_STYLES[1].name);
    screen
        .app
        .try_from_library(LibraryAction::MicromodeStyle(&MICROMODE_STYLES[0]));
    assert_eq!(screen.app.pattern, first);
    assert_eq!(screen.app.project.patterns().len(), patterns + 1);
    assert_eq!(name(&screen), MICROMODE_STYLES[0].name);
    let tried = screen.app.tried();
    assert!(tried.row_style.is_none());
    assert!(
        tried
            .micromode_style
            .is_some_and(|style| std::ptr::eq(style, &MICROMODE_STYLES[0]))
    );
    // Each try is one step to undo.
    for _ in 0..3 {
        screen.app.undo();
    }
    assert_eq!(screen.app.project, before);
}

#[test]
fn plain_accents_popup_hides_instruments_and_chords_and_restores_their_saved_choices() {
    use lineup_core::library::apply::StyleParts;
    for saved in [false, true] {
        let mut screen = Screen::new();
        screen.app.rack_open = false;
        screen.app.playlist.browser_open = true;
        *screen.app.playlist.library_parts_mut() = StyleParts {
            instruments: saved,
            progression: saved,
            time_signature: false,
            tempo: false,
            swing: false,
            ..Default::default()
        };
        screen.click("📚 Library");
        screen.click("🔍 Search: bossa, 6/8, tango…");
        screen.draw(vec![Event::Text("Royal road".into())]);
        screen.click("Royal road");
        screen.click("Plain accents");
        let output = screen.draw(vec![]);
        assert!(text_rect(&output, "Drums and plain accents").is_some());
        assert!(text_rect(&output, "Playing patterns").is_none());
        assert!(text_rect(&output, "Voices / instruments").is_none());
        assert!(text_rect(&output, "Chord progression").is_none());
        assert!(text_rect(&output, "Keeps the song’s chords").is_some());
        screen.click("Preset patterns");
        let output = screen.draw(vec![]);
        assert_eq!(checkbox_checked(&output, "Voices / instruments"), saved);
        assert_eq!(checkbox_checked(&output, "Chord progression"), saved);
        screen.click("Plain accents");
        let before = screen.app.project.clone();
        screen.click("Apply");
        assert_eq!(
            screen.app.project.playlist().chord_markers(),
            before.playlist().chord_markers()
        );
        for (index, original) in before.channels().iter().enumerate() {
            assert_eq!(screen.app.project.channels()[index], *original);
        }
        let remembered = *screen.app.playlist.library_parts_mut();
        assert_eq!(
            (remembered.instruments, remembered.progression),
            (saved, saved)
        );
        let parts = screen.app.playlist.library_parts();
        assert!(!parts.instruments && !parts.progression && !parts.matching_style);
    }
}

/// The chords a trial pattern plays, as heard from its notes.
fn heard_chords(project: &Project, pattern: usize) -> Vec<String> {
    let target = project.pattern(pattern).unwrap();
    let channel = (0..project.channels().len())
        .find(|&channel| {
            let it = &project.channels()[channel];
            it.kind == lineup_core::ChannelKind::Synth
                && it.name != lineup_core::library::apply::BASS
                && !target.notes(channel).is_empty()
        })
        .expect("a channel plays the chords");
    lineup_core::library::listen::hear_chords(
        target.notes(channel),
        &[],
        target.length(),
        project.pattern_time(pattern),
    )
    .iter()
    .filter_map(|stretch| stretch.chord)
    .map(|chord| chord.name(lineup_core::Spelling::Flats))
    .collect()
}

#[test]
fn a_style_tried_with_plain_accents_plays_the_songs_chords_not_its_own() {
    for progression in [false, true] {
        let mut screen = Screen::new();
        screen.app.rack_open = false;
        screen.app.playlist.browser_open = true;
        screen.app.playlist.library_parts_mut().progression = progression;
        let chords = screen.app.project.playlist().chord_markers().to_vec();
        screen.click("📚 Library");
        screen.click("🔍 Search: bossa, 6/8, tango…");
        // Its chart is bVI | bVII | i | i: in C, Ab Bb Cm.
        screen.draw(vec![Event::Text("Aeolian climb".into())]);
        screen.click("Aeolian climb");
        screen.click("Plain accents");
        screen.click("Apply");
        let project = &screen.app.project;
        assert_eq!(project.playlist().chord_markers(), chords);
        let trial = screen.app.pattern;
        assert_eq!(project.pattern(trial).unwrap().name, "Aeolian climb");
        // The starting song plays C Am F G, two chords a bar.
        assert_eq!(
            heard_chords(project, trial),
            ["C", "Am", "F", "G"].repeat(2)
        );
    }
}

#[test]
fn preset_workflows_keep_their_apply_buttons_visible_in_small_windows() {
    let mut screen = Screen::new();
    screen.size = vec2(800.0, 600.0);
    screen.ctx.set_zoom_factor(1.1);
    screen.app.rack_open = false;
    screen.app.playlist.browser_open = true;
    screen.click("📚 Library");
    screen.click("🔍 Search: bossa, 6/8, tango…");
    screen.draw(vec![Event::Text("Aeolian".into())]);
    // The side panel keeps its width, so in a window this small the
    // list starts below the window's edge.
    screen.scroll_to_at("Aeolian climb", Pos2::new(100.0, 300.0));
    screen.click("Aeolian climb");
    for mode in ["Plain accents", "Preset patterns"] {
        screen.click(mode);
        let output = screen.draw(vec![]);
        for label in ["Apply", "Cancel"] {
            let rect = text_rect(&output, label).expect("Action stays visible");
            assert!(screen.ctx.content_rect().contains_rect(rect));
        }
    }
    screen.click("Cancel");
    assert!(!egui::Popup::is_any_open(&screen.ctx));
}

#[test]
fn searching_vals_reveals_the_normal_waltz_in_both_browser_layouts() {
    for grouped in [false, true] {
        let mut screen = Screen::new();
        crate::library_browser::set_grouped_presets(&screen.ctx, grouped);
        screen.app.rack_open = false;
        screen.app.playlist.browser_open = true;
        screen.click("📚 Library");
        screen.click("🔍 Search: bossa, 6/8, tango…");
        screen.draw(vec![Event::Text("vals".into())]);
        screen.scroll_to_at("Waltz  (3/4)", egui::pos2(100.0, 650.0));
        screen.click("Waltz  (3/4)");
        let output = screen.draw(vec![]);
        assert!(text_rect(&output, "Waltz").is_some());
        assert!(!checkbox_checked(&output, "Chord progression"));
        screen.click("Cancel");
    }
}

/// The number in the Library's "Presets (N)" heading.
fn presets_listed(output: &FullOutput) -> usize {
    output
        .shapes
        .iter()
        .find_map(|shape| match &shape.shape {
            Shape::Text(text) => text
                .galley
                .text()
                .strip_prefix("Presets (")?
                .strip_suffix(')')?
                .parse()
                .ok(),
            _ => None,
        })
        .expect("The presets heading is visible")
}

#[test]
fn a_long_line_in_the_library_leaves_the_side_panel_as_wide_as_it_was() {
    let mut screen = Screen::new();
    screen.app.rack_open = false;
    screen.app.playlist.browser_open = true;
    let playlist_left = |screen: &mut Screen| {
        let output = screen.draw(vec![]);
        text_rect(&output, "Playlist").unwrap().left()
    };
    let width = playlist_left(&mut screen);
    // "168 BPM · Shchedryk (Carol of the Bells) (demo)" is cut short.
    screen.app.open_demo(library::Demo::Song(
        lineup_core::library::Song::named("Shchedryk (Carol of the Bells)").unwrap(),
    ));
    screen.click("📚 Library");
    assert_eq!(playlist_left(&mut screen), width);
    screen.restart();
    screen.app.playlist.browser_open = true;
    assert_eq!(playlist_left(&mut screen), width);
}

#[test]
fn the_library_tick_boxes_keep_only_presets_like_the_song() {
    use lineup_core::{Tempo, TimeSignature};
    let mut screen = Screen::new();
    screen.app.project.time_signature = TimeSignature::new(4, 4);
    screen.app.project.tempo = Tempo::new(120.0);
    screen.app.rack_open = false;
    screen.app.playlist.browser_open = true;
    screen.click("📚 Library");
    let users = screen
        .app
        .playlist
        .library_browser_mut()
        .user_styles
        .clone();
    let all = crate::library_browser::catalogue::presets("", &users, false);
    let output = screen.draw(vec![]);
    assert_eq!(presets_listed(&output), all.len());
    assert!(!checkbox_checked(&output, "In 4/4"));

    // A 3/4 waltz goes; a style that plays each bar in its own meter stays.
    let in_four = |preset: &crate::library_browser::catalogue::Preset| {
        preset.follows_the_song() || preset.time().ungrouped() == TimeSignature::new(4, 4)
    };
    screen.click("In 4/4");
    let output = screen.draw(vec![]);
    assert!(checkbox_checked(&output, "In 4/4"));
    let kept = all.iter().filter(|preset| in_four(preset)).count();
    assert!(kept < all.len() && kept > 0);
    assert_eq!(presets_listed(&output), kept);

    // 120 BPM keeps what was made for 104 to 138, 15% either way.
    let near = |preset: &crate::library_browser::catalogue::Preset| {
        (120.0 / 1.15..=120.0 * 1.15).contains(&preset.tempo())
    };
    screen.click("Near 120 BPM");
    let output = screen.draw(vec![]);
    let kept_now = all
        .iter()
        .filter(|preset| in_four(preset) && near(preset))
        .count();
    assert!(kept_now < kept && kept_now > 0);
    assert_eq!(presets_listed(&output), kept_now);

    // The boxes stay ticked when the app starts again.
    screen.restart();
    screen.app.project.time_signature = TimeSignature::new(4, 4);
    screen.app.project.tempo = Tempo::new(10.0);
    screen.app.rack_open = false;
    screen.app.playlist.browser_open = true;
    screen.click("📚 Library");
    let output = screen.draw(vec![]);
    assert!(checkbox_checked(&output, "In 4/4"));
    assert!(checkbox_checked(&output, "Near 10 BPM"));
    // Nothing is made for 10 BPM: the Library says how to see more.
    assert_eq!(presets_listed(&output), 0);
    assert!(text_rect(&output, "Nothing in the library matches the ticked boxes.").is_some());
    assert!(text_rect(&output, "Untick a box to see more.").is_some());

    screen.click("Near 10 BPM");
    let output = screen.draw(vec![]);
    assert_eq!(presets_listed(&output), kept);
}

#[test]
fn f1_and_the_help_menu_show_the_keyboard_shortcuts() {
    let mut screen = Screen::new();
    let output = screen.draw(vec![]);
    assert!(text_rect(&output, "Keyboard shortcuts").is_none());

    screen.key(Key::F1);
    let output = screen.draw(vec![]);
    assert!(screen.app.shortcuts_open);
    assert!(text_rect(&output, "Keyboard shortcuts").is_some());
    assert!(text_rect(&output, "Home").is_some());
    assert!(text_rect(&output, "Go to the start").is_some());

    screen.key(Key::F1);
    let output = screen.draw(vec![]);
    assert!(!screen.app.shortcuts_open);
    assert!(text_rect(&output, "Keyboard shortcuts").is_none());

    screen.click("Help");
    screen.click("Keyboard shortcuts");
    assert!(screen.app.shortcuts_open);
}

#[test]
fn help_about_shows_the_version_the_license_and_where_the_book_is() {
    let mut screen = Screen::new();
    screen.app.rack_open = false;
    screen.click("Help");
    screen.click("About Ferrous Lineup");
    assert!(screen.app.about_open);
    let output = screen.draw(vec![]);
    let version = format!("Version {}", env!("CARGO_PKG_VERSION"));
    for text in [
        "Ferrous Lineup",
        &version,
        "Read the license",
        "The book",
        "The book (PDF)",
        "Source code",
        "Report an issue",
    ] {
        assert!(text_rect(&output, text).is_some(), "{text}");
    }
    // A link opens its page in the browser.
    let book = text_rect(&output, "The book").unwrap().center();
    screen.draw(vec![
        Event::PointerMoved(book),
        Event::PointerButton {
            pos: book,
            button: PointerButton::Primary,
            pressed: true,
            modifiers: Modifiers::NONE,
        },
    ]);
    let output = screen.draw(vec![Event::PointerButton {
        pos: book,
        button: PointerButton::Primary,
        pressed: false,
        modifiers: Modifiers::NONE,
    }]);
    let opened: Vec<&str> = output
        .platform_output
        .commands
        .iter()
        .filter_map(|command| match command {
            egui::OutputCommand::OpenUrl(open) => Some(open.url.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(opened, ["https://naelolaiz.github.io/ferrous-lineup/"]);
}

#[test]
fn main_transport_toggles_are_visible_reversible_and_saved() {
    const TOGGLES: [&str; 3] = ["Follow playhead", "Count-in", "Metronome"];
    for size in [vec2(1400.0, 1000.0), vec2(800.0, 600.0)] {
        let mut screen = Screen::new();
        screen.size = size;
        screen.ctx.set_zoom_factor(1.1);
        let project = screen.app.project.clone();
        screen.draw(vec![]);
        screen.draw(vec![]);
        let output = screen.draw(vec![]);
        for name in TOGGLES {
            let rect = icon_rect(&screen, name);
            assert!(screen.ctx.content_rect().contains_rect(rect));
        }
        assert!(icon_lit(&output, &screen, "Follow playhead"));
        assert!(!icon_lit(&output, &screen, "Count-in"));
        assert!(!icon_lit(&output, &screen, "Metronome"));
        for name in TOGGLES {
            screen.click_icon(name);
        }
        let output = screen.draw(vec![]);
        assert!(!icon_lit(&output, &screen, "Follow playhead"));
        assert!(icon_lit(&output, &screen, "Count-in"));
        assert!(icon_lit(&output, &screen, "Metronome"));
        assert!(!widgets::follow_playback(&screen.ctx));
        let restored = practice::Practice::new(&screen.ctx).view();
        assert!(restored.count_in && restored.metronome);
        for name in TOGGLES {
            screen.click_icon(name);
        }
        assert!(widgets::follow_playback(&screen.ctx));
        assert!(!screen.app.practice.view().count_in && !screen.app.practice.view().metronome);
        assert_eq!(screen.app.project, project);
        assert!(!screen.app.playing);
    }
}

#[test]
fn the_toolbar_fits_on_one_row_at_1280_points() {
    let mut screen = Screen::new();
    screen.size = vec2(1280.0, 800.0);
    screen.draw(vec![]);
    let output = screen.draw(vec![]);
    let row = text_rect(&output, "PAT").unwrap().center().y;
    let mut right: f32 = 0.0;
    for label in ["SONG", "TEMPO", "BAR : BEAT : STEP", "PATTERN", "VOL"] {
        let rect = text_rect(&output, label).unwrap_or_else(|| panic!("{label} missing"));
        assert!(
            (rect.center().y - row).abs() < 16.0,
            "{label} is on another row"
        );
        right = right.max(rect.right());
    }
    for name in [
        "Follow playhead",
        "Count-in",
        "Metronome",
        "Chord sheet",
        "Channel rack",
        "Piano roll",
        "Harmony keys",
        "Mixer",
    ] {
        let rect = icon_rect(&screen, name);
        assert!(
            (rect.center().y - row).abs() < 16.0,
            "{name} is on another row"
        );
        right = right.max(rect.right());
    }
    assert!(right < 1280.0);
}

#[test]
fn the_window_buttons_open_and_close_their_windows() {
    let mut screen = Screen::new();
    screen.app.rack_open = false;
    let windows = |app: &LineupApp| {
        [
            app.chord_sheet.open,
            app.rack_open,
            app.piano_roll.open,
            app.harmony_keys.open,
            app.mixer_open,
        ]
    };
    assert_eq!(windows(&screen.app), [false; 5]);
    let names = [
        "Chord sheet",
        "Channel rack",
        "Piano roll",
        "Harmony keys",
        "Mixer",
    ];
    for (index, name) in names.into_iter().enumerate() {
        screen.click_icon(name);
        let mut open = [false; 5];
        open[..=index].fill(true);
        assert_eq!(windows(&screen.app), open, "after {name}");
        let output = screen.draw(vec![]);
        assert!(icon_lit(&output, &screen, name));
    }
    for name in names {
        screen.click_icon(name);
    }
    assert_eq!(windows(&screen.app), [false; 5]);
}

#[test]
fn settings_style_controls_toggle_and_plain_accents_hides_instruments() {
    let mut screen = Screen::new();
    screen.app.settings_open = true;
    let project = screen.app.project.clone();
    screen.click("Library");
    for label in [
        "Patterns",
        "Instruments",
        "Chords",
        "Meter",
        "Tempo",
        "Swing",
    ] {
        let before = *screen.app.playlist.library_parts_mut();
        screen.click(label);
        assert_ne!(
            *screen.app.playlist.library_parts_mut(),
            before,
            "{label} must change a preference"
        );
        screen.click(label);
        assert_eq!(*screen.app.playlist.library_parts_mut(), before);
    }
    screen.click("Plain accents");
    let output = screen.draw(vec![]);
    assert!(!screen.app.playlist.library_parts().matching_style);
    assert!(!screen.app.playlist.library_parts().instruments);
    assert!(text_rect(&output, "Instruments").is_none());
    assert!(text_rect(&output, "Accents + drums").is_some());
    screen.scroll_to("Edit defaults…");
    screen.click("Edit defaults…");
    let output = screen.draw(vec![]);
    assert!(text_rect(&output, "Voices / instruments").is_none());
    assert!(text_rect(&output, "Drums and plain accents").is_some());
    screen.click("Preset patterns");
    let output = screen.draw(vec![]);
    assert!(checkbox_checked(&output, "Voices / instruments"));
    screen.click("Cancel");
    assert!(!screen.app.playlist.library_parts().matching_style);
    assert_eq!(screen.app.project, project);
}

#[test]
fn presets_without_chords_preserve_both_remembered_progression_choices() {
    for (remembered, saved) in [(false, false), (true, false), (false, true), (true, true)] {
        let mut screen = Screen::new();
        screen.app.playlist.browser_open = true;
        let preset = if saved { "No chart" } else { "Pop beat" };
        if saved {
            let mut style = lineup_core::library::UserStyle::blank();
            style.name = preset.into();
            style.chart.clear();
            screen
                .app
                .playlist
                .library_browser_mut()
                .user_styles
                .push(style);
        }
        let chords = screen.app.project.playlist().chord_markers().to_vec();
        screen.app.playlist.library_parts_mut().progression = remembered;
        screen.click("📚 Library");
        screen.click("🔍 Search: bossa, 6/8, tango…");
        screen.draw(vec![Event::Text(preset.into())]);
        screen.click(preset);
        let output = screen.draw(vec![]);
        assert!(!checkbox_checked(&output, "Chord progression"));
        screen.click("Chord progression");
        screen.click("Apply");
        assert_eq!(screen.app.project.playlist().chord_markers(), chords);
        if saved {
            assert_eq!(screen.app.project.band().unwrap().name(), preset);
        }
        assert_eq!(
            screen.app.playlist.library_parts_mut().progression,
            remembered
        );
        screen.click("×");
        screen.click("🔍 Search: bossa, 6/8, tango…");
        screen.draw(vec![Event::Text("Royal road".into())]);
        screen.click("Royal road");
        let output = screen.draw(vec![]);
        assert_eq!(checkbox_checked(&output, "Chord progression"), remembered);
        screen.click("Cancel");
        assert_eq!(
            screen.app.playlist.library_parts_mut().progression,
            remembered
        );
    }
}

fn band_notes(
    project: &Project,
    player: lineup_core::Player,
) -> Vec<(usize, String, lineup_core::NoteEvent)> {
    project
        .patterns()
        .iter()
        .filter(|pattern| pattern.player == Some(player))
        .flat_map(|pattern| {
            pattern.all_notes().map(|(channel, note)| {
                (
                    pattern.length(),
                    project.channels()[channel].name.clone(),
                    *note,
                )
            })
        })
        .collect()
}

fn meter_test_screen(live_band: bool, rhythm: bool) -> Screen {
    use lineup_core::{Channel, Note, NoteEvent, library::apply::StyleParts};
    let mut screen = Screen::new();
    let mut project = if live_band {
        let mut project = Project::with_band();
        project.write_progression(
            lineup_core::library::Style::named("Four chords").unwrap(),
            0,
        );
        project.follow_chord_sheet();
        project
    } else {
        Project::demo()
    };
    let lead = project
        .add_channel(Channel {
            name: "Lead".into(),
            ..Channel::synth()
        })
        .unwrap();
    project
        .pattern_mut(0)
        .unwrap()
        .set_notes(lead, [NoteEvent::exact(Note::new(60), 2, 100)]);
    let markers =
        lineup_core::chord_sheet::read_bar("C G", &project.meter().bar(0), project.scale).unwrap();
    project.set_chords(0, 16, &markers);
    project.follow_chord_sheet();
    screen.app.replace_project(project, None);
    *screen.app.playlist.library_parts_mut() = StyleParts {
        rhythm,
        instruments: false,
        time_signature: true,
        progression: false,
        tempo: false,
        swing: false,
        matching_style: false,
    };
    screen
}

fn melody(project: &Project) -> Vec<lineup_core::NoteEvent> {
    let lead = project
        .channels()
        .iter()
        .position(|channel| channel.name == "Lead")
        .unwrap();
    project
        .patterns()
        .iter()
        .flat_map(|pattern| pattern.notes(lead).iter().copied())
        .collect()
}

#[test]
fn meter_changes_without_matching_style_use_plain_drums_and_keep_the_requested_meter() {
    use lineup_core::{
        Player, TimeSignature,
        library::{Rhythm, UserStyle, apply},
    };
    for live_band in [false, true] {
        for time in [
            TimeSignature::new(3, 4),
            TimeSignature::new(7, 8),
            "2+3+2/8".parse().unwrap(),
            TimeSignature::new(13, 16),
        ] {
            let mut screen = meter_test_screen(live_band, true);
            let before = screen.app.project.clone();
            screen.app.set_song_time(time);
            assert_eq!(screen.app.project.time_signature, time);
            for channel in before.channels() {
                assert!(
                    screen.app.project.channels().contains(channel),
                    "Keep the original instruments"
                );
            }
            let rhythm = Rhythm::basic(time);
            if live_band {
                let band = screen.app.project.band().unwrap();
                assert!(band.accents_only);
                assert_eq!(
                    band.instrument_style,
                    before.band().unwrap().instrument_style
                );
                assert_eq!(band.custom_rhythm.as_ref().unwrap().name, "Basic beat");
                let mut expected = before.clone();
                lineup_core::remeter::remeter(&mut expected, time);
                expected
                    .apply_user_style(
                        &UserStyle::from_rhythm(rhythm),
                        screen.app.playlist.library_parts(),
                    )
                    .unwrap();
                expected.follow_chord_sheet();
                assert_eq!(
                    band_notes(&screen.app.project, Player::Drums),
                    band_notes(&expected, Player::Drums)
                );
            } else {
                assert_eq!(screen.app.song_library.rhythm, Some(rhythm));
                let mut expected = before.clone();
                apply::use_rhythm_parts(&mut expected, rhythm, screen.app.playlist.library_parts());
                assert_eq!(screen.app.project, expected);
            }
            let text = lineup_core::file::to_text(&screen.app.project).unwrap();
            assert_eq!(
                lineup_core::file::from_text(&text).unwrap(),
                screen.app.project
            );
        }
    }
}

#[test]
fn meter_changes_start_from_the_original_timing_and_restore_the_original_song() {
    use lineup_core::TimeSignature;
    for live_band in [false, true] {
        for (rhythm, matching_style) in [(false, false), (true, false), (false, true), (true, true)]
        {
            let mut screen = meter_test_screen(live_band, rhythm);
            screen.app.playlist.library_parts_mut().matching_style = matching_style;
            let original = screen.app.project.clone();
            let final_time = TimeSignature::new(5, 4);
            let mut direct = meter_test_screen(live_band, rhythm);
            direct.app.playlist.library_parts_mut().matching_style = matching_style;
            direct.app.set_song_time(final_time);
            for time in [
                TimeSignature::new(3, 4),
                TimeSignature::new(7, 8),
                final_time,
            ] {
                screen.app.set_song_time(time);
                screen.app.history.close();
            }
            assert_eq!(melody(&screen.app.project), melody(&direct.app.project));
            assert_eq!(
                screen.app.project.playlist().chord_markers(),
                direct.app.project.playlist().chord_markers()
            );
            assert_eq!(screen.app.project.tempo, direct.app.project.tempo);
            let changed = screen.app.project.clone();
            screen.app.set_song_time(TimeSignature::COMMON);
            screen.app.history.close();
            assert_eq!(
                screen.app.project, original,
                "Round trip, band={live_band}, rhythm={rhythm}"
            );
            screen.app.undo();
            assert_eq!(screen.app.project, changed);
            screen.app.redo();
            assert_eq!(screen.app.project, original);
            screen.app.undo();
            screen.app.set_song_time(final_time);
            assert_eq!(melody(&screen.app.project), melody(&direct.app.project));
            assert_eq!(
                screen.app.project.playlist().chord_markers(),
                direct.app.project.playlist().chord_markers()
            );
        }
    }
}

/// The melody of the song's first seconds as it is played: when each
/// note starts and how long it lasts, in seconds (see [`super::trace`]).
/// A low sample rate is enough to time the notes, and quick.
fn played_melody(app: &mut LineupApp) -> Vec<(f64, f64)> {
    const RATE: u32 = 8000;
    const SECONDS: f64 = 12.0;
    let lead = app
        .project
        .channels()
        .iter()
        .position(|channel| channel.name == "Lead")
        .unwrap();
    let mut render = app.song_render_traced(RATE);
    render.renderer.skip((f64::from(RATE) * SECONDS) as usize);
    render
        .renderer
        .take_trace()
        .unwrap_or_default()
        .notes()
        .into_iter()
        .filter(|note| note.channel == lead && note.start < SECONDS - 2.0)
        .map(|note| (note.start, note.length.unwrap_or_default()))
        .collect()
}

#[test]
fn styles_tried_one_after_another_keep_the_songs_tempo() {
    use crate::library_browser::LibraryAction;
    use lineup_core::TimeSignature;
    use lineup_core::library::{Song, Style};
    let el_choclo = Song::named("El Choclo").unwrap();
    for (mode, playing) in [
        (PlayMode::Pattern, false),
        (PlayMode::Song, false),
        (PlayMode::Song, true),
    ] {
        let mut screen = Screen::new();
        screen.app.open_demo(library::Demo::Song(el_choclo));
        screen.app.set_mode(mode);
        screen.app.set_playing(playing);
        let tempo = screen.app.project.tempo;
        let time = screen.app.project.time_signature;
        let melody = played_melody(&mut screen.app);
        assert!(!melody.is_empty());
        let clicks: [&dyn Fn(&mut LineupApp); 7] = [
            &|app| app.try_from_library(LibraryAction::Style(Style::named("Four chords").unwrap())),
            &|app| {
                app.try_from_library(LibraryAction::Style(Style::named("Emotional pop").unwrap()))
            },
            &|app| app.try_from_library(LibraryAction::Style(Style::named("Tango vals").unwrap())),
            &|app| app.try_from_library(LibraryAction::Style(Style::named("Ranchera").unwrap())),
            &|app| {
                app.try_from_library(LibraryAction::Style(
                    Style::named("Six-eight ballad").unwrap(),
                ))
            },
            &|app| app.set_song_time(TimeSignature::new(7, 8)),
            &|app| app.try_from_library(LibraryAction::Style(Style::named("Four chords").unwrap())),
        ];
        for (click, step) in clicks.iter().zip(1..) {
            let before = (screen.app.project.time_signature, screen.app.project.tempo);
            click(&mut screen.app);
            screen.app.history.close();
            let project = &screen.app.project;
            let what = format!("{mode:?}, playing: {playing}, click {step}");
            if project.time_signature == before.0 {
                assert_eq!(project.tempo, before.1, "{what}");
            }
            if project.time_signature == time {
                assert_eq!(project.tempo, tempo, "{what}");
            }
            // The song's own notes last as long as they did, in any meter.
            assert_eq!(project.swing, 0, "{what}");
            let now = played_melody(&mut screen.app);
            assert_eq!(now.len(), melody.len(), "{what}");
            for (now, then) in now.iter().zip(&melody) {
                assert!((now.0 - then.0).abs() < 2e-3, "{what}: {now:?} {then:?}");
                assert!((now.1 - then.1).abs() < 2e-3, "{what}: {now:?} {then:?}");
            }
        }
    }
}

#[test]
fn a_ticked_tempo_comes_with_a_style_only_in_the_same_meter() {
    use crate::library_browser::LibraryAction;
    use lineup_core::library::{Song, Style};
    let mut screen = Screen::new();
    screen
        .app
        .open_demo(library::Demo::Song(Song::named("El Choclo").unwrap()));
    screen.app.set_playing(false);
    screen.app.set_mode(PlayMode::Pattern);
    screen.app.playlist.library_parts_mut().tempo = true;
    let four_chords = Style::named("Four chords").unwrap();
    screen
        .app
        .try_from_library(LibraryAction::Style(four_chords));
    assert_eq!(screen.app.project.tempo.bpm(), four_chords.tempo);
    // The trial pattern goes into 3/4, and the song keeps its tempo.
    screen.app.history.close();
    let waltz = Style::named("Tango vals").unwrap();
    screen.app.try_from_library(LibraryAction::Style(waltz));
    assert_eq!(
        screen.app.project.pattern_time(screen.app.pattern),
        waltz.time
    );
    assert_eq!(screen.app.project.tempo.bpm(), four_chords.tempo);
}

#[test]
fn the_time_picker_routes_unchecked_matching_style_to_plain_accents() {
    let mut screen = meter_test_screen(true, true);
    screen.app.rack_open = false;
    screen.app.harmony_keys.open = true;
    screen.app.playlist.library_parts_mut().matching_style = true;
    screen.draw(vec![]);
    let output = screen.draw(vec![]);
    let time_label = text_rect(&output, "Time").unwrap().center();
    let time_button = output
        .shapes
        .iter()
        .filter_map(|shape| match &shape.shape {
            Shape::Text(text) if text.galley.text() == "4/4" => {
                Some(Rect::from_min_size(text.pos, text.galley.size()).center())
            }
            _ => None,
        })
        .min_by(|a, b| {
            a.distance_sq(time_label)
                .total_cmp(&b.distance_sq(time_label))
        })
        .unwrap();
    screen.click_at(time_button, Modifiers::NONE);
    screen.click("Use matching style");
    screen.click("7/8");
    assert!(!screen.app.playlist.library_parts().matching_style);
    assert_eq!(
        screen.app.project.time_signature,
        lineup_core::TimeSignature::new(7, 8)
    );
    let band = screen.app.project.band().unwrap();
    assert!(band.accents_only);
    assert_eq!(band.custom_rhythm.as_ref().unwrap().name, "Basic beat");
}

#[test]
fn editing_a_remetered_song_establishes_a_new_reference() {
    use lineup_core::{Note, NoteEvent, TimeSignature};
    let mut screen = meter_test_screen(false, false);
    screen.app.set_song_time(TimeSignature::new(3, 4));
    let lead = screen
        .app
        .project
        .channels()
        .iter()
        .position(|channel| channel.name == "Lead")
        .unwrap();
    screen.app.edit("edit melody", |project| {
        project
            .pattern_mut(0)
            .unwrap()
            .set_notes(lead, [NoteEvent::exact(Note::new(67), 5, 80)]);
    });
    let edited = screen.app.project.clone();
    screen.app.set_song_time(TimeSignature::new(5, 4));
    screen.app.set_song_time(TimeSignature::new(3, 4));
    assert_eq!(screen.app.project, edited);
    screen.app.replace_project(Project::demo(), None);
    assert!(screen.app.song_sources.is_empty());
}

#[test]
fn later_meter_changes_keep_unchecked_voices_and_generated_beats_play_the_whole_band() {
    use lineup_core::{Player, TimeSignature};
    let mut screen = meter_test_screen(true, true);
    screen.app.playlist.library_parts_mut().matching_style = true;
    screen.app.playlist.library_parts_mut().instruments = true;
    screen.app.set_song_time(TimeSignature::new(7, 8));
    let instrument_style = screen.app.project.band().unwrap().instrument_style;
    assert_eq!(instrument_style.name, "Rachenitsa");
    screen.app.playlist.library_parts_mut().matching_style = false;
    screen.app.set_song_time(TimeSignature::new(5, 4));
    assert_eq!(
        screen.app.project.band().unwrap().instrument_style,
        instrument_style
    );
    let piano = screen.app.project.player_channel(Player::Piano).unwrap();
    assert_eq!(screen.app.project.channels()[piano].program, Some(21));
    screen.app.playlist.library_parts_mut().matching_style = true;
    screen.app.set_song_time(TimeSignature::new(13, 16));
    assert_eq!(
        screen.app.project.time_signature,
        TimeSignature::new(13, 16)
    );
    for player in [Player::Piano, Player::Bass, Player::Drums] {
        assert!(
            !band_notes(&screen.app.project, player).is_empty(),
            "Generated beat left {player:?} silent"
        );
    }
}

#[test]
fn library_meter_changes_preserve_the_source_timing_and_independent_voice_choices() {
    use lineup_core::{
        Player, TimeSignature,
        library::{Rhythm, UserStyle},
    };
    for live_band in [false, true] {
        for saved in [false, true] {
            let mut screen = meter_test_screen(live_band, true);
            screen.app.playlist.library_parts_mut().matching_style = true;
            screen.app.playlist.library_parts_mut().tempo = saved;
            let original = screen.app.project.clone();
            for name in ["Waltz", "Rachenitsa", "Five-four rock"] {
                let rhythm = Rhythm::named(name).unwrap();
                if saved {
                    screen.app.playlist.library_browser_mut().user_styles =
                        vec![UserStyle::from_rhythm(rhythm)];
                    screen.app.apply_user_library(0, None);
                } else {
                    screen.app.try_from_library(
                        crate::library_browser::LibraryAction::RhythmInSong(rhythm),
                    );
                }
            }
            let mut direct = original.clone();
            lineup_core::remeter::remeter(&mut direct, TimeSignature::new(5, 4));
            assert_eq!(melody(&screen.app.project), melody(&direct));
            if saved {
                assert_eq!(
                    screen.app.project.tempo.bpm(),
                    Rhythm::named("Five-four rock").unwrap().tempo
                );
            }
            assert_eq!(
                screen.app.project.playlist().chord_markers(),
                direct.playlist().chord_markers()
            );
            if let Some(piano) = screen.app.project.player_channel(Player::Piano) {
                assert_eq!(screen.app.project.channels()[piano].program, Some(0));
            }
        }
    }
}

#[test]
fn band_lineup_changes_keep_the_current_performance_and_support_undo_and_save() {
    use lineup_core::Player;
    let mut screen = Screen::new();
    let mut project = Project::with_band();
    project.write_progression(
        lineup_core::library::Style::named("Four chords").unwrap(),
        0,
    );
    project.follow_chord_sheet();
    screen.app.replace_project(project, None);
    screen.app.rack_open = false;
    screen.app.chord_sheet.open = true;
    // Band voices must be independent of the Library's remembered apply options.
    *screen.app.playlist.library_parts_mut() = lineup_core::library::apply::StyleParts {
        matching_style: false,
        progression: true,
        ..Default::default()
    };
    let defaults = *screen.app.playlist.library_parts_mut();
    let before = screen.app.project.clone();
    screen.click("🎷 Band");
    screen.click("Instrument lineup");
    screen.click("Electric piano + synth bass");
    let changed = screen.app.project.clone();
    assert_eq!(changed.band().unwrap().instrument_style.name, "Boom bap");
    assert_eq!(changed.band().unwrap().style, before.band().unwrap().style);
    assert_eq!(
        changed.band().unwrap().harmony_style,
        before.band().unwrap().harmony_style
    );
    assert_eq!(
        changed.playlist().chord_markers(),
        before.playlist().chord_markers()
    );
    assert_eq!(changed.meter(), before.meter());
    assert_eq!(changed.tempo, before.tempo);
    assert_eq!(changed.swing, before.swing);
    for player in [Player::Piano, Player::Bass, Player::Drums] {
        assert_eq!(band_notes(&changed, player), band_notes(&before, player));
    }
    assert_eq!(*screen.app.playlist.library_parts_mut(), defaults);
    let text = lineup_core::file::to_text(&changed).unwrap();
    let mut loaded = lineup_core::file::from_text(&text).unwrap();
    assert_eq!(loaded, changed);
    assert!(!loaded.follow_chord_sheet());
    screen.app.undo();
    assert_eq!(screen.app.project, before);
    screen.app.redo();
    assert_eq!(screen.app.project, changed);
}

#[test]
fn band_player_and_voice_controls_change_only_the_requested_track_or_sound() {
    use lineup_core::Player;
    let mut screen = Screen::new();
    let mut project = Project::with_band();
    project.write_progression(
        lineup_core::library::Style::named("Four chords").unwrap(),
        0,
    );
    project.follow_chord_sheet();
    screen.app.replace_project(project, None);
    screen.app.rack_open = false;
    screen.app.chord_sheet.open = true;
    let before = screen.app.project.clone();
    screen.click("🎷 Band");
    screen.click("Piano");
    let mut muted = before.clone();
    muted.set_you_play(Player::Piano, true);
    assert_eq!(screen.app.project, muted);
    screen.click("Piano");
    assert_eq!(screen.app.project, before);
    screen.click("Acoustic Grand Piano");
    screen.click("Piano");
    screen.click("Electric Piano 2");
    let mut voiced = before.clone();
    let channel = voiced.player_channel(Player::Piano).unwrap();
    voiced.channel_mut(channel).unwrap().program = Some(5);
    assert_eq!(screen.app.project, voiced);
    screen.app.project.follow_chord_sheet();
    assert_eq!(screen.app.project, voiced);
}

#[test]
fn library_presets_open_their_options_and_apply_in_the_full_app() {
    for grouped in [false, true] {
        let mut screen = Screen::new();
        crate::library_browser::set_grouped_presets(&screen.ctx, grouped);
        screen.app.playlist.browser_open = true;
        screen.click("📚 Library");
        screen.click("🔍 Search: bossa, 6/8, tango…");
        screen.draw(vec![Event::Text("Aeolian climb".into())]);
        let before = screen.app.project.clone();
        screen.click("Aeolian climb");
        screen.draw(vec![]);
        let output = screen.draw(vec![]);
        assert!(
            egui::Popup::is_any_open(&screen.ctx),
            "Preset popup stays open after clicking"
        );
        assert!(text_rect(&output, "Playing patterns").is_some());
        screen.click("Apply");
        assert_ne!(screen.app.project, before);
    }
}

#[test]
fn settings_owns_preferences_and_view_keeps_window_actions() {
    let mut screen = Screen::new();
    let output = screen.draw(vec![]);
    assert!(text_rect(&output, "Sounds").is_none());
    assert!(text_rect(&output, "Practice").is_none());
    screen.click("View");
    let output = screen.draw(vec![]);
    assert!(text_rect(&output, "Settings…").is_none());
    assert!(text_rect(&output, "Channel rack").is_some());
    assert!(text_rect(&output, "Interface size (100%)").is_none());
    assert!(!output.shapes.iter().any(|shape| matches!(&shape.shape,
        Shape::Text(text) if text.galley.text().contains("zoom:")
    )));
    screen.key(Key::Escape);
    screen.click("Edit");
    screen.click("Settings…");
    let output = screen.draw(vec![]);
    assert!(text_rect(&output, "Editor zoom").is_some());
    assert!(text_rect(&output, "SoundFont instruments").is_none());
    assert!(text_rect(&output, "Playing patterns").is_none());
    screen.click("Audio");
    let output = screen.draw(vec![]);
    assert!(text_rect(&output, "SoundFont instruments").is_some());
    assert!(text_rect(&output, "Editor zoom").is_none());
    screen.click("Playback");
    screen.click("Count-in");
    screen.click("Metronome");
    assert!(screen.app.practice.view().count_in);
    assert!(screen.app.practice.view().metronome);
}

#[test]
fn the_speed_trainer_speeds_up_each_lap_and_stop_starts_it_again() {
    let mut screen = Screen::new();
    screen.app.rack_open = false;
    screen.app.chord_sheet.open = true;
    let project = screen.app.project.clone();
    screen.click("Speed trainer");
    let practice = screen.app.practice.view();
    assert!(practice.training);
    assert_eq!(practice.speed, 70, "the trainer starts at its first speed");
    let output = screen.draw(vec![]);
    assert!(
        text_rect(&output, "every").is_some(),
        "its speeds show beside it"
    );
    // There is no sound card here: hand over the laps the audio thread
    // would count as the loop comes round.
    screen.app.playing = true;
    screen.app.train(0);
    assert_eq!(screen.app.practice.view().speed, 70);
    screen.app.train(1);
    assert_eq!(screen.app.practice.view().speed, 75);
    screen.app.train(3);
    assert_eq!(screen.app.practice.view().speed, 85);
    // A speed chosen by hand is where it carries on from.
    screen.app.set_speed(90);
    screen.app.train(4);
    assert_eq!(screen.app.practice.view().speed, 95);
    screen.app.train(9);
    assert_eq!(screen.app.practice.view().speed, 100, "the target holds");
    // Pause keeps the speed; Stop starts the next run slow again.
    screen.app.pause();
    assert_eq!(screen.app.practice.view().speed, 100);
    screen.app.playing = true;
    screen.app.set_playing(false);
    assert_eq!(screen.app.practice.view().speed, 70);
    // A new first speed, while stopped, is the speed Play starts at, and
    // the trainer's speeds are remembered.
    let trainer = chord_sheet::Trainer {
        from: 60,
        to: 90,
        by: 10,
        every: 2,
    };
    screen.app.set_trainer(trainer);
    assert_eq!(screen.app.practice.view().speed, 60);
    screen.draw(vec![]);
    assert_eq!(practice::Practice::new(&screen.ctx).view().trainer, trainer);
    // Off, the speed stays where it got to, and laps change nothing.
    screen.click("Speed trainer");
    assert!(!screen.app.practice.view().training);
    screen.app.train(20);
    assert_eq!(screen.app.practice.view().speed, 60);
    // None of it is part of the song.
    assert_eq!(screen.app.project, project);
}

#[test]
fn style_defaults_edit_separately_and_only_save_on_confirmation() {
    let mut screen = Screen::new();
    screen.app.settings_open = true;
    let project = screen.app.project.clone();
    let original = *screen.app.playlist.library_parts_mut();
    screen.click("Library");
    screen.click("Group by genre");
    assert!(crate::library_browser::grouped_presets(&screen.ctx));
    let output = screen.draw(vec![]);
    assert!(text_rect(&output, "Style application defaults").is_some());
    assert!(text_rect(&output, "Playing patterns").is_none());
    screen.click("Edit defaults…");
    screen.click("Tempo");
    screen.click("Cancel");
    assert_eq!(*screen.app.playlist.library_parts_mut(), original);
    screen.click("Edit defaults…");
    screen.click("Chord progression");
    screen.click("Save defaults");
    let saved = lineup_core::library::apply::StyleParts {
        progression: true,
        ..original
    };
    assert_eq!(*screen.app.playlist.library_parts_mut(), saved);
    assert_eq!(screen.app.project, project);
    assert_eq!(
        screen.ctx.data_mut(
            |data| data.get_persisted::<lineup_core::library::apply::StyleParts>(egui::Id::unique(
                "library-options"
            ))
        ),
        Some(saved)
    );
}

const MINOR_SCALE_NUMERALS: &str = "III VI VII: counted from the minor scale (classical harmony)";

#[test]
fn minor_key_numerals_follow_the_harmony_setting() {
    use lineup_core::harmony::{MinorNumerals, Scale, ScaleKind};
    // Restores the numerals of this thread when the test ends.
    MinorNumerals::FromMajor.with(|| {
        let mut screen = Screen::new();
        screen.app.project.scale = Scale::new(lineup_core::PitchClass::A, ScaleKind::NaturalMinor);
        screen.app.harmony_keys.open = true;
        screen.draw(vec![]);
        let output = screen.draw(vec![]);
        for numeral in ["i", "bIII", "bVI", "bVII"] {
            assert!(text_rect(&output, numeral).is_some(), "{numeral} missing");
        }
        assert!(text_rect(&output, "III").is_none());
        screen.app.settings_open = true;
        screen.click("Harmony");
        screen.click(MINOR_SCALE_NUMERALS);
        screen.click("Done");
        let output = screen.draw(vec![]);
        for numeral in ["i", "III", "VI", "VII"] {
            assert!(text_rect(&output, numeral).is_some(), "{numeral} missing");
        }
        assert!(text_rect(&output, "bVII").is_none());
        assert_eq!(
            screen.ctx.data_mut(
                |data| data.get_persisted::<MinorNumerals>(egui::Id::unique("minor-numerals"))
            ),
            Some(MinorNumerals::FromMinor)
        );
    });
}

#[test]
fn settings_navigation_and_footer_stay_visible_on_small_screens() {
    let mut screen = Screen::new();
    screen.size = vec2(640.0, 480.0);
    screen.app.settings_open = true;
    screen.draw(vec![]);
    let output = screen.draw(vec![]);
    let footer = text_rect(&output, "Done").unwrap();
    for page in ["Appearance", "Harmony", "Library", "Audio", "Playback"] {
        screen.click(page);
        let output = screen.draw(vec![]);
        let done = text_rect(&output, "Done").expect("Settings footer is visible");
        assert!(Rect::from_min_size(Pos2::ZERO, screen.size).contains_rect(done));
        assert!(
            (done.bottom() - footer.bottom()).abs() < 1.0,
            "Changing categories must not grow the window"
        );
        for page in ["Appearance", "Harmony", "Library", "Audio", "Playback"] {
            assert!(
                text_rect(&output, page).is_some(),
                "Missing settings category {page}"
            );
        }
    }
    screen.click("Done");
    assert!(!screen.app.settings_open);
}

#[test]
fn settings_categories_never_clip_controls_horizontally() {
    for size in [vec2(1385.0, 748.0), vec2(640.0, 480.0)] {
        let mut screen = Screen::new();
        screen.size = size;
        screen.settings_only = true;
        screen.app.settings_open = true;
        screen.ctx.set_zoom_factor(1.1);
        for page in [
            "Appearance",
            "Harmony",
            "Library",
            "Audio",
            "Playback",
            "Edit defaults…",
        ] {
            if page == "Edit defaults…" {
                screen.click("Library");
                screen.scroll_to("Edit defaults…");
            }
            screen.click(page);
            match page {
                "Appearance" => screen.scroll_to("Ctrl+0"),
                "Harmony" => screen.scroll_to(MINOR_SCALE_NUMERALS),
                "Library" => screen.scroll_to("Styles folder…"),
                "Audio" => screen.scroll_to("Choose SoundFont…"),
                "Playback" => screen.scroll_to("Practice speed"),
                _ => {}
            }
            let output = screen.draw(vec![]);
            let mut labels = 0;
            for shape in output.shapes {
                if let Shape::Text(text) = shape.shape {
                    let rect = Rect::from_min_size(text.pos, text.galley.size());
                    let clip = shape.clip_rect;
                    assert!(
                        rect.left() >= clip.left() - 1.0 && rect.right() <= clip.right() + 1.0,
                        "{page} at {size:?}: clipped {} ({rect:?}, clip {clip:?})",
                        text.galley.text()
                    );
                    labels += 1;
                }
            }
            assert!(labels > 10);
        }
    }
}

#[test]
fn rack_channels_switch_deselect_and_clear_with_escape() {
    let mut screen = Screen::new();
    let before = screen.app.project.clone();
    let first = screen.app.project.channels()[0].name.clone();
    let second = screen.app.project.channels()[1].name.clone();
    screen.click(&first);
    assert_eq!(screen.app.piano_roll.selected_channel(), Some(0));
    screen.click(&second);
    assert_eq!(screen.app.piano_roll.selected_channel(), Some(1));
    screen.click(&second);
    assert_eq!(screen.app.piano_roll.selected_channel(), None);
    screen.click(&first);
    assert_eq!(screen.app.piano_roll.selected_channel(), Some(0));
    screen.key(Key::Escape);
    assert_eq!(screen.app.piano_roll.selected_channel(), None);
    assert_eq!(
        screen.app.project, before,
        "channel selection must not edit the song"
    );
}

#[test]
fn rack_transport_and_space_play_pause_and_stop_the_current_pattern() {
    let mut screen = Screen::new();
    screen.click("▶ Play pattern");
    assert_eq!(screen.app.mode, PlayMode::Pattern);
    assert!(screen.app.playing);
    screen.app.cursor = 4; // No sound card to advance the playhead in this test.
    screen.click("⏸ Pause pattern");
    assert!(!screen.app.playing);
    assert_eq!(screen.app.cursor, 4);
    screen.click("■ Stop pattern");
    assert_eq!(screen.app.cursor, 0);
    screen.key(Key::Space);
    assert!(screen.app.playing);
    screen.key(Key::Space);
    assert!(!screen.app.playing);
}

#[test]
fn the_rack_plays_its_pattern_over_the_playing_song_until_clicked_again() {
    let mut screen = Screen::new();
    assert!(screen.app.project.patterns().len() > 1);
    screen.app.set_mode(PlayMode::Song);
    screen.app.set_playing(true);
    // The rack's first frame only measures it.
    screen.draw(Vec::new());
    let output = screen.draw(Vec::new());
    assert!(text_rect(&output, "▶ Play pattern").is_none());
    assert!(!button_lit(&output, "▶ Play over the song"));
    screen.click("▶ Play over the song");
    assert_eq!(screen.app.over_song, Some(screen.app.pattern));
    assert_eq!(screen.app.mode, PlayMode::Song, "the song plays on");
    assert!(screen.app.playing);
    let output = screen.draw(Vec::new());
    assert!(button_lit(&output, "▶ Play over the song"));
    // It plays the pattern the rack shows.
    screen.app.select_pattern(1);
    assert_eq!(screen.app.over_song, Some(1));
    screen.click("▶ Play over the song");
    assert_eq!(screen.app.over_song, None, "clicked again, it stops");
    assert!(screen.app.playing, "the song plays on");
    screen.click("▶ Play over the song");
    screen.click("■ Stop pattern");
    assert_eq!(screen.app.over_song, None, "Stop pattern stops it too");
    assert!(screen.app.playing);
    // Stopping the song ends it, and the rack plays the pattern again.
    screen.click("▶ Play over the song");
    screen.app.set_playing(false);
    let output = screen.draw(Vec::new());
    assert_eq!(screen.app.over_song, None);
    assert!(text_rect(&output, "▶ Play pattern").is_some());
}

#[test]
fn settings_keyboard_input_does_not_start_song_playback() {
    let mut screen = Screen::new();
    screen.app.cursor = 8;
    screen.app.marker = 8;
    screen.app.settings_open = true;
    screen.click("Appearance");
    screen.key(Key::Space);
    assert!(!screen.app.playing);
    screen.key(Key::Home);
    assert_eq!(screen.app.cursor, 8);
    screen.app.settings_open = false;
    screen.draw(vec![]);
    screen.key(Key::Home);
    assert_eq!(
        screen.app.cursor, 0,
        "Closing Settings releases transport shortcuts"
    );
}

#[test]
fn pattern_home_end_and_page_keys_use_the_pattern_boundaries() {
    let mut screen = shortcut_screen();
    screen.app.set_mode(PlayMode::Pattern);
    let before = screen.app.project.clone();
    screen.key(Key::End);
    assert_eq!((screen.app.cursor, screen.app.marker), (63, 63));
    screen.key(Key::PageDown);
    assert_eq!(screen.app.cursor, 63);
    screen.key(Key::PageUp);
    assert_eq!(screen.app.cursor, 48);
    screen.key(Key::Home);
    screen.key(Key::PageUp);
    assert_eq!((screen.app.cursor, screen.app.marker), (0, 0));
    assert_eq!(screen.app.project, before);
}

fn shortcut_screen() -> Screen {
    let mut screen = Screen::new();
    let mut project = Project::empty();
    project.add_channel(lineup_core::Channel::synth());
    let pattern = project.pattern_mut(0).unwrap();
    pattern.name = "Shortcut test".into();
    pattern.set_length(64);
    pattern.add_note(0, lineup_core::NoteEvent::new(Note::C4, 4, 4));
    pattern.add_note(0, lineup_core::NoteEvent::new(Note::new(67), 12, 4));
    project.add_clip(lineup_core::Clip::new(0, 0, 0));
    project.add_clip(lineup_core::Clip::new(0, 0, 512));
    screen.app.replace_project(project, None);
    screen.app.rack_open = false;
    screen.draw(vec![]);
    screen.draw(vec![]);
    screen
}

#[test]
fn timeline_shortcuts_reveal_boundaries_with_follow_on_or_off_and_keep_playback_state() {
    for follow in [false, true] {
        let mut screen = shortcut_screen();
        screen
            .ctx
            .data_mut(|data| data.insert_persisted(egui::Id::unique("follow-playback"), follow));
        let before = screen.app.project.clone();
        let pos = screen.app.playlist.area().center();
        screen.draw(vec![
            Event::PointerMoved(pos),
            Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: vec2(-1800.0, 0.0),
                phase: egui::TouchPhase::Move,
                source: egui::MouseWheelSource::Wheel,
                modifiers: Modifiers::NONE,
            },
        ]);
        for _ in 0..20 {
            screen.draw(vec![]);
        }
        let output = screen.draw(vec![]);
        assert!(
            text_rect(&output, "1").is_none(),
            "The timeline was scrolled away"
        );
        for modifiers in [Modifiers::NONE, Modifiers::COMMAND] {
            screen.key_with_modifiers(Key::Home, modifiers);
            assert_eq!((screen.app.cursor, screen.app.marker), (0, 0));
            assert!(text_rect(&screen.draw(vec![]), "1").is_some());
            screen.key_with_modifiers(Key::End, modifiers);
            let end = screen.app.loop_length() - 1;
            assert_eq!((screen.app.cursor, screen.app.marker), (end, end));
            assert!(text_rect(&screen.draw(vec![]), "36").is_some());
        }
        screen.key(Key::PageDown);
        assert_eq!(screen.app.cursor, screen.app.loop_length() - 1);
        screen.key(Key::PageUp);
        assert_eq!(screen.app.cursor, 560);
        screen.app.set_playing(true);
        screen.key(Key::Home);
        assert!(screen.app.playing);
        assert_eq!(screen.app.cursor, 0);
        assert_eq!(screen.app.project, before);
    }
}

#[test]
fn clicking_notes_and_clips_selects_for_delete_and_undo_restores_the_exact_project() {
    for note in [false, true] {
        for key in [Key::Delete, Key::Backspace] {
            let mut screen = shortcut_screen();
            let before = screen.app.project.clone();
            if note {
                screen.app.piano_roll.open_for(0);
                screen.draw(vec![]);
                screen.draw(vec![]);
                let output = screen.draw(vec![]);
                let label = screen.app.project.scale.note_name(Note::C4);
                let pos = output
                    .shapes
                    .iter()
                    .filter_map(|shape| match &shape.shape {
                        Shape::Text(text) if text.galley.text() == label => {
                            let rect = Rect::from_min_size(text.pos, text.galley.size());
                            let visible = shape.clip_rect.intersect(rect);
                            visible.is_positive().then_some(visible.center())
                        }
                        _ => None,
                    })
                    .max_by(|left, right| left.x.total_cmp(&right.x))
                    .expect("The C4 note is visible");
                screen.click_at(pos, Modifiers::NONE);
            } else {
                screen.click("Shortcut test");
            }
            assert_eq!(
                screen.app.project, before,
                "Selection must not edit the song"
            );
            screen.key(key);
            if note {
                assert_eq!(
                    screen.app.project.pattern(0).unwrap().notes(0),
                    &before.pattern(0).unwrap().notes(0)[1..]
                );
                assert_eq!(
                    screen.app.project.playlist().clips(),
                    before.playlist().clips()
                );
            } else {
                assert_eq!(
                    screen.app.project.playlist().clips().len(),
                    before.playlist().clips().len() - 1
                );
                assert_eq!(screen.app.project.patterns(), before.patterns());
            }
            let deleted = screen.app.project.clone();
            screen.key(key);
            assert_eq!(
                screen.app.project, deleted,
                "Delete without a selection does nothing"
            );
            screen.key_with_modifiers(Key::Z, Modifiers::COMMAND);
            assert_eq!(screen.app.project, before);
            screen.key_with_modifiers(Key::Y, Modifiers::COMMAND);
            assert_eq!(screen.app.project, deleted);
        }
    }
}

#[test]
fn rack_delete_removes_only_the_selected_channel_and_is_undoable() {
    let mut screen = Screen::new();
    let before = screen.app.project.clone();
    let channel = before.channels()[0].name.clone();
    screen.click(&channel);
    screen.key(Key::Delete);
    assert_eq!(screen.app.project.channels(), &before.channels()[1..]);
    assert_eq!(screen.app.piano_roll.selected_channel(), None);
    let deleted = screen.app.project.clone();
    screen.key(Key::Delete);
    assert_eq!(screen.app.project, deleted);
    screen.key_with_modifiers(Key::Z, Modifiers::COMMAND);
    assert_eq!(screen.app.project, before);
}

#[test]
fn sets_and_micromodes_can_switch_back_to_single_note_drawing() {
    for browser in ["Sets…", "Micromodes…"] {
        let mut screen = shortcut_screen();
        screen.app.piano_roll.open_for(0);
        screen.click(browser);
        screen.scroll_to_at("Draw with it", egui::pos2(600.0, 550.0));
        let before = screen.app.project.clone();
        screen.click("▶ Hear it");
        assert!(
            !screen.app.playing,
            "Preview does not start the song transport"
        );
        screen.click("Draw with it");
        assert!(button_lit(&screen.draw(vec![]), "Draw with it"));
        screen.click("Draw with it");
        assert!(!button_lit(&screen.draw(vec![]), "Draw with it"));
        assert!(button_lit(&screen.draw(vec![]), "Single note"));
        screen.click("Draw with it");
        screen.click("Single note");
        assert!(button_lit(&screen.draw(vec![]), "Single note"));
        assert!(!button_lit(&screen.draw(vec![]), "Draw with it"));
        assert_eq!(
            screen.app.project, before,
            "Drawing mode changes no notes or key"
        );
    }
}

#[test]
fn home_end_and_delete_edit_focused_text_without_moving_or_changing_the_song() {
    let mut screen = shortcut_screen();
    screen.app.playlist.browser_open = true;
    screen.app.cursor = 8;
    screen.app.marker = 8;
    screen.click("📚 Library");
    screen.click("🔍 Search: bossa, 6/8, tango…");
    screen.draw(vec![Event::Text("abc".into())]);
    let before = screen.app.project.clone();
    screen.key(Key::Home);
    screen.key(Key::Delete);
    assert!(text_rect(&screen.draw(vec![]), "bc").is_some());
    screen.key(Key::End);
    screen.key(Key::Backspace);
    assert!(text_rect(&screen.draw(vec![]), "b").is_some());
    assert_eq!((screen.app.cursor, screen.app.marker), (8, 8));
    assert_eq!(screen.app.project, before);
}

#[test]
fn theory_browser_actions_stay_reachable_in_short_zoomed_windows() {
    for browser in ["Sets…", "Micromodes…"] {
        let mut screen = shortcut_screen();
        screen.app.piano_roll.open_for(0);
        screen.click(browser);
        let before = screen.app.project.clone();
        screen.size = vec2(640.0, 480.0);
        screen.ctx.set_zoom_factor(1.25);
        screen.draw(vec![]);
        screen.draw(vec![]);
        let output = screen.draw(vec![]);
        let title = text_rect(
            &output,
            if browser == "Sets…" {
                "Pitch-class sets"
            } else {
                "Micromodes"
            },
        )
        .expect("Browser title remains visible");
        let scroll = title.center() + vec2(80.0, 80.0);
        for label in ["▶ Hear it", "Draw with it", "Single note", "Use as the key"] {
            screen.scroll_to_at(label, scroll);
            let output = screen.draw(vec![]);
            let rect = text_rect(&output, label).expect("Browser action is fully visible");
            assert!(screen.ctx.content_rect().contains_rect(rect));
        }
        screen.click("Draw with it");
        screen.click("Single note");
        assert_eq!(screen.app.project, before);
    }
}

#[test]
fn initial_space_plays_the_song_until_an_editor_is_clicked() {
    let mut screen = Screen::new();
    screen.draw(vec![]);
    screen.draw(vec![]);
    screen.key(Key::Space);
    assert_eq!(screen.app.mode, PlayMode::Song);
    assert!(screen.app.playing);
}

#[test]
fn the_unsaved_mark_is_worked_out_again_after_an_edit_and_undo() {
    let mut screen = Screen::new();
    assert!(!screen.app.has_unsaved_changes());
    screen
        .app
        .edit("change swing", |project| project.swing = 20);
    assert!(screen.app.has_unsaved_changes());
    screen.app.undo();
    assert!(!screen.app.has_unsaved_changes(), "undoing back is saved");
}

#[test]
fn changing_a_project_back_by_hand_clears_the_unsaved_mark() {
    let mut screen = Screen::new();
    let swing = screen.app.project.swing;
    screen
        .app
        .edit("change swing", |project| project.swing = swing + 20);
    assert!(screen.app.has_unsaved_changes());
    // A frame ends the step, as after a click.
    screen.draw(vec![]);
    screen
        .app
        .edit("change swing", |project| project.swing = swing);
    assert!(
        !screen.app.has_unsaved_changes(),
        "putting the swing back by hand is what was saved"
    );
}

#[test]
fn a_drag_that_ends_where_it_started_clears_the_unsaved_mark() {
    let mut screen = Screen::new();
    let swing = screen.app.project.swing;
    // The mouse button goes down on the empty end of the menu bar and
    // stays down, as in a drag.
    let pos = Pos2::new(1390.0, 8.0);
    let press = |pressed| Event::PointerButton {
        pos,
        button: PointerButton::Primary,
        pressed,
        modifiers: Modifiers::NONE,
    };
    screen.draw(vec![Event::PointerMoved(pos), press(true)]);
    screen
        .app
        .edit("drag", |project| project.swing = swing + 20);
    screen.draw(vec![]);
    screen.app.edit("drag", |project| project.swing = swing);
    screen.draw(vec![]);
    assert!(screen.app.history.is_open(), "the drag goes on");
    // Letting go ends the drag back where it started.
    screen.draw(vec![press(false)]);
    assert!(!screen.app.history.is_open());
    assert!(
        !screen.app.has_unsaved_changes(),
        "the drag put back what was saved"
    );
}

#[test]
fn changes_in_one_drag_are_one_undo_step() {
    let mut screen = Screen::new();
    let before = screen.app.project.clone();
    screen.app.edit("drag", |project| project.swing = 1);
    screen.app.edit("drag", |project| project.swing = 2);
    screen.app.edit("drag", |project| project.swing = 3);
    assert_eq!(screen.app.project.swing, 3);
    screen.app.undo();
    assert_eq!(screen.app.project, before, "one undo takes back the drag");
}

#[test]
fn a_drag_held_still_reaches_the_sound() {
    let mut screen = Screen::new();
    // The mouse button goes down on the empty end of the menu bar and
    // stays down, as in a drag.
    let pos = Pos2::new(1390.0, 8.0);
    screen.draw(vec![
        Event::PointerMoved(pos),
        Event::PointerButton {
            pos,
            button: PointerButton::Primary,
            pressed: true,
            modifiers: Modifiers::NONE,
        },
    ]);
    screen.app.edit("drag", |project| project.swing = 1);
    screen.app.edit("drag", |project| project.swing = 2);
    assert!(screen.app.step_unsynced, "the second change waits its turn");

    // The pointer is held still: no change follows, but its turn comes.
    screen.app.last_sync -= std::time::Duration::from_millis(100);
    screen.draw(vec![]);
    assert!(!screen.app.step_unsynced, "the waiting change was sent");
    assert_eq!(screen.app.synced, screen.app.project);
    assert!(screen.app.history.is_open(), "the drag is still one step");
}

#[test]
fn unsaved_work_from_a_crash_recovers_as_unsaved() {
    let mut screen = Screen::new();
    let recovered = Project::template();
    screen.app.recovery.pending = vec![super::recovery::Recovered::new(
        recovered.clone(),
        None,
        Some("Recovered song".to_string()),
    )];
    // The modal needs a frame to lay itself out.
    screen.draw(vec![]);
    let output = screen.draw(vec![]);
    assert!(text_rect(&output, "Recover unsaved work?").is_some());
    screen.click("Recover it");
    assert_eq!(screen.app.project, recovered);
    assert!(
        screen.app.has_unsaved_changes(),
        "recovered work has never been saved"
    );
}

#[test]
fn a_recovery_copy_can_be_discarded() {
    let mut screen = Screen::new();
    let before = screen.app.project.clone();
    screen.app.recovery.pending = vec![super::recovery::Recovered::new(
        Project::template(),
        None,
        None,
    )];
    screen.click("Discard it");
    assert!(screen.app.recovery.pending.is_empty());
    assert_eq!(screen.app.project, before);
}

#[test]
fn the_file_menu_lists_recent_projects() {
    let mut screen = Screen::new();
    screen.app.recent.add(&PathBuf::from("/music/Song.lineup"));
    screen.click("File");
    screen.click("Open recent");
    let output = screen.draw(vec![]);
    assert!(text_rect(&output, "Song.lineup").is_some());
}

#[test]
fn audio_settings_offer_a_retry_when_audio_is_unavailable() {
    let mut screen = Screen::new();
    screen.app.settings_open = true;
    screen.click("Audio");
    let output = screen.draw(vec![]);
    assert!(text_rect(&output, "Try again").is_some());
}

#[test]
fn save_confirmation_blocks_background_shortcuts_and_escape_keeps_the_project() {
    let mut screen = Screen::new();
    screen
        .app
        .edit("change swing", |project| project.swing = 20);
    let channel = screen.app.project.channels()[0].name.clone();
    screen.click(&channel);
    screen.app.cursor = 8;
    screen.app.marker = 8;
    let before = screen.app.project.clone();
    screen.key_with_modifiers(Key::N, Modifiers::COMMAND);
    assert!(text_rect(&screen.draw(vec![]), "Save changes?").is_some());
    for key in [
        Key::Space,
        Key::Delete,
        Key::Home,
        Key::End,
        Key::L,
        Key::F7,
    ] {
        screen.key(key);
        assert_eq!(
            screen.app.project, before,
            "{key:?} edits behind a confirmation"
        );
        assert_eq!((screen.app.cursor, screen.app.marker), (8, 8));
        assert!(!screen.app.playing && !screen.app.piano_roll.open);
    }
    screen.key_with_modifiers(Key::Z, Modifiers::COMMAND);
    assert_eq!(screen.app.project, before);
    assert_eq!((screen.app.cursor, screen.app.marker), (8, 8));
    assert!(!screen.app.playing && !screen.app.piano_roll.open);
    screen.key(Key::Escape);
    assert!(text_rect(&screen.draw(vec![]), "Save changes?").is_none());
    assert_eq!(screen.app.project, before);
    screen.key(Key::Delete);
    assert_eq!(
        screen.app.project.channels().len(),
        before.channels().len() - 1
    );
}

#[test]
fn moving_selected_clips_is_one_undoable_edit() {
    let mut screen = Screen::new();
    let before = screen.app.project.clone();
    let clips = before.playlist().clips();
    screen.app.apply_playlist(PlaylistAction::MoveClips {
        indices: vec![0, 1],
        tracks: 1,
        steps: 16,
    });
    for (index, old) in clips.iter().take(2).enumerate() {
        let moved = screen.app.project.playlist().clips()[index];
        assert_eq!((moved.track, moved.start), (old.track + 1, old.start + 16));
    }
    screen.app.undo();
    assert_eq!(screen.app.project, before);
}

#[test]
fn row_window_saves_a_separate_pattern_and_still_writes_the_current_one() {
    use lineup_core::harmony::RowPreset;
    use lineup_core::serial::{RowWriting, write_row};
    let mut screen = Screen::new();
    let channel = first_synth(&screen.app.project);
    let source = screen.app.project.add_pattern().unwrap();
    screen.app.select_pattern(source);
    let time = lineup_core::TimeSignature::new(3, 4);
    let length = 36;
    let pattern = screen.app.project.pattern_mut(source).unwrap();
    pattern.time = Some(time);
    pattern.set_length(length);
    pattern.set_notes(
        channel,
        [lineup_core::NoteEvent::new(
            lineup_core::Note::new(60),
            0,
            4,
        )],
    );
    let before = screen.app.project.clone();
    screen.app.rack_open = false;
    screen.app.piano_roll.open_for(channel);
    screen.click("Row…");
    screen.scroll_to_at("Save to new pattern", egui::pos2(450.0, 500.0));
    screen.click("1: a melody");
    screen.click("3: trichords");
    screen.click("Dynamics");
    screen.scroll_to_at("Save to new pattern", egui::pos2(450.0, 500.0));
    screen.click("Save to new pattern");
    let index = before.patterns().len();
    assert_eq!(screen.app.pattern, index);
    assert_eq!(screen.app.piano_roll.selected_channel(), Some(channel));
    assert_eq!(screen.app.project.patterns().len(), index + 1);
    assert_eq!(&screen.app.project.patterns()[..index], before.patterns());
    assert_eq!(screen.app.project.playlist(), before.playlist());
    assert_eq!(screen.app.project.channels(), before.channels());
    assert_eq!(screen.app.project.scale, before.scale);
    let row = RowPreset::named("Lineup row").unwrap().row;
    let expected = lineup_core::tidy_notes(write_row(
        row,
        &RowWriting {
            label: row.label(),
            together: 3,
            dynamics: true,
            ..Default::default()
        },
        length,
    ));
    let created = screen.app.project.pattern(index).unwrap();
    assert_eq!(created.name, format!("Row {}", row.label()));
    assert_eq!(created.length(), length);
    assert_eq!(screen.app.project.pattern_time(index), time);
    assert_eq!(created.notes(channel), expected);
    assert!(created.all_notes().all(|(index, _)| index == channel));
    let saved = lineup_core::file::to_text(&screen.app.project).unwrap();
    assert_eq!(
        lineup_core::file::from_text(&saved).unwrap(),
        screen.app.project
    );
    let created_project = screen.app.project.clone();
    screen.app.undo();
    assert_eq!(screen.app.project, before);
    screen.app.redo();
    assert_eq!(screen.app.project, created_project);
    screen.app.undo();
    screen.app.select_pattern(source);
    screen.click(&format!("Write {} into current pattern", row.label()));
    let mut replaced = before.clone();
    replaced
        .pattern_mut(source)
        .unwrap()
        .set_notes(channel, expected);
    assert_eq!(screen.app.project, replaced);
    screen.app.undo();
    assert_eq!(screen.app.project, before);
}

#[test]
fn row_window_cannot_create_a_pattern_after_the_project_limit() {
    let mut screen = Screen::new();
    while screen.app.project.add_pattern().is_some() {}
    let before = screen.app.project.clone();
    screen.app.rack_open = false;
    screen
        .app
        .piano_roll
        .open_for(first_synth(&screen.app.project));
    screen.click("Row…");
    screen.scroll_to_at("Save to new pattern", egui::pos2(450.0, 500.0));
    screen.click("Save to new pattern");
    assert_eq!(screen.app.project, before);
    assert_eq!(
        screen.app.project.patterns().len(),
        lineup_core::MAX_PATTERNS
    );
}

#[test]
fn delete_at_startup_keeps_the_channel_the_piano_roll_picked_by_itself() {
    for key in [Key::Delete, Key::Backspace] {
        let mut screen = Screen::new();
        screen.draw(vec![]);
        screen.draw(vec![]);
        let before = screen.app.project.clone();
        screen.key(key);
        assert_eq!(screen.app.project, before, "{key:?} deleted a channel");
        // Choosing a channel and then deselecting it leaves none to delete.
        let step = screen.app.project.channels()[0].name.clone();
        screen.click(&step);
        screen.click(&step);
        screen.key(key);
        assert_eq!(screen.app.project, before, "{key:?} after deselecting");
    }
}

#[test]
fn a_window_opened_by_double_clicking_a_clip_takes_the_keyboard() {
    let mut screen = shortcut_screen();
    screen.click("Shortcut test");
    let before = screen.app.project.clone();
    screen
        .app
        .apply_playlist(crate::playlist::PlaylistAction::EditClip(0));
    screen.draw(vec![]);
    screen.draw(vec![]);
    screen.key_with_modifiers(Key::A, Modifiers::COMMAND);
    screen.key(Key::Delete);
    assert_eq!(
        screen.app.project.playlist().clips(),
        before.playlist().clips(),
        "Ctrl+A and Delete in the piano roll deleted clips"
    );
    assert!(screen.app.project.pattern(0).unwrap().notes(0).is_empty());
}

#[test]
fn toolbar_clicks_do_not_give_the_playlist_the_keyboard() {
    let mut screen = shortcut_screen();
    screen.click("Shortcut test");
    let before = screen.app.project.clone();
    screen.click_icon("Follow playhead");
    screen.click_icon("Follow playhead");
    screen.key(Key::Delete);
    screen.key_with_modifiers(Key::A, Modifiers::COMMAND);
    screen.key(Key::Delete);
    assert_eq!(screen.app.project, before);
    // A click in the playlist itself still does.
    screen.click("Shortcut test");
    screen.key(Key::Delete);
    assert_eq!(
        screen.app.project.playlist().clips().len(),
        before.playlist().clips().len() - 1
    );
}

#[test]
fn a_hidden_rack_gives_up_space_and_escape() {
    let mut screen = Screen::new();
    screen.draw(vec![]);
    let name = screen.app.project.channels()[0].name.clone();
    screen.click(&name);
    let chosen = screen.app.piano_roll.selected_channel();
    screen.key(Key::F6);
    screen.draw(vec![]);
    assert!(!screen.app.rack_open);
    screen.app.set_mode(PlayMode::Song);
    screen.key(Key::Space);
    assert_eq!(screen.app.mode, PlayMode::Song);
    assert!(screen.app.playing);
    screen.key(Key::Escape);
    assert_eq!(screen.app.piano_roll.selected_channel(), chosen);
}

#[test]
fn opening_a_song_forgets_the_clips_selected_in_the_last_one() {
    use lineup_core::{Channel, Clip};
    let mut screen = shortcut_screen();
    screen.click("Shortcut test");
    let middle = screen.app.playlist.area().center();
    screen.click_at(middle, Modifiers::NONE);
    screen.key_with_modifiers(Key::A, Modifiers::COMMAND);
    let mut other = Project::empty();
    other.add_channel(Channel::synth());
    other.add_clip(Clip::new(0, 0, 0)).unwrap();
    other.add_clip(Clip::new(0, 1, 64)).unwrap();
    screen.app.replace_project(other, None);
    // The playlist still has the keyboard, but nothing selected.
    screen.key(Key::Delete);
    assert_eq!(screen.app.project.playlist().clips().len(), 2);
}

#[test]
fn the_tools_menu_pastes_what_another_roll_copied() {
    use lineup_core::NoteEvent;
    let mut screen = shortcut_screen();
    screen.app.piano_roll.open_for(0);
    screen.draw(vec![]);
    // This roll's own copy, of the notes it then deletes.
    screen.click("Tools");
    screen.key(Key::Escape);
    screen.key_with_modifiers(Key::A, Modifiers::COMMAND);
    screen.key_with_modifiers(Key::C, Modifiers::COMMAND);
    screen.key(Key::Delete);
    assert!(screen.app.project.pattern(0).unwrap().notes(0).is_empty());
    // Another roll copied a high C since.
    let copied = NoteEvent::new(Note::new(72), 0, 2);
    let text = format!(
        "Ferrous Lineup notes\n{}",
        ron::to_string(&vec![copied]).unwrap()
    );
    screen.click("Tools");
    let menu = text_rect(&screen.draw(vec![]), "Up a semitone").unwrap();
    screen.scroll_to_at("Paste", menu.center());
    let pos = text_rect(&screen.draw(vec![]), "Paste").unwrap().center();
    screen.draw(vec![
        Event::PointerMoved(pos),
        Event::PointerButton {
            pos,
            button: PointerButton::Primary,
            pressed: true,
            modifiers: Modifiers::NONE,
        },
    ]);
    let output = screen.draw(vec![Event::PointerButton {
        pos,
        button: PointerButton::Primary,
        pressed: false,
        modifiers: Modifiers::NONE,
    }]);
    let asked = output.viewport_output.values().any(|viewport| {
        viewport
            .commands
            .contains(&egui::ViewportCommand::RequestPaste)
    });
    assert!(asked, "the menu asks for the system clipboard");
    screen.draw(vec![Event::Paste(text)]);
    let pitches: Vec<u8> = screen
        .app
        .project
        .pattern(0)
        .unwrap()
        .notes(0)
        .iter()
        .map(|note| note.pitch.midi())
        .collect();
    assert_eq!(pitches, [72]);
}

/// An export whose thread runs until it is cancelled.
fn running_export() -> super::export::Export {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    let cancel = Arc::new(AtomicBool::new(false));
    let worker_cancel = Arc::clone(&cancel);
    let worker = std::thread::spawn(move || {
        while !worker_cancel.load(Ordering::Relaxed) {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        Ok(())
    });
    super::export::Export::new(
        PathBuf::from("test.wav"),
        10,
        Arc::new(AtomicUsize::new(0)),
        Some(cancel),
        worker,
    )
}

#[test]
fn closing_while_an_export_runs_asks_before_stopping_it() {
    use std::time::Duration;

    let mut screen = Screen::new();
    screen.app.export = Some(running_export());
    screen.request_close();
    assert!(
        text_rect(&screen.draw(vec![]), "An export is still running").is_some(),
        "the export question is asked"
    );

    // Keeping the export leaves it running, and keeps the window open.
    screen.click("Keep exporting");
    assert!(screen.app.export.is_some());
    assert!(!screen.app.file.is_quitting());

    // Stopping it cancels the thread, waits for it, then closes.
    screen.request_close();
    assert!(text_rect(&screen.draw(vec![]), "An export is still running").is_some());
    screen.click("Stop it and quit");
    for _ in 0..100 {
        if screen.app.file.is_quitting() {
            break;
        }
        screen.draw(vec![]);
        std::thread::sleep(Duration::from_millis(1));
    }
    assert!(screen.app.export.is_none(), "the export was waited for");
    assert!(screen.app.file.is_quitting(), "the window closes");
}

mod library_parts {
    use super::*;
    use crate::library_browser::LibraryAction;
    use lineup_core::library::{Rhythm, Song, Style, UserStyle, apply::StyleParts};

    const NOTHING: StyleParts = StyleParts {
        rhythm: false,
        instruments: false,
        time_signature: false,
        progression: false,
        tempo: false,
        swing: false,
        matching_style: true,
    };

    fn el_choclo() -> Screen {
        let mut screen = Screen::new();
        screen
            .app
            .open_demo(library::Demo::Song(Song::named("El Choclo").unwrap()));
        screen
    }

    fn muted(project: &Project) -> Vec<&str> {
        project
            .channels()
            .iter()
            .filter(|channel| channel.muted)
            .map(|channel| channel.name.as_str())
            .collect()
    }

    #[test]
    fn undo_and_redo_keep_a_demos_style_selected() {
        let mut screen = el_choclo();
        let tried = |screen: &Screen| {
            let tried = screen.app.tried();
            (
                tried.style.map(|style| style.name),
                tried.rhythm.map(|rhythm| rhythm.name),
            )
        };
        let opened = tried(&screen);
        assert_eq!(opened, (Some("Tango"), Some("Milonga")));
        screen.app.edit("change the tempo", |project| {
            project.tempo = Tempo::new(100.0);
        });
        screen.app.undo();
        assert_eq!(tried(&screen), opened);
        screen.app.redo();
        assert_eq!(tried(&screen), opened);
        // A style the Library put in still goes away with its undo.
        screen
            .app
            .try_from_library(LibraryAction::Style(Style::named("Doo-wop").unwrap()));
        assert_eq!(tried(&screen).0, Some("Doo-wop"));
        screen.app.undo();
        assert_eq!(tried(&screen), opened);
    }

    #[test]
    fn the_piano_roll_names_only_the_chords_whose_names_fit() {
        // El Choclo's "A" has chords and notes too short for their names
        // at the usual zoom. Each name drawn fits its block, instead of
        // being cut to a piece such as "7/".
        let mut screen = el_choclo();
        screen.app.set_playing(false);
        let a = screen
            .app
            .project
            .patterns()
            .iter()
            .position(|pattern| pattern.name == "A")
            .unwrap();
        screen.app.edit_clip(a);
        screen.draw(vec![]);
        let output = screen.draw(vec![]);
        let names: Vec<_> = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                Shape::Text(text) if text.fallback_color == crate::piano_roll::LANE_TEXT => Some((
                    text.galley.text().to_string(),
                    text.galley.size().x,
                    shape.clip_rect,
                )),
                _ => None,
            })
            .collect();
        assert!(!names.is_empty(), "the chord lane names some chords");
        for (name, width, clip) in names {
            assert!(
                width <= clip.width(),
                "{name} is {width} wide in a block {} wide",
                clip.width()
            );
        }
    }

    #[test]
    fn applying_a_style_to_the_song_keeps_the_selected_pattern() {
        // The band brings channels, which give every pattern new rows:
        // the patterns are still the song's own, so "Pickup" stays
        // selected instead of the song's last pattern.
        let mut screen = el_choclo();
        screen.app.set_playing(false);
        screen.app.select_pattern(0);
        let selected = screen.app.project.patterns()[0].name.clone();
        *screen.app.playlist.library_parts_mut() = StyleParts::default();
        screen.app.set_mode(PlayMode::Song);
        screen
            .app
            .try_from_library(LibraryAction::Style(Style::named("Tango nuevo").unwrap()));
        assert!(screen.app.project.band().is_some());
        assert_eq!(
            screen.app.project.patterns()[screen.app.pattern].name,
            selected
        );
    }

    #[test]
    fn the_time_box_plays_the_rhythm_picked_last() {
        let mut screen = meter_test_screen(true, true);
        *screen.app.playlist.library_parts_mut() = StyleParts::default();
        screen.app.set_mode(PlayMode::Song);
        screen.app.try_from_library(LibraryAction::RhythmInSong(
            Rhythm::named("Tango marcato").unwrap(),
        ));
        assert_eq!(
            screen.app.project.band().map(|band| band.style.name),
            Some("Tango marcato")
        );
        screen
            .app
            .set_song_time(lineup_core::TimeSignature::new(3, 4));
        assert_eq!(
            screen.app.project.band().map(|band| band.style.name),
            Some("Tango vals")
        );
    }

    #[test]
    fn the_time_box_brings_in_the_band_only_in_song_mode() {
        let three_four = lineup_core::TimeSignature::new(3, 4);
        // Pattern mode: the demo's own Keys and Bass, in 3/4.
        let mut screen = el_choclo();
        *screen.app.playlist.library_parts_mut() = StyleParts::default();
        screen.app.set_mode(PlayMode::Pattern);
        let keys = |project: &Project| {
            let keys = project
                .channels()
                .iter()
                .position(|channel| channel.name == "Keys")
                .unwrap();
            project
                .patterns()
                .iter()
                .map(|pattern| pattern.notes(keys).len())
                .sum::<usize>()
        };
        assert!(keys(&screen.app.project) > 0);
        screen.app.set_song_time(three_four);
        assert_eq!(screen.app.project.time_signature, three_four);
        assert!(screen.app.project.band().is_none());
        assert!(muted(&screen.app.project).is_empty());
        assert!(keys(&screen.app.project) > 0);
        // Song mode: the band, in the demo's style, instead of them.
        let mut screen = el_choclo();
        *screen.app.playlist.library_parts_mut() = StyleParts::default();
        screen.app.set_song_time(three_four);
        assert_eq!(screen.app.project.time_signature, three_four);
        assert_eq!(
            screen.app.project.band().map(|band| band.style.name),
            Some("Tango vals")
        );
        assert_eq!(muted(&screen.app.project), ["Keys", "Bass"]);
    }

    #[test]
    fn the_progression_menu_does_not_show_the_bands_style_as_the_sheets_chords() {
        let mut screen = Screen::new();
        screen.app.replace_project(Project::with_band(), None);
        screen.app.set_mode(PlayMode::Song);
        *screen.app.playlist.library_parts_mut() = StyleParts::default();
        let doo_wop = Style::named("Doo-wop").unwrap();
        screen.app.try_from_library(LibraryAction::Style(doo_wop));
        assert_eq!(
            screen.app.tried().style.map(|style| style.name),
            Some("Doo-wop")
        );
        screen.app.chord_sheet.open = true;
        screen.app.rack_open = false;
        screen.click("Progression");
        let menu = text_rect(&screen.draw(vec![]), "Progression").unwrap();
        screen.scroll_to_at("Doo-wop", menu.center() + vec2(0.0, 120.0));
        let output = screen.draw(vec![]);
        assert!(!button_lit(&output, "Doo-wop"));
    }

    #[test]
    fn a_saved_styles_tempo_alone_does_not_bring_in_a_band() {
        let mut screen = el_choclo();
        let mut style = UserStyle::from_rhythm(Rhythm::named("Waltz").unwrap());
        style.tempo = Tempo::new(150.0);
        screen.app.playlist.library_browser_mut().user_styles = vec![style];
        *screen.app.playlist.library_parts_mut() = StyleParts {
            tempo: true,
            ..NOTHING
        };
        screen.app.apply_user_library(0, None);
        assert!(screen.app.project.band().is_none());
        assert!(muted(&screen.app.project).is_empty());
        assert_eq!(screen.app.project.tempo.bpm(), 150.0);

        // Its instruments alone have no band to play in, and say so.
        let before = screen.app.project.clone();
        *screen.app.playlist.library_parts_mut() = StyleParts {
            instruments: true,
            ..NOTHING
        };
        screen.app.apply_user_library(0, None);
        assert!(screen.app.project.band().is_none());
        assert_eq!(screen.app.project, before);
        assert!(
            screen
                .app
                .notice
                .as_deref()
                .is_some_and(|notice| notice.contains("Playing patterns")),
            "{:?}",
            screen.app.notice
        );
    }

    #[test]
    fn a_saved_chart_that_does_not_fit_the_bars_changes_nothing() {
        let mut screen = Screen::new();
        screen.app.replace_project(Project::with_band(), None);
        screen.app.edit("remeter", |project| {
            lineup_core::remeter::remeter(project, lineup_core::TimeSignature::new(3, 4));
        });
        let mut style = UserStyle::from_builtin(&lineup_core::library::STYLES[0]);
        style.time = lineup_core::TimeSignature::COMMON;
        style.chart = "C D E F".into();
        style.tempo = Tempo::new(150.0);
        style.validate().unwrap();
        screen.app.playlist.library_browser_mut().user_styles = vec![style];
        *screen.app.playlist.library_parts_mut() = StyleParts {
            progression: true,
            tempo: true,
            rhythm: true,
            ..NOTHING
        };
        let before = screen.app.project.clone();
        screen.app.apply_user_library(0, None);
        assert!(screen.app.message.is_some(), "the error is shown");
        assert_eq!(screen.app.project, before);
    }

    #[test]
    fn instruments_or_tempo_alone_leave_your_pattern_yours() {
        let mut screen = Screen::new();
        screen.app.set_mode(PlayMode::Pattern);
        let pattern = screen.app.pattern;
        let name = screen.app.project.patterns()[pattern].name.clone();
        let count = screen.app.project.patterns().len();
        *screen.app.playlist.library_parts_mut() = StyleParts {
            instruments: true,
            tempo: true,
            ..NOTHING
        };
        let royal_road = Style::named("Royal road").unwrap();
        screen
            .app
            .try_from_library(LibraryAction::Style(royal_road));
        assert_eq!(screen.app.project.patterns()[pattern].name, name);
        assert_eq!(screen.app.project.tempo.bpm(), royal_road.tempo);
        // The next style tried gets a pattern of its own.
        let yours = screen.app.project.patterns()[pattern].clone();
        *screen.app.playlist.library_parts_mut() = StyleParts::default();
        screen
            .app
            .try_from_library(LibraryAction::Style(Style::named("Four chords").unwrap()));
        assert_eq!(screen.app.project.patterns().len(), count + 1);
        let now = &screen.app.project.patterns()[pattern];
        assert_eq!(now.name, yours.name);
        for channel in 0..yours.rows() {
            assert_eq!(now.notes(channel), yours.notes(channel));
        }
    }

    #[test]
    fn instruments_alone_with_no_channel_to_take_them_say_so() {
        let mut screen = Screen::new();
        let mut project = Project::empty();
        project.add_channel(lineup_core::Channel::drum(lineup_core::Drum::Kick));
        project.pattern_mut(0).unwrap().set(0, 0, true);
        screen.app.replace_project(project, None);
        screen.app.set_mode(PlayMode::Pattern);
        let before = screen.app.project.clone();
        *screen.app.playlist.library_parts_mut() = StyleParts {
            instruments: true,
            ..NOTHING
        };
        screen
            .app
            .try_from_library(LibraryAction::Style(Style::named("Royal road").unwrap()));
        assert_eq!(screen.app.project, before);
        assert!(
            screen
                .app
                .notice
                .as_deref()
                .is_some_and(|notice| notice.contains("Keys")),
            "{:?}",
            screen.app.notice
        );
    }
}

mod mix {
    use super::*;
    use crate::library_browser::LibraryAction;
    use lineup_core::library::{Song, Style, apply::StyleParts};
    use lineup_core::{Channel, Drum};

    #[test]
    fn the_rack_lights_go_out_for_channels_a_solo_silences() {
        let mut screen = Screen::new();
        screen.app.rack_open = true;
        let lit = |output: &FullOutput| {
            output
                .shapes
                .iter()
                .filter(|shape| {
                    matches!(&shape.shape, Shape::Circle(circle) if circle.fill == crate::widgets::LED_ON)
                })
                .count()
        };
        screen.draw(vec![]);
        let all = lit(&screen.draw(vec![]));
        screen
            .app
            .edit("solo", |project| project.mixer.solo = Some(0));
        screen.draw(vec![]);
        let solo = lit(&screen.draw(vec![]));
        assert_eq!(all - solo, screen.app.project.channels().len() - 1);
    }

    #[test]
    fn reset_mix_keeps_the_band_playing_instead_of_the_songs_own_parts() {
        let mut screen = Screen::new();
        screen
            .app
            .open_demo(library::Demo::Song(Song::named("El Choclo").unwrap()));
        *screen.app.playlist.library_parts_mut() = StyleParts::default();
        screen
            .app
            .try_from_library(LibraryAction::Style(Style::named("Doo-wop").unwrap()));
        let muted = |project: &Project| -> Vec<String> {
            project
                .channels()
                .iter()
                .filter(|channel| channel.muted)
                .map(|channel| channel.name.clone())
                .collect()
        };
        assert_eq!(muted(&screen.app.project), ["Keys", "Bass"]);
        let band_piano = screen
            .app
            .project
            .player_channel(lineup_core::Player::Piano)
            .unwrap();
        screen.app.edit("mix", |project| {
            project.mixer.solo = Some(0);
            for channel in 0..project.channels().len() {
                let state = project.channel_mut(channel).unwrap();
                state.volume = 1.0;
                state.muted = channel == band_piano;
            }
        });
        screen.app.mixer_open = true;
        screen.app.rack_open = false;
        screen.app.chord_sheet.open = false;
        screen.draw(vec![]);
        screen.click("Reset mix");
        let project = &screen.app.project;
        assert_eq!(muted(project), ["Keys", "Bass"]);
        assert_eq!(project.mixer.solo, None);
        assert_eq!(project.channels()[band_piano].volume, 0.45);
        let kick = project
            .channels()
            .iter()
            .position(|channel| channel.name == "Kick")
            .unwrap();
        assert_eq!(project.channels()[kick].volume, 0.8);
    }

    #[test]
    fn the_mixer_keeps_the_drums_together_and_each_one_still_solos_alone() {
        let mut screen = Screen::new();
        screen.app.edit("add a band", |project| {
            project.add_channel(Channel {
                name: "Keys".into(),
                program: Some(lineup_core::general_midi::ACOUSTIC_GRAND_PIANO),
                ..Channel::synth()
            });
            project.add_channel(Channel {
                name: "Bass".into(),
                program: Some(lineup_core::general_midi::FINGER_BASS),
                ..Channel::synth()
            });
            project.add_channel(Channel::drum(Drum::Ride));
        });
        let ride = screen.app.project.channels().len() - 1;
        screen.app.mixer_open = true;
        screen.app.rack_open = false;
        screen.draw(vec![]);
        let output = screen.draw(vec![]);
        let y = |label: &str| {
            text_rect(&output, label)
                .unwrap_or_else(|| panic!("{label} is in the mixer"))
                .center()
                .y
        };
        let mut drums = ["Kick", "Snare", "Ride", "Hat", "Clap", "Drums"];
        drums.sort_by(|a, b| y(a).total_cmp(&y(b)));
        assert_eq!(drums, ["Drums", "Clap", "Hat", "Ride", "Snare", "Kick"]);
        // The band comes after the drums, if it is in view at all.
        for later in [
            "Harp and guitars",
            "Bass",
            "Keyboards",
            "Keys",
            "Lineup Osc",
        ] {
            assert!(text_rect(&output, later).is_none_or(|rect| rect.top() > y("Kick")));
        }
        let solo = output
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                Shape::Text(text)
                    if text.galley.text() == "S"
                        && (text.pos.y + text.galley.size().y / 2.0 - y("Ride")).abs() < 4.0 =>
                {
                    Some(Rect::from_min_size(text.pos, text.galley.size()).center())
                }
                _ => None,
            })
            .expect("the Ride row has a solo button");
        screen.click_at(solo, Modifiers::NONE);
        assert_eq!(screen.app.project.mixer.solo, Some(ride));
        assert_eq!(screen.app.project.channels()[ride].name, "Ride");
    }
}

#[test]
fn a_drawn_note_is_not_selected_so_tools_still_change_every_note() {
    let mut screen = shortcut_screen();
    screen.app.piano_roll.open_for(0);
    screen.draw(vec![]);
    let key = text_rect(&screen.draw(vec![]), "C4").expect("the C4 key is in view");
    // On the C4 row, well right of the C4 note at steps 4 to 7.
    screen.click_at(
        egui::pos2(key.right() + 400.0, key.center().y),
        Modifiers::NONE,
    );
    let pitches = |screen: &Screen| -> Vec<u8> {
        let mut pitches: Vec<u8> = screen
            .app
            .project
            .pattern(0)
            .unwrap()
            .notes(0)
            .iter()
            .map(|note| note.pitch.midi())
            .collect();
        pitches.sort_unstable();
        pitches
    };
    assert_eq!(pitches(&screen), [60, 60, 67], "a C4 was drawn");
    screen.key(Key::ArrowUp);
    assert_eq!(pitches(&screen), [61, 61, 68]);
}

#[test]
fn space_in_the_rack_stops_the_song_then_plays_the_pattern() {
    let mut screen = Screen::new();
    screen.app.rack_open = true;
    screen.draw(vec![]);
    let name = screen.app.project.channels()[0].name.clone();
    screen.app.set_mode(PlayMode::Song);
    screen.app.set_playing(true);
    screen.click(&name);
    screen.key(Key::Space);
    assert_eq!(screen.app.mode, PlayMode::Song);
    assert!(!screen.app.playing, "the song stopped");
    screen.key(Key::Space);
    assert_eq!(screen.app.mode, PlayMode::Pattern);
    assert!(screen.app.playing, "the pattern plays");
}

#[test]
fn nudges_of_the_master_volume_are_one_step_to_undo() {
    let mut screen = Screen::new();
    let before = screen.app.project.mixer.master;
    for master in [0.5, 0.6, 0.7] {
        screen.app.set_master_volume(master);
        screen.app.history.close();
    }
    assert_eq!(screen.app.project.mixer.master, 0.7);
    screen.app.undo();
    assert_eq!(screen.app.project.mixer.master, before);
    // Any other change in between starts a new step.
    screen.app.set_master_volume(0.5);
    screen.app.history.close();
    screen.app.edit("change the tempo", |project| {
        project.tempo = Tempo::new(100.0);
    });
    screen.app.history.close();
    screen.app.set_master_volume(0.6);
    screen.app.undo();
    assert_eq!(screen.app.project.mixer.master, 0.5);
}

#[test]
fn saving_a_style_elsewhere_keeps_the_styles_folder() {
    use lineup_core::library::UserStyle;
    let root = std::env::temp_dir().join(format!("lineup-save-as-{}", std::process::id()));
    let (folder, elsewhere) = (root.join("styles"), root.join("desktop"));
    std::fs::create_dir_all(&folder).unwrap();
    std::fs::create_dir_all(&elsewhere).unwrap();
    let mut screen = Screen::new();
    screen.app.style_files.folder = folder.clone();
    let mut style = UserStyle::blank();
    style.name = "Elsewhere".into();
    screen.app.style_editor.start(style.clone(), None);
    screen.app.style_editor.path = Some(elsewhere.join("Elsewhere.lineup-style"));
    screen.app.save_user_style(style, false);
    assert!(elsewhere.join("Elsewhere.lineup-style").exists());
    assert_eq!(screen.app.style_files.folder, folder);
    assert!(
        screen
            .app
            .notice
            .as_deref()
            .is_some_and(|notice| notice.contains("Styles folder")),
        "{:?}",
        screen.app.notice
    );
    std::fs::remove_dir_all(&root).unwrap();
}

mod part_sources {
    use super::*;
    use crate::library_browser::{LibraryAction, part_source_text};
    use lineup_core::SongPart;
    use lineup_core::library::{Rhythm, Song, Style, apply::StyleParts};

    const ONLY_TEMPO: StyleParts = StyleParts {
        rhythm: false,
        instruments: false,
        time_signature: false,
        progression: false,
        tempo: true,
        swing: false,
        matching_style: true,
    };

    fn source(screen: &Screen, part: SongPart) -> String {
        part_source_text(&screen.app.project, part)
    }

    /// Every part but `part` comes from `from`.
    fn others_from(screen: &Screen, part: SongPart, from: &str) {
        for other in SongPart::ALL.into_iter().filter(|&other| other != part) {
            assert_eq!(source(screen, other), from, "{other:?}");
        }
    }

    fn el_choclo() -> Screen {
        let mut screen = Screen::new();
        screen
            .app
            .open_demo(library::Demo::Song(Song::named("El Choclo").unwrap()));
        screen
    }

    #[test]
    fn a_demo_brings_in_every_part_and_a_style_only_those_ticked() {
        let screen = Screen::new();
        others_from(&screen, SongPart::Tempo, "The starting song (demo)");
        assert_eq!(source(&screen, SongPart::Tempo), "The starting song (demo)");

        let mut screen = el_choclo();
        assert!(screen.app.playing && screen.app.mode == PlayMode::Song);
        others_from(&screen, SongPart::Tempo, "El Choclo (demo)");
        let tempo = screen.app.project.tempo;
        *screen.app.playlist.library_parts_mut() = ONLY_TEMPO;
        let doo_wop = Style::named("Doo-wop").unwrap();
        screen.app.try_from_library(LibraryAction::Style(doo_wop));
        assert_ne!(screen.app.project.tempo, tempo);
        assert_eq!(source(&screen, SongPart::Tempo), "Doo-wop (style)");
        others_from(&screen, SongPart::Tempo, "El Choclo (demo)");

        // Your own change, and its undo. (A click starts each step.)
        screen.app.history.close();
        screen.app.edit("change the tempo", |project| {
            project.tempo = Tempo::new(97.0);
        });
        assert_eq!(source(&screen, SongPart::Tempo), "edited, was Doo-wop");
        others_from(&screen, SongPart::Tempo, "El Choclo (demo)");
        screen.app.undo();
        assert_eq!(source(&screen, SongPart::Tempo), "Doo-wop (style)");
        screen.app.undo();
        assert_eq!(source(&screen, SongPart::Tempo), "El Choclo (demo)");
    }

    #[test]
    fn a_part_ticked_comes_from_the_rhythm_even_if_the_song_had_it_already() {
        let mut screen = Screen::new();
        assert_eq!(screen.app.mode, PlayMode::Song);
        let afrobeats = Rhythm::named("Afrobeats").unwrap();
        assert_eq!(afrobeats.time, screen.app.project.time_signature);
        *screen.app.playlist.library_parts_mut() = StyleParts {
            tempo: false,
            ..StyleParts::default()
        };
        screen
            .app
            .try_from_library(LibraryAction::Rhythm(afrobeats));
        for part in [
            SongPart::Patterns,
            SongPart::Instruments,
            SongPart::TimeSignature,
            SongPart::Swing,
        ] {
            assert_eq!(source(&screen, part), "Afrobeats (rhythm)", "{part:?}");
        }
        for part in [SongPart::Chords, SongPart::Tempo] {
            assert_eq!(
                source(&screen, part),
                "The starting song (demo)",
                "{part:?}"
            );
        }
    }

    #[test]
    fn the_time_box_time_is_yours_and_the_band_comes_from_its_rhythm() {
        let mut screen = el_choclo();
        let time = screen.app.project.time_signature;
        *screen.app.playlist.library_parts_mut() = StyleParts::default();
        screen
            .app
            .set_song_time(lineup_core::TimeSignature::new(3, 4));
        assert_eq!(
            source(&screen, SongPart::TimeSignature),
            "edited, was El Choclo"
        );
        let band = screen.app.project.band().unwrap().style.name;
        let rhythm = format!("{band} (rhythm)");
        assert_eq!(source(&screen, SongPart::Patterns), rhythm);
        assert_eq!(source(&screen, SongPart::Instruments), rhythm);
        // "Use rhythm's swing" is ticked: the rhythm's, even if the same.
        assert_eq!(source(&screen, SongPart::Swing), rhythm);
        // Moved into the new bars, and kept at their pace: still the demo's.
        for part in [SongPart::Chords, SongPart::Tempo] {
            assert_eq!(source(&screen, part), "El Choclo (demo)", "{part:?}");
        }
        // Back in its own time, the song is the demo's again.
        screen.app.set_song_time(time);
        others_from(&screen, SongPart::Tempo, "El Choclo (demo)");
        assert_eq!(source(&screen, SongPart::Tempo), "El Choclo (demo)");
    }

    #[test]
    fn a_trial_pattern_does_not_change_where_the_songs_patterns_come_from() {
        let mut screen = el_choclo();
        screen.app.set_playing(false);
        screen.app.set_mode(PlayMode::Pattern);
        *screen.app.playlist.library_parts_mut() = StyleParts::default();
        let doo_wop = Style::named("Doo-wop").unwrap();
        screen.app.try_from_library(LibraryAction::Style(doo_wop));
        assert_eq!(
            screen.app.project.pattern(screen.app.pattern).unwrap().name,
            "Doo-wop"
        );
        assert_eq!(source(&screen, SongPart::Patterns), "El Choclo (demo)");
        assert_eq!(source(&screen, SongPart::Chords), "El Choclo (demo)");
        // The Tempo option starts off: the song keeps its own.
        assert_eq!(source(&screen, SongPart::Tempo), "El Choclo (demo)");
    }

    #[test]
    fn the_library_and_its_apply_options_say_where_each_part_comes_from() {
        let mut screen = el_choclo();
        screen.app.rack_open = false;
        screen.app.playlist.browser_open = true;
        *screen.app.playlist.library_parts_mut() = ONLY_TEMPO;
        screen
            .app
            .try_from_library(LibraryAction::Style(Style::named("Doo-wop").unwrap()));
        screen.click("📚 Library");
        let output = screen.draw(Vec::new());
        let tempo = format!(
            "{:.0} BPM · Doo-wop (style)",
            screen.app.project.tempo.bpm()
        );
        assert!(text_rect(&output, &tempo).is_some(), "{tempo}");
        assert!(text_rect(&output, "El Choclo (demo)").is_some());
        screen.click("🔍 Search: bossa, 6/8, tango…");
        screen.draw(vec![Event::Text("Emotional pop".into())]);
        screen.click("Emotional pop");
        let output = screen.draw(Vec::new());
        assert!(text_rect(&output, "now: Doo-wop (style)").is_some());
        assert!(text_rect(&output, "now: El Choclo (demo)").is_some());
    }

    #[test]
    fn a_saved_song_keeps_where_its_parts_came_from() {
        let mut screen = el_choclo();
        *screen.app.playlist.library_parts_mut() = ONLY_TEMPO;
        screen
            .app
            .try_from_library(LibraryAction::Style(Style::named("Doo-wop").unwrap()));
        screen
            .app
            .edit("change the swing", |project| project.swing = 20);
        let text = lineup_core::file::to_text(&screen.app.project).unwrap();
        let opened = lineup_core::file::from_text(&text).unwrap();
        screen.app.replace_project(opened, None);
        assert_eq!(source(&screen, SongPart::Tempo), "Doo-wop (style)");
        assert_eq!(source(&screen, SongPart::Swing), "edited, was El Choclo");
        assert_eq!(source(&screen, SongPart::Chords), "El Choclo (demo)");
    }
}

mod locks {
    use super::*;
    use crate::channel_rack::RackAction;
    use crate::library_browser::LibraryAction;
    use lineup_core::TimeSignature;
    use lineup_core::library::{Song, Style, apply::StyleParts};

    fn ode_to_joy() -> Screen {
        let mut screen = Screen::new();
        screen
            .app
            .open_demo(library::Demo::Song(Song::named("Ode to Joy").unwrap()));
        screen.app.set_playing(false);
        // A song opens on the chord sheet; the locks are in the rack.
        screen.app.chord_sheet.open = false;
        screen.app.rack_open = true;
        screen
    }

    fn channel(screen: &Screen, name: &str) -> usize {
        screen
            .app
            .project
            .channels()
            .iter()
            .position(|channel| channel.name == name)
            .unwrap()
    }

    #[test]
    fn the_rack_locks_a_channel_and_undo_unlocks_it() {
        let mut screen = ode_to_joy();
        let keys = channel(&screen, "Keys");
        screen.draw(Vec::new());
        screen.draw(Vec::new());
        // The name's menu has the lock too.
        let output = screen.draw(Vec::new());
        let name = text_rect(&output, "Keys")
            .expect("Keys is in the rack")
            .center();
        screen.draw(vec![Event::PointerMoved(name)]);
        for pressed in [true, false] {
            screen.draw(vec![Event::PointerButton {
                pos: name,
                button: PointerButton::Secondary,
                pressed,
                modifiers: Modifiers::NONE,
            }]);
        }
        screen.click("Lock");
        assert!(screen.app.project.is_locked(keys));
        assert_eq!(screen.app.history.undo_label(), Some("lock a channel"));
        screen.app.undo();
        assert!(!screen.app.project.is_locked(keys));
        screen.app.apply(RackAction::SetLocked {
            channel: keys,
            locked: true,
        });
        assert!(screen.app.project.is_locked(keys));
        // A locked channel draws, in the rack and in the mixer.
        screen.app.mixer_open = true;
        screen.draw(Vec::new());
        let output = screen.draw(Vec::new());
        assert!(text_rect(&output, "🔒").is_some());
    }

    #[test]
    fn a_style_tried_in_a_pattern_plays_around_a_locked_keys() {
        let mut screen = ode_to_joy();
        screen.app.set_mode(PlayMode::Pattern);
        let keys = channel(&screen, "Keys");
        let song = screen
            .app
            .project
            .patterns()
            .iter()
            .position(|pattern| !pattern.notes(keys).is_empty())
            .unwrap();
        screen.app.select_pattern(song);
        screen.app.set_locked(keys, true);
        let chords = screen
            .app
            .project
            .pattern(song)
            .unwrap()
            .notes(keys)
            .to_vec();
        *screen.app.playlist.library_parts_mut() = StyleParts {
            time_signature: false,
            ..StyleParts::default()
        };
        let patterns = screen.app.project.patterns().len();
        screen
            .app
            .try_from_library(LibraryAction::Style(Style::named("Doo-wop").unwrap()));
        let trial = screen.app.pattern;
        assert_ne!(trial, song);
        let tried = screen.app.project.pattern(trial).unwrap();
        assert_eq!(tried.notes(keys), chords);
        let bass = channel(&screen, "Bass");
        assert!(
            !tried.notes(bass).is_empty(),
            "the bass follows the locked Keys"
        );
        // The next style rewrites the same pattern, around the same Keys.
        screen
            .app
            .try_from_library(LibraryAction::Style(Style::named("Bossa nova").unwrap()));
        assert_eq!(screen.app.pattern, trial);
        assert_eq!(screen.app.project.patterns().len(), patterns + 1);
        assert_eq!(
            screen.app.project.pattern(trial).unwrap().notes(keys),
            chords
        );
    }

    #[test]
    fn the_time_box_goes_back_exactly_with_a_channel_locked_since() {
        let mut screen = ode_to_joy();
        screen.app.set_mode(PlayMode::Pattern);
        let original = screen.app.project.clone();
        screen.app.set_song_time(TimeSignature::new(3, 4));
        let keys = channel(&screen, "Keys");
        screen.app.set_locked(keys, true);
        screen.app.set_song_time(TimeSignature::COMMON);
        assert!(screen.app.project.is_locked(keys));
        assert_eq!(screen.app.project.patterns(), original.patterns());
        assert!(
            screen
                .app
                .notice
                .as_deref()
                .is_some_and(|notice| notice.contains("as it was")),
            "{:?}",
            screen.app.notice
        );
    }

    #[test]
    fn the_library_and_the_band_say_what_is_kept() {
        let mut screen = ode_to_joy();
        let keys = channel(&screen, "Keys");
        screen.app.set_locked(keys, true);
        screen.app.mode = PlayMode::Song;
        *screen.app.playlist.library_parts_mut() = StyleParts::default();
        screen
            .app
            .try_from_library(LibraryAction::Style(Style::named("Bossa nova").unwrap()));
        let band = screen.app.project.band().expect("the band came in");
        assert!(!band.muted().channels.contains(&"Keys".to_string()));
        assert!(!screen.app.project.channels()[keys].muted);
        let notice = screen.app.notice.clone().unwrap_or_default();
        assert!(notice.ends_with("🔒 Kept: Keys."), "{notice}");
        screen.app.rack_open = false;
        screen.app.playlist.browser_open = true;
        screen.click("📚 Library");
        let output = screen.draw(Vec::new());
        assert!(text_rect(&output, "🔒 Keys").is_some());
        screen.app.playlist.browser_open = false;
        screen.app.chord_sheet.open = true;
        screen.draw(Vec::new());
        let output = screen.draw(Vec::new());
        assert!(
            text_rect(
                &output,
                "🔒 Keys is locked: the band leaves the piano to it."
            )
            .is_some()
        );
        // Unlocked, the band plays the piano, and says so.
        screen.app.set_locked(keys, false);
        assert!(screen.app.project.channels()[keys].muted);
        let notice = screen.app.notice.clone().unwrap_or_default();
        assert!(notice.starts_with("Keys unlocked"), "{notice}");
    }
}

mod bars {
    use super::*;

    /// A song of four chords, C G Am F, a bar each, with a band and the
    /// chord sheet open.
    fn song() -> Screen {
        let mut screen = Screen::new();
        let mut project = Project::with_band();
        project.write_progression(
            lineup_core::library::Style::named("Four chords").unwrap(),
            0,
        );
        project.follow_chord_sheet();
        screen.app.replace_project(project, None);
        screen.app.rack_open = false;
        screen.app.chord_sheet.open = true;
        screen
    }

    fn chords(screen: &Screen) -> Vec<(usize, String)> {
        screen
            .app
            .project
            .playlist()
            .chord_markers()
            .iter()
            .map(|marker| (marker.start, marker.harmony.to_string()))
            .collect()
    }

    /// Right-clicks the topmost `label`.
    fn right_click(screen: &mut Screen, label: &str) {
        screen.draw(Vec::new());
        let output = screen.draw(Vec::new());
        let at = text_rect(&output, label)
            .unwrap_or_else(|| panic!("Missing: {label}"))
            .center();
        screen.draw(vec![Event::PointerMoved(at)]);
        for pressed in [true, false] {
            screen.draw(vec![Event::PointerButton {
                pos: at,
                button: PointerButton::Secondary,
                pressed,
                modifiers: Modifiers::NONE,
            }]);
        }
    }

    #[test]
    fn a_bars_menu_inserts_a_bar_and_deletes_it() {
        let mut screen = song();
        let before = chords(&screen);
        assert_eq!(
            before,
            [
                (0, "C".to_string()),
                (16, "G".into()),
                (32, "Am".into()),
                (48, "F".into())
            ]
        );
        let length = screen.app.project.song_length();
        right_click(&mut screen, "G");
        screen.click("Insert a bar after");
        // An empty bar after the G: the G goes on through it, and the
        // rest comes a bar later.
        assert_eq!(
            chords(&screen),
            [
                (0, "C".to_string()),
                (16, "G".into()),
                (48, "Am".into()),
                (64, "F".into())
            ]
        );
        assert_eq!(screen.app.project.song_length(), length + 16);
        assert_eq!(screen.app.history.undo_label(), Some("insert bar"));
        assert_eq!(screen.app.chord_sheet.selected_bars(), Some((2, 2)));
        // Typing goes in the new bar.
        screen.draw(vec![Event::Text("D7".into())]);
        screen.key(Key::Enter);
        assert_eq!(screen.app.project.chord_at(32).map(|m| m.start), Some(32));
        // The bar's menu deletes it again.
        right_click(&mut screen, "D7");
        screen.click("Delete this bar");
        assert_eq!(chords(&screen), before);
        assert_eq!(screen.app.project.song_length(), length);
        assert_eq!(screen.app.history.undo_label(), Some("delete bar"));
    }

    #[test]
    fn in_solfege_l_types_la_chords_and_in_letters_it_switches_mode() {
        let mut screen = song();
        let mode = screen.app.mode;
        // A keyboard sends the key L and the text it types.
        let type_l = |screen: &mut Screen, text: &str| {
            screen.draw(vec![
                Event::Key {
                    key: Key::L,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: Modifiers::SHIFT,
                },
                Event::Text(text.into()),
            ]);
        };
        crate::i18n::set_note_naming(&screen.ctx, lineup_core::NoteNaming::Solfege);
        screen.click("Fa");
        type_l(&mut screen, "Lam7");
        screen.key(Key::Enter);
        assert_eq!(screen.app.mode, mode, "L typed a chord");
        let a_minor_seventh = lineup_core::chord_sheet::HarmonyName::parse("Am7").unwrap();
        assert_eq!(
            screen.app.project.chord_at(48).map(|marker| marker.harmony),
            Some(a_minor_seventh)
        );
        // In letters, no chord starts with L: it switches pattern and song.
        crate::i18n::set_note_naming(&screen.ctx, lineup_core::NoteNaming::Letters);
        screen.click("Am7");
        type_l(&mut screen, "L");
        assert_ne!(screen.app.mode, mode);
        assert_eq!(
            screen.app.project.chord_at(48).map(|marker| marker.harmony),
            Some(a_minor_seventh)
        );
    }

    #[test]
    fn the_bar_numbers_insert_and_delete_bars_too() {
        let mut screen = song();
        screen.app.chord_sheet.open = false;
        let before = chords(&screen);
        // The playlist's bar number 2.
        screen.draw(Vec::new());
        let output = screen.draw(Vec::new());
        let two = output
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                Shape::Text(text) if text.galley.text() == "2" => Some(text.pos),
                _ => None,
            })
            .expect("bar 2 on the ruler");
        let at = two + vec2(2.0, 4.0);
        screen.draw(vec![Event::PointerMoved(at)]);
        for pressed in [true, false] {
            screen.draw(vec![Event::PointerButton {
                pos: at,
                button: PointerButton::Secondary,
                pressed,
                modifiers: Modifiers::NONE,
            }]);
        }
        screen.click("Insert a bar before");
        assert_eq!(
            chords(&screen),
            [
                (0, "C".to_string()),
                (32, "G".into()),
                (48, "Am".into()),
                (64, "F".into())
            ]
        );
        screen.key_with_modifiers(Key::Z, Modifiers::COMMAND);
        assert_eq!(chords(&screen), before);
    }

    #[test]
    fn in_spanish_the_loop_button_and_the_bands_choruses_read_as_spanish() {
        let mut screen = song();
        crate::i18n::set_language(&screen.ctx, crate::i18n::Language::Spanish);
        assert!(screen.app.project.set_choruses(3));
        // Bars 2 and 3 selected, with a click and a Shift+click.
        screen.click("G");
        screen.draw(vec![]);
        let am = text_rect(&screen.draw(vec![]), "Am").unwrap().center();
        screen.click_at(am, Modifiers::SHIFT);
        assert_eq!(screen.app.chord_sheet.selected_bars(), Some((1, 2)));
        screen.draw(vec![]);
        let output = screen.draw(vec![]);
        assert!(text_rect(&output, "🔁 Repetir los compases 2–3").is_some());
        // A chorus here is a pass through the tune, not a pop chorus.
        assert!(text_rect(&output, "Vuelta ×3").is_some());
        assert!(text_rect(&output, "Estribillo ×3").is_none());
    }
}

mod section_styles {
    use super::*;
    use lineup_core::BandStyle;

    /// A song with a band, four bars of a verse and four of a chorus,
    /// with the chord sheet open.
    fn song() -> Screen {
        let mut screen = Screen::new();
        let mut project = Project::with_band();
        project.write_progression(
            lineup_core::library::Style::named("Four chords").unwrap(),
            0,
        );
        project.write_progression(
            lineup_core::library::Style::named("Four chords").unwrap(),
            64,
        );
        project.add_section(lineup_core::Section::new("Verse", 0, 64));
        project.add_section(lineup_core::Section::new("Chorus", 64, 64));
        project.follow_chord_sheet();
        screen.app.replace_project(project, None);
        screen.app.rack_open = false;
        screen.app.chord_sheet.open = true;
        screen
    }

    fn right_click(screen: &mut Screen, label: &str) {
        screen.draw(Vec::new());
        let output = screen.draw(Vec::new());
        let at = text_rect(&output, label)
            .unwrap_or_else(|| panic!("Missing: {label}"))
            .center();
        screen.draw(vec![Event::PointerMoved(at)]);
        for pressed in [true, false] {
            screen.draw(vec![Event::PointerButton {
                pos: at,
                button: PointerButton::Secondary,
                pressed,
                modifiers: Modifiers::NONE,
            }]);
        }
    }

    /// Clicks `label` in a menu that may move as it opens: the pointer
    /// goes to it, and then clicks where it is drawn next.
    fn follow(screen: &mut Screen, label: &str) {
        let output = screen.draw(Vec::new());
        let at = text_rect(&output, label)
            .unwrap_or_else(|| panic!("Missing: {label}"))
            .center();
        let output = screen.draw(vec![Event::PointerMoved(at)]);
        let at = text_rect(&output, label)
            .unwrap_or_else(|| panic!("Missing: {label}"))
            .center();
        screen.draw(vec![Event::PointerMoved(at)]);
        for pressed in [true, false] {
            screen.draw(vec![Event::PointerButton {
                pos: at,
                button: PointerButton::Primary,
                pressed,
                modifiers: Modifiers::NONE,
            }]);
        }
    }

    #[test]
    fn a_sections_menu_plays_it_in_another_style() {
        let mut screen = song();
        let own = screen.app.project.band().unwrap().name().to_string();
        right_click(&mut screen, "Chorus");
        screen.click(&format!("Style: {own}"));
        follow(&mut screen, "Latin");
        follow(&mut screen, "Bossa nova");
        let bossa = BandStyle::named("Bossa nova");
        assert_eq!(screen.app.project.section_style_at(64), bossa);
        assert_eq!(screen.app.project.section_style_at(63), None);
        assert_eq!(screen.app.history.undo_label(), Some("section style"));
        assert!(
            screen
                .app
                .notice
                .as_deref()
                .is_some_and(|notice| notice.contains("Bars 5–8: the band plays Bossa nova")),
            "{:?}",
            screen.app.notice
        );
        // The sheet names the style where it starts.
        screen.app.chord_sheet.new_song();
        screen.draw(Vec::new());
        let output = screen.draw(Vec::new());
        assert!(text_rect(&output, "Bossa nova").is_some());
        // A bar's menu goes back to the song's style from there on.
        right_click(&mut screen, "Bossa nova");
        screen.click("Style: Bossa nova");
        follow(&mut screen, &format!("The song's style: {own}"));
        assert!(screen.app.project.section_styles().is_empty());
        screen.app.undo();
        assert_eq!(screen.app.project.section_style_at(64), bossa);
    }
}

#[test]
fn the_chord_sheet_shows_its_chords_as_sets_and_micromodes_and_reads_the_notes() {
    let mut screen = Screen::new();
    let mut project = Project::with_band();
    project.write_progression(
        lineup_core::library::Style::named("Four chords").unwrap(),
        0,
    );
    project.follow_chord_sheet();
    screen.app.replace_project(project, None);
    screen.app.rack_open = false;
    screen.app.chord_sheet.open = true;
    let before = screen.app.project.clone();
    let shows = |screen: &mut Screen, text: &str| {
        screen.draw(Vec::new());
        let output = screen.draw(Vec::new());
        text_rect(&output, text).is_some()
    };
    assert!(shows(&mut screen, "C") && shows(&mut screen, "Am"));
    // C G Am F in C major: the same notes as sets...
    screen.click("As written");
    screen.click("Pitch-class sets");
    for set in ["3-11 T7I", "3-11 T2I", "3-11 T9", "3-11 T0I"] {
        assert!(shows(&mut screen, set), "{set}");
    }
    assert!(shows(&mut screen, "C E G"));
    // ...and as micromodes, all four the major-and-minor triad.
    screen.click("Pitch-class sets");
    screen.click("Micromodes");
    assert!(shows(&mut screen, "Acorde menor-mayor T7I"));
    assert!(shows(&mut screen, "Acorde menor-mayor T9"));
    // The notes the band plays, read in threes from the lowest up.
    screen.click("The sheet's harmonies");
    screen.click("Notes together (vertical)");
    assert!(shows(&mut screen, "3 notes"));
    assert!(shows(&mut screen, "All tracks"));
    screen.click("Notes together (vertical)");
    screen.click("Notes in a line (serial)");
    assert!(shows(&mut screen, "of"));
    // And back to chords: nothing in the song has changed.
    screen.click("Micromodes");
    screen.click("Chords");
    assert!(shows(&mut screen, "Am") && !shows(&mut screen, "Notes in a line (serial)"));
    assert_eq!(screen.app.project, before);
}

mod chord_sheet_layout {
    use super::*;

    /// A song of four chords whose first bar changes harmony on every
    /// beat, with the chord sheet open in its window.
    fn busy_song() -> Screen {
        let mut screen = Screen::new();
        let mut project = Project::with_band();
        project.write_progression(
            lineup_core::library::Style::named("Four chords").unwrap(),
            0,
        );
        let bar = project.meter().bar(0);
        let markers =
            lineup_core::chord_sheet::read_bar("Dm7/C F/C G7/D C/E", &bar, project.scale).unwrap();
        assert!(project.set_chords(bar.start, bar.end(), &markers));
        project.follow_chord_sheet();
        screen.app.replace_project(project, None);
        screen.app.rack_open = false;
        screen.app.chord_sheet.open = true;
        screen
    }

    /// The size `name` is written at on the sheet.
    fn size(output: &FullOutput, name: &str) -> f32 {
        output
            .shapes
            .iter()
            .rev()
            .find_map(|shape| match &shape.shape {
                Shape::Text(text) if text.galley.text() == name => text
                    .galley
                    .job
                    .sections
                    .first()
                    .map(|section| section.format.font_id.size),
                _ => None,
            })
            .unwrap_or_else(|| panic!("Missing: {name}"))
    }

    /// How many bars the lines hold, as "Bars per line" says when they
    /// fit the names.
    fn fit(output: &FullOutput) -> usize {
        (1..=crate::chord_sheet::layout::MOST_BARS)
            .find(|bars| text_rect(output, &format!("Fit ({bars})")).is_some())
            .expect("Bars per line says how many fit")
    }

    /// Opens the "Bars per line" choices: the numbers on it are also
    /// bars' numbers, so it is found by its label.
    fn open_bars_per_line(screen: &mut Screen) {
        screen.draw(Vec::new());
        let output = screen.draw(Vec::new());
        let label = text_rect(&output, "Bars per line").expect("Bars per line");
        screen.click_at(
            Pos2::new(label.right() + 24.0, label.center().y),
            Modifiers::NONE,
        );
    }

    #[test]
    fn lines_hold_as_many_bars_as_show_every_name_in_full() {
        let mut screen = busy_song();
        screen.draw(Vec::new());
        let output = screen.draw(Vec::new());
        // Four harmonies in a bar need more room than a quarter of the
        // window: the lines hold fewer bars, and every name is whole.
        let fits = fit(&output);
        assert!(fits < 4, "{fits}");
        for name in ["Dm7/C", "F/C", "G7/D", "C/E"] {
            assert_eq!(size(&output, name), 20.0, "{name}");
        }
        // A number chosen is kept, and the names that don't fit then
        // are written smaller. The numbers are in rows of four, every one
        // in sight: 16 is next to 13, not scrolled out of the menu.
        open_bars_per_line(&mut screen);
        let output = screen.draw(Vec::new());
        let row = |number: &str| text_rect(&output, number).expect(number).center().y;
        assert_eq!(row("16"), row("13"));
        assert!(row("1") < row("16"));
        screen.click("8");
        let output = screen.draw(Vec::new());
        assert!(text_rect(&output, "Fit (8)").is_none());
        assert!(size(&output, "Dm7/C") < 20.0);
        // Back to fitting the names.
        open_bars_per_line(&mut screen);
        screen.click("Fit the names");
        let output = screen.draw(Vec::new());
        assert_eq!(fit(&output), fits);
        assert_eq!(size(&output, "Dm7/C"), 20.0);
    }

    #[test]
    fn the_sheet_fills_the_main_window_and_goes_back() {
        let mut screen = busy_song();
        screen.app.playlist.browser_open = true;
        screen.draw(Vec::new());
        let output = screen.draw(Vec::new());
        let in_window = fit(&output);
        assert!(text_rect(&output, "Playlist").is_some());
        screen.click("⛶ Fill the window");
        assert!(screen.app.chord_sheet.fills_window());
        screen.draw(Vec::new());
        let output = screen.draw(Vec::new());
        // The sheet is where the playlist was, beside the patterns, with
        // more bars on a line and the names still whole.
        assert!(text_rect(&output, "Playlist").is_none());
        assert!(text_rect(&output, "Patterns").is_some());
        assert!(text_rect(&output, "Back to the playlist").is_some());
        assert!(fit(&output) > in_window);
        assert_eq!(size(&output, "Dm7/C"), 20.0);
        // F5 puts it away and brings it back where it was.
        screen.key(Key::F5);
        assert!(!screen.app.chord_sheet.open);
        let output = screen.draw(Vec::new());
        assert!(text_rect(&output, "Playlist").is_some());
        screen.key(Key::F5);
        assert!(screen.app.chord_sheet.fills_window());
        // The next run opens it there too.
        screen.restart();
        screen.app.chord_sheet.open = true;
        screen.draw(Vec::new());
        assert!(screen.app.chord_sheet.fills_window());
        // The button puts it back in its window, and the View menu
        // there again.
        screen.click("Back to the playlist");
        assert!(!screen.app.chord_sheet.fills_window());
        assert!(screen.app.chord_sheet.open);
        let output = screen.draw(Vec::new());
        assert!(text_rect(&output, "Playlist").is_some());
        assert!(text_rect(&output, "⛶ Fill the window").is_some());
        screen.click("View");
        screen.click("Chord sheet in the main window");
        assert!(screen.app.chord_sheet.fills_window());
    }
}

/// With Spanish chosen, every window, menu and settings page writes in
/// Spanish: nothing it asks to translate is missing from the catalog,
/// and no text it draws is an English one the catalog has Spanish for.
#[test]
fn every_window_writes_in_spanish() {
    use lineup_core::i18n::{note_missed, spanish, take_missed};
    use lineup_core::library::{MICROMODE_STYLES, ROW_STYLES, STYLES, Song};
    use std::collections::BTreeSet;

    fn english(output: &FullOutput, found: &mut BTreeSet<String>) {
        for shape in &output.shapes {
            if let Shape::Text(text) = &shape.shape {
                let shown = text.galley.text();
                if spanish(shown).is_some_and(|spanish| spanish != shown) {
                    found.insert(shown.to_string());
                }
            }
        }
    }

    let mut screen = Screen::new();
    crate::i18n::set_language(&screen.ctx, crate::i18n::Language::Spanish);
    note_missed();
    let mut found = BTreeSet::new();
    let demos = [
        library::Demo::Start,
        library::Demo::Song(Song::named("El Choclo").unwrap()),
        library::Demo::Song(Song::named("Ode to Joy").unwrap()),
        library::Demo::Style(&STYLES[0]),
        library::Demo::Row(&ROW_STYLES[0]),
        library::Demo::Micromode(&MICROMODE_STYLES[0]),
    ];
    for demo in demos {
        screen.app.open_demo(demo);
        screen.app.set_playing(false);
        screen.app.rack_open = true;
        screen.app.piano_roll.open = true;
        screen.app.harmony_keys.open = true;
        screen.app.chord_sheet.open = true;
        screen.app.mixer_open = true;
        screen.app.playlist.browser_open = true;
        screen.app.ear_training.open = true;
        screen.app.lead_sheet.open = true;
        for _ in 0..3 {
            english(&screen.draw(vec![]), &mut found);
        }
    }
    for open in [
        |app: &mut LineupApp| app.shortcuts_open = true,
        |app: &mut LineupApp| app.about_open = true,
        |app: &mut LineupApp| app.ask_for_video(),
        |app: &mut LineupApp| {
            app.start_style_editor(lineup_core::library::UserStyle::blank(), None)
        },
    ] {
        open(&mut screen.app);
        english(&screen.draw(vec![]), &mut found);
        english(&screen.draw(vec![]), &mut found);
        screen.key(Key::Escape);
    }
    screen.app.settings_open = true;
    for page in ["Appearance", "Harmony", "Library", "Audio", "Playback"] {
        screen.click(tr(page));
        english(&screen.draw(vec![]), &mut found);
    }
    screen.app.settings_open = false;
    for menu in ["File", "Edit", "View", "MIDI", "Help"] {
        screen.click(tr(menu));
        english(&screen.draw(vec![]), &mut found);
        screen.key(Key::Escape);
    }
    let missed = take_missed();
    assert!(
        missed.is_empty() && found.is_empty(),
        "{} texts have no Spanish:\n{}\n\n{} are drawn in English:\n{}",
        missed.len(),
        missed.into_iter().collect::<Vec<_>>().join("\n"),
        found.len(),
        found.into_iter().collect::<Vec<_>>().join("\n"),
    );
}
