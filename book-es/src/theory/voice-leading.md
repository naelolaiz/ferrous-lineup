# Conducción de voces y movimiento de las partes

Archivos: [`crates/lineup-core/src/harmony/voice_leading.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/voice_leading.rs),
[`crates/lineup-core/src/harmony/voice_motion.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/voice_motion.rs)
y [`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs)

La línea de acordes nombra cada acorde que escribes. Esa es la mitad de
la armonía. La otra mitad es la **conducción de voces**: cómo va cada
parte desde su nota en un acorde hasta su nota en el siguiente. Una
parte es una línea de notas, como un cantante, un instrumento o una
línea dentro de los acordes de un piano. Dos obras pueden usar los
mismos acordes y sonar muy distintas, fluidas o torpes, porque sus
partes se mueven de otra manera.

Marca **Conducción de voces**, junto a **Notas fantasma** en el piano
roll, y hace dos cosas que hace un profesor de armonía. Marca dónde las
partes rompen las reglas de la escritura clásica a varias voces, la
lapicera roja; y muestra cómo se mueve cada parte en cada cambio de
acorde, bien o mal, que es casi todo lo que hay que aprender. Este
capítulo trata la teoría, las reglas y sus excepciones, y el código:
programación dinámica para partir una pista en partes, cadenas de
`Option` para encontrar quintas paralelas, y una caché para que la
revisión corra solo cuando algo cambió.

## La música, en pocas palabras

Los cursos de armonía clásica enseñan la conducción de voces con la
escritura a cuatro partes: soprano, contralto, tenor y bajo, como en
los corales de Bach. Dan unas pocas reglas que mantienen a cada parte
como una línea propia. La revisión sigue las tres más conocidas: **sin
quintas ni octavas paralelas**, **la sensible sube**, **la séptima
baja**. Se explican una por una en [Las reglas](#las-reglas), más
abajo. Otras dos ideas describen cada movimiento, bueno o malo.

### Grados conjuntos, saltos pequeños y saltos

Lo lejos que se mueve una parte tiene un nombre:

| Movimiento | Intervalo | Semitonos | Ejemplo |
|---|---|---|---|
| **Sostenida** | la misma nota | 0 | Sol3 a Sol3 |
| **Grado conjunto** | una segunda | 1 o 2 | Fa4 a Mi4 |
| **Salto pequeño** | una tercera | 3 o 4 | Si3 a Sol3 |
| **Salto** | una cuarta o más | 5 o más | Sol2 a Do3 |

Las partes internas de un coral se mueven casi siempre por grado
conjunto o sostienen sus notas. El bajo salta más, porque va de
fundamental a fundamental: Sol2 subiendo una cuarta a Do3 en una
cadencia es uno de los movimientos de bajo más comunes que hay.

### El movimiento entre dos partes

Dos partes van juntas de una de cuatro maneras:

| Movimiento | Las dos partes | Ejemplo |
|---|---|---|
| **Paralelo** | van en el mismo sentido y conservan el mismo intervalo | terceras que van a terceras |
| **Directo** | van en el mismo sentido, a otro intervalo | una sexta que va a una quinta, las dos bajando |
| **Contrario** | van en sentidos opuestos | el bajo sube, la parte superior baja |
| **Oblicuo** | una se mueve, la otra se queda | una nota sostenida debajo de una que se mueve |

La escritura a varias voces prefiere el movimiento **contrario** y el
**oblicuo** entre las partes extremas, la línea superior y el bajo,
porque son las dos que más sigue el oído. Ir en sentidos opuestos las
mantiene como dos líneas. Las terceras y sextas paralelas están bien en
cualquier lugar, y suenan dulces. Las quintas y octavas paralelas son lo
que marca la revisión, porque hacen que dos partes suenen como una.

«El mismo intervalo» quiere decir el mismo **número** de intervalo tal
como lo escribe la tonalidad, no la misma cantidad de semitonos. De Mi4
sobre Sol3 a Fa4 sobre La3, la parte de arriba sube una segunda menor y
la de abajo una segunda mayor. Mi sobre Sol es una sexta mayor y Fa
sobre La una sexta menor: una sexta las dos veces, así que es movimiento
paralelo, sextas paralelas. Un profesor de armonía también lo cuenta
así.

## Lo que ves

- **Una fila Voces** debajo de las dos filas de nombres de acordes.
  Cada hallazgo es un bloque encima del acorde al que llegan las
  partes, nombrado con unas pocas letras: **5as** y **8vas** para
  quintas y octavas paralelas, **unís.** para unísonos, **sens.** para
  una sensible que no sube, **7.ª** para una séptima que no baja.
- **Líneas magenta sobre la grilla**, desde la nota que deja una parte
  hasta la nota a la que va, donde se rompe una regla, y **un contorno
  punteado** donde debería haber ido una sensible o una séptima.
- **Una línea de color para cada parte**, de cada una de sus notas a la
  siguiente, encima de las notas, para que el ojo pueda seguir una
  parte por todo el patrón, como en una partitura. Una nota sostenida
  no tiene línea: la parte se queda donde está. Los movimientos del
  cambio al que apuntas se dibujan más gruesos; apuntar a cualquier
  lugar de un acorde elige el cambio que llevó a él.
- **Una línea encima de la grilla.** Cuando apuntas a un cambio sin nada
  que marcar, nombra cada parte de arriba abajo, con adónde va, y luego
  el movimiento entre las partes extremas: «Piano: superior Re5→Do5 ·
  2.ª Fa4→Mi4 · 3.ª Si3→Sol3 · inferior Sol2→Do3; partes extremas en
  movimiento contrario». En una sola pista las partes se nombran por su
  lugar; con varias pistas, por pista. Su tooltip dice cada movimiento
  en palabras («baja una segunda mayor de Re5 a Do5, por grado
  conjunto»), y el movimiento entre cada par de partes vecinas. Cuando
  apuntas a un paso con un hallazgo, la línea dice en su lugar qué pasa
  allí, en magenta, y su tooltip dice por qué lo evita la escritura
  clásica a varias voces. Sin apuntar a nada, resume la revisión:
  «Conducción de voces, 4 partes: quintas paralelas (1), …».

El tooltip escribe cada intervalo según la tonalidad: de Si3 a Sol3 hay
una tercera mayor, no «cuatro semitonos», y de Fa♯ a Sol una segunda
menor. Una nota tocada otra vez aparece como sostenida en la línea, y
el tooltip dice «vuelve a tocar Do5», para que la distingas de una nota
que se mantiene.

## Qué notas forman una parte

La revisión necesita partes, pero el piano roll tiene pistas. Una pista
que toca una nota a la vez es una parte. Una pista que toca acordes,
como un piano, tiene varias partes. La aplicación la parte como
leerías una partitura: la nota de arriba de cada acorde va a la parte
de arriba, la siguiente a la parte siguiente, y cada nota se une a la
parte cuya última nota estaba más cerca. Los mensajes las llaman «Mano
derecha (voz superior)» y «Mano derecha (voz inferior)», o «Piano (voz
más aguda)», «Piano (voz 2 de 4)» y «Piano (voz más grave)».

Con **Notas fantasma** activado, se revisan también las otras pistas.
Las pistas de batería quedan fuera.

Un silencio de una negra o más corta una línea, así que las notas a
cada lado de él no se comparan entre sí. Un par de partes que toca en
octavas casi todo el tiempo, como los contrabajos que doblan a los
violonchelos, es una línea más fuerte, no dos partes en octavas
paralelas: al menos cuatro notas juntas, y siete de cada ocho a una
octava o al unísono. La revisión cuenta ese par una sola vez, y el
resumen dice «una línea duplicada en octavas».

## Las reglas

### Sin quintas ni octavas paralelas

Toma dos partes a una quinta justa de distancia. Si las dos se mueven
en el mismo sentido hacia otra quinta justa, son **quintas paralelas**.
Lo mismo vale para las octavas y los unísonos. Esos intervalos se
funden tan bien que, en ese momento, las dos partes suenan como una. La
textura pierde una línea, así que la escritura clásica a varias voces
los evita. Una quinta más una octava (una duodécima) sigue contando
como quinta.

Cuando las partes van en sentidos opuestos, de una quinta a una
duodécima, forman **quintas por movimiento contrario**. Cada parte se
mueve distinto, pero el oído sigue oyendo dos quintas seguidas, así que
la escritura estricta también las evita. Las octavas por movimiento
contrario aparecen de vez en cuando entre la parte superior y el bajo
en una cadencia, y el mensaje lo dice.

Do mayor a Re menor con todas las partes subiendo por grado conjunto es
el caso de manual: el soprano Sol4 y el tenor Do4 están a una quinta, y
van a La4 y Re4, otra vez a una quinta: «Quintas paralelas: Soprano y
Tenor van de Sol4 sobre Do4 (una quinta) a La4 sobre Re4 (una quinta),
las dos suben.» El tenor y el bajo, de Do4 sobre Do3 a Re4 sobre Re3,
forman octavas paralelas.

Una nota sostenida o repetida no cuenta. Las dos partes tienen que
moverse.

### La sensible sube

La **sensible** es la nota un semitono por debajo de la tónica: Si en
Do mayor, Sol♯ en La menor. Cuando un acorde de dominante (V, V7, vii°,
vii°7) va a la tónica, la sensible tira hacia arriba, a la tónica.

La regla es estricta en la parte superior y en el bajo, donde el oído
sigue la línea. Una parte interna es más libre. En V7 a I, una sensible
interna a menudo baja a la quinta del acorde de tónica, para que ese
acorde tenga sus tres notas. Los libros la llaman sensible «frustrada»,
y la revisión deja tranquilas las partes internas.

La revisión conoce tres lugares donde la sensible tiene que subir:

- **V o vii° a I**, la cadencia.
- **V a vi** (V a VI en menor), la **cadencia rota**. La sensible igual
  sube, a la tónica, que ahora es la tercera de vi.
- **Una dominante secundaria** que va a su acorde: D7 a G en Do mayor.
  D7 toma prestado Fa♯ de fuera de la tonalidad, y Fa♯ es la sensible de
  Sol, así que sube a Sol. C7 yendo a F en Do mayor también es una
  dominante secundaria (V7/IV), pero su sensible, Mi, pertenece a la
  tonalidad, y conserva la libertad que tiene en Do.

Si los dos acordes tienen la sensible, puede quedarse donde está. Una
sensible que baja por una escala (Do, Si, La) es una nota de paso, no
una que tenga que subir. Y V seguido de I⁶₄ no es una llegada, porque
las partes siguen antes de que la sensible tenga adónde ir.

Cuando dos partes tienen la sensible, el mensaje dice también que está
duplicada. La escritura clásica a varias voces nunca duplica la
sensible: las dos copias no pueden subir sin formar octavas paralelas.

### La séptima baja

En un acorde de séptima como G7 (Sol Si Re Fa), la séptima, Fa, es una
disonancia. **Resuelve bajando por grado conjunto** en el acorde
siguiente. En G7 a C, Fa baja a Mi. Si el Fa5 del soprano sube a Sol5,
la revisión marca una séptima que no baja: la séptima de G7 (V7) sube a
Sol5 en C (I) en lugar de bajar por grado conjunto a Mi5, y el
contorno punteado marca Mi5.

La revisión lee algunos acordes como lo hacen los libros de armonía.
Fa La Do Re yendo a G se llama F6 por sus notas, pero funciona como Dm7
con Fa en el bajo, ii⁶₅ en Do. Su séptima es Do, que debería bajar a
Si. La fundamental de un acorde de séptima disminuida sale de adónde
va: Sol♯ Si Re Fa yendo a A tiene su fundamental en Sol♯, y su séptima
es Fa.

Las excepciones de manual no se marcan:

- **El acorde siguiente tiene la nota.** Entonces la séptima puede
  quedarse donde está.
- **Un retardo.** La séptima se mantiene en el acorde siguiente y baja
  un poco después.
- **Notas de paso y bordaduras.** Una nota corta que llega y se va por
  grado conjunto forma una séptima solo de paso. Lo mismo una nota
  añadida fuera del pulso mientras las otras partes sostienen las
  suyas.
- **V⁴₃ a I⁶.** La séptima (Fa en G7 con Re en el bajo) puede subir a
  Sol mientras el bajo sube de Re a Mi una décima por debajo.
- **Un acorde de paso.** En C, luego Em7 sobre Re, luego C sobre Mi, el
  Re del bajo es una nota de paso entre dos formas del mismo acorde de
  Do.
- **Conducción cromática.** Una séptima puede subir un semitono hacia la
  sensible propia del acorde siguiente. En Am7 con Sol en el bajo yendo
  a E7 con Sol♯ en el bajo, Sol sube a Sol♯. La línea sigue subiendo, y
  el acorde siguiente la resuelve.
- **El bajo en otra octava.** El bajo puede tomar su nota una octava
  más allá: Si2 a Do2 es la sensible yendo a la tónica.

### Lo que la revisión no juzga

Son las reglas de un estilo: la escritura a varias voces de Bach a
Brahms, y de los ejercicios de armonía. El pop y el jazz mueven acordes
enteros en paralelo a propósito, y Debussy también. Nadie espera que
una séptima de jazz resuelva. Lee la revisión como un profesor del
estilo clásico, no como un juez de toda la música. Donde la tonalidad
es una serie dodecafónica, la revisión sigue en cambio las reglas de la
escritura dodecafónica (ver [Las reglas de la escritura
dodecafónica](twelve-tone.md#las-reglas-de-la-escritura-dodecafónica)),
y donde es un conjunto de clases de altura, no se revisa nada:

```rust
pub fn of(key: Scale) -> Rules {
    match key.kind {
        ScaleKind::Row(_) => Rules::TwelveTone,
        ScaleKind::Set(_) => Rules::Neither,
        _ => Rules::Classical,
    }
}
```

Algunas reglas de los libros de armonía no se revisan: las quintas y
octavas directas (ocultas) en las partes extremas, la distancia entre
las partes, el cruce de partes, y qué nota de un acorde duplicar.

## En el código

### Repartir las notas de un acorde: programación dinámica

`voices_of` convierte las notas de una pista en partes. Recorre los
acordes en el tiempo, cortando las notas donde cambia su comienzo con
`chunk_by`. Para cada acorde, `share_out` decide a qué parte va cada
nota. Las notas y las partes están ordenadas de la más aguda a la más
grave, y una nota solo puede ir a una parte por debajo de la que tomó
la nota de arriba, así que las partes nunca se cruzan. Entre todas las
maneras de hacerlo, elige la de menor distancia total en semitonos.

Probar todas las maneras tardaría demasiado con un acorde grande. La
**programación dinámica** lo resuelve con una tabla. `cost[i][j]` es el
menor costo de ubicar las primeras `i` notas usando solo las primeras
`j` partes:

```rust
for i in 1..=notes {
    for j in 1..=lines {
        let skip_part = cost[i][j - 1];
        let new_part = cost[i - 1][j] + NEW_PART;
        let take = cost[i - 1][j - 1] + distance(i - 1, j - 1);
        cost[i][j] = skip_part.min(new_part).min(take);
    }
}
```

Cada celda tiene tres opciones. Saltar la parte `j`, que no recibe
ninguna nota en este acorde. Empezar una parte nueva para la nota `i`,
con el costo grande `NEW_PART` (10.000, más que cualquier distancia),
así que solo pasa cuando un acorde tiene más notas que partes. O poner
la nota `i` en la parte `j`, con el costo de la distancia entre ellas.
Cada celda se arma con celdas ya llenas, así que toda la tabla lleva
`notas × partes` pasos. Volver hacia atrás desde la última celda
recupera qué opción dio cada costo, y así adónde fue cada nota. La
[revisión
dodecafónica](twelve-tone.md#la-mejor-lectura-programación-dinámica)
usa la misma idea para encontrar qué formas de una serie se tocan.

### Comparar dos partes nota contra nota

`together` lista cada momento en que alguna de las dos partes empieza
una nota, con la nota en la que está cada parte en ese momento:

```rust
onsets
    .into_iter()
    .map(|tick| a.current(tick).zip(b.current(tick)))
    .collect()
```

`Option::zip` convierte dos `Option` en un `Option` de un par. Es
`Some` solo cuando los dos son `Some`, es decir, cuando las dos partes
están tocando. `consecutive` mira después cada par de momentos vecinos,
y `kind_of_move` decide qué forma el movimiento:

```rust
fn kind_of_move(x: Hop, y: Hop) -> Option<SlipKind> {
    let moves = |(from, to): Hop| {
        let rested = to.start_tick() >= from.end_tick() + LINE_BREAK;
        (from.pitch != to.pitch && !rested).then(|| to.pitch > from.pitch)
    };
    let (up_x, up_y) = (moves(x)?, moves(y)?);
    let first = perfect(x.0.pitch.midi().abs_diff(y.0.pitch.midi()))?;
    let second = perfect(x.1.pitch.midi().abs_diff(y.1.pitch.midi()))?;
    let same_way = up_x == up_y;
    match (first, second) {
        (Perfect::Fifth, Perfect::Fifth) if same_way => Some(SlipKind::ParallelFifths),
        (Perfect::Fifth, Perfect::Fifth) => Some(SlipKind::ContraryFifths),
        (Perfect::Unison, Perfect::Unison) => Some(SlipKind::ParallelUnisons),
        (Perfect::Fifth, _) | (_, Perfect::Fifth) => None,
        _ if same_way => Some(SlipKind::ParallelOctaves),
        _ => Some(SlipKind::ContraryOctaves),
    }
}
```

`moves` da `Some(true)` para una parte que sube, `Some(false)` para una
que baja, y `None` para una nota sostenida o un silencio de una negra o
más: `bool::then` corre su closure solo cuando la condición se cumple.
Cada `?` devuelve `None` de la función en cuanto algo falta, y
`perfect` da `None` para un intervalo que no es unísono, quinta ni
octava. Lo que queda después de los `?` es el caso interesante, dos
intervalos justos seguidos, y un `match` sobre el par le pone nombre.
Las ramas se prueban en orden, así que la rama de quinta a otra cosa
tiene que ir antes de las dos ramas de octavas que atrapan todo lo que
queda.

### Nombrar una parte con un closure

Un hallazgo conoce sus partes por número. Solo la aplicación conoce los
nombres de las pistas, así que `describe` recibe una función que nombra
una parte:

```rust
pub fn describe(&self, voice_name: impl Fn(usize) -> String) -> String {
```

El piano roll le pasa un closure:

```rust
let name = |voice: usize| voice_name(drawing.project, check, voice);
// ...
let why: Vec<String> = here.iter().map(|slip| slip.describe(name)).collect();
```

`name` se pasa por valor, una vez por cada hallazgo, y aun así se puede
seguir usando después. Funciona porque el closure solo guarda
referencias compartidas (`&Project`, `&VoiceLeading`). Un closure cuyas
capturas son todas `Copy` es `Copy` también, así que cada llamada
recibe su propia copia. (Clippy lo dice también: marca
`describe(&name)` como un préstamo innecesario.)

### Revisar solo cuando algo cambió

Revisar un patrón lleva menos de un milisegundo en una compilación
release, pero el piano roll dibuja 60 fotogramas por segundo mientras
suena una canción. Así que guarda la última revisión junto con aquello
de lo que se hizo:

```rust
#[derive(Debug, Clone, PartialEq)]
struct VoiceInput {
    pattern: usize,
    /// The notes of each channel checked.
    notes: Vec<(usize, Vec<NoteEvent>)>,
    time: TimeSignature,
    /// The key on each beat.
    keys: Vec<Scale>,
}
```

Cada fotograma arma un `VoiceInput` nuevo y lo compara con el guardado.
`#[derive(PartialEq)]` compara campo por campo, hasta cada nota.
Comparar es mucho más barato que revisar, y cuando nada cambió se usa
la revisión anterior:

```rust
if let Some((made_from, check)) = &self.voice_check
    && *made_from == input
{
    return Rc::clone(check);
}
```

`if let … && …` es una **let chain**, de la edición 2024. Prueba un
patrón y una condición en un solo `if`.

La revisión se guarda en un `Rc<VoiceLeading>`, un puntero compartido.
Dibujar necesita la revisión mientras también cambia el piano roll
(`&mut self`). Tomar prestada la revisión de `self` bloquearía `self`
todo el fotograma, así que el borrow checker lo rechazaría.
`Rc::clone` solo suma uno a una cuenta, así que el fotograma recibe un
handle propio por el costo de incrementar un entero, sin copiar los
hallazgos.

### Seguir las partes en cada cambio

`voice_motion::changes` recorre los momentos en que empieza una nota
nueva en alguna parte. Para cada parte toma la nota que tenía justo
antes y la nota que tiene allí:

```rust
fn before(voice: &Voice, tick: usize) -> Option<NoteEvent> {
    let index = voice
        .notes
        .partition_point(|note| note.start_tick() < tick)
        .checked_sub(1)?;
    let note = voice.notes[index];
    (tick < note.end_tick() + LINE_BREAK).then_some(note)
}
```

Las notas de una parte están ordenadas por comienzo, así que
`partition_point` encuentra con una búsqueda binaria cuántas empiezan
antes de `tick`. La anterior a esa cuenta es la última nota de la
parte; `checked_sub(1)?` se rinde cuando no hay ninguna. Después de un
silencio de `LINE_BREAK`, una negra, la línea terminó, y la nota
siguiente empieza una nueva. `bool::then_some` convierte la condición
en un `Option`: `Some(note)` cuando se cumple y `None` cuando no.

### Nombrar el movimiento

```rust
pub fn motion(upper: &Move, lower: &Move, key: Scale) -> Motion {
    let (a, b) = (upper.semitones(), lower.semitones());
    match (a.signum(), b.signum()) {
        (0, 0) => Motion::Static,
        (0, _) | (_, 0) => Motion::Oblique,
        (x, y) if x != y => Motion::Contrary,
        _ => {
            let number = |upper: Note, lower: Note| {
                let (upper, lower) = (
                    key.spell(upper.pitch_class()),
                    key.spell(lower.pitch_class()),
                );
                lower.letter.steps_up_to(upper.letter)
            };
            if number(upper.from.pitch, lower.from.pitch) == number(upper.to.pitch, lower.to.pitch)
            {
                Motion::Parallel
            } else {
                Motion::Similar
            }
        }
    }
}
```

`signum` es −1, 0 o 1: baja, se queda o sube. Hacer `match` sobre el
par de signos resuelve primero los casos fáciles. Las dos se quedan.
Una se queda (el `|` en un patrón quiere decir «o»). Signos opuestos.
Solo cuando las dos van en el mismo sentido importa el intervalo.
Entonces el closure `number` escribe las dos notas según la tonalidad y
cuenta letras de la de abajo a la de arriba: Mi sobre Sol es
`G.steps_up_to(E)`, 5 letras hacia arriba, una sexta. Contar letras, no
semitonos, es lo que hace paralela una sexta mayor que va a una sexta
menor (ver [Los intervalos también cuentan
letras](pitch.md#los-intervalos-también-cuentan-letras)).

### Conservar la línea mientras la lees

La línea de encima de la grilla muestra el cambio del paso que está
debajo del puntero. Pero para leer su tooltip, el puntero tiene que
dejar las notas e ir a la línea, y entonces no hay ningún paso debajo.
Así que el piano roll recuerda el último paso al que se apuntó, y la
línea lo usa mientras el puntero está sobre la línea misma:

```rust
let line = Rect::from_min_size(
    ui.cursor().min,
    vec2(ui.available_width(), ui.text_style_height(&TextStyle::Body)),
);
let step = self
    .hovered_step
    .or(drawing.playhead)
    .or(self.last_step.filter(|_| ui.rect_contains_pointer(line)));
```

`ui.cursor()` es donde egui pondrá el próximo widget, así que `line` es
el rectángulo que la línea está por ocupar, calculado antes de
dibujarla. `Option::filter` conserva `last_step` solo cuando el closure
dice que sí: cuando el puntero está dentro de ese rectángulo. Mientras
suena una canción, el cursor de reproducción elige el paso cuando el
puntero está en otro lugar.

## Cómo se prueba

Aprendes armonía con esta revisión, así que una marca equivocada te
enseñaría algo equivocado. Dos clases de pruebas la protegen:

- **Ejemplos de manual.** Las pruebas de `voice_leading.rs` escriben
  pequeños ejemplos a cuatro partes, unos veinte, cada parte como una
  línea de nombres de notas. Cada prueba escribe una regla y sus
  excepciones como notas y comprueba qué se encuentra: una cadencia de
  manual sin nada que encontrar, quintas paralelas por una duodécima,
  una nota sostenida, un silencio que corta una línea, una línea
  doblada, la sensible en una parte extrema y en una interna, la
  cadencia rota, una dominante secundaria, ii⁶₅, V⁴₃ a I⁶, un retardo,
  un acorde de paso, una nota añadida fuera del pulso, la conducción
  cromática, el bajo en otra octava, un acorde de seis-cuatro, y una
  pista de acordes partida en partes.
- **Los corales de Bach.** La biblioteca music21, que usan los
  musicólogos, tiene su propia revisión de quintas y octavas paralelas.
  Las dos se corrieron sobre 350 corales de Bach, y encontraron las
  mismas 129 paralelas, en los mismos lugares y entre las mismas
  partes. Para la sensible y la séptima, cada uno de los hallazgos que
  quedan en esos corales es un apartamiento real de la escritura
  estricta, como un soprano que salta lejos de la sensible en una
  cadencia. Bach rompía las reglas de vez en cuando, a propósito y con
  buenos motivos.

```rust
#[test]
fn a_textbook_cadence_has_nothing_to_avoid() {
    // I IV V7 I in C major, as harmony books write it.
    let check = satb(["C5 C5 B4 C5", "G4 A4 G4 G4", "E4 F4 F4 E4", "C3 F3 G3 C3"]);
    assert_eq!(kinds(&check), vec![]);
    assert_eq!(
        check.summary(),
        "Voice leading, 4 parts: nothing classical part writing avoids."
    );
}
```

`voice_motion.rs` sigue un coral corto en una sola pista, Do mayor (Do3
Sol3 Mi4 Do5) a Fa mayor (Fa2 La3 Fa4 Do5) a G7 (Sol2 Si3 Fa4 Re5), y
comprueba la línea de movimientos en el primer cambio, que con letras
dice «S C5 held · A E4→F4 · T G3→A3 · B C3→F2; outer parts in oblique
motion»; los tamaños sostenida, grado conjunto, grado conjunto y salto;
las palabras («B: down a perfect fifth from C3 to F2, a leap»); la
contralto y el tenor en movimiento paralelo, sextas paralelas, y el
tenor y el bajo en movimiento contrario. En el segundo cambio, el
soprano y el bajo suben los dos por grado conjunto, a una quinta y dos
octavas de distancia: movimiento paralelo, y quintas paralelas. Otras
pruebas comprueban una melodía sobre una nota sostenida (oblicuo), y
que una quinta a una duodécima, o una sexta mayor a una sexta menor,
sigue siendo paralelo.

Los ejercicios sobre la conducción de voces están en [Ejercicios: la
teoría musical en código](exercises.md).
