//! How well [`lineup_listen::bass`] hears a bass line through the rest
//! of a mix, measured on the 23 MedleyDB tracks of MDB Drums
//! (CC BY-NC-SA 4.0, https://github.com/CarlSouthall/MDBDrums), each of
//! which has its bass (electric or double) recorded on a track of its
//! own.
//!
//! No one has written these bass lines down, so this is a stand-in: the
//! notes heard on the bass's own track are taken as the written ones,
//! and the mix is measured against them. It measures what the rest of
//! the band costs, not how right the notes heard on a bass alone are.
//!
//! The dataset is not in the repository. Clone it, then:
//!
//! ```text
//! MDB_DRUMS="/path/to/MDBDrums/MDB Drums" cargo test --release \
//!     -p lineup-listen --test bass_notes -- --ignored --nocapture
//! ```
//!
//! To measure basses separated from the mixes by another program (as a
//! source-separation model), set `MDB_SEPARATED` to one or more folders,
//! joined by `:` as in `PATH`, each holding a folder per track (named
//! as the mix, without `_MIX.wav`) with the bass in `bass.wav`.
//!
//! For each track and for all of them, it prints:
//!
//! - **notes**: the *F-measure* of the notes heard starting within 50 ms
//!   of a note heard on the bass alone, on the same pitch within a
//!   quarter tone (combining how many of those are found and how many
//!   notes found are among them); and the same allowing a wrong octave;
//! - **time**: of the time the bass alone sounds a note, how much the
//!   mix sounds a note of the same pitch.

use std::path::{Path, PathBuf};

use lineup_core::take::BassNote;
use lineup_listen::bass;
use lineup_listen::decode_file;

const TOLERANCE: f64 = 0.05;
const STEP: f64 = 0.01;

/// The file of the bass's own track: the stem the dataset's metadata
/// says is an electric or a double bass.
fn bass_stem(root: &Path, name: &str) -> Option<PathBuf> {
    let folder = root.join(format!("audio/multi-tracks/{name}"));
    let metadata = std::fs::read_to_string(folder.join(format!("{name}_METADATA.yaml"))).ok()?;
    let mut file = None;
    for line in metadata.lines() {
        // Stems are indented by four spaces, their raw files by eight.
        if let Some(value) = line.strip_prefix("    filename: ") {
            file = Some(value.trim().to_string());
        } else if let Some(value) = line.strip_prefix("    instrument: ")
            && matches!(value.trim(), "electric bass" | "double bass")
        {
            return Some(folder.join(format!("{name}_STEMS")).join(file?));
        }
    }
    None
}

fn same_pitch(a: f32, b: f32, any_octave: bool) -> bool {
    let apart = (a - b).abs();
    if any_octave {
        let apart = apart % 12.0;
        apart.min(12.0 - apart) <= 0.5
    } else {
        apart <= 0.5
    }
}

/// How many notes heard start within [`TOLERANCE`] of a written note
/// on the same pitch, each note counted at most once.
fn matched(heard: &[BassNote], written: &[BassNote], any_octave: bool) -> usize {
    let mut used = vec![false; written.len()];
    let mut matched = 0;
    for note in heard {
        let best = written
            .iter()
            .enumerate()
            .filter(|&(w, write)| {
                !used[w]
                    && (write.start - note.start).abs() <= TOLERANCE
                    && same_pitch(write.pitch, note.pitch, any_octave)
            })
            .min_by(|a, b| {
                (a.1.start - note.start)
                    .abs()
                    .total_cmp(&(b.1.start - note.start).abs())
            });
        if let Some((w, _)) = best {
            used[w] = true;
            matched += 1;
        }
    }
    matched
}

fn pitch_at(notes: &[BassNote], time: f64) -> Option<f32> {
    notes
        .iter()
        .find(|note| note.start <= time && time < note.end)
        .map(|note| note.pitch)
}

/// Of the time `written` sounds a note: (how much `heard` sounds the
/// same pitch, all of it), in steps of [`STEP`].
fn time_right(heard: &[BassNote], written: &[BassNote]) -> (usize, usize) {
    let end = written.last().map_or(0.0, |note| note.end);
    let mut right = 0;
    let mut all = 0;
    let mut time = 0.0;
    while time < end {
        if let Some(pitch) = pitch_at(written, time) {
            all += 1;
            if pitch_at(heard, time).is_some_and(|other| same_pitch(pitch, other, false)) {
                right += 1;
            }
        }
        time += STEP;
    }
    (right, all)
}

