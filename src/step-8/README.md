# Step 8: Harmony keys, a library of styles, and demos

Pattern-based studios give you a row of big chord keys: one for each
chord of the key you are in, so you can play a progression with one
finger and *see* how harmony works. Step 8 adds them to Ferrous Loom,
and a library to go with them:

- **the harmony keys**, along the bottom of the main window. On the
  left, the project's **key** and **time signature**, always in view.
  In the middle, seven big keys, **I** to **vii°**, one for each chord
  of the key. They are coloured by what the chord *does*: blue for
  home (tonic), green for leaving home (subdominant), orange for
  tension (dominant). Hold one down, with the mouse or the number keys
  **1** to **7**, and it plays its chord on the selected channel; with
  **● Rec** on, it is recorded. The chords that often come next light
  up with a yellow outline, and the line below says why. On the right,
  a keyboard shows the notes of the scale and the notes playing,
- **chords followed as the music plays**: while a pattern or the song
  plays, the harmony key of the chord playing lights up, and the
  keyboard shows its notes in blue. Press Play on any demo and watch
  the progression move between home, away and tension,
- **time signatures**: 2/4, 3/4, 4/4, 5/4, 3/8, 5/8, 6/8, 7/8, 9/8 and
  12/8. The playlist's bars, the snap, the channel rack's beat colours,
  the piano roll's lines and the pattern lengths all follow it, and it
  goes into project files and MIDI files,
- **a library** in a new **Library** tab next to the pattern list:
  **78 harmony styles** and **49 rhythms** from 13 genres, from pop and
  the blues to bossa nova, flamenco, reggae and, of course, **tango**
  (marcato, síncopa, milonga, habanera and the tango vals). Click a
  style to hear it in your key, click another to swap, and **Ctrl+Z**
  takes it back. Hover over a style to see its chords in your key,
- **demos** under **File > Demos**: **16 well-known songs** that are in
  the public domain, from *Ode to Joy* and *Für Elise* to *Korobeiniki*
  and *When the Saints Go Marching In*, each with its melody, chords,
  bass and drums. And every harmony style as a short song,
- **nine new drums**, made by the synthesizer like the first four:
  open hi-hat, rim, low and high toms, ride, crash, cowbell, shaker and
  conga,
- **patterns up to sixteen bars long**, so a whole twelve-bar blues fits
  in one pattern.

![The harmony keys while the demo song plays. F (IV) is held down, in orange on the keyboard; the song is playing A minor, in blue. After IV, the keys outlined in yellow (I, ii and V) often come next.](../images/step-8-harmony-keys.png)

![The Library tab. Bossa nova has just been tried: it is playing in its own pattern, and its tooltip shows its chords in C major. The harmony keys follow it: Dm7 is playing, so ii is lit, and V and vii° are outlined as the chords that often come next.](../images/step-8-library.png)

![File > Demos: the starting song, the songs, and every style by genre](../images/step-8-demos.png)

## How to use it

| Do this | To |
|---------|----|
| hold a harmony key, or **1** to **7** | play its chord on the selected channel |
| Triads, 7ths, 9ths (under the key) | play three-, four- or five-note chords |
| Key, Time (left of the harmony keys) | change the project's key and time signature |
| **F8**, or the 🎼 Harmony button | show or hide the harmony keys |
| ● Rec, Space, then hold harmony keys | record chords into the pattern |
| Library tab, click a style | play it in a new pattern, in your key |
| click another style or rhythm | swap it in the same pattern |
| Ctrl+Z | take it back |
| Use their tempo (Library tab) | also set the style's usual tempo |
| File > Demos | open a song or a style as a whole project, and play it |

The number keys play the harmony keys only while the typing keyboard is
off, because the typing keyboard uses them for black keys.

Once you change a pattern the library wrote (add a note, rename it),
it is yours: the next click in the library starts a new pattern instead
of replacing it.

## How it was tested

- The library is data, and its tests read every entry: every chart and
  melody parses, every bar adds up to the time signature, every style
  and rhythm can be written into a project, every demo survives saving
  and loading, and at least 80% of every melody's notes are in its key
  (the rest are passing notes).
