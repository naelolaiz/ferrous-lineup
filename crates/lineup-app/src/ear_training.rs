//! The ear-training window: questions to answer by ear, an experiment.
//!
//! Three exercises sit on tabs at the top: intervals, chords and
//! progressions. Under them, what to ask: which intervals, which kinds
//! of chord, which chords of a key. Then the question: it plays as soon
//! as it is asked, Play again plays it as often as needed, and the
//! answers are big buttons. A progression is answered on harmony keys,
//! coloured as in the bar at the bottom of the window but with the chord
//! names hidden, one chord after another.
//!
//! After an answer the window says whether it was right and explains
//! it, and every answer button plays its own interval or chord, so the
//! one picked can be heard next to the right one.
//!
//! The questions and their explanations come from
//! `lineup_core::ear_training`; this module only draws, reads the mouse
//! and asks for sounds with [`EarAction`]s.

use std::time::{Instant, SystemTime, UNIX_EPOCH};

use eframe::egui::{self, Align2, Color32, FontId, RichText, Sense, Stroke, StrokeKind, Ui, vec2};
use lineup_core::ChordQuality;
use lineup_core::ear_training::{
    CHORD_QUALITIES, ChordPool, ChordQuestion, Cue, Direction, EarSettings, Exercise, Interval,
    IntervalQuestion, KeyChoice, PROGRESSION_LENGTHS, PROGRESSION_TEMPOS, Players,
    ProgressionQuestion, Score, duration, next_seed,
};
use lineup_core::harmony::keys::ChordKey;

use crate::harmony_keys::{KEY_TEXT, KEY_TEXT_DIM, function_color, paint_playing};
use crate::theme;
use crate::twelve_tone::Front;
use crate::widgets::{self, Opening};

/// The id of the window, to keep it in front.
pub(crate) fn window_id() -> egui::Id {
    egui::Id::unique("ear-training-window")
}

fn settings_id() -> egui::Id {
    egui::Id::unique("ear-training-settings")
}

/// Something the window wants to hear.
#[derive(Debug, Clone, PartialEq)]
pub enum EarAction {
    /// Play these notes now, in place of anything it played before.
    Play(Vec<Cue>),
    /// Stop what it is playing.
    Stop,
}

/// The question asked, and the answers given so far.
enum Asked {
    Interval {
        question: IntervalQuestion,
        answer: Option<Interval>,
    },
    Chord {
        question: ChordQuestion,
        answer: Option<ChordQuality>,
    },
    Progression {
        question: ProgressionQuestion,
        /// The degree given for each chord after the first, in order.
        answers: Vec<usize>,
    },
}

impl Asked {
    fn is_answered(&self) -> bool {
        match self {
            Asked::Interval { answer, .. } => answer.is_some(),
            Asked::Chord { answer, .. } => answer.is_some(),
            Asked::Progression { question, answers } => answers.len() + 1 >= question.degrees.len(),
        }
    }
}

/// How an answer button looks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mark {
    Plain,
    Right,
    Wrong,
}

impl Mark {
    fn of(this: bool, right: bool, answered: bool) -> Mark {
        match (answered, right, this) {
            (true, true, _) => Mark::Right,
            (true, false, true) => Mark::Wrong,
            _ => Mark::Plain,
        }
    }
}

/// Darker than the toolbar's green and red, so white text reads on them.
const RIGHT_FILL: Color32 = Color32::from_rgb(56, 132, 62);
const WRONG_FILL: Color32 = Color32::from_rgb(160, 52, 46);

/// A progression playing: when it started, how long a bar is, and how
/// long it lasts, in seconds.
struct Playing {
    since: Instant,
    bar: f64,
    length: f64,
}

/// The window.
pub struct EarTraining {
    pub open: bool,
    settings: EarSettings,
    /// The settings as last saved, to save them again when they change.
    saved: EarSettings,
    asked: Option<Asked>,
    /// One for each exercise, in the order of [`Exercise::ALL`].
    scores: [Score; 3],
    seed: u64,
    playing: Option<Playing>,
    was_open: bool,
    front: Front,
}

