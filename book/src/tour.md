# A tour of the app

This chapter is the app's manual on one page: what each window does, and
the keys and menus that work it. It used to be the README of the app's
repository. The chapters of each step tell how every part was built and
why; this one only says how to use it.

## Playing, the playlist and the channel rack

Press **Space** to play the demo song: two bars of a C, Am, F, G chord
progression, then a beat comes in, and a drum fill ends it. The play
button pauses while playing, and Stop goes back to where the playhead was
put. **Home** goes to the start, **End** to the last step, and **Ctrl+Home/End**
also work (⌘ on Mac). These jumps reveal the destination even with
**Follow playhead** off. **Page Up** and **Page Down** skip a bar
back or forward, and clicking the bar numbers moves the playhead. The playlist
arranges patterns into a song: click to place a clip of the selected
pattern, drag clips around, right-click to delete, and double-click a
clip to edit it: melodies and chords open in the piano roll, drums in
the channel rack. Drag a clip's right edge to repeat its pattern. Drag
along the strip above the bar numbers to mark sections (Intro, Verse,
Chorus…), and click a section to rename it, play it several times in a
row, loop it or copy it. Drag along the bar numbers to loop part of the
song. Right-click a track's name to insert a track above or below it,
move it up or down, or delete it. **F6** shows or hides
the channel rack, where you click steps and drop WAV files to use your
own sounds. Its **Play pattern**, **Pause pattern** and **Stop pattern**
controls play the selected pattern directly. While the song plays,
**Play over the song** plays the pattern on top of it instead, from the
next bar and in time with it, until you click it again: a way to try a
pattern out with the rest of the song. The pattern picker switches what
is being edited (and what plays over the song). After clicking the rack
or piano roll, **Space** plays or pauses that pattern; while the song
plays, it stops the song first. Click a channel name to select and hear it;
click it again, click empty rack space, or press **Escape** to deselect.
**Delete** or **Backspace** removes the selected rack channel; Undo restores it.
Double-click its name to open its piano roll. An explicitly selected
channel stays selected even when it has no notes in the current pattern.
**F7** shows the piano roll, on the selected pattern: when
the channel has no notes there, it says so and offers the patterns that
do. Pick a key to highlight its notes,
draw notes or whole chords, and watch the chord names above the grid.
Click an existing note to select it; **Delete** or **Backspace** removes selected
notes. Add or remove notes from the selection with Ctrl+click, or select a range
with a Ctrl+drag box, then move them with the
arrow keys (Alt+↑ stays in the key), invert or arpeggiate them from the
Tools menu, and set velocities in the lane under the grid. The chord
buttons light up with the chords that often come next.
**Ctrl+S** saves the project as a `.lineup` file (⌘S on a Mac), **Ctrl+Z**
undoes any change and **Ctrl+Shift+Z** redoes it, and **File > Export
WAV…** saves the song as a sound file. **File > Open recent** lists the
last ten projects. If the app did not close cleanly, it offers the
recovery copy it wrote at startup; [Losing nothing](step-9/recovery.md)
explains both.

## MIDI

The **MIDI** menu connects a MIDI keyboard, which plays the channel
selected in the channel rack, and an output port for **MIDI Out**
channels, which play an external synth. Any one channel can be sent
there too: its instrument menu, in the mixer or on the rack's
right-click, starts with **Send over MIDI**, where you pick its MIDI
channel (1 to 16) and the output. It keeps its instrument, sent as a
program change, and a drum goes out as its note of a General MIDI drum
set. **Play it here instead** brings back the app's own sound. No MIDI keyboard? Turn on
**Typing keyboard to piano** there, and the letter keys play notes. The
status bar names the chord you hold down, and **● Rec** records what
you play into the pattern while it loops. **File > Export song as
MIDI…** writes a `.mid` file that other studios can open.

## Tempo and feel

The **tempo strip** at the top of the playlist draws the song's tempo as
a line. Click it to change the tempo from that bar on, at once or
gradually over a beat to eight bars, which is how a ritardando (slowing
down) or an accelerando (speeding up) is written; WAV and MIDI exports
play the changes too. **Loose**, next to it, makes notes land a few
milliseconds off the grid, as when a person plays (off by default). The
demos play like a performance: the tempo changes and fermatas their
scores mark, a little slowing down at the end of songs and hymns, and
velocities shaped by the beat and the phrase.

