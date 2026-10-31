//! The chord sheet's notation row: how the harmony is named (as written,
//! as chords, as pitch-class sets or as micromodes) and, for sets and
//! micromodes, which notes make each group: the sheet's own harmonies,
//! the notes sounding together, the notes one after another in a line,
//! or the forms of the key's set heard in both ways at once; and under
//! it, for sets and micromodes, the classes the sheet shows the most.
//! See `lineup_core::chord_sheet::notation`.

use std::rc::Rc;

use eframe::egui::{self, Color32, Ui};
use lineup_core::chord_sheet::notation::{
    ClassUse, Grouping, HeardGroups, Notation, Tally, heard_groups, heard_names, most_used,
    spelled_set, stacked_classes, stacked_notes, stacked_text, written_names,
};
use lineup_core::harmony::HeardNote;
use lineup_core::lead_sheet::default_melody_channel;
use lineup_core::{ChordMarker, HarmonyName, KeyMap, Note, Project, Scale};
use serde::{Deserialize, Serialize};

use crate::i18n::{tr, tr_name, trf};
use crate::theme;

fn choices_id() -> egui::Id {
    egui::Id::unique("chord-sheet-notation")
}

/// The most notes a group can be asked to hold: a whole twelve-tone row.
const MOST_NOTES: usize = 12;

/// What the row remembers from one run of the app to the next: the
/// sizes of the groups. The notation and the grouping are each song's
/// own, and a song starts in the notation of its key (see
/// [`Notation::Key`]): a song in C major is not shown in micromodes
/// because the song before was.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
struct Choices {
    /// How many different notes a group of notes sounding together
    /// holds, or `None` for all of them.
    together: Option<usize>,
    /// How many notes a group of a line holds.
    line: usize,
}

impl Default for Choices {
    fn default() -> Self {
        Choices {
            together: None,
            line: 3,
        }
    }
}

/// Whose notes are grouped.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Track {
    /// The tune for a line (the track called Lead, or else the highest
    /// one, as the lead sheet picks it), and every track for notes
    /// sounding together.
    Auto,
    All,
    Channel(usize),
}

/// How many classes the line of the most used shows by name; the rest
/// are in its hint.
const MOST_SHOWN: usize = 4;

/// The groups last read from the song's notes, as the bars show them,
/// and what they were read from, so they are only read again when the
/// notes, the keys or the choices change.
struct Read {
    from: (Notation, Grouping, Option<usize>, Option<usize>),
    notes: Vec<HeardNote>,
    keys: KeyMap,
    shown: Rc<[Shown]>,
    /// The classes of the groups, the most used first.
    tally: Rc<Tally>,
    /// The pass of the frame it was last checked in: the row and the
    /// bars both ask for it, and the song does not change in between.
    pass: u64,
}

/// A harmony as the sheet shows it, in the notation chosen: a harmony
/// written on the sheet, or a group of notes read from the song.
#[derive(Clone)]
pub(super) struct Shown {
    /// The step it starts on, counted from the start of the song.
    pub start: usize,
    /// What is drawn: "^3-11 T7I.", or "3-11 T1I|3-11 T7I".
    pub text: String,
    /// For sets and micromodes, the text with their classes alone,
    /// "3-11|3-11", where the whole one does not fit.
    pub short: Option<String>,
    pub caption: Option<String>,
    /// The colour of the caption, when it is not the usual one: the
    /// numeral of a borrowed or a secondary chord, or of a pivot.
    pub tint: Option<Color32>,
    /// It is a set, a micromode or a row form, written in their colour.
    pub group: bool,
    pub no_chord: bool,
    /// The notes to hear when it is held down with the mouse.
    pub notes: Vec<Note>,
    /// A sentence about it, for its hint.
    pub about: String,
}

/// The notation row and what it chose.
pub(super) struct NotationView {
    notation: Notation,
    grouping: Grouping,
    /// The key the song starts in, which says what the row offers in
    /// [`Notation::Key`].
    song_key: Scale,
    choices: Choices,
    /// What was last saved for the next run, or `None` before the
    /// choices were read back.
    saved: Option<Choices>,
    /// Whose notes are grouped.
    track: Track,
    read: Option<Read>,
}

