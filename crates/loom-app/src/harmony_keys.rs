//! The harmony keys bar along the bottom of the window.
//!
//! On the left, the key and the time signature of the project. In the
//! middle, a big key for each of the seven chords of the key, coloured by
//! what the chord does (home, away, tension); hold one down to play its
//! chord, and the chords that often come next are outlined, the most
//! usual one brightest. The hint line below can explain in sentences, in
//! a few words, or not at all (see [`HintLevel`]). On the right, a
//! keyboard showing the notes of the scale and the notes playing.
//!
//! Like the other views, [`HarmonyKeys::show`] does not change anything
//! itself: it returns [`HarmonyAction`]s for the app to carry out.

use eframe::egui::{
    self, Align2, Color32, FontId, Key, Painter, Rect, Response, Sense, Stroke, StrokeKind, Ui,
    pos2, vec2,
};
use loom_core::harmony::keys::{self, ChordKey, ChordSize, Function, GroupKey};
use loom_core::harmony::{Harmony, HintLevel, Suggestion, analyze, next_chords};
use loom_core::{
    Chord, Note, PitchClass, PitchSet, Project, Scale, ScaleKind, Spelling, TimeSignature,
};

use crate::piano_roll::{KEY_HELP, both_names};

/// Something the user did in the harmony keys bar.
#[derive(Debug, Clone, PartialEq)]
pub enum HarmonyAction {
    SetScale(Scale),
    SetTime(TimeSignature),
    /// A harmony key went down: start these notes.
    NoteOn(Vec<Note>),
    /// A harmony key came up: stop these notes.
    NoteOff(Vec<Note>),
}

const KEY_SIZE: egui::Vec2 = vec2(76.0, 66.0);
const KEY_GAP: f32 = 4.0;
/// The keyboard on the right: from C2, four octaves.
const PIANO_LOWEST: u8 = 36;
const PIANO_OCTAVES: u8 = 4;
const WHITE_WIDTH: f32 = 11.0;
const PIANO_HEIGHT: f32 = 58.0;

/// The colour of each function: blue for home, green for away, orange for
/// tension.
fn function_color(function: Function) -> Color32 {
    match function {
        Function::Tonic => Color32::from_rgb(76, 136, 222),
        Function::Subdominant => Color32::from_rgb(86, 176, 110),
        Function::Dominant => Color32::from_rgb(226, 124, 64),
    }
}

const KEY_TEXT: Color32 = Color32::from_rgb(244, 246, 250);
const KEY_TEXT_DIM: Color32 = Color32::from_rgb(196, 202, 214);
const SUGGESTED: Color32 = Color32::from_rgb(255, 214, 92);
const WHITE_KEY: Color32 = Color32::from_rgb(214, 218, 226);
const BLACK_KEY: Color32 = Color32::from_rgb(40, 42, 48);
const KEY_BORDER: Color32 = Color32::from_rgb(90, 94, 104);
const SCALE_DOT: Color32 = Color32::from_rgb(76, 136, 222);
const ROOT_DOT: Color32 = Color32::from_rgb(226, 124, 64);
const HELD: Color32 = Color32::from_rgb(255, 152, 48);
const HEARD: Color32 = Color32::from_rgb(132, 178, 240);

/// The colour of the keys of a row or a set, which have no functions.
const GROUP_COLOR: Color32 = Color32::from_rgb(128, 100, 196);

/// What a harmony key plays when it goes down.
struct Play {
    /// The notes of the chord without the bass, for the next chord to
    /// move smoothly from.
    voicing: Vec<Note>,
    notes: Vec<Note>,
    /// The chord, for the keys of a key; none for a row or a set.
    chord: Option<Chord>,
}

/// What is holding a harmony key down.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Holder {
    Mouse,
    /// A number key on the computer keyboard, 1 to 7.
    Digit(Key),
}

