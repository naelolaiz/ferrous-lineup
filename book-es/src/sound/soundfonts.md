# SoundFonts y General MIDI

Archivos: [`crates/lineup-core/src/general_midi.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/general_midi.rs),
[`crates/lineup-audio/src/soundfont.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/soundfont.rs),
[`crates/lineup-audio/src/instrument.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/instrument.rs)
y [`crates/lineup-app/src/app/sounds.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/sounds.rs)

Un canal dice qué instrumento toca con un número: 0 es un piano de cola,
40 un violín, 52 un coro. Los números vienen de **General MIDI**, una
lista en la que están de acuerdo todos los sintetizadores. En Ferrous
Lineup, tres cosas convierten ese número en sonido:

- el Lineup Osc elige una onda y una envolvente que se le acercan
  ([Timbres](instruments.md#timbres-sonar-como-un-instrumento-general-midi));
- a un sintetizador externo se le pide por MIDI que lo toque, con un
  *cambio de programa* ([MIDI](midi.md));
- un **SoundFont**, un archivo de grabaciones de instrumentos reales, lo
  toca dentro de la aplicación.

Este capítulo trata de los números, de dónde saca uno cada canal, y del
SoundFont. Cómo activar los sonidos y elegir instrumentos en la
aplicación está en el capítulo
[Sonidos](https://naelolaiz.github.io/ferrous-lineup/es/manual/sounds.html)
del manual.

## General MIDI: 128 instrumentos por número

Un cable MIDI o un archivo MIDI llevan notas, no sonidos. Para decir qué
sonido debe tocarlas, MIDI tiene el mensaje de cambio de programa: «a
partir de ahora, el canal 3 toca el programa 40». Son dos bytes: `0xC2`
(cambio de programa, `0xC0`, en el canal 3, que contado desde 0 es el 2)
y `40`.

Por sí solo, 40 no significa nada. General MIDI, publicado en 1991, fijó
una lista de 128 programas, para que el 40 sea un violín en cualquier
sintetizador que la siga, desde un teléfono hasta un teclado de
escenario. Los programas vienen en dieciséis familias de ocho: pianos
del 0 al 7, percusión cromática (celesta, vibráfono, marimba) del 8 al
15, órganos del 16 al 23, guitarras del 24 al 31, bajos del 32 al 39,
cuerdas del 40 al 47, y así hasta los efectos de sonido, del 120 al 127.
El canal MIDI 10 está reservado a la batería: allí cada *nota* es un
tambor, el 36 el bombo, el 38 la caja, el 42 el charles cerrado, sea
cual sea el programa elegido. Los manuales cuentan los programas desde 1
(«41 Violin»); los bytes cuentan desde 0.

`general_midi.rs`, en `lineup-core`, guarda la lista. Los programas que
usan las demostraciones y los estilos tienen nombre, y cada programa
tiene su nombre y su familia:

```rust
pub const VIOLIN: u8 = 40;
pub const CELLO: u8 = 42;
pub const CHOIR_AAHS: u8 = 52;
```

```rust
/// The name of program `program`, such as "Violin" for 40.
pub fn name(program: u8) -> &'static str {
    NAMES[usize::from(program.min(LAST))]
}

/// The family program `program` belongs to, 0 (piano) to 15 (sound effects).
pub fn family(program: u8) -> usize {
    usize::from(program.min(LAST)) / 8
}
```

`NAMES` es un `[&str; 128]` y `FAMILIES` un `[&str; 16]`, los dos en el
orden de General MIDI. Como cada familia tiene ocho programas, «qué
familia» es una división por 8, y los programas de la familia `f` son
`f * 8..f * 8 + 8`. `.min(LAST)` impide que un número mayor que 127
indexe más allá del final: un `u8` puede valer hasta 255, y un archivo
dañado podría traer uno. Los menús se construyen con esos mismos dos
arrays, y los nombres pasan por la tabla de traducciones, así que el
menú en español dice «Violín».

La batería integrada conserva sus propios sonidos, pero todo lo que
habla General MIDI necesita cada tambor como una nota del canal 10.
`general_midi_drum`, en `midi_file.rs`, es esa tabla:

```rust
/// The note General MIDI drum sets play for each built-in drum.
pub fn general_midi_drum(drum: Drum) -> Note {
    Note::new(match drum {
        Drum::Kick => 36,
        Drum::Snare => 38,
        Drum::Clap => 39,
        Drum::HiHat => 42,
        Drum::OpenHat => 46,
        Drum::Rim => 37,
        Drum::LowTom => 45,
        Drum::HighTom => 50,
        Drum::Ride => 51,
        Drum::Crash => 49,
        Drum::Cowbell => 56,
        Drum::Shaker => 70,
        Drum::Conga => 63,
    })
}
```

El 37 es el «Side Stick» de General MIDI, el golpe de aro; el 70 son las
«Maracas», lo más parecido que tiene a un shaker; el 63 es «Open Hi
Conga», la conga aguda abierta. `drum_name` va en sentido contrario para
los 47 tambores del 35 al 81, y así el mezclador puede decir qué tambor
toca una nota. Un test compara unos cuantos con la lista de General
MIDI, y otro comprueba los nombres de los programas, de modo que una
errata en cualquiera de las dos tablas hace fallar los tests.

## El instrumento de un canal

`Channel` tiene un campo para él:

```rust
/// The General MIDI instrument the channel plays, 0 to 127 (see
/// [`general_midi`](crate::general_midi)), or `None` for none in
/// particular. MIDI files and the MIDI output send it to the synth as
/// a program change; the Lineup Osc picks a sound close to it.
#[serde(default, skip_serializing_if = "Option::is_none")]
pub program: Option<u8>,
```

`Option<u8>` porque un canal puede no tener ningún instrumento en
particular. Entonces no se envía nada a un sintetizador, y el Lineup Osc
conserva su diente de sierra brillante. Los dos atributos de serde
mantienen legibles los archivos viejos: un archivo sin el campo se lee
como `None` (`default`), y un canal sin instrumento no escribe el campo
(`skip_serializing_if`). Solo un proyecto en el que algún canal tiene
instrumento necesita el formato 9, así que un Ferrous Lineup más antiguo
sigue abriendo el resto ([El número de
versión](../program/saving.md#el-número-de-versión)).

Un canal de batería no tiene programa: en el canal MIDI 10 son las notas
las que eligen los tambores. Un sampler que toca un archivo WAV tampoco
lo tiene, porque su sonido es el archivo.

### De dónde sale el número

- **El rack y el mezclador.** Haz clic derecho en el nombre de un canal
  y abre **Instrumento**, o haz clic en el nombre del instrumento en el
  mezclador. Los dos muestran `instrument_choices`: «Ninguno (el sonido
  propio del Lineup Osc)», y luego las dieciséis familias, cada una un
  submenú con sus ocho programas, numerados desde 1 como en los
  manuales. La familia del programa actual lleva un punto delante, para
  encontrarla en una lista larga.
- **La banda.** El panel de la banda de la hoja de acordes da a cada
  músico un instrumento con las mismas familias ([Cómo toca la banda una
  hoja de acordes](../band/band.md)).
- **La biblioteca.** Los canales **Teclados** y **Bajo** de un estilo los
  crea `synth_named`, que les da un piano y un bajo:

  ```rust
  let program = match name {
      KEYS => Some(general_midi::ACOUSTIC_GRAND_PIANO),
      BASS => Some(general_midi::FINGER_BASS),
      _ => None,
  };
  ```

- **Las demostraciones.** Cada voz de una demostración dice su
  instrumento: violines y un violonchelo en el Canon de Pachelbel, un
  bandoneón («Acordeón de tango» en General MIDI) en un tango ([Cómo se
  escribe una demostración](../songs/demos.md)). Un test construye
  cada canción de demostración y comprueba que todos sus canales de
  sintetizador tienen programa.
- **Los archivos MIDI.** Al importar un archivo, cada canal MIDI toma
  como instrumento su primer cambio de programa ([MIDI](midi.md)).

### Adónde va el número

1. **La exportación MIDI** escribe un cambio de programa al principio de
   la pista de cada canal, antes de su primera nota, para que el
   sintetizador esté listo:

   ```rust
   // The instrument first, so the synth is ready before the first note.
   if let Some(program) = channel.program
       && midi_channels[index] != DRUM_CHANNEL
   {
       let change = MidiMessage::ProgramChange {
           channel: midi_channels[index],
           program,
       };
       track.event(0, &change.bytes());
   }
   ```

2. **La salida MIDI** lo envía justo antes de la primera nota del canal,
   y de nuevo después de cada parada, por si entretanto se cambió el
   sonido en el propio sintetizador ([MIDI](midi.md)).
3. **El Lineup Osc** toca un `Timbre` según la familia del programa.
4. **El SoundFont** toca las grabaciones del programa, que es el resto
   de este capítulo.

## Qué es un SoundFont

Un sampler toca una grabación a distintas velocidades
([El sampler](instruments.md#el-sampler)): al doble de
velocidad suena una octava más aguda. Eso funciona para unas pocas
notas, pero un piano grabado en el Do central y tocado dos octavas más
arriba suena a juguete. El timbre de un instrumento real cambia a lo
largo de su registro, y acelerado, su ataque se vuelve demasiado corto.

Un SoundFont (un archivo `.sf2`, un formato de las tarjetas de sonido de
los años noventa) es toda una biblioteca de sampler en un solo archivo:

- **muestras**: grabaciones cortas, el do, el mi, el sol♯ de un piano y
  así cada pocas notas, cada una con la nota en la que se grabó. Los
  sonidos que duran, como un órgano o las cuerdas, tienen un **bucle**:
  una vez sonado el ataque, el centro de la grabación se repite mientras
  la tecla siga pulsada;
- **instrumentos**: qué muestra tocar para qué teclas y velocidades (una
  nota suave puede tocar la grabación de una nota suave), y cómo: la
  envolvente de volumen, un filtro, el vibrato;
- **presets**: lo que elige un cambio de programa MIDI, por **banco** y
  **programa**.

Un SoundFont General MIDI pone los 128 instrumentos de General MIDI en
el banco 0, en su orden numerado, y un kit de batería en el banco 128,
que el sintetizador toca en el canal MIDI 10. Así, el programa 0 es
siempre un piano de cola, el 33 un bajo eléctrico tocado con los dedos,
y la tecla 36 del canal 10 un bombo.

Tocar una nota es buscar la muestra cuyo rango de teclas y velocidades
la contiene, y tocarla más rápido o más lento según la distancia entre
las dos notas. Cada semitono es un factor de 2^(1/12), alrededor de
1,0595, así que una muestra de La4 (440 Hz) tocada para Si4, dos
semitonos más arriba, va 2^(2/12) ≈ 1,122 veces más rápido: 494 Hz.

## Quién lo lee y lo toca: rustysynth

Leer un archivo `.sf2` y tocarlo es mucho código: interpretar el
archivo, los rangos de teclas, las envolventes, los filtros, los bucles,
mezclar decenas de voces. El crate `rustysynth` lo hace todo, en Rust
puro sin ninguna biblioteca de C que instalar, con la licencia MIT, que
un programa GPL puede usar. Su interfaz es pequeña; en resumen:

```rust
let sound_font = Arc::new(SoundFont::new(&mut file)?);
let mut synthesizer = Synthesizer::new(&sound_font, &settings)?;
synthesizer.process_midi_message(0, 0xC0, 33, 0); // channel 1: program 33
synthesizer.note_on(0, 40, 100); // E2, velocity 100
synthesizer.render(&mut left, &mut right); // the next block of sound
```

`soundfont::load` es la primera línea con los errores explicados: un
archivo que no se puede abrir es un `SoundFontError::Io`, y uno que no es
un SoundFont, o está dañado, es un `SoundFontError::Invalid`. Cada uno
imprime un mensaje traducido, así que la barra de estado puede decir «no
es un archivo SoundFont (…)» en el idioma de la aplicación.

Habla MIDI. Y la aplicación ya tiene un instrumento que habla MIDI: la
**salida MIDI** (`MidiOut`), que lleva la cuenta de qué notas suenan y
cuándo debe acabar cada una, y junta mensajes de nota activada, nota
desactivada y cambio de programa en un buzón de salida para un
sintetizador externo ([MIDI](midi.md)). Así que un instrumento
de SoundFont es una salida MIDI cuyos mensajes van a un sintetizador
dentro de la aplicación.

## Una salida MIDI que toca dentro de la aplicación

```rust
/// A channel's instrument played from a SoundFont.
pub struct SoundFontPlayer {
    /// Which notes sound and until when, as for an external synth; its
    /// messages go to the synthesizer once per block.
    notes: MidiOut,
    /// Boxed so an `Instrument` stays small.
    block: Box<Block>,
}

/// The synthesizer and the block of sound it rendered last.
struct Block {
    synthesizer: Synthesizer,
    sound_font: Arc<SoundFont>,
    /// [`GAIN`] or [`DRUM_GAIN`].
    gain: f32,
    left: [f32; BLOCK],
    right: [f32; BLOCK],
    /// The next frame of `left` and `right` to hand out. At `BLOCK`, a
    /// new block is due.
    read: usize,
}
```

El hilo de audio pide a cada instrumento una muestra cada vez. El
sintetizador trabaja en bloques de `BLOCK`, 64 frames, así que el
reproductor reparte el bloque que tiene, y cuando se le acaba le pasa
los mensajes de los últimos 64 frames y le pide el bloque siguiente:

```rust
pub fn next_sample(&mut self) -> f32 {
    self.notes.next_sample();
    let block = &mut *self.block;
    if block.read == BLOCK {
        let synthesizer = &mut block.synthesizer;
        self.notes.flush(|message| play(synthesizer, message));
        synthesizer.render(&mut block.left, &mut block.right);
        block.read = 0;
    }
    let sample = (block.left[block.read] + block.right[block.read]) * 0.5 * block.gain;
    block.read += 1;
    sample
}
```

`self.notes.next_sample()` no hace sonido: el `next_sample` de una
salida MIDI cuenta frames, para saber cuándo se cumple la duración de
cada nota y enviar su nota desactivada. Una nota que empieza a mitad de
un bloque se oye desde el siguiente: como mucho 64 frames tarde, 1,3 ms a
48 kHz, muy por debajo de lo que nadie oye.

`let block = &mut *self.block;` atraviesa el `Box` una sola vez, y así
las líneas siguientes toman prestados `block.synthesizer` y `block.left`
como campos separados. Dentro del `if`, `self.notes` y
`block.synthesizer` están prestados a la vez, uno por `flush` y otro por
el closure; Rust lo permite porque son campos distintos.

Los sonidos del SoundFont son estéreo, y cada canal de la aplicación es
mono hasta que el mezclador lo coloca con su balance
([El mezclador](mixer.md)). Así que los dos lados se suman y
se dividen por dos.

`flush` es el mismo método que entrega los mensajes de una salida MIDI
al puerto MIDI, y `play` convierte cada `MidiMessage` en la llamada
correspondiente del sintetizador:

```rust
/// Hands one MIDI message to the synthesizer.
fn play(synthesizer: &mut Synthesizer, message: MidiMessage) {
    match message {
        MidiMessage::NoteOn {
            channel,
            pitch,
            velocity,
        } => synthesizer.note_on(
            i32::from(channel),
            i32::from(pitch.midi()),
            i32::from(velocity),
        ),
        MidiMessage::NoteOff { channel, pitch } => {
            synthesizer.note_off(i32::from(channel), i32::from(pitch.midi()))
        }
        MidiMessage::ProgramChange { channel, program } => {
            synthesizer.process_midi_message(i32::from(channel), 0xC0, i32::from(program), 0)
        }
    }
}
```

`i32::from` ensancha un `u8` al `i32` que pide rustysynth; no puede
fallar, y por eso es `from` y no `try_from`.

Todo lo que una salida MIDI ya hace bien viene gratis: notas que acaban
al cumplirse su duración, teclas sostenidas desde el teclado, notas que
se callan cuando se silencia su pista, duraciones que se estiran con un
cambio de tempo. El motor también trata a las dos igual en una cosa más:
MIDI tiene una sola nota de cada altura por canal, así que una nota que
vuelve a empezar termina la que suena (`one_per_pitch` en `engine.rs`).

Un reproductor se crea para un programa:

```rust
pub fn new(sound_font: &Arc<SoundFont>, sample_rate: u32, program: u8) -> Option<Self> {
    let notes = MidiOut::new(0).with_program(Some(program));
    Self::on(sound_font, sample_rate, notes, GAIN)
}
```

`with_program` hace que la salida MIDI envíe el cambio de programa antes
de su primera nota, de modo que el sintetizador está en el instrumento
correcto cuando llega la primera nota. Un tambor es una salida MIDI en el
canal 10 que toca una sola nota, sea cual sea la nota de sus pasos
(`playing_as`), como hace un tambor enviado a un sintetizador externo:

```rust
pub fn drum(sound_font: &Arc<SoundFont>, sample_rate: u32, drum: Note) -> Option<Self> {
    let notes = MidiOut::new(DRUM_CHANNEL).playing_as(Some(drum));
    Self::on(sound_font, sample_rate, notes, DRUM_GAIN)
}
```

## Un sintetizador por canal, un SoundFont para todos

Un sintetizador puede tocar dieciséis canales MIDI a la vez, así que uno
solo podría tocar toda la banda. En cambio, cada canal de la aplicación
tiene el suyo. Así su volumen, su balance, sus efectos, su luz de
silencio y su medidor de nivel funcionan exactamente como en cualquier
otro canal, porque el motor mezcla canales, no canales MIDI.

El SoundFont en sí ocupa decenas de megabytes, y no se copia. Se lee una
vez en un `Arc<SoundFont>`, un puntero con *recuento de referencias
atómico*: `Arc::clone` copia el puntero y suma uno a un contador, y la
memoria se libera cuando desaparece el último clon. Cada sintetizador
guarda un clon, y la aplicación otro. Un instrumento que el hilo de
audio reemplaza vuelve por la cola de basura y se descarta en el hilo de
la ventana ([Liberar memoria en otro
sitio](../program/threads.md#liberar-memoria-en-otro-sitio-la-cola-de-basura)),
así que, aunque guarde el último clon, el SoundFont nunca se libera en
el hilo de audio.

`Instrument` tiene una variante para él, y un constructor que dice qué
canales toca el SoundFont:

```rust
pub fn from_sound_font(
    channel: &Channel,
    sample_rate: u32,
    sound_font: &Arc<SoundFont>,
    drums: bool,
) -> Option<Self> {
    let player = match &channel.kind {
        ChannelKind::Synth => SoundFontPlayer::new(sound_font, sample_rate, channel.program?),
        ChannelKind::Sampler(SampleSource::Builtin(drum))
            if drums && channel.sent_on.is_none() =>
        {
            SoundFontPlayer::drum(sound_font, sample_rate, general_midi_drum(*drum))
        }
        _ => None,
    };
    player.map(Instrument::SoundFont)
}
```

El `?` después de `channel.program` devuelve `None` de toda la función
cuando el canal no tiene programa: eso es lo que mantiene en el Lineup
Osc un canal en «Ninguno». Un tambor integrado toca el kit del SoundFont
cuando el ajuste de la batería está activado, salvo que se esté enviando
a un sintetizador externo (`sent_on`). Un archivo de muestra y una
salida MIDI no le tocan al SoundFont. El `build_instrument` de la
aplicación pregunta esto primero y recurre a `Instrument::for_channel`
cuando la respuesta es `None`:

```rust
fn build_instrument(&mut self, channel: &Channel, sample_rate: u32) -> Instrument {
    if let Some((sound_font, drums)) = self.sounds.in_use()
        && let Some(instrument) =
            Instrument::from_sound_font(channel, sample_rate, sound_font, drums)
    {
        return instrument;
    }
    match Instrument::for_channel(channel, sample_rate, &mut self.samples) {
        ...
    }
}
```

Todos los instrumentos se construyen ahí, para tocar y para exportar,
así que **Archivo > Exportar WAV…** y la exportación de video también
suenan con el SoundFont.

Los dos bloques de 64 muestras de un sintetizador ocupan medio
kilobyte, y cada hueco de la cola hacia el hilo de audio es tan grande
como el comando más grande. Por eso el `Block` va en un `Box`, igual que
los demás instrumentos guardan sus voces en uno ([Un número fijo de
voces, en un `Box`](instruments.md#un-número-fijo-de-voces-en-un-box));
el test `commands_stay_small` detectaría un comando que creciera.

### Nada se reserva en el hilo de audio

El hilo de audio nunca pide memoria ([La regla de
oro](../program/threads.md#la-regla-de-oro-del-hilo-de-audio)). Un
sintetizador reserva todo lo que necesita al crearse: sus voces
(`VOICES`, 128 por canal; una nota de piano puede usar dos, su muestra
izquierda y la derecha), sus buffers, la tabla de presets.
`Synthesizer::new` se ejecuta en el hilo de la ventana, dentro de
`build_instrument`, y el instrumento terminado se envía al otro lado. Su
`note_on` y su `render` solo reutilizan esas voces y esos buffers.

`on` prepara el sintetizador:

```rust
let mut settings = SynthesizerSettings::new(i32::try_from(sample_rate).ok()?);
settings.block_size = BLOCK;
settings.maximum_polyphony = VOICES;
// Reverb and chorus would cost as much as the voices, once for
// every channel. The sounds are recorded with some room already.
settings.enable_reverb_and_chorus = false;
let synthesizer = Synthesizer::new(sound_font, &settings).ok()?;
```

El mezclador tiene una reverberación a la que envían todos los canales,
así que la sala sigue ahí, una sola vez ([El
mezclador](mixer.md)). `i32::try_from(sample_rate).ok()?` es
la conversión cuidadosa: un `u32` mayor que `i32::MAX` no cabría, y `?`
sobre el `Option` se rinde en lugar de dar la vuelta. El propio
sintetizador rechaza frecuencias de muestreo por debajo de 16 kHz y por
encima de 192 kHz, y entonces el canal se queda con su Lineup Osc.

### Con volumen suficiente

rustysynth toca bajo, a medio volumen, porque espera que lo comparta
toda una banda. El reproductor lo sube por `GAIN` (5) y la batería por
`DRUM_GAIN` (7). Esos números salen de una medición, no del gusto: se
tocaron las mismas notas con el Lineup Osc y con el SoundFont, y se
compararon sus picos y sus niveles medios. Un piano real se apaga
mientras el Lineup Osc sostiene sus notas, así que los dos nunca
coinciden del todo, y otro SoundFont puede sonar más fuerte o más
suave; el volumen de cada canal hace el resto.

## La primera reproducción, y la descarga

La aplicación no trae un SoundFont, porque uno bueno ocupa decenas de
megabytes. La primera vez que algo suena, una ventana ofrece uno:

![La primera reproducción: la canción sigue sonando mientras la ventana «Sonidos de instrumentos reales» ofrece GeneralUser GS, unos 30 MB, con Descargar, Elegir un archivo… y Ahora no](../images/step-9-soundfont-offer.png)

**Descargar** baja GeneralUser GS, un SoundFont General MIDI gratuito de
S. Christian Collins, desde su propio repositorio, y lo guarda en la
carpeta de datos de la aplicación: `~/.local/share/ferrouslineup/soundfonts`
en Linux, la carpeta `Application Support` en macOS, `AppData\Roaming`
en Windows. **Elegir un archivo…** toma cualquier otro `.sf2`. **Ahora
no** se queda con el Lineup Osc. **Edición > Ajustes… > Audio** tiene
las mismas opciones para más adelante: **Instrumentos del SoundFont**,
**Batería del SoundFont**, **Elegir SoundFont…** y **Usar GeneralUser
GS**.

### Se pregunta una vez

Todas las formas de empezar a tocar (el botón ▶, **Espacio**, una
escucha previa de la biblioteca, abrir una demostración) pasan por
`set_playing`, que termina con

```rust
self.playing = true;
self.offer_sound_font_on_first_play();
```

y eso pregunta una sola vez:

```rust
pub(super) fn offer_sound_font_on_first_play(&mut self) {
    let sounds = &mut self.sounds;
    if sounds.settings.offered {
        return;
    }
    sounds.settings.offered = true;
    sounds.changed = true;
    let missing = sounds.wanted_path().is_some_and(|path| !path.is_file());
    if !sounds.settings.on && sounds.settings.file.is_none() && missing {
        self.set_sound_font(true);
    }
}
```

`offered` se marca antes que nada, así que, sea cual sea la respuesta,
Reproducir no vuelve a preguntar. La oferta es lo mismo que marcar el
ajuste: activa los sonidos, y como falta el archivo, se abre la ventana;
**Ahora no** los vuelve a desactivar. No se pregunta nada si los sonidos
ya están activados, si elegiste un archivo propio, o si GeneralUser GS
ya está en el disco: entonces los sonidos se desactivaron a propósito.
La reproducción sigue mientras la ventana pregunta, así que el primer
sonido nunca espera a una pregunta. La ventana se dibuja en el orden
`Foreground` de egui, encima de todas las demás, porque una
demostración abre su hoja de acordes y la trae al frente en el mismo
frame en que empieza a sonar.

Los ajustes son una pequeña struct guardada en la memoria de egui, que
eframe conserva entre ejecuciones ([Recordar entre
ejecuciones](../program/egui.md#recordar-entre-ejecuciones)):

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
struct Settings {
    /// Whether channels play from the SoundFont.
    on: bool,
    /// Whether the built-in drums do too.
    drums: bool,
    /// A SoundFont file chosen with "Choose SoundFont…", or `None` for
    /// GeneralUser GS.
    file: Option<PathBuf>,
    /// Whether the first Play has offered the SoundFont: it does once.
    offered: bool,
}
```

`#[serde(default)]` sobre la struct rellena cualquier campo que falte en
los datos guardados con su valor de `Default`, de modo que se puede
añadir un campo sin perder los ajustes de nadie. `Default` está escrito
a mano en lugar de derivado, porque `drums` empieza en `true`.

### Descargar en un hilo propio

Una descarga de 30 MB tarda entre un segundo y un minuto, y leer un
SoundFont también lleva un momento. Las dos cosas se hacen en un hilo
propio, como la exportación WAV, y la ventana sigue dibujándose. El hilo
y la ventana comparten un `Progress`:

```rust
/// How far a download is. Its thread writes it and the UI thread reads
/// it; atomics need no lock for that.
#[derive(Default)]
struct Progress {
    /// Bytes downloaded so far.
    done: AtomicU64,
    /// The size of the file, or 0 while it is not known.
    total: AtomicU64,
    /// Set by the UI thread to stop the download.
    cancel: AtomicBool,
}
```

Los *atómicos* son números que dos hilos pueden leer y escribir a la vez
sin un cerrojo: la descarga guarda cuántos bytes lleva, la ventana lo
lee para dibujar la barra, y **Detener** guarda `true` en `cancel`, que
la descarga mira antes de cada lectura de hasta 64 KB.
`Ordering::Relaxed` basta, porque ningún otro dato depende de estos
números: una barra que va un frame atrasada no importa.

![Descargando: una barra de progreso, «2,5 de 6,0 MB», y lo mismo en la barra de estado (un SoundFont pequeño de prueba; GeneralUser GS ocupa unos 30 MB)](../images/step-9-soundfont-download.png)

La petición la hace `ureq`, un cliente HTTP pequeño, con los
certificados en los que confía el sistema operativo (así funciona detrás
del proxy de una empresa que tenga los suyos) y con el proxy de
`HTTPS_PROXY`, si lo hay. La variable de entorno
`FERROUS_LINEUP_SOUNDFONT_URL` da otra dirección, como una copia en tu
propia red. Un archivo de más de un gigabyte se rechaza: ningún
SoundFont General MIDI es tan grande, así que es otra cosa.

El archivo se escribe como `GeneralUser-GS.sf2.part`, luego se lee como
SoundFont, lo que comprueba que lo es, y solo entonces se renombra:

```rust
fn download_to(url: &str, path: &Path, progress: &Progress) -> Result<Arc<SoundFont>, String> {
    let part = path.with_extension("sf2.part");
    let result = download_part(url, &part, progress).and_then(|()| {
        let sound_font = soundfont::load(&part).map_err(|err| err.to_string())?;
        fs::rename(&part, path).map_err(|err| err.to_string())?;
        Ok(sound_font)
    });
    if result.is_err() {
        let _ = fs::remove_file(&part);
    }
    result
}
```

Una descarga cortada a medias, o una página web que llegó en lugar del
archivo, nunca deja un `GeneralUser-GS.sf2` roto que fallaría cada vez
que arranca la aplicación. `and_then` ejecuta el segundo paso solo si el
primero salió bien, y `let _ =` dice a propósito que no vale la pena
informar de un archivo `.part` que no se puede borrar.

Una vez por frame, `check_sounds` guarda los ajustes si cambiaron y
pregunta al hilo si ha terminado (`JoinHandle::is_finished`). Mientras
no, le pide a egui que vuelva a dibujar dentro de una décima de segundo,
para que la barra avance aunque el ratón no se mueva. Cuando termina,
`join` entrega el `Arc<SoundFont>`, y cada canal necesita un
instrumento nuevo hecho con él. La aplicación ya sabe hacerlo: cuando
algunos canales empiezan o dejan de enviarse por MIDI, `send_routes`
reconstruye los instrumentos cuya ruta cambió. Olvidar todas las rutas
hace que los reconstruya todos en el frame siguiente:

```rust
/// Has every channel of the audio thread given its instrument again,
/// on the next frame: the SoundFont was turned on or off.
pub(super) fn instruments_changed(&mut self) {
    self.routes_sent = None;
}
```

Desactivar los sonidos hace lo mismo, y cada canal recupera su Lineup
Osc o su tambor integrado. El SoundFont sigue cargado mientras los
sonidos están desactivados, así que volver a activarlos es instantáneo.

## Compilaciones de depuración

`cargo run` compila sin optimizaciones, y un sintetizador calcula cada
muestra de cada voz: sin optimizar, una banda podría ser demasiado para
el hilo de audio. Cargo puede optimizar una sola dependencia mientras el
resto sigue compilando rápido, en el `Cargo.toml` del workspace:

```toml
[profile.dev.package.rustysynth]
opt-level = 3
```

## Cómo se prueba

Los tests no deberían necesitar una descarga de 30 MB, y el repositorio
no debería guardar ninguna. Así que el módulo de tests escribe un
SoundFont byte a byte: una muestra, una onda senoidal de 440 Hz en
bucle, dos instrumentos (uno toca la muestra tal como se grabó, otro una
octava más aguda) y tres presets (el programa 0, el programa 33 y un kit
de batería en el banco 128).

Un archivo `.sf2` es un archivo RIFF, el mismo contenedor que un archivo
WAV: un árbol de *chunks*, cada uno con un nombre de cuatro letras, un
tamaño y sus bytes. Dos funciones pequeñas los construyen:

```rust
/// One chunk of a RIFF file: its name, its size and its bytes, padded
/// to an even length.
fn chunk(id: &[u8; 4], body: &[u8]) -> Vec<u8> {
    let mut bytes = id.to_vec();
    bytes.extend((body.len() as u32).to_le_bytes());
    bytes.extend(body);
    if body.len() % 2 == 1 {
        bytes.push(0);
    }
    bytes
}

/// A list of chunks: a chunk whose bytes are a type and other chunks.
fn list(kind: &[u8; 4], chunks: &[Vec<u8>]) -> Vec<u8> {
    let mut body = kind.to_vec();
    for each in chunks {
        body.extend(each);
    }
    chunk(b"LIST", &body)
}
```

El archivo es una lista de tres listas: `INFO` (una versión y un
nombre), `sdta` (las muestras, como números de 16 bits) y `pdta` (los
presets, los instrumentos y las cabeceras de las muestras, en nueve
tablas de tamaño fijo). Luego los tests lo tocan y escuchan como lo
haría un osciloscopio: una onda senoidal cruza el cero dos veces por
periodo, así que contar los cruces en un cuarto de segundo, por dos, da
su frecuencia.

```rust
#[test]
fn the_program_picks_the_instrument() {
    let mut player = SoundFontPlayer::new(&tiny_sound_font(), RATE, 33).unwrap();
    player.start(Note::new(A4), 100, HELD, None);
    let sound = render(&mut player, RATE as usize / 2);
    assert!(
        (frequency(&sound) - 880.0).abs() <= 8.0,
        "{}",
        frequency(&sound)
    );
}
```

El programa 33 es el instrumento una octava más agudo, así que el La4
sale a 880 Hz. Otros tests comprueban que un reproductor está en
silencio hasta que empieza una nota, que el la3 sale a 220 Hz, que una
nota empieza dentro de un bloque y acaba al cumplirse su duración, que
las notas más fuertes suenan más fuerte, que un tambor toca su propia
tecla sea cual sea la nota en la que está escrito, que se rechaza un
archivo que no es un SoundFont, y qué canales toma el SoundFont
(`channels_without_a_sound_in_the_sound_font_keep_theirs`).

Los tests de ventana no deben depender de si el ordenador que los
ejecuta ha descargado GeneralUser GS. Así que cada aplicación de prueba
lo busca en una ruta que nunca existe,
`keep_general_user_in(no_sound_font())`, y arranca como si la primera
reproducción ya hubiera preguntado. El único test sobre la oferta,
`the_first_play_offers_real_instrument_sounds_once`, deshace eso: pulsa
**Espacio** y comprueba que la ventana se abre mientras la canción
suena, hace clic en **Ahora no**, vuelve a reproducir, y comprueba que no
se pregunta nada, ni siquiera tras reiniciar. Luego pone un archivo
vacío donde va GeneralUser GS y comprueba que reproducir no pregunta
nada.
`a_demo_asks_for_real_instrument_sounds_in_front_of_its_chord_sheet`
comprueba que la ventana queda encima, y que sus botones reciben el
clic.