impl NotationView {
    pub fn new() -> Self {
        NotationView {
            notation: Notation::Key,
            grouping: Grouping::Written,
            song_key: Scale::new(lineup_core::PitchClass::C, lineup_core::ScaleKind::Major),
            choices: Choices::default(),
            saved: None,
            track: Track::Auto,
            read: None,
        }
    }

    /// Reads back the choices of the last run, the first time; then
    /// saves them for the next when they change.
    pub fn remember(&mut self, ctx: &egui::Context) {
        match self.saved {
            None => {
                self.choices = ctx
                    .data_mut(|data| data.get_persisted(choices_id()))
                    .unwrap_or_default();
                self.saved = Some(self.choices);
            }
            Some(saved) if saved != self.choices => {
                let choices = self.choices;
                ctx.data_mut(|data| data.insert_persisted(choices_id(), choices));
                self.saved = Some(choices);
            }
            Some(_) => {}
        }
    }

    /// Another song: its tracks are not the last one's, and it is shown
    /// in the notation of its own key.
    pub fn new_song(&mut self) {
        self.notation = Notation::Key;
        self.grouping = Grouping::Written;
        self.track = Track::Auto;
        self.read = None;
    }

    /// The notation chosen: [`Notation::Key`] names each harmony in the
    /// key it is in.
    fn notation(&self) -> Notation {
        self.notation
    }

    /// The notation the song's own key gives, which says whether its
    /// notes can be read in groups, and how big.
    fn song_notation(&self) -> Notation {
        self.notation.in_key(self.song_key)
    }

    /// Whether the bars show groups read from the song's notes rather
    /// than the harmonies written on them.
    pub fn reads_notes(&self) -> bool {
        self.song_notation().groups() && self.grouping != Grouping::Written
    }

    /// How many notes a group holds, or `None` for all those sounding
    /// together.
    fn size(&self) -> Option<usize> {
        if let Some(size) = self.song_notation().fixed_size() {
            return Some(size);
        }
        match self.grouping {
            Grouping::InLine => Some(self.choices.line),
            _ => self.choices.together,
        }
    }

    /// Draws the row, with `after` at its end, and under it, for sets
    /// and micromodes, the classes the sheet shows the most.
    pub fn row(&mut self, ui: &mut Ui, project: &Project, after: impl FnOnce(&mut Ui)) {
        self.song_key = project.scale;
        ui.horizontal_wrapped(|ui| {
            self.items(ui, project);
            ui.separator();
            after(ui);
        });
        if let Some(tally) = self.tally(project, ui.ctx().cumulative_pass_nr())
            && !tally.classes.is_empty()
        {
            ui.horizontal_wrapped(|ui| most_used_line(ui, &tally, self.reads_notes()));
        }
    }

    /// The classes of sets or micromodes the sheet shows, the most used
    /// first, as it groups the notes; `None` where it names its harmony
    /// as chords.
    fn tally(&mut self, project: &Project, pass: u64) -> Option<Rc<Tally>> {
        if !self.song_notation().groups() {
            return None;
        }
        if self.reads_notes() {
            self.heard(project, &project.key_map(), pass);
            return self.read.as_ref().map(|read| read.tally.clone());
        }
        let (names, end) = written_names(project, self.notation());
        Some(Rc::new(most_used(&names, end)))
    }

