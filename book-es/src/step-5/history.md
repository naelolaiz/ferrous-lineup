# Deshacer y rehacer: un historial de copias

Archivos: [`crates/lineup-core/src/history.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/history.rs)
y [`crates/lineup-app/src/app/edit.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/edit.rs)

## Dos formas de deshacer

Hay dos formas clásicas de construir el deshacer.

El **patrón comando** registra cada cambio como una instrucción que sabe
revertirse a sí misma: "add note Do4 at step 3" se deshace con "remove
note Do4 at step 3". El historial es una lista de instrucciones. Usa muy
poca memoria, pero cada tipo de cambio necesita su propia operación
inversa, escrita a mano, y cada una es un lugar nuevo para equivocarse.
Piensa en lo que necesitaría deshacer "eliminar patrón": volver a poner el
patrón en su antigua posición, con su nombre y su longitud y las notas de
cada canal, volver a poner cada clip que lo usaba y renumerar de nuevo
los clips de los patrones que van detrás. Si se olvida un detalle,
deshacer devuelve en silencio una canción ligeramente distinta.

Las **instantáneas** guardan una copia completa del proyecto de antes de
cada paso deshacible.
Deshacer vuelve a poner la copia. No hay nada que revertir, así que no
hay nada que revertir mal: haga lo que haga el cambio, la copia es
exactamente lo que había antes. El coste es la memoria y el tiempo de
hacer una copia.

Ferrous Lineup usa instantáneas. Un `NoteEvent` ocupa 24 bytes, y la
canción de demostración tiene unas pocas docenas, así que una copia de
todo el proyecto de demostración son unos pocos kilobytes. Incluso un
proyecto grande con 10,000 notas guarda unos 240 KB de notas, y hacer
una copia tarda mucho menos de un milisegundo. La aplicación intenta
guardar como mucho 256 MiB de instantáneas: una canción pequeña obtiene
los 100 pasos completos, mientras que una muy grande obtiene al menos 10
(que pueden pasarse del presupuesto). Es un precio justo por un
deshacer que es correcto por construcción. (El ejercicio 6 muestra cómo
abaratar mucho las copias, si un proyecto llega a ser lo bastante grande
para necesitarlo.)

## `History<T>`

El historial vive en `lineup-core`, y no sabe nada de proyectos:

```rust
struct Step<T> {
    label: &'static str,
    state: T,
}

pub struct History<T> {
    /// The oldest step is at the front, the newest at the back.
    undo: VecDeque<Step<T>>,
    redo: Vec<Step<T>>,
    limit: usize,
    /// Whether the newest undo step still takes in new changes.
    open: bool,
}

impl<T> History<T> {
    pub fn new(limit: usize) -> Self { ... }
    pub fn record(&mut self, label: &'static str, before: T) { ... }
    pub fn close(&mut self) { ... }
    pub fn undo(&mut self, current: &mut T) -> Option<&'static str> { ... }
    pub fn redo(&mut self, current: &mut T) -> Option<&'static str> { ... }
    pub fn undo_label(&self) -> Option<&'static str> { ... }
    pub fn redo_label(&self) -> Option<&'static str> { ... }
    pub fn clear(&mut self) { ... }
}
```

`History<T>` es **genérico**: `T` es aquello que se está recordando. La
aplicación usa un `History<Project>`; las pruebas usan un `History<i32>`,
lo que las hace cortas y fáciles de leer:

```rust
#[test]
fn changes_join_the_open_step() {
    let mut history = History::default();
    let mut value = 0;
    // A drag: three changes before the mouse button is released.
    for new in [1, 2, 3] {
        history.record("drag", value);
        value = new;
    }
    history.close();
    history.undo(&mut value);
    assert_eq!(value, 0, "one undo goes back to before the drag");
}
```

