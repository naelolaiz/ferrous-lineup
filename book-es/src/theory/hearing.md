# Oír la armonía: acordes y tonalidades a partir de las notas

Archivos: [`crates/lineup-core/src/harmony/hearing.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/hearing.rs),
[`crates/lineup-core/src/harmony/key_finding.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/key_finding.rs),
[`crates/lineup-core/src/library/listen.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/listen.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs),
[`crates/lineup-app/src/app/key_finder.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/key_finder.rs)
y [`crates/lineup-core/tests/chord_reading.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/tests/chord_reading.rs)

Un patrón guarda notas, no acordes, y un archivo MIDI importado o una
canción que tocaste no tienen acordes escritos en ningún sitio. Aun así,
muchas partes de la aplicación necesitan la armonía: **Escribir los
acordes que toca la canción** en la hoja de acordes, la fila **Todo**
del carril de acordes del piano roll, la tecla de armonía que se
ilumina mientras suena la canción, la comprobación de la conducción de
voces, un estilo que vuelve a tocar una canción entera con otro ritmo,
y la tonalidad de un archivo MIDI sin armaduras. Todas escuchan las
notas como lo hace un músico de oído. Este capítulo trata de cómo:
primero los acordes, después las tonalidades.

`analyze`, en [Acordes: construirlos, nombrarlos e
invertirlos](chords.md#nombrar-lo-que-suena), nombra las
notas que suenan en un momento, y eso está bien para un acorde
sostenido en un teclado. La música que se mueve necesita más.

## La música, en pocas palabras

Nombrar las notas de un solo momento falla de cinco maneras comunes:

- Un **arpegio** toca su acorde una nota cada vez: Do, luego Mi, luego
  Sol. En cada momento suena una sola nota, y una nota no nombra ningún
  acorde.
- Una **melodía** pasa por notas que no son del acorde. Sobre un acorde
  de Do mayor, una melodía en Re hace que el momento sea Do, Re, Mi,
  Sol, que se lee como Cadd9 aunque la armonía sea un Do mayor sin más.
- Un **bajo caminante** (*walking bass*) va de un acorde a otro por
  grados: Do, Si♭, La camino de C a A7. El Si♭ debajo de Do, Mi y Sol
  forma C7 por un momento, aunque nadie toque C7.
- Una orquesta puede tocar **dos acordes a la vez**, uno sobre otro: un
  **poliacorde**. En los Augurios primaverales de *La consagración de la
  primavera* de Stravinsky, las cuerdas graves tocan Fa♭ mayor (Fa♭,
  La♭, Do♭) mientras arriba suena E♭7 (Mi♭, Sol, Si♭, Re♭). En
  *Petrushka* los clarinetes tocan Do mayor sobre Fa♯ mayor. Ningún
  cifrado de un solo acorde contiene ninguno de los dos.
- Algunas armonías **no son superposiciones de terceras**. La feria de
  *Petrushka* empieza con una figura de la flauta en Re, Mi, Sol y La,
  que se apilan por cuartas: Mi, La, Re, Sol. Llamar acorde a eso le
  impone un nombre.

Una **tonalidad** se oye del mismo modo, a lo largo del tiempo. En Do
mayor, Do, Mi y Sol vuelven más que ninguna, luego las demás teclas
blancas, y las negras casi nunca. La menor usa las mismas teclas
blancas, pero mandan La, Do y Mi, y Sol♯ aparece en cada cadencia. Una
pieza **modula** cuando pasa a una tonalidad nueva y se queda en ella;
un solo acorde de otra tonalidad, como D7 llevando a G en una canción
en Do mayor (una *dominante secundaria*), solo *toniciza* Sol un
momento y no es un cambio de tonalidad.

## Escuchar un tramo, no un momento

La escucha de acordes escucha un **tramo** de música, un pulso, medio
compás o un compás, y pesa cada nota según cuánto suena en él. Una nota
escuchada es un `HeardNote`, con tiempos en pasos desde el inicio del
tramo:

```rust
pub struct HeardNote {
    pub pitch: Note,
    /// When it starts, in steps from the start of the stretch: below 0
    /// for a note that started before it.
    pub start: f32,
    /// When it ends, in steps from the start of the stretch: beyond its
    /// length for a note that goes on after it.
    pub end: f32,
    /// Which part plays it, such as a channel: an instrument's notes are
    /// a layer of their own when two chords sound at once.
    pub part: usize,
}
```

Los tiempos son `f32`, no pasos enteros, porque una nota puede empezar
entre dos pasos (su `nudge`, ver [El proyecto: patrones, canales,
pistas, clips y marcas](../program/project.md#notas-pasos-y-patrones)).
Las notas también pueden quedar del todo fuera del tramo. Esas no
pesan nada, pero cuentan qué hace cada parte alrededor, como muestra la
sección siguiente. `Project::heard_notes` reúne las notas de todos los
canales que tocan notas (los sintetizadores y los canales MIDI Out que
no están silenciados; la batería y los samplers no) entre dos pasos de
la canción.

Sumar cuánto suena cada una de las doce clases de altura da un
**perfil**:

```rust
struct Profile {
    weight: [f32; 12],
    total: f32,
    /// How long the pitch class heard longest sounds.
    loudest: f32,
}
```

Un arpegio de Do mayor a lo largo de un compás es Do, Mi y Sol sonando
cada uno un tercio del compás: las mismas proporciones que un acorde
sostenido, así que el arpegio se oye como su acorde. Esa sola idea
resuelve el primer problema de la lista. Los demás necesitan saber qué
hace cada parte.

## Qué hace cada parte

Cuánto pesa una nota depende de la parte que la toca. `weights`
distingue cuatro papeles:

| Papel | Cómo se reconoce | Cuánto pesan sus notas |
|------|----------------|----------------------|
| Acordes | en ella suenan juntas tres notas distintas o más, o toca acordes nota a nota | entero |
| Línea de bajo | una nota cada vez, todas por debajo de los acordes | entero si los acordes tienen la nota; 0,15 si no, una nota de paso |
| Melodía | cualquier otra parte, cuando alguna parte toca acordes | un cuarto |
| Capa | una parte que sería una melodía, pero cuyas notas son tres o más de una misma tríada o acorde de séptima, ninguna de ellas de los acordes | entero |

El final de `weights` aplica la tabla, nota por nota:

```rust
let weight = if tune {
    w * TUNE
} else if lowest_chord.is_some()
    && below
    && !chord_notes.contains(note.pitch.pitch_class())
{
    w * PASSING_BASS
} else {
    w
};
```

`w` es cuánto suena la nota dentro del tramo. Cuando ninguna parte toca
acordes, como en una orquesta donde cada instrumento toca una línea,
`lowest_chord` es `None`, nadie es una melodía y cada línea cuenta
entera.

La fila de la **capa** está ahí por *La consagración de la primavera*.
Su ostinato Re♭, Si♭, Mi♭, Si♭ es una sola línea, pero deletrea E♭7
contra los arpegios de Do mayor y Mi menor que suenan debajo: una
armonía propia, no una melodía.

Dos preguntas más necesitan los compases de alrededor, porque un solo
pulso no puede contestarlas:

- ¿Una parte **mueve sus acordes junto con una melodía**, todas las voces
  en el mismo sentido por grado o por tercera, como en las últimas
  exposiciones del *Bolero* de Ravel? Entonces sus acordes colorean la
  melodía y la armonía es lo que pasa debajo, así que la parte cuenta
  como melodía. Eso es `moves_in_parallel`; dos acordes no bastan para
  mostrar un movimiento paralelo, pero una frase sí.
- ¿Una parte **toca acordes nota a nota**, como un bajo de Alberti o los
  tresillos de la sonata *Claro de luna*? Notas cortas, casi todas
  saltos de tercera menor o más, sobre tres clases de altura por lo
  menos: `breaks_chords`. Entonces la parte toca acordes tanto como una
  que los golpea enteros.

## Una puntuación para cada acorde

Cada acorde, doce fundamentales por cada clase de acorde que conoce la
aplicación, recibe una puntuación según lo bien que explica el perfil:

```rust
fn chord_fit_with(chord: Chord, profile: &Profile, also: PitchSet) -> (f32, bool) {
    if profile.total <= 0.0 {
        return (f32::NEG_INFINITY, false);
    }
    let explained = PitchSet::from_bits(chord.pitch_set().bits() | also.bits());
    let inside: f32 = explained.iter().map(|p| profile.of(p)).sum();
    let outside = profile.total - inside;
    let full = PRESENT * profile.loudest;
    let mut missing = 0.0;
    let mut core = true;
    for (&interval, &tone) in chord.quality.intervals().iter().zip(chord.quality.tones()) {
        let pitch_class = chord.root.transpose(i32::from(interval));
        let presence = (profile.of(pitch_class) / full).min(1.0);
        let loss = if tone == 5 && interval % 12 == 7 {
            MISSING_FIFTH
        } else {
            MISSING
        };
        missing += loss * (1.0 - presence);
        if matches!(tone, 1..=4) && presence < 0.5 {
            core = false;
        }
    }
    let extra = chord.quality.intervals().len().saturating_sub(3) as f32;
    let fit = (inside - OUTSIDE * outside) / profile.total - missing - SIZE * extra;
    (fit, core)
}
```

- `inside` es cuánto del sonido explica el acorde, y `outside` lo que
  deja fuera, que cuenta una vez y media (`OUTSIDE = 1.5`): una nota
  equivocada es una prueba más fuerte que una acertada. Dividido por el
  total, un acorde que lo explica todo puntúa 1.
- `missing` es una pequeña pérdida por cada nota del acorde que apenas
  se oye: 0,15 por una nota que falta y solo 0,05 por una quinta justa
  que falta, porque los músicos la omiten sin problema. Una nota que
  suena al menos el 35% de lo que suena la más fuerte (`PRESENT`) cuenta
  como presente del todo.
- `extra` cuenta las notas más allá de una tríada, 0,06 cada una. Un
  acorde de decimotercera contiene casi todas las notas de una escala,
  así que sin este coste lo explicaría todo; gana solo cuando sus notas
  suenan de verdad.
- `core` dice si la fundamental y la tercera (o la nota que hace sus
  veces en un acorde sus) se oyen de verdad. Un acorde sin ellas se
  descarta.

`PitchSet` guarda un conjunto de clases de altura como los bits de un
solo `u16` (ver [Clases de altura, notas y su
escritura](pitch.md#un-conjunto-de-clases-de-altura-en-doce-bits)).
`|` sobre los bits es la unión de dos conjuntos, así que `explained` es
«las notas del acorde y las de `also`» en una sola instrucción. `also`
está vacío para un acorde solo; en un poliacorde guarda las notas del
otro acorde.

El bajo da el último toque:

```rust
fn bass_fit(chord: Chord, bass: Option<PitchClass>) -> f32 {
    match bass.map(|bass| (bass, chord.tone_of(bass))) {
        Some((bass, _)) if bass == chord.root => BASS_ROOT,
        Some((_, Some(3 | 5 | 7))) | None => 0.0,
        Some(_) => -BASS_OUTSIDE,
    }
}
```

Una fundamental en el bajo gana 0,06; una tercera, quinta o séptima es
una inversión y no cuesta nada; cualquier otra nota cuesta 0,06. El
brazo del medio muestra dos herramientas de los patrones: `3 | 5 | 7`
dentro de `Some(...)` encaja con cualquiera de los tres valores, y
`| None` une al brazo un segundo patrón completo.

### Un bajo caminante, paso a paso

Un piano sostiene Do, Mi y Sol durante dos pulsos (ocho pasos) mientras
el bajo camina Do y luego Si♭, bajando hacia La para el acorde
siguiente. El bajo toca por debajo de los acordes y Si♭ no es una de sus
notas, así que pesa 4 × 0,15 = 0,6. El perfil es Do 12 (piano y bajo),
Mi 8, Sol 8 y Si♭ 0,6, 28,6 en total; `loudest` queda limitado a la
duración del tramo, 8, para que una nota doblada en dos partes no haga
parecer débiles a las demás.

- **C** lo explica todo menos el Si♭: (28 − 1,5 × 0,6) / 28,6 = 0,948,
  sin nada que falte, más 0,06 por tener su fundamental en el bajo:
  **1,008**.
- **C7** lo explica todo, 1,0. Pero su Si♭ suena 0,6 de los 2,8 que
  cuentan como presencia completa, lo que cuesta 0,15 × (1 − 0,21) =
  0,12, y su cuarta nota cuesta 0,06. Con el premio del bajo: **0,882**.

Gana C, como diría un cifrado, y C7 ni siquiera se guarda como segunda
lectura (para eso tendría que quedar a 0,12 o menos del mejor). Si la
nota de paso pesara 0,35, C seguiría ganando, por 0,941 a 0,925, pero C7
aparecería como alternativa cercana.

### Quedarse con los tres mejores

Solo los tres mejores acordes se convierten en lecturas, así que no hace
falta ordenarlos todos. Un vector corto se mantiene ordenado a medida
que llegan los acordes:

```rust
let at = found.partition_point(|&(_, other): &(Chord, f32)| other >= fit);
if at < MOST_READINGS {
    found.insert(at, (chord, fit));
    found.truncate(MOST_READINGS);
}
```

`partition_point` hace una búsqueda binaria del primer lugar donde la
condición pasa a ser falsa: el primer acorde que encaja peor que el
nuevo. Un acorde que encaja exactamente igual va detrás de los que ya
están, así que el resultado es el mismo que el de un ordenamiento
estable, con mucho menos trabajo.

## Dos acordes a la vez

Si ningún acorde solo encaja bien (una puntuación por debajo de 0,7,
`ONE_CHORD`), la escucha prueba con dos. `best_polychord` divide las
notas en una capa inferior y otra superior de todas las maneras que
tienen sentido:

- **por registro**: todo lo que está por debajo de una altura es la capa
  inferior. Se salta una división que cortaría una línea, porque una
  línea que va de La hasta Sol es un arpegio de A7, no la parte de
  arriba de Sol mayor;
- **por instrumento**: las partes ordenadas de la que suena más grave a
  la más aguda, cortadas entre dos cualesquiera.

Cada capa tiene que formar una tríada mayor o menor, o un acorde de
séptima mayor, menor o de dominante, con tres de sus notas oídas y su
fundamental entre ellas. Dos acordes más grandes a la vez explicarían
cualquier nota. Después la pareja tiene que pasar cuatro pruebas:

1. Las fundamentales son distintas, y el acorde de arriba añade al menos
   dos notas propias. C sobre La menor es solo Am7.
2. Entre los dos acordes no forman **una sola tonalidad**. Fa mayor sobre
   Sol mayor usa solo las notas de Do mayor: eso es G con oncena, una
   sola armonía en Do, no dos tonalidades a la vez.
3. Los dos acordes **suenan juntos** durante al menos el 80% del tiempo
   en que suena algo en el tramo (`covered`). Do mayor y luego G7 es una
   progresión, no un poliacorde. El oído sostiene las notas de un
   arpegio, o de un acorde en staccato golpeado una y otra vez, así que
   un hueco de dos pasos o menos cuenta como sonando.
4. La pareja, menos 0,2 por necesitar dos acordes (`POLYCHORD`), puntúa
   mejor que el mejor acorde solo.

La segunda prueba son unas pocas líneas:

```rust
fn in_one_key(notes: PitchSet) -> bool {
    let major = PitchSet::from_semitones(&[0, 2, 4, 5, 7, 9, 11]);
    (0..12).any(|tonic| notes.is_subset_of(major.transpose(tonic)))
}
```

`transpose` rota los bits, así que esto prueba las doce escalas mayores,
y con ellas sus modos. ¿Por qué solo escalas mayores? Porque el propio
acorde de los Augurios encaja en una escala **menor armónica**. Sus siete
notas, Fa♭, Sol, La♭, Si♭, Do♭, Re♭ y Mi♭, son exactamente La♭ menor
armónica. Si la prueba contara la menor armónica como una tonalidad, el
acorde habría quedado descartado.

El acorde de Petrushka pasa del mismo modo. Do mayor y Fa♯ mayor están a
un tritono y no comparten ninguna nota, y entre los dos tienen seis
notas, Do, Do♯, Mi, Fa♯, Sol y La♯, que ninguna escala mayor contiene.
Las dos tríadas salen de la escala **octatónica**, Do, Re♭, Mi♭, Mi,
Fa♯, Sol, La, Si♭, que alterna semitonos y tonos.

## Un conjunto cuando ningún acorde encaja

Por último, las notas se puntúan como un **conjunto de clases de
altura** (ver [Conjuntos de clases de altura, y una armonía con cuatro
nombres](sets.md)). Un conjunto explica todas las notas que
se oyen lo bastante como para contar, así que siempre encaja, pero paga
0,35 por renunciar a un nombre de acorde. Una superposición de cuartas
paga solo 0,1, porque desde Debussy es una armonía por derecho propio,
tan clara al oído como una tríada:

```rust
/// The sets that are stacks of fourths: C F Bb (3-9, the same notes as a
/// sus chord), C F Bb Eb (4-23) and C F Bb Eb Ab (5-35, the pentatonic
/// scale), as their prime forms.
const QUARTAL_SETS: [&[u8]; 3] = [&[0, 2, 7], &[0, 2, 5, 7], &[0, 2, 4, 7, 9]];
```

El Re, Mi, Sol y La de la flauta al principio de la feria de *Petrushka*
es 4-23, así que se nombra como ese conjunto, en lugar de con un acorde
forzado como D6sus2.

## La mejor lectura, y las que le siguen de cerca

Un `Hearing` guarda todas las lecturas que quedan a 0,12 o menos de la
mejor (`CLOSE`), tres como máximo:

```rust
pub struct Hearing {
    readings: Vec<Reading>,
    /// The pitch classes heard enough to be part of the harmony, without
    /// the passing notes.
    notes: PitchSet,
    /// How long each pitch class sounds, weighed.
    profile: [f32; 12],
}
```

`best()` es la primera lectura, o `Harmony::Silence` si no hay ninguna,
y `alternatives()` el resto:

```rust
pub fn alternatives(&self) -> &[Reading] {
    self.readings.get(1..).unwrap_or_default()
}
```

`get(1..)` devuelve `None` para un vector vacío, donde
`&self.readings[1..]` provocaría un pánico, y `unwrap_or_default`
convierte ese `None` en un slice vacío, el valor por defecto de
`&[Reading]`.

Los campos son privados, así que un `Hearing` solo se puede crear
escuchando: nada fuera del módulo puede construir uno con las lecturas
desordenadas. La aplicación muestra las lecturas como muestra sus demás
indicaciones, la más probable más brillante y las otras más tenues:

- Junto a la sugerencia de las teclas de armonía, una línea dice qué se
  escucha cuando no es simplemente un acorde: «Se escucha: Eb7|Fb, dos
  acordes a la vez», «Se escuchan las notas 4-23», o un acorde seguido
  de «o» y sus rivales cercanos. En las teclas de armonía, cada acorde de
  un poliacorde ilumina su tecla.
- La línea de estado del piano roll, para el paso que está bajo el
  puntero, añade las otras lecturas entre paréntesis: «(o C6)».

Una última regla protege a una melodía sola. *Para Elisa* empieza con
Mi, Re♯, Mi, Re♯, Mi, Si, Re, Do y nada debajo: una melodía, no un
acorde, aunque sumadas sus notas se ganarían algún nombre de acorde. Así
que cuando una sola línea suena sola y su mejor lectura encaja menos de
0,85 (`ALONE`), la escucha no nombra ninguna armonía, y sigue el acorde
de antes.

## Cortar un compás en acordes

**Escribir los acordes que toca la canción** llena la hoja de acordes a
partir de las notas. También se ejecuta sola cuando llega un estilo de
banda a una canción con la hoja vacía, porque la banda toca a partir de
la hoja. Recorre la canción compás a compás y pregunta: ¿dónde hay que
cortar este compás? Un compás de 4/4 tiene líneas de pulso en 0, 1, 2, 3
y 4, y dos cualesquiera de ellas forman un tramo: diez tramos, desde un
solo pulso hasta el compás entero. La mejor manera de cortar se
encuentra con **programación dinámica**:

```rust
// best[i]: the best score for the beats before line i, where its
// last stretch starts, and how that stretch is heard.
let mut best: Vec<(f32, usize, Option<Hearing>)> = vec![(0.0, 0, None)];
for i in 1..lines.len() {
    let mut here = (f32::NEG_INFINITY, 0, None);
    for j in 0..i {
        let hearing = heard(lines[j], lines[i]);
        let cut = match lines[j] {
            line if line == bar.start => 0.0,
            line if line == half => CUT_HALF_BAR,
            _ => CUT_BEAT,
        };
        let total = best[j].0 + worth(&hearing) * (i - j) as f32 - cut;
        if total > here.0 {
            here = (total, j, Some(hearing));
        }
    }
    best.push(here);
}
```

La idea: la mejor manera de cortar los primeros `i` pulsos termina con
algún último tramo, de la línea `j` a la línea `i`. Sea cual sea `j`,
los cortes anteriores tienen que ser la mejor manera de cortar los
primeros `j` pulsos, o habría una respuesta mejor. Así que `best[i]` se
calcula a partir de los `best[j]` anteriores, que ya se conocen,
probando cada `j`. Un compás de cuatro pulsos se puede cortar solo de
2³ = 8 maneras, pero el mismo tramo aparece en muchas de ellas, y así
cada uno de los diez tramos se escucha una sola vez. El oyente de la
biblioteca, más abajo, corta un patrón entero del mismo modo, donde
probar todas las maneras sería imposible: un patrón de 64 pulsos se
puede cortar de 2⁶³ maneras, y el bucle hace unas 64 × 64 / 2
preguntas.

Cada tramo gana su puntuación una vez por cada pulso que cubre
(`worth`), y cada corte cuesta un poco: 0,08 a mitad de compás, 0,12 en
cualquier otro pulso. Así un compás se divide solo donde dos acordes
encajan de verdad mejor que uno, y con más facilidad a la mitad que en
un pulso débil, como cambian los acordes en la música real.
`f32::NEG_INFINITY` es menor que cualquier otro número de coma flotante,
así que el primer total real siempre lo supera, sin un caso especial.
Recorrer los `j` hacia atrás desde el final da los cortes, el último
primero, y `reverse` los pone en orden.

`worth` da a un silencio la puntuación completa, y a una línea sola un
poco menos:

```rust
fn worth(hearing: &Hearing) -> f32 {
    match hearing.best() {
        Harmony::Silence => 1.0,
        Harmony::Single(_) | Harmony::Interval { .. } => 0.8,
        _ => hearing.fit(),
    }
}
```

Un pulso de silencio, o de una melodía sola, se une entonces al tramo
que lo rodea en lugar de quedar cortado aparte, y no escribe nada: sigue
el acorde de antes.

Un poliacorde también **continúa** mientras continúan sus notas. En la
habitación de Petrushka, la mitad de Fa♯ mayor del acorde descansa un
compás mientras sigue Do mayor. Escuchado solo, ese compás es C. Pero Do
mayor y Fa♯ mayor siguen explicando casi todo, así que donde ninguna
otra cosa encaja bien la hoja mantiene C|F♯ en lugar de saltar a C y
volver (`Hearing::goes_on`).

### Escuchar rápido

Escribir la hoja escucha diez tramos de cada compás de 4/4, y las
pruebas escriben las hojas de todas las demostraciones, así que la
velocidad importa. Tres cosas la mantienen rápida.

**Hacer una sola vez el trabajo compartido.** Qué partes se mueven en
paralelo, y cuáles tocan acordes nota a nota, depende del compás y de
los compases de alrededor, no del tramo. Un pequeño struct lo calcula
una vez y escucha los tramos a partir de ahí:

```rust
pub struct Listening<'a> {
    notes: &'a [HeardNote],
    /// The parts that move their chords along with a tune.
    planing: Vec<usize>,
    /// The parts that play chords one note at a time.
    broken: Vec<usize>,
}
```

`'a` es un **tiempo de vida** (*lifetime*). `Listening` toma prestadas
las notas en lugar de copiarlas, y `'a` le dice al compilador que un
`Listening` no puede vivir más que el slice que toma prestado.
`hear(start, end)` elige entonces las notas de un tramo y las vuelve a
medir desde su inicio. El piano roll hace lo mismo para su fila Todo:
reúne una vez las notas de cada compás y escucha sus pulsos a partir de
ellas.

