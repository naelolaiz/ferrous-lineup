# Project file format

A Ferrous Lineup project is a text file in **RON**, "Rusty Object
Notation", which looks like Rust's own struct and enum syntax. The app
writes it through `serde`, reads it through the same types, and saves it
with the extension `.lineup`. A file with the old `.loom` extension
still opens: that was the extension while the app was called Ferrous
Loom ([A new name](step-9/new-name.md)). Saving a `.loom` file keeps its
name.

```text
(
    format: 27,
    tempo: 130.0,
    scale: (root: C, kind: Major),
    channels: [
        (name: "Kick", kind: Sampler(Builtin(Kick)), volume: 0.8, muted: false),
        // ...
    ],
    // ...
)
```

[Saving and loading projects](step-4/saving.md) introduces RON, serde
and the `ProjectFile` struct that stands between the file and the live
`Project`. This page is the reference for the number at the top:
`FORMAT_VERSION` is **27** in
[`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs).

## The format number

The number says which features a file may contain. It is not the app's
version: 0.1 and 0.2 both write format 1 when the song uses nothing new.

`format_for` writes the **lowest** format that can hold the project, not
always 27. A plain project stays format 1, so an older Ferrous Lineup
can still open it. A song with a chord sheet but no band is format 12;
a band is format 13; a locked channel is at least format 23. The number
is the highest feature in the song, and features are cumulative.

Later formats added fields with serde defaults, so the text of an older
song is unchanged. A field at its default is left out, and a missing
field gets its default when the file is read. That is how the format
grew without a migration step: old files are simply old, and new files
carry only what they need.

| Format | What it added | What an older version would get wrong | Where it is explained |
|---:|---|---|---|
| 1 | The first layout: channels, patterns, tracks, clips, tempo and key. | Nothing older exists; this is the floor. | [Saving and loading projects](step-4/saving.md) |
| 2 | Channels that play an external MIDI synth. | It would not know the `MidiOut` channel and would not send the notes out. | [Live keys](step-7/live.md), [MIDI Out for every channel](step-8/midi-out-every-channel.md) |
| 3 | Nine built-in drums, patterns longer than four bars, and time signatures. | It would not know the drums, would cut long patterns short without a word, and would draw every bar in four. | [Time signatures](step-8/time-signatures.md), [The library](step-8/library.md) |
| 4 | Sections, clips that repeat, and a loop region. | It would play every repeat once and lose the loop. | [Song structure](step-8/song-structure.md) |
| 5 | Time markers, which change the time signature partway through a song. | It would draw every bar in the song's first time signature. | [Changing time signature partway through](step-8/time-markers.md) |
| 6 | A twelve-tone row or a pitch-class set as the key. | It would not know either kind of key and could not open the file. | [Twelve-tone rows and pitch-class sets](step-8/twelve-tone.md) |
| 7 | Tempo changes through a song, and loose timing. | It would play the whole song at its first tempo and put every note on the grid. | [Tempo changes, and playing like a person](step-8/tempo-and-feel.md) |
| 8 | Key changes through a song. | It would show and play the whole song in its first key. | [Changing key](step-8/changing-key.md), [Key changes partway through](step-8/key-changes.md) |
| 9 | A General MIDI instrument on a channel. | It would play every Lineup Osc with the same sound and send no program changes. | [Every voice on its own channel](step-8/voices-and-instruments.md) |
| 10 | Notes that start or end between steps, such as an imported MIDI file's triplets. | It would move them onto the grid and change the rhythm. | [Opening MIDI files](step-8/midi-import.md) |
| 11 | Beats grouped irregularly, such as 7/8 counted 2+2+3. | It would read 7/8 but draw a beat line on every eighth note. | [Any time signature, and irregular meters](step-8/any-time-signature.md) |
| 12 | The chord sheet: the song's harmony, bar by bar. | It would lose the sheet without a word when saving. | [The chord sheet](step-9/chord-sheet.md) |
| 13 | The band that plays the chord sheet, and the patterns it writes. | It would keep the band's patterns as plain patterns and stop following the sheet. | [The band plays the chord sheet](step-9/band.md) |
| 14 | The band's song form: intro, repeated choruses and an ending. | It would play the song once, as written, with no repeats or ending. | [Song form](step-9/song-form.md) |
| 15 | The style's second variation, chosen by markers on the chord sheet. | It would play the whole song in the first variation. | [Variations a and b](step-9/variations.md) |
| 16 | Swing, the eighth notes played long-short. | It would play every pair of eighths evenly. | [Swing](step-9/swing.md) |
| 17 | The band's guitar, strings and organ. | It would know only piano, bass and drums, and could not read their parts. | [A bigger band](step-9/bigger-band.md) |
| 18 | Shots, holds, rests and pushes on the chord sheet. | It would drop the marks and play the style straight through. | [Shots, holds, rests and pushes](step-9/stops.md) |
| 19 | The band's soloist. | It would know no soloist player or soloist part. | [The soloist](step-9/soloist.md) |
| 20 | A band's rhythm separated from its instruments, the exact harmony style, and plain accents. | It would use one style for every part and lose the accents-only setting. | [An orchestra for music that changes meter](step-9/orchestra.md), [Styles keep the song's tempo](step-9/style-tempo.md) |
| 21 | The saved master mixer, solo and stereo reverb. | It would play the song with the default master and reverb, not the mix you saved. | [The mixer](step-9/mixer.md) |
| 22 | User style patterns and instruments embedded with the band. | It would fall back to the style it knows by name, so the band would play something else. | [The style editor](step-9/style-editor.md) |
| 23 | Locked channels. | It would let styles and the band write over the locked part. | [Locking a track](step-9/locks.md) |
| 24 | A different style for part of a song. | It would play the whole song in the band's own style. | [A style for each section](step-9/section-styles.md) |
| 25 | Two chords at once on the chord sheet, written `Eb7` over `Fb`. | It could not read a polychord, so it could not open the file. | [Hearing chords](step-9/chord-hearing.md) |
| 26 | A pan and a reverb send on each channel. | It would play every channel in the middle with all of it in the reverb. | [The mixer](step-9/mixer.md) |
| 27 | Sending a built-in drum over MIDI. | It would play the drum itself instead of sending it to the external synth. | [Sending a channel over MIDI](step-9/midi-out-menus.md) |

A file can also say where the parts of the song came from, in any
format. Older versions skip that record and lose nothing they could
play.

## Files from the future

A newer Ferrous Lineup may write a format this one has never seen. Such
a file may contain names and fields this version cannot parse, and even
if it parses, the meaning may have changed. So the reader checks the
format number and refuses anything above `FORMAT_VERSION` with:

```text
this file uses format 28, made by a newer version of Ferrous Lineup (this one reads up to format 27)
```

It checks after reading the whole file, so a RON error in a future file
is explained by its format number rather than by a puzzling missing
field. A file that fails to parse is also scanned for the number at its
start, without a full RON read; that keeps a long song from taking a
minute to report the real problem.

## Saving safely

`file::save` writes to a temporary file next to the target, then renames
it. A crash or a full disk therefore leaves the old project intact, and
a half-written file is never mistaken for a saved one. The temporary
ends in `.lineup.tmp` (or `.loom.tmp` for an old name) and is removed
if the save fails. Line endings are always `\n`, whatever the system,
and a byte-order mark at the start of a file is ignored when reading.

The same pattern is used by WAV and video export, and by the recovery
copy. See [Troubleshooting](troubleshooting.md) if an export fails.