/// The harmony key that is down, and the notes it started.
#[derive(Debug, Clone, PartialEq)]
struct Down {
    key: usize,
    holder: Holder,
    notes: Vec<Note>,
}

/// The number keys that play the harmony keys, I to vii.
const DIGITS: [Key; 7] = [
    Key::Num1,
    Key::Num2,
    Key::Num3,
    Key::Num4,
    Key::Num5,
    Key::Num6,
    Key::Num7,
];

/// What the harmony keys bar remembers between frames.
pub struct HarmonyKeys {
    pub open: bool,
    /// How much the hints explain, here and in the piano roll.
    pub hints: HintLevel,
    size: ChordSize,
    down: Option<Down>,
    /// The notes of the last chord played (without the bass), so the next
    /// one moves as little as possible from it.
    last_voicing: Option<Vec<Note>>,
    /// The last chord played, here or on a MIDI keyboard, for the
    /// suggestions of what could come next.
    last_chord: Option<Chord>,
}

impl HarmonyKeys {
    pub fn new() -> Self {
        HarmonyKeys {
            open: true,
            hints: HintLevel::default(),
            size: ChordSize::Triads,
            down: None,
            last_voicing: None,
            last_chord: None,
        }
    }

    /// Lets go of the key that is down, if any. Called when the bar is
    /// closed, so no chord keeps playing.
    pub fn release(&mut self) -> Vec<HarmonyAction> {
        self.down
            .take()
            .map(|down| HarmonyAction::NoteOff(down.notes))
            .into_iter()
            .collect()
    }

    /// Draws the bar. `held` are the notes held down on any keyboard,
    /// harmony keys included, and `heard` the chord notes playing in the
    /// pattern or the song; `digits` says whether the number keys may
    /// play the harmony keys (not while they play notes on the typing
    /// keyboard, or a text field has the keyboard).
    pub fn show(
        &mut self,
        ui: &mut Ui,
        project: &Project,
        channel_name: &str,
        held: &[Note],
        heard: &[Note],
        digits: bool,
    ) -> Vec<HarmonyAction> {
        let scale = project.scale;
        if let Some(groups) = keys::group_keys(scale, self.size) {
            return self.show_groups(ui, project, &groups, channel_name, held, heard, digits);
        }
        let mut actions = Vec::new();
        let chord_keys = keys::chord_keys(scale, self.size);

        // A chord played on any keyboard, or else playing in the music,
        // lights its key, and is what the suggestions follow.
        let chord_of = |notes: &[Note]| match analyze(notes.iter().copied()) {
            Harmony::Chord { chord, .. } => Some(chord),
            _ => None,
        };
        let played = chord_of(held).or_else(|| chord_of(heard));
        if let Some(chord) = played {
            self.last_chord = Some(chord);
        }
        let playing = played.and_then(|chord| keys::key_for(&chord_keys, chord));
        let suggestions = self
            .last_chord
            .map(|chord| next_chords(keys::chord_scale(scale), chord))
            .unwrap_or_default();

        ui.add_space(4.0);
        let mut pressed = None;
        // Wrapped, so on a narrow window the little piano moves to a row
        // of its own instead of running off the edge.
        ui.horizontal_wrapped(|ui| {
            ui.vertical(|ui| self.settings(ui, project, &mut actions));
            ui.separator();
            for (index, key) in chord_keys.iter().enumerate() {
                // The rank of the key among the suggestions: 0 for the
                // most usual next chord.
                let rank = suggestions.iter().position(|s| s.degree == key.degree);
                let lit =
                    playing == Some(index) || self.down.as_ref().is_some_and(|d| d.key == index);
                let response = chord_key(ui, key, scale.spelling(), lit, rank);
                let mut hover = describe_key(key, scale.spelling(), self.hints);
                if let Some(rank) = rank {
                    let often = if rank == 0 { "Most often" } else { "Often" };
                    hover = match suggestions[rank].why(self.hints) {
                        Some(why) if self.hints == HintLevel::Detailed => {
                            format!("{hover}\n\n{often} comes next. {why}")
                        }
                        Some(why) => format!("{hover}\n\n{often} next: {why}."),
                        None => format!("{hover}\n\n{often} next."),
                    };
                }
                let response = crate::theme::hint(response, hover);
                if response.is_pointer_button_down_on() {
                    pressed = Some(index);
                }
            }
            ui.separator();
            piano(ui, scale, held, heard);
        });

        let suggests = matches!(
            keys::chord_scale(scale).kind,
            ScaleKind::Major
                | ScaleKind::NaturalMinor
                | ScaleKind::HarmonicMinor
                | ScaleKind::MelodicMinor
        );
        let outside = suggests
            && self
                .last_chord
                .is_some_and(|chord| keys::key_for(&chord_keys, chord).is_none());
        ui.horizontal_wrapped(|ui| {
            self.hints_menu(ui);
            let hint = hint_line(
                self.hints,
                scale,
                self.last_chord,
                &suggestions,
                suggests,
                outside,
                channel_name,
            );
            ui.weak(hint);
        });

        let plays: Vec<Play> = chord_keys
            .iter()
            .map(|key| {
                let (voicing, notes) = keys::play(key.chord, self.last_voicing.as_deref());
                Play {
                    voicing,
                    notes,
                    chord: Some(key.chord),
                }
            })
            .collect();
        self.follow_mouse(pressed, &plays, &mut actions);
        self.follow_digits(ui, digits, &plays, &mut actions);
        actions
    }

