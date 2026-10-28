//! The chord sheet: the harmony of a song, written bar by bar as on a
//! lead sheet.
//!
//! A [`ChordMarker`] says "from this step on, the harmony is this", and
//! lasts until the next one, the way a chord on a lead sheet lasts until
//! the next chord is written. A bar with no marker goes on with the
//! harmony before it. The markers live in the playlist with the time,
//! tempo and key markers, so they move with the song when a section is
//! copied or the song is put into another time signature.
//!
//! A bar can name its harmony in six ways ([`HarmonyName`]):
//!
//! | Written | What it is |
//! |---------|------------|
//! | `Am7`, `G/B`, `F#m7b5` | a chord symbol ([`ChordSymbol`]) |
//! | `Eb7\|Fb`, `C\|F#` | two chords at once, the upper one first: a polychord |
//! | `3-11 T2`, `4-Z15`, `{0,1,4}` | a pitch-class set, by Forte name or by its notes |
//! | `Menor 3 T2I`, `Major 1` | one of Kröpfl's micromodes |
//! | `P0`, `I5`, `R3`, `RI7` | a form of the twelve-tone row of the key |
//! | `N.C.` | no chord |
//!
//! `Tn` moves a set or a micromode up `n` semitones from its prime form,
//! and `TnI` turns it upside down first, as in the Sets and Micromodes
//! windows. In a key of seven notes, Roman numerals can be typed too:
//! "ii7 V7 I" in C major is written down as Dm7, G7 and C.
//!
//! Marks around a harmony tell the band to leave its style there, the
//! way arrangers mark a chart: dots after it make a [`Stop`] (`C.` a
//! shot, `C..` a hold, `C...` a rest), and a caret before it a
//! [`Push`] (`^C` an eighth early, `^^C` a sixteenth).

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::harmony::{
    Chord, ChordQuality, ChordSymbol, Micromode, MinorNumerals, PitchSet, RowLabel, Scale,
    ScaleKind, SetClass, describe_chord, pc_digit,
};
use crate::library::numeral;
use crate::meter::{Bar, TimeSignature};
use crate::pitch::{Note, NoteName, NoteNaming, PitchClass};
use crate::tools;

pub mod function;
pub mod notation;
pub mod text;

/// The harmony of a bar, or of part of one, named in one of the ways
/// the app knows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(into = "String", try_from = "String")]
pub enum HarmonyName {
    /// A chord symbol: "Am7", "G/B".
    Chord(ChordSymbol),
    /// Two chords at once, one over the other, the upper one first:
    /// "Eb7|Fb" is E♭7 over F♭ major, the chord that opens the Augurs of
    /// Spring. Lead sheets draw the two chords as a fraction, with a
    /// line between them; a slash would mean a bass note.
    Polychord(ChordSymbol, ChordSymbol),
    /// No chord, written "N.C.": nothing plays the harmony here.
    NoChord,
    /// A pitch-class set on these very notes, named by its Forte number
    /// and how it is moved from the prime form: "3-11 T2" is D F A.
    Set(PitchSet),
    /// One of Kröpfl's micromodes on these three notes: "Menor 3 T2".
    Micromode(PitchSet),
    /// A form of the twelve-tone row of the key here: "RI7".
    Row(RowLabel),
}

/// A harmony that starts on a step of the song and lasts until the next.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChordMarker {
    /// The step it starts on, counted from the start of the song.
    pub start: usize,
    pub harmony: HarmonyName,
    /// Whether the band plays it early.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push: Option<Push>,
    /// Whether the band stops playing its style here.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop: Option<Stop>,
}

impl ChordMarker {
    pub fn new(start: usize, harmony: HarmonyName) -> Self {
        ChordMarker {
            start,
            harmony,
            push: None,
            stop: None,
        }
    }

    /// The same marker, played early.
    pub fn pushed(self, push: Option<Push>) -> Self {
        ChordMarker { push, ..self }
    }

    /// The same marker, where the band stops as `stop` says.
    pub fn stopped(self, stop: Option<Stop>) -> Self {
        ChordMarker { stop, ..self }
    }

    /// The step the band plays it from: its own, or a little earlier
    /// when it is pushed.
    pub fn played_start(&self) -> usize {
        self.start.saturating_sub(self.push.map_or(0, Push::steps))
    }

    /// The harmony with its marks, as it is typed: "^C..".
    pub fn text(&self) -> String {
        let carets = self.push.map_or("", Push::marks);
        let dots = self.stop.map_or("", Stop::marks);
        format!("{carets}{}{dots}", self.harmony)
    }
}

/// How the band stops playing its style at a harmony, until the next
/// one. Written with dots after the harmony.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stop {
    /// "C.": the whole band strikes the chord once, short, with a kick
    /// and a crash cymbal, and then is silent.
    Shot,
    /// "C..": the band strikes the chord and holds it.
    Hold,
    /// "C...": the band is silent. The harmony goes on, for whoever
    /// plays over it.
    Rest,
}

impl Stop {
    pub const ALL: [Stop; 3] = [Stop::Shot, Stop::Hold, Stop::Rest];

    /// The dots that write it.
    pub fn marks(self) -> &'static str {
        match self {
            Stop::Shot => ".",
            Stop::Hold => "..",
            Stop::Rest => "...",
        }
    }

    /// One word for it.
    pub fn name(self) -> &'static str {
        match self {
            Stop::Shot => "shot",
            Stop::Hold => "hold",
            Stop::Rest => "rest",
        }
    }

    /// What the band does, for hints.
    pub fn describe(self) -> &'static str {
        match self {
            Stop::Shot => "the band strikes it once, short, and stops until the next harmony",
            Stop::Hold => "the band strikes it and holds it until the next harmony",
            Stop::Rest => "the band stops until the next harmony",
        }
    }
}

/// How early the band plays a harmony: a "push", or anticipation. The
/// chord comes in just before the beat it is written on, and is held
/// over into it. Written with a caret before the harmony.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Push {
    /// "^C": an eighth note early.
    Eighth,
    /// "^^C": a sixteenth note early.
    Sixteenth,
}

impl Push {
    /// How many steps early.
    pub fn steps(self) -> usize {
        match self {
            Push::Eighth => 2,
            Push::Sixteenth => 1,
        }
    }

