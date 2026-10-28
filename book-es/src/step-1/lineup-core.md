# lineup-core: notas y tempo

`lineup-core` es el cerebro musical del proyecto. En el paso 1 contiene
dos tipos pequeños, `Note` y `Tempo`. Son simples, lo que los convierte
en un buen lugar para conocer los bloques básicos de Rust: structs,
bloques `impl`, constantes asociadas, traits y pruebas.

## `Note`: un newtype alrededor de un número

Archivo:
[`crates/lineup-core/src/pitch.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/pitch.rs)

### La música

El software musical suele nombrar las notas con **números de nota MIDI**,
de 0 a 127. El do central es 60, y cada paso de 1 es un **semitono** (una
tecla del piano, blanca o negra). Doce semitonos forman una **octava**, y
la nota una octava más aguda tiene exactamente el doble de frecuencia. El
La4, número MIDI 69, es la nota de afinación a 440 Hz. Todo junto da la
fórmula del *temperamento igual*:

```text
frequency = 440 × 2^((midi − 69) / 12)
```

### El código

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Note(u8);
```

Es una **tuple struct** con un campo, un `u8` (un entero de 8 bits sin
signo, de 0 a 255). Envolver un número simple en su propio tipo se llama
**patrón newtype**. No cuesta nada en tiempo de ejecución, ya que un
`Note` es solo un `u8` en memoria, pero ahora el compilador sabe que un
`Note` no es una velocity ni un número de canal, y se negará a
mezclarlos.

El campo no tiene `pub`, así que es privado: otros crates no pueden
escribir `Note(200)`. Deben usar `Note::new`, que limita el valor a 127.
Esta es una idea común en Rust: **haz imposible construir valores
inválidos**.

`#[derive(...)]` pide al compilador que escriba algunas implementaciones
de traits por nosotros automáticamente:

| Trait | Te da |
|-------|-------|
| `Debug` | impresión con `{:?}`, para depurar |
| `Clone`, `Copy` | el valor se copia al asignarse, como un número |
| `PartialEq`, `Eq` | `==` y `!=` |
| `PartialOrd`, `Ord` | `<`, `>`, ordenar |
| `Hash` | se puede usar como clave en un `HashMap` |

### Bloques `impl` y constantes asociadas

Los métodos se definen en un bloque `impl`, separados de los datos:

```rust
impl Note {
    pub const C4: Note = Note(60);
    pub const A4: Note = Note(69);

    pub fn new(midi: u8) -> Self { Note(midi.min(Self::MAX.0)) }
    pub fn midi(self) -> u8 { self.0 }
    pub fn frequency(self) -> f32 { ... }
}
```

- `pub const C4` es una **constante asociada**, se usa como `Note::C4`.
- `Self` es una abreviatura de «el tipo de este `impl`», aquí `Note`.
- `self.0` lee el primer (y único) campo de la tuple struct.
- Una función cuyo primer parámetro es `self` es un **método**, se llama
  como `note.frequency()`. Una función sin `self`, como `new`, es una
  **función asociada**, se llama como `Note::new(60)`. Rust no tiene
  constructores especiales; `new` es solo una convención de nombres.
- La última expresión de una función, escrita sin `;`, es su valor de
  retorno. No hace falta la palabra clave `return`.

Los métodos toman `self` **por valor** en lugar de `&self` (por
referencia). Eso está bien porque `Note` es `Copy`: pasarlo copia un
byte.

### Implementar un trait: `Display`

Un **trait** es un conjunto de métodos que un tipo puede prometer
proporcionar, parecido a una interfaz en otros lenguajes. El trait
`fmt::Display` de la librería estándar controla cómo se imprime un valor
con `{}`:

```rust
impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = NOTE_NAMES[self.pitch_class() as usize];
        write!(f, "{}{}", name, self.octave())
    }
}
```

Una vez implementado `Display`, `Note::new(61).to_string()` da `"C#4"` y
`format!("{note}")` también funciona. La interfaz usa esto para etiquetar
el deslizador del tono de prueba.

`as usize` e `as i8` son **casts**. Rust nunca convierte entre tipos
numéricos en silencio, así que tienes que decirlo. Los índices de array
deben ser `usize`, el tipo que Rust usa para tamaños y posiciones en
memoria.

## `Tempo`: mantener válido un valor

Archivo:
[`crates/lineup-core/src/tempo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/tempo.rs)

```rust
pub struct Tempo { bpm: f32 }

impl Tempo {
    pub const MIN: f32 = 10.0;
    pub const MAX: f32 = 522.0;

    pub fn new(bpm: f32) -> Self {
        if bpm.is_nan() { return Self::default(); }
        Tempo { bpm: bpm.clamp(Self::MIN, Self::MAX) }
    }
}
```

Misma idea que `Note`: el campo `bpm` es privado, así que la única forma
de obtener un `Tempo` es mediante `new`, que lo limita al rango. El rango
de 10 a 522 BPM coincide con el del estudio original.

`f32` es un número de coma flotante de 32 bits. Los floats tienen un
valor especial «not a number» (NaN), producido por cosas como
`0.0 / 0.0`. NaN se compara como falso con todo, así que `clamp` no puede
arreglarlo, y en su lugar lo sustituimos por el tempo por defecto.

### El trait `Default`

```rust
impl Default for Tempo {
    fn default() -> Self { Tempo { bpm: 130.0 } }
}
```

`Default` es el trait estándar para «un valor inicial sensato». Muchas
funciones de librerías y `#[derive(Default)]` dependen de él.

## Pruebas unitarias

Ambos archivos terminan con un módulo de pruebas:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a4_is_440_hz() {
        assert_eq!(Note::A4.frequency(), 440.0);
    }
}
```

- `#[cfg(test)]` significa «compila esto solo al ejecutar las pruebas»,
  de modo que las pruebas no añaden nada al programa real.
- `use super::*;` importa todo del módulo padre, incluidos los elementos
  privados. Por eso las pruebas unitarias pueden probar también funciones
  privadas.
- Cada función `#[test]` es una prueba. Pasa si termina con normalidad y
  falla si entra en **pánico**, que es lo que hacen `assert!` y
  `assert_eq!` cuando su condición es falsa.

Comparar floats con `==` es arriesgado por el redondeo, así que la
mayoría de las pruebas comprueban en su lugar que la diferencia es
diminuta:

```rust
assert!((freq - 261.626).abs() < 0.01, "got {freq}");
```

La cadena que sigue a la condición se imprime si la prueba falla.