    /// The bar for a twelve-tone row or a pitch-class set: a key for each
    /// group of its notes (see [`keys::group_keys`]), all one colour,
    /// since music without a key has no home, away or tension.
    #[allow(clippy::too_many_arguments)]
    fn show_groups(
        &mut self,
        ui: &mut Ui,
        project: &Project,
        groups: &[GroupKey],
        channel_name: &str,
        held: &[Note],
        heard: &[Note],
        digits: bool,
    ) -> Vec<HarmonyAction> {
        let mut actions = Vec::new();
        let scale = project.scale;
        let spelling = scale.spelling();
        let heard_set: PitchSet = heard.iter().map(|note| note.pitch_class()).collect();
        let mut pressed = None;
        ui.add_space(4.0);
        ui.horizontal_wrapped(|ui| {
            ui.vertical(|ui| self.settings(ui, project, &mut actions));
            ui.separator();
            for (index, group) in groups.iter().enumerate() {
                let lit = self.down.as_ref().is_some_and(|d| d.key == index)
                    || (heard_set.len() >= 2 && group.pitch_set() == heard_set);
                let response = group_key(ui, group, spelling, lit);
                let response = crate::theme::hint(
                    response,
                    describe_group(group, spelling, index, self.hints),
                );
                if response.is_pointer_button_down_on() {
                    pressed = Some(index);
                }
            }
            ui.separator();
            piano(ui, scale, held, heard);
        });
        ui.horizontal_wrapped(|ui| {
            self.hints_menu(ui);
            let keys = format!("1 to {}", groups.len().min(7));
            let hint = match (self.hints, scale.kind) {
                (HintLevel::Minimal, _) => String::new(),
                (HintLevel::Short, ScaleKind::Row(_)) => {
                    format!("The row's notes in groups, in order. Hold a key or press {keys}.")
                }
                (HintLevel::Short, _) => {
                    format!("Chords stacked from the set's notes. Hold a key or press {keys}.")
                }
                (_, ScaleKind::Row(_)) => format!(
                    "Twelve-tone music has no home chord: each key plays the next notes of \
                     the row together, as twelve-tone composers build their chords. Hold one \
                     (or press {keys}) to play on \"{channel_name}\". 3, 4 or 6 notes on the left."
                ),
                _ => format!(
                    "Each key stacks every other note of the set, from one of its notes, as \
                     the chords of a key stack every other note of the scale. Hold one (or \
                     press {keys}) to play it."
                ),
            };
            ui.weak(hint);
        });
        let plays: Vec<Play> = groups
            .iter()
            .map(|group| Play {
                voicing: Vec::new(),
                notes: group.play(),
                chord: None,
            })
            .collect();
        self.follow_mouse(pressed, &plays, &mut actions);
        self.follow_digits(ui, digits, &plays, &mut actions);
        actions
    }

