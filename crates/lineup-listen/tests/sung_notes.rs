//! How well [`lineup_listen::voice`] hears real singing, measured on the
//! vocadito dataset (40 solo singers, each with the pitch and the notes
//! written down by hand; CC BY 4.0, https://zenodo.org/records/5578807).
//!
//! The dataset is not in the repository. Download and unpack it, then:
//!
//! ```text
//! VOCADITO=/path/to/vocadito cargo test --release -p lineup-listen \
//!     --test sung_notes -- --ignored --nocapture
//! ```
//!
//! It prints, for each take and for all of them:
//!
//! - **pitch**: of the frames where the singer sings, how many the pitch
//!   finder hears within a quarter tone of the written pitch;
//! - **voice**: how many of those it hears as voice at all, and how many
//!   of the silent frames it takes for voice;
//! - **notes**: how many written notes it finds (starting within 50 ms,
//!   or 100 ms, on the same pitch within a quarter tone), and how many of
//!   the notes it finds are written: the *F-measure* combines the two.

use std::path::{Path, PathBuf};

use lineup_listen::decode_file;
use lineup_listen::voice::{self, PitchFrame, SungNote, midi_of};

/// The dataset writes its pitch down 25 ms late, against its notes and
/// the sound: comparing each written pitch with the frame heard 25 ms
/// earlier lines them up best (85% right, against 78% at 0 ms and 82%
/// at 10 or 40 ms), while the notes found start on average within 6 ms
/// of the written ones.
const PITCH_WRITTEN_LATE: f64 = 0.025;

/// One written note: its start in seconds and its pitch.
struct Written {
    start: f64,
    pitch: f32,
}

fn rows(path: &Path) -> Vec<Vec<f64>> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|field| field.trim().parse().ok())
                .collect()
        })
        .filter(|row: &Vec<f64>| !row.is_empty())
        .collect()
}

/// Of the frames the dataset says are sung: how many are heard within a
/// quarter tone, and how many are heard as voice. And of the frames it
/// says are silent, how many are heard as voice.
fn pitch_scores(track: &[PitchFrame], truth: &[Vec<f64>]) -> (usize, usize, usize, usize, usize) {
    let (mut sung, mut right, mut voiced, mut silent, mut false_voice) = (0, 0, 0, 0, 0);
    for row in truth {
        let (time, hz) = (row[0], row[1] as f32);
        // The frame nearest to the written time.
        let first = track.first().map_or(0.0, |f| f.time);
        let index = ((time - PITCH_WRITTEN_LATE - first) / voice::HOP_SECONDS).round();
        let Some(frame) = track.get(index.max(0.0) as usize) else {
            continue;
        };
        if hz > 0.0 {
            sung += 1;
            if let Some(pitch) = frame.pitch {
                voiced += 1;
                if (pitch - midi_of(hz)).abs() < 0.5 {
                    right += 1;
                }
            }
        } else {
            silent += 1;
            if frame.pitch.is_some() {
                false_voice += 1;
            }
        }
    }
    (sung, right, voiced, silent, false_voice)
}

/// How many of `found` match a note of `written` (each note matched at
/// most once): starting within `tolerance` seconds, a quarter tone apart
/// at most.
fn matched(found: &[SungNote], written: &[Written], tolerance: f64) -> usize {
    let mut used = vec![false; written.len()];
    let mut count = 0;
    for note in found {
        let best = written
            .iter()
            .enumerate()
            .filter(|(index, w)| {
                !used[*index]
                    && (w.start - note.start).abs() <= tolerance
                    && (w.pitch - note.pitch).abs() < 0.5
            })
            .min_by(|a, b| {
                (a.1.start - note.start)
                    .abs()
                    .total_cmp(&(b.1.start - note.start).abs())
            });
        if let Some((index, _)) = best {
            used[index] = true;
            count += 1;
        }
    }
    count
}

fn f_measure(matched: usize, found: usize, written: usize) -> f64 {
    if found == 0 || written == 0 {
        return 0.0;
    }
    let precision = matched as f64 / found as f64;
    let recall = matched as f64 / written as f64;
    if precision + recall == 0.0 {
        0.0
    } else {
        2.0 * precision * recall / (precision + recall)
    }
}

#[test]
#[ignore = "needs the vocadito dataset: see the top of this file"]
fn sung_notes_against_vocadito() {
    let Some(root) = std::env::var_os("VOCADITO").map(PathBuf::from) else {
        eprintln!("Set VOCADITO to the folder of the unpacked dataset.");
        return;
    };
    let mut totals = (0, 0, 0, 0, 0);
    let (mut all_found, mut all_written, mut all_50, mut all_100) = (0, 0, 0, 0);
    println!("take   pitch  voice  false   notes  written  F(50ms) F(100ms)");
    for take in 1..=40 {
        let audio = root.join(format!("Audio/vocadito_{take}.wav"));
        let Ok(recording) = decode_file(&audio) else {
            continue;
        };
        let track = voice::pitch_track(&recording.samples, recording.sample_rate);
        let found = voice::notes(&track);
        let truth = rows(&root.join(format!("Annotations/F0/vocadito_{take}_f0.csv")));
        let written: Vec<Written> = rows(&root.join(format!(
            "Annotations/Notes/vocadito_{take}_notesA1.csv"
        )))
        .iter()
        .map(|row| Written {
            start: row[0],
            pitch: midi_of(row[1] as f32),
        })
        .collect();

        let scores = pitch_scores(&track, &truth);
        totals.0 += scores.0;
        totals.1 += scores.1;
        totals.2 += scores.2;
        totals.3 += scores.3;
        totals.4 += scores.4;
        let m50 = matched(&found, &written, 0.05);
        let m100 = matched(&found, &written, 0.1);
        all_found += found.len();
        all_written += written.len();
        all_50 += m50;
        all_100 += m100;
        println!(
            "{take:>4}  {:>5.1}% {:>5.1}% {:>5.1}%  {:>5}  {:>7}  {:>6.3}  {:>6.3}",
            100.0 * scores.1 as f64 / scores.0.max(1) as f64,
            100.0 * scores.2 as f64 / scores.0.max(1) as f64,
            100.0 * scores.4 as f64 / scores.3.max(1) as f64,
            found.len(),
            written.len(),
            f_measure(m50, found.len(), written.len()),
            f_measure(m100, found.len(), written.len()),
        );
    }
    println!(
        " all  {:>5.1}% {:>5.1}% {:>5.1}%  {:>5}  {:>7}  {:>6.3}  {:>6.3}",
        100.0 * totals.1 as f64 / totals.0.max(1) as f64,
        100.0 * totals.2 as f64 / totals.0.max(1) as f64,
        100.0 * totals.4 as f64 / totals.3.max(1) as f64,
        all_found,
        all_written,
        f_measure(all_50, all_found, all_written),
        f_measure(all_100, all_found, all_written),
    );
}
