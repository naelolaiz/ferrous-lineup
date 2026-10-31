# Project file format

A Ferrous Lineup project is a text file in **RON**, "Rusty Object
Notation", which looks like Rust's own struct and enum syntax. The app
writes it through `serde`, reads it through the same types, and saves it
with the extension `.lineup`.

```text
(
    format: 31,
    tempo: 130.0,
    scale: (root: C, kind: Major),
    channels: [
        (name: "Kick", kind: Sampler(Builtin(Kick)), volume: 0.8, muted: false),
        // ...
    ],
    // ...
)
```

[Saving](../program/saving.md) explains RON, serde and the `ProjectFile`
struct that stands between the file and the live `Project`. This page is
the reference for the number at the top: `FORMAT_VERSION` is **31** in
[`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs).

## The format number

The number says which features a file may contain. It is not the app's
version.

`format_for` writes the **lowest** format that can hold the project, not
always 31. A plain project stays format 1, so any version of Ferrous
Lineup can open it. A song with a chord sheet but no band is format 12;
a band is format 13; a locked channel is at least format 23. The number
is that of the highest feature the song uses, and each format can hold
everything the ones below it hold.

Every field a format adds has a serde default. A field at its default
is left out when writing, and a missing field gets its default when
reading. So a song that uses nothing new is written exactly as before,
and a file needs no conversion step when it is opened: it carries only
what it uses.

| Format | What it adds | What a version that reads only the formats below would get wrong | Explained in |
|---:|---|---|---|
| 1 | The base: channels, patterns, tracks, clips, tempo and key. | Nothing; every version reads it. | [Saving](../program/saving.md) |
| 2 | Channels that play an external MIDI synth. | It would not know the `MidiOut` channel and would not send the notes out. | [MIDI](../sound/midi.md#midi-out-an-instrument-with-an-outbox) |
| 3 | Nine more built-in drums, patterns longer than four bars, and time signatures. | It would not know the drums, would cut long patterns short without a word, and would draw every bar in four. | [Tempo, meter, swing and feel](../sound/tempo.md#timesignature-two-numbers-and-a-grouping) |
| 4 | Sections, clips that repeat, and a loop region. | It would play every repeat once and lose the loop. | [The project](../program/project.md#sections-repeats-and-loops) |
| 5 | Time markers, which change the time signature partway through a song. | It would draw every bar in the song's first time signature. | [Tempo, meter, swing and feel](../sound/tempo.md#meter-time-signatures-that-change) |
| 6 | A twelve-tone row or a pitch-class set as the key. | It would not know either kind of key and could not open the file. | [Twelve-tone rows](../theory/twelve-tone.md#a-row-as-the-key), [Pitch-class sets](../theory/sets.md#a-key-as-a-set-the-same-notes-another-name) |
| 7 | Tempo changes through a song, and loose timing. | It would play the whole song at its first tempo and put every note on the grid. | [Tempo, meter, swing and feel](../sound/tempo.md#tempo-changes) |
| 8 | Key changes through a song. | It would show and play the whole song in its first key. | [Scales, modes and keys](../theory/scales.md#key-changes-partway-through-key-markers) |
| 9 | A General MIDI instrument on a channel. | It would play every Lineup Osc with the same sound and send no program changes. | [SoundFonts](../sound/soundfonts.md#a-channels-instrument) |
| 10 | Notes that start or end between steps, such as an imported MIDI file's triplets. | It would move them onto the grid and change the rhythm. | [MIDI](../sound/midi.md#notes-between-steps) |
| 11 | Beats grouped irregularly, such as 7/8 counted 2+2+3. | It would read 7/8 but draw a beat line on every eighth note. | [Tempo, meter, swing and feel](../sound/tempo.md#beats-of-different-lengths) |
| 12 | The chord sheet: the song's harmony, bar by bar. | It would lose the sheet without a word when saving. | [The chord sheet](../band/chord-sheet.md#in-the-file) |
| 13 | The band that plays the chord sheet, and the patterns it writes. | It would keep the band's patterns as plain patterns and stop following the sheet. | [How the band plays](../band/band.md#in-the-file) |
| 14 | The band's song form: an intro, repeated choruses and an ending. | It would play the song once, as written, with no repeats or ending. | [Song form](../band/song-form.md#the-form-an-intro-choruses-and-an-ending) |
| 15 | The style's second variation, chosen by markers on the chord sheet. | It would play the whole song in the first variation. | [Song form](../band/song-form.md#variations-a-and-b-the-band-digs-in) |
| 16 | Swing, the eighth notes played long-short. | It would play every pair of eighths evenly. | [Tempo, meter, swing and feel](../sound/tempo.md#swing) |
| 17 | The band's guitar, strings and organ. | It would know only piano, bass and drums, and could not read their parts. | [How the band plays](../band/band.md#a-bigger-band-guitar-strings-and-organ) |
| 18 | Shots, holds, rests and pushes on the chord sheet. | It would drop the marks and play the style straight through. | [Shots, holds, rests and pushes](../band/stops.md#two-optional-marks-on-a-marker) |
| 19 | The band's soloist. | It would know no soloist player or soloist part. | [The soloist](../band/soloist.md) |
| 20 | A band's rhythm taken from one style and its instruments from another, the exact harmony style, and accents alone. | It would use one style for every part and lose the accents-only setting. | [How the band plays](../band/band.md#a-band-in-the-song), [The orchestra](../band/orchestra.md) |
| 21 | The master mixer, solo and stereo reverb. | It would play the song with the default master and reverb, not the mix you saved. | [The mixer](../sound/mixer.md#what-is-saved) |
| 22 | Your own styles, saved with the band. | It would fall back to the style it knows by name, so the band would play something else. | [Your own styles](../band/style-editor.md#the-style-travels-with-the-song) |
| 23 | Locked channels. | It would let styles and the band write over the locked part. | [Song form](../band/song-form.md#locking-a-track-styles-play-around-it) |
| 24 | A different style for part of a song. | It would play the whole song in the band's own style. | [Song form](../band/song-form.md#a-style-for-each-section) |
| 25 | Two chords at once on the chord sheet, written `Eb7` over `Fb`. | It could not read a polychord, so it could not open the file. | [Hearing harmony](../theory/hearing.md#two-chords-at-once) |
| 26 | A pan and a reverb send on each channel. | It would play every channel in the middle with all of it in the reverb. | [The mixer](../sound/mixer.md#pan-two-speakers-one-channel) |
| 27 | Sending a built-in drum over MIDI. | It would play the drum itself instead of sending it to the external synth. | [MIDI](../sound/midi.md#one-channel-from-its-instrument-menu) |
| 28 | A horn section in the band, and EQ, compression and delay on a channel. | It would play no horns, and every channel without its effects. | [How the band plays](../band/band.md#a-horn-section), [The mixer](../sound/mixer.md#each-channels-effects) |
| 29 | A band playing the drums of another style than its chord and bass parts, or leaving the drums to the song. | It would play the drums of the band's own style. | [How the band plays](../band/band.md#a-band-in-the-song), [The library](../band/library.md#which-parts-to-take) |
| 30 | A band playing the chord sheet in a micromode style. | It would play the forms as chords and a bass line. | [Micromodes](../theory/micromodes.md#in-a-song-the-band-plays-the-forms-on-the-sheet) |
| 31 | Takes: a sampler channel playing a voice recorded along with the song, `Sampler(Take(...))` with the path of its WAV file. | It could not read a take's channel, so it could not open the file. | [Takes](../theory/listen.md#writing-it-in) |

Two records can be in a file of any format, because a version that
skips them still plays the song exactly. One says where each part of the
song came from ([The library](../band/library.md#where-each-part-came-from)).
The other is a key written the other way round, such as G♭ major rather
than F♯ major: the key is saved as `(root: FSharp, kind: Major,
respelled: true)`, and a version that does not know `respelled` reads
it as F♯ major, with the same notes ([Scales, modes and
keys](../theory/scales.md#keys-that-can-be-written-two-ways)).

## Files from a newer version

A newer Ferrous Lineup may write a format this one has never seen. Such
a file may contain names and fields this version cannot read, and even
if it reads, their meaning may have changed. So the reader checks the
format number and refuses anything above `FORMAT_VERSION`:

```text
this file uses format 32, made by a newer version of Ferrous Lineup (this one reads up to format 31)
```

The whole file is read first, and the number checked after. When the
text cannot be read at all, the reader looks for the number at its
start without reading the rest, so a file from a newer version is
explained by its format number rather than by a puzzling unknown field
([Saving](../program/saving.md#the-version-number)).

## Saving safely

`file::save` writes to a temporary file next to the target, ending in
`.lineup.tmp`, then renames it. A crash or a full disk therefore leaves
the old project as it was, and a half-written file is never mistaken for
a saved one; if the save fails, the temporary file is removed. Line
endings are always `\n`, whatever the system, and a byte-order mark at
the start of a file is ignored when reading ([Saving without losing the
old file](../program/saving.md#saving-without-losing-the-old-file)).