    /// How much the hints explain.
    fn hints_menu(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_id_salt("harmony-hints")
            .width(78.0)
            .selected_text(self.hints.label())
            .show_ui(ui, |ui| {
                for level in HintLevel::ALL {
                    ui.selectable_value(&mut self.hints, level, level.label())
                        .on_hover_text(level.description());
                }
            })
            .response
            .on_hover_text("How much the hints explain, here and in the piano roll");
    }

    /// The key, the time signature and the size of the chords.
    fn settings(&mut self, ui: &mut Ui, project: &Project, actions: &mut Vec<HarmonyAction>) {
        let scale = project.scale;
        ui.horizontal(|ui| {
            ui.strong("Key");
            let mut root = scale.root;
            egui::ComboBox::from_id_salt("harmony-root")
                .width(56.0)
                .selected_text(egui::RichText::new(root.name(scale.spelling())).strong())
                .show_ui(ui, |ui| {
                    for pitch_class in PitchClass::ALL {
                        ui.selectable_value(&mut root, pitch_class, both_names(pitch_class));
                    }
                })
                .response
                .on_hover_text(KEY_HELP);
            if let Some(new) = crate::twelve_tone::kind_menu(ui, "harmony-kind", scale, root, 110.0)
            {
                actions.push(HarmonyAction::SetScale(new));
            }
        });
        ui.horizontal(|ui| {
            ui.strong("Time");
            let time = project.time_signature;
            let mut chosen = time;
            egui::ComboBox::from_id_salt("harmony-time")
                .width(56.0)
                .height(320.0)
                .selected_text(egui::RichText::new(time.to_string()).strong())
                .show_ui(ui, |ui| {
                    for option in TimeSignature::ALL {
                        ui.selectable_value(&mut chosen, option, option.to_string())
                            .on_hover_text(describe_time(option));
                    }
                })
                .response
                .on_hover_text(describe_time(time));
            if chosen != time {
                actions.push(HarmonyAction::SetTime(chosen));
            }
            ui.weak(format!("{} steps a bar", time.steps_per_bar()));
        });
        ui.horizontal(|ui| {
            for size in ChordSize::ALL {
                let (label, hover) = match (scale.kind, size) {
                    (ScaleKind::Row(_), _) => {
                        let n = keys::row_group(size);
                        let name = match n {
                            3 => "trichords",
                            4 => "tetrachords",
                            _ => "hexachords",
                        };
                        (
                            format!("{n} notes"),
                            format!("The row in groups of {n}: {name}"),
                        )
                    }
                    (ScaleKind::Set(_), _) => {
                        let n = size as usize + 3;
                        (
                            format!("{n} notes"),
                            format!("Chords of {n} notes of the set"),
                        )
                    }
                    (_, ChordSize::Triads) => (
                        size.label().to_string(),
                        "Three notes: every other note of the scale".to_string(),
                    ),
                    (_, ChordSize::Sevenths) => (
                        size.label().to_string(),
                        "Four notes: the triad and its seventh".to_string(),
                    ),
                    (_, ChordSize::Ninths) => (
                        size.label().to_string(),
                        "Five notes: the seventh chord and its ninth".to_string(),
                    ),
                };
                ui.selectable_value(&mut self.size, size, label)
                    .on_hover_text(hover);
            }
        });
    }

