//! Key changes through a song.
//!
//! Many pieces do not stay in one key. Mozart's Rondo alla Turca starts
//! in A minor, turns to A major at bar 25 (the key signature gets three
//! sharps), and goes back to A minor at bar 65. Moving to a new key like
//! this is a *modulation*.
//!
//! Each change is a [`KeyMarker`] on the playlist, like the time
//! signature and tempo markers: from its step on, the song is in its
//! key. A marker changes only what the key is called, what the piano
//! roll and the harmony keys show, and which chords count as "in the
//! key"; it never moves a note. A [`KeyMap`] works out the key of every
//! step from the markers.

use serde::{Deserialize, Serialize};

use crate::Scale;

/// A change of key partway through a song: from step `start` on, the
/// song is in `scale`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyMarker {
    /// The step it starts on, counted from the start of the song.
    pub start: usize,
    pub scale: Scale,
}

impl KeyMarker {
    pub fn new(start: usize, scale: Scale) -> Self {
        KeyMarker { start, scale }
    }
}

/// The key of every step of a song: the one it starts in, and every
/// change after that.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyMap {
    first: Scale,
    /// Sorted by start, all after step 0, no two on the same step.
    markers: Vec<KeyMarker>,
}

impl KeyMap {
    /// A map that starts in `first` and changes at each of `markers`.
    /// Markers on step 0, or on the same step as another, are left out.
    pub fn new(first: Scale, markers: impl IntoIterator<Item = KeyMarker>) -> Self {
        let mut markers: Vec<KeyMarker> = markers
            .into_iter()
            .filter(|marker| marker.start > 0)
            .collect();
        markers.sort_by_key(|marker| marker.start);
        markers.dedup_by_key(|marker| marker.start);
        KeyMap { first, markers }
    }

    /// The key the song starts in.
    pub fn first(&self) -> Scale {
        self.first
    }

    /// The changes after the start, in order.
    pub fn markers(&self) -> &[KeyMarker] {
        &self.markers
    }

    /// Whether the song stays in one key.
    pub fn is_steady(&self) -> bool {
        self.markers.is_empty()
    }

    /// The key on `step`.
    pub fn key_at(&self, step: usize) -> Scale {
        self.marker_at(step)
            .map_or(self.first, |marker| marker.scale)
    }

    /// The last change at or before `step`, if there is one.
    pub fn marker_at(&self, step: usize) -> Option<&KeyMarker> {
        self.markers
            .iter()
            .take_while(|marker| marker.start <= step)
            .last()
    }

    /// The same map without the markers that change nothing: those
    /// that name the key already in effect.
    pub fn tidy(&self) -> KeyMap {
        let mut current = self.first;
        let mut markers = Vec::new();
        for marker in &self.markers {
            if marker.scale != current {
                markers.push(*marker);
                current = marker.scale;
            }
        }
        KeyMap {
            first: self.first,
            markers,
        }
    }
}

