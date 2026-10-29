# Ejercicios

## 1. Teclas de octava (lineup-app, fácil)

El teclado de escritura cubre de Do4 a Mi6. Añade dos teclas que lo
muevan una octava abajo y arriba, como `-` y `=` (`Key::Minus` y
`Key::Equals`), que aún no tocan ninguna nota. Mantén el desplazamiento
entre Do1 y Do7, y muéstralo en el menú MIDI («Teclado de escritura: de
Do3 a Mi5»).

El caso difícil: mantén Z, pulsa `=`, y luego suelta Z. ¿Qué nota debe
terminar? Mira `typing_down`, que guarda la nota que tocó cada tecla,
no solo la tecla. ¿Por qué se escribió así, si la nota podría deducirse
de nuevo a partir de la tecla?

*Prácticas:* aritmética de `Note` con `checked_add_signed` (paso 6),
recordar lo que se hizo en lugar de recalcularlo.

## 2. Tecleo más suave y más fuerte (lineup-app, fácil)

Las teclas escritas siempre tocan a `DEFAULT_VELOCITY`, así que las
tomas grabadas desde el teclado de escritura tienen todas la misma
velocidad, mientras que las de un teclado MIDI conservan con qué fuerza
se pulsó cada tecla. Añade un deslizador «Velocidad de escritura» al
menú MIDI (`egui::Slider::new(&mut value, 1..=127)`), y haz que Mayús
toque a 127 mientras se mantiene.

Mayús no cambia `physical_key`, así que el piano sigue funcionando.
Pero mira el filtro de `typing_keyboard`: ¿una tecla que bajó con Mayús
sube bien si se suelta Mayús primero?

*Prácticas:* deslizadores de egui, modificadores del teclado.

## 3. Un clic en los contratiempos (lineup-audio, medio)

El metrónomo básico que una vez pedía este ejercicio ya existe: la fila
de práctica tiene `Command::SetMetronome`, y la cuenta de entrada usa
los mismos clics. Solo hace clic en los pulsos. Añade un ajuste de
**subdivisión** (corcheas, tresillos o semicorcheas) que haga clic al
metrónomo también entre los pulsos, más suave que el pulso y manteniendo
el acento de compás. `Project::played_clicks` y `pattern_clicks`
construyen un `Click` para cada paso, `metronome::click_at` lo elige, y
`ClickVoice` hace el sonido; decide si la subdivisión va en `Click` o
en un segundo ajuste enviado al hilo de audio. ¿Debería la cuenta de
entrada quedarse en el pulso?

El clic no debe estar en la exportación WAV ni en la exportación MIDI.
¿De dónde sale cada una, y cuál recibiría el clic por accidente si
viviera en un canal?

*Prácticas:* un ajuste nuevo de punta a punta, sonido en tiempo real sin
reservar memoria.

## 4. Notas a través del bucle (lineup-core, medio)