    /// The row's own choices.
    fn items(&mut self, ui: &mut Ui, project: &Project) {
        ui.label(crate::i18n::tr("Show"));
        let notation = self.notation();
        let combo = egui::ComboBox::from_id_salt("chord-sheet-notation")
            .selected_text(crate::i18n::tr(notation.name()))
            .show_ui(ui, |ui| {
                for choice in Notation::ALL {
                    let item = ui.selectable_value(
                        &mut self.notation,
                        choice,
                        crate::i18n::tr(choice.name()),
                    );
                    theme::hint(item, tr(choice.describe()));
                }
            });
        theme::hint(
            combo.response,
            tr(
                "How the sheet names its harmony. The notes stay as they are: a set or a \
             micromode is another way to write the same chord",
            ),
        );
        if !self.song_notation().groups() {
            return;
        }
        ui.separator();
        let grouping = self.grouping;
        let combo = egui::ComboBox::from_id_salt("chord-sheet-grouping")
            .selected_text(tr(grouping.name()))
            .show_ui(ui, |ui| {
                for choice in Grouping::ALL {
                    let item = ui.selectable_value(&mut self.grouping, choice, tr(choice.name()));
                    theme::hint(item, tr(choice.describe()));
                }
            });
        theme::hint(
            combo.response,
            trf(
                "{}. Typing in a bar still writes its harmony",
                &[&tr(self.grouping.describe())],
            ),
        );
        if self.reads_notes() {
            match self.grouping.reads_forms() {
                true => self.forms_note(ui, project),
                false => self.size_choice(ui),
            }
            self.track_choice(ui, project);
        }
    }

    /// For the forms of the key: how big they are, or that there are
    /// none to read where no key of the song is a set.
    fn forms_note(&self, ui: &mut Ui, project: &Project) {
        let keys = project.key_map();
        let sets: Vec<Scale> = std::iter::once(keys.first())
            .chain(keys.markers().iter().map(|marker| marker.scale))
            .filter(|key| matches!(key.kind, lineup_core::ScaleKind::Set(_)))
            .collect();
        match sets.as_slice() {
            [] => {
                let label = ui.weak(tr("No key here is a set"));
                theme::hint(
                    label,
                    tr(
                        "The forms are read where the key is a pitch-class set or a \
                         micromode: pick one in the Key menu, or as a key change",
                    ),
                );
            }
            [key, ..] => {
                let size = key.pitch_set().len();
                let label = ui.weak(trf("{} notes", &[&size]));
                theme::hint(
                    label,
                    tr("A form of the key's set has as many notes as the set"),
                );
            }
        }
    }

    /// How many notes a group holds: in a micromode, always three.
    fn size_choice(&mut self, ui: &mut Ui) {
        if self.song_notation().fixed_size().is_some() {
            let label = ui.weak(tr("3 notes"));
            theme::hint(label, tr("A micromode is a group of three notes"));
            return;
        }
        let together = self.grouping == Grouping::Together;
        let shown = |size: Option<usize>| match size {
            None => tr("All notes").to_string(),
            Some(2) => tr("2 notes (dyads)").to_string(),
            Some(3) => tr("3 notes (trichords)").to_string(),
            Some(4) => tr("4 notes (tetrachords)").to_string(),
            Some(5) => tr("5 notes (pentachords)").to_string(),
            Some(6) => tr("6 notes (hexachords)").to_string(),
            Some(size) => trf("{} notes", &[&size]),
        };
        let size = self.size();
        let combo = egui::ComboBox::from_id_salt("chord-sheet-group-size")
            .selected_text(shown(size))
            .show_ui(ui, |ui| {
                if together {
                    let all = ui.selectable_label(size.is_none(), shown(None));
                    if theme::hint(all, tr("Every note sounding at once is one set")).clicked() {
                        self.choices.together = None;
                    }
                }
                for count in 2..=MOST_NOTES {
                    if ui
                        .selectable_label(size == Some(count), shown(Some(count)))
                        .clicked()
                    {
                        match together {
                            true => self.choices.together = Some(count),
                            false => self.choices.line = count,
                        }
                    }
                }
            });
        let hint = match together {
            true => {
                "How many different notes make a group, counted from the lowest note up; \
                 the highest group may hold fewer, and a note left over at the top joins \
                 the group under it"
            }
            false => {
                "How many notes make a group, one after another: 3 cuts a twelve-tone row \
                 into four trichords, 6 into its two halves"
            }
        };
        theme::hint(combo.response, tr(hint));
    }

