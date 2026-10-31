//! The language the app writes in, and the Spanish for each text.
//!
//! Every text the app shows is written in English in the code and goes
//! through [`tr`] (or [`trf`] when it has numbers or names in it), which
//! looks it up in the Spanish catalog when Spanish is chosen. A text with
//! no Spanish yet stays in English.
//!
//! Like [`NoteNaming`](crate::NoteNaming), the language is kept per
//! thread. What the app saves (project files, chord sheets as text, the
//! playback trace) is written in English whatever the language, inside
//! [`Language::with`].

use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::collections::{BTreeSet, HashMap};
use std::sync::{LazyLock, Mutex};

use serde::{Deserialize, Serialize};

use crate::{NoteName, NoteNaming};

#[cfg(test)]
mod coverage;
mod es;

/// The language the app writes in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Language {
    #[default]
    English,
    Spanish,
}

thread_local! {
    /// The language this thread writes in. The window's thread sets it
    /// from the settings each frame; each thread has its own, so tests
    /// running side by side never see each other's.
    static LANGUAGE: Cell<Language> = const { Cell::new(Language::English) };

    /// Whether texts with no Spanish are written down, and those seen.
    static MISSED: RefCell<Option<BTreeSet<String>>> = const { RefCell::new(None) };
}

impl Language {
    pub const ALL: [Language; 2] = [Language::English, Language::Spanish];

    /// The language's name, written in that language, so that anyone
    /// can find their own.
    pub fn name(self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Spanish => "Español",
        }
    }

    /// The language this thread writes in.
    pub fn current() -> Language {
        LANGUAGE.with(Cell::get)
    }

    /// Writes in this language on this thread from now on.
    pub fn set(self) {
        LANGUAGE.with(|language| language.set(self));
    }

    /// Runs `f` in this language, then goes back to the one before, even
    /// if `f` panics.
    pub fn with<T>(self, f: impl FnOnce() -> T) -> T {
        struct Restore(Language);
        impl Drop for Restore {
            fn drop(&mut self) {
                self.0.set();
            }
        }
        let _restore = Restore(Language::current());
        self.set();
        f()
    }
}

/// Translates a text written in the code into the current language. A
/// text with no translation stays as it is.
#[inline]
pub fn tr(text: &'static str) -> &'static str {
    match Language::current() {
        Language::English => text,
        Language::Spanish => in_spanish(text).unwrap_or_else(|| missed(text)),
    }
}

/// Translates a short text whose translation depends on where it is
/// shown: `trc("clef", "Bass")` is "Fa", where `tr("Bass")` is "Bajo".
/// The catalog has it as "clef|Bass".
pub fn trc(context: &'static str, text: &'static str) -> &'static str {
    match Language::current() {
        Language::English => text,
        Language::Spanish => {
            in_spanish(&format!("{context}|{text}")).unwrap_or_else(|| missed(text))
        }
    }
}

/// Translates a text that comes from elsewhere, such as the name of a
/// style or an instrument, into the current language.
#[inline]
pub fn tr_str(text: &str) -> &str {
    match Language::current() {
        Language::English => text,
        Language::Spanish => in_spanish(text).unwrap_or_else(|| missed(text)),
    }
}

/// `template` in the current language, with each `{}` in it replaced by
/// the next of `args`: `trf("{} steps a bar", &[&12])` is "12 steps a
/// bar", or "12 pasos por compás" in Spanish. The translation may put
/// the `{}` among other words, but takes the arguments in the same
/// order.
pub fn trf(template: &'static str, args: &[&dyn std::fmt::Display]) -> String {
    use std::fmt::Write;
    let mut text = String::with_capacity(template.len() + 8 * args.len());
    let mut args = args.iter();
    let mut parts = tr(template).split("{}");
    if let Some(first) = parts.next() {
        text.push_str(first);
    }
    for part in parts {
        if let Some(arg) = args.next() {
            let _ = write!(text, "{arg}");
        }
        text.push_str(part);
    }
    text
}

/// Translates a text built from a few known pieces, as "The bass is
/// locked", whose translation depends on all of them together (a
/// Spanish adjective agrees with its noun). The catalog has each whole
/// sentence.
pub fn tr_string(text: String) -> String {
    match Language::current() {
        Language::English => text,
        Language::Spanish => match in_spanish(&text) {
            Some(spanish) => spanish.to_string(),
            None => missed(&text).to_string(),
        },
    }
}

