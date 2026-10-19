//! How well the key is found from the notes, measured on the library's
//! demo songs, whose key (and the key of every part that modulates) is
//! known from their scores.
//!
//! `cargo test -p lineup-core --test key_finding -- --ignored --nocapture`
//! prints the whole report.

use lineup_core::harmony::key_finding::KeySpan;
use lineup_core::library::SONGS;
use lineup_core::library::demo::song_demo;
use lineup_core::library::song::Song;
use lineup_core::{PitchClass, Scale, ScaleKind};

/// The major or minor key a scale belongs to, for comparing: a mode
/// counts as the major or minor key on its tonic with the same third.
fn family(scale: Scale) -> Option<(PitchClass, bool)> {
    use ScaleKind::*;
    let minor = match scale.kind {
        Major | Lydian | Mixolydian | MajorPentatonic => false,
        NaturalMinor | HarmonicMinor | MelodicMinor | Dorian | Phrygian | Locrian
        | MinorPentatonic | Blues => true,
        _ => return None,
    };
    Some((scale.root, minor))
}

/// The key of every timeline step of the song's demo.
fn truth(song: &Song) -> Vec<Scale> {
    // The demo's key markers, which follow the score.
    let project = song_demo(song);
    let length = project.song_length();
    (0..length).map(|step| project.key_at(step)).collect()
}

struct SongResult {
    name: &'static str,
    bars: usize,
    right_bars: usize,
    true_changes: Vec<usize>,
    found_changes: Vec<usize>,
    spans: Vec<KeySpan>,
}

fn measure(song: &'static Song) -> SongResult {
    let project = song_demo(song);
    let spans = project.heard_keys();
    let truth = truth(song);
    let meter = project.meter();
    let mut bars = 0;
    let mut right_bars = 0;
    for bar in meter.bars().take_while(|bar| bar.start < truth.len()) {
        let Some(expected) = family(truth[bar.start]) else {
            continue;
        };
        let found = spans
            .iter()
            .find(|span| (span.start..span.end).contains(&bar.start))
            .and_then(KeySpan::key)
            .and_then(family);
        bars += 1;
        if found == Some(expected) {
            right_bars += 1;
        }
    }
    let true_changes = (1..truth.len())
        .filter(|&i| family(truth[i]) != family(truth[i - 1]))
        .collect();
    let found_changes = spans.iter().skip(1).map(|span| span.start).collect();
    SongResult {
        name: song.name,
        bars,
        right_bars,
        true_changes,
        found_changes,
        spans,
    }
}

fn describe(span: &KeySpan) -> String {
    let mut text = span.key().map_or("?".into(), |key| key.name());
    if !span.is_clear() {
        let others: Vec<String> = span.alternatives().iter().map(|g| g.key.name()).collect();
        text += &format!(" (or {})", others.join(", "));
    }
    text
}

/// A change found within a bar (16 steps) of a true one.
fn near(found: usize, truth: &[usize]) -> bool {
    truth.iter().any(|&t| found.abs_diff(t) <= 16)
}

#[test]
#[ignore = "a report, not a check"]
fn report() {
    let results: Vec<SongResult> = SONGS.iter().map(measure).collect();
    let (mut bars, mut right) = (0, 0);
    let (mut true_total, mut true_found, mut extra) = (0, 0, 0);
    let mut steady_with_extra = 0;
    let mut steady = 0;
    let (mut clear_bars, mut clear_right, mut unclear_bars, mut unclear_listed) = (0, 0, 0, 0);
    for (song, result) in SONGS.iter().zip(&results) {
        bars += result.bars;
        right += result.right_bars;
        true_total += result.true_changes.len();
        true_found += result
            .true_changes
            .iter()
            .filter(|&&t| near(t, &result.found_changes))
            .count();
        let extras = result
            .found_changes
            .iter()
            .filter(|&&f| !near(f, &result.true_changes))
            .count();
        extra += extras;
        if result.true_changes.is_empty() {
            steady += 1;
            if extras > 0 {
                steady_with_extra += 1;
            }
        }
        let truth = truth(song);
        for span in &result.spans {
            let length = (span.end.min(truth.len()).saturating_sub(span.start)) / 16;
            let expected = truth.get(span.start).copied().and_then(family);
            let listed = std::iter::once(span.key())
                .chain(span.alternatives().iter().map(|g| Some(g.key)))
                .any(|key| key.and_then(family) == expected);
            if span.is_clear() {
                clear_bars += length;
                if span.key().and_then(family) == expected {
                    clear_right += length;
                }
            } else {
                unclear_bars += length;
                if listed {
                    unclear_listed += length;
                }
            }
        }
        let wrong = result.right_bars < result.bars;
        if wrong || !result.true_changes.is_empty() || extras > 0 {
            let truth_keys: Vec<String> = std::iter::once(0)
                .chain(result.true_changes.iter().copied())
                .map(|at| format!("{}@{}", truth[at].name(), at / 16))
                .collect();
            let found: Vec<String> = result
                .spans
                .iter()
                .map(|span| format!("{}@{}", describe(span), span.start / 16))
                .collect();
            println!(
                "{:32} {:3}/{:3}  truth: {}\n{:42}found: {}",
                result.name,
                result.right_bars,
                result.bars,
                truth_keys.join(" → "),
                "",
                found.join(" → ")
            );
        }
    }
    println!();
    println!(
        "bars right: {right}/{bars} ({:.1}%)",
        100.0 * right as f64 / bars as f64
    );
    println!("true key changes found within a bar: {true_found}/{true_total}");
    println!("changes found that are not in the scores: {extra}");
    println!("songs in one key with a change found: {steady_with_extra}/{steady}");
    println!(
        "clear spans right: {clear_right}/{clear_bars} bars; unclear spans with the true key among those shown: {unclear_listed}/{unclear_bars} bars"
    );
}