    /// The carets that write it.
    pub fn marks(self) -> &'static str {
        match self {
            Push::Eighth => "^",
            Push::Sixteenth => "^^",
        }
    }

    /// What the band does, for hints.
    pub fn describe(self) -> &'static str {
        match self {
            Push::Eighth => "the band plays it an eighth note early, held over the beat",
            Push::Sixteenth => "the band plays it a sixteenth note early, held over the beat",
        }
    }
}

/// Splits the marks off a harmony as typed: the carets of a push before
/// it and the dots of a stop after it. "N.C." keeps its dots, which are
/// part of its name, pushed or not.
pub(crate) fn read_marks(text: &str) -> Result<(Option<Push>, &str, Option<Stop>), String> {
    let name = text.trim_start_matches('^');
    let push = match text.len() - name.len() {
        0 => None,
        1 => Some(Push::Eighth),
        2 => Some(Push::Sixteenth),
        _ => return Err("a push is written ^ (an eighth early) or ^^ (a sixteenth).".to_string()),
    };
    if is_no_chord(name) {
        return Ok((push, name, None));
    }
    let bare = name.trim_end_matches('.');
    let stop = match name.len() - bare.len() {
        0 => None,
        1 => Some(Stop::Shot),
        2 => Some(Stop::Hold),
        3 => Some(Stop::Rest),
        _ => {
            return Err(
                "after a harmony, . is a shot, .. a hold and ... a rest: no more dots.".to_string(),
            );
        }
    };
    Ok((push, bare, stop))
}

/// "N.C.", "NC" or "n.c.": no chord.
fn is_no_chord(text: &str) -> bool {
    let plain: String = text.chars().filter(|c| *c != '.').collect();
    plain.eq_ignore_ascii_case("NC")
}

/// Splits a move such as "T2" or "T7I" off the end of `text`. Returns
/// the rest and the move: how many semitones up, and whether it is
/// turned upside down first.
fn read_move(text: &str) -> (&str, Option<(u8, bool)>) {
    let Some((rest, last)) = text.rsplit_once(char::is_whitespace) else {
        return (text, None);
    };
    let Some(number) = last.strip_prefix(['T', 't']) else {
        return (text, None);
    };
    let (number, inverted) = match number.strip_suffix(['I', 'i']) {
        Some(number) => (number, true),
        None => (number, false),
    };
    match number.parse::<u8>() {
        Ok(n) if n < 12 => (rest.trim_end(), Some((n, inverted))),
        _ => (text, None),
    }
}

/// The prime form `prime` moved as `step` says.
fn moved(prime: PitchSet, step: Option<(u8, bool)>) -> PitchSet {
    let (n, inverted) = step.unwrap_or((0, false));
    let form = if inverted { prime.invert() } else { prime };
    form.transpose(n)
}

/// " T2", " T7I", or nothing for the prime form itself.
fn move_text(prime: PitchSet, set: PitchSet) -> String {
    match prime.moved_to(set) {
        Some((0, false)) | None => String::new(),
        Some((n, false)) => format!(" T{n}"),
        Some((n, true)) => format!(" T{n}I"),
    }
}

/// A set written by its notes: `{0,4,7}`, `[0 4 7]`, `(047)`, or with
/// note names, `{C E G}`. Numbers go from 0 (C) to 11 (B), with T and E
/// for 10 and 11 so each fits in one character. When every note can be
/// read as a note name, it is one: `{E}` is E, but `{0 4 E}` has an 11.
/// `None` if `text` is not in brackets.
fn read_notes(text: &str) -> Option<Result<PitchSet, String>> {
    let inside = text
        .strip_prefix('{')
        .and_then(|rest| rest.strip_suffix('}'))
        .or_else(|| text.strip_prefix('[')?.strip_suffix(']'))
        .or_else(|| text.strip_prefix('(')?.strip_suffix(')'))?;
    let number = |item: &str| match item {
        "T" | "t" => Some(10),
        "E" | "e" => Some(11),
        _ => item.parse::<u8>().ok().filter(|&n| n < 12),
    };
    let words: Vec<&str> = inside
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter(|word| !word.is_empty())
        .collect();
    // "047" in one word is three numbers.
    let packed = words.len() == 1
        && words[0].len() > 1
        && words[0]
            .chars()
            .all(|c| c.is_ascii_digit() || "TtEe".contains(c));
    let items: Vec<String> = if packed {
        words[0].chars().map(String::from).collect()
    } else {
        words.iter().map(|word| word.to_string()).collect()
    };
    let names = items.iter().all(|item| NoteName::parse(item).is_some());
    let mut set = PitchSet::EMPTY;
    for item in &items {
        let pitch_class = if names {
            NoteName::parse(item).map(NoteName::pitch_class)
        } else {
            number(item).map(PitchClass::from_semitone)
        };
        match pitch_class {
            Some(pitch_class) => set.insert(pitch_class),
            None => {
                return Some(Err(format!(
                    "\"{item}\" is not a note: write pitch classes as 0 to 11 (or T and E), or \
                     as note names, like {{0,4,7}} or {{C E G}}."
                )));
            }
        }
    }
    if set.is_empty() {
        return Some(Err("the brackets have no notes in them.".to_string()));
    }
    Some(Ok(set))
}

/// A row form, such as "P0", "I5", "R3" or "RI11".
fn read_row_form(text: &str) -> Option<RowLabel> {
    use crate::harmony::RowForm;
    let forms = [
        ("RI", RowForm::RetrogradeInversion),
        ("P", RowForm::Prime),
        ("I", RowForm::Inversion),
        ("R", RowForm::Retrograde),
    ];
    let (form, number) = forms
        .into_iter()
        .find_map(|(letters, form)| Some((form, text.strip_prefix(letters)?)))?;
    let number: u8 = number.parse().ok()?;
    (number < 12).then(|| RowLabel::new(form, number))
}

/// A Forte name such as "3-11" or "4-Z15".
fn read_forte(text: &str) -> Option<Result<&'static SetClass, String>> {
    let (size, _) = text.split_once('-')?;
    if size.is_empty() || !size.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let found = SetClass::all()
        .iter()
        .find(|class| class.name().eq_ignore_ascii_case(text));
    Some(found.ok_or_else(|| format!("\"{text}\" is not one of Forte's set classes.")))
}

