//! Choosing a time signature: one click for the usual ones, or any top
//! and bottom number typed in, with the beats grouped as you like for
//! irregular meters such as 2+2+3/8.
//!
//! The same picker is used on the playlist's bar numbers and in the
//! harmony keys, so both offer the same time signatures.

use eframe::egui::{self, Id, Ui};
use lineup_core::TimeSignature;
use lineup_core::library::apply::StyleParts;

use crate::theme;

/// What has been typed so far. It lives in egui's memory between frames
/// (a menu is drawn again every frame), and starts again from the
/// current time signature when that changes.
#[derive(Clone)]
struct Draft {
    /// The time signature the draft started from.
    from: TimeSignature,
    beats: u8,
    unit: u8,
    /// The grouping as typed, like "2+2+3". Empty for the usual beats.
    groups: String,
}

impl Draft {
    fn new(time: TimeSignature) -> Self {
        Draft {
            from: time,
            beats: time.beats,
            unit: time.unit,
            groups: if time.is_grouped() {
                let groups: Vec<String> = time.groups().iter().map(u8::to_string).collect();
                groups.join("+")
            } else {
                String::new()
            },
        }
    }

    /// The time signature it makes, or what is wrong with it.
    fn time(&self) -> Result<TimeSignature, String> {
        let groups = self.groups.trim();
        if groups.is_empty() {
            return format!("{}/{}", self.beats, self.unit).parse();
        }
        let time: TimeSignature = format!("{groups}/{}", self.unit).parse()?;
        if time.beats == self.beats {
            Ok(time)
        } else {
            Err(format!(
                "the groups add up to {}, not {}",
                time.beats, self.beats
            ))
        }
    }
}

/// The picker: the usual time signatures as buttons, then a row to type
/// any other. `current` is shown selected. Returns the one chosen, if
/// one was, and closes the menu it is in. Whole-song meter changes get
/// the shared style options; later time markers keep the current style.
pub fn time_picker(
    ui: &mut Ui,
    id: Id,
    current: TimeSignature,
    parts: Option<&mut StyleParts>,
) -> Option<TimeSignature> {
    let mut chosen = None;
    if let Some(parts) = parts {
        ui.checkbox(&mut parts.matching_style, "Use matching style")
            .on_hover_text(
                "Turn off to mark the new meter's accents with your current instruments, \
                 even when the library has a matching style.",
            );
        ui.checkbox(&mut parts.rhythm, "Apply rhythm patterns / accents")
            .on_hover_text(
                "Turn off to change the meter while keeping the current playing patterns",
            );
        ui.add_enabled_ui(parts.matching_style, |ui| {
            ui.checkbox(&mut parts.instruments, "Use style instruments")
                .on_hover_text("Turn off to keep the current voices and instruments");
        });
        ui.checkbox(&mut parts.swing, "Use rhythm's swing")
            .on_hover_text("Turn off to keep the current swing amount");
        ui.separator();
    }
    ui.horizontal_wrapped(|ui| {
        ui.set_max_width(220.0);
        for time in TimeSignature::ALL {
            if ui
                .selectable_label(current == time, time.to_string())
                .on_hover_text(describe_time(time))
                .clicked()
            {
                chosen = Some(time);
            }
        }
    });

    ui.separator();
    ui.label("Any other").on_hover_text(
        "The top number is how many notes make a bar, the bottom one which note: 4 for \
         quarter notes, 8 for eighths, 16 for sixteenths",
    );
    let mut draft = ui.data_mut(|data| data.get_temp::<Draft>(id));
    let draft = match &mut draft {
        Some(draft) if draft.from == current => draft,
        _ => draft.insert(Draft::new(current)),
    };
    ui.horizontal(|ui| {
        let max = TimeSignature::max_beats(draft.unit).max(1);
        ui.add(egui::DragValue::new(&mut draft.beats).range(1..=max))
            .on_hover_text("The top number: how many notes make a bar");
        ui.label("/");
        // The bottom numbers as buttons, not a drop-down list: a list
        // inside another would close it.
        for unit in TimeSignature::UNITS {
            ui.selectable_value(&mut draft.unit, unit, unit.to_string())
                .on_hover_text(format!(
                    "The bottom number: {}. Thirty-second notes would not fit the grid of \
                     sixteenth-note steps",
                    unit_name(unit).to_lowercase()
                ));
        }
        // Changing the bottom number can leave too many notes for a
        // pattern: 64/4 fits, 64/2 would not.
        draft.beats = draft.beats.min(TimeSignature::max_beats(draft.unit).max(1));
    });
    ui.horizontal(|ui| {
        ui.label("Beats grouped as");
        ui.add(
            egui::TextEdit::singleline(&mut draft.groups)
                .desired_width(70.0)
                .hint_text(match draft.beats {
                    5 => "3+2",
                    7 => "2+2+3",
                    8 => "3+3+2",
                    9 => "2+2+2+3",
                    11 => "3+3+2+3",
                    _ => "e.g. 2+3",
                }),
        )
        .on_hover_text(
            "Irregular meters group their notes in twos and threes: 7/8 is often counted \
             2+2+3, as in many Balkan dances. Leave it empty for the usual beats",
        );
    });
    match draft.time() {
        Ok(time) => {
            if ui
                .button(format!("Use {time}"))
                .on_hover_text(describe_time(time))
                .clicked()
            {
                chosen = Some(time);
            }
        }
        Err(problem) => {
            ui.colored_label(theme::RED, problem);
        }
    }
    let draft = draft.clone();
    ui.data_mut(|data| data.insert_temp(id, draft));

    if chosen.is_some() {
        ui.close();
    }
    chosen
}

