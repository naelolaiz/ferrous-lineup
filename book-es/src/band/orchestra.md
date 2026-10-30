# Una orquesta para música que cambia de compás

Archivos: [`crates/lineup-core/src/library/rhythm.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/rhythm.rs),
[`crates/lineup-core/src/library/style.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/style.rs),
[`crates/lineup-core/src/library/apply.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/apply.rs),
[`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-core/src/band/following.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/following.rs),
[`crates/lineup-app/src/app/library.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/library.rs)
y [`crates/lineup-app/src/library_browser.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/library_browser.rs)

Cada estilo de la Biblioteca está escrito en un solo compás, y
aplicarlo pone la canción en ese compás (ver [Pasar una canción a otro
compás](../sound/tempo.md#pasar-una-canción-a-otro-compás)). Eso
está bien para una canción pop, y está mal para La consagración de la
primavera. El **Juego del rapto** cambia de compás 32 veces en 67
compases: un estilo en 4/4 lo convertiría todo en 4/4, y una banda que
silencia los acordes y el bajo propios de la canción silenciaría a
Stravinsky.

El estilo **Orquesta** está hecho para esa música. Toca cada compás en
la métrica de ese compás, conserva los compases y el tempo de la
canción, y toca junto a la partitura en lugar de silenciarla. Es el
estilo que encaja con once demostraciones hechas a partir de partituras
de orquesta (ver [De una interpretación MIDI o una partitura a una
demostración](../songs/from-midi-and-scores.md)): Promenade, el vals
en cinco de la *Sinfonía Patética*, Boléro, La Mer, los Augurios
primaverales, el Juego del rapto y cinco escenas de Petrushka.

## Probarlo

1. Abre **Archivo > Demostraciones**, escribe `augurios` en el cuadro
   de búsqueda y pulsa Intro: **La consagración de la primavera,
   Augurios primaverales** se abre y suena. Detenla.
2. Mira la pestaña **Biblioteca**. **Orquesta (cualquier métrica)**
   está iluminado en el grupo **Clásica y de cine**, como lo está
   siempre un estilo que encaja. «Cualquier métrica» dice que el estilo
   no está en ningún compás propio.

   ![La Library con el grupo Classical and film abierto y «Orchestra (any meter)» iluminado, entre «Neapolitan» y «Picardy third».](../images/step-9-orchestra-library.png)

3. Hazle clic. Bajo **Acompañamiento**, **Batería**, **Partes de
   acordes y bajo** y **Voces / instrumentos** están marcadas; bajo
   **Armonía**, **Progresión de acordes** no lo está. Bajo **Tiempo** no hay
   casillas Compás ni Tempo, solo una línea que dice que el preajuste
   conserva los compases y el tempo de la canción.

   ![El popup de Orchestra: Drums, Chord and bass parts y Voices / instruments marcados, Chord progression sin marcar, y bajo Timing: «Keeps the song's time signatures and tempo: this preset plays every bar in that bar's meter.»](../images/step-9-orchestra-popup.png)

4. Haz clic en **Aplicar**. Aparecen cuatro pistas bajo la canción:
   **Piano**, **Bajo**, **Batería** y **Cuerdas**. La pista Canción
   sigue encendida, el cuadro Compás sigue diciendo 2/4 y el tempo
   sigue diciendo 112.
5. Pulsa Reproducir. Las cuerdas mantienen cada acorde bajo las cuerdas
   que martillean de Stravinsky. Los cornos, los timbales y el bombo se
   quedan callados hasta el compás 3, y luego golpean *con* los acentos
   de la partitura: fuera del pulso dos veces en el compás 3, fuera del
   pulso en el compás 5, en el pulso en los compases 6 y 7, fuera de
   nuevo en el 8. En los compases tranquilos 9 a 12 descansan, y las
   cuerdas tocan suave.

   ![The Augurs of Spring después de Apply: bajo la pista Song, las pistas Piano, Bass y Drums tienen trazos cortos en los compases 3 y 5 a 8, ninguno en los compases 9 a 13, y más desde el compás 14; la pista Strings mantiene acordes todo el rato.](../images/step-9-orchestra-augurs.png)

Las pistas conservan los nombres de los músicos de la banda, pero en
este estilo la pista Piano toca cornos franceses y la pista Bajo
timbales; el **Mezclador** muestra los instrumentos.

Ahora haz lo mismo con **La consagración de la primavera, Juego del
rapto**, que empieza en 9/8 a la negra con puntillo = 132 de Stravinsky
(el cuadro de tempo muestra 198, la misma velocidad contada en negras)
y pasa por 4/8, 5/8, 12/8, 6/8, 7/8, 3/4, 2/8 y más. Las pistas se
cortan en clips cortos, uno por cada tramo de compases en una misma
métrica, y cada clip toca sus compases como se siente esa métrica.

![Ritual of Abduction después de Apply: la pista Song arriba, y debajo las pistas Piano, Bass, Drums y Strings. Sobre los compases 9 a 11 la regla muestra 4/8, 5/8 y 9/8, y los clips de la banda están cortados en cada cambio.](../images/step-9-orchestra-ritual.png)

La orquesta toca los acordes que la aplicación reconoció en la
partitura, los de la hoja de acordes (**F5**). Donde no se reconoció
ningún acorde, solo toca el bombo. Si marcas **Progresión de acordes**
en el popup, toca sus propios acordes en su lugar, I y ♭VII del modo
mixolidio, como la progresión de un estilo siempre reemplaza los
acordes de la canción.

### Solo las voces

Desmarca **Batería** y **Partes de acordes y bajo** antes de
**Aplicar**, de modo que solo **Voces / instrumentos** quede marcado, y
entran las mismas cuatro pistas. Marca solo la batería, o solo las
partes de acordes y bajo, y también entran: una orquesta que sigue a la
canción siempre trae sus propios músicos y sus propios patrones, en
pistas propias, y los canales de la partitura conservan sus
instrumentos y sus notas.

Los demás estilos tratan una sola casilla de otra manera (ver [Qué
partes tomar](library.md#qué-partes-tomar)). Elegido con sus partes de
acordes y bajo y sus voces pero sin su batería, un estilo trae su banda
en pistas propias, y la batería propia de la canción sigue sonando.
Elegido solo con sus voces, no trae banda: sus sonidos van a los
músicos propios de la canción. Elegido solo con su batería o solo con
sus partes de acordes y bajo, se abre en un patrón de prueba para que
lo escuches primero.

## La música

### Pulsos sentidos

Un compás de 9/8 tiene nueve corcheas, pero nadie cuenta nueve. Las
corcheas vienen en tres grupos de tres, y se sienten tres pulsos, cada
uno de una negra con puntillo. Un compás de 5/8 tiene cinco corcheas en
dos pulsos, uno corto de dos y uno largo de tres. Un compás de 2/4 a
tempo rápido pasa como un solo pulso. Estos son los pulsos *sentidos*,
y `TimeSignature::felt_groups` los da para cualquier compás (ver
[Pulsos de distinta
longitud](../sound/tempo.md#pulsos-de-distinta-longitud)).

Un director marca los pulsos sentidos, no cada corchea: en el Juego del
rapto, tres pulsos para un compás de 9/8, dos para 5/8, uno para 2/8.
En música cuyo compás cambia a cada rato, eso es lo que mantiene unida
a una orquesta, y es lo que toca el estilo Orquesta:

| Compás | Se siente como | Los cornos y los timbales golpean en la corchea (o negra) |
|---|---|---|
| 2/8, 3/8, 2/4, 3/4 | un pulso | 1 |
| 4/8 | 2 + 2 | 1 y 3 |
| 5/8 | 2 + 3 | 1 y 3 |
| 6/8 | 3 + 3 | 1 y 4 |
| 7/8 | 2 + 2 + 3 | 1, 3 y 5 |
| 9/8 | 3 + 3 + 3 | 1, 4 y 7 |
| 12/8 | 3 + 3 + 3 + 3 | 1, 4, 7 y 10 |
| 4/4 | 2 + 2 | negras 1 y 3 |

Los cornos golpean el acorde en cada uno de esos pulsos y los timbales
tocan su fundamental; cuando un acorde cambia entre dos pulsos, tocan
el nuevo donde empieza. La línea de batería propia del estilo golpea
fuerte el primer pulso del compás y suave los demás, pero junto a una
partitura el bombo conserva solo el primer pulso, y solo donde la
música suena fuerte: un bombo en cada pulso de un compás tranquilo
taparía la partitura.

### Tocar junto a la partitura

Una banda toca el mismo groove compás tras compás. Una orquesta que se
une a una sinfonía no puede: la música crece y se apaga, se detiene, y
pone sus acentos donde el compositor los quiere. Los Augurios
primaverales son el caso famoso. Las cuerdas martillean el mismo acorde
en cada corchea, todas igual de fuerte, y ocho cornos tiran acentos
sobre algunas de esas corcheas en lugares que nadie puede predecir: la
segunda y la cuarta corchea de un compás, la segunda de otro, la
primera de los dos siguientes. El impacto de la pieza es que los
acentos nunca se asientan en un patrón. Un bombo en cada primer pulso
los convertiría en uno.

Así que la Orquesta escucha la partitura antes de tocar, y cambia lo
que tocarían sus músicos:

- **Dinámica.** Toca tan fuerte como la partitura, compás a compás: más
  suave en un compás *piano*, más fuerte en uno *forte*. Donde la
  partitura calla, calla.
- **Acentos.** Donde la partitura acentúa un acorde, los cornos lo
  golpean, los timbales golpean su bajo y el bombo golpea con ellos,
  tan fuerte como la partitura.
- **Pasajes de acentos.** Donde los acentos vienen juntos, en al menos
  dos compases de nueve (un compás y los cuatro a cada lado), los
  cornos, los timbales y el bombo tocan *solo* los acentos, para que el
  patrón irregular siga siendo el del compositor.
- En todo lo demás se atienen a los pulsos sentidos, y las cuerdas
  mantienen los acordes.

¿Qué cuenta como acento? Un acorde que pasa las dos primeras pruebas de
abajo, o una nota que pasa la tercera, tal como tú los oirías:

1. Toda la orquesta lo toca mucho más fuerte que el resto de su compás,
   *y* que el mismo lugar en los compases de alrededor: casi el doble
   de fuerte. Un acorde tutti después de una textura fina.
2. Un instrumento en él toca una nota corta, entre silencios: nada una
   corchea antes, nada una corchea después, y silencio cuando acaba la
   nota. Los cornos de los Augurios hacen eso. Una melodía de negras no
   (sus notas se tocan), y tampoco un acorde mantenido cuatro
   compases, que solo entra. Por eso el Villancico de las campanas,
   cuyo coro mantiene un acorde bajo la melodía, no tiene acentos, y su
   orquesta toca un golpe por compás desde el principio.
3. La partitura lo marca ella misma, un sforzando o un acento fuerte:
   muy fuerte (velocidad 120 o más) y una quinta parte más fuerte que
   el término medio de su parte en ese compás.

Un compás donde la mayoría de las corcheas contarían como acentos no
tiene ninguno: la música simplemente subió de volumen.

### Por qué cornos y timbales

El estilo necesita una parte que marque la armonía en el pulso y una
parte que marque el bajo, los trabajos que hacen un piano y un bajo en
una banda. En una orquesta esos trabajos son desde hace mucho de los
cornos y los timbales: en una sinfonía de Haydn o de Beethoven, los
acordes fuertes que cierran una frase son los cornos y las trompetas
tocando la armonía mientras los timbales golpean la tónica o la
dominante debajo. Las cuerdas mantienen cada acorde mientras dura, como
hace el colchón de cuerdas de una banda (ver [Una banda más
grande](band.md#una-banda-más-grande-guitarra-cuerdas-y-órgano)).

La orquesta toca *junto* con la partitura. En una demostración pop, la
banda silencia los acordes y el bajo propios de la canción, porque dos
pianos tocando los mismos acordes con dos disposiciones distintas
suenan a error. En La consagración, las partes de la canción *son* la
pieza: silenciarlas dejaría solo a la banda. Así que la Orquesta entra
por encima, y cada canal de la partitura conserva su propio silencio.

## Cómo funciona

### Un ritmo que sigue el compás

Un ritmo de la biblioteca está escrito en un solo compás: sus líneas de
batería son compases de pasos, `X... .... o... ....` para un compás de
4/4, donde `X` es un golpe fuerte, `o` uno suave y `.` silencio. La
banda extiende una línea compás tras compás, y en una canción con
marcadores de compás escribe cada tramo de compases en una misma
métrica como un patrón propio.

El ritmo de la Orquesta tiene una línea, ese compás de 4/4 para el
bombo, y una bandera en `Rhythm`:

```rust
pub struct Rhythm {
    // ...
    pub follows_the_meter: bool,
}
```

Los ritmos se construyen con un `const fn rhythm(...)`, y un método
constructor pone la bandera, igual que `swung` pone el swing:

```rust
const fn following_the_meter(self) -> Rhythm {
    Rhythm {
        follows_the_meter: true,
        ..self
    }
}
```

`..self` es la *struct update syntax*: cada campo no nombrado toma su
valor de `self`. Un `const fn` puede hacerlo porque solo mueve valores,
así que toda la tabla de ritmos se sigue construyendo en tiempo de
compilación.

Las notas de batería de un compás salen de
`Rhythm::notes_in(drum, time, length)`. Para un ritmo corriente
extiende la línea compás a compás, como hace `Rhythm::notes`. Para uno
que sigue el compás, primero escribe la línea de nuevo para `time`:

```rust
fn in_meter(line: &str, own: TimeSignature, time: TimeSignature) -> String {
    let steps: Vec<char> = line
        .split('|')
        .next()
        .unwrap_or_default()
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect();
    let first = steps.first().copied().unwrap_or('.');
    let other = felt_starts(own)
        .get(1)
        .and_then(|&start| steps.get(start).copied())
        .unwrap_or('.');
    let mut bar = vec!['.'; time.steps_per_bar()];
    for (index, start) in felt_starts(time).into_iter().enumerate() {
        if let Some(step) = bar.get_mut(start) {
            *step = if index == 0 { first } else { other };
        }
    }
    bar.into_iter().collect()
}
```

Lee el primer compás de la línea (una línea puede tener varios,
separados por `|`), se queda con el golpe de su primer paso, `first`,
y con el del comienzo de su segundo pulso sentido, `other`, y los
vuelve a poner en los pulsos sentidos de `time`. `felt_starts(time)`
convierte los grupos sentidos en pasos: un grupo de tres corcheas son
tres veces `time.unit_steps()`, los pasos que dura una corchea. Para
9/8 eso da los pasos 0, 6 y 12, y la `X` y la `o` del compás de 4/4
caen ahí: `X.....o.....o.....`.

Un ritmo que sigue el compás no tiene compás propio, así que
`Rhythm::for_time`, que elige un ritmo para el compás nuevo de una
canción, nunca lo elige.

### Un estilo que sigue a la canción

Un estilo sabe si sigue a la canción preguntándole a su ritmo:

```rust
pub fn follows_the_song(&self) -> bool {
    super::Rhythm::named(self.rhythm).is_some_and(|rhythm| rhythm.follows_the_meter)
}
```

`Option::is_some_and` es `true` solo cuando hay un valor y el closure
dice que sí para él, así que un estilo cuyo ritmo falta no sigue nada.

Aplicar un estilo puede poner la canción en el compás y el tempo del
estilo, cuando esas casillas están marcadas. Las partes que se aplican,
`StyleParts`, tienen un paso que quita las dos cuando el estilo sigue a
la canción:

```rust
pub fn following(mut self, follows: bool) -> Self {
    if follows {
        self.time_signature = false;
        self.tempo = false;
    }
    self
}
```

**Aplicar** en la Biblioteca pasa por ahí, y `Project::set_band_style`
pregunta `follows_the_song` antes de poner la canción en el compás del
estilo, así que ninguno de los dos cambia el compás de La consagración.
El menú **Estilo** de una sección en la hoja de acordes, que solo
ofrece estilos en el compás propio del compás elegido, ofrece la
Orquesta en cualquier métrica. En la aplicación, la misma bandera
cambia las casillas de Tiempo por la línea que viste en el popup, y la
Biblioteca añade «(cualquier métrica)» al nombre del estilo.

### Unirse a la canción

**Aplicar** en la Biblioteca termina en `band_plays_parts`, en
`crates/lineup-app/src/app/library.rs`. Primero decide si entra una
banda:

```rust
let follows = match action {
    LibraryAction::Style(style) | LibraryAction::StyleInSong(style) => {
        style.follows_the_song()
    }
    LibraryAction::Rhythm(rhythm) | LibraryAction::RhythmInSong(rhythm) => {
        rhythm.follows_the_meter
    }
    _ => false,
};
let has_music = !self.project.playlist().chord_markers().is_empty()
    || self.project.playlist().clips().iter().any(|clip| {
        self.project
            .pattern(clip.pattern)
            .is_some_and(|pattern| !pattern.is_empty())
    });
// A beat made up in the library has no chord and bass parts of its
// own: it brings no band, only its drums.
let made_up = match action {
    LibraryAction::Rhythm(rhythm) | LibraryAction::RhythmInSong(rhythm) => {
        BandStyle::named(rhythm.name).is_none()
    }
    _ => false,
};
let joins = if follows {
    parts.rhythm || parts.matching_style || parts.instruments
} else {
    parts.instruments && parts.matching_style && has_music && !made_up
};
if self.project.band().is_none() && !joins {
    return false;
}
```

`match` con `|` entre dos patrones ejecuta el mismo brazo para los dos,
y `_ => false` cubre cualquier otro tipo de elemento de la Biblioteca.
`has_music` es verdadero cuando la canción tiene acordes en la hoja de
acordes, o notas cuyos acordes la banda puede oír; `Iterator::any` se
detiene en el primer clip cuyo patrón tiene una nota. Una banda toca
las partes de acordes y bajo de un estilo, así que solo entra cuando se
toman, con las voces en que las toca; la batería sola es cosa de los
canales de batería de la propia canción. Cuando
`band_plays_parts` devuelve `false`, quien la llamó sigue por el camino
de siempre: abre el estilo en un patrón de prueba o, mientras la
canción suena, lo aplica a los patrones propios de la canción.

La misma regla está en `Project::apply_band_parts`, en `band.rs`, que
construye la banda, con una línea más: una banda que entró sin la
batería del estilo se la deja a la canción.

```rust
if !parts.rhythm && !follows {
    band.drums = BandDrums::Song;
}
```

### Escuchar la partitura

`crates/lineup-core/src/band/following.rs` es un módulo de la banda.
`band.rs` lo declara con `mod following;`, y Rust lo encuentra en
`band/following.rs`, una carpeta con el nombre del archivo padre. Su
tipo es `pub(super)`: visible para `band.rs` y para nada más.

`Following::hear(project, bars)` recoge lo que toca la canción. Recorre
los clips que se oyen, salta los patrones propios de la banda y anota
el comienzo de cada nota en un `BTreeMap<usize, Vec<(usize, u8, usize)>>`:
de paso a una lista de (canal, velocidad, duración). Un `BTreeMap`
mantiene sus claves en orden, así que `struck.range(bar.start..bar.end())`
da las notas de un compás, en orden de tiempo, sin mirar las demás.

Devuelve un `Option<Following>`: `None` cuando la canción no toca
ninguna nota, una hoja de acordes sin nada más. Entonces no hay
partitura que seguir, y la orquesta toca como lo escribió su estilo. Lo
mismo vale para los compases antes de la primera nota de la partitura
y después de que acabe la última:

```rust
let first = *struck.keys().next()?;
let last = struck
    .iter()
    .flat_map(|(&at, notes)| notes.iter().map(move |&(_, _, length)| at + length.max(1)))
    .max()?;
```

El `?` después de `next()` y de `max()` devuelve `None` desde `hear`
cuando no hay nada, igual que pasa un error hacia arriba desde una
función que devuelve un `Result`. `flat_map` convierte la lista de
notas de cada paso en los pasos donde terminan, un solo iterador largo,
y `max` toma el último. El `move` delante del closure interior copia
`at` dentro de él: el iterador interior se usa después de que el
closure exterior haya vuelto, cuando su propio `at` ya no existe.

Qué tan fuerte suena un compás: la nota más fuerte en cada paso que
toca, y la del medio de esas, dividida entre 100 (mezzoforte en las
demostraciones).

```rust
fn middle(mut values: Vec<u32>) -> u32 {
    if values.is_empty() {
        return 0;
    }
    values.sort_unstable();
    values[(values.len() - 1) / 2]
}
```

El valor del medio (la *mediana*) en lugar del promedio, porque un solo
acento fuerte no debe volver fuerte todo el compás.

Los umbrales se escriben como fracciones, para que toda la aritmética
quede en números enteros:

```rust
const TUTTI: (u32, u32) = (9, 5);
// ...
let (times, by) = TUTTI;
let stands_out =
    strength(at) * by >= in_bar * times && strength(at) * by >= elsewhere * times;
```

`strength(at) * 5 >= in_bar * 9` es `strength(at) >= 1,8 * in_bar`
sin un número de coma flotante a la vista. `elsewhere` es el mismo
lugar en los dos compases anteriores y en los dos siguientes, el más
suave de los dos lados; el primer compás no tiene compases antes, así
que compara solo con los de después:

```rust
let elsewhere = match (same_place(before.clone()), same_place(after.clone())) {
    (Some(before), Some(after)) => before.min(after),
    (one, other) => one.or(other).unwrap_or(0),
};
```

Hacer `match` sobre un par de `Option` cubre los cuatro casos en dos
brazos: el primero toma los dos valores, el segundo el que exista, o 0.

Para saber si un canal golpea «solo», `hear` guarda en orden los
comienzos de cada canal y busca el lugar de la nota entre ellos con
`partition_point`, una búsqueda binaria: el número de comienzos antes
de `at`.

```rust
let place = steps.partition_point(|&step| step < at);
let before = place
    .checked_sub(1)
    .is_none_or(|before| steps[before] + ALONE < at);
let after = steps
    .get(place + 1)
    .is_none_or(|&after| after > at + ALONE.max(length));
```

`checked_sub(1)` es `None` para la primera nota en lugar de bajar de
cero, y `Option::is_none_or` es `true` para `None`: no haber nota antes
de la primera vale tanto como un silencio.

`Following::play_along(player, notes, ...)` toma luego lo que escribió
un músico del estilo y lo cambia: se quitan las notas de un compás en
silencio, se quitan las notas de los cornos y los timbales en un pasaje
de acentos, el bombo conserva solo los primeros pulsos de los compases
fuertes, y cada velocidad se escala según lo fuerte que suena el
compás. Después añade los golpes: en cada acento, los cornos tocan las
notas del acorde que suena ahí, los timbales su bajo y el bombo un
golpe, tan fuerte como el acento de la partitura.

`write_band`, que escribe la parte de cada músico, lo llama solo para
un estilo que sigue a la canción:

```rust
let mut part = self.band_notes(&playing, player, &score);
if let Some(following) = &following
    && playing.style.follows_the_song()
{
    part = self.played_along(following, &playing, player, part, &score);
}
```

`if let ... && ...` es una *let chain*: el `if` se ejecuta solo cuando
el patrón coincide y la condición después de `&&` se cumple.

### Muchos patrones cortos

La banda escribe un patrón para cada músico y cada tramo de compases en
una misma métrica. Dos tramos pueden compartir un patrón cuando tocan
las mismas notas, como la batería de dos tramos de 9/8, pero los
acordes cambian de un tramo a otro. El Juego del rapto tiene 33
tramos, y su orquesta escribe 92 patrones además de los 34 de la
partitura. Un proyecto admite hasta 999 patrones (`MAX_PATTERNS` en
`project.rs`), y la ventana dodecafónica lee la misma constante cuando
dice que una serie necesitaría más patrones de los que caben en un
proyecto.

## Cómo se prueba

Los tests de `band.rs`:

- `an_orchestra_plays_every_bar_of_the_rite_of_spring_in_its_own_meter`
  comprueba que el Juego del rapto conserva cada cambio de compás, sus
  cambios de tempo y el silencio de cada canal, que los músicos son un
  corno francés, timbales y un conjunto de cuerdas, que el bombo toca
  solo con los acentos de la partitura o en el primer pulso de un
  compás, y no en todos los compases, y que los cornos y los timbales
  tocan en pulsos sentidos, cambios de acorde o acentos.
- `the_orchestra_strikes_with_the_accents_of_the_augurs_of_spring`
  copia solo las voces y comprueba que los canales y las notas de la
  canción quedan como estaban, que los cornos, los timbales y el bombo
  golpean en los pasos 18, 22, 34, 40, 48 y 58 de los primeros ocho
  compases y en ningún otro, y que en los compases 9 a 12 descansan y
  las cuerdas tocan por debajo de la velocidad 60.
- `a_tune_or_a_held_chord_is_not_an_accent` no oye ningún acento en el
  Villancico de las campanas, y comprueba que los cornos tocan el
  primer pulso de cada uno de sus primeros ocho compases.
- `with_no_score_to_follow_the_orchestra_plays_as_written` la aplica a
  una hoja de acordes sin notas, y comprueba que todos los músicos
  tocan.
- `a_band_brought_without_drums_leaves_them_to_the_song` copia las
  partes de acordes y bajo y las voces de un estilo que no sigue a la
  canción, Pop emotivo, sobre el tango El Choclo, sin su batería, y
  comprueba que su banda entra sin batería propia mientras la canción
  conserva sus sonidos, sus notas y su batería; tomar la batería
  después la trae en lugar de la de la canción.
- `copying_only_the_voices_brings_no_band` pasa solo las voces de Pop
  emotivo a `apply_band_parts` sobre El Choclo, y comprueba que deja la
  canción como estaba: las voces solas van a los canales propios de la
  canción, por el camino de la Biblioteca, no a través de una banda.
- `picking_the_orchestra_keeps_the_meters_of_the_song` hace de la
  Orquesta el estilo de la banda en los Augurios, con
  `set_band_style`, y después de **Ritmo pop**: la primera conserva el
  2/4, el segundo pone la canción en 4/4.

En `rhythm.rs`,
`a_rhythm_that_follows_the_meter_plays_each_bar_in_its_own` comprueba
los golpes que escribe `notes_in` para 2/4, 3/4, 9/8, 5/8, 7/8 y
3+2+2/8, y que `for_time` nunca elige la Orquesta.

En la aplicación,
`copying_only_the_voices_or_the_patterns_brings_the_band_and_leaves_the_song_alone`
en `ux_tests.rs` recorre a clics el popup de la Biblioteca para los
Augurios, el Juego del rapto, La habitación de Petrushka, el Villancico
de las campanas y un tango, con una sola casilla marcada, y comprueba
que la banda entró y que los canales y las notas de la canción no
cambiaron.

Para oírlo sin escuchar, guarda los Augurios después de **Aplicar** y
lee su [traza de reproducción](../sound/trace.md):

```sh
ferrous-lineup --trace augurs.lineup --out augurs.tsv
awk -F'\t' '$11 == "Kick" && $3 <= 16 { print $3, $4, $5, $16 }' augurs.tsv
```

La columna 3 es el compás, la 4 el pulso, la 5 cuánto se ha avanzado
dentro del pulso en pasos (una negra tiene cuatro, así que 2 es la
corchea después del pulso) y la 16 la velocidad:

```text
3 1 2.000 127
3 2 2.000 127
5 1 2.000 127
6 1 0.000 127
7 1 0.000 127
8 1 2.000 127
14 1 2.000 127
14 2 2.000 127
16 1 2.000 127
```

Fuera del pulso dos veces en el compás 3, fuera de él en el compás 5,
en él en los compases 6 y 7, fuera en el compás 8: los acentos de los
cornos tal como los escribió Stravinsky, sin nada en los compases
tranquilos entre el 8 y el 14.
