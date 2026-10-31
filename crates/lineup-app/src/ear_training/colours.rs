//! The exercises beyond keys: what to ask, and the answers.
//!
//! - **Micromodes**: answered by family (three buttons with Kröpfl's
//!   names, Menor, Mayor and Acorde) or by micromode (one button for
//!   each chosen, named as in the key menu: Menor 3, Mayor 2, m/M).
//! - **Forms**: two buttons, moved (Tn) or upside down (TnI).
//! - **Rows**: four, P, I, R and RI.
//!
//! As in the other exercises, after an answer every button plays its
//! own sound, next to the question's: the micromode clicked on the same
//! lowest note, the figure moved or upside down, the row's other form.

use eframe::egui::{self, Ui};
use lineup_core::ear_training::{
    FORM_SIZES, FormQuestion, MicromodeAnswer, MicromodeQuestion, RowQuestion, RowSource, Texture,
};
use lineup_core::harmony::{Micromode, MicromodeFamily, RowForm};

use super::{EarAction, EarTraining, Mark, MicromodeGiven, answer_button, feedback, feedback_with};
use crate::i18n::{tr, trc, trf};
use crate::theme;
use crate::twelve_tone::{family_word, micromode_short};

/// The words on a micromode's answer button: its name in its family
/// ("Menor 3") or its chord sign, and its prime form, "[014]".
fn micromode_button(micromode: &Micromode) -> (String, String) {
    let short = match micromode.family {
        MicromodeFamily::Chord => micromode_short(micromode).to_string(),
        family => format!("{} {}", family_word(family), micromode_short(micromode)),
    };
    (short, micromode.set_class().prime_text())
}

impl EarTraining {
    pub(super) fn micromode_settings(&mut self, ui: &mut Ui) {
        let settings = &mut self.settings.micromodes;
        ui.horizontal_wrapped(|ui| {
            ui.label(tr("Micromodes"));
            for family in MicromodeFamily::ALL {
                ui.separator();
                ui.label(family_word(family));
                for (place, micromode) in Micromode::all().iter().enumerate() {
                    if micromode.family != family {
                        continue;
                    }
                    let on = settings.micromodes.has(place);
                    let chip = ui.add(egui::Button::selectable(on, micromode_short(micromode)));
                    let hint = format!("{}: {}", micromode.full_name(), micromode.intervals());
                    if theme::hint(chip, hint).clicked() {
                        settings.micromodes.toggle(place);
                    }
                }
            }
        });
        ui.horizontal_wrapped(|ui| {
            ui.label(tr("Answer"));
            for answer in MicromodeAnswer::ALL {
                let chip = ui.selectable_value(&mut settings.answer, answer, tr(answer.name()));
                theme::hint(
                    chip,
                    tr(match answer {
                        MicromodeAnswer::Family => {
                            "Minor (with a semitone), major (a whole tone, no semitone) or a \
                             chord (two thirds or more)"
                        }
                        MicromodeAnswer::Micromode => "Which of the twelve",
                    }),
                );
            }
            ui.separator();
            ui.label(tr("Played"));
            for (place, direction) in super::Direction::ALL.into_iter().enumerate() {
                let on = settings.directions.has(place);
                let text = match direction {
                    super::Direction::Up => "the lowest note first, going up",
                    super::Direction::Down => "the highest note first, going down",
                    super::Direction::Together => "the three notes at once",
                };
                let chip = ui.add(egui::Button::selectable(on, tr(direction.name())));
                if theme::hint(chip, tr(text)).clicked() {
                    settings.directions.toggle(place);
                }
            }
            let spread = ui.checkbox(&mut settings.spread, tr("Spread out"));
            theme::hint(
                spread,
                tr(
                    "The three notes in any order from the bottom, up to two octaves wide: C \
                    C# E can sound as E C C#",
                ),
            );
        });
    }

    pub(super) fn form_settings(&mut self, ui: &mut Ui) {
        let settings = &mut self.settings.forms;
        ui.horizontal_wrapped(|ui| {
            ui.label(tr("Notes"));
            for (place, size) in FORM_SIZES.into_iter().enumerate() {
                let on = settings.sizes.has(place);
                let chip = ui.add(egui::Button::selectable(on, size.to_string()));
                let hint = tr(match size {
                    3 => "Sets of three notes: the seven micromodes that change upside down",
                    4 => "Sets of four notes, such as the dominant seventh chord",
                    _ => "Sets of five notes",
                });
                if theme::hint(chip, hint).clicked() {
                    settings.sizes.toggle(place);
                }
            }
            ui.separator();
            ui.label(tr("Played"));
            for (place, texture) in Texture::ALL.into_iter().enumerate() {
                let on = settings.textures.has(place);
                let chip = ui.add(egui::Button::selectable(on, tr(texture.name())));
                let hint = tr(match texture {
                    Texture::Line => "One note after another: a figure",
                    Texture::Chord => "All the notes at once",
                });
                if theme::hint(chip, hint).clicked() {
                    settings.textures.toggle(place);
                }
            }
        });
    }

