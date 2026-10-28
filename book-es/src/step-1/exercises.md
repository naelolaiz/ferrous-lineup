# Ejercicios

Pruébalos en tu propia rama. Cada uno practica algo de este paso.
Ejecuta `cargo test` y `cargo clippy` después de cada cambio.

> Estos ejercicios apuntan al `main` de hoy, no a la instantánea del
> paso 1. El tono de prueba y `synth.rs` se sustituyeron en el paso 2;
> el código de abajo usa en su lugar el Lineup Osc de `instrument.rs`.
> La antigua rama `step-1-foundation` ya no se publica.

## 1. Nombres con bemoles, a la manera existente (lineup-core, fácil)

`Note` ya conoce las dos grafías: `Note::new(61).name(Spelling::Flats)`
es `Db4`, mientras que `Display` sigue escribiendo `C#4`. Lee
`Spelling` y los dos métodos `name` en `pitch.rs`.

Escribe `fn flat_name(self) -> String` que use esa maquinaria, con una
prueba para una tecla negra con sostenido, una con bemol y una nota
natural. Luego añade una variante `Spelling::Auto` que elija sostenidos
para Do♯ y Fa♯ y bemoles para las demás teclas negras (como hace
`NoteName::chromatic`), y haz que `name` la maneje. ¿Qué matches te pide
actualizar el compilador, y por qué `Spelling` puede seguir siendo
`Copy`?

*Practica:* enums, `match`, bloques `impl`, pruebas.

## 2. Transponer, con límite (lineup-core, fácil)

`Note::transpose(self, i32) -> Option<Note>` ya está; devuelve `None`
cuando el movimiento saldría de 0..=127. Léelo y lee sus pruebas.

Añade `fn transpose_clamped(self, semitones: i16) -> Note`: el mismo
movimiento, pero detente en `Note::MIN` o `Note::MAX` en lugar de
devolver `None`. Escribe pruebas para ambas direcciones y los bordes.
¿Por qué `Option` encaja mejor en un piano roll, y cuándo es mejor
limitar? Busca `i16::clamp`.

*Practica:* casts entre tipos numéricos, `Option` frente a saturación,
desbordamiento.

## 3. Una onda nueva para el Lineup Osc (lineup-audio, medio)

En este paso el sintetizador era `TestTone`; hoy es `Synth` en
[`instrument.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/instrument.rs).
Lee `Wave::at` y `Wave::gain` en
[`timbre.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/timbre.rs),
y luego `SynthVoice::next_sample` en `instrument.rs`.

Añade una variante `Wave::Pulse` que valga `1.0` durante el primer
cuarto del ciclo y `-1.0` el resto. El compilador enumera los matches
que hay que actualizar en `Wave::at` y `Wave::gain`; luego da un pulso a
un grupo de programas de General MIDI en `Timbre::for_program` (los Reed
de onda cuadrada son una elección natural). Añade una prueba en
`timbre.rs` para los cuatro cuartos del ciclo. ¿Por qué `Eq` sigue
funcionando en una variante sin campos, y qué se rompería si `Pulse`
llevara un ancho `f32`?

*Practica:* enums, `match`, `derive`, seguir un programa de General MIDI
del channel rack al hilo de audio.

## 4. Un botón de reinicio (lineup-app, fácil)

La nota y el volumen del tono de prueba se fueron con el tono de prueba.
La misma idea vive en el volumen master de la barra de herramientas:
añade un botón «Reset» junto a él que devuelva el master a
`lineup_core::Mixer::default().master` (0.8).

Llama a `LineupApp::set_master_volume` en
`crates/lineup-app/src/app/mixer.rs` en lugar de asignar el campo, para
que el cambio se pueda deshacer y el hilo de audio reciba
`Command::SetMasterVolume`. Ejecuta la app y comprueba que el deslizador
se mueve.

*Practica:* interfaz de modo inmediato, enviar comandos, encaminar un
cambio a través de `edit`.

## 5. Rómpelo a propósito (cualquier crate)

Aprender a leer los errores del compilador es la mitad de aprender Rust.
Prueba cada una de estas cosas, lee el error y luego deshazla:

- En `engine.rs`, llama a
  `build_stream::<f32>(&device, config.clone(), audio_thread, Arc::clone(&problems))`
  y luego a
  `build_stream::<f32>(&device, config, audio_thread, Arc::clone(&problems))`,
  de modo que la segunda llamada falle porque `audio_thread` se movió.
- Quita `move` del closure del callback de audio.
- Añade una variante `SetName(String)` a `Command`. ¿Por qué `Command`
  no puede ser `Copy`? ¿Qué variantes existentes ya lo hacen imposible,
  y qué dice el compilador cuando pides `Clone`?
- Haz `pub` el campo de `Note` y escribe `Note(200)` en `app.rs`. ¿Por
  qué compila eso, y por qué es una mala idea?

*Practica:* propiedad, closures `move`, tipos `Copy` frente a no-`Copy`,
privacidad.
