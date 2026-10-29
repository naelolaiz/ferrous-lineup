# La sala de conciertos: Boléro, La Mer, La consagración de la primavera y Petrushka

Archivos: [`crates/lineup-core/src/library/song/classical.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/classical.rs),
[`crates/lineup-core/src/library/song.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song.rs),
[`crates/lineup-core/src/general_midi.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/general_midi.rs)
y [`crates/lineup-core/tests/key_finding.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/tests/key_finding.rs)

Hasta ahora, las demos clásicas eran piezas de piano, o una melodía con
banda. Esta ronda añade cuatro piezas para orquesta de principios del
siglo veinte. Están escritas para muchos instrumentos, y sus compases y
su armonía no se parecen a las canciones del resto de la biblioteca:

- **Boléro** (Maurice Ravel, 1928), completo: 340 compases, un solo
  crescendo largo.
- **La Mer**, su comienzo (Claude Debussy, 1905): los compases 1 a 11,
  el amanecer en el mar.
- Dos danzas de **La consagración de la primavera** (Igor Stravinsky,
  1913): **Los augurios primaverales** y el **Ritual del rapto**.
- **Petrushka** (Igor Stravinsky, 1911, revisado en 1947), sus cuatro
  cuadros, como cinco canciones: **La feria de Carnaval**, **Danza
  rusa**, **La habitación de Petrushka**, **La habitación del Moro** y
  **La feria de Carnaval al atardecer**.

## Probarlo

Abre **Archivo > Demostraciones > Canciones > Clásica y de cine > Boléro** y pulsa ▶. La caja toca los mismos dos compases, en 3/4, del principio al
final. Cuéntalo: **1** (y un tresillo) **2** (y un tresillo) **3 y**.
Mira el mezclador mientras suena: un instrumento tras otro toma la melodía,
y a la caja se le une cada vez más orquesta.

Abre luego **La consagración de la primavera, Juego del rapto** y mira el compás
en la regla. Cambia casi en cada compás: 9/8, 6/8, 7/8, 3/4, y cerca
del final 5/8, 3/8 y 2/8, un compás de un solo golpe.

Por último, abre **Petrushka, La habitación de Petrushka** y escucha los dos
clarinetes después del redoble y del golpe que abre la habitación. Uno
toca un arpegio de Do mayor y el otro uno de Fa sostenido mayor, a la
vez.

Para añadir una orquesta que toque junto a cualquiera de estas piezas
en cualquier métrica, mira [Una orquesta para la música que cambia de
métrica](orchestra.md).

## La música

### Boléro: una melodía, cada vez más fuerte

Boléro no desarrolla sus melodías. Dos melodías de dieciséis compases
cada una vuelven una y otra vez sobre el mismo ritmo de caja y el mismo
Do y Sol punteados en el bajo. Todo lo demás es **orquestación**, la
elección de los instrumentos. La flauta, el clarinete, el fagot agudo
en su registro, el pequeño clarinete en Mi bemol, el oboe d'amore, una
trompeta con sordina, los saxofones: cada uno toca una melodía, y cada
vez se une al ritmo un poco más de la orquesta.

La primera melodía está en **Do mayor**. La segunda empieza en Si bemol,
la séptima de Do mayor rebajada, así que está en **Do mixolidio**, y toma
prestados Re bemol, Mi bemol y La bemol, la segunda, tercera y sexta
rebajadas de **Do frigio**. La demo marca sus partes como Do mixolidio. El
buscador de tonalidad de la app oye en cambio sus primeras apariciones
como Do menor, porque Mi bemol y La bemol son también notas de Do menor.

En un momento, Ravel mezcla colores para hacer otros nuevos. Una trompa
toca la melodía, la celesta la toca una y dos octavas más aguda, y dos
flautines la tocan en **Sol** y en **Mi**, a una octava y una quinta y a
dos octavas y una tercera por encima de la trompa: la tercera y la
quinta notas de la serie de armónicos de la trompa. Los flautines se
funden entonces con la trompa como los registros de un órgano, y oyes un
instrumento nuevo en lugar de tres melodías en tres tonalidades. Solo
al final del todo abandona la música el Do: ocho compases en **Mi mayor**,
y de vuelta a Do para el crash.

### La Mer: un pedal y una escala pentatónica

El comienzo de La Mer no tiene acordes en el sentido habitual. Los
contrabajos y los timbales sostienen un **Si** grave, un **pedal**, y
por encima las arpas y los violonchelos se mecen entre Fa sostenido y Sol
sostenido, notas de una escala pentatónica sobre Si. La armadura es la
de Si menor, pero el Sol sostenido la hace **Si dórico**: el modo menor con
la sexta elevada. La hoja de acordes se queda vacía, porque ningún símbolo
de acorde dice lo que está pasando.

### La consagración de la primavera: dos acordes a la vez, y acentos fuera de sitio

