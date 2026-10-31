//! Twelve-tone styles: ways of playing a row, to try from the library.
//!
//! A harmony style is a chord progression and a way of playing it. A
//! twelve-tone piece has no progression: everything comes from the row.
//! So a row style says which row to use, and how each instrument plays
//! it: which form, as a melody or as chords, how fast, in which octaves,
//! and whether durations and dynamics follow the row too (see
//! [`crate::serial`]). The rows are the ones in
//! [`ROW_PRESETS`](crate::harmony::ROW_PRESETS) written for Ferrous Lineup,
//! and the row of Webern's Concerto Op. 24 (the row only: the rhythm and
//! the instruments are Ferrous Lineup's).

use crate::harmony::{RowForm, RowLabel, RowPreset, Scale, ToneRow};
use crate::meter::TimeSignature;
use crate::pitch::{Note, PitchClass};
use crate::project::Project;
use crate::serial::{Chain, Register, RowWriting, write_row};

use super::apply::{BASS, KEYS, LEAD, chords_channel, find_synth, synth_named};
use super::rhythm::Rhythm;

/// How one instrument plays the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RowVoice {
    /// The channel: [`KEYS`], [`BASS`] or [`LEAD`].
    pub channel: &'static str,
    /// The form it starts with.
    pub form: RowForm,
    /// How many semitones above the row's first note the form is
    /// numbered: 0 for the row's own `P` and `I`, 3 for `I3` of a row on C.
    pub offset: u8,
    pub chain: Chain,
    /// How many notes sound together (see [`RowWriting::together`]).
    pub together: usize,
    pub step: usize,
    pub register: Register,
    /// The lowest note, as a MIDI note number.
    pub low: u8,
    pub durations: bool,
    pub dynamics: bool,
    pub octaves: bool,
    /// Whether it plays the combinatorial partner of the form the
    /// section is read as (see [`ToneRow::partner`]), when the row has
    /// one, in place of its own form: the other six notes of each half.
    pub partner: bool,
}

/// A voice playing `form` as a melody in eighth notes from middle C.
const fn voice(channel: &'static str, form: RowForm) -> RowVoice {
    RowVoice {
        channel,
        form,
        offset: 0,
        chain: Chain::Repeat,
        together: 1,
        step: 2,
        register: Register::Nearest,
        low: 60,
        durations: false,
        dynamics: false,
        octaves: false,
        partner: false,
    }
}

impl RowVoice {
    const fn offset(self, offset: u8) -> Self {
        RowVoice { offset, ..self }
    }
    const fn chain(self, chain: Chain) -> Self {
        RowVoice { chain, ..self }
    }
    const fn chords(self, together: usize, step: usize) -> Self {
        RowVoice {
            together,
            step,
            ..self
        }
    }
    const fn step(self, step: usize) -> Self {
        RowVoice { step, ..self }
    }
    const fn from(self, register: Register, low: u8) -> Self {
        RowVoice {
            register,
            low,
            ..self
        }
    }
    const fn serial(self, durations: bool, dynamics: bool, octaves: bool) -> Self {
        RowVoice {
            durations,
            dynamics,
            octaves,
            ..self
        }
    }
    const fn partner(self) -> Self {
        RowVoice {
            partner: true,
            ..self
        }
    }

    /// How to write this voice of a piece on `row` in a section read as
    /// the form `section` (P0 for the piece as written on a row starting
    /// on C, R0 for it backwards, and so on): the voice's form is read
    /// the same way, so a voice playing I plays P in a section read as
    /// I. A voice that plays the partner plays that of `section`, if the
    /// row has one.
    pub fn writing(&self, row: ToneRow, section: RowLabel) -> RowWriting {
        if self.partner
            && let Some(partner) = row.partner(section)
        {
            return RowWriting {
                label: partner,
                ..self.writing_from(PitchClass::from_semitone(section.number), section.form)
            };
        }
        self.writing_from(PitchClass::from_semitone(section.number), section.form)
    }