    /// Which track's notes are grouped: every track, or one.
    fn track_choice(&mut self, ui: &mut Ui, project: &Project) {
        let tracks = self.tracks(project);
        if matches!(self.track, Track::Channel(channel) if !tracks.contains(&channel)) {
            self.track = Track::Auto;
        }
        let name = |channel: usize| {
            project
                .channels()
                .get(channel)
                .map_or_else(String::new, |channel| tr_name(&channel.name).into_owned())
        };
        let in_line = self.grouping == Grouping::InLine;
        let auto = match (in_line, default_melody_channel(project)) {
            (true, Some(tune)) => trf("Auto ({})", &[&name(tune)]),
            _ => tr("All tracks").to_string(),
        };
        let shown = match self.track {
            Track::Auto => auto.clone(),
            Track::All => tr("All tracks").to_string(),
            Track::Channel(channel) => name(channel),
        };
        ui.label(tr("of"));
        let combo = egui::ComboBox::from_id_salt("chord-sheet-group-track")
            .selected_text(shown)
            .show_ui(ui, |ui| {
                if in_line {
                    ui.selectable_value(&mut self.track, Track::Auto, auto)
                        .on_hover_text(tr(
                            "The tune: the track called Lead, or else the highest one",
                        ));
                }
                ui.selectable_value(&mut self.track, Track::All, tr("All tracks"));
                for &channel in &tracks {
                    ui.selectable_value(&mut self.track, Track::Channel(channel), name(channel));
                }
            });
        theme::hint(
            combo.response,
            tr(
                "Whose notes are grouped: every track that plays notes together, or one, \
             such as the tune or a chord part",
            ),
        );
    }

    /// The channel whose notes are grouped, or `None` for every one.
    fn channel(&self, project: &Project) -> Option<usize> {
        match self.track {
            Track::Channel(channel) => Some(channel),
            Track::All => None,
            Track::Auto => match self.grouping {
                Grouping::InLine => default_melody_channel(project),
                _ => None,
            },
        }
    }

    /// The tracks that play notes in the song, in the rack's order.
    fn tracks(&self, project: &Project) -> Vec<usize> {
        let mut tracks: Vec<usize> = match &self.read {
            Some(read) => read.notes.iter().map(|note| note.part).collect(),
            None => project
                .heard_notes(0, project.song_length())
                .iter()
                .map(|note| note.part)
                .collect(),
        };
        tracks.sort_unstable();
        tracks.dedup();
        tracks
    }

    /// What the bars show, read from the song's notes: every group, in
    /// order, for the sheet to place in its bars, read again only when
    /// the notes, the keys or the choices change. Empty when the bars
    /// show what is written.
    /// `pass` is the frame's (see `egui::Context::cumulative_pass_nr`):
    /// what was read in it is not checked again.
    pub fn heard(&mut self, project: &Project, keys: &KeyMap, pass: u64) -> Rc<[Shown]> {
        if !self.reads_notes() {
            self.read = None;
            return Rc::from([]);
        }
        self.song_key = project.scale;
        let from = (
            self.notation(),
            self.grouping,
            self.size(),
            self.channel(project),
        );
        if let Some(read) = &self.read
            && read.from == from
            && read.pass == pass
        {
            return read.shown.clone();
        }
        let notes = project.heard_notes(0, project.song_length());
        match &mut self.read {
            Some(read) if read.from == from && read.notes == notes && read.keys == *keys => {
                read.pass = pass;
                read.shown.clone()
            }
            _ => {
                let (notation, grouping, size, channel) = from;
                let groups = heard_groups(project, grouping, size, channel);
                let shown: Rc<[Shown]> = self.show(project, &groups, keys).into();
                let names = heard_names(&groups, notation, keys);
                let tally = Rc::new(most_used(&names, project.song_length()));
                self.read = Some(Read {
                    from,
                    notes,
                    keys: keys.clone(),
                    shown: shown.clone(),
                    tally,
                    pass,
                });
                shown
            }
        }
    }

