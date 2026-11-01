# lineup-listen

Listening to recordings for Ferrous Lineup: reading audio files, and
hearing the melody, bass line, drums and chords in them. The app uses it
for **takes**: the microphone button records a take while the song plays,
and **File > Add a recording…** (or dropping an audio file on the window,
or `ferrous-lineup song.mp3`) lays a recording on the song from the
playhead. The "Your take" window lets you choose any combination of the
four layers to write into the song.

## What is used for what

| Module | Job | How |
|--------|-----|-----|
| `decode` | Reading MP3, WAV, FLAC and Ogg Vorbis into samples | [Symphonia](https://crates.io/crates/symphonia) 0.6: pure Rust, no system library on any platform |
| `voice` | The pitch of a voice moment by moment, then its notes: start, end, pitch and loudness | Our own probabilistic YIN (pYIN) with a Viterbi search over the whole take, FFTs from [realfft](https://crates.io/crates/realfft) |
| `bass` | Bass notes: start, end, pitch and loudness | pYIN in the bass register (28–300 Hz), with 75 ms frames and a 20 Hz high-pass / 500 Hz low-pass |
| `drums` | Kick, snare and closed/open hi-hat hits | Harmonic-percussive separation, semi-adaptive NMF templates and spectral onset picking |
| `chords` | Chord changes and bass inversions | Chordino-style NNLS note spectra and a Viterbi chord-state decoder |
| `separation` | Optional local source-separation helpers before DSP extraction | Independently compiled adapters for charon_audio, stem-splitter-core and demucs-rs; no model/runtime linked into the app |
| `wav` | Saving a take as a WAV file, kept next to the song | [hound](https://crates.io/crates/hound), 32-bit float mono |

Lining the layers up with the song (which step each note or hit starts
on, the key and meter, and where chords belong on the sheet) is done in
`lineup_core::take`, which knows the song.

## How well it works

The melody was measured on the vocadito dataset (40 solo singers, pitch
and notes written down by hand; see `tests/sung_notes.rs` for how to run
it):

- **Pitch**: 84.7% of the sung frames heard within a quarter tone of the
  written pitch.
- **Notes**: F-measure 0.708 for notes starting within 50 ms on the same
  pitch.

Played back through a loopback (the speakers recorded as the microphone)
every note of a melody came back on its step, with a median timing error
of about 2 ms.

The drum extractor was measured on MDB Drums; the chord extractor on
GuitarSet's 180 accompaniment recordings. The measurements below are
from the ignored tests in `tests/drum_hits.rs` and
`tests/heard_chords.rs`, which need those datasets:

| Layer | Dataset / measure | Result |
|-------|-------------------|--------|
| Drums | MDB Drums, full mixes: mean F for kick, snare and hi-hat, hits within 50 ms | 0.652 |
| Drums | MDB Drums, drums alone: same measure | 0.784 |
| Chords | GuitarSet, root / major-minor / exact chord accuracy by time | 0.764 / 0.700 / 0.562 |
| Bass | MDB Drums proxy: mean per-track note F on mix / separated bass stem | 0.133 / 0.899 |

The bass numbers are **not transcription accuracy**: MDB Drums has no
bass-note annotations. Instead, pYIN notes from each isolated bass track
stand in for the written notes, measuring how much the rest of the mix
interferes. The drum extractor reports kick, snare and hi-hats; it does
not yet transcribe toms, cymbals or percussion. The chord results are
strongest on GuitarSet's rock and singer-songwriter recordings and
weaker on its funk and jazz recordings.

### Source-separation comparison

We also ran the same DSP extractors on HTDemucs-separated stems. On all
23 MDB Drums tracks, charon_audio's bass stems raised the bass proxy's
mean per-track note F from 0.133 (mix) to 0.899. The drum hit F for
kick, snare and hi-hat rose from 0.652 to 0.758; drums alone score 0.784.
Separation helps bass substantially, but helps drum hits only modestly.

| Separator | Measurement | Result |
|-----------|-------------|--------|
| charon_audio | Median scale-invariant SDR against MDB drum / bass multitrack stems (23 tracks) | 11.47 / 12.21 dB |
| stem-splitter-core | Same SDR (three-track subset only) | 14.72 / 12.36 dB |
| stem-splitter-core vs charon_audio | Median SDR between their drums / bass on the same three tracks | 24.3 / 23.0 dB |
| demucs-rs CPU vs charon_audio | SDR between outputs on one 13-second track, drums / bass | 17.9 / 21.4 dB |

These are different Rust runtimes for the same family of HTDemucs model,
not independent separation algorithms. stem-splitter-core's three-track
subset is too small to claim better quality; the close stem agreement
suggests the model is the main factor. Some individual SDR scores vary
widely, so the medians are more informative than any one track.

On this machine (Ryzen 7 7735U, 16 logical CPUs), charon_audio took
about 78 minutes to process 21.8 minutes of audio and peaked at 1.6 GB
RSS. stem-splitter-core peaked at 5.0 GB RSS on its three-track run.
demucs-rs's CPU backend took 9.4 minutes for one 13-second track; its
Vulkan backend lost the Radeon driver's device context. These costs are
why source separation is optional and disabled by default, rather than a
linked model/runtime dependency. It is most promising for bass-heavy
extraction. Melody and chord separation were not
scored: these datasets do not provide suitable mixed-recording
annotations for those layers.

To reproduce the separated-stem measurements, set `MDB_SEPARATED` to
directories laid out as `<root>/<track>/drums.wav` and
`<root>/<track>/bass.wav`, separated by `:`:

```text
MDB_DRUMS="/path/to/MDB Drums" \
MDB_SEPARATED="/path/to/charon:/path/to/stem-splitter" \
  cargo test --release -p lineup-listen --test drum_hits -- --ignored --nocapture
MDB_DRUMS="/path/to/MDB Drums" \
MDB_SEPARATED="/path/to/charon:/path/to/stem-splitter" \
  cargo test --release -p lineup-listen --test bass_notes -- --ignored --nocapture
```

### Trying the backends in the app

Build `lineup-app` with `--features separation` for all adapters, or any
combination of `separation-charon`, `separation-stem-splitter` and
`separation-demucs`. With no features, no helper can run. Choose a compiled
backend in **Settings > Audio > Source separation**; **Off** preserves
the DSP-on-mix behavior. A saved backend missing from a later build is
reported rather than silently replaced.

[The helper installer](../../scripts/install-separators.sh) builds the
three pinned upstream CLIs independently and installs them beside a
release build. This is intentional: charon_audio currently pins `ort`
rc.13 while stem-splitter-core pins rc.11, with the same native-library
link name. They cannot coexist unchanged in one Cargo dependency graph.
The adapter passes arguments without a shell and uses Charon's
`--no-server` mode, so a resident server cannot override the chosen model.

The original stereo file is passed to the helper; microphone samples
are written to a temporary WAV. All four stems must decode successfully
and match the input duration within 50 ms. Their own sample rates are
used by the extractors. The original recording is kept separately for
playback. Errors are shown, never silently replaced with analysis of the
mix. Cancellation stops and reaps the helper. Temporary audio and logs
are removed on completion; model caches are retained.

`tests/separation.rs` is an ignored, real-helper smoke test with its
environment-variable instructions at the top. Ordinary feature-enabled
tests use local fixtures, without model downloads. The app maps vocals
to melody, drums to hits, bass to bass notes and `other` to chords; the
vocal stem is not a general instrumental-melody stem.

## Rough edges

- Notes sung again on the same pitch with no gap and no new attack (a
  legato repeat) come out as one long note.
- The bass extractor is a monophonic pYIN baseline; on dense mixes it
  can follow another low instrument, and has not yet been measured
  against a bass transcription dataset.
- A voice over loud backing is heard as whichever pitch is strongest.