Mira la línea `impl`: `impl<T> History<T>`, sin ningún **trait bound**
sobre `T`. Ni siquiera `Clone`. El historial nunca copia un `T`; solo
mueve los valores que recibe de una lista a otra. Quien llama hace la
copia (`self.project.clone()`) y se la entrega. Si un método de `History`
llamara alguna vez a `.clone()` sobre un estado, el compilador se negaría
con "no method named `clone` found for type parameter `T`", y tendríamos
que escribir `impl<T: Clone>`. Un tipo genérico sin bounds es una promesa
fuerte: el código no puede hacer nada con un `T` salvo guardarlo,
moverlo y soltarlo.

El compilador hace una copia separada del código para cada `T` con el que
se usa (esto es la **monomorfización**, del paso 1), así que un
`History<Project>` es tan rápido como si se hubiera escrito a mano para
`Project`.

### Las etiquetas `&'static str`

Cada paso tiene una etiqueta, como `"move clip"`, para el menú Edición. Su
tipo es `&'static str`: una referencia a texto que vive durante todo el
programa. Cada string literal escrito en el código tiene este tipo,
porque el texto se guarda dentro del propio archivo del programa. Así que
una etiqueta no cuesta nada: no se construye ningún `String`, no se
reserva nada, y el historial no guarda ningún préstamo que pueda terminar
demasiado pronto. El precio es que una etiqueta no se puede crear en
tiempo de ejecución ("rename pattern Beat"), porque ese texto no
existiría hasta que el programa lo construyera. El ejercicio 7 pregunta
cómo permitir ambas cosas.

### `VecDeque`: una lista con dos extremos baratos

Cuando ya hay `limit` pasos, registrar uno más olvida el más antiguo. En
un `Vec`, quitar el primer elemento (`remove(0)`) mueve todos los demás
un lugar hacia abajo. Un **`VecDeque`** ("cola de dos extremos", se dice
"vec deck") es un ring buffer, como las colas entre hilos del paso 2:
lleva la cuenta de dónde está su frente, así que `push_back` y
`pop_front` son ambos baratos.

El límite no es una constante en la aplicación. `History::new` lo recibe,
y `undo_limit_for` lo calcula a partir del tamaño del proyecto:

```rust
const UNDO_BYTES: usize = 256 * 1024 * 1024;
const MIN_UNDO_STEPS: usize = 10;

pub(super) fn undo_limit_for(project: &Project) -> usize {
    limit_for_bytes(project.heap_bytes())
}

fn limit_for_bytes(bytes: usize) -> usize {
    (UNDO_BYTES / bytes.max(1)).clamp(MIN_UNDO_STEPS, DEFAULT_LIMIT)
}
```

`heap_bytes` cuenta las notas y los nombres, las cosas cuyo número crece
con la canción. Un proyecto diminuto divide 256 MiB entre unos pocos
kilobytes y obtiene más que `DEFAULT_LIMIT`, así que se limita a 100. Un
proyecto enorme obtiene menos, pero nunca menos de 10, para que deshacer
no desaparezca solo porque la canción es larga. Abrir un archivo
construye un `History` nuevo con el límite de ese proyecto.
`History::default()` sigue significando 100, que es lo que usan las
pruebas pequeñas.

```rust
pub fn record(&mut self, label: &'static str, before: T) {
    self.redo.clear();
    if self.open {
        return;
    }
    self.undo.push_back(Step { label, state: before });
    if self.undo.len() > self.limit {
        self.undo.pop_front();
    }
    self.open = true;
}
```

Cada cambio nuevo vacía la lista de redo. Después de deshacer tres pasos
y cambiar algo, esos tres pasos pertenecen a un futuro que ya no existe.
(Unos pocos programas guardan un árbol entero de historiales, para no
perder nunca ninguna rama; la mayoría simplemente descarta el futuro
antiguo, como hace este.)

La lista de redo es un `Vec` normal, porque los pasos solo se añaden y se
quitan por su final.

### `mem::swap`: deshacer sin copiar

```rust
pub fn undo(&mut self, current: &mut T) -> Option<&'static str> {
    let mut step = self.undo.pop_back()?;
    // Now `current` holds the old state and `step` the newer one.
    std::mem::swap(current, &mut step.state);
    let label = step.label;
    self.redo.push(step);
    self.open = false;
    Some(label)
}
```