    pub(super) fn row_settings(&mut self, ui: &mut Ui) {
        let settings = &mut self.settings.rows;
        ui.horizontal_wrapped(|ui| {
            ui.label(tr("Forms"));
            for (place, form) in RowForm::ALL.into_iter().enumerate() {
                let on = settings.forms.has(place);
                let chip = ui.add(egui::Button::selectable(on, form.letters()));
                if theme::hint(chip, tr(form.name())).clicked() {
                    settings.forms.toggle(place);
                }
            }
            ui.separator();
            ui.label(tr("Rows"));
            for source in RowSource::ALL {
                let chip = ui.selectable_value(&mut settings.source, source, tr(source.name()));
                theme::hint(
                    chip,
                    tr(match source {
                        RowSource::Pieces => {
                            "The rows of the Row window: Schoenberg's, Berg's, Webern's and \
                             more"
                        }
                        RowSource::Random => "The twelve notes in any order",
                    }),
                );
            }
        });
    }

    pub(super) fn micromode_answers(
        &mut self,
        ui: &mut Ui,
        question: &MicromodeQuestion,
        answer: &mut Option<MicromodeGiven>,
        actions: &mut Vec<EarAction>,
    ) {
        let by_family = self.settings.micromodes.answer == MicromodeAnswer::Family;
        let chosen = self.settings.micromodes.micromodes;
        let right = question.micromode;
        ui.horizontal_wrapped(|ui| {
            if by_family {
                for family in MicromodeFamily::ALL {
                    let given = MicromodeGiven::Family(family);
                    let is_right = question.is_family(family);
                    let mark = Mark::of(*answer == Some(given), is_right, answer.is_some());
                    let button = answer_button(ui, family_word(family), tr(family.name()), mark);
                    if button.clicked() && answer.is_none() {
                        *answer = Some(given);
                        self.score().add(is_right);
                    }
                }
            } else {
                for (place, micromode) in Micromode::all().iter().enumerate() {
                    if !chosen.has(place) && micromode != right {
                        continue;
                    }
                    let given = MicromodeGiven::Number(micromode.number);
                    let is_right = micromode == right;
                    let mark = Mark::of(*answer == Some(given), is_right, answer.is_some());
                    let (short, name) = micromode_button(micromode);
                    let button = answer_button(ui, &short, &name, mark);
                    if theme::hint(button, micromode.full_name()).clicked() {
                        if answer.is_none() {
                            *answer = Some(given);
                            self.score().add(is_right);
                        } else {
                            self.play(question.cues_as(micromode), None, actions);
                        }
                    }
                }
            }
        });
        if let Some(given) = *answer {
            let (is_right, said) = match given {
                MicromodeGiven::Family(family) => {
                    (question.is_family(family), tr(family.name()).to_string())
                }
                MicromodeGiven::Number(number) => (
                    number == right.number,
                    Micromode::number(number).map_or_else(String::new, Micromode::full_name),
                ),
            };
            let wrong = (!is_right).then(|| trf("You said {}.", &[&said]));
            feedback_with(ui, is_right, wrong, &question.explain(), !by_family);
        } else {
            ui.add_space(4.0);
            ui.weak(tr(
                "Menor: with a semitone. Mayor: a whole tone and no semitone. Acorde: two \
                 thirds or more.",
            ));
        }
    }

    pub(super) fn form_answers(
        &mut self,
        ui: &mut Ui,
        question: &FormQuestion,
        answer: &mut Option<bool>,
        actions: &mut Vec<EarAction>,
    ) {
        ui.horizontal_wrapped(|ui| {
            for upside_down in [false, true] {
                let (short, name) = match upside_down {
                    false => (tr("Moved"), "Tn"),
                    true => (trc("form", "Upside down"), "TnI"),
                };
                let is_right = upside_down == question.upside_down;
                let mark = Mark::of(*answer == Some(upside_down), is_right, answer.is_some());
                if answer_button(ui, short, name, mark).clicked() {
                    if answer.is_none() {
                        *answer = Some(upside_down);
                        self.score().add(is_right);
                    } else {
                        self.play(question.cues_as(upside_down), None, actions);
                    }
                }
            }
        });
        if let Some(given) = *answer {
            let is_right = given == question.upside_down;
            let said = tr(if given { "upside down" } else { "moved" });
            let wrong = (!is_right).then(|| trf("You said {}.", &[&said]));
            feedback(ui, is_right, wrong, &question.explain());
        } else {
            ui.add_space(4.0);
            ui.weak(tr(
                "Moved, every step stays the same; upside down, every step goes the other way.",
            ));
        }
    }

    pub(super) fn row_answers(
        &mut self,
        ui: &mut Ui,
        question: &RowQuestion,
        answer: &mut Option<RowForm>,
        actions: &mut Vec<EarAction>,
    ) {
        let chosen = self.settings.rows.forms;
        ui.horizontal_wrapped(|ui| {
            for (place, form) in RowForm::ALL.into_iter().enumerate() {
                if !chosen.has(place) && form != question.form {
                    continue;
                }
                let is_right = question.is_right(form);
                let mark = Mark::of(*answer == Some(form), is_right, answer.is_some());
                if answer_button(ui, form.letters(), tr(form.name()), mark).clicked() {
                    if answer.is_none() {
                        *answer = Some(form);
                        self.score().add(is_right);
                    } else {
                        self.play(question.cues_as(form), None, actions);
                    }
                }
            }
        });
        if let Some(given) = *answer {
            let is_right = question.is_right(given);
            let wrong = (!is_right).then(|| trf("You said {}.", &[&given.letters()]));
            feedback(ui, is_right, wrong, &question.explain());
        } else {
            ui.add_space(4.0);
            ui.weak(tr(
                "P: the same shape, moved. I: every step the other way. R: backwards. RI: \
                 upside down and backwards.",
            ));
        }
    }
}
