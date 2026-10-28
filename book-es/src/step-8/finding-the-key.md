# Encontrar la tonalidad a partir de la música (experimental)

Archivos: [`crates/lineup-core/src/harmony/key_finding.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/key_finding.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs),
[`crates/lineup-listen/src/keys.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-listen/src/keys.rs),
[`crates/lineup-app/src/app/key_finder.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/key_finder.rs)
y [`crates/lineup-app/src/harmony_keys.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/harmony_keys.rs)

La pregunta que empezó todo: *«¿Comprobarán la importación MIDI (y la
importación de audio) los cambios de tonalidad en fuentes nuevas
desconocidas, o los acordes tocados por la entrada MIDI y el contexto de
los compases anteriores? Si no está claro, debería resaltar la mejor
coincidencia y las otras alternativas (menos resaltadas)».*

Hasta ahora, una tonalidad venía de algún sitio: el menú Key del
proyecto, un marcador de tonalidad, una armadura en un archivo MIDI, o
una deducción para una grabación entera. A las propias notas nunca se
les preguntaba. Este capítulo enseña a la app a escuchar la tonalidad,
compás a compás, y a decirlo cuando no está segura.

Es un experimento, así que está apagado hasta que actives **🔍 Find the
key** bajo las harmony keys.

## Cómo suena una tonalidad

Una tonalidad es una nota hogar y la escala a su alrededor. Puedes oírla
porque algunas notas vuelven mucho más a menudo que otras. En Do mayor,
Do, Mi y Sol (el acorde de tónica) son los que más suenan, luego las
demás teclas blancas; las teclas negras casi nunca. En La menor suenan
las mismas teclas blancas, pero La, Do y Mi van a la cabeza, y Sol♯
aparece siempre que la música hace una cadencia (la séptima elevada del
menor armónico).

Los psicólogos lo midieron. Carol Krumhansl y Edward Kessler (1982)
tocaban una escala, luego una sola nota, y preguntaban a los oyentes
cuán bien encajaba la nota. Las respuestas forman un *perfil de
tonalidad*: doce números, cuánto pertenece cada nota sobre la tónica.
Más tarde, otros contaron notas en partituras reales. Ferrous Lineup usa
los perfiles de Albrecht y Shanahan (2013), contados sobre 490 piezas
clásicas:

| nota sobre la tónica | 1 | ♭2 | 2 | ♭3 | 3 | 4 | ♯4 | 5 | ♭6 | 6 | ♭7 | 7 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| mayor | .24 | .01 | .11 | .01 | .14 | .09 | .02 | .21 | .01 | .08 | .01 | .08 |
| menor | .22 | .01 | .10 | .12 | .02 | .10 | .01 | .21 | .06 | .02 | .06 | .05 |

La tónica y la quinta destacan en ambas; la tercera distingue mayor (3)
de menor (♭3).

## El método de Krumhansl-Schmuckler

Para encontrar la tonalidad de un tramo de música:

1. Suma cuánto suena cada una de las doce notas, en cualquier octava.
   Esos doce números son el *chroma* de la música, empezando por Do.
2. Gira cada perfil para que empiece en cada una de las doce tónicas
   posibles: 24 tonalidades.
3. Compara el chroma con cada uno de los 24 perfiles usando su
   *correlación*: 1 cuando suben y bajan juntos, 0 cuando no tienen
   relación.
4. Gana la tonalidad con la correlación más alta.

En Rust, la comparación son dos líneas: resta la media para que ambas
listas se centren en 0, escálalas a longitud 1, y toma el producto
escalar.

```rust
fn normalized(values: &Chroma) -> Option<Chroma> {
    let mean = values.iter().sum::<f32>() / 12.0;
    let centered = values.map(|v| v - mean);
    let length = centered.iter().map(|v| v * v).sum::<f32>().sqrt();
    (length > 1e-9).then(|| centered.map(|v| v / length))
}
```

`Option` se ocupa del silencio: cuando no suena nada, los doce números
son iguales, la longitud es 0 y no hay tonalidad que encontrar, así que
la función devuelve `None` en lugar de dividir por cero.

## Segura o no segura

Las correlaciones por sí solas no dicen cuán segura es la respuesta. Do
mayor y La menor tienen las mismas notas, así que una melodía que se
queda en las teclas blancas puede encajar en ambas casi igual de bien.
`rank_keys` convierte las 24 correlaciones en *cuotas* que suman 1 (un
*softmax*): la cuota de una tonalidad es `e^(12 × (ajuste − mejor
ajuste))`, dividido por el total. Un ajuste mejor en 0,05 recibe una
cuota unas 1,8 veces mayor.

Una tonalidad es **clara** cuando su cuota es al menos el doble que la
segunda mejor. Si no, las siguientes tonalidades cuya cuota es al menos
el 40% de la mejor son sus **alternativas**, como máximo dos. Eso es lo
que la app muestra más tenue después de la mejor tonalidad. Los números
se eligieron midiendo (mira más abajo): en el informe actual, una
tonalidad mostrada como clara acierta en 13.900 de 16.153 compases,
alrededor del 86%.