/// The Spanish for `text`, whatever language is chosen: for searches,
/// which find a style by its name in either language.
pub fn spanish(text: &str) -> Option<&'static str> {
    in_spanish(text)
}

/// The Spanish for `text` from the catalog, with each note in it named
/// as the settings say: the catalog writes "{Bb} mayor", which reads "Bb
/// mayor" with letters and "Sib mayor" with solfège.
fn in_spanish(text: &str) -> Option<&'static str> {
    es::translate(text).map(named_notes)
}

/// `text` with its notes in braces named as the settings say, kept for
/// the next time: there are only so many texts and two namings.
fn named_notes(text: &'static str) -> &'static str {
    static NAMED: LazyLock<Mutex<HashMap<(usize, NoteNaming), &'static str>>> =
        LazyLock::new(Mutex::default);
    let has_notes = text
        .match_indices('{')
        .any(|(at, _)| text[at + 1..].starts_with(|c: char| ('A'..='G').contains(&c)));
    if !has_notes {
        return text;
    }
    let key = (text.as_ptr() as usize, NoteNaming::current());
    let mut named = NAMED
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    named
        .entry(key)
        .or_insert_with(|| Box::leak(with_notes(text).into_boxed_str()))
}

/// `text` with each note written in braces, {A} or {G#}, named as the
/// settings say: "as here in {A} minor" reads "as here in La minor" in
/// solfège. Braces around anything else, as in the set {0,1,4}, stay.
pub fn with_notes(text: &str) -> String {
    let mut shown = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(open) = rest.find('{') {
        shown.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        let note = after.find('}').and_then(|close| {
            let name = NoteName::read(&after[..close]).filter(|(_, left)| left.is_empty());
            name.map(|(name, _)| (name, close))
        });
        match note {
            Some((name, close)) => {
                shown.push_str(&name.to_string());
                rest = &after[close + 1..];
            }
            None => {
                shown.push('{');
                rest = after;
            }
        }
    }
    shown.push_str(rest);
    shown
}

/// A name the app gave something, as for a channel, a track or a
/// pattern, in the current language: "Bass" is "Bajo" and "Pattern 2" is
/// "Patrón 2" in Spanish. A name you typed yourself stays as it is.
pub fn tr_name(name: &str) -> Cow<'_, str> {
    if Language::current() == Language::English {
        return Cow::Borrowed(name);
    }
    match spanish_name(name) {
        Some(spanish) => Cow::Owned(spanish),
        None => Cow::Borrowed(name),
    }
}

/// The Spanish for a name the app gives things, or `None` for a name it
/// does not know, as one you typed. Besides the names in the catalog,
/// it knows the ones the app makes from them: with a number after
/// ("Pattern 12", "Bars 1–16"), a copy ("Bass copy"), a part from a bar
/// on ("Verse, bar 9"), a piece of a pattern ("Verse (2)"), a channel
/// a demo adds ("Bolero drums", "Band bass") and an instrument named by
/// a MIDI file's track ("Melody: Flute").
pub fn spanish_name(name: &str) -> Option<String> {
    // A name can read otherwise than the same word in a sentence: the
    // channel "Keys" plays keyboards, not keys of music.
    if let Some(spanish) = in_spanish(&format!("name|{name}")).or_else(|| in_spanish(name)) {
        return Some(spanish.to_string());
    }
    // Numbers and signs, and the letters of a song's sections, as "A"
    // or "B2", read the same in Spanish.
    let section = name.len() <= 3
        && name.starts_with(|c: char| c.is_ascii_uppercase())
        && name[1..].chars().all(|c| c.is_ascii_digit() || c == '\'');
    if section || !name.chars().any(char::is_alphabetic) {
        return Some(name.to_string());
    }
    let digits = |text: &str| {
        text.starts_with(|c: char| c.is_ascii_digit())
            && text.chars().all(|c| c.is_ascii_digit() || c == '–')
    };
    let in_spanish = |template: &'static str, part: &str| {
        Language::Spanish.with(|| spanish_name(part).map(|part| trf(template, &[&part])))
    };
    if let Some((part, bar)) = name.rsplit_once(", bar ")
        && digits(bar)
    {
        return Language::Spanish
            .with(|| spanish_name(part).map(|part| trf("{}, bar {}", &[&part, &bar])));
    }
    if let Some((word, number)) = name.rsplit_once(' ')
        && digits(number)
    {
        return spanish_name(word).map(|word| format!("{word} {number}"));
    }
    if let Some((part, piece)) = name
        .strip_suffix(')')
        .and_then(|name| name.rsplit_once(" ("))
        && digits(piece)
    {
        return spanish_name(part).map(|part| format!("{part} ({piece})"));
    }
    if let Some(original) = name.strip_suffix(" copy") {
        return in_spanish("{} copy", original);
    }
    if let Some(piece) = name.strip_suffix(" drums") {
        return in_spanish("{} drums", piece);
    }
    if let Some(player) = name.strip_prefix("Band ") {
        let mut letters = player.chars();
        let capital: String = letters.next()?.to_uppercase().chain(letters).collect();
        return in_spanish("Band {}", &capital);
    }
    if let Some((track, instrument)) = name.split_once(": ") {
        return spanish_name(instrument).map(|instrument| format!("{track}: {instrument}"));
    }
    None
}