    /// The groups read from the notes, as the bars show them.
    fn show(&self, project: &Project, groups: &[HeardGroups], keys: &KeyMap) -> Vec<Shown> {
        let notation = self.notation();
        let grouping = self.grouping;
        let channel = self
            .channel(project)
            .and_then(|channel| project.channels().get(channel));
        let what = |grouping: Grouping| match (grouping, channel) {
            (Grouping::InLine, None) => tr("Notes of every track one after another").to_string(),
            (Grouping::Mixed, None) => {
                tr("Forms of the key heard in the notes of every track").to_string()
            }
            (_, None) => tr("Notes of every track sounding together").to_string(),
            (Grouping::InLine, Some(channel)) => trf(
                "Notes of the {} one after another",
                &[&tr_name(&channel.name)],
            ),
            (Grouping::Mixed, Some(channel)) => trf(
                "Forms of the key heard in the notes of the {}",
                &[&tr_name(&channel.name)],
            ),
            (_, Some(channel)) => trf(
                "Notes of the {} sounding together",
                &[&tr_name(&channel.name)],
            ),
        };
        groups
            .iter()
            .map(|heard| {
                let key = keys.key_at(heard.start);
                let names: Vec<HarmonyName> = heard
                    .sets()
                    .into_iter()
                    .map(|set| notation.name_group(set, key))
                    .collect();
                let mut notes: Vec<Note> = heard
                    .groups
                    .iter()
                    .flat_map(|group| group.notes.iter().copied())
                    .collect();
                notes.sort();
                notes.dedup();
                let what = what(grouping);
                let about = match names.as_slice() {
                    [name] => format!("{what}. {}", name.describe(key)),
                    _ => {
                        let groups: Vec<String> =
                            names.iter().rev().map(|name| name.describe(key)).collect();
                        trf(
                            "{}, in groups from the lowest up, written upper first. {}",
                            &[&what, &groups.join(tr(" Under it, "))],
                        )
                    }
                };
                Shown {
                    start: heard.start,
                    text: stacked_text(&names),
                    short: stacked_classes(&names),
                    caption: Some(stacked_notes(&heard.groups, key)),
                    tint: None,
                    group: true,
                    no_chord: false,
                    notes,
                    about,
                }
            })
            .collect()
    }

    /// A harmony written on the sheet, as the notation names it.
    /// `caption` and `describe` say what the sheet writes under and
    /// about a single harmony, and `tint` in which colour, if not the
    /// usual one; `voicing` gives the notes to hear.
    pub fn written(
        &self,
        marker: &ChordMarker,
        key: Scale,
        caption: impl Fn(&ChordMarker) -> Option<String>,
        describe: impl Fn(&ChordMarker) -> String,
        tint: impl Fn(&ChordMarker) -> Option<Color32>,
        voicing: impl Fn(HarmonyName) -> Vec<Note>,
    ) -> Shown {
        let names = self.notation().rename(marker.harmony, key);
        let notes = voicing(marker.harmony);
        let no_chord = marker.harmony == HarmonyName::NoChord;
        let group = names.iter().any(|name| super::is_group(*name));
        if let [name] = names.as_slice() {
            let shown = ChordMarker {
                harmony: *name,
                ..*marker
            };
            let about = match *name == marker.harmony {
                true => describe(marker),
                false => trf("{} Written as {}.", &[&describe(&shown), &marker.harmony]),
            };
            let marks = |text: String| {
                let carets = marker.push.map_or("", |push| push.marks());
                let dots = marker.stop.map_or("", |stop| stop.marks());
                format!("{carets}{text}{dots}")
            };
            return Shown {
                start: marker.start,
                text: shown.text(),
                short: stacked_classes(&names).map(marks),
                caption: caption(&shown),
                tint: tint(&shown),
                group,
                no_chord,
                notes,
                about,
            };
        }
        // Two chords at once, as two groups: the marks go around both,
        // and the caption has the notes of each, upper first, and what
        // the band does there.
        let carets = marker.push.map_or("", |push| push.marks());
        let dots = marker.stop.map_or("", |stop| stop.marks());
        let words: Vec<String> = [names
            .iter()
            .rev()
            .filter_map(|name| name.pitch_set())
            .map(|set| spelled_set(set, key))
            .collect::<Vec<String>>()
            .join(" | ")]
        .into_iter()
        .chain(marker.push.map(|_| tr("push").to_string()))
        .chain(marker.stop.map(|stop| tr(stop.name()).to_string()))
        .collect();
        let groups: Vec<String> = names.iter().rev().map(|name| name.describe(key)).collect();
        Shown {
            start: marker.start,
            text: format!("{carets}{}{dots}", stacked_text(&names)),
            short: stacked_classes(&names).map(|short| format!("{carets}{short}{dots}")),
            caption: Some(words.join(" · ")),
            tint: None,
            group,
            no_chord,
            notes,
            about: trf(
                "{} Under it, {} Written as {}, two chords at once.",
                &[
                    &groups[0],
                    &groups[1..].join(tr(" Under it, ")),
                    &marker.harmony,
                ],
            ),
        }
    }
}

