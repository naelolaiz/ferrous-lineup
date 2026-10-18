//! The Demos submenu of the File menu: the demos in alphabetical order,
//! and a search box above them.
//!
//! Every list in the menu, the genres too, is sorted by the title it
//! shows, ignoring capitals, accents and punctuation: "Für Elise" comes
//! after "Funeral March", as if it were written "Fur Elise", and "Oh My
//! Darling, Clementine" before "Oh! Susanna". The lists in the library's
//! source files stay in the order they were written; only the menu sorts
//! them.
//!
//! Typing in the search box swaps the submenus for one list of the demos
//! that match: songs, styles, twelve-tone and micromode pieces alike. A
//! search looks through each demo's title, who wrote it and when, its
//! feel and genre, its key and time signature, and last of all the notes
//! about it. Every word typed has to be found, at the start of a word:
//! "bach" finds Bach, not Offenbach. Demos whose title starts with what
//! was typed come first. Weaker matches come last, and fainter: the
//! words typed found apart ("d minor" in Hungarian Dance No. 5, which
//! is in G minor), or only in the notes.

use std::cmp::Ordering;
use std::sync::LazyLock;

use eframe::egui;
use lineup_core::harmony::{Scale, ScaleKind};
use lineup_core::library::{Genre, MICROMODE_STYLES, ROW_STYLES, RowStyle, SONGS, STYLES};

use super::LineupApp;
use super::files::Pending;
use super::library::Demo;
use crate::theme;

/// A demo as the menu lists it and a search finds it.
struct Entry {
    demo: Demo,
    /// What its button says, such as "El Choclo · Tango".
    label: String,
    /// Where the menu keeps it, such as "Songs ▸ Tango".
    place: String,
    hover: String,
    /// The title, folded (see [`fold`]): what the menu sorts by and a
    /// search looks at first.
    title: String,
    /// Who wrote it, its feel, genre, key and time signature, folded.
    details: String,
    /// The notes about it, folded.
    about: String,
}

impl Entry {
    fn new(demo: Demo, label: String, place: String, hover: String) -> Entry {
        Entry {
            demo,
            label,
            place,
            hover,
            title: fold(demo.name()),
            details: String::new(),
            about: String::new(),
        }
    }

    fn details(self, details: &[&str]) -> Entry {
        Entry {
            details: fold(&details.join(" ")),
            ..self
        }
    }

    fn about(self, about: &[&str]) -> Entry {
        Entry {
            about: fold(&about.join(" ")),
            ..self
        }
    }

    /// How well the entry matches a search for `words`, best first, or
    /// `None` if one of the words starts no word of the entry.
    fn rank(&self, words: &[String]) -> Option<Rank> {
        let phrase = words.join(" ");
        let in_title = |word: &String| starts_a_word(&self.title, word);
        let in_details = |word: &String| in_title(word) || starts_a_word(&self.details, word);
        let anywhere = |word: &String| in_details(word) || starts_a_word(&self.about, word);
        if words.is_empty() || !words.iter().all(anywhere) {
            None
        } else if self.title.starts_with(&phrase) {
            Some(Rank::TitleStart)
        } else if words.iter().all(in_title) {
            Some(Rank::TitleWords)
        } else if starts_a_word(&self.details, &phrase) {
            Some(Rank::Details)
        } else if words.iter().all(in_details) {
            Some(Rank::Words)
        } else {
            Some(Rank::About)
        }
    }
}

/// Where a search found a demo, from the best match to the weakest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    /// The title starts with what was typed: "choc" in "El Choclo"
    /// does not, "el ch" does.
    TitleStart,
    /// Every word typed starts a word of the title: "choc" in "El Choclo".
    TitleWords,
    /// What was typed is in who wrote it, the feel, genre, key or time,
    /// as typed: "d minor" in a song in D minor.
    Details,
    /// Each word typed is in the title or those, but apart: "d minor"
    /// in Hungarian Dance No. 5, in G minor. Shown fainter, as is `About`.
    Words,
    /// Some word typed is only in the notes about it.
    About,
}

/// Every demo, sorted by title, and the genres, sorted by name.
struct Demos {
    entries: Vec<Entry>,
    genres: Vec<Genre>,
}

