//! Shared names, searches and ordering for the Library and chord sheet menus.

use lineup_core::TimeSignature;
use lineup_core::library::{Genre, RHYTHMS, Rhythm, STYLES, Style, UserStyle};

/// One catalogue, regardless of whether a preset supplies a chord chart.
#[derive(Clone, Copy)]
pub enum Preset<'a> {
    Style(&'static Style),
    Rhythm(&'static Rhythm),
    User(usize, &'a UserStyle),
}

impl<'a> Preset<'a> {
    pub fn name(self) -> &'a str {
        match self {
            Self::Style(style) => style.name,
            Self::Rhythm(rhythm) => rhythm.name,
            Self::User(_, style) => &style.name,
        }
    }

    pub fn time(self) -> TimeSignature {
        match self {
            Self::Style(style) => style.time,
            Self::Rhythm(rhythm) => rhythm.time,
            Self::User(_, style) => style.time,
        }
    }

    pub fn has_progression(self) -> bool {
        match self {
            Self::Style(style) => !style.chart.trim().is_empty(),
            Self::Rhythm(_) => false,
            Self::User(_, style) => !style.chart.trim().is_empty(),
        }
    }
}

pub fn presets<'a>(
    query: &str,
    users: &'a [UserStyle],
    progressions_only: bool,
) -> Vec<Preset<'a>> {
    let mut presets: Vec<_> = harmony_styles(query, None)
        .into_iter()
        .map(Preset::Style)
        .chain(rhythms(query, None).into_iter().map(Preset::Rhythm))
        .chain(users.iter().enumerate().filter_map(|(index, style)| {
            let text = format!("{} {} {} user", style.name, style.about, style.time).to_lowercase();
            query
                .to_lowercase()
                .split_whitespace()
                .all(|word| text.contains(word))
                .then_some(Preset::User(index, style))
        }))
        .filter(|preset| !progressions_only || preset.has_progression())
        .collect();
    presets.sort_by_cached_key(|preset| (preset.name().to_lowercase(), preset.name().to_string()));
    presets
}

pub fn alphabetical<T>(
    entries: impl IntoIterator<Item = T>,
    name: impl Fn(&T) -> &'static str,
) -> Vec<T> {
    let mut entries: Vec<_> = entries.into_iter().collect();
    entries.sort_by_cached_key(|entry| (name(entry).to_lowercase(), name(entry)));
    entries
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
    fn the_merged_catalogue_keeps_every_source_and_sorts_them_together() {
        let mut user = UserStyle::blank();
        user.name = "A custom groove".into();
        let users = [user];
        let all = presets("", &users, false);
        assert_eq!(all.len(), STYLES.len() + RHYTHMS.len() + 1);
        assert_eq!(
            all.iter().filter(|p| matches!(p, Preset::Style(_))).count(),
            STYLES.len()
        );
        assert_eq!(
            all.iter()
                .filter(|p| matches!(p, Preset::Rhythm(_)))
                .count(),
            RHYTHMS.len()
        );
        assert!(all.iter().any(|p| matches!(p, Preset::User(0, _))));
        assert!(
            all.windows(2)
                .all(|pair| pair[0].name().to_lowercase() <= pair[1].name().to_lowercase())
        );
        assert!(
            presets("balkan 7/8", &users, false)
                .iter()
                .any(|p| matches!(p, Preset::Style(_)))
        );
        assert!(
            presets("balkan 7/8", &users, false)
                .iter()
                .any(|p| matches!(p, Preset::Rhythm(_)))
        );
        assert!(matches!(
            presets("custom groove", &users, false)[0],
            Preset::User(0, _)
        ));
    }

    #[test]
    fn progression_choices_include_only_presets_that_supply_chords() {
        let users = [UserStyle::blank()];
        let choices = presets("", &users, true);
        assert_eq!(choices.len(), STYLES.len() + users.len());
        assert!(choices.iter().all(|preset| preset.has_progression()));
        let beat = presets("pop beat", &[], false);
        assert!(beat.iter().any(|preset| !preset.has_progression()));
        assert!(
            presets("pop beat", &[], true)
                .iter()
                .all(|preset| preset.has_progression())
        );
    }

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
    fn style_lists_are_alphabetical() {
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
