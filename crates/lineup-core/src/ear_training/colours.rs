//! Ear training beyond keys: the colour of three notes, the forms of a
//! set, the forms of a row.
//!
//! Music without a key has an ear of its own to learn. Its harmony is
//! heard as a colour, the sound of one set of notes, and its lines as
//! one shape moved, turned upside down or read backwards. Three
//! exercises train that ear:
//!
//! - **Micromodes**: three notes; say which of Kröpfl's twelve
//!   micromodes they are, or first only its family: minor (with a
//!   semitone), major (with a whole tone and no semitone) or a chord.
//! - **Forms**: a figure of three, four or five notes, then another
//!   form of the same set; say whether the second is the first moved or
//!   turned upside down. Moved, every step of a line stays the same;
//!   upside down, every step goes the other way. As chords, the steps
//!   from the bottom up come in the other order, and a major triad
//!   upside down is a minor one.
//! - **Rows**: a twelve-tone row, then one of its forms; say whether it
//!   is the row moved (P), upside down (I), backwards (R) or upside
//!   down and backwards (RI).
//!
//! Only sets that change when turned upside down are asked in Forms: a
//! diminished triad upside down is a diminished triad moved, so there
//! would be nothing to hear. A row whose forms coincide (a palindrome is
//! its own retrograde, moved) takes either answer.

use serde::{Deserialize, Serialize};

use super::{Choices, Cue, Direction};
use crate::harmony::{
    Chord, ChordQuality, Micromode, MicromodeFamily, PitchSet, ROW_PRESETS, Random, RowForm,
    RowLabel, RowPreset, SetClass, ToneRow, interval_name, interval_with_article,
};
use crate::i18n::{list, tr, trf};
use crate::{Note, NoteName, PitchClass};

/// Shuffles `items` in place: the Fisher-Yates shuffle.
fn shuffle<T>(items: &mut [T], random: &mut Random) {
    for i in (1..items.len()).rev() {
        items.swap(i, random.below(i + 1));
    }
}

/// The lowest note of `pitch_class` from `at_least` up.
fn from(pitch_class: PitchClass, at_least: u8) -> Note {
    let bottom = Note::new(at_least);
    Note::new(at_least + bottom.pitch_class().semitones_up_to(pitch_class))
}

/// The notes of `order` stacked from `lowest` up, each the first of its
/// pitch class above the one before: in the order of a set's normal
/// form, the set packed into less than an octave.
fn stack(order: &[PitchClass], lowest: Note) -> Vec<Note> {
    let mut notes = vec![lowest];
    for &pitch_class in &order[1..] {
        let last = notes[notes.len() - 1];
        let up = last.pitch_class().semitones_up_to(pitch_class);
        notes.push(last.transpose(i32::from(up)).unwrap_or(last));
    }
    notes
}

/// A line through the notes of `order` from `first`: each note the
/// nearest of its pitch class to the one before, so the line moves by
/// steps of a tritone or less, but kept within `reach` semitones of its
/// first note (at least 6, so one of the two choices always fits). A
/// tritone goes back towards the first note.
fn walk(order: &[PitchClass], first: Note, reach: u8) -> Vec<Note> {
    let (low, high) = (first.midi() - reach, first.midi() + reach);
    let middle = i32::from(first.midi());
    let mut notes = vec![first];
    for &pitch_class in &order[1..] {
        let last = notes[notes.len() - 1];
        let up = i32::from(last.pitch_class().semitones_up_to(pitch_class));
        let (near, far) = match up {
            6 if i32::from(last.midi()) > middle => (-6, 6),
            0..=6 => (up, up - 12),
            _ => (up - 12, up),
        };
        let fits = |step: i32| {
            last.transpose(step)
                .filter(|note| (low..=high).contains(&note.midi()))
        };
        notes.push(fits(near).or_else(|| fits(far)).unwrap_or(last));
    }
    notes
}

/// The notes moved `semitones` up (down if negative).
fn moved(notes: &[Note], semitones: i32) -> Vec<Note> {
    notes
        .iter()
        .map(|note| note.transpose(semitones).unwrap_or(*note))
        .collect()
}

/// The notes turned upside down around the point halfway between `a`
/// and `b`: each as far on the other side of it.
fn mirrored(notes: &[Note], a: Note, b: Note) -> Vec<Note> {
    let sum = i32::from(a.midi()) + i32::from(b.midi());
    notes
        .iter()
        .map(|note| Note::new((sum - i32::from(note.midi())).clamp(0, 127) as u8))
        .collect()
}

