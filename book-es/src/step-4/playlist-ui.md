# La ventana de la lista de reproducción

Archivos: [`crates/lineup-app/src/playlist.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/playlist.rs)
y [`crates/lineup-app/src/app.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app.rs)

## Un diseño nuevo

Los estudios basados en patrones ponen la canción en el centro: la lista de reproducción llena la ventana principal, y el rack de canales y el piano roll
flotan encima de él, apareciendo y desapareciendo con F6 y F7. Ferrous
Lineup ahora hace lo mismo. El panel central muestra la lista de reproducción, y el
rack se mudó a una `egui::Window`, como el piano roll en el paso 3:

```rust
let rack_actions = egui::Window::new(format!("Channel rack · {pattern_name}"))
    .id(egui::Id::unique("channel-rack"))
    .open(&mut rack_open)
    .default_pos([170.0, 380.0])
    .resizable(false)
    .show(&ctx, |ui| {
        channel_rack::show(ui, &self.project, self.pattern, playhead)
    })
    .and_then(|window| window.inner)
    .unwrap_or_default();
```

`Window::show` devuelve `None` cuando la ventana está cerrada, y su
`inner` es `None` cuando la ventana está plegada, así que las acciones
están envueltas dos veces. `.and_then(|window| window.inner)` desenvuelve
el `Option` exterior y devuelve el interior; `.unwrap_or_default()`
convierte `None` en un `Vec` vacío.

Una ventana que no es redimensionable se encoge para ajustarse a su
contenido, pero un área de scroll dentro de ella no sabe qué ancho quieren
sus contenidos hasta que los ha dibujado. La primera captura de pantalla
mostraba un rack de cinco pasos de ancho. El arreglo le dice al área de
scroll el ancho de una fila por adelantado:

```rust
let row_width = controls + length as f32 * (widgets::STEP_SIZE.x + gap);
egui::ScrollArea::both()
    .min_scrolled_width(row_width.min(room.x))
```

## El patrón seleccionado

La app recuerda qué patrón está seleccionado:

```rust
pub struct LineupApp {
    // ...
    /// The selected pattern: the one the channel rack and the piano roll
    /// show, pattern mode plays, and new playlist clips use.
    pattern: usize,
    mode: PlayMode,
    // ...
}
```

Es un campo de la app, no del proyecto: es *hacia dónde miras*, no parte
de la canción, así que no se guarda y seleccionar un patrón no cuenta
como un cambio sin guardar. El rack y el piano roll lo reciben como
argumento (`channel_rack::show(ui, &self.project, self.pattern,
cursor de reproducción)`), así que siguen siendo funciones de lo que se les da.

El cursor de reproducción que reciben depende del modo. En modo patrón, el paso del hilo
de audio *es* un paso del patrón seleccionado. En modo canción es un paso
de la canción, y la app le pregunta al proyecto dónde está el patrón
seleccionado en ese momento:

```rust
fn pattern_playhead(&self) -> Option<usize> {
    let step = self.playhead()?;
    match self.mode {
        PlayMode::Pattern => Some(step),
        PlayMode::Song => self.project.pattern_step_in_song(self.pattern, step),
    }
}
```

Así que mientras suena la canción, el cursor de reproducción naranja del rack corre
siempre que un clip del patrón seleccionado está sonando, y desaparece en
los compases donde no lo está, como en la captura del inicio de este paso.

## Dibujar una rejilla grande: `show_viewport`

La lista de reproducción puede ser mucho más grande que la ventana: 16 pistas de 36
píxeles, y al menos 32 compases de 72 píxeles, más de 2300 píxeles de
ancho. Va en un área de scroll. Pero dos partes *no* deben desaparecer al
hacer scroll: los nombres de las pistas a la izquierda, y los números de
compás arriba.

`ScrollArea::show_viewport` hace esto posible. Su closure recibe la parte
visible del contenido, como un rectángulo medido desde la esquina superior
izquierda del contenido:

```rust
egui::ScrollArea::both()
    .auto_shrink(false)
    .show_viewport(ui, |ui, viewport| {
        self.grid(ui, viewport, project, current, playhead, &mut actions)
    });
```