impl EarTraining {
    /// The window, closed, with the settings it had last time.
    pub fn new(ctx: &egui::Context) -> Self {
        let settings: EarSettings = ctx
            .data_mut(|data| data.get_persisted(settings_id()))
            .unwrap_or_default();
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(1, |time| time.as_nanos() as u64);
        EarTraining {
            open: false,
            saved: settings.clone(),
            settings,
            asked: None,
            scores: [Score::default(); 3],
            seed,
            playing: None,
            was_open: false,
            front: Front::default(),
        }
    }

    /// Draws the window, if open. Closing it stops its sound.
    pub fn show(&mut self, ctx: &egui::Context, area: egui::Rect) -> Vec<EarAction> {
        let mut actions = Vec::new();
        if self.open {
            self.front.keep(ctx, window_id());
            let mut open = true;
            let window = egui::Window::new(crate::i18n::tr("Ear training"))
                .id(window_id())
                .open(&mut open)
                .default_pos([300.0, 110.0])
                .constrain_to(area);
            // As tall as the exercise, up to the height of the playlist;
            // it scrolls past that.
            let size = vec2(640.0, area.height());
            widgets::open_at(ctx, window, window_id(), size, Opening::EveryTime).show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| self.contents(ui, &mut actions))
            });
            self.open = open;
        }
        if self.was_open && !self.open {
            actions.push(EarAction::Stop);
            self.playing = None;
        }
        self.was_open = self.open;
        if self.settings != self.saved {
            self.saved = self.settings.clone();
            let settings = self.saved.clone();
            ctx.data_mut(|data| data.insert_persisted(settings_id(), settings));
        }
        actions
    }

    fn score(&mut self) -> &mut Score {
        let place = Exercise::ALL
            .iter()
            .position(|&exercise| exercise == self.settings.exercise)
            .unwrap_or(0);
        &mut self.scores[place]
    }

    /// Plays `cues`; `progression` says how long its bars are, to light
    /// the chord playing.
    fn play(&mut self, cues: Vec<Cue>, bar: Option<f64>, actions: &mut Vec<EarAction>) {
        self.playing = bar.map(|bar| Playing {
            since: Instant::now(),
            bar,
            length: duration(&cues),
        });
        actions.push(EarAction::Play(cues));
    }

    /// Asks a new question of the exercise chosen, never the same as the
    /// last one, and plays it.
    fn ask(&mut self, actions: &mut Vec<EarAction>) {
        let settings = &self.settings;
        let mut next = None;
        for _ in 0..8 {
            self.seed = next_seed(self.seed);
            let seed = self.seed;
            let asked = match settings.exercise {
                Exercise::Intervals => Asked::Interval {
                    question: IntervalQuestion::new(&settings.intervals, seed),
                    answer: None,
                },
                Exercise::Chords => Asked::Chord {
                    question: ChordQuestion::new(&settings.chords, seed),
                    answer: None,
                },
                Exercise::Progressions => Asked::Progression {
                    question: ProgressionQuestion::new(&settings.progressions, seed),
                    answers: Vec::new(),
                },
            };
            let same = match (&asked, &self.asked) {
                (
                    Asked::Interval { question: a, .. },
                    Some(Asked::Interval { question: b, .. }),
                ) => a.interval == b.interval && a.low == b.low,
                (Asked::Chord { question: a, .. }, Some(Asked::Chord { question: b, .. })) => {
                    a.chord == b.chord
                }
                (
                    Asked::Progression { question: a, .. },
                    Some(Asked::Progression { question: b, .. }),
                ) => a.degrees == b.degrees,
                _ => false,
            };
            next = Some(asked);
            if !same {
                break;
            }
        }
        self.asked = next;
        self.replay(actions);
    }

    /// Plays the question again.
    fn replay(&mut self, actions: &mut Vec<EarAction>) {
        let progression = &self.settings.progressions;
        let (cues, bar) = match &self.asked {
            Some(Asked::Interval { question, .. }) => (question.cues(), None),
            Some(Asked::Chord { question, .. }) => (question.cues(), None),
            Some(Asked::Progression { question, .. }) => (
                question.cues(progression.players, progression.tempo),
                Some(ProgressionQuestion::bar_seconds(progression.tempo)),
            ),
            None => return,
        };
        self.play(cues, bar, actions);
    }

    fn contents(&mut self, ui: &mut Ui, actions: &mut Vec<EarAction>) {
        ui.horizontal(|ui| {
            for exercise in Exercise::ALL {
                let on = self.settings.exercise == exercise;
                if theme::light_button(ui, exercise.name(), on, None).clicked() && !on {
                    self.settings.exercise = exercise;
                    self.asked = None;
                    actions.push(EarAction::Stop);
                    self.playing = None;
                }
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.weak("Experimental");
            });
        });
        ui.add_space(4.0);

        let before = self.settings.clone();
        // A frame as wide as the window, so the rows of settings wrap
        // at its edge instead of widening it.
        egui::Frame::new()
            .fill(theme::GROUP)
            .stroke(Stroke::new(1.0, theme::OUTLINE))
            .corner_radius(4)
            .inner_margin(egui::Margin::symmetric(6, 4))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                match self.settings.exercise {
                    Exercise::Intervals => self.interval_settings(ui),
                    Exercise::Chords => self.chord_settings(ui),
                    Exercise::Progressions => self.progression_settings(ui),
                }
            });
        if self.changes_questions(&before) {
            self.asked = None;
        }
        ui.add_space(8.0);

        if self.asked.is_none() {
            self.start(ui, actions);
        } else {
            self.question(ui, actions);
        }

        ui.add_space(10.0);
        ui.separator();
        let score = *self.score();
        ui.horizontal(|ui| {
            let text = match score.asked {
                0 => "Nothing answered yet".to_string(),
                asked => format!(
                    "Right {} of {asked} ({}%) · {} in a row",
                    score.right,
                    score.right * 100 / asked,
                    score.streak
                ),
            };
            ui.label(RichText::new(text).color(theme::TEXT_DIM));
            if score.asked > 0 && ui.small_button("Start again").clicked() {
                *self.score() = Score::default();
            }
        });
    }

    /// Whether a change of settings makes the question asked out of
    /// place: what is asked changed, not only how it is played.
    fn changes_questions(&self, before: &EarSettings) -> bool {
        let (a, b) = (&self.settings, before);
        match a.exercise {
            Exercise::Intervals => a.intervals != b.intervals,
            Exercise::Chords => a.chords != b.chords,
            Exercise::Progressions => {
                let (a, b) = (&a.progressions, &b.progressions);
                a.keys != b.keys
                    || a.chords != b.chords
                    || a.sevenths != b.sevenths
                    || a.length != b.length
            }
        }
    }

    fn interval_settings(&mut self, ui: &mut Ui) {
        let settings = &mut self.settings.intervals;
        ui.horizontal_wrapped(|ui| {
            ui.label("Intervals");
            for interval in Interval::ALL {
                let on = settings.intervals.has(interval.place());
                let chip = ui.add(egui::Button::selectable(on, interval.short()));
                if theme::hint(chip, interval.name()).clicked() {
                    settings.intervals.toggle(interval.place());
                }
            }
        });
        ui.horizontal_wrapped(|ui| {
            ui.label("Played");
            for (place, direction) in Direction::ALL.into_iter().enumerate() {
                let on = settings.directions.has(place);
                let text = match direction {
                    Direction::Up => "the lower note first, going up",
                    Direction::Down => "the higher note first, going down",
                    Direction::Together => "both notes at once",
                };
                let chip = ui.add(egui::Button::selectable(on, direction.name()));
                if theme::hint(chip, text).clicked() {
                    settings.directions.toggle(place);
                }
            }
        });
    }

    fn chord_settings(&mut self, ui: &mut Ui) {
        let settings = &mut self.settings.chords;
        ui.horizontal_wrapped(|ui| {
            ui.label("Chords");
            for (place, quality) in CHORD_QUALITIES.into_iter().enumerate() {
                let on = settings.qualities.has(place);
                let chip = ui.add(egui::Button::selectable(on, short_name(quality)));
                if theme::hint(chip, quality.name()).clicked() {
                    settings.qualities.toggle(place);
                }
            }
        });
        let inversions = ui.checkbox(&mut settings.inversions, "Inversions");
        theme::hint(
            inversions,
            "Any note of the chord in the bass, not only the root: C/E is C major with E lowest",
        );
    }

    fn progression_settings(&mut self, ui: &mut Ui) {
        let settings = &mut self.settings.progressions;
        ui.horizontal_wrapped(|ui| {
            ui.label("Keys");
            for keys in KeyChoice::ALL {
                ui.selectable_value(&mut settings.keys, keys, keys.name());
            }
            ui.separator();
            ui.label("Chords");
            for pool in ChordPool::ALL {
                let chip = ui.selectable_value(&mut settings.chords, pool, pool.name());
                theme::hint(
                    chip,
                    match pool {
                        ChordPool::Three => {
                            "Home, away and tension: I, IV and V (i, iv and V in minor)"
                        }
                        ChordPool::Five => "Those and ii and vi (ii° and bVI in minor)",
                        ChordPool::Seven => "Every chord of the key",
                    },
                );
            }
            let sevenths = ui.checkbox(&mut settings.sevenths, "Sevenths");
            theme::hint(sevenths, "Seventh chords: V7, ii7, Imaj7...");
        });
        ui.horizontal_wrapped(|ui| {
            ui.label("Players");
            for players in Players::ALL {
                ui.selectable_value(&mut settings.players, players, players.name());
            }
            ui.separator();
            ui.label("Length");
            for length in PROGRESSION_LENGTHS {
                ui.selectable_value(&mut settings.length, length, format!("{length} chords"));
            }
        });
        ui.horizontal(|ui| {
            ui.label("Tempo");
            ui.add(egui::Slider::new(&mut settings.tempo, PROGRESSION_TEMPOS).suffix(" BPM"));
        });
    }

    /// Before the first question: what to do, and a button to begin.
    fn start(&mut self, ui: &mut Ui, actions: &mut Vec<EarAction>) {
        ui.vertical_centered(|ui| {
            ui.add_space(6.0);
            let text = match self.settings.exercise {
                Exercise::Intervals => "Two notes play. How far apart are they?",
                Exercise::Chords => "A chord plays. What kind of chord is it?",
                Exercise::Progressions => {
                    "The band plays a few chords in a key, starting at home (I, or i in \
                     a minor key). What are the others?"
                }
            };
            ui.label(RichText::new(text).size(15.0));
            ui.add_space(8.0);
            let start = egui::Button::new(RichText::new("▶  Start").size(16.0).strong())
                .min_size(vec2(140.0, 36.0));
            if ui.add(start).clicked() {
                self.ask(actions);
            }
            ui.add_space(4.0);
            ui.weak("Play a question again as often as you like before answering.");
        });
    }

    fn question(&mut self, ui: &mut Ui, actions: &mut Vec<EarAction>) {
        let answered = self.asked.as_ref().is_some_and(Asked::is_answered);
        ui.horizontal(|ui| {
            let prompt = match &self.asked {
                Some(Asked::Interval { .. }) => "What interval is this?".to_string(),
                Some(Asked::Chord { .. }) => "What kind of chord is this?".to_string(),
                Some(Asked::Progression { question, .. }) => format!(
                    "In {}: what are the chords after {}?",
                    question.scale.name(),
                    question.keys[0].numeral
                ),
                None => String::new(),
            };
            ui.label(RichText::new(prompt).size(16.0).strong());
        });
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            if theme::light_button(ui, "▶ Play again", false, None).clicked() {
                self.replay(actions);
            }
            match &self.asked {
                Some(Asked::Chord { question, .. }) => {
                    let broken = theme::light_button(ui, "Note by note", false, None);
                    if theme::hint(broken, "The chord one note at a time, from the bass up")
                        .clicked()
                    {
                        let cues = question.broken();
                        self.play(cues, None, actions);
                    }
                }
                Some(Asked::Progression { question, .. }) => {
                    let key = theme::light_button(ui, "Hear the key", false, None);
                    // I, IV, V and I; in a minor key i, iv, V and i.
                    let [home, four, five] = [0, 3, 4].map(|degree| &question.keys[degree].numeral);
                    let hint = format!("{home}, {four}, {five} and {home}: where home is");
                    if theme::hint(key, hint).clicked() {
                        let cues = question.key_cues();
                        self.play(cues, None, actions);
                    }
                }
                _ => {}
            }
            let next = if answered { "Next ▸" } else { "Skip ▸" };
            let next = theme::light_button(ui, next, answered, Some(theme::GREEN));
            if next.clicked() {
                self.ask(actions);
            }
        });
        ui.add_space(8.0);

        match self.asked.take() {
            Some(Asked::Interval {
                question,
                mut answer,
            }) => {
                self.interval_answers(ui, &question, &mut answer, actions);
                self.asked = Some(Asked::Interval { question, answer });
            }
            Some(Asked::Chord {
                question,
                mut answer,
            }) => {
                self.chord_answers(ui, &question, &mut answer, actions);
                self.asked = Some(Asked::Chord { question, answer });
            }
            Some(Asked::Progression {
                question,
                mut answers,
            }) => {
                self.progression_answers(ui, &question, &mut answers, actions);
                self.asked = Some(Asked::Progression { question, answers });
            }
            None => {}
        }
    }

    fn interval_answers(
        &mut self,
        ui: &mut Ui,
        question: &IntervalQuestion,
        answer: &mut Option<Interval>,
        actions: &mut Vec<EarAction>,
    ) {
        let chosen = self.settings.intervals.intervals;
        ui.horizontal_wrapped(|ui| {
            for interval in Interval::ALL {
                if !chosen.has(interval.place()) && interval != question.interval {
                    continue;
                }
                let mark = Mark::of(
                    *answer == Some(interval),
                    interval == question.interval,
                    answer.is_some(),
                );
                let button = answer_button(ui, interval.short(), interval.name(), mark);
                if button.clicked() {
                    if answer.is_none() {
                        *answer = Some(interval);
                        self.score().add(interval == question.interval);
                    } else {
                        self.play(question.cues_for(interval), None, actions);
                    }
                }
            }
        });
        if let Some(given) = *answer {
            let wrong = (given != question.interval).then(|| format!("You said {}.", given.name()));
            feedback(ui, given == question.interval, wrong, &question.explain());
        }
    }

    fn chord_answers(
        &mut self,
        ui: &mut Ui,
        question: &ChordQuestion,
        answer: &mut Option<ChordQuality>,
        actions: &mut Vec<EarAction>,
    ) {
        let chosen = self.settings.chords.qualities;
        let right = question.chord.quality;
        ui.horizontal_wrapped(|ui| {
            for (place, quality) in CHORD_QUALITIES.into_iter().enumerate() {
                if !chosen.has(place) && quality != right {
                    continue;
                }
                let mark = Mark::of(*answer == Some(quality), quality == right, answer.is_some());
                let button = answer_button(ui, short_name(quality), quality.name(), mark);
                if button.clicked() {
                    if answer.is_none() {
                        *answer = Some(quality);
                        self.score().add(quality == right);
                    } else {
                        self.play(question.cues_as(quality), None, actions);
                    }
                }
            }
        });
        if let Some(given) = *answer {
            let wrong = (given != right).then(|| format!("You said {}.", given.name()));
            feedback(ui, given == right, wrong, &question.explain());
        }
    }

    fn progression_answers(
        &mut self,
        ui: &mut Ui,
        question: &ProgressionQuestion,
        answers: &mut Vec<usize>,
        actions: &mut Vec<EarAction>,
    ) {
        let done = answers.len() + 1 >= question.degrees.len();
        // The chord sounding now, while the progression plays.
        let now = match &self.playing {
            Some(playing) => {
                let elapsed = playing.since.elapsed().as_secs_f64();
                if elapsed < playing.length {
                    ui.ctx().request_repaint();
                    Some((elapsed / playing.bar) as usize)
                } else {
                    None
                }
            }
            None => None,
        };

        ui.horizontal_wrapped(|ui| {
            for place in 0..question.degrees.len() {
                let right = question.chord(place);
                let given = match place {
                    0 => Some(0),
                    _ => answers.get(place - 1).copied(),
                };
                let slot = Slot {
                    numeral: given.map(|degree| question.keys[degree].numeral.as_str()),
                    right: &right.numeral,
                    first: place == 0,
                    correct: given == Some(question.degrees[place]),
                    next: place == answers.len() + 1,
                    playing: now == Some(place),
                };
                if slot.show(ui).clicked() && done {
                    let cues = question.chord_cues(question.degrees[place]);
                    self.play(cues, None, actions);
                }
            }
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            for key in &question.keys {
                let usable = question.pool.contains(&key.degree);
                if numeral_key(ui, key, usable || done).clicked() {
                    if done {
                        self.play(question.chord_cues(key.degree), None, actions);
                    } else if usable {
                        let place = answers.len() + 1;
                        answers.push(key.degree);
                        self.score().add(question.degrees[place] == key.degree);
                    }
                }
            }
        });
        if done {
            let wrong = answers
                .iter()
                .zip(&question.degrees[1..])
                .filter(|(given, right)| given != right)
                .count();
            let names = question.chord_names().join(" – ");
            let numerals: Vec<&str> = (0..question.degrees.len())
                .map(|place| question.chord(place).numeral.as_str())
                .collect();
            let mut text = format!(
                "{names}: {} in {}.",
                numerals.join(" "),
                question.scale.name()
            );
            for line in question.moves() {
                text += "\n• ";
                text += &line;
            }
            let summary = match wrong {
                0 => None,
                1 => Some("One chord was not what you said.".to_string()),
                n => Some(format!("{n} chords were not what you said.")),
            };
            feedback(ui, wrong == 0, summary, &text);
        } else {
            ui.add_space(4.0);
            ui.weak("Pick the chords in order. Colours: blue is home, green away, orange tension.");
        }
    }
}