**No hacer por cada nota una pregunta que se puede hacer por cada
parte.** Que una parte sea una melodía depende de todas sus notas.
Preguntarlo otra vez para cada nota del tramo es un trabajo de orden n²;
`weights` pregunta una vez por parte, guarda las respuestas en `roles`
y busca allí la parte de cada nota.

**No escuchar dos veces el mismo compás.** Las canciones se repiten. Un
compás tocado otra vez exactamente como antes, entre los mismos
compases, se escucha igual, así que `write_heard_chords` guarda un
`HashMap` que va de las líneas de pulso y las notas de cada compás a
cómo se cortó:

```rust
type HeardBars = HashMap<(Vec<usize>, Vec<(i64, i64, u8, usize)>), Vec<(usize, Hearing)>>;
```

Las notas entran en la clave como números enteros, `(start, end, pitch,
part)`, porque un `f32` no puede ser clave: no tiene `Hash` ni `Eq`,
porque `NaN` no es igual a sí mismo. `type` da un nombre corto al tipo
largo; es el mismo tipo, no uno nuevo.

Para encontrar lo que es lento, hay que medir. `valgrind
--tool=callgrind` ejecuta un programa sobre un procesador simulado y
cuenta cada instrucción, por función y por línea:

```sh
CARGO_PROFILE_RELEASE_DEBUG=true cargo test --release -p lineup-core --no-run
valgrind --tool=callgrind ./target/release/deps/<test binary> --ignored one
callgrind_annotate --inclusive=yes callgrind.out.<pid>
```