    /// How to write this voice when the row starts on `start`, inside a
    /// section that reads the whole piece as `section`.
    fn writing_from(&self, start: PitchClass, section: RowForm) -> RowWriting {
        let form = combine(section, self.form);
        let inverted = matches!(section, RowForm::Inversion | RowForm::RetrogradeInversion);
        // Inverting around the row's first note turns "3 above it" into
        // "3 below it".
        let offset = if inverted {
            (12 - self.offset % 12) % 12
        } else {
            self.offset % 12
        };
        RowWriting {
            label: RowLabel::new(form, start.semitone() + offset),
            chain: self.chain,
            together: self.together,
            step: self.step,
            register: self.register,
            low: Note::new(self.low),
            durations: self.durations,
            dynamics: self.dynamics,
            octaves: self.octaves,
            partner: false,
        }
    }
}

/// The form `voice` becomes in a section read as `section`: inverting an
/// inversion gives the prime form back, and reading a retrograde
/// backwards gives it forwards.
pub fn combine(section: RowForm, voice: RowForm) -> RowForm {
    let flags = |form| match form {
        RowForm::Prime => (false, false),
        RowForm::Inversion => (true, false),
        RowForm::Retrograde => (false, true),
        RowForm::RetrogradeInversion => (true, true),
    };
    let (a, b) = (flags(section), flags(voice));
    match (a.0 != b.0, a.1 != b.1) {
        (false, false) => RowForm::Prime,
        (true, false) => RowForm::Inversion,
        (false, true) => RowForm::Retrograde,
        (true, true) => RowForm::RetrogradeInversion,
    }
}

/// A way of playing a row.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RowStyle {
    pub name: &'static str,
    pub about: &'static str,
    /// The name of a row in [`ROW_PRESETS`](crate::harmony::ROW_PRESETS).
    pub row: &'static str,
    pub tempo: f32,
    pub time: TimeSignature,
    pub bars: usize,
    pub voices: &'static [RowVoice],
    /// A rhythm from the library for the drums, or "" for none.
    pub rhythm: &'static str,
}

impl RowStyle {
    pub fn named(name: &str) -> Option<&'static RowStyle> {
        ROW_STYLES.iter().find(|style| style.name == name)
    }

    /// The row, starting on C.
    pub fn row(&self) -> ToneRow {
        RowPreset::named(self.row)
            .map_or(ToneRow::CHROMATIC, |preset| preset.row)
            .starting_on(PitchClass::C)
    }

    /// How long one pass is, in steps.
    pub fn length(&self) -> usize {
        self.bars * self.time.steps_per_bar()
    }

    pub fn rhythm(&self) -> Option<&'static Rhythm> {
        Rhythm::named(self.rhythm)
    }
}

const THREE_FOUR: TimeSignature = TimeSignature::new(3, 4);

