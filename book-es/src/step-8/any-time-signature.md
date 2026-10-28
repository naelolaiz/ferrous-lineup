# Cualquier compás, y métricas irregulares

Archivos: [`crates/lineup-core/src/meter.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/meter.rs),
[`crates/lineup-app/src/time_picker.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/time_picker.rs),
[`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs)

Los menús solían ofrecer diez compases, de 2/4 a 12/8. La música
tiene muchos más: 11/8, 13/16, 7/4, 5/2. Un compás no es más que dos
números, así que ahora puedes escribir cualquiera, y dar a los pulsos
de un compás irregular su agrupación.

![La lista Time de las harmony keys: los compases habituales, luego "Any other" con 7 sobre 8, pulsos agrupados como 2+2+3, y el botón "Use 2+2+3/8".](../images/step-8-time-picker.png)

## La música: los compases irregulares

En 6/8 las seis corcheas se sienten siempre como dos pulsos de tres:
*UNO* dos tres *CUATRO* cinco seis. En 7/8 no hay tal regla. Siete
no puede repartirse en pulsos iguales, así que se siente como pulsos
de dos y de tres, y cuál va primero cambia la música por completo:

| Escrito | Se cuenta | Pulsos (en pasos) | Dónde se oye |
|---------|-----------|-------------------|--------------|
| 2+2+3/8 | *un* dos *un* dos *un* dos tres | 4, 4, 6 | la *rachenitsa* búlgara |
| 3+2+2/8 | *un* dos tres *un* dos *un* dos | 6, 4, 4 | el *kalamatianos* griego, el *lesnoto* macedonio |
| 3+3+2/8 | *un* dos tres *un* dos tres *un* dos | 6, 6, 4 | el *tresillo* de la música latina, los tangos de Piazzolla |
| 2+2+2+3/8 | cuatro pulsos, el último largo | 4, 4, 4, 6 | el *karşılama* turco, *Blue Rondo à la Turk* de Brubeck |
| 3+3+3+2/16 | tres pulsos largos y uno corto | 3, 3, 3, 2 | una forma de contar un 11 rápido |

Estos se llaman compases *irregulares*, *aditivos* o *aksak* ("aksak"
es turco y significa "cojo": el pulso largo se siente como una cojera).
Observa que 3+3+2/8 tiene ocho corcheas, como 4/4, pero no se cuenta
como 4/4: los pulsos caen en las notas 1, 4 y 7. Escribirlo 3+3+2 lo
dice.

Una partitura escribe la agrupación en el propio compás, como número
de arriba: (2+2+3)/8. Ferrous Lineup hace lo mismo, y lo muestra en la
regla del playlist:

![Los números de compás del playlist: el compás 5 empieza un cambio a 3+3+3+2/16, once pasos por compás.](../images/step-8-irregular-ruler.png)

## Cómo usarlo

- **Harmony keys > Time**, o **clic derecho en un número de compás** del
  playlist para un cambio desde ese compás en adelante.
- Haz clic en uno de los compases habituales, o bajo **Any other** pon
  el número de arriba, haz clic en el de abajo (1, 2, 4, 8 o 16) y,
  si quieres, escribe cómo se agrupan los pulsos. **Use …** lo aplica.
- Si algo no encaja, el menú lo dice en rojo, por ejemplo "los grupos
  suman 8, no 11".

No hay un 32 abajo: la cuadrícula de Ferrous Lineup son pasos de
semicorchea, y un compás de 7/32 terminaría en mitad de un paso.

## Una agrupación dentro de un tipo `Copy`

`TimeSignature` se pasa por todas partes por valor, porque es `Copy`:
dos números pequeños, copiados como un entero. Una agrupación como
`[2, 2, 3]` sería naturalmente un `Vec<u8>`, pero un `Vec` posee memoria
en el montículo, y un tipo que contiene uno no puede ser `Copy`. Así
que la agrupación se guarda como una *máscara de bits*:

```rust
pub struct TimeSignature {
    pub beats: u8,
    pub unit: u8,
    /// bit `i` is set when a beat starts on the bar's `i`-th note.
    /// 0 means the usual beats.
    groups: u64,
}
```

2+2+3/8 tiene pulsos que empiezan en las notas 0, 2 y 4, así que su
máscara es `0b10101`: los bits 0, 2 y 4. Un `u64` tiene sitio para 64
notas, más que cualquier compás real. Para construirla:

```rust
let mut mask = 0_u64;
let mut at = 0;
for &group in groups {      // [2, 2, 3]
    mask |= 1 << at;        // a beat starts here
    at += usize::from(group);
}
```

`1 << at` es un 1 movido `at` posiciones a la izquierda, y `|=` («o-igual»)
enciende ese bit sin tocar los demás.

El campo es privado (no hay `pub`), así que el código fuera de
`meter.rs` solo puede crear una agrupación mediante
`TimeSignature::grouped`, que comprueba que los grupos no estén vacíos
y que quepan. Así es como Rust mantiene válido un valor: lo que podría
romper las reglas queda oculto, y las funciones públicas las mantienen.

Una regla más: si la agrupación es la habitual, la máscara se queda en
0. Así 3+3/8 y 6/8 son el mismo valor y resultan iguales con `==`. Si
no, el menú pensaría que "6/8" y "3+3/8" son dos compases distintos.

## Pulsos que no miden todos lo mismo

Antes, un compás tenía un único `steps_per_beat`, y el código encontraba
el pulso de un paso dividiendo. Con 2+2+3/8 eso ya no sirve: los pulsos
miden 4, 4 y 6 pasos. Así que `steps_per_beat` desapareció, y cada
sitio que lo usaba pregunta a `beat_at` en su lugar:

```rust
pub struct Beat {
    pub index: usize,   // 0 for the first beat of the bar
    pub start: usize,   // steps from the start of the bar
    pub length: usize,  // in steps
}

let balkan: TimeSignature = "2+2+3/8".parse().unwrap();
assert_eq!(balkan.beat_at(9), Beat { index: 2, start: 8, length: 6 });
```

Eliminar la función vieja fue a propósito: el compilador enumeró
entonces cada sitio que seguía llamándola (las líneas del playlist, los
colores de los pulsos del channel rack, la pantalla BAR:BEAT:STEP, el
ajuste a pulsos, los tempo glides y la escucha de acordes de la
biblioteca), y cada uno se cambió para preguntar por el pulso real.
Borrar una función para que el compilador encuentre a sus usuarios es
un truco habitual de Rust.

## Leer "2+2+3/8": `FromStr`

`"7/8".parse::<TimeSignature>()` funciona porque `TimeSignature`
implementa el trait estándar `FromStr`. Su tipo de error es un `String`
que el menú muestra tal cual:

```rust
impl FromStr for TimeSignature {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let (top, bottom) = text
            .split_once('/')
            .ok_or("write it as two numbers, like 7/8")?;
        // ... the bottom must be 1, 2, 4, 8 or 16, and the top
        // a number or numbers joined by "+"
    }
}
```

`split_once('/')` devuelve `Some((before, after))` o `None`; `ok_or`
convierte el `None` en un error, y `?` devuelve ese error al momento.
La implementación de `Display` escribe el mismo texto de vuelta, así
que una prueba puede comprobar que todo compás sobrevive a `to_string`
y luego `parse`.

## Guardarlo sin romper los archivos viejos

Un archivo de proyecto guarda un compás como `(beats: 7, unit: 8)`.
Los archivos sin agrupación deben quedarse exactamente así, para que
las versiones viejas de Ferrous Lineup sigan abriéndolos. El atributo
de serde `#[serde(from = "SavedTime", into = "SavedTime")]` dice: para
guardar un `TimeSignature`, conviértelo primero en un `SavedTime`, y
para cargar uno, lee un `SavedTime` y conviértelo de vuelta:

```rust
#[derive(Serialize, Deserialize)]
struct SavedTime {
    beats: u8,
    unit: u8,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    groups: Vec<u8>,
}
```

`SavedTime` puede guardar un `Vec` amigable (`groups: [2, 2, 3]`),
porque solo existe un instante, mientras se guarda o se carga. Cuando
no hay agrupación el campo se omite, y el archivo queda igual que antes.

Un archivo *con* agrupación se guarda como formato 11. Una versión
vieja leería el 7/8 pero ignoraría la agrupación y dibujaría una línea
en cada corchea, así que se le dice que el archivo es más nuevo. 11/8 o
13/16 sin agrupación no necesitan nada nuevo: los dos números siempre
se guardaron tal cual.

## Pruébalo

1. Pon el Time de las harmony keys en 7/8, luego en 2+2+3/8, y mira el
   channel rack: los colores de los botones de paso cambian de siete
   parejas a dos, dos, tres.
2. Haz clic derecho en el compás 5 del playlist y ponlo en 3+3+2/8.
   Sus compases miden 16 pasos como 4/4, pero las líneas de pulso caen
   en otro sitio.
3. En una prueba, comprueba que `"3+3/8".parse::<TimeSignature>()` es
   igual a `TimeSignature::new(6, 8)`. ¿Por qué le importa eso al menú?
