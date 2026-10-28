# La biblioteca: el conocimiento como datos

Archivos: [`crates/lineup-core/src/library/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates/lineup-core/src/library)
y [`crates/lineup-audio/src/drums.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/drums.rs)

Un músico que se sienta a tocar una bossa nova sabe cuatro cosas: los
acordes, cómo los toca la guitarra, qué hace el bajo y el patrón de
batería. Nada de eso es código. Es *conocimiento*, y la mejor forma de
poner conocimiento en un programa es escribirlo como datos, en una forma
que una persona pueda leer y comprobar, y mantener pequeño el código que
lo usa.

El paso 6 hizo esto con las sugerencias de acordes, con una tabla
`const`. El paso 8 lo hace con una biblioteca entera: 78 estilos de
armonía, 49 ritmos, 6 fills y 62 canciones. Cada uno son unas pocas
líneas en una tabla. Este capítulo recorre las notaciones, de los
acordes a las canciones, y el Rust que las lee.

## Acordes como números romanos

`numeral.rs`

Una progresión escrita con nombres de acordes, "C G Am F", solo funciona
en Do. La misma progresión con numerales, "I V vi IV", funciona en
cualquier tonalidad: el numeral dice sobre qué nota de la tonalidad se
construye el acorde, y su caja dice mayor (mayúscula) o menor
(minúscula). Lo que sigue añade el resto: "7", "maj7", "°", "sus4"...

Hay una elección que hacer: ¿contados a lo largo de *qué* escala? La
biblioteca cuenta a lo largo de la escala **mayor** de la tónica, sea
cual sea la tonalidad, como hacen la mayoría de los libros de pop y de
jazz. III está siempre cuatro semitonos sobre la tónica, y un bemol
delante lo baja, así que los acordes de La menor se escriben i, ii°,
bIII, iv, v, bVI, bVII. Eso permite que una tabla contenga progresiones
en cualquier modo, y "bVII" significa el mismo acorde en una tonalidad
menor, una canción de rock mixolidia y una jam dórica.

```rust
pub struct RomanChord {
    pub semitones: u8,
    pub quality: ChordQuality,
    pub spelling: Option<Spelling>,
    /// The bass note of a slash chord, in semitones above the tonic:
    /// in C, G/B has 11. `None` means the root is in the bass.
    pub bass: Option<u8>,
}
```

`parse` lee un numeral. Los numerales más largos se prueban primero,
para que "III" no se lea como "I" seguido de "II", y los sufijos salen
de una tabla que dice qué calidad significa cada sufijo tras un numeral
en mayúscula y tras uno en minúscula:

```rust
const SUFFIXES: [(&str, Option<ChordQuality>, Option<ChordQuality>); 20] = {
    use ChordQuality::*;
    [
        ("", Some(Major), Some(Minor)),
        ("7", Some(Dominant7), Some(Minor7)),
        ("maj7", Some(Major7), Some(MinorMajor7)),
        // ...
        ("°", None, Some(Diminished)),
        // ...
    ]
};
```

Un `const` puede ser un bloque entero que termina en un valor, como
aquí, y el `use` de dentro solo vale dentro del bloque. `None` significa
"no permitido": "V°" no es un acorde que nadie escriba.

`spelling` recuerda un bemol o un sostenido delante, así que bII7 en Do
se escribe Re♭7, no Do♯7: la misma tecla en el piano, pero el nombre le
dice a un músico qué hace el acorde.

`bass` recuerda la nota del bajo de un acorde con barra, también en
semitonos sobre la tónica: Sol/Si en Do es Sol mayor con 11, por Si, en el
bajo. `None` significa que la fundamental está en el bajo, que es casi
todo acorde.

## Tablas de acordes: acordes por compás

`chart.rs`

Un **lead sheet**, la página que lee un músico en activo, tiene la
melodía con los nombres de los acordes encima, compás a compás. La
biblioteca escribe los acordes igual, con `|` entre compases:

```text
"I | IV V | vi | ii7 V7"
```

