//! The style's two variations. A band does not play every part of a song
//! the same way: it plays a verse lightly and digs into the chorus or
//! the bridge. Variation **a** is the style as it is; variation **b** is
//! the same style played fuller. Markers on the chord sheet say where
//! each starts. The drums play a fill in the bar before a change and
//! crash on its first beat.
//!
//! Variation b opens the style up the way players do:
//!
//! - The drummer moves the time from the closed hi-hat to the ride
//!   cymbal, unless the style keeps time on the ride already. A pop or
//!   rock ballad's cross-stick becomes a full snare.
//! - Where a style has a well-known busier way to play, the bass or the
//!   piano switches to it: a jazz bass playing "in two" walks in four, a
//!   blues shuffle's bass plays the boogie line, a held bass in pop,
//!   rock, soul or folk plays every beat, and a pop or rock ballad's
//!   piano plays steady eighth notes.
//! - Everyone plays a little louder.

use serde::{Deserialize, Serialize};

use super::BandStyle;
use crate::library::Genre;
use crate::library::comping::{Bass, Comping};
use crate::meter::TimeSignature;
use crate::project::{Drum, Project};

/// One of the two ways a band style can be played.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Variation {
    /// The style as it is.
    #[default]
    A,
    /// The style played fuller.
    B,
}

impl Variation {
    /// Its letter, as the chord sheet shows it: "a" or "b".
    pub fn letter(self) -> &'static str {
        match self {
            Variation::A => "a",
            Variation::B => "b",
        }
    }
}

/// Where the band starts playing a variation, on the playlist's
/// timeline: always at the start of a bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariationMarker {
    pub start: usize,
    pub variation: Variation,
}

impl VariationMarker {
    pub fn new(start: usize, variation: Variation) -> Self {
        VariationMarker { start, variation }
    }
}

/// The variation sounding at `step`: the one of the last marker at or
/// before it, or a before the first.
pub(crate) fn variation_in(markers: &[VariationMarker], step: usize) -> Variation {
    markers
        .iter()
        .rev()
        .find(|marker| marker.start <= step)
        .map_or(Variation::A, |marker| marker.variation)
}

/// Puts `markers` in order, the later of two on one step winning, and
/// drops the ones that change nothing: a marker for the variation that
/// is already playing.
pub(crate) fn tidy(markers: &mut Vec<VariationMarker>) {
    markers.sort_by_key(|marker| marker.start);
    markers.dedup_by(|later, earlier| {
        later.start == earlier.start && {
            *earlier = *later;
            true
        }
    });
    let mut playing = Variation::A;
    markers.retain(|marker| {
        let changes = marker.variation != playing;
        playing = marker.variation;
        changes
    });
}

/// A note's velocity in variation b: a tenth louder.
pub(crate) fn louder(velocity: u8) -> u8 {
    velocity.saturating_add(velocity / 10).min(127)
}

impl BandStyle {
    /// How the piano comps in `variation`.
    pub fn comping_in(&self, variation: Variation) -> Comping {
        match (variation, self.genre, self.comping) {
            (Variation::B, Genre::Pop | Genre::Rock, Comping::Ballad) => Comping::Pulse,
            (_, _, comping) => comping,
        }
    }

    /// The bass line in `variation`.
    pub fn bass_in(&self, variation: Variation) -> Bass {
        let common = self.time == TimeSignature::COMMON;
        match (variation, self.genre, self.bass) {
            (Variation::B, Genre::Jazz, Bass::RootFifth) if common => Bass::Walking,
            (Variation::B, Genre::Blues, Bass::Quarters) if common => Bass::Boogie,
            (Variation::B, Genre::Pop | Genre::Rock | Genre::Soul | Genre::Folk, Bass::Held)
                if common =>
            {
                Bass::Quarters
            }
            (_, _, bass) => bass,
        }
    }

