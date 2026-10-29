# Una tonalidad como conjunto de clases de alturas: las mismas notas, otro nombre

Archivos: [`crates/lineup-app/src/twelve_tone.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/twelve_tone.rs),
[`crates/lineup-core/src/harmony/scale.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/scale.rs)

Toca la canción de inicio en Do mayor y elige **conjunto de clases de
alturas** en el menú Tonalidad. Hasta ahora el menú tomaba el último conjunto
que hubieras usado, o el acorde de séptima disminuida Do Mi♭ Sol♭ La si no
habías usado ninguno. La canción seguía tocando C, Am, Dm y G, pero las
teclas de armonía mostraban Cdim, D#dim, F#dim y Adim. Ninguno de esos
sonaba, así que las teclas estaban equivocadas sobre la música.

## Por qué aquí las notas no deben moverse

Cambiar de **mayor a menor** cambia la música: Mi, La y Si se vuelven Mi♭,
La♭ y Si♭, así que [cambiar de tonalidad](changing-key.md) mueve las
notas.

Un **conjunto de clases de alturas** no es otra escala en la que tocar.
Es otra *forma de nombrar* las notas (véase ["Conjuntos de clases de
alturas"](twelve-tone.md)). La escala de Do mayor ya es un conjunto de
clases de alturas: Do Re Mi Fa Sol La Si, la clase de conjunto **7-35**. Así
que cambiar la tonalidad a "conjunto de clases de alturas" mantiene
cada nota y solo muestra la misma armonía en notación de conjuntos:

![Do mayor como el conjunto 7-35: los mismos siete acordes, cada uno con su clase de conjunto.](../images/step-8-key-as-set.png)

| Tonalidad | Notas | Clase de conjunto |
|---|---|---|
| Do | Do Mi Sol | 3-11 |
| Dm | Re Fa La | 3-11 |
| Em | Mi Sol Si | 3-11 |
| Fa | Fa La Do | 3-11 |
| Sol | Sol Si Re | 3-11 |
| Am | La Do Mi | 3-11 |
| Bdim | Si Re Fa | 3-10 |

Esto merece una mirada por sí solo. Las tríadas mayor y menor tienen la
**misma** clase de conjunto, 3-11, porque una tríada menor es una
tríada mayor puesta boca abajo (Do Mi Sol invertido alrededor de Do es Do La♭
Fa, que es Fa menor), y las clases de conjunto cuentan un conjunto y su
inversión como uno. La tríada disminuida es otra clase, 3-10.

Un conjunto que eliges tú mismo es distinto: **Usar como tonalidad** en el
navegador de conjuntos hace de ese conjunto la tonalidad, y puede ser
cualquier conjunto.

## El código

`set_key` decide a qué conjunto cambia el menú. Una serie ya daba sus
tres primeras notas (las notas que tocaban las teclas de armonía); ahora una
tonalidad da su escala entera:

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

La escala cromática se deja fuera: contiene las doce notas, así que no
dice nada de la armonía, y el menú sigue ofreciendo el último conjunto
usado.

## Deletrear un conjunto que es una escala

Los conjuntos no tienen nombres de nota propios, así que sus notas tomaron los
nombres de siempre: Do♯ Mi♭ Fa♯ La♭ Si♭. Para Mi mayor como conjunto eso
está mal: Mi Fa♯ **La♭** La Si Do♯ **Mi♭**, y el acorde sobre la tercera se
llamaría Abm. Un conjunto cuyas notas son exactamente una escala sobre
su fundamental se deletrea ahora como esa escala:

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

`let ... else` saca el conjunto del enum o vuelve pronto. Luego `find`
busca entre las escalas tonales una con los mismos semitonos sobre la
fundamental. La fundamental importa: las teclas blancas sobre Do son Do
mayor, pero sobre Re son Re dórico, así que `Scale::spell` y
`Scale::spell_root` preguntan primero a `set_as_scale` y deletrean como
esa escala. Mi mayor como conjunto es otra vez Mi Fa♯ Sol♯ La Si Do♯ Re♯, y su
tercer acorde es G#m. Un conjunto que no es una escala, como Do Mi♭ Fa♯ La,
mantiene los nombres de siempre.

## Probarlo

1. Abre la canción de inicio y pulsa Espacio.
2. Elige **conjunto de clases de alturas** en el menú Tonalidad bajo las
   teclas de armonía. La canción suena igual, y la tecla que se ilumina es
   el acorde que suena, ahora con su clase de conjunto debajo.
3. Elige **mayor** otra vez: las teclas vuelven a I a vii°, y nada se
   mueve.
4. Elige **menor** en su lugar: esta vez la canción cambia, como antes.
