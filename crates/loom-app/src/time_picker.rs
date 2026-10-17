//! Choosing a time signature: one click for the usual ones, or any top
//! and bottom number typed in, with the beats grouped as you like for
//! irregular meters such as 2+2+3/8.
//!
//! The same picker is used on the playlist's bar numbers and in the
//! harmony keys, so both offer the same time signatures.

use eframe::egui::{self, Id, Ui};
use loom_core::TimeSignature;

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
/// one was, and closes the menu it is in.
pub fn time_picker(ui: &mut Ui, id: Id, current: TimeSignature) -> Option<TimeSignature> {
    let mut chosen = None;
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
