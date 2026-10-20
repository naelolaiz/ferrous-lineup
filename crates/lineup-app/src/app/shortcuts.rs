//! Help > Keyboard shortcuts (F1): every key the app answers to, window by
//! window.
//!
//! The list is built from the same constants the windows read, where there
//! are constants, so it names a key as the menus do: Ctrl on Linux and
//! Windows, ⌘ on a Mac.

use eframe::egui::{self, Key, KeyboardShortcut, Modifiers, Rect};

use super::{LineupApp, edit, files, view};
use crate::piano_roll;

/// The keys of one line of the list.
#[derive(Clone)]
enum Keys {
    /// Any of these does it.
    Any(Vec<KeyboardShortcut>),
    /// The first does the first half of what the line says, the second the
    /// second: "Up / down".
    Pair(KeyboardShortcut, KeyboardShortcut),
    /// The mouse wheel, turned while holding these.
    Wheel(Modifiers),
    /// Keys that are not one shortcut, written out.
    Written(&'static str),
}

/// A heading of the list, a few words on when its keys work, and its lines.
pub(super) struct Section {
    pub(super) title: &'static str,
    note: &'static str,
    lines: Vec<(Keys, &'static str)>,
}

const fn key(key: Key) -> KeyboardShortcut {
    KeyboardShortcut::new(Modifiers::NONE, key)
}

const fn command(key: Key) -> KeyboardShortcut {
    KeyboardShortcut::new(Modifiers::COMMAND, key)
}

/// Copy, cut, paste and duplicate, as the piano roll and the playlist
/// read them.
fn clipboard_lines(what: &'static str) -> Vec<(Keys, &'static str)> {
    vec![
        (Keys::Any(vec![command(Key::C)]), "Copy"),
        (Keys::Any(vec![command(Key::X)]), "Cut"),
        (Keys::Any(vec![command(Key::V)]), "Paste"),
        (Keys::Any(vec![command(Key::D)]), "Duplicate"),
        (
            Keys::Any(vec![piano_roll::DELETE, piano_roll::BACKSPACE]),
            "Delete",
        ),
        (Keys::Any(vec![piano_roll::SELECT_ALL]), what),
        (Keys::Any(vec![piano_roll::ESCAPE]), "Select nothing"),
    ]
}

/// Everything in the list, in the order it is shown.
pub(super) fn sections() -> Vec<Section> {
    use Keys::{Any, Pair, Wheel, Written};
    vec![
        Section {
            title: "Anywhere",
            note: "Except while typing in a text field.",
            lines: vec![
                (
                    Any(vec![key(Key::Space)]),
                    "Play or stop (in the rack or the piano roll: the pattern)",
                ),
                (Any(vec![key(Key::Home)]), "Go to the start"),
                (Any(vec![key(Key::End)]), "Go to the end"),
                (
                    Pair(key(Key::PageUp), key(Key::PageDown)),
                    "Back / forward one bar",
                ),
                (Any(vec![key(Key::L)]), "Pattern or song mode"),
                (Any(vec![view::HELP]), "This list"),
            ],
        },
        Section {
            title: "Windows",
            note: "Show or hide them.",
            lines: vec![
                (Any(vec![view::CHORD_SHEET]), "Chord sheet"),
                (Any(vec![view::RACK]), "Channel rack"),
                (Any(vec![view::PIANO_ROLL]), "Piano roll"),
                (Any(vec![view::HARMONY]), "Harmony keys"),
                (Any(vec![view::BROWSER]), "Patterns and library"),
                (Any(vec![view::MIXER]), "Mixer"),
            ],
        },
        Section {
            title: "File and Edit",
            note: "",
            lines: vec![
                (Any(vec![files::NEW]), "New"),
                (Any(vec![files::OPEN]), "Open"),
                (Any(vec![files::SAVE]), "Save"),
                (Any(vec![files::SAVE_AS]), "Save as"),
                (Any(vec![files::QUIT]), "Quit"),
                (Any(vec![edit::UNDO]), "Undo"),
                (Any(vec![edit::REDO, edit::REDO_Y]), "Redo"),
            ],
        },
        Section {
            title: "Zoom",
            note: "What is under the pointer: the playlist, the piano roll, the rack, \
                   or the whole window anywhere else.",
            lines: vec![
                (Pair(view::WIDER, view::NARROWER), "Wider / narrower"),
                (Pair(view::TALLER, view::SHORTER), "Taller / shorter"),
                (Any(vec![view::RESET]), "Back to 100%"),
                (Wheel(Modifiers::COMMAND), "Wider or narrower"),
                (Wheel(Modifiers::ALT), "Taller or shorter"),
            ],
        },
        Section {
            title: "Chord sheet",
            note: "After clicking a bar.",
            lines: vec![
                (
                    Written("Arrows, Tab, Shift+Tab"),
                    "Move between bars (with Shift, select them)",
                ),
                (
                    Any(vec![key(Key::Enter), key(Key::F2)]),
                    "Type in the bar (or just start typing)",
                ),
                (
                    Any(vec![key(Key::Delete), key(Key::Backspace)]),
                    "Clear the selected bars",
                ),
                (Any(vec![command(Key::A)]), "Select every bar"),
                (Any(vec![command(Key::C)]), "Copy the bars, as text"),
                (Any(vec![command(Key::X)]), "Cut the bars"),
                (Any(vec![command(Key::V)]), "Paste bars, or a chord chart"),
                (Any(vec![key(Key::Escape)]), "Select nothing"),
            ],
        },
        Section {
            title: "Piano roll",
            note: "The selected notes, or all of the channel's when none is selected.",
            lines: [
                vec![
                    (
                        Pair(piano_roll::KEY_UP, piano_roll::KEY_DOWN),
                        "Up / down a semitone",
                    ),
                    (
                        Pair(piano_roll::OCTAVE_UP, piano_roll::OCTAVE_DOWN),
                        "Up / down an octave",
                    ),
                    (
                        Pair(piano_roll::IN_KEY_UP, piano_roll::IN_KEY_DOWN),
                        "Up / down a note in the key",
                    ),
                    (
                        Pair(piano_roll::KEY_LEFT, piano_roll::KEY_RIGHT),
                        "A step earlier / later",
                    ),
                    (
                        Pair(piano_roll::BEAT_LEFT, piano_roll::BEAT_RIGHT),
                        "A beat earlier / later",
                    ),
                ],
                clipboard_lines("Select every note"),
            ]
            .concat(),
        },
        Section {
            title: "Playlist",
            note: "The selected clips.",
            lines: clipboard_lines("Select every clip"),
        },
        Section {
            title: "Channel rack",
            note: "After clicking a channel's name.",
            lines: vec![
                (
                    Any(vec![key(Key::Delete), key(Key::Backspace)]),
                    "Delete the channel",
                ),
                (Any(vec![key(Key::Escape)]), "Select no channel"),
            ],
        },
        Section {
            title: "Harmony keys",
            note: "",
            lines: vec![(
                Written("1 to 7"),
                "Play the chord on each degree of the key",
            )],
        },
        Section {
            title: "Typing keyboard",
            note: "When MIDI > Typing keyboard to piano is on.",
            lines: vec![
                (Written("Z X C V B N M"), "White keys from C4"),
                (Written("S D G H J"), "Black keys from C#4"),
                (Written("Q W E R T Y U I O P"), "White keys from C5"),
                (Written("2 3 5 6 7 9 0"), "Black keys from C#5"),
            ],
        },
        Section {
            title: "Style editor",
            note: "",
            lines: vec![
                (Any(vec![key(Key::Space)]), "Play the style"),
                (Any(vec![files::SAVE]), "Save the style"),
                (Any(vec![files::SAVE_AS]), "Save the style as"),
                (Any(vec![edit::UNDO]), "Undo"),
                (Any(vec![edit::REDO, edit::REDO_Y]), "Redo"),
                (Any(vec![key(Key::Escape)]), "Select no channel"),
            ],
        },
    ]
}

impl Keys {
    fn text(&self, ctx: &egui::Context) -> String {
        match self {
            Keys::Any(keys) => keys
                .iter()
                .map(|key| ctx.format_shortcut(key))
                .collect::<Vec<_>>()
                .join(" or "),
            Keys::Pair(first, second) => format!(
                "{} / {}",
                ctx.format_shortcut(first),
                ctx.format_shortcut(second)
            ),
            Keys::Wheel(modifiers) => format!("{}+wheel", ctx.format_modifiers(*modifiers)),
            Keys::Written(text) => (*text).to_owned(),
        }
    }
}

impl LineupApp {
    /// Help, at the right of the menus.
    pub(super) fn help_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Help", |ui| {
            let shortcut = ui.ctx().format_shortcut(&view::HELP);
            let item = egui::Button::selectable(self.shortcuts_open, "Keyboard shortcuts")
                .shortcut_text(shortcut);
            if ui.add(item).clicked() {
                self.shortcuts_open = !self.shortcuts_open;
                ui.close();
            }
        });
    }

    pub(super) fn shortcuts_window(&mut self, ctx: &egui::Context, area: Rect) {
        if !self.shortcuts_open {
            return;
        }
        let mut open = self.shortcuts_open;
        egui::Window::new("Keyboard shortcuts")
            .id(egui::Id::unique("keyboard-shortcuts"))
            .open(&mut open)
            .default_size([460.0, 560.0])
            .constrain_to(area)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink(false)
                    .show(ui, |ui| {
                        for section in sections() {
                            ui.add_space(4.0);
                            ui.strong(section.title);
                            if !section.note.is_empty() {
                                ui.weak(section.note);
                            }
                            // As wide as the longest keys, so the sections'
                            // columns line up.
                            egui::Grid::new(("shortcuts", section.title))
                                .num_columns(2)
                                .striped(true)
                                .min_col_width(180.0)
                                .show(ui, |ui| {
                                    for (keys, what) in &section.lines {
                                        ui.monospace(keys.text(ctx));
                                        ui.label(*what);
                                        ui.end_row();
                                    }
                                });
                            ui.add_space(6.0);
                        }
                    });
            });
        self.shortcuts_open = open;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The shortcuts of a section, each alone.
    fn pressed(section: &Section) -> Vec<KeyboardShortcut> {
        section
            .lines
            .iter()
            .flat_map(|(keys, _)| match keys {
                Keys::Any(keys) => keys.clone(),
                Keys::Pair(first, second) => vec![*first, *second],
                Keys::Wheel(_) | Keys::Written(_) => Vec::new(),
            })
            .collect()
    }

    #[test]
    fn no_two_global_shortcuts_share_a_key() {
        // These work wherever the keyboard is, so a key listed twice would
        // do two things at once.
        let global: Vec<KeyboardShortcut> = sections()
            .iter()
            .filter(|section| {
                ["Anywhere", "Windows", "File and Edit", "Zoom"].contains(&section.title)
            })
            .flat_map(pressed)
            .collect();
        for (index, shortcut) in global.iter().enumerate() {
            assert!(
                !global[index + 1..].contains(shortcut),
                "{shortcut:?} is listed twice"
            );
        }
    }

    #[test]
    fn keys_are_written_as_the_menus_write_them() {
        let ctx = egui::Context::default();
        // As on Linux and Windows. On a Mac, egui writes ⇧⌘Z, if the
        // font it has by then can show those symbols.
        ctx.set_os(egui::os::OperatingSystem::Nix);
        assert_eq!(
            Keys::Any(vec![edit::REDO, edit::REDO_Y]).text(&ctx),
            "Ctrl+Shift+Z or Ctrl+Y"
        );
        assert_eq!(
            Keys::Pair(key(Key::PageUp), key(Key::PageDown)).text(&ctx),
            "PageUp / PageDown"
        );
        assert_eq!(Keys::Wheel(Modifiers::ALT).text(&ctx), "Alt+wheel");
    }

    #[test]
    fn every_section_has_lines_and_a_unique_title() {
        let sections = sections();
        for (index, section) in sections.iter().enumerate() {
            assert!(!section.lines.is_empty(), "{} is empty", section.title);
            assert!(
                sections[index + 1..]
                    .iter()
                    .all(|other| other.title != section.title),
                "{} is there twice",
                section.title
            );
        }
    }
}
