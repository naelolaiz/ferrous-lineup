//! A style for part of a song. A band does not have to play a whole song
//! one way: the verse can be a bossa nova and the bridge a swing, as a
//! bandleader calls a change of feel. Markers on the chord sheet say
//! where each style starts. Before the first one, and from a marker back
//! to the song's style, the band plays its own style ([`Band::style`]).
//!
//! A section's style changes how the band plays: the piano's comping,
//! the bass line, the drums, the guitar's and the pads' patterns, and
//! how much the band swings. It does not change who plays or on which
//! instruments: those stay the band's. The drums play the fill of the
//! style they leave in the bar before a change, and crash into the new
//! one, as they do into a variation.
//!
//! [`Band::style`]: super::Band::style

use serde::{Deserialize, Serialize};

use super::{Band, BandStyle};
use crate::i18n::trf;
use crate::pattern::NoteEvent;
use crate::project::Project;

/// Where the band starts playing another style, on the playlist's
/// timeline: always at the start of a bar. A marker without a style goes
/// back to the song's own.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StyleMarker {
    pub start: usize,
    pub style: Option<&'static BandStyle>,
}

impl StyleMarker {
    pub fn new(start: usize, style: Option<&'static BandStyle>) -> Self {
        StyleMarker { start, style }
    }
}

/// A style marker as it is saved: the style by its name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct StyleMarkerFile {
    pub start: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

impl From<&StyleMarker> for StyleMarkerFile {
    fn from(marker: &StyleMarker) -> Self {
        StyleMarkerFile {
            start: marker.start,
            style: marker.style.map(|style| style.name.to_string()),
        }
    }
}

impl StyleMarkerFile {
    pub(crate) fn marker(&self) -> Result<StyleMarker, String> {
        let style = match &self.style {
            Some(name) => Some(BandStyle::named(name).ok_or_else(|| {
                trf(
                    "a section of its band plays \"{}\", a style this version does not know",
                    &[&name],
                )
            })?),
            None => None,
        };
        Ok(StyleMarker::new(self.start, style))
    }
}

/// The style of the last marker at or before `step`, or `None`, the
/// song's own style, before the first.
pub(crate) fn style_in(markers: &[StyleMarker], step: usize) -> Option<&'static BandStyle> {
    markers
        .iter()
        .rev()
        .find(|marker| marker.start <= step)
        .and_then(|marker| marker.style)
}

/// Where the style playing at `step` stops: the next marker after it, if
/// there is one.
pub(crate) fn next_change(markers: &[StyleMarker], step: usize) -> Option<usize> {
    markers
        .iter()
        .map(|marker| marker.start)
        .find(|&start| start > step)
}

/// Puts `markers` in order, the later of two on one step winning, and
/// drops the ones that change nothing: a marker for the style that is
/// already playing.
pub(crate) fn tidy(markers: &mut Vec<StyleMarker>) {
    markers.sort_by_key(|marker| marker.start);
    markers.dedup_by(|later, earlier| {
        later.start == earlier.start && {
            *earlier = *later;
            true
        }
    });
    let mut playing = None;
    markers.retain(|marker| {
        let changes = marker.style != playing;
        playing = marker.style;
        changes
    });
}

/// The notes of `notes` (by channel, on the song's timeline) that start
/// where `style` plays, as `markers` say, each cut where the style stops.
pub(crate) fn where_it_plays(
    notes: Vec<(usize, NoteEvent)>,
    markers: &[StyleMarker],
    style: Option<&'static BandStyle>,
) -> impl Iterator<Item = (usize, NoteEvent)> {
    notes
        .into_iter()
        .filter(move |(_, note)| style_in(markers, note.start) == style)
        .map(move |(channel, mut note)| {
            if let Some(change) = next_change(markers, note.start)
                && note.start + note.length > change
            {
                note.length = change - note.start;
                note.trim = 0;
            }
            (channel, note)
        })
}