    /// Presses or releases keys with the mouse. `pressed` is the key the
    /// mouse button is down on, if any.
    fn follow_mouse(
        &mut self,
        pressed: Option<usize>,
        plays: &[Play],
        actions: &mut Vec<HarmonyAction>,
    ) {
        let by_mouse = self
            .down
            .as_ref()
            .filter(|d| d.holder == Holder::Mouse)
            .map(|d| d.key);
        if by_mouse.is_some() && by_mouse != pressed {
            actions.extend(self.release());
        }
        if let Some(index) = pressed
            && self.down.is_none()
        {
            self.press(index, Holder::Mouse, plays, actions);
        }
    }

    /// Presses or releases keys with the number keys 1 to 7.
    fn follow_digits(
        &mut self,
        ui: &Ui,
        digits: bool,
        plays: &[Play],
        actions: &mut Vec<HarmonyAction>,
    ) {
        let by_digit = self
            .down
            .as_ref()
            .is_some_and(|d| matches!(d.holder, Holder::Digit(_)));
        // A key let go while the window is not listening never sends its
        // "up" here, so it is let go now.
        let focused = ui.input(|input| input.focused);
        if !digits || !focused {
            if by_digit {
                actions.extend(self.release());
            }
            return;
        }
        let events: Vec<(Key, bool)> = ui.input(|input| {
            input
                .events
                .iter()
                .filter_map(|event| match event {
                    egui::Event::Key {
                        key,
                        pressed,
                        repeat: false,
                        modifiers,
                        ..
                    } if DIGITS.contains(key) && (!*pressed || modifiers.is_none()) => {
                        Some((*key, *pressed))
                    }
                    _ => None,
                })
                .collect()
        });
        for (key, pressed) in events {
            let holder = Holder::Digit(key);
            let index = DIGITS.iter().position(|&digit| digit == key).unwrap_or(0);
            if pressed {
                actions.extend(self.release());
                self.press(index, holder, plays, actions);
            } else if self.down.as_ref().is_some_and(|d| d.holder == holder) {
                actions.extend(self.release());
            }
        }
    }

    fn press(
        &mut self,
        index: usize,
        holder: Holder,
        plays: &[Play],
        actions: &mut Vec<HarmonyAction>,
    ) {
        let Some(play) = plays.get(index) else {
            return;
        };
        let notes = play.notes.clone();
        if let Some(chord) = play.chord {
            self.last_voicing = Some(play.voicing.clone());
            self.last_chord = Some(chord);
        }
        actions.push(HarmonyAction::NoteOn(notes.clone()));
        self.down = Some(Down {
            key: index,
            holder,
            notes,
        });
    }
}

/// The line of text under the keys: what often comes after the last
/// chord, at the chosen level of detail. `suggests` says whether the key
/// is one that gets suggestions (major or minor), and `outside` whether
/// the last chord is not one of the key's seven.
fn hint_line(
    level: HintLevel,
    scale: Scale,
    last_chord: Option<Chord>,
    suggestions: &[Suggestion],
    suggests: bool,
    outside: bool,
    channel_name: &str,
) -> String {
    let spelling = scale.spelling();
    match (last_chord, suggestions.first()) {
        (Some(chord), Some(next)) => {
            let (from, to) = (chord.name(spelling), next.chord.name(spelling));
            match level {
                HintLevel::Detailed => {
                    format!("After {from}, {to} often comes next: {}", next.reason)
                }
                HintLevel::Short => format!("{from} → {to}: {}", next.short),
                HintLevel::Minimal => format!("{from} → {to}"),
            }
        }
        (Some(chord), None) if outside => match level {
            HintLevel::Detailed => format!(
                "{} is not one of the seven chords of {}: it comes from outside the key.",
                chord.name(spelling),
                scale.name()
            ),
            _ => format!("{}: outside the key", chord.name(spelling)),
        },
        _ if !suggests => match level {
            HintLevel::Detailed => {
                "Suggestions of what comes next are given in major and minor keys.".to_string()
            }
            HintLevel::Short => "Suggestions in major and minor keys only.".to_string(),
            HintLevel::Minimal => String::new(),
        },
        _ => match level {
            HintLevel::Detailed => format!(
                "Hold a key (or 1 to 7) to play its chord on \"{channel_name}\". With Rec on, \
                 the chords are recorded. Outlined keys are chords that often come next, \
                 the brightest the most usual."
            ),
            HintLevel::Short => {
                format!("Hold a key or press 1 to 7 to play on \"{channel_name}\".")
            }
            HintLevel::Minimal => String::new(),
        },
    }
}