/// The short name on a chord button: "maj", "m7b5".
fn short_name(quality: ChordQuality) -> &'static str {
    match quality {
        ChordQuality::Major => "maj",
        other => other.symbol(),
    }
}

/// A big answer button: the short name, and the full name under it.
fn answer_button(ui: &mut Ui, short: &str, name: &str, mark: Mark) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(vec2(100.0, 52.0), Sense::click());
    if !ui.is_rect_visible(rect) {
        return response;
    }
    let fill = match mark {
        Mark::Right => RIGHT_FILL,
        Mark::Wrong => WRONG_FILL,
        Mark::Plain if response.is_pointer_button_down_on() => theme::BUTTON_PRESSED,
        Mark::Plain if response.hovered() => theme::BUTTON_HOVER,
        Mark::Plain => theme::BUTTON,
    };
    let painter = ui.painter();
    painter.rect_filled(rect, 6.0, fill);
    painter.rect_stroke(
        rect,
        6.0,
        Stroke::new(1.0, theme::OUTLINE),
        StrokeKind::Inside,
    );
    painter.text(
        rect.center_top() + vec2(0.0, 19.0),
        Align2::CENTER_CENTER,
        short,
        FontId::proportional(19.0),
        KEY_TEXT,
    );
    // A long name ("half-diminished seventh") is written smaller, to
    // fit in the button.
    let wide = painter
        .layout_no_wrap(name.to_string(), FontId::proportional(10.5), KEY_TEXT_DIM)
        .size()
        .x;
    let size = (10.5 * (rect.width() - 8.0) / wide).clamp(8.0, 10.5);
    painter.text(
        rect.center_bottom() - vec2(0.0, 11.0),
        Align2::CENTER_CENTER,
        name,
        FontId::proportional(size),
        KEY_TEXT_DIM,
    );
    response
}

