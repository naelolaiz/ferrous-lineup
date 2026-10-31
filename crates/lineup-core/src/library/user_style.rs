//! Portable user styles: repeating parts written over C major, voiced to
//! each harmony of a song. Files contain their instruments and notes.

use super::{
    Style,
    apply::StyleParts,
    comping::{BassPart, Played},
};
use crate::i18n::{tr, tr_name, trf};
use crate::{
    Bar, Channel, ChordMarker, HarmonyName, KeyMap, MAX_CHANNELS, MAX_STEPS, Note, NoteEvent,
    NoteNaming, Player, Project, Scale, ScaleKind, Tempo, TimeSignature,
};
use serde::{Deserialize, Serialize};

pub const EXTENSION: &str = "lineup-style";

/// The name of a copy of the style called `name`, in the app's language:
/// "Bossa nova copy", or "Bossa nova (copia)" in Spanish. The copy is a
/// style of your own, so it keeps the name it was given.
pub fn copy_name(name: &str) -> String {
    trf("{} copy", &[&name])
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StyleLane {
    pub player: Player,
    pub channel: Channel,
    pub notes: Vec<NoteEvent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserStyle {
    pub format: u32,
    pub name: String,
    pub about: String,
    pub time: TimeSignature,
    pub tempo: Tempo,
    pub swing: u8,
    pub mode: ScaleKind,
    pub chart: String,
    pub bars: usize,
    pub lanes: Vec<StyleLane>,
}

impl UserStyle {
    pub fn blank() -> Self {
        let mut style = Self {
            format: 1,
            name: crate::i18n::tr("New style").into(),
            about: String::new(),
            time: TimeSignature::COMMON,
            tempo: Tempo::default(),
            swing: 0,
            mode: ScaleKind::Major,
            chart: "I | V | vi | IV".into(),
            bars: 1,
            lanes: Vec::new(),
        };
        for drum in [crate::Drum::Kick, crate::Drum::Snare, crate::Drum::HiHat] {
            style.lanes.push(StyleLane {
                player: Player::Drums,
                channel: Channel::drum(drum),
                notes: Vec::new(),
            });
        }
        style.lanes.push(StyleLane {
            player: Player::Piano,
            channel: Channel::synth(),
            notes: Vec::new(),
        });
        style
    }

    pub fn from_builtin(style: &'static Style) -> Self {
        let band = crate::BandStyle::of_style(style).expect("built-in style has a band");
        Self::from_band(band, Some(style))
    }

    /// A copy of what a band in `band` plays, comping and walking the
    /// bass as the library style `harmony` does, if given, with all its
    /// players: what Edit style… opens for a song whose band plays it.
    pub fn from_band(band: &'static crate::BandStyle, harmony: Option<&'static Style>) -> Self {
        let time = harmony.map_or(band.time, |style| style.time);
        let mut project = Project::with_band();
        project.apply_band_parts(
            band,
            harmony,
            StyleParts {
                progression: false,
                ..StyleParts::default()
            },
        );
        project.set_chords(
            0,
            time.steps_per_bar() * 2,
            &[ChordMarker::new(0, HarmonyName::parse("C").unwrap())],
        );
        project.end_song_at(1);
        let bars = 2;
        let length = bars * time.steps_per_bar();
        let mut lanes = Vec::new();
        for player in Player::ALL {
            for (channel, state) in project.channels().iter().enumerate() {
                let notes: Vec<_> = project
                    .patterns()
                    .iter()
                    .filter(|pattern| pattern.player == Some(player))
                    .flat_map(|pattern| pattern.notes(channel).iter().copied())
                    .filter(|note| note.start < length)
                    .collect();
                if !notes.is_empty() {
                    lanes.push(StyleLane {
                        player,
                        channel: state.clone(),
                        notes,
                    });
                }
            }
        }
        let (name, about) =
            harmony.map_or((band.name, band.about), |style| (style.name, style.about));
        Self {
            format: 1,
            name: copy_name(&tr_name(name)),
            about: tr(about).into(),
            time,
            tempo: Tempo::new(harmony.map_or(band.tempo, |style| style.tempo)),
            swing: project.swing,
            mode: harmony.map_or(ScaleKind::Major, |style| style.mode),
            // A band of its own plays the song's chords.
            chart: harmony.map_or("", |style| style.chart).into(),
            bars,
            lanes,
        }
    }

    pub fn length(&self) -> usize {
        self.bars * self.time.steps_per_bar()
    }

    pub fn from_rhythm(rhythm: &'static super::Rhythm) -> Self {
        let mut style = Self::blank();
        // A rhythm has no chords of its own to bring.
        style.chart.clear();
        style.name = copy_name(&tr_name(rhythm.name));
        style.about = tr(rhythm.about).into();
        style.time = rhythm.time;
        style.tempo = Tempo::new(rhythm.tempo);
        style.swing = rhythm.swing;
        style.bars = rhythm.bars();
        style.lanes = rhythm
            .drums()
            .map(|drum| StyleLane {
                player: Player::Drums,
                channel: Channel::drum(drum),
                notes: rhythm.notes(drum, style.time.steps_per_bar(), style.length()),
            })
            .collect();
        style
    }

    pub fn id(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hash = std::collections::hash_map::DefaultHasher::new();
        self.name.hash(&mut hash);
        self.time.hash(&mut hash);
        self.tempo.bpm().to_bits().hash(&mut hash);
        self.swing.hash(&mut hash);
        self.mode.hash(&mut hash);
        self.chart.hash(&mut hash);
        self.bars.hash(&mut hash);
        for lane in &self.lanes {
            lane.player.hash(&mut hash);
            lane.channel.name.hash(&mut hash);
            lane.channel.program.hash(&mut hash);
            lane.channel.volume.to_bits().hash(&mut hash);
            lane.channel.muted.hash(&mut hash);
            std::mem::discriminant(&lane.channel.kind).hash(&mut hash);
            match &lane.channel.kind {
                crate::ChannelKind::Synth => {}
                crate::ChannelKind::MidiOut { channel } => channel.hash(&mut hash),
                crate::ChannelKind::Sampler(source) => match source {
                    crate::SampleSource::Builtin(drum) => drum.hash(&mut hash),
                    crate::SampleSource::File(path) | crate::SampleSource::Take(path) => {
                        path.hash(&mut hash)
                    }
                },
            }
            lane.notes.hash(&mut hash);
        }
        hash.finish()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.format != 1 {
            return Err(tr("Unsupported style format").to_string());
        }
        if self.name.trim().is_empty() {
            return Err(tr("Give the style a name").to_string());
        }
        if !(1..=16).contains(&self.bars) || self.length() > MAX_STEPS {
            return Err(tr("The pattern must fit in 1–16 bars and 256 steps").to_string());
        }
        if self.lanes.len() > MAX_CHANNELS {
            return Err(tr("Too many instrument lanes").to_string());
        }
        if self.swing > crate::MAX_SWING {
            return Err(tr("Swing is outside its range").to_string());
        }
        let scale = Scale::new(crate::PitchClass::C, self.mode);
        for (index, chart) in self.chart.split('|').enumerate() {
            if index >= 256 {
                return Err(tr("The progression is too long").to_string());
            }
            crate::chord_sheet::read_bar(
                chart.trim(),
                &crate::Meter::new(self.time, []).bar(index),
                scale,
            )
            .map_err(|error| trf("Progression bar {}: {}", &[&(index + 1), &error]))?;
        }
        let mut players = std::collections::HashSet::new();
        for lane in &self.lanes {
            if lane.player != Player::Drums && !players.insert(lane.player) {
                return Err(trf(
                    "Assign only one lane to {}",
                    &[&tr(lane.player.name())],
                ));
            }
            if !(0.0..=1.0).contains(&lane.channel.volume)
                || lane.channel.program.is_some_and(|p| p > 127)
            {
                return Err(tr("An instrument setting is outside its range").to_string());
            }
            if lane.notes.iter().any(|note| {
                note.start >= self.length()
                    || note.length == 0
                    || note
                        .start
                        .checked_add(note.length)
                        .is_none_or(|end| end > self.length())
                    || note.velocity > 127
                    || usize::from(note.nudge) >= crate::TICKS_PER_STEP
                    || usize::from(note.trim) >= crate::TICKS_PER_STEP
                    || note.start_tick() >= note.end_tick()
            }) {
                return Err(tr("A note is outside the repeating pattern").to_string());
            }
        }
        Ok(())
    }

    pub fn to_text(&self) -> Result<String, String> {
        self.validate()?;
        // Names are written in letters, whichever way the app shows them.
        NoteNaming::Letters
            .with(|| ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default()))
            .map_err(|e| e.to_string())
    }
    pub fn from_text(text: &str) -> Result<Self, String> {
        let style: Self = NoteNaming::Letters
            .with(|| ron::from_str(text))
            .map_err(|e| e.to_string())?;
        style.validate()?;
        Ok(style)
    }

    pub fn pattern_project(&self) -> Project {
        let mut project = Project::empty();
        project.time_signature = self.time;
        project.tempo = self.tempo;
        project.swing = self.swing;
        for lane in &self.lanes {
            project.add_channel(lane.channel.clone());
        }
        let pattern = project.pattern_mut(0).unwrap();
        pattern.name = self.name.clone();
        pattern.set_length(self.length());
        for (channel, lane) in self.lanes.iter().enumerate() {
            pattern.set_notes(channel, lane.notes.clone());
        }
        project
    }

    pub fn players(&self) -> Vec<Player> {
        Player::ALL
            .into_iter()
            .filter(|player| self.lanes.iter().any(|lane| lane.player == *player))
            .collect()
    }

    /// Repeating bar positions follow meter changes without drifting. Held
    /// notes are split at harmony changes so they take the new chord.
    pub fn play_lane(
        &self,
        lane: &StyleLane,
        bars: &[Bar],
        harmonies: &[Played],
        keys: &KeyMap,
    ) -> Vec<NoteEvent> {
        let source_bar = self.time.steps_per_bar();
        let mut notes = Vec::new();
        for (bar_index, bar) in bars.iter().enumerate() {
            let from = bar.number % self.bars * source_bar;
            for original in lane
                .notes
                .iter()
                .filter(|note| (from..from + source_bar).contains(&note.start))
            {
                let offset = original.start - from;
                if offset >= bar.length {
                    continue;
                }
                let start = bar.start + offset;
                let last_step = offset + original.length - 1;
                let end = bars.get(bar_index + last_step / source_bar).map_or_else(
                    || bars.last().map_or(bar.end(), |last| last.end()),
                    |last| last.start + (last_step % source_bar + 1).min(last.length),
                );
                if lane.player == Player::Drums {
                    notes.push(NoteEvent {
                        start,
                        length: end - start,
                        ..*original
                    });
                } else {
                    for harmony in harmonies.iter().filter(|h| h.start < end && h.end > start) {
                        let key = keys.key_at(harmony.start);
                        let Some(pitch) = adapt_pitch(original.pitch, lane.player, harmony, key)
                        else {
                            continue;
                        };
                        let at = start.max(harmony.start);
                        notes.push(NoteEvent {
                            pitch,
                            start: at,
                            length: end.min(harmony.end) - at,
                            nudge: if at == start { original.nudge } else { 0 },
                            trim: if harmony.end >= end { original.trim } else { 0 },
                            ..*original
                        });
                    }
                }
            }
        }
        crate::pattern::tidy_timeline_notes(notes)
    }
}

/// The note a template note written over C major plays over `harmony`,
/// in `key`. C, E and G are the chord's root, third and fifth, and B is
/// its seventh, or sixth, if it has one. Over a chord of the key, B, D, F
/// and A take the note of the key as many steps above the root: an A over
/// Em in C major is C, not C#, and a B over G is F. Over a chord from
/// outside the key they keep their distance from the root, as in its own
/// major, but a minor chord takes the flat seventh. The other notes are
/// chromatic colours and keep their distance from the root.
fn adapt_pitch(source: Note, player: Player, harmony: &Played, key: Scale) -> Option<Note> {
    let (root, intervals) = match harmony.bass {
        BassPart::Chord { chord, note, .. } => {
            let root = if player == Player::Bass && source.midi().is_multiple_of(12) {
                note.pitch_class()
            } else {
                chord.root
            };
            (root, chord.quality.intervals())
        }
        BassPart::Pedal(note) => (note.pitch_class(), &[0, 4, 7][..]),
        BassPart::Rest => return None,
    };
    let semitone = i32::from(root.semitone());
    let shift = if semitone > 6 {
        semitone - 12
    } else {
        semitone
    };
    let has = |interval: u8| intervals.contains(&interval);
    let own_seventh = [10, 11, 9].into_iter().find(|&seventh| has(seventh));
    let pc = source.midi() % 12;
    let target = match (pc, own_seventh) {
        (0, _) => 0,
        (4, _) => intervals.get(1).copied().unwrap_or(4),
        (7, _) => intervals.get(2).copied().unwrap_or(7),
        (11, Some(seventh)) => seventh,
        (11, None) if has(3) => 10,
        _ => pc,
    };
    let note = Note::new(
        (i32::from(source.midi()) + shift + i32::from(target) - i32::from(pc)).clamp(0, 127) as u8,
    );
    let steps = match (pc, own_seventh) {
        (2, _) => 1,
        (5, _) => 3,
        (9, _) => 5,
        (11, None) => 6,
        _ => return Some(note),
    };
    let in_key = key.is_heptatonic()
        && intervals
            .iter()
            .all(|&interval| key.contains(root.transpose(i32::from(interval))));
    let Some(degree) = key.degree_of(root).filter(|_| in_key) else {
        return Some(note);
    };
    let wanted = key.pitch_classes().nth((degree + steps) % 7)?;
    let up = note.pitch_class().semitones_up_to(wanted);
    Some(Note::new(if up <= 6 {
        (note.midi() + up).min(127)
    } else {
        note.midi().saturating_sub(12 - up)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_notes_stay_in_the_key() {
        use crate::{Chord, ChordQuality::*, PitchClass::*};
        let c_major = Scale::new(C, ScaleKind::Major);
        // The note `template` (written over C) plays over `chord`, as a
        // pitch class.
        let over = |template: crate::PitchClass, chord: Chord| {
            let harmony = Played {
                start: 0,
                end: 16,
                voicing: Vec::new(),
                bass: BassPart::Chord {
                    chord,
                    note: Note::new(48 + chord.root.semitone()),
                    next: Note::new(48 + chord.root.semitone()),
                },
            };
            let source = Note::new(60 + template.semitone());
            adapt_pitch(source, Player::Piano, &harmony, c_major)
                .unwrap()
                .pitch_class()
        };
        let chord = Chord::new;
        // The seventh: the chord's own, else the key's.
        assert_eq!(over(B, chord(C, Major)), B);
        assert_eq!(over(B, chord(A, Minor)), G);
        assert_eq!(over(B, chord(G, Major)), F);
        assert_eq!(over(B, chord(F, Major)), E);
        assert_eq!(over(B, chord(A, Minor6)), FSharp);
        assert_eq!(over(B, chord(D, Dominant7)), C);
        // The sixth, ninth and fourth of the key over its chords.
        assert_eq!(over(A, chord(E, Minor)), C);
        assert_eq!(over(A, chord(A, Minor)), F);
        assert_eq!(over(A, chord(D, Minor)), B);
        assert_eq!(over(D, chord(E, Minor)), F);
        assert_eq!(over(D, chord(A, Minor)), B);
        assert_eq!(over(F, chord(F, Major)), B);
        assert_eq!(over(F, chord(G, Major)), C);
        // A chord from outside the key keeps its own notes: Fm's flat
        // seventh and Bb's major seventh, not B and B.
        assert_eq!(over(B, chord(F, Minor)), DSharp);
        assert_eq!(over(F, chord(F, Minor)), ASharp);
        assert_eq!(over(B, chord(ASharp, Major)), A);
        // Chord tones and chromatic colours as before.
        assert_eq!(over(E, chord(A, Minor)), C);
        assert_eq!(over(G, chord(B, Diminished)), F);
        assert_eq!(over(DSharp, chord(A, Minor)), C);
    }
    #[test]
    fn styles_with_the_same_name_keep_distinct_patterns_selected() {
        let style = UserStyle::from_builtin(&super::super::STYLES[0]);
        let mut edited = style.clone();
        edited.lanes[0].notes[0].velocity = edited.lanes[0].notes[0].velocity.saturating_sub(1);
        assert_ne!(style.id(), edited.id());
        let reopened = UserStyle::from_text(&edited.to_text().unwrap()).unwrap();
        assert_eq!(edited.id(), reopened.id());
    }
    #[test]
    fn every_builtin_can_be_copied_edited_and_saved() {
        for style in super::super::STYLES {
            let mut copy = UserStyle::from_builtin(style);
            copy.validate()
                .unwrap_or_else(|error| panic!("{}: {error}", style.name));
            assert!(!copy.lanes.is_empty());
            copy.name.push_str(" edited");
            assert_eq!(
                UserStyle::from_text(&copy.to_text().unwrap()).unwrap(),
                copy
            );
        }
    }
    #[test]
    fn every_band_style_can_be_copied_with_all_its_players() {
        for band in crate::band_styles() {
            let copy = UserStyle::from_band(band, None);
            copy.validate()
                .unwrap_or_else(|error| panic!("{}: {error}", band.name));
            let plays = |player| copy.lanes.iter().any(|lane| lane.player == player);
            assert!(plays(Player::Piano) && plays(Player::Bass), "{}", band.name);
            assert_eq!(plays(Player::Drums), band.rhythm.is_some(), "{}", band.name);
            assert!(
                copy.chart.is_empty(),
                "{} plays the song's chords",
                band.name
            );
        }
    }

    #[test]
    fn a_copy_is_named_and_described_in_the_apps_language() {
        let bossa = super::super::Style::named("Bossa nova").unwrap();
        let copy = UserStyle::from_builtin(bossa);
        assert_eq!(copy.name, "Bossa nova copy");
        assert_eq!(copy.about, bossa.about);
        crate::i18n::Language::Spanish.with(|| {
            let copy = UserStyle::from_builtin(bossa);
            assert_eq!(copy.name, "Bossa nova (copia)");
            assert_eq!(copy.about, tr(bossa.about));
            assert_ne!(copy.about, bossa.about);
            let rhythm = &super::super::RHYTHMS[0];
            let copy = UserStyle::from_rhythm(rhythm);
            assert_eq!(copy.name, format!("{} (copia)", tr_name(rhythm.name)));
            assert_eq!(copy.about, tr(rhythm.about));
        });
    }

    #[test]
    fn user_parts_follow_chords_and_survive_a_project_round_trip() {
        let mut style = UserStyle::blank();
        style.name = "My syncopation".into();
        style.lanes.retain(|lane| lane.player == Player::Piano);
        style.lanes[0].notes = vec![
            NoteEvent::new(Note::C4, 2, 8),
            NoteEvent::new(Note::new(64), 2, 8),
        ];
        let mut project = Project::with_band();
        project.set_chords(
            0,
            16,
            &[
                ChordMarker::new(0, HarmonyName::parse("Dm").unwrap()),
                ChordMarker::new(8, HarmonyName::parse("G7").unwrap()),
            ],
        );
        let parts = StyleParts {
            progression: false,
            instruments: false,
            time_signature: false,
            tempo: false,
            swing: false,
            ..StyleParts::default()
        };
        let sounds: Vec<_> = project
            .channels()
            .iter()
            .map(|channel| channel.program)
            .collect();
        project.apply_user_style(&style, parts).unwrap();
        assert_eq!(project.band().unwrap().name(), "My syncopation");
        assert_eq!(
            project
                .channels()
                .iter()
                .map(|channel| channel.program)
                .collect::<Vec<_>>(),
            sounds
        );
        let channel = project.player_channel(Player::Piano).unwrap();
        let pattern = project
            .patterns()
            .iter()
            .find(|p| p.player == Some(Player::Piano))
            .unwrap();
        let notes = pattern.notes(channel);
        assert!(notes.iter().any(|n| n.start == 2 && n.pitch.midi() == 65)); // D minor third
        assert!(
            notes
                .iter()
                .any(|n| n.start == 8 && n.pitch.pitch_class() == crate::PitchClass::B)
        ); // G7 third
        assert!(notes.iter().filter(|n| n.start == 2).all(|n| n.end() == 8));
        let text = crate::file::to_text(&project).unwrap();
        assert_eq!(crate::file::format_for(&project), 22);
        let mut reopened = crate::file::from_text(&text).unwrap();
        assert_eq!(reopened, project);
        reopened.set_chords(
            0,
            16,
            &[ChordMarker::new(0, HarmonyName::parse("F").unwrap())],
        );
        reopened.follow_chord_sheet();
        let pattern = reopened
            .patterns()
            .iter()
            .find(|p| p.player == Some(Player::Piano))
            .unwrap();
        assert!(
            pattern
                .notes(channel)
                .iter()
                .any(|n| n.pitch.pitch_class() == crate::PitchClass::A)
        );
    }
}