/// The line of the most used classes: the first, which sounds the
/// longest, brightest, the next ones fainter, each with how much of the
/// song it fills; its hint tells how they are counted. `heard` says the
/// groups are read from the notes rather than written on the sheet.
fn most_used_line(ui: &mut Ui, tally: &Tally, heard: bool) {
    let label = ui.label(tr("Most used:"));
    let counted = match heard {
        true => tr(
            "The classes of the groups the sheet reads from the notes, the one that sounds \
             longest first. Each group lasts from where it starts to where the next one \
             starts, as the sheet writes a harmony lasting; two of one class at once count \
             once",
        ),
        false => tr(
            "The classes of the harmonies written on the sheet, as the notation names them, \
             the one that sounds longest first. Each lasts from where it is written to the \
             next one; chords, row forms and N.C. are left out",
        ),
    };
    let every: Vec<String> = tally
        .classes
        .iter()
        .map(|class| about_class(tally, class))
        .collect();
    theme::hint(label, format!("{counted}.\n\n{}", every.join("\n")));
    for (index, class) in tally.classes.iter().take(MOST_SHOWN).enumerate() {
        if index > 0 {
            ui.weak("·");
        }
        let text = format!("{} {}", class.class, share(tally, class.steps));
        let item = match index {
            0 => ui.label(egui::RichText::new(text).strong()),
            _ => ui.weak(text),
        };
        theme::hint(item, about_class(tally, class));
    }
    let rest = tally.classes.len().saturating_sub(MOST_SHOWN);
    if rest > 0 {
        let more = ui.weak(trf("· {} more", &[&rest]));
        let others: Vec<String> = tally.classes[MOST_SHOWN..]
            .iter()
            .map(|class| about_class(tally, class))
            .collect();
        theme::hint(more, others.join("\n"));
    }
}

/// How much of the song `steps` fills: "62%", or "<1%" for a little.
fn share(tally: &Tally, steps: usize) -> String {
    match tally.percent(steps) {
        0 if steps > 0 => "<1%".to_string(),
        percent => format!("{percent}%"),
    }
}

/// A class and how the song uses it, for a hint: "Menor 3: 62% of the
/// song, 14 times. Menor 3 T4I 30%, Menor 3 20%".
fn about_class(tally: &Tally, class: &ClassUse) -> String {
    let times = match class.times {
        1 => tr("once").to_string(),
        times => trf("{} times", &[&times]),
    };
    let forms: Vec<String> = class
        .forms
        .iter()
        .map(|(form, steps)| format!("{form} {}", share(tally, *steps)))
        .collect();
    trf(
        "{}: {} of the song, {}. {}",
        &[
            &class.class,
            &share(tally, class.steps),
            &times,
            &forms.join(", "),
        ],
    )
}
