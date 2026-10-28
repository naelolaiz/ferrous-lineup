# La ventana del piano roll

Archivos: [`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs),
[`crates/lineup-app/src/channel_rack.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/channel_rack.rs),
[`crates/lineup-app/src/widgets.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/widgets.rs)
y [`crates/lineup-app/src/app.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app.rs)

![El piano roll en Fa mayor: Si♭ está resaltado, Si no, y las notas Si se ponen rojas](../images/step-3-f-major.png)

## Una ventana flotante

```rust
egui::Window::new(format!("Piano roll · {}", channel.name))
    .id(egui::Id::unique("piano-roll"))
    .open(&mut open)
    .default_pos([12.0, 246.0])
    .default_size([1060.0, 486.0])
    .show(ctx, |ui| self.contents(ui, project, playhead, &mut actions));
```

Un `egui::Window` flota sobre los paneles; puedes moverlo,
redimensionarlo, plegarlo con el triángulo y cerrarlo con la ×. Los
estudios basados en patrones funcionan igual: el channel rack y el piano
roll son ventanas dentro de la ventana principal.

- El título cambia con el canal, y egui normalmente identifica una
  ventana por su título. `.id(...)` le da en su lugar una identidad fija,
  así que conserva su posición y su tamaño cuando cambias de canal.
- `.open(&mut open)` añade el botón de cerrar: egui pone `open` a
  `false` cuando se hace clic en él.

Aquí hay un pequeño rompecabezas de préstamos. `open` es una copia de
`self.open`:

```rust
let mut open = self.open;
egui::Window::new(/* ... */)
    .open(&mut open)
    .show(ctx, |ui| self.contents(ui, project, playhead, &mut actions));
self.open = open;
```

Si la ventana prestara `&mut self.open` mientras el closure presta
`&mut self` (para llamar a `self.contents`), `self.open` estaría
prestado de forma mutable dos veces a la vez, lo que Rust prohíbe.
Copiar el `bool` hacia fuera y de vuelta evita la superposición.

## Estado que perdura: un struct en lugar de una función

El channel rack es una función: todo lo que muestra viene del proyecto.
El piano roll necesita recordar cosas que el proyecto no conoce, de un
frame al siguiente:

```rust
pub struct PianoRoll {
    pub open: bool,
    channel: usize,
    stamp: ChordStamp,
    note_length: usize,
    drag: Option<Drag>,
    scroll_to_notes: bool,
    hovered_step: Option<usize>,
}
```

Sigue sin cambiar nunca el propio proyecto. `show` devuelve una lista de
acciones, como hace el rack, y `LineupApp::apply_roll` las lleva a cabo:

```rust
pub enum RollAction {
    AddNotes { channel: usize, notes: Vec<NoteEvent> },
    RemoveNote { channel: usize, index: usize },
    ReplaceNote { channel: usize, index: usize, note: NoteEvent },
    Preview { channel: usize, pitches: Vec<Note> },
    SetScale(Scale),
}
```

Así que el struct guarda el estado de la *interfaz* (qué se está
arrastrando, qué opción de menú está seleccionada) y el proyecto guarda
el estado del *documento* (las notas). El paso 4 guardará el proyecto en
un archivo; nada de `PianoRoll` necesita guardarse.

## Píxeles y música: un struct `Copy` pequeño

La mayor parte del código de dibujo convierte pasos y alturas en
posiciones de pantalla, o la posición del puntero de vuelta en un paso y
una altura. Esa aritmética vive en un solo lugar:

```rust
#[derive(Debug, Clone, Copy)]
struct Grid {
    rect: Rect,
    step_width: f32,
    length: usize,
}

impl Grid {
    fn x(self, step: usize) -> f32 {
        self.rect.left() + step as f32 * self.step_width
    }

    fn row_top(self, midi: u8) -> f32 {
        self.rect.top() + (HIGHEST - midi) as f32 * ROW_HEIGHT
    }

    fn cell_at(self, pos: Pos2) -> (usize, Note) {
        let last_step = self.length.saturating_sub(1) as f32;
        let step = ((pos.x - self.rect.left()) / self.step_width)
            .floor()
            .clamp(0.0, last_step) as usize;
        let row = ((pos.y - self.rect.top()) / ROW_HEIGHT)
            .floor()
            .clamp(0.0, (HIGHEST - LOWEST) as f32) as u8;
        (step, Note::new(HIGHEST - row))
    }
}
```