#[test]
fn most_bars_of_the_demos_are_heard_in_their_key() {
    let results: Vec<SongResult> = SONGS.iter().map(measure).collect();
    let bars: usize = results.iter().map(|r| r.bars).sum();
    let right: usize = results.iter().map(|r| r.right_bars).sum();
    // 95.2% when this was written, 91.6% once the swing standards came in:
    // the finder hears their chains of dominants (D7, G7 and C7 before the F
    // of Sweet Georgia Brown) as keys of their own. 91.0% once Entarisi ala
    // benziyor came in: its makam hicaz is written as A Phrygian, but its
    // third is raised to C sharp, so the finder hears A major. 89.6% once
    // more makam songs came in, whose notes are those of another key: the
    // hicaz of Çayeli'nden öteye and Geamparaua de la Murfatlar (A, B flat,
    // C sharp, D, E, F, G) is D harmonic minor's, and the segâh of İzmir'in
    // kavakları, written as B Phrygian, G major's, which is what the finder
    // hears; and the Mission: Impossible theme, in G minor, is heard in C
    // minor, its iv.
    assert!(right * 100 >= bars * 89, "{right}/{bars}");
}

#[test]
fn clear_modulations_are_found_on_their_bar() {
    for name in ["Habanera", "Tarantella napoletana", "Alma llanera"] {
        let song = SONGS.iter().find(|song| song.name == name).unwrap();
        let result = measure(song);
        assert_eq!(result.found_changes, result.true_changes, "{name}");
    }
}

/// El Choclo and Gallo ciego, played in full, go back to their first key
/// after a single strain of eight bars in another (El Choclo's A major,
/// Gallo ciego's B flat), and the finder hears so short a strain as passing
/// through, since two changes of key cost more than it gains. Every change
/// it does hear must still be one the score makes, on its bar or the next.
#[test]
fn the_changes_heard_in_songs_with_short_strains_are_real() {
    for name in ["El Choclo", "Gallo ciego"] {
        let song = SONGS.iter().find(|song| song.name == name).unwrap();
        let result = measure(song);
        assert!(!result.found_changes.is_empty(), "{name}");
        assert!(
            result
                .found_changes
                .iter()
                .all(|&found| near(found, &result.true_changes)),
            "{name}: found {:?}, true {:?}",
            result.found_changes,
            result.true_changes
        );
    }
}

#[test]
fn the_keys_heard_can_be_marked_on_the_playlist() {
    let song = SONGS.iter().find(|song| song.name == "Habanera").unwrap();
    let mut project = song_demo(song);
    let spans = project.heard_keys();
    project.mark_heard_keys(&spans);
    assert_eq!(
        project.scale,
        Scale::new(PitchClass::D, ScaleKind::NaturalMinor)
    );
    let map = project.key_map();
    assert_eq!(map.markers().len(), 1);
    assert_eq!(map.markers()[0].start, spans[1].start);
    assert_eq!(
        map.markers()[0].scale,
        Scale::new(PitchClass::D, ScaleKind::Major)
    );
}