Señala la función, y la línea, en que se va el tiempo; una pregunta
hecha por cada nota que podría hacerse por cada parte es el tipo de
cosa que encuentra.

### Escribirlo

Cada tramo se convierte en una marca de acorde donde cambia la armonía,
y otra vez donde empieza una sección, como un cifrado vuelve a escribir
el acorde al principio de cada parte. Un acorde se escribe en la
tonalidad de la canción en el lugar donde suena, con la nota más grave
de la banda después de la barra: acordes de piano con Do, Mi, La sobre
un bajo que toca La son Am, no Am/C. Un poliacorde se escribe
`superior|inferior`, `Eb7|Fb`, y un conjunto por su nombre, `4-23 T2`
(ver [La hoja de acordes: modelo, distribución y
texto](../band/chord-sheet.md)). En una tonalidad que es un conjunto o
una serie dodecafónica, donde los acordes no tienen función, cada pulso
se escribe en cambio como el conjunto que forman sus notas, o como un
micromodo (ver [Micromodos](micromodes.md)).

## Un segundo oído, para los patrones de la propia banda

La biblioteca también escucha acordes, en una situación más sencilla.
Para tocar una canción con un ritmo nuevo, o para que la banda siga los
acordes de un patrón, necesita los acordes de un **canal de acordes**
sobre una **línea de bajo**, en pasos enteros: los patrones que escribe
un estilo. `library/listen.rs` lo hace con las mismas dos ideas, una
puntuación para cada acorde y programación dinámica para los cortes,
ajustadas para que todos los estilos de la biblioteca, toquen como
toquen sus acordes, se escuchen como su cifrado.

