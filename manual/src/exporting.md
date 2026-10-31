# Exporting

To take a song out of Ferrous Lineup, the **File** menu writes it as a
sound file, a video, a MIDI file or a text chord sheet. A song with
nothing in the playlist has nothing to export: the items are greyed out,
with **Put some clips in the playlist first**.

| Item | Writes |
|---|---|
| **Export WAV…** | the song's sound, to play anywhere |
| **Export video…** | the sound, with the chord sheet or other views playing along |
| **Export song as MIDI…** | the notes of the whole song, for another music program |
| **Export pattern as MIDI…** | the notes of the selected pattern |
| **Export chord sheet…** | the chords as text, to share or paste anywhere |

Each asks where to save, and adds the file's ending if you leave it out.

## A sound file

**Export WAV…** saves what you hear when the song plays, from start to
end, as a WAV file: CD quality (44.1 kHz, 16-bit stereo), with every
repeat, the band, the mixer's volumes, pans, reverb and effects, and a
couple of seconds at the end for the last notes and the room to die
away. It is always at the song's own tempo, even when you practise at a
slower speed.

It is made in the background, faster than the song plays, while you go
on working: the status bar says **Exporting** and the file's name, with
a **Stop** button. Any music player can play the WAV, and any audio
program can turn it into an MP3.

## A video

**Export video…** makes an MP4 video of the song, with views of the app
following the music, ready to post or to show a class:

![The Export video window for the El Choclo demo: Chord sheet and Harmony keys ticked, each with its zoom at 100%, and the preview of the picture under them: the title line "El Choclo", "Bar 1, beat 1", the key A harmonic minor, 4/4 and 112 BPM, the chord sheet in the top box and the harmony keys in the bottom one; then Stack, Preview at, the sizes 1280 × 720 and 1920 × 1080, and Export…](images/export-video.png)

- **The views.** Tick the ones to show: **Chord sheet** (the bar
  playing framed), **Lead sheet** (the tune on a staff with its chords),
  **Playlist**, **Piano roll** and **Harmony keys** (the chord playing
  lit). Each has a **zoom**, from 50% to 250%; the lead sheet instead
  grows with its box.
- **The picture.** The preview shows one frame. Drag a view to move it,
  drag its corner to make it bigger or smaller, and scroll over it to
  zoom. **Stack** puts the views one above the other again. **Preview
  at** shows the picture at another place in the song.
- **Size**: **1280 × 720** or **1920 × 1080**.

A line along the top names the song, and shows the bar and beat, the
key, the time signature and the tempo as they change. The video is an
MP4 file, with H.264 pictures, 30 a second, and MP3 sound, which plays
almost anywhere; nothing else needs to be installed to make it. The
window remembers your choices for next time.

## A MIDI file

**Export song as MIDI…** writes the notes of the song, for another
music program, a notation program, or a synthesizer:

- each channel gets a track of its own, with its General MIDI
  instrument, so the file opens with the right sounds;
- the drums go on channel 10, as General MIDI drums; a channel you send
  over MIDI keeps its channel, and the others take 1, 2, 3 and so on;
- every repeat is written out, with the song's swing in the notes, and
  the tempo, time signature and key are written as they change;
- muted tracks and channels, and channels silenced by a solo, are left
  out, as they are when the song plays. The mixer's volumes, pans and
  effects are not written: the program that opens the file makes its
  own mix.

**Export pattern as MIDI…** writes only the selected pattern, from its
first step: a beat or a part to use somewhere else.

Ferrous Lineup can open the file again (see [Opening MIDI files and
chord sheets](importing.md)), and keeps the exact key it was in.

## A chord sheet as text

**Export chord sheet…** writes the song's chord sheet as a text file,
bar by bar, as in `| C | Am7 | Dm7 G7 |`. Paste it in a message, print
it, or open it again with **File > Import chord sheet…** (see [The
chord sheet](chord-sheet.md#chord-sheets-as-text)). It always uses
letter names, which every musician can read.

## Lead sheets and printing

The **lead sheet**, the tune on a staff with its chords, is printed or
saved as a PDF from its own window, in concert pitch or for a B♭, E♭ or
F instrument: see [Lead sheets](lead-sheets.md).
