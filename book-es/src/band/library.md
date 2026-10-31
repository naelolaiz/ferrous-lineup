# La biblioteca: estilos y ritmos como datos

Archivos: [`crates/lineup-core/src/library/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates/lineup-core/src/library),
[`crates/lineup-core/src/sources.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/sources.rs),
[`crates/lineup-app/src/library_browser.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/library_browser.rs)
y [`crates/lineup-app/src/app/library.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/library.rs)

Un músico que se sienta a tocar una bossa nova sabe cuatro cosas: los
acordes, cómo los toca la guitarra, qué hace el bajo y el patrón de la
batería. Nada de eso es código. Es *conocimiento*, y la mejor manera de
meter conocimiento en un programa es escribirlo como datos, en una forma
que una persona pueda leer y comprobar, y que el código que lo usa sea
pequeño.

La biblioteca hace eso: 215 estilos armónicos, 139 ritmos y 6 rellenos,
cada uno unas pocas líneas en una tabla, más los estilos dodecafónicos y
de micromodos de los capítulos de teoría y las canciones de [Cómo se
escribe una demostración](../songs/demos.md). Este capítulo recorre
las notaciones y después sigue un estilo desde la tabla hasta una
canción: qué canal toca qué, cómo cambia de estilo una canción entera,
cómo cada parte de la canción recuerda de dónde vino, y la pestaña
Biblioteca que lo lista todo. Cómo se usa la Biblioteca está en el
capítulo del manual [Los estilos y la
Biblioteca](https://naelolaiz.github.io/ferrous-lineup/es/manual/styles.html).

## Cifrados: acordes compás por compás

Una progresión escrita con nombres de acordes, «C G Am F», solo sirve
en Do. Escrita con números romanos, «I V vi IV», sirve en cualquier
tonalidad (ver [Leer un número
romano](../theory/numerals.md#leer-un-número-romano)). Un **cifrado**
escribe los números compás por compás, como una hoja guía, con `|` entre
compases:

```text
I | IV V | vi | ii7 V7
```

Los acordes de un compás se lo reparten por igual, así que «IV V» es
medio compás cada uno. Un acorde que repite el anterior se sostiene, no
se vuelve a tocar: «I | I» son dos compases de un mismo acorde.
`chart::parse_chart(text, bar)` convierte un cifrado en `ChordSpan`s,
en pasos de un compás de `bar` pasos:

```rust
pub struct ChordSpan {
    pub chord: RomanChord,
    pub start: usize,
    pub length: usize,
    /// Whether the band plays it early (see [`Push`]).
    pub push: Option<Push>,
    /// Whether the band stops playing its style here (see [`Stop`]).
    pub stop: Option<Stop>,
}
```

Devuelve `None` cuando no se puede leer un acorde, un compás está vacío
o un compás no se puede repartir por igual entre sus acordes: una
entrada de la tabla que está mal hace fallar una prueba en lugar de
tocar algo sin sentido. `push` y `stop` son las marcas de [Golpes,
sostenidos, silencios y anticipaciones](stops.md).

## El comping: cómo se tocan los acordes

Los mismos cuatro acordes pueden ser una balada, un reggae o un tema de
house. Lo que cambia es el ritmo con que se golpean los acordes, lo que
los músicos de jazz llaman **comping** (de *accompanying*, acompañar), y
lo que hace el bajo. Cada manera de hacer comping es una tabla corta de
**golpes**: cuándo empieza una nota, cuánto dura, qué notas del acorde
toca y con qué fuerza.

```rust
struct Hit<T> {
    /// When it starts, in steps from the start of the pattern's period.
    at: usize,
    /// How long it lasts, in steps.
    len: usize,
    what: T,
    velocity: u8,
}
```

Para los acordes, `what` es un `Voices`: `All`, todas las notas;
`Upper`, todas menos la más grave, el golpe ascendente más suave de un
rasgueo; o `Nth(n)`, una nota contada desde abajo, para los arpegios.
Para el bajo es un `Tone`: la fundamental, la quinta, la octava, la
tercera, la séptima, la fundamental del acorde siguiente tocada antes
(una **anticipación**) o la nota a un semitono de ella, que conduce a
la siguiente. Un único struct genérico sirve para los dos, con `T`
puesto dos veces.

```rust
Comping::Pulse => (4, &[hit!(0, 2, All, 96), hit!(2, 2, All, 78)]),
Comping::Quarters => (4, &[hit!(0, 2, All, 100)]),
Comping::Offbeat => (4, &[hit!(2, 1, All, 92)]),
```

El primer número es el **período**: los golpes se repiten cada 4 pasos,
un pulso. `Pulse` son corcheas parejas, la segunda más suave; `Offbeat`
es el *skank* del reggae, un acorde corto en la segunda corchea de cada
pulso. El largo, corto, largo, largo de la habanera ocupa 8 pasos:

```rust
Comping::Habanera => (
    8,
    &[
        hit!(0, 3, All, 96),
        hit!(3, 1, Upper, 74),
        hit!(4, 2, All, 86),
        hit!(6, 2, All, 84),
    ],
),
```

### `macro_rules!`, y por qué no una función

`hit!` es una pequeña **macro declarativa**:

```rust
macro_rules! hit {
    ($at:expr, $len:expr, $what:expr, $velocity:expr) => {
        Hit {
            at: $at,
            len: $len,
            what: $what,
            velocity: $velocity,
        }
    };
}
```

Una macro es código que escribe código. `macro_rules!` compara lo que
recibe con un patrón, aquí cuatro expresiones (`$at:expr` y las demás),
y las pega en la plantilla de la derecha. `hit!(0, 2, All, 96)` se
convierte en `Hit { at: 0, len: 2, what: All, velocity: 96 }` antes de
que el compilador compruebe nada.

¿Por qué no una `const fn hit(...) -> Hit`? Hay que mirar lo que
devuelve el `match`: `&'static [Hit<Voices>]`, un slice prestado que
vive todo el programa. Escribir `&[ ... ]` con valores simples da
exactamente eso: el compilador ve una expresión hecha solo de
constantes, la pone en los datos de solo lectura del programa y deja
que el préstamo viva para siempre. Eso se llama **promoción de
constantes**. Solo ocurre con valores simples: una *llamada a función*
dentro de los corchetes, aunque sea una `const fn`, no se promueve, y el
compilador dice que el temporal prestado se descarta al final de la
sentencia. La macro hace que la tabla sea corta de escribir y siga
hecha de valores simples. El comentario sobre la macro lo dice, para
que nadie la «simplifique» convirtiéndola en una función.

## Estilos: una tabla en un `static`

Un estilo lo junta todo: un cifrado en un modo, una manera de hacer
comping, una línea de bajo, un tempo habitual y el ritmo que lo
acompaña.

```rust
pub struct Style {
    pub name: &'static str,
    pub genre: Genre,
    /// The mode the chords are written in: the project's scale becomes
    /// this mode when the style is used.
    pub mode: ScaleKind,
    pub time: TimeSignature,
    /// The progression, as a chord chart (see [`chart::parse_chart`]):
    /// numerals counted from the tonic, bars separated by `|`.
    pub chart: &'static str,
    pub comping: Comping,
    pub bass: Bass,
    /// The tempo the style is usually played at, in BPM.
    pub tempo: f32,
    /// The name of the rhythm in [`super::RHYTHMS`] that goes with it, or
    /// "" for none.
    pub rhythm: &'static str,
    /// One or two sentences on what to listen for.
    pub about: &'static str,
}
```

Cada campo es un `&'static str`, algo `Copy` o un enum pequeño, así que
un `Style` es `Copy` y la tabla no necesita reservar memoria. Una
entrada se lee como una frase:

```rust
style(
    "Four chords",
    Pop,
    Major,
    "I | V | vi | IV",
    Comping::Pulse,
    Bass::Pulse,
    110.0,
    "Pop beat",
    "The four chords behind hundreds of hits: home, the dominant, the sad relative minor, \
     and the subdominant.",
),
```

`style` es una `const fn` que pone 4/4, con
`#[allow(clippy::too_many_arguments)]` porque nueve argumentos en una
fila de tabla son más claros que nueve nombres de campo. En la
aplicación, este estilo se llama «Cuatro acordes» y su ritmo «Ritmo
pop»: las tablas guardan los nombres en inglés y la interfaz los
traduce al mostrarlos. Los estilos en otro compás añaden
`.in_time(THREE_FOUR)`, otra `const fn`, que usa la **sintaxis de
actualización de structs**:

```rust
const fn in_time(self, time: TimeSignature) -> Style {
    Style { time, ..self }
}
```

`..self` quiere decir «todos los demás campos como en `self`». La tabla
misma es un `static`:

```rust
pub static STYLES: &[Style] = &[
```

Un `const` se pega en cada lugar que lo usa, como una macro; un
`static` es un único valor en una única dirección durante todo el
programa. Para una tabla grande que se usa en muchos lugares, `static`
guarda una sola copia, y un `&'static Style` que apunta a ella es tan
barato de pasar como un número y nunca puede quedar colgando.

`Style::render(tonic)` toca un estilo en cualquier tonalidad: el
cifrado se convierte en tramos, cada número romano en un acorde de la
tonalidad, los acordes se disponen uno tras otro moviéndose lo menos
posible, y el comping y el bajo los tocan. Ese es el trabajo propio de
la banda, en [Cómo toca la banda una hoja de
acordes](band.md).

## Ritmos, una línea por tambor

Los bateristas y las cajas de ritmos piensan un ritmo como una
cuadrícula: una fila por tambor, una columna por paso. La biblioteca
escribe exactamente eso, como texto:

```rust
rhythm(
    "Pop beat",
    Pop,
    110.0,
    &[
        (Kick, "X... .... X.x. ...."),
        (Snare, ".... X... .... X..."),
        (HiHat, "x.x. x.x. x.x. x.x."),
    ],
    "Snare roll",
    "The backbeat: kick on 1 and 3, snare on 2 and 4, hi-hat on every eighth note.",
),
```

Cada carácter es un paso. `X` es un golpe fuerte, `x` uno normal, `o`
una **nota fantasma** (*ghost note*) suave y `.` silencio. `h` y `l`
tocan el tambor cinco semitonos más agudo o más grave, para los tonos
abierto y apagado de una conga o para dos toms. Los espacios solo
sirven para ver bien los pulsos, y `|` separa compases, en los ritmos
de más de uno. «Snare roll» (el «Redoble de caja» de la aplicación)
nombra el **relleno**, un corte de un compás que cierra una sección.

Un carácter se convierte en un golpe:

```rust
fn hit(character: char) -> Option<Option<Hit>> {
    match character {
        '.' => Some(None),
        'X' => Some(Some((120, 0))),
        'x' => Some(Some((96, 0))),
        'o' => Some(Some((56, 0))),
        'h' => Some(Some((96, 5))),
        'l' => Some(Some((96, -5))),
        _ => None,
    }
}
```

Un `Option` dentro de un `Option` dice dos cosas distintas: el `None`
de fuera es «este no es un carácter que un ritmo pueda tener», así que
una línea con una errata se rechaza; el `None` de dentro es «un paso de
silencio», una respuesta perfectamente válida. Aquí `Hit` es un alias
de `type`, `(u8, i32)`: una intensidad y un desplazamiento en
semitonos.

Las mismas dos letras escriben los usul turcos de los ritmos balcánicos.
En la darbuka, `l` es el golpe grave, **düm**, y `h` el agudo, **tek**,
y los comentarios citan de dónde viene cada patrón:

```rust
rhythm(
    "Aksak",
    Balkan,
    90.0,
    &[
        (Kick, "X... .... X... ......"),
        (Conga, "l... h.h. l... h...h."),
        (Shaker, "x.o. x.o. x.o. x.o.o."),
    ],
    "",
    "The Turkish aksak, 9/8 counted 2 + 2 + 2 + 3, on the darbuka: düm, te-ke, \
     düm, tek-tek, the low düm on the first and third beats, the long last beat \
     split two and one.",
)
.in_time(TimeSignature::of_groups(&[2, 2, 2, 3], 8)),
```

Cuando un ritmo entra en un patrón, se reparte compás por compás en el
compás del patrón:

```rust
for start in (0..length).step_by(bar) {
    let steps = &bars[(start / bar) % bars.len()];
    for (step, hit) in steps.iter().enumerate().take(bar) {
```

`step_by(bar)` da el comienzo de cada compás, y `take(bar)` corta un
compás largo en la barra de compás, así que un ritmo de 4/4 en un
patrón de 3/4 pierde su último pulso en cada compás en lugar de
desplazarse por encima de las barras. Unos pocos ritmos, los marcados
`follows_the_meter`, se escriben de nuevo para el compás propio de cada
compás, para la música cuyo compás cambia todo el tiempo (ver [Una
orquesta para música que cambia de
compás](orchestra.md)).

### Repartido sobre las partes de una canción

La mayoría de los ritmos son un giro de un compás o dos, y la batería lo
toca una y otra vez, con rellenos donde terminan las partes de la
canción (ver [La batería por partes](song-form.md#la-batería-por-partes)).
Alguna música se construye de otra manera: los tambores de un candombe
abren con la clave tocada en la madera del repique, hacen el groove con
la base del repique, y cierran cada frase con su llamada, el «fraseo».
`Rhythm::laid_out` le da a un ritmo un `Layout`, y sus líneas se leen
entonces como esas piezas, una tras otra:

```rust
const CANDOMBE: Layout = Layout {
    opening: 4,
    grooves: &[4, 4, 4, 4],
    turns: 2,
    closing: 2,
    closings: 2,
};
```

Así, las líneas del candombe duran 24 compases: cuatro de apertura; dos
grooves de cuatro compases para las partes A y dos para las B; y dos
cierres de dos compases. Una frase se turna con los grooves de su clase
y con los cierres (`Layout::groove`, `Layout::closing_at`), contando las
frases de esa clase que la canción ya tocó: la primera frase de una
parte A toca el primer groove y el primer cierre, la siguiente los
segundos, y así sucesivamente. El segundo groove abre con un compás de
clave en la madera y varía más el piano; el segundo cierre arranca
desde la base más cargada del repique, para que la clave nunca llegue
justo antes de una parte nueva. Una frase demasiado corta para todo eso
toca el comienzo de la apertura y el cierre. Un ritmo así no necesita
un relleno de la tabla: su cierre es su relleno.

### Un ritmo para cualquier compás

Cuando cambias a mano el compás de una canción, hace falta un ritmo en
el nuevo. `Rhythm::for_time` pregunta, por orden, hasta que algo
responde:

```rust
RHYTHMS
    .iter()
    .filter(in_time)
    .find(same_kind)
    .or_else(|| {
        USUAL
            .iter()
            .filter_map(|name| Rhythm::named(name))
            .find(in_time)
    })
    .or_else(|| RHYTHMS.iter().find(in_time))
    .or_else(|| Rhythm::grouped_for(time, same_kind))
    .unwrap_or_else(|| Rhythm::basic(time))
```

Un ritmo del mismo tipo en el compás nuevo (un tango en 3/4 es un vals
de tango); si no, el habitual (un vals para 3/4); si no, cualquiera en
ese compás; si no, uno con el mismo número de las mismas notas por
compás, agrupadas a su manera (7/8 se toca como 2+2+3/8, una
rachenitsa); y si no, un ritmo sencillo hecho para ese compás. Cada
`or_else` solo se ejecuta si todo lo anterior no encontró nada. Cómo
pasan las notas de la canción al nuevo compás está en [Pasar una
canción a otro compás](../sound/tempo.md#pasar-una-canción-a-otro-compás).

Ese último ritmo sencillo tiene un problema: un `Rhythm` guarda líneas
`&'static str`, y 5/4 no está en ninguna tabla. `Rhythm::basic` hace el
texto la primera vez que se lo piden, y lo guarda mientras dure el
programa:

```rust
pub fn basic(time: TimeSignature) -> &'static Rhythm {
    static MADE: Mutex<Vec<&'static Rhythm>> = Mutex::new(Vec::new());
    let mut made = MADE.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(beat) = made.iter().find(|beat| beat.time == time) {
        return beat;
    }
    let lines: Vec<(Drum, &'static str)> = basic_lines(time)
        .into_iter()
        .map(|(drum, line)| (drum, &*String::leak(line)))
        .collect();
```

`String::leak`, `Vec::leak` y `Box::leak` renuncian a propósito a su
memoria: nunca se libera, y a cambio se obtiene una referencia
`&'static` a ella. Filtrar memoria cada vez crecería sin fin, así que
los ritmos hechos se guardan en un `static Mutex<Vec<...>>` y cada
compás se hace una sola vez: un puñado de cadenas pequeñas durante la
vida del programa. `Mutex::new` es una `const fn`, y eso es lo que deja
que un `static` guarde uno. Un candado envenenado (un hilo entró en pánico
mientras lo tenía) sigue guardando datos válidos aquí, así que
`unwrap_or_else(PoisonError::into_inner)` los toma igualmente.

## Adónde van las notas

Las tablas no saben nada de canales. `library/apply.rs` decide adónde
van sus notas: los acordes a un sintetizador llamado «Keys» (o al canal
que tengas seleccionado), la línea de bajo a un sintetizador llamado
«Bass», la melodía de una canción a «Lead» y cada tambor al canal que
lo toca. Los canales que faltan se añaden, con un piano para Keys y un
bajo eléctrico tocado con los dedos para Bass como sonidos General
MIDI.

```rust
pub fn chords_channel(project: &mut Project, selected: Option<usize>) -> Option<usize> {
    let free = |(index, channel): (usize, &Channel)| {
        plays_notes(channel)
            && channel.name != BASS
            && channel.name != LEAD
            && !project.is_locked(index)
    };
    if let Some(index) = selected
        && project
            .channels()
            .get(index)
            .is_some_and(|channel| free((index, channel)))
    {
        return Some(index);
    }
    let other = project
        .channels()
        .iter()
        .enumerate()
        .position(|(index, channel)| channel.kind == ChannelKind::Synth && free((index, channel)));
    find_synth(project, KEYS)
        .or(other)
        .or_else(|| synth_named(project, KEYS))
}
```

`free` es un closure guardado en una variable, para usarlo dos veces.
Un canal **bloqueado** nunca se elige para los acordes (ver [Forma,
secciones, variaciones y bloqueos](song-form.md)). La cadena
del final muestra la diferencia entre `or` y `or_else`: `other` ya está
calculado, así que `or` lo toma como valor; `synth_named` *añade un
canal*, así que solo debe ejecutarse si no se encontró nada más, y va
en un closure con `or_else`.

Los tambores se encuentran por lo que son, no por su nombre: el canal
cuyo tipo es `Sampler(SampleSource::Builtin(drum))`. Un bombo
renombrado «Bombo» sigue siendo el bombo. Un sintetizador llamado
«Bass» que se envía por MIDI a un sintetizador externo sigue siendo el
bajo, porque `plays_notes` acepta también los canales de salida MIDI
(ver [MIDI](../sound/midi.md)).

## Tocar una canción en otro estilo

Probar un estilo en un patrón propio es fácil. Cambiar el estilo de una
canción entera es más difícil: «hagamos esta como un reggae» quiere
decir los mismos acordes, tocados de otra manera por todos. Pero un
patrón no guarda sus acordes; guarda notas. Así que la biblioteca
primero los **oye**, con el segundo oído de [Oír la
armonía](../theory/hearing.md#un-segundo-oído-para-los-patrones-de-la-propia-banda),
y después los vuelve a tocar.

Un ritmo dice cómo le gusta que se toquen los acordes a través del
estilo del que es la batería:

```rust
pub fn feel(rhythm: &Rhythm) -> (Comping, Bass) {
    let same_time = |style: &&Style| style.time == rhythm.time;
    STYLES
        .iter()
        .find(|style| style.rhythm == rhythm.name)
        .or_else(|| {
            STYLES
                .iter()
                .filter(same_time)
                .find(|style| style.genre == rhythm.genre)
        })
        .or_else(|| STYLES.iter().find(same_time))
        .map_or((Comping::Pulse, Bass::Pulse), |style| {
            (style.comping, style.bass)
        })
}
```

El ritmo «Salsa» es la batería del estilo «Salsa montuno» (el «Montuno
de salsa» de la aplicación), así que toca sus acordes como un montuno
sobre un bajo de tumbao. Un ritmo que ningún estilo usa toma prestado
un estilo de su género en su compás, así que un ritmo en 3/4 nunca
recibe una manera de tocar pensada para 4/4. ¿Por qué `&&Style`?
`STYLES.iter()` da `&Style`, y `filter` le pasa a su closure una
referencia a cada elemento: una referencia a una referencia.

`recomp` vuelve a tocar un patrón: oye los acordes de su canal de
acordes sobre su bajo, los dispone y deja que el nuevo comping y el
nuevo bajo toquen cada tramo. Los tramos donde no se oyó ningún acorde
conservan sus notas, así que una introducción sin acordes queda como
estaba. `play_song` lo hace con cada patrón, y deja tres cosas en paz a
propósito: la melodía, los patrones de la banda (que siguen la hoja de
acordes, ver [Cómo toca la banda una hoja de
acordes](band.md)) y los finales, un patrón cuyas notas
empiezan todas en su primer paso, como un acorde final tenido con un
platillo. Donde la batería anterior tenía un relleno en su último
compás, va el relleno del ritmo nuevo. Un relleno es un último compás
que no suena como ningún compás anterior:

```rust
// A fill is a last bar that plays like no bar before it.
let fill = length >= 2 * bar && {
    let last = drums_in(project, pattern, length - bar, bar);
    (0..length - bar)
        .step_by(bar)
        .all(|start| drums_in(project, pattern, start, bar) != last)
};
```

En Rust un bloque `{ ... }` es una expresión, cuyo valor es su última
línea, así que puede ir después de `&&`; y como `&&` se detiene en
cuanto su lado izquierdo es `false`, el bloque solo se ejecuta para
patrones de dos compases o más. Un ritmo en otro compás primero cambia
el compás de toda la canción, compás por compás, así que una canción en
4/4 tocada como vals pasa a ser una canción en 3/4.

### Qué partes tomar

La ventana que abre un estilo en la aplicación pregunta qué partes
tomar, en tres grupos: bajo **Acompañamiento**, **Batería**, **Partes
de acordes y bajo** y **Voces / instrumentos**; bajo **Armonía**,
**Progresión de acordes**; bajo **Tiempo**, **Compás**, **Tempo** y
**Swing**. Son un struct pequeño:

```rust
pub struct StyleParts {
    pub rhythm: bool,
    pub instruments: bool,
    pub time_signature: bool,
    pub progression: bool,
    /// Take the preset's usual tempo where the meter stays the same. Off
    /// by default: the song keeps its tempo, so its own notes last as
    /// long as before.
    pub tempo: bool,
    pub swing: bool,
    /// Copy the preset's chord, bass and other accompaniment parts (a
    /// rhythm's are those of its matching style). Off, the song's own
    /// stay, as its drums do with `rhythm` off.
    pub matching_style: bool,
}
```

`rhythm` es la casilla **Batería** y `matching_style` la casilla
**Partes de acordes y bajo**. Cada casilla va por su cuenta: lo que
queda sin marcar, la canción lo conserva. En un patrón, cada casilla es
un paso de `write_style_parts`, y cada paso toca solo sus propias
notas:

```rust
if parts.time_signature {
    meter(project, index, style.time);
}
if parts.matching_style {
    recomp(project, index, style.comping, style.bass);
}
if parts.progression {
    progression(project, index, style);
}
// ...
if parts.rhythm
    && let Some(rhythm) = Rhythm::named(style.rhythm)
{
    write_rhythm(project, index, rhythm);
}
if parts.instruments {
    style_instruments(project, style, selected);
}
```

`recomp` vuelve a tocar los canales de acordes y de bajo y deja la
batería; `write_rhythm` escribe los carriles de batería y deja el
resto. Así una canción puede tomar la batería de un estilo y las partes
de acordes y bajo de otro, en cualquier orden. Una banda guarda esa
misma separación en un campo propio, `drums` (ver [Cómo toca la banda
un cifrado](band.md)). La línea `if parts.rhythm && let Some(rhythm) =
...` es una *cadena de let* (*let chain*): el bloque se ejecuta solo
cuando la casilla está marcada *y* se encuentra el ritmo del estilo,
con `rhythm` ligado dentro de él.

**Voces / instrumentos** da los sonidos del estilo a los canales que
tocan sus partes: Teclados y Bajo, o el canal elegido en el rack. Una
canción importada de un archivo MIDI no tiene ninguno de los dos; sus
canales llevan los nombres que les dio el archivo, como «PIANO
MELODIA», «BANDONEON» o «CONTRABAJO». Entonces `own_players_instruments`
da los sonidos del estilo a los músicos propios de la canción, a cada
uno según la parte que toca, y deja la melodía en paz:

```rust
let program = match role(&channel.name, channel.program)? {
    Role::Piano => piano,
    Role::Organ => extras.organ.as_ref().map_or(piano, |organ| organ.program),
    Role::Guitar => extras
        .guitar
        .as_ref()
        .map_or(piano, |guitar| guitar.program),
    Role::Bass => band.bass_program(),
    Role::Strings => extras.strings.as_ref()?.program,
};
```

`role` lee primero el nombre del canal («slapbass», «drive guitar»,
«organ», «warmstrings», o bajo, guitarra, órgano, cuerdas), y solo
después la familia de su sonido General MIDI, contando el contrabajo
como bajo aunque General MIDI lo archive entre las cuerdas. El nombre
va primero porque un estilo puede haber cambiado la familia del
sonido: en una banda sin guitarra, la guitarra toca la parte del piano,
y por su sonido solo seguiría siendo un piano en la banda siguiente,
que tiene un banjo para ella. El código está dentro de un cierre que se
pasa a `filter_map`, que devuelve un `Option`, así que cada `?`
significa «este canal conserva su sonido»: un canal cuyo nombre y
sonido no dicen nada (un coro, un pad), o las cuerdas cuando el estilo
no tiene. La melodía es cualquier canal cuyo nombre lo diga (Lead,
Melody, Melodía, Voice, Voz, Canto), o si no el que mostraría la
partitura de melodía.

La entrada de la banda, `apply_band_parts`, llega al mismo código por
`own_players_sounds`, dejando fuera los canales propios de la banda,
así que los músicos del archivo cambian de sonido tanto si el estilo
toca como banda como si no: el primer estilo sobre un archivo MIDI hace
entrar a la banda, y cada estilo después de él vuelve a dar sus sonidos
al órgano, las guitarras y las cuerdas del archivo.

Dos métodos pequeños corrigen la elección según el tipo de preajuste,
cada uno tomando `self` por valor y devolviendo una copia cambiada:
`for_rhythm` apaga la progresión, que un ritmo no tiene; y `following`
mantiene los compases y el tempo de la canción para un estilo que toca
cada compás en su propio compás. Se encadenan:
`parts.for_rhythm().following(rhythm.follows_the_meter)`.

La elección se guarda entre ejecuciones, en la memoria de egui, con el
nombre `apply-options`. Hasta que la cambies, es todo menos la
progresión de acordes, así que la canción conserva sus propios
acordes:

```rust
const FIRST_OPTIONS: StyleParts = StyleParts {
    rhythm: true,
    instruments: true,
    time_signature: true,
    progression: false,
    tempo: true,
    swing: true,
    matching_style: true,
};
```

`#[serde(default)]` en el struct hace que un campo añadido más tarde se
lea con su valor por defecto desde una copia guardada más antigua. Una
`const` de un struct se escribe entera allí donde se usa, así que
`FIRST_OPTIONS` no cuesta nada al ejecutarse.

### El mismo estilo, la misma canción

Prueba Bluegrass, después Celtic, después Bluegrass otra vez: la
tercera vez debería sonar exactamente como la primera. Un estilo no
debe aplicarse encima del anterior, con el compás y el tempo del
anterior, o cada clic alejaría un poco más la canción de sí misma. Así
que la aplicación guarda, junto a cada versión de la canción que hizo
la Biblioteca, la canción tal como vino, en su propio compás:

```rust
pub(super) struct SongSource {
    original: Project,
    library: Tried,
    /// Latest preset components, applied without changing the source meter.
    reference: Project,
}
```

Cada clic vuelve a empezar desde `reference`, y pregunta si el compás
del estilo es distinto del de *esa* canción, no del que le dejó el
último estilo:

```rust
let changes_meter = !lineup_core::remeter::is_in(&source.reference, time);
```

El tempo sigue la misma idea. **Tempo** pone el tempo del estilo donde
suena la mayor parte de la canción, con `take_tempo` (ver [La franja
de tempo](../sound/tempo.md#la-franja-de-tempo)): una canción que
empieza con un compás lento y luego toca a 121 BPM, con un estilo hecho
para 180, toca a 180 donde tocaba a 121, y su compás lento sigue siendo
igual de más lento que antes. Dar el tempo del estilo al primer compás
de la canción haría que un archivo MIDI con una entrada lenta se
disparara.

## De dónde vino cada parte

Tomar solo algunas partes significa que una canción puede tener su
batería de un estilo, sus partes de acordes y bajo de otro, su tempo de
un tercero y sus acordes de la demostración con la que empezó. La
Biblioteca lo muestra, una línea por parte: Tempo «Tango marcato
(estilo)», Acordes «El Choclo (demostración)», y **editado, era …** en
cuanto cambias tú esa parte. `sources.rs` lleva ese registro.

Las nueve partes son un enum, en el orden en que las lista la
Biblioteca: la batería, las partes de acordes y bajo, la melodía, los
instrumentos, el compás, la tonalidad, los acordes, el tempo y el
swing. Un conjunto de ellas son nueve bits de un `u16`, un bit por
parte:

```rust
pub struct PartSet(u16);
```

```rust
pub fn contains(self, part: SongPart) -> bool {
    self.0 & part.bit() != 0
}
```

Todo cambio que trae partes pasa por una única función, que compara la
canción antes y después:

```rust
pub fn bring_in<R>(
    &mut self,
    source: &Source,
    taken: PartSet,
    edited: PartSet,
    change: impl FnOnce(&mut Project) -> R,
) -> R {
    let before = SongPart::ALL.map(|part| self.part_fingerprint(part));
    let result = change(self);
    for (part, before) in SongPart::ALL.into_iter().zip(before) {
        let now = self.part_fingerprint(part);
        // A key that changed with what was taken came with it: a
        // minor progression puts the song in minor, a row style puts
        // it in its row.
        let came = taken.contains(part)
            || (part == SongPart::Key && now != before && taken != PartSet::NONE);
        if came && !edited.contains(part) {
            self.sources.set(
                part,
                Some(PartSource {
                    source: source.clone(),
                    print: Some(now),
                }),
            );
        } else if now != before && !edited.contains(part) {
            self.follow_part(part, before, now);
        }
    }
    result
}
```

Las partes tomadas ahora vienen de `source`, aunque no hayan cambiado:
un estilo en 4/4 aplicado a una canción que ya está en 4/4 le da igual
su compás. Una parte que cambió solo junto con ellas, como unos acordes
movidos a un compás nuevo, *las sigue*: sigue viniendo de donde venía,
y está como vino si lo estaba antes. La tonalidad es la única parte que
ninguna casilla nombra: viene con lo que la trajo, así que la tonalidad
de una progresión menor viene de su estilo.

El `print` es una **huella**: un hash `u64` de la parte tal como vino,
hecho con `std::hash::DefaultHasher` a partir de sus notas, su tempo,
sus acordes. Si una parte está editada no se guarda en ningún sitio; se
pregunta cada vez, comparando la huella con la parte tal como está
ahora:

```rust
edited: came.print != Some(self.part_fingerprint(part)),
```

Así, editar el tempo lo deja «editado, era …», y volverlo a poner como
estaba, o deshacer, lo deja otra vez como vino, sin código que mantenga
al día un indicador. El archivo del proyecto guarda solo la fuente y un
indicador `edited`, nunca el hash: `DefaultHasher` puede cambiar de
algoritmo en otra versión de Rust, y un hash guardado llamaría entonces
editada a cada parte de cada archivo antiguo. Al cargar, la huella se
vuelve a calcular a partir de la parte tal como está.

Tres de las partes son notas de los mismos patrones: la batería, la
melodía y las partes de acordes y bajo, que son todo lo demás. Se
distinguen por canal, con un único ayudante que calcula el hash de las
notas de los canales que elige un cierre:

```rust
SongPart::Melody => self.hash_notes(&mut hasher, is_lead),
```

y las partes de acordes y bajo pasan `|channel| !is_drum(channel) &&
!is_lead(channel)`. Con una banda, la batería y las partes de acordes y
bajo son lo que toca la banda, así que sus huellas calculan en cambio
el hash de los nombres de los estilos de los que vienen.

Las versiones anteriores conocían solo seis de las partes, y
rechazarían un archivo que nombrara una que no conocen. Así que la
batería, la melodía y la tonalidad se guardan en un campo propio,
`more_sources`, que esas versiones se saltan. Se escribe siempre que
alguna parte vino de algún sitio, aunque esté vacío, así que un archivo
sin él es de antes; para un archivo así, `sources_of_an_older_file`
deduce de dónde vinieron esas tres: la batería con los patrones, la
melodía de una demostración con la demostración, y la tonalidad con los
acordes.

Un detalle más. El registro de fuentes no es la música, y dos
canciones que suenan igual deberían ser iguales: la aplicación compara
proyectos para saber si hay cambios sin guardar. Así que su `PartialEq`
dice que dos registros cualesquiera son iguales:

```rust
impl PartialEq for Sources {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
```

Un `impl` escrito a mano es la manera en que un tipo participa en un
`PartialEq` derivado (el del `Project`) sin cambiar la respuesta.

## La pestaña Biblioteca

La Biblioteca lista los estilos armónicos, los ritmos y tus propios
estilos en un único catálogo, ordenado por los nombres tal como se
muestran en el idioma elegido:

```rust
pub enum Preset<'a> {
    Style(&'static Style),
    Rhythm(&'static Rhythm),
    User(usize, &'a UserStyle),
}
```

Dos tipos de entrada toman prestado de tablas que viven para siempre;
tus propios estilos viven en la aplicación, así que `Preset` tiene un
tiempo de vida, `'a`, que dice que no puede vivir más que la lista de
la que toma prestado. Cada pregunta que hace la lista (el nombre, el
compás, el tempo, si trae una progresión) es un método con un único
`match`, así que el resto del navegador nunca pregunta de qué tipo es.
Un ritmo que es justo la batería de un estilo con el mismo nombre,
tempo y aire se lista una sola vez, como el estilo.

Las casillas bajo la búsqueda dejan solo los preajustes parecidos a la
canción, «En 4/4» (en su compás) o «Cerca de 120 BPM» (cerca de su
tempo), o los que tienen lo que buscas: «Con acordes» (una progresión),
«Swing» (una batería con swing) o un instrumento elegido en un menú,
«Cualquier instrumento». Lo que miran en un preajuste es un struct
`Copy` pequeño,

```rust
struct Traits {
    time: TimeSignature,
    /// The speed it was made for, in beats a minute.
    tempo: f32,
    /// Whether it plays every bar in its own meter, so it fits any time
    /// signature.
    any_meter: bool,
    /// Whether it brings a chord progression.
    chords: bool,
    /// Whether its drums swing.
    swing: bool,
}
```

y la decisión es un único método:

```rust
fn keeps(self, traits: Traits, instruments: impl FnOnce() -> Vec<Instrument>) -> bool {
    let filters = self.filters;
    // 7/8 as 2+2+3 is still 7/8.
    let same_time = traits.any_meter || traits.time.ungrouped() == self.time.ungrouped();
    let ratio = traits.tempo / self.tempo;
    let near = ratio.max(1.0 / ratio) <= NEAR_SPEED;
    (!filters.time || same_time)
        && (!filters.speed || near)
        && (!filters.chords || traits.chords)
        && (!filters.swing || traits.swing)
        && filters
            .instrument
            .is_none_or(|instrument| instruments().contains(&instrument))
}
```

Las líneas se leen como reglas unidas por «y»:
`!filters.time || same_time` es «o la casilla no está marcada, o el
compás es el mismo». `ratio.max(1.0 / ratio)` convierte «la mitad de
rápido» (0,5) y «el doble de rápido» (2,0) en el mismo número, 2, así
que más rápido y más lento se juzgan igual; con `NEAR_SPEED` en 1,15,
una canción a 130 BPM conserva los preajustes hechos para 113 a 149,5.
Comparar proporciones en lugar de una diferencia en BPM trata igual las
canciones lentas y las rápidas: ±15 BPM sería mucho a 60 y poco a 180.
`keeps` toma `self` por valor: el struct es `Copy` y ocupa unos pocos
bytes, así que no cuesta más que una referencia.

Los instrumentos son la única pregunta que cuesta algo: los de un
preajuste son los de la banda que lo tocaría, calculados a partir de su
estilo. Así que llegan como un cierre, `instruments`, al que se llama
solo al final, cuando hay un instrumento elegido. `Option::is_none_or`
es `true` para `None`, ningún instrumento elegido, y si no pregunta al
cierre por el que hay dentro. `&&` se detiene en el primer `false`, así
que un preajuste que una casilla anterior ya dejó fuera nunca llega a
calcular sus instrumentos.

La lista se agrupa por género, un `egui::CollapsingHeader` por género
con su cuenta y tus propios estilos primero bajo **Estilos de
usuario**, salvo que **Ajustes > Biblioteca de estilos** diga **Lista
alfabética**. Una búsqueda abre cada grupo que tiene una coincidencia.
Cuando la canción cambia de estilo, el grupo del nuevo se abre durante
medio segundo, lo justo para que egui desplace su fila iluminada hasta
la vista, y luego deja los grupos como los tenías.

### Probar, cambiar y deshacer

Lo que hace un clic depende de la canción. Con una banda, como en una
canción de **Archivo > Nuevo** o en una demostración, el estilo pasa a
ser la manera en que la banda toca la hoja de acordes (ver [Cómo toca
la banda una hoja de acordes](band.md)). Mientras la canción
suena sin banda, el clic cambia la canción entera, así que la música
sigue. Si no, escribe el estilo en un **patrón de prueba** propio y lo
toca ahí.

Hacer clic en otro estilo debería *reemplazar* la prueba, no ir
apilando patrones, para poder hojear la biblioteca. Pero en cuanto
empiezas a editar ese patrón, es tuyo. La aplicación distingue los dos
casos guardando una copia de lo que escribió:

```rust
pub(super) struct Audition {
    pattern: usize,
    /// The pattern as the library left it. If it still looks like this,
    /// the next click may rewrite it.
    written: Pattern,
    style: Option<&'static Style>,
    rhythm: Option<&'static Rhythm>,
    row_style: Option<&'static RowStyle>,
    micromode_style: Option<&'static MicromodeStyle>,
}
```

```rust
fn unchanged_audition(&self) -> Option<&Audition> {
    let pattern = self.project.pattern(self.pattern)?;
    self.auditions
        .iter()
        .rev()
        .find(|audition| audition.pattern == self.pattern && &audition.written == pattern)
}
```

`==` entre dos `Pattern`s compara cada nota, gracias al `PartialEq`
derivado: en lugar de seguir cada edición, se guarda una copia y se
compara, la misma idea que con los cambios sin guardar. La aplicación
guarda las últimas 64 pruebas, no solo la más nueva, y busca desde la
más nueva con `rev()`. Eso es por el deshacer: después de que Ctrl+Z
traiga de vuelta el estilo anterior, el patrón coincide con una copia
más antigua, y el siguiente clic lo sigue cambiando en el mismo lugar.

Los estilos armónicos y los ritmos toman las partes que marcaste (ver
[Qué partes tomar](#qué-partes-tomar)). Los estilos de series y los de micromodos no tienen esa
elección: se prueban de la misma manera, y solo se diferencian en las
notas que escriben. Así que sus pasos comunes son un único método, y el
paso distinto es un closure que se le pasa:

```rust
fn write_pattern_style(
    &mut self,
    name: &'static str,
    tempo: f32,
    rhythm: Option<&'static Rhythm>,
    write: impl FnOnce(&mut Project, usize),
) -> Option<(usize, Pattern)> {
```

`impl FnOnce(&mut Project, usize)` acepta cualquier closure que se
pueda llamar una vez con esos dos argumentos. El compilador hace una
copia del método para cada closure que se le pasa, así que la llamada
no cuesta más que el código escrito a mano. `FnOnce` es el que menos
pide de los tres traits de closure (`Fn`, `FnMut`, `FnOnce`): todo
closure es al menos `FnOnce`, así que pedirlo los acepta todos.

Cada escritura pasa por `edit`, así que una prueba es un solo paso de
deshacer, el hilo de audio se entera por el diff de siempre, y guardar
funciona sin más. Dentro, `bring_in` registra de dónde vinieron las
partes.

## Cómo se prueba

Las tablas son datos, y los datos pueden estar mal tan fácilmente como
el código, sin que nada se rompa. Así que las pruebas leen la
biblioteca entera: `every_style_parses_and_fits_in_a_pattern`,
`names_are_unique_and_rhythms_exist`, `every_genre_has_styles`,
`styles_play_in_any_key`, y para los ritmos
`every_line_is_whole_bars_of_known_characters`,
`names_are_unique_and_fills_exist` y `congas_play_high_and_low`.
`a_new_time_signature_keeps_the_kind_of_rhythm`,
`an_odd_meter_gets_a_rhythm_grouped_its_way` y
`a_time_signature_with_no_rhythm_gets_a_basic_beat` siguen a
`for_time` por su cadena.

`apply.rs` comprueba adónde van las notas (`styles_go_to_keys_and_bass`,
`chords_go_around_a_locked_channel`), que `recomping_keeps_the_chords`,
que una canción entera toma un ritmo o un estilo nuevos, que un vals
pone la canción en 3/4 y, la más amplia,
`every_demo_takes_every_kind_of_library_click`. `sources.rs` comprueba
una parte que llega, se edita y se deshace:

```rust
#[test]
fn a_part_brought_in_is_edited_once_changed_and_not_after_undoing_it() {
    let mut project = Project::demo();
    project.bring_in_all(&demo());
    assert_eq!(from(&project, SongPart::Tempo), Some(("El Choclo", false)));
    let tempo = project.tempo;
    project.scale_tempo(Tempo::new(tempo.bpm() + 10.0));
    assert_eq!(from(&project, SongPart::Tempo), Some(("El Choclo", true)));
    assert_eq!(from(&project, SongPart::Swing), Some(("El Choclo", false)));
    project.scale_tempo(tempo);
    assert_eq!(from(&project, SongPart::Tempo), Some(("El Choclo", false)));
}
```

Para las casillas,
`a_styles_sounds_go_to_an_imported_songs_players_by_family_but_the_tune`
arma una canción como un tango importado, con un «PIANO MELODIA», un
bandoneón, una guitarra, un contrabajo, cuerdas y un coro, y comprueba
qué canales toman los sonidos de Tango y de Bossa nova y cuáles
conservan los suyos;
`a_songs_keys_and_bass_still_take_a_styles_sounds_alone` comprueba que
una canción con Teclados y Bajo cambia solo ahí. En `band.rs`,
`drums_and_chord_parts_come_on_their_own_from_builtin_and_saved_styles`
da a una banda de Rock ride primero la batería de Bossa nova, del estilo
incorporado y de una copia guardada de él, y comprueba que el piano y el
bajo quedan como estaban, que el archivo se vuelve a leer igual y que,
cuando llegan también las partes de acordes y bajo, la banda vuelve a
ser simplemente Bossa nova, en el formato que necesitaba antes;
`a_styles_chord_parts_can_come_without_its_drums` toma las partes de
acordes y bajo de Bossa nova sobre la batería de Rock ride, lo que
necesita el formato 29. `a_style_tempo_goes_where_most_of_the_song_is`
comprueba `take_tempo` en una canción con un primer compás lento.

En la aplicación, `the_library_tick_boxes_keep_only_presets_like_the_song`
cuenta los preajustes listados a medida que se marcan las casillas;
`the_same_style_brings_the_same_tempo_whatever_was_tried_before` pulsa
Bluegrass, Celtic, Bluegrass y Celtic en una canción en 2/4 y en 4/4, y
comprueba que cada estilo trae el mismo tempo cada vez; y
`trying_row_and_micromode_styles_rewrites_one_pattern_while_it_is_unchanged`
comprueba que las pruebas caen en un único patrón y que cada una es un
solo paso de deshacer.

Los ejercicios sobre la biblioteca están en [Ejercicios: la
banda](exercises.md).