`pop_back()` devuelve un `Option`, y el operador `?` funciona sobre
`Option` en una función que devuelve un `Option`: si no hay nada que
deshacer, `undo` devuelve `None` al momento y no cambia nada.

**`std::mem::swap`** intercambia los valores que hay detrás de dos
referencias `&mut`. Después, `current` (el proyecto de la aplicación)
guarda la versión más antigua, y el paso guarda la versión de la que
acabamos de salir, lista para redo. No se copia ni se reserva nada: un
`Project` es sobre todo `Vec`s, y un `Vec` es un puntero, una longitud y
una capacidad, así que intercambiar dos proyectos intercambia unos pocos
de esos valores pequeños, sea cual sea el tamaño de la canción. El
borrow checker es lo que hace esto sencillo: `swap` necesita dos
referencias `&mut`, y dos referencias `&mut` nunca pueden apuntar al
mismo valor, así que `swap` puede intercambiar los bytes de los dos
valores sin comprobar nada.

`redo` es la imagen especular: saca de `redo` y mete en `undo`.

## ¿Cuándo termina un paso?

Arrastrar un clip por la lista de reproducción lo mueve un poco en muchos fotogramas,
y cada movimiento es una edición. Si cada uno fuera su propio paso,
deshacer el arrastre necesitaría tantas pulsaciones de Ctrl+Z como
fotogramas hubiera. Así que el historial mantiene el paso más nuevo
**abierto**, y `record` no hace nada mientras lo esté: el paso ya guarda
el proyecto de antes de que empezara el arrastre, que es exactamente
adonde debería volver deshacer.

La aplicación decide cuándo termina un paso, porque solo ella sabe qué
está haciendo el ratón. Llama al historial al principio y al final de
cada fotograma:

```rust
/// Called at the start of each frame. A press of a mouse button
/// always begins a new undo step.
pub(super) fn start_frame_for_undo(&mut self, ctx: &egui::Context) {
    if ctx.input(|input| input.pointer.any_pressed()) {
        self.finish_step_sync();
        self.history.close();
    }
}

/// Called at the end of each frame. While a mouse button is held (a
/// drag), a text field has the keyboard (typing a name), or a take is
/// being recorded, changes join one undo step; once all are over, the
/// step ends.
pub(super) fn end_frame_for_undo(&mut self, ctx: &egui::Context) {
    let holding = ctx.input(|input| input.pointer.any_down());
    if !holding && !ctx.text_edit_focused() && !self.is_recording() {
        self.finish_step_sync();
        self.history.close();
    }
}
```

Sigue unos cuantos gestos a través de estas dos funciones:

- **Un clic en un paso.** egui informa del clic en el fotograma en que se
  suelta el botón. La edición de ese fotograma abre un paso, y el final
  del mismo fotograma lo cierra, porque ya no hay ningún botón pulsado:
  un cambio, un paso.
- **Un arrastre.** La pulsación cierra lo que estuviera abierto. Cada
  fotograma del arrastre edita el proyecto mientras el botón está
  pulsado, así que las ediciones se unen en un paso. El fotograma en que
  se suelta el botón lo cierra.
- **Escribir un nombre de patrón.** Mientras el campo de texto tiene el
  foco del teclado, cada letra se une al mismo paso. Hacer clic en otro
  sitio le quita el foco y lo cierra, así que deshacer devuelve todo el
  nombre antiguo de una vez.
- **Escribir un tempo.** Hacer clic en la casilla de BPM la convierte en
  un campo de texto, así que se aplica la misma regla: todo lo escrito
  ahí es un paso, que termina cuando sales de la casilla.

`start_frame_for_undo` importa en un caso: un clic que empieza cuando un
campo de texto aún tiene el foco. La pulsación cierra el paso de
escritura, para que el cambio del clic no se le una.

Cerrar un paso es ahora más que poner `open = false`. Mientras un paso
está abierto, la aplicación no copia el proyecto una y otra vez, y no
envía el cambio de cada fotograma al hilo de audio. Guarda una copia
`synced` del último estado que recibió el hilo de audio, y como mucho
cada 50 ms envía la diferencia respecto a esa copia. Cerrar el paso llama
a `finish_step_sync`, que envía lo que aún falte antes de que el
historial deje de aceptar cambios. El capítulo siguiente muestra la
función `edit` que hace esto.

