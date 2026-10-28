# De los pasos a las notas

Archivos: [`crates/lineup-core/src/pattern.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/pattern.rs)
y [`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs)

En el paso 2, un patrón era una cuadrícula de interruptores de encendido
y apagado, una fila de `[bool; 64]` por canal. Un piano roll necesita
más: cada nota tiene una altura, un inicio y una duración, y varias
notas pueden empezar en el mismo paso (un acorde). Así que el paso 3
reemplaza los interruptores por una lista de notas por canal, y el step
sequencer se convierte en una forma especial de mirar esa lista.

## Una nota en un patrón

```rust
pub const STEP_NOTE: Note = Note::C4;
pub const DEFAULT_VELOCITY: u8 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoteEvent {
    pub pitch: Note,
    pub start: usize,  // in steps
    pub length: usize, // in steps, at least 1
    pub velocity: u8,  // 1..=127, how hard it is played
}

impl NoteEvent {
    pub fn step(start: usize) -> Self {
        NoteEvent::new(STEP_NOTE, start, 1)
    }

    pub fn sounds_at(&self, step: usize) -> bool {
        (self.start..self.end()).contains(&step)
    }

    pub fn is_step(&self) -> bool {
        self.pitch == STEP_NOTE && self.length == 1
    }
}
```

El nombre es `NoteEvent` porque `Note` ya estaba ocupado por el tipo de
altura del paso 1. En la programación de audio, algo que sucede en un
momento (una nota que empieza, un mando que gira) suele llamarse
*evento*.

`self.start..self.end()` es un **rango**, un valor con su propio tipo
(`Range<usize>`), y `contains` pregunta si un número está dentro. Los
rangos incluyen su inicio y excluyen su final, así que una nota desde el
paso 4 con duración 2 suena en los pasos 4 y 5, no en el 6.

Todos los campos son `Copy`, así que `NoteEvent` también deriva `Copy`:
las notas se pasan de un lado a otro como números, lo que mantiene el
código libre de rompecabezas de préstamos.

La **velocity** es la fuerza con la que se pulsa una tecla, de 1 a 127,
como en MIDI. El motor de audio la convierte en volumen. El piano roll
todavía no la edita (un ejercicio al final de este paso lo hace).

## El patrón

```rust
pub struct Pattern {
    length: usize,
    rows: Vec<Vec<NoteEvent>>,
}
```

Una fila ahora es un `Vec<NoteEvent>`, las notas del canal en el orden
en que se añadieron. Las viejas funciones de pasos conservan sus nombres
y su significado, escritas en términos de notas:

```rust
/// Whether a note starts on this step (a step is "on").
pub fn is_on(&self, channel: usize, step: usize) -> bool {
    self.notes(channel).iter().any(|note| note.start == step)
}

/// Switches a step on (adds a step note) or off (removes every note starting there).
pub fn set(&mut self, channel: usize, step: usize, on: bool) -> bool {
    if step >= MAX_STEPS {
        return false;
    }
    let Some(row) = self.rows.get_mut(channel) else {
        return false;
    };
    row.retain(|note| note.start != step);
    if on {
        row.push(NoteEvent::step(step));
    }
    true
}
```

`retain` conserva los elementos para los que el closure devuelve `true`
y descarta el resto, en el sitio. Como las funciones de pasos
conservaron sus firmas, las pruebas de patrones del paso 2 siguen
pasando con solo cambios pequeños.

`notes` devuelve un slice, y un canal que no existe simplemente no tiene
ninguno:

```rust
pub fn notes(&self, channel: usize) -> &[NoteEvent] {
    match self.rows.get(channel) {
        Some(row) => row,
        None => &[],
    }
}
```

`row` es un `&Vec<NoteEvent>`, y Rust lo convierte en `&[NoteEvent]`
automáticamente (una **coerción de deref**: un `Vec` siempre se puede
ver como un slice de sus elementos). `&[]` es un slice vacío que, como
un literal de string, vive en el programa y no cuesta nada.

## Mantener las notas en la cuadrícula: sintaxis de actualización de structs

`add_note` y `replace_note` primero hacen que la nota quepa en la
cuadrícula. Una nota debe empezar antes de `MAX_STEPS`, durar al menos
un paso, y se acorta si se pasaría del final:

```rust
fn fitted(self) -> Option<NoteEvent> {
    if self.start >= MAX_STEPS || self.length == 0 {
        return None;
    }
    Some(NoteEvent {
        length: self.length.min(MAX_STEPS - self.start),
        ..self
    })
}
```

`..self` es la **sintaxis de actualización de structs**: «cada campo que
no escribí toma su valor de `self`». Es la forma habitual de hacer una
copia modificada de un struct. El piano roll la usa para mover una nota
(`NoteEvent { pitch, start, ..note }`) y para redimensionar otra
(`NoteEvent { length, ..note }`).

