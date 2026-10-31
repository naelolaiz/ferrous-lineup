# lineup-listen

Listening to recordings for Ferrous Lineup: reading audio files, and
hearing the notes someone sings or whistles in them. The app uses it for
**takes**: the microphone button records a take while the song plays,
and **File > Add a recording…** (or dropping an audio file on the window,
or `ferrous-lineup song.mp3`) lays a recording on the song from the
playhead. Either way the notes heard are offered in the "Your take"
window, to write into a part.

## What is used for what

| Module | Job | How |
|--------|-----|-----|
| `decode` | Reading MP3, WAV, FLAC and Ogg Vorbis into samples | [Symphonia](https://crates.io/crates/symphonia) 0.6: pure Rust, no system library on any platform |
| `voice` | The pitch of a voice moment by moment, then its notes: start, end, pitch and loudness | Our own probabilistic YIN (pYIN) with a Viterbi search over the whole take, FFTs from [realfft](https://crates.io/crates/realfft) |
| `wav` | Saving a take as a WAV file, kept next to the song | [hound](https://crates.io/crates/hound), 32-bit float mono |

Lining the notes up with the song (which step each note starts on, at
the song's tempo, and which note of the key it is) is done in
`lineup_core::take`, which knows the song.

## How well it works

Measured on the vocadito dataset (40 solo singers, pitch and notes
written down by hand; see `tests/sung_notes.rs` for how to run it):

- **Pitch**: 84.7% of the sung frames heard within a quarter tone of the
  written pitch.
- **Notes**: F-measure 0.708 for notes starting within 50 ms on the same
  pitch.

Played back through a loopback (the speakers recorded as the microphone)
every note of a melody came back on its step, with a median timing error
of about 2 ms.

## Rough edges

- Notes sung again on the same pitch with no gap and no new attack (a
  legato repeat) come out as one long note.
- One voice at a time: a chord, or a voice over loud backing, is heard
  as whichever pitch is strongest.
