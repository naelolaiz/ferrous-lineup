# MIDI keyboards and synths

**MIDI** is how electronic instruments and computers pass notes to each
other: a keyboard says which key went down and how hard, and a
synthesizer that receives it plays the note. Ferrous Lineup can listen
to a MIDI keyboard, so you play along with the band or record into a
pattern, and it can send its notes to a synthesizer outside the app.

You don't need a MIDI keyboard to play: the computer's own keyboard can
be one.

## Connecting a keyboard

Plug the keyboard in (most connect by USB) and open the **MIDI** menu.
Under **Input: a keyboard to play**, it lists the keyboards it finds:
click yours. The menu looks again each time it opens, so a keyboard
plugged in after the app started shows up too. **None** disconnects it.
The choice lasts until Ferrous Lineup closes.

What you play sounds on the **selected channel**, the one the piano roll
shows: click a channel's name in the channel rack to pick it. Each note
keeps how hard you played it. While you hold keys, the status bar names
what they make, as the piano roll does: the chord, and its number in the
key.

If the menu says **MIDI unavailable**, the system gave the app no MIDI
at all. On Linux, MIDI needs the ALSA sequencer; see
[Troubleshooting](troubleshooting.md).

## The computer keyboard

Tick **MIDI > Typing keyboard to piano**, and the letter keys play the
selected channel, laid out as a piano:

| Keys | Play |
|---|---|
| **Z X C V B N M** | the white keys from middle C (C4) up to B |
| **S D G H J** | the black keys between them, from C♯4 |
| **Q W E R T Y U I O P** | the white keys from the C above (C5) |
| **2 3 5 6 7 9 0** | the black keys between those, from C♯5 |

The keys go by where they are on the keyboard, not by the letter
printed on them, so the piano keeps its shape on any keyboard layout.
While it is on, the number keys play notes instead of the harmony keys.
The keys don't play while you type in a text box or a menu is open, and
**Ctrl** and **Alt** shortcuts such as **Ctrl+Z** still work.

## Recording

The **●** button, next to ▶, records what you play into a pattern:

1. Select the channel to record on, and the pattern to record into.
2. Click **●**. It turns red, and the app switches to pattern mode.
3. Press **Space** to play the pattern, and play along. Each note goes
   into the pattern when you let go of its key.
4. Click **●** again to stop recording.

The pattern goes round and round while you record, and every pass adds
to it: play the bass notes on one pass and the chords on the next. Each
note is moved to the nearest step of the grid, so a note a little early
or late lands on the beat, and a quick tap still makes a note one step
long. How hard you played is kept. A note still held when the pattern
ends is cut there.

A take lasts until the music stops or you click **●**, and **Ctrl+Z**
takes the whole take back. With **Snap notes you play** on (right-click
**Snap to scale** in the piano roll), every key records as the nearest
note of the key. To make recording easier, slow the song down while you
play (see [Practising](practising.md)).

## Playing chords to the band

**MIDI > Accompaniment follows chords** makes the band follow your left
hand, as an arranger keyboard does: while the music plays, the chord
you hold becomes the chord the accompaniment plays. The chords and the
bass line keep their rhythm and move to your chord from their next
note.

You don't need to play the whole chord:

| You hold | The band plays |
|---|---|
| one key | its major chord: C plays C |
| two keys a minor third apart | a minor chord on the lower one: A and C play Am |
| two keys a major third or a fifth apart | a major chord on the lower one |
| two keys a minor seventh apart | a seventh chord on the lower one: G and F play G7 |
| three keys or more | the chord they make |

The last chord stays until you play another, and the status bar says
which chord the accompaniment is in. When playback stops the written
notes come back: the song itself does not change. [The harmony
keys](harmony-keys.md) can play chords to the band too.

## Sending to a synthesizer

Ferrous Lineup can play its channels on a synthesizer outside the app,
a hardware synth or a program, instead of its own sounds.

**One channel at a time.** Right-click a channel's name in the channel
rack, or click its instrument in the mixer, and open **Send over
MIDI**:

- **MIDI channel** picks which of the sixteen channels of MIDI it plays
  on. A General MIDI synth plays drums on channel 10.
- **Output, for every channel sent over MIDI** picks the synth. It is
  the same output as in the MIDI menu.

The menu then says **● Sent over MIDI, channel 2**, for example, and
**Play it here instead** gives the channel back its own sound. **+ Add
channel > MIDI Out (external synth)** in the channel rack adds a
channel that plays only on the synth.

**The whole song.** In the MIDI menu, pick the synth under **Output: an
external synth to play**, and tick **Send every channel**. Every channel
then plays on the synth instead of the speakers, on the channels an
exported MIDI file uses: drums on channel 10 as a General MIDI drum set,
the others on 1, 2, 3 and so on. Each channel's General MIDI instrument
goes with it. Untick it to hear the app's own sounds again.

Nothing is sent while the app has no sound card: the MIDI menu says so.
