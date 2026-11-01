# Números romanos, las teclas de armonía y las funciones de los acordes

Archivos: [`crates/lineup-core/src/harmony/scale.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/scale.rs),
[`crates/lineup-core/src/harmony/keys.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/keys.rs),
[`crates/lineup-core/src/harmony/progression.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/progression.rs),
[`crates/lineup-core/src/library/numeral.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/numeral.rs),
[`crates/lineup-core/src/chord_sheet/function.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/chord_sheet/function.rs)
y [`crates/lineup-app/src/harmony_keys.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/harmony_keys.rs)

El nombre de un acorde dice qué es; su **número romano** dice qué es *en
la tonalidad*. Los números son la manera en que Ferrous Lineup enseña
armonía: rotulan las teclas de armonía, van debajo de cada acorde de la
hoja de acordes, explican las sugerencias sobre lo que podría venir
después, y la biblioteca escribe sus progresiones con ellos. Este
capítulo trata de escribir y leer números romanos, de las teclas de
armonía, de las tablas de movimientos frecuentes, y de cómo averigua la
hoja de acordes qué está haciendo ahí un acorde de fuera de la
tonalidad.

## La música, en pocas palabras

Una tonalidad tiene siete acordes, uno sobre cada nota de su escala, y
los músicos los numeran con números romanos: en mayúscula para un
acorde mayor, en minúscula para uno menor, ° para disminuido, + para
aumentado, ø para semidisminuido.

```text
  I    ii    iii   IV    V     vi    vii°
  C    Dm    Em    F     G     Am    Bdim
```

Los números permiten hablar de una progresión en todas las tonalidades
a la vez: I vi IV V es C Am F G en Do mayor y G Em C D en Sol mayor.

Cada acorde hace uno de tres trabajos en su tonalidad, su **función**:

| Función | Acordes | Se siente como |
|---------|---------|----------------|
| **Tónica** | I, iii, vi | casa: la música puede descansar aquí |
| **Subdominante** | ii, IV | salir de casa, camino de la dominante |
| **Dominante** | V, vii° | tensión que tira de vuelta a casa, al I |

El tirón de la dominante viene de su tercera, Si en Sol mayor, un
semitono por debajo de Do: la **sensible**, que quiere subir a la
tónica. En una tonalidad menor la escala no tiene sensible (en La menor,
Sol está un tono entero por debajo de La), así que la música en menor
la sube a Sol♯, que es lo que es la escala menor armónica, y su V es
mayor: E, no Em.

Unos pocos movimientos entre los acordes son la columna vertebral de
casi toda la música:

| Movimiento | En Do | Por qué funciona |
|------------|-------|------------------|
| V a I | G a C | el final más fuerte: la sensible sube a la tónica |
| IV a V | F a G | un paso hacia arriba, de alejarse a querer volver a casa |
| ii a V | Dm a G | la fundamental baja una quinta; ii V I está en todas partes, del jazz al pop |
| IV a I | F a C | la cadencia plagal, el suave «amén» |
| V a vi | G a Am | la cadencia rota: esperas C y llega su relativa menor |

Una fundamental que **baja una quinta** (Sol a Do, Re a Sol, La a Re) es
el movimiento más fuerte que hay, y muchas de las sugerencias son de
ese tipo.