[`recorded_note`](recording.md#lo-que-no-puede-saber) tiene dos límites.
Una nota mantenida a través del final del bucle se corta allí, y una
tecla mantenida más de un bucle entero se graba como una nota corta.

- Haz que una tecla mantenida a través del final del bucle grabe **dos**
  notas: una hasta el final del patrón, y otra desde el paso 0 para el
  resto. La función devuelve entonces un `Vec<NoteEvent>` (o un array y
  una cuenta). Escribe primero las pruebas: (14, 2) en un patrón de 16
  pasos da (14, 2) y (0, 2).
- Para las notas largas, la posición sola no basta. Haz que el hilo de
  audio cuente también cuántas veces ha dado la vuelta el bucle, o que
  publique el número de pasos tocados desde ▶ en lugar de la
  posición en el bucle. ¿Cuál es más sencillo de compartir a través de
  un solo `AtomicU64`? ¿Cuál puede seguir usando `finish_recording`?

*Prácticas:* cambiar el tipo de retorno de una función y sus llamadores,
las pruebas como especificación.

## 5. Grabar en modo canción (los tres crates, más difícil)

Rec cambia a modo patrón, porque una toma va al patrón que se repite.
En modo canción, la posición es un paso de la *canción* (la longitud
del sequencer es entonces la de la canción), así que una tecla podría
ir al clip que suena en ese momento, en `step - clip.start`.

Resuelve las preguntas antes de escribir código. Dos clips pueden sonar
a la vez, en pistas distintas: ¿cuál recibe la nota? ¿Y si allí no
suena ningún clip, habría que crear uno nuevo? Un clip puede tocarse
varias veces en la canción: grabar en una copia los cambia todos,
porque son el mismo patrón. ¿Es eso lo que querrías? ¿Cómo lo manejan
los otros estudios que conoces?

*Prácticas:* diseñar el comportamiento antes que el código, la lista de reproducción
del paso 4.

## 6. Las otras piezas de un archivo de formato 2 (lineup-core, más difícil)

Abrir archivos MIDI, que una vez pedía este ejercicio, ya existe:
`midi_file::read::read` convierte los bytes en eventos, y
`midi_file::import::import` crea el proyecto detrás de **Archivo > Importar archivo MIDI…**. Quedan dos rincones.

Un archivo de **formato 2** no es una canción, sino varias piezas
independientes, una por pista. `import` conserva solo la primera pista
y no dice nada de las demás. Añade una manera de elegir una pieza:
listarlas en el aviso, preguntar en el diálogo o importar cada una como
su propio proyecto. ¿Qué deberían decir las pruebas sobre un archivo de
formato 2 con tres piezas?

El otro es el meta evento **SMPTE offset** (`FF 54`): cinco bytes que
dicen dónde empieza una pista en un timecode, para música escrita
contra una película. El timing SMPTE de la cabecera
(`Timing::PerSecond`) ya funciona; este evento se lee de pasada y se
descarta. Léelo y decide qué debería hacer con él un proyecto sin
vídeo. Prueba ambos casos con archivos pequeños construidos en el
módulo de pruebas, como hacen las pruebas existentes.

*Prácticas:* analizar datos binarios con patrones de slice, diseñar una
elección, tipos de error.

## 7. Compensación de latencia (lineup-audio y lineup-app, más difícil)

La [latencia](recording.md#latencia-lo-que-oyes-está-en-el-pasado) hace
que cada tecla grabada llegue un poco tarde. cpal sabe cuánto: el
callback de audio recibe un `&cpal::OutputCallbackInfo`, cuyo
`timestamp()` tiene dos instantes, `callback` (ahora) y `playback`
(cuándo se oirá este bloque). La diferencia entre ellos es la latencia
de salida.

Hoy el callback ignora ese argumento (`_: &cpal::OutputCallbackInfo`
en `build_stream`). Pasa la latencia al hilo de audio, publícala como
un número de pasos en otro atómico, y réstala de la posición de cada
tecla antes de que `recorded_note` la redondee. Recuerda que una
posición justo después del paso 0 menos la latencia debe dar la vuelta
hasta el final del bucle, no quedarse por debajo de cero.

Luego piensa en la otra dirección. Los mensajes de salida MIDI salen cuando
se hace su bloque, pero el sonido de ese mismo bloque de los
instrumentos integrados se oye unos 21 ms más tarde. Así que un
sintetizador externo toca un poco *antes* que el resto. ¿Cómo podría el
hilo de envío retener cada mensaje durante la latencia? ¿Qué tendría
que llevar cada mensaje?

*Prácticas:* leer la documentación de un crate (`cargo doc --open -p cpal`),
el tiempo como dato.

## 8. Piensa en ello

- La cola live tiene 256 mensajes, y `LiveInput::send` devuelve
  `false` cuando está llena. `on_message` lo ignora. ¿Qué es peor
  perder, un key-down o un key-up? ¿Podría un diseño hacer imposible
  perder un key-up, por ejemplo compartiendo *qué teclas están
  pulsadas* (128 bits) en lugar de enviar eventos?
- El hilo de envío se despierta mil veces por segundo, incluso cuando
  no existe ningún canal de salida MIDI. ¿Podría dormir más cuando no hay
  nada que enviar? ¿Qué costaría un sueño más largo cuando sí lo hay?
- `release_live_keys` suelta todas las teclas cuando se inserta o se
  quita un canal. La otra manera sería renumerar `live_keys` para que
  siga a los canales. ¿Cuál es más fácil de hacer bien? ¿Notaría alguna
  vez la diferencia quien toca?
- midir le da a `on_message` una marca de tiempo en microsegundos, que
  el código ignora en favor de la posición del hilo de audio. ¿Cuándo
  sería más precisa la marca de tiempo de midir? ¿Por qué no se pueden
  comparar sin más los dos relojes?
- Sin la comprobación de formato, una versión de formato 1 de Ferrous
  Lineup abriendo un archivo con un canal de salida MIDI fallaría igualmente:
  serde jamás ha oído hablar de `MidiOut`. Entonces, ¿por qué subir el
  formato? Compara los dos mensajes de error que vería el usuario.
- Un patrón tocado por un canal de salida MIDI y exportado como archivo
  `.mid` envía las mismas notas en ambos casos. ¿Podrían la exportación
  y el hilo de audio compartir más código? ¿Qué necesitaría saber ese
  código sobre el tiempo?
