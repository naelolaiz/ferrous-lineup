# Los editores: rack de canales, piano roll y lista de reproducción

Archivos: [`crates/lineup-app/src/channel_rack.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/channel_rack.rs),
[`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs),
[`crates/lineup-app/src/playlist.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/playlist.rs),
[`crates/lineup-app/src/widgets.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/widgets.rs)
y [`crates/lineup-app/src/app.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app.rs)

Tres ventanas editan la música en sí. La **lista de reproducción** llena
la ventana principal: patrones colocados como clips sobre pistas, que
juntos forman la canción. El **rack de canales** (F6) muestra un patrón
como filas de pasos, una fila por canal. El **piano roll** (F7) muestra
un canal de un patrón como notas sobre una cuadrícula, con las notas
agudas arriba. El rack y el piano roll flotan sobre la lista de
reproducción, como cuenta el [capítulo
anterior](egui.md#ventanas-sobre-la-lista-de-reproducción).

Las tres están hechas de la misma manera, y eso es lo primero de lo que
trata este capítulo.

## Dibujar devuelve acciones

Ninguno de los editores cambia el proyecto. Cada uno solo lo mira y
devuelve lo que hizo el usuario, como una lista de acciones. Las del
rack son un enum:

```rust
pub enum RackAction {
    PlayPattern,
    // ...
    SelectChannel(Option<usize>),
    ToggleStep { channel: usize, step: usize },
    ClearStep { channel: usize, step: usize },
    Preview(usize),
    SetVolume { channel: usize, volume: f32 },
    ToggleMute(usize),
    Remove(usize),
    OpenPianoRoll(usize),
    AddDrum(Drum),
    AddSynth,
    // ...sending over MIDI, instruments...
    LoadSample { channel: Option<usize>, path: PathBuf },
}

pub fn show(
    ui: &mut Ui,
    project: &Project,
    pattern: usize,
    selected: Option<usize>,
    playhead: Option<usize>,
    playback: Option<RackPlayback>,
    locks: bool,
) -> Vec<RackAction>
```

`project: &Project` es un **préstamo compartido**: la función puede leer
el proyecto pero no cambiarlo, y el compilador lo hace cumplir. Después
la app aplica las acciones:

```rust
let rack_actions = rack
    .show(ctx, |ui| {
        // ...
        channel_rack::show(ui, &self.project, self.pattern, selected, playhead, /* ... */)
    })
    .and_then(|window| window.inner)
    .unwrap_or_default();
for action in rack_actions {
    self.apply(action);
}
```

`Window::show` devuelve `None` cuando la ventana está cerrada, y su
`inner` es `None` cuando la ventana está plegada, así que las acciones
vienen envueltas dos veces. `.and_then(|window| window.inner)`
desenvuelve el `Option` de fuera y devuelve el de dentro;
`.unwrap_or_default()` convierte `None` en un `Vec` vacío.

Esta separación también resuelve un rompecabezas de préstamos. Mientras
el rack se dibuja, tiene un préstamo compartido del proyecto, así que no
podría cambiarlo a la vez (eso necesitaría un `&mut` al mismo tiempo,
cosa que el borrow checker prohíbe). Reunir primero las acciones y
aplicarlas cuando el dibujo ha terminado mantiene los dos préstamos
separados en el tiempo.

`apply` es un único `match` con una rama por acción. Cada rama que
cambia la canción pasa por `edit` o `edit_pattern`, que llevan el
historial de deshacer y le dicen al hilo de audio qué cambió (ver
[Decirle al hilo de audio lo que ha cambiado](sync.md)):

```rust
RackAction::ToggleStep { channel, step } => {
    self.edit_pattern("change steps", |pattern| {
        pattern.toggle(channel, step);
    });
}
```

La misma acción hace lo mismo venga de donde venga: un clic, una tecla o
un menú. El piano roll devuelve `RollAction` y la lista de reproducción
`PlaylistAction`, y la app las aplica de la misma manera.

## El patrón seleccionado

El rack y el piano roll muestran **un patrón**, el seleccionado. La app
lo guarda:

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

Es un campo de la app, no del proyecto: es *dónde estás mirando*, no
parte de la canción, así que no se guarda, y seleccionar un patrón no
cuenta como un cambio sin guardar. Los editores lo reciben como
argumento, así que siguen siendo funciones de lo que se les da.

El cursor de reproducción que reciben depende del modo. En el modo
patrón, el paso del hilo de audio *es* un paso del patrón seleccionado.
En el modo canción es un paso de la canción, y la app le pregunta al
proyecto dónde está el patrón seleccionado en ese momento:

```rust
fn pattern_playhead(&self) -> Option<usize> {
    let step = self.playhead()?;
    match self.mode {
        PlayMode::Pattern => Some(step),
        PlayMode::Song => {
            // ...a pattern played over the song gives its own step...
            let (step, _) = self.song_position()?;
            self.project.pattern_step_in_song(self.pattern, step)
        }
    }
}
```

`pattern_step_in_song` busca un clip del patrón, en una pista que no
esté silenciada, que cubra el paso de la canción, y devuelve `None` si
no hay ninguno. Así, mientras suena la canción, el cursor del rack corre
cada vez que suena un clip del patrón seleccionado, y desaparece en los
compases donde no.

## El rack de canales

![El rack de canales mientras suena la canción: una fila de lucecitas
sobre los pasos, la octava encendida, y la octava columna de cada fila
teñida de naranja.](../images/step-9-rack-lights-song.png)

### Escribir tus propios widgets

egui tiene botones y deslizadores, pero no botones de paso ni perillas.
Hacer uno lleva tres movimientos: reservar espacio, leer el ratón,
pintar.

```rust
pub fn step_button(ui: &mut Ui, on: bool, beat_group: usize, is_playhead: bool) -> Response {
    let (rect, response) = ui.allocate_exact_size(step_size(ui), Sense::click());

    if ui.is_rect_visible(rect) {
        let mut fill = if on {
            STEP_ON
        } else {
            STEP_OFF[beat_group % 2]
        };
        if response.hovered() {
            fill = fill.gamma_multiply(1.25);
        }
        if is_playhead {
            // The whole column that is sounding lights up, down every row.
            fill = fill.lerp_to_gamma(PLAYHEAD, if on { 0.25 } else { 0.35 });
        }
        let painter = ui.painter();
        // ...a dark edge along the bottom, the pad, a light strip on a
        // step that is on, an orange outline on the step playing...
    }
    response
}
```

- `allocate_exact_size` reserva un rectángulo en la distribución y dice
  qué interacciones queremos (`Sense::click()`). Devuelve el rectángulo
  y un `Response` que describe lo que hizo el ratón ahí en este frame.
  `step_size(ui)` es el tamaño del paso multiplicado por el zoom del
  rack.
- `is_rect_visible` se salta el pintado de las filas que quedan fuera
  de la vista.
- El `Painter` dibuja formas. Los colores son constantes `Color32`
  definidas arriba del archivo, así que la paleta vive en un solo sitio.
- `lerp_to_gamma` mezcla algo de naranja en el paso que suena, un poco
  menos en un paso encendido, para que un paso encendido siga blanco.
- Devolver el `Response` permite a quien llama usarlo como cualquier
  widget de serie: `if response.clicked() { ... }`.

Los pulsos alternan colores, para que puedas contarlos, y vuelven a
empezar en cada compás para que todos los compases se vean iguales: en
3/4, oscuro, claro, oscuro. El rack le pregunta al compás en qué pulso
cae cada paso (`time.beat_at(step).index`), así que en 6/8 agrupa sus
pasos de tres en tres.

La perilla usa `Sense::click_and_drag()`, convierte el movimiento
vertical del ratón en un cambio de valor, y llama a
`response.mark_changed()` para que quien la llama pueda usar
`.changed()` como en un deslizador. Su aguja se dibuja con un poco de
trigonometría: el ángulo recorre 270 grados, y `(sin, -cos)` del ángulo
da la dirección (la *y* de la pantalla crece hacia abajo, de ahí el
menos).

### Menús

```rust
name.context_menu(|ui| self.channel_menu(ui, index, channel, locked, actions));
```

`context_menu` abre un menú con el clic derecho; `menu_button` abre uno
con un clic normal (el botón «Añadir canal»). Los dos reciben un closure
que dibuja el contenido del menú, y `ui.close()` cierra el menú tras una
elección. `add_enabled_ui(false, ...)` pone en gris todo lo que hay
dentro, y así se desactiva «Añadir canal» cuando el rack está lleno:

```rust
let rack_full = project.channels().len() >= MAX_CHANNELS;
ui.add_enabled_ui(!rack_full, |ui| {
    ui.menu_button(tr("➕ Add channel"), |ui| { /* ... */ });
});
```

### Arrastrar y soltar

egui informa de los archivos que se arrastran sobre la ventana en
`input.raw.hovered_files`, y de los que se sueltan en
`input.raw.dropped_files`. Mientras dibuja, el rack guarda el rectángulo
de cada fila; después busca la fila que está bajo el ratón:

```rust
let target_row = pointer.and_then(|p| row_rects.iter().position(|rect| rect.contains(p)));
```

`position` devuelve el índice del primer elemento para el que el closure
es verdadero, o `None`. Un archivo WAV soltado sobre una fila sustituye
el sonido de ese canal; un archivo soltado en cualquier otro sitio se
convierte en un canal nuevo con el nombre del archivo
(`path.file_stem()` es el nombre del archivo sin su extensión). Un
archivo de proyecto, MIDI o de hoja de acordes, o una grabación, se
deja para la app, que lo abre, lo importa o lo añade a la canción como
toma.

La carga ocurre en `LineupApp::load_sample`, en el hilo de la ventana,
porque leer archivos es lento y nunca debe ocurrir en el hilo de audio.
Lee el archivo *antes* de cambiar nada, así que un archivo roto deja el
canal como estaba y pone la razón en la barra de estado.

### Una fila de luces sobre los pasos

Una caja de ritmos tiene una fila de lucecitas, una sobre cada paso, y
la luz sobre el paso que suena corre junto con la música. El rack
también tiene esa fila. Las luces deben quedar exactamente sobre los
pasos, pero los pasos empiezan después de la luz de silencio, la
perilla, el candado y el nombre del canal, y sus anchos cambian con el
zoom del rack. En lugar de sumar esos anchos (y equivocarse la próxima
vez que se añada un control a una fila), el rack deja que las filas se
lo digan:

```rust
let (lights, _) = ui.allocate_exact_size(
    egui::vec2(1.0, widgets::step_lights_height(ui)),
    egui::Sense::hover(),
);
let mut steps_left = None;
for (index, channel) in project.channels().iter().enumerate() {
    let row = ui.horizontal(|ui| self.row(ui, index, channel, actions));
    steps_left.get_or_insert(row.inner);
    row_rects.push(row.response.rect);
}
if let Some(left) = steps_left {
    let marks = step_marks(self.time, self.pattern.length());
    widgets::step_lights(ui, lights, left, &marks, self.playhead);
}
```

`allocate_exact_size` reserva una franja vacía arriba, así que las filas
van debajo. Cada `row` devuelve dónde empiezan sus pasos
(`ui.cursor().left()` justo antes del primer paso); `get_or_insert`
guarda el valor de la primera fila e ignora los demás. Cuando las filas
terminan, `step_lights` pinta en la franja reservada. egui dibuja todo
al final del frame, así que pintar en un espacio reservado antes no es
problema.

`step_lights` recibe un slice de `StepMark`, un enum pequeño que dice si
cada paso empieza un compás, empieza un pulso o ninguna de las dos
cosas:

```rust
pub enum StepMark {
    Bar,
    Beat,
    Plain,
}
```

La luz sobre el primer paso de cada compás es un poco más brillante que
las demás, y la luz sobre el primer paso de cada pulso un poco más
brillante que las de los pasos intermedios, así que la fila se puede
contar como los pasos que tiene debajo. Cada luz se coloca un ancho de
paso (el botón más el hueco que lo sigue) después de la anterior, la
misma aritmética que usa el mini piano roll, así que luces, botones y
vistas previas se alinean con cualquier zoom.

### Un mini piano roll para las melodías

Un canal cuyas notas son todas pasos simples muestra botones de paso.
Cualquier otro canal muestra `widgets::mini_roll`, un dibujo diminuto de
sus notas, y un clic en él abre el piano roll. El widget tiene que
alinearse con los botones de paso, que tienen un hueco entre ellos:

```rust
let gap = ui.spacing().item_spacing.x;
let step_size = step_size(ui);
let step_width = step_size.x + gap;
let size = vec2(step_width * length as f32 - gap, step_size.y);
```

El rango de alturas se comprime en la altura de la fila, pero nunca
menos de una octava, para que tres notas cercanas no llenen la fila de
arriba abajo:

```rust
let visible = || notes.iter().filter(|note| note.start < length);
let lowest = visible().map(|note| note.pitch.midi()).min().unwrap_or(0);
let highest = visible().map(|note| note.pitch.midi()).max().unwrap_or(0);
let span = (highest - lowest).max(12) as f32;
```

`visible` es un closure que crea un iterador nuevo cada vez que se le
llama, porque `min` y `max` gastan uno cada uno. La columna que suena se
rellena de un naranja transparente (`gamma_multiply(0.25)` deja el color
a una cuarta parte de su fuerza), igual que se tiñen los botones de
paso.

### La batería en otro patrón, y un rack que se ajusta

Las canciones suelen guardar su batería en un patrón propio, «Batería de
A» junto a «A». El rack de «A» mostraría filas de batería sin nada, y
parecería roto. Así que cuando un patrón no tiene pasos de batería y
otros sí, `drums_elsewhere` lo dice encima de las filas, con un botón
para cada uno de esos patrones.

El rack se abre tan grande como sus filas. `natural_size` las suma (los
controles, los pasos, las luces, el menú «Añadir canal», sitio para una
barra de desplazamiento), y la ventana recibe ese tamaño durante un
frame cada vez que las filas cambian, como las [ventanas del tamaño de
lo que muestran](egui.md#ventanas-del-tamaño-de-lo-que-muestran).
Se puede hacer más pequeña igualmente, y entonces se desplaza. Cuando
caben todas las filas, la rueda del ratón sola desplaza los pasos hacia
los lados. egui tiene un ajuste de estilo justo para eso, que funciona
cuando el scroll area permite una sola dirección:

```rust
ui.style_mut().always_scroll_the_only_direction = true;
let vertical = rows_height > ui.available_height();
let scroll = egui::ScrollArea::new([true, vertical])
```

## El piano roll

![El piano roll en Fa mayor: Si♭ está resaltado, Si no, y las notas Si
están en rojo.](../images/step-3-f-major.png)

### Una ventana con estado propio

El rack de canales es una función: todo lo que muestra sale del
proyecto. El piano roll necesita recordar cosas que el proyecto no
conoce, de un frame al siguiente: qué canal muestra, qué dibuja un clic,
las notas seleccionadas, un arrastre en curso. Así que es un struct con
métodos:

```rust
pub struct PianoRoll {
    pub open: bool,
    channel: usize,
    stamp: ChordStamp,
    /// The length of new notes, in steps. Resizing a note changes it, so
    /// the next note drawn is as long as the last one edited.
    note_length: usize,
    drag: Option<Drag>,
    group_drag: Option<GroupDrag>,
    selection: Vec<NoteEvent>,
    ghosts: bool,
    zoom: Vec2,
    // ...
}
```

Aun así nunca cambia el proyecto él mismo. `show` devuelve una lista de
`RollAction`, como hace el rack:

```rust
pub enum RollAction {
    /// Add one note, or the notes of a chord.
    AddNotes { channel: usize, notes: Vec<NoteEvent> },
    RemoveNote { channel: usize, index: usize },
    /// Replace a note that was moved or resized.
    ReplaceNote { channel: usize, index: usize, note: NoteEvent },
    /// Replace some notes with others, all at once: a moved selection,
    /// a tool, new velocities. `label` names the undo step.
    ReplaceNotes { channel: usize, old: Vec<NoteEvent>, new: Vec<NoteEvent>, label: &'static str },
    /// Play some notes right now, without adding them.
    Preview { channel: usize, pitches: Vec<Note> },
    SetScale(Scale),
    /// Show another pattern: one where the channel has notes.
    SelectPattern(usize),
    /// Open the channel rack, where drum steps are edited.
    OpenRack,
    // ...
}
```

Así que el struct guarda estado de la *vista*, que nunca se guarda en el
archivo, y el proyecto guarda el estado del *documento* (las notas), que
sí.

La ventana se construye así:

```rust
// `open` is a copy: the window needs `&mut open` while the closure
// below needs `&mut self`, and both cannot borrow `self` at once.
let mut open = self.open;
let window = egui::Window::new(roll_title)
    .id(id)
    .open(&mut open)
    .default_pos([150.0, 160.0])
    .constrain_to(area);
// ...the size it opens at...
window.show(ctx, |ui| {
    self.contents(ui, project, pattern_notes, playhead, held, &mut actions)
});
self.open = open;
```

- El título cambia con el patrón y el canal, y egui normalmente
  identifica una ventana por su título. `.id(...)` le da en cambio una
  identidad fija, para que conserve su posición y su tamaño cuando
  cambias de canal.
- `.open(&mut open)` añade el botón de cerrar: egui pone `open` a
  `false` cuando se pulsa.
- Si la ventana tomara prestado `&mut self.open` mientras el closure
  toma prestado `&mut self` (para llamar a `self.contents`), `self.open`
  quedaría prestado de forma mutable dos veces a la vez, cosa que Rust
  prohíbe. Copiar el `bool` fuera y volver a meterlo evita el solape.

### Píxeles y música: `Grid`

Casi todo el código de dibujo convierte pasos y alturas en posiciones de
pantalla, o la posición del puntero de vuelta en un paso y una altura.
Esa aritmética vive en un solo sitio:

```rust
#[derive(Debug, Clone, Copy)]
struct Grid {
    /// Where all the steps and rows are, even those scrolled out of view.
    rect: Rect,
    step_width: f32,
    /// How tall a row (one pitch) is: the vertical zoom.
    row_height: f32,
    length: usize,
    time: TimeSignature,
    /// The left edge of the keyboard. The keyboard stays at the left of
    /// the view while the grid scrolls sideways under it.
    keys_left: f32,
}

impl Grid {
    /// The x coordinate where `step` begins.
    fn x(self, step: usize) -> f32 {
        self.rect.left() + step as f32 * self.step_width
    }

    /// The y coordinate of the top of a pitch's row (high notes at the top).
    fn row_top(self, midi: u8) -> f32 {
        self.rect.top() + (HIGHEST - midi) as f32 * self.row_height
    }

    /// The step and pitch under a point. Points outside the grid give the
    /// nearest cell, so a drag can go past the edges.
    fn cell_at(self, pos: Pos2) -> (usize, Note) {
        let row = ((pos.y - self.rect.top()) / self.row_height)
            .floor()
            .clamp(0.0, (HIGHEST - LOWEST) as f32) as u8;
        (self.step_at(pos.x), Note::new(HIGHEST - row))
    }
    // ...keys, cells, step_at, note_rect, text_size
}
```

Las notas agudas están arriba, así que la fila cuenta hacia abajo desde
`HIGHEST`. `cell_at` limita el valor, así que un arrastre que sale de la
cuadrícula sigue obteniendo la celda más cercana, y devuelve una
**tupla**, `(usize, Note)`, que quien llama desarma con
`let (step, pitch) = grid.cell_at(pos);`. `note_rect` dibuja una nota
desde su inicio exacto hasta su final exacto, incluido cuánto se aparta
del paso, así que un tresillo queda entre dos pasos, como se oye.

Como `Grid` es `Copy` y sus métodos toman `self` por valor, se puede
pasar a todas las funciones de dibujo sin ningún `&` ni lifetimes. Para
un struct de unos pocos números, copiar es tan barato como pasar una
referencia.

Al 100% el patrón entero cabe a lo ancho de la ventana, así que el
ancho del paso se calcula en cada frame a partir del ancho de la
ventana. Al hacer zoom los pasos se ensanchan, y la cuadrícula se
desplaza hacia los lados bajo el teclado.

### Un teclado que se queda, carriles que siguen

La cuadrícula está dentro de un scroll area, pero dos cosas no deben
desplazarse con ella. Las teclas del piano se dibujan en el borde
izquierdo de lo *visible* (`keys_left: visible.left()`), encima de la
cuadrícula, así que las notas se deslizan por debajo. El carril de
acordes sobre la cuadrícula y el carril de velocidad debajo se dibujan
fuera del scroll area, así que no saben cuánto se ha desplazado la
cuadrícula. Un struct pequeño lleva lo que necesitan para alinearse:

```rust
/// What the chord and velocity lanes need to line up with the grid.
#[derive(Debug, Clone, Copy)]
struct Lanes {
    step_width: f32,
    /// How far the grid is scrolled sideways, in pixels.
    scroll_x: f32,
    length: usize,
}
```

Cada carril construye su propia `Grid` a partir de él (`Lanes::grid`),
desplazada por `scroll_x`, y usa las mismas funciones que la cuadrícula
de notas para encontrar dónde está un paso. Una única fuente de verdad
para «¿dónde está el paso 12?» es lo que mantiene alineadas tres franjas
dibujadas por separado.

### La entrada del ratón como una pequeña máquina de estados

egui informa de lo que hace el ratón *en este frame*. Un arrastre dura
muchos frames, así que el piano roll lo recuerda en un enum:

```rust
enum Drag {
    /// Moving a note. `grab` is how many steps after the note's start the
    /// pointer took hold of it, so the note does not jump under the pointer.
    Move { index: usize, grab: usize },
    /// Changing a note's length by dragging its right end.
    Resize { index: usize },
}
```

`handle_input` es largo, pero cada parte responde a una pregunta:

1. **¿Se acaba de pulsar el botón izquierdo sobre la cuadrícula?** Si
   dio en una nota, empieza un `Move`, o un `Resize` si la pulsación fue
   cerca del extremo derecho de la nota. Si dio en una celda vacía,
   añade lo que dibuja un clic (una nota o un acorde); una sola nota
   nueva también empieza un `Move`, para que puedas colocar y arrastrar
   en un solo gesto.
2. **¿Está el botón derecho pulsado sobre una nota?** Bórrala.
   Comprobarlo en cada frame significa que puedes barrer varias notas
   para borrarlas todas.
3. **¿Está el botón izquierdo arriba?** Entonces cualquier arrastre ha
   terminado.
4. **¿Hay un arrastre en curso?** Calcula dónde estaría ahora la nota y,
   si eso difiere de donde está, envía un `ReplaceNote`.

La cuarta parte usa una **let chain** con tres condiciones, y la
sintaxis de actualización de structs para cambiar uno o dos campos:

```rust
if let Some(drag) = self.drag
    && let Some(pos) = pointer
    && let Some(&note) = notes.get(drag.index())
{
    let (step, pitch) = grid.cell_at(pos);
    let pitch = self.snapped(pitch, scale);
    let changed = match drag {
        Drag::Move { grab, .. } => NoteEvent {
            pitch,
            start: step.saturating_sub(grab),
            ..note
        },
        Drag::Resize { .. } => {
            let length = (step + 1).saturating_sub(note.start).max(1);
            self.note_length = length;
            // ...
            NoteEvent { length, trim, ..note }
        }
    };
    if changed != note {
        // ...play the new pitch, keep the note selected...
        actions.push(RollAction::ReplaceNote { channel, index: drag.index(), note: changed });
    }
}
```

`Some(&note)` en un patrón copia la nota fuera del `&NoteEvent` que
devuelve `get`. Comparar `changed != note` (gracias al `PartialEq`
derivado) hace que no se envíe nada mientras el puntero se quede dentro
de la misma celda, aunque este código se ejecute en cada frame.
`snapped` lleva la altura a la tonalidad cuando **Ajustar a la escala**
está encendido.

Dos detalles hacen que los clics caigan donde se quería. Un clic derecho
rápido puede pulsar y soltar el botón dentro de un mismo frame, así que
«¿está el botón pulsado?» ya es falso cuando se dibuja el frame. El
código pregunta en cambio «¿se pulsó en este frame, o está pulsado?»:

```rust
pointer.secondary_pressed() || pointer.secondary_down(),
```

Y en el frame en que baja el botón izquierdo, la pulsación se atiende
donde el botón *bajó*, que `widgets::press_pos` lee de los eventos del
frame (ver [Clics que hacen lo que querías
decir](egui.md#clics-que-hacen-lo-que-querías-decir)): una
mano rápida puede pulsar y seguir moviéndose en el mismo frame.

Cuando el puntero está sobre una celda vacía, el piano roll dibuja el
contorno de lo que colocaría un clic, a partir del mismo `stamp_notes`
que usa el clic. Con «Acorde de la tonalidad (séptima)» elegido, ves el
acorde entero antes de colocarlo.

### La selección guarda notas, no posiciones

La manera más simple de recordar qué notas están seleccionadas serían
sus posiciones en la lista del canal: `selection: Vec<usize>`. El
arrastre de una sola nota hace exactamente eso
(`Drag::Move { index, .. }`), y ahí funciona porque nada más cambia la
lista durante un arrastre.

Una selección vive mucho más que un arrastre. Mientras hay notas
seleccionadas, una herramienta las saca de la lista y pone las nuevas al
final, deshacer coloca un proyecto anterior entero, y borrar una nota
sube un lugar a todas las que van después. Después de cualquiera de esas
cosas, la posición 3 puede ser otra nota, o no ser ninguna nota. Una
selección de posiciones seleccionaría en silencio las notas
equivocadas, y la siguiente herramienta cambiaría notas que nunca
elegiste.

Así que la selección guarda las propias notas:

```rust
/// The selected notes, by value. A note that is no longer in the
/// pattern (after an undo, say) is simply not found, and so not
/// selected: the selection can never point at the wrong note.
selection: Vec<NoteEvent>,
```

Esto funciona porque un canal nunca tiene dos notas en el mismo lugar:
la regla del patrón (misma altura, mismo inicio, mismo desplazamiento)
hace único el valor de cada nota, de modo que un valor nombra
exactamente una nota. `NoteEvent` es `Copy` y `PartialEq`, así que
guardar copias y compararlas es barato y fácil.

Al comienzo de cada frame, las notas que han desaparecido se olvidan:

```rust
let notes = pattern.notes(self.channel);
self.selection.retain(|note| notes.contains(note));
```

Y cuando el piano roll sustituye algunas notas, hace que la selección
las siga, de modo que pulsar ↑ tres veces mueve el mismo acorde tres
veces:

```rust
fn replace(
    &mut self,
    pattern: &Pattern,
    actions: &mut Vec<RollAction>,
    label: &'static str,
    old: Vec<NoteEvent>,
    new: Vec<NoteEvent>,
) -> Option<Vec<NoteEvent>> {
    let new = tidy_notes(new);
    if old == new || !pattern.can_replace_notes(self.channel, &old, &new) {
        return None;
    }
    let was_selected = old.iter().any(|note| self.selection.contains(note));
    self.selection.retain(|note| !old.contains(note));
    if was_selected {
        self.selection.extend(&new);
    }
    actions.push(RollAction::ReplaceNotes {
        channel: self.channel,
        old,
        new: new.clone(),
        label,
    });
    Some(new)
}
```

`replace` es la única puerta por la que pasa todo cambio de grupo: las
herramientas, el arrastre de grupo y el carril de velocidad. Ordena las
notas nuevas igual que lo hará el patrón, le pregunta al patrón si el
cambio está permitido, y solo entonces envía la acción. Ordenarlas
también aquí significa que la selección guarda exactamente las notas que
estarán en el patrón, y comprobar antes significa que un movimiento
rechazado no hace nada en absoluto en lugar de hacer media cosa.

La comparación cuesta un poco: `contains` recorre la lista entera, así
que comprobar cada nota contra la selección lleva un tiempo proporcional
al número de notas multiplicado por el tamaño de la selección. Para un
piano roll con unos cientos de notas, eso está muy por debajo de un
milisegundo. Si algún día importara, un `HashSet<NoteEvent>` haría cada
comprobación de tiempo constante (`NoteEvent` ya deriva `Hash` para
eso).

### Seleccionar con el ratón

Ctrl+clic sobre una nota la añade a la selección, o la quita:

```rust
if let Some(at) = self.selection.iter().position(|&n| n == note) {
    self.selection.remove(at);
} else {
    self.selection.push(note);
}
```

`input.modifiers.command` es «Ctrl» en Linux y Windows y «⌘» en un Mac,
así que el mismo código sigue las costumbres de cada sistema.

Ctrl+arrastrar sobre un espacio vacío dibuja un recuadro. Solo se guarda
la esquina donde empezó; la selección se reconstruye desde cero en cada
frame, a partir de esa esquina y del puntero, así que volver a achicar el
recuadro deselecciona notas:

```rust
if let Some(origin) = self.selection_box
    && let Some(pos) = pointer
{
    let area = Rect::from_two_pos(origin, pos);
    self.selection = notes
        .iter()
        .filter(|note| note.start < grid.length && grid.note_rect(note).intersects(area))
        .copied()
        .collect();
}
```

`Rect::from_two_pos` hace un rectángulo a partir de dos esquinas
cualesquiera, en el sentido en que se haya arrastrado el recuadro, e
`intersects` es verdadero cuando dos rectángulos se solapan aunque sea
un poco, así que basta con que el recuadro toque una nota.

### Mover un grupo

Pulsar una nota seleccionada (en cualquier sitio salvo su extremo
derecho, que sigue cambiando el largo) empieza un arrastre de grupo:

```rust
struct GroupDrag {
    /// The cell where the drag started.
    from_step: usize,
    from_pitch: Note,
    /// The notes as they were when the drag started.
    originals: Vec<NoteEvent>,
    /// The notes as they are now, after the moves so far.
    moved: Vec<NoteEvent>,
    /// Whether the notes have moved at all.
    edited: bool,
}
```

En cada frame del arrastre, la posición nueva se calcula a partir de los
*originales*, con la distancia entera desde la celda de inicio hasta el
puntero:

```rust
let steps = step as isize - drag.from_step as isize;
let semitones = i32::from(pitch.midi()) - i32::from(drag.from_pitch.midi());
drag.edited |= steps != 0 || semitones != 0;
let snap = self.snap_to_scale && drag.edited;
let new = tools::shift(&drag.originals, steps, grid.length)
    .and_then(|shifted| tools::transpose(&shifted, semitones))
    .map(|moved| match snap {
        true => tools::snap_to_scale(&moved, scale),
        false => moved,
    })
    .filter(|moved| fits_view(moved));
```

Mover las notas un poco en cada frame (desde `moved`) sería la manera
obvia, pero los pequeños errores se irían sumando: un movimiento
rechazado en el borde del patrón se perdería, y las notas acabarían en
otro sitio que bajo el puntero. Empezar cada vez desde los originales
hace que las notas estén siempre exactamente donde dice el puntero, y
arrastrar de vuelta al inicio las deja exactamente como estaban.

Las herramientas (ver [Herramientas de edición como funciones
simples](../theory/tools.md)) se encadenan con `Option::and_then`:
desplazar, y *si eso funcionó*, transportar el resultado. `map` ajusta
las notas a la tonalidad cuando Ajustar a la escala está encendido, pero
solo una vez que se han movido, así que arrastrar una selección un
semitono arriba y de vuelta abajo la mete en la tonalidad. `filter`
rechaza después las notas que quedan fuera de las filas que muestra el
piano roll. La sustitución va de `moved` (lo que está ahora en el patrón)
a `new`, y `moved` se actualiza con lo que devolvió `replace`.

El arrastre se saca de `self` mientras se trabaja con él:

```rust
if let Some(mut drag) = self.group_drag.take()
    && let Some(pos) = pointer
{
    // ... self.replace(pattern, actions, "move notes", drag.moved.clone(), new) ...
    self.group_drag = Some(drag);
}
```

`self.replace` necesita `&mut self`, el struct entero. Si `drag` fuera
una referencia dentro de `self.group_drag`, el borrow checker se negaría:
una parte de `self` estaría prestada mientras todo `self` se presta de
forma mutable. `Option::take` saca el arrastre y deja `None`, así que
`drag` es un valor aparte y `self` queda libre; el arrastre se vuelve a
poner al final.

Cada frame del arrastre produce un `ReplaceNotes`, y cada uno pasa por
`edit_pattern`. Aun así forman un solo paso de deshacer, porque un paso
sigue abierto mientras se mantiene pulsado un botón del ratón (ver
[Deshacer y rehacer](undo.md#cuándo-termina-un-paso)). Y
como el paso ya está abierto, `edit` no vuelve a copiar el proyecto
entero en cada frame; le envía al hilo de audio lo que cambió como mucho
cada 50 ms, y el resto al soltar el botón. Nada en el piano roll tiene
que saber nada de ninguna de las dos cosas.

### Quién recibe las teclas

Las flechas y Suprimir son útiles en el piano roll, pero el rack y la
lista de reproducción también las usan, y una tecla que cambia notas en
una ventana que no estás mirando sería una sorpresa desagradable. Así
que las teclas de los editores van a la ventana en la que se hizo clic
la última vez, y a ninguna otra:

```rust
/// Editor shortcuts belong to the last clicked window, including its title
/// and toolbar. Popup controls and text fields keep their own keyboard input.
pub fn editor_keyboard(ctx: &egui::Context, layer: egui::LayerId) -> bool {
    editor_active(ctx, layer)
        && !ctx.text_edit_focused()
        && !egui::Popup::is_any_open(ctx)
        && ctx.memory(|memory| memory.is_above_modal_layer(layer))
}
```

`editor_active` recuerda la *capa* de la última pulsación (cada ventana
es una capa, ver [¿Qué hay bajo el
puntero?](egui.md#qué-hay-bajo-el-puntero)), en la memoria
de egui, y responde si es esta. Una ventana que la app abre por ti, como
cuando un doble clic en un clip abre el piano roll, recibe el teclado
con `widgets::give_keyboard`, como si se hubiera hecho clic en ella. La
lista de reproducción comparte la capa de fondo con las barras de
herramientas, así que para ella también importa el lugar del clic
(`editor_keyboard_in`). Las teclas se ignoran mientras un campo de
texto tiene el teclado, para que escribir el nombre de un patrón nunca
mueva notas, y mientras hay un menú abierto.

Espacio, que toca y detiene, tiene el problema contrario: debe funcionar
en todas partes, pero egui también deja pulsar con Espacio un botón que
tiene el foco. Haz clic en ▶ Play, pulsa Espacio, y el atajo detendría
la música mientras el botón la volvía a poner. Así que los atajos se
leen antes de dibujar ningún panel, y `consume_key` se lleva la
pulsación una vez usada:

```rust
let typing = ctx.text_edit_focused() || egui::Popup::is_any_open(ctx);
// `consume_key` reports whether the key was pressed and removes the
// press, so a focused button does not also react to Space.
let pressed = |key| ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, key));
if !typing && !controls_active && pressed(egui::Key::Space) {
    // ...play or stop the song, or the pattern in the rack or piano roll...
}
```

### El orden de los atajos de teclado

Cada tecla es un `KeyboardShortcut`: una tecla y sus modificadores.

```rust
pub(crate) const KEY_UP: KeyboardShortcut = shortcut(Modifiers::NONE, Key::ArrowUp);
pub(crate) const OCTAVE_UP: KeyboardShortcut = shortcut(Modifiers::SHIFT, Key::ArrowUp);
pub(crate) const IN_KEY_UP: KeyboardShortcut = shortcut(Modifiers::ALT, Key::ArrowUp);
```

`shortcut` es una `const fn`, una función lo bastante simple como para
ejecutarse mientras se compila, y por eso sus resultados pueden ser
`const`. Son `pub(crate)` para que la lista de **Ayuda > Atajos de
teclado** use las mismas constantes (ver [Todas las teclas en una
lista](egui.md#todas-las-teclas-en-una-lista)).

El piano roll le pregunta a egui si se pulsó cada uno con
`input.consume_shortcut(&shortcut)`. Eso devuelve `true` una vez, y
«consume» la pulsación para que nada más reaccione a ella. Un detalle de
egui importa aquí: compara los atajos de forma *lógica*, lo que significa
que unas teclas Mayús o Alt de más no impiden la coincidencia. Mayús+↑
cuenta también como una pulsación de ↑. Comprobados en el orden
equivocado, Mayús+↑ se lo comería el atajo de «subir un semitono» antes
de que el de «subir una octava» llegara a verlo. Así que la lista va de
lo más específico a lo menos:

```rust
// egui counts Shift+↑ and Alt+↑ as ↑ too, so the longer shortcuts
// are checked first.
let keys = [
    (IN_KEY_UP, Tool::InKey(1)),
    (IN_KEY_DOWN, Tool::InKey(-1)),
    (OCTAVE_UP, Tool::Transpose(12)),
    (OCTAVE_DOWN, Tool::Transpose(-12)),
    (KEY_UP, Tool::Transpose(1)),
    // ...
];
keys.into_iter()
    .find(|&(shortcut, _)| pressed(shortcut))
    .map(|(_, tool)| tool)
