# Ejercicios: la teoría musical en código

Estos ejercicios practican esta parte sobre el código real: las notas y
los acordes, los números romanos, las herramientas de edición, la
escucha de acordes y tonalidades, los conjuntos, las series y los
micromodos, la conducción de voces y la importación de grabaciones.
Cada uno dice qué **ya existe**, para que lo amplíes en lugar de
volver a construirlo. Algunos son cosas que tocar en la aplicación y
escuchar; otros son piezas de Rust con un final claro; los que se
llaman «Piénsalo» son preguntas, sin código que escribir. Muchos son
teoría musical antes que nada: saca la respuesta en papel, y después
deja que el código la compruebe.

Haz los de Rust en una rama propia. Ejecuta `cargo test` después de
cada cambio, y `cargo fmt --all --check` y
`cargo clippy --workspace --all-targets -- -D warnings` antes de dar
uno por terminado: la CI ejecuta los tres.

## Notas, escalas y acordes

### 1. Un acorde que todavía falta (lineup-core, fácil)

`ChordQuality::ALL` tiene treinta cualidades, desde las tríadas hasta
`Dominant13` y `Major13` (mira [Acordes: construirlos, nombrarlos e
invertirlos](chords.md)). Lee `ChordQuality::intervals`,
`symbol`, `name` y `numeral_suffix`, y luego añade una que no esté,
como la novena menor-mayor, `MinorMajor9` (0, 3, 7, 11, 14), también a
`ChordQuality::ALL`, cuya longitud, que es parte de su tipo, sube a 31.
El compilador lista cada `match` que hay que actualizar. `SPELLINGS`,
en `harmony/symbol.rs`, necesita una línea para la nueva cualidad, y
`every_quality_is_written_and_read_back` falla hasta que su símbolo se
lea de vuelta como la nueva cualidad. `no_two_qualities_have_the_same_notes`
te avisa si su conjunto de alturas choca con uno que ya existe.

Como clases de altura, ¿por qué los acordes grandes se parecen tan a
menudo a otros acordes sobre otra fundamental? Escribe las notas de
C13 y encuentra un acorde de séptima escondido en ellas.

*Practica:* `match` exhaustivo, arrays cuya longitud es parte del tipo,
dejar que los tests cuiden tus datos.

### 2. Cadd2 y el power chord (lineup-core, medio)

Los cifrados pop a veces escriben **Cadd2**: las mismas notas que
Cadd9, con el Re tocado abajo, junto al Do. Añade «add2» a las
escrituras de `Add9` en `SPELLINGS`, y un test que lo lea. ¿Qué se
pierde al leerlo como `Add9`?

**C5** es un **power chord**: solo Do y Sol, sin tercera, el favorito
de la guitarra. No es una `ChordQuality`. Antes de añadirlo, haz la
lista de lo que más tendría que conocerlo: `ChordQuality::family`, que
todo acorde debe responder como mayor, menor o disminuido; `is_minor`;
las teclas de armonía; y `find_chord`, que entonces nombraría cada
quinta sola. ¿Un acorde sin tercera es mayor o menor? Decídelo, y
escribe la decisión en un test.

*Practica:* seguir un tipo por todo el programa antes de cambiarlo.

### 3. El cliché de línea (lineup-core, pequeño)