/// The pitch classes of `notes`.
fn set_of(notes: &[Note]) -> PitchSet {
    notes.iter().map(|note| note.pitch_class()).collect()
}

/// The chord `set` is, if it is a triad or a seventh chord of the chord
/// exercise: the only chords that turn into others upside down are
/// among them (a major triad becomes minor, a dominant seventh
/// half-diminished).
fn chord_of(set: PitchSet) -> Option<Chord> {
    set.normal_form().into_iter().find_map(|root| {
        ChordQuality::identify(set.relative_to(root))
            .filter(|quality| super::CHORD_QUALITIES.contains(quality))
            .map(|quality| Chord::new(root, quality))
    })
}

/// The notes with their octaves, spelled as the chord they make if they
/// make one (C E G, not C E G), and otherwise with the usual names, C#
/// Eb F# Ab Bb: "C4 C#4 E4".
fn names(notes: &[Note]) -> String {
    let chord = chord_of(set_of(notes));
    let name = |note: Note| {
        let pitch_class = note.pitch_class();
        let spelled = chord
            .and_then(|chord| chord.spell_note(chord.root_name(), pitch_class))
            .unwrap_or_else(|| NoteName::chromatic(pitch_class));
        note.name_as(spelled)
    };
    notes
        .iter()
        .map(|&note| name(note))
        .collect::<Vec<_>>()
        .join(" ")
}

/// A chord in words: "C major", "B half-diminished seventh".
fn chord_words(chord: Chord) -> String {
    format!("{} {}", chord.root_name(), tr(chord.quality.name()))
}

/// The steps from each note to the next, in words: "up a minor second",
/// "down a major third".
fn steps(notes: &[Note]) -> Vec<String> {
    notes
        .windows(2)
        .map(|pair| {
            let step = i32::from(pair[1].midi()) - i32::from(pair[0].midi());
            let name = interval_with_article(interval_name(step.unsigned_abs() as u8));
            if step > 0 {
                trf("up {}", &[&name])
            } else {
                trf("down {}", &[&name])
            }
        })
        .collect()
}

/// The intervals between neighbouring notes of a chord, from the bottom
/// up, in words: "a minor second and a minor third".
fn stacked(notes: &[Note]) -> String {
    let intervals: Vec<String> = notes
        .windows(2)
        .map(|pair| interval_with_article(interval_name(pair[1].midi() - pair[0].midi())))
        .collect();
    list(&intervals)
}

/// How far a form is moved, in words: "up 3 semitones".
fn distance(semitones: i32) -> String {
    match semitones {
        1 => tr("up a semitone").to_string(),
        -1 => tr("down a semitone").to_string(),
        n if n > 0 => trf("up {} semitones", &[&n]),
        n => trf("down {} semitones", &[&-n]),
    }
}

/// A number from `-5` to `6`, not 0 when `zero` is false.
fn shift(random: &mut Random, zero: bool) -> i32 {
    if zero {
        random.below(12) as i32 - 5
    } else {
        match random.below(11) as i32 - 5 {
            0 => 6,
            n => n,
        }
    }
}

/// Plays `notes` the way a question of these exercises plays a line:
/// one after another from `start`, the last held. Returns where it ends.
fn line(notes: &[Note], start: f64, gap: f64, cues: &mut Vec<Cue>) -> f64 {
    for (i, &note) in notes.iter().enumerate() {
        let last = i + 1 == notes.len();
        let length = if last { gap * 2.4 } else { gap * 1.1 };
        cues.push(Cue::piano(start + gap * i as f64, length, note, 82));
    }
    start + gap * notes.len().saturating_sub(1) as f64 + gap * 2.4
}

/// Plays `notes` together from `start`. Returns where they end.
fn block(notes: &[Note], start: f64, length: f64, cues: &mut Vec<Cue>) -> f64 {
    for &note in notes {
        cues.push(Cue::piano(start, length, note, 78));
    }
    start + length
}

// ----------------------------------------------------------------------
// Micromodes

/// How a micromode question is answered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum MicromodeAnswer {
    /// Its family: minor, major or a chord.
    #[default]
    Family,
    /// Which of the twelve.
    Micromode,
}

impl MicromodeAnswer {
    pub const ALL: [MicromodeAnswer; 2] = [MicromodeAnswer::Family, MicromodeAnswer::Micromode];

    pub fn name(self) -> &'static str {
        match self {
            MicromodeAnswer::Family => "Its family",
            MicromodeAnswer::Micromode => "The micromode",
        }
    }
}