```

`find` se detiene en el primer atajo pulsado, así que como mucho se usa
una herramienta por frame, y a los atajos más cortos de después ni
siquiera se les pregunta.

### Un enum para el menú y las teclas

El menú Herramientas y las teclas producen los dos una `Tool`:

```rust
enum Tool {
    Copy,
    Cut,
    Paste,
    Duplicate,
    Transpose(i32),
    InKey(i32),
    Shift(isize),
    Quantize,
    SnapToKey,
    Invert { up: bool },
    Arpeggiate,
    Delete,
    SelectAll,
    SelectNone,
}
```

y `use_tool` convierte cualquier `Tool` en una llamada a la función
correspondiente de `lineup_core::tools`, seguida de `replace`. Una
herramienta cambia las notas seleccionadas, o todas las notas cuando no
hay ninguna seleccionada; Borrar solo borra una selección. Venga del
menú o de una tecla, una herramienta sigue exactamente el mismo camino,
así que las dos nunca pueden comportarse distinto. El menú muestra la
tecla de cada herramienta a su lado, escrita como el sistema escribe las
teclas, así que sirve también como lista de atajos.

### Un menú que se queda abierto

Los menús de egui se cierran con cualquier clic dentro de ellos, que es
lo correcto para una lista de órdenes. Pero el menú Herramientas también
tiene opciones: el orden y la velocidad del arpegio. Cerrar el menú al
elegir «arriba y abajo» te obligaría a abrirlo otra vez para pulsar
«Arpegiar los acordes». Así que a este menú se le dice que se cierre
solo con un clic *fuera* de él, y cada herramienta lo cierra a mano:

```rust
let stay_open = egui::containers::menu::MenuConfig::new()
    .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside);