`add_note` rechaza una segunda nota con la misma altura en el mismo paso
de inicio, y devuelve el índice de la nota nueva, que el piano roll usa
para seguir arrastrando una nota que acaba de crear:

```rust
pub fn add_note(&mut self, channel: usize, note: NoteEvent) -> Option<usize> {
    let note = note.fitted()?;
    let row = self.rows.get_mut(channel)?;
    let duplicate = row
        .iter()
        .any(|other| other.pitch == note.pitch && other.start == note.start);
    if duplicate {
        return None;
    }
    row.push(note);
    Some(row.len() - 1)
}
```

## Un iterador que presta: `impl Iterator + '_`

Para nombrar el acorde en un paso, el piano roll necesita «las alturas
sostenidas en el paso `s`». El patrón las devuelve como un iterador en
lugar de un `Vec` nuevo:

```rust
pub fn sounding_at(&self, channel: usize, step: usize) -> impl Iterator<Item = Note> + '_ {
    self.notes(channel)
        .iter()
        .filter(move |note| note.sounds_at(step))
        .map(|note| note.pitch)
}
```

No se recolecta ni se reserva nada: el iterador recorre las notas solo
cuando alguien saca elementos de él, y `analyze` (capítulo anterior)
hace exactamente eso, leyendo las alturas directamente del patrón.

El iterador guarda una referencia a `self.rows`, así que no debe
sobrevivir al patrón. `+ '_` lo dice. `'_` es un **lifetime anónimo**:
«el lifetime del argumento prestado», aquí `&self`. Desde la edición
2024, el compilador asume esto para todo tipo de retorno `impl Trait`,
así que el código compilaría sin él; está escrito para que quien lea
pueda ver el préstamo. (En ediciones antiguas era obligatorio.) `move`
le da al closure de `filter` su propia copia de `step`, porque el
closure sobrevive a la llamada a la función que recibió `step`.

Los iteradores así son **perezosos** (*lazy*), y se componen. En el
piano roll, todo el carril de acordes es una expresión:

```rust
let labels: Vec<Option<String>> = (0..length)
    .map(|step| analyze(project.pattern().sounding_at(channel, step)).label(spelling))
    .collect();
```

## ¿Pasos o una melodía?

```rust
pub fn is_step_row(&self, channel: usize) -> bool {
    self.notes(channel).iter().all(NoteEvent::is_step)
}
```

`all` es verdadero si el closure es verdadero para cada elemento (y
también para una lista vacía). En lugar de un closure, se le pasa el
propio método, `NoteEvent::is_step`, una función que recibe
`&NoteEvent`, que es exactamente lo que produce `iter()`. El channel
rack dibuja una fila como botones de paso cuando esto es verdadero, y
como una imagen pequeña de piano roll cuando no.

## El proyecto: una tonalidad y una progresión de acordes

`Project` ganó un `scale: Scale` (Do mayor por defecto): la tonalidad en
la que el piano roll resalta y nombra los acordes. `ChannelKind::Synth`
perdió su campo `note`, porque la altura del sintetizador ahora viene de
cada nota.

La demo reproduce C, Am, F, G, una de las progresiones más usadas del
pop (I, vi, IV, V). Cada acorde es una nota de bajo y tres notas del
acorde:

```rust
let chords: [[u8; 4]; 4] = [
    [48, 60, 64, 67], // C:  C3, C4 E4 G4
    [45, 57, 60, 64], // Am: A2, A3 C4 E4
    [41, 57, 60, 65], // F:  F2, A3 C4 F4
    [43, 59, 62, 67], // G:  G2, B3 D4 G4
];
for (i, chord) in chords.iter().enumerate() {
    for &midi in chord {
        let note = NoteEvent::new(Note::new(midi), i * 8, 8);
        project.pattern.add_note(synth, note);
    }
}
```

Mira las tres notas superiores: Do4 se mantiene en los tres primeros
acordes, Mi4 se mantiene de C a Am, La3 se mantiene de Am a F. Cada acorde
cambia la menor cantidad posible de notas, y esas se mueven uno o dos
pasos. Esto se llama **conducción de voces**, y es por lo que una
progresión suena suave en lugar de ir saltando. En el piano roll puedes
verlo: líneas horizontales largas de notas que continúan de un acorde al
siguiente.

Una prueba reproduce la demo a través de `analyze` y comprueba los
nombres de los acordes, así que si alguno de estos números está mal,
`cargo test` dice qué acorde se rompió:

```rust
assert_eq!(chords, ["C", "Am", "F", "G"]);
```
