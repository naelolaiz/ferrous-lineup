//! Shared names, searches and ordering for the Library and chord sheet menus.

use lineup_core::library::{Genre, RHYTHMS, Rhythm, STYLES, Style, UserStyle};
use lineup_core::{BandStyle, Player, TimeSignature, general_midi};

/// An instrument a preset brings: a General MIDI program, or the drums.
/// In this order, the instruments come family by family, as General MIDI
/// numbers them, and the drums last.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum Instrument {
    Program(u8),
    Drums,
}

impl Instrument {
    /// Its name, in the language chosen.
    pub fn name(self) -> &'static str {
        match self {
            Instrument::Program(program) => crate::i18n::tr(general_midi::name(program)),
            Instrument::Drums => crate::i18n::tr("Drums"),
        }
    }
}

/// The instruments `band` brings: its players', and the drums if it has
/// a rhythm.
fn band_instruments(band: Option<&BandStyle>) -> Vec<Instrument> {
    let Some(band) = band else {
        return Vec::new();
    };
    let drums = band.rhythm.map(|_| Instrument::Drums);
    band.programs()
        .into_iter()
        .map(Instrument::Program)
        .chain(drums)
        .collect()
}

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

    /// The speed it was made for, in beats a minute.
    pub fn tempo(self) -> f32 {
        match self {
            Self::Style(style) => style.tempo,
            Self::Rhythm(rhythm) => rhythm.tempo,
            Self::User(_, style) => style.tempo.bpm(),
        }
    }

    pub fn genre(self) -> Option<Genre> {
        match self {
            Self::Style(style) => Some(style.genre),
            Self::Rhythm(rhythm) => Some(rhythm.genre),
            Self::User(_, _) => None,
        }
    }

    /// Whether it plays every bar in that bar's meter, keeping the song's
    /// time signatures and tempo (see [`Style::follows_the_song`]).
    pub fn follows_the_song(self) -> bool {
        match self {
            Self::Style(style) => style.follows_the_song(),
            Self::Rhythm(rhythm) => rhythm.follows_the_meter,
            Self::User(_, _) => false,
        }
    }

    /// The instruments it brings when its instruments are copied: those
    /// of the band that plays it, or of a style of your own's lanes.
    pub fn instruments(self) -> Vec<Instrument> {
        match self {
            Self::Style(style) => band_instruments(BandStyle::of_style(style)),
            Self::Rhythm(rhythm) => band_instruments(BandStyle::named(rhythm.name)),
            Self::User(_, style) => style
                .lanes
                .iter()
                .filter_map(|lane| match lane.player {
                    Player::Drums => Some(Instrument::Drums),
                    _ => lane.channel.program.map(Instrument::Program),
                })
                .collect(),
        }
    }

    /// Whether its drums swing (see [`lineup_core::swing`]).
    pub fn swings(self) -> bool {
        match self {
            Self::Style(style) => {
                Rhythm::named(style.rhythm).is_some_and(|rhythm| rhythm.swing > 0)
            }
            Self::Rhythm(rhythm) => rhythm.swing > 0,
            Self::User(_, style) => style.swing > 0,
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
    let mut presets: Vec<_> =
        harmony_styles(query, None)
            .into_iter()
            .map(Preset::Style)
            .chain(rhythms(query, None).into_iter().map(|rhythm| {
                equivalent_style(rhythm).map_or(Preset::Rhythm(rhythm), Preset::Style)
            }))
            .chain(users.iter().enumerate().filter_map(|(index, style)| {
                let instruments = instrument_names(query, Preset::User(index, style));
                let text = format!(
                    "{} {} {} user {instruments}",
                    style.name, style.about, style.time
                )
                .to_lowercase();
                matches_text(query, &text).then_some(Preset::User(index, style))
            }))
            .filter(|preset| !progressions_only || preset.has_progression())
            .collect();
    presets.sort_by_cached_key(|preset| {
        let shown = lineup_core::i18n::tr_name(preset.name());
        (shown.to_lowercase(), preset.name().to_string())
    });
    presets.dedup_by(|a, b| {
        matches!((*a, *b),
            (Preset::Style(a), Preset::Style(b)) if std::ptr::eq(a, b)
        )
    });
    presets
}

/// Every instrument the presets bring, in [`Instrument`]'s order, each
/// once: the choices of the Library's instrument filter.
pub fn instruments(users: &[UserStyle]) -> Vec<Instrument> {
    let mut instruments: Vec<_> = presets("", users, false)
        .into_iter()
        .flat_map(Preset::instruments)
        .collect();
    instruments.sort_unstable();
    instruments.dedup();
    instruments
}

/// Prefer the entry with optional chords when both sources play the same groove.
/// Different meters, tempos or accompaniment patterns remain separate choices.
pub fn equivalent_style(rhythm: &Rhythm) -> Option<&'static Style> {
    let feel = lineup_core::library::apply::feel(rhythm);
    STYLES.iter().find(|style| {
        style.name == rhythm.name
            && style.rhythm == rhythm.name
            && style.genre == rhythm.genre
            && style.time == rhythm.time
            && style.tempo == rhythm.tempo
            && (style.comping, style.bass) == feel
    })
}

