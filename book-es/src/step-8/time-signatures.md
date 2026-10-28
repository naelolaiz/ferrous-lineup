# Los compases

Archivos: [`crates/lineup-core/src/meter.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/meter.rs),
[`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs),
[`crates/lineup-core/src/midi_file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/midi_file.rs)
y [`crates/lineup-app/src/playlist.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/playlist.rs)

Hasta ahora cada compás de Ferrous Lineup tenía dieciséis pasos:
cuatro pulsos de cuatro semicorcheas. Eso es 4/4, y la mayor parte del
pop está en él. Pero un vals cuenta *un dos tres*, un tango vals
también, y *Greensleeves*, *Amazing Grace* y *Für Elise* tampoco están
en cuatro. La biblioteca los necesita, así que el paso 8 añade compases.

## La música

Un compás parece una fracción. El número de abajo dice qué nota es un
pulso (4 para la negra, 8 para la corchea), y el de arriba dice
cuántas forman un compás:

| Compás | Un compás son | Pasos | Se siente como |
|--------|---------------|-------|----------------|
| 4/4 | cuatro negras | 16 | *un* dos tres cuatro |
| 3/4 | tres negras | 12 | *un* dos tres: un vals |
| 2/4 | dos negras | 8 | *un* dos: una marcha, una polca |
| 6/8 | seis corcheas | 12 | *un* dos tres *cuatro* cinco seis: dos pulsos grandes de tres |
| 7/8 | siete corcheas | 14 | dos y dos y tres, u otra mezcla |

3/4 y 6/8 tienen ambos doce pasos por compás, y las notas en ellos
pueden ser las mismas. Lo que cambia es cómo se *cuentan*: tres pulsos
de cuatro pasos, o dos pulsos de seis. Por eso un compás en Ferrous
Lineup cambia dónde se dibujan las líneas, no lo que se toca.

## `TimeSignature`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TimeSignature {
    pub beats: u8,
    pub unit: u8,
}

impl TimeSignature {
    pub const COMMON: TimeSignature = TimeSignature::new(4, 4);

    pub const fn new(beats: u8, unit: u8) -> Self {
        TimeSignature { beats, unit }
    }

    pub fn steps_per_bar(self) -> usize {
        let unit = usize::from(self.unit.clamp(1, 16));
        (usize::from(self.beats) * 16 / unit).max(1)
    }
}
```

`new` es un `const fn`, así que se puede llamar donde Rust necesita un
valor mientras compila: en `COMMON`, y en la lista de todos los
compases que ofrece el menú, `ALL`. `steps_per_bar` se protege contra
disparates de un archivo dañado: `clamp` mantiene la unidad entre 1 y
16, para que nunca divida por cero, y `max(1)` asegura que un compás
nunca mida cero pasos, lo que haría que todo "¿qué compás es este?"
fuera también una división por cero.

Dónde caen los pulsos es otra cuestión:

```rust
pub fn steps_per_beat(self) -> usize {
    let unit_steps = (16 / usize::from(self.unit.clamp(1, 16))).max(1);
    if self.unit == 8 && self.beats.is_multiple_of(3) && self.beats > 3 {
        unit_steps * 3
    } else {
        unit_steps
    }
}