**Los augurios primaverales** martillea un acorde en corcheas iguales, y
ese acorde son dos acordes a la vez, un **polychord**: Fa bemol mayor (Mi,
Sol sostenido, Si) en las cuerdas graves bajo Mi bemol séptima (Mi bemol, Sol,
Si bemol, Re bemol) por encima. El compás se queda en 2/4, pero ocho
trompas golpean el acorde en acentos que caen donde ningún pulso lleva
al oído a esperarlos, así que el pulso constante se tambalea.

El **Ritual del rapto** cambia de compás todo el tiempo, y no está en
ninguna tonalidad. Unas tres de cada cuatro de sus notas vienen de una
**escala octatónica**, una escala de ocho notas que va semitono, tono,
semitono, tono: Do, Re bemol, Mi bemol, Mi, Fa sostenido, Sol, La, Si bemol.

### Petrushka: dos tríadas a un tritono de distancia

Petrushka es una marioneta, y Stravinsky le dio un acorde propio, el
**acorde de Petrushka**: Do mayor (Do, Mi, Sol) y Fa sostenido mayor (Fa
sostenido, La sostenido, Do sostenido) a la vez. Las dos tríadas están a
un **tritono** de distancia, el intervalo de tres tonos enteros que
parte la octava por la mitad, y no comparten ninguna nota. Las seis
notas siguen estando en una escala octatónica, la misma que en la
Consagración: Do, Re bemol (Do sostenido), Mi bemol, Mi, Fa sostenido, Sol, La,
Si bemol (La sostenido). Ninguna tonalidad mayor o menor las contiene.

El resto de Petrushka es la feria alrededor de su caseta, y está lleno
de **melodías populares rusas**. Son **modales**: sus notas son las de
una tonalidad mayor o menor, pero giran alrededor de otra nota. La
melodía inicial de flauta está en **Re dórico** (Re menor con Si natural).
La Danza rusa se mueve en **acordes paralelos**, cada voz del acorde
moviéndose con la melodía, sobre un bajo en Sol, en **Sol mixolidio** (Sol
mayor con Fa natural). Los cocheros del último cuadro patean en **Mi
mixolidio** sobre un **bordón** en Mi.

### Modos sobre bordones, y el buscador de tonalidad

El buscador de tonalidad de la app (la opción **Buscar la tonalidad** bajo las
teclas de armonía) mira qué notas se tocan y durante cuánto, y elige la
tonalidad mayor o menor que mejor les encaja. Las melodías modales le
cuestan: Mi mixolidio tiene las mismas notas que La mayor, y el buscador
suele elegir La mayor. La tónica de Mi mayor escrita solo se ve en el
bordón sobre Mi. De la misma manera oye el **Re dórico** de teclas blancas
del mercader en Petrushka como Do mayor, y el **Si dórico** de La Mer como
Si mayor. Por eso la prueba que mide el buscador sobre la biblioteca
permite unos cuantos compases mal más cada vez que entra música así.

## El código

### Una orquesta en quince canales

La melodía de una demo, su mano izquierda y su batería reciben cada una
canales propios. Una parte puede decir qué instrumentos de **General
MIDI** tocan su melodía con `played_by`, y a qué volumen con `marked`:

```rust
Part::new(
    "Petrushka's chord",
    "-:6 (3 -:1 C4 E4 | C5:8 | ...",
    "",
)
.with_left("-:6 (3 -:1 Bb3 C#4 | Bb4:8 | ...")
.in_key(C, Chromatic)
.played_by(&[(gm::CLARINET, 0)])
.marked(Dynamic::Piano),
```

Aquí dos clarinetes tocan el acorde de Petrushka. Un instrumento toca el
arpegio de Do mayor, y la mano izquierda, en el canal Teclados, toca el de Fa
sostenido mayor. `played_by` recibe una lista de parejas
`(program, semitones)`: cada instrumento toca la melodía, desplazada
esos semitonos, así que una lista con varias entradas dobla la melodía a
la octava. Cada instrumento nuevo recibe un canal propio en la demo, y
una demo puede tener como mucho quince canales con altura. Para Boléro
se añadieron dos programas a `general_midi.rs`: `MUTED_TRUMPET` y
`SOPRANO_SAX`.

### Una tonalidad que ninguna escala contiene

`in_key(C, Chromatic)` marca una parte como sin tonalidad: las doce
notas son su escala. El Ritual del rapto, los compases sobre el acorde
de Petrushka y la pelea del Moro están marcados así. La prueba de
búsqueda de tonalidad se salta los compases cromáticos, así que no se
cuentan como mal.

### Métricas que cambian cada compás

`in_meters` da a una parte una métrica para cada compás:

```rust
.in_meters(&[
    TWO_EIGHT,
    SIX_EIGHT,
    THREE_FOUR,
    THREE_FOUR,
    SIX_EIGHT,
    // ...
])
```

