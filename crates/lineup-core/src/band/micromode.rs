//! The band playing a micromode style: the forms written on the chord
//! sheet, in the style's textures.
//!
//! A micromode style (see [`crate::library::micromodes`]) says how each
//! of its voices plays a walk of forms: held chords, a pulse, a line, an
//! arpeggio or a bass. Played by the band, its voices play whatever
//! forms the chord sheet holds instead of a walk of their own, so the
//! order of the forms is yours to write, as the order of the chords is
//! in a tonal style: harmony keys, the Progression menu of a set key, or
//! the forms typed by name. The keys voice play in place of the piano,
//! the bass voice in place of the bass, and a line in place of the
//! soloist; the guitar, the strings and the organ do not play. The drums
//! and the horns play as with any other style.
//!
//! The forms can be those of any set, not only of a micromode: a style
//! is a way of playing, and plays a tetrachord or a hexachord as it
//! plays three notes.

use crate::chord_sheet::{ChordMarker, HarmonyName};
use crate::general_midi;
use crate::harmony::{Degree, Micromode, ScaleKind};
use crate::library::MicromodeStyle;
use crate::library::apply::{BASS, KEYS, LEAD, StyleParts};
use crate::library::comping::{BassPart, Played};
use crate::library::interpret::{self, Role};
use crate::library::micromodes::{DEMO_SECTIONS, MicroVoice};
use crate::micromodal::{self, MicroWriting, Span, Texture};
use crate::pattern::NoteEvent;
use crate::project::Project;
use crate::tempo::Tempo;

use super::{Band, BandDrums, BandStyle, Player, Score, louder_in_b};

/// The band style a micromode style without drums is played over: one
/// without drums of its own.
const WITHOUT_DRUMS: &str = "Chorale";

/// The voice of `style` that `player` plays: the keys for the piano, the
/// bass for the bass, the line for the soloist.
fn voice_of(style: &MicromodeStyle, player: Player) -> Option<&'static MicroVoice> {
    let channel = match player {
        Player::Piano => KEYS,
        Player::Bass => BASS,
        Player::Soloist => LEAD,
        _ => return None,
    };
    style.voices.iter().find(|voice| voice.channel == channel)
}

/// Whether `style` decides what `player` plays: every player but the
/// drums and the horns, and the soloist only when the style has a line
/// for it (else a soloist switched on improvises as in any style).
pub(super) fn takes_over(style: &MicromodeStyle, player: Player) -> bool {
    match player {
        Player::Drums | Player::Horns => false,
        Player::Soloist => voice_of(style, player).is_some(),
        _ => true,
    }
}

/// The channel name and the General MIDI instrument `player` plays
/// `style` on, if it has a voice in it: the chords on a piano and the
/// bass on a bass, as when the style is tried in a pattern, and the line
/// on a clarinet, which has every register the styles' lines go to.
pub(super) fn sound(style: &MicromodeStyle, player: Player) -> Option<(&'static str, u8)> {
    voice_of(style, player)?;
    Some(match player {
        Player::Piano => (KEYS, general_midi::ACOUSTIC_GRAND_PIANO),
        Player::Bass => (BASS, general_midi::FINGER_BASS),
        _ => (player.name(), general_midi::CLARINET),
    })
}

/// The forms of `harmonies`, each with the steps it lasts: a chord's
/// notes, or the notes a set, a micromode or a group of a row is voiced
/// with. No chord has none: the voices rest.
fn spans(harmonies: &[Played]) -> Vec<Span> {
    harmonies
        .iter()
        .map(|played| Span {
            start: played.start,
            end: played.end,
            form: match played.bass {
                BassPart::Chord { chord, .. } => chord.pitch_set(),
                _ => played
                    .voicing
                    .iter()
                    .map(|note| note.pitch_class())
                    .collect(),
            },
        })
        .collect()
}

