//! The harmony keys bar along the bottom of the window.
//!
//! On the left, the key and the time signature of the project. In the
//! middle, a big key for each of the seven chords of the key, coloured by
//! what the chord does (home, away, tension; in a mode, which has no such
//! jobs, all one colour); hold one down to play its chord. The chord
//! sounding now is lit up and framed in white, so it is the first thing
//! the eye finds; the chords that often come next only get a thin yellow
//! outline, the most usual one a little brighter. The
//! hint line below can explain in sentences, in a few words, or not at
//! all (see [`HintLevel`]). On the right, a keyboard showing the notes of
//! the scale and the notes playing.
//!
//! In a song, the chord playing is the one heard over the whole beat (see
//! [`lineup_core::harmony::hearing`]), so an arpeggio lights its chord.
//! Two chords at once light both their keys; other readings that fit
//! nearly as well light theirs faintly, and the line under the keys names
//! what is heard when it is not one chord: two chords at once, such as
//! the Rite of Spring's Eb7|Fb, or a set of notes, such as 4-23.
//!
//! In a key of three notes, one of Kröpfl's micromodes, the keys are the
//! micromode's own forms instead, each called by its degree (T0 for the
//! key's own form, T2, T5I...), home first and the nearest next; the
//! forms that keep two notes of the last one played get the outline,
//! since keeping two notes and moving one is the smoothest way on.
//!
//! Under it, the experimental Find the key option shows the key the
//! music sounds like it is in, and the next best keys, fainter, when it
//! is not clear, each with what the chord playing is in it.
//!
//! Like the other views, [`HarmonyKeys::show`] does not change anything
//! itself: it returns [`HarmonyAction`]s for the app to carry out.

use eframe::egui::{
    self, Align2, Color32, FontId, Key, Painter, Rect, Response, Sense, Stroke, StrokeKind, Ui,
    pos2, vec2,
};
use lineup_core::harmony::colour_finding::{Colour, ColourGuess, Colours};
use lineup_core::harmony::degree::{self, Degree};
use lineup_core::harmony::key_finding::{KeyGuess, read_chord};
use lineup_core::harmony::keys::{self, ChordKey, ChordSize, Function, GroupKey};
use lineup_core::harmony::{
    Harmony, Hearing, HintLevel, Micromode, Suggestion, analyze, next_chords_over, with_figures,
};
use lineup_core::{
    Chord, HarmonyName, Note, PitchClass, PitchSet, Project, Scale, ScaleKind, TimeSignature,
};

use crate::i18n::{tr, trc, trf};
use crate::time_picker::{describe_time, time_picker};

/// Something the user did in the harmony keys bar.
#[derive(Debug, Clone, PartialEq)]
pub enum HarmonyAction {
    SetScale(Scale),
    SetTime(TimeSignature),
    /// A harmony key went down: start these notes.
    NoteOn(Vec<Note>),
    /// A harmony key came up: stop these notes.
    NoteOff(Vec<Note>),
    /// Find the key: write the key changes heard in the song as key
    /// markers.
    MarkHeardKeys,
    /// Find the key: make a colour heard the key of its passage, from
    /// timeline step `start` up to `end`, moving no note.
    UseColour {
        start: usize,
        end: usize,
        key: Scale,
    },
}

/// The key the music sounds like it is in, for Find the key.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyHeard {
    pub best: KeyGuess,
    /// The keys that come close, best first; empty when the key is clear.
    pub alternatives: Vec<KeyGuess>,
    /// Where it was heard: "in the notes here", "in what you play"...
    pub from: &'static str,
    /// Whether the key changes heard in the whole song can be written
    /// as key markers (in song mode, from the notes or a recording).
    pub can_mark: bool,
    /// The micromode or the set the passage is made of, if any fits.
    pub colours: Option<ColoursHeard>,
}

/// The colours of the passage around the playhead, for Find the key: the
/// micromodes it fits and the collection of notes it is made of.
#[derive(Debug, Clone, PartialEq)]
pub struct ColoursHeard {
    pub colours: Colours,
    /// Where it was heard: "in bars 9 to 16", "in this pattern".
    pub from: String,
    /// The passage, in timeline steps, when a colour can be made its key.
    pub passage: Option<(usize, usize)>,
    /// The key there now: a colour that is already it is not offered.
    pub key: Scale,
}

/// Find the key's line for the colour of the passage: the micromode it
/// fits best, strong, the others that fit almost as well, fainter, and
/// the collection of notes it is made of, each with a button that makes
/// it the passage's key.
fn colour_line(ui: &mut Ui, heard: &ColoursHeard, actions: &mut Vec<HarmonyAction>) {
    let percent = |guess: &ColourGuess| (guess.fit * 100.0).round();
    let notes = |key: Scale| -> String {
        let order = key.pitch_set().normal_form();
        let names: Vec<String> = order.iter().map(|&p| key.spell(p).to_string()).collect();
        names.join(" ")
    };
    // A colour that is the key there already needs no button.
    let is_key = |colour: Colour| match (heard.key.kind, colour) {
        (ScaleKind::Set(_), Colour::Micromode(micromode)) => {
            Micromode::of(heard.key.pitch_set()) == Some(micromode)
        }
        (ScaleKind::Set(_), Colour::Collection(class)) => {
            heard.key.pitch_set().set_class() == class
        }
        _ => false,
    };
    let use_button = |ui: &mut Ui, guess: &ColourGuess, actions: &mut Vec<HarmonyAction>| {
        if is_key(guess.colour) {
            ui.weak(tr("(the key)"));
            return;
        }
        let Some((start, end)) = heard.passage else {
            return;
        };
        let button = ui.small_button(tr("Use as the key")).on_hover_text(trf(
            "Make {} the key {}, with key markers. No note moves; Undo takes it back.",
            &[&crate::i18n::key_name(guess.key), &heard.from],
        ));
        if button.clicked() {
            actions.push(HarmonyAction::UseColour {
                start,
                end,
                key: guess.key,
            });
        }
    };
    ui.horizontal_wrapped(|ui| {
        ui.weak(trf("Colour {}:", &[&heard.from]));
        let micromodes = &heard.colours.micromodes;
        if let Some(best) = micromodes.first()
            && let Colour::Micromode(micromode) = best.colour
        {
            ui.label(
                egui::RichText::new(crate::i18n::key_name(best.key))
                    .strong()
                    .size(15.0),
            )
            .on_hover_text(trf(
                "Kröpfl's micromode {}: {}% of the notes are in its forms, read as the \
                 colour check reads them. Its form heard longest, {}, is its home.",
                &[&micromode.full_name(), &percent(best), &notes(best.key)],
            ));
            use_button(ui, best, actions);
            for guess in &micromodes[1..] {
                let Colour::Micromode(other) = guess.colour else {
                    continue;
                };
                ui.weak(tr("·  or"));
                ui.label(egui::RichText::new(other.full_name()).weak())
                    .on_hover_text(trf(
                        "Fits almost as well ({}% of the notes in its forms).",
                        &[&percent(guess)],
                    ));
            }
        }
        if let Some(collection) = &heard.colours.collection
            && let Colour::Collection(class) = collection.colour
        {
            if !micromodes.is_empty() {
                ui.separator();
            }
            let name = match class.nickname() {
                Some(nickname) => format!("{} ({})", class.name(), tr(nickname)),
                None => class.name(),
            };
            ui.weak(tr("made of"));
            ui.label(egui::RichText::new(name).strong())
                .on_hover_text(trf(
                    "The notes that sound {}% of the time make this set: {}.",
                    &[&percent(collection), &notes(collection.key)],
                ));
            use_button(ui, collection, actions);
        }
    });
}

