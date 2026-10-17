# loom-listen

A spike: import an MP3 or WAV file, listen to it, and build a harmony
track that follows it. **File > Import recording…**, or drop the file
on the window, or `ferrous-lineup song.mp3`.

What comes out is an ordinary project: the recording on a sampler
channel, started once at the beginning of the song, and a "Harmony" Loom
Osc channel with the chords heard on every beat, in patterns of 16 bars.
The tempo, time signature and key are set from the recording, so the bar
lines, the piano roll and the harmony keys line up with it, and the
harmony keys light up with the chord that is playing.

## What is used for what

| Job | How | Why |
|-----|-----|-----|
| Decoding MP3, WAV, FLAC, Ogg | [Symphonia](https://crates.io/crates/symphonia) 0.6 | Pure Rust, no system library on any platform |
| Beats and tempo | our own `beats` module: the Ellis (2007) dynamic-programming tracker with librosa's parameters, a port of [beat-track-rs](https://crates.io/crates/beat-track-rs) 0.1.2 | Best beat placement of the crates tried; the port gives the same beats on every test file, ten times faster, without the crate's second copy of Symphonia |
| Key | [bpm-key](https://crates.io/crates/bpm-key) 0.1 (Krumhansl-Kessler profiles) | Right most often, fastest |
| Chords per beat | our own `chroma` and `chords` modules: a log-magnitude FFT chroma, chord templates, and Viterbi smoothing | No crate did this well (see below) |
| Meter and bar 1 | our own `meter` module: where chord changes land | Nothing available does it |

## How well it works

Tried on 29 songs rendered from the library demos (so the tempo, key and
chord of every beat are known exactly) and 7 Creative Commons recordings
from librosa's example set (jazz, strings, ragtime, folk, drum and bass),
with librosa 1.0 as the reference for those.

**Tempo**, 29 demos, within 2 BPM / off by exactly half or double /
wrong, and time for a 2-minute song:

| Crate | Right | Half or double | Wrong | Time |
|-------|-------|----------------|-------|------|
| beat-track-rs (now the `beats` module; same results, 0.06–0.17 s) | 20 | 8 | 1 | 0.5–1.3 s |
| bpm-key | 20 | 9 | 0 | 0.2–0.5 s |
| stratum-dsp | 19 | 7 | 3 | 1.7–7.5 s |

Half or double tempo is the classic ambiguity of every tempo tracker,
and for playing along it does not matter much: the beats are still on
the beat. beat-track-rs put 100% of its beats on the eighth-note grid of
the true tempo in 27 of 29 songs; stratum-dsp's beat grid managed that in
none (48% to 97% on the grid, with its first beat sometimes 8 or 23
seconds in). On the real recordings beat-track-rs agreed with librosa on 6
of 7 tempos (the seventh was double), bpm-key on 5, stratum-dsp on 3.

**Key**, 29 demos (root and major/minor family; the modal demos counted as
their nearest major or minor):

| Crate | Right | Notes |
|-------|-------|-------|
| bpm-key | 24 | 3 of the 5 misses are the relative major or minor |
| stratum-dsp | 13 | often the dominant or the relative key |

**Chords**, 3 887 beats of the 29 demos, root and major/minor family:

| Method | Beats right |
|--------|-------------|
| loom-listen (templates + Viterbi, with the key as a prior) | 79% |
| chord_detector crate (Stark & Plumbley, frame by frame, majority vote per beat) | 68% |

Whole songs come out right (Canon in D, Four chords, Andalusian cadence,
UK garage: 100%); the misses are seventh chords heard as their triad or
the other way around, passing notes in melodies played as block chords
(Jingle Bells 33%, Greensleeves 38%), and the diminished chords of the
jazzier styles.

**Meter**: 3/4 was recognized on all 5 three-time demos and 4/4 on the
other 24. The first downbeat landed on a bar line (within 50 ms) on 22 of
29; the rest were a beat or two off, which the user can fix by dragging
the recording's clip.

## Not tried

- `beat-this` and `beatnet-rs` (neural beat trackers: need ONNX models, so
  not pure Rust at run time) and `stratum-dsp`'s optional `ml` feature.
- Separating the melody from the chords, so a tune played in single
  notes is followed by its implied harmony rather than by nothing.

## Rough edges

- The recording is saved again as a 32-bit WAV next to the original, with
  silence in front so bar 1 falls on a bar line. A sampler that can start
  at an offset would make the copy unnecessary.
- The sampler now fades out sounds longer than 10 seconds when playback
  stops, so the recording stops with the transport. That is a rule of
  thumb; a per-channel setting would be cleaner.
- Only 3/4 and 4/4 are guessed; 6/8 comes out as 3/4 at double speed.
