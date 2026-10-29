# Ejercicios

## 1. Un cencerro grave (lineup-audio, fácil)

`Drum::Cowbell` y `drums::cowbell` ya están en la app: el paso 8 añadió
el cencerro con el resto del kit de ritmo. Lee `cowbell`: dos ondas
cuadradas a 540 y 800 Hz, un paso alto a 400 Hz, un paso bajo a 3 kHz y
dos caídas.

Luego amplíalo. Convierte las dos frecuencias en parámetros
(`cowbell_at(sample_rate, low, high)`) y añade un `Drum::LowCowbell` una
octava más abajo. El compilador enumera todos los `match` que hay que
actualizar en `drums::render`, `Drum::name` y `general_midi_drum`; añade
también el nuevo tambor a `Drum::ALL` (y su longitud), o los menús y las
pruebas de batería no lo verán. Añadir un tambor también cambia qué
proyectos pueden abrir las versiones anteriores: lee `format_for` e
`is_new_in_format_3` antes de quedártelo.

*Practica:* enums, `match` exhaustivo, closures, un cambio que cruza
crates.

## 2. Limpiar una fila (lineup-app, fácil)

Hacer clic derecho en un paso ya lo apaga (`RackAction::ClearStep`).
Añade «Clear steps» al menú de nombre de un canal en `channel_menu`, con
un `RackAction::ClearRow { channel }` nuevo. En `LineupApp::apply`, llama
a `edit_pattern` para apagar todos los pasos de ese canal, sin enviar a
mano un comando al hilo de audio: el diff de la edición envía las notas
nuevas. ¿Por qué una edición de fila entera es mejor que un bucle de
ediciones de un solo paso?

*Practica:* el patrón de acciones, mantener sincronizados proyecto y
audio.

## 3. Pintar pasos arrastrando (lineup-app, medio)

En la app puedes hacer clic en un paso cada vez. Da a
`widgets::step_button` un `Sense::click_and_drag()` y haz que
`Rows::steps` pinte cada paso por el que pasa el puntero. `Rows` se
reconstruye en cada frame y solo devuelve acciones, así que guarda el
primer paso y si se estaba encendiendo o apagando en `LineupApp` (o en
la memoria de egui) y pásalo.

Añade `RackAction::SetStep { channel, step, on }` en lugar de reutilizar
`ToggleStep`, para que un paso por el que el puntero cruza dos veces no
se vuelva a alternar. Comprueba `response.hovered()` junto con
`ui.input(|i| i.pointer.primary_down())`, y deja de pintar cuando el
puntero salga de la fila.

*Practica:* estado en modo inmediato, entrada de egui, el patrón de
acciones.

## 4. Swing, ya incorporado (lineup-core, medio)

El swing ya está en la app: `lineup-core/src/swing.rs` retrasa la
segunda corchea de cada pulso al *reproducir* las notas, y el paso 9
añade el control. Lee `delay`, `swung` y `Project::played_notes`.

Luego amplíalo. Hoy hace swing con corcheas. Añade una variante de
semicorcheas: dentro de cada pulso, retrasa la segunda semicorchea de
cada pareja (la «e» y la «a» de «1 e & a») una cantidad menor, elegida
por un segundo campo o un enum `SwingUnit`. Escribe una prueba que
compruebe que las parejas de pasos siguen tardando el mismo tiempo total
y que las notas escritas no se tocan. ¿Qué más tocaría el nuevo ajuste:
el archivo de proyecto, el control, la exportación MIDI?

*Practica:* aritmética con ticks, pruebas unitarias con números exactos,
mantener separadas la reproducción y la edición.

## 5. Velocidad, ya incorporada (lineup-core y lineup-app, más difícil)

Las notas ya llevan velocity: `NoteEvent` tiene `velocity: u8`, y el
`velocity_lane` del piano roll la edita (`piano_roll.rs`). Lee cómo el
carril convierte la altura en `1..=127` y envía un
`RollAction::ReplaceNotes`.

Luego amplíalo con una rampa de velocity: una herramienta que toma las
notas seleccionadas y reparte un crescendo o un diminuendo entre la
primera y la última. Escribe la interpolación en `lineup_core::tools`,
junto a `transpose` y `snap_to_scale`, con pruebas para una selección
ascendente, descendente y de una sola nota. ¿Qué tipo transporta la
nueva operación desde la interfaz: un `RollAction` nuevo, un `Tool`, o
ambos? ¿Dónde debería ejecutarse la rampa para que sea un solo paso de
deshacer?

*Practica:* aritmética con `u8`, seguir una acción por el workspace,
mantener la lógica fuera de la interfaz.

## 6. Piénsalo

- ¿Por qué `Command::InsertChannel { instrument: Box<Instrument>, ... }`
  sería peor que enviar el instrumento por valor? (`Instrument` ya
  guarda sus arrays de voces en boxes; ¿qué aportaría un segundo box?)
- ¿Qué pasaría si la interfaz olvidara llamar a `collect_garbage`?
- `AudioEngine::playhead` usa `Ordering::Relaxed`. ¿Podría la interfaz
  llegar a ver un número de paso que nunca se reprodujo?
