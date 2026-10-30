# Sounds

Ferrous Lineup can play its instruments in two ways:

- with its own synthesizer, the **Lineup Osc**, which builds every
  sound from a few simple waves. It needs nothing else, and it is what
  plays until you choose otherwise;
- with a **SoundFont**, a file of recordings of real instruments: a
  piano, a double bass, a bandoneón, a choir, a drum kit. This is how
  the band should sound.

## The first Play

The first time anything plays, when you press Play or open a demo
(which starts playing at once), a window called **Real instrument
sounds** offers to download **GeneralUser GS**, a free SoundFont by
S. Christian Collins, of about 30 MB, and asks "Play the instruments
with it?". The song goes on playing while it asks.

![The first Play: a demo plays on while the Real instrument sounds window offers GeneralUser GS and asks "Play the instruments with it?", with Download, Choose a file… and Not now](images/soundfont-offer.png)

- **Download** fetches it once and keeps it on your computer. A bar
  shows how far it is, and **Stop** gives up. When it is done, the
  status bar says "Instruments play from GeneralUser GS", and every
  instrument plays its recording from then on, next time too.
- **Choose a file…** uses another SoundFont you already have, any
  `.sf2` file made for General MIDI.
- **Not now** keeps the Lineup Osc. Play does not ask again; the
  setting below is there when you change your mind.

If GeneralUser GS is already on your computer, downloaded by an earlier
version for instance, the window says so and offers **Use it** instead
of **Download**: a moment later the instruments play from it. If you
close the app without answering, it asks again the next time anything
plays. Your answer and the sound settings below stay when you update
Ferrous Lineup.

If the download fails (no network, a firewall), the window says why.
You can download `GeneralUser-GS.sf2` from [its
page](https://github.com/mrbumpy409/GeneralUser-GS) with a browser and
pick it with **Choose a file…**.

## The sound settings

**Edit > Settings…**, under **Audio**, holds the same choices:

| Setting | What it does |
|---|---|
| **SoundFont instruments** | Channels with an instrument play the SoundFont's recording of it. Untick it to go back to the Lineup Osc. |
| **SoundFont drums** | The drum kit plays the SoundFont's drums too: a recorded kick, snare and hi-hat. |
| **Choose SoundFont…** | Plays another `.sf2` file instead. |
| **Use GeneralUser GS** | Goes back to GeneralUser GS after you chose another file. |

The Audio page also says which output the sound goes to, and has
**Restart audio** for when the sound card stops (see
[Troubleshooting](troubleshooting.md)).

WAV and video exports, the computer keyboard and a MIDI keyboard all
play the SoundFont too.

## An instrument for each channel

Every channel that plays notes has an instrument, one of the 128 of
**General MIDI**, the standard list of instruments that every
synthesizer and SoundFont numbers the same way, from the grand piano to
the gunshot. The band's players, the Library's piano and bass,
and every voice of every demo come with the right one.

To change a channel's instrument:

- in the **channel rack**, right-click the channel's name and open
  **Instrument**, or
- in the **mixer**, click the instrument's name on the channel's line.

The instruments are grouped as General MIDI groups them: pianos,
organs, guitars, basses, strings, brass, reeds and so on. **None (the
Lineup Osc's own sound)** gives the channel back to the Lineup Osc,
whatever the SoundFont setting says.

The same menu can also send the channel to an external synthesizer
over MIDI: see [MIDI keyboards and synths](midi.md).

## Your own sounds

The channel rack also plays sounds of your own. Drag a WAV file from
your file manager onto a channel of the rack, and that channel plays it
on each step you light, as a drum machine plays its samples. See [The
channel rack](channel-rack.md).