    /// The drum that plays what the style's rhythm gives `drum` in
    /// `variation`.
    pub fn drum_in(&self, drum: Drum, variation: Variation) -> Drum {
        let Some(rhythm) = self.rhythm else {
            return drum;
        };
        let has = |wanted: Drum| rhythm.drums().any(|drum| drum == wanted);
        match (variation, drum) {
            (Variation::B, Drum::HiHat) if !has(Drum::Ride) => Drum::Ride,
            (Variation::B, Drum::Rim)
                if matches!(self.genre, Genre::Pop | Genre::Rock)
                    && !has(Drum::Snare)
                    && !has(Drum::Clap) =>
            {
                Drum::Snare
            }
            _ => drum,
        }
    }
}

impl Project {
    /// Where the band changes variation, in order. Empty without a band,
    /// or when it plays variation a all through.
    pub fn variations(&self) -> &[VariationMarker] {
        self.band
            .as_ref()
            .map_or(&[], |band| band.variations.as_slice())
    }

    /// The variation the band plays at `step`.
    pub fn variation_at(&self, step: usize) -> Variation {
        variation_in(self.variations(), step)
    }

    /// Has the band play `variation` from step `start` (the start of a
    /// bar) up to `end`, where it goes back to what it played there
    /// before. Without an `end`, it plays it up to the next change, which
    /// stays. Returns `false`, changing nothing, without a band.
    pub fn set_variation(
        &mut self,
        start: usize,
        end: Option<usize>,
        variation: Variation,
    ) -> bool {
        let after = end.map(|end| (end, self.variation_at(end)));
        let Some(band) = &mut self.band else {
            return false;
        };
        let markers = &mut band.variations;
        markers.retain(|marker| match end {
            Some(end) => marker.start < start || marker.start >= end,
            None => marker.start != start,
        });
        markers.push(VariationMarker::new(start, variation));
        if let Some((end, after)) = after {
            markers.push(VariationMarker::new(end, after));
        }
        tidy(markers);
        self.write_band();
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::band::Player;
    use crate::chord_sheet::read_bar;

    /// A song with a band in `style` and `bars` written on its sheet.
    fn song(style: &str, bars: &[&str]) -> Project {
        let mut project = Project::with_band();
        project.set_band_style(BandStyle::named(style));
        let meter = project.meter();
        for (number, text) in bars.iter().enumerate() {
            let bar = meter.bar(number);
            let markers = read_bar(text, &bar, project.key_at(bar.start)).unwrap();
            assert!(project.set_chords(bar.start, bar.end(), &markers));
        }
        project.follow_chord_sheet();
        project
    }

    /// The channels `player` plays on from `start` to `end`, by name.
    fn heard(project: &Project, player: Player, start: usize, end: usize) -> Vec<String> {
        let mut names: Vec<String> = Vec::new();
        for clip in project.playlist().clips() {
            let pattern = project.pattern(clip.pattern).unwrap();
            if pattern.player != Some(player) {
                continue;
            }
            for (channel, note) in pattern.all_notes() {
                let at = clip.start + note.start;
                let name = &project.channels()[channel].name;
                if (start..end).contains(&at) && !names.contains(name) {
                    names.push(name.clone());
                }
            }
        }
        names.sort();
        names
    }

    /// The notes `player` starts from `start` to `end`: (step, velocity).
    fn notes(project: &Project, player: Player, start: usize, end: usize) -> Vec<(usize, u8)> {
        let mut notes = Vec::new();
        for clip in project.playlist().clips() {
            let pattern = project.pattern(clip.pattern).unwrap();
            if pattern.player != Some(player) {
                continue;
            }
            for (_, note) in pattern.all_notes() {
                let at = clip.start + note.start;
                if (start..end).contains(&at) {
                    notes.push((at, note.velocity));
                }
            }
        }
        notes.sort_unstable();
        notes
    }

    #[test]
    fn markers_keep_only_changes() {
        let a = |start| VariationMarker::new(start, Variation::A);
        let b = |start| VariationMarker::new(start, Variation::B);
        let mut markers = vec![b(32), a(0), b(16), a(48), a(64), b(48)];
        tidy(&mut markers);
        assert_eq!(markers, vec![b(16), a(64)]);
        let mut markers = vec![b(16), a(32), a(48)];
        tidy(&mut markers);
        assert_eq!(markers, vec![b(16), a(32)]);
    }

    #[test]
    fn a_stretch_goes_back_to_what_played_after_it() {
        let mut project = song("Pop beat", &["C", "Am", "F", "G7", "C", "Am", "F", "G7"]);
        assert!(project.set_variation(64, Some(96), Variation::B));
        assert_eq!(
            project.variations(),
            [
                VariationMarker::new(64, Variation::B),
                VariationMarker::new(96, Variation::A)
            ]
        );
        assert_eq!(project.variation_at(63), Variation::A);
        assert_eq!(project.variation_at(64), Variation::B);
        assert_eq!(project.variation_at(96), Variation::A);
        // From bar 2 on: up to the next change, which stays. The marker
        // at bar 5 changes nothing any more, and goes.
        assert!(project.set_variation(16, None, Variation::B));
        assert_eq!(
            project.variations(),
            [
                VariationMarker::new(16, Variation::B),
                VariationMarker::new(96, Variation::A)
            ]
        );
        // Bar 2 back to a: only the change at bar 7 is left, and it
        // changes nothing either.
        assert!(project.set_variation(16, None, Variation::A));
        assert!(project.variations().is_empty());
    }

    #[test]
    fn the_drummer_moves_to_the_ride() {
        let mut project = song("Pop beat", &["C", "Am", "F", "G7", "C", "Am", "F", "G7"]);
        assert_eq!(
            heard(&project, Player::Drums, 64, 112),
            ["Hat", "Kick", "Snare"]
        );
        project.set_variation(64, None, Variation::B);
        assert_eq!(
            heard(&project, Player::Drums, 64, 112),
            ["Crash", "Kick", "Ride", "Snare"]
        );
        // Variation a before it keeps the hi-hat.
        assert_eq!(
            heard(&project, Player::Drums, 0, 48),
            ["Hat", "Kick", "Snare"]
        );
        // The crash comes once, on the first beat of the change.
        let crash = project
            .channels()
            .iter()
            .position(|channel| channel.name == "Crash")
            .unwrap();
        let crashes: Vec<usize> = project
            .playlist()
            .clips()
            .iter()
            .flat_map(|clip| {
                let pattern = project.pattern(clip.pattern).unwrap();
                pattern
                    .notes(crash)
                    .iter()
                    .map(|note| clip.start + note.start)
                    .collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(crashes, [64]);
    }

    #[test]
    fn the_drums_fill_into_a_change() {
        let bars = ["C", "Am", "F", "G7", "C", "Am", "F", "G7"];
        let plain = song("Pop beat", &bars);
        let mut project = plain.clone();
        project.set_variation(64, None, Variation::B);
        assert_ne!(
            notes(&project, Player::Drums, 48, 64),
            notes(&plain, Player::Drums, 48, 64),
            "bar 4 is a fill now"
        );
        assert_eq!(
            notes(&project, Player::Drums, 0, 48),
            notes(&plain, Player::Drums, 0, 48)
        );
    }

    #[test]
    fn variation_b_is_louder() {
        let plain = song("Pop beat", &["C", "Am", "F", "G7"]);
        let mut project = plain.clone();
        project.set_variation(0, None, Variation::B);
        for player in [Player::Piano, Player::Bass] {
            let soft = notes(&plain, player, 0, 64);
            let loud = notes(&project, player, 0, 64);
            assert_eq!(soft.len(), loud.len());
            for ((_, soft), (_, loud)) in soft.iter().zip(&loud) {
                assert_eq!(*loud, louder(*soft));
            }
        }
    }

    #[test]
    fn players_switch_to_their_busier_way() {
        let style = |name| BandStyle::named(name).unwrap();
        // A jazz ballad's bass in two walks in four.
        assert_eq!(style("Jazz ballad").bass_in(Variation::A), Bass::RootFifth);
        assert_eq!(style("Jazz ballad").bass_in(Variation::B), Bass::Walking);
        // A shuffle's bass plays the boogie line.
        assert_eq!(style("Shuffle").bass_in(Variation::B), Bass::Boogie);
        // A pop ballad: eighth notes on the piano, every beat on the bass,
        // the full snare instead of the cross-stick, and the ride.
        let ballad = style("Pop ballad");
        assert_eq!(ballad.comping_in(Variation::B), Comping::Pulse);
        assert_eq!(ballad.bass_in(Variation::B), Bass::Quarters);
        assert_eq!(ballad.drum_in(Drum::Rim, Variation::B), Drum::Snare);
        assert_eq!(ballad.drum_in(Drum::HiHat, Variation::B), Drum::Ride);
        // Swing keeps time on the ride already: its hi-hat, on 2 and 4,
        // stays.
        assert_eq!(
            style("Swing").drum_in(Drum::HiHat, Variation::B),
            Drum::HiHat
        );
        // A bossa nova's cross-stick is its clave, not a backbeat.
        assert_eq!(
            style("Bossa nova").drum_in(Drum::Rim, Variation::B),
            Drum::Rim
        );
        // Classical styles play their own way in both.
        let chorale = style("Chorale");
        assert_eq!(chorale.bass_in(Variation::B), Bass::Held);
        assert_eq!(chorale.comping_in(Variation::B), Comping::Held);
    }

    #[test]
    fn a_jazz_ballad_walks_in_variation_b() {
        let mut project = song("Jazz ballad", &["Cmaj7", "Am7", "Dm7", "G7"]);
        let in_two = notes(&project, Player::Bass, 0, 64).len();
        project.set_variation(0, None, Variation::B);
        let in_four = notes(&project, Player::Bass, 0, 64).len();
        assert!(in_four > in_two, "{in_two} notes in two, {in_four} walking");
    }

    #[test]
    fn markers_move_with_the_song() {
        let mut project = song("Pop beat", &["C", "Am", "F", "G7", "C", "Am", "F", "G7"]);
        project.set_variation(64, Some(96), Variation::B);
        assert!(project.set_intro(2));
        assert_eq!(
            project.variations(),
            [
                VariationMarker::new(96, Variation::B),
                VariationMarker::new(128, Variation::A)
            ]
        );
        assert!(project.set_intro(0));
        assert_eq!(project.variations()[0].start, 64);
    }

    #[test]
    fn a_copied_section_plays_the_same_variations() {
        let mut project = song("Pop beat", &["C", "Am", "F", "G7"]);
        project.add_section(crate::Section::new("Verse", 0, 64));
        project.set_variation(32, None, Variation::B);
        assert!(project.duplicate_section(0, 1));
        // The copy, bars 5 to 8, starts in a and changes to b in its
        // third bar, as the verse does.
        assert_eq!(
            project.variations(),
            [
                VariationMarker::new(32, Variation::B),
                VariationMarker::new(64, Variation::A),
                VariationMarker::new(96, Variation::B)
            ]
        );
    }

    #[test]
    fn variations_are_saved() {
        let mut project = song("Pop beat", &["C", "Am", "F", "G7"]);
        project.set_variation(32, None, Variation::B);
        assert_eq!(crate::file::format_for(&project), 15);
        let text = crate::file::to_text(&project).unwrap();
        let loaded = crate::file::from_text(&text).unwrap();
        assert_eq!(loaded, project);
        assert_eq!(
            loaded.variations(),
            [VariationMarker::new(32, Variation::B)]
        );
    }
}
