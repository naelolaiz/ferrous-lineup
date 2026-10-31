//! The band playing a twelve-tone style: the forms of the row written on
//! the chord sheet, in the style's textures.
//!
//! A twelve-tone style (see [`crate::library::rows`]) says how each of
//! its voices plays a row: which form, as a melody or as chords, how
//! fast, in which octaves. Played by the band, its voices play the forms
//! written on the chord sheet (`P0`, `RI5`, `I7`…), each for as long as
//! it is written there, on the row of the key there: the plan of the
//! piece is yours to write, as the chords are in a tonal style. Each
//! voice reads its own form the way the sheet's form reads the row: where
//! the sheet says `I5`, a voice that plays the row backwards plays `RI5`,
//! and a voice that plays the partner plays the partner of `I5`. The
//! keys voice plays in place of the piano, the bass voice in place of
//! the bass, and a melody in place of the soloist; the guitar, the
//! strings and the organ do not play. The drums and the horns play as
//! with any other style.

use crate::chord_sheet::{ChordMarker, HarmonyName};
use crate::general_midi;
use crate::harmony::{RowLabel, Scale, ScaleKind};
use crate::library::RowStyle;
use crate::library::apply::{BASS, KEYS, LEAD, StyleParts};
use crate::library::comping::{BassPart, Played};
use crate::library::interpret::{self, Role};
use crate::library::rows::{RowVoice, SECTIONS};
use crate::pattern::NoteEvent;
use crate::project::Project;
use crate::serial::write_row;

use super::{Band, Player, Score, louder_in_b};

/// The voice of `style` that `player` plays: the keys for the piano, the
/// bass for the bass, the melody for the soloist.
fn voice_of(style: &RowStyle, player: Player) -> Option<&'static RowVoice> {
    let channel = match player {
        Player::Piano => KEYS,
        Player::Bass => BASS,
        Player::Soloist => LEAD,
        _ => return None,
    };
    style.voices.iter().find(|voice| voice.channel == channel)
}

/// Whether `style` decides what `player` plays: every player but the
/// drums and the horns, and the soloist only when the style has a melody
/// for it (else a soloist switched on improvises as in any style).
pub(super) fn takes_over(style: &RowStyle, player: Player) -> bool {
    match player {
        Player::Drums | Player::Horns => false,
        Player::Soloist => voice_of(style, player).is_some(),
        _ => true,
    }
}

/// The channel name and the General MIDI instrument `player` plays
/// `style` on, if it has a voice in it: the chords on a piano and the
/// bass on a bass, as when the style is tried in a pattern, and the
/// melody on a clarinet, which has every register the styles' lines go
/// to.
pub(super) fn sound(style: &RowStyle, player: Player) -> Option<(&'static str, u8)> {
    voice_of(style, player)?;
    Some(match player {
        Player::Piano => (KEYS, general_midi::ACOUSTIC_GRAND_PIANO),
        Player::Bass => (BASS, general_midi::FINGER_BASS),
        _ => (player.name(), general_midi::CLARINET),
    })
}

/// The notes of `played`, struck at once and held: the bass's note for
/// the bass, the others' chord for the rest.
fn struck(played: &Played, voice: &RowVoice) -> Vec<NoteEvent> {
    let length = played.end.saturating_sub(played.start).max(1);
    let notes = match (voice.channel, played.bass) {
        (BASS, BassPart::Pedal(note)) => vec![note],
        (BASS, BassPart::Chord { note, .. }) => vec![note],
        (BASS, BassPart::Rest) => Vec::new(),
        _ => played.voicing.clone(),
    };
    notes
        .into_iter()
        .map(|note| NoteEvent::new(note, played.start, length))
        .collect()
}

impl Band {
    /// Who plays `style`: a player for each of its voices, the drums if
    /// the band has drums, and the horns if they are on.
    pub(super) fn row_players(&self, style: &RowStyle) -> Vec<Player> {
        Player::ALL
            .into_iter()
            .filter(|&player| match player {
                Player::Drums => self.has_drums() || self.sections_drum(),
                Player::Horns => self.has_horns(),
                Player::Soloist => voice_of(style, player).is_some() || self.has_soloist(),
                _ => voice_of(style, player).is_some(),
            })
            .collect()
    }
}