![Find the key en Dorian Strut, una canción en Mi dórico: las notas encajan mejor en La menor, con Mi menor y La mayor cerca y más tenues. Cada tonalidad muestra qué es en ella el acorde que suena, A7.](../images/step-8-find-key-unclear.png)

Mi dórico usa las mismas notas que La menor y Sol mayor, así que los
perfiles no pueden elegir su nota hogar con confianza, y la app lo dice:
La menor, o Mi menor, o La mayor. Los modos son el caso más difícil para
este método, porque los perfiles solo conocen mayor y menor.

## Dónde cambia la tonalidad

Una canción entera no tiene un único chroma si modula. Así que la música
se corta en compases, y cada compás recibe una puntuación para cada
tonalidad. Un solo compás es poco en lo que basarse, sin embargo: un
compás que solo contiene D7 encaja mejor en Sol mayor que en Do mayor,
incluso en medio de una canción en Do mayor (D7 es el "V de V", una
*dominante secundaria*). Así que cada compás se puntúa con los compases
de alrededor: él mismo cuenta tres veces, sus vecinos dos, los que están
a dos compases una.

Luego, el mismo truco que para los acordes en `lineup-listen` (mira el
capítulo del spike): **cambiar de tonalidad cuesta una penalización**,
1,5 en unidades de correlación, y el *algoritmo de Viterbi* encuentra la
secuencia de tonalidades con la mejor puntuación total. Para cada compás
y cada tonalidad se queda solo con la mejor manera de llegar hasta ahí,
ya sea quedándose en la misma tonalidad o cambiando desde la mejor
tonalidad hasta el momento y pagando la penalización:

```rust
for k in 0..count {
    let (previous, total) = if best[k] >= leader_score - CHANGE_COST {
        (k, best[k])
    } else {
        (leader, leader_score - CHANGE_COST)
    };
    from[i][k] = previous;
    next[k] = total + here[k];
}
```

Al final, recorre `from` hacia atrás para leer el camino. Una tonalidad
nueva gana solo cuando encaja mejor el tiempo suficiente para pagar la
penalización, que es exactamente "un cambio solo cuando la tonalidad
nueva se sostiene". Un acorde prestado no es una modulación.

Puntuar con los compases vecinos difumina *dónde* ocurre un cambio en
uno o dos compases. Así que cada cambio se mueve después, dentro de tres
compases, al compás en el que las propias notas de los compases pasan de
encajar en la tonalidad vieja a encajar en la nueva. En la Habanera de
*Carmen* aterriza exactamente en el compás 10, donde el Re menor de
Bizet se vuelve Re mayor.

`Project::heard_keys` hace todo esto con las notas de una canción:
cuentan los sintetizadores y los canales MIDI Out, no la batería ni los
samplers (como una grabación importada), y cada nota cuenta mientras
suene en cada compás. Devuelve `KeySpan`s: un comienzo, un final y todas
las tonalidades ordenadas.

## Mientras tocas

Para los acordes tocados en un teclado MIDI o de escritura no hay
partitura en la que mirar hacia delante, solo los compases tocados hasta
ahora. `KeyListener` guarda un chroma del pasado que se desvanece: cada
compás, todo lo oído antes cuenta 0,84 veces menos, así que un compás de
hace cuatro compases cuenta la mitad. Una tonalidad nueva toma el mando
solo después de ir por delante durante dos compases seguidos.

Los compases anteriores a tu entrada también cuentan. Cuando empiezas a
tocar, el listener arranca creyéndose la tonalidad en la que está la
música a su alrededor (las notas en el playhead, o la tonalidad del
proyecto), como si hubiera oído un compás en esa tonalidad. Toca Do, Fa
y G7 sobre una canción en Mi dórico y primero los oye a la luz de La
menor; unos compases después se asienta en Do mayor, claro.

![Tocando C, F y G7 en el teclado de escritura sobre la canción dórica: tras unos pocos compases, la tonalidad oída es Do mayor, clara, y Do es I, la tónica.](../images/step-8-find-key-playing.png)

## Qué es un acorde en cada tonalidad

El mismo acorde hace un trabajo distinto en cada tonalidad a la que
podría pertenecer, y ese es el sentido de mostrar las alternativas. G7
es:

| tonalidad | numeral | qué hace |
|---|---|---|
| Do mayor | V7 | dominante: tira hacia Do |
| La menor | ♭VII7 = V7/♭III | la dominante de Do, el relativo mayor |
| Sol mayor | I7 = V7/IV | el acorde de tónica con la séptima bemol, tirando hacia Do, el IV |

`read_chord` resuelve esto con lo que la app ya sabe de las tonalidades:
el numeral romano (contado a lo largo de la escala mayor de la tónica,
como en todo Ferrous Lineup, así que Sol en La menor es ♭VII, no VII),
la función de los acordes de la tonalidad (tónica, subdominante,
dominante), una séptima dominante oída como la dominante del acorde al
que resuelve, y los acordes prestados de la tonalidad homónima (La♭ en
Do mayor es ♭VI, prestado de Do menor).