static DEMOS: LazyLock<Demos> = LazyLock::new(|| {
    let mut genres = Genre::ALL.to_vec();
    genres.sort_by(|a, b| by_title(a.name(), b.name()));
    let mut entries = all_entries();
    entries.sort_by(|a, b| {
        a.title
            .cmp(&b.title)
            .then_with(|| a.label.cmp(&b.label))
            .then_with(|| a.place.cmp(&b.place))
    });
    Demos { entries, genres }
});

fn all_entries() -> Vec<Entry> {
    let mut entries = vec![Entry::new(
        Demo::Start,
        "The starting song".into(),
        String::new(),
        "The beat and chords Ferrous Lineup opens with".into(),
    )];
    for song in SONGS {
        let scale = Scale::new(song.tonic, song.mode);
        let tonic = scale.spell(song.tonic);
        let mut key = format!("{tonic} {}", song.mode.name());
        // A song in D harmonic minor is in D minor too.
        if matches!(
            song.mode,
            ScaleKind::HarmonicMinor | ScaleKind::MelodicMinor
        ) {
            key.push_str(&format!(" {tonic} minor"));
        }
        let time = song.time.to_string();
        let entry = Entry::new(
            Demo::Song(song),
            format!("{} · {}", song.name, song.feel),
            format!("Songs ▸ {}", song.genre.name()),
            format!("{}\n\n{}", song.origin, song.about),
        );
        entries.push(
            entry
                .details(&[song.origin, song.feel, song.genre.name(), &key, &time])
                .about(&[song.about]),
        );
    }
    for style in STYLES {
        let time = style.time.to_string();
        let entry = Entry::new(
            Demo::Style(style),
            style.name.into(),
            format!("Styles ▸ {}", style.genre.name()),
            style.about.into(),
        );
        entries.push(
            entry
                .details(&[style.genre.name(), style.mode.name(), style.rhythm, &time])
                .about(&[style.about]),
        );
    }
    for style in &ROW_STYLES {
        let time = style.time.to_string();
        let entry = Entry::new(
            Demo::Row(style),
            style.name.into(),
            "Twelve-tone".into(),
            style.about.into(),
        );
        entries.push(
            entry
                .details(&["twelve-tone row", style.row, &time])
                .about(&[style.about]),
        );
    }
    for style in &MICROMODE_STYLES {
        let micromode = style.micromode();
        let time = style.time.to_string();
        let entry = Entry::new(
            Demo::Micromode(style),
            format!("{} · {}", style.name, micromode.name),
            "Micromodes".into(),
            format!("{}\n\n{}: {}", style.about, micromode.name, micromode.about),
        );
        entries.push(
            entry
                .details(&["micromode", micromode.name, micromode.english, &time])
                .about(&[style.about, micromode.about]),
        );
    }
    entries
}

/// Writes `text` the way the menu sorts and searches it: in small
/// letters, without accents, and without punctuation, so that "Für" is
/// found by typing "fur", and "St. James" by "st james". A hyphen is a
/// space, so "Tico-Tico" is found by "tico". Sharps ("F#"),
/// time signatures ("3/4") and grouped beats ("2+2+3/8") keep their marks.
fn fold(text: &str) -> String {
    let mut folded = String::with_capacity(text.len());
    for c in text.chars().flat_map(char::to_lowercase) {
        match c {
            'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'ā' | 'ă' | 'ą' => folded.push('a'),
            'æ' => folded.push_str("ae"),
            'ç' | 'ć' | 'ĉ' | 'ċ' | 'č' => folded.push('c'),
            'ď' | 'đ' | 'ð' => folded.push('d'),
            'è' | 'é' | 'ê' | 'ë' | 'ē' | 'ĕ' | 'ė' | 'ę' | 'ě' => folded.push('e'),
            'ĝ' | 'ğ' | 'ġ' | 'ģ' => folded.push('g'),
            'ĥ' | 'ħ' => folded.push('h'),
            'ì' | 'í' | 'î' | 'ï' | 'ĩ' | 'ī' | 'ĭ' | 'į' | 'ı' => folded.push('i'),
            'ĵ' => folded.push('j'),
            'ķ' => folded.push('k'),
            'ĺ' | 'ļ' | 'ľ' | 'ŀ' | 'ł' => folded.push('l'),
            'ñ' | 'ń' | 'ņ' | 'ň' => folded.push('n'),
            'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' | 'ō' | 'ŏ' | 'ő' => folded.push('o'),
            'œ' => folded.push_str("oe"),
            'ŕ' | 'ŗ' | 'ř' => folded.push('r'),
            'ś' | 'ŝ' | 'ş' | 'š' | 'ș' => folded.push('s'),
            'ß' => folded.push_str("ss"),
            'ţ' | 'ť' | 'ŧ' | 'ț' => folded.push('t'),
            'ù' | 'ú' | 'û' | 'ü' | 'ũ' | 'ū' | 'ŭ' | 'ů' | 'ű' | 'ų' => folded.push('u'),
            'ŵ' => folded.push('w'),
            'ý' | 'ÿ' | 'ŷ' => folded.push('y'),
            'ź' | 'ż' | 'ž' => folded.push('z'),
            '♯' => folded.push('#'),
            '♭' => folded.push('b'),
            c if c.is_alphanumeric() || matches!(c, '#' | '/' | '+') => folded.push(c),
            // A space, or a mark that stands between words, as in "Für
            // Elise · Piano" or "Tico-Tico": one space between words,
            // however many.
            c if (c.is_whitespace()
                || matches!(c, '-' | '–' | '—' | '(' | ')' | '·' | '▸' | ':' | ';'))
                && !folded.is_empty()
                && !folded.ends_with(' ') =>
            {
                folded.push(' ');
            }
            // Any other mark is dropped: "Won't", "St. James", "Oh!
            // Susanna", "Row, Row, Row Your Boat".
            _ => {}
        }
    }
    let kept = folded.trim_end().len();
    folded.truncate(kept);
    folded
}

