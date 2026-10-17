//! How well key changes are found in recordings, measured on library
//! demos rendered to audio, whose keys are known from their scores.
//!
//! `cargo test --release -p lineup-listen --test key_changes -- --ignored --nocapture`
//! prints the report. It renders about 50 songs, so it takes a while.

use lineup_audio::{ChannelNotes, Command, Instrument, PlayMode, Renderer, SampleCache};
use lineup_core::harmony::key_finding::KeySpan;
use lineup_core::library::SONGS;
use lineup_core::library::demo::song_demo;
use lineup_core::library::song::Song;
use lineup_core::{PitchClass, Project, Scale, ScaleKind};
use lineup_listen::{Recording, analyze};

const RATE: u32 = 44_100;

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

fn truth(song: &Song) -> Vec<Scale> {
    // The demo's key markers, which follow the score.
    let project = song_demo(song);
    let length = project.song_length();
    (0..length).map(|step| project.key_at(step)).collect()
}

/// The project played into samples, mono, and when each song step starts.
fn render(project: &Project) -> (Recording, Vec<f64>) {
    let mut renderer = Renderer::new(RATE, 1);
    let mut samples = SampleCache::new();
    renderer.send(Command::SetTempo(project.tempo));
    for (index, pattern) in project.patterns().iter().enumerate() {
        renderer.send(Command::InsertPattern {
            index,
            length: pattern.length(),
        });
    }
    for (index, channel) in project.channels().iter().enumerate() {
        renderer.send(Command::InsertChannel {
            index,
            instrument: Instrument::for_channel(channel, RATE, &mut samples).unwrap(),
            notes: ChannelNotes::new(project.patterns().iter().map(|p| p.notes(index).to_vec())),
            volume: channel.volume,
            muted: channel.muted,
        });
    }
    let song = project.arrangement();
    let tempos = project.played_tempos(&song);
    let mut starts = vec![0.0];
    for step in 0..song.length {
        let tempo = tempos.get(step).copied().unwrap_or(project.tempo);
        starts.push(starts[step] + lineup_core::tempo::seconds(1, |_| tempo));
    }
    renderer.send(Command::SetSong {
        plays: song.plays,
        length: song.length + 64,
    });
    renderer.send(Command::SetSongTempos(tempos));
    renderer.send(Command::SetMode(PlayMode::Song));
    renderer.send(Command::Play);
    let mut out = vec![0.0; (starts[song.length] * f64::from(RATE)) as usize];
    renderer.render(&mut out);
    (
        Recording {
            samples: out,
            sample_rate: RATE,
        },
        starts,
    )
}

fn describe(span: &KeySpan) -> String {
    let mut text = span.key().map_or("?".into(), |key| key.name());
    if !span.is_clear() {
        let others: Vec<String> = span.alternatives().iter().map(|g| g.key.name()).collect();
        text += &format!(" (or {})", others.join(", "));
    }
    text
}

#[test]
#[ignore = "a report, not a check: renders many songs"]
fn report() {
    let modulating: Vec<&Song> = SONGS
        .iter()
        .filter(|song| song.parts.iter().any(|part| part.key.is_some()))
        .collect();
    let steady: Vec<&Song> = SONGS
        .iter()
        .filter(|song| song.parts.iter().all(|part| part.key.is_none()))
        .step_by(5)
        .collect();
    let (mut beats_total, mut beats_right) = (0, 0);
    let (mut main_right, mut bpm_key_right, mut songs) = (0, 0, 0);
    let (mut steady_changes, mut true_changes, mut found_changes) = (0, 0, 0);
    for song in modulating.iter().chain(&steady) {
        let project = song_demo(song);
        let truth = truth(song);
        let (recording, starts) = render(&project);
        let analysis = analyze(&recording);
        let step_at = |time: f64| starts.partition_point(|&s| s <= time).saturating_sub(1);
        let (mut total, mut right) = (0, 0);
        for (beat, &time) in analysis.beats.iter().enumerate() {
            let Some(expected) = truth.get(step_at(time)).copied().and_then(family) else {
                continue;
            };
            let found = analysis
                .keys
                .iter()
                .find(|span| (span.start..span.end).contains(&beat))
                .and_then(KeySpan::key)
                .and_then(family);
            total += 1;
            right += usize::from(found == Some(expected));
        }
        beats_total += total;
        beats_right += right;
        songs += 1;
        let main = analysis
            .keys
            .iter()
            .max_by_key(|span| span.end - span.start)
            .and_then(KeySpan::key);
        main_right += usize::from(main.and_then(family) == family(song.scale()));
        bpm_key_right += usize::from(analysis.scale.and_then(family) == family(song.scale()));
        let truth_changes: Vec<usize> = (1..truth.len())
            .filter(|&i| family(truth[i]) != family(truth[i - 1]))
            .collect();
        // Within two beats of a true change.
        let found: Vec<f64> = analysis
            .keys
            .iter()
            .skip(1)
            .filter_map(|span| analysis.beats.get(span.start).copied())
            .collect();
        let beat = analysis.beat_seconds();
        if truth_changes.is_empty() {
            steady_changes += usize::from(!found.is_empty());
        } else {
            true_changes += truth_changes.len();
            found_changes += truth_changes
                .iter()
                .filter(|&&step| {
                    found
                        .iter()
                        .any(|&t| (t - starts[step]).abs() <= 2.0 * beat)
                })
                .count();
        }
        let truth_text: Vec<String> = std::iter::once(0)
            .chain(truth_changes.iter().copied())
            .map(|step| format!("{}@{:.0}s", truth[step].name(), starts[step]))
            .collect();
        let found_text: Vec<String> = analysis
            .keys
            .iter()
            .map(|span| {
                let at = analysis.beats.get(span.start).copied().unwrap_or(0.0);
                format!("{}@{:.0}s", describe(span), at)
            })
            .collect();
        println!(
            "{:32} {:3}/{:3} beats  bpm-key: {}\n    truth: {}\n    found: {}",
            song.name,
            right,
            total,
            analysis.scale.map_or("-".into(), |s| s.name()),
            truth_text.join(" → "),
            found_text.join(" → "),
        );
    }
    println!();
    println!(
        "beats in the right key: {beats_right}/{beats_total} ({:.1}%)",
        100.0 * beats_right as f64 / beats_total as f64
    );
    println!(
        "main key right: {main_right}/{songs}; bpm-key's one key right: {bpm_key_right}/{songs}"
    );
    println!("true changes found within two beats: {found_changes}/{true_changes}");
    println!(
        "songs in one key with a change found: {steady_changes}/{}",
        steady.len()
    );
}

#[test]
#[ignore = "writes a WAV file for trying the import by hand"]
fn write_wav() {
    let name = std::env::var("SONG").unwrap();
    let path = std::env::var("WAV").unwrap();
    let song = SONGS.iter().find(|song| song.name == name).unwrap();
    let (recording, _) = render(&song_demo(song));
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(path, spec).unwrap();
    for s in recording.samples {
        writer
            .write_sample((s.clamp(-1.0, 1.0) * 32767.0) as i16)
            .unwrap();
    }
    writer.finalize().unwrap();
}