/// Where a key change lands when the song's key moves from `from` to
/// `to` (see [`Project::change_key`](crate::Project::change_key)): every
/// key moves by the same number of semitones as the song's root, so the
/// distance between the keys stays the same. A key in the song's old
/// mode takes its new one as well (a C major song going to G major, in
/// C minor, goes to G minor); a key in another mode keeps it (the A
/// major march of a song in A minor stays major when the song goes to
/// A Dorian). So does a key in the song's old row or set: a song that
/// goes from one row to another takes its sections in the old row,
/// moved, into the new one. Where the song's notes stay put, as from a
/// key to a twelve-tone row or a set, its key changes stay put too.
///
/// A key that moves takes the usual name of the key it lands on (see
/// [`Scale::spelling`]). One that stays where it is keeps its name, so
/// writing the song's key the other way round, as Gb major rather than
/// F# major, leaves the key changes as they were written.
pub fn moved_key(key: Scale, from: Scale, to: Scale) -> Scale {
    let from = crate::tools::from_key(from, to);
    if !from.kind.notes_follow(to.kind) {
        return key;
    }
    let up = from.root.semitones_up_to(to.root);
    let root = key.root.transpose(up as i32);
    let kind = if key.kind == from.kind {
        to.kind
    } else {
        key.kind
    };
    if root == key.root && kind == key.kind {
        return key;
    }
    Scale::new(root, kind)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PitchClass, ScaleKind};

    fn key(root: PitchClass, kind: ScaleKind) -> Scale {
        Scale::new(root, kind)
    }

    #[test]
    fn the_key_changes_from_each_marker_on() {
        let a_minor = key(PitchClass::A, ScaleKind::HarmonicMinor);
        let a_major = key(PitchClass::A, ScaleKind::Major);
        let map = KeyMap::new(
            a_minor,
            [KeyMarker::new(64, a_major), KeyMarker::new(128, a_minor)],
        );
        assert_eq!(map.key_at(0), a_minor);
        assert_eq!(map.key_at(63), a_minor);
        assert_eq!(map.key_at(64), a_major);
        assert_eq!(map.key_at(127), a_major);
        assert_eq!(map.key_at(128), a_minor);
        assert_eq!(map.marker_at(70).map(|m| m.start), Some(64));
        assert!(map.marker_at(10).is_none());
    }

    #[test]
    fn markers_that_change_nothing_are_tidied_away() {
        let c = key(PitchClass::C, ScaleKind::Major);
        let g = key(PitchClass::G, ScaleKind::Major);
        let map = KeyMap::new(
            c,
            [
                KeyMarker::new(0, g),
                KeyMarker::new(16, c),
                KeyMarker::new(32, g),
                KeyMarker::new(48, g),
            ],
        );
        assert_eq!(map.tidy().markers(), &[KeyMarker::new(32, g)]);
    }

    #[test]
    fn keys_move_with_the_song() {
        let a_minor = key(PitchClass::A, ScaleKind::HarmonicMinor);
        let a_major = key(PitchClass::A, ScaleKind::Major);
        let c_minor = key(PitchClass::C, ScaleKind::HarmonicMinor);
        let f_sharp_minor = key(PitchClass::FSharp, ScaleKind::NaturalMinor);
        // Up a minor third: the march goes to C major, the episode to A minor.
        assert_eq!(
            moved_key(a_major, a_minor, c_minor),
            key(PitchClass::C, ScaleKind::Major)
        );
        assert_eq!(
            moved_key(f_sharp_minor, a_minor, c_minor),
            key(PitchClass::A, ScaleKind::NaturalMinor)
        );
        // A key in the song's own mode takes the new mode.
        let c_major = key(PitchClass::C, ScaleKind::Major);
        let c_dorian = key(PitchClass::C, ScaleKind::Dorian);
        assert_eq!(
            moved_key(key(PitchClass::G, ScaleKind::Major), c_major, c_dorian),
            key(PitchClass::G, ScaleKind::Dorian)
        );
        // A row as the key moves no notes, so the G major section of a C
        // major song stays in G major (it used to go up to B with the
        // row's first note, E).
        let row = Scale::row(crate::harmony::ToneRow::CHROMATIC.starting_on(PitchClass::E));
        let g_major = key(PitchClass::G, ScaleKind::Major);
        assert_eq!(moved_key(g_major, c_major, row), g_major);
        // From one row to another, a section in the old row takes the
        // new one, a fifth above the song's key note as before; a section
        // in a row of its own keeps its row and moves with the key note.
        use crate::harmony::RowPreset;
        let row_on = |name: &str, root: PitchClass| {
            Scale::row(RowPreset::named(name).unwrap().row.starting_on(root))
        };
        let (lineup, berg) = (
            row_on("Lineup row", PitchClass::C),
            row_on("Berg's Lyric Suite", PitchClass::D),
        );
        assert_eq!(
            moved_key(row_on("Lineup row", PitchClass::G), lineup, berg),
            row_on("Berg's Lyric Suite", PitchClass::A)
        );
        assert_eq!(
            moved_key(row_on("Webern's Symphony", PitchClass::G), lineup, berg),
            row_on("Webern's Symphony", PitchClass::A)
        );
    }
}
