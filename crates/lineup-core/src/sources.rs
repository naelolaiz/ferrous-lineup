//! Where the parts of a song come from.
//!
//! Opening a demo brings in every part of the song from it: its
//! patterns, instruments, time signature, chords, tempo and swing.
//! Applying a style with only some of its parts ticked brings in only
//! those, and the rest stay as they were. So a song can end up with its
//! patterns from one style, its tempo from another and its chords from
//! the demo, and each part remembers where it came from.
//!
//! Each part also remembers what it was like when it came, as a number
//! worked out from it (see [`Project::part_fingerprint`]), so the app can tell
//! when you changed it since: "edited, was Tango marcato". A part that
//! came from nowhere, as in a new song, is yours.

use std::hash::{DefaultHasher, Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::Project;
use crate::library::apply::StyleParts;

/// A part of a song that a demo or a style can bring in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SongPart {
    /// What the accompaniment plays: the band's style, or the song's own
    /// patterns when it has no band.
    Patterns,
    /// The sounds of the channels and the band's players.
    Instruments,
    TimeSignature,
    /// The chord sheet.
    Chords,
    Tempo,
    Swing,
}

impl SongPart {
    pub const ALL: [SongPart; 6] = [
        SongPart::Patterns,
        SongPart::Instruments,
        SongPart::TimeSignature,
        SongPart::Chords,
        SongPart::Tempo,
        SongPart::Swing,
    ];

    /// Its name, such as "Time signature".
    pub fn name(self) -> &'static str {
        match self {
            SongPart::Patterns => "Patterns",
            SongPart::Instruments => "Instruments",
            SongPart::TimeSignature => "Time signature",
            SongPart::Chords => "Chords",
            SongPart::Tempo => "Tempo",
            SongPart::Swing => "Swing",
        }
    }

    fn bit(self) -> u8 {
        1 << self as u8
    }
}

/// A set of parts, such as those ticked when a style is applied.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PartSet(u8);

impl PartSet {
    pub const NONE: PartSet = PartSet(0);
    pub const ALL: PartSet = PartSet((1u8 << SongPart::ALL.len()) - 1);

    pub fn of(parts: &[SongPart]) -> Self {
        PartSet(parts.iter().fold(0, |bits, part| bits | part.bit()))
    }

    /// The parts ticked in a style's options.
    pub fn ticked(parts: StyleParts) -> Self {
        let ticked = [
            (parts.rhythm, SongPart::Patterns),
            (parts.instruments, SongPart::Instruments),
            (parts.time_signature, SongPart::TimeSignature),
            (parts.progression, SongPart::Chords),
            (parts.tempo, SongPart::Tempo),
            (parts.swing, SongPart::Swing),
        ];
        PartSet(
            ticked
                .into_iter()
                .filter(|(on, _)| *on)
                .fold(0, |bits, (_, part)| bits | part.bit()),
        )
    }

    pub fn contains(self, part: SongPart) -> bool {
        self.0 & part.bit() != 0
    }

    /// These parts, but not those in `other`.
    pub fn minus(self, other: PartSet) -> Self {
        PartSet(self.0 & !other.0)
    }

    /// The parts both in these and in `other`.
    pub fn and(self, other: PartSet) -> Self {
        PartSet(self.0 & other.0)
    }
}

/// What kind of thing a part came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceKind {
    Demo,
    Style,
    Rhythm,
    /// A style saved from the style editor.
    SavedStyle,
}

/// What a part came from: a demo or a style, by name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Source {
    pub kind: SourceKind,
    pub name: String,
}

impl Source {
    pub fn new(kind: SourceKind, name: &str) -> Self {
        Source {
            kind,
            name: name.to_string(),
        }
    }
}

/// Where one part came from, and what it was like then.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PartSource {
    pub(crate) source: Source,
    /// [`Project::part_fingerprint`] of the part as it came, or `None` once
    /// it is known to have changed (read from a file that said so).
    pub(crate) print: Option<u64>,
}

/// Where each part of a song came from. A part with no source is yours.
///
/// It is not part of the music: two songs that play the same are equal,
/// wherever their parts came from. So a song saved and opened again is
/// the same song, although a part edited before saving no longer
/// remembers what it was like when it came, only that it was edited.
#[derive(Debug, Clone, Default)]
pub(crate) struct Sources([Option<PartSource>; 6]);

impl PartialEq for Sources {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Sources {
    pub(crate) fn get(&self, part: SongPart) -> Option<&PartSource> {
        self.0[part as usize].as_ref()
    }