/// One chord of a progression, as answered so far.
struct Slot<'a> {
    /// The numeral given, if any (the first chord is given: I).
    numeral: Option<&'a str>,
    right: &'a str,
    first: bool,
    correct: bool,
    /// Whether it is the one to answer next.
    next: bool,
    /// Whether it is sounding now.
    playing: bool,
}

impl Slot<'_> {
    fn show(&self, ui: &mut Ui) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(vec2(70.0, 56.0), Sense::click());
        if !ui.is_rect_visible(rect) {
            return response;
        }
        let fill = match self.numeral {
            Some(_) if self.first => theme::GROUP,
            Some(_) if self.correct => RIGHT_FILL,
            Some(_) => WRONG_FILL,
            None => theme::LCD,
        };
        let painter = ui.painter();
        painter.rect_filled(rect, 6.0, fill);
        let edge = if self.next {
            Stroke::new(2.0, theme::ACCENT)
        } else {
            Stroke::new(1.0, theme::OUTLINE)
        };
        painter.rect_stroke(rect, 6.0, edge, StrokeKind::Inside);
        if self.playing {
            paint_playing(painter, rect);
        }
        let (text, color) = match self.numeral {
            Some(numeral) => (numeral, KEY_TEXT),
            None => ("?", theme::TEXT_DIM),
        };
        let centre = if self.numeral.is_some() && !self.correct && !self.first {
            rect.center() - vec2(0.0, 8.0)
        } else {
            rect.center()
        };
        painter.text(
            centre,
            Align2::CENTER_CENTER,
            text,
            FontId::proportional(20.0),
            color,
        );
        if self.numeral.is_some() && !self.correct && !self.first {
            painter.text(
                rect.center_bottom() - vec2(0.0, 11.0),
                Align2::CENTER_CENTER,
                format!("was {}", self.right),
                FontId::proportional(11.0),
                KEY_TEXT,
            );
        }
        response
    }
}

