# Enviar un canal por MIDI desde su menú de instrumento, también la batería

Archivos: [`crates/lineup-app/src/channel_rack.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/channel_rack.rs),
[`crates/lineup-app/src/app/mixer.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/mixer.rs),
[`crates/lineup-app/src/app/live.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/live.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs),
[`crates/lineup-core/src/general_midi.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/general_midi.rs),
[`crates/lineup-core/src/midi_file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/midi_file.rs)
y [`crates/lineup-audio/src/instrument.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/instrument.rs)

[Salida MIDI para cada canal](../step-8/midi-out-every-channel.md) dio al
menú MIDI un interruptor, **Enviar todos los canales**: la canción entera va
a un sintetizador externo, o no va nada de ella. A menudo quieres algo
intermedio. El bajo en un sintetizador de hardware y el resto de la
banda en la app, o la batería en una caja de ritmos. Cada una de esas
cosas solía significar añadir un canal de salida MIDI en el rack y mover las
notas a él.

Ahora el lugar donde eliges qué toca un canal es también donde lo
envías. El **menú de instrumento**, en el mezclador y en el rack de canales,
empieza con **Enviar por MIDI**. Elige un canal MIDI del 1 al 16 y la
salida, y ese canal solo va al sintetizador. La batería integrada tiene
el mismo menú.

![El menú de instrumento de las Keys en el mixer: Send over MIDI, con sus dieciséis canales MIDI y la salida (esta máquina no tiene ninguna), encima de los instrumentos de General MIDI.](../images/step-9-midi-out-menu.png)

## Probarlo

1. Conecta un sintetizador, o en Linux arranca uno por software como
   FluidSynth o Qsynth. Sin uno, puedes seguir los menús igualmente;
   las notas simplemente no van a ninguna parte.
2. Abre una demo y el mezclador (**F9**). Haz clic en el instrumento de un
   canal, por ejemplo «Piano de cola acústico», luego en **Enviar por MIDI**, luego en **3**. Abre el menú otra vez y, bajo **Salida, para todos los canales enviados por MIDI**, elige tu sintetizador. Es la misma
   salida que en el menú MIDI.
3. El mezclador dice ahora **Piano de cola acústico · MIDI 3**. Reproduce:
   ese canal suena en el sintetizador, como un piano, ya que su
   instrumento sale como un *program change*. Todo lo demás suena en la
   app.
4. Haz clic en el **Percusión incorporada** de un tambor, luego en **Enviar por MIDI** y en **10**. El mezclador dice **Bombo 1 · MIDI 10** para el
   kick: la nota del set de batería de General MIDI como la que se
   envía.
5. Haz clic de nuevo en el instrumento del canal y elige **Tocarlo aquí en su lugar** para recuperar el propio sonido de la app.

En el rack de canales el mismo menú está en el menú de clic derecho de un
canal: bajo **Instrumento** para un sintetizador, directamente para un
tambor.

## Qué viaja por el cable

Un cable MIDI (o uno virtual entre dos programas) transporta 16
*canales*. Cada canal de un sintetizador toca un instrumento a la vez,
y el mensaje de program change dice cuál. En los menús se numeran del 1
al 16, como en todo sintetizador; en el código, como en los propios
mensajes MIDI, son del 0 al 15. El menú convierte uno en el otro con un
`+ 1` cuando escribe la etiqueta.

El **canal 10** es especial: los sintetizadores General MIDI guardan
ahí su set de batería. En él, cada *número de nota* es un tambor
distinto, del 35 al 81:

```text
35 Bombo acústico    36 Bombo 1          38 Caja acústica
39 Aplausos          42 Hi-Hat cerrado   46 Hi-Hat abierto
49 Platillo crash 1  51 Platillo ride 1  56 Cencerro   …
```