Los acordes de un compás se lo reparten a partes iguales, así que "IV V"
es medio compás cada uno. Un acorde que repite al anterior se
*mantiene*, no se toca otra vez: "I | I" son dos compases de un mismo
acorde. `parse_chart` convierte una tabla en `ChordSpan`s, un acorde con
dónde empieza y cuánto dura, en pasos del compás del estilo.

## Comping: cómo se tocan los acordes

`comping.rs`

Los mismos cuatro acordes pueden ser una balada, un tema de reggae o una
pista de house. Lo que cambia es el ritmo de los acordes, que los
músicos de jazz llaman **comping** (abreviación de acompañar), y lo que
hace el bajo. Cada manera de hacer comping es una tabla corta de *hits*:
cuándo empieza una nota, cuánto dura, qué notas del acorde toca y con
qué fuerza:

```rust
Comping::Pulse => (4, &[hit!(0, 2, All, 96), hit!(2, 2, All, 78)]),
Comping::Offbeat => (4, &[hit!(2, 1, All, 92)]),
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

El primer número es el *periodo*: los hits se repiten cada 4 pasos en
`Pulse` (corcheas), cada 8 en el largo, corto, largo, largo de la
habanera. `Offbeat` es el "skank" del reggae: un acorde corto en la
segunda corchea de cada pulso. `Upper` toca todas las notas del acorde
menos la más grave, como el golpe ascendente más suave de un
guitarrista.

### `macro_rules!`, y por qué no una función

`hit!` es una pequeña *macro declarativa*:

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
recibe con un patrón (aquí cuatro expresiones, `$at:expr` y demás) y las
pega en la plantilla de la derecha. `hit!(0, 2, All, 96)` se convierte
en `Hit { at: 0, len: 2, what: All, velocity: 96 }` antes de que el
compilador compruebe nada.

¿Por qué no un `const fn hit(...) -> Hit`? Mira el tipo que devuelve el
match: `&'static [Hit<Voices>]`, un slice prestado que vive todo el
programa. Escribir `&[ ... ]` de valores simples da exactamente eso: el
compilador ve una expresión hecha solo de constantes, la pone en los
datos de solo lectura del programa y deja que el préstamo viva para
siempre. Eso se llama **promoción de constantes**. Pero solo ocurre con
valores simples: una *llamada a función* dentro de los corchetes, aunque
sea un `const fn`, no se promueve, y el compilador se queja de que el
temporal prestado se suelta al final de la sentencia. La macro mantiene
la tabla corta de escribir y hecha de valores simples.

El bajo funciona igual, con `Tone`s en lugar de `Voices`: la
fundamental, la quinta, la fundamental del acorde siguiente tocada antes
(una "anticipación"), o la nota a un semitono de ella, que conduce a la
entrada.

## Estilos

`style.rs`

Un estilo lo junta todo:

```rust
style(
    "Tango síncopa",
    Tango,
    HarmonicMinor,
    "i | iv | V7 | i",
    Comping::Tresillo,
    Bass::Tresillo,
    118.0,
    "Tango síncopa",
    "Accents on 1, the \"and\" of 2, and 4: the 3 + 3 + 2 that tango inherited from the habanera.",
),
```

El nombre, el género, el modo en el que están los acordes, la tabla, el
comping, el bajo, el tempo habitual, el ritmo que lo acompaña y una
frase sobre qué escuchar. `style` es un `const fn` que rellena 4/4; los
pocos estilos en otro compás añaden `.in_time(THREE_FOUR)`, otro
`const fn`, que usa la *sintaxis de actualización de struct*:

```rust
const fn in_time(self, time: TimeSignature) -> Style {
    Style { time, ..self }
}
```

`..self` significa "todos los demás campos como en `self`".

La tabla en sí es un `static`:

```rust
pub static STYLES: &[Style] = &[ /* ... */ ];
```

Un `const` se pega en cada sitio que lo usa, como una macro; un `static`
es un único valor en una única dirección de memoria para todo el
programa. Para una tabla grande que se usa en muchos sitios, `static`
guarda una sola copia.

