//! The song's form, as the band plays it: an intro, the chorus some
//! number of times, and an ending.
//!
//! "Chorus" is the jazz musician's word: one time through the whole
//! tune, from the top of the chord sheet to its last bar. It is not the
//! chorus of a pop song, the part that comes back after each verse: a
//! pop song's verses and choruses together make one chorus here.
//!
//! ```text
//! | Intro | ‖: the bars written on the sheet :‖ ×3 | Ending |
//! ```
//!
//! - The **intro** is bars put in front of the chorus, with chords that
//!   lead into its first one (see [`intro_harmonies`]). They are written
//!   on the sheet like any others, so they can be changed.
//! - The **chorus** plays as many times as [`Form::choruses`] says. The
//!   playlist and the sheet show it once; the arrangement plays it again
//!   and again (see [`crate::arrangement`]), as it does a section that
//!   repeats.
//! - The **ending** is two bars the band adds after the last chorus, on
//!   the key's tonic (see [`ending_harmony`]): one bar played as the style
//!   goes, then the chord struck once and held, with a crash of the
//!   cymbal. It is not written on the sheet. A chord typed in it makes
//!   the chorus longer, and the ending moves on after it.

use serde::{Deserialize, Serialize};

use crate::chord_sheet::{ChordMarker, HarmonyName};
use crate::harmony::{ChordFamily, ChordSymbol, Scale, ScaleKind};
use crate::library::numeral;
use crate::playlist::{MAX_REPEATS, MAX_SONG_STEPS};
use crate::project::Project;

/// The intros the band can make, in bars.
pub const INTRO_BARS: [usize; 3] = [2, 4, 8];

/// How long the ending is, in bars.
pub const ENDING_BARS: usize = 2;

/// How the band plays the song through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Form {
    /// How many steps of intro come before the chorus: 0 for none.
    pub intro: usize,
    /// How many times the chorus plays: 1 to [`MAX_REPEATS`].
    pub choruses: usize,
    /// Whether the band plays an ending after the last chorus.
    pub ending: bool,
}

impl Default for Form {
    fn default() -> Self {
        Form {
            intro: 0,
            choruses: 1,
            ending: false,
        }
    }
}

impl Form {
    /// Whether it is the form of a song that plays once, as written,
    /// with no intro or ending: what a file leaves out.
    pub fn is_plain(&self) -> bool {
        *self == Form::default()
    }
}

/// The ending: where it is on the playlist's timeline, and the harmony
/// the band plays it on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ending {
    pub start: usize,
    pub end: usize,
    pub harmony: HarmonyName,
}

/// The turnaround in a major key, as numerals from the chord it leads
/// to: I, vi7, ii7, V7, and back to I. The first is the chord itself.
const MAJOR_TURNAROUND: [&str; 4] = ["", "vi7", "ii7", "V7"];

/// The same in a minor key: i, bVImaj7, iiø7, V7, and back to i.
const MINOR_TURNAROUND: [&str; 4] = ["", "bVImaj7", "iiø7", "V7"];

/// The harmonies of an intro `bars` long that leads into `first`, the
/// chorus's first harmony: one for each bar, or `None` where the harmony
/// before goes on. Empty if `bars` is not one of [`INTRO_BARS`].
///
/// Into a chord, the intro is a turnaround, the chords a band plays to
/// lead back to the top of a tune. They are counted from the chord the
/// intro leads to, as if it were the tonic, so they lead into it
/// whatever its place in the key:
///
/// - 2 bars: ii7, V7 (Dm7 G7 into C); into a minor chord iiø7, V7
///   (Bm7b5 E7 into Am).
/// - 4 bars: the chord itself, vi7, ii7, V7 (C Am7 Dm7 G7); into a
///   minor chord i, bVImaj7, iiø7, V7 (Am Fmaj7 Bm7b5 E7).
/// - 8 bars: the same four chords, two bars each.
///
/// A set, a micromode or a row form has no dominant to lead into it, so
/// the intro plays it from the first bar, as a vamp; so does a
/// diminished chord, which no turnaround resolves to.
pub(crate) fn intro_harmonies(first: HarmonyName, bars: usize) -> Vec<Option<HarmonyName>> {
    if !INTRO_BARS.contains(&bars) {
        return Vec::new();
    }
    let vamp = || {
        let mut harmonies = vec![None; bars];
        harmonies[0] = Some(first);
        harmonies
    };
    let HarmonyName::Chord(symbol) = first else {
        return vamp();
    };
    let (kind, numerals) = match symbol.quality.family() {
        ChordFamily::Major => (ScaleKind::Major, MAJOR_TURNAROUND),
        ChordFamily::Minor => (ScaleKind::NaturalMinor, MINOR_TURNAROUND),
        ChordFamily::Diminished => return vamp(),
    };
    let key = Scale::new(symbol.root.pitch_class(), kind);
    let chords: Vec<HarmonyName> = numerals
        .iter()
        .map(|text| match numeral::parse(text) {
            Some(numeral) if !text.is_empty() => HarmonyName::Chord(numeral.symbol_in(key)),
            _ => first,
        })
        .collect();
    match bars {
        2 => chords[2..].iter().copied().map(Some).collect(),
        4 => chords.into_iter().map(Some).collect(),
        _ => chords
            .into_iter()
            .flat_map(|chord| [Some(chord), None])
            .collect(),
    }
}