El **cliché de línea** mantiene un acorde mientras una sola voz baja
por semitonos: `Am | Am/G# | Am/G | Am/F#`. Léelo con
`library::chart::parse_chart_in`, tócalo con `chart::accompany` y
`Bass::Held`, y comprueba las notas del bajo en un test. ¿Está Sol♯ en
La menor? ¿En qué menor? ¿Y qué es Am/F♯, como acorde propio, con los
nombres de
[Acordes](chords.md#acordes-con-barra-y-el-acorde-de-sexta-y-novena)?

*Practica:* leer texto de cifrados, acordes con barra, un test sobre
música que puedes cantar.

### 4. Una disposición drop 2 (lineup-core, medio)

`tools::inversion(pitches, times)` y `tools::invert(notes, up)` invierten
acordes (mira [Herramientas de edición como funciones
simples](tools.md#inversiones)). Los pianistas y arreglistas
de jazz usan también otra disposición, el **drop 2**: toma un acorde en
posición cerrada y baja una octava su segunda nota más aguda. Do Mi Sol
Si, desde Do4 hacia arriba, queda Sol3 Do4 Mi4 Si4.

Añade `tools::drop_two(notes)` junto a `invert`, con un test como
`inverting_chords_up_and_down`, y devuelve `None` cuando una nota caería
por debajo de `Note::MIN`. Después ponlo en el menú **Herramientas** del
piano roll. ¿El carril de acordes sigue nombrando el acorde igual, o el
bajo nuevo le cambia el nombre? ¿Por qué?

*Practica:* manipular un `Vec` (`remove`, `insert`, `sort`), tests que
comprueban tu propio resultado.

### 5. Ajustar a la escala, con dirección (lineup-core y lineup-app, medio)

`tools::nearest_in_key` y `tools::snap_to_scale` llevan las notas que
están fuera de la tonalidad a la nota más cercana de ella, para
**Ajustar a la escala** del piano roll. `nearest_in_key` prueba un
semitono abajo, luego arriba, luego dos, así que en un empate gana la
nota de abajo, y en Do mayor Do♯ va a Do.

Añade `nearest_in_key_towards(note, scale, up)`, que desempata en la
dirección en que se mueve la melodía: un Do♯ que va subiendo hacia Re
debería volverse Re. Escribe tests para notas exactamente a mitad de
camino entre dos notas de la tonalidad, en los dos sentidos.
`snapping_moves_only_notes_outside_the_key` fija la regla vieja;
¿cambiaría sus respuestas la función nueva?

*Practica:* buscar con iteradores (`find_map`), dejar la lógica fuera
de la ventana.

### 6. Los números romanos en menor, de dos maneras (la aplicación y lineup-core, fácil)

**Edición > Ajustes… > Armonía > Números romanos en tonalidades
menores** escribe los números de una tonalidad menor de dos maneras
(mira [Tonalidades menores: bIII o
III](numerals.md#tonalidades-menores-biii-o-iii)).

1. En La menor armónica, ¿qué escribe cada manera para Do aumentado
   (Do Mi Sol♯)? ¿Y para Mi menor? Sácalo, y compruébalo con las
   teclas de armonía.
2. El dórico también es un modo menor, pero `Scale::is_minor_key` es
   falso para él, así que sus números siempre cuentan desde la escala
   mayor. Re dórico es Re Mi Fa Sol La Si Do. ¿Cuáles de sus números
   cambiarían si contaran desde su propia escala, y por qué eso podría
   confundir más que ayudar?
3. `MinorNumerals::with` guarda el ajuste viejo en un guardián,
   `let _restore = Restore(MinorNumerals::current());`, cuyo `drop` lo
   vuelve a poner. Renombra `_restore` a `restore` y ejecuta
   `cargo clippy`. Luego cámbialo por un `_` solo y ejecuta
   `cargo test -p lineup-core`. ¿Qué pasa cada vez, y por qué `_` se
   comporta tan distinto de `_restore`?

*Practica:* leer números romanos, guardianes que limpian en `Drop`.

### 7. Piénsalo: nombrar acordes

- `find_chord` prueba primero la nota del bajo como fundamental.
  ¿Puedes encontrar un conjunto de notas en que eso dé un nombre
  sorprendente? ¿Hay una regla mejor?
- Cuando nada encaja, `find_chord` añade la quinta que falta y vuelve a
  probar, pero solo acepta acordes de cuatro notas o más. ¿Por qué
  estaría mal aceptar así una tríada? Piensa en Do y Mi solos.

## Los números romanos y lo que hacen los acordes

### 8. Después viene la napolitana (lineup-core, medio)

Después de un acorde, las teclas de armonía remarcan los acordes que
suelen seguirlo, sacados de las tablas de `harmony/progression.rs`
(mira [Qué viene después: el conocimiento como
datos](numerals.md#qué-viene-después-el-conocimiento-como-datos)).
La hoja de acordes ya nombra el **acorde napolitano**, ♭II, un acorde
mayor sobre el segundo grado rebajado (Re♭ en Do), pero `next_chords`
no tiene un lugar para él, así que después de un acorde de Re♭ no se
ilumina nada.

La napolitana suele ir a V, o a I⁶₄ y luego a V. Añade un `Slot` con
fundamental 1 y `ChordFamily::Major`, con un movimiento a V y su razón,
a `MAJOR` y a `MINOR`. Escribe primero los tests: después de Re♭ en Do
mayor, Sol; después de Re♭ en Do menor, Sol también. ¿Es Re♭ una nota
de la escala de Do menor, o la napolitana está fuera de esa tonalidad
también? Una `Suggestion` solo se hace para un acorde cuya fundamental
es una nota de la escala: encuentra el campo de `Suggestion` que lo
necesita, y explica por qué la napolitana misma nunca puede ser
sugerida.

*Practica:* ampliar una tabla de datos, tests como especificación.

### 9. Las funciones de oído (la aplicación, fácil)

Escribe esto en la hoja de acordes y escucha.

1. `C Fm C`, y luego `C F C`. La única nota que cambia es La a La♭.
   ¿Qué le hace al ánimo? Busca una canción que conozcas que haga
   esto.
2. En Sol mayor, ¿qué acorde es V7/V? ¿Cuál de sus notas no está en Sol
   mayor, y hacia dónde quiere ir esa nota? Escríbelo y mira la función
   que le da la hoja.
3. Escribe una modulación de Do mayor a Fa mayor a través de un acorde
   pivote. ¿Qué acordes comparten las dos tonalidades? ¿Qué acorde
   muestra que Fa mayor ha llegado? (¿Qué nota tiene Fa mayor que Do
   mayor no tiene?)

*Practica:* oír acordes prestados, dominantes secundarias y pivotes.

### 10. La escritura decide (lineup-core, pequeño)

1. `is_neapolitan` comprueba cómo se escribe el acorde con
   `key.tonic_name().letter.steps_up_to(symbol.root.letter) == 1`.
   Explica por qué Do♯ mayor en Do no pasa esa prueba mientras que Re♭
   mayor sí, aunque los dos se tocan en las mismas teclas del piano.
   Encuentra el test que lo dice.
2. La hoja de acordes no nombra los acordes de **sexta aumentada**
   (italiana, francesa y alemana), que también llevan a la dominante.
   Busca sus notas en Do menor. ¿Cuál de ellos suena exactamente como
   una séptima de dominante sobre La♭, y qué en su escritura distingue
   a los dos? ¿Qué tendría que comprobar un `Role::AugmentedSixth`?

*Practica:* por qué la música escribe las notas, con letras además de
semitonos.

### 11. Una fila de acordes prestados (lineup-app, medio)

Las canciones en tonalidades mayores suelen tomar prestados acordes de
la tonalidad menor sobre la misma tónica: iv (Fa menor en Do), ♭VI
(La♭), ♭VII (Si♭) y ♭III (Mi♭). Es el **intercambio modal**, detrás de
muchos momentos agridulces del pop, y la hoja de acordes ya nombra
esos acordes como prestados. Las teclas de armonía, en cambio, solo
tocan los acordes propios de la tonalidad.

Añade una segunda fila de teclas, más pequeña, debajo de las teclas de
armonía, con estos cuatro acordes, construidos a partir de números
romanos con `library::numeral::parse` y `RomanChord::in_key`. Dales su
propio color, y una ayuda emergente que diga de qué acorde de la
tonalidad menor viene cada uno.

*Practica:* reutilizar los números romanos de la biblioteca, la
distribución en egui.

### 12. Piénsalo: tablas o reglas

`next_chords` guarda lo que sabe en tablas de fundamental, familia y
movimientos. Casi toda la tabla se podría calcular a partir de unas
pocas reglas: fundamentales que bajan una quinta, pasos hacia arriba
hasta la dominante. ¿Qué entradas se perderían las reglas? ¿Es mejor
una tabla que dice claramente lo que significa que unas reglas más
cortas?

## Las herramientas de edición

### 13. Humanizar las velocidades (lineup-core y lineup-app, fácil)

Las notas tocadas todas exactamente igual de fuerte suenan mecánicas.
Las demostraciones ya varían sus velocidades (`library/interpret.rs`),
pero el piano roll no tiene ninguna herramienta para tus propias notas.
Añade `tools::humanize(notes, amount, seed) -> Vec<NoteEvent>`, que sube
o baja cada velocidad como mucho `amount`, dentro de 1 a 127, y un
elemento «Humanizar velocidad» en el menú **Herramientas** del piano
roll.

Ferrous Lineup no tiene un crate de números aleatorios, y no lo
necesita: `ToneRow::random` y el solista usan pequeños generadores
propios, y `library/interpret.rs` usa el mezclador «SplitMix64», unas
pocas líneas de `wrapping_mul` y desplazamientos. ¿Por qué la función
recibe una `seed` en lugar de inventarse una? ¿Cómo la probarías, y
qué debería dar la misma semilla dos veces seguidas? ¿De dónde sacaría
el piano roll una semilla distinta cada vez?

*Practica:* funciones puras con una semilla explícita, `clamp`.

### 14. Retrogradar una frase (lineup-core y lineup-app, medio)

Tocar una frase al revés es una manera clásica de hacer una variación.
Las series tienen su retrógrado (mira [Series dodecafónicas y sus
reglas](twelve-tone.md)), pero el piano roll no tiene una
herramienta para una frase. Añade
`tools::retrograde(notes: &[NoteEvent], span: usize) -> Vec<NoteEvent>`:
dentro de `span` pasos, cada nota conserva su altura y su duración, y
su lugar se refleja.

Primero decide qué significa «reflejar» para una nota de duración 2 en
un tramo de 16: ¿se refleja su comienzo, su final, o los dos? ¿Qué
elección mantiene la frase dentro del tramo y suena más como la
original tocada al revés? ¿Qué pasa con `nudge` y `trim`? Añade un
elemento «Retrogradar» al menú **Herramientas**, con un test sobre una
frase de cuatro notas que puedas cantar.

*Practica:* decidir una regla musical antes de programarla, los campos
de `NoteEvent`.

### 15. Herramientas de duración (lineup-core y lineup-app, medio)

**Duración**, en el piano roll, fija cuánto duran las notas *nuevas*,
pero ninguna herramienta cambia las notas ya escritas. Añade dos:

- `tools::set_note_length(notes, steps)` hace que cada nota dure
  `steps`, conservando su comienzo y su altura;
- `tools::legato(notes, pattern_length)` hace que cada nota dure hasta
  que empiece la siguiente, para que una línea quede ligada.

`NoteEvent` guarda su duración en pasos enteros, más `trim`, cuántos
ticks antes del final de su último paso se detiene. Una nota alargada
tiene que borrar un `trim` que ya no significa nada, y `tidy_notes`
mantiene las notas de un patrón en orden y dentro de él. Comprueba qué
pasa cuando dos notas de un canal se solaparían. ¿Por qué
`set_note_length` no es simplemente `note.length = steps`?

*Practica:* duraciones de notas y ticks, respetar las reglas de un
patrón.

### 16. Una herramienta de rasgueo, o un rasgueo por canal (los tres crates, más difícil)

Un guitarrista toca un acorde una cuerda tras otra. La guitarra de la
banda ya lo hace: `strum`, en `band/guitar.rs`, empieza cada cuerda
unos ticks después de la anterior. Tus propias notas todavía no pueden.
Piensa dos diseños antes de escribir código.

- **En las notas.** Una herramienta Rasguear da a cada nota de un
  acorde un `nudge` creciente, desde la nota más grave hacia arriba,
  con unos ticks de separación. Nada más cambia en `NoteEvent`: el
  archivo ya guarda `nudge`, y Cuantizar devuelve las notas a la
  cuadrícula. Un nudge es como mucho 23 ticks, un paso menos uno.
- **En el instrumento.** Cada canal recibe un ajuste de «rasgueo» en
  milisegundos. Cuando varias notas de un canal empiezan en el mismo
  frame, el hilo de audio las empieza con esa separación, la más grave
  primero. Nada cambia en el patrón, pero `Channel`, el comando
  `InsertChannel`, el `project_commands` de la aplicación y el formato
  del archivo necesitan el campo nuevo.

¿Cuál te deja rasguear un acorde y el siguiente no? ¿Cuál cambia menos
archivos? Los estudios de verdad tienen los dos.

*Practica:* diseñar a través de varios crates.

### 17. Piénsalo: las herramientas

- La selección guarda las notas por valor, lo que funciona porque un
  canal nunca tiene dos notas en el mismo lugar: `same_place` compara
  altura, comienzo y `nudge`. Imagina dos notas exactamente en el mismo
  lugar. ¿Qué fallaría en la selección y en `replace_notes`? ¿Lo
  arreglaría un identificador permanente para cada nota, y qué costarían
  los identificadores en el archivo?
- `tools::transpose` se niega a mover un acorde si una nota saldría del
  rango MIDI. Otro diseño movería las notas que pueden moverse y dejaría
  el resto. ¿Cuándo sería mejor? ¿Cómo podría el piano roll decirte por
  qué no pasó nada?

## Oír la armonía y encontrar la tonalidad

### 18. Un bajo caminante, a mano (lápiz y papel, fácil)

[Oír la armonía](hearing.md#un-bajo-caminante-paso-a-paso)
calcula cómo puntúan C y C7 sobre un bajo caminante, con el Si♭ de paso
pesando 0,15 por paso, y dice que con 0,35 C seguiría ganando, por
0,941 a 0,925. Calcula tú esos dos números: el Si♭ pesa entonces 1,4 y
el perfil 29,4. Luego cambia `PASSING_BASS` en `harmony/hearing.rs` y
escribe el caso como un test. ¿Con qué peso gana C7?

*Practica:* seguir una regla de puntuación a mano antes de confiar en
ella.

### 19. Cuándo cortar un compás (lápiz y papel, fácil)

Cuando la hoja de acordes se llena a partir de las notas, un corte
cuesta 0,08 a mitad del compás (`CUT_HALF_BAR`) y 0,12 en cualquier
otro pulso (`CUT_BEAT`) (mira [Cortar un compás en
acordes](hearing.md#cortar-un-compás-en-acordes)). Un
compás de 4/4 toca C, y luego G7. Cada uno, oído solo, puntúa 1,02 por
pulso; el compás entero, oído como un solo tramo, 1,0 por pulso.

¿Corta la hoja el compás cuando G7 llega en el pulso 4, durante un
pulso? ¿Y cuando llega en el pulso 3, durante dos? Mira el
`if total > here.0` del bucle: ¿qué pasa en un empate? ¿Qué tan alta
tiene que ser la puntuación de cada acorde solo para que haya corte en
cada caso?

*Practica:* programación dinámica sobre un caso pequeño.

### 20. El acorde de los Augurios (la aplicación y lápiz, fácil)

Toca Mi, Sol♯ y Si con la mano izquierda y Mi♭, Sol, Si♭ y Re♭ con la
derecha: el acorde que abre los Augurios primaverales de *La
consagración de la primavera*. Ahora toca la escala de La♭ menor
armónica, La♭ Si♭ Do♭ Re♭ Mi♭ Fa♭ Sol, y comprueba que todas las notas
del acorde están en ella. ¿Por qué `in_one_key`, en
`harmony/hearing.rs`, deja fuera la menor armónica?

Luego una pregunta sobre la caché de `write_heard_chords`: su clave
guarda también las notas de los compases de alrededor, no solo las del
compás. ¿Por qué?

*Practica:* la escritura enarmónica, qué tiene que incluir la clave de
una caché.

### 21. Medir el oído de la biblioteca (lineup-core, medio)

La biblioteca tiene un oído propio más sencillo,
`library::listen::hear_chords`, para los patrones de la banda, y
`every_style_is_heard_as_its_chart` lo comprueba sobre cada estilo tal
como está escrito. Escribe un informe propio, un test ignorado: recorre
los estilos, toca cada cifrado con cada `Comping` y algunas líneas de
`Bass` a través de `chart::accompany`, escúchalo, y cuenta cuántas
veces vuelven los acordes como estaban escritos. Imprime las
combinaciones que más fallan. Luego intenta subir el número cambiando
una constante al principio de `listen.rs`, sin romper el test que ya
existe. Ejecútalo con `cargo test --release`: el mismo test corre
muchas veces más rápido.

*Practica:* medir antes de cambiar, bucles anidados sobre enums.

### 22. Dónde cambia de tonalidad la Habanera (la aplicación, fácil)

1. Abre la demostración Habanera (la de Bizet, de *Carmen*), activa
   **🔍 Buscar la tonalidad**, y haz clic en los compases 9, 10 y 11
   de la regla. ¿Dónde exactamente pasa Re menor a Re mayor? ¿Es el
   estribillo en Re mayor solo «la misma melodía en otra tonalidad»?
   Compara las primeras notas de los dos.
2. Mantén La, Do y Mi en el teclado de escribir durante unos compases,
   y luego añade Sol♯, la séptima elevada. ¿Se vuelve el oyente más
   seguro de La menor? ¿Por qué importa tanto una sola nota?

*Practica:* oír una modulación, qué hace que una tonalidad sea una
tonalidad.

### 23. El precio de un cambio de tonalidad (lineup-core, fácil)

En `harmony/key_finding.rs`, pon `CHANGE_COST` en 0,8 y ejecuta el
informe:

```sh
cargo test --release -p lineup-core --test key_finding report -- --ignored --nocapture
```

¿Cuántos cambios se encuentran ahora, y cuántos de ellos no están en
las partituras? Prueba 3,0. ¿Qué te dice eso de la penalización?

*Practica:* ajustar midiendo.

## Conjuntos, series y micromodos

### 24. Un recorrido por la ventana Serie (la aplicación, fácil)

1. Abre **Archivo > Demostraciones > Dodecafónico > Coral de tríadas**
   y pulsa Espacio. Mira los nombres de las secciones en la lista de
   reproducción: la misma música, luego al revés, invertida, y las dos
   cosas.
2. Abre el piano roll (**F7**) y haz clic en **Serie…**. Elige **Series
   famosas y de ejemplo > Suite de Schoenberg**. ¿Qué dice la ventana
   de sus grupos de tres, y con qué forma es combinatoria?
3. Haz clic en la etiqueta **I11** de arriba de la matriz, pon
   **Juntas** en **6: hexacordios**, y haz clic en **Escribir I11 en el
   patrón actual**: dos hexacordios como acordes. Pulsa **Ctrl+Z** para
   deshacerlo.
4. Marca las tres casillas de **Serialismo**, pon **Octavas** en
   **Amplio**, y vuelve a escribir P4: puntos de sonido al estilo de
   Webern.
5. Haz clic en **Conjuntos…**, escribe `octatonic`, y haz clic en
   **Usar como tonalidad**. Las teclas de armonía tocan ahora acordes de
   la escala octatónica.

### 25. Formas distintas (lineup-core, pequeño)

Una serie tiene 48 formas, pero no siempre son 48 series distintas.
Escribe un test que las junte en un `HashSet<ToneRow>` y las cuente.
Comprueba que la serie de la Sinfonía de Webern tiene 24 y la de la
Suite de Schoenberg 48. ¿Puede alguna serie tener menos de 24? (¿Podría
una serie ser una de sus propias formas I? Escribe qué significa eso
para cada nota, y verás que a lo sumo dos notas podrían cumplirlo.)

*Practica:* `HashSet`, una demostración contando.

### 26. Series derivadas (lineup-core, medio)

El Concierto, Op. 24, de Webern usa una serie cuyos cuatro grupos de
tres pertenecen todos a la misma clase, 3-3 [014] (mira
[Micromodos](micromodes.md#una-serie-hecha-de-un-solo-micromodo-el-concierto-de-webern)).
Una serie así se llama **derivada**. Añade
`ToneRow::is_derived(size)`, verdadero cuando cada segmento de ese
tamaño tiene la misma clase de conjunto, y un `random_derived` que
construya una con vuelta atrás, como hace `random_all_interval`.

*Practica:* vuelta atrás, `segments` y clases de conjuntos.

### 27. A la manera de Forte (lineup-core, pequeño)

En seis clases, la forma prima de Forte difiere de la de Rahn, que es
la que usa Ferrous Lineup (mira [Conjuntos de clases de
altura](sets.md)). Escribe `forte_prime()`, que desempata
desde el principio del conjunto en lugar de desde el final, y muéstrala
en los detalles de la ventana Conjuntos cuando sea distinta. ¿Por qué
el truco de «el número más pequeño» no puede hacer esta directamente?
(Piensa en qué bits mira primero una comparación de números.)

*Practica:* bits como conjuntos, leer una definición con cuidado.

### 28. Multiplicar por 5 (lineup-core, medio)

Multiplicar cada clase de altura por 5 (módulo 12) convierte la escala
cromática en el círculo de cuartas: 0 5 10 3 8 1… Esta operación,
**M5**, es otra manera de transformar una serie o un conjunto, que
usaron algunos compositores después de Webern. Añade
`PitchSet::multiply` y `ToneRow::multiply`, y encuentra qué clases de
conjuntos convierte M5 en una clase distinta. (Vale la pena mirar los
pares relacionados por Z.)

*Practica:* aritmética modular, probar sobre todas las clases de
conjuntos.

### 29. Una serie propia (lineup-core y lineup-app, medio)

Con **Duraciones** marcada, la serie escribe sus propios números como
duraciones. Deja que el usuario escriba una segunda serie, usada solo
para las duraciones, y muestra las dos en la ventana. `RowWriting`
deriva `Copy`: ¿por qué sigue compilando cuando guarda una segunda
`ToneRow`, y por qué no compilaría si la segunda serie fuera un
`Vec<u8>`?

*Practica:* `Copy` y lo que le pide a cada campo.

### 30. Las reglas dodecafónicas de oído (la aplicación, fácil)

1. Elige **serie dodecafónica** como tonalidad, elige **Suite de
   Schoenberg** en **Serie…** con Mi como nota de la tonalidad, y
   escribe P4 (Mi Fa Sol Do♯ Fa♯ Mi♭ La♭ Re Si Do La Si♭) como melodía.
   Marca **Conducción de voces**. Luego intercambia dos notas, y luego
   deja una fuera. ¿Cuántas notas marca la revisión cada vez, y adónde
   va el contorno punteado?
2. Escribe las tres primeras notas de P4 como un acorde, y el resto
   como melodía. Luego escribe las notas 1, 2 y 4 como el acorde. ¿Qué
   cambia?
3. Escribe P4 en una pista y P4 una octava más abajo en otra, con el
   mismo ritmo. ¿Qué dice la revisión, y por qué lo dice una sola vez?
4. Escribe un acorde de Do, Mi y Sol con una serie como tonalidad.
   Luego Sol, Si, Re y Fa. ¿Qué regla rompen, y por qué Berg la rompía a
   propósito?

### 31. Una nota repetida (lineup-core, pequeño)

En `Walk::take`, en `harmony/voice_leading/serial.rs`, una nota que solo
repite la nota recién tocada nunca está fuera de lugar. Encuentra las
líneas que lo hacen así. Luego escribe un test en que un acorde
comparte una nota con el acorde anterior, y comprueba que no se marca
nada.

*Practica:* leer máscaras de bits, un test para una excepción a una
regla.

### 32. Un recorrido por los micromodos (la aplicación, fácil)

1. Abre el piano roll (**F7**) en el patrón **Acordes** de la canción
   inicial, pulsa **Ctrl+A**, y haz clic en **Micromodos…**. Lee los
   colores de Do mayor: ocho Mayor 2, seis tríadas, una disminuida.
2. Haz clic en **Menor 3**, y luego en **▶ Escucharlo**. Prueba
   **Invertido (I)** y **Transportarlo**, y mira cómo el triángulo gira
   y se da vuelta en el reloj.
3. Haz clic en **3-3 en Conjuntos →**. La ventana Conjuntos se abre en
   3-3. Elige **7-35** y haz clic en uno de los micromodos de
   **Micromodos que contiene** para volver.
4. Abre **Archivo > Demostraciones > Micromodos > Tríadas, dos notas
   fijas** y pulsa Espacio. Escucha la única nota que se mueve en cada
   acorde.
5. Haz clic en **Los tricordios de Webern**, en el grupo **Series
   dodecafónicas** de la Biblioteca. Con **3 notas** elegido bajo la
   tonalidad, las teclas de armonía tocan sus cuatro grupos de tres,
   cada uno con la etiqueta 3-3. Ahora cambia el menú de la tonalidad
   de **serie dodecafónica** a **conjunto de clases de alturas**: la
   tonalidad pasa a ser el primer grupo de tres de la serie, y las
   teclas de armonía tocan sus formas.
6. De vuelta en la canción inicial, mientras suena, haz clic en
   **Groove de cuartas** en la Biblioteca: la canción sigue en acordes
   sus.

### 33. Los micromodos de una melodía (lineup-core, pequeño)

Escribe `melody_micromodes(notes: &[PitchClass]) -> Vec<Option<&'static
Micromode>>`, que nombra el micromodo de cada tres notas seguidas: las
notas 1 a 3, 2 a 4, 3 a 5 y así (`windows(3)` hace el recorrido). ¿Por
qué la respuesta es `None` en algunas ventanas? Pruébalo con la serie
del Concierto de Webern: sus cuatro grupos son Menor 3, pero ¿qué son
las ventanas que quedan a caballo entre dos grupos?

*Practica:* `windows`, `Option` en una lista.

### 34. Todos los colores a la vez (lineup-core, pequeño)

Recorre `SetClass::all()` y encuentra los conjuntos más pequeños cuyo
contenido de micromodos no tiene ningún cero: conjuntos que contienen
los doce micromodos. ¿Cuántas notas necesitan, y cuáles son? (Para el
tamaño: la tríada aumentada y Menor 1 tienen que caber los dos.)

*Practica:* buscar en una tabla, `micromodes_in`.

### 35. Las tonalidades de Schat (lineup-core, medio)

La serie del Concierto de Webern reparte las doce notas en cuatro
formas de Menor 3. Escribe una búsqueda con vuelta atrás que encuentre
todas las maneras de repartir las doce notas en cuatro formas de un
micromodo, y cuéntalas para cada uno de los doce. Un micromodo no puede
hacerlo de ninguna manera: ¿cuál, y por qué? (Piensa en qué notas
pueden tomar las formas de cada micromodo.)

*Practica:* vuelta atrás, contar con cuidado.

### 36. Un recorrido que recuerda más (lineup-core, medio)

El recorrido de un estilo de micromodos que conserva dos notas elige
primero la nota nueva más fresca, y solo después la forma usada hace
más tiempo (mira [El recorrido: elegir con una
tupla](micromodes.md#el-recorrido-elegir-con-una-tupla)).
Haz un recorrido que use con seguridad todas las formas a las que puede
llegar antes de que vuelva ninguna, y comprueba en un test que Menor 3
visita sus 24 formas en 24 pasos. ¿Es siempre posible? ¿Para qué
micromodos puede un recorrido de «conservar dos notas» visitar cada
forma exactamente una vez? (Es una pregunta sobre caminos en un
grafo.)

*Practica:* grafos, vuelta atrás.

### 37. Escuchar la familia (lineup-app, pequeño)

Añade un **▶** junto al nombre de cada familia en la ventana Micromodos
que toque sus micromodos uno tras otro, desde el Do central.
`MicroAction::Hear` toca las notas juntas; ¿qué añadirías para tocarlas
por turnos? Mira cómo se detiene la vista previa de un acorde en el
piano roll.

*Practica:* acciones como datos, el tiempo en la ventana.

## La conducción de voces

### 38. Octavas paralelas (la aplicación, fácil)

Escribe I–IV–V–I en Do mayor a cuatro voces, con el bajo Do3 Fa3 Sol3
Do3 y la soprano Mi5 Fa5 Re5 Mi5. Marca **Conducción de voces**. Luego
mueve el Re5 de la soprano a Sol5 y lee lo que dice la revisión. ¿Por
qué se evitan las octavas entre la soprano y el bajo, mientras que las
terceras y las sextas entre ellos son bienvenidas?

### 39. La sensible y la séptima (la aplicación, fácil)

1. Escribe G7 a C con la séptima, Fa, en la contralto bajando a Mi.
   Luego escribe G7 a Am, la cadencia rota. ¿Adónde tiene que ir ahora
   la sensible? ¿Y la séptima?
2. La revisión deja en paz la sensible en las voces interiores.
   Escribe V7 a I con el tenor yendo de Si hacia abajo a Sol. ¿Qué gana
   el acorde de Do, y qué le faltaría si el tenor subiera a Do?
3. Escribe C, F, G7, C a cuatro voces con el bajo Do3 Fa2 Sol2 Do3, y
   cada voz interior moviéndose por grado conjunto o manteniéndose. Lee
   la línea de movimientos en cada cambio: ¿están las voces extremas
   alguna vez en movimiento paralelo o directo?

### 40. De un unísono a una octava (lineup-core, pequeño)

En `kind_of_move`, un unísono que va a una octava cae hasta los dos
últimos brazos del `match`. Saca qué tipo recibe cuando las dos voces
suben, y luego escribe un test en `voice_leading.rs` que lo compruebe,
con el ayudante `satb` que usan los otros tests.

*Practica:* leer un `match` brazo por brazo.

### 41. Un paso de tres semitonos (lineup-core, pequeño)

`Move::size` llama paso a uno o dos semitonos. Una segunda aumentada,
La♭ a Si, son tres semitonos, y su nombre dice «segunda». ¿Qué tamaño
le da el código, y cuál le daría un profesor de armonía? Cambia `size`
para que cuente letras, como hace `motion`, y escribe un test. ¿Qué
otros movimientos cambian de tamaño?

*Practica:* letras y semitonos, otra vez.

## Importar una grabación

### 42. Una plantilla sus4 (lineup-listen, medio)

`chords.rs` conoce seis cualidades. Añade `ChordQuality::Sus4` a
`QUALITIES` y mira qué hace en una grabación con un acorde sus: renderiza
a WAV una demostración que tenga uno (el test `write_wav` de
`tests/key_changes.rs` lo hace). Más candidatos son más maneras de
equivocarse. ¿Necesita `SEVENTH_COST` un primo para los acordes raros?

*Practica:* medir un cambio sobre datos conocidos.

### 43. Seis por ocho (lineup-listen, medio)

`meter::guess` conoce 3 y 4 pulsos por compás. Añade el 6/8: seis
pulsos en dos grupos de tres, donde los acordes cambian en los pulsos 1
y 4. ¿Cómo distingues 6/8 de 3/4 al doble del tempo, cuando los dos
ponen cambios cada tres pulsos? (Quizás no se pueda solo con los
acordes: ¿qué otra cosa podría distinguirlos?)

*Practica:* los límites de un método.

### 44. Un sampler que empieza por la mitad (lineup-audio, lineup-core y lineup-listen, más difícil)

La importación guarda una copia de la grabación con una entrada, y la
grabación solo suena cuando la canción empieza desde su primer paso
(mira [Asperezas](listen.md#asperezas)). Las dos cosas
vienen de que el sampler siempre toca un sonido desde su primera
muestra. Dale a `SampleSource::File` un desplazamiento de comienzo en
segundos, y deja que una voz empiece en una `position` distinta de 0 en
el `note_on` del sampler. Luego:

- borra `write_wav`, y usa la entrada como desplazamiento;
- cuando la reproducción empieza en un paso posterior a la nota de un
  sonido largo, empieza ese sonido en el lugar justo. ¿Qué parte del
  motor sabe dónde está la canción, y qué notas ya están sonando?

*Practica:* cambiar un tipo que se guarda en archivos, el hilo de
audio.

### 45. La tonalidad del tramo más largo (lineup-listen, fácil)

El proyecto recibe la tonalidad que `bpm-key` oyó en toda la grabación.
El informe de [Qué tan bien
funciona](listen.md#qué-tan-bien-funciona) dice que la
tonalidad del tramo más largo de `Analysis.keys` acierta más a menudo.
Úsala en `arrange::follow` y en el resumen, conserva la tonalidad de
`bpm-key` para las grabaciones donde ningún tramo es claro, y vuelve a
ejecutar el informe.

*Practica:* dejar que decida una medición.

### 46. Piénsalo: escuchar

- Los acordes se nombran por pulso, y el chroma de un pulso es el
  promedio de sus ventanas. ¿Qué le pasa a un acorde que cambia en el
  «y» de un pulso, como en mucho jazz y mucha música latina?
- ¿Por qué el compás se adivina a partir de los acordes y no de la
  intensidad de los pulsos, como siente el tiempo fuerte quien
  escucha?
- La importación corre en su propio hilo y se suelta, no se detiene,
  cuando se abre otro proyecto. ¿Cuándo valdría la pena detenerla, y
  cómo le dirías a un hilo que se detenga? (Mira cómo se detiene una
  exportación.)