impl HarmonyName {
    /// Reads a harmony written in any of the ways in the module
    /// documentation, except Roman numerals, which need a key: see
    /// [`HarmonyName::parse_in`].
    pub fn parse(text: &str) -> Result<HarmonyName, String> {
        let text = text.trim();
        if text.is_empty() {
            return Err("nothing is written.".to_string());
        }
        if is_no_chord(text) {
            return Ok(HarmonyName::NoChord);
        }
        if let Some(set) = read_notes(text) {
            return set.map(HarmonyName::Set);
        }
        if let Some(label) = read_row_form(text) {
            return Ok(HarmonyName::Row(label));
        }
        let (name, step) = read_move(text);
        if let Some(class) = read_forte(name) {
            return class.map(|class| HarmonyName::Set(moved(class.prime, step)));
        }
        if let Some(micromode) = Micromode::named(name) {
            return Ok(HarmonyName::Micromode(moved(micromode.prime, step)));
        }
        if let Some((upper, lower)) = text.split_once('|') {
            let upper = ChordSymbol::parse(upper)?;
            let lower = ChordSymbol::parse(lower)?;
            return Ok(HarmonyName::Polychord(upper, lower));
        }
        if NoteName::read_either(text, |_| true).is_some() {
            return ChordSymbol::parse(text).map(HarmonyName::Chord);
        }
        Err(format!(
            "\"{text}\" is not a chord (Am7, G/B), two chords at once (Eb7|Fb), a set (3-11 T2, \
             {{0,4,7}}), a micromode (Menor 3), a row form (P0, RI7) or N.C."
        ))
    }

    /// Reads a harmony typed where the song is in `key`. In a key of
    /// seven notes, Roman numerals are read too and written down as the
    /// chord they are in the key: "V7" in Bb major is F7. "I6" is then
    /// the tonic with a sixth (Bb6), not the row form I6, which needs a
    /// twelve-tone key.
    pub fn parse_in(text: &str, key: Scale) -> Result<HarmonyName, String> {
        Self::parse_counting(text, key, MinorNumerals::FromMajor)
    }

    /// [`HarmonyName::parse_in`], with numerals in a minor key counting
    /// its degrees as `numerals` says.
    fn parse_counting(
        text: &str,
        key: Scale,
        numerals: MinorNumerals,
    ) -> Result<HarmonyName, String> {
        let row_key = matches!(key.kind, ScaleKind::Row(_));
        if !row_key
            && key.is_heptatonic()
            && let Some(numeral) = numeral::parse_in(text.trim(), key, numerals)
        {
            return Ok(HarmonyName::Chord(numeral.symbol_in(key)));
        }
        HarmonyName::parse(text)
    }

    /// The notes it names, as pitch classes: the chord's notes with its
    /// bass, or the set's or micromode's. `None` for no chord and for a
    /// row form, which is a line of twelve notes, not a group.
    pub fn pitch_set(self) -> Option<PitchSet> {
        match self {
            HarmonyName::Chord(symbol) => {
                let mut set = symbol.chord().pitch_set();
                set.insert(symbol.bass_pitch_class());
                Some(set)
            }
            HarmonyName::Polychord(upper, lower) => {
                let mut set = HarmonyName::Chord(lower).pitch_set()?;
                for pitch_class in upper.chord().pitch_set().iter() {
                    set.insert(pitch_class);
                }
                Some(set)
            }
            HarmonyName::Set(set) | HarmonyName::Micromode(set) => Some(set),
            HarmonyName::NoChord | HarmonyName::Row(_) => None,
        }
    }

    /// The same harmony after the song moves from `from` to `to`, the
    /// way [`tools::change_key`] moves the notes: a new root takes every
    /// chord with it, and a new mode moves each note of a chord to the
    /// same degree of the new scale, so C major in C major becomes C
    /// minor in C minor. Sets and micromodes keep their shape and only
    /// move with the root, since an atonal set has no degrees; a row
    /// form moves with the row. When the notes of the song stay put (a
    /// key and a row have nothing to move by), so does the harmony.
    pub fn moved(self, from: Scale, to: Scale) -> HarmonyName {
        // A key shown as a pitch-class set moves on as that key.
        let from = tools::from_key(from, to);
        if from == to || !from.kind.notes_follow(to.kind) {
            return self;
        }
        let up = from.root.semitones_up_to(to.root);
        match self {
            HarmonyName::Chord(symbol) if from.kind == to.kind => {
                HarmonyName::Chord(symbol.transposed(from.spell(from.root), to.spell(to.root)))
            }
            HarmonyName::Chord(symbol) => HarmonyName::Chord(chord_moved(symbol, from, to)),
            HarmonyName::Polychord(upper, lower) if from.kind == to.kind => {
                let (from, to) = (from.spell(from.root), to.spell(to.root));
                HarmonyName::Polychord(upper.transposed(from, to), lower.transposed(from, to))
            }
            HarmonyName::Polychord(upper, lower) => {
                HarmonyName::Polychord(chord_moved(upper, from, to), chord_moved(lower, from, to))
            }
            HarmonyName::NoChord => HarmonyName::NoChord,
            HarmonyName::Set(set) => HarmonyName::Set(set.transpose(up)),
            HarmonyName::Micromode(set) => HarmonyName::Micromode(set.transpose(up)),
            HarmonyName::Row(label) => {
                HarmonyName::Row(RowLabel::new(label.form, label.number + up))
            }
        }
    }

    /// A few words under the name on the chord sheet: the Roman numeral
    /// of a chord in `key` (with the figures of an inversion, "V⁶₅"),
    /// or the notes of a set or a micromode. `None` when there is
    /// nothing to add.
    pub fn caption(self, key: Scale) -> Option<String> {
        match self {
            HarmonyName::Chord(symbol) => numeral_with_figures(symbol, key),
            HarmonyName::Polychord(upper, lower) => {
                let upper = numeral_with_figures(upper, key)?;
                let lower = numeral_with_figures(lower, key)?;
                Some(format!("{upper} over {lower}"))
            }
            HarmonyName::Set(set) | HarmonyName::Micromode(set) => Some(spell_set(set, key)),
            HarmonyName::Row(label) => match key.kind {
                ScaleKind::Row(row) => {
                    let notes: Vec<String> = row
                        .form(label)
                        .pitch_classes()
                        .iter()
                        .take(4)
                        .map(|&pitch_class| key.spell(pitch_class).to_string())
                        .collect();
                    Some(format!("{}…", notes.join(" ")))
                }
                _ => None,
            },
            HarmonyName::NoChord => None,
        }
    }