Sus puntuaciones necesitan saber «cuántos pasos suena cada clase de
altura entre el paso `a` y el paso `b`», para miles de tramos. Así que
suma una vez, desde el principio:

```rust
struct Heard {
    sums: Vec<[f32; 12]>,
    busy: Vec<f32>,
    quiet: Vec<f32>,
    /// The pitch class of the lowest bass note that starts at each step,
    /// if any.
    bass: Vec<Option<PitchClass>>,
}
```

`sums[s][pc]` es en cuántos de los pasos *anteriores* a `s` suena la
clase de altura `pc`. El tiempo entre `a` y `b` es entonces una sola
resta, sea cual sea la longitud del tramo:

```rust
fn between(&self, start: usize, end: usize) -> Stretch {
    let mut heard = [0.0; 12];
    for (pc, time) in heard.iter_mut().enumerate() {
        *time = self.sums[end][pc] - self.sums[start][pc];
    }
    // ...
}
```

Esto es una **suma de prefijos** (*prefix sum*), y aparece siempre que
un programa pregunta muchas veces «cuánto hay en este rango». Los arrays
de tamaño fijo son `Copy` cuando lo son sus elementos, así que al
construir las sumas
`let mut next = heard.sums[heard.sums.len() - 1];` copia la última
fila, y sumar a `next` deja intacta la fila del vector.