### Todo pasa por `edit`

El campo del nombre del patrón muestra lo estricta que es la regla "cada
cambio pasa por `edit`". El campo de texto de egui quiere un
`&mut String` para cambiar. Darle el nombre del propio proyecto cambiaría
el proyecto directamente, a espaldas del historial: sin paso de deshacer
y (como muestra el capítulo siguiente) sin mensaje al hilo de audio
tampoco. Así que el campo edita una copia, y la copia pasa por `edit`
cuando ha cambiado:

```rust
// The field edits a copy, so the change can go through `edit`.
let mut name = self.project.patterns()[self.pattern].name.clone();
if ui.text_edit_singleline(&mut name).changed() {
    self.edit_pattern("rename pattern", |pattern| pattern.name = name);
}
```

El tipo `Project` ayuda aquí: sus listas son privadas, y solo sus métodos
pueden cambiarlas. Cualquier cosa que cambie un proyecto necesita un
`&mut Project`, y en la aplicación el único sitio que lo entrega es
`edit`.

## Atajos de teclado

```rust
const UNDO: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::Z);
const REDO: KeyboardShortcut =
    KeyboardShortcut::new(Modifiers::COMMAND.plus(Modifiers::SHIFT), Key::Z);
/// The other common shortcut for redo, used on Windows.
const REDO_Y: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::Y);

pub(super) fn edit_shortcuts(&mut self, ctx: &egui::Context) {
    let pressed = |shortcut| ctx.input_mut(|input| input.consume_shortcut(&shortcut));
    // Ctrl+Z would also match Ctrl+Shift+Z, so redo is checked first.
    if pressed(REDO) || pressed(REDO_Y) {
        self.redo();
    } else if pressed(UNDO) {
        self.undo();
    }
}
```

`Modifiers::COMMAND` es Ctrl en Linux y Windows y ⌘ en un Mac, como en el
paso 4. El `consume_shortcut` de egui es indulgente con las teclas de
más: un atajo sin Mayús también encaja cuando Mayús está pulsado.
Normalmente eso es lo que quieres (en muchos teclados `+` necesita
Mayús, y Ctrl++ debería seguir funcionando), pero aquí significa que
Ctrl+Mayús+Z contaría también como Ctrl+Z. Comprobar primero redo, y usar
`else if`, lo resuelve.

`||` se detiene en el primer `true`, así que si se pulsó Ctrl+Mayús+Z,
Ctrl+Y ni siquiera se mira. "Consume" significa que la pulsación se saca
de la entrada de egui, para que nada más en el mismo fotograma reaccione
a ella.

Estos atajos se omiten mientras un campo de texto tiene el teclado. Los
campos de texto de egui tienen su propio deshacer para escribir, y allí
Ctrl+Z debe deshacer las últimas letras, no el último cambio de la
canción.

El menú Edición muestra los mismos atajos, escritos como los escribe el
sistema, con `ctx.format_shortcut(&UNDO)` ("Ctrl+Z" en Linux, "⌘Z" en un
Mac), y pone la etiqueta en el elemento: `format!("Undo {label}")`.

## Deshacer y la estrella de cambios sin guardar

El paso 4 muestra una estrella en el título de la ventana cuando el
proyecto difiere del archivo guardado. No compara los dos proyectos en
cada fotograma: una edición pone un flag `check_dirty`, y la comparación
ocurre la próxima vez que se calcula el título. Deshacer y rehacer
vuelven a poner el flag, para que puedan hacer desaparecer la estrella,
porque el proyecto vuelve a ser igual que el guardado. Un simple
`dirty: bool` puesto en cada edición habría mantenido la estrella
después de deshacer.

Abrir un archivo, o Archivo > Nuevo, sustituye todo el proyecto y vacía el
historial. Deshacer metiéndote en otra canción sería desconcertante, y la
pregunta "¿Guardar cambios?" ya protege lo que dejas atrás.
