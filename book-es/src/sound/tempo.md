# Tempo, compás, swing y sensación

Archivos: [`crates/lineup-core/src/meter.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/meter.rs),
[`crates/lineup-core/src/tempo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/tempo.rs),
[`crates/lineup-core/src/remeter.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/remeter.rs),
[`crates/lineup-core/src/swing.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/swing.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs),
[`crates/lineup-app/src/playlist/tempo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/playlist/tempo.rs),
[`crates/lineup-app/src/time_picker.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/time_picker.rs)
y [`crates/lineup-app/src/app/swing.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/swing.rs)

En Ferrous Lineup un paso es siempre una semicorchea. Un vals, un
ritardando, un shuffle y un 7/8 búlgaro se escriben todos sobre esa
misma cuadrícula. Lo que cambia es cómo se *cuentan* los pasos y cómo se
*tocan*:

| Qué | Qué cambia | Dónde |
|---|---|---|
| Una indicación de compás | Dónde caen las líneas de compás y de pulso; nunca una nota | `meter.rs` |
| Un cambio de tempo | Cuánto dura cada paso | `tempo.rs`, `engine.rs` |
| Pasar la canción a otro compás | Todas las notas, para que cada compás sea un compás del nuevo | `remeter.rs` |
| El swing | Cuándo se toca la segunda corchea de cada pulso, no dónde está escrita | `swing.rs` |
| La soltura | Cuánto tarde empieza cada nota, al tocar | `engine.rs` |

La música que hay detrás de todo esto (pulsos y compases, el dos y el
tres, las métricas irregulares, qué es un ritardando y qué es el swing)
se explica en el manual de usuario: [Ritmo y
métrica](https://naelolaiz.github.io/ferrous-lineup/es/manual/rhythm-and-meter.html),
[Tempo, compases y cambios de
tonalidad](https://naelolaiz.github.io/ferrous-lineup/es/manual/tempo-meter-key.html),
[Swing y sensación](https://naelolaiz.github.io/ferrous-lineup/es/manual/swing.html)
y [Métricas irregulares y los
Balcanes](https://naelolaiz.github.io/ferrous-lineup/es/manual/odd-meters-and-balkans.html).
Este capítulo trata del código.

## `TimeSignature`: dos números y una agrupación

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "SavedTime", into = "SavedTime")]
pub struct TimeSignature {
    /// How many notes of `unit` length make a bar: the top number.
    pub beats: u8,
    /// Which note counts as a beat: 4 for quarter notes, 8 for eighths.
    /// The bottom number.
    pub unit: u8,
    /// Where the felt beats start, when they are not the usual ones: bit
    /// `i` is set when a beat starts on the bar's `i`-th `unit` note.
    /// 0 means the usual beats (see [`TimeSignature::usual_groups`]).
    ///
    /// A `u64` keeps `TimeSignature` `Copy`, so it can be passed around
    /// as freely as a number; a `Vec` could not be.
    groups: u64,
}
```

El número de arriba puede ser cualquiera, desde 1 hasta lo que quepa en
un patrón; el de abajo es 1, 2, 4, 8 o 16 (`UNITS`). No hay 32: un
compás de 7/32 terminaría en mitad de un paso de semicorchea.

Todo lo demás se calcula a partir de esos números, y la aritmética está
escrita para que un archivo dañado no pueda hacerla dividir por cero:

```rust
pub fn unit_steps(self) -> usize {
    (16 / usize::from(self.unit.clamp(1, 16))).max(1)
}

pub fn steps_per_bar(self) -> usize {
    let unit = usize::from(self.unit.clamp(1, 16));
    (usize::from(self.beats) * 16 / unit).max(1)
}
```

`clamp` deja la unidad entre 1 y 16, así que la división siempre tiene
por qué dividir, y `max(1)` mantiene un compás de al menos un paso,
porque «¿en qué compás está el paso 50?» divide por la longitud del
compás. Los menús y `set_time_marker` solo aceptan indicaciones de
compás que pasan `is_valid`, pero un archivo editado a mano todavía
puede decir `beats: 0` o `unit: 0`, y el test
`nonsense_does_not_divide_by_zero` comprueba que la aritmética lo
aguanta.

`new` es una `const fn`, así que las indicaciones de compás pueden ser
constantes:

```rust
pub const COMMON: TimeSignature = TimeSignature::new(4, 4);

pub const ALL: [TimeSignature; 10] = [
    TimeSignature::new(2, 4),
    TimeSignature::new(3, 4),
    // ...
];
```

### Una agrupación dentro de un tipo `Copy`

En 7/8 las siete corcheas se sienten como pulsos de dos y de tres, y el
orden importa: 2+2+3/8 es una rachenitsa búlgara, 3+2+2/8 un
kalamatianós griego. La agrupación es parte de la indicación de compás.

`TimeSignature` se pasa por valor en todas partes, y para eso tiene que
ser `Copy`: se copia como un entero, sin memoria en el montón. Lo natural
sería guardar la agrupación en un `Vec<u8>`, `[2, 2, 3]`, pero un tipo
que contiene un `Vec` no puede ser `Copy`. Así que la agrupación es una
**máscara de bits**: el bit `i` está encendido cuando un pulso empieza en
la nota `i` del compás. 2+2+3/8 empieza sus pulsos en las notas 0, 2 y
4, así que su máscara es `0b10101`. Un `u64` tiene sitio para 64 notas
(`MAX_GROUPED_BEATS`), más que cualquier compás real.

Las canciones y los ritmos son tablas `const`, así que una agrupación
también tiene que poder construirse al compilar:

```rust
pub const fn of_groups(groups: &[u8], unit: u8) -> Self {
    let mut mask = 0_u64;
    let mut at = 0;
    let mut index = 0;
    // `for` loops are not allowed in a `const fn` yet: `while` is.
    while index < groups.len() {
        assert!(groups[index] > 0, "a group of no notes");
        assert!(at < Self::MAX_GROUPED_BEATS as usize, "too many notes");
        mask |= 1 << at;
        at += groups[index] as usize;
        index += 1;
    }
    assert!(at > 0 && at <= Self::MAX_GROUPED_BEATS as usize);
    let mut time = TimeSignature::new(at as u8, unit);
    // The usual grouping is written the plain way, so 3+3/8 and 6/8
    // are the same time signature.
    if mask != time.usual_mask() {
        time.groups = mask;
    }
    time
}
```

`1 << at` es un 1 desplazado `at` posiciones a la izquierda, y `|=`
enciende ese bit sin tocar los demás. Un `assert!` que falla dentro de
una `const fn` evaluada para una constante detiene la **compilación**,
no el programa, así que una agrupación mal escrita en la tabla de una
canción nunca llega a nadie. Para las agrupaciones que escribe una
persona está `grouped`, que comprueba lo mismo y devuelve un `Option`.

Las últimas líneas mantienen una regla: la agrupación habitual se guarda
como 0. Así `"3+3/8"` y `"6/8"` dan el mismo valor, y `==` dice que son
iguales. Sin ella, el menú Compás mostraría 6/8 sin seleccionar después
de que alguien escribiera 3+3.

El campo `groups` es privado, así que el código de fuera de `meter.rs`
solo puede crear una agrupación con `grouped` u `of_groups`. `beats` y
`unit` son públicos, eso sí, de modo que alguien podría cambiar `beats` y
dejar una máscara que ya no encaja. El `mask()` privado lo cubre: una
máscara que no encaja con el número de arriba se ignora, y se usan los
pulsos habituales.

### Pulsos de distinta longitud

En 2+2+3/8 los pulsos duran 4, 4 y 6 pasos, así que «¿en qué pulso está
este paso?» no puede ser una división. Todo le pregunta a `beat_at`:

```rust
pub struct Beat {
    /// Counted from 0: the first beat of the bar is 0.
    pub index: usize,
    /// The step it starts on, counted from the start of the bar.
    pub start: usize,
    /// How many steps it lasts.
    pub length: usize,
}
```

Del test `irregular_meters_group_their_beats`, donde `typed` es una
forma corta de `text.parse().unwrap()`:

```rust
let balkan = typed("2+2+3/8");
assert_eq!(balkan.beat_starts(), [0, 4, 8]);
assert_eq!(balkan.beat_at(9).length, 6);
assert_eq!(balkan.beat_at(9).index, 2);
```

`beat_at` busca el último comienzo de pulso en el paso o antes con
`rposition`, que recorre un slice desde el final.

No hay ninguna función de «pasos por pulso», a propósito. Cualquier
código que supusiera pulsos iguales estaría mal en una métrica
irregular, y sin una función así que llamar nadie puede cometer ese
error. (Quitar una función es además la forma más rápida de encontrar a
todos los que la llaman: el compilador los enumera.)

A un compás se le puede preguntar por sus pulsos de tres maneras, porque
tres partes del programa entienden cosas distintas por «pulso»:

| Función | Responde | 4/4 | 6/8 | 7/8 | 2+2+3/8 | 5/4 |
|---|---|---|---|---|---|---|
| `groups` | los pulsos que marca un metrónomo, en unidades | 1 1 1 1 | 3 3 | 1×7 | 2 2 3 | 1×5 |
| `usual_groups` | las notas de la unidad en un pulso habitual | 1 | 3 | 1 | 1 | 1 |
| `felt_groups` | los pulsos en que se apoya un músico | 2 2 | 3 3 | 2 2 3 | 2 2 3 | 2 3 |

`felt_groups` es lo que usa la banda para su «um-pa»: la agrupación si
la hay, los pulsos de tres corcheas del 6/8, y si no, pulsos de dos con
uno de tres al final. Los compases de tres notas o menos, como 3/4, son
un solo pulso sentido. Cómo los toca la banda está en [La
banda](../band/band.md).

### Leer y escribir «2+2+3/8»

`TimeSignature` implementa `Display`, así que `format!("{time}")`
escribe «3/4», o «2+2+3/8» con la agrupación como número de arriba, en
todas partes del mismo modo. La dirección contraria es el trait estándar
`FromStr`, que es lo que hace funcionar `"7/8".parse::<TimeSignature>()`:

```rust
impl FromStr for TimeSignature {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let (top, bottom) = text
            .split_once('/')
            .ok_or(tr("write it as two numbers, like 7/8"))?;
        // ... then the bottom number, the top number or numbers, and the bar's size
    }
}
```

`split_once('/')` da `Some((antes, después))` o `None`; `ok_or`
convierte el `None` en un error, y `?` lo devuelve en el acto. El error
es un `String` que el menú Compás muestra tal cual, en rojo, así que los
mensajes pasan por `tr` y salen en el idioma de la aplicación («escríbelo
como dos números, como 7/8»; ver [Dos
idiomas](../program/languages.md)).

La fila «Otro» del menú Compás guarda lo que se va escribiendo en un
pequeño `Draft` (el número de arriba, el de abajo, y la agrupación como
texto) en la memoria de egui entre fotogramas, y construye la indicación
de compás con el mismo `parse`. Si los grupos no suman el número de
arriba, lo dice: «los grupos suman 8, no 11».

### Guardarla sin cambiar los archivos viejos

Una indicación de compás sin agrupación tiene que guardarse exactamente
como antes, `(beats: 7, unit: 8)`, para que las versiones anteriores
sigan abriendo el archivo. El atributo `#[serde(from = "SavedTime", into
= "SavedTime")]` de `TimeSignature` dice: para guardarla, conviértela
antes en un `SavedTime`; para cargarla, lee un `SavedTime` y conviértelo
de vuelta:

```rust
#[derive(Serialize, Deserialize)]
struct SavedTime {
    beats: u8,
    unit: u8,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    groups: Vec<u8>,
}
```

`SavedTime` puede tener un `Vec` cómodo (`groups: [2, 2, 3]`) porque solo
vive un momento, mientras se guarda o se carga. La conversión
`From<SavedTime>` deja fuera una agrupación que no suma el número de
arriba, así que un archivo editado a mano no puede crear un compás cuyos
pulsos no encajen.

## `Meter`: indicaciones de compás que cambian

Una canción puede cambiar de compás a mitad de camino: la última línea
de un himno en 3/4, la Promenade de Músorgski alternando 5/4 y 6/4. Cada
cambio es una marca en la lista de reproducción:

```rust
pub struct TimeMarker {
    /// The step it starts on, counted from the start of the song.
    pub start: usize,
    pub time: TimeSignature,
}
```

Una marca siempre empieza un compás nuevo. Una que se pone en mitad de
un compás lo deja más corto, que es como una partitura escribe un compás
al que le falta un pulso.

Con marcas, «¿en qué compás está el paso 50?» depende de todo lo que hay
antes del paso 50, así que todas las preguntas sobre compases van a un
solo tipo:

```rust
pub struct Meter {
    first: TimeSignature,
    /// Sorted by start, all after step 0, no two on the same step.
    markers: Vec<TimeMarker>,
}
```

El comentario de `markers` es un *invariante*, una regla que siempre se
cumple. Los campos son privados y `Meter::new` la hace cumplir (quita las
marcas del paso 0, ordena las demás y elimina duplicados con
`dedup_by_key`), así que ningún método tiene que volver a comprobarla.

`bar_at` recorre la canción por **tramos**, uno por indicación de
compás, contando los compases que pasa:

```text
4/4 desde 0, 3/4 desde 32:
tramo 1: pasos 0..32,  16 por compás -> compases 0 y 1
tramo 2: pasos 32..,   12 por compás -> compases 2, 3, 4, ...
```

```rust
fn stretches(&self) -> impl Iterator<Item = (usize, usize, TimeSignature)> + '_ {
    let starts =
        std::iter::once(TimeMarker::new(0, self.first)).chain(self.markers.iter().copied());
    let ends = self
        .markers
        .iter()
        .map(|marker| marker.start)
        .chain(std::iter::once(usize::MAX));
    starts
        .zip(ends)
        .map(|(from, end)| (from.start, end, from.time))
}

pub fn bar_at(&self, step: usize) -> Bar {
    let mut bars_before = 0;
    for (start, end, time) in self.stretches() {
        let bar = time.steps_per_bar();
        if step < end {
            let index = (step - start) / bar;
            let bar_start = start + index * bar;
            return Bar {
                number: bars_before + index,
                start: bar_start,
                length: bar.min(end - bar_start),
                time,
            };
        }
        bars_before += (end - start).div_ceil(bar);
    }
    unreachable!("the last stretch never ends")
}
```

`stretches` construye un iterador a partir de otros: los comienzos (la
primera indicación de compás, luego cada marca) se emparejan con `zip`
con los finales (cada marca, luego `usize::MAX` para «nunca»).
`div_ceil` divide redondeando hacia arriba, así que un compás corto
antes de una marca sigue contando como compás, y `bar.min(end -
bar_start)` le da a ese compás corto su longitud real. `unreachable!` le
dice al lector, y al compilador, que el bucle siempre devuelve algo: el
último tramo termina en `usize::MAX`.

`bars()` da todos los compases, sin fin:

```rust
pub fn bars(&self) -> impl Iterator<Item = Bar> + '_ {
    let mut next = 0;
    std::iter::from_fn(move || {
        let bar = self.bar_at(next);
        next = bar.end();
        Some(bar)
    })
}
```

`std::iter::from_fn` convierte un closure en un iterador: cada llamada a
`next` ejecuta el closure. Nunca devuelve `None`, así que el iterador no
se detiene solo; la lista de reproducción dibuja su cuadrícula con
`meter.bars().take_while(|bar| bar.start <= steps)` y decide ella cuánto
usar.

El resto de `Meter` son las preguntas que hace la aplicación: `time_at`,
`bar(number)`, `is_bar_start`, `floor_to_bar`, `floor_to_beat`,
`beat_end`, `round_up_to_bar`, y `position(step)` para el indicador
COMPÁS:PULSO:PASO. `Meter::steady(time)` es una canción sin marcas, y un
test comprueba, paso a paso, que responde exactamente igual que la
`TimeSignature` sola. `tidy()` quita las marcas que no cambian nada: una
que mantiene la indicación de compás anterior y cae en una de sus líneas
de compás.

### Quién le pregunta al compás

- **La lista de reproducción**: su cuadrícula, los números de compás,
  las secciones y el ajuste. `Snap::floor` y `Snap::next` dan el
  principio y el final del compás o del pulso bajo el puntero, que es lo
  que necesita arrastrar un clip, una sección o el bucle.
- **El rack de canales y el piano roll** muestran un patrón, no la
  canción. Preguntan `Project::pattern_time(pattern)`: la indicación de
  compás donde empieza el primer clip del patrón, o, para un patrón que
  no está en la canción, su propio `Pattern::time` (un vals probado desde
  la Biblioteca está en 3/4 aunque la canción esté en 4/4), o si no, la
  del proyecto.
- **La longitud de la canción**, redondeada hasta la siguiente línea de
  compás.
- **El menú de longitud del patrón**, que ofrece compases enteros
  (`pattern_lengths`: 1, 2, 3, 4, 6, 8, 12 o 16, mientras quepan) y
  escribe «4 compases» en vez de «64 pasos» (`describe_length`).

### La canción tal como suena

Las secciones pueden repetirse ([Secciones, repeticiones y
bucles](../program/project.md#secciones-repeticiones-y-bucles)), así
que la canción tal como se oye es más larga que la lista de reproducción
tal como se dibuja. Si una sección en 3/4 suena dos veces, la canción que
se oye tiene dos tramos de 3/4. Las *pasadas* del arreglo dicen qué parte
de la línea de tiempo suena cuándo, y `Project::played_meter` construye
un segundo `Meter`, en pasos de la canción:

```rust
pub fn played_meter(&self, arrangement: &Arrangement) -> Meter {
    let meter = self.meter();
    let first = arrangement
        .passes
        .first()
        .map_or(self.time_signature, |pass| meter.time_at(pass.from));
    let mut markers = Vec::new();
    for pass in &arrangement.passes {
        markers.push(TimeMarker::new(pass.start, meter.time_at(pass.from)));
        for marker in meter.markers() {
            if marker.start > pass.from && marker.start < pass.from + pass.length {
                markers.push(TimeMarker::new(
                    pass.start + marker.start - pass.from,
                    marker.time,
                ));
            }
        }
    }
    Meter::new(first, markers).tidy()
}
```

Cada pasada recibe una marca con la indicación de compás en que empieza,
más las marcas que hay dentro, y luego `tidy` quita las que no cambian
nada, como una pasada en 4/4 después de otra en 4/4 sobre una línea de
compás. El transporte usa este compás para el indicador de posición y
para RePág/AvPág, así que «el compás siguiente» es el siguiente que se
oye.

El metrónomo también lo usa. `played_clicks` lo convierte en un `Click`
por cada paso de la canción (`Bar`, `Beat` o `None`), y la aplicación
envía esa lista al hilo de audio con `Command::SetSongClicks`, así que el
metrónomo marca los pulsos de cada compás: las dos negras con puntillo
del 6/8, los tres grupos del 2+2+3/8, un clic más agudo en cada «uno». En
modo patrón es `pattern_clicks`, en el compás del propio patrón. La
cuenta previa y el sonido del metrónomo están en [Herramientas de
práctica](../band/practice.md).

## Cambios de tempo

### `Tempo`, siempre válido

```rust
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(from = "f32", into = "f32")]
pub struct Tempo {
    bpm: f32,
}
```

El campo es privado, así que la única forma de crear un `Tempo` es
`Tempo::new`, que deja el valor entre `MIN` (10) y `MAX` (522) BPM y
convierte un NaN en el tempo por defecto, 130. Todo `Tempo` del programa
es, por tanto, válido. Los archivos pasan por la misma puerta: un tempo
se guarda como un número sin más, y `From<f32>` lo lee con `Tempo::new`,
así que un archivo que dice `tempo: 9000.0` se abre a 522
([Guardar](../program/saving.md)).

### Marcas, y un mapa

Un cambio de tempo es una marca en la lista de reproducción, como un
cambio de compás:

```rust
pub struct TempoMarker {
    /// The step it starts on, counted from the start of the song.
    pub start: usize,
    pub tempo: Tempo,
    /// How many steps the change takes; 0 for at once. Project files
    /// leave it out when it is 0.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub glide: usize,
}
```

Con `glide` en 0 el tempo cambia de golpe, como después de una pausa.
Con un `glide` se mueve un poco en cada paso hasta llegar: hacia abajo en
un ritardando, hacia arriba en un accelerando. Un paso es una
semicorchea, así que un cambio gradual a lo largo de dos compases de 4/4
son 32 cambios pequeños, tan cerca de una curva que nadie oye los
escalones. El tempo nunca cambia en mitad de un paso.

```rust
fn bpm_on(&self, step: usize, from: f32) -> f32 {
    let into = step - self.start + 1;
    if into >= self.glide {
        return self.tempo.bpm();
    }
    let done = into as f32 / self.glide as f32;
    from + (self.tempo.bpm() - from) * done
}
```

Un cambio gradual de 4 pasos de 100 a 60 BPM toca 90, 80, 70 y 60: el
último paso del cambio ya está en el tempo nuevo.

`TempoMap` es al tempo lo que `Meter` a los compases: el primer tempo y
las marcas, con el mismo invariante, y una pregunta principal:

```rust
pub fn tempo_at(&self, step: usize) -> Tempo {
    // The marker in effect so far, and the tempo its glide started from.
    let mut current: Option<(&TempoMarker, f32)> = None;
    let bpm_on = |current: Option<(&TempoMarker, f32)>, step: usize| match current {
        Some((marker, from)) => marker.bpm_on(step, from),
        None => self.first.bpm(),
    };
    for marker in self.markers.iter().take_while(|m| m.start <= step) {
        let from = bpm_on(current, marker.start - 1);
        current = Some((marker, from));
    }
    Tempo::new(bpm_on(current, step))
}
```

Un cambio gradual parte del tempo que sonaba en el paso anterior a su
marca, aunque fuera en mitad de otro cambio gradual, así que el bucle
lleva `from` consigo: cada marca le pregunta a la anterior «¿dónde
estabas un paso antes de mí?». `marker.start - 1` no puede desbordarse
hacia abajo, porque ninguna marca está en el paso 0.

### La franja de tempo

La franja que está sobre los números de compás de la lista de
reproducción dibuja el mapa como una línea. Solo necesita los pasos
donde la línea se dobla (el principio, cada marca y el final de cada
cambio gradual), unidos de dos en dos con `windows(2)`:

```rust
let mut bends = vec![0, steps];
for marker in map.markers() {
    bends.push(marker.start);
    bends.push(marker.start + marker.glide);
}
bends.retain(|&step| step <= steps);
bends.sort_unstable();
bends.dedup();
let mut points = Vec::new();
for pair in bends.windows(2) {
    let (from, to) = (pair[0], pair[1]);
    points.push(pos2(layout.x(from), y(map.tempo_at(from))));
    points.push(pos2(layout.x(to), y(map.tempo_at(to.saturating_sub(1)))));
}
```

Cada tramo va del tempo de su primer paso al tempo del último, así que un
cambio de golpe se convierte en un salto vertical y un cambio gradual en
una pendiente. Las etiquetas se escriben como las escribe una partitura:
`describe` compara el tempo de la marca con el de `tempo_before` de su
paso, y escribe «♩ = 90» para un cambio de golpe, «rit. → 60» para uno
gradual hacia abajo y «accel. → 140» para uno hacia arriba.

El cuadro TEMPO de la barra de herramientas fija el tempo con el que
*empieza* la canción, y `scale_tempo` mueve todas las marcas en la misma
proporción:

```rust
pub fn scale_tempo(&mut self, tempo: Tempo) {
    let ratio = tempo.bpm() / self.tempo.bpm();
    self.tempo = tempo;
    for marker in &mut self.playlist.tempo_markers {
        marker.tempo = Tempo::new(marker.tempo.bpm() * ratio);
    }
}
```

Practicar una pieza más despacio conserva su ritardando, solo que más
lento también. Mientras suena una canción cuyo tempo cambia, el cuadro
muestra TEMPO AHORA: `tempo_at` del lugar del cabezal en la línea de
tiempo.

Un estilo de la Biblioteca trae un tempo propio, y ese tiene que caer
donde la canción toca de verdad, no en su primera nota: un archivo MIDI
empieza a menudo con unas pocas notas lentas antes de su tempo real.
Así que `main_tempo` busca el tempo que la canción mantiene durante más
pasos, y `take_tempo` escala toda la canción para que ese se convierta
en el del estilo:

```rust
pub fn take_tempo(&mut self, tempo: Tempo) {
    let ratio = tempo.bpm() / self.main_tempo().bpm();
    self.scale_tempo(Tempo::new(self.tempo.bpm() * ratio));
}
```

`main_tempo` suma, para cada tempo, los pasos que dura: desde el
principio hasta la primera marca con el tempo inicial, y luego desde
cada marca hasta la siguiente. Un tempo que vuelve más tarde se suma a
lo que ya duraba. De los que duran más, gana el primero, así que una
canción con un único tempo constante obtiene ese, y un empate va al
tempo anterior:

```rust
let (mut tempo, mut from) = (self.tempo, 0);
for marker in &self.playlist.tempo_markers {
    let start = marker.start.clamp(from, end.max(from));
    hold(tempo, start - from);
    (tempo, from) = (marker.tempo, start);
}
hold(tempo, end.saturating_sub(from));
```

`hold` es un cierre que toma prestada la lista `held` de forma
mutable, así que se declara `let mut hold`: llamar a un cierre que
cambia lo que capturó cambia el estado del propio cierre.
`(tempo, from) = (…, …)` asigna dos variables a la vez, una
*asignación por desestructuración*.

### Cada paso a su propio tempo

`Project::played_tempos` da el tempo de cada paso de la canción *tal como
suena*, así que una sección que frena al final frena cada vez que se
repite:

```rust
pub fn played_tempos(&self, arrangement: &Arrangement) -> Vec<Tempo> {
    let map = self.tempo_map();
    if map.is_steady() {
        return Vec::new();
    }
    (0..arrangement.length)
        .map(|step| {
            let timeline = arrangement.timeline_step(step).unwrap_or(step);
            map.tempo_at(timeline)
        })
        .collect()
}
```

La lista se construye en el hilo de la interfaz y se envía con
`Command::SetSongTempos`, junto con la canción. El hilo de audio no puede
reservar memoria, y cuando llega una lista nueva la vieja vuelve al hilo
de la interfaz para liberarse, como las canciones viejas y las notas
viejas ([Dos hilos](../program/threads.md)). Una lista vacía, el caso
habitual, significa «toca el tempo del cuadro TEMPO», y no cuesta nada.

Un paso tiene que durar lo que dice *su propio* tempo, así que el tempo
debe fijarse antes de que el paso empiece. `due`, del secuenciador, dice
qué paso empieza en el próximo tick ([El
secuenciador](sequencer.md)), y el hilo de audio lo pregunta
primero, en cada frame:

```rust
if let Some(step) = self.sequencer.due() {
    // ...
    let tempo = self.tempo_of_step(step);
    if tempo != self.tempo_now {
        self.set_tempo(tempo);
    }
}
if let Some(step) = self.sequencer.tick() {
```

`tempo_of_step` es el tempo de la canción para ese paso en modo canción,
y el del cuadro TEMPO en modo patrón, donde se trabaja sobre un patrón y
se quiere un bucle estable. (La parte que se omite arriba, `lap_tempos`,
es el próximo tempo del entrenador de velocidad, que entra cuando el
bucle vuelve a empezar: [Herramientas de
práctica](../band/practice.md).)

Las notas que ya suenan tienen una longitud en frames, calculada con el
tempo anterior. `set_tempo` las estira para que sigan terminando en el
paso que deben:

```rust
fn set_tempo(&mut self, tempo: Tempo) {
    self.tempo_now = tempo;
    let before = self.sequencer.step_frames();
    let next_step = self.sequencer.frames_to_next_step();
    self.sequencer.set_tempo(tempo);
    let stretch = self.sequencer.step_frames() / before;
    for channel in &mut self.channels {
        channel.instrument.tempo_changed(next_step, stretch);
        channel.effects.set_tempo(tempo);
    }
    // ...
}
```

y cada instrumento calcula lo que queda de cada nota sostenida:

```rust
pub(crate) fn stretched(left: f64, next_step: f64, stretch: f64) -> f64 {
    if left <= next_step {
        left
    } else {
        (next_step + (left - next_step) * stretch).round()
    }
}
```

Los frames hasta el próximo paso pertenecen al paso que ya suena, que
conserva su longitud; solo se estira el resto. A una nota con 10 000
frames por delante, 2 000 de ellos en este paso, a la mitad del tempo
(un estiramiento de 2) le quedan 2 000 + 8 000 × 2 = 18 000. Los efectos
que siguen el pulso, como un eco ajustado en valores de nota, también
reciben el tempo nuevo ([El mezclador](mixer.md)).

Una exportación toca la canción con el mismo hilo de audio, así que
recibe la misma lista. Su longitud ya no es «pasos por la longitud de un
paso»: `played_seconds` suma cada paso a su propio tempo, con
`tempo::seconds`:

```rust
pub fn seconds(steps: usize, tempo: impl Fn(usize) -> Tempo) -> f64 {
    (0..steps)
        .map(|step| f64::from(tempo(step).seconds_per_beat()) / 4.0)
        .sum()
}
```

`impl Fn(usize) -> Tempo` acepta cualquier closure que vaya de un paso a
un tempo, así que la misma función sirve para un `TempoMap` en un test y
para la lista tocada en `played_seconds`.

## Pasar una canción a otro compás

Una indicación de compás solo mueve líneas de compás. Pero si eliges un
vals en la Biblioteca mientras suena una canción en 4/4, sus compases de
doce pasos puestos en compases de dieciséis tocan «un dos tres» y luego
un pulso de silencio, en cada compás. Para sonar a vals, cada compás de
la canción tiene que convertirse en un compás de 3/4. `remeter` lo hace,
con una regla: **el compás n de la canción vieja pasa a ser el compás n
de la nueva, y cada nota conserva su lugar en su compás**.

```text
4/4: | Do . . . Mi . . . Sol . . . Mi . . . |   16 pasos
3/4: | Do . . Mi . . Sol . . Mi . . |           12 pasos
```

Es un estiramiento mecánico, como lo haría un magnetófono, no como un
compositor reescribiría una melodía para otro compás.

### Apretar un compás, al tick

Apretar dieciséis pasos en doce deja muchas notas entre pasos: corcheas
separadas por dos pasos pasan a estar a 1,5 pasos. Redondearlas a la
cuadrícula las volvería desiguales. Así que el apretón se hace en
**ticks**, 24 por paso (`TICKS_PER_STEP`, la unidad que usa la
importación MIDI para las notas fuera de la cuadrícula, ver
[MIDI](midi.md)):

```rust
pub struct Remap {
    old: Meter,
    /// Steps in one new bar.
    bar: usize,
}

impl Remap {
    pub fn tick(&self, tick: usize) -> usize {
        let bar = self.old.bar_at(tick / TICKS_PER_STEP);
        let start = bar.start * TICKS_PER_STEP;
        let length = bar.length * TICKS_PER_STEP;
        let new_bar = self.bar * TICKS_PER_STEP;
        // Rounded to the nearest tick.
        bar.number * new_bar + ((tick - start) * new_bar + length / 2) / length
    }
}
```

En la última línea hay dos costumbres de la aritmética con enteros.
**Multiplicar antes de dividir**: `(tick - start) / length * new_bar`
daría 0 para cada tick dentro del compás. **Sumar la mitad antes de
dividir** para redondear al entero más cercano en vez de siempre hacia
abajo. La segunda corchea de un compás de 4/4, el tick 48 de 384, cae en
(48 × 288 + 192) / 384 = 36 de un compás de 3/4 de 288 ticks: 1,5 pasos,
exactos.

Un `Remap` guarda el `Meter` viejo de la canción, así que sirve para
canciones que ya cambian de compás: cada compás viejo, en cualquier
indicación (también uno corto), se convierte en un compás nuevo entero.
`Remap::step` es lo mismo, redondeado al paso más cercano, para lo que
vive en pasos.

### Cada compás conserva su duración

El tempo cuenta negras. Un compás de 3/4 tiene tres, uno de 4/4 cuatro,
así que con el mismo tempo la canción en 3/4 terminaría en tres cuartas
partes del tiempo. Por eso `remeter` escala el tempo en la misma
proporción que el compás: 4/4 a 112 pasa a ser 3/4 a 84, y un compás
dura 2,14 segundos en los dos. Las marcas de tempo se escalan igual, y un
cambio de compás en mitad de la canción que cambiaba la longitud del
compás recibe una marca de tempo propia, para que cada tramo conserve su
paso.

El escalado tiene que volver exacto. 108,5 BPM en 7/8, devuelto a 4/4,
tiene que ser 108,5 × 16 / 14 = 124, no 124,00001. Un `f32` tiene unas
siete cifras significativas, y la razón 16/14 no tiene valor exacto en
él. Así que el código multiplica primero y divide al final, en `f64`:

```rust
let scaled = |tempo: Tempo, step: usize| {
    Tempo::new((f64::from(tempo.bpm()) * bar as f64 / old_bar(step) as f64) as f32)
};
```

108,5 × 16 = 1736 es exacto, y 1736 / 14 = 124 también. Un test lleva
varios tempos por 7/8, 3/4, 13/16, 6/8 y 12/8 y de vuelta a 4/4, y
comprueba que vuelven sin cambios.

Así que después de `remeter` cada nota sigue empezando en el mismo
momento, en segundos, y dura lo mismo. La música suena igual; solo
cambia cómo se escribe y cómo se cuenta. Lo que hace que una canción
*suene* a vals es la batería y la forma de atacar los acordes, y por eso
la Biblioteca y el cuadro Compás pasan la canción al nuevo compás **y**
le dan un ritmo en él. Cómo se elige ese ritmo está en [La
biblioteca](../band/library.md).

### Patrones que se quedan largos

Un patrón tiene como mucho 256 pasos (`MAX_STEPS`). Dieciséis compases
de 4/4 son justo eso, pero dieciséis compases de 12/8 son 384. En vez de
perder notas, `remeter` corta un patrón así en trozos de compases enteros
que quepan:

```rust
let chunk = MAX_STEPS / bar * bar;
```

256 / 24 × 24 son 240 pasos: diez compases de 12/8, y luego seis. Los
trozos se llaman «Tema», «Tema (2)» y así sucesivamente, y cada clip del
patrón se convierte en clips de los trozos, uno detrás de otro. La
canción suena igual; la lista de reproducción muestra un clip más.

### Todo lo demás se mueve con su compás

Los clips, las secciones, el bucle, las marcas de tonalidad y las marcas
de la banda pasan todos por `song.step`. El cambio gradual de una marca
de tempo se traslada por sus dos extremos, así que un ritardando a lo
largo de dos compases de 4/4 dura dos compases de 3/4. Los acordes
cambian en un pulso, así que van al pulso más cercano del compás nuevo
(`snap_to_beat`). Las marcas de compás desaparecen: ahora toda la
canción está en una sola indicación de compás.

Dos marcas a un paso de distancia pueden quedar apretadas en el mismo
paso. `Vec::dedup_by` quita los vecinos que su closure considera
iguales; el closure recibe primero el elemento posterior y después el
anterior, y puede cambiar el anterior, que es como se escribe «gana la
marca posterior»:

```rust
playlist_mut.tempo_markers.dedup_by(|later, earlier| {
    later.start == earlier.start && {
        *earlier = *later;
        true
    }
});
```

`a && { ... }` ejecuta el bloque solo cuando `a` es verdadero: la copia
solo ocurre para marcas en el mismo paso.

`remeter_song` además calcula todos los patrones nuevos a partir del
proyecto sin tocar, se guarda un clon de la lista de reproducción vieja,
y solo entonces escribe. Leer el proyecto mientras se escribe
necesitaría dos préstamos del mismo `Project` a la vez, uno compartido y
otro mutable, cosa que el comprobador de préstamos rechaza con razón. Y
se ejecuta dentro de `project.moving_the_band(...)`: las notas de la
banda se mueven con todo lo demás, y eso no es una edición tuya, así que
la banda sigue la hoja de acordes ([La banda](../band/band.md)).

### No perder el sitio

Mientras la canción suena estás en algún lugar, compás 13, pulso 2, por
ejemplo. Después del apretón, el paso 196 ya no está en el compás 13. Así
que después de cualquier cambio de compás la aplicación busca el compás
en que estaba el cabezal, y cuánto había avanzado en él, y va al mismo
compás y a la misma fracción:

```rust
let place = |step: usize| {
    let bar = old.bar_at(step);
    let into = (step - bar.start) as f64 / bar.length.max(1) as f64;
    let target = new.bar(bar.number);
    target.start + (into * target.length as f64).round() as usize
};
```

`old` y `new` son los `played_meter` de antes y de después, así que las
repeticiones cuentan. El mismo closure mueve el lugar desde donde toca
▶ y al que vuelve ⏹. Funciona también después de deshacer
y rehacer, que pueden traer una canción con marcas de compás propias,
donde un `Remap` no encajaría.

## Swing

El swing toca las dos corcheas de cada pulso larga-corta. El proyecto
tiene una cantidad, de 0 («sin swing») a `MAX_SWING`, 100 (los dos
tercios y el tercio de un tresillo):

```rust
pub swing: u8,
```

Las notas de los patrones nunca se mueven. En cambio, todo lo que *toca*
la canción pide las notas tal como se tocan:

```rust
pub fn played_notes(&self, pattern: usize, channel: usize) -> Cow<'_, [NoteEvent]> {
    let Some(written) = self.pattern(pattern).map(|p| p.notes(channel)) else {
        return Cow::Borrowed(&[]);
    };
    match self.pattern_swing(pattern) {
        0 => Cow::Borrowed(written),
        swing => Cow::Owned(swung(written, swing, self.plays_hits(channel))),
    }
}
```

`Cow` («clone on write», de `std::borrow`) es o bien **prestado** o bien
**propio**. La mayoría de las canciones no tienen swing, y para ellas
`played_notes` devuelve las notas del propio patrón sin copiar ninguna.
Solo un patrón con swing recibe un `Vec` nuevo. Los dos se leen igual,
como un slice, y quien necesita un `Vec` propio llama a `.into_owned()`,
que solo copia si era prestado. El `'_` dice que el caso prestado toma
prestado de `self`: el resultado no puede vivir más que el proyecto.

El hilo de audio, la exportación MIDI y el acompañamiento que sigue lo
que tocas usan `played_notes`. El piano roll, los botones de paso y la
hoja de acordes leen las notas escritas, así que la cuadrícula que editas
sigue siendo una cuadrícula.

### Mover la segunda corchea

Un pulso son 96 ticks. La segunda corchea empieza en el tick 48; la
tercera nota de un tresillo, en el tick 64. Así que el swing completo
llega 16 ticks tarde:

```rust
const BEAT: usize = 4 * TICKS_PER_STEP;
const OFFBEAT: usize = BEAT / 2;
const FULL_DELAY: usize = BEAT * 2 / 3 - OFFBEAT;
const NEAR: usize = BEAT / 16;

pub fn delay(swing: u8) -> usize {
    let swing = usize::from(swing.min(MAX_SWING));
    (FULL_DELAY * swing + usize::from(MAX_SWING) / 2) / usize::from(MAX_SWING)
}
```

Sumar la mitad de 100 antes de dividir por 100 vuelve a redondear al tick
más cercano: 50 son 8 ticks, un swing de 7:5; 70 son 11,2 y quedan en 11.

`swung` mueve cada comienzo y cada final que cae en un contratiempo o
cerca de él:

```rust
pub fn swung(notes: &[NoteEvent], swing: u8, hits: bool) -> Vec<NoteEvent> {
    let late = delay(swing);
    let played = |tick: usize| match tick % BEAT {
        offset if offset.abs_diff(OFFBEAT) < NEAR => tick + late,
        _ => tick,
    };
    notes
        .iter()
        .map(|note| {
            let start = played(note.start_tick());
            let end = match hits {
                true => note.end_tick(),
                false => played(note.end_tick()),
            };
            NoteEvent {
                velocity: note.velocity,
                ..NoteEvent::exact(note.pitch, start, end.saturating_sub(start))
            }
        })
        .collect()
}
```

El primer brazo es una **guarda de match**: `offset` es una variable
nueva que encaja con cualquier cosa, y el `if` que la sigue decide si el
brazo se aplica. «Cerca» es menos de seis ticks a cada lado, porque las
cuerdas de un acorde rasgueado empiezan con un tick de diferencia, 48,
49, 50, y todas tienen que moverse juntas. `abs_diff` es la distancia
entre dos números, sea cual sea el mayor; `offset - OFFBEAT` sobre un
`usize` menor que 48 se desbordaría hacia abajo y entraría en pánico en
una compilación de depuración.

Un patrón de constante es distinto de un enlace a variable. Si el brazo
fuera `OFFBEAT => ...`, compararía con el valor de la constante, 48. Si
fuera un `offbeat => ...` en minúsculas, Rust lo leería como una variable
nueva que encaja con *todo*, y el brazo `_` no se ejecutaría nunca. El
compilador avisa de eso, y la convención de mayúsculas para las
constantes mantiene separados los dos casos.

Una nota que termina en el contratiempo también termina más tarde, así
que la primera corchea de verdad es larga, ligada a la segunda. Los
golpes de batería son distintos: un tambor muestreado suena hasta el
final por mucho que dure la nota, así que solo se mueven sus comienzos
(`hits`). La nota nueva se construye con `NoteEvent::exact`, la misma
función que usa la importación MIDI para las notas fuera de la
cuadrícula: una corchea con swing en el paso 2 pasa a ser el paso 2 con
un `nudge` de 16 ticks, que el hilo de audio ya sabe tocar tarde ([El
secuenciador](sequencer.md)). Las notas que no están en un
contratiempo (semicorcheas, una corchea con puntillo y una semicorchea)
quedan como están.

### Dónde se aplica el swing

```rust
pub fn swings_in(time: TimeSignature) -> bool {
    time.unit <= 4
}
```

Solo las indicaciones de compás que se cuentan en negras o en notas más
largas tienen pares de corcheas en un pulso. En 6/8 o 12/8 un pulso ya
son tres corcheas, y en 5/8 o 7/8 la corchea es el pulso.
`pattern_swing` devuelve 0 para un patrón en un compás así, y el control
Swing aparece en gris cuando ninguna parte de la canción puede tener
swing, con una pista que dice por qué. Una sección de la banda tocada en
otro estilo puede traer el swing de ese estilo (`section_swing`).

### Ningún comando nuevo para el hilo de audio

El hilo de audio se entera de cada edición por `diff`, que compara el
proyecto antes y después, y compara las notas *tocadas* ([Decirle al
hilo de audio lo que ha cambiado](../program/sync.md)). Así que cambiar
el swing no necesita un comando propio: los patrones cuyas notas tocadas
se movieron se envían de nuevo, y los demás se quedan como están. Un
patrón que pasa a 6/8, y deja de tener swing, se detecta igual, y lo
mismo deshacer, que pone en su sitio un proyecto anterior entero. Cuando
nada tiene swing, los dos lados de la comparación son `Cow` prestados, y
compararlos compara los slices sin copiar nada.

### Recordado por estilo

Cada ritmo de la Biblioteca lleva su propio swing (`Rhythm::swing`), y un
estilo toma el de su ritmo: Swing, Jazz ballad, Jazz waltz, Shuffle y
Five-four swing a 100, Rock and roll a 50, el resto sin swing. Sus
patrones de batería están escritos en corcheas iguales, como los escriben
las partituras, y el swing hace el resto. Cuando cambias la cantidad, la
aplicación la recuerda para ese estilo, entre una ejecución y otra, en
`app/swing.rs`:

```rust
#[derive(Debug, Default)]
pub(super) struct SwingMemory {
    amounts: BTreeMap<String, u8>,
    /// Whether an amount changed since they were last stored.
    changed: bool,
}

/// What is stored: a newtype, so it has a type of its own in egui's
/// store.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
struct Remembered(BTreeMap<String, u8>);
```

egui guarda los valores persistentes por un `Id` *y* un tipo. Envolver
el mapa en un struct propio, un **newtype**, le da un tipo que nadie más
usa, así que un `BTreeMap<String, u8>` guardado por otra parte de la
aplicación nunca podría leerse como este. Un `BTreeMap` mantiene sus
claves ordenadas, así que el archivo de ajustes enumera los estilos
siempre en el mismo orden. `store_swings` se ejecuta una vez por
fotograma y solo escribe cuando `changed` está activo.

Elegir un estilo le da primero a la canción el swing propio del estilo,
en `lineup-core`, que no sabe nada de ajustes; luego la aplicación pone
en su lugar la cantidad recordada, en el mismo paso de deshacer.

## Soltura

Un buen músico nunca está exactamente en la cuadrícula. **Soltura**, en
la barra de herramientas de la lista de reproducción, fija cuánto pueden
desviarse las notas, de *no* a 40 ms (`MAX_LOOSE_TIMING`). Se guarda en
el proyecto como `loose_timing`, en milisegundos, y el hilo de audio la
convierte en frames una sola vez, cuando llega `Command::SetLooseTiming`.

El retraso de cada nota se calcula cuando empieza su paso:

```rust
fn delay(loose_frames: f64, channel: usize, song_step: usize, pitch: Note) -> f64 {
    let onset = ((song_step as u64) << 8) | channel as u64;
    let together = scatter(onset);
    let apart = scatter(onset ^ (u64::from(pitch.midi()) << 40));
    loose_frames * (0.8 * together + 0.2 * apart)
}
```

Un pianista se adelanta o se atrasa más o menos lo mismo en todas las
notas de un acorde, así que cuatro quintos del retraso son comunes a
todas las notas que un canal empieza en un paso, y un quinto cambia de
nota a nota. Los números salen de `scatter`, que mezcla los bits de una
semilla con la función SplitMix64:

```rust
fn scatter(seed: u64) -> f64 {
    let mut x = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^= x >> 31;
    // The top 53 bits, as many as an `f64` holds exactly.
    (x >> 11) as f64 / (1u64 << 53) as f64
}
```

Parece aleatorio, pero la misma semilla da siempre el mismo número, así
que una canción suena igual cada vez y una exportación WAV suena como la
reproducción. `wrapping_mul` deja que la multiplicación dé la vuelta más
allá de `u64::MAX` en vez de entrar en pánico en una compilación de
depuración: aquí dar la vuelta es justo lo que se busca. `scatter`
tampoco necesita estado, lo que le viene bien al hilo de audio: ningún
generador aleatorio que guardar, bloquear o sembrar.

Una nota retrasada espera en una lista hasta su frame:

```rust
pub(crate) struct Pending {
    channel: usize,
    origin: Origin,
    pitch: Note,
    velocity: u8,
    gate: u32,
    /// The frame it starts on, counted on [`AudioThread::now`]'s clock.
    at: u64,
    /// How many frames after its step that is, for a trace.
    late: u32,
}
```

La lista se crea con sitio para 512 notas (`PENDING_NOTES`) en el hilo
de la interfaz, y el hilo de audio nunca la deja crecer:

```rust
// A full list never grows (that would allocate): the note
// then starts on time.
if late > 0 && pending.len() < pending.capacity() {
```

En cada frame se empiezan las notas cuyo `at` ha llegado, y se sacan con
`swap_remove`, que pone la última nota en el hueco en vez de desplazar
toda la lista. El orden de las notas que esperan no importa, así que la
forma más barata de quitarlas es la correcta.

¿Por qué solo tarde, nunca antes? Para empezar una nota antes, el hilo
de audio tendría que mirar el paso siguiente antes de que empiece. Tarde
entre 0 y 20 ms suena igual que antes o después 10 ms alrededor de una
cuadrícula que va ella misma 10 ms tarde, y nadie oye una canción entera
10 ms tarde.

## En archivos y en archivos MIDI

Cada una de estas funciones se guarda solo cuando se usa, así que un
proyecto que no la usa todavía lo pueden abrir versiones anteriores
([Guardar](../program/saving.md)). `format_for` escribe el formato
más antiguo que puede contener el proyecto:

| Formato | Hace falta para |
|---|---|
| 3 | Una indicación de compás distinta de 4/4, o un patrón con la suya propia |
| 5 | Marcas de compás |
| 7 | Marcas de tempo, o soltura |
| 10 | Notas fuera de la cuadrícula, como las de una canción pasada a otro compás |
| 11 | Una indicación de compás agrupada, como 2+2+3/8 |
| 16 | Swing |

Al leer un archivo, cada marca de compás y de tempo pasa por
`set_time_marker` o `set_tempo_marker`, que rechazan una que empieza
demasiado tarde o cuyos compases no cabrían en un patrón, como si se
hubiera añadido a mano.

Un Standard MIDI File guarda el tempo y el compás como *metaeventos* en
su primera pista. Un tempo se escribe como microsegundos por pulso, en
tres bytes, y un cambio recibe un evento en el paso donde cambia el
tempo tocado, así que un cambio gradual se convierte en un evento por
paso, que es como los archivos MIDI escriben un ritardando:

```rust
let tempo_event = |tempo: Tempo| {
    let micros = (60_000_000.0 / f64::from(tempo.bpm())).round() as u32;
    micros.min(0xFF_FFFF).to_be_bytes()[1..].to_vec()
};
// ...
for (step, pair) in tempos.windows(2).enumerate() {
    if pair[1] != pair[0] {
        let tick = (step as u32 + 1) * TICKS_PER_STEP;
        events.push((tick, TEMPO, tempo_event(pair[1])));
    }
}
```

`to_be_bytes` da los cuatro bytes del `u32`, el más significativo
primero («big-endian», el orden que usan los archivos MIDI), y `[1..]`
quita el primero, que siempre es 0 por debajo de `0xFF_FFFF`.

Un evento de indicación de compás lleva el número de arriba, el de abajo
como potencia de dos (`unit.ilog2()`: 2 para una negra, 3 para una
corchea), cuántos relojes MIDI dura un clic de metrónomo (24 por negra,
así que 6 por paso), y un 8 fijo. Hay uno al principio y uno en cada
cambio del compás tocado. MIDI no tiene sitio para una agrupación, así
que un compás agrupado marca cada nota (`ungrouped().first_beat()`).
Cómo se escribe un archivo MIDI entero está en
[MIDI](midi.md#escribir-un-archivo-midi).

## Cómo se prueba

Cada tipo tiene a su lado tests que se leen como las reglas que
comprueban:

- `meter.rs`: `a_steady_meter_counts_like_its_time_signature`,
  `a_marker_off_the_bar_line_leaves_a_short_bar`,
  `the_usual_grouping_is_written_plainly` (3+3/8 es 6/8),
  `a_grouping_is_saved_only_when_there_is_one`,
  `nonsense_does_not_divide_by_zero`.
- `tempo.rs`: `a_glide_moves_a_little_every_step` (el 90, 80, 70, 60 de
  más arriba), `a_glide_starts_from_where_the_last_one_got_to` (un
  ritardando cortado por un *a tempo*), `seconds_add_up_every_step`.
- `remeter.rs`: `every_note_keeps_its_place_in_the_bar`,
  `notes_between_steps_keep_their_exact_place`,
  `every_bar_lasts_as_long_as_before` (la duración de la canción en
  segundos, antes y después, con menos de un 0,2 % de diferencia),
  `a_tempo_put_in_another_meter_and_back_is_the_same_tempo`,
  `a_pattern_too_long_for_the_new_bars_is_cut`.
- `swing.rs`: `full_swing_plays_the_eighths_as_triplets`,
  `half_swing_is_seven_to_five`, `a_strum_on_the_offbeat_moves_as_a_whole`,
  `notes_off_the_eighths_stay`.
- `engine.rs` toca unas pocas notas y comprueba dónde empieza el sonido,
  frame a frame: `a_song_plays_each_step_at_its_own_tempo` (dos pasos a
  120 BPM, luego dos a 60, que duran el doble) y
  `loose_timing_starts_notes_a_little_late` (cada nota dentro de los 960
  frames, 20 ms a 48 kHz, después de su paso; no todas con el mismo
  retraso; y lo mismo cada vez).

Un test recorre la aplicación entera:
`styles_tried_one_after_another_keep_the_songs_tempo` abre El Choclo y
lo pasa por siete estilos y compases (entre ellos un tango vals en 3/4 y
el cuadro Compás en 7/8), en modo patrón, en modo canción parado y en
modo canción sonando. Después de cada uno, el tempo tiene que seguir
igual cuando el compás no cambia, y ser el de la canción cada vez que
vuelve a 4/4. Luego toca el principio de la canción con la [traza de
reproducción](trace.md) y comprueba que cada nota de la
melodía empieza y dura lo mismo que antes, con menos de 2 ms de
diferencia. Para que sea rápido en una compilación de depuración, solo
toca doce segundos, a 8000 muestras por segundo: de sobra para medir
notas.