/// Compares two titles the way the menu sorts them.
fn by_title(a: &str, b: &str) -> Ordering {
    fold(a).cmp(&fold(b)).then_with(|| a.cmp(b))
}

/// Whether `words` is in `text` where a word starts: "bach" is in
/// "johann sebastian bach", but not in "jacques offenbach".
fn starts_a_word(text: &str, words: &str) -> bool {
    text.match_indices(words)
        .any(|(at, _)| at == 0 || text[..at].ends_with(' '))
}

/// The demos a search for `query` finds, best match first, with how well
/// each matched. Matches that are as good as each other keep the menu's
/// alphabetical order.
fn search(query: &str) -> Vec<(&'static Entry, Rank)> {
    let words: Vec<String> = fold(query).split(' ').map(str::to_owned).collect();
    let words: Vec<String> = words.into_iter().filter(|word| !word.is_empty()).collect();
    let mut found: Vec<(&'static Entry, Rank)> = DEMOS
        .entries
        .iter()
        .filter_map(|entry| entry.rank(&words).map(|rank| (entry, rank)))
        .collect();
    // A stable sort, so equal ranks stay in alphabetical order.
    found.sort_by_key(|&(_, rank)| rank);
    found
}

/// The ids under which the menu keeps what was typed, and the last pass
/// it was shown in.
fn query_id() -> egui::Id {
    egui::Id::unique("demo-search")
}

fn shown_id() -> egui::Id {
    egui::Id::unique("demo-search-shown")
}

impl LineupApp {
    /// The Demos submenu of the File menu.
    pub(super) fn demos_menu(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        // A click in the search box must not close the menu.
        let stay_open = egui::containers::menu::MenuConfig::new()
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside);
        let menu = egui::containers::menu::SubMenuButton::new("Demos").config(stay_open);
        let (_, picked) = menu.ui(ui, demos);
        let picked = picked.and_then(|inner| inner.inner);
        if let Some(demo) = picked {
            ui.close();
            self.request(&ctx, Pending::Demo(demo));
        }
    }
}