let menu = egui::containers::menu::MenuButton::new(tr("Tools")).config(stay_open);
menu.ui(ui, |ui| {
    // ...
    let mut item =
        |ui: &mut Ui, text: &str, tool: Tool, shortcut: Option<KeyboardShortcut>| {
            // ...
            if ui.add(button).clicked() {
                picked = Some(tool);
                ui.close();
            }
        };
    item(ui, "Up a semitone", Tool::Transpose(1), Some(KEY_UP));
    // ...
});
```

`item` es un closure guardado en una variable y llamado muchas veces.
Asigna a `picked`, una variable de fuera, así que toma prestado `picked`
de forma mutable mientras exista el closure, y hay que declararlo
`let mut item` (llamarlo cambia lo que capturó: es un `FnMut`). El `ui`
se le pasa como argumento en cada llamada en lugar de capturarlo, porque
el código entre las llamadas también usa `ui`, y un closure que lo
retuviera lo bloquearía. Los menús que tienen un campo de texto, como el
menú de sección de la lista de reproducción, se quedan abiertos de la
misma manera, para que un clic en el campo no los cierre.

### Notas fantasma: oír los otros canales

Un acorde a menudo se reparte entre instrumentos: el pad toca Do y Sol,
el bajo toca Mi, y juntos forman C/E. Mirando solo el canal que se
edita, el piano roll diría «C5» (un power chord) donde tú oyes C/E.

Así que el piano roll puede escuchar también a los otros canales. No a
todos: los canales de batería tienen pasos simples, y un bombo no forma
parte de la armonía. Un canal cuenta si alguna de sus notas no es un
paso simple:

```rust
let others: Vec<usize> = (0..pattern.rows())
    .filter(|&other| other != self.channel)
    .filter(|&other| pattern.notes(other).iter().any(|note| !note.is_step()))
    .collect();
