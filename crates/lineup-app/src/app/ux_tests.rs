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
        Self {
            ctx,
            app,
            frame: eframe::Frame::_new_kittest(),
            size: vec2(1400.0, 1000.0),
            settings_only: false,
        }
    }

    fn draw(&mut self, events: Vec<Event>) -> FullOutput {
        let mut output = self.ctx.run_pass(
            RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, self.size)),
                time: Some(self.ctx.cumulative_frame_nr() as f64 * 0.2),
                events,
                ..Default::default()
            },
            |ctx| {
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
            },
        );
        output.textures_delta.clear();
        output
    }

    fn click(&mut self, label: &str) {
        self.draw(vec![]);
        let output = self.draw(vec![]);
        let pos = text_rect(&output, label)
            .map(|rect| rect.center())
            .unwrap_or_else(|| panic!("Missing control: {label}"));
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
fn plain_accents_popup_hides_instruments_and_restores_their_saved_choice() {
    use lineup_core::library::apply::StyleParts;
    for instruments in [false, true] {
        let mut screen = Screen::new();
        screen.app.rack_open = false;
        screen.app.playlist.browser_open = true;
        *screen.app.playlist.library_parts_mut() = StyleParts {
            instruments,
            progression: false,
            time_signature: false,
            tempo: false,
            swing: false,
            ..Default::default()
        };
        screen.click("📚 Library");
        screen.click("🔍 Search: bossa, 6/8, tango…");
        screen.draw(vec![Event::Text("Tango".into())]);
        screen.click("Tango");
        screen.click("Plain accents");
        let output = screen.draw(vec![]);
        assert!(text_rect(&output, "Drums and plain accents").is_some());
        assert!(text_rect(&output, "Playing patterns").is_none());
        assert!(text_rect(&output, "Voices / instruments").is_none());
        assert!(text_rect(&output, "Chord progression").is_some());
        assert!(text_rect(&output, "Chord progression (when included)").is_none());
        screen.click("Preset patterns");
        let output = screen.draw(vec![]);
        assert_eq!(
            checkbox_checked(&output, "Voices / instruments"),
            instruments
        );
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
        assert_eq!(
            screen.app.playlist.library_parts_mut().instruments,
            instruments
        );
        assert!(!screen.app.playlist.library_parts().instruments);
        assert!(!screen.app.playlist.library_parts().matching_style);
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
    screen.draw(vec![Event::Text("Aeolian climb".into())]);
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

#[test]
fn main_transport_toggles_are_visible_reversible_and_saved() {
    for size in [vec2(1400.0, 1000.0), vec2(800.0, 600.0)] {
        let mut screen = Screen::new();
        screen.size = size;
        screen.ctx.set_zoom_factor(1.1);
        let project = screen.app.project.clone();
        screen.draw(vec![]);
        screen.draw(vec![]);
        let output = screen.draw(vec![]);
        for label in ["Follow playhead", "Count-in", "Metronome"] {
            let rect = text_rect(&output, label).unwrap();
            assert!(screen.ctx.content_rect().contains_rect(rect));
        }
        assert!(button_lit(&output, "Follow playhead"));
        assert!(!button_lit(&output, "Count-in"));
        assert!(!button_lit(&output, "Metronome"));
        for label in ["Follow playhead", "Count-in", "Metronome"] {
            screen.click(label);
        }
        let output = screen.draw(vec![]);
        assert!(!button_lit(&output, "Follow playhead"));
        assert!(button_lit(&output, "Count-in"));
        assert!(button_lit(&output, "Metronome"));
        assert!(!widgets::follow_playback(&screen.ctx));
        let restored = practice::Practice::new(&screen.ctx).view();
        assert!(restored.count_in && restored.metronome);
        for label in ["Follow playhead", "Count-in", "Metronome"] {
            screen.click(label);
        }
        assert!(widgets::follow_playback(&screen.ctx));
        assert!(!screen.app.practice.view().count_in && !screen.app.practice.view().metronome);
        assert_eq!(screen.app.project, project);
        assert!(!screen.app.playing);
    }
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
                "preset-copy-options"
            ))
        ),
        Some(saved)
    );
}

#[test]
fn settings_navigation_and_footer_stay_visible_on_small_screens() {
    let mut screen = Screen::new();
    screen.size = vec2(640.0, 480.0);
    screen.app.settings_open = true;
    screen.draw(vec![]);
    let output = screen.draw(vec![]);
    let footer = text_rect(&output, "Done").unwrap();
    for page in ["Appearance", "Library", "Audio", "Playback"] {
        screen.click(page);
        let output = screen.draw(vec![]);
        let done = text_rect(&output, "Done").expect("Settings footer is visible");
        assert!(Rect::from_min_size(Pos2::ZERO, screen.size).contains_rect(done));
        assert!(
            (done.bottom() - footer.bottom()).abs() < 1.0,
            "Changing categories must not grow the window"
        );
        for page in ["Appearance", "Library", "Audio", "Playback"] {
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
