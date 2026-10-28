# Conducción de voces: comprobar las partes como lo haría un profesor de armonía

Archivos: [`crates/lineup-core/src/harmony/voice_leading.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/voice_leading.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs)
y [`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs)

El carril de acordes nombra cada acorde que escribes. Eso es la mitad de
la armonía. La otra mitad es la **conducción de voces**: cómo se mueve
cada parte desde su nota de un acorde hasta su nota del siguiente. Una
parte es una línea de notas, como un cantante, un instrumento o una
línea de los acordes de un piano. Dos piezas pueden usar los mismos
acordes y sonar muy distintas, suaves o torpes, porque sus partes se
mueven de forma distinta.

Los cursos de armonía clásica enseñan conducción de voces con escritura
a cuatro partes: soprano, alto, tenor y bajo, como en los corales de
Bach. Dan unas pocas reglas que mantienen cada parte como una línea
propia. Cuando escribes un ejercicio, un profesor marca dónde las
rompiste. El piano roll ahora puede hacer esas marcas. Marca
**Conducción de voces** junto a Ghost notes.

![Cuatro partes en Do mayor con la conducción de voces comprobada. La fila Voices bajo los nombres de acordes marca quintas paralelas, una séptima que sube, octavas paralelas y una sensible que baja. Las líneas magenta muestran los movimientos, y los contornos discontinuos muestran adónde deberían haber ido la séptima y la sensible.](../images/step-9-voice-leading.png)

## Qué ves

- **Una fila Voices** debajo de las dos filas de nombres de acordes.
  Cada hallazgo es un bloque sobre el acorde al que llegan las partes,
  nombrado con unas pocas letras: **5ths** y **8ves** para quintas y
  octavas paralelas, **1sns** para unísonos, **LT** para una sensible
  que no sube, **7th** para una séptima que no baja.
- **Líneas magenta sobre la cuadrícula**, desde la nota que una parte
  deja hasta la nota a la que va. Los movimientos del step al que
  apuntas se dibujan más gruesos.
- **Un contorno discontinuo** donde una sensible o una séptima debería
  haber ido.
- **Una línea bajo los nombres de acordes.** Resume la comprobación
  ("Voice leading, 4 parts: parallel fifths (1), …"). Cuando apuntas a
  un step con un hallazgo, dice en su lugar qué pasa allí. Pasa el
  cursor sobre la línea, o sobre un bloque de la fila Voices, para leer
  por qué la escritura clásica a partes lo evita.

![Apuntando al bloque 7th: la séptima Fa de G7 sube a Sol en lugar de bajar a Mi.](../images/step-9-voice-leading-tooltip.png)

## Qué notas forman una parte

La comprobación necesita partes, pero el piano roll tiene pistas. Una
pista que toca una nota a la vez es una parte. Una pista que toca
acordes, como un piano o la pista Right hand de la imagen, contiene
varias partes. La app la divide como leerías una partitura: la nota de
arriba de cada acorde va a la parte de arriba, la nota siguiente a la
parte siguiente, y cada nota se une a la parte cuya última nota estaba
más cerca. En la imagen, la pista Right hand contiene la soprano y el
alto. Los mensajes las llaman "Right hand (upper voice)" y "Right hand
(lower voice)".

Con **Ghost notes** activado, las demás pistas también se comprueban.
Las pistas de batería se dejan fuera.

Un silencio de un pulso o más termina una línea, así que las notas a
ambos lados de él no se comprueban entre sí. Un par de partes que toca
en octavas casi todo el tiempo, como los contrabajos doblando a los
violonchelos, es una línea hecha más fuerte, no dos partes en octavas
paralelas. La comprobación lo cuenta una vez.

## Las reglas

### Nada de quintas ni octavas paralelas

Toma dos partes separadas por una quinta justa. Si ambas se mueven en la
misma dirección hacia otra quinta justa, son **quintas paralelas**. Lo
mismo vale para octavas y unísonos. Estos intervalos se funden tan bien
que, por un momento, las dos partes suenan como una sola. La textura
pierde una línea, así que la escritura clásica a partes los evita. Una
quinta más una octava (una doceava) sigue contando como quinta.