La demo corta la parte en tramos de compases en una misma métrica, cada
uno con un marcador de tiempo en la regla (mira [Métricas
irregulares](odd-meters.md)). Estas canciones necesitaban métricas que
la biblioteca no tenía todavía, así que `song.rs` ganó constantes para
2/8, 3/8, 4/8, 5/8, 7/8, 1/4, 2/2, 3/2 y 13/16.

La Consagración también encontró un bug. Una mano izquierda escrita
puede tener varias voces, unidas con `&`. Cuando una parte con métricas
que cambian se cortaba en tramos de compases, `bars_of` contaba las
líneas de compás de todas las voces como una sola, así que los compases
de la segunda voz se tomaban como más compases de la primera. Ahora
corta cada voz por su cuenta y une los trozos otra vez con `&`. Una
prueba unitaria en `song.rs` lo cubre.

### Una prueba para canciones politonales

La prueba de búsqueda de tonalidad mide cuántos compases de las demos
oye el buscador en su tonalidad escrita. Los augurios primaverales
harían esa cifra insignificante: está en dos tonalidades a la vez, así
que nombre la tonalidad que nombre el buscador, la mitad de la música
discrepa. Tales canciones van a una lista propia:

```rust
const POLYTONAL: &[&str] = &["The Rite of Spring, The Augurs of Spring"];
```

La prueba principal las deja fuera, y una segunda prueba las comprueba
por su cuenta: la tonalidad escrita debe estar entre las tonalidades
oídas, pero no ser la única:

```rust
assert!(result.right_bars > 0, ...);
assert!(
    result.right_bars < result.bars,
    "{name} is heard in one key: drop it from POLYTONAL"
);
```

El segundo assert mantiene honesta la lista. Si un cambio en el buscador
le hiciera oír alguna vez los Augurios en una sola tonalidad, la canción
ya no pertenecería a la lista.

### Cómo se escribieron las piezas

Leer partituras orquestales manuscritas nota a nota es lento, así que
estas demos vienen de versiones legibles por máquina:

- **Boléro** viene de una edición en LilyPond de la partitura completa
  (el proyecto `nsceaux/Ravel_Bolero`, dominio público), compilada a
  MIDI con LilyPond y comprobada contra el autógrafo de Ravel.
- El comienzo de **La Mer** se leyó del autógrafo de Debussy.
- **La Consagración** y **Petrushka** vienen de secuencias MIDI de las
  partituras. Los tempos, las fermatas, las dinámicas y las métricas se
  comprobaron contra la partitura completa de 1921 de la Consagración.
  Para Petrushka, se conserva cada métrica de la secuencia, y el
  comentario de cada parte da los compases que cubre.

Los scripts que leen esos archivos mantienen la melodía delante, la
siguen de instrumento en instrumento con `played_by`, y escriben en el
canal Teclados todo lo demás que importa. Las líneas dobladas al unísono o
a la octava se escriben una vez. Las notas de adorno, los trinos, los
trémolos y los glissandos se simplifican, y el comentario `// Source:`
de cada canción dice cómo.

### Derechos de autor

La consagración de la primavera y la revisión de 1947 de Petrushka
siguen bajo derechos de autor en muchos países, ya que Stravinsky murió
en 1971. Se incluyen a petición del dueño del proyecto, y un comentario
encima de cada una de estas canciones lo dice. El Boléro de Ravel y La
Mer de Debussy son más antiguos, y sus compositores murieron hace
bastante como para estar en el dominio público en la mayoría de los
países.

## Ejercicios

1. Toca **Boléro** y apunta, para cada aparición de la melodía, qué
   instrumento la toca. ¿Cuántas apariciones hay antes de que la música
   abandone Do mayor?
2. Abre **La consagración de la primavera, Augurios primaverales** y, en el teclado,
   toca Mi, Sol sostenido, Si con la mano izquierda y Mi bemol, Sol, Si bemol,
   Re bemol con la derecha. ¿Qué notas de los dos acordes están a un
   semitono de distancia?
3. Toca Do, Mi, Sol y Fa sostenido, La sostenido, Do sostenido juntos: el
   acorde de Petrushka. Toca ahora la escala octatónica Do, Re bemol, Mi
   bemol, Mi, Fa sostenido, Sol, La, Si bemol. ¿Qué notas de la escala no
   están en el acorde?
4. Abre **Petrushka, La feria de carnaval al atardecer**, activa
   **Buscar la tonalidad** y toca la Danza de los cocheros. ¿Qué tonalidad
   muestra el buscador, y por qué se oye así Mi mixolidio?
5. Imprime el informe de búsqueda de tonalidad con
   `cargo test -p lineup-core --test key_finding -- --ignored --nocapture`
   y encuentra en él el último cuadro de Petrushka. ¿Qué tonalidades da
   la partitura, y qué tonalidades oye el buscador en su lugar?