    pub(crate) fn set(&mut self, part: SongPart, source: Option<PartSource>) {
        self.0[part as usize] = source;
    }
}

/// Where a part of the song came from, as [`Project::part_source`] tells it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartOrigin<'a> {
    pub source: &'a Source,
    /// Whether you changed the part since it came.
    pub edited: bool,
}

impl Project {
    /// Where `part` came from, or `None` if it is yours.
    pub fn part_source(&self, part: SongPart) -> Option<PartOrigin<'_>> {
        let came = self.sources.get(part)?;
        Some(PartOrigin {
            source: &came.source,
            edited: came.print != Some(self.part_fingerprint(part)),
        })
    }

    /// Brings every part of the song in from `source`, as it is now: what
    /// opening a demo does.
    pub fn bring_in_all(&mut self, source: &Source) {
        for part in SongPart::ALL {
            let print = Some(self.part_fingerprint(part));
            self.sources.set(
                part,
                Some(PartSource {
                    source: source.clone(),
                    print,
                }),
            );
        }
    }

    /// Changes the song with `change`, which brings parts in from
    /// `source`. The parts in `taken` come from `source` from now on,
    /// even those that were already as `source` has them: a 4/4 style
    /// applied to a song in 4/4 gives it its time signature all the same.
    /// Of the other parts it changes, those in `edited` are your edits, as
    /// a time signature picked in the Time box: they still came from
    /// where they came from, and are edited now. Any other part it changes
    /// only follows along, as chords moved into a new time signature are
    /// still the same chords, and is as it came if it was before.
    pub fn bring_in<R>(
        &mut self,
        source: &Source,
        taken: PartSet,
        edited: PartSet,
        change: impl FnOnce(&mut Project) -> R,
    ) -> R {
        let before = SongPart::ALL.map(|part| self.part_fingerprint(part));
        let result = change(self);
        for (part, before) in SongPart::ALL.into_iter().zip(before) {
            let now = self.part_fingerprint(part);
            if taken.contains(part) && !edited.contains(part) {
                self.sources.set(
                    part,
                    Some(PartSource {
                        source: source.clone(),
                        print: Some(now),
                    }),
                );
            } else if now != before && !edited.contains(part) {
                self.follow_part(part, before, now);
            }
        }
        result
    }

    /// Changes the song with `change`, which only moves parts along
    /// without anyone editing them, as filling in the chords heard in
    /// the patterns does: each part keeps where it came from.
    pub fn follow_along<R>(&mut self, change: impl FnOnce(&mut Project) -> R) -> R {
        let before = SongPart::ALL.map(|part| self.part_fingerprint(part));
        let result = change(self);
        for (part, before) in SongPart::ALL.into_iter().zip(before) {
            let now = self.part_fingerprint(part);
            if now != before {
                self.follow_part(part, before, now);
            }
        }
        result
    }

    /// A part changed from `before` to `now` along with others: if it was
    /// as it came, it still is. If you had edited it, it stays edited.
    fn follow_part(&mut self, part: SongPart, before: u64, now: u64) {
        if let Some(came) = &self.sources.0[part as usize]
            && came.print == Some(before)
        {
            let source = came.source.clone();
            self.sources.set(
                part,
                Some(PartSource {
                    source,
                    print: Some(now),
                }),
            );
        }
    }

    /// A number worked out from everything `part` is made of, which
    /// changes when the part does: two different numbers mean the part
    /// changed. Only for comparing within one run of the app; files
    /// keep whether a part was edited instead.
    pub fn part_fingerprint(&self, part: SongPart) -> u64 {
        let mut hasher = DefaultHasher::new();
        match part {
            SongPart::Patterns => match self.band() {
                // The band plays its style, whatever the chords make of it.
                Some(band) => {
                    band.name().hash(&mut hasher);
                    band.accents_only.hash(&mut hasher);
                    band.custom_rhythm
                        .as_ref()
                        .map(|style| style.id())
                        .hash(&mut hasher);
                }
                None => {
                    for pattern in &self.patterns {
                        pattern.length().hash(&mut hasher);
                        for channel in 0..self.channels.len() {
                            for note in pattern.notes(channel) {
                                (
                                    channel,
                                    note.pitch.midi(),
                                    note.start,
                                    note.length,
                                    note.velocity,
                                    note.nudge,
                                    note.trim,
                                )
                                    .hash(&mut hasher);
                            }
                        }
                    }
                }
            },
            SongPart::Instruments => {
                for channel in &self.channels {
                    format!("{:?}", channel.kind).hash(&mut hasher);
                    channel.program.hash(&mut hasher);
                }
                if let Some(band) = self.band() {
                    band.instrument_name().hash(&mut hasher);
                    band.custom_instruments
                        .as_ref()
                        .map(|style| style.id())
                        .hash(&mut hasher);
                }
            }
            SongPart::TimeSignature => {
                format!("{:?}", self.time_signature).hash(&mut hasher);
                format!("{:?}", self.playlist.time_markers()).hash(&mut hasher);
            }
            SongPart::Chords => {
                format!("{:?}", self.playlist.chord_markers()).hash(&mut hasher);
            }
            SongPart::Tempo => {
                self.tempo.bpm().to_bits().hash(&mut hasher);
                format!("{:?}", self.playlist.tempo_markers()).hash(&mut hasher);
            }
            SongPart::Swing => self.swing.hash(&mut hasher),
        }
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Tempo;

    fn demo() -> Source {
        Source::new(SourceKind::Demo, "El Choclo")
    }

    fn style() -> Source {
        Source::new(SourceKind::Style, "Tango marcato")
    }

    fn from(project: &Project, part: SongPart) -> Option<(&str, bool)> {
        project
            .part_source(part)
            .map(|origin| (origin.source.name.as_str(), origin.edited))
    }

    #[test]
    fn a_new_song_is_all_yours() {
        let project = Project::demo();
        assert!(
            SongPart::ALL
                .iter()
                .all(|&part| from(&project, part).is_none())
        );
    }

    #[test]
    fn a_part_brought_in_is_edited_once_changed_and_not_after_undoing_it() {
        let mut project = Project::demo();
        project.bring_in_all(&demo());
        assert_eq!(from(&project, SongPart::Tempo), Some(("El Choclo", false)));
        let tempo = project.tempo;
        project.scale_tempo(Tempo::new(tempo.bpm() + 10.0));
        assert_eq!(from(&project, SongPart::Tempo), Some(("El Choclo", true)));
        assert_eq!(from(&project, SongPart::Swing), Some(("El Choclo", false)));
        project.scale_tempo(tempo);
        assert_eq!(from(&project, SongPart::Tempo), Some(("El Choclo", false)));
    }

    #[test]
    fn only_the_parts_taken_come_from_the_style() {
        let mut project = Project::demo();
        project.bring_in_all(&demo());
        let tempo = Tempo::new(project.tempo.bpm() + 20.0);
        project.bring_in(
            &style(),
            PartSet::of(&[SongPart::Tempo, SongPart::Swing]),
            PartSet::NONE,
            |project| {
                project.scale_tempo(tempo);
                project.swing = 40;
                // Changed along with them, not taken: it follows along.
                project.time_signature = crate::TimeSignature::new(3, 4);
            },
        );
        assert_eq!(
            from(&project, SongPart::Tempo),
            Some(("Tango marcato", false))
        );
        assert_eq!(
            from(&project, SongPart::Swing),
            Some(("Tango marcato", false))
        );
        assert_eq!(
            from(&project, SongPart::TimeSignature),
            Some(("El Choclo", false))
        );
        assert_eq!(from(&project, SongPart::Chords), Some(("El Choclo", false)));
    }

    #[test]
    fn a_part_taken_comes_from_the_style_even_if_it_was_already_as_the_style_has_it() {
        let mut project = Project::demo();
        project.bring_in_all(&demo());
        let tempo = project.tempo;
        project.scale_tempo(Tempo::new(tempo.bpm() + 10.0));
        project.bring_in(
            &style(),
            PartSet::of(&[SongPart::Tempo, SongPart::TimeSignature]),
            PartSet::NONE,
            |project| project.scale_tempo(tempo),
        );
        assert_eq!(
            from(&project, SongPart::TimeSignature),
            Some(("Tango marcato", false))
        );
        assert_eq!(
            from(&project, SongPart::Tempo),
            Some(("Tango marcato", false))
        );
        assert_eq!(from(&project, SongPart::Swing), Some(("El Choclo", false)));
    }

    #[test]
    fn an_edited_part_stays_edited_when_others_move_it_along() {
        let mut project = Project::demo();
        project.bring_in_all(&demo());
        project.swing = 30;
        project.follow_along(|project| project.swing = 50);
        assert_eq!(from(&project, SongPart::Swing), Some(("El Choclo", true)));
    }

    #[test]
    fn parts_you_choose_are_edited() {
        let mut project = Project::demo();
        project.bring_in_all(&demo());
        project.bring_in(
            &style(),
            PartSet::of(&[SongPart::Patterns, SongPart::TimeSignature]),
            PartSet::of(&[SongPart::TimeSignature]),
            |project| project.time_signature = crate::TimeSignature::new(3, 4),
        );
        assert_eq!(
            from(&project, SongPart::TimeSignature),
            Some(("El Choclo", true))
        );
    }
}