    /// A sentence about the harmony in `key`, for its tooltip.
    pub fn describe(self, key: Scale) -> String {
        match self {
            HarmonyName::Chord(symbol) => {
                let notes: Vec<String> = symbol.notes().iter().map(NoteName::to_string).collect();
                let notes = notes.join(" ");
                if !key.is_heptatonic() {
                    return format!("{symbol}: {notes}.");
                }
                // Said as the harmony keys say it ("A dominant seventh,
                // V7/ii in C major"), but with the root and the numeral
                // as written: Gb7 is a Gb chord, not an F# one.
                let about = describe_chord(
                    key,
                    symbol.chord(),
                    symbol.bass_pitch_class(),
                    symbol.root,
                    numeral_with_figures(symbol, key),
                );
                format!("{symbol}: {about} Its notes: {notes}.")
            }
            HarmonyName::Polychord(upper, lower) => {
                let notes = |symbol: ChordSymbol| {
                    let notes: Vec<String> =
                        symbol.notes().iter().map(NoteName::to_string).collect();
                    notes.join(" ")
                };
                let mut lower_name = format!("{} {}", lower.root, lower.quality.name());
                if let Some(bass) = lower.bass {
                    lower_name += &format!(" with {bass} in the bass");
                }
                let numerals = match self.caption(key) {
                    Some(numerals) => format!(" In {}: {numerals}.", key.name()),
                    None => String::new(),
                };
                format!(
                    "{self}: {} {} over {lower_name}, two chords at once: a polychord. Upper: {}. \
                     Lower: {}.{numerals}",
                    upper.root,
                    upper.quality.name(),
                    notes(upper),
                    notes(lower),
                )
            }
            HarmonyName::NoChord => "No chord: nothing plays the harmony here.".to_string(),
            HarmonyName::Set(set) => {
                let class = set.set_class();
                format!(
                    "{}: {}. Set class {}.",
                    self,
                    spell_set(set, key),
                    class.describe()
                )
            }
            HarmonyName::Micromode(set) => match Micromode::of(set) {
                Some(micromode) => format!(
                    "{}: {}. Micromode {}, Forte's {}; {}.",
                    self,
                    spell_set(set, key),
                    micromode.full_name(),
                    micromode.set_class().name(),
                    micromode.about
                ),
                None => format!("{self}: {}.", spell_set(set, key)),
            },
            HarmonyName::Row(label) => match key.kind {
                ScaleKind::Row(row) => {
                    let notes: Vec<String> = row
                        .form(label)
                        .pitch_classes()
                        .iter()
                        .map(|&pitch_class| key.spell(pitch_class).to_string())
                        .collect();
                    format!("{label} of the row: {}.", notes.join(" "))
                }
                _ => format!(
                    "{label} is a form of a twelve-tone row, but the key here is {}: pick a row \
                     as the key to hear which notes it means.",
                    key.name()
                ),
            },
        }
    }
}

/// A chord moved to a new mode, note by note (see [`HarmonyName::moved`]).
fn chord_moved(symbol: ChordSymbol, from: Scale, to: Scale) -> ChordSymbol {
    let move_note = |pitch_class: PitchClass| {
        tools::change_key(Note::new(60 + pitch_class.semitone()), from, to).pitch_class()
    };
    let chord = symbol.chord();
    let root = move_note(chord.root);
    let notes: PitchSet = chord.pitch_set().iter().map(move_note).collect();
    let quality = ChordQuality::identify(notes.relative_to(root)).unwrap_or(chord.quality);
    let chord = Chord::new(root, quality);
    let root_name = to.spell_root(chord);
    let moved = ChordSymbol::new(root_name, quality);
    match symbol.bass {
        Some(_) => {
            let bass = move_note(symbol.bass_pitch_class());
            let name = chord
                .spell_note(root_name, bass)
                .unwrap_or_else(|| to.spell(bass));
            moved.over(name)
        }
        None => moved,
    }
}

/// The notes of a set, spelled as `key` spells them, in normal form:
/// "D F A". Three notes that make a chord are spelled as the chord, up
/// from its root, as in the Micromodes window: C Eb Gb, not C Eb F#.
fn spell_set(set: PitchSet, key: Scale) -> String {
    let names: Vec<NoteName> = match Micromode::of(set).and_then(|m| m.chord(set)) {
        Some(chord) => chord.spell(key.spell_root(chord)),
        None => set
            .normal_form()
            .into_iter()
            .map(|p| key.spell(p))
            .collect(),
    };
    let names: Vec<String> = names.iter().map(NoteName::to_string).collect();
    names.join(" ")
}

/// The Roman numeral of a chord in `key`, with the figures of figured
/// bass for an inversion ("V⁶₅", see [`Scale::figured_numeral`]). `None`
/// in a key without seven notes.
fn numeral_with_figures(symbol: ChordSymbol, key: Scale) -> Option<String> {
    key.spelled_figured_numeral(symbol.root, symbol.chord(), symbol.bass_pitch_class())
}

impl fmt::Display for HarmonyName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            HarmonyName::Chord(symbol) => write!(f, "{symbol}"),
            HarmonyName::Polychord(upper, lower) => write!(f, "{upper}|{lower}"),
            HarmonyName::NoChord => write!(f, "N.C."),
            HarmonyName::Set(set) => {
                let class = set.set_class();
                write!(f, "{}{}", class.name(), move_text(class.prime, set))
            }
            HarmonyName::Micromode(set) => match Micromode::of(set) {
                Some(micromode) => {
                    write!(f, "{}{}", micromode.name, move_text(micromode.prime, set))
                }
                None => {
                    let digits: String = set.iter().map(|p| pc_digit(p.semitone())).collect();
                    write!(f, "{{{digits}}}")
                }
            },
            HarmonyName::Row(label) => write!(f, "{label}"),
        }
    }
}

impl FromStr for HarmonyName {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, String> {
        HarmonyName::parse(text)
    }
}

/// Project files write a harmony as its name, so they stay readable:
/// `(start: 32, harmony: "G7/B")`. Always in letters, whichever way the
/// app names notes, so the file is the same for everyone.
impl From<HarmonyName> for String {
    fn from(harmony: HarmonyName) -> Self {
        NoteNaming::Letters.with(|| harmony.to_string())
    }
}

impl TryFrom<String> for HarmonyName {
    type Error = String;

    fn try_from(text: String) -> Result<Self, String> {
        NoteNaming::Letters.with(|| HarmonyName::parse(&text))
    }
}