`grid` reserva el tamaño completo del contenido y luego calcula dónde está
la parte visible en pantalla:

```rust
let (rect, response) = ui.allocate_exact_size(size, Sense::click_and_drag());
let visible = viewport.translate(rect.min.to_vec2());
```

`rect` es todo el contenido en coordenadas de pantalla (su esquina
superior izquierda se mueve hacia arriba y a la izquierda al hacer
scroll). Sumar esa esquina a `viewport` da `visible`, la parte de la
pantalla donde se muestra el contenido. Entonces:

- las pistas y los clips se dibujan en su lugar del contenido, así que
  hacen scroll;
- los nombres de las pistas se dibujan en `visible.left()`, y la regla en
  `visible.top()`, así que se quedan quietos mientras los clips se
  deslizan por debajo.

Se dibujan *después* de los clips, así que los tapan. La esquina donde se
encuentran se dibuja la última de todas.

La aritmética de pasos, pistas y píxeles vive en un pequeño struct `Copy`,
como en el piano roll:

```rust
#[derive(Debug, Clone, Copy)]
struct Layout {
    /// The top-left corner of step 0 of track 0.
    origin: Pos2,
    tracks: usize,
}

impl Layout {
    fn x(self, step: usize) -> f32 {
        self.origin.x + step as f32 * STEP_WIDTH
    }

    fn track_top(self, track: usize) -> f32 {
        self.origin.y + track as f32 * TRACK_HEIGHT
    }
    // ... clip_rect, cell_at
}
```

## Widgets reales en una zona pintada: `ui.put`

Todo en la rejilla se pinta a mano, pero los nombres de las pistas deben
ser campos de texto editables y las luces de silencio deben ser la misma
luz que en el rack. `ui.put` coloca cualquier widget en el rectángulo que
elijas:

```rust
let light_response = ui
    .put(light, |ui: &mut Ui| widgets::mute_light(ui, enabled))
    .on_hover_text("Click to mute or unmute the track");

let mut name = track.name.clone();
let field = egui::TextEdit::singleline(&mut name)
    .frame(egui::Frame::NONE)
    .text_color(HEADER_TEXT);
if ui.put(name_rect, field).changed() {
    actions.push(PlaylistAction::RenameTrack { track: index, name });
}
```

- Un closure `|ui: &mut Ui| -> Response` cuenta como un widget: egui
  implementa su trait `Widget` para cualquier closure así. Así es como un
  widget con estilo de función como `mute_light` se puede pasar a `put`.
- El campo de texto edita una *copia* del nombre. Si cambió, la copia va a
  una acción `RenameTrack`, y la app cambia el proyecto. La lista de reproducción
  mantiene la regla de que él nunca cambia el proyecto.
- `Frame::NONE` quita el borde del campo, para que parezca una etiqueta
  hasta que haces clic en él.

## Entrada del ratón, y un bug encontrado por un robot

Colocar, mover y borrar clips sigue el patrón del piano roll: una
pulsación decide qué es el arrastre, y el arrastre sigue al puntero hasta
que se suelta el botón.

```rust
struct ClipDrag {
    index: usize,
    grab: usize,
}
```

Probar con `xdotool` (un programa que mueve el ratón por ti) encontró un
bug. Un script pulsó sobre un clip y bajó una pista en unos pocos
milisegundos, y la lista de reproducción *añadió un clip nuevo* debajo en lugar de
mover el viejo. egui informa de la posición del puntero en el momento en
que se dibuja el frame. Cuando la pulsación y el movimiento llegan en el
mismo frame, «dónde está ahora el puntero» ya está una pista más abajo,
sobre espacio vacío.

El arreglo es atender la pulsación donde el botón bajó, cosa que egui
recuerda como el **origen de la pulsación**:

```rust
// A press is handled where the button went down. A quick hand can
// press and move in the same frame, and the clip or empty cell
// under the pointer *now* may not be the one that was clicked.
let pressed_at = press_origin
    .filter(|pos| primary_pressed && response.hovered() && cells.contains(*pos));
if let Some(pos) = pressed_at {
    let (track, step) = layout.cell_at(pos);
    match clip_at(project, layout, pos) {
        Some(index) => { /* start moving that clip, and select its pattern */ }
        None => { /* add a clip of the selected pattern, and start moving it */ }
    }
}
```

