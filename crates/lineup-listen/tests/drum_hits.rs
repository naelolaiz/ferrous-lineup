//! How well [`lineup_listen::drums`] hears real drums, measured on MDB
//! Drums (23 tracks of MedleyDB with every drum hit written down by hand;
//! CC BY-NC-SA 4.0, https://github.com/CarlSouthall/MDBDrums). Nothing
//! is learned from it: it only measures.
//!
//! The dataset is not in the repository. Clone it, then:
//!
//! ```text
//! MDB_DRUMS="/path/to/MDBDrums/MDB Drums" cargo test --release \
//!     -p lineup-listen --test drum_hits -- --ignored --nocapture
//! ```
//!
//! For the drums alone and for the whole mix, it prints how well each
//! kind of drum is heard (kick, snare, hi-hat, toms, cymbals): the
//! *F-measure* of the hits heard within 50 ms of a written hit of the
//! same kind, which combines how many written hits are heard and how
//! many hits heard are written. Then how often an open hi-hat is told
//! from a closed one, and a ride from a crash, among the hits heard.

use std::path::{Path, PathBuf};

use lineup_core::Drum;
use lineup_listen::decode_file;
use lineup_listen::drums::{self, DrumHit};

const TOLERANCE: f64 = 0.05;

/// The kinds of drum the dataset's classes count.
const CLASSES: [&str; 5] = ["KD", "SD", "HH", "TT", "CY"];

/// One written hit: when, its class and its subclass.
struct Written {
    time: f64,
    class: &'static str,
    subclass: String,
}

fn class_of_subclass(subclass: &str) -> &'static str {
    match subclass {
        "KD" => "KD",
        "SD" | "SDB" | "SDD" | "SDF" | "SDG" | "SDNS" | "SST" => "SD",
        "CHH" | "OHH" | "PHH" => "HH",
        "HIT" | "MHT" | "HFT" | "LFT" => "TT",
        "RDC" | "RDB" | "CRC" | "CHC" | "SPC" => "CY",
        _ => "OT",
    }
}

fn class_of(drum: Drum) -> &'static str {
    match drum {
        Drum::Kick => "KD",
        Drum::Snare | Drum::Clap | Drum::Rim => "SD",
        Drum::HiHat | Drum::OpenHat => "HH",
        Drum::LowTom | Drum::HighTom | Drum::Conga => "TT",
        Drum::Ride | Drum::Crash => "CY",
        Drum::Cowbell | Drum::Shaker => "OT",
    }
}

fn written(path: &Path) -> Vec<Written> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            let time = fields.next()?.parse().ok()?;
            let subclass = fields.next()?.to_string();
            Some(Written {
                time,
                class: class_of_subclass(&subclass),
                subclass,
            })
        })
        .collect()
}

/// Pairs each heard hit of `class` with the nearest written hit of
/// `class` within the tolerance, each written hit used once: the pairs,
/// as indices into `heard` and `written`.
fn pairs(heard: &[DrumHit], written: &[Written], class: &str) -> Vec<(usize, usize)> {
    let mut used = vec![false; written.len()];
    let mut pairs = Vec::new();
    for (h, hit) in heard.iter().enumerate() {
        if class_of(hit.drum) != class {
            continue;
        }
        let best = written
            .iter()
            .enumerate()
            .filter(|(w, write)| {
                !used[*w] && write.class == class && (write.time - hit.time).abs() <= TOLERANCE
            })
            .min_by(|a, b| {
                (a.1.time - hit.time)
                    .abs()
                    .total_cmp(&(b.1.time - hit.time).abs())
            });
        if let Some((w, _)) = best {
            used[w] = true;
            pairs.push((h, w));
        }
    }
    pairs
}

fn f_measure(matched: usize, heard: usize, written: usize) -> f64 {
    if heard + written == 0 {
        return 1.0;
    }
    2.0 * matched as f64 / (heard + written) as f64
}

#[derive(Default, Clone, Copy)]
struct Count {
    matched: usize,
    heard: usize,
    written: usize,
}

#[derive(Default)]
struct Totals {
    classes: [Count; 5],
    /// Hi-hats heard and written: (right open or closed, all).
    hats: (usize, usize),
    /// Open hi-hats written and heard: (heard open, all); and closed
    /// ones heard open.
    open: (usize, usize, usize),
    /// Cymbals heard and written: (right ride or crash, all).
    cymbals: (usize, usize),
    /// The F-measure of each class on each track, to average.
    per_track: [Vec<f64>; 5],
    /// How far each paired hit is from the written one, in ms.
    offsets: Vec<f64>,
    /// For the hits heard of each class that are not written: how many
    /// fall on a written hit of each other class (the sixth: of none).
    confused: [[usize; 6]; 5],
    /// For each subclass: (written hits heard, written hits).
    subclasses: std::collections::BTreeMap<String, (usize, usize)>,
}