/// The twelve-tone styles of the library.
pub static ROW_STYLES: [RowStyle; 7] = [
    RowStyle {
        name: "Row melody",
        about: "The row as a tune in eighth notes, then backwards: the simplest way to hear it.",
        row: "Lineup row",
        tempo: 96.0,
        time: TimeSignature::COMMON,
        bars: 3,
        voices: &[voice(LEAD, RowForm::Prime).chain(Chain::FourForms)],
        rhythm: "",
    },
    RowStyle {
        name: "Triad chorale",
        about: "A row made of four triads, played as chords in all four forms, over its \
                retrograde in the bass. Twelve-tone music that sounds almost tonal.",
        row: "Four triads",
        tempo: 72.0,
        time: TimeSignature::COMMON,
        bars: 4,
        voices: &[
            voice(KEYS, RowForm::Prime)
                .chain(Chain::FourForms)
                .chords(3, 4)
                .from(Register::Nearest, 55),
            voice(BASS, RowForm::Retrograde)
                .step(8)
                .from(Register::Close, 36),
        ],
        rhythm: "",
    },
    RowStyle {
        name: "Hexachord pads",
        about: "Each bar holds half the row as a chord while the melody plays the other six \
                notes from I3: every bar has all twelve notes. This is hexachordal \
                combinatoriality.",
        row: "Lineup row",
        tempo: 84.0,
        time: THREE_FOUR,
        bars: 4,
        voices: &[
            voice(KEYS, RowForm::Prime)
                .chords(6, 12)
                .from(Register::Close, 48),
            voice(LEAD, RowForm::Inversion)
                .offset(3)
                .partner()
                .from(Register::Nearest, 67),
        ],
        rhythm: "",
    },
    RowStyle {
        name: "Pointillism",
        about: "Single notes leaping across three octaves, each with its own length and \
                loudness from the row: points of sound, as in Webern.",
        row: "Mirror row",
        tempo: 72.0,
        time: TimeSignature::COMMON,
        bars: 4,
        voices: &[voice(KEYS, RowForm::Prime)
            .chain(Chain::FourForms)
            .from(Register::Wide, 48)
            .serial(true, true, false)],
        rhythm: "",
    },
    RowStyle {
        name: "Total serialism",
        about: "Pitches, durations, dynamics and octaves all follow the row, as in the music \
                of Boulez and Babbitt around 1950. Under it, the retrograde inversion in \
                two-note chords.",
        row: "All the intervals",
        tempo: 90.0,
        time: TimeSignature::COMMON,
        bars: 4,
        voices: &[
            voice(LEAD, RowForm::Prime)
                .chain(Chain::FourForms)
                .from(Register::Nearest, 55)
                .serial(true, true, true),
            voice(KEYS, RowForm::RetrogradeInversion)
                .chords(2, 4)
                .from(Register::Close, 48)
                .serial(true, true, false),
        ],
        rhythm: "",
    },
    RowStyle {
        name: "Twelve-tone lo-fi",
        about: "The row's trichords as chords over a lo-fi beat, its prime form in the bass: \
                a row can groove too.",
        row: "Lineup row",
        tempo: 80.0,
        time: TimeSignature::COMMON,
        bars: 4,
        voices: &[
            voice(KEYS, RowForm::Prime)
                .chain(Chain::FourForms)
                .chords(3, 4)
                .from(Register::Nearest, 55),
            voice(BASS, RowForm::Prime)
                .step(4)
                .from(Register::Close, 36),
        ],
        rhythm: "Lo-fi",
    },
    RowStyle {
        name: "Webern's trichords",
        about: "The row of Webern's Concerto Op. 24 (1934), whose four groups of three notes \
                are all one micromode, Menor 3. The melody leaps over three octaves, as \
                Webern's lines do, while each group sounds as a chord under it. Webern gave \
                the groups to oboe, flute, trumpet and clarinet; this rhythm is Ferrous \
                Lineup's.",
        row: "Webern's Concerto",
        tempo: 80.0,
        time: TimeSignature::COMMON,
        bars: 3,
        voices: &[
            voice(LEAD, RowForm::Prime)
                .chain(Chain::FourForms)
                .from(Register::Wide, 55),
            voice(KEYS, RowForm::Prime)
                .chain(Chain::FourForms)
                .chords(3, 6)
                .from(Register::Nearest, 48),
        ],
        rhythm: "",
    },
];

/// Writes the voices of `style` into pattern `pattern`, as long as the
/// pattern, on `row` with the piece read as the form `section`. Chords
/// go to the `selected` channel if it can play them (see
/// [`chords_channel`]). Returns `false` if the pattern does not exist or
/// the rack is full.
fn write_voices(
    project: &mut Project,
    pattern: usize,
    style: &RowStyle,
    row: ToneRow,
    section: RowLabel,
    selected: Option<usize>,
) -> bool {
    let Some(length) = project.pattern(pattern).map(|p| p.length()) else {
        return false;
    };
    let mut writes = Vec::new();
    for voice in style.voices {
        let channel = match voice.channel {
            KEYS => chords_channel(project, selected),
            name => synth_named(project, name),
        };
        let Some(channel) = channel else {
            return false;
        };
        let notes = write_row(row, &voice.writing(row, section), length);
        writes.push((channel, notes));
    }
    let Some(mut target) = project.style_pattern(pattern) else {
        return false;
    };
    for (channel, notes) in writes {
        target.set_notes(channel, notes);
    }
    true
}

