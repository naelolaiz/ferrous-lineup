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

use super::apply::{
    self, BASS, KEYS, LEAD, RhythmChange, chords_channel, drums_in, find_synth,
    heard_chords_channel, is_ending, synth_named,
};
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

    /// How to write this voice when the row starts on `start`, inside a
    /// section that reads the whole piece as `section` (P for the piece
    /// as written, R for it backwards, and so on).
    pub fn writing(&self, start: PitchClass, section: RowForm) -> RowWriting {
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
        row: "Loom row",
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
        row: "Loom row",
        tempo: 84.0,
        time: THREE_FOUR,
        bars: 4,
        voices: &[
            voice(KEYS, RowForm::Prime)
                .chords(6, 12)
                .from(Register::Close, 48),
            voice(LEAD, RowForm::Inversion)
                .offset(3)
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
        row: "Loom row",
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
                Loom's.",
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

/// Writes the voices of `style` into pattern `pattern`, `length` steps
/// long, with the row starting on `start` and the piece read as
/// `section`. Chords go to the `selected` channel if it can play them
/// (see [`chords_channel`]). Returns `false` if the pattern does not
/// exist or the rack is full.
fn write_voices(
    project: &mut Project,
    pattern: usize,
    style: &RowStyle,
    start: PitchClass,
    section: RowForm,
    selected: Option<usize>,
) -> bool {
    let row = style.row();
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
        let notes = write_row(row, &voice.writing(start, section), length);
        writes.push((channel, notes));
    }
    let Some(target) = project.pattern_mut(pattern) else {
        return false;
    };
    for (channel, notes) in writes {
        target.set_notes(channel, notes);
    }
    true
}

/// Writes `style` into pattern `pattern`, with the row starting on
/// `start`: its voices, its length and its time signature, and the
/// project's key becomes the row. Lead, Keys and Bass channels the style
/// does not use go quiet in the pattern, so a style tried after another
/// is heard alone. Returns `false` if the pattern does not exist or the
/// rack is full.
pub fn write_row_style(
    project: &mut Project,
    pattern: usize,
    style: &RowStyle,
    start: PitchClass,
    selected: Option<usize>,
) -> bool {
    write_row_style_as(project, pattern, style, start, selected, RowForm::Prime)
}

/// [`write_row_style`], with the piece read as `section`.
fn write_row_style_as(
    project: &mut Project,
    pattern: usize,
    style: &RowStyle,
    start: PitchClass,
    selected: Option<usize>,
    section: RowForm,
) -> bool {
    let Some(target) = project.pattern_mut(pattern) else {
        return false;
    };
    target.set_length(style.length());
    project.time_signature = style.time;
    project.scale = Scale::row(style.row().starting_on(start));
    let unused: Vec<usize> = [KEYS, BASS, LEAD]
        .into_iter()
        .filter(|name| !style.voices.iter().any(|voice| voice.channel == *name))
        .filter_map(|name| find_synth(project, name))
        .collect();
    if let Some(target) = project.pattern_mut(pattern) {
        for channel in unused {
            target.set_notes(channel, []);
        }
    }
    write_voices(project, pattern, style, start, section, selected)
}

/// Plays the whole song the way `style` plays its row. In every pattern
/// with chords, a bass line or a melody, the style takes over the
/// harmony: its chords replace the song's (on the channel that plays
/// them), its bass line the song's, and what the style does not play
/// goes quiet, since tonal chords or a tonal bass under a row only clash.
/// The melody stays, unless the style has a melody of its own. The
/// patterns are read in turn as P, R, I and RI so the song does not
/// repeat itself, and the drums take the style's rhythm if it has one.
/// The time signature and the endings stay as they are. The key becomes
/// the row, starting on the song's key note.
pub fn use_row_style(project: &mut Project, style: &RowStyle) -> RhythmChange {
    const SECTIONS: [RowForm; 4] = [
        RowForm::Prime,
        RowForm::Retrograde,
        RowForm::Inversion,
        RowForm::RetrogradeInversion,
    ];
    let start = project.scale.root;
    let uses = |name: &str| style.voices.iter().any(|voice| voice.channel == name);
    let change = take_over_harmony(
        project,
        &uses,
        style.rhythm(),
        |project, pattern, n, keys| {
            write_voices(project, pattern, style, start, SECTIONS[n % 4], keys);
        },
    );
    if change.harmony > 0 {
        project.scale = Scale::row(style.row().starting_on(start));
    }
    change
}

/// What [`use_row_style`] does, for any style that writes the harmony
/// without chords: in every pattern with chords, a bass line or a
/// melody, the song's chords and bass go quiet unless the style `uses`
/// that channel, then `write` writes the style into the pattern, with
/// the number of patterns written so far (to vary them) and the channel
/// for its chords: the one that played the song's chords, or a Keys
/// channel. The drums take `rhythm`, if there is one, where there were
/// drums. Endings stay as they are.
pub(super) fn take_over_harmony(
    project: &mut Project,
    uses: &dyn Fn(&str) -> bool,
    rhythm: Option<&Rhythm>,
    mut write: impl FnMut(&mut Project, usize, usize, Option<usize>),
) -> RhythmChange {
    let mut change = RhythmChange::default();
    let mut written = 0;
    for pattern in 0..project.patterns().len() {
        if is_ending(project, pattern) {
            continue;
        }
        let length = project.pattern(pattern).map_or(0, |p| p.length());
        let plays = |project: &Project, channel: Option<usize>| {
            channel.is_some_and(|channel| {
                project
                    .pattern(pattern)
                    .is_some_and(|p| !p.notes(channel).is_empty())
            })
        };
        let chords = heard_chords_channel(project, pattern);
        let bass = find_synth(project, BASS);
        let melody = plays(project, find_synth(project, LEAD));
        let has_drums = !drums_in(project, pattern, 0, length).is_empty();
        if chords.is_some() || plays(project, bass) || melody {
            let before = project.pattern(pattern).cloned();
            let quiet: Vec<usize> = [(KEYS, chords), (BASS, bass)]
                .into_iter()
                .filter(|(name, _)| !uses(name))
                .filter_map(|(_, channel)| channel)
                .collect();
            if let Some(target) = project.pattern_mut(pattern) {
                for channel in quiet {
                    target.set_notes(channel, []);
                }
            }
            // Without chords to replace, the style's chords go to a
            // Keys channel, never over a melody on another synth.
            let keys = match chords {
                Some(channel) => Some(channel),
                None if uses(KEYS) => synth_named(project, KEYS),
                None => None,
            };
            write(project, pattern, written, keys);
            if project.pattern(pattern) != before.as_ref() {
                written += 1;
                change.harmony += 1;
            }
        }
        if let Some(rhythm) = rhythm
            && has_drums
            && apply::write_rhythm(project, pattern, rhythm)
        {
            change.drums += 1;
        }
    }
    change
}

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
    write_row_style_as(project, pattern, style, PitchClass::C, None, section);
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
        assert!(write_row_style(&mut project, 0, style, PitchClass::C, None));
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
                    write_row_style(&mut project, 0, style, start, None),
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

    #[test]
    fn a_song_takes_a_row_style() {
        let mut project = Project::demo();
        let style = RowStyle::named("Twelve-tone lo-fi").unwrap();
        let change = use_row_style(&mut project, style);
        assert!(change.harmony > 0 && change.drums > 0, "{change:?}");
        assert_eq!(project.scale.tone_row().unwrap().first(), PitchClass::C);
    }

    /// The style's chords replace the song's on the channel that plays
    /// them, rather than playing over them on a channel of their own.
    #[test]
    fn a_row_style_replaces_the_chords_of_a_song() {
        let original = Project::demo();
        let pattern = (0..original.patterns().len())
            .find(|&p| heard_chords_channel(&original, p).is_some())
            .unwrap();
        let chords = heard_chords_channel(&original, pattern).unwrap();

        let mut project = original.clone();
        use_row_style(&mut project, RowStyle::named("Triad chorale").unwrap());
        assert_eq!(find_synth(&project, KEYS), None);
        let notes = project.pattern(pattern).unwrap().notes(chords);
        assert!(!notes.is_empty());
        assert_ne!(notes, original.pattern(pattern).unwrap().notes(chords));

        // A style that is only a melody leaves no tonal chords under it.
        let mut project = original.clone();
        let change = use_row_style(&mut project, RowStyle::named("Row melody").unwrap());
        assert!(change.harmony > 0);
        let target = project.pattern(pattern).unwrap();
        assert!(target.notes(chords).is_empty());
        let lead = find_synth(&project, LEAD).unwrap();
        assert!(!target.notes(lead).is_empty());
    }
}