Las notas agudas están arriba, así que la fila cuenta hacia abajo desde
`HIGHEST`. `cell_at` recorta (clamp), así que un arrastre que sale de la
cuadrícula sigue obteniendo la celda más cercana, y devuelve una
**tupla**, `(usize, Note)`, que quien llama desarma con
`let (step, pitch) = grid.cell_at(pos);`.

Como `Grid` es `Copy` y sus métodos reciben `self` por valor, se puede
pasar a todas las funciones de dibujo sin ningún `&` ni lifetimes. Para
un struct de unos pocos números, copiar es tan barato como pasar una
referencia.

El ancho de paso se calcula en cada frame a partir del ancho de la
ventana, así que el patrón completo siempre cabe a lo ancho y la ventana
se puede redimensionar libremente.

## La entrada del ratón como una máquina de estados pequeña

egui informa de lo que hace el ratón *en este frame*. Un arrastre dura
muchos frames, así que el piano roll lo recuerda en un enum:

```rust
enum Drag {
    /// Moving a note; `grab` is how many steps after its start the pointer took hold.
    Move { index: usize, grab: usize },
    /// Changing a note's length by dragging its right end.
    Resize { index: usize },
}
```

`handle_input` es largo, pero cada parte responde a una pregunta:

1. **¿Se acaba de pulsar el botón izquierdo sobre la cuadrícula?** Si
   tocó una nota, empieza un `Move`, o un `Resize` si la pulsación fue a
   menos de 6 píxeles del extremo derecho de la nota. Si tocó una celda
   vacía, añade las notas del sello; una sola nota nueva también empieza
   un `Move`, así que puedes colocar y arrastrar en un solo gesto.
2. **¿Está el botón derecho pulsado sobre una nota?** Bórrala. Comprobar
   cada frame significa que puedes barrer varias notas para borrarlas
   todas.
3. **¿Está el botón izquierdo levantado?** Entonces cualquier arrastre
   ha terminado: `self.drag = None`.
4. **¿Hay un arrastre en curso?** Calcula dónde estaría ahora la nota y,
   si difiere de dónde está, envía un `ReplaceNote`.

La cuarta parte usa una **cadena de let** con tres condiciones, y
sintaxis de actualización de structs para cambiar uno o dos campos:

```rust
if let Some(drag) = self.drag
    && let Some(pos) = pointer
    && let Some(&note) = notes.get(drag.index())
{
    let (step, pitch) = grid.cell_at(pos);
    let changed = match drag {
        Drag::Move { grab, .. } => NoteEvent {
            pitch,
            start: step.saturating_sub(grab),
            ..note
        },
        Drag::Resize { .. } => {
            let length = (step + 1).saturating_sub(note.start).max(1);
            self.note_length = length;
            NoteEvent { length, ..note }
        }
    };
    if changed != note {
        actions.push(RollAction::ReplaceNote { channel, index: drag.index(), note: changed });
    }
}
```

`Some(&note)` en un patrón copia la nota fuera del `&NoteEvent` que
devuelve `get`. Comparar `changed != note` (gracias al `PartialEq`
derivado) significa que no se envía nada mientras el puntero se queda
dentro de la misma celda, aunque este código se ejecute en cada frame.

Un detalle se encontró probando: un clic derecho rápido puede pulsar y
soltar el botón dentro de un solo frame, así que «¿está el botón
pulsado?» ya es falso cuando se dibuja el frame. El código pregunta en
su lugar «¿se pulsó en este frame, o está pulsado?»:

```rust
pointer.secondary_pressed() || pointer.secondary_down(),
```

### El fantasma

Cuando el puntero está sobre una celda vacía, el piano roll dibuja
contornos de lo que un clic colocaría, usando el mismo
`stamp.notes_at(pitch, scale)` que usa el propio clic. Con «Acorde en la
tonalidad (séptima)» seleccionado, ves el acorde completo antes de
comprometerte con él.

## El carril de acordes: `chunk_by`

El carril sobre la cuadrícula nombra la armonía en cada paso y luego une
los pasos vecinos que tienen el mismo nombre en una sola caja:

```rust
let labels: Vec<Option<String>> = (0..length)
    .map(|step| analyze(project.pattern().sounding_at(self.channel, step)).label(spelling))
    .collect();

let mut start = 0;
for run in labels.chunk_by(|a, b| a == b) {
    let end = start + run.len();
    if let Some(label) = &run[0] {
        // draw a box from step `start` to step `end`, with the label
    }
    start = end;
}
```

`chunk_by` divide un slice en tramos allí donde el closure dice que dos
vecinos no van juntos. En la demo, 32 etiquetas se convierten en cuatro
tramos: ocho `Some("C")`, ocho `Some("Am")`, ocho `Some("F")`, ocho
`Some("G")`. Cada `run` es un slice del vector original, así que no se
copia nada.

Ejecutar `analyze` para cada paso en cada frame suena a mucho, pero son
como mucho 64 loops pequeños sobre unas pocas notas. Mide antes de
optimizar: aquí no se nota en absoluto.

## Resaltado de escala

`paint_rows` colorea cada fila preguntándole a la escala por su clase de
alturas, con un `match` sobre una tupla de dos `bool`:

```rust
let fill = match (is_root, in_scale) {
    (true, _) => ROOT_ROW,
    (false, true) => SCALE_ROW,
    (false, false) => OTHER_ROW,
};
```

Las teclas de la izquierda reciben un punto para las notas de la
tonalidad (naranja para la fundamental), y las notas fuera de la
tonalidad se dibujan en rojo. Cambia la tonalidad y cada color la sigue
en el frame siguiente, porque no hay nada en caché: el modo inmediato
dibuja desde el estado actual cada vez.

## De vuelta en el rack: un mini piano roll

Un canal cuyas notas son todas pasos simples sigue mostrando botones de
paso. Cualquier otro canal muestra `widgets::mini_roll`, un dibujo
diminuto de sus notas, y hacer clic en él abre allí el piano roll. El
widget tiene que alinearse con los botones de paso, que tienen un hueco
entre ellos:

```rust
let gap = ui.spacing().item_spacing.x;
let step_width = STEP_SIZE.x + gap;
let size = vec2(step_width * length as f32 - gap, STEP_SIZE.y);
```

La primera versión usaba solo `STEP_SIZE.x` y salió más de un cuarto
demasiado corto; la captura de pantalla lo hizo evidente. Mirar el
programa en marcha es parte de probar una interfaz.

El rango de alturas se comprime en la altura de la fila, pero nunca
menos que una octava, así que tres notas cercanas no llenan la fila de
arriba abajo:

```rust
let visible = || notes.iter().filter(|note| note.start < length);
let lowest = visible().map(|note| note.pitch.midi()).min().unwrap_or(0);
let highest = visible().map(|note| note.pitch.midi()).max().unwrap_or(0);
let span = (highest - lowest).max(12) as f32;
```

`visible` es otra vez un closure que fabrica un iterador nuevo cada vez
que se llama, porque `min` y `max` se gastan uno cada uno.

## Conectarlo en la aplicación

`LineupApp` posee un `PianoRoll`, lo abre en el primer canal de
sintetizador al arrancar, y lo alterna con F7 o el botón 🎹. Después de
dibujar los paneles:

```rust
let roll_actions = self.piano_roll.show(ui.ctx(), &self.project, playhead);
for action in roll_actions {
    self.apply_roll(action);
}
```

`self.piano_roll.show(..., &self.project, ...)` presta un campo de forma
mutable y otro de forma inmutable al mismo tiempo, la misma regla de
campos separados que en el hilo de audio. Cada acción cambia entonces el
proyecto y envía una copia nueva de las notas del canal al hilo de
audio:

```rust
fn send_notes(&mut self, channel: usize) {
    let notes = self.project.notes(channel).to_vec();
    self.send(Command::SetNotes { channel, notes });
}
```

`to_vec` copia un slice a un `Vec` nuevo. Esta reserva de memoria ocurre
en el hilo de la interfaz, donde está permitida.

Cuando se elimina un canal, `PianoRoll::channel_removed` o bien cierra
la ventana (su canal ya no existe) o desplaza su número de canal uno
hacia abajo, para que siga mostrando el mismo canal.
