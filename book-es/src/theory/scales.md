# Escalas, modos y tonalidades

Archivos: [`crates/lineup-core/src/harmony/scale.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/scale.rs),
[`crates/lineup-core/src/key_map.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/key_map.rs),
[`crates/lineup-core/src/tools.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/tools.rs)
y [`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs)

La tonalidad es lo que mira casi toda la aplicación antes de mostrar
algo: el piano roll ilumina las filas de la tonalidad, las teclas de
armonía tocan sus acordes, la hoja de acordes nombra los acordes en
ella, la banda toca en ella, y el nombre de cada nota depende de ella.
Este capítulo trata del tipo `Scale` que guarda una tonalidad, de cómo
escribe una tonalidad cada una de sus notas, y de qué pasa cuando la
canción cambia de tonalidad, entera o a mitad de camino.

## La música, en pocas palabras

Una **escala** es una elección de algunas de las doce clases de altura,
contadas hacia arriba desde una **tónica**. La escala mayor son las
teclas blancas desde Do, Do Re Mi Fa Sol La Si: en semitonos por encima
de la tónica, 0 2 4 5 7 9 11. El mismo dibujo empezado en otra nota da
la escala mayor de esa nota: desde Re es Re Mi Fa♯ Sol La Si Do♯. Una
escala sobre una tónica concreta, usada como casa de una pieza, es una
**tonalidad**: Re mayor.

Las escalas que conoce Ferrous Lineup, en semitonos por encima de la
tónica:

| Escala | Semitonos |
|---|---|
| mayor | 0 2 4 5 7 9 11 |
| menor (natural) | 0 2 3 5 7 8 10 |
| menor armónica | 0 2 3 5 7 8 11 |
| menor melódica | 0 2 3 5 7 9 11 |
| dórico | 0 2 3 5 7 9 10 |
| frigio | 0 1 3 5 7 8 10 |
| lidio | 0 2 4 6 7 9 11 |
| mixolidio | 0 2 4 5 7 9 10 |
| locrio | 0 1 3 5 6 8 10 |
| pentatónica mayor | 0 2 4 7 9 |
| pentatónica menor | 0 3 5 7 10 |
| blues | 0 3 5 6 7 10 |
| cromática | las doce |

La menor armónica sube la séptima nota de la escala menor para que se
apoye en la tónica, un semitono más arriba; la menor melódica sube
también la sexta. (La menor melódica clásica vuelve a la menor natural
al bajar; la aplicación usa la forma ascendente en los dos sentidos,
como hacen los músicos de jazz.) Los **modos**, de dórico a locrio, son
la escala mayor empezada en otra de sus notas: Re dórico son las teclas
blancas desde Re. Así que cada modo comparte sus notas, y su armadura,
con una tonalidad mayor: su **relativa mayor**. La relativa mayor de La
menor es Do mayor, tres semitonos más arriba.

