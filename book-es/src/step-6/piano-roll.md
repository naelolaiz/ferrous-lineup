# El piano roll, ya crecido

Archivo: [`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs)

Las herramientas son funciones puras, pero algo tiene que decidir qué
notas reciben, llamarlas en el momento adecuado y mostrar el resultado.
Ese es el trabajo del piano roll, y este paso casi duplicó su tamaño.
Este capítulo recorre las partes nuevas en el orden en que te las
encuentras al usarlas: seleccionar, mover, el teclado y el menú, los
otros canales y la velocidad.

## La selección guarda notas, no posiciones

La manera más simple de recordar qué notas están seleccionadas serían
sus posiciones en la lista del canal: `selection: Vec<usize>`. El
arrastre de una sola nota del paso 3 hace exactamente eso
(`Drag::Move { index, .. }`), y allí funciona porque nada más cambia la
lista durante un arrastre.

Una selección vive mucho más que un arrastre. Mientras hay notas
seleccionadas, una herramienta las saca de la lista y pone las nuevas al
final (`replace_notes` hace eso), el deshacer coloca un proyecto
anterior entero, y borrar una nota sube un lugar a todas las que van
después. Después de cualquiera de esas cosas, la posición 3 puede ser
otra nota, o no ser ninguna nota. Una selección de posiciones
seleccionaría en silencio las notas equivocadas, y la siguiente
herramienta cambiaría notas que nunca elegiste.

Así que la selección guarda las propias notas:

```rust
/// The selected notes, by value. A note that is no longer in the
/// pattern (after an undo, say) is simply not found, and so not
/// selected: the selection can never point at the wrong note.
selection: Vec<NoteEvent>,
```

Esto funciona porque un canal nunca tiene dos notas en el mismo lugar:
la regla del patrón (`same_place`: misma altura, mismo inicio, mismo
`nudge`) hace único el valor de cada nota, de modo que un valor nombra
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
fn replace(&mut self, pattern: &Pattern, actions: &mut Vec<RollAction>,
           label: &'static str, old: Vec<NoteEvent>, new: Vec<NoteEvent>)
    -> Option<Vec<NoteEvent>>
{
    let new = tidy_notes(new);
    if old == new || !pattern.can_replace_notes(self.channel, &old, &new) {
        return None;
    }
    let was_selected = old.iter().any(|note| self.selection.contains(note));
    self.selection.retain(|note| !old.contains(note));
    if was_selected {
        self.selection.extend(&new);
    }
    actions.push(RollAction::ReplaceNotes { channel: self.channel, old, new: new.clone(), label });
    Some(new)
}
```

`replace` es la única puerta por la que pasa todo cambio de grupo: las
herramientas, el arrastre de grupo y el carril de velocidad. Limpia las
notas nuevas igual que hará el patrón, le pregunta al patrón si el
cambio está permitido, y solo entonces envía la acción. Limpiar también
aquí significa que la selección guarda exactamente las notas que
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

Ctrl+clic sobre una nota la añade a la selección, o la saca de ella:

```rust
if let Some(at) = self.selection.iter().position(|&n| n == note) {
    self.selection.remove(at);
} else {
    self.selection.push(note);
}
```

`input.modifiers.command` es «Ctrl» en Linux y Windows y «⌘» en un Mac,
de modo que el mismo código sigue las costumbres de cada sistema.

Ctrl+arrastrar sobre el espacio vacío dibuja un recuadro. Solo se guarda
la esquina donde empezó; la selección se reconstruye desde cero en cada
frame, a partir de esa esquina y del puntero, de modo que volver a hacer
el recuadro más pequeño deselecciona notas:

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
cualesquiera, en cualquier dirección en que se arrastrara el recuadro, y
`intersects` es verdadero cuando dos rectángulos se solapan en lo más
mínimo, así que basta con que el recuadro toque una nota.

## Mover un grupo

Pulsar una nota seleccionada (en cualquier parte salvo su extremo
derecho, que sigue redimensionando) inicia un arrastre de grupo:

```rust
struct GroupDrag {
    /// The cell where the drag started.
    from_step: usize,
    from_pitch: Note,
    /// The notes as they were when the drag started.
    originals: Vec<NoteEvent>,
    /// The notes as they are now, after the moves so far.
    moved: Vec<NoteEvent>,
}
```

En cada frame del arrastre, la nueva posición se calcula a partir de las
*originales*, con la distancia completa desde la celda de inicio hasta
el puntero:

```rust
let steps = step as isize - drag.from_step as isize;
let semitones = i32::from(pitch.midi()) - i32::from(drag.from_pitch.midi());
let new = tools::shift(&drag.originals, steps, grid.length)
    .and_then(|shifted| tools::transpose(&shifted, semitones))
    .filter(|moved| fits_view(moved));
```

Mover las notas un poco en cada frame (desde `moved`) sería la manera
obvia, pero los pequeños errores se acumularían: un movimiento rechazado
en el borde del patrón se perdería, y las notas acabarían en un sitio
distinto al que marca el puntero. Empezar desde las originales cada vez
significa que las notas están siempre exactamente donde dice el puntero,
y arrastrar de vuelta al inicio las devuelve exactamente a como estaban.

Las dos herramientas se unen con `Option::and_then`: mover, y *si eso ha
funcionado*, transponer el resultado. `filter` rechaza entonces las
notas fuera de las filas que muestra el piano roll. La sustitución va de
`moved` (lo que ahora está en el patrón) a `new`, y `moved` se
actualiza a lo que devolvió `replace`.

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
una referencia a `self.group_drag`, el borrow checker lo rechazaría: una
parte de `self` quedaría prestada mientras todo `self` está prestado de
forma mutable. `Option::take` mueve el arrastre fuera y deja `None`, de
modo que `drag` es un valor aparte y `self` queda libre; el arrastre se
devuelve al final.

Cada frame del arrastre produce un `ReplaceNotes`, y cada uno pasa por
`edit`. Aun así forman una sola acción de deshacer: el
[historial del paso 5](../step-5/history.md) mantiene una acción abierta
mientras un botón del ratón esté pulsado. Y como la acción ya está
abierta, `edit` no copia todo el proyecto otra vez en cada frame; envía
al hilo de audio la diferencia respecto al último estado que se le dio
como mucho cada 50 ms, y el resto cuando se suelta el botón. Nada en el
piano roll tuvo que saber ninguno de los dos detalles.

## El teclado

### Quién recibe las teclas

Las teclas de flecha y Supr son útiles en el piano roll, pero Ferrous
Lineup tiene otras ventanas, y una tecla que cambie notas en una ventana
que no estás mirando sería una sorpresa desagradable. Así que el piano
roll solo toma las teclas después de un clic dentro de su ventana, hasta
que haya un clic en cualquier otro sitio:

```rust
let press = ctx.input(|input| {
    input
        .pointer
        .any_pressed()
        .then(|| input.pointer.press_origin())
        .flatten()
});
if let Some(pos) = press {
    self.has_keyboard = self.window_rect.is_some_and(|rect| rect.contains(pos));
}
```

- `bool::then(closure)` es el primo perezoso de `then_some`:
  `Some(closure())` si la condición es verdadera, `None` si no, sin
  llamar al closure cuando es falsa.
- El propio `press_origin()` devuelve un `Option<Pos2>`, así que `then`
  produce un `Option<Option<Pos2>>`, y `flatten` lo convierte en un
  `Option<Pos2>`.
- El rectángulo de la ventana es el del frame *anterior*, guardado
  después de `Window::show`. Durante este frame, la ventana todavía no
  se ha dibujado, así que la posición del último frame es lo mejor que
  hay, y una posición de hace un frame está lo bastante cerca para un
  clic.

Las teclas también se ignoran mientras un campo de texto tiene el
teclado (`ctx.text_edit_focused()`), de modo que escribir el nombre de
un patrón nunca mueve notas.

### El orden de los atajos de teclado

Cada tecla es un `KeyboardShortcut`: una tecla y sus modificadores.

```rust
const KEY_UP: KeyboardShortcut = shortcut(Modifiers::NONE, Key::ArrowUp);
const OCTAVE_UP: KeyboardShortcut = shortcut(Modifiers::SHIFT, Key::ArrowUp);
const IN_KEY_UP: KeyboardShortcut = shortcut(Modifiers::ALT, Key::ArrowUp);
```

`shortcut` es una `const fn`, una función lo bastante simple como para
ejecutarse durante la compilación, y por eso sus resultados pueden ser
`const`s.

El piano roll le pregunta a egui si cada una se pulsó con
`input.consume_shortcut(&shortcut)`. Eso devuelve `true` una vez y
«consume» la pulsación para que nada más reaccione a ella. Aquí importa
un detalle de egui: empareja los atajos de forma *lógica*, lo que
significa que las teclas Mayús o Alt de más no impiden un
emparejamiento. Mayús+↑ cuenta también como una pulsación de ↑.
Comprobados en el orden equivocado, Mayús+↑ sería devorado por el atajo
de «subir un semitono» antes de que el de «subir una octava» llegara a
verlo. Así que la lista va de lo más específico a lo menos:

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

`find` se detiene en el primer atajo pulsado, así que como mucho se
ejecuta una herramienta por frame, y a los atajos posteriores, más
cortos, ni siquiera se les pregunta.

### Un enum para el menú y las teclas

El menú Herramientas y las teclas producen ambos un `Tool`:

```rust
enum Tool {
    Transpose(i32),
    InKey(i32),
    Shift(isize),
    SnapToKey,
    Invert { up: bool },
    Arpeggiate,
    Delete,
    SelectAll,
    SelectNone,
}
```

y `use_tool` convierte cualquier `Tool` en una llamada a la función
adecuada de `lineup_core::tools`, seguida de `replace`. Venga una
herramienta del menú o de una tecla, sigue exactamente el mismo camino,
así que las dos nunca pueden comportarse de manera distinta. El menú
muestra la tecla de cada herramienta a su lado, escrita como el sistema
escribe las teclas (`ctx.format_shortcut`), así que sirve también de
lista de atajos de teclado.

**Cambiado después.** El enum ha crecido a medida que llegaban más
herramientas: `Quantize`, y `Copy`, `Cut`, `Paste` y `Duplicate` para el
portapapeles de notas. Todas se convierten aún en un `Tool` y pasan por
`use_tool`; un pegado añade las notas con `AddNotes`, y un corte
sustituye la selección por un `new` vacío mediante `ReplaceNotes`.

### Un menú que se queda abierto

Los menús de egui se cierran con cualquier clic dentro de ellos, lo cual
está bien para una lista de comandos. Pero el menú Herramientas también
contiene opciones: el orden y la velocidad del arpegio. Que el menú se
cerrara al elegir «arriba y abajo» te obligaría a abrirlo otra vez para
pulsar «Arpegiar acordes». Así que a este menú se le dice que se cierre
solo con un clic *fuera* de él, y cada herramienta lo cierra a mano:

```rust
let stay_open = egui::containers::menu::MenuConfig::new()
    .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside);
let menu = egui::containers::menu::MenuButton::new("Tools").config(stay_open);
menu.ui(ui, |ui| {
    let mut item = |ui: &mut Ui, text: &str, tool: Tool, shortcut: Option<KeyboardShortcut>| {
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
Asigna a `picked`, una variable externa, así que toma prestado `picked`
de forma mutable durante toda la vida del closure, y hay que declararlo
`let mut item` (llamarlo cambia lo que capturó: es un `FnMut`). El `ui`
se pasa como argumento en cada llamada en lugar de capturarse, porque el
código entre llamadas también usa `ui`, y un closure que lo retuviera lo
impediría.

## Notas fantasma: oír los otros canales

Un acorde a menudo se reparte entre instrumentos: el pad toca Do y Sol, el
bajo toca Mi, y juntos forman C/E. Antes de este paso, el carril de
acordes solo miraba el canal que se estaba editando, así que decía «Do5»
(un power chord) donde tú oyes C/E.

Ahora el piano roll escucha también a los otros canales. No a todos: los
canales de batería guardan steps planos, y un bombo no forma parte de la
armonía. Un canal cuenta si alguna de sus notas no es un step plano:

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
este, y los otros cuando «Notas fantasma» está marcado. El carril de
acordes, la línea que nombra el acorde bajo el puntero y las sugerencias
reciben todos `heard`, y reúnen sus notas con una pequeña función
auxiliar:

```rust
/// The pitches held during `step` on any of `channels`.
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

Las notas fantasma se pintan antes que las notas de este canal, de modo
que las notas de este canal quedan siempre encima, en gris con un borde
fino. Solo se dibujan: los clics miran únicamente las notas de este
canal, así que las notas fantasma nunca pueden seleccionarse, moverse ni
borrarse por accidente. (No son lo mismo que el contorno que muestra lo
que colocaría un clic, al que el paso 3 también llamaba fantasma. En el
código, a ese ahora se le llama contorno.)

Cuando ningún otro canal toca melodías o acordes, la casilla aparece en
gris, y su texto al pasar el ratón dice por qué:

```rust
ui.add_enabled_ui(!alone, |ui| {
    ui.checkbox(&mut self.ghosts, "Ghost notes")
        .on_hover_text("Show the notes of the other channels faintly, ...")
        .on_disabled_hover_text("No other channel plays melodies or chords yet.");
});
```

## El carril de velocidad

### Hacerle sitio

El carril se coloca bajo la cuadrícula. La cuadrícula es un
`ScrollArea` que llena la altura que se le da, y la primera versión le
daba toda la altura que quedaba en la ventana. El carril se dibujaba
debajo, así que el contenido de la ventana era un poco más alto que la
ventana. Las ventanas de egui crecen para adaptarse a su contenido, así
que al frame siguiente la ventana era más alta, la cuadrícula llenaba la
nueva altura, el carril volvía a quedar debajo... y la ventana crecía
unos píxeles en cada frame, durante todo el tiempo que la app estuviera
en marcha.

La solución es dar a la cuadrícula exactamente la altura que queda una
vez que el carril tiene su parte:

```rust
// The velocity lane goes below the grid, so the grid leaves room
// for it: the lane, the gap, and egui's spacing between widgets.
// Taking even a pixel too much would make the window grow a
// little on every frame.
let below = VELOCITY_HEIGHT + 4.0 + 2.0 * ui.spacing().item_spacing.y;
let grid_height = (ui.available_height() - below).max(96.0);
egui::ScrollArea::vertical()
    .auto_shrink(false)
    .max_height(grid_height)
    .show(ui, |ui| { /* the grid */ });
ui.add_space(4.0);
self.velocity_lane(ui, pattern, step_width, actions);
```

Lo fácil de olvidar es `item_spacing`: egui pone un pequeño hueco
después de cada widget, aquí después del scroll area y después del
espacio. Olvidar esos dos huecos bastaba para hacer avanzar la ventana.
La manera de comprobarlo era simple: tomar dos capturas de pantalla con
unos segundos de diferencia y compararlas.

### Fijar velocidades

El carril es un solo widget que detecta clics y arrastres. Mientras el
botón está pulsado sobre él, la altura del puntero fija la velocidad, y
su posición x elige el step:

```rust
if response.is_pointer_button_down_on()
    && let Some(pos) = response.interact_pointer_pos()
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
  del carril durante un arrastre. Convertir un float con `as` nunca se
  desborda (300.0 se convierte en 255 y -5.0 en 0), pero 0 sería una
  nota que no puedes oír, así que el clamp se asegura de que la
  velocidad sea al menos 1.
- Un acorde tiene varias notas en el mismo step. Todas cambian, a menos
  que algunas estén seleccionadas: entonces solo lo hacen las
  seleccionadas. Así es como haces que la nota más aguda de un acorde
  suene más fuerte que las demás, como hacen los pianistas para destacar
  una melodía.
- Las notas que ya tienen esa velocidad se dejan fuera de `old`, así que
  mantener el botón quieto no envía el mismo cambio en cada frame.

Barrer por el carril cambia cada step a medida que el puntero pasa por
encima, y toda la barrida es una sola acción de deshacer, como cualquier
arrastre.

La velocidad también se ve en la cuadrícula. El color de una nota se
multiplica por un factor entre 0,55 y 1 según su velocidad
(`fill.gamma_multiply(0.55 + 0.45 * f32::from(note.velocity) / 127.0)`),
así que las notas suaves son más oscuras, pero nunca tanto como para
desaparecer.

## Probar a mano

Las herramientas están cubiertas por pruebas unitarias, pero el piano
roll solo puede probarse usándolo. Ejecutar la app y probar cada función
encontró tres problemas que ninguna prueba unitaria habría encontrado:

- la ventana que crecía en cada frame (arriba),
- el menú Herramientas que se cerraba al elegir un orden de arpegio
  (arreglado con `CloseOnClickOutside`),
- las inversiones estampadas que caían por encima del puntero en lugar
  de a su alrededor (`stamp_notes` bajando el acorde de nuevo).

Esa es una buena costumbre para cualquier código de interfaz: escribe la
lógica de modo que se pueda probar sin la ventana, y luego prueba la
ventana igualmente.