impl Project {
    /// What `player` plays of `style` all through `score`: its voice over
    /// each form of the row on the chord sheet, where the key is a row,
    /// nothing if it has none. Shots and holds strike the notes the band
    /// plays there and hold them, rests leave the chords and the bass
    /// silent; the melody goes on through them, as a soloist does.
    pub(super) fn row_notes(
        &self,
        band: &Band,
        style: &RowStyle,
        player: Player,
        score: &Score,
    ) -> Vec<(usize, NoteEvent)> {
        let (Some(voice), Some(channel)) = (voice_of(style, player), band.channel(player)) else {
            return Vec::new();
        };
        let mut notes = Vec::new();
        for &(start, stop, harmony) in &score.spans {
            let (HarmonyName::Row(label), ScaleKind::Row(row)) =
                (harmony, score.keys.key_at(start).kind)
            else {
                continue;
            };
            let written = write_row(row, &voice.writing(row, label), stop - start);
            notes.extend(written.into_iter().map(|note| NoteEvent {
                start: note.start + start,
                ..note
            }));
        }
        let mut notes = match player {
            Player::Soloist => notes,
            _ => score
                .breaks
                .apply(notes, &score.harmonies, |played| struck(played, voice)),
        };
        let role = match player {
            Player::Soloist => Role::Melody,
            _ => Role::Accompaniment,
        };
        let seed = interpret::seed("band", style.name, player.name());
        interpret::perform(&mut notes, score.time, role, seed);
        louder_in_b(&mut notes, &score.variations);
        notes.into_iter().map(|note| (channel, note)).collect()
    }

    /// Whether the chord sheet holds forms of the row and nothing else
    /// but bars of no chord: the key is a row where each is written.
    pub fn sheet_holds_row_forms(&self) -> bool {
        let keys = self.key_map();
        let mut forms = 0;
        for marker in &self.playlist.chord_markers {
            match marker.harmony {
                HarmonyName::NoChord => {}
                HarmonyName::Row(_)
                    if matches!(keys.key_at(marker.start).kind, ScaleKind::Row(_)) =>
                {
                    forms += 1;
                }
                _ => return false,
            }
        }
        forms > 0
    }

    /// Has the band play `style`, from the chord sheet.
    ///
    /// The row is the key's, when the key is a row, so a style plays your
    /// row: its textures are the style's, the notes yours. When the sheet
    /// holds forms of the row (see [`Project::sheet_holds_row_forms`]),
    /// they stay, in the order they were written, and the style plays
    /// them. Otherwise the style writes its plan on the sheet, from the
    /// first bar to the end of the song (or one pass of the style, in an
    /// empty song): the row as written, backwards, upside down and both,
    /// part after part, each as long as one pass of the style.
    ///
    /// With `parts.progression`, the style's own row and plan replace
    /// the key and the sheet, the row starting on the key note; a key
    /// that is not a row becomes the style's row that way too.
    ///
    /// A band coming in mutes the song's own chords and bass, as any band
    /// does. With `parts.rhythm`, the style's drums play, and a style
    /// without drums leaves the song without; otherwise the drums that
    /// played go on. The time signature, the tempo and the swing come
    /// with their boxes, as from any style.
    pub fn apply_row_style(&mut self, style: &'static RowStyle, parts: StyleParts) {
        let writes = parts.progression || !self.sheet_holds_row_forms();
        self.bring_a_band_of_voices(style.rhythm(), parts);
        if let Some(band) = &mut self.band {
            band.row = Some(style);
            band.micromode = None;
        }
        self.take_timing(style.time, style.tempo, style.rhythm(), parts);
        if writes {
            self.write_row_plan(style, parts.progression);
        }
        self.band_channels(parts.instruments);
        self.write_band();
    }

    /// Writes the plan of `style` on the chord sheet, from the first bar
    /// to the end of the song or of the last bar written (one pass of the
    /// style in an empty song), and puts the whole song in its row: the
    /// key's own, unless `own`, or else the style's, starting on the key
    /// note.
    fn write_row_plan(&mut self, style: &RowStyle, own: bool) {
        let meter = self.meter();
        let written = self
            .playlist
            .chord_markers
            .last()
            .map_or(0, |last| meter.bar_at(last.start).end());
        let end = match self.song_length().max(written) {
            0 => meter.bar(style.bars.max(1)).start,
            end => end,
        };
        let row = style.row_in(self.scale, own);
        self.scale = Scale::row(row);
        self.playlist.key_markers.clear();
        // A plan holds a form for a whole part: the band plays to its end.
        if let Some(band) = &mut self.band {
            band.end = band.end.max(end);
        }
        let bars: Vec<usize> = meter
            .bars()
            .take_while(|bar| bar.start < end)
            .map(|bar| bar.start)
            .collect();
        let plan: Vec<ChordMarker> = bars
            .chunks(style.bars.max(1))
            .zip(SECTIONS.iter().cycle())
            .map(|(part, &form)| {
                let label = RowLabel::new(form, row.first().semitone());
                ChordMarker::new(part[0], HarmonyName::Row(label))
            })
            .collect();
        self.set_chords(0, end, &plan);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::band::tests::part;
    use crate::harmony::{PitchSet, RowForm, RowPreset, ToneRow};
    use crate::library::ROW_STYLES;
    use crate::pitch::{Note, PitchClass};

    fn named(name: &str) -> &'static RowStyle {
        RowStyle::named(name).unwrap()
    }

