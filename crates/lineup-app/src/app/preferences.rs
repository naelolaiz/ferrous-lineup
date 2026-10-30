//! Preferences kept between runs under names of their own.
//!
//! egui's memory keeps values between runs too (`insert_persisted`), and
//! eframe writes them to `app.ron`. But egui finds each value again by
//! its Rust type, through a `TypeId`, and the `TypeId` of a type changes
//! with every new version of the app, of the Rust compiler, and of the
//! type itself (a field added or renamed). After such an update those
//! values are not found any more, and the app starts as if they had never
//! been chosen. eframe's own storage finds a value by its name, which
//! stays the same, so what is kept here survives updates.
//!
//! eframe hands its storage to the app only as it starts and when it
//! saves (every 30 seconds, and when the window closes). So the
//! preferences are read once at the start into a map that egui's context
//! shares with every part of the app, and written back from that map.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use eframe::egui;
use serde::Serialize;
use serde::de::DeserializeOwned;

/// The name eframe keeps them all under, in `app.ron`.
const STORAGE_KEY: &str = "preferences";

/// Each preference by name, written as RON. egui's context holds one
/// handle to the map and the app another, to save it.
#[derive(Clone, Default)]
pub(super) struct Preferences(Arc<Mutex<BTreeMap<String, String>>>);

impl Preferences {
    /// The preferences of the app that draws with `ctx`.
    pub(super) fn of(ctx: &egui::Context) -> Self {
        ctx.data_mut(|data| {
            data.get_temp_mut_or_default::<Self>(egui::Id::unique("preferences"))
                .clone()
        })
    }

    /// Takes the preferences eframe kept from the last run.
    pub(super) fn load(&self, storage: &dyn eframe::Storage) {
        *self.map() = eframe::get_value(storage, STORAGE_KEY).unwrap_or_default();
    }

    /// Hands the preferences to eframe, which writes them to disk.
    pub(super) fn save(&self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, STORAGE_KEY, &*self.map());
    }

    fn map(&self) -> MutexGuard<'_, BTreeMap<String, String>> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

/// The preference kept as `name`, if there is one and it can be read.
pub(super) fn get<T: DeserializeOwned>(ctx: &egui::Context, name: &str) -> Option<T> {
    let text = Preferences::of(ctx).map().get(name).cloned()?;
    ron::from_str(&text).ok()
}

/// Keeps `value` as the preference `name`.
pub(super) fn set<T: Serialize>(ctx: &egui::Context, name: &str, value: &T) {
    if let Ok(text) = ron::to_string(value) {
        Preferences::of(ctx).map().insert(name.to_string(), text);
    }
}

/// For the tests: eframe's storage, kept in memory instead of `app.ron`.
#[cfg(test)]
#[derive(Default)]
pub(super) struct Stored(std::collections::HashMap<String, String>);

#[cfg(test)]
impl eframe::Storage for Stored {
    fn get_string(&self, key: &str) -> Option<String> {
        self.0.get(key).cloned()
    }
    fn set_string(&mut self, key: &str, value: String) {
        self.0.insert(key.to_string(), value);
    }
    fn remove_string(&mut self, key: &str) {
        self.0.remove(key);
    }
    fn flush(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::Storage;

    #[derive(Debug, PartialEq, Serialize, serde::Deserialize)]
    struct Choice {
        on: bool,
        name: String,
    }

    #[test]
    fn a_preference_comes_back_by_its_name_in_the_next_run() {
        let ctx = egui::Context::default();
        assert_eq!(get::<Choice>(&ctx, "choice"), None);
        let choice = Choice {
            on: true,
            name: "GeneralUser GS".to_string(),
        };
        set(&ctx, "choice", &choice);
        assert_eq!(get(&ctx, "choice").as_ref(), Some(&choice));
        let mut storage = Stored::default();
        Preferences::of(&ctx).save(&mut storage);

        // Kept by name: nothing in it depends on the Rust types.
        let text = storage.get_string(STORAGE_KEY).unwrap();
        assert!(text.contains("choice"), "{text}");

        let next = egui::Context::default();
        Preferences::of(&next).load(&storage);
        assert_eq!(get(&next, "choice"), Some(choice));
        assert_eq!(get::<bool>(&next, "choice"), None, "another type");
        assert_eq!(get::<Choice>(&next, "other"), None);
    }
}