- Every one of the 94 demos (16 songs and 78 styles) was rendered
  offline, through the same audio code as the WAV export, and measured:
  none clips, and the loudest peaks at 0.96 of full scale.
- In the running app: the harmony keys with the mouse and the number
  keys, the suggestions, the Library tab (trying, swapping, undoing),
  the Demos menu with its "Save changes?" question, and time
  signatures in the playlist, the rack and the piano roll.
- The time signature in exported MIDI files is checked by a test.

## What changed in the architecture

```text
  loom-core
    meter.rs (new):          TimeSignature: steps per bar and per beat
    harmony/keys.rs (new):   the seven chords of a key, their functions, playing them
    library/ (new):
      numeral.rs     "I V vi IV": chords counted from the tonic
      chart.rs       chord charts by the bar, melodies note by note
      voicing.rs     voice leading: each chord close to the last
      comping.rs     rhythms for chords and bass lines
      style.rs       78 harmony styles
      rhythm.rs      49 rhythms and 6 fills, one text line per drum
      song.rs        16 public-domain songs
      apply.rs       writing styles and rhythms into patterns
      demo.rs        whole demo projects
    project.rs:     the time signature, nine new drums, the chord at a step
    pattern.rs:     MAX_STEPS 256, set_notes
    file.rs:        format 3
    midi_file.rs:   the time signature and the new drums in .mid files

  loom-audio
    drums.rs:       nine new drum sounds

  loom-app
    harmony_keys.rs (new):      the harmony keys bar
    library_browser.rs (new):   the Library tab
    app/library.rs (new):       trying styles and rhythms, the demos
    playlist.rs, channel_rack.rs, piano_roll.rs:  bars and beats of any time signature
```

Three ideas carry this step:

1. **Knowledge as data.** A musician carries a lot in their head: which
   chords make a bossa nova, where the snare goes in a reggae beat, how
   *Greensleeves* goes. The library writes all of it down as short
   strings and tables that a person can read and check, and a few small
   functions turn them into notes. Adding a style is adding a few lines,
   not writing code.
2. **Relative, not absolute.** Styles are written in Roman numerals and
   rhythms in steps of a bar, so the same table plays in any key, and a
   rhythm keeps its place in any time signature.
3. **The library only writes patterns.** Trying a style is an ordinary
   edit through `LoomApp::edit`, so undo, saving, the audio thread and
   everything else work without knowing the library exists.

## Files in this step

```text
crates/loom-core/src/meter.rs               TimeSignature (new)
crates/loom-core/src/harmony/keys.rs        chord_keys, Function, ChordSize, play (new)
crates/loom-core/src/harmony/scale.rs       Scale::ninth
crates/loom-core/src/library/*.rs           the library (new)
crates/loom-core/src/project.rs             time_signature, new drums, chord_notes_in_song
crates/loom-core/src/pattern.rs             MAX_STEPS, set_notes
crates/loom-core/src/file.rs                format 3
crates/loom-core/src/midi_file.rs           time signature meta event, new drum notes
crates/loom-audio/src/drums.rs              open hat, rim, toms, ride, crash, cowbell, shaker, conga
crates/loom-app/src/harmony_keys.rs         the harmony keys bar (new)
crates/loom-app/src/library_browser.rs      the Library tab (new)
crates/loom-app/src/app/library.rs          auditions and demos (new)
crates/loom-app/src/app.rs                  the bar, the chord playing, F8
crates/loom-app/src/app/files.rs            File > Demos
crates/loom-app/src/app/live.rs             play_typed is shared with the harmony keys
crates/loom-app/src/playlist.rs             the Library tab, bars of any length
crates/loom-app/src/channel_rack.rs         beat colours by time signature
crates/loom-app/src/piano_roll.rs           bar and beat lines by time signature
```

The chapters start with the harmony keys, the part you see first. Then
time signatures, which everything else depends on. Then the library:
how a style, a rhythm and a song are written as data, and how they
become patterns and demos.