`Style::render` convierte un estilo en notas en cualquier tonalidad: la
tabla se vuelve spans, el numeral de cada span se vuelve un acorde en la
tonalidad, los acordes se colocan en voces suavemente uno tras otro
(mira el [capítulo anterior](harmony-keys.md#tocar-un-acorde-voice-leading)),
y el comping y el bajo los tocan.

## Ritmos, una línea por tambor

`rhythm.rs`

Los bateristas piensan un ritmo como una cuadrícula: una fila por
tambor, una columna por paso. La biblioteca escribe exactamente eso,
como texto:

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

`X` es un golpe fuerte, `x` uno normal, `o` una "ghost note" suave y `.`
silencio. `h` y `l` tocan el tambor cinco semitonos más agudo o más
grave, para los tonos abiertos y apagados de una conga o dos toms. Los
espacios solo hacen los pulsos fáciles de ver, y `|` separa compases,
para ritmos de más de uno. Un ritmo puede nombrar un *fill*, una ruptura
de un compás que las demos ponen al final de una sección.

Cuando un ritmo entra en un patrón, se distribuye compás a compás en el
compás del proyecto. Un ritmo de 4/4 en un patrón de 3/4 pierde su
último pulso en cada compás, en lugar de deslizarse a través de las
líneas de compás:

```rust
for start in (0..length).step_by(bar) {
    let steps = &bars[(start / bar) % bars.len()];
    for (step, hit) in steps.iter().enumerate().take(bar) {
        // ...
    }
}
```

`step_by(bar)` da el comienzo de cada compás, y `take(bar)` corta un
compás largo en la línea de compás.

### Nueve tambores nuevos

Los ritmos necesitaban más que el kick, la snare, el clap y el hi-hat
del paso 2: un hi-hat abierto, un golpe de rim, dos toms, un ride y un
crash, un cencerro, un shaker y una conga. Se sintetizan como los cuatro
primeros, con ondas senoidales, ruido y filtros.

Los platillos usan un truco de las cajas de ritmo analógicas de
principios de los 80. Seis ondas cuadradas en frecuencias que no son
múltiplos unas de otras se suman:

```rust
const METAL: [f32; 6] = [205.3, 304.4, 369.6, 522.7, 540.0, 800.0];

fn metal(t: f32, ratio: f32) -> f32 {
    METAL.iter().map(|&f| square(t, f * ratio)).sum::<f32>() / METAL.len() as f32
}
```

Las notas cuyas frecuencias son múltiplos simples unas de otras se
funden en un acorde. Estas no, así que juntas chocan y titilan como
metal. Un filtro paso alto se queda con la parte brillante, y la
envolvente decide si es un hat cerrado, un hat abierto, un ride o un
crash. El cencerro son solo dos de esas ondas cuadradas, a poco más de
una quinta de distancia.

## Canciones

`song.rs`

Las demos necesitaban canciones que todo el mundo conozca, y tenían que
ser libres de usar. Una canción es libre cuando está en el **dominio
público**: canciones folclóricas que no son de nadie, y música de
compositores que murieron hace mucho. *Ode to Joy* (Beethoven, 1824),
*Für Elise*, *Canon in Re*, *Greensleeves*, *Frère Jacques*,
*Korobeiniki* y las demás lo están todas. Algunas melodías antiguas
famosas no, o no en todas partes, porque un arreglo más nuevo o la letra
siguen protegidos; esas se dejaron fuera.

Una melodía se escribe nota a nota:

```text
"F#5:4 F#5 G5 A5 | A5 G5 F#5 E5 | D5 D5 E5 F#5 | F#5:6 E5:2 E5:8"
```

`F#5` es el Fa sostenido de la quinta octava, y `:4` su longitud en
pasos (una negra). Una nota sin longitud dura lo mismo que la anterior,
lo que mantiene la línea corta. `-` es un silencio, `_` liga la nota
anterior a través de una línea de compás, y `|` es una línea de compás,
que no cambia nada pero permite que una prueba compruebe que cada compás
suma lo que marca su compás.

Una canción tiene **partes** (una estrofa, un estribillo, una sección
"A" y una "B"), cada una con su melodía y su tabla de acordes, y una
**forma**: el orden en que se tocan las partes. Cada canción se viste de
un estilo con su comping, su bajo y su ritmo: *Ode to Joy* como pista de
house, *Twinkle, Twinkle* como lo-fi, *When the Saints* como dixieland,
*Drunken Sailor* como folk punk.

### Rondas

*Frère Jacques* es una **ronda**: la misma melodía cantada por varias
voces, cada una entrando dos compases después de la anterior. Escribir
todas las voces sería largo y fácil de equivocar, así que una parte
puede decir dónde entran sus voces, en compases:

```rust
Part::new("Round", FRERE_JACQUES_TUNE, FRERE_JACQUES_CHORDS).with_voices(&[
    (1, 0),
    (2, 2),
    (3, 4),
    (4, 6),
]),
Part::new("Round again", FRERE_JACQUES_TUNE, FRERE_JACQUES_CHORDS).with_voices(&[
    (1, 0),
    (2, -6),
    (2, 2),
    (3, -4),
    (3, 4),
    (4, -2),
    (4, 6),
]),
```

Cada pareja es `(voice, bar)`: qué voz canta, y el compás de la parte en
el que entra. El primer campo también elige el canal: la voz 1 es el
canal Lead, y las voces 2, 3 y 4 reciben "Voice 2", "Voice 3" y "Voice
4". En "Round again", las copias que empezaron en la parte anterior
siguen: la voz 2 en -6 empezó seis compases antes, así que en esta parte
oyes sus dos últimos compases. Las notas que caen fuera de la parte se
descartan:

```rust
for &(voice, start_bar) in part.voices {
    let Some(notes) = voice.checked_sub(1).and_then(|index| sung.get_mut(index)) else {
        continue;
    };
    let offset = start_bar as isize * bar as isize;
    for note in &melody.notes {
        let start = note.start as isize + offset;
        if (0..length as isize).contains(&start) {
            notes.push(NoteEvent {
                start: start as usize,
                ..*note
            });
        }
    }
}
```

`checked_sub(1)` convierte la voz 1 en el índice 0, y `get_mut` rechaza
una voz que la canción no tiene. Los desplazamientos pueden ser
negativos, así que las sumas se hacen en `isize`, un entero con signo
del tamaño de `usize`. Solo se conservan los comienzos dentro de la
parte, `0..length`, así que la conversión de vuelta a `usize` no puede
salir mal. `..*note` es otra vez sintaxis de actualización de struct:
una copia de la nota con un comienzo nuevo. *Canon in Re* usa el mismo
truco para sus tres violines.

## ¿Está bien?

Los datos pueden estar mal con la misma facilidad que el código, y una
nota equivocada en una tabla no hace fallar nada. Así que las pruebas
leen la biblioteca entera:

- cada tabla, línea de ritmo y melodía se parsea,
- cada compás de cada melodía dura exactamente lo que dicta su compás, y
  cada tabla de acordes reparte sus compases a partes iguales,
- el ritmo de cada estilo y las partes de cada canción existen,
- cada nombre de estilo es único, y cada género tiene estilos,
- las primeras notas de cada canción son las que la gente conoce,
- al menos el 80% de las notas de cada melodía están en la tonalidad de
  la canción. El resto son notas de paso y giros cromáticos, como el
  Re♯ de *Für Elise*; una melodía con menos notas en su tonalidad casi
  seguro está escrita en la tonalidad equivocada.

Una primera versión de esa última prueba comprobaba que cada nota
estuviera "a un semitono de la escala". Siempre pasaba: en una escala de
siete notas, *toda* nota está a un semitono de la escala. Una prueba que
no puede fallar no prueba nada, así que se reemplazó. Cuando escribas
una prueba, intenta romper el código a propósito una vez, y comprueba
que la prueba se da cuenta.