/// The styles `markers` call for, each once, the song's own (`None`)
/// first.
pub(crate) fn styles_played(markers: &[StyleMarker]) -> Vec<Option<&'static BandStyle>> {
    let mut styles = vec![None];
    for marker in markers {
        if !styles.contains(&marker.style) {
            styles.push(marker.style);
        }
    }
    styles
}

impl Band {
    /// This band playing `style` instead of its own: its instruments,
    /// form and variations, with the style's patterns. `None` is the band
    /// as it is.
    pub(crate) fn playing(&self, style: Option<&'static BandStyle>) -> Band {
        let mut band = self.clone();
        if let Some(style) = style {
            band.style = style;
            band.custom_rhythm = None;
            band.harmony_style = None;
            band.accents_only = false;
            band.drums = super::BandDrums::Style;
            band.micromode = None;
            band.row = None;
        }
        band
    }
}

impl Project {
    /// Where the band changes style, in order. Empty without a band, or
    /// when it plays its own style all through.
    pub fn section_styles(&self) -> &[StyleMarker] {
        self.band
            .as_ref()
            .map_or(&[], |band| band.styles.as_slice())
    }

    /// The style the band plays at `step` in place of its own, if a
    /// marker says so.
    pub fn section_style_at(&self, step: usize) -> Option<&'static BandStyle> {
        style_in(self.section_styles(), step)
    }

    /// The name of what the band plays at `step`: a section's style, or
    /// the band's own. Empty without a band.
    pub fn style_name_at(&self, step: usize) -> &str {
        match (self.section_style_at(step), self.band()) {
            (Some(style), _) => style.name,
            (None, Some(band)) => band.name(),
            (None, None) => "",
        }
    }

    /// Has the band play `style` (or, with `None`, its own) from step
    /// `start` (the start of a bar) up to `end`, where it goes back to
    /// what it played there before. Without an `end`, it plays it up to
    /// the next change, which stays. Returns `false`, changing nothing,
    /// without a band.
    pub fn set_section_style(
        &mut self,
        start: usize,
        end: Option<usize>,
        style: Option<&'static BandStyle>,
    ) -> bool {
        let after = end.map(|end| (end, self.section_style_at(end)));
        let Some(band) = &mut self.band else {
            return false;
        };
        let markers = &mut band.styles;
        markers.retain(|marker| match end {
            Some(end) => marker.start < start || marker.start >= end,
            None => marker.start != start,
        });
        markers.push(StyleMarker::new(start, style));
        if let Some((end, after)) = after {
            markers.push(StyleMarker::new(end, after));
        }
        tidy(markers);
        self.write_band();
        true
    }

    /// How much band pattern `pattern` swings, if a section's style says:
    /// the swing of the style playing where the pattern's first clip is.
    /// `None` for the song's own patterns, and where the band plays its
    /// own style, which swing as the song does.
    pub(crate) fn section_swing(&self, pattern: usize) -> Option<u8> {
        let markers = self.section_styles();
        if markers.is_empty() || self.patterns.get(pattern)?.player.is_none() {
            return None;
        }
        let start = self
            .playlist
            .clips
            .iter()
            .filter(|clip| clip.pattern == pattern)
            .map(|clip| clip.start)
            .min()?;
        style_in(markers, start).map(BandStyle::swing)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::band::Player;
    use crate::chord_sheet::read_bar;
    use crate::project::Drum;
    use crate::{ChannelKind, SampleSource};

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

    /// The notes `player` plays from step `start` to `end`, on the
    /// song's timeline, as (step, pitch).
    fn notes(project: &Project, player: Player, start: usize, end: usize) -> Vec<(usize, u8)> {
        let mut notes = Vec::new();
        for clip in &project.playlist().clips {
            let Some(pattern) = project.pattern(clip.pattern) else {
                continue;
            };
            if pattern.player != Some(player) {
                continue;
            }
            for channel in 0..pattern.rows() {
                for note in pattern.notes(channel) {
                    let at = clip.start + note.start;
                    if (start..end).contains(&at) {
                        notes.push((at, note.pitch.midi()));
                    }
                }
            }
        }
        notes.sort_unstable();
        notes
    }

    /// The same notes, moved to start at 0.
    fn from_zero(notes: &[(usize, u8)], start: usize) -> Vec<(usize, u8)> {
        notes
            .iter()
            .map(|&(at, pitch)| (at - start, pitch))
            .collect()
    }

    fn bar(project: &Project, number: usize) -> (usize, usize) {
        let bar = project.meter().bar(number);
        (bar.start, bar.end())
    }

    fn style(name: &str) -> Option<&'static BandStyle> {
        Some(BandStyle::named(name).unwrap())
    }

    #[test]
    fn markers_keep_only_changes() {
        let mut markers = vec![
            StyleMarker::new(32, style("Bossa nova")),
            StyleMarker::new(0, None),
            StyleMarker::new(16, style("Bossa nova")),
            StyleMarker::new(16, style("Swing")),
            StyleMarker::new(48, style("Swing")),
        ];
        tidy(&mut markers);
        assert_eq!(
            markers,
            vec![
                StyleMarker::new(16, style("Swing")),
                StyleMarker::new(32, style("Bossa nova")),
                StyleMarker::new(48, style("Swing")),
            ]
        );
    }

    #[test]
    fn a_section_plays_as_its_style_does() {
        let bars = ["C", "Am", "F", "G", "C", "Am", "F", "G"];
        let mut project = song("Pop beat", &bars);
        let alone = song("Bossa nova", &bars);
        let (start, _) = bar(&project, 4);
        let (_, end) = bar(&project, 7);
        assert!(project.set_section_style(start, Some(end), style("Bossa nova")));
        assert_eq!(project.section_style_at(start), style("Bossa nova"));
        assert_eq!(project.section_style_at(0), None);
        assert_eq!(project.style_name_at(start), "Bossa nova");
        assert_eq!(project.style_name_at(0), "Pop beat");
        // The second half: the bossa nova's bass, as the bossa nova song
        // plays it there.
        assert_eq!(
            notes(&project, Player::Bass, start, end),
            notes(&alone, Player::Bass, start, end)
        );
        // The first half as before.
        let before = song("Pop beat", &bars);
        assert_eq!(
            notes(&project, Player::Bass, 0, start),
            notes(&before, Player::Bass, 0, start)
        );
    }

    #[test]
    fn a_section_plays_its_drums_and_starts_its_pattern_on_its_first_bar() {
        let bars = ["C", "Am", "F", "G", "C", "Am", "F", "G"];
        let mut project = song("Pop beat", &bars);
        let (start, _) = bar(&project, 2);
        let (_, end) = bar(&project, 3);
        assert!(project.set_section_style(start, Some(end), style("Bossa nova")));
        // Bars 3 and 4 play the bossa nova's comping from its start, as
        // the bossa nova plays bars 1 and 2.
        let alone = song("Bossa nova", &["F", "G"]);
        let (_, two) = bar(&alone, 1);
        assert_eq!(
            from_zero(&notes(&project, Player::Piano, start, end), start),
            notes(&alone, Player::Piano, 0, two)
        );
        // Bar 3 starts with a crash, and bar 2 ends in a fill: the drums
        // play something there they do not play in bar 1.
        let crash = project
            .channels()
            .iter()
            .position(|channel| {
                channel.kind == ChannelKind::Sampler(SampleSource::Builtin(Drum::Crash))
            })
            .unwrap();
        let drums_at = |project: &Project, at: usize| {
            project.playlist().clips.iter().any(|clip| {
                project.pattern(clip.pattern).is_some_and(|pattern| {
                    pattern.player == Some(Player::Drums)
                        && pattern
                            .notes(crash)
                            .iter()
                            .any(|note| clip.start + note.start == at)
                })
            })
        };
        assert!(drums_at(&project, start));
        let (two_start, two_end) = bar(&project, 1);
        let one = from_zero(&notes(&project, Player::Drums, 0, two_start), 0);
        let fill = from_zero(
            &notes(&project, Player::Drums, two_start, two_end),
            two_start,
        );
        assert_ne!(one, fill);
        // Back in the ballad after it: the same as the ballad alone.
        let before = song("Pop beat", &bars);
        let (after, _) = bar(&project, 5);
        let (_, last) = bar(&project, 7);
        assert_eq!(
            notes(&project, Player::Bass, after, last),
            notes(&before, Player::Bass, after, last)
        );
    }

    #[test]
    fn a_section_without_drums_rests_them() {
        let bars = ["C", "F", "G", "C"];
        let mut project = song("Pop beat", &bars);
        let piano = style("Chorale");
        let (start, _) = bar(&project, 2);
        assert!(project.set_section_style(start, None, piano));
        let (_, end) = bar(&project, 3);
        assert!(notes(&project, Player::Drums, start, end).is_empty());
        assert!(!notes(&project, Player::Piano, start, end).is_empty());
        assert!(!notes(&project, Player::Drums, 0, start).is_empty());
    }

    #[test]
    fn going_back_to_the_songs_style_drops_the_markers() {
        let mut project = song("Pop beat", &["C", "F", "G", "C"]);
        let before = notes(&project, Player::Piano, 0, MAX);
        let (start, _) = bar(&project, 1);
        let (_, end) = bar(&project, 2);
        project.set_section_style(start, Some(end), style("Bossa nova"));
        assert_eq!(project.section_styles().len(), 2);
        project.set_section_style(start, Some(end), None);
        assert!(project.section_styles().is_empty());
        assert_eq!(notes(&project, Player::Piano, 0, MAX), before);
    }

    const MAX: usize = crate::playlist::MAX_SONG_STEPS;

    #[test]
    fn a_swing_section_swings_in_a_straight_song() {
        let mut project = song("Pop beat", &["C", "F", "G", "C"]);
        assert_eq!(project.swing, 0);
        let (start, _) = bar(&project, 2);
        project.set_section_style(start, None, style("Swing"));
        let swing = BandStyle::named("Swing").unwrap().swing();
        assert!(swing > 0);
        let pattern_at = |project: &Project, at: usize| {
            project
                .playlist()
                .clips
                .iter()
                .find(|clip| {
                    clip.start == at
                        && project
                            .pattern(clip.pattern)
                            .is_some_and(|pattern| pattern.player == Some(Player::Bass))
                })
                .map(|clip| clip.pattern)
                .unwrap()
        };
        assert_eq!(project.pattern_swing(pattern_at(&project, start)), swing);
        assert_eq!(project.pattern_swing(pattern_at(&project, 0)), 0);
    }

    #[test]
    fn markers_move_with_the_song_and_are_saved() {
        let mut project = song("Pop beat", &["C", "F", "G", "C"]);
        let (start, _) = bar(&project, 2);
        project.set_section_style(start, None, style("Bossa nova"));
        let text = crate::file::to_text(&project).unwrap();
        assert!(text.contains("Bossa nova"));
        let read = crate::file::from_text(&text).unwrap();
        assert_eq!(read.section_styles(), project.section_styles());
        assert_eq!(
            notes(&read, Player::Bass, 0, MAX),
            notes(&project, Player::Bass, 0, MAX)
        );
        let bar_steps = project.meter().first().steps_per_bar();
        assert!(project.make_room(0, bar_steps));
        assert_eq!(
            project.section_style_at(start + bar_steps),
            style("Bossa nova")
        );
        assert_eq!(project.section_style_at(start), None);
    }
}
