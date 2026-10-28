# La teoría musical como tipos: clases de alturas y escalas

Archivos: [`crates/lineup-core/src/pitch.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/pitch.rs),
[`crates/lineup-core/src/harmony/pitch_set.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/pitch_set.rs)
y [`crates/lineup-core/src/harmony/scale.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/scale.rs)

## La música

Un piano tiene 88 teclas, pero solo **doce nombres de nota distintos** que
se repiten cada octava: Do, Do♯, Re, Re♯, Mi, Fa, Fa♯, Sol, Sol♯, La, La♯, Si, y luego
otra vez Do, una octava más aguda. El nombre de una nota sin su octava es
su **clase de alturas**. La armonía trabaja sobre todo con clases de
alturas: un acorde de Do mayor es «Do, Mi y Sol», en cualquier octava y en
cualquier orden.

La distancia entre dos teclas vecinas (blancas o negras) es un
**semitono**. Todo en este capítulo es contar semitonos:

```text
 C  C# D  D# E  F  F# G  G# A  A# B  C
 0  1  2  3  4  5  6  7  8  9  10 11 12
```

Una **escala** es una elección de algunas de las doce, empezando por una
**fundamental**. La escala mayor son las teclas blancas desde Do: Do Re Mi Fa
Sol La Si. Como semitonos sobre la fundamental, eso es `0, 2, 4, 5, 7, 9,
11`. El mismo patrón empezado en otra nota da la escala mayor de esa
tonalidad: desde Re, es Re Mi Fa♯ Sol La Si Do♯.

Las teclas negras tienen dos nombres. La tecla entre La y Si es La♯
(«La sostenido», un semitono por encima de La) y también Si♭ («Si bemol», un
semitono por debajo de Si). Qué nombre es el correcto depende de la
tonalidad: las notas de Fa mayor son Fa Sol La **Si♭** Do Re Mi, porque cada
nombre de nota se usa una vez. Ferrous Lineup sigue esa regla de una forma
sencilla: las tonalidades cuya armadura tiene bemoles (Fa, Si♭, Mi♭, La♭, Re♭
y sus relativas menores) se escriben con bemoles, y todas las demás con
sostenidos.

## Un enum para las doce clases de alturas

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PitchClass {
    C, CSharp, D, DSharp, E, F, FSharp, G, GSharp, A, ASharp, B,
}
```

Hay exactamente doce clases de alturas, así que un enum encaja
perfectamente: un `PitchClass` nunca puede ser «13» ni «-1». Las
variantes no tienen datos (un enum *sin campos*), y Rust las numera desde
0 en el orden en que están escritas. La palabra clave `as` convierte un
enum sin campos en ese número:

```rust
pub fn semitone(self) -> u8 {
    self as u8 // C is 0, CSharp is 1, ..., B is 11
}
```

Ir en la dirección contraria requiere cuidado, porque no todos los
números son una clase de alturas. `from_semitone` primero envuelve el
valor y luego busca la respuesta en el array `ALL`, así que nunca puede
fallar:

```rust
pub const ALL: [PitchClass; 12] = [PitchClass::C, PitchClass::CSharp, /* ... */ PitchClass::B];

pub fn from_semitone(semitone: u8) -> Self {
    Self::ALL[(semitone % 12) as usize]
}
```

`transpose` suma semitonos y da la vuelta alrededor de la octava. Recibe
un `i32` para poder ir también hacia abajo. Para números negativos, `%`
daría un resto negativo (`-1 % 12` es `-1` en Rust), así que usa
`rem_euclid`, que siempre da `0..12`:

```rust
pub fn transpose(self, semitones: i32) -> Self {
    let wrapped = (self.semitone() as i32 + semitones).rem_euclid(12);
    Self::from_semitone(wrapped as u8)
}
```

## Hacer match con dos valores a la vez

El nombre de una clase de alturas depende de dos cosas: de cuál es y de
si la tonalidad usa sostenidos o bemoles. `match` puede mirar las dos a
la vez si las pones en una **tupla**:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Spelling {
    #[default]
    Sharps,
    Flats,
}

pub fn name(self, spelling: Spelling) -> &'static str {
    use PitchClass::*;
    use Spelling::*;
    match (self, spelling) {
        (C, _) => "C",
        (CSharp, Sharps) => "C#",
        (CSharp, Flats) => "Db",
        (D, _) => "D",
        // ...
        (B, _) => "B",
    }
}
```

Cosas que observar:

- `use PitchClass::*;` dentro de la función trae las variantes al
  alcance, así que puedes escribir `C` en lugar de `PitchClass::C`.
  Hacerlo dentro de la función evita que esos nombres cortos se filtren a
  todo el archivo.
- `_` coincide con cualquier cosa: las teclas blancas tienen el mismo
  nombre en ambos casos.
- El compilador comprueba que los brazos cubran las 24 combinaciones.
  Olvida `(GSharp, Flats)` y se niega a compilar, nombrando el caso que
  falta.
- `#[derive(Default)]` en un enum necesita `#[default]` en una variante,
  la que devuelve `Spelling::default()`.