/// The harmony the ending is played on, in `key`, the key the last
/// chorus ends in, when the chorus starts with `first`.
///
/// In a key of seven notes, a song ends on its tonic: the chorus's
/// first chord if it is on the tonic (C7 for a blues in C, Cmaj7 for a
/// tune that starts on it), or else the key's own triad (C in C major,
/// Am in A minor). In a set or a row, where nothing is the tonic, the
/// piece ends where it started, on its first harmony.
pub(crate) fn ending_harmony(first: HarmonyName, key: Scale) -> HarmonyName {
    if let HarmonyName::Chord(symbol) = first
        && symbol.root.pitch_class() == key.root
    {
        return HarmonyName::Chord(ChordSymbol::new(symbol.root, symbol.quality));
    }
    if key.kind.is_tonal()
        && key.is_heptatonic()
        && let Some(triad) = key.triad(0)
    {
        return HarmonyName::Chord(ChordSymbol::new(key.spell(triad.root), triad.quality));
    }
    first
}

impl Project {
    /// The band's form, if the song has a band.
    pub fn form(&self) -> Option<Form> {
        self.band.as_ref().map(|band| band.form)
    }

    /// The chorus on the playlist's timeline: from the end of the intro
    /// up to the end of the last bar written. `None` without a band, or
    /// with nothing after the intro.
    pub fn chorus(&self) -> Option<(usize, usize)> {
        let band = self.band.as_ref()?;
        let (start, end) = (band.form.intro, band.end);
        (start < end).then_some((start, end))
    }

    /// The chorus, and how many times it plays, if more than once.
    pub(crate) fn repeated_chorus(&self) -> Option<(usize, usize, usize)> {
        let choruses = self.band.as_ref()?.form.choruses;
        let (start, end) = self.chorus()?;
        (choruses > 1).then_some((start, end, choruses))
    }

    /// The first harmony of the chorus, leaving out "no chord".
    fn chorus_opening(&self) -> Option<HarmonyName> {
        let (start, end) = self.chorus()?;
        let sounding = self
            .chord_at(start)
            .filter(|marker| marker.harmony != HarmonyName::NoChord);
        let later = self.playlist.chord_markers.iter().find(|marker| {
            (start..end).contains(&marker.start) && marker.harmony != HarmonyName::NoChord
        });
        sounding.or(later.copied()).map(|marker| marker.harmony)
    }

    /// The ending the band plays, if it plays one: the [`ENDING_BARS`]
    /// bars after the chorus, and their harmony (see [`ending_harmony`]).
    pub fn ending(&self) -> Option<Ending> {
        let band = self.band.as_ref()?;
        if !band.form.ending {
            return None;
        }
        let (_, chorus_end) = self.chorus()?;
        let first = self.chorus_opening()?;
        let key = self.key_at(chorus_end - 1);
        let meter = self.meter();
        let last = meter
            .bars()
            .skip_while(|bar| bar.start < chorus_end)
            .nth(ENDING_BARS - 1)?;
        (last.end() <= MAX_SONG_STEPS).then(|| Ending {
            start: chorus_end,
            end: last.end(),
            harmony: ending_harmony(first, key),
        })
    }

    /// Has the band play the chorus `choruses` times. Returns `false`,
    /// changing nothing, without a band.
    pub fn set_choruses(&mut self, choruses: usize) -> bool {
        let Some(band) = &mut self.band else {
            return false;
        };
        band.form.choruses = choruses.clamp(1, MAX_REPEATS);
        self.write_band();
        true
    }