let mut heard = vec![self.channel];
if self.ghosts {
    heard.extend(&others);
}
```

`heard` es la lista de canales cuyas notas cuentan para la armonía:
este, y los otros cuando «Notas fantasma» está marcado. La línea que
nombra el acorde bajo el puntero, las sugerencias de acorde siguiente y
los nombres de las notas reciben todos `heard`, y reúnen sus notas con
una pequeña función auxiliar:

```rust
fn sounding<'a>(
    pattern: &'a Pattern,
    channels: &'a [usize],
    step: usize,
) -> impl Iterator<Item = Note> + 'a {
    channels
        .iter()
        .flat_map(move |&channel| pattern.sounding_at(channel, step))
}
```

- `flat_map` llama al closure por cada canal, recibe cada vez un
  iterador de notas, y los une todos en un solo iterador, como si las
  notas de los canales fueran una sola lista.
- El resultado toma prestados `pattern` y `channels`, así que no debe
  sobrevivirlos. `<'a>` nombra ese préstamo, y `+ 'a` en el tipo de
  retorno dice «este iterador no vive más que `'a`». `analyze` acepta
  cualquier iterador de notas, así que lee las notas directamente del
  patrón, sin copiarlas antes a un `Vec`.
- `move` hace que el closure tome sus propias copias de lo que usa:
  `step` (un número) y `pattern` (una referencia, que es lo que se
  copia, no el patrón). Sin `move`, el closure tomaría prestada la
  variable `step`, que pertenece a `sounding` y desaparece cuando esta
  devuelve, mientras el iterador sigue vivo.