/// What the Demos submenu shows, and the demo picked in it, if any.
fn demos(ui: &mut egui::Ui) -> Option<Demo> {
    // Each time the menu opens, the search box starts empty and takes the
    // keyboard, so you can type straight away.
    let pass = ui.ctx().cumulative_pass_nr();
    let last = ui.ctx().data(|data| data.get_temp::<u64>(shown_id()));
    ui.ctx().data_mut(|data| data.insert_temp(shown_id(), pass));
    let opened = last != Some(pass.saturating_sub(1)) && last != Some(pass);
    let mut query = if opened {
        String::new()
    } else {
        ui.ctx()
            .data(|data| data.get_temp::<String>(query_id()))
            .unwrap_or_default()
    };
    // The down arrow goes from the box to the first match. The box would
    // take it for itself, so it is taken first; from there on, the menu's
    // own arrow keys go up and down the matches.
    let field_id = egui::Id::unique("demo-search-field");
    let down = ui.memory(|memory| memory.has_focus(field_id))
        && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown));
    let field = ui.add(
        egui::TextEdit::singleline(&mut query)
            .id(field_id)
            .hint_text("🔍 Search demos")
            .desired_width(240.0),
    );
    if opened {
        field.request_focus();
    }
    let field = theme::hint(
        field,
        "Find a demo by its title, composer, genre, key or time signature, \
         such as \"choclo\", \"bach\", \"tango\", \"D minor\" or \"3/4\". \
         Enter opens the first one; the arrow keys go through the rest",
    );
    let enter = field.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
    ui.ctx()
        .data_mut(|data| data.insert_temp(query_id(), query.clone()));

    if query.trim().is_empty() {
        ui.separator();
        return listed(ui);
    }
    let found = search(&query);
    if enter && let Some((entry, _)) = found.first() {
        return Some(entry.demo);
    }
    ui.separator();
    if found.is_empty() {
        ui.weak("No demo matches");
        return None;
    }
    let mut picked = None;
    // The menu keeps the size it had for the last search unless told how
    // tall the new matches are: as tall as they are, up to most of the
    // window, and scrolled beyond that.
    let row = ui.spacing().interact_size.y + ui.spacing().item_spacing.y;
    let height = (found.len() as f32 * row).min(ui.ctx().content_rect().height() * 0.6);
    egui::ScrollArea::vertical()
        .max_height(height)
        .min_scrolled_height(height)
        .show(ui, |ui| {
            // One line each: the menu widens instead.
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
            for (index, (entry, rank)) in found.iter().copied().enumerate() {
                // Weaker matches are shown fainter: the words typed were
                // found apart, or only in the notes.
                let label = if rank >= Rank::Words {
                    egui::RichText::new(&entry.label).weak()
                } else {
                    egui::RichText::new(&entry.label)
                };
                let place = egui::RichText::new(&entry.place).weak();
                let button = egui::Button::new(label).right_text(place);
                let response = ui.add(button);
                if down && index == 0 {
                    response.request_focus();
                }
                if response.on_hover_text(&entry.hover).clicked() {
                    picked = Some(entry.demo);
                }
            }
        });
    picked
}

/// The demos in their submenus, each list in alphabetical order.
fn listed(ui: &mut egui::Ui) -> Option<Demo> {
    let demos = &*DEMOS;
    let mut picked = None;
    let mut button = |ui: &mut egui::Ui, entry: &Entry, label: &str| {
        if ui.button(label).on_hover_text(&entry.hover).clicked() {
            picked = Some(entry.demo);
        }
    };
    if let Some(start) = demos.entries.iter().find(|e| e.demo == Demo::Start) {
        button(ui, start, &start.label);
    }
    ui.separator();
    ui.menu_button("Songs", |ui| {
        ui.weak("Tunes with their melodies, free for everyone to use");
        for &genre in &demos.genres {
            let songs: Vec<&Entry> = demos
                .entries
                .iter()
                .filter(|e| matches!(e.demo, Demo::Song(song) if song.genre == genre))
                .collect();
            // A genre with no songs gets no empty submenu.
            if songs.is_empty() {
                continue;
            }
            ui.menu_button(genre.name(), |ui| {
                for entry in songs {
                    button(ui, entry, &entry.label);
                }
            });
        }
    });
    ui.menu_button("Styles", |ui| {
        ui.weak("Each harmony style as a short song");
        for &genre in &demos.genres {
            let styles: Vec<&Entry> = demos
                .entries
                .iter()
                .filter(|e| matches!(e.demo, Demo::Style(style) if style.genre == genre))
                .collect();
            if styles.is_empty() {
                continue;
            }
            ui.menu_button(genre.name(), |ui| {
                for entry in styles {
                    button(ui, entry, &entry.label);
                }
            });
        }
    });
    ui.menu_button("Twelve-tone", |ui| {
        ui.weak("Each row style as a short piece: the row, backwards, upside down, both");
        for entry in demos
            .entries
            .iter()
            .filter(|e| matches!(e.demo, Demo::Row(_)))
        {
            button(ui, entry, &entry.label);
        }
    });
    ui.menu_button("Micromodes", |ui| {
        ui.weak(
            "Each micromode style as a short piece: from the micromode, upside down, \
             a tritone away, upside down there, and back",
        );
        for entry in demos
            .entries
            .iter()
            .filter(|e| matches!(e.demo, Demo::Micromode(_)))
        {
            button(ui, entry, &entry.label);
        }
        ui.separator();
        ui.weak("A famous row made of one micromode");
        if let Some(style) = RowStyle::named("Webern's trichords")
            && let Some(entry) = demos.entries.iter().find(|e| e.demo == Demo::Row(style))
        {
            button(ui, entry, &format!("{} · Menor 3", style.name));
        }
    });
    picked
}