/// One thing typed in a bar: a harmony with its marks, or a beat that
/// goes on with the one before ("/").
enum Item {
    Harmony(HarmonyName, Option<Push>, Option<Stop>),
    Hold,
}

/// Reads a harmony with its marks (see [`read_marks`]).
fn read_marked(
    text: &str,
    key: Scale,
    numerals: MinorNumerals,
) -> Result<(HarmonyName, Option<Push>, Option<Stop>), String> {
    let (push, name, stop) = read_marks(text)?;
    if name.is_empty() {
        return Err(format!("\"{text}\" has marks but no harmony."));
    }
    let harmony = HarmonyName::parse_counting(name, key, numerals)?;
    Ok((harmony, push, stop))
}

/// The most words one harmony takes: "Acorde menor-mayor T11I" is three.
const MOST_WORDS: usize = 4;

/// Reads what is typed in one bar of the chord sheet: harmonies
/// separated by spaces, as in "Am7 D7".
///
/// Harmonies spread over the bar's beats the way lead sheets do: two in
/// a bar of four beats take two beats each, three take two, one and
/// one, and two in a bar of three take two and one. A "/" (or ".") is
/// one beat of the harmony before it, to place them yourself: "C / / G7"
/// puts G7 on the fourth beat. "%", or nothing at all, leaves the bar
/// going on with the harmony before it. A harmony can carry the marks of
/// a [`Push`] and a [`Stop`]: "^C.".
///
/// Returns the markers, each on the step its beat starts on, or why the
/// text could not be read.
///
/// Roman numerals are read as the library writes them, from the major
/// scale (bVII in a minor key); [`read_typed_bar`] reads them as the
/// sheet writes them.
pub fn read_bar(text: &str, bar: &Bar, key: Scale) -> Result<Vec<ChordMarker>, String> {
    read_bar_counting(text, bar, key, MinorNumerals::FromMajor)
}

/// [`read_bar`] for a bar you type: numerals in a minor key count its
/// degrees as [`MinorNumerals::current`] writes them, so that "VII"
/// typed in A minor is the G major the sheet writes as VII. What is
/// stored is the chord, so a song reads the same whichever way it was
/// typed.
pub fn read_typed_bar(text: &str, bar: &Bar, key: Scale) -> Result<Vec<ChordMarker>, String> {
    read_bar_counting(text, bar, key, MinorNumerals::current())
}

fn read_bar_counting(
    text: &str,
    bar: &Bar,
    key: Scale,
    numerals: MinorNumerals,
) -> Result<Vec<ChordMarker>, String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() || words == ["%"] {
        return Ok(Vec::new());
    }
    let mut items = Vec::new();
    let mut at = 0;
    while at < words.len() {
        if matches!(words[at], "/" | ".") {
            items.push(Item::Hold);
            at += 1;
            continue;
        }
        // The longest run of words that names a harmony, so "Menor 3 T2"
        // is one micromode, not three things. A "/" on its own is a beat,
        // never part of a name: "E / B" is E held, then B, not E/B.
        let holds_at = words[at..]
            .iter()
            .position(|word| matches!(*word, "/" | "."))
            .map_or(words.len(), |offset| at + offset);
        let longest = (at + 1..=holds_at.min(at + MOST_WORDS))
            .rev()
            .find_map(|end| {
                read_marked(&words[at..end].join(" "), key, numerals)
                    .ok()
                    .map(|marked| (marked, end))
            });
        match longest {
            Some(((harmony, push, stop), end)) => {
                items.push(Item::Harmony(harmony, push, stop));
                at = end;
            }
            None => return Err(read_marked(words[at], key, numerals).unwrap_err()),
        }
    }
    let beats: Vec<usize> = bar.beat_starts().collect();
    let harmonies = items
        .iter()
        .filter(|item| matches!(item, Item::Harmony(..)))
        .count();
    let holds = items.len() > harmonies;
    let written = if holds { items.len() } else { harmonies };
    if written > beats.len() {
        let what = if holds { "beats are" } else { "harmonies are" };
        return Err(format!(
            "this bar has {} beats, and {written} {what} written in it.",
            beats.len()
        ));
    }
    let mut markers = Vec::new();
    let mut harmony_count = 0;
    for (place, item) in items.into_iter().enumerate() {
        if let Item::Harmony(harmony, push, stop) = item {
            let beat = if holds {
                place
            } else {
                (harmony_count * beats.len()).div_ceil(harmonies)
            };
            markers.push(
                ChordMarker::new(beats[beat], harmony)
                    .pushed(push)
                    .stopped(stop),
            );
            harmony_count += 1;
        }
    }
    Ok(markers)
}

/// The text that [`read_bar`] reads back as `markers`, the harmonies of
/// one bar: "Am7 D7" when they fall where they would on their own, or
/// with a "/" for each beat that goes on, such as "C / / G7".
pub fn write_bar(markers: &[ChordMarker], bar: &Bar) -> String {
    let beats: Vec<usize> = bar.beat_starts().collect();
    let beat_of = |start: usize| nearest_beat(&beats, start);
    let places: Vec<usize> = markers.iter().map(|marker| beat_of(marker.start)).collect();
    let spread = |count: usize| -> Vec<usize> {
        (0..count)
            .map(|k| (k * beats.len()).div_ceil(count))
            .collect()
    };
    let names: Vec<String> = markers.iter().map(ChordMarker::text).collect();
    if places == spread(markers.len()) {
        return names.join(" ");
    }
    let mut words: Vec<String> = Vec::new();
    for beat in 0..beats.len() {
        match places.iter().position(|&place| place == beat) {
            Some(index) => words.push(names[index].clone()),
            None => words.push("/".to_string()),
        }
    }
    // A "/" at the end says nothing, as long as another "/" is left to
    // say that the harmonies are placed by hand: "C G7 C /" must stay
    // as it is, or it reads as three harmonies spread over the bar.
    while words.last().is_some_and(|word| word == "/")
        && words[..words.len() - 1].iter().any(|word| word == "/")
    {
        words.pop();
    }
    words.join(" ")
}

/// Which of `beats` (steps, in order) `step` is closest to; on a tie,
/// the later one.
fn nearest_beat(beats: &[usize], step: usize) -> usize {
    (0..beats.len())
        .rev()
        .min_by_key(|&beat| beats[beat].abs_diff(step))
        .unwrap_or(0)
}

