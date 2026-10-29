# Practising

A band that plays your chords is most useful when you play along. The
chord sheet's **practice row** has what a musician reaches for when
practising with a recording: a count-in, a metronome, a slower speed,
tap tempo, looping the bars you are working on, a speed trainer, and
taking a part away from the band to play it yourself. The **ear
training** window then trains you to hear harmony without looking.

![The chord sheet of the demo Classical cadence with its practice row: Count-in and Metronome lit, Speed 70%, Speed trainer lit with "from 70% to 100% by 5% every 1 pass", and "Loop bars 3–4" lit next to "Stop looping". Bars 3 and 4 are shaded, with a green line along their top. The toolbar shows TEMPO 70% and the playlist "Looping bars 3–4", and the status bar says "Speed trainer: 70%, then 5% faster each time the loop comes round, up to 100%".](images/practising.png)

## Count-in and metronome

- **Count-in**: when you press Play, one bar of clicks comes first, and
  the music starts on the next "one". Start in the middle of a bar and
  the count runs on to where you start, so the beat never jumps.
- **Metronome** (**Ctrl+M**): a click on every beat while the music
  plays. The first beat of each bar is higher, so you always know where
  "one" is.

The metronome clicks the beats you feel, not every note of the time
signature: twice a bar in 6/8, three times in 2+2+3/8 (see [Rhythm and
meter](rhythm-and-meter.md)). Both are in the toolbar too, so they work
with any song or pattern, and both are remembered for next time.

## Speed and tap tempo

**Speed** plays the band slower or faster than the song, from 25% to
200%. The song keeps its own tempo: the **TEMPO** box still shows it,
with the speed over it, as in "TEMPO 70%", and a saved song or an
exported WAV is at full speed. The speed goes back to 100% each time the
app starts, so a song is never slow by surprise.

**Tap** sets the song's tempo to yours: click it on the beat a few
times, in time with a recording or with the tempo in your head.

## Looping bars

Click a bar, **Shift+click** another (or hold **Shift** and press an
arrow key), and click **Loop bars 3–4**: those bars play over and over.
They are shaded on the sheet, and the playlist's toolbar says
**Looping bars 3–4**. **Stop looping** plays the whole song again from
where you are.

## The speed trainer

A passage you can't play at full speed is learned the same way on every
instrument: slowly until it's right, then a little faster, and again.
The **Speed trainer** does it for you. Click it, and four boxes appear
beside it:

- **from**: the speed Play starts at;
- **to**: the speed to get to. 100% is the song's tempo; 110% goes past
  it, to have some room to spare;
- **by**: how much faster each new speed is;
- **every**: how many times round at each speed before the next one.
  With **every 2 passes** you play each speed twice: once to get it,
  once to check you have it.

Press Play, and each time the loop comes round the band plays a little
faster, until it reaches the speed you are aiming for and stays there.
The loop is whatever playback goes round: the bars you loop, the whole
song when nothing is looped, or the pattern in pattern mode. **Stop**
(⏹) sets the speed back to where it started, ready for the next run;
**Pause** (❚❚) keeps it, so Play carries on at the speed you got to.
Clicking **Speed trainer** again turns it off and leaves the speed where
it got to.

The trainer's numbers are remembered, and **Edit > Settings… >
Playback** has them too, next to the count-in, the metronome and the
practice speed.

## Playing a part yourself

**You play**, in the same row, takes one player away from the band so
you can play that part, with your keyboard switched to that player's
sound. See [Locking a track, and playing your part](locks.md#you-play).

## Ear training (experimental)

Musicians need to *hear* harmony too: to tell a minor third from a
major one, or the V of a song from its IV, without looking. **View > Ear
training** opens a window that plays something and asks you to name
it, and tells you at once whether you were right, and why:

![The Ear training window on its Intervals tab: the intervals m3, M3, P4, P5 and P8 chosen, played Up. The answer P5 is red and M3 green, and the box says "Not quite. You said perfect fifth. E4 up to G#4: a major third, 4 semitones. The bottom of a major chord: from its root up to its third. Like the first two notes of When the Saints Go Marching In." The score line says "Right 0 of 1 (0%) · 0 in a row".](images/ear-training.png)

It has three tabs:

- **Intervals**: two notes play, and you name the interval. Choose
  which intervals can come up and how they are **Played**: **Up**,
  **Down** or **Together**. Start with a few that sound very different,
  such as a third, a fifth and the octave, and add the others one at a
  time.
- **Chords**: a chord plays, and you name its kind: major, minor,
  diminished, augmented, or one of the seventh chords. With
  **Inversions**, its lowest note is not always the root; **Note by
  note** plays it one note at a time from the bass up.
- **Progressions**: the band plays a few chords in a key, one bar
  each, and you name them by their numerals on a row of harmony keys
  with the names hidden. The first chord is always I, so you know where
  you start, and **Hear the key** plays a cadence that says where home
  is. Choose the **Keys**, the chords that can come up, **Sevenths**,
  the **Players**, the **Length** and the **Tempo**.

After you answer, the right answer turns green, a wrong one red, and a
box explains it: the notes, the full name, how many semitones and, for
an interval going up, a tune that starts with it. Click any answer
button to hear it next to the right one. **▶ Play again** repeats the
question as often as you like, and **Next ▸** asks another (before you
answer it is **Skip ▸**, and a skipped question doesn't count). The line at the bottom keeps the score,
for each tab; **Start again** sets it back to zero.

The window changes nothing in the song, and the song pauses while a
question plays.
