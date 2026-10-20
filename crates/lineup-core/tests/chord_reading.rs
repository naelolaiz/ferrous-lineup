//! How well the chord sheet names what the demos play.
//!
//! The checks take bars of the hardest demos whose harmony is known from
//! their scores: the Rite of Spring's Augurs chord, Petrushka's chord and
//! the stack of fourths that opens Petrushka's fair.
//!
//! `cargo test --release -p lineup-core --test chord_reading -- --ignored
//! --nocapture` prints a report: the chord sheet heard from the notes of
//! every demo that has a chart, beat by beat against that chart.

use lineup_core::library::SONGS;
use lineup_core::library::demo::song_demo;
use lineup_core::library::song::Song;
use lineup_core::{HarmonyName, PitchSet, Project};

/// The chord sheet of the demo of the song called `name`, written from
/// its notes.
fn heard(name: &str) -> Project {
    let mut project = song_demo(Song::named(name).expect("a demo"));
    project.write_heard_chords();
    project
}

/// What the chord sheet says at the start of bar `number` (from 1).
fn at_bar(project: &Project, number: usize) -> String {
    let start = project.meter().bar(number - 1).start;
    project
        .playlist()
        .chord_markers()
        .iter()
        .rev()
        .find(|marker| marker.start <= start)
        .map(|marker| marker.text())
        .unwrap_or_default()
}

/// The bars in `bars` whose chord is not `expected`, with what they say.
fn wrong(
    project: &Project,
    bars: &[std::ops::RangeInclusive<usize>],
    expected: &str,
) -> Vec<String> {
    bars.iter()
        .cloned()
        .flatten()
        .map(|number| (number, at_bar(project, number)))
        .filter(|(_, text)| text != expected)
        .map(|(number, text)| format!("bar {number}: {text}"))
        .collect()
}

#[test]
fn the_augurs_chord_is_e_flat_seven_over_f_flat() {
    // Rehearsal 13 and the bars where the strings hammer the chord again
    // (15, and 18 to 21 under the bassoons' and oboe's tunes): F flat
    // major in the low strings, E flat seventh over it.
    let project = heard("The Rite of Spring, The Augurs of Spring");
    let bars = [1..=8, 13..=16, 18..=22, 35..=69];
    assert_eq!(wrong(&project, &bars, "Eb7|Fb"), Vec::<String>::new());
}

#[test]
fn petrushka_s_chord_is_c_over_f_sharp() {
    // Petrushka's Room: C major and F sharp major at once.
    let project = heard("Petrushka, Petrushka's Room");
    let wrong: Vec<String> = (47..=53)
        .map(|number| (number, at_bar(&project, number)))
        .filter(|(_, text)| !text.starts_with("C|F#"))
        .map(|(number, text)| format!("bar {number}: {text}"))
        .collect();
    assert_eq!(wrong, Vec::<String>::new());
}

#[test]
fn the_fair_opens_on_a_stack_of_fourths() {
    // The flute's D, E, G and A: the set 4-23, fourths stacked A D G C
    // without the C.
    let project = heard("Petrushka, The Shrovetide Fair");
    for number in 1..=5 {
        let start = project.meter().bar(number - 1).start;
        let marker = project
            .playlist()
            .chord_markers()
            .iter()
            .rev()
            .find(|marker| marker.start <= start)
            .expect("a chord");
        assert_eq!(
            marker.harmony,
            HarmonyName::Set(PitchSet::from_semitones(&[2, 4, 7, 9])),
            "bar {number}"
        );
    }
}

/// The root and the notes of a chord on a chart or a chord sheet.
fn chord_of(harmony: &HarmonyName) -> Option<(u8, PitchSet)> {
    match harmony {
        HarmonyName::Chord(symbol) => {
            let chord = symbol.chord();
            Some((chord.root.semitone(), chord.pitch_set()))
        }
        _ => None,
    }
}

/// Beats of `song`'s chart, and how many of them the chord sheet heard
/// from the notes names exactly, with a compatible chord (the same notes,
/// or the same root with notes added or left out) and with the right
/// root.
fn measure(song: &Song) -> (usize, usize, usize, usize) {
    let project = song_demo(song);
    let chart = project.playlist().chord_markers().to_vec();
    let mut heard = project.clone();
    heard.write_heard_chords();
    let found = heard.playlist().chord_markers().to_vec();
    let at = |markers: &[lineup_core::ChordMarker], beat: usize| {
        markers
            .iter()
            .rev()
            .find(|marker| marker.start <= beat)
            .and_then(|marker| chord_of(&marker.harmony))
    };
    let (mut beats, mut exact, mut compatible, mut root) = (0, 0, 0, 0);
    let end = project.song_length();
    for bar in project.meter().bars().take_while(|bar| bar.start < end) {
        for beat in bar.beat_starts() {
            let Some(truth) = at(&chart, beat) else {
                continue;
            };
            let found = at(&found, beat);
            beats += 1;
            exact += usize::from(found == Some(truth));
            root += usize::from(found.is_some_and(|f| f.0 == truth.0));
            compatible += usize::from(found.is_some_and(|f| {
                f.1 == truth.1
                    || (f.0 == truth.0 && (f.1.is_subset_of(truth.1) || truth.1.is_subset_of(f.1)))
            }));
        }
    }
    (beats, exact, compatible, root)
}

#[test]
#[ignore = "a report, not a check"]
fn report() {
    let (mut beats, mut exact, mut compatible, mut root) = (0, 0, 0, 0);
    let mut songs = 0;
    for song in SONGS {
        let (b, e, c, r) = measure(song);
        if b == 0 {
            continue;
        }
        songs += 1;
        beats += b;
        exact += e;
        compatible += c;
        root += r;
        if e * 10 < b * 7 {
            println!(
                "{:50} {b:5} beats, {:5.1}% exact, {:5.1}% root",
                song.name,
                100.0 * e as f32 / b as f32,
                100.0 * r as f32 / b as f32
            );
        }
    }
    let percent = |n: usize| 100.0 * n as f32 / beats as f32;
    println!(
        "{songs} charted demos, {beats} beats: {:.1}% exact, {:.1}% compatible, {:.1}% right root",
        percent(exact),
        percent(compatible),
        percent(root)
    );
}
