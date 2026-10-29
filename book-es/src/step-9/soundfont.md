# Sonidos de instrumentos reales: un SoundFont

Archivos: [`crates/lineup-audio/src/soundfont.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/soundfont.rs),
[`crates/lineup-audio/src/instrument.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/instrument.rs),
[`crates/lineup-app/src/app/sounds.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/sounds.rs)
y [`crates/lineup-core/src/library/apply.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/apply.rs)

Hasta ahora, cada nota que tocaba la app salía del Lineup Osc, su
propio sintetizador: dos osciladores, un filtro y una envolvente. Dado
un instrumento de la lista de General MIDI, elige ondas y un filtro que
se le *acercan* ([Cada voz en su propio canal, con un instrumento
real](../step-8/voices-and-instruments.md)): un «piano» es una onda de
sierra golpeada y filtrada, un «violín» una sierra que crece despacio.
Puedes distinguir las partes, pero nadie las tomaría por las de verdad.

Los programas de banda de acompañamiento reproducen grabaciones de
instrumentos reales. Este cambio deja que Ferrous Lineup haga lo mismo,
con un **SoundFont**.

## Qué ves

La primera vez que pulsas ▶ (o **Espacio**), una ventana explica qué es
un SoundFont y se ofrece a descargar uno, mientras la canción sigue
sonando. Marcar **Instrumentos del SoundFont** en **Edición > Ajustes… >
Audio** también la abre:

![La primera reproducción: la canción de inicio sigue sonando, con el cursor de reproducción en el compás 7, mientras la ventana «Real instrument sounds» ofrece GeneralUser GS, unos 30 MB, guardado en ~/.local/share/ferrouslineup/soundfonts, con Download, Choose a file… y Not now.](../images/step-9-soundfont-offer.png)

**Descargar** trae GeneralUser GS, un SoundFont General MIDI libre hecho
por S. Christian Collins, desde su página de GitHub. La ventana y la
barra de estado muestran cuánto lleva; **Detener** abandona. Cuando
termina, la barra de estado dice «Los instrumentos suenan desde
GeneralUser GS», y a partir de entonces:

- todo canal con un instrumento reproduce una grabación de él. Son los
  acordes de la canción de inicio (ahora un piano), los Teclados y el Bajo
  de cada estilo de la Biblioteca (un piano y un bajo), y cada voz de cada
  demo: los violines y el violonchelo del Canon, el coro y el órgano de
  las canciones de gospel, el bandoneón de los tangos (un acordeón, en
  General MIDI);
- con **Batería del SoundFont** marcado (viene marcado por defecto), la
  batería integrada toca la batería del SoundFont: un kick, una snare y
  un hi-hat grabados;
- un canal en **Ninguno (el sonido propio del Lineup Osc)** se queda con el
  Lineup Osc. Haz clic derecho en el nombre de un canal y elige un
  instrumento bajo **Instrumento** para pasarlo al SoundFont.

![El viejo menú Sounds, con el texto al pasar el ratón de SoundFont instruments. La captura es anterior al traslado a Edit > Settings… > Audio.](../images/step-9-sounds-menu.png)

El SoundFont se descarga una sola vez. La app lo guarda en su carpeta
de datos (`~/.local/share/ferrouslineup/soundfonts` en Linux,
`~/Library/Application Support/Ferrous-Lineup/soundfonts` en macOS,
`AppData\Roaming\Ferrous Lineup\data\soundfonts` en Windows) y recuerda
las marcas de los ajustes: la próxima vez arranca con los sonidos
activados. **Elegir SoundFont…** reproduce en su lugar cualquier otro
archivo `.sf2`; **Usar GeneralUser GS** vuelve atrás.

Dos cosas más usan el SoundFont sin que se les diga: **Archivo > Exportar WAV** escribe la canción tal como la oyes, SoundFont incluido, y el
teclado de la computadora y un teclado MIDI también lo tocan. **MIDI > Enviar todos los canales** sigue enviando las notas al sintetizador externo, como
antes.

