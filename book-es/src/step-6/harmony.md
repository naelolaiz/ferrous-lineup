# Más acordes, inversiones y lo que viene después

Archivos: [`crates/lineup-core/src/harmony/chord.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/chord.rs),
[`crates/lineup-core/src/harmony/progression.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/progression.rs)
y [`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs)

Este capítulo es mitad teoría musical y mitad Rust. La teoría es lo que
ahora enseña el piano roll; el Rust es cómo se guarda ese conocimiento
como datos.

## Siete acordes nuevos

El ejercicio 1 del paso 3 sugería añadir un acorde o dos. El paso 6
añade siete, los acordes extendidos del jazz, el soul y el funk, con lo
que sumaron 28 en total (en pasos posteriores llegaron dos cualidades
más; hoy `ChordQuality::ALL` tiene 30):

| Nombre | Semitonos sobre la fundamental | En Do | Dónde lo oyes |
|--------|-------------------------------|------|---------------|
| 6/9 | 0, 4, 7, 9, 14 | Do Mi Sol La Re | un acorde mayor suave y abierto; finales de jazz y bossa nova |
| 7b9 | 0, 4, 7, 10, 13 | Do Mi Sol Si♭ Re♭ | una dominante tensa, muy común como V en tonalidades menores |
| 7#9 | 0, 4, 7, 10, 15 | Do Mi Sol Si♭ Re♯ | el «acorde de Hendrix» del funk y el rock |
| 11 | 0, 4, 7, 10, 14, 17 | Do Mi Sol Si♭ Re Fa | soul y gospel (a menudo se toca sin su tercera) |
| m11 | 0, 3, 7, 10, 14, 17 | Do Mi♭ Sol Si♭ Re Fa | un acorde menor cálido, presente en todo el neo-soul |
| 13 | 0, 4, 7, 10, 14, 21 | Do Mi Sol Si♭ Re La | la dominante completa del jazz y el funk |
| maj13 | 0, 4, 7, 11, 14, 21 | Do Mi Sol Si Re La | un acorde mayor exuberante |

Los nombres cuentan las notas de la escala desde la fundamental, más
allá de la octava: la novena es la segunda una octava más arriba
(14 = 12 + 2 semitonos), la oncena la cuarta (17 = 12 + 5), la trecena
la sexta (21 = 12 + 9). Escribir 14 en lugar de 2 importa a la hora de
estampar, porque las notas de un acorde estampado se construyen a partir
de estos números: una 9 colocada un tono por encima de la fundamental
sonaría como un clúster, no como un acorde de novena.

Un acorde de trecena «debería» contener también la oncena, siete notas
en total, pero los intérpretes la omiten porque choca con la tercera, un
semitono por debajo de ella. Ferrous Lineup hace lo mismo, y lo dice en
un comentario junto a los números. El *nombrado* de acordes va un paso
más allá, como hace desde el paso 3: si a un acorde de cuatro notas o
más solo le falta la quinta, se le pone nombre igualmente. Así que
Do, Mi, Si♭, Re, La se llama C13.

Añadir un acorde fue tan poca cosa como prometía el paso 3: una variante
en el enum, una línea en `ALL` y una rama en cada `match`. El compilador
señaló cada `match` que tenía que crecer, y la prueba
`no_two_qualities_have_the_same_notes` comprobó que ningún acorde nuevo
tuviera las mismas notas que uno antiguo. Una prueba nueva nombra unas
cuantas disposiciones reales:

```rust
#[test]
fn extended_chords() {
    assert_eq!(label(&[48, 64, 67, 69, 74]).unwrap(), "C6/9");
    assert_eq!(label(&[55, 59, 62, 65, 68]).unwrap(), "G7b9");
    // E, G#, B, D and G: the "Hendrix chord".
    assert_eq!(label(&[52, 56, 59, 62, 67]).unwrap(), "E7#9");
    assert_eq!(label(&[55, 59, 62, 65, 69, 72]).unwrap(), "G11");
    assert_eq!(label(&[45, 48, 52, 55, 59, 62]).unwrap(), "Am11");
    assert_eq!(label(&[48, 64, 67, 71, 74, 81]).unwrap(), "Cmaj13");
    // Thirteenth chords are often played without their fifth too.
    assert_eq!(label(&[48, 64, 70, 74, 81]).unwrap(), "C13");
}
```

Mira con atención Am11 y Cmaj13. Ambas son La, Si, Do, Re, Mi y Sol: las mismas
seis clases de alturas. Los acordes grandes se solapan así
continuamente, y por eso el análisis prueba primero la nota del bajo
como fundamental. Con La en el bajo oyes La menor, con Do en el bajo Do
mayor, y el nombre va detrás.

## Inversiones al estampar

[El capítulo de las herramientas](tools.md#inversiones) mostró
`tools::inversion`. El piano roll también la usa para los acordes
estampados. Cuando el menú Dibujar estampa un acorde, aparece un segundo
menú a su lado:

```rust
// A chord of n notes has n - 1 inversions.
let chord_size = self.stamp.notes_at(Note::C4, scale).len();
if chord_size > 1 {
    self.inversion = self.inversion.min(chord_size - 1);
    egui::ComboBox::from_id_salt("inversion")
        .selected_text(INVERSIONS[self.inversion])
        .show_ui(ui, |ui| {
            for (times, name) in INVERSIONS.iter().enumerate().take(chord_size) {
                ui.selectable_value(&mut self.inversion, times, *name);
            }
        });
}
```

- El menú solo se dibuja cuando el acorde a estampar tiene más de una
  nota. Para notas sueltas no hay nada que invertir.
- `INVERSIONS` enumera seis nombres, de «Posición fundamental» a
  «5.ª inversión», porque los acordes más grandes tienen seis notas.
  `.take(chord_size)` ofrece tantas como tiene el acorde elegido: una
  tríada recibe tres.
- `self.inversion.min(chord_size - 1)` mantiene la elección válida
  cuando pasas de un acorde grande a uno más pequeño: una 4.ª inversión
  se convierte en la última que tiene una tríada.

Invertir mueve la nota más grave hacia arriba, así que invertir el
acorde al que apuntas movería todo el acorde hacia arriba, lejos del
puntero. Estampar Do mayor sobre Do5 en su segunda inversión daría
Sol5, Do6, Mi6, todo por encima de donde hiciste clic. Por eso el piano roll
baja el acorde una octava, y la nota sobre la que hiciste clic sigue
siendo la fundamental:

```rust
fn stamp_notes(&self, pitch: Note, scale: Scale) -> Vec<Note> {
    let notes = tools::inversion(&self.stamp.notes_at(pitch, scale), self.inversion);
    if notes.contains(&pitch) {
        return notes;
    }
    notes
        .iter()
        .map(|note| note.transpose(-12))
        .collect::<Option<Vec<Note>>>()
        .unwrap_or(notes)
}
```

`notes.contains(&pitch)` es verdadero en posición fundamental (nada se
ha movido), y también cuando una inversión no pudo mover la fundamental
(en lo alto del rango MIDI). En caso contrario, todas las notas bajan
una octava. Es el mismo patrón de «recolectar `Option`s» que las
herramientas, escrito esta vez con un **turbofish**
(`collect::<Option<Vec<Note>>>()`). En las herramientas, el tipo de
retorno le decía a `collect` qué construir. Aquí se llama a un método,
`unwrap_or`, directamente sobre el resultado, y Rust debe conocer el
tipo del resultado antes de poder buscar ese método, así que el tipo se
escribe explícitamente. Si alguna nota fuera a caerse por el borde
inferior del rango, `unwrap_or(notes)` deja el acorde como estaba.

El contorno que sigue al puntero se dibuja con el mismo `stamp_notes`,
así que lo que ves es siempre lo que coloca un clic.

## Lo que viene después: armonía funcional

Algunos acordes suenan como si quisieran ir a algún sitio. Toca G7 y
detente: sientes que la canción no ha terminado, y Do mayor es donde
quiere aterrizar. La teoría musical llama a esto **armonía funcional**:
cada acorde de una tonalidad tiene un trabajo.

- La **tónica** (I y, en cierto modo, vi y iii) es el hogar. Una canción
  suele empezar y terminar ahí.
- La **subdominante** (IV, y ii) se aleja del hogar.
- La **dominante** (V, y vii°) es la tensión que quiere volver a casa.
  Su tercera, Si en Sol mayor, está un semitono por debajo de Do: la
  **sensible**, que tira hacia arriba, a la fundamental de la tonalidad.

La mayoría de las progresiones de acordes se componen de unos pocos
movimientos fuertes entre estos trabajos:

| Movimiento | Ejemplo en Do | Por qué funciona |
|------------|--------------|------------------|
| V a I | G a C | el final más fuerte; la sensible sube a la fundamental |
| IV a V | F a G | un paso arriba, de alejarse a querer volver a casa |
| ii a V | Dm a G | la fundamental cae una quinta; ii, V, I está en todas partes, del jazz al pop |
| IV a I | F a C | la cadencia suave de «amén» (plagal) |
| V a vi | G a Am | la cadencia «rota»: esperas C y recibes su relativo menor |
| vi a IV | Am a F | como en I, V, vi, IV, la progresión de incontables canciones |

Una fundamental que **cae una quinta** (Sol a Do, Re a Sol, La a Re...) es el
movimiento más fuerte que existe, y por eso tantas sugerencias son de
ese tipo.

### El conocimiento como datos

`next_chords(scale, current)` da los acordes que suelen seguir a
`current`, primero los más habituales, cada uno con su motivo. El
conocimiento no está en el código; está en una tabla. Cada acorde se
localiza por su fundamental (un número de semitonos sobre la tónica) y
su familia de tríada, y cada entrada enumera los movimientos que suelen
venir después:

```rust
use ChordFamily::{Diminished, Major, Minor};

struct Move {
    /// The root of the next chord, in semitones above the tonic.
    to: u8,
    short: &'static str,
    reason: &'static str,
}

struct Slot {
    root: u8,
    family: ChordFamily,
    moves: &'static [Move],
}

const fn go(to: u8, short: &'static str, reason: &'static str) -> Move {
    Move { to, short, reason }
}

const fn slot(root: u8, family: ChordFamily, moves: &'static [Move]) -> Slot {
    Slot { root, family, moves }
}

/// The chords of a major key, and the ones it borrows from minor.
const MAJOR: &[Slot] = &[
    slot(
        0,
        Major,
        &[
            go(5, "away from home",
               "I to IV: moving away from home, toward the dominant."),
            go(7, "tension, wants to go home",
               "I to V: tension that wants to come back to I."),
            go(9, "relative minor, darker",
               "I to vi: the relative minor, a darker colour."),
            go(2, "leads on to V",
               "I to ii: a minor chord that leads on to V."),
        ],
    ),
    // ...ii, iii, IV, V, vi, vii°, and the borrowed iv, ii°, bIII, bVI, bVII
];
```

Lee un slot de dentro hacia fuera: `go` construye un movimiento con su
motivo en una frase y en unas pocas palabras; `slot` construye la
entrada de un acorde; `MAJOR` es un slice de slots, así que puede
contener un número distinto de entradas para cada tipo de tonalidad y
para los acordes prestados. Todo ello es `const`: está integrado en el
programa y vive tanto como el programa.

Una sugerencia lleva ambos tipos de motivo:

```rust
use std::borrow::Cow;

pub struct Suggestion {
    /// The degree of the key the chord is built on, from 0 (the tonic).
    pub degree: usize,
    pub chord: Chord,
    pub reason: Cow<'static, str>,
    pub short: Cow<'static, str>,
}
```

`short` es el motivo en pocas palabras («la fundamental cae una
quinta»), para una pequeña línea de ayuda; `reason` es la frase
completa. Son `Cow<'static, str>`, «clonar al escribir»: normalmente
`Cow::Borrowed`, que apunta directamente al `&'static str` de la tabla,
sin reservar memoria. Pero el motivo puede *reescribirse* en tiempo de
ejecución: el paso 9 deja que el chord sheet escriba los numerales
romanos de las tonalidades menores de una manera o de otra, y entonces
una frase como «I a IV» tiene que convertirse en texto propio. `Cow`
admite ambos casos.

**Cambiado en el paso 8.** `Suggestion` también ganó un método
`why(level)`: `HintLevel::Detailed` devuelve la frase, `Short` las pocas
palabras y `Minimal` nada. Elige el ajuste de ayudas del piano roll. La
tabla y la idea son las de arriba.

Guardar el conocimiento como datos tiene una agradable consecuencia:
cambiar lo que sugiere el piano roll significa editar una tabla, no la
lógica, y las pruebas comprueban la tabla, no el código. Una de ellas
recorre todos los grados de tres tipos de tonalidades y comprueba que
cada uno tiene sugerencias, y que ningún acorde se sugiere a sí mismo.

### Las tonalidades menores toman prestado un V mayor

En La menor natural, el acorde del quinto grado es Mi menor: Mi, Sol, Si. No
tiene sensible (Sol está un tono por debajo de La), así que no tira con
mucha fuerza hacia casa. Hace siglos, los músicos empezaron a subir su
tercera a Sol♯, convirtiéndolo en Mi mayor, y la mayoría de la música en
tonalidades menores lo sigue haciendo. Esa nota elevada es de lo que
está hecha la escala *menor armónica*.

Así que en las tres escalas menores, `next_chords` usa los acordes de la
escala menor natural, con dos excepciones de la menor armónica. La tabla
de fundamentales y cualidades deja ambas a la vista:

```rust
/// Natural minor, with a major V and the leading-tone vii°.
const MINOR_CHORDS: [(u8, ChordQuality); 8] = [
    (0, ChordQuality::Minor),        // i
    (2, ChordQuality::Diminished),   // ii°
    (3, ChordQuality::Major),        // bIII
    (5, ChordQuality::Minor),        // iv
    (7, ChordQuality::Major),        // V, not v
    (8, ChordQuality::Major),        // bVI
    (10, ChordQuality::Major),       // bVII
    (11, ChordQuality::Diminished),  // vii°
];
```

Las fundamentales son semitonos sobre la tónica, así que la búsqueda
funciona en cualquier tonalidad menor sin nombrar las notas. La quinta
entrada da el `V` mayor, con la sensible; la última da el acorde de la
sensible. El propio `MINOR` también tiene una entrada para un v *menor*
(Mi menor en La menor), porque una canción puede usar el acorde de la
menor natural; sus movimientos son los mismos «a casa o a bVI» que los
del V mayor.

Los botones «Acordes en La menor» muestran los acordes propios de la
tonalidad, así que en La menor natural el botón del quinto grado dice
«v · Em». Cuando se sugiere Mi mayor, ese botón se ilumina y su texto al
pasar el ratón dice «tocado como Mi». También por eso una sugerencia
lleva su propio `chord`: el grado es el de la tonalidad, pero la
cualidad puede ser la de la menor armónica.

### Dominantes secundarias

¿Y un acorde que *no* está en la tonalidad? Toca Do, y después Re mayor,
en Do mayor. Re mayor tiene un Fa♯, que Do mayor no tiene. Suena como si
fuera a algún sitio, y es cierto: Re mayor es el V de Sol. Por un momento,
la música se comporta como si Sol fuera el hogar, y Re tira hacia Sol igual
que Sol tira hacia Do. A esto se le llama **dominante secundaria**, escrita
V/V («dominante de la dominante»), y es una de las maneras más comunes de añadir
color a una progresión.

`next_chords` lo comprueba antes de mirar la tabla:

```rust
// A major or dominant chord from outside the key, a fifth above one
// of the key's chords, is most often its dominant: D major in C
// major leads to G.
let root = scale.semitones_up_to(current.root);
let dominant_kind =
    current.quality == ChordQuality::Major || current.quality.is_dominant();
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

El `if` es una **let chain**: condiciones simples y patrones `let`
unidos con `&&`, cada uno capaz de usar lo que los anteriores han
ligado. Léelo como la regla que es: el acorde suena como una dominante,
no es uno de los acordes propios de la tonalidad, la nota una quinta por
debajo de él *sí* está en la tonalidad, esa nota no es la tónica, y el
acorde que se construye ahí no es disminuido.

- «una quinta por debajo» es `(root + 5) % 12`: una quinta hacia abajo y
  una cuarta hacia arriba son la misma clase de alturas, y `% 12` da la
  vuelta a la octava.
- `is_chord_of_key` es más que «sus notas están en la escala»: también
  acepta el V mayor de la tonalidad menor, que es la dominante propia de
  la tonalidad aunque la escala menor natural no tenga Sol♯.
- `target != 0`: una dominante de la tónica es simplemente V. En La
  menor, Mi mayor no es el acorde de la menor natural de la tonalidad,
  pero *sí* es el V propio de la tonalidad menor, así que va a la tabla,
  que lo manda a casa, a La menor.
- `!= Diminished`: un acorde disminuido es demasiado inestable para
  sentirse como un hogar, ni siquiera por un momento, así que nunca
  recibe función de tónica de esta manera.
- `suggest` devuelve un `Option<Suggestion>`; el `let Some` también
  rechaza un objetivo cuya fundamental no es un grado de la escala.

Una prueba comprueba los casos clásicos en Do mayor:

```rust
// D major is V of G; A7 is V of Dm; E is V of Am; C7 is V of F.
for (chord, target) in [
    (major(D, ChordQuality::Major), "G"),
    (major(A, ChordQuality::Dominant7), "Dm"),
    (major(E, ChordQuality::Major), "Am"),
    (major(C, ChordQuality::Dominant7), "F"),
] {
    let suggestions = next_chords(c_major, chord);
    assert_eq!(names(&suggestions), [target], "{chord:?}");
    assert_eq!(suggestions[0].reason, SECONDARY_DOMINANT);
}
```

C7 es uno bonito: Do es la tónica, pero C7 tiene un Si♭, así que ya no es
el acorde de tónica. Es el V de Fa, y suena como tal.

La tabla ya conoce los acordes comunes tomados prestados de la tonalidad
menor homónima: iv, ii°, bIII, bVI y bVII. Si♭ mayor en Do mayor es bVII,
y sugiere Do (la cadencia de rock) o Fa (el «doble plagal»). Lo que no
conoce es el **napolitano** bII, el acorde mayor sobre el segundo grado
rebajado (Re♭ en Do). El ejercicio 4 lo añade.

### En el piano roll

El piano roll pide sugerencias sobre un acorde, elegido por
`current_chord`:

```rust
fn current_chord(&self, drawing: Drawing) -> Option<(ChordAt, Chord, PitchClass)> {
    let held_chord = match analyze(drawing.held.iter().copied()) {
        Harmony::Chord { chord, bass } => Some((ChordAt::Keys, chord, bass)),
        _ => None,
    };
    held_chord.or_else(|| {
        let step = self.last_step.or(drawing.playhead)?;
        match analyze(sounding(drawing.pattern, drawing.heard, step)) {
            Harmony::Chord { chord, bass } => Some((ChordAt::Step(step), chord, bass)),
            _ => None,
        }
    })
}
```

Un acorde mantenido en un teclado va primero: es en lo que estás
pensando ahora mismo. Si no, se usa el acorde del último step señalado,
o el step que se está reproduciendo cuando el puntero no ha estado sobre
la cuadrícula. `last_step` se recuerda a propósito cuando el puntero
sale de la cuadrícula; si no, las sugerencias desaparecerían en cuanto
movieras el ratón hacia arriba, a los botones, para leerlas. Durante la
reproducción, las sugerencias siguen a la música.

La llamada es `next_chords_over(scale, chord, bass)`, no `next_chords`,
porque el bajo importa: un acorde de tónica con la quinta en el bajo
(C/G en Do mayor) se oye como una sexta y cuarta cadencial y conduce a V.
Para todo lo demás, simplemente le pasa el trabajo a `next_chords`.

El botón de cada acorde sugerido se dibuja con `Button::selected(true)`,
el mismo aspecto resaltado que una pestaña elegida, y su motivo va
primero en su texto al pasar el ratón. La línea gris bajo los botones
enumera todas las sugerencias en orden, para poder leerlas de un
vistazo: «Después de C (I) en el step 5, prueba: F (IV), G (V), Am (vi),
Dm (ii)».