/// One harmony key: the numeral, the chord's name and its function, on
/// the function's colour. Bright while it plays; outlined when it is a
/// chord that often comes next: thick and bright for the most usual one
/// (`rank` 0), thin and dim for the others.
fn chord_key(
    ui: &mut Ui,
    key: &ChordKey,
    spelling: Spelling,
    lit: bool,
    rank: Option<usize>,
) -> Response {
    let (rect, response) = ui.allocate_exact_size(KEY_SIZE, Sense::click());
    ui.add_space(KEY_GAP - ui.spacing().item_spacing.x);
    if !ui.is_rect_visible(rect) {
        return response;
    }
    let color = function_color(key.function);
    let fill = if lit {
        color
    } else if response.hovered() {
        color.gamma_multiply(0.7)
    } else {
        color.gamma_multiply(0.45)
    };
    let painter = ui.painter();
    // A pad: a dark edge at the bottom, the colour, and a lighter band
    // across the top.
    painter.rect_filled(rect, 6.0, fill.gamma_multiply(0.55));
    let body = Rect::from_min_max(rect.min, rect.max - vec2(0.0, 3.0));
    painter.rect_filled(body, 6.0, fill);
    let band = Rect::from_min_size(body.min, vec2(body.width(), 30.0));
    painter.rect_filled(
        band,
        egui::CornerRadius {
            nw: 6,
            ne: 6,
            sw: 0,
            se: 0,
        },
        Color32::from_white_alpha(if lit { 40 } else { 18 }),
    );
    match rank {
        Some(0) => {
            painter.rect_stroke(rect, 6.0, Stroke::new(3.0, SUGGESTED), StrokeKind::Inside);
        }
        Some(_) => {
            let dim = SUGGESTED.gamma_multiply(0.45);
            painter.rect_stroke(rect, 6.0, Stroke::new(1.5, dim), StrokeKind::Inside);
        }
        None => {}
    }
    painter.text(
        rect.center_top() + vec2(0.0, 18.0),
        Align2::CENTER_CENTER,
        &key.numeral,
        FontId::proportional(20.0),
        KEY_TEXT,
    );
    painter.text(
        rect.center() + vec2(0.0, 9.0),
        Align2::CENTER_CENTER,
        key.chord.name(spelling),
        FontId::proportional(13.0),
        KEY_TEXT,
    );
    painter.text(
        rect.center_bottom() - vec2(0.0, 9.0),
        Align2::CENTER_CENTER,
        key.function.name(),
        FontId::proportional(10.0),
        KEY_TEXT_DIM,
    );
    response
}