## The harmony keys, the Library and the demos

Along the bottom are the **harmony keys**: the key and time signature
of the project, and a big key for each chord of the key (I to vii°),
coloured by what it does: home, away or tension. Hold one, or press
**1** to **7**, to play its chord; the chords that often come next
light up, and while the music plays, the chord you hear lights up too.
In a minor key, **Edit > Settings… > Harmony** chooses how the numerals
on the third, sixth and seventh degrees are written: bIII bVI bVII,
counted from the major scale as pop and jazz books do, or III VI VII,
counted from the minor scale as classical books do.
The **Library** tab next to the pattern list holds an alphabetical
**Presets** list, combining all 177 harmony presets, 112 rhythm presets
and your saved styles. **Edit > Settings… > Library > Style list** can instead group
them by genre, with a separate **User styles** group. Equivalent
style/rhythm entries are combined; distinct playing patterns remain available. They cover pop, rock, blues, jazz, soul, hip hop,
electronic, Latin, tango, flamenco, reggae, folk and classical music.
Tick boxes under the search keep only the presets like the song: in
its time signature (**In 4/4**), made for about its tempo (**Near 120
BPM**, at most 1.15 times faster or slower) and **With chords** (those
that bring a progression). Their labels show the song's own time
signature and tempo.
Click one to review its copy options, then **Apply** to hear it in your key.
**Ctrl-click** (also ⌘-click on a Mac) applies the previous options immediately.
Right-click and choose **Play the whole
song with this preset** to apply it throughout the song while stopped.
The melody stays in place. **File > Demos** opens
343 songs with their melodies, by genre and in alphabetical order: from
*Ode to Joy*, *Für Elise* and the *Habanera* to tangos, Latin American
songs, swing and big band tunes, dances in odd meters and orchestral
pieces such as *Boléro* and *The Rite of Spring*, plus new tunes written
to show off the blues, tango, flamenco and more. It also plays every
style as a short song. Type a few letters of a title, composer, genre,
key or time signature as soon as the menu opens to search them all.
A song demo opens with the chord sheet in front; **File > New beat**
opens with the channel rack instead.

The Library groups its application options into accompaniment, harmony and timing: **Playing patterns**, **Voices / instruments**, **Time
signature**, **Chord progression**, **Tempo**, and **Swing**.
These appear in the popup when you click a preset, preselected with the
last confirmed options. **Apply** remembers them between runs; **Cancel**
discards popup edits. **Edit > Settings… > Library** shows the defaults as a
compact set of clickable toggles; **Edit defaults…** opens their full editor. Saving defaults
changes the next style application and leaves the current song as it is.
Only the checked components change. Entries marked **+ chords** include a
progression; other entries say **keeps chords** and show the progression
checkbox unchecked and disabled. Applying a preset without chords preserves
the remembered progression choice for the next preset with chords. Chord
progression is off initially; check it to copy a preset's chart as well. An empty audition
uses example chords to demonstrate the preset. The combined list keeps
every original playing pattern and instrument choice available.

The top of the Library says where each part of the song comes from:
patterns, instruments, time signature, chords, tempo and swing. Opening a
demo brings in every part from it; applying a preset brings in the parts
checked, even one that was already the same. Each line names the demo,
style, rhythm or saved style, says **edited, was …** once you change that
part yourself, or **yours** when nothing brought it in. A time chosen with the
**Time** picker is your edit; a part that only moves along with another, such as
the tempo kept with a new time signature, keeps its source. The popup shows
the same next to each checkbox (**now: …**), so you can see what it would
replace. Undo, redo and saving keep the sources.

## Locking a channel

The padlock next to each channel's volume knob in the channel rack locks
that channel: presets, the **Time** picker and the band keep its notes and its
instrument, and change the rest of the song. A locked Keys keeps the song's
chords, and the bass follows them; a locked drum keeps its notes while the
other drums take the new rhythm. When the band comes into a song whose own
Keys or Bass is locked, it leaves that part to it. A new time signature still
fits a locked channel's notes into the new bars, and you can still edit it by
hand. The Library and the popup list what is locked (**🔒 Kept: …**).

