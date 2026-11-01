//! How well [`lineup_listen::chords`] hears real chords, measured on
//! GuitarSet (360 recordings of a guitar played by six players, with the
//! chords each was asked to play; CC BY 4.0,
//! https://guitarset.weebly.com). Its 180 accompaniments, the
//! recordings named `..._comp`, are the ones played in chords. Nothing
//! is learned from it: it only measures.
//!
//! The dataset is not in the repository. Download `annotation.zip` and
//! `audio_mono-mic.zip` from https://zenodo.org/records/3371780, unzip
//! them into `annotation` and `audio` in one folder, then:
//!
//! ```text
//! GUITARSET=/path/to/guitarset cargo test --release \
//!     -p lineup-listen --test heard_chords -- --ignored --nocapture
//! ```
//!
//! It prints, by style and in all, the share of the time where the chord
//! heard has the right root, and where it is also rightly major or
//! minor, as MIREX measures chord recognition ("majmin": a seventh
//! counts as its triad, and the time of the half-diminished chords, which
//! are neither, is left out). Then the share where the chord is exactly
//! the one written, among the four kinds the dataset writes: major,
//! minor, seventh and half-diminished seventh.

use std::path::{Path, PathBuf};

use lineup_core::PitchClass;
use lineup_core::harmony::ChordQuality;
use lineup_listen::chords::{self, HeardChord};
use lineup_listen::decode_file;

/// How often the chords written and heard are compared, in seconds.
const STEP: f64 = 0.01;

/// A written chord: from when to when, its root and its kind, as the
/// dataset writes it ("maj", "min", "7", "hdim7").
struct Written {
    start: f64,
    end: f64,
    root: PitchClass,
    kind: String,
}

/// The value of `"key": ` in `object`, as written.
fn field<'a>(object: &'a str, key: &str) -> Option<&'a str> {
    let at = object.find(&format!("\"{key}\""))? + key.len() + 2;
    let rest = object[at..].trim_start().strip_prefix(':')?.trim_start();
    let end = rest.find([',', '}', '\n']).unwrap_or(rest.len());
    Some(rest[..end].trim().trim_matches('"'))
}

fn root_of(name: &str) -> Option<PitchClass> {
    let mut letters = name.chars();
    let base = match letters.next()? {
        'C' => 0,
        'D' => 2,
        'E' => 4,
        'F' => 5,
        'G' => 7,
        'A' => 9,
        'B' => 11,
        _ => return None,
    };
    let shift: i32 = letters
        .map(|sign| match sign {
            '#' => 1,
            'b' => -1,
            _ => 0,
        })
        .sum();
    Some(PitchClass::from_semitone(
        (base + shift).rem_euclid(12) as u8
    ))
}

/// The chords the player was asked to play: the first chord annotation
/// of the file (the second is what was played, chord by chord, with
/// every added note).
fn written(path: &Path) -> Vec<Written> {
    let text = std::fs::read_to_string(path).unwrap_or_default();
    let Some(at) = text.find("\"namespace\": \"chord\"") else {
        return Vec::new();
    };
    let Some(data) = text[at..].find("\"data\": [").map(|data| at + data) else {
        return Vec::new();
    };
    let end = text[data..].find(']').map_or(text.len(), |end| data + end);
    text[data..end]
        .split('{')
        .skip(1)
        .filter_map(|object| {
            let start: f64 = field(object, "time")?.parse().ok()?;
            let duration: f64 = field(object, "duration")?.parse().ok()?;
            let (root, kind) = field(object, "value")?.split_once(':')?;
            Some(Written {
                start,
                end: start + duration,
                root: root_of(root)?,
                kind: kind.to_string(),
            })
        })
        .collect()
}

/// Major or minor, as MIREX's "majmin" hears a kind: by its third and
/// fifth. `None` for a kind that is neither.
fn major_or_minor(intervals: &[u8]) -> Option<bool> {
    let has = |semitones: u8| intervals.iter().any(|&i| i % 12 == semitones);
    match (has(4), has(3), has(7)) {
        (true, false, true) => Some(true),
        (false, true, true) => Some(false),
        _ => None,
    }
}

fn written_intervals(kind: &str) -> &'static [u8] {
    match kind {
        "maj" => ChordQuality::Major.intervals(),
        "min" => ChordQuality::Minor.intervals(),
        "7" => ChordQuality::Dominant7.intervals(),
        "hdim7" => ChordQuality::HalfDiminished7.intervals(),
        _ => &[],
    }
}

