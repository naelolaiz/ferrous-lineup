//! The band's players and sounds, independent of its playing style.

use eframe::egui::{self, Ui};
use lineup_core::{BandStyle, ChannelKind, Player, Project, general_midi};

use super::{SheetAction, horns_items, soloist_items};
use crate::i18n::{tr, tr_name};
use crate::{theme, widgets};

/// Instrument lineups drawn from the existing arrangements. Only their sounds
/// and players are copied; their rhythm and timing are never applied here.
const LINEUPS: [(&str, &str); 8] = [
    ("Grand piano + acoustic bass", "Chorale"),
    ("Electric piano + synth bass", "Boom bap"),
    ("Piano, acoustic guitar + strings", "Pop beat"),
    ("Piano, electric guitar + organ", "Rock ride"),
    ("Piano + strings", "Jazz ballad"),
    ("Piano + nylon guitar", "Bossa nova"),
    ("Accordion + acoustic bass", "Rachenitsa"),
    ("Horns, timpani + strings", "Orchestra"),
];

pub(super) fn menu(ui: &mut Ui, project: &Project, actions: &mut Vec<SheetAction>) {
    let band_label = format!("🎷 {}", crate::i18n::tr("Band"));
    let menu = egui::containers::menu::MenuButton::new(band_label).config(
        egui::containers::menu::MenuConfig::new()
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside),
    );
    let (response, _) = menu.ui(ui, |ui| {
        ui.set_width(320.0);
        widgets::scroll_menu(ui, |ui| {
            ui.strong(tr("Voices and players"));
            ui.weak(tr("Choose who plays and their sounds. Playing patterns are chosen in the Library."));
            ui.separator();
            if let Some(band) = project.band() {
                widgets::scroll_menu_button(ui, tr("Instrument lineup"), |ui| {
                    ui.weak(tr("The current rhythm, chords and timing are kept."));
                    for (name, source) in LINEUPS {
                        if let Some(style) = BandStyle::named(source)
                            && ui.selectable_label(band.custom_instruments.is_none() && band.instrument_style == style, tr(name)).clicked()
                        {
                            actions.push(SheetAction::BandVoices(style));
                            ui.close();
                        }
                    }
                });
                ui.separator();
                for player in Player::ALL {
                    let Some(track) = project.player_track(player) else { continue };
                    ui.horizontal(|ui| {
                        let mut on = !project.playlist().tracks()[track].muted;
                        if ui.checkbox(&mut on, tr(player.name())).on_hover_text(tr("Include this player in the band")).changed() {
                            actions.push(SheetAction::BandPlayer { player, on });
                        }
                        if let Some(channel) = project.player_channel(player) {
                            voice_menu(ui, project, channel, actions);
                        }
                    });
                }
                ui.separator();
                horns_items(ui, band, actions);
                soloist_items(ui, band, actions);
                ui.separator();
                if ui.button(tr("No band")).clicked() {
                    actions.push(SheetAction::SetBand { style: None, harmony: None });
                    ui.close();
                }
            } else {
                ui.weak(tr("Apply a Library preset to add accompaniment. Existing song voices can be changed here."));
                for (index, channel) in project.channels().iter().enumerate() {
                    if matches!(channel.kind, ChannelKind::Synth | ChannelKind::MidiOut { .. }) {
                        ui.label(tr_name(&channel.name));
                        voice_menu(ui, project, index, actions);
                    }
                }
            }
        });
    });
    theme::hint(
        response,
        tr(
            "Choose the band’s voices and players. Rhythms and playing patterns are in the Library.",
        ),
    );
}

fn voice_menu(ui: &mut Ui, project: &Project, channel: usize, actions: &mut Vec<SheetAction>) {
    let current = project.channels()[channel].program;
    let label = current.map_or("Lineup Osc", |program| tr(general_midi::name(program)));
    widgets::scroll_menu_button(ui, label, |ui| {
        if ui
            .selectable_label(current.is_none(), tr("Lineup Osc"))
            .clicked()
        {
            actions.push(SheetAction::BandProgram {
                channel,
                program: None,
            });
            ui.close();
        }
        for (family, name) in general_midi::FAMILIES.iter().enumerate() {
            ui.menu_button(tr(name), |ui| {
                for program in (family * 8..family * 8 + 8).map(|index| index as u8) {
                    if ui
                        .selectable_label(current == Some(program), tr(general_midi::name(program)))
                        .clicked()
                    {
                        actions.push(SheetAction::BandProgram {
                            channel,
                            program: Some(program),
                        });
                        ui.close();
                    }
                }
            });
        }
    });
}