impl RowStyle {
    /// The row it plays in a song in `key`: the key's own row, when the
    /// key is a row and `own` is false, or else its own row, starting on
    /// the key note.
    pub fn row_in(&self, key: Scale, own: bool) -> ToneRow {
        match key.tone_row() {
            Some(row) if !own => row,
            _ => self.row().starting_on(key.root),
        }
    }
}

/// Writes `style` into pattern `pattern`, on `row`: its voices, its
/// length and its time signature, and the project's key becomes the
/// row. Lead, Keys and Bass channels the style does not use go quiet in
/// the pattern, so a style tried after another is heard alone. Returns
/// `false` if the pattern does not exist or the rack is full.
pub fn write_row_style(
    project: &mut Project,
    pattern: usize,
    style: &RowStyle,
    row: ToneRow,
    selected: Option<usize>,
) -> bool {
    write_row_style_as(project, pattern, style, row.label(), selected, row)
}

/// [`write_row_style`], with the piece read as the form `section` of
/// `row`.
fn write_row_style_as(
    project: &mut Project,
    pattern: usize,
    style: &RowStyle,
    section: RowLabel,
    selected: Option<usize>,
    row: ToneRow,
) -> bool {
    let Some(mut target) = project.style_pattern(pattern) else {
        return false;
    };
    target.set_length(style.length());
    project.time_signature = style.time;
    project.scale = Scale::row(row);
    let unused: Vec<usize> = [KEYS, BASS, LEAD]
        .into_iter()
        .filter(|name| !style.voices.iter().any(|voice| voice.channel == *name))
        .filter_map(|name| find_synth(project, name))
        .collect();
    if let Some(mut target) = project.style_pattern(pattern) {
        for channel in unused {
            target.set_notes(channel, []);
        }
    }
    write_voices(project, pattern, style, row, section, selected)
}

/// The forms a row piece is read as in turn, part after part: as
/// written, backwards, upside down, and both.
pub const SECTIONS: [RowForm; 4] = [
    RowForm::Prime,
    RowForm::Retrograde,
    RowForm::Inversion,
    RowForm::RetrogradeInversion,
];

/// The four sections of a row demo, and the names they are given.
pub const DEMO_SECTIONS: [(RowForm, &str); 4] = [
    (RowForm::Prime, "P: the row"),
    (RowForm::Retrograde, "R: backwards"),
    (RowForm::Inversion, "I: upside down"),
    (RowForm::RetrogradeInversion, "RI: both"),
];

