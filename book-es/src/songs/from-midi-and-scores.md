# De una interpretación MIDI o una partitura a una demostración

Archivos: [`crates/lineup-core/src/library/song/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates/lineup-core/src/library/song),
[`crates/lineup-core/src/midi_file/import.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/midi_file/import.rs)
y [`crates/lineup-core/tests/key_finding.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/tests/key_finding.rs)

Una demostración es una constante de texto (ver [Cómo se escribe una
demostración](demos.md)). Las melodías cortas se escriben a
mano. Las piezas más largas se leen de una versión que una computadora
puede leer, un archivo MIDI, un grabado o una partitura guía, con
pequeños scripts escritos para cada tipo de fuente. Los scripts no son
parte de la aplicación: lo que escriben sí, como constantes de Rust
corrientes, y el comentario `// Source:` encima de cada canción dice de
dónde se leyó y qué se dejó afuera. Este capítulo trata de los métodos:
cómo se leen de las notas un pulso, una melodía, una tonalidad y unos
acordes, cómo reciben las notas su nombre, y qué conserva una
demostración de una partitura. Casi todo es teoría musical puesta a
trabajar. Qué escuchar en las piezas está en la [guía de
escucha](https://naelolaiz.github.io/ferrous-lineup/es/manual/finding-a-demo.html)
del manual.

## Tipos de fuente

| Fuente | Qué contiene | Para |
|---|---|---|
| **ABC** (las colecciones de John Chambers y Henrik Norbeck, The Traditional Tune Archive) | una melodía escrita con letras, a menudo con nombres de acordes | canciones y danzas folk, los Balcanes |
| **Partituras guía en MusicXML** (OpenEWLD, la parte de dominio público del archivo Wikifonia) | una melodía con los nombres de los acordes encima | canciones de casi todos los géneros: estándares de swing, blues, latinas, soul, pop |
| **Grabados de LilyPond** (el Mutopia Project, OpenScore) | la partitura entera, cada voz, como texto | piezas clásicas |
| **Secuencias MIDI de una partitura** | cada voz, tocada con clic | la Misa Criolla, La consagración de la primavera, Petrushka, arreglos de tango y de folclore |
| **Interpretaciones MIDI** | un pianista tocando libre, sin clic | tangos tocados al piano |
| **Una transcripción en grados de la escala** (TheoryTab de Hooktheory) | la melodía y los acordes como números en una tonalidad | el tema de The Big Bang Theory |
| **SymbTr** | música turca de makam, nota a nota | las canciones turcas |
| **Imágenes de las páginas** de una partitura impresa | lo que lee el ojo, compás por compás | algunos tangos y canciones |

Los formatos de texto ya se parecen a la notación de la aplicación, así
que sus scripts sobre todo traducen. Los tresillos de una partitura
guía pasan a las semicorcheas más cercanas, como los redondea la propia
[partitura guía](../band/lead-sheet.md) de la aplicación, y donde un
acorde de una partitura guía choca con su propia melodía se usa el
acorde que suelen tocar los músicos de jazz ahí, con una nota en el
comentario. Donde una fuente no trae acordes, el comentario dice que
los acordes son una armonización sencilla. Muchas danzas de los
Balcanes se escriben en semicorcheas (7/16, 11/16); aquí se escriben en
corcheas, cada nota el doble de larga, lo que suena igual al tempo
indicado.

## Leer una partitura

### Un grabado, a través de MIDI

**LilyPond** es un lenguaje para imprimir música a partir de texto.
Estas son las primeras notas de los primeros violines de «Porque un
niño nos ha nacido» de Handel:

```text
re'''4 sol''8 sol'' do''' do''' do''' do''' |
```

Los nombres de las notas son los italianos, los mismos que en español
(`re`, `sol`, `do`), cada `'` sube la nota una octava, y el número es
la duración (`4` una negra, `8` una corchea). LilyPond escribe una
partitura como archivo MIDI además de como página, y el script lee ese
archivo compás por compás:

- **Dinámicas.** LilyPond toca cada indicación de dinámica a una
  velocidad fija (*p* 69, *mf* 86, *f* 95), así que esas velocidades se
  leen de vuelta como las indicaciones, `!p`, `!mf`, `!f`.
- **Staccato.** LilyPond toca una nota en staccato la mitad de larga de
  lo escrito. Para distinguir una negra en staccato de una corchea, la
  partitura se compila dos veces, una sin sus puntos de staccato: una
  nota que es más corta en el primer archivo está en staccato, y lleva
  un `.` en la demostración.
- **Fusas.** Los trinos escritos de Bach son más rápidos que un paso.
  La notación puede escribir medio paso, `D5:1/2`.

Algunas partituras dejan cosas afuera, y el comentario dice qué se
agregó: la edición del Mesías imprime sus fortes solo en los primeros
violines, así que un forte vale para todas las cuerdas. Las ediciones
también tienen errores. El grabado de Die Forelle escribe re natural en
los compases 43 y 44, contra un re bemol en la mano izquierda. Los
compases 19 y 20 tienen la misma música con re bemol, y también una
segunda edición, la de OpenScore: la demostración toca re bemol.

### Una anacrusa de su propia duración

Alfonsina y el mar empieza una corchea antes de su primer compás
completo. En lugar de un compás de 6/8 que es casi todo silencio, como
escribe su anacrusa «Clementine», la anacrusa puede ser un compás de
su propia duración, porque una parte puede tener su propio compás:

```rust
const ONE_EIGHT: TimeSignature = TimeSignature::new(1, 8);
```

```rust
Part::new("Pickup", "!mf F#4:2", "")
    .in_time(ONE_EIGHT)
```

`TimeSignature::new` es una `const fn`, así que una indicación de compás
también puede ser una constante.

### Un coro son tres canales, no uno

Dos voces de un coro a menudo cantan la misma nota. Supón que las
contraltos empiezan un mi en semicorcheas mientras los tenores
sostienen un mi durante un pulso entero: en un mismo canal son dos
notas con la misma altura en el mismo paso, y un patrón nunca guarda
dos notas así (ver [Nunca dos notas en el mismo
lugar](../program/project.md#nunca-dos-notas-en-el-mismo-lugar)):
gana la primera, y el mi sostenido de los tenores se perdería. Por eso
el coro tiene sus contraltos, tenores y bajos en tres líneas, tres
canales, como están en tres pentagramas de la partitura. El coro de la
Misa Criolla es una sola línea, «Choir» (Coro), con sus voces unidas
por `&`: donde dos de ellas coinciden en una nota, esa nota suena una
vez.

### Bajo cifrado: acordes escritos como números

Los compositores barrocos no escribían los acordes del teclado. Debajo
de la línea del bajo escribían números pequeños, las **cifras**, cada
una un intervalo sobre la nota del bajo, y quien tocaba completaba los
acordes. La edición de Chrysander del Mesías imprime cifras sobre parte
del bajo, y el cifrado de la demostración de Handel las sigue donde
aparecen:

| Cifra | Significa | En este coro |
|---|---|---|
| ninguna | una tríada en estado fundamental | casi todos los compases |
| 6 | una tríada en primera inversión: la tercera en el bajo | compás 1: si con 6 es `G/B` |
| 6/5 | un acorde de séptima en primera inversión | compás 1: fa♯ con 6/5 es `D7/F#` |
| 4/2 | un acorde de séptima en tercera inversión: la séptima en el bajo | compás 28: sol con 4/2 es `A7/G`; compás 43: do con 4/2 es `D7/C` |
| 6 con una barra | primera inversión, con la sexta alzada un semitono | compás 26: mi con un 6 alzado es `C#dim/E` |
| + solo | la tercera alzada | compás 22: la con do♯, `A` |
| un becuadro | la tercera natural | compás 58: re con fa natural, `Dm` |

Una séptima quiere bajar un grado, así que un acorde de 4/2, con la
séptima en el bajo, va casi siempre seguido de un 6 sobre la nota de
abajo: `A7/G` va a `D/F#` en el compás 29, `D7/C` a `G/B` en el 44.
Cómo escribe las inversiones la hoja de acordes está en [Acordes con
barra e
inversiones](https://naelolaiz.github.io/ferrous-lineup/es/manual/chords.html#acordes-con-barra-e-inversiones)
del manual.

## Leer una secuencia

Una **secuencia** es un archivo MIDI hecho en un secuenciador, cada voz
en su pista, con clic. La Misa Criolla, La consagración de la primavera
y Petrushka salen de secuencias de sus partituras. Las barras de compás
están en el archivo; lo que tiene que hacer el script es volver a leer
la interpretación como partitura:

- cada nota va a la semicorchea más cercana, o al tercio de una más
  cercano donde la secuencia toca un tresillo, que entonces cae justo
  en el tick (ver [Notas entre
  pasos](../sound/midi.md#notas-entre-pasos));
- el mapa de tempo se lee como las indicaciones de la partitura: una
  caída profunda que vuelve es un **calderón**, una serie de cambios
  pequeños un **deslizamiento** (*rit.* o *accel.*), y cualquier otra
  cosa un tempo nuevo;
- las velocidades se leen como dinámicas, de *pp* a *ff*;
- un **trémolo**, el mismo acorde tocado más rápido que una
  semicorchea, se escribe como un solo acorde sostenido.

Una partitura de orquesta tiene más líneas que canales una banda. El
script mantiene la melodía delante y la sigue de instrumento en
instrumento con `played_by`, da canales propios a las líneas que
importan, y escribe el resto en el canal Teclados. Las líneas dobladas
al unísono o a la octava se escriben una vez. Las notas de adorno, los
trinos y los glissandos se simplifican, y el comentario dice cómo.

## Leer una interpretación

Un pianista que toca libre, sin clic, deja un archivo MIDI sin barras de
compás: la cuadrícula del archivo no significa nada. Los tangos de
`tango_played.rs` salen de archivos así. Todo lo que diría una
partitura hay que oírlo.

### Encontrar el pulso

El pulso se encuentra como lo encuentra quien escucha, con el método de
«Beat tracking by dynamic programming» de Dan Ellis (2007), el mismo
que la aplicación usa con una grabación (ver [Pulsos: arranques,
autocorrelación y programación
dinámica](../theory/listen.md#pulsos-arranques-autocorrelación-y-programación-dinámica)):
cuánto sonido nuevo empieza en cada momento, el período más estable en
eso, y luego la cadena de pulsos que cae en más arranques sin alejarse
de ese período. Cada compás recibe entonces el tempo al que se tocó, y
se escribe una marca de tempo donde el tempo se mueve medio pulso por
minuto o más. Uno tiene 61.

### Un pulso estable fuera de la cuadrícula

Algunos archivos se secuencian con un clic estable pero se guardan a
otro tempo, así que los pulsos del clic duran una fracción de tick y se
corren a través de los compases del propio archivo. Las notas de
Balderrama no caen cerca de la cuadrícula del archivo, pero su bombo
golpea cada 293 o 294 ticks del primer compás al último: un pulso
estable. Para encontrarlo exactamente, numera los golpes por el pulso
en que caen, `k = 0, 1, 2, ...`, y traza una recta por ellos, tick
contra `k`. La recta de **mínimos cuadrados** es la que tiene la menor
suma de errores al cuadrado, y su pendiente, la duración del pulso,
tiene una fórmula:

```text
period = Σ (k − k̄)(t − t̄) / Σ (k − k̄)²
```

donde `k̄` y `t̄` son los promedios. En Balderrama la corchea dura
293,966 ticks, y los golpes se apartan de la recta unos pocos ticks,
milésimas de segundo: una máquina, no una persona.

La recta da el pulso pero no el compás: ¿cuál de las seis corcheas es
la primera? La armonía cambia en el primer tiempo, y de las notas largas
del coro la mayoría empieza en una misma corchea de las seis. Esa
corchea es la primera del compás, y resulta que la melodía empieza dos
corcheas antes, con una anacrusa, como suele hacer una zamba.

### La melodía y la tonalidad

La **melodía** es la línea de arriba: de las notas que suenan en cada
momento en la pista de la melodía, la más aguda (el **perfil**, como
lee la melodía la [partitura guía](../band/lead-sheet.md) también).
Las notas debajo de ella que la doblan, como una segunda voz en
terceras, van en una línea propia.

La **tonalidad** sale de cuánto suena cada una de las doce clases de
altura, comparado con perfiles de tonalidad, como lo hace el propio
detector de tonalidad de la aplicación (ver [Comparar: el método de
Krumhansl-Schmuckler](../theory/hearing.md#comparar-el-método-de-krumhansl-schmuckler)).
Donde elige mal, la tonalidad se pone a mano, y una parte que cambia de
tonalidad recibe su propio `in_key`. El test de que al menos cuatro
notas de cada cinco estén en la tonalidad de su parte
(`melodies_are_mostly_in_their_key`) atrapa también una elección
equivocada de menor: la melodía de Balderrama canta re natural mucho
más que re sostenido, así que en mi menor armónica solo 228 de sus 288
notas están en la tonalidad y el test falla, mientras que en mi menor
natural están 257.

### Los acordes

Para cada compás, medio compás o pulso, el script puntúa cada acorde
común sobre las doce fundamentales: las notas dentro del acorde suman a
su favor, las de afuera en contra, y una nota del bajo en la
fundamental del acorde suma de más. Los acordes de la tonalidad reciben
un pequeño premio, así una nota de paso no convierte un la menor en algo
exótico, y un compás se divide solo donde la división puntúa claramente
mejor. Es la misma idea que la [escucha de
acordes](../theory/hearing.md) de la propia aplicación, y tiene la
misma debilidad: las notas solas nombran cada color como un acorde
propio. Una sexta que agrega el pianista convierte Bm en Bm6, y una nota
de la melodía de paso le agrega una novena bemol a F♯7.

Una **partitura guía** de la canción arregla eso. Sus compases no
coinciden con una interpretación libre (el pianista agrega una
introducción, repite una sección, estira un compás), así que sus
acordes no se ponen sobre los compases uno por uno. En cambio, la
partitura le da a la lectura su **vocabulario**: solo se pueden elegir
los acordes que usa la partitura. Una partitura en otra tonalidad
primero se **transporta**, cada acorde movido el mismo intervalo, lo
que mantiene la función de cada acorde en la tonalidad: la partitura de
Uno está en sol y el pianista toca en do, así que G pasa a C, D7 a G7 y
Em a Am.

Los compases 38 a 40 de Yira, yira, leídos solo de las notas, son
`Bm6 F#7b9 | Bm F#7 | Bm A7`. Limitados a los acordes de la partitura
son

```text
Bm F#7 | Bm F#7 | Bm A7
```

La mano izquierda colorea el si menor con su sexta, sol♯, y la melodía
toca sol, una novena bemol sobre F♯7. Debajo de los colores están los
acordes sobre los que se construye la canción, i y V7 de si menor.

Sin partitura guía, los acordes se comprueban de oído y por lo que
hacen. Tiempos viejos termina en re, fa, la y si. Esas cuatro notas son
también si, re, fa, la, **Bm7♭5**, pero el bajo toca re, y el acorde
viene después de A7, la dominante de re menor: es **Dm6**, re menor con
sexta agregada. El nombre de un acorde sigue a su bajo y a su función,
no solo a sus notas.

### El nombre de las notas

Un archivo MIDI tiene números, no nombres. La nota 70 es la sostenido o
si bemol, y el archivo no dice cuál. La tonalidad nombra las siete notas
de su escala; las otras cinco siguen al acorde que las rodea:

- **Una nota del acorde se escribe como la tiene el acorde.** En F♯7
  (fa♯, la♯, do♯, mi) la nota 70 es la sostenido, la tercera del
  acorde; en Gm (sol, si♭, re) es si bemol.
- **Un acorde sobre una nota de fuera de la tonalidad se nombra por
  adónde va.** Una séptima de dominante baja una quinta, así que el
  acorde antes de C♯m es G♯7. Un acorde de séptima que baja un semitono
  es un **sustituto tritonal**: el acorde antes de G es A♭7, no G♯7. Una
  séptima disminuida sube un semitono: G♯dim7 va a A.
- **Una nota de fuera del acorde se escribe por adónde va.** Una nota
  de paso cromática entre dos notas a un tono de distancia es sostenido
  al subir y bemol al bajar: la, la♯, si y sol, sol♭, fa.
- **Nada de mi sostenido, si sostenido, do bemol ni fa bemol para una
  nota de paso**: en el papel parecen errores por fa, do, si y mi. Las
  notas propias de la tonalidad sí pueden serlo: en si menor la cuarta
  alzada que lleva a la dominante, fa sostenido, es mi sostenido.

Cómo nombra la propia aplicación las notas de fuera de la tonalidad,
sin mirar ningún acorde, está en [Notas de fuera de la
tonalidad](../theory/scales.md#notas-de-fuera-de-la-tonalidad); cómo
las nombra la partitura guía, que mira el acorde como estas reglas, en
[Qué nombre de
nota](../band/lead-sheet.md#qué-nombre-de-nota-spellingrs).

## Una transcripción en grados de la escala

El tema de The Big Bang Theory sale de una **transcripción** del sitio
TheoryTab de Hooktheory, donde las canciones se escriben como las lee
una clase de teoría: cada nota un **grado** de la escala en la
tonalidad (1 es la tónica, 5 la quinta), cada acorde un número romano.
El conjunto de datos Sheet Sage (Chris Donahue y otros, 2022) reúne
unas 26.000 en un archivo. Una nota ahí se ve así:

```json
{"sd": "6", "octave": 0, "beat": 1, "duration": 1}
```

Grado 6, en la octava que empieza en la tónica, en el pulso 1, de un
pulso de duración. El tema está en sol mayor, y con la tónica en G4 la
melodía va de D4 a G5, el registro en que la escribe una partitura
guía:

| Escrito ahí | En sol | Aquí |
|---|---|---|
| sd 6, octava 0, 1 pulso | mi, encima de la tónica | `E5:4` |
| sd 5, octava 0, ½ pulso | re | `D5:2` |
| sd 1, octava 0, ¼ de pulso | sol, la tónica | `G4:1` |
| sd 5, octava −1, ½ pulso | re, debajo de la tónica | `D4:2` |

La transcripción también está **alineada** con la grabación: el pulso 1
de la estrofa está a los 7,6 segundos y su pulso 33 a los 19,66
segundos. 32 pulsos en 12,06 segundos son 32 × 60 / 12,06 = 159,2
pulsos por minuto, así que la estrofa lleva
`.with_tempo(&[tempo(0, 159)])` después de una introducción a 130.

## Lo que una demostración tiene que conservar

Dos tests sobre todas las canciones deciden cómo se corta una pieza en
partes y cómo se escribe su tempo.

**Una exportación MIDI tiene que volver nota por nota.**
`every_demo_comes_back_note_for_note` exporta cada demostración como
archivo MIDI, lo importa, lo vuelve a exportar y compara los dos
archivos. El importador corta una canción larga en patrones en barras
de compás que ninguna nota cruza (ver [Cortar la canción en
patrones](../sound/midi.md#cortar-la-canción-en-patrones)); donde una
nota sostenida cruza cada barra de compás durante muchos compases, tiene
que cortar una, y la nota cortada se vuelve a tocar. Una demostración
vuelve a tocar las notas sostenidas en los límites de sus propias partes
de todos modos, así que un límite en la barra de compás correcta da un
corte limpio: el Credo de la Misa Criolla, cuyo bajo se liga sobre casi
todas las barras de compás, tiene uno en el compás 30, donde entran los
solistas.

**Una demostración mantiene un tempo razonable.**
`every_demo_keeps_a_sensible_tempo` mantiene cada marca de tempo entre
tres décimos y dos veces y media el tempo de la canción. El último
acorde de una secuencia a menudo se sostiene frenando el clic casi del
todo: la secuencia del Credo sostiene su último acorde a 20 pulsos por
minuto, y la demostración a 46.

Otro test lee las canciones de una manera que pide cuidado. El test del
detector de tonalidad mide cuántos compases de las demostraciones oye
en su tonalidad escrita, y una pieza en dos tonalidades a la vez haría
que esa cifra no significara nada: diga la tonalidad que diga, la mitad
de la música no está de acuerdo. Esas canciones van en una lista
propia:

```rust
const POLYTONAL: &[&str] = &["The Rite of Spring, The Augurs of Spring"];
```

El test principal las deja afuera, y un segundo test comprueba que la
tonalidad escrita esté entre las que se oyen, pero que no sea la única:

```rust
assert!(
    result.right_bars < result.bars,
    "{name} is heard in one key: drop it from POLYTONAL"
);
```

El segundo assert mantiene honesta la lista: si el detector llegara a
oír los Augurios en una sola tonalidad, la canción dejaría de
pertenecer a ella.

## Comparar una demostración con su fuente

El último paso es tocar la demostración con la [traza de
reproducción](../sound/trace.md):

```text
ferrous-lineup --trace "Balderrama" --out balderrama.tsv
```

y buscar cada nota de la fuente en la traza: para una partitura en el
mismo paso y la misma altura, en el canal correcto y con su duración;
para una interpretación en la misma altura con un margen de un quinto
de segundo. Lo que falta tiene que ser lo que una demostración deja
afuera a propósito:

- los unísonos en un mismo canal, que suenan una vez (dos manos en el
  canal Teclados, dos guitarras que tocan la misma nota, las voces de un
  coro en una sola línea);
- los trémolos y los rasgueos más rápidos que una semicorchea, escritos
  como acordes sostenidos;
- los golpes de batería que caen en el mismo paso de un carril;
- las notas que nombra el comentario, como un error de una edición
  corregido.