## More Library options

In song mode, copying **Playing patterns** and **Voices / instruments** brings in
the preset's full band, including its extra guitar, strings or organ, even when
the demo has no live band yet. Leave **Chord progression** off to keep the song's
exact chords and change only its accompaniment. Enabling it replaces the harmony
with the preset's chart while retaining the melody.

The chord sheet's **Progression** menu uses the Library's browser too, with the same preset names, search and grouping preference.
Search by name, genre or time signature; the search
carries between views, and **×** clears it. **Progression** shows harmony
presets that include a chord progression.

Select **Plain accents** to play chord and bass accents
on the meter's beat groups with the current instruments. This works with
every preset, including saved styles. **Drums and plain accents** applies
the preset’s drums with simple chord and bass accents. The instrument option
is hidden while preserving its remembered value; **Preset patterns** restores
normal style copying. The **Time** picker and bar 1's
time-signature menu also show the matching style, rhythm/accent,
instrument and swing checkboxes directly. Turning off **Use matching style**
there uses a plain drum beat and pitched accents in the requested meter.
Repeated meter changes use the original song as their reference until you
explicitly edit the project; returning to its original meter restores its
exact performance. Undo and redo retain the corresponding reference.
The Library shows the band's
patterns and its instruments as separate parts; the chord sheet remains the
source of the song's chords. These band choices survive saving and
undo/redo. A meter change keeps each bar's duration by adjusting tempo.

## The style editor

Use **Edit style…** or **New style…** in the channel rack (also **View >
Style editor…**) to edit repeating drum, chord and bass parts.
[The style editor](step-9/style-editor.md) tells how it works. The editor
reuses the rack and piano roll, with its own undo history and a four-repeat
preview. **Start from…** copies an existing style; right-clicking a style
or rhythm in the Library also opens an editable copy. Pitched templates
are written over C major and follow the song’s chords when applied.
Save them as `.lineup-style` files; **Settings > Library > Styles folder…** chooses the folder
loaded into **Presets** in the shared browser. **Start from…** uses that same
list, including rhythm presets. Applied styles are
embedded in project files, so reopening a song keeps its custom patterns.

## The mixer

The mixer button in the toolbar (three faders), **View > Mixer**, or
**F9** opens the mixer. Its channels are listed in the order of a
conductor's score, in sections, with all the drums together. Each has
its instrument (click it to choose another, or to send the channel over
MIDI), a level meter, a volume, a pan from L100 to R100 (double-click
for the middle), how much of it goes into the reverb, mute and solo.
The master has a meter for each speaker. They work with both SoundFont
and Lineup Osc sounds, plus samples. The master stereo reverb has
wet/dry, decay, damping and pre-delay controls. Mixer settings are
saved with the project, support undo, and are used for WAV export.

## Track heights, selections and shortcuts

Resize an individual playlist track by dragging the lower edge of its
name header. Double-click that edge to reset it, or use **Track height**
in the header’s right-click menu. Heights follow track moves, are saved
with the project, and support Undo.

**F1**, or **Help > Keyboard shortcuts**, lists every key the app
answers to, window by window. The same list is in the
[Keyboard shortcuts appendix](appendix-shortcuts.md), for printing.

**Follow playhead** in the toolbar and **Settings > Playback** keeps the playing
position visible in the playlist, chord sheet and piano roll. Turning it
off lets you scroll freely; the preference is remembered. Editor shortcuts
follow the active window: **Ctrl+A** selects its bars, notes or clips.
The piano roll supports **Ctrl+C/X/V** and **Ctrl+D** for notes, including
pasting between the song and the style editor; its Tools menu lists these
actions. Pasting or duplicating notes extends the pattern when needed.
The playlist also supports **Ctrl+C/X/V**, **Ctrl+D**, **Delete**, and
**Ctrl-click** to add or remove a clip from the selection. Ctrl-click empty
playlist space sets the paste position without placing a clip.
**Ctrl-drag** empty playlist space to select a box of clips; dragging a
selected clip moves the selected clips together. **Ctrl-click** a section
marker (Intro, Verse, Chorus…) to select its overlapping clips, or select
**Select clips in this section** from its menu. Ctrl-click it again or
press **Escape** to clear the selection. On the chord sheet, clicking a
section heading selects all of that section's bars; clicking it again
clears them. **Shift-click** or **Shift+arrows** selects a range of bars,
and **Escape** clears it. Selection stays with clips when earlier clips
are deleted. Open popups and configuration controls keep their own keys.