Un tambor en la app es un canal que toca un sonido, y todos sus pasos
se escriben en una sola nota, Do4. Así que un tambor enviado por MIDI
envía **su propia nota** del set de batería, sea cual sea la nota en la
que están escritos sus pasos: el kick envía siempre 36, el clap 39.
Puedes enviarlo por cualquier canal, no solo el 10, para una caja de
ritmos que escuche en otro sitio.

El mezclador nombra el tambor que envía, a partir de la tabla de General
MIDI en `general_midi.rs`:

```rust
/// The drum General MIDI plays for `note` on channel 10, such as "Bass
/// Drum 1" for 36, if the drum set has one there.
pub fn drum_name(note: u8) -> Option<&'static str> {
    DRUMS
        .get(usize::from(note.checked_sub(FIRST_DRUM)?))
        .copied()
}
```

Aquí se encuentran dos clases de «quizá». `checked_sub` devuelve `None`
en lugar de dar la vuelta cuando la nota está por debajo de 35 (un `u8`
no puede bajar de 0), y el `?` devuelve ese `None` de la función al
momento. `get` devuelve `None` pasado el final de la tabla, para notas
por encima de 81. `copied` convierte el `Option<&&str>` que da `get`
en un `Option<&str>`.

## Un canal que cambia de tipo, y un tambor que no

Un sintetizador enviado por MIDI **se convierte en** un canal de salida MIDI,
conservando su instrumento:

```rust
pub fn send_over_midi(&mut self, midi_channel: u8) {
    let midi_channel = midi_channel.min(15);
    match self.kind {
        ChannelKind::Sampler(SampleSource::Builtin(_)) => {
            self.sent_on = Some(midi_channel);
            return;
        }
        ChannelKind::Sampler(SampleSource::File(_)) => return,
        _ => {}
    }
    let plain = match self.kind {
        ChannelKind::Synth => self.name == Channel::synth().name,
        ChannelKind::MidiOut { channel } => self.name == Channel::midi_out(channel).name,
        _ => false,
    };
    if plain {
        self.name = Channel::midi_out(midi_channel).name;
    }
    self.kind = ChannelKind::MidiOut {
        channel: midi_channel,
    };
}
```

Un nombre que solo decía qué lo tocaba, «Lineup Osc» o «Salida MIDI 2»,
sigue al cambio y se vuelve «Salida MIDI 3». Un nombre que tú le diste,
como «Bajo», se queda. `play_here` hace lo contrario.

Un **tambor** no cambia de tipo. Recibe un campo nuevo, `sent_on`, con
el canal MIDI, y sigue siendo un `Sampler` que toca su tambor
integrado. Eso le importa a la banda: los estilos encuentran el kick,
la caja y los hats por lo que son (`Sampler(Builtin(Drum::Kick))`),
para escribir sus ritmos sobre ellos. Un kick convertido en canal de salida MIDI ya no se encontraría, y el siguiente estilo añadiría un segundo
kick. Por la misma razón, los estilos encuentran ahora un «Bajo» o unos «Teclados» enviados por MIDI como el bajo y los teclados: antes, aplicar un
estilo después de enviar el bajo añadía un segundo canal Bajo.

En el hilo de audio, `Instrument::for_channel` construye lo que toca
cada canal. Para un tambor, ahora mira `sent_on`:

```rust
ChannelKind::Sampler(SampleSource::Builtin(drum)) => match channel.sent_on {
    Some(number) => Instrument::MidiOut(
        MidiOut::new(number).playing_as(Some(general_midi_drum(*drum))),
    ),
    None => {
        Instrument::Sampler(Sampler::new(Arc::new(drums::render(*drum, sample_rate))))
    }
},
```

`playing_as` hace que la salida MIDI envíe esa única nota por cada nota
que se le pida tocar. Así que la parte del tambor, la banda y los
estilos se quedan exactamente como estaban; solo el sonido va a otra
parte.