Después de los cortes se unen los vecinos. Dos tramos con el mismo
acorde se vuelven uno, y lo mismo un C y un C7 un pulso después, porque
un arpegio muestra sus notas una a una:

```rust
match joined.last_mut() {
    Some(last) if last.chord == stretch.chord => last.length += stretch.length,
    // An arpeggio shows its notes one by one: a C that turns into
    // C7 a beat later is one C7 still unfolding. A change on a
    // bar line or half way through the bar is taken as meant.
    Some(last) if !half_bar(stretch.start) && grows(last.chord, stretch.chord) => {
        last.chord = bigger(last.chord, stretch.chord);
        last.length += stretch.length;
    }
    _ => joined.push(stretch),
}
```

`last_mut()` da un `Option<&mut HeardChord>`, una referencia a través de
la cual se cambia el último tramo, así que crece en su sitio. Los brazos
se prueban en orden con sus **guardas** (el `if` después de un patrón);
una guarda falsa pasa al brazo siguiente, y `_` recoge el resto. Qué hace
la banda con los acordes que oye está en [La biblioteca: estilos y
ritmos como datos](../band/library.md) y en [Cómo toca la banda una
hoja de acordes](../band/band.md).

## Encontrar la tonalidad

La segunda mitad del capítulo es la tonalidad. **🔍 Buscar la
tonalidad**, debajo de las teclas de armonía, muestra la tonalidad en la
que parece estar la música, con sus rivales más cercanas al lado, más
tenues, y qué hace el acorde que suena en cada una. Es un experimento,
así que está desactivado hasta que lo actives. El trabajo está en
`harmony/key_finding.rs`.