Las notas fantasma se pintan antes que las de este canal, de modo que
las de este canal quedan siempre encima, en gris con un borde fino. Solo
se dibujan: los clics miran únicamente las notas de este canal, así que
las notas fantasma nunca pueden seleccionarse, moverse ni borrarse por
accidente. Cuando ningún otro canal toca melodías o acordes, la casilla
aparece en gris, y su texto al pasar el ratón dice por qué.

### El carril de acordes: `chunk_by`, y luego medir

El carril sobre la cuadrícula nombra la armonía. Su primera fila nombra
lo que toca este canal en cada paso; su segunda fila, **Todo**, nombra
cómo suena ahí la canción entera, pulso a pulso, tal como lo calcula
[Oír la armonía](../theory/hearing.md). Cada fila une los pasos
vecinos con el mismo nombre en un solo bloque.

Qué es «el mismo» depende del menú **Mostrar** del carril. Como
acordes, una armonía guarda su bajo, así que C y C/E son dos bloques;
como conjuntos de clases de alturas son uno, las mismas notas (ver [En
el piano roll](../theory/sets.md#en-el-piano-roll)). Así que primero
cada paso recibe el nombre que le dará el carril, y los bloques se
hacen a partir de eso:

```rust
#[derive(PartialEq)]
enum LaneName {
    Heard(Harmony),
    Named(Vec<HarmonyName>),
}

let steps: Vec<LaneName> = harmonies
    .iter()
    .map(|&(harmony, notes)| match self.notation.name_heard(&harmony, notes, scale) {
        Some(names) => LaneName::Named(names),
        None => LaneName::Heard(harmony),
    })
    .collect();
let mut start = 0;
for run in steps.chunk_by(|a, b| a == b) {
    let end = start + run.len();
    // ...a block from step `start` to step `end`, named by `run[0]`...
    start = end;
}
```

`chunk_by` parte un slice en tramos allí donde el closure dice que dos
vecinos no van juntos. Cada `run` es un slice del vector original, así
que no se copia nada. El `==` que usa es el que escribe
`#[derive(PartialEq)]`: dos `LaneName` son iguales cuando son la misma
variante con el mismo contenido. Comparar armonías enteras, y no sus
etiquetas, importa: todo grupo de notas sin nombre de acorde lleva la etiqueta «?»,
y unir por etiqueta convertiría varios acordes distintos sin nombre
seguidos en un solo bloque «?» largo, como si se mantuviera un acorde.

Ejecutar `analyze` en cada paso y en cada frame parece mucho, pero son
como mucho unos cientos de bucles pequeños sobre unas pocas notas. Mide
antes de optimizar: aquí ni siquiera se nota.

Un nombre tiene que caber en su bloque. Cortado por los bordes, «E7/B»
en un bloque estrecho mostraría «7/», que no es un nombre en absoluto, y
para quien aprende armonía mirando la pantalla un nombre equivocado es
peor que ninguno. egui dibuja el texto en dos pasos. Primero lo
**compone**: elige el glifo de cada carácter en la fuente, y calcula
dónde va cada uno y cuánto espacio ocupa el conjunto. El resultado es una
*galley* (el nombre viene de la imprenta: la galera, una bandeja de
tipos compuestos en líneas, lista para imprimir). Después pinta la
galley. `painter.text(...)` hace las dos cosas a la vez; para decidir
*antes* de dibujar, los dos pasos se separan:

```rust
let name = painter.layout_no_wrap(label, FontId::proportional(11.0), LANE_TEXT);
if name.size().x <= chord_rect.width() - 2.0 {
    // A long chord's name stays in view while it scrolls.
    let shown = chord_rect.intersect(cells);
    painter.with_clip_rect(chord_rect).galley(
        shown.center() - name.size() / 2.0,
        name,
        LANE_TEXT,
    );
}
```

`layout_no_wrap` compone el texto en una sola línea, y `name.size()` es
cuánto mide de ancho y de alto. Si cabe en el bloque, con un punto de
margen a cada lado, se pinta esa misma galley; nada se compone dos
veces. `galley` recibe la esquina superior izquierda del texto, así que
el centro se corre hacia atrás la mitad del tamaño. Un conjunto o un
micromodo tiene un nombre más corto para probar antes, su clase sola,
«3-11» por «3-11 T7I»; si ni eso cabe, no se dibuja nada. Apuntar al
bloque sigue nombrándolo entero en la línea bajo la barra de
herramientas.

### Resaltado de la escala

`paint_rows` colorea cada fila preguntándole a la escala por su clase de
altura, con un `match` sobre una tupla de dos `bool`:

```rust
let fill = match (is_root, in_scale) {
    (true, _) => ROOT_ROW,
    (false, true) => SCALE_ROW,
    (false, false) => OTHER_ROW,
};
```

Las teclas de la izquierda llevan un punto en las notas de la tonalidad
(naranja para la tónica), y las notas fuera de la tonalidad se dibujan
en rojo. La tonalidad es la de la canción allí donde suena el patrón
(`project.pattern_key`), así que un patrón usado después de un cambio de
tonalidad se muestra en la nueva. Cambia la tonalidad y todos los
colores la siguen en el frame siguiente, porque nada se guarda en caché:
el modo inmediato dibuja a partir del estado actual cada vez.

### El carril de velocidad

#### Hacerle sitio

El carril está bajo la cuadrícula. La cuadrícula es un `ScrollArea` que
llena la altura que se le da. Si se le diera toda la altura que queda en
la ventana, empujaría el carril por debajo del borde; las ventanas de
egui crecen para que quepa su contenido, así que la ventana crecería, la
cuadrícula llenaría la nueva altura, el carril volvería a quedar fuera, y
la ventana iría creciendo unos píxeles en cada frame. Así que a la
cuadrícula se le da exactamente la altura que queda cuando el carril
tiene su parte:

```rust
// The velocity lane goes below the grid, so the grid leaves room
// for it: the lane, the gap, and egui's spacing between widgets.
// Taking even a pixel too much would make the window grow a
// little on every frame.
let below = VELOCITY_HEIGHT + 4.0 + 2.0 * ui.spacing().item_spacing.y;
let grid_height = (ui.available_height() - below).max(96.0);
```

Lo fácil de olvidar es `item_spacing`: egui pone un pequeño hueco
después de cada widget, aquí después del scroll area y después del
espacio. Una manera simple de pillar una ventana que crece sola es hacer
dos capturas de pantalla con unos segundos de diferencia y compararlas.

#### Fijar velocidades

El carril es un único widget que detecta clics y arrastres. Mientras el
botón está pulsado sobre él, la altura del puntero fija la velocidad, y
su posición x elige el paso:

```rust
if response.is_pointer_button_down_on()
    && !crate::widgets::dismissing(ui.ctx())
    && let Some(pos) = response.interact_pointer_pos()
    && (cells.contains(pos) || response.dragged())
{
    let step = lane.step_at(pos.x);
    let height = (lane.rect.bottom() - pos.y) / lane.rect.height();
    let velocity = (height * 127.0).round().clamp(1.0, 127.0) as u8;
    let selected_here = notes
        .iter()
        .any(|note| note.start == step && self.selection.contains(note));
    let old: Vec<NoteEvent> = notes
        .iter()
        .filter(|note| note.start == step && note.velocity != velocity)
        .filter(|note| !selected_here || self.selection.contains(note))
        .copied()
        .collect();
    let new = old.iter().map(|&note| NoteEvent { velocity, ..note }).collect();
    self.replace(pattern, actions, "change velocity", old, new);
}
```

- La altura es una fracción de 0 (abajo) a 1 (arriba), escalada al 1 a
  127 de MIDI. `clamp` la mantiene dentro incluso cuando el puntero sale
  del carril durante un arrastre. Convertir un float con `as` nunca da
  la vuelta (300.0 pasa a 255 y -5.0 a 0), pero 0 sería una nota que no
  se oye, así que el clamp asegura que la velocidad sea al menos 1.
- Un acorde tiene varias notas en el mismo paso. Cambian todas, salvo
  que algunas estén seleccionadas: entonces solo cambian las
  seleccionadas. Así se hace que la nota de arriba de un acorde suene más
  fuerte que las demás, como hacen los pianistas para destacar una
  melodía.
- Las notas que ya tienen esa velocidad quedan fuera de `old`, así que
  mantener el botón quieto no envía el mismo cambio en cada frame.

Barrer el carril cambia cada paso a medida que el puntero pasa por
encima, y todo el barrido es un solo paso de deshacer, como cualquier
arrastre.

La velocidad también se ve en la cuadrícula. El color de una nota se
multiplica por un factor entre 0,55 y 1 según su velocidad
(`fill.gamma_multiply(0.55 + 0.45 * f32::from(note.velocity) / 127.0)`),
así que las notas suaves son más oscuras, pero nunca tanto como para
desaparecer.

### Encontrar tus notas

El piano roll muestra un canal de un patrón, y esa pareja puede estar
vacía mientras la canción suena de lo lindo: los acordes de una canción
pueden estar en «Acordes», no en «Ritmo». Una cuadrícula vacía sin una
palabra de explicación parece notas perdidas. Así que el piano roll hace
dos cosas.

Primero, sigue a las notas. Cuando se abre, o cambia el patrón, y el
canal mostrado no tiene notas ahí, pasa al canal con más notas de
melodía o de acordes:

```rust
pub fn busiest_channel(pattern: &Pattern) -> Option<usize> {
    (0..pattern.rows())
        .map(|channel| {
            let melodic = pattern.notes(channel).iter().filter(|note| !note.is_step());
            (channel, melodic.count())
        })
        .filter(|&(_, count)| count > 0)
        .max_by_key(|&(channel, count)| (count, std::cmp::Reverse(channel)))
        .map(|(channel, _)| channel)
}
```

`max_by_key` devuelve el *último* de varios máximos iguales. Comparar la
tupla `(count, Reverse(channel))` deshace los empates hacia el número de
canal más bajo: `std::cmp::Reverse` envuelve un valor e invierte su
orden. Los pasos de batería se dejan fuera a propósito: en un patrón de
solo batería, el piano roll no salta al bombo, cuyas «notas» son todas el
mismo paso simple. Un canal que elegiste tú, en el rack o en el editor,
se conserva.

Para notar que la ventana *se abre*, el piano roll recuerda si estaba
abierta en el frame anterior. `open` es un campo público que cambian el
botón de la barra de herramientas, F7 y el menú Ver, así que este es el
único sitio que ve todas las maneras de entrar:

```rust
let opened = self.open && !self.was_open;
// ... at the end of `show`:
self.was_open = open;
```

Al abrirse, también se desplaza al centro de las notas. El centro no se
calcula partiendo las alturas por la mitad: con enteros, `/` tira el
resto, y `41 / 2 + 67 / 2` da 53, una fila por debajo del centro de
verdad, 54. En cambio, le pide a egui que centre todo el bloque de
filas, desde arriba de la nota más aguda hasta abajo de la más grave:

```rust
let top = grid.row_top(highest.clamp(LOWEST, HIGHEST));
let bottom = grid.row_top(lowest.clamp(LOWEST, HIGHEST)) + row_height;
let target = Rect::from_min_max(pos2(rect.left(), top), pos2(rect.left() + 1.0, bottom));
ui.scroll_to_rect(target, Some(Align::Center));
```

Segundo, cuando el canal de verdad no tiene notas en el patrón, lo dice
sobre la cuadrícula, con botones para ir adonde están las notas:

![El piano roll en «Ritmo»: una línea de color dice que Lineup Osc no
tiene notas ahí, con un botón para «Acordes», donde sí las tiene, y otro
para el rack de canales, donde se edita la batería de
«Ritmo».](../images/step-8-no-notes.png)

Encontrar los patrones donde toca el canal es un filtro sobre todos
ellos, que conserva el índice (para seleccionarlo) y el nombre (para
mostrarlo):

```rust
let elsewhere: Vec<(usize, &str)> = project
    .patterns()
    .iter()
    .enumerate()
    .filter(|&(index, other)| {
        index != self.pattern && !other.notes(self.channel).is_empty()
    })
    .map(|(index, other)| (index, other.name.as_str()))
    .collect();
```

El `&str` del vector toma prestado de `project`. Eso está bien porque el
vector solo vive mientras se dibuja este frame, y `project` dura más. Un
botón para otro patrón no puede seleccionarlo directamente: el patrón
seleccionado pertenece a la app. Así que devuelve
`RollAction::SelectPattern`, igual que cualquier otro cambio sale del
piano roll.

El mensaje se coloca *sobre* la cuadrícula en lugar de ocupar una línea
encima, para que la cuadrícula no salte hacia arriba cuando se dibuja la
primera nota y el mensaje desaparece. Se dibuja después de la
cuadrícula, en un `Ui` hijo colocado encima, y la cuadrícula recuerda
dónde se dibujó en el frame anterior (`self.banner`) y deja en paz los
clics de ahí: un clic en uno de sus botones no debe dibujar también una
nota debajo.

### Conectarlo en la aplicación

La app tiene un `PianoRoll` y, mientras dibuja las ventanas, llama:

```rust
let roll_actions = self.piano_roll.show(
    ctx,
    &self.project,
    self.pattern,
    playhead,
    &held,
    area,
    roll_zoom,
);
for action in roll_actions {
    self.apply_roll(action);
}
```

`self.piano_roll.show(..., &self.project, ...)` toma prestado un campo
de forma mutable y otro de forma inmutable a la vez, cosa que el borrow
checker permite porque son campos distintos. Después `apply_roll`
cambia el patrón con `edit_pattern`, como hace el rack:

```rust
RollAction::ReplaceNotes { channel, old, new, label } => {
    self.edit_pattern(label, |pattern| {
        pattern.replace_notes(channel, &old, &new);
    });
}
```

Cuando se quita un canal, `PianoRoll::channel_removed` o bien suelta su
canal (ya no existe) o bien baja en uno su número de canal, para seguir
mostrando el mismo canal; `channel_inserted` hace lo contrario cuando
deshacer devuelve un canal.

En una ventana baja, de menos de 420 puntos, la cuadrícula va primero:
las líneas de ayuda bajo la barra de herramientas pasan al tooltip de la
línea que nombra el acorde, así que una pantalla pequeña sigue mostrando
una docena de filas de notas.

## La lista de reproducción

### Una rejilla grande: `show_viewport`

La lista de reproducción puede ser mucho más grande que la ventana:
decenas de compases, y tantas pistas como use la canción. Va dentro de
un scroll area. Pero algunas partes *no* deben desplazarse: los nombres
de las pistas a la izquierda, y los números de compás y las franjas de
encima arriba.

`ScrollArea::show_viewport` lo hace posible. Su closure recibe la parte
visible del contenido, como un rectángulo medido desde la esquina
superior izquierda del contenido:

```rust
let output = area.show_viewport(ui, |ui, viewport| {
    self.grid(ui, viewport, project, current, playhead, &mut actions)
});
```

`grid` reserva el tamaño entero del contenido, y luego calcula dónde
está en pantalla la parte visible:

```rust
let (rect, response) = ui.allocate_exact_size(size, Sense::click_and_drag());

// `viewport` is the visible part, measured from the top-left of the
// content. Moving it by the content's position on screen gives the
// visible part in screen coordinates, where everything is drawn.
let visible = viewport.translate(rect.min.to_vec2());
```

`rect` es el contenido entero en coordenadas de pantalla (su esquina
superior izquierda se mueve hacia arriba y a la izquierda al
desplazarte). Sumar esa esquina a `viewport` da `visible`, la parte de
la pantalla donde se ve el contenido. Entonces:

- las pistas y los clips se dibujan en su lugar dentro del contenido,
  así que se desplazan;
- los nombres de las pistas se dibujan en `visible.left()`, y la regla y
  las franjas en `visible.top()`, así que se quedan quietos mientras los
  clips se deslizan por debajo.

Se dibujan *después* de los clips, así que los tapan.

La aritmética de pasos, pistas y píxeles vive en un pequeño struct
`Copy`, como en el piano roll:

```rust
#[derive(Debug, Clone, Copy)]
struct Layout<'a> {
    /// The top-left corner of step 0 of track 0.
    origin: Pos2,
    /// How wide a step is and how tall a track is: the zoom.
    step_width: f32,
    track_tops: &'a [f32],
    tracks: usize,
    /// Whether there is an empty row below the last track, where a clip
    /// starts a new track. There is unless the playlist is full.
    spare: bool,
}
```

Cada pista puede tener su propia altura, así que `Layout` guarda dónde
empieza cada pista (`track_tops`), calculado una vez por frame. Para
encontrar la pista bajo el puntero, `cell_at` usa `partition_point`, una
búsqueda binaria en esa lista ordenada: el número de comienzos que
quedan por encima del puntero, menos uno, es la pista.

### Widgets reales en una zona pintada: `ui.put`

Todo en la rejilla se pinta a mano, pero los nombres de las pistas deben
ser campos de texto editables y las luces de silencio deben ser la misma
luz que en el rack. `ui.put` coloca cualquier widget en el rectángulo que
elijas:

```rust
let light_response = ui
    .put(light, |ui: &mut Ui| widgets::mute_light(ui, enabled))
    .on_hover_text(tr("Click to mute or unmute the track"));

let shown = tr_name(&track.name).into_owned();
let mut name = shown.clone();
let field = egui::TextEdit::singleline(&mut name)
    .frame(egui::Frame::NONE)
    .text_color(HEADER_TEXT);
let name_response = ui.put(name_rect, field);
if name_response.changed() && name != shown {
    actions.push(PlaylistAction::RenameTrack { track: index, name });
}
```

- Un closure `|ui: &mut Ui| -> Response` cuenta como widget: egui
  implementa su trait `Widget` para cualquier closure así. Así es como
  un widget con forma de función, como `mute_light`, se puede pasar a
  `put`.
- El campo de texto edita una *copia* del nombre. Si cambió, la copia
  va en una acción `RenameTrack`, y la app cambia el proyecto. La lista
  de reproducción mantiene la regla de que nunca cambia el proyecto ella
  misma.
- `Frame::NONE` quita el borde del campo, así que parece una etiqueta
  hasta que haces clic en él.

### Colocar, mover y recortar clips

Colocar, mover y recortar clips sigue el modelo del piano roll: una
pulsación decide qué es el arrastre, y el arrastre sigue al puntero
hasta que se suelta el botón.

```rust
struct ClipDrag {
    index: usize,
    grab: usize,
    /// The edge dragged, if the drag trims the clip.
    edge: Option<Edge>,
}

enum Edge {
    Left,
    Right,
}
```

Una pulsación en un espacio vacío coloca un clip del patrón seleccionado
y empieza a moverlo; una pulsación en un clip empieza a mover ese clip y
selecciona su patrón; una pulsación en el borde izquierdo o derecho de
un clip empieza a recortarlo (ver [Recortar un
clip](project.md#recortar-un-clip)). Arrastrado hacia adentro, un borde
oculta el comienzo o el final del patrón; arrastrado hacia afuera más
allá del final, el borde derecho lo repite. La pulsación se atiende
donde bajó el botón:

```rust
// A press is handled where the button went down. A quick hand can
// press and move in the same frame, and the clip or empty cell
// under the pointer *now* may not be the one that was clicked.
// A click that closes a menu does nothing else.
let dismissing = widgets::dismissing(ui.ctx());
let pressed_at = press_origin.filter(|pos| {
    primary_pressed && !dismissing && response.hovered() && cells.contains(*pos)
});
if let Some(pos) = pressed_at {
    let (track, step) = layout.cell_at(pos);
    match clip_at(project, layout, pos) {
        Some(index) => { /* start moving or trimming that clip */ }
        None => { /* add a clip of the selected pattern, and start moving it */ }
    }
}
```

`Option::filter` conserva el valor solo si el closure devuelve `true`,
así que `pressed_at` es `Some` solo en el frame de una pulsación, sobre
la rejilla.

Las posiciones se ajustan a la rejilla que elijas en la barra de
herramientas: un compás, un pulso o un paso. Dónde caen compases y
pulsos depende del compás en ese punto de la canción, así que el ajuste
le pregunta al `Meter` de la canción:

```rust
/// The snap point at or before `step`. Where the bars and beats are
/// depends on the time signature there: a bar of 3/4 is 12 steps, a
/// beat of 6/8 is 6.
fn floor(self, step: usize, meter: &Meter) -> usize {
    match self {
        Snap::Bar => meter.floor_to_bar(step),
        Snap::Beat => meter.floor_to_beat(step),
        Snap::Step => step,
    }
}
```

Un borde arrastrado se ajusta igual, y también al lugar más cercano
donde el patrón del clip vuelve a empezar, para que un número entero de
repeticiones siga siendo fácil de alcanzar. Gana el que esté más cerca
del puntero:

```rust
let mut nearest = vec![self.snap.floor(step, meter), self.snap.next(step, meter)];
if pattern_length > 0 {
    let length = pattern_length as f32;
    let origin = clip.start as f32 - clip.first_step(pattern_length) as f32;
    let round = origin + ((at - origin) / length).round() * length;
    if round >= 0.0 {
        nearest.push(round as usize);
    }
}
```

`at` es el paso del puntero con su fracción, y `origin` el paso de la
canción donde empieza la primera vuelta del patrón, suene o no. Cada
borde tiene una franja del tamaño de una mano, pero en un clip más
angosto que dos franjas el borde izquierdo recibe solo un tercio, así
que gana el derecho, que siempre se puede arrastrar hacia afuera para
hacer lugar.

Ctrl o Mayús y un clic añaden un clip a la selección, Ctrl o Mayús y un
arrastre en un espacio vacío dibujan un recuadro alrededor de clips, y
arrastrar uno de varios clips seleccionados los mueve todos, sin salirse
de la lista de reproducción.

### Doble clic en un clip para editarlo

En la mayoría de los programas de música, haces doble clic en un clip
para editarlo. egui cuenta los clics:

```rust
let double_clicked = ui.input(|input| {
    input
        .pointer
        .button_double_clicked(egui::PointerButton::Primary)
});
if double_clicked
    && !dismissing
    && response.hovered()
    && let Some(pos) = pointer.filter(|pos| cells.contains(*pos))
    && let Some(index) = clip_at(project, layout, pos)
{
    actions.push(PlaylistAction::EditClip(clips[index].pattern));
}
```

La app elige el editor: el piano roll, en el canal con más notas, cuando
el patrón tiene melodías o acordes; el rack de canales cuando solo tiene
batería. En los dos casos la ventana pasa al frente, y se lleva el
teclado:

```rust
fn bring_window_to_front(&mut self, ctx: &egui::Context) {
    if let Some(id) = self.bring_to_front.take() {
        let layer = egui::LayerId::new(egui::Order::Middle, id);
        ctx.move_to_top(layer);
        // Its keys go with it: Delete after double-clicking a clip
        // deletes notes in the piano roll, not clips in the playlist.
        widgets::give_keyboard(ctx, layer);
    }
}
```

La capa de una ventana se nombra con el id que se le dio a la ventana,
y por eso el rack y el piano roll obtienen el suyo de pequeñas funciones
(`rack_id()` y `PianoRoll::id()`) en lugar de escribir el mismo
`Id::unique(...)` en varios sitios.

![Un doble clic en un clip de «Ritmo» abre el rack de canales en
«Ritmo», ajustado a sus filas.](../images/step-8-rack-double-click.png)

### Un clic derecho abre el menú del clip

Un clic derecho en un clip antes lo eliminaba, y arrastrar con el botón
derecho eliminaba todos los clips que cruzaba. Es rápido, pero un clic
derecho abre un menú en todos los demás lugares de la aplicación (el
nombre de una pista, una sección, los números de compás), así que
eliminar ahí sorprendía. Ahora abre el menú del clip: **Transcribir otra
vez…** para la grabación de una toma, o **Editar su patrón**, y
**Eliminar**.

`egui::Popup::context_menu(response)` abre un menú donde se hizo clic
derecho en la respuesta. Pero la cuadrícula es una sola respuesta
grande, y un clic derecho en una celda vacía no debería abrir nada. Así
que la lista de reproducción recuerda en qué clip se hizo clic derecho,
y dónde, y le dice al menú emergente si abrirse:

```rust
let mut open = None;
if response.secondary_clicked() {
    self.clip_menu = response
        .interact_pointer_pos()
        .filter(|pos| cells.contains(*pos))
        .and_then(|pos| {
            clip_at(project, layout, pos).map(|index| (index, layout.step_at(pos.x)))
        });
    open = Some(egui::SetOpenCommand::Bool(self.clip_menu.is_some()));
} else if response.clicked() {
    open = Some(egui::SetOpenCommand::Bool(false));
}
```

`open_memory(open)` reemplaza la orden que daría `context_menu`, que se
abre con cualquier clic derecho. `None` deja el menú como estaba,
abierto o cerrado. El clip se recuerda por su índice, que sigue siendo
correcto mientras el menú está abierto: el menú es un emergente, así
que un clic en él no llega a la cuadrícula, y nada más edita los clips
mientras tanto.

**Eliminar** elimina el clip, o todos los clips seleccionados cuando es
uno de ellos, con el mismo `RemoveClips` que manda la tecla Supr.

### Dibujar las notas de un clip: `Box<dyn Fn>`

Cada clip muestra un esbozo de su patrón. Los patrones de batería se ven
mejor como un secuenciador de pasos diminuto (una línea por canal), y
cualquier cosa con melodía como un piano roll diminuto (las notas
agudas más arriba). Las dos necesitan maneras distintas de convertir una
nota en una altura, elegidas una vez por clip:

```rust
// Turns a channel and a note into a height from 0.0 (bottom) to 1.0 (top).
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
    // ...the lowest and highest note, as in the rack's mini piano roll...
    Box::new(move |_, midi| ((midi - lowest) as f32 + centering) / span)
};
```

Cada closure tiene su propio tipo, sin nombre, así que las dos ramas del
`if` producen valores de tipos distintos, y una variable solo puede
tener un tipo. `Box<dyn Fn(usize, u8) -> f32>` es el tipo común: «un
closure en el montículo que recibe un canal y una altura y devuelve una
altura en pantalla». `dyn Fn` es un trait object, como `dyn App` en
`main.rs`. La palabra clave `move` hace que cada closure se quede con lo
que usa (`used`, `lowest`...), que debe sobrevivir al `if` que los creó.

Esto se ejecuta en el hilo de la ventana, una vez por clip y por frame,
así que la pequeña reserva de memoria del `Box` no importa. En el hilo
de audio sí importaría.

El color de un clip es el de su patrón. Cada patrón guarda un `color`,
un lugar en la paleta de la lista de reproducción, guardado en el
archivo del proyecto. No sale del número del patrón, porque entonces
borrar el primer patrón cambiaría el color de todos los demás.

### Solo las pistas que usas

Un proyecto tiene solo las pistas hasta su último clip: Archivo > Nuevo
tiene una pista, una demostración tiene las que usa. Debajo de la última
pista hay una **fila de reserva** más tenue: coloca un clip en ella (o
arrastra uno hasta ella) y se convierte en una pista. Su cabecera tiene
también un botón **➕ Nueva pista**.

![La demostración de inicio después de colocar un clip en la fila de
reserva y pulsar «Nueva pista».](../images/step-8-spare-row.png)

`Project::clip_fits` decide si un clip puede ir en un sitio. Se permite
una fila más que las pistas, la que está justo debajo de la última
pista, siempre que la lista de reproducción no esté llena:

```rust
fn clip_fits(&self, clip: Clip) -> bool {
    let tracks = self.playlist.tracks.len();
    clip.pattern < self.patterns.len()
        && (clip.track < tracks || (clip.track == tracks && tracks < MAX_TRACKS))
        && clip.start < MAX_SONG_STEPS
        && (1..=MAX_REPEATS).contains(&clip.repeats)
}
```

`add_clip` y `move_clip` llaman después a `open_track`, que añade una
pista cuando el clip está en esa fila nueva. Como la pista y el clip se
añaden en la misma edición, un solo Ctrl+Z quita los dos. La vista de la
lista de reproducción solo tiene que dibujar una fila más: `Layout`
tiene su indicador `spare` y un método `rows()`, y `cell_at` limita a
`rows()`.

Las demostraciones se construyen sobre una lista de reproducción de
dieciséis pistas, porque nombrar la pista 0 y la pista 1 es más fácil
cuando existen; al final, `remove_unused_tracks` quita las pistas vacías
de abajo. Los archivos guardados antes de que se recortaran las pistas
necesitan casi lo mismo, con una diferencia: una pista que alguien
renombró o silenció se dejó así a propósito, así que al cargar solo se
quitan las pistas que están exactamente como empieza una pista nueva.
Dos reglas, un bucle, que recibe la regla como closure:

```rust
pub fn remove_unused_tracks(&mut self) {
    self.trim_tracks(|_, _| true);
}

pub(crate) fn remove_blank_tracks(&mut self) {
    self.trim_tracks(|index, track| *track == Track::numbered(index + 1));
}

fn trim_tracks(&mut self, drop: impl Fn(usize, &Track) -> bool) {
    // ...from the bottom, pop tracks with no clips while `drop` says so...
}
```

`impl Fn(usize, &Track) -> bool` significa «cualquier función o closure
que reciba un número de fila y una pista y responda sí o no». El
compilador hace una copia de `trim_tracks` para cada closure, así que
esto no cuesta nada en tiempo de ejecución. Comparar con
`Track::numbered(index + 1)` funciona porque `Track` deriva `PartialEq`:
una pista en blanco es igual a una completamente nueva para su fila.

Recortar solo quita pistas **por debajo** del último clip. Una pista
vacía en medio se queda, porque quitarla subiría una fila todos los
clips de debajo.

### Insertar, mover y borrar pistas

Un clip nombra su pista por **número**. Inserta una pista en la fila 2, y
todo clip en la fila 2 o más abajo debe bajar una, o la batería de
repente quedaría en la pista equivocada:

```rust
pub fn insert_track(&mut self, index: usize) -> Option<usize> {
    let tracks = &mut self.playlist.tracks;
    if tracks.len() >= MAX_TRACKS {
        return None;
    }
    let index = index.min(tracks.len());
    tracks.insert(index, Track::numbered(tracks.len() + 1));
    for clip in &mut self.playlist.clips {
        if clip.track >= index {
            clip.track += 1;
        }
    }
    Some(index)
}
```

`remove_track` es la imagen en espejo: quita los clips de la pista con
`retain`, y luego sube una fila los de debajo. `swap_tracks` intercambia
dos filas y los clips que hay en ellas. Quien cambia la lista arregla los
números que apuntan a ella, y por eso los campos son privados y solo los
métodos de `Project` los cambian (ver [El proyecto](project.md)).
Cada uno de ellos es una sola llamada dentro de `edit`, así que deshacer
devuelve una pista borrada con sus clips.

![Clic derecho en el nombre de una pista.](../images/step-8-track-menu.png)

El menú es el `context_menu` de egui sobre el campo del nombre de la
pista. Un campo de texto sigue abriendo un menú con el clic derecho,
porque escribir solo necesita el botón izquierdo. El borde inferior de
una pista se puede arrastrar para hacerla más alta o más baja, y un
doble clic en él le devuelve su altura habitual.

### Seguir el cursor de reproducción más allá de los nombres

Con **Seguir el cursor de reproducción** activado, la lista de
reproducción se desplaza para mantener el cursor a la vista. Cuando una
canción llega al final y vuelve a empezar, o un bucle vuelve a su
inicio, el cursor salta a la izquierda y la vista lo sigue. Pero los
nombres de las pistas de la izquierda se dibujan *sobre* los compases, y
para el scroll area no existen: mostrar el cursor podría dejarlo debajo
de ellos. Así que a `follow_x` se le dice cuánto mide la parte fija, y
el rectángulo que muestra llega esa distancia más a la izquierda:

```rust
pub fn follow_x(ui: &Ui, step: Option<usize>, x: f32, view: Rect, covered: f32) {
    // ...only when the step changed, and Follow playhead is on...
        let margin = (view.width() * 0.1).min(60.0);
        let y = view.center().y;
        ui.scroll_to_rect(
            Rect::from_min_max(
                pos2(x - margin - covered, y - 0.5),
                pos2(x + margin, y + 0.5),
            ),
            None,
        );
}
```

`scroll_to_rect` se desplaza lo menos posible para que el rectángulo
quede dentro de la vista. Hacia la izquierda, eso deja el borde
izquierdo del rectángulo en el borde izquierdo de la vista, `covered`
puntos a la izquierda del cursor, así que el cursor acaba a la derecha
de los nombres. La lista de reproducción pasa `HEADER_WIDTH` (sus
nombres), el piano roll `KEY_WIDTH` (su teclado). Hacia la derecha no
cambia nada. Solo un paso nuevo desplaza la vista, así que puedes seguir
desplazándote a mano mientras suena la música.

## Probar las ventanas

Las herramientas y el proyecto están cubiertos por pruebas unitarias, y
muchos comportamientos de los editores por pruebas que dibujan la
ventana sin pantalla (ver [Probar una ventana sin
pantalla](egui.md#probar-una-ventana-sin-pantalla)). Pero
algunos problemas solo aparecen cuando una persona usa la ventana: una
ventana que crece un poco en cada frame, un menú que se cierra al elegir
una opción dentro de él, un acorde que cae por encima del puntero en
lugar de alrededor. Es un buen hábito para cualquier código de interfaz:
escribe la lógica para que se pueda probar sin la ventana, y después
prueba la ventana igualmente.
