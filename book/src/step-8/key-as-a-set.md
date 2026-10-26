# A key as a pitch-class set: the same notes, another name

Files: [`crates/lineup-app/src/twelve_tone.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/twelve_tone.rs),
[`crates/lineup-core/src/harmony/scale.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/scale.rs)

Play the starting song in C major and pick **pitch-class set** in the
Key menu. Until now the menu took the last set you had used, or the
diminished seventh chord C Eb Gb A if you had used none. The song went
on playing C, Am, Dm and G, but the harmony keys showed Cdim, D#dim,
F#dim and Adim. None of those was playing, so the keys were wrong
about the music.

## Why the notes must not move here

Changing from **major to minor** changes the music: E, A and B become
Eb, Ab and Bb, so [changing key](changing-key.md) moves the notes.

A **pitch-class set** is not another scale to play in. It is another
*way of naming* notes (see ["Pitch-class sets"](twelve-tone.md)). The C
major scale already is a pitch-class set: C D E F G A B, the set class
**7-35**. So switching the key to "pitch-class set" keeps every note
and only shows the same harmony in set notation:

![C major as the set 7-35: the same seven chords, each with its set class.](../images/step-8-key-as-set.png)

| Key | Notes | Set class |
|---|---|---|
| C | C E G | 3-11 |
| Dm | D F A | 3-11 |
| Em | E G B | 3-11 |
| F | F A C | 3-11 |
| G | G B D | 3-11 |
| Am | A C E | 3-11 |
| Bdim | B D F | 3-10 |

This is worth a look on its own. Major and minor triads have the
**same** set class, 3-11, because a minor triad is a major triad turned
upside down (C E G inverted around C is C Ab F, which is F minor), and
set classes count a set and its inversion as one. The diminished triad
is another class, 3-10.

A set you choose yourself is different: **Use as the key** in the set
browser makes that set the key, and it may be any set at all.

## The code

`set_key` decides which set the menu switches to. A row already gave
its first three notes (the notes the harmony keys were playing); now a
key gives its whole scale:

```rust
let up = scale.root.semitones_up_to(root);
if let Some(group) = scale
    .tone_row()
    .and_then(|row| row.segments(3).first().copied())
{
    return Scale::set(root, group.transpose(up));
}
if scale.kind.is_tonal() && scale.kind != ScaleKind::Chromatic {
    return Scale::set(root, scale.pitch_set().transpose(up));
}
```

The chromatic scale is left out: it holds all twelve notes, so it says
nothing about the harmony, and the menu still offers the last set used.

## Spelling a set that is a scale

Sets have no letters of their own, so their notes took the usual
names: C# Eb F# Ab Bb. For E major as a set that is wrong: E F# **Ab**
A B C# **Eb**, and the chord on the third would be called Abm. A set
whose notes are exactly a scale on its root is now spelled as that
scale:

```rust
fn set_as_scale(self) -> Option<Scale> {
    let ScaleKind::Set(set) = self.kind else {
        return None;
    };
    let semitones = set.semitones();
    ScaleKind::ALL
        .into_iter()
        .filter(|&kind| kind != ScaleKind::Chromatic)
        .find(|kind| kind.tonal_intervals() == semitones.as_slice())
        .map(|kind| Scale::new(self.root, kind))
}
```

`let ... else` takes the set out of the enum or returns early. Then
`find` looks through the tonal scales for one with the same semitones
above the root. The root matters: the white keys on C are C major, but
on D they are D Dorian, so `Scale::spell` and `Scale::spell_root` ask
`set_as_scale` first and spell as that scale. E major as a set is E F#
G# A B C# D# again, and its third chord is G#m. A set that is no scale,
such as C Eb F# A, keeps the usual names.

## Trying it

1. Open the starting song and press Space.
2. Pick **pitch-class set** in the Key menu under the harmony keys.
   The song sounds the same, and the key that lights up is the chord
   playing, now with its set class under it.
3. Pick **major** again: the keys go back to I to vii°, and still
   nothing moves.
4. Pick **minor** instead: this time the song changes, as before.