/// Which micromodes are asked, and how.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MicromodeSettings {
    /// By place in [`Micromode::all`]: Menor 1 is 0.
    pub micromodes: Choices,
    pub answer: MicromodeAnswer,
    /// By place in [`Direction::ALL`].
    pub directions: Choices,
    /// Whether the notes may come in any order from the bottom, spread
    /// over up to two octaves, as music spreads them.
    pub spread: bool,
}

impl Default for MicromodeSettings {
    /// All twelve, answered by family, as a chord and going up, packed
    /// close together.
    fn default() -> Self {
        MicromodeSettings {
            micromodes: Choices::of(&(0..12).collect::<Vec<_>>()),
            answer: MicromodeAnswer::Family,
            directions: Choices::of(&[0, 2]),
            spread: false,
        }
    }
}

/// The lowest note of a micromode question: G3, so its three notes sit
/// around middle C.
const MICROMODE_LOWEST: u8 = 55;

/// Three notes to name the micromode of.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MicromodeQuestion {
    pub micromode: &'static Micromode,
    /// The form played: one of the micromode's 24 (or 12, or 4).
    pub form: PitchSet,
    /// The notes, lowest first.
    pub notes: Vec<Note>,
    pub direction: Direction,
}

impl MicromodeQuestion {
    /// A question from `seed`, asking one of the micromodes chosen.
    pub fn new(settings: &MicromodeSettings, seed: u64) -> Self {
        let mut random = Random(seed);
        let chosen = settings.micromodes.places(12);
        let micromode = &Micromode::all()[chosen[random.below(chosen.len())]];
        let members = micromode.members();
        let form = members[random.below(members.len())];
        let directions = settings.directions.places(Direction::ALL.len());
        let direction = Direction::ALL[directions[random.below(directions.len())]];
        let mut order = form.normal_form();
        if settings.spread {
            shuffle(&mut order, &mut random);
        }
        let notes = stack(&order, from(order[0], MICROMODE_LOWEST));
        MicromodeQuestion {
            micromode,
            form,
            notes,
            direction,
        }
    }

    /// Whether `family` is the right answer.
    pub fn is_family(&self, family: MicromodeFamily) -> bool {
        self.micromode.family == family
    }

    /// What to play for the question.
    pub fn cues(&self) -> Vec<Cue> {
        Self::play(&self.notes, self.direction)
    }

    /// What to play for `other` instead: its form from the same lowest
    /// note, packed, played the same way. To hear an answer given next
    /// to the right one.
    pub fn cues_as(&self, other: &Micromode) -> Vec<Cue> {
        let lowest = self.notes[0];
        let order = other.key(lowest.pitch_class()).pitch_set().normal_form();
        Self::play(&stack(&order, lowest), self.direction)
    }

    fn play(notes: &[Note], direction: Direction) -> Vec<Cue> {
        let mut cues = Vec::new();
        match direction {
            Direction::Up => {
                line(notes, 0.0, 0.5, &mut cues);
            }
            Direction::Down => {
                let down: Vec<Note> = notes.iter().rev().copied().collect();
                line(&down, 0.0, 0.5, &mut cues);
            }
            Direction::Together => {
                block(notes, 0.0, 2.0, &mut cues);
            }
        }
        cues
    }

    /// The right answer, explained: the micromode, its notes, its
    /// intervals, its family and where it is heard.
    pub fn explain(&self) -> String {
        let micromode = self.micromode;
        trf(
            "{}: {}, with {}. Family: {}. {} {}",
            &[
                &micromode.full_name(),
                &names(&self.notes),
                &micromode.intervals(),
                &tr(micromode.family.name()),
                &tr(micromode.family.rule()),
                &tr(micromode.about),
            ],
        )
    }
}

// ----------------------------------------------------------------------
// Forms

/// How the two forms of a Forms question are played.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Texture {
    /// One note after another, a figure.
    Line,
    /// All the notes at once.
    Chord,
}

impl Texture {
    pub const ALL: [Texture; 2] = [Texture::Line, Texture::Chord];

    pub fn name(self) -> &'static str {
        match self {
            Texture::Line => "As a line",
            Texture::Chord => "As a chord",
        }
    }
}

/// How many notes the sets of the Forms exercise have.
pub const FORM_SIZES: [u8; 3] = [3, 4, 5];

