# Cómo se escribe una demostración

Archivos: [`crates/lineup-core/src/library/song.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song.rs),
[`crates/lineup-core/src/library/song/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates/lineup-core/src/library/song),
[`crates/lineup-core/src/library/chart.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/chart.rs),
[`crates/lineup-core/src/library/demo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/demo.rs),
[`crates/lineup-core/src/library/interpret.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/interpret.rs)
y [`crates/lineup-app/src/app/demo_menu.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/demo_menu.rs)

**Archivo > Demostraciones** abre 343 canciones, de canciones infantiles
a La consagración de la primavera, y una demostración corta de cada
estilo, de cada estilo dodecafónico y de cada estilo de micromodos.
Haces clic en una y se abre, con sus acordes en la hoja de acordes, y
suena. Qué escuchar en ellas está en la [guía de
escucha](https://naelolaiz.github.io/ferrous-lineup/es/manual/finding-a-demo.html)
del manual. Este capítulo trata de qué *es* una demostración.

Una demostración de canción no es un archivo. Es una constante de
Rust, compilada dentro del programa: la melodía y los acordes escritos
como texto, como los escribe una partitura guía, y unos pocos
campos que dicen cómo se toca la canción. `song_demo` convierte la
constante en un `Project` corriente cada vez que la abres. Eso tiene
tres ventajas:

- **Los tests leen todas las canciones.** Un compás con una
  semicorchea de más, un acorde que no se puede leer, una forma que
  nombra una parte que no existe: cada cosa hace fallar `cargo test`
  antes de que nadie la oiga.
- **Una demostración es igual en todas partes.** No puede faltar,
  estar dañada ni quedar vieja; cada versión nueva de la aplicación
  trae su propia copia.
- **Lo que necesitan las canciones, lo aprende la biblioteca.** Una
  pieza que cambia de tonalidad, de compás o de tempo, una ronda, una
  orquesta que pasa su melodía de la flauta al clarinete: cada una se
  convirtió en una función de `Part`, y cualquier canción puede usarla.

Por lo mismo, una demostración es de solo lectura: no hay archivo donde
escribir tus cambios, así que **Guardar** pide uno nuevo (ver [La barra
de título](../program/saving.md#la-barra-de-título)).

## Una canción es una constante

Esta es una canción entera, la ronda Frère Jacques:

```rust
pub(super) const FRERE_JACQUES: Song = Song {
    name: "Frère Jacques",
    origin: "French round, 18th century",
    feel: "Round",
    genre: Genre::Folk,
    tonic: C,
    mode: Major,
    time: COMMON,
    tempo: 104.0,
    comping: Comping::ArpUpDown,
    bass: Bass::RootFifth,
    rhythm: "Folk",
    band: SONG_AND_GUITAR,
    style: "Folk",
    parts: &[
        Part::new("Alone", FRERE_JACQUES_TUNE, FRERE_JACQUES_CHORDS),
        Part::new("Round", FRERE_JACQUES_TUNE, FRERE_JACQUES_CHORDS).with_voices(&[
            (1, 0),
            (2, 2),
            (3, 4),
            (4, 6),
        ]),
        // ...
    ],
    form: &["Alone", "Round", "Round again", "Last voices"],
    drums_from: 0,
    about: "A round: four voices sing the same tune, each two bars after the last. It works \
            because every two bars of the tune fit the same chord.",
};
```

La melodía y los acordes son constantes aparte, encima, porque las
usan cuatro partes:

```rust
const FRERE_JACQUES_TUNE: &str = "C5:4 D5 E5 C5 | C5 D5 E5 C5 | E5 F5 G5:8 | E5:4 F5 G5:8 | \
     G5:2 A5 G5 F5 E5:4 C5 | G5:2 A5 G5 F5 E5:4 C5 | C5 G4 C5:8 | C5:4 G4 C5:8";
const FRERE_JACQUES_CHORDS: &str = "I | I | I | I | I | I | I | I";
```

Los campos se leen de arriba abajo como la cabecera de una partitura
guía: qué es y de dónde viene, su tonalidad, su compás y su
tempo; luego cómo la viste la banda, una manera de acompañar y una
línea de bajo de la biblioteca (ver [El comping: cómo se tocan los
acordes](../band/library.md#el-comping-cómo-se-tocan-los-acordes)),
un ritmo por su nombre, los instrumentos (`band`) y el estilo al que
más se parece, que la Biblioteca muestra seleccionado mientras la
demostración está abierta. Después, la música: las **partes**
(`parts`), cada una una sección como una estrofa o un estribillo, y la
**forma** (`form`), el orden en que se tocan, por nombre. Una parte que
se toca tres veces se escribe una vez y se nombra tres. `drums_from` es
la entrada de la forma donde entra la batería: 0 para el principio, 1
cuando la canción abre con una anacrusa o una introducción que la
batería deja tranquila.

### Un archivo por género

343 canciones harían un archivo de decenas de miles de líneas. Así que
`song.rs` guarda los tipos, las constantes compartidas, la lista y los
tests, y las canciones viven al lado, un archivo por género:

```text
library/
  song.rs          Song, Part, Band, SONGS, los tests
  song/
    balkan.rs
    blues.rs
    classical.rs
    ...
    tango_played.rs
```

`mod tango;` dentro de `song.rs` hace que Rust busque `song/tango.rs`:
cada archivo de género es un **módulo hijo** de `song`. Una canción es
`pub(super) const`, visible para su padre y nada más: fuera de la
biblioteca nadie necesita el nombre `HABANERA`, solo la canción, que
encuentra en la lista.

Cada archivo de género empieza con una línea:

```rust
use super::*;
```

`super` es el módulo padre y `*` trae todos los nombres que este ve,
también los privados: en Rust un elemento privado de un módulo es
visible para los hijos de ese módulo. Así una canción puede usar
`COMMON`, `SIX_EIGHT`, las bandas y cualquier otra constante que
`song.rs` se guarda para sí. Dos de los `use` del propio padre son los
que más importan:

```rust
use PitchClass::*;
use ScaleKind::*;
```

Permiten que una canción escriba `tonic: A` y `mode: Dorian` en lugar
de `PitchClass::A` y `ScaleKind::Dorian`. `Genre` no se importa así, a
propósito: `Genre::Blues` y `ScaleKind::Blues` serían los dos `Blues`,
y dos importaciones con `*` que traen el mismo nombre son un error en
cuanto se usa ese nombre. Así que un género se escribe siempre
completo, `genre: Genre::Blues`, y `mode: Blues` es la escala.

### La lista

```rust
pub static SONGS: &[Song] = &[
    pop::AULD_LANG_SYNE,
    pop::LONDON_BRIDGE,
    pop::OLD_MACDONALD,
```

Cada canción aparece en la lista por su nombre. Rust no puede unir
slices mientras compila, así que los archivos de género no pueden traer
cada uno su propia lista; nombrar cada `const` es la manera sencilla.
Una `const` se copia allí donde se usa, así que la lista guarda las
canciones mismas, y como es una `static`, tiene un único lugar en
memoria durante toda la ejecución: el menú y la Biblioteca pueden
repartir referencias `&'static Song` a ella. La lista va agrupada por
género, en el orden de `Genre::ALL`, y un test lo comprueba con
`windows(2)`, que da cada par de vecinos:

```rust
SONGS.windows(2).all(|pair| pair[0].genre <= pair[1].genre),
```

`<=` funciona con géneros porque `Genre` deriva `PartialOrd`, que
ordena las variantes de un enum tal como están escritas. El menú ordena
las canciones por título por su cuenta (ver [El menú
Demostraciones](#el-menú-demostraciones)), así que una canción nueva va
al final del bloque de su género, y dos cambios que agregan canciones a
la vez no se pelean por las mismas líneas.

### De dónde salen las notas

Cada canción lleva `origin`, quién la escribió y cuándo, y encima un
comentario `// Source:` que nombra la partitura, el cifrado, el grabado
o la grabación de donde se tomaron sus notas, y qué se simplificó. El
comentario de la Promenade, por ejemplo, nombra el grabado de LilyPond
del que se leyó y explica cómo se eligió un acorde para cada pulso.
Casi todas las canciones son tan viejas que son de todos; las pocas que
todavía tienen derechos de autor en algún lugar lo dicen en su
comentario. Veinticinco son melodías nuevas para géneros con pocas
libres, escritas como esos géneros escriben las suyas, y su `origin`
dice «Written for Ferrous Lineup». Cómo una interpretación o una
partitura se convierte en una constante es el capítulo siguiente, [De
una interpretación MIDI o una partitura a una
demostración](from-midi-and-scores.md).

## La melodía, como texto

Una melodía se escribe en una notación pequeña, que lee
`chart::parse_melody`:

| Texto | Significa |
|---|---|
| `C4`, `F#5`, `Bb3` | una nota: su letra, `#` o `b`, y su octava (`C4` es el do central) |
| `:4` | su duración en pasos (4 es una negra); sin ella, tan larga como la nota anterior |
| `:1/2`, `:1/4` | más corta que un paso: una fusa, una semifusa, para adornos y escalas rápidas |
| `-:8` | un silencio, aquí medio compás de 4/4 |
| `_:8` | una ligadura: la nota anterior sigue, también a través de una barra de compás |
| `\|` | una barra de compás: no cambia nada, pero cada compás se comprueba |
| `A3+C#4+E4:4` | un acorde, sus notas a la vez |
| `&` | otra voz, desde el principio, al mismo tiempo: `E5:4 D5 C5 B4 & C3:16` |
| `(3` | las tres notas o silencios siguientes son un tresillo, tres en el tiempo de dos |
| `!p`, `!f`, `!sfz` | una dinámica, de `!pp` a `!ff`, o un sforzando en la nota siguiente |
| `!<`, `!>` | un crescendo o un diminuendo, hasta la dinámica siguiente |
| `>A4:2`, `D5:2.` | un acento delante de una nota; un punto de staccato después |

Las marcas no ocupan tiempo. Las notas de un tresillo empiezan entre los
pasos de la cuadrícula, así que se colocan al tick (ver [Notas entre
pasos](../sound/midi.md#notas-entre-pasos)). La estrofa de
Greensleeves se lee así:

```text
C5:8 D5:4 | E5:6 F#5:2 E5:4 | D5:8 B4:4 | G4:6 A4:2 B4:4 | ...
```

Una melodía que empieza antes de su primer compás, como el «In a» de
«Clementine», tiene una **anacrusa**. Se escribe como una parte propia,
un compás que es casi todo silencio:

```rust
Part::new("Pickup", "-:8 C5:3 C5:1", "I"),
```

Eso es un compás de 3/4: dos pulsos de silencio y luego «In a», una
corchea con puntillo y una semicorchea. El primer tiempo fuerte de la
melodía cae entonces en el primer tiempo de la estrofa, donde lo
esperan la batería y los acordes.

## Los acordes, como cifrado

Debajo de la melodía va un cifrado, compás por compás (ver [Cifrados:
acordes compás por
compás](../band/library.md#cifrados-acordes-compás-por-compás)):
números romanos como `i | bIII | bVII | v`, o nombres como los escribe
una partitura guía, `C#m | G#7/B#`, leídos en la tonalidad de la
parte. Las marcas de la hoja de acordes también funcionan: un cifrado
puede anticipar un acorde o detener la banda (ver [Golpes, sostenidos,
silencios y anticipaciones](../band/stops.md)). El cifrado es lo que
toca la banda, con el acompañamiento y la línea de bajo de la canción,
y lo que muestra la hoja de acordes.

En algunas músicas el acompañamiento es parte de la pieza: la mano
izquierda de un rag, un coral de Bach, un coro. Entonces la parte lo
escribe, y sus acordes no se acompañan:

- `with_left(...)` escribe notas para el canal **Teclados**, en la
  notación de las melodías, `&` incluido;
- `with_lines(&[...])` escribe **otros instrumentos**, cada uno una
  `Line` con el nombre de su canal, un programa de General MIDI y sus
  notas. Las líneas con el mismo nombre en cualquier parte suenan en el
  mismo canal, así que el coro de la Misa Criolla es una sola línea,
  «Choir» (Coro), con su contralto, su tenor y su bajo unidos por `&`.

Una parte escrita entera puede tener cifrado igual: entonces nombra lo
que toca la música, para la hoja de acordes, y la banda no lo toca.

## Partes que cambian algo

`Part::new(name, melody, chords)` crea una parte sencilla. Las demás son
la misma parte con un campo cambiado, por una `const fn` que usa la
**sintaxis de actualización de structs**:

```rust
const fn in_key(self, tonic: PitchClass, mode: ScaleKind) -> Part {
    Part {
        key: Some((tonic, mode)),
        ..self
    }
}
```

`..self` quiere decir «todos los demás campos como en `self`». Como
solo mueve valores de un lado a otro, puede ser `const`, y una canción
sigue siendo una constante con `.in_key(FSharp, Major)` al final de una
de sus partes. Las demás funcionan igual:

| Método | La parte... | Por ejemplo |
|---|---|---|
| `in_key(tonic, mode)` | está en otra tonalidad: una modulación | el rey de la montaña sube a fa♯ mayor |
| `in_time(time)` | está en otro compás | un estribillo en 3/4 en una canción en 2/4 |
| `in_meters(&[...])` | tiene un compás para cada compás | los 5/4 y 6/4 de la Promenade |
| `with_tempo(&[...])` | tiene las indicaciones de tempo de la partitura | «Più vivo», un ritardando, un calderón |
| `played(comping, bass, rhythm)` | se acompaña a su manera | el estribillo en 3/4 necesita el acompañamiento de un vals |
| `with_voices(&[...])` | la cantan varias voces | una ronda |
| `played_by(&[...])` | pasa su melodía a otros instrumentos | Boléro, de la flauta al clarinete |
| `marked(dynamic)` | se toca así de fuerte | el comienzo pianissimo de Boléro |
| `with_drums(&[...])` | tiene la batería escrita | la caja de Boléro |
| `with_left`, `with_lines` | escribe su acompañamiento | la mano izquierda de un rag, un coro |

En la gruta del rey de la montaña usa dos:

```rust
Part::new("Higher", MOUNTAIN_HIGHER, MOUNTAIN_HIGHER_CHORDS).in_key(FSharp, Major),
// "Più vivo": faster at once, at Grieg's own metronome mark.
Part::new("Più vivo", MOUNTAIN_THEME, MOUNTAIN_CHORDS).with_tempo(&[tempo(0, 154)]),
```

### Una tonalidad propia

Una parte en otra tonalidad lee sus números romanos en esa tonalidad,
así que `I` en la parte en fa♯ mayor es un acorde de fa♯, y la
demostración pone una marca de tonalidad donde empieza y otra donde la
música vuelve. La canción responde en qué tonalidad está una parte:

```rust
pub fn part_scale(&self, part: &Part) -> Scale {
    match part.key {
        Some((tonic, mode)) => written_as_tune(Scale::new(tonic, mode), [part]),
        None => self.scale(),
    }
}
```

`written_as_tune` decide el *nombre* de la tonalidad. Un `PitchClass`
tiene una variante para cada una de las doce notas, con sostenidos
(`ASharp`, nunca un si bemol), pero la Promenade está en si♭ mayor, no
en la♯ mayor, y Take Five en mi♭ menor, no en re♯ menor. En lugar de
hacer que cada canción deletree su tonalidad, la biblioteca cuenta las
alteraciones de la melodía y nombra la tonalidad como está escrita la
melodía:

```rust
fn written_as_tune<'a>(key: Scale, parts: impl IntoIterator<Item = &'a Part>) -> Scale {
    let (sharps, flats) = parts
        .into_iter()
        .map(|part| accidentals(part.melody))
        .fold((0, 0), |(sharps, flats), (s, f)| (sharps + s, flats + f));
    match sharps.cmp(&flats) {
        Ordering::Greater => key.with_spelling(Spelling::Sharps),
        Ordering::Less => key.with_spelling(Spelling::Flats),
        Ordering::Equal => key,
    }
}
```

`fold` lleva un par a través de las partes y suma las dos cuentas a la
vez, y `cmp` devuelve un `Ordering`, que el `match` lee como «más
sostenidos», «más bemoles» o «igual». Una melodía con tantos de unos
como de otros, o sin ninguno, deja a la tonalidad su nombre habitual.
Cómo se escribe una tonalidad está en [Sostenidos o bemoles: cómo se
escribe una
tonalidad](../theory/scales.md#sostenidos-o-bemoles-cómo-se-escribe-una-tonalidad).

La música que no está en ninguna tonalidad, donde las doce notas están
en casa, lo dice con la escala cromática: el Juego del rapto de
principio a fin, con `mode: Chromatic`, y partes de Petrushka con
`.in_key(C, Chromatic)`. Una parte así no tiene armadura, y cada nota
que toca cuenta como dentro de su tonalidad.

### Un compás para cada compás

La mayoría de las partes está en el compás de la canción o en uno
propio. Algunas músicas cambian de compás casi en cada compás: la
Promenade alterna 5/4 y 6/4, La consagración de la primavera pasa por
2/8, 3/8, 3/4, 5/8 y más. Una parte con `in_meters(&[...])` tiene una
indicación de compás por compás. Un patrón tiene una sola indicación
de compás, así que `Song::part_runs` corta una parte así en **tramos**
de compases con la misma indicación, cada uno una parte propia con su
porción de la melodía y del cifrado:

```rust
let mut runs: Vec<(usize, usize, TimeSignature)> = Vec::new();
for (bar, &time) in part.meters.iter().enumerate() {
    match runs.last_mut() {
        Some((_, bars, last)) if *last == time => *bars += 1,
        _ => runs.push((bar, 1, time)),
    }
}
```

`last_mut` da el último tramo, si lo hay, para cambiarlo en su sitio: un
compás con la misma indicación lo alarga un compás, y cualquier otro
empieza un tramo nuevo. Los trozos del texto salen de `bars_of`, que
devuelve un slice del propio `&'static str` de la parte entre dos
barras de compás, así que un tramo es una `Part` como cualquier otra y
no cuesta ninguna copia. La demostración nombra el patrón de cada tramo
siguiente por el compás donde empieza, «Llamada, compás 2», y pone una
marca de compás donde cambia la indicación. La consagración de la
primavera necesitaría más patrones de los que admite un proyecto, así
que un tramo con las mismas notas que un tramo anterior, en el mismo
compás, vuelve a tocar ese patrón anterior, como un productor reusa un
loop.

### El tempo de la partitura

Un `TempoMark` es una de las tres cosas que escribe una partitura:

```rust
pub enum TempoMark {
    /// From `at` on, `bpm` beats a minute, at once: a new section with
    /// a new tempo, such as "Più mosso" (more movement), "Adagio" or
    /// "Tempo I" (the first tempo again).
    At { at: usize, bpm: u16 },
    /// From `at`, slowing down or speeding up bit by bit until `bpm`,
    /// reached `over` steps later: "rit." or "rall." (slowing down),
    /// "accel." or "stringendo" (speeding up).
    Glide { at: usize, bpm: u16, over: usize },
    /// A fermata (𝄐), the "bird's eye" over a note or a rest: from `at`,
    /// for `steps` steps, the music waits, lasting twice as long; then
    /// it carries on at the tempo before.
    Fermata { at: usize, steps: usize },
}
```

`tempo(at, bpm)`, `glide(at, bpm, over)` y `fermata(at, steps)` son
`const fn` que los crean, contando pasos desde el principio de la
parte. La demostración convierte cada uno en marcas de tempo en la
lista de reproducción, cada vez que se toca la parte. Un calderón se
convierte en dos: la mitad del tempo mientras dura la nota sostenida, y
después el tempo de antes. El proyecto no tiene calderón propio; la
mitad de rápido es exactamente «el doble de largo» (ver [Cambios de
tempo](../sound/tempo.md#cambios-de-tempo)).

### Rondas y cánones

`with_voices` dice quién canta la melodía, y desde qué compás de la
parte: un `(voice, bar)` para cada copia. La ronda de Frère Jacques son
cuatro voces, cada una dos compases después de la anterior:

```rust
Part::new("Round", FRERE_JACQUES_TUNE, FRERE_JACQUES_CHORDS).with_voices(&[
    (1, 0),
    (2, 2),
    (3, 4),
    (4, 6),
]),
```

Un compás negativo es una voz que empezó en la parte anterior, así que
solo se oye su final: «Round again» deja que las voces 2 a 4 terminen
sus últimos compases mientras empiezan de nuevo. La voz 1 es el canal
Melodía, y cada otra voz recibe un canal propio, «Voz 2», «Voz 3» y
así.

### Otros instrumentos, más fuerte y más suave

`played_by` toma una lista de `(program, semitones)`: cada instrumento
toca la melodía, esa cantidad de semitonos más arriba, en un canal con
su nombre. Boléro pasa su melodía de la flauta al clarinete, al fagot y
así; su clarinete en mi♭ la toca una octava más arriba,
`(gm::CLARINET, 12)`. `marked` fija lo fuerte que se toca la parte, de
pianissimo a fortissimo:

```rust
pub fn scale(self) -> f32 {
    match self {
        Dynamic::Pianissimo => 0.4,
        Dynamic::Piano => 0.55,
        Dynamic::MezzoPiano => 0.75,
        Dynamic::MezzoForte => 1.0,
        Dynamic::Forte => 1.15,
        Dynamic::Fortissimo => 1.3,
    }
}
```

`with_drums` escribe una parte de batería, en la notación de las
melodías, con `x` para un golpe y `X` para uno acentuado. En lugar de un
segundo analizador, `drum_line` convierte cada `x` en un `C4` y cada `X`
en un `D4`, lee la línea como una melodía, y después hace de cada nota
un golpe de un paso, acentuado si era un `D4`, tan fuerte como dicen las
dinámicas de la línea: tresillos, silencios y dinámicas salen gratis.

## La banda de una canción

`band` nombra los programas de General MIDI de los canales Melodía,
Teclados y Bajo, y quizá una sección de vientos:

```rust
pub struct Band {
    pub lead: u8,
    pub keys: u8,
    pub bass: u8,
    pub horns: Option<Horns>,
}
```

La mayoría de las bandas son constantes que comparten las canciones de
un género:

```rust
const BOSSA: Band = Band::new(gm::TENOR_SAX, gm::NYLON_GUITAR, gm::ACOUSTIC_BASS);
```

`Band::new` y `with_horns` son `const fn`, así que una banda se puede
construir dentro de una constante. Una sección de vientos es un
programa y una manera de acompañar: los saxofones de una big band
sostienen los acordes (`Comping::Held`) y sus metales los golpean en el
pulso y justo antes del pulso 3 (`Comping::Charleston`). Los violines
de una orquesta típica de tango también son una «sección de vientos»,
con un programa de cuerdas: la demostración llama al canal «Cuerdas»
para los programas 40 a 51 y «Vientos» para cualquier otro. Una canción
con una banda con vientos es algo distinto de la [sección de
vientos](../band/band.md#una-sección-de-vientos) del estilo de banda,
que cualquier canción puede encender.

## De una canción a un proyecto

`song_demo(song)` construye el proyecto en cuatro pasos.

**Canales.** La Melodía, un canal para cada voz de más, los Teclados
solo si alguna parte toca acordes o una mano izquierda, el Bajo salvo
que la canción no tenga línea de bajo, los vientos, un canal para cada
instrumento al que se pasa una melodía, y uno para cada nombre de
línea. Cada uno recibe su programa.

**Un patrón para cada parte,** o para cada tramo de una parte cuyo
compás cambia. Ahí va la melodía en los canales de sus voces, el
cifrado acompañado con el acompañamiento y el bajo de la parte
(`accompany_in_bars`, que mantiene el acompañamiento al paso de los
compases, así un acorde de un pulso igual se toca), la parte de los
vientos, la mano izquierda y las líneas escritas, y la batería escrita.
Después dos pasadas hacen que suenen tocados (ver la sección
siguiente) y la dinámica de la parte los escala.

**Un patrón de batería para cada parte,** con el ritmo de la canción o
el de la parte, en el compás de la parte, con un fill al final de las
partes de cuatro compases o más, salvo que la parte escriba su batería.

**El arreglo.** La forma se recorre una vez, y cada entrada pone los
patrones de su parte uno tras otro en la pista Canción y su batería en
la pista Batería desde `drums_from`, con marcas de tonalidad, de tempo y
de compás donde cambian, y el cifrado escrito en la hoja de acordes. Un
último compás en el acorde de tónica, con un platillo, termina la
canción, en la tonalidad de la última parte, así una canción que
modula termina en la tonalidad nueva. Una pieza cuya última parte está
escrita entera termina donde termina su partitura, y lo mismo un
cifrado que termina en un corte, como el «chan-chan» de un tango: la
partitura ya dice cómo se detiene la música.

Dos detalles la terminan. Las canciones, los himnos, los tangos y las
piezas de concierto se frenan un poco en su último compás, como hace
quien toca donde no hay caja de ritmos a la que seguir:

```rust
fn ends_broadening(song: &Song) -> bool {
    use super::Genre::*;
    matches!(song.genre, Classical | Folk | Soul | Tango | Folklore)
}
```

Eso es un *poco ritardando* hasta nueve décimos del tempo, salvo que la
partitura ya marque cómo va el tempo de la última parte. Y el swing es
el del ritmo de la canción, salvo en el folk y la música clásica, cuyas
corcheas quedan parejas sobre cualquier ritmo:

```rust
pub fn swing(&self) -> u8 {
    match self.genre {
        Genre::Folk | Genre::Classical => 0,
        _ => Rhythm::named(self.rhythm).map_or(0, |rhythm| rhythm.swing),
    }
}
```

El resultado es un proyecto corriente, para desarmar y cambiar: nada en
él está marcado como demostración.

## Tocar como una persona

Una partitura escrita nota a nota, cada nota tan fuerte como la
siguiente, suena a máquina. `interpret.rs` cambia las velocidades unos
pocos puntos sobre 127, como lo haría quien toca; las notas se quedan
donde están. Para una melodía, `perform` le da forma a cada frase:

- el primer tiempo del compás un poco más fuerte, los otros tiempos
  menos, las notas a contratiempo más ligeras;
- cuanto más aguda una nota en su frase, más se canta;
- una nota larga, apoyada; la última nota de una frase, relajada;
- y nunca dos notas del todo iguales.

Una frase termina en un silencio de una corchea o más, la respiración
más corta de quien canta, o después de cuatro compases. Los acordes y el
bajo, cuyos acentos ya escribió el acompañamiento, solo pierden su
regularidad: las notas de un acorde un poco distintas entre sí, los
acordes un poco más. Después `follow` hace que la banda toque tan fuerte
como está marcada la melodía donde empieza cada una de sus notas, así un
crescendo en la melodía arrastra a la banda, y un acento en la melodía
es medio acento en la banda.

«Nunca dos iguales» no puede querer decir «distinto cada vez»: una
demostración tiene que sonar igual cada vez que se abre, y un test que
comprueba sus notas tiene que ver las mismas notas. Así que las
pequeñas diferencias salen de una función que parece aleatoria pero no
lo es:

```rust
fn wobble(seed: u64, value: u64) -> f32 {
    let mut x = seed ^ value.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^= x >> 31;
    (x >> 40) as f32 / (1u64 << 23) as f32 - 1.0
}
```

Es el paso de mezcla de **SplitMix64**: multiplicaciones y
desplazamientos que esparcen los bits de la entrada tan a fondo que
entradas vecinas dan salidas sin relación. La entrada es el paso y la
altura de la nota; la semilla se hace con los nombres de la canción, la
parte y el canal, con FNV-1a (ver [FNV-1a](../band/band.md#fnv-1a)).
Los 24 bits más altos del resultado, divididos por 2²³, dan un número
de 0 a 2, y el menos uno lo lleva de −1 a 1. `wrapping_mul` multiplica
y descarta lo que no cabe en 64 bits, lo que un `*` normal trataría como
desbordamiento. La forma que se le da queda pequeña al lado de la
diferencia entre piano y forte: una velocidad cambiada por `clamp`
queda entre 24 y 124, o donde estaba escrita si eso queda más afuera,
así un pianissimo sigue suave.

## El menú Demostraciones

El menú muestra las demostraciones por tipo (**Canciones**,
**Estilos**, **Dodecafónico**, **Micromodos**), las canciones y los
estilos por género, y cada lista en orden alfabético. Las listas de la
biblioteca guardan el orden en que se escribieron; el menú ordena una
copia, que construye la primera vez que se muestra, en un `LazyLock`:

```rust
static DEMOS: LazyLock<Demos> = LazyLock::new(|| Language::English.with(all_demos));
static DEMOS_ES: LazyLock<Demos> = LazyLock::new(|| Language::Spanish.with(all_demos));
```

Una `static` tiene que conocerse cuando se compila el programa; un
`LazyLock` es una `static` cuyo valor calcula la clausura la primera vez
que alguien la lee, y lo guarda. Hay dos, porque los títulos se ordenan
tal como se muestran, y en español muchos se muestran en español.

### Alfabético, para personas

Comparar texto letra por letra, por número de Unicode, pone todas las
mayúsculas antes de todas las minúsculas y «Ábalos» después de
«Zamba». Así que el menú ordena por una copia **plegada** de cada
título: en minúsculas, sin tildes, sin puntuación, con el guion como un
espacio. «Für Elise» se ordena como «fur elise», justo después de
«Funeral March»:

```rust
fn by_title(a: &str, b: &str) -> Ordering {
    fold(a).cmp(&fold(b)).then_with(|| a.cmp(b))
}
```

`then_with` desempata, dos títulos que se pliegan igual, por los
títulos mismos, así el orden nunca depende del azar. `fold` es un solo
`match` sobre los caracteres: cada letra con tilde se convierte en la
simple, `♯` y `♭` en `#` y `b`, las letras, los dígitos, `#`, `/` y `+`
se quedan (así se pueden buscar «F#», «3/4» y «2+2+3/8»), y cualquier
grupo de espacios y signos entre palabras se convierte en un espacio.

### Buscar

Un cuadro de búsqueda está encima de los submenús y tiene el teclado
en cuanto se abre el menú. Cada demostración tiene tres textos
plegados: su título (en los dos idiomas, así «Noche de paz» encuentra
Silent Night en cualquiera de los dos), sus detalles (quién la
escribió, su estilo, su género, su tonalidad y su compás) y las notas
sobre ella. Cada palabra escrita tiene que empezar una palabra en uno
de ellos: `bach` encuentra a Bach, no a Offenbach:

```rust
fn starts_a_word(text: &str, words: &str) -> bool {
    text.match_indices(words)
        .any(|(at, _)| at == 0 || text[..at].ends_with(' '))
}
```

Las coincidencias se ordenan, la mejor primero:

```rust
enum Rank {
    TitleStart,
    TitleWords,
    Details,
    Words,
    About,
}
```

Primero va el título que empieza con lo escrito, luego cada palabra
encontrada en el título, luego la frase encontrada en los detalles,
como «re menor» en una canción en re menor, luego las palabras
encontradas por separado, y al final una palabra encontrada solo en las
notas. `Rank` deriva `Ord`, así que las variantes se comparan en el
orden en que están escritas, y un orden **estable** por rango mantiene
el orden alfabético entre iguales. Los dos rangos más débiles se
muestran más tenues, así resaltan las coincidencias que más
probablemente buscabas.

## Cómo se prueba

Las canciones son datos, y los tests los leen todos. En `song.rs`:
`every_bar_of_every_part_adds_up` (cada compás de cada melodía y mano
izquierda dura lo que su indicación de compás, cada cifrado se puede
leer, una parte con un compás por compás tiene tantas indicaciones como
compases), `forms_name_real_parts_and_rhythms_exist`,
`every_song_names_a_real_style`, `songs_in_three_play_in_three`,
`songs_that_change_meter_have_a_style_that_follows_it`,
`every_genre_has_songs_and_they_are_in_order`,
`keys_are_written_as_their_tunes_are` (Take Five en mi♭ menor, el trío
del Maple Leaf Rag en re♭ mayor), `melodies_are_mostly_in_their_key`
(al menos el 80% de las notas de una canción en la tonalidad de su
parte, lo que atrapa un fa escrito por un fa♯) y
`the_first_notes_are_right` (Oda a la alegría empieza fa♯ fa♯ sol la).

En `demo.rs`, `every_song_makes_a_demo` construye las 343, y otros
comprueban una cosa cada uno: `a_round_has_its_voices_in_the_right_places`,
`a_part_in_its_own_time_gets_time_markers`,
`runs_of_the_same_bars_share_a_pattern`,
`tempo_markings_go_on_the_playlist`,
`songs_and_hymns_slow_down_a_little_at_the_end`,
`a_song_that_ends_after_a_modulation_ends_in_the_new_key`,
`a_piece_written_out_in_full_ends_where_its_score_ends`,
`a_big_band_has_its_horns`,
`the_band_follows_the_dynamics_of_the_tune_and_the_breaks_of_the_chart`
y `every_demo_keeps_a_sensible_tempo` (cada marca de tempo entre tres
décimos y dos veces y media el tempo de la canción). `interpret.rs`
comprueba que `piano_stays_soft_and_fortissimo_loud` y que
`a_tune_gets_its_accents_but_stays_close_to_how_it_was_written`.
`demo_menu.rs` comprueba el plegado, el orden y los rangos, como
`matches_only_in_the_notes_come_last`.

Además de los tests unitarios, cualquier demostración se puede tocar
sin ventana con la [traza de
reproducción](../sound/trace.md), `ferrous-lineup --trace "Frère
Jacques" --out round.tsv`, y `ferrous-lineup --demos` lista los nombres
que acepta.