**Ahora no** cierra la ventana y se queda con el Lineup Osc. Reproducir
no vuelve a preguntar: la marca de **Edición > Ajustes… > Audio** está
ahí para cuando cambies de idea.

Si la descarga falla (sin red, un firewall), la ventana dice por qué.
Puedes descargar `GeneralUser-GS.sf2` de
[su página](https://github.com/mrbumpy409/GeneralUser-GS) tú mismo y
elegirlo con **Elegir un archivo…**.

![Descargando: una barra de progreso, «2.5 de 6.0 MB», y lo mismo en la barra de estado.](../images/step-9-soundfont-download.png)

(La captura se tomó con un SoundFont de prueba pequeño de 6 MB;
GeneralUser GS ocupa unos 30 MB.)

## Qué es un SoundFont

Un sampler reproduce una grabación a distintas velocidades
([Instrumentos: batería, sampler y
sintetizador](../step-2/instruments.md)): al doble de rápido suena una
octava más agudo. Eso funciona para unas pocas notas, pero un piano
grabado en el Do central y tocado dos octavas arriba suena a juguete: el
timbre de un instrumento real cambia a lo largo de su registro, y su
ataque se queda demasiado corto.

Un SoundFont (un archivo `.sf2`, un formato de las tarjetas de sonido
de los años 90) es la biblioteca entera de un sampler en un solo
archivo:

- **muestras**: grabaciones cortas, el Do, Mi, Sol♯ de un piano y así cada
  pocas notas, cada una con la nota en la que fue grabada. Los sonidos
  que duran, como un órgano o unas cuerdas, tienen un **bucle**: una vez
  que el ataque ha sonado, el centro de la grabación se repite mientras
  la tecla siga pulsada;
- **instrumentos**: qué muestra tocar para qué teclas y velocidades (una
  nota suave puede reproducir la grabación de una nota suave), y cómo:
  la envolvente de volumen, un filtro, vibrato;
- **presets**: lo que elige un cambio de programa MIDI, por **banco** y
  **programa**.

Un SoundFont *General MIDI* pone los 128 instrumentos General MIDI en
el banco 0, en su orden numerado, y una batería en el banco 128. Así
que el programa 0 es siempre un piano de cola y el programa 33 un bajo
digitado, y en el **canal MIDI 10** (9, contando desde 0) cada tecla es
un tambor: 36 el kick, 38 la snare, 42 el hi-hat cerrado.

Tocar una nota significa encontrar la muestra cuya región contiene esa
tecla y esa velocidad, y reproducirla más rápido o más despacio según
la proporción entre las dos notas. Cada semitono es un factor de
2^(1/12), unas 1.0595 veces, así que una muestra de La4 (440 Hz) tocada
para Si4, dos semitonos arriba, corre 2^(2/12) = 1.122 veces más
rápido: 494 Hz.

## Quién hace el trabajo: rustysynth

Leer un archivo `.sf2` y tocarlo es mucho código: parsear el archivo,
las regiones, las envolventes, los filtros, los bucles, mezclar hasta
cientos de voces. El crate `rustysynth` lo hace todo, en Rust puro sin
bibliotecas de C, bajo la licencia MIT (que un programa GPL puede
usar). Su interfaz es pequeña:

```rust
let sound_font = Arc::new(SoundFont::new(&mut file)?);
let mut synthesizer = Synthesizer::new(&sound_font, &settings)?;
synthesizer.process_midi_message(0, 0xC0, 33, 0); // channel 1: program 33
synthesizer.note_on(0, 40, 100);                  // E2, velocity 100
synthesizer.render(&mut left, &mut right);        // the next block of sound
```

Habla MIDI. Y la app ya tiene un canal que habla MIDI: el **Salida MIDI**
([el paso 7](../step-7/README.md)). Una salida MIDI lleva la cuenta de qué
notas suenan y cuándo debe terminar cada una, y reúne mensajes de
note-on, note-off y cambio de programa para un sintetizador externo.
Así que el instrumento nuevo es una salida MIDI cuyos mensajes van a un
sintetizador dentro de la app:

```rust
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
    gain: f32,
    left: [f32; BLOCK],
    right: [f32; BLOCK],
    read: usize,
}
```

El hilo de audio pide a cada instrumento una muestra cada vez. El
sintetizador trabaja en bloques de 64 frames, así que el player reparte
el bloque que tiene, y cuando el bloque se agota pasa los mensajes de
los últimos 64 frames y pide el siguiente:

```rust
pub fn next_sample(&mut self) -> f32 {
    self.notes.next_sample(); // counts frames, for note lengths
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

Una nota que empieza en mitad de un bloque empieza en el siguiente:
como mucho 64 frames tarde, 1.3 ms a 48 kHz, muy por debajo de lo que
cualquiera oye. Los sonidos del SoundFont son estéreo; cada canal de la
app es mono, así que los dos lados se suman y se dividen a la mitad.

`flush` es el mismo método que entrega los mensajes de una salida MIDI al
puerto MIDI, y `play` convierte cada `MidiMessage` en la llamada del
sintetizador:

```rust
fn play(synthesizer: &mut Synthesizer, message: MidiMessage) {
    match message {
        MidiMessage::NoteOn { channel, pitch, velocity } => synthesizer.note_on(
            i32::from(channel), i32::from(pitch.midi()), i32::from(velocity)),
        MidiMessage::NoteOff { channel, pitch } => {
            synthesizer.note_off(i32::from(channel), i32::from(pitch.midi()))
        }
        MidiMessage::ProgramChange { channel, program } => {
            synthesizer.process_midi_message(i32::from(channel), 0xC0, i32::from(program), 0)
        }
    }
}
```

Así que todo lo que una salida MIDI ya hace bien viene gratis: notas que
terminan al acabar su duración, teclas sostenidas, notas que se
detienen cuando su pista se mutea, duraciones de nota que se estiran
con un cambio de tempo.

Un tambor es una salida MIDI en el canal 10 que toca una misma nota sea
cual sea la nota que reciba (`playing_as`), exactamente como **Enviar todos los canales** ya envía la batería integrada ([Salida MIDI para cada
canal](../step-8/midi-out-every-channel.md)):

```rust
pub fn drum(sound_font: &Arc<SoundFont>, sample_rate: u32, drum: Note) -> Option<Self> {
    let notes = MidiOut::new(DRUM_CHANNEL).playing_as(Some(drum));
    Self::on(sound_font, sample_rate, notes, DRUM_GAIN)
}
```

## Un sintetizador por canal, un SoundFont para todos

Un sintetizador puede tocar dieciséis canales a la vez, así que uno
solo podría tocar toda la banda. En su lugar, cada canal de la app
recibe el suyo: así su mando de volumen, su luz de mute y su medidor
funcionan exactamente como en cualquier otro canal, porque el motor
mezcla canales, no canales MIDI.

El SoundFont en sí ocupa decenas de megabytes, y no se copia: se lee
una vez en un `Arc<SoundFont>`, un puntero *contado por referencias
atómicamente*. `Arc::clone` copia el puntero y suma uno a un contador;
la memoria se libera cuando se va el último clon. Cada sintetizador
guarda un clon, y la app también, así que el SoundFont nunca se libera
en el hilo de audio (liberar memoria allí está tan prohibido como
reservarla).

`Instrument` ganó un cuarto tipo:

```rust
pub enum Instrument {
    Sampler(Sampler),
    Synth(Synth),
    SoundFont(SoundFontPlayer),
    MidiOut(MidiOut),
}
```

y un constructor que dice qué canales toca el SoundFont:

```rust
pub fn from_sound_font(
    channel: &Channel,
    sample_rate: u32,
    sound_font: &Arc<SoundFont>,
    drums: bool,
) -> Option<Self> {
    let player = match &channel.kind {
        ChannelKind::Synth => SoundFontPlayer::new(sound_font, sample_rate, channel.program?),
        ChannelKind::Sampler(SampleSource::Builtin(drum)) if drums => {
            SoundFontPlayer::drum(sound_font, sample_rate, general_midi_drum(*drum))
        }
        _ => None,
    };
    player.map(Instrument::SoundFont)
}
```

El `?` después de `channel.program` devuelve `None` desde toda la
función cuando el canal no tiene instrumento: eso es lo que mantiene a
un canal del Lineup Osc en el Lineup Osc. Un archivo de muestra y una
salida MIDI tampoco son cosa del SoundFont. El `build_instrument` de la
app pregunta esto primero y cae a `for_channel` cuando la respuesta es
`None`.

Cada comando que el hilo de UI envía al hilo de audio viaja en un hueco
de la cola de comandos, y cada hueco es tan grande como el comando más
grande. Clippy y la prueba `commands_stay_small` detectaron ambos que
los dos bloques de 64 muestras de un sintetizador habían hecho crecer
`Instrument` de 32 bytes a 568; el `Block` metido en un `Box` lo
devuelve al tamaño de antes.

### Nada de memoria reservada en el hilo de audio

La regla de [El hilo de audio, ya crecido](../step-2/realtime.md) sigue
en pie: el hilo de audio nunca pide memoria. Un sintetizador reserva
todo lo que necesita cuando se crea: sus voces (aquí 128 por canal),
sus búferes, la tabla de preajustes. `Synthesizer::new` se ejecuta en el
hilo de UI, dentro de `build_instrument`, y el instrumento terminado se
envía al otro lado. Antes de elegir rustysynth, se leyeron su `note_on`
y su `render` para comprobar que solo reutilizan esas voces y búferes.

La reverb y el chorus están apagados: costarían tanto como las propias
voces, una vez por canal, y las grabaciones ya traen algo de ambiente.

## Lo bastante alto

El sintetizador suena bajo, a medio volumen, porque espera que toda una
banda lo comparta. El player lo sube con `GAIN` (5) y la batería con
`DRUM_GAIN` (7). Esos números salen de una medición, no del gusto: se
renderizaron las mismas notas en el Lineup Osc y en el SoundFont, y se
compararon sus picos y sus niveles medios. Un piano real se apaga
mientras el Lineup Osc sostiene sus notas, así que los dos nunca
coinciden del todo, y otro SoundFont puede sonar más alto o más bajo;
el mando de volumen de cada canal hace el resto.

## Descargar en segundo plano

`sounds.rs`, en la app, hace el resto: el menú, la ventana y la
descarga. Una descarga de 30 MB tarda de un segundo a un minuto, así
que, como la exportación a WAV ([Exportar la canción como archivo
WAV](../step-5/export.md)), corre en un hilo propio, y la UI le
pregunta cada frame si ha terminado.

El hilo y la UI comparten un `Progress`:

```rust
#[derive(Default)]
struct Progress {
    done: AtomicU64,
    total: AtomicU64,
    cancel: AtomicBool,
}
```

Los *atomics* son números que dos hilos pueden leer y escribir a la vez
sin un lock: la descarga guarda cuántos bytes lleva, la ventana los lee
para dibujar la barra, y **Detener** guarda `true` en `cancel`, que la
descarga comprueba antes de cada trozo que lee.

La petición HTTP se hace con `ureq`, un cliente HTTP pequeño, usando
los certificados en los que confía el sistema operativo (así funciona
detrás de un proxy de empresa que tenga los suyos) y el proxy de
`HTTPS_PROXY`, si hay uno. El archivo se escribe como
`GeneralUser-GS.sf2.part`, luego se lee como SoundFont, lo que
comprueba que lo sea, y solo entonces se renombra a
`GeneralUser-GS.sf2`:

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

Una descarga cortada a la mitad, o una página web que vuelve en lugar
del archivo, nunca deja un `GeneralUser-GS.sf2` roto que fallaría cada
vez que la app arranca.

Cuando el hilo termina, la app tiene un `Arc<SoundFont>`, y cada canal
necesita un instrumento nuevo hecho de él. La app ya sabe hacerlo:
cuando cambia **Enviar todos los canales**, reconstruye los instrumentos cuya
ruta cambió. Olvidar todas las rutas hace que reconstruya todos y cada
uno en el siguiente frame:

```rust
pub(super) fn instruments_changed(&mut self) {
    self.routes_sent = None;
}
```

Apagar los sonidos hace lo mismo, y cada canal recupera su Lineup Osc o
su tambor integrado.

Las marcas del menú se guardan como el zoom: en la memoria de egui, que
eframe salva junto con el tamaño de la ventana. Basta un struct
`Settings` con `#[derive(Serialize, Deserialize)]`:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
struct Settings {
    on: bool,
    drums: bool,
    file: Option<PathBuf>,
    /// Whether the first Play has offered the SoundFont: it does once.
    offered: bool,
}
```

`#[serde(default)]` deja que una versión futura añada un campo: los
ajustes guardados antes reciben el valor por defecto del campo en lugar
de fallar al cargar. `offered` llegó así: los ajustes guardados antes se
leen como «nunca preguntado», así que a quien ya tenía la app también se
le pregunta en su próxima reproducción, salvo que ya tenga los sonidos
configurados.

## Se pregunta en la primera reproducción

El Lineup Osc hace cada instrumento con unas pocas ondas simples, y no
es así como debería sonar una banda la primera vez que la oyes. Así que
la app no espera a que encuentres el ajuste: `set_playing`, por donde
pasa toda manera de empezar a reproducir (el botón ▶, **Espacio**, el
**Tocar patrón** del rack, una escucha de la Biblioteca), termina con

```rust
self.playing = true;
self.offer_sound_font_on_first_play();
```

y eso pregunta una vez:

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

`offered` se marca antes que nada, así que respondas lo que respondas,
reproducir no vuelve a preguntar. La oferta es la misma que la del
ajuste: activa los sonidos, y `prepare_sound_font` abre la ventana
porque falta el archivo; **Ahora no** los vuelve a desactivar. No se
pregunta nada cuando los sonidos ya están activados, cuando elegiste un
archivo propio, o cuando GeneralUser GS ya está en el disco: entonces
los sonidos se desactivaron a propósito. La reproducción sigue mientras
la ventana pregunta, así que una pregunta nunca retrasa el primer sonido
que oyes.

## La banda recibe instrumentos

La Biblioteca creaba sus canales **Teclados** y **Bajo** sin instrumento, así
que se quedaban con el sonido propio del Lineup Osc, y enviados a un
sintetizador MIDI tocaban lo que tuviera (a menudo un piano para
ambos). Ahora `synth_named` les da uno:

```rust
let program = match name {
    KEYS => Some(general_midi::ACOUSTIC_GRAND_PIANO),
    BASS => Some(general_midi::FINGER_BASS),
    _ => None,
};
```

Los acordes de la canción de inicio también son un piano. Con el
SoundFont apagado, el Lineup Osc los imita como imita los instrumentos
de las demos.

Un proyecto con un instrumento en cualquier canal se guarda en formato
de archivo 9 ([Cada voz en su propio canal, con un instrumento
real](../step-8/voices-and-instruments.md)), así que una canción
empezada desde un estilo de la Biblioteca necesita ahora el formato 9. Las
pruebas que comprueban qué funciones necesitan qué formato parten de la
canción de inicio sin su piano (`plain_demo`), de modo que cada una
sigue viendo solo la función que prueba.

## Probar sin un archivo SoundFont

Las pruebas no deberían necesitar una descarga de 30 MB, y el
repositorio no debería guardar una. Así que el módulo de pruebas
escribe un SoundFont byte a byte: una muestra, una onda senoidal a 440
Hz que se repite en bucle, dos instrumentos (uno toca la muestra tal como se
grabó, otro una octava más arriba) y tres preajustes (el programa 0, el
programa 33 y la batería en el banco 128).

Un archivo `.sf2` es un archivo RIFF, el mismo contenedor que un
archivo WAV: un árbol de *chunks*, cada uno con un nombre de cuatro
letras, un tamaño y sus bytes. Dos funciones pequeñas los construyen:

```rust
fn chunk(id: &[u8; 4], body: &[u8]) -> Vec<u8> {
    let mut bytes = id.to_vec();
    bytes.extend((body.len() as u32).to_le_bytes());
    bytes.extend(body);
    if body.len() % 2 == 1 {
        bytes.push(0);
    }
    bytes
}

fn list(kind: &[u8; 4], chunks: &[Vec<u8>]) -> Vec<u8> {
    let mut body = kind.to_vec();
    for each in chunks {
        body.extend(each);
    }
    chunk(b"LIST", &body)
}
```

El archivo es una lista de tres listas: `INFO` (una versión y un
nombre), `sdta` (las muestras, como números de 16 bits) y `pdta` (los preajustes, los instrumentos y las cabeceras de las muestras, en nueve
tablas de tamaño fijo). Las pruebas lo tocan luego y escuchan como lo
haría un osciloscopio: una onda senoidal cruza el cero dos veces por
periodo, así que contar los cruces en un cuarto de segundo da su
frecuencia.

```rust
#[test]
fn the_program_picks_the_instrument() {
    let mut player = SoundFontPlayer::new(&tiny_sound_font(), RATE, 33).unwrap();
    player.start(Note::new(A4), 100, HELD, None);
    let sound = render(&mut player, RATE as usize / 2);
    assert!((frequency(&sound) - 880.0).abs() <= 8.0, "{}", frequency(&sound));
}
```

El programa 33 es el instrumento una octava arriba, así que La4 sale a
880 Hz. Otras pruebas comprueban que una nota empieza dentro de un
bloque, termina al acabar su duración, suena más fuerte con la
velocidad, que un tambor toca su propia tecla esté escrita en la nota
que esté, y qué canales toma el SoundFont.

Las pruebas de UX no deben depender de si la computadora que las ejecuta
descargó GeneralUser GS. Así que cada app de prueba lo busca en una ruta
que nunca existe, `keep_general_user_in(no_sound_font())`, y arranca como
si la primera reproducción ya hubiera preguntado. La única prueba sobre
la oferta, `the_first_play_offers_real_instrument_sounds_once`, deshace
eso: pulsa **Espacio** y comprueba que la ventana se abre y los sonidos
están activados, hace clic en **Ahora no**, vuelve a reproducir, y
comprueba que no se pregunta nada, tampoco tras un reinicio. Luego pone
un archivo vacío donde va GeneralUser GS y comprueba que reproducir no
pregunta nada.

## Builds de debug

`cargo run` compila sin optimizaciones, y un sintetizador calcula cada
muestra de cada voz: sin optimizar, una banda podría ser demasiado para
el hilo de audio. Cargo puede optimizar una dependencia mientras el
resto sigue compilando rápido:

```toml
[profile.dev.package.rustysynth]
opt-level = 3
```

## Ejercicios

1. **Otro SoundFont.** Descarga otro SoundFont General MIDI en formato
   `.sf2` (*FluidR3_GM*, o el pequeño *TimGM6mb*), elígelo con
   **Elegir SoundFont…** y toca el Canon en Re. (Los archivos `.sf3`,
   que comprimen sus muestras, no están soportados.) ¿Qué cuerdas
   prefieres? ¿Suena una mucho más alta que la otra? ¿Dónde pondrías un
   ajuste de volumen para el SoundFont entero?
2. **La Melodía.** Las series dodecafónicas y los micromodos escriben su
   melodía en un canal llamado Melodía, que no tiene instrumento y se queda
   con el Lineup Osc. Elige un instrumento para él en el rack. ¿Qué
   instrumento General MIDI le darías a la Melodía por defecto, y dónde iría
   en `synth_named`?
3. **Estéreo.** Cada canal es mono, así que el piano estéreo del
   SoundFont se pliega en un lado. ¿Qué haría falta para dar a los
   canales un mando de paneo, y para mantener separados el izquierdo y
   el derecho de un SoundFont? (Mira cómo mezcla el motor `next_sample`
   en los frames de salida.)
4. **Reverberación.** Activa `enable_reverb_and_chorus` y escucha. Cuesta una
   reverb por canal; ¿cómo podrían todos los canales compartir una sola
   reverb, como hace el envío de una mesa de mezclas?
