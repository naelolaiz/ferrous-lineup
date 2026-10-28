# Ejercicios

## 1. Más acordes, uno que sigue faltando (lineup-core, fácil)

`Dominant13` y `Minor11` ya están ahora en `ChordQuality::ALL`; el paso 6
los añadió junto con otros cinco. Lee `ChordQuality::intervals`,
`symbol`, `name` y `numeral_suffix`, y luego añade una cualidad que siga
ausente, como `MinorMajor9` (0, 3, 7, 11, 14), y añádela también a
`ChordQuality::ALL`. El compilador enumera cada `match` que hay que
actualizar; `no_two_qualities_have_the_same_notes` te dirá si el nuevo
conjunto de clases de alturas choca con uno existente. Como clases de
alturas, ¿por qué los acordes grandes a menudo se parecen a otros
acordes construidos sobre otra fundamental?

*Practica:* `match` exhaustivo, dejar que las pruebas protejan tus
datos.

## 2. Mostrar el grado de la escala en las teclas (lineup-app, fácil)

`paint_keys` en `piano_roll.rs` dibuja un punto en cada nota de la
tonalidad. Dibuja en su lugar el número de grado (1 para la fundamental,
2, 3...), usando `Scale::degree_of`. Una fila ya numera sus notas; deja
ese camino como está. Los músicos usan estos números todo el tiempo
(«el acorde 5», «una 7 bemol»).

*Practica:* `Option` en código de dibujo, `painter.text`.

## 3. Velocity desde el teclado (lineup-app, medio)

La velocity ya está en la aplicación: `NoteEvent` tiene un `velocity`,
el `velocity_lane` del piano roll lo edita con el ratón, y el
[paso 6](../step-6/piano-roll.md#el-carril-de-velocidad) explica el carril.
Lee cómo `replace` envía un `RollAction::ReplaceNotes` y cómo la lista
de atajos comprueba primero las combinaciones más largas.

Luego amplíalo: haz que Ctrl/Cmd+↑ y Ctrl/Cmd+↓ suban o bajen las notas
seleccionadas en 8, recortado a 1..=127, como un solo paso de deshacer.
Añade los atajos junto a `KEY_UP`/`KEY_DOWN`, antes que ellos en la
lista para que «subir un semitono» no se coma el Ctrl+↑, y llama a
`replace` con una etiqueta que nombre el cambio.

*Practica:* sintaxis de actualización de structs, leer la entrada de
egui, aritmética de `u8`.

## 4. Inversiones, y un drop-2 (lineup-core, medio)

`tools::inversion(pitches, times)` y `tools::invert(notes, up)` ya están
en la aplicación: la lista de inversiones del menú Draw usa `inversion`
para los acordes estampados, y `Tool::Invert { up }` llama a `invert`.
Lee `chords`, `above`/`below` e `invert`.

Luego amplíalo con un voicing **drop 2**: para cada acorde, baja una
octava la segunda nota más aguda. Añádelo junto a `invert` con una
prueba como `inverting_chords_up_and_down`, y devuelve `None` cuando la
nota caería por debajo de `Note::MIN`. ¿El carril de acordes sigue
nombrando Do/Mi, o cambia el bajo? ¿Por qué?

*Practica:* manipulación de `Vec` (`remove`, `push`, `sort`), pruebas
que comprueban tu propia salida.

## 5. Ajustar a la escala, con una dirección (lineup-core y lineup-app, medio)

`tools::nearest_in_key` y `tools::snap_to_scale` ya están en la
aplicación, y la casilla «Snap to scale» del piano roll los usa. Lee
`nearest_in_key`: prueba un semitono por debajo, luego por encima, luego
dos, así que en caso de empate gana la nota más grave.

Luego amplíalo con `nearest_in_key_towards(note, scale, direction)`, que
rompe el empate en la dirección en la que se mueve la melodía. Llámalo
desde una segunda casilla o un elemento de Tools, y escribe pruebas para
notas exactamente a mitad de camino entre dos notas de la escala, en
ambas direcciones. ¿Qué prueba existente en `tools.rs` fija la regla del
empate, y la función nueva cambiaría su respuesta?

*Practica:* buscar con iteradores (`min_by_key` o `find_map`), mantener
la lógica fuera de la interfaz.

## 6. Un sintetizador legato (lineup-audio, más difícil)

El Lineup Osc todavía no tiene modo mono / legato. Dale uno: solo suena
una voz, y una nota nueva cambia la altura de esa voz sin reiniciar la
envolvente. Añade un tiempo de `glide` para que la altura se deslice de
la nota vieja a la nueva (usa `dsp::Smoothed` para la frecuencia). ¿Cómo
cambia `pick_voice`? ¿Qué partes de la máquina de estados `Stage` se
saltan?

*Practica:* máquinas de estados, reutilizar piezas pequeñas de DSP.

## 7. Piénsalo

- `find_chord` prueba primero la nota del bajo. ¿Puedes encontrar un
  conjunto de notas donde esto dé un nombre sorprendente? ¿Hay una regla
  mejor?
- `SetNotes` envía la lista completa en cada cambio. ¿La qué tamaño se
  volvería eso un problema, y qué enviarías en su lugar?
- El arrastre de una sola nota sigue recordando una nota por su índice
  en la lista (`Drag::Move { index, .. }`), mientras que una selección
  recuerda valores. ¿Qué podría salir mal con el índice si la lista se
  mantuviera ordenada por tiempo de inicio, y se moviera una nota
  mientras se arrastra?
- ¿Por qué `Garbage` es un enum, en lugar de dos colas de basura
  separadas? ¿Qué costarían dos colas, y qué ganarían?