La **armadura** son los sostenidos o bemoles que se escriben al
principio de cada línea de una partitura. Cada tonalidad mayor tiene la
suya, y subir una quinta añade un sostenido: Do mayor no tiene ninguno,
Sol mayor uno, Re mayor dos. Bajar una quinta añade un bemol: Fa mayor
tiene uno, Si♭ mayor dos. Ese círculo de tonalidades a una quinta de
distancia es el **círculo de quintas**. El capítulo [Tonalidades y
modos](https://naelolaiz.github.io/ferrous-lineup/es/manual/keys-and-modes.html)
del manual cuenta todo esto con sonidos para probar.

## Clases de escala: un enum con datos

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScaleKind {
    Major,
    NaturalMinor,
    HarmonicMinor,
    MelodicMinor,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Locrian,
    MajorPentatonic,
    MinorPentatonic,
    Blues,
    Chromatic,
    Row(ToneRow),
    Set(PitchSet),
}
```

Las trece escalas de la música tonal son variantes simples. Las dos
últimas llevan sus notas consigo: una serie dodecafónica y un conjunto
de clases de altura también pueden ser la «tonalidad» de una canción,
para música que no tiene tonalidad en el sentido habitual. Tienen
capítulos propios ([Series dodecafónicas y sus
reglas](twelve-tone.md) y [Conjuntos de clases de altura, y
una armonía con cuatro nombres](sets.md)); aquí aparecen
sobre todo como los casos distintos. Un método los separa:

```rust
pub fn is_tonal(self) -> bool {
    !matches!(self, ScaleKind::Row(_) | ScaleKind::Set(_))
}
```

`Row(_)` encaja con un `Row` sea cual sea la serie que lleva.

Las notas de una escala se listan como semitonos por encima de la
tónica. Para las escalas tonales esas listas no cambian nunca, así que
están escritas dentro del programa y se prestan como `&'static [u8]`,
slices que pueden tener cualquier longitud, cinco para una escala
pentatónica, siete para la escala mayor:

```rust
fn tonal_intervals(self) -> &'static [u8] {
    match self {
        ScaleKind::Major => &[0, 2, 4, 5, 7, 9, 11],
        ScaleKind::NaturalMinor => &[0, 2, 3, 5, 7, 8, 10],
        // ...
        ScaleKind::Chromatic | ScaleKind::Row(_) | ScaleKind::Set(_) => {
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
        }
    }
}
```

Una serie o un conjunto tiene sus propias notas, que solo existen dentro
del valor, así que nada `'static` puede apuntar a ellas. Por eso el
`intervals` público devuelve un `Vec<u8>` propio, hecho a partir de la
serie, del conjunto o de la tabla:

```rust
pub fn intervals(self) -> Vec<u8> {
    match self {
        ScaleKind::Row(row) => row.semitones().to_vec(),
        ScaleKind::Set(set) => set.semitones(),
        kind => kind.tonal_intervals().to_vec(),
    }
}
```

El patrón de la última rama es un nombre simple, `kind`, que encaja con
cualquier cosa y la liga, así que todas las escalas tonales van ahí.

## Una tonalidad: una tónica y una clase

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Scale {
    pub root: PitchClass,
    pub kind: ScaleKind,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    respelled: bool,
}
```

`respelled` es para las pocas tonalidades que se pueden escribir de dos
maneras, más abajo. La mayoría de los métodos de `Scale` son cadenas
cortas de iteradores. Las clases de altura de la escala son la tónica
subida por cada intervalo:

```rust
pub fn pitch_classes(self) -> impl Iterator<Item = PitchClass> {
    self.kind
        .intervals()
        .into_iter()
        .map(move |interval| self.root.transpose(interval as i32))
}
```

`impl Iterator<Item = PitchClass>` dice «algún iterador de clases de
altura» sin nombrar su tipo real, que incluye un closure, y los closures
tienen tipos que nadie puede escribir. `into_iter()` sobre el `Vec` da
sus números por valor, y `move` le da al closure su propia copia de
`self` (un `Scale` es `Copy`), porque el iterador vive más que la
llamada.

El **grado** de una nota es su lugar en la escala. `position` lo
encuentra, contando desde 0, donde los músicos cuentan desde 1:

```rust
pub fn degree_of(self, pitch_class: PitchClass) -> Option<usize> {
    self.pitch_classes().position(|p| p == pitch_class)
}
```

En La menor, Mi es el grado 4 (los músicos dicen «el quinto»), y Fa♯ no
tiene grado. `pitch_set` reúne las mismas clases de altura en un
[`PitchSet`](pitch.md#un-conjunto-de-clases-de-altura-en-doce-bits),
así que el test de que cada modo comparte sus notas con su relativa
mayor es una comparación de tres números:

```rust
let c_major = Scale::default().pitch_set();
assert_eq!(Scale::new(D, ScaleKind::Dorian).pitch_set(), c_major);
assert_eq!(Scale::new(G, ScaleKind::Mixolydian).pitch_set(), c_major);
assert_eq!(Scale::new(A, ScaleKind::NaturalMinor).pitch_set(), c_major);
```

`Scale::default()` es Do mayor, e `is_heptatonic` dice si una escala
tiene siete notas. Los acordes apilados por terceras y los números
romanos solo tienen sentido en esas, así que muchos métodos empiezan
preguntándolo ([Acordes: construirlos, nombrarlos e
invertirlos](chords.md)).

## Sostenidos o bemoles: cómo se escribe una tonalidad

Cada tonalidad menor y cada modo comparte su armadura con su relativa
mayor, así que la pregunta «¿sostenidos o bemoles?» la responde esa
tonalidad mayor. `relative_major` dice cuánto más arriba está:

```rust
pub(crate) fn relative_major(self) -> u8 {
    match self {
        ScaleKind::Major
        | ScaleKind::MajorPentatonic
        | ScaleKind::Chromatic
        | ScaleKind::Row(_)
        | ScaleKind::Set(_) => 0,
        ScaleKind::NaturalMinor
        | ScaleKind::HarmonicMinor
        | ScaleKind::MelodicMinor
        | ScaleKind::MinorPentatonic
        | ScaleKind::Blues => 3,
        ScaleKind::Dorian => 10,
        ScaleKind::Phrygian => 8,
        ScaleKind::Lydian => 7,
        ScaleKind::Mixolydian => 5,
        ScaleKind::Locrian => 1,
    }
}
```

Re dórico son las teclas blancas desde Re, y Do está diez semitonos por
encima de Re; el Do de Mi frigio está ocho por encima de Mi, y así
sucesivamente. `pub(crate)` hace que el método se vea solo dentro de
`lineup-core`: el resto del programa hace las preguntas construidas
sobre él.

La escritura habitual depende entonces de esa tonalidad mayor. Las
tonalidades cuya armadura tiene bemoles son Fa, Si♭, Mi♭, La♭ y Re♭
mayor, y sus menores y modos:

```rust
fn usual_spelling(self) -> Spelling {
    use PitchClass::*;
    match self.root.transpose(self.kind.relative_major() as i32) {
        F | ASharp | DSharp | GSharp | CSharp => Spelling::Flats,
        // F# major and Gb major are the same keys of the piano, and
        // the tonic's name decides between them. Their modes are
        // named with sharps (G# Dorian, B Lydian), except F Locrian,
        // whose notes are those of Gb major: F Gb Ab Bb Cb Db Eb.
        FSharp if self.root == F => Spelling::Flats,
        _ => Spelling::Sharps,
    }
}
```

La rama del medio tiene una **guarda de match**, `if self.root == F`: la
rama se toma solo si el patrón encaja y la condición se cumple. Fa
locrio es el único modo sobre esas teclas cuya tónica es una tecla
blanca con nombre propio; escrito con sostenidos sería Mi♯ locrio, así
que toma los bemoles de Sol♭ mayor.

### La armadura, alrededor del círculo de quintas

Una armadura es un número: cuántos sostenidos, o cuántos bemoles como
número negativo. Dando la vuelta al círculo de quintas, la tonalidad
mayor que está `n` quintas por encima de Do tiene `n` sostenidos.
`key_signature` encuentra `n` probando cada paso del círculo hasta caer
en la relativa mayor:

```rust
pub fn key_signature(self) -> Option<(i8, bool)> {
    if !self.kind.is_tonal() || self.kind == ScaleKind::Chromatic {
        return None;
    }
    let major = self.root.transpose(self.kind.relative_major() as i32);
    // Going round the circle of fifths: each fifth up adds a sharp.
    let fifths = (0..12i8)
        .find(|&n| PitchClass::C.transpose(7 * i32::from(n)) == major)
        .unwrap_or(0);
    let sharps = match self.spelling() {
        Spelling::Flats => fifths - 12,
        Spelling::Sharps => fifths,
    };
    let minor = self.kind.relative_major() == 3;
    Some((sharps, minor))
}
```

Doce quintas hacia arriba vuelven a Do, así que una tonalidad con
bemoles es el mismo número de quintas contado al revés: Fa mayor está
once quintas arriba, que es una quinta abajo, así que 11 - 12 = -1, un
bemol. Re♭ mayor está siete quintas arriba y cinco abajo: cinco bemoles.
El segundo valor dice si la tonalidad es menor, algo que también guarda
el evento de armadura de un archivo MIDI ([MIDI: mensajes,
dispositivos, grabación y archivos](../sound/midi.md)). Una serie, un
conjunto o la escala cromática no tienen armadura, de ahí el `Option`.

El rango `0..12i8` lleva su tipo escrito en el número, `12i8`, así que
`n` es un `i8` como el resultado. `find` se detiene en el primer `n` que
sirve; siempre hay uno, y `unwrap_or(0)` solo evita que el compilador
pregunte qué pasa si no.

### Tonalidades que se pueden escribir de dos maneras

Las tonalidades de seis sostenidos y de seis bemoles son las mismas
teclas del piano: Fa♯ mayor y Sol♭ mayor. A su alrededor, Do♯ mayor
(siete sostenidos) es Re♭ mayor (cinco bemoles), y Si mayor (cinco
sostenidos) es Do♭ mayor (siete bemoles). Los compositores eligen el
nombre que va con la música que la rodea; Debussy escribe el centro de
*Golliwogg's Cakewalk* en Sol♭ mayor, una tonalidad de bemoles como el
Mi♭ mayor del que viene. Cualquier otra tonalidad necesitaría ocho
signos o más escrita al revés: Re♯ mayor necesita nueve sostenidos, así
que siempre es Mi♭ mayor.

Así que un `Scale` puede estar **reescrito**: las mismas notas, el otro
nombre. `other_spelling` da esa tonalidad cuando existe:

```rust
pub fn other_spelling(self) -> Option<Scale> {
    use PitchClass::*;
    let key = self.as_key();
    let major = key.root.transpose(key.kind.relative_major() as i32);
    let either_way = key.kind.is_tonal()
        && key.kind != ScaleKind::Chromatic
        && matches!(major, FSharp | CSharp | B);
    either_way.then_some(Scale {
        respelled: !self.respelled,
        ..self
    })
}
```

`Scale { respelled: !self.respelled, ..self }` es la **sintaxis de
actualización de structs**: una copia de `self` con un campo cambiado.
`spelling()` invierte la respuesta habitual cuando la tonalidad está
reescrita, y la tónica toma la letra siguiente: Fa♯ pasa a ser Sol♭, Si
pasa a ser Do♭.

El campo es privado, así que las únicas formas de cambiarlo son los
métodos, que comprueban que la tonalidad se puede escribir al revés.
`Scale::written(tonic, kind)` construye una tonalidad a partir de cómo
se escribe su tónica, que es lo que usan los menús **Tonalidad** y la
línea de la tonalidad de una hoja de acordes escrita como texto: Sol♭
con «mayor» da Fa♯ mayor reescrito, y Sol♭ con «menor», una tonalidad
que necesitaría un doble bemol en su armadura, da el Fa♯ menor de
siempre.

En un archivo, el atributo `#[serde(default, skip_serializing_if =
"std::ops::Not::not")]` deja fuera el campo cuando es `false`, que es
casi siempre, y lee uno que falta como `false`. `skip_serializing_if`
recibe la ruta de una función que toma `&bool` y devuelve `true` para
saltarlo; `std::ops::Not::not` es el operador `!` como función, así que
el campo se salta cuando `!respelled`. Un archivo guardado con una
tonalidad reescrita y abierto por una versión de la aplicación que no
conoce el campo recibe la tonalidad con su nombre habitual, con las
mismas notas.

## Una letra para cada nota

`spelling()` solo dice cómo se escriben las teclas negras de la tónica.
Las notas de una tonalidad necesitan más: Re menor armónica tiene un Si♭
y un Do♯, y Fa♯ mayor tiene un Mi♯, que no es ni el nombre sostenido ni
el bemol de ninguna tecla negra. La regla es la de [Clases de altura,
notas y su escritura](pitch.md#letras-y-nombres): una
escala de siete notas usa cada letra una vez. `spell` le da a una clase
de altura la letra de su lugar en la escala, contada hacia arriba desde
la letra de la tónica, y deja que `NoteName::with_letter` encuentre los
sostenidos o bemoles:

```rust
pub fn spell(self, pitch_class: PitchClass) -> NoteName {
    if let Some(scale) = self.set_as_scale() {
        return scale.spell(pitch_class);
    }
    if !self.kind.is_tonal() {
        return NoteName::chromatic(pitch_class);
    }
    let name = NoteName::with_letter(
        pitch_class,
        self.tonic_name()
            .letter
            .up(i32::from(self.letters_above_tonic(pitch_class))),
    );
    if self.has_letter_for(pitch_class) {
        name
    } else {
        name.simplified()
    }
}
```

Cuántas letras por encima de la tónica va una nota es su lugar en la
escala para una escala de siete notas. Las escalas pentatónicas y la de
blues se saltan letras, así que tienen una tabla propia:

```rust
fn letter_steps(self) -> &'static [u8] {
    match self {
        ScaleKind::MajorPentatonic => &[0, 1, 2, 4, 5],
        ScaleKind::MinorPentatonic => &[0, 2, 3, 4, 6],
        ScaleKind::Blues => &[0, 2, 3, 4, 4, 6],
        ScaleKind::Chromatic | ScaleKind::Row(_) | ScaleKind::Set(_) => &CHROMATIC_LETTERS,
        _ => &[0, 1, 2, 3, 4, 5, 6],
    }
}
```

Las dos entradas de la cuarta letra de la escala de blues son su «blue
note», una quinta bemol junto a la quinta: La blues es La Do Re Mi♭ Mi
Sol, no La Do Re Re♯ Mi Sol.

### Notas de fuera de la tonalidad

Una nota de fuera de la escala no tiene lugar en ella. Se escribe como
más a menudo la encuentran los músicos, contada desde la tónica: una
segunda, tercera, sexta o séptima bemol, una cuarta sostenida, y la
sexta y la séptima mayores, que en una tonalidad menor son las notas
subidas de la menor melódica y la armónica. Eso es una tabla de letras
según los semitonos por encima de la tónica:

```rust
const CHROMATIC_LETTERS: [u8; 12] = [0, 1, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6];
```

Un semitono por encima de la tónica es una letra más arriba (una segunda
bemol), seis semitonos son tres letras (una cuarta sostenida), once son
seis letras (la séptima mayor). Así, en Do mayor las teclas negras de
fuera de la tonalidad son Re♭, Mi♭, Fa♯, La♭ y Si♭, y en Re menor Do♯ es
Do♯.

`letters_above_tonic` elige entre el lugar en la escala y esta tabla:

```rust
fn letters_above_tonic(self, pitch_class: PitchClass) -> u8 {
    let semitones = self.root.semitones_up_to(pitch_class);
    if self.has_letter_for(pitch_class)
        && let Some(place) = self
            .kind
            .tonal_intervals()
            .iter()
            .position(|&interval| interval == semitones)
    {
        return self.kind.letter_steps()[place];
    }
    CHROMATIC_LETTERS[semitones as usize]
}
```

El `if` une una condición y un `let` con `&&`. Es una **cadena de
`let`** (*let chain*): el bloque se ejecuta solo si la condición es
verdadera y el patrón encaja, y `place` queda ligado dentro. Sin ella,
el código necesitaría un `if` dentro de otro `if`, o un `match` sobre
una tupla.

La tabla puede pedir dos signos. En Re♭ mayor la nota un semitono por
encima de la tónica, Re, es una segunda bemol, la letra de Mi: Mi𝄫.
Ningún músico escribe eso para una nota de fuera de la tonalidad, así
que `spell` toma el nombre más sencillo, Re, para cualquier nota a la
que la escala no da letra. Dentro de la escala los dos signos se quedan,
porque la tonalidad los necesita: la séptima nota de Sol♯ menor armónica
es Fa𝄪.

Un conjunto de clases de altura cuyas notas son exactamente una escala
tonal sobre su tónica se escribe como esa escala (`set_as_scale`), así
que Mi mayor mostrado como conjunto conserva su Sol♯, Do♯ y Re♯. Una
serie o cualquier otro conjunto no tiene letras, así que sus notas toman
los nombres cromáticos.

### Nombres con octava

`note_name` junta el nombre escrito con el número de octava correcto, a
través de
[`Note::name_as`](pitch.md#la-octava-de-una-nota-escrita):

```rust
pub fn note_name(self, note: Note) -> String {
    note.name_as(self.spell(note.pitch_class()))
}
```

Do♯ menor armónica sube su séptima, Si, a Si♯, así que en esa tonalidad
la nota MIDI 60, el Do central, es «Si♯3». La comprobación de la
conducción de voces y la vista del movimiento de las partes nombran así
las notas de las que hablan ([Conducción de voces y movimiento de las
partes](voice-leading.md)).

## Cambiar de tonalidad: mover la canción

Los menús **Tonalidad**, a la izquierda de las teclas de armonía y
arriba en el piano roll, fijan la tonalidad de toda la canción, la que
usan las teclas de armonía, las sugerencias y la banda. Así que también
mueven la canción, como una banda toca la misma canción en otra
tonalidad cuando se lo piden: elige Re y los acordes, el bajo y la
melodía suben un tono; elige menor y cada nota pasa al mismo lugar de la
escala menor. La batería se queda como está, y **Ctrl+Z** lo devuelve
todo de una vez.

Como con cualquier [herramienta de edición](tools.md), la
parte musical es una función simple que no cambia nada por sí misma.
Responde una pregunta: ¿adónde va esta nota cuando la tonalidad cambia
de `from` a `to`?

```rust
pub fn change_key(note: Note, from: Scale, to: Scale) -> Note {
    let from = from_key(from, to);
    if !from.kind.notes_follow(to.kind) {
        return note;
    }
    let up = from.root.semitones_up_to(to.root) as i32;
    let shift = if up > 6 { up - 12 } else { up };
    let Some(moved) = note.transpose(shift) else {
        return note;
    };
    // The chromatic scale holds every note, so it has no degrees to
    // match with a key's: into it or out of it, notes only take the new
    // root. (Out of it, every note would otherwise be "of the old scale"
    // and be pulled into the new key, losing every sharp and flat.)
    if from.kind == ScaleKind::Chromatic || to.kind == ScaleKind::Chromatic {
        return moved;
    }
    // The old scale moved to the new root, so `moved` can be compared
    // with it degree by degree.
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

Trabaja en dos movimientos.

1. **Una tónica nueva mueve cada nota la misma distancia.** De Do a Re
   son dos semitonos hacia arriba. De Do a Sol podrían ser siete hacia
   arriba o cinco hacia abajo; `shift` toma el más cercano, así que la
   canción se queda más o menos donde estaba en el teclado en lugar de
   saltar una quinta hacia arriba.
2. **Un modo nuevo lleva cada nota al mismo grado.** Después del primer
   movimiento, la nota se compara con la clase de escala *vieja* sobre
   la tónica *nueva*. Si es la tercera nota de Re mayor (Fa♯), pasa a ser
   la tercera nota de Re menor (Fa). La diferencia entre las dos listas
   de intervalos en el mismo lugar es cuánto se mueve ese grado:

   | grado         | 1 | 2 | 3  | 4 | 5 | 6  | 7  |
   |---------------|---|---|----|---|---|----|----|
   | mayor         | 0 | 2 | 4  | 5 | 7 | 9  | 11 |
   | menor natural | 0 | 2 | 3  | 5 | 7 | 8  | 10 |
   | se mueve      | 0 | 0 | -1 | 0 | 0 | -1 | -1 |

   Así, de Do mayor a Do menor bajan Mi, La y Si, y quedan Do, Re, Fa y
   Sol.

Algunas notas necesitan una regla propia:

- Una **nota de fuera de la escala vieja**, como una nota de un acorde
  prestado o de una parte escrita en otra tonalidad, no tiene grado, así
  que el `let ... else` la devuelve solo con el primer movimiento.
  Conserva su distancia a la tónica, y una modulación sigue siendo una
  modulación.
- **Escalas de tamaños distintos**, como la mayor (siete notas) y la
  pentatónica mayor (cinco), no tienen grados que se correspondan. Una
  nota que no está en la escala nueva va a la más cercana que sí está,
  con `nearest_in_key` ([Herramientas de edición como funciones
  simples](tools.md)).
- La **escala cromática** tiene todas las notas, así que entrar en ella
  o salir de ella solo mueve la tónica.
- Entre una tonalidad y una **serie** o un **conjunto** no hay con qué
  mover, y `notes_follow` lo dice: solo cambia lo que ilumina el piano
  roll. Un conjunto cuyas notas son exactamente una tonalidad, como Do
  mayor mostrado como el conjunto 7-35, sigue moviéndose como esa
  tonalidad (`from_key`).

`let Some(moved) = ... else { return note; };` es un **let-else**: si el
patrón no encaja, el bloque `else` tiene que salir de la función. Aquí
cubre «la nota se saldría del rango MIDI» y «la nota no está en la
escala».

### El proyecto entero

`Project::change_key` pasa esa función por cada nota que debe moverse.
Su bucle central:

```rust
let pitched: Vec<bool> = self.channels.iter().map(Channel::is_pitched).collect();
for (index, pattern) in self.patterns.iter_mut().enumerate() {
    let (old, new) = (before[index], after[index]);
    for (channel, _) in pitched.iter().enumerate().filter(|(_, p)| **p) {
        if pattern.is_step_row(channel) {
            continue;
        }
        let notes: Vec<NoteEvent> = pattern
            .notes(channel)
            .iter()
            .map(|note| NoteEvent {
                pitch: tools::change_key(note.pitch, old, new),
                ..*note
            })
            .collect();
        pattern.set_notes(channel, notes);
    }
}
```

- `pitched` se recoge antes del bucle porque el bucle presta
  `self.patterns` de forma mutable, y Rust no deja que el código mire
  `self.channels` a través de `self` al mismo tiempo. Un `Vec<bool>`
  pequeño leído antes termina ese préstamo antes.
- `Channel::is_pitched` es falso para la batería incorporada y para un
  canal MIDI Out en el canal de batería de General MIDI: la batería no
  se mueve.
- Una fila de pasos del secuenciador (cada nota un Do4 de un paso) es un
  sonido tocado a tiempo, no una melodía, así que se queda como está.
- `before[index]` y `after[index]` son las tonalidades en que se toca
  cada patrón antes y después del cambio, que no siempre son la
  tonalidad de la canción, como muestra la sección siguiente.

La aplicación la llama a través de `edit`, que guarda el proyecto como
estaba antes ([Deshacer y rehacer](../program/undo.md)), así que un
solo **Ctrl+Z** devuelve la tonalidad y todas las notas:

```rust
self.edit("change key", |project| project.change_key(scale));
```

La hoja de acordes y las partes de la banda se mueven con las notas;
están en [La hoja de acordes: modelo, distribución y
texto](../band/chord-sheet.md) y [Cómo toca la banda una hoja de
acordes](../band/band.md).

## Cambios de tonalidad a mitad de camino: marcas de tonalidad

Muchas piezas no se quedan en una tonalidad. El Rondó alla turca de
Mozart empieza en La menor, pasa a La mayor en el compás 25, donde la
armadura gana tres sostenidos, y vuelve a La menor en el compás 65.
Pasar a una tonalidad nueva es una **modulación**. Tres sostenidos son
también la armadura de Fa♯ menor, la relativa menor de La mayor, y la
parte en La mayor pasa por Fa♯ menor sin cambiar de armadura; en qué
tonalidad está la música se oye por dónde vuelve a casa, no por la
armadura.

Un cambio de tonalidad es una marca más en la lista de reproducción,
junto a las marcas de compás y de tempo:

```rust
pub struct KeyMarker {
    /// The step it starts on, counted from the start of the song.
    pub start: usize,
    pub scale: Scale,
}
```

Una marca de tonalidad nunca mueve una nota. Cambia cómo se llama la
tonalidad, qué filas ilumina el piano roll, qué acordes ofrecen las
teclas de armonía y qué acordes cuentan como «de la tonalidad». Esa es
la diferencia con los menús Tonalidad, que mueven la canción.

`KeyMap` responde «¿en qué tonalidad está el paso 300?»: la tonalidad
propia de la canción hasta la primera marca, y luego la de cada marca
por turno. Al construirlo, ordena las marcas y se queda con una por
paso:

```rust
pub fn new(first: Scale, markers: impl IntoIterator<Item = KeyMarker>) -> Self {
    let mut markers: Vec<KeyMarker> = markers
        .into_iter()
        .filter(|marker| marker.start > 0)
        .collect();
    markers.sort_by_key(|marker| marker.start);
    markers.dedup_by_key(|marker| marker.start);
    KeyMap { first, markers }
}
```

`dedup_by_key` quita un elemento cuando tiene la misma clave que el
anterior, así que después de ordenar se queda con la primera marca de
cada paso. Buscar un paso toma la última marca que empieza en él o
antes:

```rust
pub fn marker_at(&self, step: usize) -> Option<&KeyMarker> {
    self.markers
        .iter()
        .take_while(|marker| marker.start <= step)
        .last()
}
```

`take_while` se detiene en la primera marca posterior a `step`, y `last`
da la anterior, o `None` si el paso está antes de todas las marcas. Un
`Project` construye su mapa con `key_map()`, y `key_at(step)` le
pregunta.

Añadir una marca mantiene la lista ordenada con
`binary_search_by_key`, que da `Ok(index)` si ya hay una marca que
empieza en ese paso y `Err(index)`, donde iría, si no:

```rust
let markers = &mut self.playlist.key_markers;
match markers.binary_search_by_key(&marker.start, |m| m.start) {
    Ok(index) => markers[index] = marker,
    Err(index) => markers.insert(index, marker),
}
```

Una marca en el paso 0 no se añade: cambia la tonalidad con la que
empieza la canción.

### La tonalidad de un patrón

Un patrón se escribe una vez y se toca dondequiera que estén sus clips,
y una canción puede tocarlo en tonalidades distintas. El piano roll
necesita una tonalidad para iluminar sus notas, así que un patrón toma
la tonalidad donde empieza su primer clip:

```rust
pub fn pattern_key_at(&self, pattern: usize, step: usize) -> Scale {
    self.playlist
        .clips
        .iter()
        .filter(|clip| clip.pattern == pattern)
        .map(|clip| clip.start)
        .min()
        .map_or(self.scale, |start| self.key_at(start + step))
}
```

`pattern_key(p)` es esto en el paso 0. El piano roll dice *Este patrón:
La mayor* junto a los menús Tonalidad, y las teclas de armonía dicen
*Ahora en La mayor* cuando el cursor de reproducción está en una
tonalidad que no es la propia de la canción.

### Cuando la canción se mueve, las marcas se mueven

Cuando los menús Tonalidad mueven una canción con cambios de tonalidad,
cada cambio tiene que moverse el mismo intervalo, o la marcha del Rondó
se quedaría en La mayor mientras el resto iba a Do menor. `moved_key`
mueve una tonalidad como se movió la de la canción:

```rust
pub fn moved_key(key: Scale, from: Scale, to: Scale) -> Scale {
    let from = crate::tools::from_key(from, to);
    if !from.kind.notes_follow(to.kind) {
        return key;
    }
    let up = from.root.semitones_up_to(to.root);
    let root = key.root.transpose(up as i32);
    let kind = if key.kind == from.kind && from.kind.is_tonal() && to.kind.is_tonal() {
        to.kind
    } else {
        key.kind
    };
    if root == key.root && kind == key.kind {
        return key;
    }
    Scale::new(root, kind)
}
```

La tónica se mueve los mismos semitonos. Una tonalidad en el modo viejo
de la canción toma el modo nuevo: una canción en Do mayor con un pasaje
en Sol mayor, llevada a Do dórico, tiene ese pasaje en Sol dórico. Una
tonalidad en otro modo conserva el suyo: en una canción en La menor
subida una tercera menor a Do menor, la marcha en La mayor pasa a Do
mayor y el episodio en Fa♯ menor a La menor, como se leería una
partitura transportada. Después `change_key` mueve cada patrón de la
tonalidad en que se tocaba a la tonalidad en que se toca ahora, que es
lo que guardaban `before` y `after` más arriba.

Los archivos MIDI también guardan los cambios de tonalidad, como eventos
de armadura; ver [MIDI: mensajes, dispositivos, grabación y
archivos](../sound/midi.md). Una marca también la puede escribir la
propia aplicación, a partir de las tonalidades que oye en las notas
([Oír la armonía: acordes y tonalidades a partir de las
notas](hearing.md)).

## Ajustar a la escala

El botón **Ajustar a la escala** del piano roll hace que una nota que
dibujas o arrastras caiga en la nota más cercana de la tonalidad, y su
menú de clic derecho añade **Ajustar las notas que tocas**, para el
teclado del equipo y un teclado MIDI. Los dos se reducen a
`tools::nearest_in_key(note, scale)`, en [Herramientas de edición como
funciones simples](tools.md); la tonalidad a la que se ajusta
es la de ese punto de la canción, después de las marcas de tonalidad.
Cómo se ajustan las teclas de un teclado MIDI en el hilo de MIDI está
en [MIDI: mensajes, dispositivos, grabación y
archivos](../sound/midi.md#ajustar-las-teclas-a-la-tonalidad).

## Cómo se prueba

Los tests de `scale.rs` enuncian las reglas para todas las tonalidades a
la vez, no solo para las que se le ocurrieron a alguien. Cada escala de
siete notas, sobre cada tónica, escrita de cualquiera de las dos
maneras, usa siete letras distintas, y cada nombre sigue nombrando la
tecla correcta del piano:

```rust
let mut letters: Vec<_> = scale
    .pitch_classes()
    .map(|p| scale.spell(p).letter)
    .collect();
letters.sort();
letters.dedup();
assert_eq!(letters.len(), 7, "{}", scale.name());
```

Los signos con que se escriben las notas de una tonalidad suman su
armadura, para cada modo sobre cada tónica, así que un pentagrama nunca
necesita una alteración para una nota de la tonalidad:

```rust
let (signature, _) = scale.key_signature().unwrap();
let written: i8 = scale
    .pitch_classes()
    .map(|note| scale.spell(note).accidental)
    .sum();
assert_eq!(written, signature, "{}", scale.name());
```

Y las armaduras de las doce tonalidades mayores, desde Do hacia arriba,
son el círculo de quintas:

```rust
// Sharps count up, flats count down, round the circle of fifths:
// C, Db, D, Eb, E, F, F#, G, Ab, A, Bb, B.
let major = [0, -5, 2, -3, 4, -1, 6, 1, -4, 3, -2, 5];
```

`tools.rs` comprueba los movimientos de un cambio de tonalidad: de Do a
Sol baja una cuarta, de Do mayor a Do menor bajan Mi, La y Si, una nota
de fuera de la escala solo toma la tónica nueva, y un D7 llevado a la
escala cromática y de vuelta sigue siendo un D7. `key_map.rs` comprueba
que la marcha y el episodio del Rondó caen en Do mayor y La menor cuando
la canción sube una tercera menor.

Los ejercicios sobre tonalidades están en [Ejercicios: la teoría musical
en código](exercises.md).
