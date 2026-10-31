//! Every name and description the core gives the app has its Spanish:
//! instruments, styles, rhythms, demos and the names of what they make,
//! chords, scales and the words of each list the app shows.

use std::collections::BTreeSet;

use super::{Language, note_missed, spanish_name, take_missed, tr, trc};
use crate::PitchClass;
use crate::band::horns::Horns;
use crate::band::{Player, band_styles};
use crate::chord_sheet::notation::{Grouping, Notation};
use crate::chord_sheet::{Push, Stop};
use crate::ear_training::{
    ChordPool, ChordQuestion, ChordSettings, Direction, Exercise, Interval, KeyChoice, Players,
    chord_sound,
};
use crate::effects::Delay;
use crate::general_midi;
use crate::harmony::chord_scale::NAMES as CHORD_SCALES;
use crate::harmony::keys::{ChordSize, Function};
use crate::harmony::voice_leading::{SlipKind, why};
use crate::harmony::voice_motion::Motion;
use crate::harmony::{
    Chord, ChordQuality, ChordStamp, HintLevel, Micromode, MicromodeFamily, ROW_PRESETS, RowForm,
    Scale, ScaleKind, SetClass, next_chords, next_chords_over,
};
use crate::lead_sheet::Paper;
use crate::library::demo::{micromode_demo, row_demo, song_demo, style_demo};
use crate::library::{FILLS, Genre, MICROMODE_STYLES, RHYTHMS, ROW_STYLES, SONGS, STYLES};
use crate::project::{Drum, Project};
use crate::score_order::Section;
use crate::serial::{Chain, Register};
use crate::sources::SongPart;
use crate::tools::ArpeggioOrder;

/// Every kind of slip in the voice leading.
const SLIPS: [SlipKind; 12] = [
    SlipKind::ParallelFifths,
    SlipKind::ParallelOctaves,
    SlipKind::ParallelUnisons,
    SlipKind::ContraryFifths,
    SlipKind::ContraryOctaves,
    SlipKind::LeadingTone,
    SlipKind::Seventh,
    SlipKind::RowOrder,
    SlipKind::EarlyRepeat,
    SlipKind::RowUnfinished,
    SlipKind::OctaveDoubling,
    SlipKind::TonalChord,
];

/// The texts the core shows through `tr`, for the lists and tables the
/// app shows them from.
fn texts() -> Vec<&'static str> {
    let mut texts: Vec<&'static str> = Vec::new();
    texts.extend(general_midi::NAMES);
    texts.extend(general_midi::FAMILIES);
    texts.extend((0..=127).filter_map(general_midi::drum_name));
    texts.extend(Drum::ALL.map(Drum::name));
    texts.extend(Player::ALL.map(Player::name));
    texts.extend(Genre::ALL.map(Genre::name));
    texts.extend(ScaleKind::ALL.map(ScaleKind::name));
    texts.extend(ChordQuality::ALL.map(ChordQuality::name));
    texts.extend(HintLevel::ALL.map(HintLevel::label));
    texts.extend(HintLevel::ALL.map(HintLevel::description));
    texts.extend(ChordSize::ALL.map(ChordSize::label));
    for function in [Function::Tonic, Function::Subdominant, Function::Dominant] {
        texts.extend([function.name(), function.feel(), function.short_feel()]);
    }
    for family in MicromodeFamily::ALL {
        texts.extend([family.name(), family.rule()]);
    }
    texts.extend(Micromode::all().iter().map(|micromode| micromode.about));
    for preset in ROW_PRESETS {
        texts.extend([preset.source, preset.about]);
    }
    texts.extend(RowForm::ALL.map(RowForm::name));
    texts.extend(crate::harmony::degree::Triadic::ALL.map(crate::harmony::degree::Triadic::name));
    for kind in SLIPS {
        texts.extend([kind.name(), kind.short(), why(kind)]);
    }
    for (_, name) in CHORD_SCALES {
        texts.push(name);
    }
    for motion in [
        Motion::Static,
        Motion::Oblique,
        Motion::Contrary,
        Motion::Similar,
        Motion::Parallel,
    ] {
        texts.push(motion.name());
    }
    for notation in Notation::ALL {
        texts.extend([notation.name(), notation.describe()]);
    }
    for grouping in Grouping::ALL {
        texts.extend([grouping.name(), grouping.describe()]);
    }
    texts.extend(Stop::ALL.map(Stop::name));
    texts.extend(SongPart::ALL.map(SongPart::name));
    texts.extend(crate::SECTION_NAMES);
    texts.extend(Register::ALL.map(Register::about));
    texts.extend(Chain::ALL.map(Chain::name));
    texts.extend(ArpeggioOrder::ALL.map(ArpeggioOrder::name));
    texts.extend(Section::ALL.map(Section::name));
    texts.extend(Paper::ALL.map(Paper::name));
    texts.extend(Delay::TIMES.map(|(_, name)| name));
    texts.extend(Exercise::ALL.map(Exercise::name));
    texts.extend(Direction::ALL.map(Direction::name));
    texts.extend(KeyChoice::ALL.map(KeyChoice::name));
    texts.extend(ChordPool::ALL.map(ChordPool::name));
    texts.extend(Players::ALL.map(Players::name));
    for interval in Interval::ALL {
        texts.extend([interval.short(), interval.sound()]);
        texts.extend(interval.tune());
    }
    texts.extend(
        ChordQuality::ALL
            .map(chord_sound)
            .into_iter()
            .filter(|sound| !sound.is_empty()),
    );
    for style in STYLES {
        texts.push(style.about);
    }
    for rhythm in RHYTHMS.iter().chain(FILLS) {
        texts.push(rhythm.about);
    }
    for style in band_styles() {
        texts.push(style.about);
    }
    for style in &ROW_STYLES {
        texts.push(style.about);
    }
    for style in &MICROMODE_STYLES {
        texts.push(style.about);
    }
    for song in SONGS {
        texts.extend([song.origin, song.about]);
        if !song.feel.is_empty() {
            texts.push(song.feel);
        }
    }
    texts
}