/// A harmony key of a row or a set: which notes of the row it plays (or
/// the note its chord starts on), the notes, and their set class.
fn group_key(ui: &mut Ui, group: &GroupKey, spelling: Spelling, lit: bool) -> Response {
    let names: Vec<&str> = group.notes.iter().map(|p| p.name(spelling)).collect();
    // Hexachords need a wider key for their six names.
    let width = (names.len() as f32 * 15.0 + 12.0).max(KEY_SIZE.x);
    let (rect, response) = ui.allocate_exact_size(vec2(width, KEY_SIZE.y), Sense::click());
    ui.add_space(KEY_GAP - ui.spacing().item_spacing.x);
    if !ui.is_rect_visible(rect) {
        return response;
    }
    let fill = if lit {
        GROUP_COLOR
    } else if response.hovered() {
        GROUP_COLOR.gamma_multiply(0.7)
    } else {
        GROUP_COLOR.gamma_multiply(0.45)
    };
    let painter = ui.painter();
    painter.rect_filled(rect, 6.0, fill.gamma_multiply(0.55));
    let body = Rect::from_min_max(rect.min, rect.max - vec2(0.0, 3.0));
    painter.rect_filled(body, 6.0, fill);
    painter.text(
        rect.center_top() + vec2(0.0, 16.0),
        Align2::CENTER_CENTER,
        &group.label,
        FontId::proportional(17.0),
        KEY_TEXT,
    );
    painter.text(
        rect.center() + vec2(0.0, 9.0),
        Align2::CENTER_CENTER,
        names.join(" "),
        FontId::proportional(12.0),
        KEY_TEXT,
    );
    painter.text(
        rect.center_bottom() - vec2(0.0, 9.0),
        Align2::CENTER_CENTER,
        group.pitch_set().set_class().name(),
        FontId::proportional(10.0),
        KEY_TEXT_DIM,
    );
    response
}

/// The tooltip of a key of a row or a set.
fn describe_group(group: &GroupKey, spelling: Spelling, index: usize, level: HintLevel) -> String {
    let names: Vec<&str> = group.notes.iter().map(|p| p.name(spelling)).collect();
    let class = group.pitch_set().set_class();
    let mut text = format!(
        "{}: {} · set {}",
        group.label,
        names.join(" "),
        class.name()
    );
    if let Some(nickname) = class.nickname() {
        text += &format!(", {nickname}");
    }
    match level {
        HintLevel::Detailed => format!(
            "{text}.\nPrime form {}, interval vector {}. Hold to play; key {} on the computer \
             keyboard too.",
            class.prime_text(),
            class.vector_text(),
            index + 1
        ),
        HintLevel::Short => format!("{text}. Key {}.", index + 1),
        HintLevel::Minimal => text,
    }
}

/// The tooltip of a harmony key, at `level`: "V: G major (G B D). The
/// dominant: tension, pulls back home to I.", "V: G major (G B D),
/// dominant: tension. Key 5." or just "V: G major (G B D)".
fn describe_key(key: &ChordKey, spelling: Spelling, level: HintLevel) -> String {
    let notes: Vec<&str> = key
        .chord
        .pitch_set()
        .iter()
        .map(|pitch_class| pitch_class.name(spelling))
        .collect();
    let digit = key.degree + 1;
    let chord = format!(
        "{}: {} {} ({})",
        key.numeral,
        key.chord.root.name(spelling),
        key.chord.quality.name(),
        notes.join(" "),
    );
    match level {
        HintLevel::Detailed => format!(
            "{chord}. The {}: {}.\nHold to play; key {digit} on the computer keyboard too.",
            key.function.name(),
            key.function.feel(),
        ),
        HintLevel::Short => format!(
            "{chord}, {}: {}. Key {digit}.",
            key.function.name(),
            key.function.short_feel(),
        ),
        HintLevel::Minimal => chord,
    }
}

/// What a time signature means, for its tooltip.
fn describe_time(time: TimeSignature) -> String {
    let beats = time.beats;
    match time.unit {
        4 => format!("{beats} quarter-note beats to the bar"),
        8 if time.steps_per_beat() == 6 => format!(
            "{beats} eighth notes to the bar, felt as {} beats of three",
            beats / 3
        ),
        8 if beats == 3 => "3 eighth notes to the bar, felt as one quick beat".to_string(),
        8 => format!("{beats} eighth notes to the bar, in groups of two and three"),
        _ => format!("{beats} beats of 1/{} to the bar", time.unit),
    }
}

