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
| Bass | Synthetic low-register phrases only; no reference-dataset evaluation yet | Not benchmarked |

These are classical DSP baselines, not source-separated results. The
drum extractor reports kick, snare and hi-hats; it does not yet
transcribe toms, cymbals or percussion. The chord results are strongest
on GuitarSet's rock and singer-songwriter recordings and weaker on its
funk and jazz recordings.

## Rough edges

- Notes sung again on the same pitch with no gap and no new attack (a
  legato repeat) come out as one long note.
- The bass extractor is a monophonic pYIN baseline; on dense mixes it
  can follow another low instrument, and has not yet been measured
  against a bass transcription dataset.
- A voice over loud backing is heard as whichever pitch is strongest.