/// The names the core gives things, which the app shows through
/// `tr_name`: of styles, demos, and the patterns, channels, tracks and
/// sections a demo makes.
fn names() -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    let add_project = |names: &mut BTreeSet<String>, project: &Project| {
        names.extend(project.patterns().iter().map(|p| p.name.clone()));
        names.extend(project.channels().iter().map(|c| c.name.clone()));
        names.extend(project.playlist().tracks().iter().map(|t| t.name.clone()));
        names.extend(project.playlist().sections().iter().map(|s| s.name.clone()));
    };
    for style in STYLES {
        names.insert(style.name.to_string());
        add_project(&mut names, &style_demo(style));
    }
    for rhythm in RHYTHMS.iter().chain(FILLS) {
        names.insert(rhythm.name.to_string());
    }
    for style in band_styles() {
        names.insert(style.name.to_string());
    }
    for style in &ROW_STYLES {
        names.insert(style.name.to_string());
        add_project(&mut names, &row_demo(style));
    }
    for style in &MICROMODE_STYLES {
        names.insert(style.name.to_string());
        add_project(&mut names, &micromode_demo(style));
    }
    for preset in ROW_PRESETS {
        names.insert(preset.name.to_string());
    }
    for song in SONGS {
        names.insert(song.name.to_string());
        names.extend(song.parts.iter().map(|part| part.name.to_string()));
        add_project(&mut names, &song_demo(song));
    }
    add_project(&mut names, &Project::demo());
    add_project(&mut names, &Project::with_band());
    // What the app names the patterns, tracks and channels it adds.
    names.extend(["Pattern 2", "Track 3", "MIDI Out 1", "Channel 4"].map(String::from));
    names
}

/// What the core writes in Spanish by calling `tr` itself, for the
/// tables whose words it puts together.
fn written() {
    for style in band_styles() {
        style.extras().describe();
        Horns::of(style).describe();
    }
    for class in SetClass::all() {
        class.describe();
    }
    for stamp in ChordStamp::all() {
        stamp.label();
    }
    for stop in Stop::ALL {
        stop.describe();
    }
    for register in Register::ALL {
        trc("register", register.name());
    }
    for push in [Push::Eighth, Push::Sixteenth] {
        push.describe();
    }
    for micromode in Micromode::all() {
        micromode.intervals();
    }
    for kind in ScaleKind::ALL {
        let scale = Scale::new(PitchClass::A, kind);
        for degree in 0..7 {
            if let Some(triad) = scale.triad(degree) {
                next_chords(scale, triad);
                next_chords_over(scale, triad, triad.root);
            }
        }
        // A chord from outside the key: a secondary dominant.
        next_chords(scale, Chord::new(PitchClass::B, ChordQuality::Major));
    }
    let tonic = Scale::new(PitchClass::C, ScaleKind::Major);
    next_chords_over(
        tonic,
        Chord::new(PitchClass::C, ChordQuality::Major),
        PitchClass::G,
    );
    for seed in 0..400 {
        let settings = ChordSettings {
            inversions: true,
            ..ChordSettings::default()
        };
        ChordQuestion::new(&settings, seed).explain();
    }
    for function in [Function::Tonic, Function::Subdominant, Function::Dominant] {
        super::tr_string(format!("The {}", function.name()));
    }
}

#[test]
fn every_name_and_description_the_app_shows_has_spanish() {
    let missing: BTreeSet<String> = Language::Spanish.with(|| {
        note_missed();
        for text in texts() {
            tr(text);
        }
        written();
        let mut missing = take_missed();
        missing.extend(
            names()
                .into_iter()
                .filter(|name| spanish_name(name).is_none()),
        );
        missing
    });
    assert!(
        missing.is_empty(),
        "{} texts have no Spanish:\n{}",
        missing.len(),
        missing.into_iter().collect::<Vec<_>>().join("\n")
    );
}