/// A harmony key with its numeral and function but not its chord: the
/// answers of a progression.
fn numeral_key(ui: &mut Ui, key: &ChordKey, enabled: bool) -> egui::Response {
    let sense = if enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, response) = ui.allocate_exact_size(vec2(66.0, 56.0), sense);
    if !ui.is_rect_visible(rect) {
        return response;
    }
    let color = function_color(key.function);
    let fill = if !enabled {
        color.gamma_multiply(0.18)
    } else if response.is_pointer_button_down_on() {
        color
    } else if response.hovered() {
        color.gamma_multiply(0.75)
    } else {
        color.gamma_multiply(0.5)
    };
    let text = if enabled {
        KEY_TEXT
    } else {
        KEY_TEXT_DIM.gamma_multiply(0.5)
    };
    let painter = ui.painter();
    painter.rect_filled(rect, 6.0, fill);
    painter.text(
        rect.center() - vec2(0.0, 6.0),
        Align2::CENTER_CENTER,
        &key.numeral,
        FontId::proportional(20.0),
        text,
    );
    if let Some(function) = key.function {
        painter.text(
            rect.center_bottom() - vec2(0.0, 10.0),
            Align2::CENTER_CENTER,
            function.short_feel(),
            FontId::proportional(10.0),
            if enabled { KEY_TEXT_DIM } else { text },
        );
    }
    response
}

/// Right or not, what was said if it was not, and the explanation.
fn feedback(ui: &mut Ui, right: bool, wrong: Option<String>, explanation: &str) {
    ui.add_space(8.0);
    egui::Frame::new()
        .fill(theme::GROUP)
        .stroke(Stroke::new(1.0, theme::OUTLINE))
        .corner_radius(6)
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            let (title, color) = if right {
                ("✔ Right", theme::GREEN)
            } else {
                ("✘ Not quite", theme::RED)
            };
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(title).size(16.0).strong().color(color));
                if let Some(wrong) = wrong {
                    ui.label(RichText::new(wrong).color(theme::TEXT));
                }
            });
            ui.add_space(2.0);
            ui.label(RichText::new(explanation).size(14.0).color(theme::TEXT));
            ui.add_space(4.0);
            ui.weak("Click any answer to hear it.");
        });
}