/// Which sets are asked in Forms, and how they are played.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FormSettings {
    /// By place in [`FORM_SIZES`].
    pub sizes: Choices,
    /// By place in [`Texture::ALL`].
    pub textures: Choices,
}

impl Default for FormSettings {
    /// Sets of three notes, the micromodes, as lines.
    fn default() -> Self {
        FormSettings {
            sizes: Choices::of(&[0]),
            textures: Choices::of(&[0]),
        }
    }
}

/// The set classes Forms asks: those of the sizes chosen that change
/// when turned upside down.
pub fn form_classes(sizes: Choices) -> Vec<&'static SetClass> {
    let sizes: Vec<u8> = sizes
        .places(FORM_SIZES.len())
        .into_iter()
        .map(|place| FORM_SIZES[place])
        .collect();
    SetClass::all()
        .iter()
        .filter(|class| sizes.contains(&class.cardinality) && !class.is_symmetric())
        .collect()
}

/// Where the forms of a Forms question are played: a line starts
/// between C#4 and C5 and stays within 8 semitones of its first note;
/// a chord is packed from between A3 and G#4 up.
const FORM_LINE_START: u8 = 61;
const FORM_REACH: u8 = 8;
const FORM_CHORD_LOWEST: u8 = 57;

/// Two forms of one set: is the second the first moved, or upside down?
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormQuestion {
    pub class: &'static SetClass,
    /// The first form as played: a figure, or a chord from the bottom.
    pub first: Vec<Note>,
    /// Whether the second form is the first turned upside down.
    pub upside_down: bool,
    /// How far the second form is moved after that, in semitones.
    pub moved: i32,
    pub texture: Texture,
}

impl FormQuestion {
    /// A question from `seed`, with a set of one of the sizes chosen.
    pub fn new(settings: &FormSettings, seed: u64) -> Self {
        let mut random = Random(seed);
        let classes = form_classes(settings.sizes);
        let class = classes[random.below(classes.len())];
        let textures = settings.textures.places(Texture::ALL.len());
        let texture = Texture::ALL[textures[random.below(textures.len())]];
        let form = match random.below(2) {
            0 => class.prime,
            _ => class.prime.invert(),
        }
        .transpose(random.below(12) as u8);
        let mut order = form.normal_form();
        let first = match texture {
            Texture::Line => {
                shuffle(&mut order, &mut random);
                walk(&order, from(order[0], FORM_LINE_START), FORM_REACH)
            }
            Texture::Chord => stack(&order, from(order[0], FORM_CHORD_LOWEST)),
        };
        let upside_down = random.below(2) == 1;
        let moved = shift(&mut random, upside_down);
        FormQuestion {
            class,
            first,
            upside_down,
            moved,
            texture,
        }
    }

    /// The second form: the first moved, or turned upside down and then
    /// moved. A line turns around its first note, so its steps go the
    /// other way; a chord around its middle, so it keeps its span.
    pub fn second(&self, upside_down: bool) -> Vec<Note> {
        let turned = match (upside_down, self.texture) {
            (false, _) => self.first.clone(),
            (true, Texture::Line) => mirrored(&self.first, self.first[0], self.first[0]),
            (true, Texture::Chord) => {
                let mut chord =
                    mirrored(&self.first, self.first[0], self.first[self.first.len() - 1]);
                chord.sort();
                chord
            }
        };
        moved(&turned, self.moved)
    }

    /// What to play for the question.
    pub fn cues(&self) -> Vec<Cue> {
        self.cues_as(self.upside_down)
    }

    /// The first form, then the second as the answer `upside_down`
    /// would have it.
    pub fn cues_as(&self, upside_down: bool) -> Vec<Cue> {
        let second = self.second(upside_down);
        let mut cues = Vec::new();
        match self.texture {
            Texture::Line => {
                let end = line(&self.first, 0.0, 0.42, &mut cues);
                line(&second, end + 0.3, 0.42, &mut cues);
            }
            Texture::Chord => {
                let end = block(&self.first, 0.0, 1.6, &mut cues);
                block(&second, end + 0.5, 1.6, &mut cues);
            }
        }
        cues
    }

    /// The set by name: "Menor 3 (minor 3)", or "4-27 (dominant seventh
    /// or half-diminished seventh chord)".
    fn set_name(&self) -> String {
        match Micromode::of(self.class.prime) {
            Some(micromode) => micromode.full_name(),
            None => match self.class.nickname() {
                Some(nickname) => format!("{} ({})", self.class.name(), tr(nickname)),
                None => self.class.name(),
            },
        }
    }