pub fn is_bar_start(self, step: usize) -> bool {
    step.is_multiple_of(self.steps_per_bar())
}
```

6/8, 9/8 y 12/8 se sienten en grupos de tres corcheas, así que su pulso
mide seis pasos. `is_multiple_of` es la forma clara de escribir
`x % n == 0`, y no entra en pánico cuando `n` es 0 (solo `0` es
múltiplo de `0`); Clippy lo sugiere siempre que ve la versión con `%`.

`TimeSignature` implementa `Display`, el trait detrás de `{}` en
`format!` y detrás de `to_string()`:

```rust
impl fmt::Display for TimeSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.beats, self.unit)
    }
}
```

Implementa `Display` una vez, y todo `format!("{time}")`, toda
etiqueta de la UI y todo mensaje de error escribe "3/4" igual.

## Todo lo que contaba hasta dieciséis

Encontrarlos fue cuestión de buscar `16`, `STEPS_PER_BAR` y `% 4`.
Cada uno pregunta ahora el compás del proyecto:

- **el playlist**: el ancho de un compás, los números de compás de la
  regla, las líneas de compás y de pulso, y dónde se ajustan los clips.
  `Snap::Bar` solía ser dieciséis pasos; ahora pregunta:

  ```rust
  fn steps(self, time: TimeSignature) -> usize {
      match self {
          Snap::Bar => time.steps_per_bar(),
          Snap::Beat => time.steps_per_beat(),
          Snap::Step => 1,
      }
  }
  ```

- **el channel rack**: los botones de paso alternan colores por pulso,
  y vuelven a empezar en cada compás, para que todos los compases se
  vean iguales,
- **el piano roll**: sus líneas de compás y de pulso, en la cuadrícula
  y en el carril de velocity,
- **el menú de longitud del patrón**: ofrece compases enteros (1, 2, 3,
  4, 6, 8, 12 y 16 de ellos) y escribe "4 compases" en lugar de "64
  pasos". Una longitud que no es un número entero de compases, como 16
  pasos en 3/4, se muestra en pasos,
- **la longitud de la canción**, que se redondea hacia arriba a un
  compás entero.

![Tango vals, en 3/4: los pulsos del rack van de tres en tres, y los compases del playlist miden doce pasos de ancho](../images/step-8-waltz.png)

El hilo de audio no cambió en absoluto. Toca pasos, y un paso es una
semicorchea en todo compás.

## En los archivos: formato 3

Un archivo de proyecto recibe un campo nuevo, pero solo cuando lo
necesita:

```rust
#[serde(default, skip_serializing_if = "TimeSignature::is_common")]
time_signature: TimeSignature,
```

Son *atributos de campo* de serde. `skip_serializing_if` nombra una
función a la que se pregunta antes de escribir el campo; para 4/4
responde que sí, y el campo se omite, así que un proyecto en 4/4 se
escribe exactamente igual que antes. `default` funciona al revés, al
leer: un archivo sin el campo recibe `TimeSignature::default()`, que es
4/4. Juntos significan que los archivos viejos se abren sin cambios, y
los archivos nuevos en 4/4 pueden seguir abriéndolos versiones viejas
de Ferrous Lineup.

Un proyecto en otro compás se escribe como **formato 3**, que las
versiones viejas rechazan, en lugar de abrir un vals y dibujarlo en
compases de cuatro. El formato 3 cubre también las otras cosas que el
paso 8 añadió y que una versión vieja haría mal: los nueve tambores
nuevos, y los patrones de más de cuatro compases, que cortaría sin
decir nada. `format_for` comprueba las tres cosas, y devuelve el
formato más viejo que puede contener el proyecto, como en el paso 7.

**Hoy.** `FORMAT_VERSION` es 27, no 3, pero la regla sigue en pie: un
proyecto cuya única novedad es un compás simple, como 3/4, se escribe
como formato 3, de modo que una app vieja lo rechaza y una nueva lo
abre. Un compás agrupado, como 2+2+3/8, necesita el formato 11, y
cualquier otra característica posterior necesita su propio número más
alto.

## En los archivos MIDI

Un Standard MIDI File tiene un *meta evento* para el compás, que el
paso 7 siempre escribía como 4/4. Guarda cuatro bytes: los pulsos, la
unidad como potencia de dos (2 para la negra, porque 2² = 4; 3 para la
corchea), cuántos MIDI clocks dura un clic de metrónomo (24 por negra)
y un 8 constante:

```rust
let signature = project.time_signature;
let unit_power = signature.unit.max(1).ilog2() as u8;
let clocks = (signature.steps_per_beat() * 6).min(255) as u8;
track.meta(0, TIME_SIGNATURE, &[signature.beats, unit_power, clocks, 8]);
```

`ilog2` es el logaritmo entero en base 2: cuántas veces puedes partir
el número a la mitad antes de llegar a 1. `8u8.ilog2()` es 3. Entra en
pánico con 0, por eso la unidad pasa antes por `max(1)`. En 6/8 un clic
es una negra con puntillo, seis pasos, así que 36 clocks; una prueba
comprueba exactamente esos bytes.

> Más adelante en el paso 8 un compás puede ser dos números
> cualesquiera, y los pulsos pueden agruparse (2+2+3/8):
> `steps_per_beat` dejó paso a `beat_at`. Véase [Cualquier compás, y
> métricas irregulares](any-time-signature.md).