pub fn alphabetical<T>(
    entries: impl IntoIterator<Item = T>,
    name: impl Fn(&T) -> &'static str,
) -> Vec<T> {
    let mut entries: Vec<_> = entries.into_iter().collect();
    // In the order of the names as shown, in the language chosen.
    entries.sort_by_cached_key(|entry| {
        let shown = lineup_core::i18n::tr_name(name(entry));
        (shown.to_lowercase(), name(entry))
    });
    entries
}

/// Whether `query` finds a preset, by its name, genre, meter or about
/// text in English or in Spanish.
pub fn matches(query: &str, name: &str, genre: Genre, time: TimeSignature, about: &str) -> bool {
    let spanish = |text: &str| lineup_core::i18n::spanish(text).unwrap_or_default();
    let text = format!(
        "{name} {} {time} {}/{} {about} {} {} {}",
        genre.name(),
        time.beats,
        time.unit,
        spanish(name),
        spanish(genre.name()),
        spanish(about),
    )
    .to_lowercase();
    matches_text(query, &text)
}

fn matches_text(query: &str, text: &str) -> bool {
    query.to_lowercase().split_whitespace().all(|word| {
        text.contains(word)
            || match word {
                "vals" => text.contains("waltz"),
                "waltz" => text.contains("vals"),
                _ => false,
            }
    })
}

pub fn harmony_styles(query: &str, genre: Option<Genre>) -> Vec<&'static Style> {
    alphabetical(
        STYLES.iter().filter(|style| {
            genre.is_none_or(|genre| style.genre == genre)
                && matches(
                    query,
                    style.name,
                    style.genre,
                    style.time,
                    &format!(
                        "{} {} {} {}",
                        style.about,
                        lineup_core::i18n::spanish(style.about).unwrap_or_default(),
                        style.rhythm,
                        instrument_names(query, Preset::Style(style)),
                    ),
                )
        }),
        |style| style.name,
    )
}

pub fn rhythms(query: &str, genre: Option<Genre>) -> Vec<&'static Rhythm> {
    alphabetical(
        RHYTHMS.iter().filter(|rhythm| {
            genre.is_none_or(|genre| rhythm.genre == genre)
                && matches(
                    query,
                    rhythm.name,
                    rhythm.genre,
                    rhythm.time,
                    &format!(
                        "{} {}",
                        rhythm.about,
                        instrument_names(query, Preset::Rhythm(rhythm))
                    ),
                )
        }),
        |rhythm| rhythm.name,
    )
}