`Option::filter` conserva el valor solo si el closure devuelve `true`, así
que `pressed_at` es `Some` solo en el frame de una pulsación, sobre la
rejilla. El piano roll tenía el mismo bug y recibió el mismo arreglo. Una
persona rara vez se mueve tan rápido, pero «rara vez» no es «nunca», y en
una computadora lenta un frame dura más.

Las posiciones se ajustan a la rejilla que elijas en la barra de
herramientas:

```rust
fn floor(self, step: usize) -> usize {
    step / self.steps() * self.steps()
}
```

La división de enteros tira el resto, así que `step / 16 * 16` es «la
línea de compás en este paso o antes». Con Ajuste en Pulso usa 4, y con Paso
usa 1 (sin ajuste).

## Dibujar las notas de un clip: `Box<dyn Fn>`

Cada clip muestra un boceto de su patrón. Los patrones de batería se ven
mejor como un step sequencer diminuto (una línea por canal), cualquier
cosa con melodía como un piano roll diminuto (las notas más agudas, más
arriba). Los dos necesitan formas distintas de convertir una nota en una
altura, elegidas una vez por clip:

```rust
let height: Box<dyn Fn(usize, u8) -> f32> = if steps_only {
    let used: Vec<usize> = (0..channels)
        .filter(|&channel| !pattern.notes(channel).is_empty())
        .collect();
    let count = used.len().max(1) as f32;
    Box::new(move |channel, _| {
        let row = used.iter().position(|&c| c == channel).unwrap_or(0) as f32;
        1.0 - (row + 0.5) / count
    })
} else {
    // ... lowest and highest note, as in the rack's mini piano roll
    Box::new(move |_, midi| ((midi - lowest) as f32 + centering) / span)
};
```

Cada closure tiene su propio tipo sin nombre, así que las dos ramas del
`if` producen valores de tipos distintos, y una variable solo puede tener
un tipo. `Box<dyn Fn(usize, u8) -> f32>` es el tipo común: «un closure en
el montículo que recibe un canal y una altura y devuelve una altura».
`dyn Fn` es un objeto trait, como `dyn Error` en el capítulo de guardar.
La palabra clave `move` hace que cada closure tome la propiedad de lo que
usa (`used`, `lowest`...), que debe vivir más que el `if` que los creó.

Esto corre en el hilo de UI, una vez por clip y por frame, así que la
pequeña reserva de memoria del `Box` no importa. En el hilo de audio sí
importaría.

## Teclas que llegan al widget equivocado

Espacio arrancaba y paraba la música en el paso 3 comprobando si algún
widget tenía el foco del teclado:

```rust
let typing = ctx.memory(|memory| memory.focused().is_some());
```

Con botones por todas partes, eso se rompió. Hacer clic en ▶ le da el
foco al botón, y egui deja que un botón con foco se pulse con Espacio. Así
que Espacio hacía dos cosas a la vez: el atajo alternaba la reproducción, y
el botón la volvía a alternar. La versión nueva hace una pregunta más
estrecha, y le quita la tecla a los widgets una vez que la ha usado:

```rust
let typing = ctx.text_edit_focused();
let pressed =
    |key| ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, key));
if !typing && pressed(egui::Key::Space) {
    self.set_playing(!self.playing);
}
```

Los atajos corren antes de que se dibuje ningún panel, así que una vez que
`consume_key` se ha llevado la pulsación de Espacio, el botón con foco nunca
la ve.

Una sorpresa más: el menú ⋯ de patrones tiene un campo de texto para el
nombre del patrón, y los menús de egui se cierran con cualquier clic,
incluido un clic *dentro* del menú sobre ese campo. El menú ahora usa otro
comportamiento de cierre:

```rust
let stay_open = egui::containers::menu::MenuConfig::new()
    .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside);
let menu = egui::containers::menu::MenuButton::new("⋯").config(stay_open);
```

Los dos bugs se encontraron de la misma manera: manejando el programa real
con clics y teclas guionizados, y mirando capturas de pantalla. Las
pruebas unitarias comprueban la música; para la ventana, nada sustituye a
usarla.