    /// Has the band end the song with an ending of its own, or stop at
    /// the end of the last chorus. Returns `false`, changing nothing,
    /// without a band.
    pub fn set_ending(&mut self, ending: bool) -> bool {
        let Some(band) = &mut self.band else {
            return false;
        };
        band.form.ending = ending;
        self.write_band();
        true
    }

    /// Puts an intro of `bars` bars in front of the chorus, with chords
    /// that lead into its first harmony (see [`intro_harmonies`]), in
    /// place of the intro the song had; 0 takes the intro away. The song
    /// moves later, or earlier, to make room: its melody, its markers and
    /// its sections too.
    ///
    /// Returns `false`, changing nothing, without a band, without a
    /// harmony to lead into, for a length not in [`INTRO_BARS`], or if
    /// the song would get too long.
    pub fn set_intro(&mut self, bars: usize) -> bool {
        let Some(old) = self.form().map(|form| form.intro) else {
            return false;
        };
        let harmonies = match (bars, self.chorus_opening()) {
            (0, _) => Vec::new(),
            (bars, Some(first)) if INTRO_BARS.contains(&bars) => intro_harmonies(first, bars),
            _ => return false,
        };
        let steps = bars * self.time_signature.steps_per_bar();
        if self.song_length().saturating_sub(old) + steps > MAX_SONG_STEPS {
            return false;
        }
        let made = self.moving_the_band(|project| {
            project.cut_start(old);
            steps == 0 || project.make_room(0, steps)
        });
        if !made {
            return false;
        }
        let meter = self.meter();
        for (index, harmony) in harmonies.into_iter().enumerate() {
            let bar = meter.bar(index);
            let markers: Vec<ChordMarker> = harmony
                .map(|harmony| ChordMarker::new(bar.start, harmony))
                .into_iter()
                .collect();
            self.set_chords(bar.start, bar.end(), &markers);
        }
        if let Some(band) = &mut self.band {
            band.form.intro = steps;
        }
        self.write_band();
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::band::{BandStyle, Player};
    use crate::chord_sheet::read_bar;
    use crate::harmony::ScaleKind::*;
    use crate::pitch::PitchClass;

    /// A song with a band and `bars` written on its sheet, one per bar.
    fn song(bars: &[&str]) -> Project {
        let mut project = Project::with_band();
        let meter = project.meter();
        for (number, text) in bars.iter().enumerate() {
            let bar = meter.bar(number);
            let markers = read_bar(text, &bar, project.key_at(bar.start)).unwrap();
            assert!(project.set_chords(bar.start, bar.end(), &markers));
        }
        project.follow_chord_sheet();
        project
    }

    /// The sheet, bar by bar, as it reads: "-" where a harmony goes on.
    fn sheet(project: &Project) -> Vec<String> {
        let meter = project.meter();
        let end = project.band().unwrap().end();
        meter
            .bars()
            .take_while(|bar| bar.start < end)
            .map(|bar| {
                let names: Vec<String> = project
                    .playlist()
                    .chord_markers()
                    .iter()
                    .filter(|marker| (bar.start..bar.end()).contains(&marker.start))
                    .map(|marker| marker.harmony.to_string())
                    .collect();
                if names.is_empty() {
                    "-".to_string()
                } else {
                    names.join(" ")
                }
            })
            .collect()
    }

    /// Where `player` plays, on the timeline: (step, length) of each note.
    fn notes(project: &Project, player: Player, channel: usize) -> Vec<(usize, usize)> {
        let mut notes = Vec::new();
        for clip in project.playlist().clips() {
            let pattern = project.pattern(clip.pattern).unwrap();
            if pattern.player == Some(player) {
                for note in pattern.notes(channel) {
                    notes.push((clip.start + note.start, note.length));
                }
            }
        }
        notes.sort_unstable();
        notes
    }

    fn channel(project: &Project, name: &str) -> usize {
        project
            .channels()
            .iter()
            .position(|channel| channel.name == name)
            .unwrap()
    }

    #[test]
    fn choruses_play_the_sheet_again_and_again() {
        let mut project = song(&["C", "Am", "F", "G7"]);
        assert_eq!(project.arrangement().length, 64);
        assert!(project.set_choruses(3));
        assert_eq!(project.song_length(), 64, "the playlist shows it once");
        let played = project.arrangement();
        assert_eq!(played.length, 192);
        assert_eq!(played.timeline_step(70), Some(6));
        assert_eq!(played.pass_at(70).unwrap().chorus, Some((2, 3)));
        // A loop inside the chorus loops only its bars.
        project.set_loop_region(Some(crate::playlist::LoopRegion::new(16, 48)));
        assert_eq!(project.playback_arrangement().length, 32);
        // At least once.
        assert!(project.set_choruses(0));
        assert_eq!(project.form().unwrap().choruses, 1);
    }

    #[test]
    fn an_intro_leads_into_the_chorus() {
        let mut project = song(&["C", "Am", "F", "G7"]);
        assert!(project.set_intro(2));
        assert_eq!(sheet(&project), ["Dm7", "G7", "C", "Am", "F", "G7"]);
        assert_eq!(project.chorus(), Some((32, 96)));
        project.set_choruses(2);
        let played = project.arrangement();
        // The intro once, then the chorus twice.
        assert_eq!(played.length, 32 + 2 * 64);
        assert_eq!(played.timeline_step(96), Some(32), "back to the top");
        assert_eq!(played.pass_at(10).unwrap().chorus, None);

        // Another length takes the old intro's place.
        assert!(project.set_intro(8));
        assert_eq!(
            sheet(&project),
            [
                "C", "-", "Am7", "-", "Dm7", "-", "G7", "-", "C", "Am", "F", "G7"
            ]
        );
        assert!(project.set_intro(0));
        assert_eq!(sheet(&project), ["C", "Am", "F", "G7"]);
        assert_eq!(project.chorus(), Some((0, 64)));
        assert!(!project.set_intro(3), "not one of the lengths offered");
        assert!(
            !Project::with_band().set_intro(4),
            "nothing to lead into on an empty sheet"
        );
    }

    #[test]
    fn the_intro_moves_the_melody_along() {
        let mut project = song(&["Am", "E7"]);
        let melody = project.add_pattern().unwrap();
        project.add_clip(crate::playlist::Clip::new(melody, 0, 0));
        assert!(project.set_intro(4));
        let starts: Vec<usize> = project
            .playlist()
            .clips()
            .iter()
            .filter(|clip| clip.pattern == melody)
            .map(|clip| clip.start)
            .collect();
        assert_eq!(starts, [64]);
        assert_eq!(sheet(&project)[..4], ["Am", "Fmaj7", "Bm7b5", "E7"]);
        assert!(!project.is_frozen(Player::Piano), "moving is not an edit");
    }

    #[test]
    fn the_ending_holds_the_tonic() {
        let mut project = song(&["C", "Am", "Dm7", "G7"]);
        assert!(project.set_ending(true));
        let ending = project.ending().unwrap();
        assert_eq!((ending.start, ending.end), (64, 96));
        assert_eq!(ending.harmony.to_string(), "C");
        assert_eq!(project.song_length(), 96);
        assert_eq!(sheet(&project).len(), 4, "the ending is not written");

        // The last bar: one chord struck on the downbeat and held, over
        // a held root, and one crash of the cymbal.
        let piano = project.player_channel(Player::Piano).unwrap();
        let bass = project.player_channel(Player::Bass).unwrap();
        let last: Vec<(usize, usize)> = notes(&project, Player::Piano, piano)
            .into_iter()
            .filter(|&(at, _)| at >= 80)
            .collect();
        assert!(!last.is_empty());
        assert!(last.iter().all(|&(at, length)| at == 80 && length == 16));
        let bass_last: Vec<(usize, usize)> = notes(&project, Player::Bass, bass)
            .into_iter()
            .filter(|&(at, _)| at >= 80)
            .collect();
        assert_eq!(bass_last, [(80, 16)]);
        let crash = channel(&project, "Crash");
        assert_eq!(notes(&project, Player::Drums, crash), [(80, 1)]);
        let snare = channel(&project, "Snare");
        assert!(
            notes(&project, Player::Drums, snare)
                .iter()
                .all(|&(at, _)| at < 80),
            "nothing after the last hit"
        );
        // The first bar of the ending goes on in the style.
        assert!(
            notes(&project, Player::Drums, snare)
                .iter()
                .any(|&(at, _)| (64..80).contains(&at))
        );

        // A chord typed in the ending makes the chorus longer.
        let bar = project.meter().bar(4);
        let markers = read_bar("F", &bar, project.scale).unwrap();
        project.set_chords(bar.start, bar.end(), &markers);
        project.follow_chord_sheet();
        assert_eq!(project.chorus(), Some((0, 80)));
        assert_eq!(project.ending().unwrap().start, 80);
        assert_eq!(project.song_length(), 112);
    }

    #[test]
    fn the_drums_fill_into_each_chorus() {
        // Eight bars, so the eight-bar phrase and the chorus end agree;
        // with a two-bar intro, its last bar fills too.
        let mut project = song(&["C", "Am", "F", "G7", "C", "Am", "F", "G7"]);
        project.set_intro(2);
        project.set_choruses(2);
        let snare = channel(&project, "Snare");
        let in_bar = |project: &Project, bar: usize| {
            let start = project.meter().bar(bar).start;
            notes(project, Player::Drums, snare)
                .iter()
                .filter(|&&(at, _)| (start..start + 16).contains(&at))
                .count()
        };
        // The Pop beat's snare plays twice a bar; its fill, a roll.
        assert_eq!(in_bar(&project, 2), 2);
        assert!(in_bar(&project, 1) > 2, "into the chorus");
        assert!(in_bar(&project, 9) > 2, "into the next chorus");
        assert_eq!(in_bar(&project, 8), 2);
    }

    #[test]
    fn the_form_is_saved() {
        // A style of piano, bass and drums alone, which format 14 holds.
        let three = |project: &mut Project| project.set_band_style(BandStyle::named("Boom bap"));
        let mut project = song(&["C", "G7"]);
        three(&mut project);
        project.set_intro(2);
        project.set_choruses(4);
        project.set_ending(true);
        assert_eq!(crate::file::format_for(&project), 14);
        let text = crate::file::to_text(&project).unwrap();
        let loaded = crate::file::from_text(&text).unwrap();
        assert_eq!(loaded, project);
        assert_eq!(
            loaded.form(),
            Some(Form {
                intro: 32,
                choruses: 4,
                ending: true
            })
        );
        let mut plain = song(&["C", "G7"]);
        three(&mut plain);
        assert_eq!(crate::file::format_for(&plain), 13);
    }

    fn chord(text: &str) -> HarmonyName {
        HarmonyName::parse(text).unwrap()
    }

    fn names(harmonies: &[Option<HarmonyName>]) -> Vec<String> {
        harmonies
            .iter()
            .map(|harmony| harmony.map_or("-".to_string(), |h| h.to_string()))
            .collect()
    }

    #[test]
    fn an_intro_is_a_turnaround_into_the_first_chord() {
        assert_eq!(names(&intro_harmonies(chord("C"), 2)), ["Dm7", "G7"]);
        assert_eq!(
            names(&intro_harmonies(chord("C"), 4)),
            ["C", "Am7", "Dm7", "G7"]
        );
        assert_eq!(
            names(&intro_harmonies(chord("Bb"), 8)),
            ["Bb", "-", "Gm7", "-", "Cm7", "-", "F7", "-"]
        );
        // Into a minor chord: the minor turnaround, with its raised
        // leading note in the dominant.
        assert_eq!(names(&intro_harmonies(chord("Am"), 2)), ["Bm7b5", "E7"]);
        assert_eq!(
            names(&intro_harmonies(chord("F#m7"), 4)),
            ["F#m7", "Dmaj7", "G#m7b5", "C#7"]
        );
        // Flat keys are spelled with flats.
        assert_eq!(names(&intro_harmonies(chord("Eb"), 2)), ["Fm7", "Bb7"]);
        // A blues starts on a seventh chord, a major one.
        assert_eq!(
            names(&intro_harmonies(chord("C7"), 4)),
            ["C7", "Am7", "Dm7", "G7"]
        );
    }

    #[test]
    fn groups_of_notes_are_played_as_a_vamp() {
        let set = chord("3-11 T2");
        assert_eq!(intro_harmonies(set, 4), [Some(set), None, None, None]);
        assert_eq!(names(&intro_harmonies(chord("Bdim"), 2)), ["Bdim", "-"]);
        assert!(intro_harmonies(chord("C"), 3).is_empty(), "not offered");
    }

    #[test]
    fn a_song_ends_on_its_tonic() {
        let c_major = Scale::new(PitchClass::C, Major);
        let a_minor = Scale::new(PitchClass::A, NaturalMinor);
        assert_eq!(ending_harmony(chord("C7"), c_major), chord("C7"));
        assert_eq!(ending_harmony(chord("Cmaj7/E"), c_major), chord("Cmaj7"));
        assert_eq!(ending_harmony(chord("F"), c_major), chord("C"));
        assert_eq!(ending_harmony(chord("Dm7"), a_minor), chord("Am"));
        let bb_major = Scale::new(PitchClass::ASharp, Major);
        assert_eq!(ending_harmony(chord("Gm"), bb_major), chord("Bb"));
        let set = chord("3-11 T2");
        assert_eq!(
            ending_harmony(set, Scale::new(PitchClass::C, Chromatic)),
            set
        );
    }
}