const FIND_KEY_HELP: &str = "Experimental: works out the key from the music itself, \
bar by bar, the way the key changes as the song goes on. When the key is not clear, \
the closest other keys are shown too, fainter, with what the chord playing is in each.";

const KEY_SIZE: egui::Vec2 = vec2(76.0, 66.0);
const KEY_GAP: f32 = 4.0;
/// The keyboard on the right: from C2, four octaves, with white keys
/// 11 points wide. In a narrow window it gets smaller and shows fewer
/// octaves, so it stays beside the chord keys instead of taking a row
/// of its own: (lowest note, octaves, white key width), biggest first.
const PIANO_SIZES: [(u8, u8, f32); 6] = [
    (36, 4, 11.0),
    (36, 4, 9.0),
    (36, 3, 10.0),
    (48, 2, 10.0),
    (48, 2, 8.0),
    (48, 2, 6.0),
];
const PIANO_HEIGHT: f32 = 58.0;

/// The colour of each function: blue for home, green for away, orange for
/// tension; grey-blue for the chords of a mode, which have no function.
pub(crate) fn function_color(function: Option<Function>) -> Color32 {
    match function {
        Some(Function::Tonic) => Color32::from_rgb(76, 136, 222),
        Some(Function::Subdominant) => Color32::from_rgb(86, 176, 110),
        Some(Function::Dominant) => Color32::from_rgb(226, 124, 64),
        None => MODE_COLOR,
    }
}

/// The colour of the keys of a mode.
const MODE_COLOR: Color32 = Color32::from_rgb(92, 112, 150);

