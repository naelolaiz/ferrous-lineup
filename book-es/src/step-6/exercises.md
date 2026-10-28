# Ejercicios

Algunas de estas herramientas se construyeron más adelante en el libro.
Cuando eso ha pasado, el ejercicio señala primero el código terminado y
luego pide una extensión que aún falta. Copiar, cortar, pegar y
duplicar, por ejemplo, ya están en el piano roll: lee `Tool::Copy` y
`clipboard_tool` si quieres ver cómo reutilizan `ReplaceNotes`.

## 1. Humanizar velocidades (lineup-core, fácil)

Las notas que se tocan todas exactamente con la misma fuerza suenan
mecánicas. Añade `tools::humanize(notes, amount, seed) -> Vec<NoteEvent>`,
que mueve cada velocidad hacia arriba o hacia abajo una cantidad
aleatoria de como mucho `amount`, mantenida dentro de 1..=127, y una
opción «Humanize velocity» en el menú Tools.

Ferrous Lineup no tiene ningún crate de números aleatorios, y no
necesita ninguno: un generador minúsculo como *xorshift* son tres líneas
de desplazamientos de bits sobre un `u32` (búscalo, y usa `wrapping_mul`
o desplazamientos normales para que nada se desborde).
`lineup_core::harmony::ToneRow::random` y las preguntas del
entrenamiento auditivo usan generadores así. ¿Por qué la función recibe
un `seed` en lugar de inventarse uno? Piensa en cómo la probarías, y en
qué debería dar dos veces seguidas la misma semilla. ¿De dónde saca el
piano roll una semilla distinta cada vez?

*Prácticas:* funciones puras con una semilla explícita, operaciones de
bits, `clamp`.

## 2. Retrogradar una frase (lineup-core y lineup-app, medio)

Tocar una frase al revés es una manera clásica de hacer una variación, y
el piano roll no tiene ninguna herramienta para ello. Añade
`tools::retrograde(notes: &[NoteEvent], span: usize) -> Vec<NoteEvent>`:
dentro de un tramo de `span` steps, cada nota conserva su altura y su
longitud, pero su inicio se refleja como en un espejo, de modo que una
nota en el step 0 va al final del tramo y una nota en el step 3 va cerca
del principio.

Averigua qué significa «reflejado» para una nota de longitud 2 en un
tramo de 16 steps: ¿se refleja su *inicio*, su *final*, o ambos? ¿Qué
elección mantiene la frase dentro del tramo y suena más como la original
tocada al revés? ¿Qué pasa con `nudge` y `trim`? Añade una opción
«Retrograde» al menú Tools, con una prueba sobre una frase de
cuatro notas que puedas cantar. El resultado pasa por `replace` como el
de cualquier otra herramienta.

*Prácticas:* escribir una función pura, los campos de `NoteEvent`,
decidir una regla musical antes de programarla.

## 3. Nombrar acordes a lo largo de un pulso (lineup-core y lineup-app, medio)

Después de «Arpegiar acordes», la fila de acordes de la propia pista
nombra cada step por su cuenta: «A», «A+C», «A5»... Puedes verlo en la
captura del arpegio de este paso. Tu oído oye La menor, porque recuerda
las notas del último pulso.

La fila «All» ya hace algo parecido: `heard_by_beat` reúne cada nota que
suena durante un pulso y nombra la armonía una vez por pulso. Lo que
falta es la misma opción para la fila de la pista. Añade un pequeño
conmutador «por step / por pulso», y en el modo «por pulso» nombra cada
pulso a partir de cada nota que *empieza* en ese pulso (o que aún se
mantiene cuando empieza).

Escribe la parte de reunir en `lineup-core` como una función sobre
`Pattern`, para poder probarla: un arpegio de Do, Mi, Sol a lo largo de un
pulso debe nombrarse Do. ¿Qué debería pasar cuando un pulso contiene el
final de un acorde y el principio del siguiente?

*Prácticas:* rangos y `flat_map`, decidir qué significa «a la vez»,
pruebas sobre música que conoces.

## 4. El acorde napolitano (lineup-core, medio)

Los acordes tomados prestados de la menor homónima ya están en la tabla:
iv, ii°, bIII, bVI y bVII sugieren todos sus acordes siguientes
habituales. Toca Do, y después Si♭ mayor, en Do mayor, y `next_chords`
responde.

El **napolitano** sigue faltando: bII, el acorde mayor sobre el segundo
grado rebajado (Re♭ en Do). Suele ir a V, o directamente a casa, a I, y es
común antes de una cadencia. Añade un `Slot` con fundamental 1 y
`ChordFamily::Major`, con esos dos movimientos y sus motivos, a `MAJOR`
y a `MINOR` (las dos tonalidades lo toman prestado). ¿Qué otro campo de
`Suggestion` tiene que estar bien para que los botones «Acordes en Do
mayor» se iluminen, y qué debería hacer el piano roll con una sugerencia
cuyo acorde no es uno de los siete botones?

Escribe primero las pruebas: Re♭ a Do y Re♭ a Sol en Do mayor; después el
mismo napolitano en una tonalidad menor, donde bII también es prestado.
¿Pertenece la nota Re♭ a la escala de Do menor, o también está fuera de
esa tonalidad?

