# Cambiar de tonalidad: mover la canción y ajustarse a la escala

Archivos: [`crates/lineup-core/src/tools.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/tools.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs),
[`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs),
[`crates/lineup-app/src/midi.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/midi.rs)
y [`crates/lineup-app/src/app/live.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/live.rs)

Hasta ahora, los menús de **Key** y de modo (a la izquierda de las
harmony keys, y arriba del piano roll) solo cambiaban lo que *veías*:
qué filas resalta el piano roll y cómo se llaman las harmony keys. La
canción seguía sonando en su tonalidad vieja, así que elegir Re menor
mientras Frère Jacques sonaba en Do mayor no cambiaba nada que pudieras
oír.

Ahora una tonalidad nueva mueve la canción. Elige Re y los acordes, el
bajo y la melodía suben todos un tono. Elige menor y cada nota se mueve
al mismo grado de la escala menor. La batería se queda como está, y
**Ctrl+Z** lo devuelve todo de una vez.

![Frère Jacques en Do mayor. Las filas de Do mayor son más claras, Do está teñido, y las harmony keys van de C a Bdim.](../images/step-8-key-before.png)

![El mismo patrón tras elegir Re, y luego menor. La melodía empieza ahora Re, Mi, Fa, Re, las harmony keys van de Dm a C, y la batería (las notas fantasma grises) no se ha movido.](../images/step-8-key-after.png)

## Por qué el menú de tonalidad mueve las notas

En muchos programas de música, el menú de tonalidad y escala del piano
roll no mueve nunca una nota: solo resalta la escala y hace que las
notas que dibujas o arrastras se *ajusten* a ella. Para mover una
canción entera, transpones tú mismo sus patrones. El menú de tonalidad
de Ferrous Lineup es la tonalidad de toda la canción, la que usan las
harmony keys, las ayudas y la banda, así que también mueve la canción,
como una banda toca la misma canción en otra tonalidad cuando se lo
pides; y el piano roll recibe encima el ajuste (abajo).

## Una nota cada vez

Como con toda [herramienta](../step-6/tools.md), la parte musical es una
función normal que no cambia nada por sí misma. Responde a una pregunta:
¿adónde va esta nota cuando la tonalidad cambia de `from` a `to`?

```rust
pub fn change_key(note: Note, from: Scale, to: Scale) -> Note {
    let up = from.root.semitones_up_to(to.root) as i32;
    let shift = if up > 6 { up - 12 } else { up };
    let Some(moved) = note.transpose(shift) else {
        return note;
    };
    let old = Scale::new(to.root, from.kind);
    let Some(degree) = old.degree_of(moved.pitch_class()) else {
        return moved;
    };
    let (old_steps, new_steps) = (from.kind.intervals(), to.kind.intervals());
    if old_steps.len() == new_steps.len() {
        let offset = new_steps[degree] as i32 - old_steps[degree] as i32;
        moved.transpose(offset).unwrap_or(moved)
    } else {
        nearest_in_key(moved, to)
    }
}
```

Funciona en dos movimientos:

1. **Una fundamental nueva mueve cada nota la misma distancia.** De Do a
   Re son dos semitonos arriba. De Do a Sol podrían ser siete arriba o cinco
   abajo; `shift` toma el más cercano, así que la canción se queda más o
   menos donde estaba en el teclado en lugar de saltar una quinta
   arriba. `semitones_up_to` cuenta siempre hacia arriba (de 0 a 11), y
   todo lo que pase de 6 se convierte en un paso abajo.
2. **Un modo nuevo mueve cada nota al mismo grado.** Tras el primer
   movimiento, la nota se compara con el tipo de escala *viejo* sobre la
   fundamental *nueva*. Si es la tercera nota de Re mayor (Fa♯), se
   convierte en la tercera nota de Re menor (Fa). `intervals()` lista las
   notas de cada escala como semitonos por encima de la fundamental, así
   que la diferencia entre las dos listas en el mismo índice es
   exactamente cuánto se mueve ese grado:

   | grado         | 1 | 2 | 3  | 4 | 5 | 6  | 7  |
   |---------------|---|---|----|---|---|----|----|
   | mayor         | 0 | 2 | 4  | 5 | 7 | 9  | 11 |
   | menor natural | 0 | 2 | 3  | 5 | 7 | 8  | 10 |
   | se mueve      | 0 | 0 | -1 | 0 | 0 | -1 | -1 |

   Así que de Do mayor a Do menor se rebajan Mi, La y Si, y se quedan Do, Re, Fa
   y Sol.

Dos clases de notas necesitan una regla propia:

- **Las notas fuera de la escala vieja**, como un acorde tomado en
  préstamo o una parte escrita en una tonalidad propia, no tienen grado.
  `degree_of` devuelve `None`, y el `let ... else` devuelve la nota solo
  tras el primer movimiento. Mantiene su distancia a la fundamental, así
  que una modulación sigue siendo una modulación.
- **Las escalas de tamaños distintos**, como la mayor (siete notas) y la
  pentatónica mayor (cinco), no tienen grados que encajen. Entonces una
  nota que no está en la escala nueva va a la más cercana que sí está,
  con `nearest_in_key` del paso 6.

