//! The language and the note naming chosen in Settings, kept with the
//! window's preferences, and the names of keys and notes as they say.
//! The translation itself, [`tr`] and the Spanish catalog, is in
//! [`lineup_core::i18n`], so that texts the core writes are translated
//! too.

use eframe::egui;
use lineup_core::NoteNaming;

pub use lineup_core::i18n::{
    Language, key_name, list, tr, tr_name, tr_str, tr_string, trc, trf, with_notes,
};

fn language_id() -> egui::Id {
    egui::Id::unique("app-language")
}

fn note_naming_id() -> egui::Id {
    egui::Id::unique("app-note-naming")
}

/// The language chosen in Settings > Appearance, kept with the window's
/// other preferences.
pub fn language(ctx: &egui::Context) -> Language {
    ctx.data_mut(|data| data.get_persisted(language_id()).unwrap_or_default())
}

/// Chooses the interface language, from this frame on.
pub fn set_language(ctx: &egui::Context, language: Language) {
    language.set();
    ctx.data_mut(|data| data.insert_persisted(language_id(), language));
}

/// The note naming chosen in Settings > Harmony.
pub fn note_naming(ctx: &egui::Context) -> NoteNaming {
    ctx.data_mut(|data| data.get_persisted(note_naming_id()).unwrap_or_default())
}

/// Chooses how notes and keys are named, from this frame on.
pub fn set_note_naming(ctx: &egui::Context, naming: NoteNaming) {
    naming.set();
    ctx.data_mut(|data| data.insert_persisted(note_naming_id(), naming));
}

/// Makes this thread write in the language and name notes the way the
/// settings say. The window calls it before drawing each frame.
pub fn apply_settings(ctx: &egui::Context) {
    language(ctx).set();
    note_naming(ctx).set();
}

/// A keyboard shortcut written as the keys are named in the current
/// language: "Ctrl+Shift+Z", and in Spanish "Ctrl+Mayús+Z", "Supr" for
/// Delete.
pub fn shortcut(ctx: &egui::Context, shortcut: &egui::KeyboardShortcut) -> String {
    // The keys whose names differ by language; letters, digits, F1 and
    // the like read the same.
    const NAMED: [&str; 14] = [
        "Ctrl",
        "Shift",
        "Alt",
        "Space",
        "Delete",
        "Backspace",
        "Escape",
        "Enter",
        "Tab",
        "Home",
        "End",
        "PageUp",
        "PageDown",
        "Insert",
    ];
    let name = |key: &str| -> String {
        match NAMED.iter().find(|&&named| named == key) {
            Some(&named) => trc("key", named).to_string(),
            None => key.to_string(),
        }
    };
    let text = ctx.format_shortcut(shortcut);
    if current_language() == Language::English {
        return text;
    }
    let mut shown = String::new();
    let mut rest = text.as_str();
    while let Some((modifier, after)) = rest.split_once('+')
        && ["Ctrl", "Shift", "Alt", "Cmd", "Option"].contains(&modifier)
    {
        shown.push_str(&name(modifier));
        shown.push('+');
        rest = after;
    }
    shown.push_str(&name(rest));
    shown
}

/// The language this thread writes the interface in.
pub fn current_language() -> Language {
    Language::current()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_by_default() {
        assert_eq!(Language::current(), Language::English);
        assert_eq!(tr("File"), "File");
        assert_eq!(tr("Edit"), "Edit");
    }

    #[test]
    fn spanish_translation() {
        Language::Spanish.with(|| {
            assert_eq!(Language::current(), Language::Spanish);
            assert_eq!(tr("File"), "Archivo");
            assert_eq!(tr("Edit"), "Edición");
            assert_eq!(tr("View"), "Ver");
            assert_eq!(tr("Help"), "Ayuda");
            assert_eq!(tr("Save"), "Guardar");
            assert_eq!(tr("Settings"), "Ajustes");
            assert_eq!(tr("Chord sheet"), "Hoja de acordes");
            assert_eq!(tr("Channel rack"), "Rack de canales");
            assert_eq!(tr("Piano roll"), "Piano roll");
            assert_eq!(tr("Mixer"), "Mezclador");
            assert_eq!(tr("Master"), "Principal");
            assert_eq!(tr("Untranslated String"), "Untranslated String");
        });
        assert_eq!(tr("File"), "File", "with() puts English back");
    }

    #[test]
    fn settings_are_kept_and_applied() {
        let ctx = egui::Context::default();
        assert_eq!(language(&ctx), Language::English);
        assert_eq!(note_naming(&ctx), NoteNaming::Letters);
        set_language(&ctx, Language::Spanish);
        set_note_naming(&ctx, NoteNaming::Solfege);
        Language::English.set();
        NoteNaming::Letters.set();
        apply_settings(&ctx);
        assert_eq!(Language::current(), Language::Spanish);
        assert_eq!(NoteNaming::current(), NoteNaming::Solfege);
        // This thread only: put it back for the next test it runs.
        Language::English.set();
        NoteNaming::Letters.set();
    }

    #[test]
    fn solfege_is_a_way_of_showing_names() {
        use lineup_core::PitchClass;
        use lineup_core::chord_sheet::{ChordMarker, HarmonyName};
        use lineup_core::harmony::{Scale, ScaleKind};

        let marker = |text: &str| ChordMarker::new(0, HarmonyName::parse(text).unwrap());
        let b_flat = Scale::new(PitchClass::ASharp, ScaleKind::Major);
        let d_dorian = Scale::new(PitchClass::D, ScaleKind::Dorian);
        assert_eq!(key_name(b_flat), "Bb major");
        assert_eq!(marker("Am7").text(), "Am7");
        NoteNaming::Solfege.with(|| {
            assert_eq!(marker("Am7").text(), "Lam7");
            assert_eq!(marker("G/B").text(), "Sol/Si");
            // Typed either way, it is the same chord.
            assert_eq!(marker("Lam7"), marker("Am7"));
            assert_eq!(key_name(b_flat), "Sib major");
            Language::Spanish.with(|| {
                assert_eq!(key_name(b_flat), "Sib mayor");
                assert_eq!(key_name(d_dorian), "Re dórico");
            });
        });
        Language::Spanish.with(|| assert_eq!(key_name(b_flat), "Bb mayor"));
        // Notes in braces in a text show as the settings name them.
        let text = "as in {C} / / {G#}7, or a set {0,1,4}";
        assert_eq!(with_notes(text), "as in C / / G#7, or a set {0,1,4}");
        NoteNaming::Solfege.with(|| {
            assert_eq!(with_notes(text), "as in Do / / Sol#7, or a set {0,1,4}");
        });
    }
}