*Prácticas:* extender una tabla de datos, las pruebas como
especificación, pensar si el objetivo es un grado de la tonalidad.

## 5. Herramientas de longitud de nota (lineup-core y lineup-app, medio)

El menú Dibujar tiene un ajuste «Longitud» para las notas *nuevas*, pero
no hay manera de cambiar la longitud de las notas ya escritas. Añade dos
herramientas:

- `tools::set_note_length(notes, steps)` hace que cada nota dure `steps`
  steps, conservando su inicio y su altura, y deja sitio al final del
  patrón;
- `tools::legato(notes, pattern_length)` hace que cada nota dure hasta
  que empieza la siguiente, de modo que un acorde o una línea melódica
  quede unida.

`NoteEvent` guarda su longitud en steps enteros más `trim`, cuántos
ticks antes del final de su último step se detiene. Una nota acortada a
un step debe mantener su `trim` por debajo de un step; una nota más
larga que se acorta debe limpiar cualquier `trim` que ya no quepa.
Añade ambas al menú Tools, y comprueba qué pasa cuando dos notas
se solaparían entonces en el mismo canal. ¿Por qué `set_note_length` no
es simplemente `note.length = steps`?

*Prácticas:* usar `tidy_notes` correctamente, longitudes de nota y
ticks, mantener las reglas de un patrón.

## 6. Una herramienta de rasgueo, o un rasgueo por canal (los tres crates, más difícil)

La [introducción del paso](README.md#lo-que-no-está-aquí-una-herramienta-de-rasgueo)
explica lo que falta: no la sincronización fuera de la cuadrícula, que
`NoteEvent::nudge` ya proporciona, sino una herramienta y un ajuste de
canal para tus propias notas. La guitarra de la banda en
`crates/lineup-core/src/band/guitar.rs` es un ejemplo que funciona:
desplaza cada cuerda un tick más tarde que la de abajo. Desarrolla ambos
diseños antes de escribir ningún código.

- **En las notas.** Una herramienta de rasgueo da a cada nota de un
  acorde un `nudge` creciente, desde la nota más grave hacia arriba, con
  unos pocos ticks de separación. Nada más cambia en `NoteEvent`: el
  archivo ya guarda `nudge`, `diff` ya compara las notas que toca el
  hilo de audio, y Cuantizar puede deshacerlo. La herramienta necesita
  las notas del acorde y una dirección (hacia abajo o hacia arriba), y
  debe mantener las notas dentro de sus steps (un nudge son como mucho
  23 ticks).
- **En el instrumento.** Da a cada canal un ajuste de «rasgueo» en
  milisegundos. Cuando varias notas de un canal empiezan en el mismo
  frame, el hilo de audio las inicia con esa separación de milisegundos,
  primero la más grave. Nada cambia en el patrón, pero `Channel`, el
  comando `InsertChannel`, `project_commands` y el formato de archivo
  necesitan todos el campo nuevo.

¿Cuál te permite rasguear un acorde y el siguiente no? ¿Cuál hace
posible además colocar notas fuera de la cuadrícula (el «swing», o un
golpe de batería ligeramente adelantado)? ¿Cuál cambia menos archivos?
Los estudios de verdad tienen las dos.

*Prácticas:* diseñar a través de los crates, evolucionar un formato de
archivo, planificación con precisión de muestra.

## 7. Piensa en ello

- La selección guarda notas por valor, lo que funciona porque un canal
  nunca tiene dos notas en el mismo lugar. `same_place` compara la
  altura, el inicio y el `nudge`. Imagina que esa regla no existiera
  (dos notas exactamente en el mismo lugar). ¿Qué fallaría con la
  selección, con `replace_notes` y con el carril de velocidad? ¿Lo
  arreglaría dar a cada nota un id permanente (`NoteId(u32)`), y qué
  costarían los ids en el archivo y en `diff`?
- `tools::transpose` se niega a mover un acorde si una nota fuera a
  salirse del rango MIDI. Otro diseño movería las notas que pueden
  moverse y dejaría el resto. ¿Cuándo sería eso mejor? ¿Hay algún caso
  en el que negarse confunda al usuario, y cómo podría el piano roll
  decirle por qué no ha pasado nada?
- `next_chords` guarda su conocimiento en tablas de fundamental, familia
  y movimientos. La mayor parte de la tabla podría calcularse en su
  lugar a partir de unas pocas reglas (fundamentales que caen una
  quinta, subidas a la dominante). ¿Qué entradas se les escaparían a las
  reglas? ¿Es una tabla que dice claramente lo que significa mejor que
  unas reglas más cortas?
- `contains` sobre un `Vec` recorre todos los elementos. El piano roll
  lo llama por cada nota, en cada frame. ¿La qué tamaño de patrón
  merecería la pena un `HashSet<NoteEvent>`? ¿Cómo lo medirías, en lugar
  de adivinar? (Busca `std::time::Instant`.)
- egui empareja Shift+↑ como ↑. ¿Por qué podría una librería elegir eso
  a propósito? Piensa en teclas que necesitan Shift en algunas
  disposiciones de teclado, como `+` o `?`.