/// The name of a key as the settings say: "C major", "Do major" with
/// solfège, and "Do mayor" in Spanish. A row is named by its form, "row
/// P4 (on E)", and a set by its set class, "set 4-28 on C", as
/// [`Scale::name`](crate::harmony::Scale::name) names them in letters for files and text.
/// A set of three notes is one of Kröpfl's micromodes: "Menor 3 on C".
pub fn key_name(scale: crate::harmony::Scale) -> String {
    use crate::harmony::ScaleKind;
    let root = scale.spell(scale.root).to_string();
    match scale.kind {
        ScaleKind::Row(_) => format!(
            "{} P{} ({} {root})",
            tr("row"),
            scale.root.semitone(),
            tr("on")
        ),
        // Three notes are always one of the micromodes: "Menor 3 on C".
        ScaleKind::Set(set) => match crate::harmony::Micromode::of(set) {
            Some(micromode) => format!("{} {} {root}", micromode.name, tr("on")),
            None => format!(
                "{} {} {} {root}",
                tr("set"),
                set.set_class().name(),
                tr("on")
            ),
        },
        kind => format!("{root} {}", tr(kind.name())),
    }
}

/// `items` as a list in a sentence: "A, B and C", or "A, B y C" in
/// Spanish.
pub fn list(items: &[impl AsRef<str>]) -> String {
    match items.split_last() {
        None => String::new(),
        Some((last, [])) => last.as_ref().to_string(),
        Some((last, rest)) => {
            let rest: Vec<&str> = rest.iter().map(AsRef::as_ref).collect();
            trf("{} and {}", &[&rest.join(", "), &last.as_ref()])
        }
    }
}

fn missed(text: &str) -> &str {
    // Numbers and signs, as "1/16", read the same in every language.
    if !text.chars().any(char::is_alphabetic) {
        return text;
    }
    MISSED.with(|missed| {
        if let Some(missed) = missed.borrow_mut().as_mut() {
            missed.insert(text.to_string());
        }
    });
    text
}

/// From now on, this thread writes down each text it is asked for in
/// Spanish and has no Spanish for, until [`take_missed`]. For tests.
pub fn note_missed() {
    MISSED.with(|missed| {
        missed.borrow_mut().get_or_insert_with(BTreeSet::new);
    });
}

/// The texts with no Spanish this thread was asked for since
/// [`note_missed`], which it stops writing down.
pub fn take_missed() -> BTreeSet<String> {
    MISSED.with(|missed| missed.borrow_mut().take().unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_until_spanish_is_chosen() {
        assert_eq!(tr("File"), "File");
        Language::Spanish.with(|| {
            assert_eq!(tr("File"), "Archivo");
            assert_eq!(tr("Untranslated String"), "Untranslated String");
        });
        assert_eq!(tr("File"), "File", "with() puts English back");
    }

    #[test]
    fn numbers_and_names_go_where_the_translation_puts_them() {
        assert_eq!(trf("{} and {}", &[&"A, B", &"C"]), "A, B and C");
        Language::Spanish.with(|| {
            assert_eq!(list(&["Bajo", "Batería", "Piano"]), "Bajo, Batería y Piano");
            assert_eq!(tr_name("Pattern 12"), "Patrón 12");
            assert_eq!(tr_name("My riff 2"), "My riff 2");
        });
    }

    #[test]
    fn texts_with_no_spanish_are_written_down_when_asked() {
        Language::Spanish.with(|| {
            tr("Not in the catalog");
            assert!(take_missed().is_empty(), "not asked to");
            note_missed();
            tr("File");
            tr("Not in the catalog");
            assert_eq!(
                take_missed().into_iter().collect::<Vec<_>>(),
                ["Not in the catalog"]
            );
        });
    }
}