/// The note the bottom number stands for.
fn unit_name(unit: u8) -> &'static str {
    match unit {
        1 => "Whole notes",
        2 => "Half notes",
        4 => "Quarter notes",
        8 => "Eighth notes",
        _ => "Sixteenth notes",
    }
}

/// What a time signature means, for its tooltip.
pub fn describe_time(time: TimeSignature) -> String {
    let beats = time.beats;
    let notes = unit_name(time.unit).to_lowercase();
    if time.is_grouped() {
        let groups: Vec<String> = time.groups().iter().map(u8::to_string).collect();
        return format!(
            "{beats} {notes} to the bar, felt as {} beats: {}",
            groups.len(),
            groups.join(" + ")
        );
    }
    match (time.unit, time.usual_groups()) {
        (_, 3) => format!(
            "{beats} {notes} to the bar, felt as {} beats of three",
            beats / 3
        ),
        (4, _) => format!("{beats} quarter-note beats to the bar"),
        (8 | 16, _) if beats == 3 => format!("3 {notes} to the bar, felt as one quick beat"),
        (8 | 16, _) if beats == 1 || beats == 2 => format!("{beats} {notes} to the bar"),
        (8 | 16, _) => format!(
            "{beats} {notes} to the bar, a line on each: give it a grouping, such as \
             2+2+3, to see its beats"
        ),
        _ => format!("{beats} {notes} to the bar"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::egui::{
        Event, FullOutput, Modifiers, PointerButton, Pos2, RawInput, Rect, Shape, vec2,
    };
    use lineup_core::library::{Rhythm, Style};
    use lineup_core::{BandStyle, Player, Project};

    fn frame(
        ctx: &egui::Context,
        parts: &mut StyleParts,
        events: Vec<Event>,
    ) -> (FullOutput, Option<TimeSignature>) {
        let mut chosen = None;
        let mut output = ctx.run_pass(
            RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(480.0, 640.0))),
                events,
                time: Some(ctx.cumulative_frame_nr() as f64 * 0.1),
                ..Default::default()
            },
            |ctx| {
                egui::Window::new("Time signature")
                    .fixed_pos(Pos2::ZERO)
                    .default_size(vec2(440.0, 580.0))
                    .show(ctx, |ui| {
                        chosen = time_picker(
                            ui,
                            Id::unique("test-time-picker"),
                            TimeSignature::COMMON,
                            Some(parts),
                        );
                    });
            },
        );
        output.textures_delta.clear();
        (output, chosen)
    }

    fn label_rect(output: &FullOutput, label: &str) -> Rect {
        output
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                Shape::Text(text) if text.galley.text() == label => {
                    Some(Rect::from_min_size(text.pos, text.galley.size()))
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("{label:?} is missing from the time-signature editor"))
    }

    fn click(
        ctx: &egui::Context,
        parts: &mut StyleParts,
        output: &FullOutput,
        label: &str,
    ) -> (FullOutput, Option<TimeSignature>) {
        let pos = label_rect(output, label).center();
        frame(
            ctx,
            parts,
            vec![
                Event::PointerMoved(pos),
                Event::PointerButton {
                    pos,
                    button: PointerButton::Primary,
                    pressed: true,
                    modifiers: Modifiers::NONE,
                },
            ],
        );
        frame(
            ctx,
            parts,
            vec![Event::PointerButton {
                pos,
                button: PointerButton::Primary,
                pressed: false,
                modifiers: Modifiers::NONE,
            }],
        )
    }

    #[test]
    fn the_time_picker_can_choose_an_odd_meter_with_current_instruments() {
        let ctx = egui::Context::default();
        ctx.all_styles_mut(|style| style.animation_time = 0.0);
        let mut parts = StyleParts::default();
        frame(&ctx, &mut parts, vec![]);
        let (output, _) = frame(&ctx, &mut parts, vec![]);
        assert!(label_rect(&output, "Use matching style").top() < label_rect(&output, "7/8").top());
        let (output, _) = click(&ctx, &mut parts, &output, "Use matching style");
        assert!(!parts.matching_style);
        let (_, chosen) = click(&ctx, &mut parts, &output, "7/8");
        let time = chosen.expect("clicking the meter should apply it");
        assert_eq!(time, TimeSignature::new(7, 8));

        let mut project = Project::with_band();
        project.write_progression(Style::named("Four chords").unwrap(), 0);
        project.follow_chord_sheet();
        let piano = project.player_channel(Player::Piano).unwrap();
        project.channel_mut(piano).unwrap().program = Some(80);
        let instruments = project.band().unwrap().instrument_style;
        let rhythm = Rhythm::for_time(time, project.band().unwrap().style.rhythm);
        let parts = StyleParts {
            time_signature: true,
            ..parts.for_rhythm()
        };
        project.apply_band_parts(BandStyle::named(rhythm.name).unwrap(), None, parts);
        assert!(project.band().unwrap().accents_only);
        assert_eq!(project.band().unwrap().instrument_style, instruments);
        assert_eq!(project.channels()[piano].program, Some(80));
    }

    #[test]
    fn the_time_picker_can_keep_instruments_with_a_matching_style() {
        let ctx = egui::Context::default();
        ctx.all_styles_mut(|style| style.animation_time = 0.0);
        let mut parts = StyleParts::default();
        frame(&ctx, &mut parts, vec![]);
        let (output, _) = frame(&ctx, &mut parts, vec![]);
        click(&ctx, &mut parts, &output, "Use style instruments");
        assert!(parts.matching_style);
        assert!(!parts.instruments);
        assert!(parts.rhythm);
    }
}