**Archivo > Exportar canción como MIDI** pone un tambor enviado por MIDI en su
propio canal, y da a los demás canales los libres de alrededor, y el
[trace de reproducción](playback-trace.md) lo nombra. Una canción con
un tambor enviado por MIDI necesita al menos el formato 27 ([el formato
del archivo de proyecto](../appendix-project-format.md)). Una canción
sin uno se guarda en el formato que tenía, ya que `sent_on` se deja
fuera del archivo cuando es `None`.

## El menú necesita la lista de salidas

Los menús de instrumento los dibujan funciones de `channel_rack.rs`,
que conocen un canal y nada de las conexiones MIDI de la app. Pasar
esas conexiones hacia abajo por el rack, el mezclador y el style editor
habría añadido un parámetro a muchas funciones. En su lugar, la app
pone la lista de salidas en la memoria de egui una vez por frame, y el
menú la lee allí:

```rust
/// Tells the instrument menus which MIDI outputs there are, once a
/// frame: they offer them under "Send over MIDI".
pub(super) fn share_midi_outputs(&self, ctx: &egui::Context) {
    channel_rack::set_midi_outputs(
        ctx,
        channel_rack::MidiOutputs {
            ports: self.live.midi.outputs().to_vec(),
            connected: self.live.midi.output().cloned(),
        },
    );
}
```

`set_midi_outputs` la guarda con `insert_temp` bajo un `Id` fijo, y el
menú la recupera con `get_temp`. La memoria de egui puede contener un
valor de cualquier tipo que sea `Clone`, `Send`, `Sync` y `'static`,
uno por `Id` y tipo. Los valores guardados con `insert_temp` no se
guardan cuando la app se cierra, lo que es correcto para una lista de
puertos que mañana puede ser distinta.

## Una prueba que pasaba en todas partes menos en Windows

La primera versión de la prueba
`the_mixer_sends_a_channel_over_midi_and_back` abría **Enviar por MIDI**
y esperaba que el menú dijera «No outputs found». En las máquinas de CI
de Linux y macOS no hay salida MIDI, así que pasaba. En Windows
fallaba: Windows siempre tiene una, su integrado «Microsoft GS
Wavetable Synth».

Una prueba que depende de lo que la máquina tiene necesita
preguntárselo a la máquina. La prueba corregida lee las salidas que
encontró la app y comprueba el menú contra ellas: lista cada una, y
dice «No outputs found» solo cuando no hay ninguna. El arreglo fue un
pull request propio. Es la clase de diferencia que el CI en tres
sistemas está para atrapar.

## Las pruebas

En `project.rs`, `a_channel_goes_over_midi_with_its_instrument` y
`a_channel_sent_over_midi_can_play_here_again` comprueban los nombres y
los tipos, y `a_drum_goes_over_midi_as_the_drum_it_is` que un tambor
conserva su tipo y su nombre. En `instrument.rs`,
`a_drum_channel_sent_over_midi_plays_its_general_midi_note_there`
envía el clap por el canal MIDI 4 y comprueba que no suena en la app y
que envía allí la nota 39.
`a_drum_sent_over_midi_keeps_its_channel_and_its_note` exporta un
archivo MIDI con él, y
`drums_sent_over_midi_need_format_twenty_seven`, en `file.rs`, guarda
uno. En la app, `the_mixer_and_the_rack_send_a_drum_over_midi_and_back`
lo hace con clics: el kick desde el mezclador, la caja desde el menú de
clic derecho del rack.

## Ejercicios

1. Envía el Bajo de una demo de banda por MIDI en el canal 2, y luego
   aplica otro estilo de la Biblioteca. ¿Cuántos canales Bajo hay después?
   Lee `find_synth` en `library/apply.rs` para ver por qué.
2. ¿Qué tambor de General MIDI es la nota 60? Un tambor escrito en Do4 y
   enviado tal como está escrito, en lugar de como su propia nota, lo
   tocaría. Encuéntralo con `drum_name`.
3. `send_over_midi` limita el canal con `.min(15)` en lugar de devolver
   un error. ¿Dónde más en la app se limita así un número de un menú o
   de un archivo, y dónde se rechaza en su lugar?