Las líneas `let Some(x) = ... else { return ... };` son el *let-else*
de Rust: si el patrón no encaja, el bloque `else` debe salir de la
función. Aquí se usa dos veces, para «se saldría del rango MIDI» y
«no está en la escala».

## El proyecto entero

`Project::change_key` ejecuta la función sobre cada nota que debe
moverse:

```rust
pub fn change_key(&mut self, scale: Scale) {
    let from = self.scale;
    if from == scale {
        return;
    }
    let pitched: Vec<bool> = self.channels.iter().map(Channel::is_pitched).collect();
    for pattern in &mut self.patterns {
        for (channel, _) in pitched.iter().enumerate().filter(|(_, p)| **p) {
            let notes: Vec<NoteEvent> = pattern
                .notes(channel)
                .iter()
                .map(|note| match note.is_step() {
                    true => *note,
                    false => NoteEvent {
                        pitch: tools::change_key(note.pitch, from, scale),
                        ..*note
                    },
                })
                .collect();
            pattern.set_notes(channel, notes);
        }
    }
    self.scale = scale;
}
```

¿Por qué reunir `pitched` primero? El bucle toma prestado
`self.patterns` de forma mutable, y Rust no le deja mirar
`self.channels` a través de `self` a la vez. Leer los canales a un
`Vec<bool>` pequeño antes del bucle termina ese préstamo pronto.

`Channel::is_pitched` es un `matches!` sobre el tipo de canal: la
batería integrada y un canal MIDI Out en el canal de batería de General
MIDI (10) no se mueven. Los pasos del step sequencer (`is_step`) tampoco
se mueven, porque su altura solo significa «toca el sample».

Los dos menús de tonalidad envían su elección a la app, que hace

```rust
self.edit("change key", |project| project.change_key(scale));
```

y [`edit`](../step-5/history.md) guarda el proyecto tal como estaba
antes, así que un Ctrl+Z devuelve la tonalidad *y* cada nota. El hilo de
audio recibe los patrones nuevos por el camino de siempre, así que una
canción que está sonando sigue, ahora en la tonalidad nueva.

## Snap to scale

Junto a las notas de la tonalidad, el piano roll tiene ahora un botón
**Snap to scale**. Mientras está encendido:

- una nota que dibujas cae en la nota más cercana de la tonalidad (el
  contorno que sigue al puntero muestra adónde irá);
- una nota que arrastras se mueve solo por las filas de la tonalidad;
- una selección que arrastras se ajusta una vez que se ha movido. Las
  notas que ya están se dejan en paz hasta que las tocas: selecciónalas
  y arrástralas un semitono arriba y de vuelta para ajustarlas todas.

Haz clic derecho en el botón para ver su menú, que también tiene **Snap
notes you play**: las teclas tocadas en el teclado de escritura o en un
teclado MIDI suenan, y se graban, como la nota más cercana de la
tonalidad.

![Clic derecho en Snap to scale para ver su menú.](../images/step-8-snap-menu.png)

En el piano roll es un método pequeño, usado donde sea que una altura
venga del puntero:

```rust
fn snapped(&self, pitch: Note, scale: Scale) -> Note {
    match self.snap_to_scale {
        true => tools::nearest_in_key(pitch, scale),
        false => pitch,
    }
}
```

El arrastre de grupo necesita un dato más: si las notas ya se han
movido. `GroupDrag` recibe un `edited: bool` que se vuelve true en
cuanto el puntero sale de la celda donde empezó, y solo entonces se
ajustan las notas. Sin eso, con solo hacer clic en una selección se
movería.

### Ajustar un teclado MIDI

El teclado MIDI es más peliagudo, porque sus notas no pasan nunca por la
UI de camino al sonido: midir llama a `on_message` en su propio hilo,
que envía la nota directa al hilo de audio (ver [el paso 7](../step-7/live.md)).
Así que la tonalidad a la que ajustarse tiene que compartirse con ese
hilo:

```rust
snap: Arc<Mutex<Option<Scale>>>,
```

La UI lo escribe una vez por frame (`Midi::snap_to`), y el callback lo
lee para cada Note On. Hay una pega: si mantienes una tecla, cambias la
tonalidad de la canción y sueltas, el Note Off se ajustaría a una nota
*distinta* y la primera sonaría para siempre. Así que el callback
recuerda con qué nota empezó cada tecla, en `sounding: Vec<(Note,
Note)>`, y un Note Off detiene esa. El teclado de escritura ya guardaba
una lista así (`typing_down`), y ahora usa la nota guardada allí también
para el Note Off.

## Pruébalo

1. Abre **File > Demos > Songs > Country and folk > Frère Jacques**.
   Empieza a sonar.
2. En la barra de harmony keys, elige **Re** como tonalidad: la canción
   sigue un tono más alta. Elige **minor**: se vuelve oscura.
3. Pulsa **Ctrl+Z**: de vuelta a Re mayor.
4. Abre el piano roll, enciende **Snap to scale** y mueve el puntero
   sobre una fila oscura (fuera de la tonalidad): el contorno se sienta
   en la fila de al lado.