pub(crate) const KEY_TEXT: Color32 = Color32::from_rgb(244, 246, 250);
pub(crate) const KEY_TEXT_DIM: Color32 = Color32::from_rgb(196, 202, 214);
const SUGGESTED: Color32 = Color32::from_rgb(255, 214, 92);
/// The frame of the key whose chord is sounding now.
const PLAYING: Color32 = Color32::from_rgb(255, 255, 255);
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
    /// The form, for the keys of a micromode.
    form: Option<PitchSet>,
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
    /// The lowest note under the last chord: E when it was C/E, its first
    /// inversion.
    last_bass: Option<PitchClass>,
    /// Whether Find the key is on.
    pub find_key: bool,
    /// In a key that is a set, whether every form gets a key, or only
    /// those that keep enough notes of home (see [`keys::form_keys`]).
    all_forms: bool,
    /// In a key that is a set of more than three notes, whether the keys
    /// are the set's forms rather than chords stacked from its notes. A
    /// set of three notes is its one chord, so its keys are always its
    /// forms.
    forms: bool,
    /// The last form of the key's set played, here or in the music, for
    /// the outline on the forms that keep all but one of its notes.
    last_form: Option<PitchSet>,
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
            last_bass: None,
            find_key: false,
            all_forms: false,
            forms: false,
            last_form: None,
        }
    }

    /// Forgets the chords played so far, for another song: its first
    /// chord gets no "what comes next" from the last song's last chord.
    pub fn forget_chords(&mut self) {
        self.last_voicing = None;
        self.last_chord = None;
        self.last_bass = None;
        self.last_form = None;
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

    /// Draws the bar. `here` is the key the music is in where the
    /// playhead is, which the chord keys follow: the song's own key, or
    /// the one a key change in the playlist moved to. `held` are the
    /// notes held down on any keyboard,
    /// harmony keys included, and `heard` the chord notes playing in the
    /// pattern or the song, with `hearing`, how the song's beat at the
    /// playhead is heard, if there is one; `digits` says whether the
    /// number keys may play the harmony keys (not while they play notes
    /// on the typing keyboard, or a text field has the keyboard).
    #[allow(clippy::too_many_arguments)]
    pub fn show(
        &mut self,
        ui: &mut Ui,
        project: &Project,
        here: Scale,
        channel_name: &str,
        held: &[Note],
        heard: &[Note],
        hearing: Option<&Hearing>,
        digits: bool,
        key_heard: Option<&KeyHeard>,
        parts: &mut lineup_core::library::apply::StyleParts,
    ) -> Vec<HarmonyAction> {
        let scale = here;
        let size = self.size_for(scale);
        if self.shows_forms(scale)
            && let Some(forms) = keys::form_keys(scale, self.all_forms)
        {
            let mut actions =
                self.show_forms(ui, project, here, &forms, held, heard, digits, parts);
            self.find_key_line(ui, key_heard, &mut actions);
            return actions;
        }
        if let Some(groups) = keys::group_keys(scale, size) {
            let mut actions = self.show_groups(
                ui,
                project,
                here,
                &groups,
                channel_name,
                held,
                heard,
                digits,
                parts,
            );
            self.find_key_line(ui, key_heard, &mut actions);
            return actions;
        }
        let mut actions = Vec::new();
        let chord_keys = keys::chord_keys(scale, size);
        // The key the chords are spelled in: for a pentatonic or blues
        // scale, the major or minor key its chords come from.
        let tonal = keys::chord_scale(scale);

        // A chord played on any keyboard, or else playing in the music,
        // lights its key, and is what the suggestions follow. In a song,
        // the music's chord is the one heard over the beat.
        let chord_of = |notes: &[Note]| match analyze(notes.iter().copied()) {
            Harmony::Chord { chord, bass } => Some((chord, bass)),
            _ => None,
        };
        let music = hearing.filter(|_| held.is_empty());
        let played = chord_of(held).or_else(|| match music.map(Hearing::best) {
            Some(Harmony::Chord { chord, bass }) => Some((chord, bass)),
            Some(_) => None,
            None => chord_of(heard),
        });
        if let Some((chord, bass)) = played {
            self.last_chord = Some(chord);
            self.last_bass = Some(bass);
        }
        let playing = played.and_then(|(chord, _)| keys::key_for(&chord_keys, chord));
        // Both chords of two at once light up; a reading that fits nearly
        // as well lights its key faintly.
        let key_of = |chord: Chord| keys::key_for(&chord_keys, chord);
        let layers: Vec<usize> = match music.map(Hearing::best) {
            Some(Harmony::Polychord { upper, lower, .. }) => {
                [upper, lower].into_iter().filter_map(key_of).collect()
            }
            _ => Vec::new(),
        };
        let faint: Vec<usize> = music
            .map(|hearing| {
                hearing
                    .alternatives()
                    .iter()
                    .flat_map(|reading| match reading.harmony {
                        Harmony::Chord { chord, .. } => vec![chord],
                        Harmony::Polychord { upper, lower, .. } => vec![upper, lower],
                        _ => Vec::new(),
                    })
                    .filter_map(key_of)
                    .collect()
            })
            .unwrap_or_default();
        // What follows depends on the inversion too: C/G, I⁶₄, goes to V.
        let suggestions = self
            .last_chord
            .map(|chord| next_chords_over(tonal, chord, self.last_bass.unwrap_or(chord.root)))
            .unwrap_or_default();

        ui.add_space(4.0);
        let mut pressed = None;
        // Wrapped, so on a narrow window the little piano moves to a row
        // of its own instead of running off the edge.
        ui.horizontal_wrapped(|ui| {
            ui.vertical(|ui| self.settings(ui, project, here, parts, &mut actions));
            ui.separator();
            for (index, key) in chord_keys.iter().enumerate() {
                let rank = suggestion_rank(&suggestions, &chord_keys, index);
                let lit = playing == Some(index)
                    || layers.contains(&index)
                    || self.down.as_ref().is_some_and(|d| d.key == index);
                let sounding = played.filter(|_| playing == Some(index));
                let faintly = !lit && faint.contains(&index);
                let response = chord_key(ui, key, tonal, lit, faintly, sounding, rank);
                let mut hover = describe_key(key, tonal, self.hints);
                if let Some(rank) = rank {
                    let most = rank == 0;
                    hover = match suggestions[rank].why(self.hints) {
                        Some(why) if self.hints == HintLevel::Detailed => {
                            let often = match most {
                                true => tr("Most often comes next."),
                                false => tr("Often comes next."),
                            };
                            format!("{hover}\n\n{often} {why}")
                        }
                        Some(why) => {
                            let often = match most {
                                true => trf("Most often next: {}.", &[&why]),
                                false => trf("Often next: {}.", &[&why]),
                            };
                            format!("{hover}\n\n{often}")
                        }
                        None => {
                            let often = match most {
                                true => tr("Most often next."),
                                false => tr("Often next."),
                            };
                            format!("{hover}\n\n{often}")
                        }
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

        let suggests = keys::has_functions(tonal.kind);
        let outside = suggests
            && self
                .last_chord
                .is_some_and(|chord| keys::key_for(&chord_keys, chord).is_none());
        ui.horizontal_wrapped(|ui| {
            self.hints_menu(ui);
            if let Some((best, others)) = music.and_then(|hearing| heard_line(hearing, tonal)) {
                ui.label(best);
                if !others.is_empty() {
                    ui.weak(others);
                }
            }
            let hint = hint_line(
                self.hints,
                tonal,
                self.last_chord
                    .map(|chord| (chord, self.last_bass.unwrap_or(chord.root))),
                &suggestions,
                suggests,
                outside,
                channel_name,
            );
            ui.weak(hint);
        });
        self.find_key_line(ui, key_heard, &mut actions);

        let plays: Vec<Play> = chord_keys
            .iter()
            .map(|key| {
                let (voicing, notes) = keys::play(key.chord, self.last_voicing.as_deref());
                Play {
                    voicing,
                    notes,
                    chord: Some(key.chord),
                    form: None,
                }
            })
            .collect();
        self.follow_mouse(pressed, &plays, &mut actions);
        self.follow_digits(ui, digits, &plays, &mut actions);
        actions
    }

    /// The bar for a key of three notes, a micromode: a key for each of
    /// its forms (see [`keys::form_keys`]), the brighter the more notes it
    /// keeps from home. The form playing, held on a keyboard or heard in
    /// the music, is lit, and the forms that keep two of its notes (or of
    /// the last one played) are outlined: the smoothest ways on.
    #[allow(clippy::too_many_arguments)]
    fn show_forms(
        &mut self,
        ui: &mut Ui,
        project: &Project,
        here: Scale,
        forms: &[GroupKey],
        held: &[Note],
        heard: &[Note],
        digits: bool,
        parts: &mut lineup_core::library::apply::StyleParts,
    ) -> Vec<HarmonyAction> {
        let mut actions = Vec::new();
        let scale = here;
        let key = scale.pitch_set();
        let set_of =
            |notes: &[Note]| -> PitchSet { notes.iter().map(|n| n.pitch_class()).collect() };
        let playing = [set_of(held), set_of(heard)]
            .into_iter()
            .find(|&set| set.len() == key.len() && Degree::of(scale, set).is_some());
        if playing.is_some() {
            self.last_form = playing;
        }
        // A form of another key's micromode means nothing here.
        let last = self
            .last_form
            .filter(|&form| Degree::of(scale, form).is_some());
        let next = last.map(degree::smooth_links).unwrap_or_default();
        let mut pressed = None;
        ui.add_space(4.0);
        ui.horizontal_wrapped(|ui| {
            ui.vertical(|ui| self.settings(ui, project, here, parts, &mut actions));
            ui.separator();
            for (index, form) in forms.iter().enumerate() {
                let set = form.pitch_set();
                let lit =
                    self.down.as_ref().is_some_and(|d| d.key == index) || playing == Some(set);
                let outlined = !lit && next.contains(&set);
                let response = form_key(ui, form, scale, lit, outlined);
                let hover = describe_form(form, index, self.hints, scale, last);
                let response = crate::theme::hint(response, hover);
                if response.is_pointer_button_down_on() {
                    pressed = Some(index);
                }
            }
            ui.separator();
            piano(ui, scale, held, heard);
        });
        ui.horizontal_wrapped(|ui| {
            self.hints_menu(ui);
            let keys = trf("1 to {}", &[&forms.len().min(7)]);
            let from = last
                .and_then(|form| Degree::of(scale, form))
                .map(|degree| degree.to_string());
            let micromode = scale.micromode().is_some();
            let hint = match (self.hints, from) {
                (HintLevel::Minimal, _) => String::new(),
                (HintLevel::Short, Some(from)) if micromode => trf(
                    "The micromode's forms, nearest home first. Outlined: they keep two notes of {}.",
                    &[&from],
                ),
                (HintLevel::Short, Some(from)) => trf(
                    "The set's forms, nearest home first. Outlined: they keep all but one note \
                     of {}.",
                    &[&from],
                ),
                (HintLevel::Short, None) if micromode => trf(
                    "The micromode's forms, nearest home first. Hold a key or press {}.",
                    &[&keys],
                ),
                (HintLevel::Short, None) => trf(
                    "The set's forms, nearest home first. Hold a key or press {}.",
                    &[&keys],
                ),
                (_, from) if micromode => {
                    let outline = match from {
                        Some(from) => trf(
                            " The outlined forms keep two notes of {} and move one: the \
                             smoothest ways on.",
                            &[&from],
                        ),
                        None => String::new(),
                    };
                    trf(
                        "Music on one micromode makes its chords and its tunes of the \
                         micromode's own forms, so these are its keys, each called by its \
                         degree: T0 is the key's own form, T2 the same moved up two \
                         semitones, T2I upside down. The more notes a form keeps from home, the \
                         brighter its key.{} Hold one (or press {}) to play it.",
                        &[&outline, &keys],
                    )
                }
                (_, from) => {
                    let outline = match from {
                        Some(from) => trf(
                            " The outlined forms keep all but one note of {}: the smoothest \
                             ways on.",
                            &[&from],
                        ),
                        None => String::new(),
                    };
                    trf(
                        "Music on one pitch-class set makes its harmony of the set's own \
                         forms, so these are its keys, each called by its degree: T0 is the \
                         key's own form, T2 the same moved up two semitones, T2I upside down. \
                         The more notes a form keeps from home, the brighter its key.{} Hold \
                         one (or press {}) to play it.",
                        &[&outline, &keys],
                    )
                }
            };
            ui.weak(hint);
        });
        let plays: Vec<Play> = forms
            .iter()
            .map(|form| Play {
                voicing: Vec::new(),
                notes: form.play(),
                chord: None,
                form: Some(form.pitch_set()),
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
        here: Scale,
        groups: &[GroupKey],
        channel_name: &str,
        held: &[Note],
        heard: &[Note],
        digits: bool,
        parts: &mut lineup_core::library::apply::StyleParts,
    ) -> Vec<HarmonyAction> {
        let mut actions = Vec::new();
        let scale = here;
        let heard_set: PitchSet = heard.iter().map(|note| note.pitch_class()).collect();
        let mut pressed = None;
        ui.add_space(4.0);
        ui.horizontal_wrapped(|ui| {
            ui.vertical(|ui| self.settings(ui, project, here, parts, &mut actions));
            ui.separator();
            for (index, group) in groups.iter().enumerate() {
                let lit = self.down.as_ref().is_some_and(|d| d.key == index)
                    || (heard_set.len() >= 2 && group.pitch_set() == heard_set);
                let response = group_key(ui, group, lit);
                let response =
                    crate::theme::hint(response, describe_group(group, index, self.hints));
                if response.is_pointer_button_down_on() {
                    pressed = Some(index);
                }
            }
            ui.separator();
            piano(ui, scale, held, heard);
        });
        ui.horizontal_wrapped(|ui| {
            self.hints_menu(ui);
            let keys = trf("1 to {}", &[&groups.len().min(7)]);
            let channel_name = crate::i18n::tr_name(channel_name);
            let hint = match (self.hints, scale.kind) {
                (HintLevel::Minimal, _) => String::new(),
                (HintLevel::Short, ScaleKind::Row(_)) => trf(
                    "The row's notes in groups, in order. Hold a key or press {}.",
                    &[&keys],
                ),
                (HintLevel::Short, _) => trf(
                    "Chords stacked from the set's notes. Hold a key or press {}.",
                    &[&keys],
                ),
                (_, ScaleKind::Row(_)) => trf(
                    "Twelve-tone music has no home chord: each key plays the next notes of \
                     the row together, as twelve-tone composers build their chords. Hold one \
                     (or press {}) to play on \"{}\". 3, 4 or 6 notes on the left.",
                    &[&keys, &channel_name],
                ),
                _ => trf(
                    "Each key stacks every other note of the set, from one of its notes, as \
                     the chords of a key stack every other note of the scale. Hold one (or \
                     press {}) to play it.",
                    &[&keys],
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
                form: None,
            })
            .collect();
        self.follow_mouse(pressed, &plays, &mut actions);
        self.follow_digits(ui, digits, &plays, &mut actions);
        actions
    }

    /// The Find the key switch and, when it is on, the key heard: the
    /// best key strong, the close ones after it, fainter, each with what
    /// the last chord is in it.
    fn find_key_line(
        &mut self,
        ui: &mut Ui,
        heard: Option<&KeyHeard>,
        actions: &mut Vec<HarmonyAction>,
    ) {
        ui.horizontal_wrapped(|ui| {
            ui.toggle_value(&mut self.find_key, tr("🔍 Find the key"))
                .on_hover_text(tr(FIND_KEY_HELP));
            if !self.find_key {
                return;
            }
            let Some(heard) = heard else {
                ui.weak(tr("Nothing to listen to yet."));
                return;
            };
            ui.weak(trf("Sounds {}:", &[&tr(heard.from)]));
            let reading = |guess: &KeyGuess| {
                self.last_chord
                    .and_then(|chord| {
                        let bass = self.last_bass.unwrap_or(chord.root);
                        let reading =
                            read_chord(chord, guess.key)?.inverted(chord.inversion_figures(bass));
                        Some(trf(
                            "{} is {}",
                            &[&chord_label(chord, bass, guess.key), &reading.label()],
                        ))
                    })
                    .unwrap_or_default()
            };
            let share = |guess: &KeyGuess| (guess.share * 100.0).round();
            let best = heard.best;
            let text = egui::RichText::new(crate::i18n::key_name(best.key))
                .strong()
                .size(15.0);
            let clear = if heard.alternatives.is_empty() {
                tr("It is well ahead of every other key.")
            } else {
                tr("Other keys come close, so they are shown too.")
            };
            ui.label(text).on_hover_text(trf(
                "The key that fits the notes best ({}% of the weight of all 24 keys). {}",
                &[&share(&best), &clear],
            ));
            let chord = reading(&best);
            if !chord.is_empty() {
                ui.label(chord);
            }
            for guess in &heard.alternatives {
                ui.weak(tr("·  or"));
                ui.label(egui::RichText::new(crate::i18n::key_name(guess.key)).weak())
                    .on_hover_text(trf(
                        "Fits almost as well ({}% of the weight).",
                        &[&share(guess)],
                    ));
                let chord = reading(guess);
                if !chord.is_empty() {
                    ui.weak(chord);
                }
            }
            if heard.can_mark
                && ui
                    .small_button(tr("Mark on the playlist"))
                    .on_hover_text(tr(
                        "Write the key changes heard in the whole song as key markers, \
                         replacing the ones there. No note moves; Undo takes them back.",
                    ))
                    .clicked()
            {
                actions.push(HarmonyAction::MarkHeardKeys);
            }
        });
        let colours = heard.and_then(|heard| heard.colours.as_ref());
        if self.find_key
            && let Some(colours) = colours.filter(|colours| !colours.colours.is_empty())
        {
            colour_line(ui, colours, actions);
        }
    }

    /// How much the hints explain.
    fn hints_menu(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_id_salt("harmony-hints")
            .width(78.0)
            .selected_text(tr(self.hints.label()))
            .show_ui(ui, |ui| {
                for level in HintLevel::ALL {
                    ui.selectable_value(&mut self.hints, level, tr(level.label()))
                        .on_hover_text(tr(level.description()));
                }
            })
            .response
            .on_hover_text(tr("How much the hints explain, here and in the piano roll"));
    }

    /// The key, the time signature and the size of the chords. The key
    /// menus are the key the song starts in; when the playhead is past a
    /// key change, the key there is written under them.
    fn settings(
        &mut self,
        ui: &mut Ui,
        project: &Project,
        here: Scale,
        parts: &mut lineup_core::library::apply::StyleParts,
        actions: &mut Vec<HarmonyAction>,
    ) {
        let scale = project.scale;
        ui.horizontal(|ui| {
            ui.strong(tr("Key"));
            let name = egui::RichText::new(scale.spell(scale.root).to_string()).strong();
            let tonic = crate::twelve_tone::root_menu(ui, "harmony-root", scale, 56.0, name);
            if let Some(new) =
                crate::twelve_tone::kind_menu(ui, "harmony-kind", scale, tonic, 110.0, project)
            {
                actions.push(HarmonyAction::SetScale(new));
            }
        });
        if here != scale {
            ui.label(
                egui::RichText::new(trf("Now in {}", &[&crate::i18n::key_name(here)]))
                    .color(SUGGESTED),
            )
            .on_hover_text(tr(
                "The song changes key here (see the key names on the playlist's bar \
                     numbers), so the chord keys are this key's. The menus above are the key \
                     the song starts in",
            ));
        }
        ui.horizontal(|ui| {
            ui.strong(trc("meter", "Time"));
            let time = project.time_signature;
            let mut chosen = None;
            // The list holds fields to type a time signature in, so it
            // must stay open while typing: only a click outside closes it.
            egui::ComboBox::from_id_salt("harmony-time")
                .width(56.0)
                .height(420.0)
                .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
                .selected_text(egui::RichText::new(time.to_string()).strong())
                .show_ui(ui, |ui| {
                    chosen = time_picker(
                        ui,
                        egui::Id::unique("harmony-time-picker"),
                        time,
                        Some(parts),
                    );
                })
                .response
                .on_hover_text(describe_time(time));
            if let Some(chosen) = chosen.filter(|&chosen| chosen != time) {
                actions.push(HarmonyAction::SetTime(chosen));
            }
            ui.weak(trf("{} steps a bar", &[&time.steps_per_bar()]));
        });
        let scale = here;
        let set = scale.pitch_set();
        let trichord = set.len() == 3;
        if matches!(scale.kind, ScaleKind::Set(_))
            && !trichord
            && keys::form_keys(scale, false).is_some()
        {
            ui.horizontal(|ui| {
                let chords = ui
                    .selectable_label(!self.forms, tr("Chords"))
                    .on_hover_text(tr(
                        "Chords stacked from the set's notes, as a key stacks its scale's",
                    ));
                if chords.clicked() {
                    self.forms = false;
                }
                let forms = ui
                    .selectable_label(self.forms, tr("Forms"))
                    .on_hover_text(tr(
                        "The set itself, moved and turned upside down, each form called by its \
                         degree",
                    ));
                if forms.clicked() {
                    self.forms = true;
                }
            });
        }
        if self.shows_forms(scale) {
            ui.horizontal(|ui| {
                let near_hover = if trichord {
                    tr("The forms that share at least one note with the key's own form").to_string()
                } else {
                    trf(
                        "The forms that keep at least {} notes of the key's own form",
                        &[&(set.len() / 2)],
                    )
                };
                let near = ui
                    .selectable_label(!self.all_forms, tr("Near home"))
                    .on_hover_text(near_hover);
                if near.clicked() {
                    self.all_forms = false;
                }
                let all_hover = if scale.micromode().is_some() {
                    tr("Every form of the micromode, moved and upside down")
                } else {
                    tr("Every form of the set, moved and upside down")
                };
                let all = ui
                    .selectable_label(self.all_forms, tr("All forms"))
                    .on_hover_text(all_hover);
                if all.clicked() {
                    self.all_forms = true;
                }
            });
            return;
        }
        ui.horizontal(|ui| {
            let current = self.size_for(scale);
            for size in keys::chord_sizes(scale) {
                let (label, hover) = match (scale.kind, size) {
                    (ScaleKind::Row(_), _) => {
                        let n = keys::row_group(size);
                        let name = match n {
                            3 => tr("trichords"),
                            4 => tr("tetrachords"),
                            _ => tr("hexachords"),
                        };
                        (
                            trf("{} notes", &[&n]),
                            trf("The row in groups of {}: {}", &[&n, &name]),
                        )
                    }
                    (ScaleKind::Set(set), _) => {
                        // A set of two notes makes chords of two.
                        let n = (size as usize + 3).min(set.len());
                        let hover = if n == set.len() {
                            trf("All {} notes of the set, from each of them in turn", &[&n])
                        } else {
                            trf("Chords of {} notes of the set", &[&n])
                        };
                        (trf("{} notes", &[&n]), hover)
                    }
                    (_, ChordSize::Triads) => (
                        tr(size.label()).to_string(),
                        tr("Three notes: every other note of the scale").to_string(),
                    ),
                    (_, ChordSize::Sevenths) => (
                        tr(size.label()).to_string(),
                        tr("Four notes: the triad and its seventh").to_string(),
                    ),
                    (_, ChordSize::Ninths) => (
                        tr(size.label()).to_string(),
                        tr("Five notes: the seventh chord and its ninth").to_string(),
                    ),
                };
                if ui
                    .selectable_label(current == size, label)
                    .on_hover_text(hover)
                    .clicked()
                {
                    self.size = size;
                }
            }
        });
    }

    /// Whether the keys are the forms of the set `scale` is: always for a
    /// set of three notes, and for a bigger one when Forms is chosen.
    fn shows_forms(&self, scale: Scale) -> bool {
        match scale.kind {
            ScaleKind::Set(set) => set.len() == 3 || (self.forms && (2..12).contains(&set.len())),
            _ => false,
        }
    }

    /// The chord size to play in `scale`: the one chosen, or the biggest
    /// the scale has if it has none that big (a set of three notes makes
    /// no four-note chords). The choice is kept for the next scale.
    fn size_for(&self, scale: Scale) -> ChordSize {
        let sizes = keys::chord_sizes(scale);
        if sizes.contains(&self.size) {
            self.size
        } else {
            sizes.last().copied().unwrap_or_default()
        }
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
        if play.form.is_some() {
            self.last_form = play.form;
        }
        if let Some(chord) = play.chord {
            self.last_voicing = Some(play.voicing.clone());
            self.last_chord = Some(chord);
            self.last_bass = play.notes.iter().min().map(|note| note.pitch_class());
        }
        actions.push(HarmonyAction::NoteOn(notes.clone()));
        self.down = Some(Down {
            key: index,
            holder,
            notes,
        });
    }
}

/// The rank of harmony key `index` among the suggestions: 0 if it plays
/// the most usual next chord, `None` if it plays none of them. A
/// suggestion counts for a key only if the key plays that chord: in A
/// melodic minor, whose IV is D major, the suggestion Dm (iv) outlines
/// no key.
fn suggestion_rank(
    suggestions: &[Suggestion],
    chord_keys: &[ChordKey],
    index: usize,
) -> Option<usize> {
    suggestions
        .iter()
        .position(|s| keys::key_for(chord_keys, s.chord) == Some(index))
}

/// The line of text under the keys: what often comes after the last
/// chord, at the chosen level of detail, with the chords spelled in
/// `scale`. `suggests` says whether the key is one that gets suggestions
/// (major or minor), and `outside` whether the last chord is not one of
/// the key's seven.
fn hint_line(
    level: HintLevel,
    scale: Scale,
    last: Option<(Chord, PitchClass)>,
    suggestions: &[Suggestion],
    suggests: bool,
    outside: bool,
    channel_name: &str,
) -> String {
    match (last, suggestions.first()) {
        (Some((chord, bass)), Some(next)) => {
            let from = chord_label(chord, bass, scale);
            let to = scale.chord_name(next.chord);
            match level {
                HintLevel::Detailed => trf(
                    "After {}, {} often comes next: {}",
                    &[&from, &to, &next.reason],
                ),
                HintLevel::Short => format!("{from} → {to}: {}", next.short),
                HintLevel::Minimal => format!("{from} → {to}"),
            }
        }
        (Some((chord, bass)), None) if outside => match level {
            HintLevel::Detailed => trf(
                "{} is not one of the seven chords of {}: it comes from outside the key.",
                &[
                    &chord_label(chord, bass, scale),
                    &crate::i18n::key_name(scale),
                ],
            ),
            _ => trf("{}: outside the key", &[&chord_label(chord, bass, scale)]),
        },
        _ if !suggests => match level {
            HintLevel::Detailed => {
                tr("Suggestions of what comes next are given in major and minor keys.").to_string()
            }
            HintLevel::Short => tr("Suggestions in major and minor keys only.").to_string(),
            HintLevel::Minimal => String::new(),
        },
        _ => {
            let channel_name = crate::i18n::tr_name(channel_name);
            match level {
                HintLevel::Detailed => trf(
                    "Hold a key (or 1 to 7) to play its chord on \"{}\". With Rec on, \
                     the chords are recorded. Outlined keys are chords that often come next, \
                     the brightest the most usual.",
                    &[&channel_name],
                ),
                HintLevel::Short => trf(
                    "Hold a key or press 1 to 7 to play on \"{}\".",
                    &[&channel_name],
                ),
                HintLevel::Minimal => String::new(),
            }
        }
    }
}

/// What the music is heard as, when it is not simply one chord: two
/// chords at once or a set of notes, or a chord that could be heard
/// another way too. The best reading, then the others, fainter.
fn heard_line(hearing: &Hearing, scale: Scale) -> Option<(String, String)> {
    let label = |harmony: Harmony| harmony.label_in(scale);
    let best = match hearing.best() {
        harmony @ Harmony::Polychord { .. } => {
            trf("Heard: {}, two chords at once", &[&label(harmony)?])
        }
        harmony @ Harmony::Unknown(_) => trf("Heard: the notes {}", &[&label(harmony)?]),
        harmony @ Harmony::Chord { .. } if !hearing.is_clear() => {
            trf("Heard: {}", &[&label(harmony)?])
        }
        _ => return None,
    };
    let others: Vec<String> = hearing
        .alternatives()
        .iter()
        .filter_map(|reading| label(reading.harmony))
        .collect();
    let others = if others.is_empty() {
        String::new()
    } else {
        trf("or {}", &[&others.join(", ")])
    };
    Some((best, others))
}

/// One harmony key: the numeral, the chord's name (spelled in `scale`)
/// and its function, on the function's colour. While it plays it is
/// bright and framed in white, the strongest mark on the bar; `faintly`
/// lights it less, with a thin frame, for a chord the music could be
/// heard as too. When it is a chord that often comes next it gets a thin
/// yellow outline instead, brighter for the most usual one (`rank` 0): a
/// hint of what may come, never louder than what is sounding.
fn chord_key(
    ui: &mut Ui,
    key: &ChordKey,
    scale: Scale,
    lit: bool,
    faintly: bool,
    sounding: Option<(Chord, PitchClass)>,
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
    } else if response.hovered() || faintly {
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
    if lit {
        paint_playing(painter, rect);
    } else if faintly {
        let stroke = Stroke::new(1.5, PLAYING.gamma_multiply(0.45));
        painter.rect_stroke(rect, 6.0, stroke, StrokeKind::Inside);
    } else {
        match rank {
            Some(0) => {
                let stroke = Stroke::new(1.5, SUGGESTED.gamma_multiply(0.8));
                painter.rect_stroke(rect, 6.0, stroke, StrokeKind::Inside);
            }
            Some(_) => {
                let stroke = Stroke::new(1.0, SUGGESTED.gamma_multiply(0.4));
                painter.rect_stroke(rect, 6.0, stroke, StrokeKind::Inside);
            }
            None => {}
        }
    }
    // While it plays, it names the chord heard: with the figures of an
    // inversion and the bass after a slash (I⁶, C/E), and with its
    // seventh when the key of a triad lights for a seventh chord.
    let (numeral, name) = match sounding {
        Some((chord, bass)) => {
            let numeral = if chord == key.chord {
                Some(with_figures(&key.numeral, chord.inversion_figures(bass)))
            } else {
                scale.figured_numeral(chord, bass)
            };
            (
                numeral.unwrap_or_else(|| key.numeral.clone()),
                chord_label(chord, bass, scale),
            )
        }
        None => (key.numeral.clone(), scale.chord_name(key.chord)),
    };
    let numeral_size = fitting_size(ui, &numeral, 20.0, rect.width() - 6.0);
    let name_size = fitting_size(ui, &name, 13.0, rect.width() - 6.0);
    let painter = ui.painter();
    painter.text(
        rect.center_top() + vec2(0.0, 18.0),
        Align2::CENTER_CENTER,
        numeral,
        FontId::proportional(numeral_size),
        KEY_TEXT,
    );
    painter.text(
        rect.center() + vec2(0.0, 9.0),
        Align2::CENTER_CENTER,
        name,
        FontId::proportional(name_size),
        KEY_TEXT,
    );
    if let Some(function) = key.function {
        painter.text(
            rect.center_bottom() - vec2(0.0, 9.0),
            Align2::CENTER_CENTER,
            tr(function.name()),
            FontId::proportional(10.0),
            KEY_TEXT_DIM,
        );
    }
    response
}

/// The chord's name spelled in `scale`, with `bass` after a slash when
/// it is one of the chord's notes other than the root: "C/E".
fn chord_label(chord: Chord, bass: PitchClass, scale: Scale) -> String {
    Harmony::Chord { chord, bass }
        .label_in(scale)
        .unwrap_or_else(|| scale.chord_name(chord))
}

/// The font size, at most `size`, at which `text` fits in `width`: a
/// long chord name such as Cm(maj7)/Eb is written smaller instead of
/// running over the edges of its key.
fn fitting_size(ui: &Ui, text: &str, size: f32, width: f32) -> f32 {
    let wide = ui
        .painter()
        .layout_no_wrap(text.to_string(), FontId::proportional(size), KEY_TEXT)
        .size()
        .x;
    if wide <= width {
        size
    } else {
        (size * width / wide).max(10.0)
    }
}

/// A harmony key of a row or a set: its label (which notes of the row it
/// plays, or for a set the chord they make), the notes, and underneath
/// what the label does not already say (see [`group_footer`]).
fn group_key(ui: &mut Ui, group: &GroupKey, lit: bool) -> Response {
    let names: Vec<String> = group.names.iter().map(ToString::to_string).collect();
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
    if lit {
        paint_playing(painter, rect);
    }
    painter.text(
        rect.center_top() + vec2(0.0, 16.0),
        Align2::CENTER_CENTER,
        &group.label,
        FontId::proportional(fitting_size(ui, &group.label, 17.0, width - 8.0)),
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
        group_footer(group),
        FontId::proportional(10.0),
        KEY_TEXT_DIM,
    );
    response
}

/// A key of a micromode's form: its degree, its notes, and the
/// micromode's own name for the form (or the triad it is) at the bottom.
/// The more notes it keeps from home the brighter it is, so home and the
/// forms next to it stand out; `outlined` marks a smooth way on.
fn form_key(ui: &mut Ui, form: &GroupKey, scale: Scale, lit: bool, outlined: bool) -> Response {
    let (rect, response) = ui.allocate_exact_size(KEY_SIZE, Sense::click());
    ui.add_space(KEY_GAP - ui.spacing().item_spacing.x);
    if !ui.is_rect_visible(rect) {
        return response;
    }
    // From 0.45 for a form that keeps no note of home to 1 for home.
    let share = form.kept.unwrap_or(0) as f32 / form.notes.len().max(1) as f32;
    let near = 0.45 + 0.55 * share;
    let fill = if lit {
        GROUP_COLOR
    } else if response.hovered() {
        GROUP_COLOR.gamma_multiply((near + 0.2_f32).min(1.0))
    } else {
        GROUP_COLOR.gamma_multiply(near * 0.8)
    };
    let painter = ui.painter();
    painter.rect_filled(rect, 6.0, fill.gamma_multiply(0.55));
    let body = Rect::from_min_max(rect.min, rect.max - vec2(0.0, 3.0));
    painter.rect_filled(body, 6.0, fill);
    if lit {
        paint_playing(painter, rect);
    } else if outlined {
        let stroke = Stroke::new(1.5, SUGGESTED.gamma_multiply(0.8));
        painter.rect_stroke(rect, 6.0, stroke, StrokeKind::Inside);
    }
    let names: Vec<String> = form.names.iter().map(ToString::to_string).collect();
    painter.text(
        rect.center_top() + vec2(0.0, 16.0),
        Align2::CENTER_CENTER,
        &form.label,
        FontId::proportional(fitting_size(ui, &form.label, 18.0, rect.width() - 8.0)),
        KEY_TEXT,
    );
    let names = names.join(" ");
    painter.text(
        rect.center() + vec2(0.0, 9.0),
        Align2::CENTER_CENTER,
        &names,
        FontId::proportional(fitting_size(ui, &names, 12.0, rect.width() - 6.0)),
        KEY_TEXT,
    );
    let footer = form_footer(form, scale);
    painter.text(
        rect.center_bottom() - vec2(0.0, 9.0),
        Align2::CENTER_CENTER,
        &footer,
        FontId::proportional(fitting_size(ui, &footer, 10.0, rect.width() - 6.0)),
        KEY_TEXT_DIM,
    );
    response
}

/// What a form's key says under its notes: the chord or the scale it
/// is, or else the form's own name, as the chord sheet writes it, named
/// as the key `scale` is ("Menor 3 T4", or "3-3 T4" in a key of three
/// notes named as a set; "6-1 T2").
fn form_footer(form: &GroupKey, scale: Scale) -> String {
    match &form.chord {
        Some(chord) => chord.clone(),
        None => HarmonyName::form_in(form.pitch_set(), scale).to_string(),
    }
}

/// The tooltip of a micromode's key: "T2: E F G# · Menor 3 T4. Keeps 1
/// note of home." and, from the last form played, how it would go on to
/// this one: "From T0: keeps C# and E; C goes to D."
fn describe_form(
    form: &GroupKey,
    index: usize,
    level: HintLevel,
    scale: Scale,
    last: Option<PitchSet>,
) -> String {
    let names: Vec<String> = form.names.iter().map(ToString::to_string).collect();
    let set = form.pitch_set();
    let mut text = format!(
        "{}: {} · {}",
        form.label,
        names.join(" "),
        form_footer(form, scale)
    );
    let home = match form.kept {
        Some(kept) if kept == form.notes.len() => tr("the key's own form").to_string(),
        Some(1) => tr("keeps 1 note of home").to_string(),
        Some(kept) => trf("keeps {} notes of home", &[&kept]),
        None => String::new(),
    };
    if !home.is_empty() {
        text += &format!(", {home}");
    }
    text.push('.');
    if level == HintLevel::Minimal {
        return text;
    }
    if let Some(last) = last.filter(|&last| last != set)
        && let Some(from) = Degree::of(scale, last)
    {
        let mut link = degree::describe_link(last, set, |p| scale.spell(p).to_string());
        // Between triads, the neo-Riemannian name of the move.
        if let Some(triadic) = degree::Triadic::between(last, set) {
            link += &format!(" ({}, {})", triadic.letter(), tr(triadic.name()));
        }
        text += &format!(" {}", trf("From {}: {}.", &[&from, &link]));
    }
    if level == HintLevel::Detailed && index < 7 {
        text += &format!(
            " {}",
            trf(
                "Hold to play; key {} on the computer keyboard too.",
                &[&(index + 1)]
            )
        );
    }
    text
}

/// Frames the key whose chord is sounding now in thick white, so it
/// stands out above every other mark on the bar.
pub(crate) fn paint_playing(painter: &Painter, rect: Rect) {
    painter.rect_stroke(rect, 6.0, Stroke::new(3.0, PLAYING), StrokeKind::Inside);
}

/// The small line at the bottom of a group key, saying what its label
/// does not: "Eb · 3-11" for a row's group that makes a chord (the chord
/// and the set class), "3-11" for a set's key called by its chord, and
/// "[015]" (the prime form) for one called by its set class.
fn group_footer(group: &GroupKey) -> String {
    let class = group.pitch_set().set_class();
    if group.label == class.name() {
        return class.prime_text();
    }
    match &group.chord {
        Some(chord) if *chord != group.label => format!("{chord} · {}", class.name()),
        _ => class.name(),
    }
}

/// The tooltip of a key of a row or a set: "1–3: Eb G Bb, the chord Eb ·
/// set 3-11, …" for a row, "Cm/Eb: Eb G C · set 3-11, …" for a set.
fn describe_group(group: &GroupKey, index: usize, level: HintLevel) -> String {
    let names: Vec<String> = group.names.iter().map(ToString::to_string).collect();
    let class = group.pitch_set().set_class();
    let chord = match &group.chord {
        Some(chord) if *chord != group.label => trf(", the chord {}", &[chord]),
        _ => String::new(),
    };
    let mut text = trf(
        "{}: {}{} · set {}",
        &[&group.label, &names.join(" "), &chord, &class.name()],
    );
    if let Some(nickname) = class.nickname() {
        text += &format!(", {}", tr(nickname));
    }
    match level {
        HintLevel::Detailed => format!(
            "{text}.\n{}",
            trf(
                "Prime form {}, interval vector {}. Hold to play; key {} on the computer \
                 keyboard too.",
                &[&class.prime_text(), &class.vector_text(), &(index + 1)],
            )
        ),
        HintLevel::Short => format!("{text}. {}", trf("Key {}.", &[&(index + 1)])),
        HintLevel::Minimal => text,
    }
}

/// The tooltip of a harmony key, at `level`, with the notes spelled in
/// `scale` from the root up: "V: G major (G B D). The dominant: tension:
/// pulls back home to the tonic.", "V: G major (G B D), dominant:
/// tension. Key 5." or just "V: G major (G B D)". A mode has no
/// functions, so its keys say only the chord and the key to press.
fn describe_key(key: &ChordKey, scale: Scale, level: HintLevel) -> String {
    let notes: Vec<String> = scale
        .chord_notes(key.chord)
        .iter()
        .map(ToString::to_string)
        .collect();
    let digit = key.degree + 1;
    let chord = format!(
        "{}: {} {} ({})",
        key.numeral,
        scale.spell_root(key.chord),
        tr(key.chord.quality.name()),
        notes.join(" "),
    );
    let play = trf(
        "Hold to play; key {} on the computer keyboard too.",
        &[&digit],
    );
    match (level, key.function) {
        (HintLevel::Detailed, Some(function)) => format!(
            "{chord}. {}: {}.\n{play}",
            crate::i18n::tr_string(format!("The {}", function.name())),
            tr(function.feel()),
        ),
        (HintLevel::Detailed, None) => format!("{chord}.\n{play}"),
        (HintLevel::Short, Some(function)) => format!(
            "{chord}, {}: {}. {}",
            tr(function.name()),
            tr(function.short_feel()),
            trf("Key {}.", &[&digit]),
        ),
        (HintLevel::Short, None) => format!("{chord}. {}", trf("Key {}.", &[&digit])),
        (HintLevel::Minimal, _) => chord,
    }
}

/// A small keyboard: dots on the notes of the scale (orange on its root),
/// the notes held down lit, and the chord playing in the music shaded.
fn piano(ui: &mut Ui, scale: Scale, held: &[Note], heard: &[Note]) {
    // What is left of this row: in a wrapping row, `available_width`
    // would be the whole width, as if on a row of its own.
    let room = ui.max_rect().right() - ui.cursor().min.x;
    let width = |&(_, octaves, white): &(u8, u8, f32)| 7.0 * f32::from(octaves) * white;
    let (lowest, octaves, white_width) = PIANO_SIZES
        .iter()
        .copied()
        .find(|size| width(size) <= room)
        .unwrap_or(PIANO_SIZES[0]);
    let whites = 7 * usize::from(octaves);
    let size = vec2(whites as f32 * white_width, PIANO_HEIGHT);
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    if !ui.is_rect_visible(rect) {
        return;
    }
    let painter = ui.painter_at(rect.expand(1.0));
    let notes = lowest..lowest + 12 * octaves;
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
            let x = rect.left() + white as f32 * white_width;
            let key = Rect::from_center_size(
                pos2(x, rect.top() + PIANO_HEIGHT * 0.31),
                vec2(white_width * 0.62, PIANO_HEIGHT * 0.62),
            );
            black_keys.push((midi, key));
            continue;
        }
        let key = Rect::from_min_size(
            pos2(rect.left() + white as f32 * white_width, rect.top()),
            vec2(white_width, PIANO_HEIGHT),
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
    // The name of each C, so you can find your place, where the keys are
    // wide enough for it.
    let named = if white_width >= 8.0 { octaves } else { 0 };
    for octave in 0..named {
        let x = rect.left() + (f32::from(octave) * 7.0 + 0.5) * white_width;
        let c = Note::new(lowest + 12 * octave);
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
    use lineup_core::ChordQuality;
    use lineup_core::harmony::next_chords;

    #[test]
    fn after_the_tonic_six_four_the_hint_goes_on_to_v() {
        let c = Scale::new(PitchClass::C, ScaleKind::Major);
        let c_major = Chord::new(PitchClass::C, ChordQuality::Major);
        let next = next_chords_over(c, c_major, PitchClass::G);
        let last = Some((c_major, PitchClass::G));
        let line = hint_line(HintLevel::Short, c, last, &next, true, false, "Piano");
        assert_eq!(line, "C/G → G: cadential six-four, on to V");
    }

    #[test]
    fn the_hint_line_gets_shorter_by_level() {
        let c = Scale::new(PitchClass::C, ScaleKind::Major);
        let f = Chord::new(PitchClass::F, ChordQuality::Major);
        let next = next_chords(c, f);
        let line = |level| hint_line(level, c, Some((f, f.root)), &next, true, false, "Piano");
        assert_eq!(
            line(HintLevel::Detailed),
            "After F, G often comes next: IV to V: a step up to the dominant, ready to come home."
        );
        assert_eq!(line(HintLevel::Short), "F → G: step up to the dominant");
        assert_eq!(line(HintLevel::Minimal), "F → G");
    }

    #[test]
    fn the_hint_line_spells_chords_in_the_key() {
        let d_flat = Scale::new(PitchClass::CSharp, ScaleKind::Major);
        let iv = Chord::new(PitchClass::FSharp, ChordQuality::Major);
        let next = next_chords(d_flat, iv);
        let line = hint_line(
            HintLevel::Minimal,
            d_flat,
            Some((iv, iv.root)),
            &next,
            true,
            false,
            "Piano",
        );
        assert_eq!(line, "Gb → Ab");
    }

    #[test]
    fn suggestions_outline_only_the_keys_that_play_them() {
        // A melodic minor: after Am, Dm (iv) and E (V) often come next.
        // The key on D plays D major, so only the key of V is outlined.
        let a = Scale::new(PitchClass::A, ScaleKind::MelodicMinor);
        let am = Chord::new(PitchClass::A, ChordQuality::Minor);
        let next = next_chords(a, am);
        let chord_keys = keys::chord_keys(a, ChordSize::Triads);
        let ranks: Vec<Option<usize>> = (0..chord_keys.len())
            .map(|index| suggestion_rank(&next, &chord_keys, index))
            .collect();
        assert_eq!(ranks, [None, None, None, None, Some(1), None, None]);
    }

    #[test]
    fn a_key_lists_its_chord_from_the_root_up() {
        let c = Scale::new(PitchClass::C, ScaleKind::Major);
        let vii = &keys::chord_keys(c, ChordSize::Triads)[6];
        assert_eq!(
            describe_key(vii, c, HintLevel::Short),
            "vii°: B diminished (B D F), dominant: tension. Key 7."
        );
        let sevenths = keys::chord_keys(c, ChordSize::Sevenths);
        assert_eq!(
            describe_key(&sevenths[1], c, HintLevel::Minimal),
            "ii7: D minor seventh (D F A C)"
        );
        // Spelled in the key: Gb, not F#, in Db major.
        let d_flat = Scale::new(PitchClass::CSharp, ScaleKind::Major);
        let iv = &keys::chord_keys(d_flat, ChordSize::Triads)[3];
        assert_eq!(
            describe_key(iv, d_flat, HintLevel::Minimal),
            "IV: Gb major (Gb Bb Db)"
        );
    }

    #[test]
    fn a_mode_has_no_functions_to_describe() {
        let dorian = Scale::new(PitchClass::D, ScaleKind::Dorian);
        let iv = &keys::chord_keys(dorian, ChordSize::Triads)[3];
        assert_eq!(
            describe_key(iv, dorian, HintLevel::Short),
            "IV: G major (G B D). Key 4."
        );
        assert_eq!(function_color(iv.function), MODE_COLOR);
    }

    #[test]
    fn a_set_offers_no_chords_bigger_than_itself() {
        let keys = HarmonyKeys {
            size: ChordSize::Ninths,
            ..HarmonyKeys::new()
        };
        let triad = PitchSet::from_semitones(&[0, 3, 7]);
        let set = Scale::set(PitchClass::C, triad);
        assert_eq!(keys.size_for(set), ChordSize::Triads);
        let tetrachord = PitchSet::from_semitones(&[0, 3, 6, 9]);
        let set = Scale::set(PitchClass::C, tetrachord);
        assert_eq!(keys.size_for(set), ChordSize::Sevenths);
        let c = Scale::new(PitchClass::C, ScaleKind::Major);
        assert_eq!(keys.size_for(c), ChordSize::Ninths);
    }

    #[test]
    fn group_keys_say_what_their_label_does_not() {
        // A set bigger than a micromode stacks its notes into chords.
        let c7 = PitchSet::from_semitones(&[0, 4, 7, 10]);
        let groups = keys::group_keys(Scale::set(PitchClass::C, c7), ChordSize::Triads).unwrap();
        assert_eq!(group_footer(&groups[1]), "3-10");
        assert_eq!(
            describe_group(&groups[1], 1, HintLevel::Minimal),
            "Edim: E Bb G · set 3-10, diminished triad"
        );
        let chromatic = PitchSet::from_semitones(&[0, 1, 2, 3]);
        let groups =
            keys::group_keys(Scale::set(PitchClass::C, chromatic), ChordSize::Triads).unwrap();
        assert_eq!(groups[0].label, "3-1");
        assert_eq!(group_footer(&groups[0]), "[012]");
        // A row's group: the chord its notes make, and their set class.
        let row = lineup_core::harmony::ROW_PRESETS
            .iter()
            .find(|preset| preset.name == "Four triads")
            .unwrap()
            .row;
        let groups = keys::group_keys(Scale::row(row), ChordSize::Triads).unwrap();
        assert_eq!(group_footer(&groups[0]), "C · 3-11");
        assert_eq!(
            describe_group(&groups[0], 0, HintLevel::Minimal),
            "1–3: C E G, the chord C · set 3-11, major or minor triad"
        );
    }
}