    /// The notes `player` starts playing from `start` up to `end`, in
    /// order, as pitch classes.
    fn heard(project: &Project, player: Player, start: usize, end: usize) -> Vec<PitchClass> {
        part(project, player)
            .into_iter()
            .filter(|&(at, ..)| at >= start && at < end)
            .map(|(.., midi)| Note::new(midi).pitch_class())
            .collect()
    }

    #[test]
    fn a_style_writes_its_plan_and_the_band_plays_it() {
        for style in &ROW_STYLES {
            let mut project = Project::empty();
            project.apply_row_style(style, StyleParts::default());
            let band = project.band().expect("a band");
            assert_eq!(band.row, Some(style));
            assert_eq!(band.name(), style.name);
            // The key is the style's row, and the sheet its plan.
            assert_eq!(
                project.scale.tone_row(),
                Some(style.row()),
                "{}",
                style.name
            );
            assert!(project.sheet_holds_row_forms(), "{}", style.name);
            let markers = project.playlist().chord_markers().to_vec();
            assert_eq!(
                markers[0].harmony,
                HarmonyName::Row(RowLabel::new(RowForm::Prime, 0))
            );
            // Each voice plays only the notes of the row, and the check
            // reads its notes as forms of the row with none out of order.
            for player in [Player::Piano, Player::Bass, Player::Soloist] {
                if voice_of(style, player).is_none() {
                    assert!(band.channel(player).is_none(), "{}", style.name);
                    continue;
                }
                assert!(
                    !heard(&project, player, 0, usize::MAX).is_empty(),
                    "{} {player:?}",
                    style.name
                );
            }
            for player in [Player::Guitar, Player::Strings, Player::Organ] {
                assert!(band.channel(player).is_none());
            }
            assert_eq!(
                band.players().contains(&Player::Drums),
                style.rhythm().is_some(),
                "{}",
                style.name
            );
        }
    }

    /// In a song whose key is a row, a style plays that row, and the
    /// forms written on the sheet, in their order.
    #[test]
    fn a_style_plays_your_row_and_your_plan() {
        let berg = RowPreset::named("Berg's Lyric Suite")
            .unwrap()
            .row
            .starting_on(PitchClass::D);
        let mut project = Project::empty();
        project.scale = Scale::row(berg);
        let meter = project.meter();
        let plan = [
            RowLabel::new(RowForm::Prime, 2),
            RowLabel::new(RowForm::RetrogradeInversion, 7),
            RowLabel::new(RowForm::Inversion, 5),
            RowLabel::new(RowForm::Retrograde, 2),
        ];
        for (index, label) in plan.iter().enumerate() {
            let bar = meter.bar(2 * index);
            project.set_chords(
                bar.start,
                meter.bar(2 * index + 1).end(),
                &[ChordMarker::new(bar.start, HarmonyName::Row(*label))],
            );
        }
        let sheet = project.playlist().chord_markers().to_vec();
        assert!(project.sheet_holds_row_forms());
        let style = named("Row melody");
        project.apply_row_style(style, StyleParts::default());
        // The sheet and the row are as they were.
        assert_eq!(project.playlist().chord_markers(), sheet.as_slice());
        assert_eq!(project.scale.tone_row(), Some(berg));
        // The melody plays each form of the sheet from its first note:
        // the style chains P then R, so a bar of I5 starts with I5. The
        // last form lasts to the end of its bar, the end of the sheet.
        for (index, label) in plan.iter().enumerate() {
            let (start, end) = (meter.bar(2 * index).start, meter.bar(2 * index + 1).end());
            let notes = heard(&project, Player::Soloist, start, end);
            let count = notes.len().min(12);
            assert!(count >= 8, "{label}");
            assert_eq!(
                notes[..count],
                berg.form(*label).pitch_classes()[..count],
                "{label}"
            );
        }
        // The Chord progression box brings the style's own row and plan.
        let parts = StyleParts {
            progression: true,
            ..StyleParts::default()
        };
        project.apply_row_style(style, parts);
        assert_eq!(
            project.scale.tone_row(),
            Some(style.row().starting_on(PitchClass::D))
        );
    }