    /// The right answer, explained: the two forms, how their steps
    /// compare, and the chords they are, if they are chords.
    pub fn explain(&self) -> String {
        let second = self.second(self.upside_down);
        let (first_names, second_names) = (names(&self.first), names(&second));
        let mut text = match (self.upside_down, self.texture) {
            (false, Texture::Line) => trf(
                "Moved: {} became {}, {}. Every step stays the same: {}.",
                &[
                    &first_names,
                    &second_names,
                    &distance(self.moved),
                    &list(&steps(&self.first)),
                ],
            ),
            (true, Texture::Line) => trf(
                "Upside down: {} became {}. Every step goes the other way: {} became {}.",
                &[
                    &first_names,
                    &second_names,
                    &list(&steps(&self.first)),
                    &list(&steps(&second)),
                ],
            ),
            (false, Texture::Chord) => trf(
                "Moved: {} became {}, {}. From the bottom up, the intervals stay the same: {}.",
                &[
                    &first_names,
                    &second_names,
                    &distance(self.moved),
                    &stacked(&self.first),
                ],
            ),
            (true, Texture::Chord) => trf(
                "Upside down: {} became {}. From the bottom up, the intervals come in the \
                 other order: {} became {}.",
                &[
                    &first_names,
                    &second_names,
                    &stacked(&self.first),
                    &stacked(&second),
                ],
            ),
        };
        if let (Some(a), Some(b)) = (chord_of(set_of(&self.first)), chord_of(set_of(&second))) {
            text += " ";
            text += &trf(
                "As chords, {} became {}.",
                &[&chord_words(a), &chord_words(b)],
            );
        }
        text + " " + &trf("Both are forms of {}.", &[&self.set_name()])
    }
}

// ----------------------------------------------------------------------
// Rows

/// Where the rows of the Rows exercise come from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum RowSource {
    /// The rows of the Row window: Schoenberg's, Berg's, Webern's...
    #[default]
    Pieces,
    /// Any of the twelve notes in any order.
    Random,
}

impl RowSource {
    pub const ALL: [RowSource; 2] = [RowSource::Pieces, RowSource::Random];

    pub fn name(self) -> &'static str {
        match self {
            RowSource::Pieces => "From pieces",
            RowSource::Random => "Random",
        }
    }
}

/// Which forms of which rows are asked.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RowSettings {
    /// By place in [`RowForm::ALL`]: P, I, R and RI.
    pub forms: Choices,
    pub source: RowSource,
}

impl Default for RowSettings {
    fn default() -> Self {
        RowSettings {
            forms: Choices::of(&[0, 1, 2, 3]),
            source: RowSource::Pieces,
        }
    }
}

/// Where a row is played: from between B3 and A#4, within 9 semitones
/// of its first note.
const ROW_START: u8 = 59;
const ROW_REACH: u8 = 9;

/// How a form changes the row, in words.
fn form_words(form: RowForm) -> &'static str {
    match form {
        RowForm::Prime => "the row moved: every step stays the same",
        RowForm::Inversion => "the row upside down: every step goes the other way",
        RowForm::Retrograde => "the row backwards: from its last note to its first",
        RowForm::RetrogradeInversion => "the row upside down and backwards",
    }
}

/// A row, then one of its forms: which one?
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowQuestion {
    /// The row as played first: its form P, on the note it starts on.
    pub row: ToneRow,
    /// The piece it comes from, if it comes from one.
    pub preset: Option<&'static RowPreset>,
    /// The form played second.
    pub form: RowForm,
    /// How far the second form is moved from the row, in semitones,
    /// before it is turned or read backwards.
    pub moved: i32,
    /// The row's line, as played first.
    pub line: Vec<Note>,
}

impl RowQuestion {
    /// A question from `seed`, asking one of the forms chosen.
    pub fn new(settings: &RowSettings, seed: u64) -> Self {
        let mut random = Random(seed);
        let preset = match settings.source {
            RowSource::Pieces => Some(&ROW_PRESETS[random.below(ROW_PRESETS.len())]),
            RowSource::Random => None,
        };
        let row = preset.map_or_else(|| ToneRow::random(random.next()), |preset| preset.row);
        let row = row.starting_on(PitchClass::from_semitone(random.below(12) as u8));
        let forms = settings.forms.places(RowForm::ALL.len());
        let form = RowForm::ALL[forms[random.below(forms.len())]];
        let moved = shift(&mut random, form != RowForm::Prime);
        let line = walk(
            &row.pitch_classes(),
            from(row.first(), ROW_START),
            ROW_REACH,
        );
        RowQuestion {
            row,
            preset,
            form,
            moved,
            line,
        }
    }