/// A small keyboard: dots on the notes of the scale (orange on its root),
/// the notes held down lit, and the chord playing in the music shaded.
fn piano(ui: &mut Ui, scale: Scale, held: &[Note], heard: &[Note]) {
    let whites = 7 * usize::from(PIANO_OCTAVES);
    let size = vec2(whites as f32 * WHITE_WIDTH, PIANO_HEIGHT);
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    if !ui.is_rect_visible(rect) {
        return;
    }
    let painter = ui.painter_at(rect.expand(1.0));
    let notes = PIANO_LOWEST..PIANO_LOWEST + 12 * PIANO_OCTAVES;
    let is_held = |midi: u8| held.iter().any(|note| note.midi() == midi);
    let is_heard = |midi: u8| heard.iter().any(|note| note.midi() == midi);
    let fill = |midi: u8, normal: Color32| {
        if is_held(midi) {
            HELD
        } else if is_heard(midi) {
            HEARD
        } else {
            normal
        }
    };

    // White keys first, then the black keys on top of them.
    let mut white = 0;
    let mut black_keys = Vec::new();
    for midi in notes {
        let pitch_class = Note::new(midi).pitch_class();
        if pitch_class.is_black_key() {
            let x = rect.left() + white as f32 * WHITE_WIDTH;
            let key = Rect::from_center_size(
                pos2(x, rect.top() + PIANO_HEIGHT * 0.31),
                vec2(WHITE_WIDTH * 0.62, PIANO_HEIGHT * 0.62),
            );
            black_keys.push((midi, key));
            continue;
        }
        let key = Rect::from_min_size(
            pos2(rect.left() + white as f32 * WHITE_WIDTH, rect.top()),
            vec2(WHITE_WIDTH, PIANO_HEIGHT),
        );
        painter.rect_filled(key.shrink(0.5), 1.5, fill(midi, WHITE_KEY));
        painter.rect_stroke(key, 1.5, Stroke::new(1.0, KEY_BORDER), StrokeKind::Inside);
        scale_dot(&painter, scale, pitch_class, key, PIANO_HEIGHT * 0.84);
        white += 1;
    }
    for (midi, key) in black_keys {
        painter.rect_filled(key, 1.5, fill(midi, BLACK_KEY));
        let pitch_class = Note::new(midi).pitch_class();
        scale_dot(&painter, scale, pitch_class, key, PIANO_HEIGHT * 0.5);
    }
    // The name of each C, so you can find your place.
    for octave in 0..PIANO_OCTAVES {
        let x = rect.left() + (f32::from(octave) * 7.0 + 0.5) * WHITE_WIDTH;
        let c = Note::new(PIANO_LOWEST + 12 * octave);
        painter.text(
            pos2(x, rect.bottom() - 4.0),
            Align2::CENTER_BOTTOM,
            c.to_string(),
            FontId::proportional(7.5),
            BLACK_KEY,
        );
    }
}

/// A dot on a key of the scale, `y` below the top of the keyboard.
fn scale_dot(painter: &Painter, scale: Scale, pitch_class: PitchClass, key: Rect, y: f32) {
    if !scale.contains(pitch_class) {
        return;
    }
    let color = if pitch_class == scale.root {
        ROOT_DOT
    } else {
        SCALE_DOT
    };
    painter.circle_filled(pos2(key.center().x, key.top() + y - 6.0), 2.5, color);
}

#[cfg(test)]
mod tests {
    use super::*;
    use loom_core::ChordQuality;

    #[test]
    fn the_hint_line_gets_shorter_by_level() {
        let c = Scale::new(PitchClass::C, ScaleKind::Major);
        let f = Chord::new(PitchClass::F, ChordQuality::Major);
        let next = next_chords(c, f);
        let line = |level| hint_line(level, c, Some(f), &next, true, false, "Piano");
        assert_eq!(
            line(HintLevel::Detailed),
            "After F, G often comes next: IV to V: a step up to the dominant, ready to come home."
        );
        assert_eq!(line(HintLevel::Short), "F → G: step up to the dominant");
        assert_eq!(line(HintLevel::Minimal), "F → G");
    }
}