    /// A voice that plays the partner fills each bar with the six notes
    /// the chord leaves out, with any combinatorial row, and plays its
    /// own form with a row that has no partner.
    #[test]
    fn hexachord_pads_play_the_partner_of_your_row() {
        let style = named("Hexachord pads");
        let chromatic = ToneRow::CHROMATIC.starting_on(PitchClass::E);
        let mut project = Project::empty();
        project.scale = Scale::row(chromatic);
        project.apply_row_style(style, StyleParts::default());
        let meter = project.meter();
        for number in 0..project.playlist().chord_markers().len() * style.bars {
            let bar = meter.bar(number);
            let all: PitchSet = [Player::Piano, Player::Soloist]
                .into_iter()
                .flat_map(|player| heard(&project, player, bar.start, bar.end()))
                .collect();
            assert_eq!(all.len(), 12, "bar {number}");
        }
    }

    #[test]
    fn a_song_takes_a_row_style_from_its_sheet() {
        let mut project = crate::library::demo::song_demo(&crate::library::SONGS[0]);
        let length = project.song_length();
        let key = project.scale.root;
        let style = named("Triad chorale");
        project.apply_row_style(style, StyleParts::default());
        // A tonal sheet gives way to the style's plan, all through the
        // song, on its row from the key note, and the song's own chords
        // go quiet.
        assert!(project.sheet_holds_row_forms());
        assert_eq!(project.scale.tone_row(), Some(style.row().starting_on(key)));
        let markers = project.playlist().chord_markers();
        assert!(markers.len() > 1);
        assert!(markers.last().unwrap().start < length);
        assert!(!project.band().unwrap().muted().channels.is_empty());
        assert!(!part(&project, Player::Piano).is_empty());
        assert!(!part(&project, Player::Bass).is_empty());
        // The parts take turns: as written, backwards, upside down, both.
        let forms: Vec<RowForm> = markers
            .iter()
            .filter_map(|marker| match marker.harmony {
                HarmonyName::Row(label) => Some(label.form),
                _ => None,
            })
            .take(4)
            .collect();
        assert_eq!(forms, SECTIONS[..forms.len()]);
    }

    #[test]
    fn a_locked_keys_of_the_song_keeps_playing_its_part() {
        let song = crate::library::SONGS
            .iter()
            .find(|song| song.name == "Ode to Joy")
            .unwrap();
        let mut project = crate::library::demo::song_demo(song);
        let keys = project
            .channels()
            .iter()
            .position(|channel| channel.name == KEYS)
            .unwrap();
        let notes = |project: &Project| -> Vec<Vec<NoteEvent>> {
            project
                .patterns()
                .iter()
                .map(|pattern| pattern.notes(keys).to_vec())
                .collect()
        };
        let before = notes(&project);
        project.set_locked(keys, true);
        project.apply_row_style(named("Twelve-tone lo-fi"), StyleParts::default());
        // The song's Keys plays as it did, and the band leaves the piano's
        // part to it; the bass is the style's.
        let after = notes(&project);
        assert_eq!(after[..before.len()], before[..]);
        assert!(after[before.len()..].iter().all(Vec::is_empty));
        assert!(!project.channels()[keys].muted);
        assert!(part(&project, Player::Piano).is_empty());
        assert!(!part(&project, Player::Bass).is_empty());
    }

    #[test]
    fn a_row_band_is_saved_and_read_again() {
        let mut project = Project::empty();
        project.apply_row_style(named("Twelve-tone lo-fi"), StyleParts::default());
        let text = crate::file::to_text(&project).unwrap();
        let read = crate::file::from_text(&text).unwrap();
        assert_eq!(read.band().unwrap().row, Some(named("Twelve-tone lo-fi")));
        for player in Player::ALL {
            assert_eq!(part(&read, player), part(&project, player), "{player:?}");
        }
        assert_eq!(crate::file::format_for(&project), 31);
        // Another style takes the band back to chords and bass.
        project.set_band_style(crate::band::BandStyle::named("Bossa nova"));
        let band = project.band().unwrap();
        assert_eq!(band.row, None);
        assert!(band.channel(Player::Piano).is_some());
    }
}