- `&'static str` es un string prestado que vive durante todo el programa:
  los literales de string se guardan en el propio programa.

`is_black_key` usa la macro `matches!`, una forma corta de escribir
«¿este valor coincide con este patrón?» como un `bool`. El `|` significa
«o»:

```rust
pub fn is_black_key(self) -> bool {
    use PitchClass::*;
    matches!(self, CSharp | DSharp | FSharp | GSharp | ASharp)
}
```

`Note` (el número de nota MIDI del paso 1) ahora devuelve un
`PitchClass` desde `pitch_class()`, y ganó `transpose` (que devuelve
`None` si saldrías del rango MIDI) y `name(spelling)`, que da nombres
como «Bb3».

## Un conjunto de clases de alturas en doce bits

Los acordes y las escalas son *conjuntos* de clases de alturas. Un
conjunto de como mucho doce cosas cabe en los bits de un entero: el bit 0
significa «Do está en el conjunto», el bit 1 significa «Do♯ está en el
conjunto», y así sucesivamente.

```rust
pub struct PitchSet(u16);

pub fn insert(&mut self, pitch_class: PitchClass) {
    self.0 |= 1 << pitch_class.semitone();
}

pub fn contains(self, pitch_class: PitchClass) -> bool {
    self.0 & (1 << pitch_class.semitone()) != 0
}
```

`1 << n` es el número con solo el bit `n` activado (desplaza 1 a la
izquierda `n` posiciones). `|=` activa ese bit, y `&` conserva solo los
bits que comparten dos números, así que `x & (1 << n) != 0` pregunta
«¿está activado el bit `n`?». Do mayor (Do, Mi, Sol) son los bits 0, 4 y 7:

```text
bit:       11 10  9  8  7  6  5  4  3  2  1  0
           B  A# A  G# G  F# F  E  D# D  C# C
Do mayor:  0  0  0  0  1  0  0  1  0  0  0  1   = 0b0000_1001_0001
```

Otras operaciones de conjuntos son una instrucción cada una:
`count_ones()` cuenta los miembros, y `a & !b == 0` pregunta «¿está cada
miembro de `a` también en `b`?» (`!` invierte todos los bits).

### Ver un acorde desde su fundamental

El truco clave para nombrar acordes es **transponer un conjunto**: mover
cada clase de alturas arriba o abajo la misma cantidad. En bits, eso es
una *rotación* de los doce bits, donde los bits que se caen por un
extremo vuelven a entrar por el otro:

```rust
pub fn transpose(self, semitones: u8) -> PitchSet {
    let shift = (semitones % 12) as u32;
    let bits = self.0 as u32;
    let rotated = (bits << shift) | (bits >> (12 - shift));
    PitchSet(rotated as u16 & ALL_BITS) // keep only the 12 low bits
}

pub fn relative_to(self, root: PitchClass) -> PitchSet {
    self.transpose(12 - root.semitone())
}
```

`relative_to(root)` mueve el conjunto hacia abajo hasta que `root` se
convierte en Do. Mi, Sol, Si visto desde Mi se convierte en Do, Re♯, Sol, que es
`0, 3, 7`: exactamente la forma de la tríada menor. Comparar un conjunto
visto desde su fundamental con cada forma de acorde es como el próximo
capítulo nombra los acordes.

### Hacer que `collect` construya un `PitchSet`

`collect()` puede construir cualquier tipo que implemente el trait
`FromIterator`. `Vec`, `String` y `HashMap` lo hacen. Implementarlo para
`PitchSet` permite que el resto del código escriba
`notes.map(|note| note.pitch_class()).collect()` y obtenga un `PitchSet`:

```rust
impl FromIterator<PitchClass> for PitchSet {
    fn from_iter<I: IntoIterator<Item = PitchClass>>(iter: I) -> Self {
        let mut set = PitchSet::EMPTY;
        for pitch_class in iter {
            set.insert(pitch_class);
        }
        set
    }
}
```

## Escalas

```rust
pub enum ScaleKind {
    Major, NaturalMinor, HarmonicMinor, MelodicMinor,
    Dorian, Phrygian, Lydian, Mixolydian, Locrian,
    MajorPentatonic, MinorPentatonic, Blues, Chromatic,
}

pub fn intervals(self) -> &'static [u8] {
    match self {
        ScaleKind::Major => &[0, 2, 4, 5, 7, 9, 11],
        ScaleKind::NaturalMinor => &[0, 2, 3, 5, 7, 8, 10],
        ScaleKind::MajorPentatonic => &[0, 2, 4, 7, 9],
        // ...
    }
}
```