![Manteniendo G7 mientras la tonalidad oída es La menor: «G7 es bVII7 = V7/bIII», o Sol mayor, donde es I7 = V7/IV, o Do mayor, donde es V7, la dominante.](../images/step-8-find-key-g7.png)

## En una grabación

Una grabación no tiene notas, solo sonido. `lineup-listen` ya mide el
chroma de cada pulso para sus acordes; para las tonalidades, los pulsos
se juntan en compases y el chroma de cada compás representa sus notas.
Dos correcciones importaron:

- **Armónicos.** Una nota suena su propia altura y una serie de alturas
  más tenues: una octava más arriba, luego una octava y una quinta más
  arriba (el tercer armónico), y así. Así que una grabación de Do mayor
  está llena de Sol, Re y Si que nadie tocó, y la tonalidad salía a
  menudo una quinta demasiado aguda (Sol mayor por Do mayor, Re por
  Sol). Quitar a cada clase de alturas 0,3 de la nota una quinta por
  debajo anula gran parte de eso.
- **Solo los agudos.** El chroma del bajo es sobre todo armónicos de
  notas graves. En un informe anterior más pequeño, agudos y graves
  juntos pusieron el 66% de los pulsos en la tonalidad correcta; solo
  los agudos pusieron el 83%, y el 84% con la corrección de armónicos.
  El informe más grande de abajo usa solo los agudos y la misma
  corrección.

## Ponerlo en el playlist

En modo canción, la barra ofrece **Mark on the playlist**: los cambios
de tonalidad oídos en toda la canción se vuelven marcadores de
tonalidad (mira el capítulo sobre los cambios de tonalidad), el primero
de ellos la tonalidad con la que empieza la canción. Ninguna nota se
mueve, y Deshacer los retira. Es la manera de dar a una grabación o
archivo MIDI importados sin armaduras los cambios de tonalidad que de
verdad tiene.

![Una grabación importada de la Habanera tras Mark on the playlist: la regla dice Re menor en el compás 1 y Re mayor en el compás 10, y en el compás 11 las harmony keys son las de Re mayor.](../images/step-8-find-key-marked.png)

## Cómo de bien funciona

Medido el 26 de octubre de 2026 sobre las canciones de demo de la
biblioteca, cuyas tonalidades (y marcadores de tonalidad) vienen de sus
partituras. Los informes son pruebas que puedes ejecutar tú mismo:
`cargo test -p lineup-core --test key_finding -- --ignored --nocapture`
para las notas, y `cargo test --release -p lineup-listen --test
key_changes report -- --ignored --nocapture` para las demos renderizadas
a audio. (El filtro `report` se salta la prueba ignorada que escribe un
archivo WAV para probar la importación a mano.)

| | desde las notas | desde el audio |
|---|---|---|
| en la tonalidad correcta | 87,0% de 18.828 compases | 73,7% de 36.306 pulsos |
| cambios de tonalidad encontrados (a un compás / dos pulsos) | 132 de 315 | 63 de 315 |
| canciones de una tonalidad con un cambio que no existe | 18 de 259 | 3 de 52 |

Para la tonalidad principal de una grabación entera, acierta en 91 de
136 canciones, frente a 82 de 136 de `bpm-key`, que le da a la
importación su primera deducción.

Dónde falla, y por qué:

- **Visitas cortas.** La mayoría de los cambios que se le escapan duran
  de cuatro a ocho compases, a menudo hacia la dominante (el Fa♯ menor
  de la Badinerie, las frases en Fa♯ mayor del Mountain King). En tan
  pocos compases, el método no distingue un cambio de tonalidad de un
  pasaje que solo *toniciza* la dominante, y muchos músicos leerían esos
  compases como tonicizaciones también.
- **Tonalidades relativas.** Ballin' the Jack (Si♭ mayor) sale como Sol
  menor, y Sunday Seventh (Do mayor) como La menor: las mismas notas,
  otro hogar.
- **Modos.** Mi dórico, La dórico y Mi frigio salen como la tonalidad
  mayor o menor con las mismas notas, mostrados como poco claros, con la
  tónica real entre las alternativas.
- Algunos de los "cambios que no existen" son reales: la segunda mitad
  del Air de Bach pasa por Si menor. Las partituras usadas para las
  demos solo marcan los cambios de tonalidad más grandes.

## Ejercicios

1. Abre la Habanera, activa Find the key y haz clic en el compás 2 y
   luego en el 11 en la regla. ¿Dónde cambia exactamente la tonalidad,
   y por qué el estribillo en Re mayor no es solo «la misma melodía en
   otra tonalidad»?
2. En `key_finding.rs`, pon `CHANGE_COST` a 0.8 y ejecuta el informe de
   notas. ¿Cuántos cambios se encuentran, y cuántos de ellos no están
   en las partituras? ¿Qué te dice eso sobre la penalización?
3. Mantén La Do Mi en el teclado de escritura durante unos compases, y
   añade luego Sol♯ (la séptima elevada). ¿Se vuelve el listener más
   seguro de La menor? ¿Por qué importa tanto una sola nota?