## The chord sheet and the band

**F5**, or the first window button in the toolbar (a page of chord
names), opens the **chord sheet**: the
song's harmony bar by bar, each section under its name, as a lead sheet
writes it. **Bars per line** puts as many bars on a line as show every
name in full, at its full size (1, 2, 4, 8, 12 or 16, so that lines
start where phrases do), or the number you pick. Click a bar and type its harmony:
chords (`Am7 D7`, `G7/B`, `F#m7b5`), Roman numerals in the key
(`ii7 V7`), pitch-class sets (`3-11 T2`, `{0,1,4}`), micromodes
(`Menor 3`), twelve-tone row forms (`P0`, `RI7`) or `N.C.`; a `/` holds
the harmony before it one more beat (`C / / G7`). Under each chord is
its numeral in the key, with the figures of an inversion (V⁶₅), and
under each set its notes; hold the mouse down on a harmony to hear it.
**Write the chords the song plays** fills an empty sheet from what the
song plays, with the bass player's note after the slash, and in a set or
a row with the sets or micromodes heard. While the song plays, the bar
playing is framed in white. **Show**, at the top of the sheet, names
the same harmony **As written**, as **Chords**, as **Pitch-class sets**
(C major is 3-11 T7I) or as **Micromodes** (Acorde menor-mayor T7I),
without changing a note. For sets and micromodes it can also read the
song's notes, **together** (vertically) or **in a line** (serially), in
groups of 2 to 12 notes, from every track or one. The sheet is saved with the project and
follows key changes, re-metering and copied sections. Right-click a bar
to **Insert a bar before**, **Insert a bar after** or **Delete this
bar** (or the bars selected): everything after it moves along, notes,
chords, markers and sections, and a new bar is empty, so the harmony
before it goes on through it. The bar numbers on the playlist's ruler
have the same items. The sheet is a window over the playlist, or, with
**⛶ Fill the window** (also **View > Chord sheet in the main window**),
takes the playlist's place in the main window, with longer lines;
**Back to the playlist** puts it back.

**File > New** opens an empty chord sheet with a **band**: type chords
and a piano, a bass and drums play them, written as patterns on their
own tracks in the playlist. **🎷 Band** chooses the instrument lineup,
each player’s voice, and which players are heard. These controls keep the
current playing patterns, chords, meter, tempo and swing. **No band**
removes the accompaniment. Choose rhythms and playing patterns in the
**Library**; a waltz can put the song in 3/4 when copying its time signature. **Progression** writes a well-known progression from the
selected bar, in the song's key, and **End the song here** (right-click
a bar) says where the band stops. The band plays again whenever the
sheet, the key or the bars change; a part you edit in the piano roll
stays as you left it until you click **Play it again**. Picked over a
demo, the band plays the demo's chords and mutes its own accompaniment
until you pick **No band**. **File > New beat** is the old empty
pattern with a drum kit.