    /// The line of `form`: the row's line moved, turned upside down
    /// around its first note, read backwards, or both, so the ear hears
    /// the same shape changed.
    pub fn line_of(&self, form: RowForm) -> Vec<Note> {
        let turned = match form {
            RowForm::Prime | RowForm::Retrograde => self.line.clone(),
            RowForm::Inversion | RowForm::RetrogradeInversion => {
                mirrored(&self.line, self.line[0], self.line[0])
            }
        };
        let mut line = moved(&turned, self.moved);
        if matches!(form, RowForm::Retrograde | RowForm::RetrogradeInversion) {
            line.reverse();
        }
        line
    }

    /// The label of the form played second: "RI7".
    pub fn label(&self) -> RowLabel {
        let start = self.line[0].transpose(self.moved).unwrap_or(self.line[0]);
        RowLabel::new(self.form, start.pitch_class().semitone())
    }

    /// The forms of the row with the notes played second, in order: the
    /// form asked, and any other whose notes are the same, as a
    /// palindrome's retrograde is a prime form.
    pub fn labels(&self) -> Vec<RowLabel> {
        let played = ToneRow::from_notes(self.line_of(self.form)).ok();
        let mut labels = vec![self.label()];
        for form in RowForm::ALL {
            for number in 0..12 {
                let label = RowLabel::new(form, number);
                if label != self.label() && played == Some(self.row.form(label)) {
                    labels.push(label);
                }
            }
        }
        labels
    }

    /// Whether `form` is a right answer.
    pub fn is_right(&self, form: RowForm) -> bool {
        self.labels().iter().any(|label| label.form == form)
    }

    /// What to play for the question.
    pub fn cues(&self) -> Vec<Cue> {
        self.cues_as(self.form)
    }

    /// The row, then `form` of it: to hear an answer given next to the
    /// right one.
    pub fn cues_as(&self, form: RowForm) -> Vec<Cue> {
        let mut cues = Vec::new();
        let end = line(&self.line, 0.0, 0.3, &mut cues);
        line(&self.line_of(form), end + 0.6, 0.3, &mut cues);
        cues
    }