Las escalas tienen cantidades distintas de notas (cinco para la
pentatónica, siete para la mayor, doce para la cromática), así que
`intervals` devuelve un **slice**, `&[u8]`, que puede tener cualquier
longitud. `&[0, 2, 4, 5, 7, 9, 11]` es un array prestado que el
compilador guarda en el programa, así que es `'static`: sin reservar
memoria, jamás.

Los modos (dórico, frigio y demás) son la escala mayor empezada en otra
de sus notas: Re dórico es Re Mi Fa Sol La Si Do, las teclas blancas desde Re. Una
prueba comprueba exactamente eso.

Un `Scale` es una fundamental más un tipo de escala, y la mayoría de sus
métodos son cadenas cortas de iteradores:

```rust
pub struct Scale {
    pub root: PitchClass,
    pub kind: ScaleKind,
}

pub fn pitch_classes(self) -> impl Iterator<Item = PitchClass> {
    self.kind
        .intervals()
        .iter()
        .map(move |&interval| self.root.transpose(interval as i32))
}

pub fn degree_of(self, pitch_class: PitchClass) -> Option<usize> {
    self.pitch_classes().position(|p| p == pitch_class)
}
```

- `impl Iterator<Item = PitchClass>` en el tipo de retorno significa
  «algún iterador de clases de alturas; quien llama no necesita saber
  cuál». El tipo real es un nombre largo que involucra a
  `Map<Iter<u8>, {closure}>`, y los closures tienen tipos que ni siquiera
  puedes escribir. `impl Trait` lo oculta.
- `move` hace que el closure se lleve su propia copia de `self` (un
  `Scale` es `Copy`). Sin él, el closure tomaría prestado `self`, una
  variable local de `pitch_classes`, y no podría devolverse desde la
  función.
- `|&interval|` es un patrón en el parámetro del closure: el iterador da
  `&u8`, y `&interval` saca el `u8` de la referencia.
- `position` devuelve el índice del primer elemento que coincide, como un
  `Option`. El **grado** de una nota es su posición en la escala, así que
  en La menor, Mi es el grado 4 (contando desde 0; los músicos dicen «la
  quinta»).

### ¿Qué ortografía usa una tonalidad?

```rust
pub fn spelling(self) -> Spelling {
    use PitchClass::*;
    match self.root.transpose(self.kind.relative_major() as i32) {
        F | ASharp | DSharp | GSharp | CSharp => Spelling::Flats,
        _ => Spelling::Sharps,
    }
}
```

Toda tonalidad menor y todo modo comparten su armadura de clave con una
tonalidad mayor: La menor usa las mismas notas que Do mayor, y Re dórico
también. `relative_major` dice cuánto más arriba está esa tonalidad
mayor (3 semitonos para la menor, 10 para el dórico), y el `match`
decide según esa tonalidad mayor. Los patrones or (`F | ASharp`)
funcionan en los brazos de `match` igual que en `matches!`.

### Los acordes de una tonalidad

Para construir el acorde sobre un grado, toma una de cada dos notas de la
escala: grado, grado + 2, grado + 4 (y + 6 para un acorde de séptima).
Esto se llama **apilar terceras**. En Do mayor, empezar en Re da Re, Fa, La:
Re menor.

```rust
pub fn stack_thirds(self, note: Note, count: usize) -> Option<Vec<Note>> {
    if !self.is_heptatonic() {
        return None;
    }
    let intervals = self.kind.intervals();
    let degree = self.degree_of(note.pitch_class())?;
    let notes = (0..count)
        .map(|i| {
            let index = degree + 2 * i;
            // Going past the seventh note wraps into the next octave.
            let octave = (index / 7) as i32;
            let above_root = intervals[index % 7] as i32 + 12 * octave;
            above_root - intervals[degree] as i32
        })
        .filter_map(|semitones| note.transpose(semitones))
        .collect();
    Some(notes)
}
```

Léelo como una tubería: los números `0..count` se convierten en
posiciones de la escala, las posiciones en semitonos por encima de
`note`, y `filter_map` convierte los semitonos en notas, descartando las
que se caerían por arriba del rango MIDI (`transpose` devuelve `None`
para esas). El `?` después de `degree_of` devuelve `None` pronto si la
nota no está en la escala.

Hacer esto para los siete grados de Do mayor da los acordes que todo
compositor aprende:

| Grado | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
|-------|---|---|---|---|---|---|---|
| Tríada | C | Dm | Em | F | G | Am | Bdim |
| Numeral | I | ii | iii | IV | V | vi | vii° |
| Séptima | Cmaj7 | Dm7 | Em7 | Fmaj7 | G7 | Am7 | Bm7b5 |

Los botones de «Acordes en Do mayor» del piano roll son esta tabla.
Cambia la tonalidad y los botones la siguen, porque se calculan, no se
escriben.