#[cfg(test)]
mod tests {
    use super::*;
    use lineup_core::library::Song;
    use lineup_core::pitch::PitchClass;

    fn names(found: &[(&Entry, Rank)]) -> Vec<&'static str> {
        found.iter().map(|(entry, _)| entry.demo.name()).collect()
    }

    #[test]
    fn folding_drops_capitals_accents_and_punctuation() {
        assert_eq!(fold("Für Elise"), "fur elise");
        assert_eq!(fold("Funiculì, funiculà"), "funiculi funicula");
        assert_eq!(fold("Oh! Susanna"), "oh susanna");
        assert_eq!(fold("St. James Infirmary"), "st james infirmary");
        assert_eq!(fold("Won't You  Please"), "wont you please");
        assert_eq!(
            fold("Shchedryk (Carol of the Bells)"),
            "shchedryk carol of the bells"
        );
        assert_eq!(fold("Prelude in C-sharp minor"), "prelude in c sharp minor");
        assert_eq!(fold("Tico-Tico no Fubá"), "tico tico no fuba");
        assert_eq!(fold("F# minor, 2+2+3/8"), "f# minor 2+2+3/8");
        assert_eq!(
            fold("Mo Li Hua (Jasmine Flower)"),
            "mo li hua jasmine flower"
        );
    }

    #[test]
    fn titles_sort_without_minding_capitals_accents_or_punctuation() {
        let mut titles = vec![
            "Oh! Susanna",
            "Zamba del telar",
            "Ábalos",
            "Oh My Darling, Clementine",
            "alma llanera",
            "El Choclo",
            "Dies irae (Mozart)",
            "Dies Irae",
        ];
        titles.sort_by(|a, b| by_title(a, b));
        assert_eq!(
            titles,
            [
                "Ábalos",
                "alma llanera",
                "Dies Irae",
                "Dies irae (Mozart)",
                "El Choclo",
                "Oh My Darling, Clementine",
                "Oh! Susanna",
                "Zamba del telar",
            ]
        );
    }

    #[test]
    fn every_demo_is_listed_once_in_alphabetical_order() {
        let entries = &DEMOS.entries;
        let demos = 1 + SONGS.len() + STYLES.len() + ROW_STYLES.len() + MICROMODE_STYLES.len();
        assert_eq!(entries.len(), demos);
        for pair in entries.windows(2) {
            assert!(
                pair[0].title <= pair[1].title,
                "{} before {}",
                pair[0].label,
                pair[1].label
            );
        }
        // So each genre's submenu, which keeps this order, is sorted too.
        for genre in Genre::ALL {
            let songs: Vec<&str> = entries
                .iter()
                .filter_map(|e| match e.demo {
                    Demo::Song(song) if song.genre == genre => Some(song.name),
                    _ => None,
                })
                .collect();
            let mut sorted = songs.clone();
            sorted.sort_by(|a, b| by_title(a, b));
            assert_eq!(songs, sorted, "{}", genre.name());
        }
    }

    #[test]
    fn genres_are_in_alphabetical_order() {
        let names: Vec<&str> = DEMOS.genres.iter().map(|genre| genre.name()).collect();
        assert_eq!(names.len(), Genre::ALL.len());
        assert_eq!(names.first(), Some(&"Balkan and Turkish"));
        assert_eq!(names.last(), Some(&"Tango"));
        for pair in names.windows(2) {
            assert!(
                fold(pair[0]) < fold(pair[1]),
                "{} before {}",
                pair[0],
                pair[1]
            );
        }
    }

    #[test]
    fn a_search_finds_titles_first() {
        let found = search("choclo");
        assert_eq!(found.first().map(|(e, _)| e.demo.name()), Some("El Choclo"));
        assert_eq!(found[0].1, Rank::TitleWords);

        // Accents and capitals need not be typed.
        assert_eq!(names(&search("FUR ELISE")).first(), Some(&"Für Elise"));
        assert_eq!(names(&search("condor")).first(), Some(&"El cóndor pasa"));

        // Titles that start with what was typed, such as "Prelude in C",
        // come before titles that only have it further in, such as
        // "Cello Suite No. 1 Prelude".
        let found = search("prelude");
        assert_eq!(found[0].1, Rank::TitleStart);
        let cello = names(&found)
            .iter()
            .position(|n| *n == "Cello Suite No. 1 Prelude");
        let prelude = names(&found).iter().position(|n| *n == "Prelude in C");
        assert!(prelude < cello, "{:?}", names(&found));
    }

    #[test]
    fn a_search_needs_every_word() {
        let found = search("prelude minor");
        assert!(!found.is_empty());
        for (entry, _) in &found {
            let all = format!("{} {} {}", entry.title, entry.details, entry.about);
            assert!(
                all.contains("prelude") && all.contains("minor"),
                "{}",
                entry.label
            );
        }
        assert!(search("zzzz qqqq").is_empty());
        assert!(search("   ").is_empty());
    }

    #[test]
    fn a_search_finds_composers_genres_keys_and_time_signatures() {
        // Who wrote it, as a word: Bach, but not Offenbach.
        let bach = search("bach");
        assert!(
            names(&bach).contains(&"Air on the G String"),
            "{:?}",
            names(&bach)
        );
        assert!(!names(&bach).contains(&"Can-can"), "{:?}", names(&bach));
        // A genre brings its songs and its styles.
        let tango = search("tango");
        assert!(tango.iter().any(|(e, _)| matches!(e.demo, Demo::Song(_))));
        assert!(tango.iter().any(|(e, _)| matches!(e.demo, Demo::Style(_))));
        // A time signature.
        for (entry, _) in search("3/4") {
            if let Demo::Song(song) = entry.demo {
                // Found by its time, its title or its notes.
                let all = format!("{} {}", song.time, entry.about);
                assert!(all.contains("3/4"), "{}", song.name);
            }
        }
        assert!(
            search("3/4")
                .iter()
                .any(|(e, _)| matches!(e.demo, Demo::Song(s) if s.name == "The Blue Danube"))
        );
        // A key: every song that starts in D minor, whatever its title,
        // and whichever minor scale it is written in.
        let d_minor = search("d minor");
        let in_d_minor: Vec<&Song> = SONGS
            .iter()
            .filter(|song| {
                song.tonic == PitchClass::D
                    && matches!(
                        song.mode,
                        ScaleKind::NaturalMinor
                            | ScaleKind::HarmonicMinor
                            | ScaleKind::MelodicMinor
                    )
            })
            .collect();
        assert!(!in_d_minor.is_empty());
        for song in in_d_minor {
            let rank = d_minor.iter().find_map(|&(e, rank)| match e.demo {
                Demo::Song(s) if s.name == song.name => Some(rank),
                _ => None,
            });
            // Found as a key, ahead of songs that only have a "d" word
            // and a "minor" somewhere.
            assert!(
                rank.is_some_and(|rank| rank <= Rank::Details),
                "{}",
                song.name
            );
        }
        for &(entry, rank) in &d_minor {
            if let Demo::Song(song) = entry.demo
                && rank == Rank::Details
            {
                assert_eq!(song.tonic, PitchClass::D, "{}", song.name);
            }
        }
        // A "d" word and a "minor" apart are a weaker match.
        let dance = d_minor
            .iter()
            .find(|(e, _)| e.demo.name() == "Hungarian Dance No. 5");
        assert_eq!(dance.map(|&(_, rank)| rank), Some(Rank::Words));
        // The "d" of "prelude" is not the start of a word.
        let e_minor = d_minor
            .iter()
            .find(|(e, _)| e.demo.name() == "Prelude in E minor");
        assert!(
            e_minor.is_none_or(|&(_, rank)| rank == Rank::About),
            "{:?}",
            e_minor.map(|(_, rank)| rank)
        );
        // A micromode by its name.
        assert!(
            search("menor 3")
                .iter()
                .any(|(e, _)| matches!(e.demo, Demo::Micromode(_)))
        );
    }

    #[test]
    fn matches_only_in_the_notes_come_last() {
        let found = search("tango");
        let ranks: Vec<Rank> = found.iter().map(|&(_, rank)| rank).collect();
        let mut sorted = ranks.clone();
        sorted.sort();
        assert_eq!(ranks, sorted);
    }
}