/// The names of the instruments `preset` brings, in English and in
/// Spanish, so that searching for "accordion" finds the styles with one.
/// Nothing while there is no `query` to look for them.
fn instrument_names(query: &str, preset: Preset) -> String {
    if query.trim().is_empty() {
        return String::new();
    }
    let english = |instrument| match instrument {
        Instrument::Program(program) => general_midi::name(program),
        Instrument::Drums => "Drums",
    };
    preset
        .instruments()
        .into_iter()
        .map(|instrument| {
            let name = english(instrument);
            let spanish = lineup_core::i18n::spanish(name).unwrap_or_default();
            format!("{name} {spanish}")
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use lineup_core::BandStyle;

    #[test]
    fn vals_and_waltz_find_the_plain_rhythm_and_waltzes_across_genres() {
        let names = |query| {
            presets(query, &[], false)
                .into_iter()
                .map(|preset| preset.name().to_string())
                .collect::<Vec<_>>()
        };
        let vals = names("vals");
        assert_eq!(vals, names("waltz"));
        for name in [
            "Waltz",
            "Country waltz",
            "Tango vals",
            "Vals in minor",
            "Sevillanas",
        ] {
            assert!(vals.iter().any(|found| found == name), "Missing {name}");
        }
        let folk = presets("vals 3/4 folk", &[], false);
        assert!(!folk.is_empty());
        assert!(folk.iter().all(|preset| matches!(
            preset.genre(),
            Some(Genre::Folk | Genre::Folklore)
        ) && preset.time() == TimeSignature::new(3, 4)));
    }

    #[test]
    fn the_merged_catalogue_keeps_every_source_and_sorts_them_together() {
        let mut user = UserStyle::blank();
        user.name = "A custom groove".into();
        let users = [user];
        let all = presets("", &users, false);
        let distinct_rhythms = RHYTHMS
            .iter()
            .filter(|rhythm| equivalent_style(rhythm).is_none())
            .count();
        assert_eq!(all.len(), STYLES.len() + distinct_rhythms + 1);
        assert_eq!(
            all.iter().filter(|p| matches!(p, Preset::Style(_))).count(),
            STYLES.len()
        );
        assert_eq!(
            all.iter()
                .filter(|p| matches!(p, Preset::Rhythm(_)))
                .count(),
            distinct_rhythms
        );
        for rhythm in RHYTHMS {
            assert!(
                all.iter().any(|preset| match preset {
                    Preset::Rhythm(found) => std::ptr::eq(*found, rhythm),
                    Preset::Style(style) => equivalent_style(rhythm)
                        .is_some_and(|equivalent| std::ptr::eq(*style, equivalent)),
                    _ => false,
                }),
                "Missing groove {}",
                rhythm.name
            );
        }
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
            presets("balkan 7/8", &users, false).iter().all(|preset| {
                preset.genre() == Some(Genre::Balkan) && preset.time().beats == 7
            })
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
    fn equivalent_bossa_entries_are_combined_and_distinct_patterns_remain() {
        let all = presets("", &[], false);
        let bossa: Vec<_> = all
            .iter()
            .filter(|preset| preset.name() == "Bossa nova")
            .collect();
        assert_eq!(bossa.len(), 1);
        assert!(bossa[0].has_progression());
        assert!(all.iter().any(|preset| preset.name() == "Bossa minor"));
        assert!(
            all.iter()
                .any(|preset| preset.name() == "Pop beat" && !preset.has_progression())
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
    fn searches_find_presets_by_the_instruments_they_bring() {
        let program = (0..128).find(|&program| general_midi::name(program) == "Accordion");
        let accordion = Instrument::Program(program.unwrap());
        let with_one: Vec<_> = presets("", &[], false)
            .into_iter()
            .filter(|preset| preset.instruments().contains(&accordion))
            .map(Preset::name)
            .collect();
        assert!(!with_one.is_empty());
        for query in ["accordion", "ACORDEÓN"] {
            let found: Vec<_> = presets(query, &[], false)
                .into_iter()
                .map(Preset::name)
                .collect();
            assert!(with_one.iter().all(|name| found.contains(name)), "{query}");
        }
        let bossa = presets("bossa nylon", &[], false);
        assert!(bossa.iter().any(|preset| preset.name() == "Bossa nova"));
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