impl Band {
    /// Who plays `style`: a player for each of its voices, the drums if
    /// the band has drums, and the horns if they are on.
    pub(super) fn micromode_players(&self, style: &MicromodeStyle) -> Vec<Player> {
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
    /// the forms of the chord sheet (see [`micromodal::write_spans`]),
    /// nothing if it has none. Shots and holds strike the form and hold
    /// it, rests leave the chords and the bass silent; the line goes on
    /// through them, as a soloist does.
    pub(super) fn micromode_notes(
        &self,
        band: &Band,
        style: &MicromodeStyle,
        player: Player,
        score: &Score,
    ) -> Vec<(usize, NoteEvent)> {
        let (Some(voice), Some(channel)) = (voice_of(style, player), band.channel(player)) else {
            return Vec::new();
        };
        let writing = voice.writing();
        let notes = micromodal::write_spans(&spans(&score.harmonies), &writing);
        let struck = MicroWriting {
            texture: match writing.texture {
                Texture::Bass => Texture::Bass,
                _ => Texture::Chords,
            },
            ..writing
        };
        let mut notes = match player {
            Player::Soloist => notes,
            _ => score.breaks.apply(notes, &score.harmonies, |played| {
                micromodal::write_spans(&spans(std::slice::from_ref(played)), &struck)
            }),
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

    /// Whether the chord sheet holds forms of the key and nothing else
    /// but bars of no chord: the key is a pitch-class set (a micromode
    /// is one) where each harmony is written, and each is that set moved
    /// or turned upside down.
    pub fn sheet_holds_forms(&self) -> bool {
        let keys = self.key_map();
        let mut forms = 0;
        for marker in &self.playlist.chord_markers {
            let key = keys.key_at(marker.start);
            match marker.harmony {
                HarmonyName::NoChord => {}
                HarmonyName::Set(set) | HarmonyName::Micromode(set)
                    if matches!(key.kind, ScaleKind::Set(_)) && Degree::of(key, set).is_some() =>
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
    /// When the sheet holds forms of the key (see
    /// [`Project::sheet_holds_forms`]), they stay, in the order they were
    /// written, and the style plays them. Otherwise, or with
    /// `parts.progression`, the style writes its own walk on the sheet
    /// from the first bar to the end of the song (or one pass of the
    /// style, in an empty song), each form lasting as long as the style
    /// holds it, and the key becomes the style's micromode on the song's
    /// key note, all through the song.
    ///
    /// A band coming in mutes the song's own chords and bass, as any band
    /// does. With `parts.rhythm`, the style's drums play, if it has any;
    /// otherwise the drums that played go on. The time signature, the
    /// tempo and the swing come with their boxes, as from any style.
    pub fn apply_micromode_style(&mut self, style: &'static MicromodeStyle, parts: StyleParts) {
        let writes = parts.progression || !self.sheet_holds_forms();
        let rhythm = style
            .rhythm()
            .and_then(|rhythm| BandStyle::named(rhythm.name))
            .filter(|_| parts.rhythm);
        let mut drums_come = false;
        match &mut self.band {
            None => {
                let base = rhythm
                    .or_else(|| BandStyle::named(WITHOUT_DRUMS))
                    .expect("the band has a style without drums");
                let mut band = Band::new(base);
                if rhythm.is_none() {
                    band.drums = BandDrums::Song;
                }
                band.end = self.song_length();
                band.since = self.channels.len();
                if band.end > 0 {
                    band.muted = self.mute_the_songs_own(rhythm.is_some());
                }
                self.band = Some(band);
            }
            Some(band) => {
                match rhythm {
                    Some(rhythm) => {
                        drums_come = band.drums == BandDrums::Song;
                        band.style = rhythm;
                        band.drums = BandDrums::Style;
                    }
                    // The drums the band played stay.
                    None => {
                        if band.drums == BandDrums::Style {
                            band.drums = match band.custom_rhythm.take() {
                                Some(saved) => BandDrums::Saved(saved),
                                None => BandDrums::Of(band.style),
                            };
                        }
                    }
                }
                band.custom_rhythm = None;
                band.harmony_style = None;
                band.accents_only = false;
                band.tidy_drums();
            }
        }
        if let Some(band) = &mut self.band {
            band.micromode = Some(style);
        }
        self.drums_come(drums_come);
        let time = self.meter();
        if parts.time_signature
            && (time.first() != style.time || !self.playlist.time_markers.is_empty())
        {
            if self.playlist.chord_markers.is_empty() && self.song_length() == 0 {
                self.time_signature = style.time;
                self.playlist.time_markers.clear();
            } else {
                crate::remeter::remeter(self, style.time);
            }
        }
        if parts.tempo && time == self.meter() {
            self.take_tempo(Tempo::new(style.tempo));
        }
        // The style swings as its drums do, and plays straight without.
        if parts.swing {
            self.swing = style.rhythm().map_or(0, |rhythm| rhythm.swing);
        }
        if writes {
            self.write_micromode_walk(style);
        }
        self.band_channels(parts.instruments);
        self.write_band();
    }

    /// Writes the walk of `style` on the chord sheet, from the first bar
    /// to the end of the song or of the last bar written (one pass of the
    /// style in an empty song), and puts the whole song in its micromode,
    /// on the key note the song starts in.
    fn write_micromode_walk(&mut self, style: &MicromodeStyle) {
        let meter = self.meter();
        let written = self
            .playlist
            .chord_markers
            .last()
            .map_or(0, |last| meter.bar_at(last.start).end());
        let end = match self.song_length().max(written) {
            0 => style.length(),
            end => end,
        };
        let key = self.scale.root;
        self.scale = style.scale(key);
        self.playlist.key_markers.clear();
        let first = style.first_form(key, DEMO_SECTIONS[0].0);
        let steps = style.steps.max(1);
        let markers: Vec<ChordMarker> = style
            .forms(first, end)
            .into_iter()
            .enumerate()
            .map(|(index, form)| {
                let harmony = match Micromode::of(form) {
                    Some(_) => HarmonyName::Micromode(form),
                    None => HarmonyName::Set(form),
                };
                ChordMarker::new(index * steps, harmony)
            })
            .take_while(|marker| marker.start < end)
            .collect();
        self.set_chords(0, end, &markers);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::band::tests::part;
    use crate::harmony::{PitchSet, Scale};
    use crate::library::MICROMODE_STYLES;
    use crate::pitch::PitchClass;

    fn named(name: &str) -> &'static MicromodeStyle {
        MicromodeStyle::named(name).unwrap()
    }

    /// The pitch classes `player` starts playing from `start` up to
    /// `end`.
    fn heard(project: &Project, player: Player, start: usize, end: usize) -> PitchSet {
        part(project, player)
            .into_iter()
            .filter(|&(at, ..)| at >= start && at < end)
            .map(|(.., midi)| crate::pitch::Note::new(midi).pitch_class())
            .collect()
    }

    #[test]
    fn a_style_writes_its_walk_and_the_band_plays_it() {
        for style in &MICROMODE_STYLES {
            let mut project = Project::empty();
            project.apply_micromode_style(style, StyleParts::default());
            let band = project.band().expect("a band");
            assert_eq!(band.micromode, Some(style));
            assert_eq!(band.name(), style.name);
            // The key is the micromode, and the sheet its walk.
            assert_eq!(
                Micromode::of(project.scale.pitch_set()),
                Some(style.micromode()),
                "{}",
                style.name
            );
            assert!(project.sheet_holds_forms(), "{}", style.name);
            let markers = project.playlist().chord_markers().to_vec();
            let first = style.first_form(PitchClass::C, DEMO_SECTIONS[0].0);
            let walk = style.forms(first, style.length());
            assert_eq!(markers.len(), walk.len(), "{}", style.name);
            for (marker, form) in markers.iter().zip(&walk) {
                assert_eq!(marker.harmony.pitch_set(), Some(*form), "{}", style.name);
            }
            // Each voice plays only the notes of the form written there.
            for player in [Player::Piano, Player::Bass, Player::Soloist] {
                if voice_of(style, player).is_none() {
                    assert!(band.channel(player).is_none(), "{}", style.name);
                    continue;
                }
                for (index, marker) in markers.iter().enumerate() {
                    let end = markers.get(index + 1).map_or(style.length(), |m| m.start);
                    let notes = heard(&project, player, marker.start, end);
                    assert!(!notes.is_empty(), "{} {:?}", style.name, player);
                    let form = marker.harmony.pitch_set().unwrap();
                    assert!(notes.is_subset_of(form), "{} {:?}", style.name, player);
                }
            }
            // No one else plays the chords.
            for player in [Player::Guitar, Player::Strings, Player::Organ] {
                assert!(band.channel(player).is_none());
            }
            assert_eq!(band.channel(Player::Drums), None);
            assert_eq!(
                band.players().contains(&Player::Drums),
                style.rhythm().is_some(),
                "{}",
                style.name
            );
        }
    }

    #[test]
    fn forms_on_the_sheet_stay_and_the_style_plays_them() {
        // C7 as a set key, and a walk of its forms written by hand.
        let c7 = PitchSet::from_semitones(&[0, 4, 7, 10]);
        let mut project = Project::with_band();
        project.scale = Scale::set(PitchClass::C, c7);
        let meter = project.meter();
        let forms = [
            c7,
            PitchSet::from_semitones(&[2, 4, 7, 10]),
            PitchSet::from_semitones(&[1, 4, 7, 9]),
            c7,
        ];
        for (index, form) in forms.iter().enumerate() {
            let bar = meter.bar(index);
            project.set_chords(
                bar.start,
                bar.end(),
                &[ChordMarker::new(bar.start, HarmonyName::Set(*form))],
            );
        }
        let sheet = project.playlist().chord_markers().to_vec();
        assert!(project.sheet_holds_forms());
        let style = named("Viennese points");
        project.apply_micromode_style(style, StyleParts::default());
        // The sheet and the key are as they were.
        assert_eq!(project.playlist().chord_markers(), sheet.as_slice());
        assert_eq!(project.scale.pitch_set(), c7);
        // The points play the forms of the sheet, bar by bar.
        for (index, form) in forms.iter().enumerate() {
            let bar = meter.bar(index);
            for player in [Player::Soloist, Player::Piano] {
                let notes = heard(&project, player, bar.start, bar.end());
                assert!(!notes.is_empty());
                assert!(notes.is_subset_of(*form), "bar {index} {player:?}");
            }
        }
        // Changing a form on the sheet changes what the style plays.
        let bar = meter.bar(1);
        let other = PitchSet::from_semitones(&[3, 6, 9, 0]);
        project.set_chords(
            bar.start,
            bar.end(),
            &[ChordMarker::new(bar.start, HarmonyName::Set(other))],
        );
        project.follow_chord_sheet();
        let notes = heard(&project, Player::Piano, bar.start, bar.end());
        assert!(notes.is_subset_of(other), "{notes:?}");
        // The Progression box writes the style's own walk instead.
        let parts = StyleParts {
            progression: true,
            ..StyleParts::default()
        };
        project.apply_micromode_style(style, parts);
        assert_eq!(
            Micromode::of(project.scale.pitch_set()),
            Some(style.micromode())
        );
    }

    #[test]
    fn a_song_takes_a_micromode_style_from_its_sheet() {
        let mut project = crate::library::demo::song_demo(&crate::library::SONGS[0]);
        let length = project.song_length();
        let style = named("Fourths groove");
        project.apply_micromode_style(style, StyleParts::default());
        // A tonal sheet gives way to the style's walk, all through the
        // song, and the song's own chords go quiet.
        assert!(project.sheet_holds_forms());
        assert!(project.playlist().chord_markers().last().unwrap().start < length);
        assert!(!project.band().unwrap().muted().channels.is_empty());
        // Its drums come in, and play.
        assert!(project.band().unwrap().players().contains(&Player::Drums));
        // Another style takes the band back to chords and bass.
        project.set_band_style(BandStyle::named("Bossa nova"));
        let band = project.band().unwrap();
        assert_eq!(band.micromode, None);
        assert!(band.channel(Player::Piano).is_some());
    }

    #[test]
    fn a_micromode_band_is_saved_and_read_again() {
        let mut project = Project::empty();
        project.apply_micromode_style(named("Menor 3 colour"), StyleParts::default());
        let text = crate::file::to_text(&project).unwrap();
        let read = crate::file::from_text(&text).unwrap();
        assert_eq!(
            read.band().unwrap().micromode,
            Some(named("Menor 3 colour"))
        );
        for player in Player::ALL {
            assert_eq!(part(&read, player), part(&project, player), "{player:?}");
        }
        assert_eq!(crate::file::format_for(&project), 30);
    }
}