    /// The right answer, explained: the form, what it does to the row,
    /// both lines, the other names of the same notes, and the piece.
    pub fn explain(&self) -> String {
        let label = self.label();
        let mut text = trf(
            "{}: {}. The row: {}. {}: {}.",
            &[
                &label,
                &tr(form_words(self.form)),
                &names(&self.line),
                &label,
                &names(&self.line_of(self.form)),
            ],
        );
        let others: Vec<String> = self.labels()[1..]
            .iter()
            .filter(|other| other.form != self.form)
            .map(ToString::to_string)
            .collect();
        if !others.is_empty() {
            text += " ";
            text += &trf(
                "The same notes are also {}, so that answer is right too.",
                &[&list(&others)],
            );
        }
        if let Some(preset) = self.preset {
            text += " ";
            text += &trf("The row of {}.", &[&tr(preset.source)]);
        }
        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The questions made from the first `n` seeds.
    fn seeds(n: u64) -> impl Iterator<Item = u64> {
        (0..n).map(|seed| seed.wrapping_mul(0x9E37_79B9))
    }

    fn note(name: &str) -> Note {
        let (pitch, rest) = NoteName::read(name).expect("a note name");
        let octave: i32 = rest.parse().expect("an octave");
        Note::new((i32::from(pitch.pitch_class().semitone()) + 12 * (octave + 1)) as u8)
    }

    #[test]
    fn a_micromode_question_plays_a_form_of_its_micromode() {
        let mut asked = std::collections::BTreeSet::new();
        for spread in [false, true] {
            let settings = MicromodeSettings {
                spread,
                ..MicromodeSettings::default()
            };
            for seed in seeds(3_000) {
                let question = MicromodeQuestion::new(&settings, seed);
                assert_eq!(set_of(&question.notes), question.form);
                assert_eq!(Micromode::of(question.form), Some(question.micromode));
                assert!(question.notes.windows(2).all(|pair| pair[0] < pair[1]));
                let span = question.notes[2].midi() - question.notes[0].midi();
                // Packed, less than an octave; spread, less than two.
                assert!(span < if spread { 24 } else { 12 }, "{:?}", question.notes);
                assert!((55..=66).contains(&question.notes[0].midi()));
                asked.insert(question.micromode.number);
            }
        }
        assert_eq!(asked.len(), 12);
        // Only the micromodes chosen are asked.
        let settings = MicromodeSettings {
            micromodes: Choices::of(&[2, 10]),
            ..MicromodeSettings::default()
        };
        for seed in seeds(500) {
            let number = MicromodeQuestion::new(&settings, seed).micromode.number;
            assert!(number == 3 || number == 11, "{number}");
        }
    }

    #[test]
    fn a_micromode_is_explained_with_its_family() {
        let question = MicromodeQuestion {
            micromode: Micromode::named("Menor 3").unwrap(),
            form: PitchSet::from_semitones(&[0, 1, 4]),
            notes: vec![note("C4"), note("C#4"), note("E4")],
            direction: Direction::Together,
        };
        assert!(question.is_family(MicromodeFamily::Minor));
        assert!(!question.is_family(MicromodeFamily::Chord));
        assert!(
            question.explain().starts_with(
                "Menor 3 (minor 3): C4 C#4 E4, with a minor second, a minor third and a \
                 major third. Family: Minor (menores). Each has a minor second: a semitone."
            ),
            "{}",
            question.explain()
        );
        // A chord micromode is spelled as the chord.
        let question = MicromodeQuestion {
            micromode: Micromode::named("Acorde menor-mayor").unwrap(),
            form: PitchSet::from_semitones(&[3, 6, 10]),
            notes: vec![note("D#4"), note("F#4"), note("A#4")],
            direction: Direction::Up,
        };
        assert!(
            question.explain().contains(": Eb4 Gb4 Bb4, with"),
            "{}",
            question.explain()
        );
        // The answer played instead starts on the same note.
        let cues = question.cues_as(Micromode::named("Menor 3").unwrap());
        let notes: Vec<Note> = cues.iter().map(|cue| cue.note).collect();
        assert_eq!(notes, vec![note("D#4"), note("E4"), note("G4")]);
    }

    #[test]
    fn forms_asks_only_sets_that_change_upside_down() {
        let names = |sizes: &[usize]| -> Vec<String> {
            form_classes(Choices::of(sizes))
                .iter()
                .map(|class| class.name())
                .collect()
        };
        // Of the twelve micromodes, the seven that change: Menor 2 to 5,
        // Mayor 2 and 3, and the minor and major triad.
        assert_eq!(
            names(&[0]),
            vec!["3-2", "3-3", "3-4", "3-5", "3-7", "3-8", "3-11"]
        );
        assert!(names(&[1]).contains(&"4-27".to_string()));
        assert!(!names(&[1]).contains(&"4-28".to_string()));
        assert!(
            form_classes(Choices::of(&[1, 2]))
                .iter()
                .all(|class| class.cardinality >= 4)
        );
    }

    #[test]
    fn the_second_form_is_the_first_moved_or_upside_down() {
        for (size, &notes) in FORM_SIZES.iter().enumerate() {
            for texture in 0..2 {
                let settings = FormSettings {
                    sizes: Choices::of(&[size]),
                    textures: Choices::of(&[texture]),
                };
                for seed in seeds(1_000) {
                    let question = FormQuestion::new(&settings, seed);
                    let first = set_of(&question.first);
                    let second = question.second(question.upside_down);
                    assert_eq!(first.len(), usize::from(notes));
                    assert_eq!(first.set_class(), question.class);
                    // Moved is moved and upside down is upside down: the
                    // set has no form that is both.
                    let (_, inverted) = first.moved_to(set_of(&second)).unwrap();
                    assert_eq!(inverted, question.upside_down);
                    let gaps = |notes: &[Note]| -> Vec<i32> {
                        notes
                            .windows(2)
                            .map(|pair| i32::from(pair[1].midi()) - i32::from(pair[0].midi()))
                            .collect()
                    };
                    match (question.texture, question.upside_down) {
                        (_, false) => assert_eq!(gaps(&second), gaps(&question.first)),
                        (Texture::Line, true) => {
                            let other: Vec<i32> =
                                gaps(&question.first).iter().map(|g| -g).collect();
                            assert_eq!(gaps(&second), other);
                        }
                        (Texture::Chord, true) => {
                            let mut other = gaps(&question.first);
                            other.reverse();
                            assert_eq!(gaps(&second), other);
                        }
                    }
                    if !question.upside_down {
                        assert_ne!(question.moved, 0);
                    }
                    for note in question.first.iter().chain(&second) {
                        assert!((45..=88).contains(&note.midi()), "{:?}", question);
                    }
                }
            }
        }
    }

    #[test]
    fn a_major_triad_upside_down_is_minor() {
        let question = FormQuestion {
            class: PitchSet::from_semitones(&[0, 4, 7]).set_class(),
            first: vec![note("C4"), note("E4"), note("G4")],
            upside_down: true,
            moved: 0,
            texture: Texture::Chord,
        };
        assert_eq!(
            question.second(true),
            vec![note("C4"), note("Eb4"), note("G4")]
        );
        assert_eq!(
            question.explain(),
            "Upside down: C4 E4 G4 became C4 Eb4 G4. From the bottom up, the intervals come \
             in the other order: a major third and a minor third became a minor third and a \
             major third. As chords, C major became C minor. Both are forms of Acorde \
             menor-mayor (minor and major triad)."
        );
        let line = FormQuestion {
            first: vec![note("C4"), note("C#4"), note("E4")],
            class: PitchSet::from_semitones(&[0, 1, 4]).set_class(),
            texture: Texture::Line,
            moved: 5,
            upside_down: false,
        };
        assert_eq!(
            line.explain(),
            "Moved: C4 C#4 E4 became F4 F#4 A4, up 5 semitones. Every step stays the same: up \
             a minor second and up a minor third. Both are forms of Menor 3 (minor 3)."
        );
        let turned = FormQuestion {
            upside_down: true,
            moved: 0,
            ..line
        };
        assert!(
            turned.explain().starts_with(
                "Upside down: C4 C#4 E4 became C4 B3 Ab3. Every step goes the other way: up a \
                 minor second and up a minor third became down a minor second and down a \
                 minor third."
            ),
            "{}",
            turned.explain()
        );
    }

    #[test]
    fn a_row_form_is_the_rows_line_changed() {
        for source in RowSource::ALL {
            let settings = RowSettings {
                source,
                ..RowSettings::default()
            };
            let mut forms = std::collections::BTreeSet::new();
            for seed in seeds(2_000) {
                let question = RowQuestion::new(&settings, seed);
                assert_eq!(
                    ToneRow::from_notes(question.line.clone()).unwrap(),
                    question.row
                );
                let played = question.line_of(question.form);
                assert_eq!(
                    ToneRow::from_notes(played.clone()).unwrap(),
                    question.row.form(question.label()),
                    "{question:?}"
                );
                assert!(question.is_right(question.form));
                // The row moves by steps, never more than 9 semitones
                // from its first note.
                for pair in question.line.windows(2) {
                    let step = i32::from(pair[1].midi()) - i32::from(pair[0].midi());
                    assert!((1..=11).contains(&step.abs()), "{question:?}");
                }
                let first = i32::from(question.line[0].midi());
                assert!(
                    question
                        .line
                        .iter()
                        .all(|note| (i32::from(note.midi()) - first).abs() <= 9)
                );
                for note in question.line.iter().chain(&played) {
                    assert!((40..=90).contains(&note.midi()), "{question:?}");
                }
                // P is never the row itself again.
                if question.form == RowForm::Prime {
                    assert_ne!(question.moved, 0);
                }
                forms.insert(question.form.letters());
            }
            assert_eq!(forms.len(), 4);
        }
    }

    #[test]
    fn a_palindrome_takes_both_answers() {
        // Webern's Symphony read backwards is the row a tritone higher.
        let preset = RowPreset::named("Webern's Symphony").unwrap();
        let settings = RowSettings {
            forms: Choices::of(&[2]),
            source: RowSource::Pieces,
        };
        let question = seeds(10_000)
            .map(|seed| RowQuestion::new(&settings, seed))
            .find(|question| question.preset == Some(preset))
            .unwrap();
        assert!(question.is_right(RowForm::Retrograde));
        assert!(question.is_right(RowForm::Prime));
        assert!(!question.is_right(RowForm::Inversion));
        assert!(
            question
                .explain()
                .contains("so that answer is right too. The row of Anton Webern, Symphony"),
            "{}",
            question.explain()
        );
        // Schoenberg's is no palindrome.
        let preset = RowPreset::named("Schoenberg's Suite").unwrap();
        let question = seeds(10_000)
            .map(|seed| RowQuestion::new(&settings, seed))
            .find(|question| question.preset == Some(preset))
            .unwrap();
        assert_eq!(question.labels().len(), 1);
        assert!(!question.is_right(RowForm::Prime));
    }
}