/// Whether harmony heard in `key` is written as groups of notes rather
/// than chords: in a key that is a set or a twelve-tone row, chords
/// have no function, and a chord name would hide what the music is
/// made of.
pub(crate) fn names_groups(key: Scale) -> bool {
    matches!(key.kind, ScaleKind::Set(_) | ScaleKind::Row(_))
}

/// What to write for the notes `heard` together in `key` (see
/// [`names_groups`]): a micromode when the key is a trichord, as in the
/// micromode demos, and three notes are heard; otherwise the set, by
/// its Forte name. `None` for fewer than two notes.
pub(crate) fn heard_group(heard: PitchSet, key: Scale) -> Option<HarmonyName> {
    if heard.len() < 2 {
        return None;
    }
    let trichord_key = matches!(key.kind, ScaleKind::Set(set) if set.len() == 3);
    if trichord_key && heard.len() == 3 && Micromode::of(heard).is_some() {
        Some(HarmonyName::Micromode(heard))
    } else {
        Some(HarmonyName::Set(heard))
    }
}

/// `step` moved to the nearest beat of a song in `time` with no
/// changes of time signature; on a tie, the later beat. The start of the
/// next bar counts too.
pub fn snap_to_beat(step: usize, time: TimeSignature) -> usize {
    let bar = time.steps_per_bar();
    let start = step / bar * bar;
    let mut beats: Vec<usize> = time.beat_starts().iter().map(|beat| start + beat).collect();
    beats.push(start + bar);
    beats[nearest_beat(&beats, step)]
}