fn run(root: &Path, folder: &str, suffix: &str) -> Totals {
    let mut totals = Totals::default();
    let mut names: Vec<String> = std::fs::read_dir(root.join("annotations/subclass"))
        .map(|entries| {
            entries
                .filter_map(|entry| entry.ok()?.file_name().into_string().ok())
                .filter_map(|name| name.strip_suffix("_subclass.txt").map(String::from))
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    println!("\n{folder}");
    println!("track                         KD     SD     HH     TT     CY");
    for name in names {
        let audio = root.join(format!("audio/{folder}/{name}_{suffix}.wav"));
        let Ok(recording) = decode_file(&audio) else {
            continue;
        };
        let heard = drums::hits(&recording.samples, recording.sample_rate);
        let written = written(&root.join(format!("annotations/subclass/{name}_subclass.txt")));
        let mut line = format!("{name:<28}");
        for (index, class) in CLASSES.iter().enumerate() {
            let pairs = pairs(&heard, &written, class);
            let count = Count {
                matched: pairs.len(),
                heard: heard.iter().filter(|h| class_of(h.drum) == *class).count(),
                written: written.iter().filter(|w| w.class == *class).count(),
            };
            let total = &mut totals.classes[index];
            total.matched += count.matched;
            total.heard += count.heard;
            total.written += count.written;
            let f = f_measure(count.matched, count.heard, count.written);
            if count.written > 0 {
                totals.per_track[index].push(f);
                line += &format!(" {f:>6.3}");
            } else {
                line += &format!(" {:>6}", format!("({})", count.heard));
            }
            for (h, hit) in heard.iter().enumerate() {
                if class_of(hit.drum) != *class || pairs.iter().any(|&(p, _)| p == h) {
                    continue;
                }
                let near = written
                    .iter()
                    .filter(|w| (w.time - hit.time).abs() <= TOLERANCE)
                    .filter_map(|w| CLASSES.iter().position(|c| *c == w.class))
                    .find(|&other| other != index);
                totals.confused[index][near.unwrap_or(5)] += 1;
            }
            for write in written.iter().filter(|w| w.class == *class) {
                totals
                    .subclasses
                    .entry(write.subclass.clone())
                    .or_default()
                    .1 += 1;
            }
            for &(_, w) in &pairs {
                totals
                    .subclasses
                    .entry(written[w].subclass.clone())
                    .or_default()
                    .0 += 1;
            }
            for &(h, w) in &pairs {
                totals
                    .offsets
                    .push(1000.0 * (heard[h].time - written[w].time));
                let subclass = written[w].subclass.as_str();
                match heard[h].drum {
                    Drum::HiHat | Drum::OpenHat => {
                        totals.hats.1 += 1;
                        let open = heard[h].drum == Drum::OpenHat;
                        if open == (subclass == "OHH") {
                            totals.hats.0 += 1;
                        }
                        if subclass == "OHH" {
                            totals.open.1 += 1;
                            totals.open.0 += usize::from(open);
                        } else if open {
                            totals.open.2 += 1;
                        }
                    }
                    Drum::Ride | Drum::Crash => {
                        totals.cymbals.1 += 1;
                        let ride = matches!(subclass, "RDC" | "RDB");
                        if (heard[h].drum == Drum::Ride) == ride {
                            totals.cymbals.0 += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
        println!("{line}");
    }
    let mut line = format!("{:<28}", "all hits");
    for count in &totals.classes {
        line += &format!(
            " {:>6.3}",
            f_measure(count.matched, count.heard, count.written)
        );
    }
    println!("{line}");
    let mut line = format!("{:<28}", "mean of tracks");
    for scores in &totals.per_track {
        line += &format!(
            " {:>6.3}",
            scores.iter().sum::<f64>() / scores.len().max(1) as f64
        );
    }
    println!("{line}");
    for (name, count) in CLASSES.iter().zip(&totals.classes) {
        println!(
            "{name}: {} of {} written heard, {} of {} heard written",
            count.matched, count.written, count.matched, count.heard
        );
    }
    let mean = |range: std::ops::Range<usize>| {
        let n = range.len() as f64;
        range
            .map(|i| {
                let c = totals.classes[i];
                f_measure(c.matched, c.heard, c.written)
            })
            .sum::<f64>()
            / n
    };
    let subclasses: Vec<String> = totals
        .subclasses
        .iter()
        .map(|(name, (heard, all))| format!("{name} {heard}/{all}"))
        .collect();
    println!("written hits heard: {}", subclasses.join(", "));
    println!("hits heard but not written, by what is written there:");
    println!("        KD     SD     HH     TT     CY   none");
    for (name, row) in CLASSES.iter().zip(&totals.confused) {
        let cells: String = row.iter().map(|n| format!(" {n:>6}")).collect();
        println!("{name}  {cells}");
    }
    println!(
        "mean F: {:.3} for KD, SD, HH; {:.3} for all five",
        mean(0..3),
        mean(0..5)
    );
    println!(
        "open or closed hi-hat right: {} of {} (open hi-hats heard open: {} of {}; \
         closed ones heard open: {}); ride or crash right: {} of {}",
        totals.hats.0,
        totals.hats.1,
        totals.open.0,
        totals.open.1,
        totals.open.2,
        totals.cymbals.0,
        totals.cymbals.1
    );
    totals.offsets.sort_by(f64::total_cmp);
    if !totals.offsets.is_empty() {
        let median = totals.offsets[totals.offsets.len() / 2];
        println!("median offset of the hits heard: {median:+.1} ms");
    }
    totals
}

#[test]
#[ignore = "needs the MDB Drums dataset: see the top of this file"]
fn drum_hits_against_mdb_drums() {
    let Some(root) = std::env::var_os("MDB_DRUMS").map(PathBuf::from) else {
        eprintln!("Set MDB_DRUMS to the dataset's \"MDB Drums\" folder.");
        return;
    };
    run(&root, "drum_only", "Drum");
    run(&root, "full_mix", "MIX");
}