fn f_measure(matched: usize, heard: usize, written: usize) -> f64 {
    if heard + written == 0 {
        return 1.0;
    }
    2.0 * matched as f64 / (heard + written) as f64
}

fn notes_of(path: &Path) -> Option<Vec<BassNote>> {
    let recording = match decode_file(path) {
        Ok(recording) => recording,
        Err(error) => {
            eprintln!("Skipping {}: could not decode: {error}", path.display());
            return None;
        }
    };
    Some(bass::notes(&recording.samples, recording.sample_rate))
}

/// Measures the bass heard in the audio file `audio` gives for each
/// track's name against the bass heard alone (`written`), printed under
/// `label`.
fn run(written: &[(String, Vec<BassNote>)], label: &str, audio: &dyn Fn(&str) -> PathBuf) {
    println!("\n{label}");
    println!("track                       alone  heard  notes octave   time");
    let mut sums = [0.0; 3];
    let mut tracks = 0;
    let (mut all_matched, mut all_octave, mut all_heard, mut all_written) = (0, 0, 0, 0);
    let (mut all_right, mut all_time) = (0, 0);
    for (name, written) in written {
        let Some(heard) = notes_of(&audio(name)) else {
            continue;
        };
        let exact = matched(&heard, written, false);
        let octave = matched(&heard, written, true);
        let (right, time) = time_right(&heard, written);
        let scores = [
            f_measure(exact, heard.len(), written.len()),
            f_measure(octave, heard.len(), written.len()),
            right as f64 / time.max(1) as f64,
        ];
        println!(
            "{name:<28}{:>5}  {:>5} {:>6.3} {:>6.3} {:>6.3}",
            written.len(),
            heard.len(),
            scores[0],
            scores[1],
            scores[2]
        );
        for (sum, score) in sums.iter_mut().zip(scores) {
            *sum += score;
        }
        tracks += 1;
        all_matched += exact;
        all_octave += octave;
        all_heard += heard.len();
        all_written += written.len();
        all_right += right;
        all_time += time;
    }
    println!(
        "{:<28}{all_written:>5}  {all_heard:>5} {:>6.3} {:>6.3} {:>6.3}",
        "all notes",
        f_measure(all_matched, all_heard, all_written),
        f_measure(all_octave, all_heard, all_written),
        all_right as f64 / all_time.max(1) as f64
    );
    let tracks = f64::from(tracks).max(1.0);
    println!(
        "{:<28}              {:>6.3} {:>6.3} {:>6.3}",
        "mean of tracks",
        sums[0] / tracks,
        sums[1] / tracks,
        sums[2] / tracks
    );
}

#[test]
#[ignore = "needs the MDB Drums dataset: see the top of this file"]
fn bass_notes_against_mdb_bass_stems() {
    let Some(root) = std::env::var_os("MDB_DRUMS").map(PathBuf::from) else {
        eprintln!("Set MDB_DRUMS to the dataset's \"MDB Drums\" folder.");
        return;
    };
    let mut names: Vec<String> = std::fs::read_dir(root.join("audio/full_mix"))
        .map(|entries| {
            entries
                .filter_map(|entry| entry.ok()?.file_name().into_string().ok())
                .filter_map(|name| name.strip_suffix("_MIX.wav").map(String::from))
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    let written: Vec<(String, Vec<BassNote>)> = names
        .into_iter()
        .filter_map(|name| {
            let notes = notes_of(&bass_stem(&root, &name)?)?;
            Some((name, notes))
        })
        .collect();
    assert!(
        !written.is_empty(),
        "no bass stems under {}",
        root.display()
    );
    run(&written, "full_mix", &|name| {
        root.join(format!("audio/full_mix/{name}_MIX.wav"))
    });
    if let Some(separated) = std::env::var_os("MDB_SEPARATED") {
        for folder in std::env::split_paths(&separated) {
            let label = format!("separated: {}", folder.display());
            run(&written, &label, &|name| folder.join(name).join("bass.wav"));
        }
    }
}