Cuando las partes se mueven en direcciones opuestas, de una quinta a una
doceava, forman **quintas por movimiento contrario**. Cada parte se
mueve distinto, pero el oído sigue oyendo dos quintas seguidas, así que
la escritura estricta a partes también las evita. Las octavas por
movimiento contrario aparecen de vez en cuando entre la parte de arriba
y el bajo en una cadencia, y el mensaje lo dice.

En la imagen, el tenor va de Do4 a Re4 mientras el bajo va de Fa3 a Sol3.
Ambos suben un tono y se mantienen a una quinta de distancia: quintas
paralelas. Luego el alto va de Do5 a Sol4 y el bajo de Do3 a Sol2, ambos
bajando una cuarta y manteniéndose a dos octavas de distancia: octavas
paralelas.

Una nota mantenida o repetida no cuenta. Las dos partes tienen que
moverse.

### La sensible sube

La **sensible** es la nota un semitono por debajo de la tónica: Si en Do
mayor, Sol♯ en La menor. Cuando un acorde dominante (V, V7, vii°, vii°7)
va a la tónica, la sensible tira hacia arriba, hacia la tónica.

La regla es estricta en la parte de arriba y en el bajo, donde el oído
sigue la línea. Una parte interna es más libre. En V7 a I, una sensible
interna a menudo baja a la quinta del acorde de tónica, para que ese
acorde tenga las tres notas. Los libros de texto la llaman sensible
"frustrada", y la comprobación deja en paz a las partes internas.

La comprobación conoce tres sitios donde la sensible debe subir:

- **V o vii° a I**, la cadencia.
- **V a vi** (V a VI en menor), la **cadencia rota**. La sensible sigue
  subiendo, a la tónica, que ahora es la tercera de vi.
- **Una dominante secundaria** que va a su acorde: D7 a G en Do mayor. D7
  toma prestado Fa♯ de fuera de la tonalidad, y Fa♯ es la sensible de Sol,
  así que sube a Sol. C7 yendo a F en Do mayor también es una dominante
  secundaria (V7/IV). Pero su sensible Mi pertenece a la tonalidad, y
  conserva la libertad que tiene en Do.

Si los dos acordes tienen la sensible, puede quedarse donde está. Una
sensible que baja por una escala (Do, Si, La) es una nota de paso, no una
que tenga que subir. Y V seguido de I⁶₄ no es una llegada, porque las
partes siguen adelante antes de que la sensible tenga adónde ir.

Cuando dos partes tienen la sensible a la vez, el mensaje también dice
que está doblada. La escritura clásica a partes nunca dobla la sensible:
las dos copias no pueden subir sin formar octavas paralelas.

### Una séptima baja

En un acorde de séptima como G7 (Sol Si Re Fa), la séptima, Fa, es una
disonancia. **Resuelve bajando por grado conjunto** al siguiente acorde.
En G7 a C, Fa baja a Mi. En la imagen, el Fa de la soprano sube a Sol en su
lugar, y el contorno discontinuo marca Mi.

La comprobación lee unos pocos acordes como lo hacen los libros de
armonía. Fa La Do Re yendo a Sol se llama F6 por sus notas, pero funciona
como Dm7 con Fa en el bajo, ii⁶₅ en Do. Su séptima es Do, que debería
bajar a Si. La fundamental de un acorde de séptima disminuida sale de
adónde va: Sol♯ Si Re Fa yendo a La tiene su fundamental en Sol♯, y su séptima
es Fa.

Las excepciones de los libros de texto no se marcan:

- **El siguiente acorde tiene la nota.** Entonces la séptima puede
  quedarse donde está.
- **Una suspensión.** La séptima se mantiene dentro del siguiente acorde
  y baja un poco después.
- **Notas de paso y notas de adorno.** Una nota corta que viene y se va
  por grado conjunto forma una séptima solo de paso. Igual que una nota
  añadida fuera del pulso mientras las otras partes mantienen las suyas.
- **V⁴₃ a I⁶.** La séptima (Fa en G7 con Re en el bajo) puede subir a Sol
  mientras el bajo sube de Re a Mi una décima más abajo.
- **Un acorde de paso.** En C, luego Em7 sobre Re, luego C sobre Mi, el Re
  del bajo es una nota de paso entre dos formas del mismo acorde de Do.
- **Conducción de voces cromática.** Una séptima puede subir un semitono
  hacia la propia sensible del siguiente acorde. En Am7 con Sol en el bajo
  yendo a E7 con Sol♯ en el bajo, Sol sube a Sol♯. La línea sigue subiendo, y
  el siguiente acorde la resuelve.