fn exactly(kind: &str, quality: ChordQuality) -> bool {
    matches!(
        (kind, quality),
        ("maj", ChordQuality::Major)
            | ("min", ChordQuality::Minor)
            | ("7", ChordQuality::Dominant7)
            | ("hdim7", ChordQuality::HalfDiminished7)
    )
}

/// Time counted: (right, all) for the root, majmin and the exact chord.
#[derive(Default, Clone, Copy)]
struct Score {
    root: (f64, f64),
    majmin: (f64, f64),
    exact: (f64, f64),
    /// Time heard as no chord.
    none: f64,
}

impl Score {
    fn add(&mut self, other: Score) {
        for (mine, theirs) in [
            (&mut self.root, other.root),
            (&mut self.majmin, other.majmin),
            (&mut self.exact, other.exact),
        ] {
            mine.0 += theirs.0;
            mine.1 += theirs.1;
        }
        self.none += other.none;
    }

    fn line(&self, name: &str) -> String {
        let share = |(right, all): (f64, f64)| right / all.max(1e-9);
        format!(
            "{name:<8} root {:.3}  majmin {:.3}  exact {:.3}  no chord {:.3}  ({:.0} s)",
            share(self.root),
            share(self.majmin),
            share(self.exact),
            self.none / self.root.1.max(1e-9),
            self.root.1
        )
    }
}

fn compare(written: &[Written], heard: &[HeardChord]) -> Score {
    let mut score = Score::default();
    for chord in written {
        let mut time = chord.start + STEP / 2.0;
        while time < chord.end {
            let at = heard
                .iter()
                .find(|heard| heard.start <= time && time < heard.end)
                .and_then(|heard| heard.chord);
            let root = at.is_some_and(|at| at.root == chord.root);
            score.root.1 += STEP;
            score.none += if at.is_none() { STEP } else { 0.0 };
            score.root.0 += if root { STEP } else { 0.0 };
            if let Some(major) = major_or_minor(written_intervals(&chord.kind)) {
                score.majmin.1 += STEP;
                let right = root
                    && at.is_some_and(|at| major_or_minor(at.quality.intervals()) == Some(major));
                score.majmin.0 += if right { STEP } else { 0.0 };
            }
            score.exact.1 += STEP;
            let right = root && at.is_some_and(|at| exactly(&chord.kind, at.quality));
            score.exact.0 += if right { STEP } else { 0.0 };
            time += STEP;
        }
    }
    score
}

/// The style of a recording, from its name: `00_BN1-129-Eb_comp` is
/// bossa nova.
fn style_of(name: &str) -> String {
    let after = name.split('_').nth(1).unwrap_or("");
    after
        .chars()
        .take_while(|c| c.is_ascii_alphabetic())
        .collect()
}

#[test]
#[ignore = "needs the GuitarSet dataset: see the top of this file"]
fn chords_against_guitarset() {
    let Some(root) = std::env::var_os("GUITARSET").map(PathBuf::from) else {
        eprintln!("Set GUITARSET to the folder holding annotation and audio.");
        return;
    };
    let mut names: Vec<String> = std::fs::read_dir(root.join("annotation"))
        .map(|entries| {
            entries
                .filter_map(|entry| entry.ok()?.file_name().into_string().ok())
                .filter_map(|name| name.strip_suffix("_comp.jams").map(|n| format!("{n}_comp")))
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get());
    let scores: Vec<(String, Score)> = std::thread::scope(|scope| {
        let workers: Vec<_> = names
            .chunks(names.len().div_ceil(threads).max(1))
            .map(|chunk| {
                let root = &root;
                scope.spawn(move || {
                    chunk
                        .iter()
                        .filter_map(|name| {
                            let audio = root.join(format!("audio/{name}_mic.wav"));
                            let recording = decode_file(&audio).ok()?;
                            let heard = chords::heard(&recording.samples, recording.sample_rate);
                            let written = written(&root.join(format!("annotation/{name}.jams")));
                            Some((name.clone(), compare(&written, &heard)))
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        workers
            .into_iter()
            .flat_map(|worker| worker.join().unwrap_or_default())
            .collect()
    });
    let mut styles: std::collections::BTreeMap<String, Score> = Default::default();
    let mut all = Score::default();
    for (name, score) in &scores {
        styles.entry(style_of(name)).or_default().add(*score);
        all.add(*score);
    }
    println!("\n{} accompaniments", scores.len());
    for (style, score) in &styles {
        println!("{}", score.line(style));
    }
    println!("{}", all.line("all"));
}
