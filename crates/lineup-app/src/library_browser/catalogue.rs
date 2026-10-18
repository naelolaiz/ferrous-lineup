//! Shared names, searches and ordering for the Library and chord sheet menus.

use lineup_core::TimeSignature;
use lineup_core::library::{Genre, RHYTHMS, Rhythm, STYLES, Style};

pub fn alphabetical<T>(
    entries: impl IntoIterator<Item = T>,
    name: impl Fn(&T) -> &'static str,
) -> Vec<T> {
    let mut entries: Vec<_> = entries.into_iter().collect();
    entries.sort_by_cached_key(|entry| (name(entry).to_lowercase(), name(entry)));
    entries
}

pub fn genres() -> Vec<Genre> {
    alphabetical(Genre::ALL, |genre| genre.name())
}

pub fn matches(query: &str, name: &str, genre: Genre, time: TimeSignature, about: &str) -> bool {
    let text = format!(
        "{name} {} {time} {}/{} {about}",
        genre.name(),
        time.beats,
        time.unit
    )
    .to_lowercase();
    query
        .to_lowercase()
        .split_whitespace()
        .all(|word| text.contains(word))
}

pub fn harmony_styles(query: &str, genre: Option<Genre>) -> Vec<&'static Style> {
    alphabetical(
        STYLES.iter().filter(|style| {
            genre.is_none_or(|genre| style.genre == genre)
                && matches(query, style.name, style.genre, style.time, style.about)
        }),
        |style| style.name,
    )
}

pub fn rhythms(query: &str, genre: Option<Genre>) -> Vec<&'static Rhythm> {
    alphabetical(
        RHYTHMS.iter().filter(|rhythm| {
            genre.is_none_or(|genre| rhythm.genre == genre)
                && matches(query, rhythm.name, rhythm.genre, rhythm.time, rhythm.about)
        }),
        |rhythm| rhythm.name,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use lineup_core::BandStyle;

    #[test]
    fn searches_find_sidebar_styles_by_name_genre_and_meter() {
        let found = harmony_styles("  FOUR chords ", None);
        assert!(found.iter().any(|style| style.name == "Four chords"));
        let found = harmony_styles("balkan 7/8", None);
        assert!(!found.is_empty());
        assert!(
            found
                .iter()
                .all(|style| style.genre == Genre::Balkan && style.time.beats == 7)
        );
        assert!(harmony_styles("no such style xyz", None).is_empty());
    }

    #[test]
    fn genre_and_style_lists_are_alphabetical() {
        let genres = genres();
        assert_eq!(genres.first(), Some(&Genre::Balkan));
        assert_eq!(genres.last(), Some(&Genre::Tango));
        let styles = harmony_styles("", Some(Genre::Pop));
        let place = |name| styles.iter().position(|style| style.name == name).unwrap();
        for pair in [
            "Doo-wop",
            "Emotional pop",
            "Four chords",
            "Piano ballad",
            "Pop minor",
            "Pop punk",
            "Royal road",
        ]
        .windows(2)
        {
            assert!(place(pair[0]) < place(pair[1]));
        }
    }

    #[test]
    fn every_sidebar_style_and_rhythm_can_be_chosen_for_the_band() {
        for style in harmony_styles("", None) {
            assert!(BandStyle::of_style(style).is_some(), "{}", style.name);
        }
        for rhythm in rhythms("", None) {
            assert!(BandStyle::named(rhythm.name).is_some(), "{}", rhythm.name);
        }
    }
}