La música hecha con un micromodo u otro conjunto no tiene tonalidad
mayor ni menor. Para ella, una segunda línea nombra el **color** del
pasaje, el micromodo o la colección de notas de la que está hecho: ver
[Encontrar el color](micromodes.md#encontrar-el-color).

### Cómo suena una tonalidad: los perfiles

Carol Krumhansl y Edward Kessler (1982) tocaban a los oyentes una escala
y después una sola nota, y les preguntaban cuánto encajaba la nota. Las
respuestas forman un **perfil de tonalidad**: doce números, cuánto
pertenece a la tonalidad cada nota por encima de la tónica. Estudios
posteriores contaron notas en partituras reales. Ferrous Lineup usa los
perfiles de Albrecht y Shanahan (2013), contados sobre 490 obras
clásicas, que de los probados fueron los que mejor nombraron las
tonalidades de las demostraciones:

| nota sobre la tónica | 1 | ♭2 | 2 | ♭3 | 3 | 4 | ♯4 | 5 | ♭6 | 6 | ♭7 | 7 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| mayor | .24 | .01 | .11 | .01 | .14 | .09 | .02 | .21 | .01 | .08 | .01 | .08 |
| menor | .22 | .01 | .10 | .12 | .02 | .10 | .01 | .21 | .06 | .02 | .06 | .05 |

La tónica y la quinta destacan en los dos; la tercera distingue el
mayor (3) del menor (♭3). En menor, ♭7 y 7 están cerca: aparecen tanto
la séptima natural de la escala como la elevada de la menor armónica.

### Comparar: el método de Krumhansl-Schmuckler

Para encontrar la tonalidad de una música:

1. Se suma cuánto suena cada una de las doce clases de altura, en
   cualquier octava: el **croma** de la música, empezando por Do. (Son
   los mismos doce números que el perfil de la escucha de acordes.)
2. Se gira cada perfil para que empiece en cada una de las doce
   tónicas: 24 tonalidades.
3. Se compara el croma con cada una de las 24 mediante su
   **correlación**: 1 cuando suben y bajan juntos, 0 cuando no tienen
   relación, −1 cuando son opuestos.
4. Gana la tonalidad con la correlación más alta.

La correlación son dos pasos: llevar las dos listas a una media de 0 y
una longitud de 1, y después hacer su producto escalar.

```rust
fn normalized(values: &Chroma) -> Option<Chroma> {
    let mean = values.iter().sum::<f32>() / 12.0;
    let centered = values.map(|v| v - mean);
    let length = centered.iter().map(|v| v * v).sum::<f32>().sqrt();
    (length > 1e-9).then(|| centered.map(|v| v / length))
}
```

`Chroma` es `[f32; 12]`, y los arrays tienen un `map` propio que
devuelve un array del mismo tamaño, así que no se crea ningún `Vec`.
`Option` se ocupa del silencio: cuando no suena nada, los doce números
son iguales, la longitud es 0 y no hay tonalidad que encontrar, así que
la función devuelve `None` en lugar de dividir por cero. `then` es el
primo perezoso de `then_some`: ejecuta el closure solo cuando se cumple
la condición, así que con silencio la división nunca ocurre.

Girar un perfil hacia otra tónica usa `std::array::from_fn`, que
construye un array llamando a un closure para cada índice:

```rust
// The profile counts from the tonic; turn it to count from C.
let turned: Chroma = std::array::from_fn(|c| profile[(c + 12 - tonic) % 12]);
```

`key_fits` devuelve las 24 tonalidades con su correlación, la mejor
primero. La importación MIDI la usa para comprobar la armadura de un
archivo contra sus notas (ver [MIDI: mensajes, dispositivos, grabación
y archivos](../sound/midi.md#tempo-compás-y-tonalidad)).

### Seguro o no seguro

Las correlaciones solas no dicen lo segura que es la respuesta. Do
mayor y La menor tienen las mismas notas, así que una melodía en las
teclas blancas puede encajar en las dos casi igual de bien. `ranked`
convierte las 24 correlaciones en **partes** que suman 1, un *softmax*:

```rust
let best = fits.iter().copied().fold(f32::MIN, f32::max);
let weights: Vec<f32> = fits
    .iter()
    .map(|&fit| ((fit - best) * SHARE_SHARPNESS).exp())
    .collect();
let total: f32 = weights.iter().sum();
```

Con `SHARE_SHARPNESS` en 12, un encaje mejor por 0,05 recibe una parte
e^(12 × 0,05) ≈ 1,8 veces mayor. Restar antes `best` deja todos los
exponentes en 0 o por debajo, así que `exp` nunca puede desbordarse. Una
tonalidad es **clara** cuando su parte es al menos el doble de la
segunda:

```rust
fn clear(guesses: &[KeyGuess]) -> bool {
    match guesses {
        [] => false,
        [_] => true,
        [best, second, ..] => best.share >= CLEAR_RATIO * second.share,
    }
}
```

Estos son **patrones de slice**: `[]` encaja con un slice vacío, `[_]`
con uno de exactamente un elemento, y `[best, second, ..]` nombra los
dos primeros de uno más largo, con `..` para el resto. Cuando la
tonalidad no es clara, las siguientes cuya parte es al menos el 40% de
la mejor son sus **alternativas**, dos como máximo, y la aplicación las
muestra más tenues.

Mi dórico tiene las mismas notas que La menor y Sol mayor, así que los
perfiles, que solo conocen el mayor y el menor, no pueden elegir su
nota de reposo con confianza, y la aplicación lo dice: La menor, o Mi
menor, o La mayor. La tónica real está entre las alternativas, que es la
respuesta honesta.

### Dónde cambia la tonalidad

Una canción que modula no tiene un solo croma. Así que la música se
corta en compases, y cada compás recibe una puntuación para cada
tonalidad. Un solo compás es poco, sin embargo: un compás que solo
tiene D7 encaja mejor en Sol mayor que en Do mayor, incluso en mitad de
una canción en Do mayor. Así que cada compás se puntúa junto con los de
alrededor: él mismo cuenta tres veces, sus vecinos dos, los compases a
dos de distancia una.

Después, un cambio de tonalidad **cuesta** una penalización, 1,5 en
unidades de correlación, y el **algoritmo de Viterbi** encuentra la
secuencia de tonalidades con la mejor puntuación total. Para cada compás
y cada tonalidad guarda solo la mejor manera de llegar allí: quedarse en
esa tonalidad, o cambiar desde la mejor tonalidad hasta el momento
pagando la penalización.

```rust
for (i, here) in windowed.iter().enumerate().skip(1) {
    let (leader, leader_score) = best_of(&best);
    let mut next = vec![0.0; count];
    for k in 0..count {
        let (previous, total) = if best[k] >= leader_score - CHANGE_COST {
            (k, best[k])
        } else {
            (leader, leader_score - CHANGE_COST)
        };
        from[i][k] = previous;
        next[k] = total + here[k];
    }
    best = next;
}
```

`from[i][k]` recuerda de dónde venía el mejor camino hacia la tonalidad
`k` en el compás `i`, y al final el código lo recorre hacia atrás para
leer el camino. Es otra vez programación dinámica, sobre tonalidades en
lugar de cortes. Una tonalidad nueva gana solo cuando encaja mejor
durante el tiempo suficiente para pagar la penalización, que es
exactamente «un cambio solo cuando la tonalidad nueva se mantiene»: un
acorde prestado no es una modulación.

Puntuar junto con los compases vecinos difumina *dónde* ocurre un cambio
en uno o dos compases. Así que cada cambio se mueve después, dentro de
tres compases, al compás donde las notas propias de los compases pasan
de encajar en la tonalidad vieja a encajar en la nueva. En la Habanera
de *Carmen* cae en el compás 10, donde el Re menor de Bizet pasa a Re
mayor.

`Project::heard_keys` hace esto para la canción entera: cuentan los
sintetizadores y los canales MIDI Out que no están silenciados, la
batería y los samplers (como la grabación de una toma) no, y cada nota
cuenta tanto como suena en cada compás. Devuelve `KeySpan`s, cada uno
con un inicio, un final y todas las tonalidades ordenadas. **Marcar en
la lista de reproducción**, junto a la tonalidad escuchada, los
convierte en marcas de tonalidad (ver [Escalas, modos y
tonalidades](scales.md#cambios-de-tonalidad-a-mitad-de-camino-marcas-de-tonalidad)):
ninguna nota se mueve, y Deshacer las quita. Es la manera de darle a un
archivo MIDI importado sin armaduras los cambios de tonalidad que tiene
de verdad.

### Mientras tocas

Para los acordes tocados en un teclado MIDI o en el teclado de escribir
no hay partitura por delante en la que mirar, solo los compases tocados
hasta ahora. `KeyListener` guarda un croma del pasado que se va
apagando:

```rust
/// A bar ended: it joins the past, and the key may change.
pub fn end_bar(&mut self) {
    if normalized(&self.bar).is_none() {
        return;
    }
    let heard = self.heard();
    self.past = heard.map(|v| v * LISTENER_MEMORY);
    self.bar = [0.0; 12];
    // ...
}
```

En cada compás, todo lo oído antes cuenta 0,84 veces, así que un compás
de hace cuatro compases cuenta más o menos la mitad (0,84⁴ ≈ 0,5). Una
tonalidad nueva toma el mando solo después de ir delante dos compases
seguidos. El oyente tampoco empieza de cero: `KeyListener::starting_in`
llena el pasado con música con la forma de la tonalidad de alrededor, la
que se escucha en las notas de la canción en ese punto o la del
proyecto, así que los primeros acordes que tocas se oyen a su luz. Toca
C, F y G7 sobre una canción en Mi dórico y primero se oyen contra La
menor; unos compases después el oyente se decide por Do mayor, clara.

La aplicación muestra la tonalidad de la fuente más inmediata que tiene
(ver `key_finder.rs`): lo que tocas, si tocaste en los últimos cuatro
compases; si no, las tonalidades oídas en las notas de la canción en el
cabezal; en el modo de patrón, las notas del patrón seleccionado. La
línea dice cuál: «Suena en lo que tocas:», «en las notas de aquí», «en
este patrón».

### Qué es un acorde en cada tonalidad

El mismo acorde hace un trabajo distinto en cada tonalidad a la que
podría pertenecer, y por eso importan las alternativas. `read_chord` lo
calcula con lo que la aplicación ya sabe de las tonalidades: el número
romano, la función, un acorde de séptima de dominante oído como la
dominante del acorde hacia el que tira, y los acordes prestados de la
tonalidad homónima (ver [Números romanos, las teclas de armonía y las
funciones de los
acordes](numerals.md#qué-hace-un-acorde-las-funciones-de-la-hoja-de-acordes)).
G7 se lee:

| tonalidad | lectura | qué hace |
|---|---|---|
| Do mayor | V7, dominante | tira hacia Do, la tónica |
| La menor | ♭VII7 = V7/♭III | la dominante de Do, la relativa mayor |
| Sol mayor | I7 = V7/IV | el acorde de tónica con séptima menor, que tira hacia Do, el IV |

Los números romanos se cuentan sobre la escala mayor de la tónica, como
en todo Ferrous Lineup, así que Sol en La menor es ♭VII.

## Qué tan bien escucha

Las pruebas de `tests/chord_reading.rs` comprueban compases cuya
armonía se conoce por las partituras: el acorde de los Augurios, el
acorde de Petrushka en la habitación de Petrushka, y las cuartas del
principio de la feria. Un informe, que es una prueba ignorada, compara
pulso a pulso los acordes oídos en las notas con el cifrado de cada
demostración que tiene uno:

```sh
cargo test --release -p lineup-core --test chord_reading -- --ignored --nocapture
```

`tests/key_finding.rs` tiene un informe del mismo tipo para las
tonalidades, contra las tonalidades y las marcas de tonalidad que las
demostraciones tienen de sus partituras:

```sh
cargo test --release -p lineup-core --test key_finding report -- --ignored --nocapture
```

Medido el 30 de octubre de 2026:

| Acordes, 330 demostraciones con cifrado, 60.322 pulsos | |
|---|---|
| Exactamente el acorde del cifrado | 84,0% |
| Un acorde compatible (las mismas notas, o la misma fundamental con notas añadidas u omitidas) | 91,1% |
| La fundamental correcta | 92,1% |

| Tonalidades, a partir de las notas | |
|---|---|
| Compases en la tonalidad correcta | 87,0% de 18.828 |
| Cambios de tonalidad de las partituras encontrados a un compás o menos | 132 de 315 |
| Compases mostrados como claros que son correctos | 13.900 de 16.153 (86%) |

Cada informe también lista las demostraciones en las que más se
equivoca: el sitio donde mirar después. En las tonalidades, la mayoría
de los fallos son de tres tipos:

- **Visitas cortas.** Un cambio que dura de cuatro a ocho compases, a
  menudo hacia la dominante, es difícil de distinguir de un pasaje que
  solo la toniciza, y muchos músicos leerían esos compases también como
  tonicizaciones.
- **Tonalidades relativas.** Las mismas notas, otro reposo: una canción
  en Si♭ mayor puede salir como Sol menor.
- **Modos**, como el Mi dórico de antes.

Algunos «cambios que no están» son reales, también: las partituras de
las que salen las demostraciones marcan solo los cambios de tonalidad
más grandes.

## Cómo se prueba

Las pruebas unitarias de `hearing.rs` construyen unas pocas notas a mano
y comprueban qué se escucha. Un arpegio, una nota por paso, es su
acorde:

```rust
#[test]
fn an_arpeggio_is_its_chord() {
    // A minor broken up, one note per step: no two notes ever sound
    // together.
    let notes = [
        held(45, 0.0, 1.0),
        held(52, 1.0, 2.0),
        held(57, 2.0, 3.0),
        held(60, 3.0, 4.0),
    ];
    assert_eq!(chord_of(&hear(&notes, 4.0)).unwrap(), "Am");
}
```

`held` crea una nota de la parte 0, y `chord_of` nombra la mejor
lectura. `unwrap` está bien en una prueba: si no hay nombre, la prueba
debe fallar ahí mismo. Las demás comprueban que un acorde sostenido es
él mismo, que las notas de paso de una melodía dejan el acorde como
está, que el acorde de los Augurios es E♭7 sobre F♭ y el de Petrushka
dos arpegios a un tritono, que una superposición de cuartas es un
conjunto, y que nada oído es silencio. `key_finding.rs` comprueba que
una cadencia nombra su tonalidad, que las partes suman uno, que la
escala pentatónica sola no es clara, que un compás prestado no es una
modulación y que el oyente cambia de tonalidad cuando la nueva se
mantiene; `tests/key_finding.rs` comprueba que la mayoría de los
compases de las demostraciones se oyen en su tonalidad y que las
modulaciones claras, como la de la Habanera, se encuentran en su
compás.

Los ejercicios sobre la escucha están en [Ejercicios: la teoría musical
en código](exercises.md).
