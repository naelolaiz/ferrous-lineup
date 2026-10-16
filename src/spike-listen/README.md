# Spike: listening to a recording

A *spike* is an experiment: a branch that tries something out to learn
whether it can work and how, without the polish a real step gets. This
one asks: can Ferrous Loom take an MP3 or a WAV file, hear its tempo,
key and chords, and play a harmony track along with it?

It can, well enough to be useful. **File > Import recording…** (or
dropping the file on the window, or `ferrous-loom song.mp3`) opens the
recording as a new project: the recording itself on a sampler channel,
and a **Harmony** channel with the chord heard on every beat. The tempo,
time signature and key are set from what was heard, so the bar lines
fall on the recording's bars and the harmony keys light up with the
chord that is playing.

![Vibe Ace imported, with the harmony keys following its chords](../images/spike-listen-vibe-ace.png)

The code lives in a new crate, `loom-listen`, plus one file in the app
(`app/import.rs`) and a small change to the sampler. This chapter
explains how it works and what was learned. The crate's
[`README.md`](https://github.com/naelolaiz/ferrous-loom/tree/main/crates/loom-listen)
has the detailed numbers from trying the alternatives.

## The pipeline

Listening is five stages, each its own module, each a plain function
from one kind of data to the next:

1. **Decode** (`decode.rs`): the file becomes samples, with the
   [Symphonia](https://crates.io/crates/symphonia) crate.
2. **Beats** (`beats.rs`): when each beat falls, in seconds, and the
   tempo. The spike used the `beat-track-rs` crate here; once it was
   merged, the crate was replaced by our own port (see below).
3. **Key** (with the `bpm-key` crate): the key the recording is in.
4. **Chords** (`chroma.rs` and `chords.rs`, our own): which chord sounds
   from each beat to the next.
5. **Meter** (`meter.rs`, our own): three or four beats to the bar, and
   which beat is the first downbeat.

Then `arrange.rs` turns the result into a `Project`, with the same types
the rest of Ferrous Loom uses: `Channel`, `Pattern`, `NoteEvent`, `Clip`.
Nothing downstream knows the notes came from a recording.

### Decoding with Symphonia

An audio file is a *container* (WAV, MP3, Ogg...) holding one or more
*tracks*, each compressed with a *codec*. Symphonia separates the two:
you *probe* the container, pick its audio track, and feed the track's
*packets* to a decoder for its codec. Each packet decodes to a block of
samples, usually interleaved (left, right, left, right...), which
`decode_file` averages into one channel.

Two Rust things to notice. Symphonia wants its input as a
`Box<dyn MediaSource>`: a *trait object*, "any type that implements
`MediaSource`, behind a pointer". A `File` is one; so is an in-memory
buffer. The `Box` is needed because different types have different
sizes, and `dyn` means the actual method to call is looked up at run
time. The decoder loop also shows how errors are *matched*, not just
propagated: an I/O error at the end means a file cut short, so we keep
what we have; a decode error means one damaged packet, so we skip it;
anything else is returned with `?`.

Symphonia's codecs are Cargo *features*, turned on in the workspace
`Cargo.toml`: `mp3`, `wav`, `pcm`, `flac`, `ogg` and `vorbis`. AAC and
ALAC exist too but are left off; a feature is a line to add.

### From samples to chroma

A chord is a set of *pitch classes*: C major is C, E and G in any octave.
To find chords in sound we need to know, moment by moment, how strongly
each of the twelve pitch classes sounds. That is called the *chroma*, and
it comes from a Fourier transform.

`chroma::frames` cuts the recording into overlapping *frames* of 8 192
samples, about 190 ms each, starting a new one every 46 ms. Each frame is
multiplied by a *Hann window*, a smooth bump that fades the frame in and
out: without it, the sudden cut at each end would look to the FFT like a
click containing every frequency. The FFT (from the `realfft` crate)
then says how loud each frequency *bin* is in the frame. Bin `k` is the
frequency `k × sample_rate / 8192`, so at 44.1 kHz bins are 5.4 Hz apart,
narrow enough to tell E2 from F2 (82.4 and 87.3 Hz) but nothing lower,
which is why the frame is so long.

Each bin belongs to a note: the formula `69 + 12 × log2(f / 440)` turns a
frequency into a MIDI note number (69 is A4, and every semitone up
multiplies the frequency by the twelfth root of two). Rounding gives the
nearest note, `% 12` its pitch class, and the bin's loudness is added to
that pitch class. Three details make this work in practice:

- the loudness is taken as `ln(1 + 100 × magnitude)`, because we hear on
  a logarithmic scale: a quiet note an octave up should still count, and
  the loudest note should not drown everything;
- higher notes get more bins, since they are further apart in Hz, so each
  pitch class is divided by its number of bins;
- two chromas are kept per frame: a *treble* one (C3 to C7), where the
  chord is, and a *bass* one (E1 to C4), which mostly says which note is
  lowest, and the lowest note is usually the root.

The bins-to-pitch-class table is built once, outside the frame loop, as
a `Vec<(usize, usize)>`: the loop then does nothing but additions.

### From chroma to chords: templates and Viterbi

`chords::chords_per_beat` gets one averaged chroma per beat (the frames
between one beat and the next) and names each one. There are two ideas.

**Templates.** Every chord we can recognize (six qualities on twelve
roots, 72 chords, plus "no chord") is a *template*: twelve numbers, 1
for its notes and 0 for the rest. The beat's chroma is compared with
each template by *correlation*: both are shifted to average 0 and scaled
to length 1, and then their dot product is 1 if they rise and fall
together and 0 if unrelated. Three small adjustments: a bass note on the
chord's root adds a little, a chord that belongs to the key adds a
little (most chords of a song do, so the key is a useful *prior*), and a
four-note chord loses a little, so a plain triad wins unless the seventh
is really there.

**Smoothing.** Taken alone, the best chord of each beat jumps around: a
passing melody note makes C look like Am for one beat. Real chords last.
So changing chord costs a penalty, and the *Viterbi algorithm* finds the
sequence of chords with the best total score. It is a *dynamic
programming* algorithm: for each beat and each of the 73 candidates it
keeps the best score of any sequence that ends there, computed from the
previous beat's bests, and remembers where it came from. At the end it
takes the best last chord and walks back through the "came from" table.
With 73 candidates that would be 73 × 73 comparisons per beat, but since
changing chord costs the same from anywhere, the best way to change into
any chord is always from the previous beat's overall best, which makes it
73 per beat.

On the 29 library demos rendered to WAV, where the chord of every beat is
known, this names 79% of beats right (root and major/minor family). The
`chord_detector` crate, which implements a published frame-by-frame
method, got 68% on the same recordings; the difference is mostly the
smoothing and the key prior.

### Beats: onsets, autocorrelation and dynamic programming

`beats::track` is the beat tracker of Ellis (2007), in three stages.
First the *onset envelope*: a spectrogram on the *mel scale* (bands
spaced the way the ear hears them, narrow at the bottom and wide at the
top), in decibels, and for each frame the sum of every band's increase
since the previous frame. Decreases do not count: a note dying away is
not an onset. The envelope peaks on every drum hit and note start.

Then the tempo: the envelope is compared with itself shifted by every
possible beat length (its *autocorrelation*). At the right shift, peaks
land on peaks. Shifts near 120 BPM get a head start, because trackers
otherwise pick half or double the tempo as readily as the tempo itself.

Then the beats, by dynamic programming again: the best score for a beat
at frame `i` is its onset strength plus the best score of a beat about
one beat length earlier, minus a penalty for how far from one beat
length that is. Each frame remembers the earlier beat it chose, and the
strongest final score is traced back to the start. The autocorrelation
only knows whole frames, so the tempo it reports is a little quantized;
`lib.rs` fits a straight line through the beat times afterwards
(`tempo_of`, a least-squares fit) to get the tempo the project is set to.

This module started as the `beat-track-rs` crate, which proved the
approach. But the crate also decodes files, so it brought a second copy
of Symphonia (an older version, with every codec) and `ndarray` into
every build. Its tracker is about 250 lines, so after the spike was
merged it was ported into `beats.rs`, in the project's own style and on
the `realfft` crate the chroma already used. A comparison program ran
both on all 38 test recordings: the same tempo and the same beats on
every one, about ten times faster, and 285 lines fewer in `Cargo.lock`.
A crate that proves an approach and is then replaced by a small port is
a good outcome for a spike.

### Meter: where the bars are

Chords mostly change on the first beat of a bar. `meter::guess` counts,
for each way of grouping beats (in threes or fours, starting on any
beat), how many chord changes land on a group's first beat, and keeps
the grouping that beats chance by the most. A small head start goes to
4/4, since most popular music is in four. It told 3/4 from 4/4 on all 29
demos and found the right downbeat on 22.

### Arranging

`arrange::follow` builds the project. The chords become *spans* (one
chord held over consecutive beats) in song beats, voiced with the same
`library::voicing::voice` the styles use, so each chord moves as little
as possible from the last. They go into patterns of 16 bars, laid one
after another on a "Harmony" track; a span that crosses a pattern
boundary is cut in two.

The recording is a sampler channel with one note, at the first step of
the song. The sampler plays a file from its start at its own speed, so
the recording's first downbeat must fall on a bar line of the song. It
rarely does, so `arrange::write_wav` saves the recording again with
silence in front, the *lead-in*: exactly enough for the first downbeat to
land on the start of the next whole bar. The copy sits next to the
original as `song.ferrous-loom.wav`.

One change outside the new crate: a sampler stopped playing to the end
of its sound whatever happened, which is right for a drum hit and wrong
for a three-minute song. `Sampler::release_all` now fades out voices
whose sound is longer than ten seconds when playback stops.

## The import in the app

`app/import.rs` follows `app/export.rs` exactly: the work runs on a
thread, the app holds its `JoinHandle`, and every frame `check_import`
asks `is_finished()` and, when it is, `join()`s the thread to get its
result, a `Result<(Project, String), String>`. The UI thread never
blocks, so the music keeps playing while a new recording is being
listened to. The project is swapped in with the same `replace_project`
that File > Open uses.

Opening an audio file from the command line or by dropping it on the
window goes through the same place as a `.loom` file, `open_now`, which
now checks `loom_listen::is_audio_file` first.

## Choosing the crates

The useful question for a spike is not "does it work" but "what works
best, and how do you know". For that, a small program rendered 29 demo
songs from the library to WAV with their true tempo, key and chords
written beside them (Ferrous Loom can export WAV, and
`Project::chord_notes_in_song` names the chord at any step), and ran
every candidate on them, plus seven Creative Commons recordings with
librosa as the reference. The crate's README has the tables. In short:

- **Beats**: `beat-track-rs` (the Ellis beat tracker, with librosa's
  parameters) put its beats on the grid in 27 of 29 songs and takes
  about a second. `stratum-dsp` finds the tempo about as often but its
  beat grid drifts. Half- and double-tempo confusions happen to all of
  them, as they do to every tempo tracker. The winner was then ported
  into `beats.rs`, as described above.
- **Key**: `bpm-key` was right 24 times out of 29, `stratum-dsp` 13.
- **Chords**: no crate did well enough, hence our own 300 lines.

## Exercises

1. The chord templates cover six qualities. Add `Sus4` and see what it
   does to the score on a recording with a sus chord in it: more
   candidates means more ways to be wrong, so the penalty for rare chords
   may need to grow.
2. `meter::guess` knows 3 and 4. Add 6/8: six beats in two groups of
   three, where chord changes land on beats 1 and 4.
3. The import saves a copy of the recording with a lead-in. Give the
   sampler a start offset instead (a field on `SampleSource::File`, and
   a starting `position` in `Sampler::note_on`) and delete `write_wav`.