/// Puts markers in order, one per step, and drops those past `end`.
/// When two are on the same step the later one in the list wins.
pub(crate) fn tidy(markers: &mut Vec<ChordMarker>, end: usize) {
    markers.retain(|marker| marker.start < end);
    markers.reverse();
    markers.sort_by_key(|marker| marker.start);
    markers.dedup_by_key(|marker| marker.start);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Meter;
    use crate::harmony::RowForm;
    use crate::harmony::ToneRow;

    fn name(text: &str) -> HarmonyName {
        HarmonyName::parse(text).unwrap_or_else(|err| panic!("{text}: {err}"))
    }

    fn set(semitones: &[u8]) -> PitchSet {
        PitchSet::from_semitones(semitones)
    }

    const C_MAJOR: Scale = Scale::new(PitchClass::C, ScaleKind::Major);

    #[test]
    fn marks_tell_the_band_where_to_stop_and_push() {
        let bar = Meter::steady(TimeSignature::COMMON).bar(0);
        let read = |text| read_bar(text, &bar, C_MAJOR).unwrap();
        let marks = |markers: Vec<ChordMarker>| -> Vec<(usize, Option<Push>, Option<Stop>)> {
            markers
                .iter()
                .map(|marker| (marker.start, marker.push, marker.stop))
                .collect()
        };
        assert_eq!(marks(read("C.")), [(0, None, Some(Stop::Shot))]);
        assert_eq!(marks(read("C..")), [(0, None, Some(Stop::Hold))]);
        assert_eq!(marks(read("C...")), [(0, None, Some(Stop::Rest))]);
        assert_eq!(marks(read("^C")), [(0, Some(Push::Eighth), None)]);
        assert_eq!(
            marks(read("^^Am7/G.")),
            [(0, Some(Push::Sixteenth), Some(Stop::Shot))]
        );
        // Marks on any kind of harmony, and with beats placed by hand.
        assert_eq!(
            marks(read("Dm7 / ^G7.. /")),
            [(0, None, None), (8, Some(Push::Eighth), Some(Stop::Hold))]
        );
        assert_eq!(marks(read("Menor 3 T2.")), [(0, None, Some(Stop::Shot))]);
        assert_eq!(marks(read("^V7")), [(0, Some(Push::Eighth), None)]);
        // N.C. keeps its dots, and a lone "." is still a beat.
        assert_eq!(read("N.C.")[0].harmony, HarmonyName::NoChord);
        assert_eq!(marks(read("N.C.")), [(0, None, None)]);
        assert_eq!(marks(read("^N.C.")), [(0, Some(Push::Eighth), None)]);
        assert_eq!(marks(read("^^NC")), [(0, Some(Push::Sixteenth), None)]);
        assert_eq!(marks(read("C . G .")), [(0, None, None), (8, None, None)]);
        // Too many marks, or marks on nothing, are mistakes.
        assert!(read_bar("C....", &bar, C_MAJOR).is_err());
        assert!(read_bar("^^^C", &bar, C_MAJOR).is_err());
        assert!(read_bar("^", &bar, C_MAJOR).is_err());
    }

    #[test]
    fn marks_are_written_back_as_typed() {
        let bar = Meter::steady(TimeSignature::COMMON).bar(0);
        for text in [
            "^C.",
            "C.. G7",
            "Dm7 / / ^^G7...",
            "^Menor 3 T2..",
            "C ^N.C.",
        ] {
            let markers = read_bar(text, &bar, C_MAJOR).unwrap();
            assert_eq!(write_bar(&markers, &bar), text);
        }
    }

    #[test]
    fn every_way_of_naming_harmony_is_read() {
        assert!(matches!(name("Am7"), HarmonyName::Chord(_)));
        assert_eq!(name("N.C."), HarmonyName::NoChord);
        assert_eq!(name("nc"), HarmonyName::NoChord);
        // D minor: the minor triad's prime form, moved up 2.
        assert_eq!(name("3-11 T2"), HarmonyName::Set(set(&[2, 5, 9])));
        // Turned upside down first: a major triad, on G.
        assert_eq!(name("3-11 T2I"), HarmonyName::Set(set(&[2, 7, 11])));
        assert_eq!(name("4-z15"), HarmonyName::Set(set(&[0, 1, 4, 6])));
        assert_eq!(name("{0,4,7}"), HarmonyName::Set(set(&[0, 4, 7])));
        assert_eq!(name("[047]"), HarmonyName::Set(set(&[0, 4, 7])));
        assert_eq!(name("{D F A}"), HarmonyName::Set(set(&[2, 5, 9])));
        assert_eq!(name("{t e 0}"), HarmonyName::Set(set(&[10, 11, 0])));
        // Menor 3 is do, do#, mi: C C# E.
        assert_eq!(name("Menor 3"), HarmonyName::Micromode(set(&[0, 1, 4])));
        assert_eq!(name("minor 3 T2"), HarmonyName::Micromode(set(&[2, 3, 6])));
        assert_eq!(
            name("Acorde aumentado T1"),
            HarmonyName::Micromode(set(&[1, 5, 9]))
        );
        assert_eq!(
            name("RI7"),
            HarmonyName::Row(RowLabel::new(RowForm::RetrogradeInversion, 7))
        );
        assert_eq!(
            name("P0"),
            HarmonyName::Row(RowLabel::new(RowForm::Prime, 0))
        );
        assert_eq!(
            name("I11"),
            HarmonyName::Row(RowLabel::new(RowForm::Inversion, 11))
        );
    }

    #[test]
    fn nonsense_is_refused_with_a_reason() {
        for text in ["", "H7", "3-13", "{x}", "{}", "P12", "Cxyz", "hello"] {
            assert!(HarmonyName::parse(text).is_err(), "{text}");
        }
        assert!(HarmonyName::parse("Cxyz").unwrap_err().contains("xyz"));
    }

    #[test]
    fn every_name_is_written_back_the_same_way() {
        for text in [
            "Am7",
            "G/B",
            "F#m7b5",
            "N.C.",
            "3-11 T2",
            "3-11 T2I",
            "4-Z15",
            "7-35 T5",
            "Menor 3",
            "Mayor 2 T9I",
            "Acorde aumentado T1",
            "RI7",
            "P0",
        ] {
            assert_eq!(name(text).to_string(), text);
            assert_eq!(name(&name(text).to_string()), name(text));
        }
        // Notes in brackets are named by their set class.
        assert_eq!(name("{2,5,9}").to_string(), "3-11 T2");
        assert_eq!(name("Minor 3").to_string(), "Menor 3");
    }

    #[test]
    fn numerals_are_chords_in_the_key() {
        let bb = Scale::new(PitchClass::ASharp, ScaleKind::Major);
        let read = |text, key| match HarmonyName::parse_in(text, key).unwrap() {
            HarmonyName::Chord(symbol) => symbol.to_string(),
            other => panic!("{other:?}"),
        };
        assert_eq!(read("V7", bb), "F7");
        assert_eq!(read("ii7", bb), "Cm7");
        assert_eq!(read("bVII", C_MAJOR), "Bb");
        // In a key of seven notes "I6" is the tonic with a sixth...
        assert_eq!(read("I6", bb), "Bb6");
        // ...and in a twelve-tone key it is a form of the row.
        let row = Scale::row(ToneRow::CHROMATIC);
        assert_eq!(
            HarmonyName::parse_in("I6", row),
            Ok(HarmonyName::Row(RowLabel::new(RowForm::Inversion, 6)))
        );
    }

    #[test]
    fn typed_numerals_in_minor_follow_the_way_numerals_are_written() {
        let a_minor = Scale::new(PitchClass::A, ScaleKind::NaturalMinor);
        let bar = Meter::steady(TimeSignature::COMMON).bar(0);
        let names = |markers: Vec<ChordMarker>| -> Vec<String> {
            markers.iter().map(ChordMarker::text).collect()
        };
        let typed = |text| names(read_typed_bar(text, &bar, a_minor).unwrap());
        // From the major scale, as the library writes them.
        assert_eq!(typed("bVI bVII vii° i"), ["F", "G", "G#dim", "Am"]);
        MinorNumerals::FromMinor.with(|| {
            assert_eq!(typed("VI VII vii° i"), ["F", "G", "G#dim", "Am"]);
            assert_eq!(typed("III+ V7 bII i"), ["Caug", "E7", "Bb", "Am"]);
            // A song's or a style's own text is still read as the
            // library writes it.
            let stored = read_bar("bVI bVII", &bar, a_minor).unwrap();
            assert_eq!(names(stored), ["F", "G"]);
            // And the captions say it as typed.
            let f = HarmonyName::parse("F").unwrap();
            assert_eq!(f.caption(a_minor).as_deref(), Some("VI"));
        });
    }

    #[test]
    fn captions_give_the_numeral_and_the_notes() {
        let caption = |text: &str| name(text).caption(C_MAJOR).unwrap_or_default();
        assert_eq!(caption("Am"), "vi");
        assert_eq!(caption("G7"), "V7");
        assert_eq!(caption("C/E"), "I⁶");
        assert_eq!(caption("C/G"), "I⁶₄");
        assert_eq!(caption("G7/B"), "V⁶₅");
        assert_eq!(caption("G7/D"), "V⁴₃");
        assert_eq!(caption("G7/F"), "V⁴₂");
        assert_eq!(caption("Bm7b5/D"), "viiø⁶₅");
        // A bass note outside the chord has no figures.
        assert_eq!(caption("F/G"), "IV");
        assert_eq!(caption("3-11 T2"), "D F A");
        assert_eq!(caption("Menor 3 T2"), "D Eb F#");
        assert_eq!(caption("N.C."), "");
    }

    #[test]
    fn tooltips_say_what_the_harmony_does_with_the_name_written() {
        let a7 = name("A7").describe(C_MAJOR);
        assert!(
            a7.starts_with("A7: A dominant seventh, V7/ii in C major"),
            "{a7}"
        );
        assert!(a7.ends_with("Its notes: A C# E G."), "{a7}");
        // Written with flats, it stays written with flats.
        let gb = name("Gb7").describe(C_MAJOR);
        assert!(
            gb.starts_with("Gb7: Gb dominant seventh, bV7, from outside C major."),
            "{gb}"
        );
        assert!(gb.ends_with("Its notes: Gb Bb Db Fb."), "{gb}");
        // The letter says which degree: C#°7 is a raised tonic, #i°7.
        let raised = name("C#dim7").describe(C_MAJOR);
        assert!(raised.contains("#i°7"), "{raised}");
        assert_eq!(name("C#dim7").caption(C_MAJOR).as_deref(), Some("#i°7"));
        let set = name("3-11 T2").describe(C_MAJOR);
        assert!(set.starts_with("3-11 T2: D F A. Set class 3-11"), "{set}");
    }

    #[test]
    fn harmonies_move_with_the_key() {
        let d_major = Scale::new(PitchClass::D, ScaleKind::Major);
        let c_minor = Scale::new(PitchClass::C, ScaleKind::NaturalMinor);
        let a_minor = Scale::new(PitchClass::A, ScaleKind::NaturalMinor);
        let moved = |text: &str, to| name(text).moved(C_MAJOR, to).to_string();
        // A new root moves every chord, spelled by letters.
        assert_eq!(moved("G7/B", d_major), "A7/C#");
        assert_eq!(moved("Bb", d_major), "C");
        // A new mode moves each note to the same degree, as the notes
        // of the patterns move: C E G becomes C Eb G.
        assert_eq!(moved("C", c_minor), "Cm");
        assert_eq!(moved("G7", c_minor), "Gm7");
        assert_eq!(moved("Am", c_minor), "Ab");
        assert_eq!(moved("C", a_minor), "Am");
        assert_eq!(moved("G/B", a_minor), "Em/G");
        // Sets keep their shape and move with the root.
        assert_eq!(moved("3-11 T2", d_major), "3-11 T4");
        assert_eq!(moved("Menor 3", c_minor), "Menor 3");
        // A key change to a row leaves everything where it is.
        let row = Scale::row(ToneRow::CHROMATIC);
        assert_eq!(moved("G7", row), "G7");
        // C major shown as a pitch-class set keeps its chords, and from
        // there to C minor they move as from C major.
        let as_set = Scale::set(PitchClass::C, C_MAJOR.pitch_set());
        assert_eq!(moved("Am", as_set), "Am");
        let from_set = |text: &str, to| name(text).moved(as_set, to).to_string();
        assert_eq!(from_set("C", c_minor), "Cm");
        assert_eq!(from_set("Am", c_minor), "Ab");
    }

    fn bar_of(time: TimeSignature) -> Bar {
        Meter::steady(time).bar(1)
    }

    fn read(text: &str, time: TimeSignature) -> Vec<(usize, String)> {
        let bar = bar_of(time);
        read_bar(text, &bar, C_MAJOR)
            .unwrap_or_else(|err| panic!("{text}: {err}"))
            .iter()
            .map(|marker| (marker.start - bar.start, marker.harmony.to_string()))
            .collect()
    }

    #[test]
    fn harmonies_spread_over_the_beats_as_on_a_lead_sheet() {
        let common = TimeSignature::COMMON;
        let waltz = TimeSignature::new(3, 4);
        assert_eq!(read("Am7", common), [(0, "Am7".to_string())]);
        assert_eq!(
            read("Am7 D7", common),
            [(0, "Am7".to_string()), (8, "D7".to_string())]
        );
        // Three in four beats: two, one and one.
        let three: Vec<usize> = read("C F G", common).iter().map(|(at, _)| *at).collect();
        assert_eq!(three, [0, 8, 12]);
        // Two in three beats: two and one.
        let two: Vec<usize> = read("C G7", waltz).iter().map(|(at, _)| *at).collect();
        assert_eq!(two, [0, 8]);
        // Slashes place them: G7 on the fourth beat.
        assert_eq!(
            read("C / / G7", common),
            [(0, "C".to_string()), (12, "G7".to_string())]
        );
        // A bar that starts with a slash goes on with the chord before.
        assert_eq!(read("/ G7", common), [(4, "G7".to_string())]);
        // A slash between two chords is a beat, not a slash chord.
        assert_eq!(
            read("A E / B", common),
            [
                (0, "A".to_string()),
                (4, "E".to_string()),
                (12, "B".to_string())
            ]
        );
        assert_eq!(read("E/B", common), [(0, "E/B".to_string())]);
        // Names of several words, and numerals in the key.
        assert_eq!(
            read("Menor 3 T2 ii7", common),
            [(0, "Menor 3 T2".to_string()), (8, "Dm7".to_string())]
        );
        assert!(read("%", common).is_empty());
        assert!(read("  ", common).is_empty());
    }

    #[test]
    fn a_bar_cannot_hold_more_than_its_beats() {
        let bar = bar_of(TimeSignature::new(3, 4));
        assert!(read_bar("C F G Am", &bar, C_MAJOR).is_err());
        assert!(read_bar("C / / G", &bar, C_MAJOR).is_err());
        let error = read_bar("C Xm7", &bar, C_MAJOR).unwrap_err();
        assert!(error.contains("Xm7"), "{error}");
    }

    #[test]
    fn a_bar_is_written_back_as_it_reads() {
        for time in [
            TimeSignature::COMMON,
            TimeSignature::new(3, 4),
            TimeSignature::new(6, 8),
        ] {
            let bar = bar_of(time);
            for text in [
                "Am7",
                "Am7 D7",
                "C / / G7",
                "/ G7",
                "C F G",
                "C G7 C /",
                "A E / B",
                "",
                "3-11 T2 Menor 3",
            ] {
                let Ok(markers) = read_bar(text, &bar, C_MAJOR) else {
                    continue;
                };
                let written = write_bar(&markers, &bar);
                assert_eq!(
                    read_bar(&written, &bar, C_MAJOR),
                    Ok(markers),
                    "{text} in {time:?}"
                );
            }
        }
        let bar = bar_of(TimeSignature::COMMON);
        let markers = read_bar("C / / G7", &bar, C_MAJOR).unwrap();
        assert_eq!(write_bar(&markers, &bar), "C / / G7");
        let markers = read_bar("C / G7 /", &bar, C_MAJOR).unwrap();
        assert_eq!(write_bar(&markers, &bar), "C G7");
        // Three harmonies on the first three beats are not the three
        // spread over the bar (beats 1, 3 and 4).
        let markers = read_bar("C G7 C /", &bar, C_MAJOR).unwrap();
        assert_eq!(write_bar(&markers, &bar), "C G7 C /");
    }

    #[test]
    fn steps_snap_to_the_nearest_beat() {
        let waltz = TimeSignature::new(3, 4);
        assert_eq!(snap_to_beat(6, waltz), 8);
        assert_eq!(snap_to_beat(5, waltz), 4);
        assert_eq!(snap_to_beat(11, waltz), 12);
        assert_eq!(snap_to_beat(13, waltz), 12);
    }

    #[test]
    fn markers_are_tidied_with_the_later_winning() {
        let c = name("C");
        let g = name("G");
        let mut markers = vec![
            ChordMarker::new(16, c),
            ChordMarker::new(0, c),
            ChordMarker::new(16, g),
            ChordMarker::new(64, g),
        ];
        tidy(&mut markers, 64);
        assert_eq!(markers, [ChordMarker::new(0, c), ChordMarker::new(16, g)]);
    }
}
