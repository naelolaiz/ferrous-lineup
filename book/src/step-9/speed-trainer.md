# The speed trainer: start slow, get a little faster each time

Files: [`crates/lineup-core/src/sequencer.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/sequencer.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs),
[`crates/lineup-app/src/app/practice.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/practice.rs),
[`crates/lineup-app/src/chord_sheet/practice.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet/practice.rs)
and [`crates/lineup-app/src/app/settings.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/settings.rs)

A passage you cannot play at full speed is learned the same way on
every instrument: slowly, until it is right, then a little faster, and
again, until it is up to tempo. With a metronome that means stopping
every few times round to turn a knob. [Practising with the
band](practice.md) gave the band a **Speed** and a **Loop**; the
**speed trainer** turns the knob for you. Each time the loop comes
round, the band plays a few percent faster, until it reaches the speed
you are aiming for.

![The chord sheet's practice row while bars 3–4 loop: Speed 80%, the Speed trainer lit, "from 70% to 100% by 5% every 1 pass" beside it, and the TEMPO box saying 80%. Bar 3 is framed in white as it plays, and the status bar says "Speed trainer: 80%, on the way to 100%"](../images/step-9-speed-trainer-sheet.png)

## Trying it

1. Open the chord sheet (**F5**). Click bar 3, Shift+click bar 4 and
   click **Loop bars 3–4**.
2. Click **Speed trainer**. The speed drops to **70%**, and four boxes
   appear beside the button: **from 70% to 100% by 5% every 1 pass**.
3. Press Play. The starting song is at 130 BPM, so the band starts at
   70% of it, 91 BPM. Each time bar 4 ends and bar 3 comes round
   again, the speed goes up by 5% of the song's tempo: 75%, 80%,
   85%… On the seventh time round it plays at 100%, the song's own
   tempo, and stays there.
4. Press **Stop** (■). The speed goes back to 70%, ready to start the
   next run slow again. **Pause** (❚❚) keeps the speed, so Play
   carries on from where you were, at the speed you had got to.

The four boxes are dragged, or double-clicked to type:

- **from**: the speed Play starts at.
- **to**: the speed to get to. 100% is the song's tempo; 110% goes
  past it, to have some room to spare when you play it for real.
- **by**: how much faster each new speed is, from 1% to 25% of the
  song's tempo.
- **every**: how many times the loop plays at each speed before the
  next one. With **every 2 passes** you play each speed twice: once
  to get it, once to check that you have it.

A speed you set by hand in the **Speed** box, during a run, is where
the trainer carries on from. Clicking **Speed trainer** again turns it
off and leaves the speed where it got to.

"The loop" is whatever playback goes round: the bars you loop, the
whole song when nothing is looped (it starts again from bar 1 at its
end), or the pattern, in pattern mode. The trainer works with all
three.

The four numbers are remembered for next time, like the count-in and
the metronome. The trainer itself is off each time the app starts, as
the speed is at 100%: a song is never slow by surprise. The same
controls are in **Edit > Settings… > Playback**, under the practice
speed:

![Settings, Playback, Practice: Count-in, Metronome, Practice speed 100% with Reset, then Speed trainer with "from 70% to 100% by 5% every 1 pass"](../images/step-9-speed-trainer-settings.png)

As with the speed, nothing of this is part of the song: a saved song
and a WAV export keep the song's own tempo.

## Counting the times round

To go faster "each time the loop comes round", the app has to know
when that happens. The window could watch the playhead and call it a
lap whenever the step number goes down, from the last step back to the
first. But the step number also goes down when you click an earlier
bar while the music plays, and that is not a time round: the trainer
would speed up whenever you jumped back to try a bar again.

The one that knows is the audio thread's `Sequencer`, the clock that
fires the steps ([The sequencer: timing to the
sample](../step-2/sequencer.md)). It now counts its **laps**: the
times it played on past the last step and came round to step 0.

```rust
pub fn tick(&mut self) -> Option<usize> {
    let fired = if self.frames_until_next <= 0.0 {
        let step = self.next_step;
        if self.coming_round {
            self.laps += 1;
        }
        self.next_step = (self.next_step + 1) % self.length;
        self.coming_round = self.next_step == 0;
        self.frames_until_next += self.frames_per_step;
        Some(step)
    } else {
        None
    };
    self.frames_until_next -= 1.0;
    fired
}
```

`coming_round` is set when the last step fires, and the lap is counted
when step 0 fires after it. A jump (`start_at`, which a click on a bar
sends), a restart (`reset`) and a loop made shorter than where playback
had got to (`set_length`) all clear `coming_round`, so none of them
counts. The test `laps_count_the_loops_played_to_the_end` plays a
4-step loop and checks each of these.

The audio thread stores the count after every step in an atomic
integer, as it does the playhead, and `AudioEngine::laps()` reads it.
The window looks at it once a frame:

```rust
fn came_round(&mut self, laps: usize) -> Option<u16> {
    let seen = self.laps_seen.replace(laps);
    if !self.training {
        return None;
    }
    // A new audio thread, after a restart, counts from 0 again.
    let new = laps.checked_sub(seen?)?;
    // …
}
```

Comparing with the count it saw last, rather than looking for "a lap
happened", means a lap is never missed: if the window was hidden for a
few seconds and did not draw, it finds several new laps at once and
moves the speed on once for each.

> **Rust: `?` on an `Option`.** In a function that returns an
> `Option`, `value?` means "the value inside, or return `None` from
> here". `seen?` returns `None` the first time, when nothing was seen
> yet. `checked_sub` is a subtraction that gives `None` instead of
> going below zero: after **Restart audio** a new audio thread counts
> from 0 again, `laps` is smaller than `seen`, and the trainer waits
> for the next lap instead of working out a huge number of them. A
> plain `laps - seen` would panic there in a debug build, and wrap
> round to a number near 2⁶⁴ in a release build.

## On the beat, not a step late

The window draws about 60 frames a second, so it learns of a lap up to
a frame after it happened, and the audio thread then needs a block more
to get a command. By then step 0 of the new time round has started,
at the old tempo, and the new speed would start on step 1: the first
sixteenth note of every pass a little too slow. Nobody would notice one
sixteenth 5% too long, but the click on the first beat would be a few
milliseconds late, every time round.

So the window does not wait for the lap. As soon as it knows the next
speed (right after the last lap, or when you turn the trainer on), it
sends the audio thread the tempos of that speed, to be used **from the
next lap on**:

```rust
pub struct LapTempos {
    pub tempo: Tempo,     // in place of the SetTempo tempo
    pub song: Vec<Tempo>, // in place of the SetSongTempos list
}

Command::SetLapTempos(Option<LapTempos>)
```

The audio thread keeps them until the loop comes round. Just before a
step fires it already asks which tempo that step is at; now it first
asks whether this is step 0 coming round, and if so takes the waiting
tempos in:

```rust
if let Some(step) = self.sequencer.due() {
    if self.sequencer.comes_round()
        && let Some(tempos) = self.lap_tempos.take()
    {
        self.tempo = tempos.tempo;
        let old = std::mem::replace(&mut self.song_tempos, tempos.song);
        self.throw_away(Garbage::Tempos(old));
    }
    let tempo = self.tempo_of_step(step);
    if tempo != self.tempo_now {
        self.set_tempo(tempo);
    }
}
```

`take()` empties the slot, so the tempos are used once. The old list
goes back to the UI thread to be freed, as every list the audio thread
lets go of does ([Practising with the band](practice.md#where-the-clicks-come-from)).

When the window then sees the lap, it sends the same tempos again with
the usual `SetTempo` and `SetSongTempos`. When they arrived in time
that changes nothing: they are the very same numbers. It matters when
they did not: if the request reached the audio thread after the loop
had already come round, the resend sets the new speed a step late, and
the next request replaces the old one before it can be played a lap
too late.

### Keeping the request up to date

A request waits a whole time round, and the song can change in the
meantime: you type a chord, and the band's new part goes to the audio
thread with the song's tempo list. A tempo list worked out before that
could be the wrong length, or miss a tempo change you just made. So
every time the window sends `SetTempo` or `SetSongTempos`, `send`
counts it:

```rust
fn send(&mut self, command: Command) {
    if matches!(command, Command::SetTempo(_) | Command::SetSongTempos(_)) {
        self.practice.tempos_sent();
    }
    // …
}
```

and the window keeps, beside the speed change it asked for, the count
at the time. Once a frame it compares them with what it would ask for
now:

```rust
let change = self.practice.next_change();
let sent = change.map_or(0, |_| self.practice.tempos_sent);
if self.practice.lap_change != Some((change, sent)) {
    let tempos = change.map(|(_, next)| {
        let (tempo, song) = self.tempos_at(next);
        LapTempos { tempo, song }
    });
    self.send(Command::SetLapTempos(tempos));
    self.practice.lap_change = Some((change, sent));
}
```

`next_change` is `None` while the trainer is off, at its target, or
before the last pass at a speed when it plays each speed more than
once; then the window sends `SetLapTempos(None)`, which takes back
any tempos still waiting. Stop, a speed typed by hand, or new trainer
numbers all change `next_change`, so the request follows them too.

## Checking it in a trace

The playback trace ([A playback trace](playback-trace.md)) writes a
`# tempo:` line each time the tempo changes. Here is the start of the
trace of the run above, bars 3–4 of the starting song with the trainer
at its defaults:

```text
# tempo: 0.0000 s, step 0, bar 1: 91.00 BPM
# tempo: 5.2747 s, step 0, bar 1: 97.50 BPM
# tempo: 10.1978 s, step 0, bar 1: 104.00 BPM
# tempo: 14.8132 s, step 0, bar 1: 110.50 BPM
# tempo: 19.1571 s, step 0, bar 1: 117.00 BPM
# tempo: 23.2596 s, step 0, bar 1: 123.50 BPM
# tempo: 27.1463 s, step 0, bar 1: 130.00 BPM
```

Every change is on step 0, the loop's first step (the trace counts bars
and steps of what is played, the loop, so bar 3 of the song is its bar
1). And the times add up: two bars of 4/4 are 8 beats, which at 91 BPM
last 8 × 60 ÷ 91 = 5.2747 seconds, exactly where the second line is.
At 97.5 BPM the next 8 beats last 4.9231 seconds, and 5.2747 + 4.9231
= 10.1978.

## Exercises

1. With the defaults (from 70% to 100% by 5% every 1 pass), on which
   time round does the band first play at full speed? How long, in
   seconds, does it take to get there with bars 3–4 of the starting
   song? Check your answer against the trace above.
2. Set **from 50% to 100% by 10% every 2 passes**. Write down the
   speed of each of the first eight times round, then play it with the
   trace on and check.
3. Set **from 100% to 60%**. What does the trainer do now? Think of a
   use for it (hint: a passage you can play fast, but not yet evenly).
4. Start a run, Pause in the middle of it, then press Play. Then Stop
   and press Play. What is the speed each time, and why is it useful
   that the two buttons do different things?
5. With the trainer on and nothing looped, play the whole starting
   song. When does the speed go up?
6. (Rust) In `came_round`, `self.laps_seen.replace(laps)` runs before
   the check for `self.training`. What would go wrong when you turn the
   trainer on in the middle of a song if it ran after it instead?
7. (Rust) The audio thread throws the old list of an unused request
   away with `throw_away` when a new request (or `None`) replaces it.
   What would happen if `Command::SetLapTempos(tempos) => self.lap_tempos
   = tempos` were written instead, and why does it matter on the audio
   thread in particular?
8. (Rust) Remove the `self.practice.tempos_sent()` call from `send`
   and add a tempo change inside the loop while the trainer runs. What
   does the band play when the loop comes round, and why?