To practise with the band, the chord sheet's second row has a **Count-in** (a bar of clicks before the music
starts), a **Metronome**, the **Speed** to play at (25% to 200% of the
song's tempo, which the song itself keeps) and **Tap**, which sets the
tempo from a few clicks on the beat. **You play** leaves a player (the
piano, the bass, the drums, or a guitar, strings or organ) out of the
band so you can play the part yourself; your keyboard then plays that
part's sound. Shift+click a second bar to select several, and **Loop
bars** plays them over and over. The clicks never reach a saved song or
a WAV export. **Follow playhead**, **Count-in** and **Metronome** are also
illuminated toggle buttons beside the main transport. Practice settings
are available in **Edit > Settings… > Playback** too.

The **Form** row above it shapes the whole performance. **Intro** puts
2, 4 or 8 bars in front of the song, a turnaround that leads into its
first chord (ii–V, or I–vi–ii–V), written on the sheet so you can
change it. **Choruses** plays the tune that many times, shown once
between repeat signs on the sheet and the playlist. **Ending** adds two
bars on the key's tonic after the last chorus: one in the style, then
the chord struck once and held, with a crash of the cymbal.

Right-click a bar (or Shift+click to select several first) to pick
**Variation a** or **Variation b** of the style from there. Variation b
is the same style played fuller, the way a band digs into a chorus or a
bridge: the drums move from the hi-hat to the ride, a busier bass or
piano where the style has one (a jazz ballad's bass walks, a pop
ballad's piano plays eighth notes), a little louder. The drums play a
fill into each change and crash on its first beat, and a boxed letter
marks it on the sheet and the playlist.

The same menu's **Style** plays the bars in another style: a verse as
a bossa nova and the bridge as a swing. Right-click a section's name on
the sheet to choose for the whole section. The band keeps its
instruments and plays the new style's comping, bass line, drums and
swing, with a fill into the change. The style's name marks where it
starts, and **The song's style** goes back. Only styles in the bars'
time signature are offered.

**Swing**, next to the tempo on the sheet (and on the playlist), plays
pairs of eighth notes long-short, as jazz, blues and shuffle players
do: from *straight* (even) to 100%, the first two thirds and the last
third of a triplet. The notes stay as written; only how they are played
changes, in WAV and MIDI exports too. Each style brings its own swing
(Swing, Jazz ballad, Jazz waltz and Shuffle 100%, Rock and roll 50%), and
remembers the amount you last set for it.

Many styles bring more players than the piano, the bass and the drums.
A **guitar** plays the shapes a guitarist would (the open C or G of a
chord book, barre chords in other keys, the bass lowest under a slash
chord) and strums them: down-strokes up the strings, lighter up-strokes
on the top four. Rock plays power chords, a swing band the guitar's
"four to the bar", a shuffle the boogie line, reggae and soul a short
chop on beats 2 and 4. **Strings** hold the chords in ballads, tangos
and disco, and an **organ** bubbles in reggae or fills out a rock
chorus; some join in only in variation b. Each has its own General MIDI
instrument and track, and the style menu says who plays.

The band also stops where the music does. A dot after a harmony is a
**shot** (`C.`: the band strikes it once, short, and stops until the
next harmony), two dots a **hold** (`C..`: struck and held) and three a
**rest** (`C...`: silence). A caret before it is a **push** (`^C`): the
band plays it an eighth note early and holds it over the beat, and
`^^C` comes a sixteenth early. The drums stop with the band and hit a
kick and a crash with each shot, hold and push. Right-click a bar and
pick **Band on C** to set the same marks from a menu.

**Soloist (experimental)**, at the bottom of the band menu, adds a
soloist who improvises over the chorus: a tenor sax in jazz, a harmonica
in blues, a fiddle in folk and so on. The line puts chord tones on
the beats and fills the space between with notes of the chord's scale
(chosen from the key, so in C major G7 plays G Mixolydian and E7, which
leads to Am, plays the A harmonic minor scale from E: E Phrygian
dominant) or with chromatic notes a half step away. It breathes
between phrases, and **Another solo** plays a new one. Open its part in
the piano roll to see what each note is: chord tones green with their
degree (R 3 5 7), scale notes blue (passing, neighbour), chromatic notes
orange; point at a note to have it explained.

## Chord sheets as text

**File > Import chord sheet…** reads a chord chart typed as text, the way
charts are passed around in emails and forums (a `.txt` file, or drop it
on the window): bar lines between the bars, a section's name in
brackets over its bars, and a few settings at the top if you like.

```text
Title: Autumn Song
Key: C major
Tempo: 120
Style: Bossa nova

[Verse ×2]
| C  | Am7 | Dm7 G7 | C |
| F  | %   | G7     | C |
```

The chart becomes a new song with the band playing it and the chord
sheet open; a mistake is reported with its line. **File > Export chord
sheet…** writes the song's chord sheet the same way, lined up in
columns, and in the chord sheet **Ctrl+C** and **Ctrl+V** copy and paste
the selected bars as text, to and from a text editor or another song.

## Twelve-tone rows, pitch-class sets and micromodes

For music without a home key, the key menus also offer a **twelve-tone
row** and a **pitch-class set**. **Row…** in the piano roll opens the
twelve-tone window: type, draw or pick a row (the rows of Schoenberg's
Suite Op. 25, Berg's *Lyric Suite*, and Webern's Symphony Op. 21 and
Concerto Op. 24, all public domain now, or a few written for Ferrous
Lineup), see its matrix
with all 48 forms (P, I, R and RI on every note), and write any of them
into the pattern as a melody or as chords, with the durations, dynamics
and octaves following the row too if you like. **Write … into current pattern**
replaces the selected instrument’s notes in that pattern. **Save to new pattern**
creates and selects a separate pattern, using the same row-writing options,
length and time signature, and keeps the source pattern intact. Both actions
support Undo. **Sets…** lists all 224
pitch-class sets by Forte number, with their prime forms and interval
vectors, to draw with or use as a scale, and selecting notes names their
set. **Hear it** previews the chosen set or micromode on the selected instrument.
**Draw with it** in Sets and Micromodes lights up while active; click it
again or choose **Single note** to stop drawing the set. The piano roll's
**Draw > Single note** does the same. The Library's **Twelve-tone rows** and **File > Demos > Twelve-tone**
play seven ways of using a row.

**Micromodes…** opens Francisco Kröpfl's twelve micromodes: the twelve
groups of three notes (Forte's trichords, 3-1 to 3-12) in his three
families, minor, major and chords. Each is drawn on a clock, with its
prime form in solfège, its intervals and its hour on Peter Schat's Tone
Clock, and can be heard, drawn with or used as the key. Selecting notes
shows which micromodes they hold, and the Micromodes and Sets windows
link to each other: a three-note set is a micromode, and every larger
set is made of them. The Library's **Micromodes (Kröpfl)** and **File >
Demos > Micromodes** play music made from one micromode.

## Importing recordings and MIDI files

**File > Import recording…** opens an MP3, WAV, FLAC or Ogg file (or
drop it on the window) and listens to it: the tempo, time signature and
key are set from the recording, which plays on its own channel, and a
Harmony channel plays the chord heard on every beat, so the piano roll
and the harmony keys follow along. This is an early experiment; see
[`crates/lineup-listen`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-listen/README.md) for how it works and
how often it is right.

**File > Import MIDI file…** opens a `.mid` file (or drop it on the
window) as a new project: each track becomes a channel (the drums of
channel 10 one channel per drum), every note keeps its velocity and its
exact time, even between two steps, and the tempo changes, time
signatures and key come with it. The piano roll's Chords lane and the
harmony keys then name the chords as it plays.

## Sounds, ear training and the lead sheet

**Edit > Settings… > Audio > SoundFont instruments** plays recordings of real instruments
instead of the Lineup Osc: every channel with a General MIDI instrument
(the demos' voices, the Library's piano and bass, the demo song's
chords) and, with **SoundFont drums**, the drum kit. The first time, it
offers to download [GeneralUser GS](https://github.com/mrbumpy409/GeneralUser-GS),
a free General MIDI SoundFont by S. Christian Collins (about 30 MB, kept
for next time). **Choose SoundFont…** plays any other `.sf2` file
instead. WAV export uses it too. The Audio category also shows the active
output device, sample rate and channel count.

**View > Ear training** (experimental) asks questions to answer by ear:
which interval two notes make, which kind of chord plays (with
inversions if you like), and which chords of a key the band plays, named
on the harmony keys with the chord names hidden. Every answer is
explained with its notes spelled out, and any answer can be played next
to the right one. The song and its file are left as they are.

**View > Lead sheet** writes the song out the way a band reads it: the
tune in notation on one staff, the chord symbols above it, and the key
and time signatures, sections and repeats, laid out on A4 or Letter
pages. The melody is the channel called Lead (or else the highest one;
any other can be picked), and every note is spelled for the key and the
chord that sounds, so a G♯ over E7 stays G♯ and an A♭ over Fm stays A♭.
**None: chords only** writes rhythm slashes under the chords instead.
**For** writes the sheet for B♭, E♭ or F instruments, so a trumpet,
saxophone or horn player reads it in their own key: the key signature,
the tune and the chords all move.
While the song plays, the bar that sounds lights up. **Export PDF…**
saves the sheet to print or share, and **Print…** opens it in your PDF
viewer. The music is drawn with the Bravura font and the words with
Inter, both free fonts that come with the app.

## Zoom and settings

Too small or too big? The same keys zoom whatever the pointer is over.
Over the playlist or the piano roll, **Ctrl** with **+** and **−** or the
mouse wheel (⌘ instead of Ctrl on a Mac) makes bars and steps wider or
narrower, and **Alt** with the same keys or the wheel makes tracks and
keys taller or shorter, around what is under the pointer. Over the
channel rack they zoom its one size. Anywhere else (the toolbars, the
harmony keys, the status bar, the Patterns and Library panel) they make
the text, buttons and menus bigger or smaller everywhere, as **Settings > Appearance >
Interface size** does. **Ctrl+0** takes what is under the pointer back
to 100%. The playlist's two zooms are also at the right end of the
status bar.
**Edit > Settings…** separates preferences into **Appearance**, **Harmony**,
**Library**, **Audio** and **Playback** categories. Appearance holds interface size and
all editor zoom levels; Harmony holds how numerals are written in minor
keys; Library holds style grouping, saved style defaults
and the styles folder; Audio holds output information and SoundFont settings;
Playback holds follow playback and practice controls. The former zoom,
Sounds and Practice menus are replaced by these categories. Zoom controls
share the current values with the keyboard and wheel shortcuts, which
continue to work as before.
The View menu also shows or hides each window and panel, and **Alt+F8**
hides the Patterns and Library panel to give the playlist more room. The
zoom, the size of the app's window and where you left the windows are
remembered for next time. A window opens at the size of what it shows
(the mixer, the channel rack, the shortcuts) or, for a window you work
in (the piano roll, the chord sheet), at its own size the first time
after the app starts.

## Exporting video, and ready-built programs

**File > Export video…** saves the song as an MP4 file, with the views
you pick playing along: the chord sheet with the bar playing framed, the
lead sheet, the playlist, the piano roll and the harmony keys, arranged
on a preview, in HD or Full HD. The encoders are built into the app, so
nothing else needs installing.

Bare programs for all three systems are attached to every CI run as
`ferrous-lineup-linux`, `ferrous-lineup-macos` and
`ferrous-lineup-windows`. The **Release** workflow builds the packages
ready to use, with the README and the licence: a Linux AppImage, `.deb`
and `.tar.gz`; a Windows Setup `.exe` and portable `.zip`; and a macOS
`.dmg` with `Ferrous Lineup.app`, for Apple Silicon and Intel. They are
attached to every Release run, and pushing a `v` tag (`v0.2.0`)
publishes them on the repository's
[Releases page](https://github.com/naelolaiz/ferrous-lineup/releases).
The Windows programs are unsigned, and the macOS app is ad-hoc signed
but not notarized, so both warn before the first start;
[Release builds](step-9/releases.md) explains each download and the
warnings.

## Checking playback with a trace

A trace lists every note the player plays, one line each: when it starts
and how long it lasts in seconds and in steps, its bar and beat, the
tempo and time signature in force, the channel, its General MIDI program,
the pitch and the velocity, and how late it came after its step (nudge,
swing and loose timing). It is written where the notes start, on the
audio thread, so it shows what you hear rather than what the screen says.

```sh
ferrous-lineup --trace song.lineup --out song.tsv   # a project file
ferrous-lineup --trace "Für Elise"                  # a demo, to the terminal
ferrous-lineup --demos                              # the demos' names
```

`--trace` plays the song as File > Export WAV does, without a window or a
sound card. To trace what you play in the app instead, tick **Write a
trace while playing** in Settings > Playback: each playback, from Play to
Stop, adds a table to the file shown there. Lines starting with `#` say
what the table holds and list the tempo and time signature changes; the
rest is tab-separated, for a spreadsheet, `awk` or `grep`. On Windows,
use `--out`: the ready-built program has no terminal to print to.