- **El bajo en otra octava.** El bajo puede tomar su nota a una octava
  de distancia: Si2 a Do2 es la sensible yendo a la tónica.

## Lo que la comprobación no hace

Estas son las reglas de un estilo: la escritura a partes de Bach a
Brahms, y la de los ejercicios de armonía. El pop y el jazz mueven
acordes enteros en paralelo a propósito, y Debussy también. Nadie espera
que una séptima de jazz resuelva. Lee la comprobación como un profesor
del estilo clásico, no como un juez de toda la música. Donde la
tonalidad es una serie dodecafónica, la comprobación sigue en su lugar
las reglas de la escritura dodecafónica: ver [Reglas
dodecafónicas](twelve-tone-rules.md).

Algunas reglas de los libros de armonía todavía no se comprueban:
quintas y octavas directas (ocultas) en las partes extremas, la
separación entre las partes, las partes que se cruzan y qué nota de un
acorde doblar.

## Cómo sabemos que está bien

Aprendes armonía con esta comprobación, así que una marca equivocada te
enseñaría algo equivocado. Dos clases de pruebas la protegen:

- **Ejemplos de libro de texto.** Las pruebas de `voice_leading.rs`
  escriben pequeños ejemplos a cuatro partes, unos veinte. Cada prueba
  escribe la regla y sus excepciones como notas y comprueba qué se
  encuentra: una cadencia de libro de texto sin nada que encontrar,
  quintas paralelas a una doceava, una nota mantenida, una línea
  doblada, la sensible en una parte externa y en una interna, la cadencia
  rota, una dominante secundaria, ii⁶₅, V⁴₃ a I⁶, una suspensión, un
  acorde de paso, una nota añadida fuera del pulso, conducción de voces
  cromática, el bajo en otra octava y un acorde de sexta y cuarta.
- **Los corales de Bach.** La biblioteca music21, que usan los estudiosos
  de la música, tiene su propia comprobación de quintas y octavas
  paralelas. Ejecutamos las dos sobre 350 corales de Bach. Encontraron
  los mismos 129 paralelismos, en los mismos sitios entre las mismas
  partes. Para la sensible y la séptima, cada uno de los 25 hallazgos
  que quedan en esos corales es una desviación real de la escritura
  estricta a partes, como una soprano que salta lejos de la sensible en
  una cadencia. Bach rompía las reglas de vez en cuando, a propósito y
  con buen motivo.

## En el código

### Repartir las notas de un acorde: programación dinámica

`voices_of` convierte las notas de una pista en partes. Recorre los
acordes en el tiempo. Para cada acorde, tiene que decidir a qué parte va
cada nota. Las notas y las partes están ambas ordenadas de la más aguda
a la más grave, y una nota solo puede ir a una parte por debajo de la
que tomó la nota que está encima, así que las partes nunca se cruzan.
Entre todas las formas de hacerlo, elige la de menor distancia total en
semitonos.

Probar todas las formas tardaría demasiado para un acorde grande. La
**programación dinámica** lo resuelve con una tabla. `cost[i][j]` es el
costo mínimo de colocar las primeras `i` notas usando solo las primeras
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

Cada celda tiene tres opciones. Saltarse la parte `j`, que así no recibe
nota en este acorde. Empezar una parte nueva para la nota `i`, con el
costo grande `NEW_PART`, de modo que solo ocurre cuando un acorde tiene
más notas que partes. O poner la nota `i` en la parte `j`, al costo de
la distancia entre ambas. Cada celda se construye a partir de celdas ya
rellenadas, así que la tabla entera cuesta `notes × parts` pasos.
Recorrer hacia atrás desde la última celda recupera qué opción produjo
cada costo, y por tanto adónde fue cada nota.

### Comparar dos partes nota contra nota

`together` lista cada momento en que cualquiera de las dos partes
empieza una nota, con la nota en la que está cada parte en ese momento:

```rust
onsets
    .into_iter()
    .map(|tick| a.current(tick).zip(b.current(tick)))
    .collect()
```

`Option::zip` convierte dos `Option` en un `Option` de un par. Es `Some`
solo cuando ambas son `Some`, es decir, cuando las dos partes están
tocando. Luego `consecutive` mira cada par de momentos vecinos, y
`kind_of_move` decide qué produce el movimiento:

```rust
let (up_x, up_y) = (moves(x)?, moves(y)?);
let first = perfect(x.0.pitch.midi().abs_diff(y.0.pitch.midi()))?;
let second = perfect(x.1.pitch.midi().abs_diff(y.1.pitch.midi()))?;
```

Cada `?` devuelve `None` de la función en cuanto falta algo. `moves` da
`None` para una nota mantenida o un silencio de un pulso o más, y
`perfect` da `None` para un intervalo que no sea unísono, quinta u
octava. Lo que queda tras los `?` es el caso interesante, dos intervalos
justos seguidos, y un `match` sobre el par le pone nombre.

### Nombrar una parte con un closure

Un hallazgo conoce sus partes por número. Solo la app conoce los nombres
de las pistas, así que `describe` recibe una función que nombra una
parte:

```rust
pub fn describe(&self, voice_name: impl Fn(usize) -> String) -> String
```

El piano roll le pasa un closure:

```rust
let name = |voice: usize| voice_name(drawing.project, check, voice);
let why: Vec<String> = here.iter().map(|slip| slip.describe(name)).collect();
```

`name` se pasa por valor, una vez por cada hallazgo, y aun así puede
seguir usándose después. Funciona porque el closure solo guarda
referencias compartidas (`&Project`, `&VoiceLeading`). Un closure cuyas
capturas son todas `Copy` es él mismo `Copy`, así que cada llamada
recibe su propia copia. (Clippy lo dice también: marca `describe(&name)`
como un préstamo innecesario.)

### Comprobar solo cuando algo cambió

Comprobar un patrón tarda menos de un milisegundo en una build release,
pero el piano roll dibuja 60 fotogramas por segundo mientras suena una
canción. Así que guarda la última comprobación junto con aquello de lo
que se hizo:

```rust
#[derive(Debug, Clone, PartialEq)]
struct VoiceInput {
    pattern: usize,
    notes: Vec<(usize, Vec<NoteEvent>)>,
    time: TimeSignature,
    keys: Vec<Scale>,
}
```

Cada fotograma construye un `VoiceInput` nuevo y lo compara con el
guardado. `#[derive(PartialEq)]` compara campo por campo, hasta la
última nota. Comparar es mucho más barato que comprobar, y cuando nada
ha cambiado se usa la comprobación vieja:

```rust
if let Some((made_from, check)) = &self.voice_check
    && *made_from == input
{
    return Rc::clone(check);
}
```

`if let … && …` es una **let chain**, nueva en la edición 2024. Prueba
un patrón y una condición en un solo `if`.

La comprobación se guarda en un `Rc<VoiceLeading>`, un puntero
compartido. Dibujar necesita la comprobación mientras también cambia el
piano roll (`&mut self`). Tomar prestada la comprobación de `self`
bloquearía `self` durante todo el fotograma, así que el borrow checker
lo rechazaría. `Rc::clone` solo suma uno a un contador, así que el
fotograma obtiene un handle propio por el costo de un incremento de un
entero, sin copiar los hallazgos.

## Ejercicios

1. Escribe I–IV–V–I en Do mayor con el bajo Do3 Fa3 Sol3 Do3 y la soprano
   Mi5 Fa5 Re5 Mi5. Marca Conducción de voces. Luego mueve la Re5 de la
   soprano a Sol5 y mira qué dice la comprobación. ¿Por qué se evitan las
   octavas entre la soprano y el bajo, pero una tercera o una sexta
   entre ellos es bienvenida?
2. Escribe G7 a C con la séptima en el alto bajando a Mi. Luego escribe
   G7 a Am (la cadencia rota). ¿Adónde tiene que ir ahora la sensible?
   ¿Y adónde va la séptima?
3. La comprobación deja en paz a la sensible en las partes internas.
   Escribe V7 a I con el tenor yendo de Si a Sol. ¿Qué gana el acorde de Do,
   y qué le faltaría si el tenor subiera a Do en su lugar?
4. En `kind_of_move`, un unísono que va a una octava cae hasta los dos
   últimos brazos del `match`. Averigua qué clase recibe cuando ambas
   partes suben, y luego escribe una prueba en `voice_leading.rs` que lo
   compruebe, usando el helper `satb` que usan las demás pruebas.