/// Writes `style` into pattern `pattern`, read as `section`, with the
/// row on C.
pub(super) fn write_section(
    project: &mut Project,
    pattern: usize,
    style: &RowStyle,
    section: RowForm,
) {
    let row = style.row();
    let label = RowLabel::new(section, row.first().semitone());
    write_row_style_as(project, pattern, style, label, None, row);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harmony::ROW_PRESETS;

    #[test]
    fn every_style_uses_a_preset_row() {
        for style in &ROW_STYLES {
            assert!(RowPreset::named(style.row).is_some(), "{}", style.name);
            assert!(
                style.rhythm.is_empty() || style.rhythm().is_some(),
                "{}",
                style.name
            );
            assert!(!style.voices.is_empty());
        }
        assert_eq!(ROW_PRESETS.len(), 8);
    }

    #[test]
    fn combining_forms() {
        use RowForm::*;
        assert_eq!(combine(Prime, Inversion), Inversion);
        assert_eq!(combine(Inversion, Inversion), Prime);
        assert_eq!(combine(Retrograde, Inversion), RetrogradeInversion);
        assert_eq!(combine(RetrogradeInversion, RetrogradeInversion), Prime);
    }

    #[test]
    fn hexachord_pads_fill_every_bar_with_all_twelve() {
        let style = RowStyle::named("Hexachord pads").unwrap();
        let mut project = Project::empty();
        assert!(write_row_style(&mut project, 0, style, style.row(), None));
        assert_eq!(project.time_signature, THREE_FOUR);
        assert!(!project.scale.kind.is_tonal());
        let pattern = project.pattern(0).unwrap();
        assert_eq!(pattern.length(), 48);
        let bar = 12;
        for start in (0..48).step_by(bar) {
            let heard: crate::PitchSet = pattern
                .all_notes()
                .filter(|(_, note)| (start..start + bar).contains(&note.start))
                .map(|(_, note)| note.pitch.pitch_class())
                .collect();
            assert_eq!(heard.len(), 12, "bar from step {start}");
        }
    }

    #[test]
    fn every_style_writes_its_voices_in_any_key() {
        for style in &ROW_STYLES {
            for start in [PitchClass::C, PitchClass::FSharp] {
                let mut project = Project::empty();
                assert!(
                    write_row_style(&mut project, 0, style, style.row().starting_on(start), None),
                    "{}",
                    style.name
                );
                assert_eq!(project.scale.tone_row().unwrap().first(), start);
                let pattern = project.pattern(0).unwrap();
                for voice in style.voices {
                    let channel = find_synth(&project, voice.channel).unwrap();
                    let notes = pattern.notes(channel);
                    assert!(!notes.is_empty(), "{} {}", style.name, voice.channel);
                    assert!(notes.iter().all(|n| n.end() <= pattern.length()));
                }
            }
        }
    }

    /// Tried in a pattern while the key is a row, a style plays that row;
    /// its own row only when asked for, starting on the key note.
    #[test]
    fn a_style_tried_in_a_row_key_plays_that_row() {
        let mine = RowPreset::named("Berg's Lyric Suite").unwrap().row;
        let key = Scale::row(mine.starting_on(PitchClass::A));
        for style in &ROW_STYLES {
            assert_eq!(style.row_in(key, false), mine.starting_on(PitchClass::A));
            assert_eq!(
                style.row_in(key, true),
                style.row().starting_on(PitchClass::A)
            );
            assert_eq!(style.row_in(Scale::default(), false), style.row());
            let mut project = Project::empty();
            project.scale = key;
            let row = style.row_in(project.scale, false);
            assert!(write_row_style(&mut project, 0, style, row, None));
            assert_eq!(project.scale, key, "{}", style.name);
            let channels: Vec<usize> = (0..project.channels().len()).collect();
            let check = project.voice_leading(0, &channels);
            assert!(!check.rows.is_empty(), "{}", style.name);
        }
    }

    /// The voice-leading check reads every style as forms of its row,
    /// with no note out of order: the styles and the check follow the
    /// same rules.
    #[test]
    fn the_voice_leading_check_reads_every_style_as_forms_of_its_row() {
        use crate::harmony::voice_leading::SlipKind;
        for style in &ROW_STYLES {
            for start in [PitchClass::C, PitchClass::FSharp] {
                let mut project = Project::empty();
                assert!(write_row_style(
                    &mut project,
                    0,
                    style,
                    style.row().starting_on(start),
                    None
                ));
                let channels: Vec<usize> = (0..project.channels().len()).collect();
                let check = project.voice_leading(0, &channels);
                assert!(!check.rows.is_empty(), "{}", style.name);
                let out_of_order = check.slips.iter().filter(|slip| {
                    matches!(
                        slip.kind,
                        SlipKind::RowOrder | SlipKind::EarlyRepeat | SlipKind::RowUnfinished
                    )
                });
                assert_eq!(
                    out_of_order.count(),
                    0,
                    "{}: {}",
                    style.name,
                    check.summary()
                );
            }
        }
    }
}