Las canciones usan también acordes de fuera de la tonalidad, y la
armonía nombra lo que hacen. Un acorde **prestado** viene de la
tonalidad menor sobre la misma tónica, como Fm en Do mayor. Una
**dominante secundaria** es la dominante de otro acorde de la tonalidad:
D7 es el V de Sol, escrito V7/V, y por un momento hace que Sol suene a
casa. La **napolitana** es un acorde mayor sobre el segundo grado
rebajado, Re♭ en Do. Un **acorde pivote** pertenece a dos tonalidades, y
la música cambia de tonalidad a través de él. Los capítulos [Qué hacen
los acordes: números y
funciones](https://naelolaiz.github.io/ferrous-lineup/es/manual/chord-functions.html)
y [Acordes de fuera de la
tonalidad](https://naelolaiz.github.io/ferrous-lineup/es/manual/outside-the-key.html)
del manual enseñan todo esto con ejemplos para tocar.

## Escribir un número romano

Un número romano tiene tres partes: un sostenido o un bemol delante, el
número, y lo que lo sigue, el «7» de V7 o el «°» de vii°. Las tres salen
de una función:

```rust
fn numeral_on(self, letters: u8, chord: Chord) -> Option<String> {
    const NUMERALS: [&str; 7] = ["I", "II", "III", "IV", "V", "VI", "VII"];
    if !self.is_heptatonic() {
        return None;
    }
    let letters = usize::from(letters);
    let unaltered =
        self.unaltered_degree(letters, chord.quality, MinorNumerals::current()) as i8;
    let semitones = self.root.semitones_up_to(chord.root) as i8;
    // How far the root is from the note the numeral names without an
    // accidental, from -6 to 5.
    let shift = (semitones - unaltered + 18).rem_euclid(12) - 6;
    let accidentals = if shift < 0 { "b" } else { "#" }.repeat(shift.unsigned_abs() as usize);
    let numeral = if chord.quality.is_minor() {
        NUMERALS[letters].to_lowercase()
    } else {
        NUMERALS[letters].to_string()
    };
    Some(format!(
        "{accidentals}{numeral}{}",
        chord.quality.numeral_suffix()
    ))
}
```

- `letters` es el grado, cuántas letras por encima de la tónica está la
  fundamental. Elige el número: cuatro letras arriba es V.
- `unaltered` es la nota que nombra el número sin alteración, en
  semitonos por encima de la tónica. En una tonalidad mayor, la nota de
  la escala mayor: V nombra la nota siete semitonos arriba. (En una
  tonalidad menor puede ser otra, como muestra la sección siguiente.)
- `shift` es cuánto se aparta de esa nota la fundamental del acorde. La
  diferencia entre dos clases de altura solo se conoce salvo octavas,
  así que sumar 18, tomar el resto entre 12 y restar 6 da el camino más
  corto, de -6 a 5: una fundamental un semitono por debajo da -1, un
  bemol, no +11.
- La mayúscula o minúscula sale de la clase de acorde, y el final de
  `numeral_suffix` ([Acordes: construirlos, nombrarlos e
  invertirlos](chords.md)).

Así, en Do mayor, Si♭ mayor es ♭VII: el VII sin alteración nombra Si,
once semitonos por encima de Do; Si♭ está a diez, uno por debajo, así
que lleva un bemol.

Sobre qué grado está un acorde se puede contar de dos maneras, y se usan
las dos. `roman_numeral(chord)` cuenta desde el sonido: la letra de la
fundamental en la escritura propia de la tonalidad ([Escalas, modos y
tonalidades](scales.md#notas-de-fuera-de-la-tonalidad)).
`spelled_numeral(root, chord)` cuenta desde la fundamental como se
**escribió**, así que un acorde escrito en la hoja de acordes conserva
su letra. En Do mayor, C♯°7, una tónica subida que lleva a Dm, es ♯i°7,
y D♭7 es ♭II7. Por el sonido solo, los dos estarían sobre el segundo
grado.

```rust
pub fn spelled_numeral(self, root: NoteName, chord: Chord) -> Option<String> {
    self.numeral_on(self.tonic_name().letter.steps_up_to(root.letter), chord)
}
```

### Cifras

Las cifras de una inversión ([Acordes: construirlos, nombrarlos e
invertirlos](chords.md#inversiones)) van después del
número, en el lugar del 7 de un acorde de séptima: V7 con su tercera en
el bajo es V⁶₅. Una dominante secundaria conserva su destino después de
las cifras: V⁶₅/vi.

```rust
pub fn with_figures(numeral: &str, figures: &str) -> String {
    if figures.is_empty() {
        return numeral.to_string();
    }
    // A secondary dominant's figures go on its own numeral, before the
    // slash: V⁶₅/vi.
    let (own, target) = match numeral.split_once('/') {
        Some((own, target)) => (own, format!("/{target}")),
        None => (numeral, String::new()),
    };
    let own = own.strip_suffix('7').unwrap_or(own);
    format!("{own}{figures}{target}")
}
```

`split_once('/')` parte en la primera barra; `strip_suffix('7')` da el
texto sin un 7 final, o `None` si no lo tiene.

### Leer un número romano

La biblioteca guarda sus progresiones como números romanos, y la hoja
de acordes los acepta escritos. `numeral::read` desarma un número:

```rust
let mut shift: i32 = 0;
let mut rest = text;
loop {
    if let Some(after) = rest.strip_prefix('b') {
        shift -= 1;
        rest = after;
    } else if let Some(after) = rest.strip_prefix('#') {
        shift += 1;
        rest = after;
    } else {
        break;
    }
}
// The numeral, in either case. `get(..len)` gives `None` instead of
// panicking when `rest` is shorter than the numeral.
let (numeral, degree) = NUMERALS.iter().find(|(numeral, _)| {
    rest.get(..numeral.len())
        .is_some_and(|start| start.eq_ignore_ascii_case(numeral))
})?;
```

`strip_prefix` da el resto del texto cuando empieza con el prefijo.
Cortar una cadena con `&rest[..n]` provoca un panic cuando `n` pasa del
final; `get(..n)` da `None` en su lugar, así que un texto corto
simplemente no es ese número. La tabla pone III antes que II e I, y VII
antes que VI y V, así que un número más largo se prueba antes que uno
más corto con el que empieza. Un número con mayúsculas y minúsculas
mezcladas, como «Iv», se rechaza, porque es su mayúscula o minúscula lo
que dice mayor o menor.

`parse_in(text, key, numerals)` convierte luego el grado y la alteración
en semitonos por encima de la tónica, con el mismo `unaltered_degree`
que usa la escritura, así que escribir un número y leerlo de vuelta
nunca pueden discrepar. Un test lo comprueba con 10.368 números: las
dos maneras de contar, cuatro clases de tonalidad, doce tónicas, un
acorde sobre cada una de las doce notas y nueve clases de acorde.

## Tonalidades menores: bIII o III

En La menor, el acorde de Do mayor tiene dos nombres. Los libros de pop
y de jazz lo llaman **♭III**; los libros de armonía clásica lo llaman
**III**. Los dos tienen razón, y **Edición > Ajustes… > Armonía >
Números romanos en tonalidades menores** deja elegir.

**Contados desde la escala mayor**, los números se miden contra la
escala mayor sobre la misma tónica, La mayor, cuya tercera es Do♯. Do
está un semitono por debajo, así que su acorde es ♭III; Fa es ♭VI y Sol
♭VII. Una sola tabla de progresiones funciona entonces en todas las
tonalidades y modos: ♭VI ♭VII i es la misma progresión en La menor y en
La dórico, y una canción en Do mayor que toma prestados La♭ y Si♭ de Do
menor los escribe ♭VI y ♭VII también. La biblioteca escribe siempre sus
progresiones así.

**Contados desde la escala menor**, los números se miden contra la
escala propia de la tonalidad. Do es simplemente su tercera nota, así
que su acorde es III.

Solo difieren el tercer, el sexto y el séptimo grado, las notas que
cambian entre La mayor y La menor. Pero la sexta y la séptima nota de
una tonalidad menor además se mueven: la menor armónica sube Sol a Sol♯,
la melódica sube también Fa a Fa♯. Así que un número sin alteración en
esos grados tiene que nombrar una de dos notas, y la regla que usan los
libros clásicos es: **un número sin alteración nombra la forma cuyo
acorde pertenece a la tonalidad**. VII es Sol mayor, de la menor
natural; vii° es G♯°, el acorde de sensible de la menor armónica, ya que
G° (Sol Si♭ Re♭) no es ningún acorde de La menor.

```text
                     desde la escala mayor          desde la escala menor
menor natural   i ii° bIII  iv v  bVI bVII     i ii° III  iv v  VI  VII
menor armónica  i ii° bIII+ iv V  bVI vii°     i ii° III+ iv V  VI  vii°
menor melódica  i ii  bIII+ IV V  vi° vii°     i ii  III+ IV V  vi° vii°
```

Esa regla es `unaltered_degree`:

```rust
pub(crate) fn unaltered_degree(
    self,
    letters: usize,
    quality: ChordQuality,
    numerals: MinorNumerals,
) -> u8 {
    use ScaleKind::{HarmonicMinor, MelodicMinor, NaturalMinor};
    if numerals == MinorNumerals::FromMajor || !self.is_minor_key() {
        return ScaleKind::Major.tonal_intervals()[letters];
    }
    let natural = NaturalMinor.tonal_intervals()[letters];
    if letters < 5 {
        return natural;
    }
    // Whether the chord on `semitones` is in one of these scales,
    // counted from C, since only the distance from the tonic counts.
    let of_key = |semitones: u8, kinds: &[ScaleKind]| {
        let chord = Chord::new(PitchClass::C.transpose(i32::from(semitones)), quality);
        kinds
            .iter()
            .any(|&kind| Scale::new(PitchClass::C, kind).is_diatonic(chord))
    };
    if !of_key(natural, &[NaturalMinor]) && of_key(natural + 1, &[HarmonicMinor, MelodicMinor])
    {
        natural + 1
    } else {
        natural
    }
}
```

Para los cinco primeros grados la respuesta es la nota de la menor
natural. Para el sexto y el séptimo, la función hace la pregunta de la
regla: ¿el acorde sobre la nota natural es un acorde de la menor
natural? Si no lo es, y el acorde un semitono más arriba está en la
menor armónica o en la melódica, la nota subida es la que se quiere
decir. `of_key` construye el acorde sobre Do, porque solo importa su
distancia a la tónica.

El ajuste cambia cómo se **escriben** y se **leen** los números, nunca
un acorde. Una canción guarda acordes, no números, así que es la misma
elijas lo que elijas. Las tonalidades mayores y los modos se escriben
igual de las dos maneras.

### Un ajuste que toda función puede ver, uno por hilo

Los números se escriben en muchos lugares, las teclas de armonía, la
hoja de acordes, el piano roll, las sugerencias, una exportación de
video, y todos terminan en `numeral_on`. Pasar el ajuste como parámetro
cambiaría la firma de decenas de funciones por el camino, para una
elección que cambia de vez en cuando. Así que el ajuste se guarda donde
`numeral_on` puede leerlo:

```rust
thread_local! {
    /// How the numerals written on this thread count a minor key's
    /// degrees. The window's thread sets it from the settings each frame;
    /// each thread has its own, so tests running side by side never see
    /// each other's.
    static MINOR_NUMERALS: Cell<MinorNumerals> = const { Cell::new(MinorNumerals::FromMajor) };
}

impl MinorNumerals {
    /// How numerals are written on this thread.
    pub fn current() -> MinorNumerals {
        MINOR_NUMERALS.with(Cell::get)
    }

    /// Writes numerals this way on this thread from now on.
    pub fn set(self) {
        MINOR_NUMERALS.with(|numerals| numerals.set(self));
    }
}
```

El hilo de la ventana lo fija antes de dibujar cada frame:

```rust
// Every view writes numerals as the settings say.
settings::minor_numerals(ctx).set();
```

Dos piezas de Rust lo hacen seguro:

- **`thread_local!`** le da a cada hilo su propia copia del valor.
  `cargo test` ejecuta los tests en muchos hilos a la vez, y un test que
  escribe números desde la escala menor no debe cambiar lo que ve un
  test que corre a su lado. Con un `static` normal lo cambiaría, y el
  otro test fallaría de vez en cuando, según los tiempos.
- **`Cell`** permite cambiar un valor a través de una referencia
  compartida. A un valor de hilo solo se llega con `&`, así que un valor
  simple nunca se podría fijar. `Cell` funciona metiendo y sacando
  valores enteros (`get`, `set`), sin entregar nunca una referencia a lo
  de dentro, así que nada se puede leer mientras cambia.

Como el valor pertenece a un hilo, un hilo nuevo empieza con el valor
por defecto. Una exportación de video dibuja sus vistas en un hilo
propio, así que se lleva el ajuste, junto con el idioma y la
nomenclatura de notas ([Exportar: WAV, MIDI y
video](../sound/export.md#ajustes-que-viven-en-un-hilo)):

```rust
let numerals = MinorNumerals::current();
```

y llama a `numerals.set()` lo primero en el hilo nuevo.

#### Devolverlo, incluso después de un panic

Los tests escriben los números de una manera durante un rato, y luego
vuelven:

```rust
pub fn with<T>(self, f: impl FnOnce() -> T) -> T {
    struct Restore(MinorNumerals);
    impl Drop for Restore {
        fn drop(&mut self) {
            self.0.set();
        }
    }
    let _restore = Restore(MinorNumerals::current());
    self.set();
    f()
}
```

`Restore` guarda el valor viejo, y su `Drop` lo devuelve. Rust suelta
una variable local cuando sale de su ámbito, y también mientras un
**panic** se desenrolla a través de la función. Un `assert!` que falla
dentro de `f` es un panic, así que incluso entonces vuelve el ajuste
viejo, y el siguiente test de ese hilo empieza limpio. Limpiar en un
`Drop` así se llama una **guarda**; es como el `MutexGuard` de la
biblioteca estándar libera un mutex.

La variable se llama `_restore`, no `_`. Un `_` solo no liga nada, así
que la guarda se soltaría enseguida, en la misma línea, antes de
`self.set()`, y nada devolvería el valor viejo después de `f`.

Las sugerencias son frases escritas una vez, con los números como los
escribe la biblioteca: «I to IV: moving away from home». Cuando los
números se cuentan desde la escala menor, `numeral::written_in`
reescribe los números dentro de esa frase. Casi siempre no cambia nada,
así que devuelve un `Cow<str>`: el texto original prestado, o un
`String` nuevo solo cuando tuvo que reescribir algo.

## Las teclas de armonía

Las teclas de armonía de la parte de abajo de la ventana son una tecla
grande para cada acorde de la tonalidad, coloreadas por función, que se
tocan manteniéndolas pulsadas. El modelo es teoría pura, así que vive en
`lineup-core`, con tests, y no sabe nada de egui:

```rust
pub fn chord_keys(scale: Scale, size: ChordSize) -> Vec<ChordKey> {
    let scale = chord_scale(scale);
    let leading_tone = Scale::new(scale.root, ScaleKind::HarmonicMinor);
    (0..7)
        .filter_map(|degree| {
            let from = if scale.kind == ScaleKind::NaturalMinor && degree == 4 {
                leading_tone
            } else {
                scale
            };
            let chord = chord_on(from, degree, size)?;
            Some(ChordKey {
                degree,
                chord,
                numeral: scale.roman_numeral(chord)?,
                function: has_functions(scale.kind).then(|| Function::of(degree)),
            })
        })
        .collect()
}
```

- `filter_map` ejecuta el closure con cada grado y se queda con los
  `Some`, y dentro del closure `?` convierte un `None` en «deja este
  fuera».
- En una tonalidad menor natural, el acorde sobre V sale de la menor
  armónica: mayor, con la sensible.
- `chord_scale` le da a una escala pentatónica o de blues, que no tiene
  siete acordes propios, la escala mayor o menor sobre la misma tónica.
- `bool::then` convierte `true` en `Some` del valor del closure y
  `false` en `None`. Los modos no tienen función: no tienen una
  dominante que tire hacia casa como lo hace V. En Do locrio el acorde
  sobre la quinta nota es Sol♭ mayor, que no tira hacia Do en absoluto.

`Function::of` es la tabla de trabajos por grado:

```rust
pub fn of(degree: usize) -> Function {
    match degree % 7 {
        1 | 3 => Function::Subdominant,
        4 | 6 => Function::Dominant,
        _ => Function::Tonic,
    }
}
```

`ChordSize` elige tríadas, séptimas o novenas. En dos grados de una
tonalidad mayor, iii y vii, la novena cae un semitono por encima de la
fundamental, una octava más arriba; eso choca, y los músicos la dejan
fuera, así que `Scale::ninth` da `None` ahí y la tecla vuelve al acorde
de séptima:

```rust
ChordSize::Ninths => scale.ninth(degree).or_else(|| scale.seventh(degree)),
```

`Option::or_else` prueba el closure solo cuando el primer valor es
`None`: «si eso no funcionó, prueba esto».

### Tocar una tecla

Pulsar C y luego F podría tocar Do Mi Sol y luego Fa La Do, saltando
todas las notas hacia arriba. Un pianista mantendría el Do y movería los
otros dos dedos un paso cada uno, a Do Fa La. `keys::play` dispone cada
acorde cerca del anterior (la disposición en sí es la de la banda, en
[Cómo toca la banda una hoja de acordes](../band/band.md)) y añade la
fundamental debajo, como lo haría una mano izquierda:

```rust
pub fn play(chord: Chord, previous: Option<&[Note]>) -> (Vec<Note>, Vec<Note>) {
    let voicing = voice(chord, previous);
    let mut notes = voicing.clone();
    if let Some(&lowest) = voicing.first() {
        // The root below the lowest note: an octave below if the lowest
        // note is the root itself.
        let down = match chord.root.semitones_up_to(lowest.pitch_class()) {
            0 => 12,
            gap => gap,
        };
        if let Some(root) = lowest.transpose(-i32::from(down)) {
            notes.insert(0, root);
        }
    }
    (voicing, notes)
}
```

Devuelve una tupla de dos vectores: la disposición, para recordarla la
próxima vez, y las notas que hay que tocar. Quien llama la desarma con
`let (voicing, notes) = ...`.

Una tecla de armonía es una tecla, no un botón: su acorde empieza cuando
baja el botón del ratón y se detiene cuando sube, y las teclas de número
1 a 7 también las tocan. Quién mantiene pulsada una tecla es un enum
cuya variante lleva la tecla de número:

```rust
enum Holder {
    Mouse,
    /// A number key on the computer keyboard, 1 to 7.
    Digit(Key),
}

/// The harmony key that is down, and the notes it started.
#[derive(Debug, Clone, PartialEq)]
struct Down {
    key: usize,
    holder: Holder,
    notes: Vec<Note>,
}
```

Solo quien empezó un acorde puede detenerlo, y `Down` guarda las notas
que empezó, así que se detienen las notas correctas aunque la tonalidad
o el tamaño de acorde hayan cambiado mientras se mantenía. Las notas van
por el mismo camino que las del teclado del equipo ([MIDI: mensajes,
dispositivos, grabación y archivos](../sound/midi.md)), así que suenan
en el canal seleccionado, se nombran en la barra de estado y se graban,
sin que ese código sepa que existen las teclas de armonía.

### Qué tecla se ilumina

Mientras suena la música, la tecla del acorde que oyes se ilumina y se
enmarca en blanco, la marca más visible de la barra. Ganan las teclas
pulsadas; si no, el acorde es el que se oye durante el pulso en el
cursor de reproducción, según la escucha de acordes ([Oír la armonía:
acordes y tonalidades a partir de las
notas](hearing.md)), así que un arpegio ilumina su acorde y
dos acordes a la vez iluminan dos teclas. Un acorde ilumina una tecla
cuando tiene la misma fundamental y el mismo tipo de tríada que el
acorde de la tonalidad ahí:

```rust
pub fn key_for(keys: &[ChordKey], chord: Chord) -> Option<usize> {
    keys.iter().position(|key| {
        key.chord.root == chord.root && key.chord.quality.family() == chord.quality.family()
    })
}
```

G7 ilumina el V en Do mayor. D7 no ilumina nada: el acorde de la
tonalidad sobre Re es menor, y D7 viene de fuera de la tonalidad.

## Qué viene después: el conocimiento como datos

Mientras suena un acorde, mantenido o sonando en la canción, las teclas
que suelen venir a continuación reciben un contorno amarillo fino, la
más habitual un poco más brillante, y una línea de sugerencia dice por
qué. Suelta la tecla, o detén la canción, y el contorno se va: las
teclas muestran solo lo que está sonando, así que el acorde que sigue
el contorno es el que encuentra `sounding` en el código de las teclas,
primero una tecla mantenida, después un acorde mantenido en un teclado,
después el de la música. `next_chords(scale, current)` da esos
acordes, el más habitual primero. El conocimiento no está en el código
sino en tablas: cada acorde se encuentra por su fundamental (semitonos
por encima de la tónica) y su tipo de tríada, y lista los movimientos
que suelen seguirlo.

```rust
struct Move {
    to: u8,
    short: &'static str,
    reason: &'static str,
}

const fn go(to: u8, short: &'static str, reason: &'static str) -> Move {
    Move { to, short, reason }
}

struct Slot {
    root: u8,
    family: ChordFamily,
    moves: &'static [Move],
}
```

```rust
const MAJOR: &[Slot] = &[
    slot(
        0,
        Major,
        &[
            go(
                5,
                "away from home",
                "I to IV: moving away from home, toward the dominant.",
            ),
            go(
                7,
                "tension, wants to go home",
                "I to V: tension that wants to come back to I.",
            ),
            // ...
        ],
    ),
    // ...
];
```

Una constante solo se puede construir con código que el compilador pueda
ejecutar mientras compila, y una `const fn` es código así, de modo que
los diminutos `go` y `slot` dejan cada movimiento en una o dos líneas.
`MAJOR` es un slice de entradas, así que tiene tantas como necesite una
tonalidad, incluidos los prestados iv, ♭III, ♭VI y ♭VII. Todo está
dentro del programa. Cambiar lo que sugiere la aplicación es editar una
tabla, y los tests comprueban la tabla: cada grado de tres clases de
tonalidad tiene sugerencias, y ningún acorde se sugiere a sí mismo.

Cada movimiento lleva su razón dos veces, en una frase y en pocas
palabras. El nivel de la línea de sugerencias, **Detallado**, **Breve**
o **Mínimo**, elige una:

```rust
pub fn why(&self, level: HintLevel) -> Option<&str> {
    match level {
        HintLevel::Detailed => Some(&self.reason),
        HintLevel::Short => Some(&self.short),
        HintLevel::Minimal => None,
    }
}
```

Una `Suggestion` guarda sus razones como `Cow<'static, str>`, «clonar al
escribir» (*clone on write*): casi siempre un `&'static str` prestado de
la tabla, sin reservar memoria, y un `String` propio solo cuando
`written_in` tuvo que reescribir sus números.

Las tonalidades menores encuentran sus acordes en otra tabla, la menor
natural con el V mayor y el vii° de sensible de la menor armónica:

```rust
const MINOR_CHORDS: [(u8, ChordQuality); 8] = [
    (0, ChordQuality::Minor),
    (2, ChordQuality::Diminished),
    (3, ChordQuality::Major),
    (5, ChordQuality::Minor),
    (7, ChordQuality::Major),
    (8, ChordQuality::Major),
    (10, ChordQuality::Major),
    (11, ChordQuality::Diminished),
];
```

### Primero, las dominantes secundarias

Un acorde de fuera de la tonalidad que suena a dominante es casi siempre
la dominante del acorde una quinta más abajo, así que `next_chords` lo
comprueba antes de mirar las tablas:

```rust
let dominant_kind = current.quality == ChordQuality::Major || current.quality.is_dominant();
let target = (root + 5) % 12;
if dominant_kind
    && !scale.is_chord_of_key(current)
    && target != 0
    && let Some(suggestion) = suggest(target, SECONDARY_DOMINANT_SHORT, SECONDARY_DOMINANT)
    && suggestion.chord.quality != ChordQuality::Diminished
{
    return vec![suggestion];
}
```

Lee la cadena de `let` como la regla que es: el acorde suena a
dominante, no es uno de los acordes propios de la tonalidad, el acorde
una quinta más abajo no es la tónica (en una tonalidad menor, un acorde
mayor sobre V es la dominante propia de la tonalidad), hay un acorde de
la tonalidad ahí, y no es disminuido, que es demasiado inestable para
sonar a casa ni siquiera un momento. Una quinta hacia abajo y una cuarta
hacia arriba son la misma clase de altura, así que el destino es
`(root + 5) % 12`. En Do mayor, D mayor sugiere G, A7 Dm, E Am, y C7,
cuyo Si♭ hace que ya no sea el acorde de tónica, F.

### El bajo importa: la cuarta y sexta cadencial

C/G, el acorde de tónica con su quinta en el bajo, suena inestable:
sobre Sol, el Do y el Mi quieren bajar un paso a Si y Re, lo que forma
G. Así que en una cadencia C/G sigue hacia V, y se llama **cuarta y
sexta cadencial** (I⁶₄). `next_chords_over` recibe también el bajo, y
trata ese caso antes de pasarle todo lo demás a `next_chords`:

```rust
let six_four = current == Chord::new(scale.root, tonic) && current.tone_of(bass) == Some(5);
if !six_four {
    return next_chords(scale, current);
}
```

## Qué hace un acorde: las funciones de la hoja de acordes

La hoja de acordes escribe un número romano bajo cada acorde, y para un
acorde de fuera de la tonalidad, qué está haciendo ahí. Eso depende de
algo más que el acorde y la tonalidad: un acorde secundario se nombra
por adónde va, y un pivote por los acordes que rodean un cambio de
tonalidad. Así que `Functions::read` lee la hoja entera de una pasada, y
la hoja le pregunta por cada acorde. Lo que hace un acorde es un
`Role`, un enum cuyas variantes llevan lo que necesitan:

```rust
pub enum Role {
    /// One of the key's own chords, or a chord outside it that none of
    /// the others names.
    Plain,
    /// Taken from the parallel key, `from`.
    Borrowed { from: Scale },
    /// The dominant of `of`, one of the key's chords.
    SecondaryDominant { of: Chord },
    /// The chord on the leading tone of `of`, one of the key's chords.
    SecondaryLeadingTone { of: Chord },
    /// A major chord on the lowered second degree.
    Neapolitan,
}
```

Un `match` saca los datos donde se usan, así que `of` solo existe en las
ramas donde significa algo:

```rust
match self.role {
    Role::SecondaryDominant { of } => Some(with_figures(
        &format!(
            "V{}/{}",
            chord.quality.numeral_suffix(),
            self.target_numeral(of)?
        ),
        figures,
    )),
    // ...
    Role::Neapolitan => Some(with_figures("N", figures)),
    Role::Plain | Role::Borrowed { .. } => figured(self.key, self.symbol),
}
```

`Role::Borrowed { .. }` encaja con un acorde prestado sea cual sea la
tonalidad de la que viene.

### Los métodos de la tonalidad

Las preguntas detrás de los papeles son métodos de `Scale`:

- `parallel()`: la tonalidad con la misma tónica y el otro modo, Do
  menor para Do mayor.
- `is_chord_of_key(chord)`: de la escala, o en una tonalidad menor el V
  o el vii° con la séptima subida de la menor armónica. E7 y G♯dim7
  pertenecen a La menor.
- `borrowed_from(chord)`: la tonalidad homónima, si el acorde no es de
  la tonalidad pero sí de esa. Una tonalidad mayor también toma
  prestado de la menor armónica: B♭ y Bdim7 en Do mayor vienen los dos
  de Do menor.
- `secondary_dominant_target(chord)`: para un acorde mayor o de séptima
  de dominante de fuera de la tonalidad, el acorde mayor o menor de la
  tonalidad (no la tónica) una quinta más abajo: E7 en Do mayor es el
  V7 de Am.
- `secondary_leading_tone_target(chord)`: para un acorde disminuido, el
  acorde un semitono más arriba: F♯°7 en Do mayor lleva a G.

### Qué papel va primero

Algunos acordes podrían ser dos cosas. En La menor, La mayor es la
dominante de Dm y también el acorde de tónica mayor prestado de La
mayor; Re mayor es el IV de la menor melódica y la dominante de Sol.
`role` decide por adónde va el acorde:

```rust
let goes_to = |of: Chord| next.is_some_and(|next| arrives(next, of));
let dominant = key.secondary_dominant_target(chord);
let leading = key.secondary_leading_tone_target(chord);
let secondary = match (dominant, leading) {
    (Some(of), _) => Some(Role::SecondaryDominant { of }),
    // A leading-tone chord is named after where it goes: F#ø7 going
    // on to B7 in C major is the ii of E minor, not the vii of G.
    (_, Some(of)) if goes_to(of) => Some(Role::SecondaryLeadingTone { of }),
    _ => None,
};
// A chord that is both, such as A major in A minor (the major tonic,
// or the V of Dm), is a secondary dominant where it goes to the
// chord it leads to, and a borrowed chord anywhere else.
if let Some(of) = dominant.or(leading)
    && goes_to(of)
{
    return secondary.unwrap_or(Role::Plain);
}
// So is a chord of melodic minor, such as D major in A minor: the V
// of G going to G, and the key's own IV anywhere else.
if in_minor(key, chord, &[MelodicMinor]) {
    return Role::Plain;
}
if let Some(from) = borrowed_from(key, symbol) {
    return Role::Borrowed { from };
}
if is_neapolitan(key, symbol) {
    return Role::Neapolitan;
}
secondary.unwrap_or(Role::Plain)
```

Antes de esto, `role` ya ha despachado los acordes que no necesitan más
nombre que su número: todos los acordes de un modo (Mi mayor en Mi
frigio es el acorde de tónica del flamenco, no la dominante de La
menor), todos los acordes de la tonalidad, y en una tonalidad menor
todos los acordes de su menor natural o armónica. Un acorde «va a» su
destino cuando el acorde siguiente está sobre esa fundamental, o es un
acorde de cuarta y sexta sobre ella, que retiene el destino un momento.
`is_some_and` responde `false` para `None`, así que el último acorde,
sin nada después, no va a ninguna parte.

Un acorde prestado tiene que estar **escrito** como lo escribe la otra
tonalidad, no solo sonar como él. Sol aumentado (Sol Si Re♯) se toca en
las mismas teclas que Mi♭ aumentado (Mi♭ Sol Si), un acorde de Do menor
armónica. Pero su Re♯ sube a Mi, donde un Mi♭ bajaría: no es ninguna
nota de Do menor. Así que `borrowed_from` comprueba que cada nota del
acorde está escrita en la letra que le dan las escalas de la tonalidad,
contando hacia arriba desde la tónica. Del mismo modo la napolitana
tiene que estar escrita sobre la segunda letra: Re♭ mayor en Do lo es;
Do♯ mayor, las mismas teclas, es un acorde cromático sobre la tónica.

### Encontrar el pivote

Una **modulación** es un paso a una tonalidad nueva en la que la música
se queda. En Do mayor Am es vi; en Sol mayor es ii. En C, Am, D7, G el
oído oye Am como vi en Do; luego llega D7, que Do mayor no tiene, y el
oído entiende que Am era en realidad el ii de Sol. Am es el **acorde
pivote**, y la hoja de acordes escribe sus dos números, primero el de
la tonalidad vieja: `vi = ii`.

`modulation` mira los acordes alrededor de cada cambio de tonalidad, con
dos closures que dicen qué es un acorde para las dos tonalidades:

```rust
let new_only = |chord: &Function| {
    let chord = chord.symbol.chord();
    belongs(to, chord) && !heard_in(from, chord)
};
let common = |chord: &Function| {
    let chord = chord.symbol.chord();
    belongs(to, chord) && heard_in(from, chord)
};
```

`heard_in` cuenta los acordes prestados de la tonalidad vieja como
propios, por eso Fm, prestado en Do mayor, puede ser el pivote hacia La♭
mayor, donde es vi. La **llegada** es el primer acorde que pertenece
solo a la tonalidad nueva (D7 arriba). Se busca primero justo antes de
la marca de tonalidad, porque los músicos marcan una tonalidad nueva
donde sienten que llegó, lo que puede ser uno o dos compases tarde, y
luego después de ella. El pivote es el acorde anterior a la llegada, si
pertenece a las dos:

```rust
change.pivot = arrival
    .checked_sub(1)
    .map(|before| range[before])
    .filter(|before| common(before))
    .map(|before| (before.start, before.symbol));
```

`checked_sub(1)` da `None` cuando la llegada es el primer acorde, así
que ningún índice baja nunca de cero, y un cambio que empieza en su
llegada no tiene pivote. No todo cambio de tonalidad tiene uno: una
**modulación directa** va derecho a un acorde de la tonalidad nueva,
como hace una canción cuando sube un semitono para el último estribillo,
y un **cambio de modo**, de Do mayor a Do menor, conserva la tónica y no
necesita ningún acorde que lleve hasta ahí.

## Cómo se prueba

Los tests son hojas de acordes pequeñas al estilo de un libro de
armonía, cada una comprobando los números y lo que dicen las
descripciones:

- C Am A♭ Fm G7 C: ♭VI e iv son prestados de Do menor.
- C E7 Am D7/F♯ G7 C A7 Dm: V7/vi, V⁶₅/V y V7/ii, cada uno yendo a su
  acorde. E7 yendo a F sigue siendo V7/vi, y dice que va a otro lado.
- F♯dim7, G♯dim y C♯m7♭5: vii°7/V, vii°/vi y viiø7/ii. En Autumn Leaves
  leído en Do mayor, F♯m7♭5 yendo a B7 conserva ♯ivø7.
- D♭/F en Do menor es N⁶; C♯ en Do mayor no es una napolitana.
- En La menor, B7 es V7/V, o V7/v yendo a Em; A7 yendo a Dm es V7/iv; La
  mayor al final es una tercera de picardía.
- Sol aumentado en Do mayor no es prestado; Bdim7, cuyo La♭ es de Do
  menor, sí.
- C, Am, D7, G hacia Sol mayor, con la marca en el Am, en el D7 o en el
  G: Am es el pivote cada vez.

En `scale.rs` se comprueban los números de las tonalidades menores de
las dos maneras para la menor natural, la armónica y la melódica, y los
10.368 números se escriben y se vuelven a leer. En `keys.rs` y
`progression.rs`, las teclas de armonía de cada clase de tonalidad y las
sugerencias después de cada grado; por ejemplo, en Do mayor:

```rust
// D major is V of G; A7 is V of Dm; E is V of Am; C7 is V of F.
```

Los ejercicios sobre números romanos y funciones están en [Ejercicios:
la teoría musical en código](exercises.md).
