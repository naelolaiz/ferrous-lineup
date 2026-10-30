# La traza de reproducción

Archivos: [`crates/lineup-audio/src/trace.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/trace.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs),
[`crates/lineup-app/src/app/trace.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/trace.rs)
y [`crates/lineup-app/src/main.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/main.rs)

La pantalla muestra el proyecto; no muestra lo que el reproductor hizo
con él. Una canción pasada a otro compás puede verse bien, cada compás
redibujado en 3/4, y aun así tocar sus compases al ritmo de antes,
porque las notas se apretaron y el tempo se hizo más lento, y lo uno
compensa lo otro. Solo el sonido dice la verdad. Pero escuchar es
lento, y dice que algo está mal, no qué.

Una **traza** es una lista de cada nota que toca el reproductor,
anotada por el hilo de audio en el momento en que la toca. No se
calcula de nuevo a partir del proyecto: es lo que les llega a los
altavoces, después del swing, los cambios de tempo, los desplazamientos
y la soltura, con la canción desplegada como la toca la lista de
reproducción y sin las pistas silenciadas. Así que una traza puede
leerse en lugar de escucharse, por ti o por un test. Muchos de los
tests del programa leen trazas, y también las lee quien comprueba que
una demostración toca lo que dice su partitura.

## Probarla

Desde una terminal, con un archivo de canción o el nombre de una
demostración:

```sh
ferrous-lineup --trace song.lineup --out song.tsv
ferrous-lineup --trace "Twelve-bar blues"
ferrous-lineup --demos
```

`--trace` toca la canción como lo hace **Archivo > Exportar WAV…**, de
principio a fin, pero sin ventana, sin tarjeta de sonido y sin archivo
de sonido: obtienes las notas de la exportación, frame a frame, en uno
o unos pocos segundos. `--out` las escribe en un archivo; sin él van a
la terminal. `--rate` toca a otra frecuencia de muestreo que los 44.100
de la exportación. `--demos` lista los nombres que entiende `--trace`,
que son los nombres en inglés de las demostraciones («Twelve-bar
blues» para la que la aplicación en español llama «Blues de doce
compases»). Las mayúsculas no importan, y basta con parte de un nombre
cuando solo una demostración lo tiene.

En Windows, usa `--out`: el programa ya compilado es un programa de
ventana y no tiene terminal en la que escribir.

En la aplicación, **Edición > Ajustes… > Reproducción > Registro** tiene
una casilla, **Escribir un registro mientras suena**. Mientras está
marcada, cada reproducción, de Reproducir a Detener, añade una tabla a
`playback-trace.tsv`, en la carpeta donde la aplicación guarda sus
ajustes; el ajuste muestra la ruta completa. Empieza sin marcar cada vez
que se abre la aplicación, para que una traza olvidada nunca llene el
disco. La casilla está desactivada cuando no hay ninguna tarjeta de
sonido abierta, ya que entonces no suena nada. El manual sugiere
adjuntar este archivo al informar de un problema con la reproducción.

## Leer la tabla

Las líneas que empiezan con `#` dicen qué contiene la tabla, y después
enumeran el tempo y el compás de cada parte de la canción. Estas son las
primeras de Für Elise («Para Elisa» en la aplicación en español):

```text
# Ferrous Lineup playback trace: Für Elise
# Every note the player started, in the order they start. Times are in seconds from the start of playback;
# steps are sixteenth notes of the loop played (the song, or the pattern in pattern mode), counted from 0.
# sample rate: 44100 Hz
# tempo: 0.0000 s, step 0, bar 1: 144.00 BPM
# tempo: 123.7500 s, step 1188, bar 100: 216.00 BPM
# tempo: 130.0000 s, step 1278, bar 105: 144.00 BPM
# tempo: 156.2500 s, step 1530, bar 126: 142.80 BPM
# tempo: 156.3551 s, step 1531, bar 126: 141.60 BPM
...
# time: step 0, bar 1: 3/4
# time: step 1188, bar 100: 9/8
# time: step 1278, bar 105: 3/4
```

El tempo cambia como está escrito en los compases 100 y 105, y después
baja deslizándose desde el compás 126: un ritardando, que el
reproductor toca paso a paso, cada uno un poco más lento, así que la
traza tiene una línea por cada paso.

Después viene una línea por nota, en el orden en que empiezan, con un
tabulador entre columnas. Estas son las primeras notas del bajo y del
charles en la demostración **Blues de doce compases**, sin algunas
columnas:

```text
start   length  bar beat into_beat step   steps bpm    name pitch note velocity late_ms end
0.0000  0.4000  1   1    0.000     0.000  2.667 100.00 Bass 36    C2   104      0.0     gate
0.0000  0.1500  1   1    0.000     0.000  1.000 100.00 Hat  60    C4   96       0.0     gate
0.4000  0.0500  1   1    2.667     2.667  0.333 100.00 Hat  60    C4   96       100.0   gate
0.4000  0.2000  1   1    2.667     2.667  1.333 100.00 Bass 40    E2   90       100.0   gate
0.6000  0.4000  1   2    0.000     4.000  2.667 100.00 Bass 43    G2   96       0.0     gate
0.6000  0.1500  1   2    0.000     4.000  1.000 100.00 Hat  60    C4   96       0.0     gate
```

El swing está ahí mismo. A 100 BPM un pulso dura 0,6 segundos y tiene
cuatro pasos. Escrita, la segunda corchea del pulso está en el paso 2,
a la mitad. Tocada, llega 2,667 pasos dentro del pulso, 100 ms tarde: a
dos tercios del camino, la tercera nota de un tresillo, que es lo que
significa el swing completo. Y la nota del bajo anterior dura 2,667
pasos en lugar de 2, larga antes que corta. (Cómo el swing mueve las
notas está en [Swing](tempo.md#swing).)

| Columna | Qué dice |
|---|---|
| `start`, `length` | Cuándo empieza la nota y cuánto se sostiene, en segundos desde el inicio de la reproducción |
| `bar`, `beat` | Su compás y su pulso, contados desde 1, en el compás vigente |
| `into_beat` | Cuántos pasos después del inicio de su pulso empieza |
| `step`, `steps` | El paso en que empieza (contado desde 0, con la fracción con la que llega tarde) y cuántos pasos se sostiene |
| `bpm`, `time` | El tempo de su paso y el compás de su compás |
| `channel`, `name`, `program`, `instrument` | Su fila en el rack de canales (desde 1), el nombre del canal, su programa General MIDI y qué lo toca |
| `pitch`, `note` | El número de nota MIDI y su nombre, escrito en la tonalidad del momento |
| `velocity` | Con qué fuerza se toca, de 1 a 127 |
| `late_ms` | Cuánto después de su paso empieza: su desplazamiento, el swing y la soltura |
| `end` | Por qué terminó: `gate` cuando se acabó su duración, `cut` cuando se cortó antes (Detener, un salto, su pista silenciada, su clip quitado), `again` cuando la misma nota volvió a empezar en una Salida MIDI o una SoundFont, que tocan una de cada nota a la vez |
| `pattern` | Su patrón y el paso del patrón del que viene |

Tablas así están hechas para `grep` y `awk`. Las notas del bajo del
compás 12, o cada nota más fuerte que 110:

```sh
ferrous-lineup --trace song.lineup | awk -F'\t' '$11 == "Bass" && $3 == 12'
ferrous-lineup --trace song.lineup | awk -F'\t' '!/^#/ && $16+0 > 110'
```

Una hoja de cálculo también abre el archivo: dile que las columnas
están separadas por tabuladores y que las líneas que empiezan con `#`
son comentarios. La tabla está siempre en inglés y nombra las notas con
letras, muestre lo que muestre la ventana, así que una traza se lee
igual para todos.

## Lo que una traza no dice

Una traza sigue las notas, no el sonido. Una muestra de batería suena
hasta su final sea cual sea la duración de la nota, el release de un
sintetizador sigue apagándose después de que la nota termina, y cuando
todas las voces del Lineup Osc están ocupadas, la más vieja se toma
para la nota nueva. Nada de eso está en la traza. Para un problema
dentro de un instrumento, escuchar (o mirar las muestras) sigue siendo
el camino.

## Escribir en el hilo de audio

El hilo de audio tiene [reglas estrictas](../program/threads.md#la-regla-de-oro-del-hilo-de-audio):
nunca debe esperar, nunca reservar memoria, nunca escribir en un
archivo. Escribir una traza es justo lo que no puede hacer. Así que
hace lo que ya hace con la Salida MIDI: entrega mensajes pequeños a un
ring buffer, y otro hilo, que puede tomarse su tiempo, los lee.

Los mensajes son un enum `Copy`, así que meter uno copia unos pocos
bytes y no reserva nada:

```rust
pub enum TraceEvent {
    /// Playback started on `step`: of the song, or of pattern `pattern`
    /// when `song` is false. After a count-in, when the count is over.
    Play {
        frame: u64,
        step: usize,
        song: bool,
        pattern: usize,
    },
    /// Playback jumped to `step` while playing.
    Jump { frame: u64, step: usize },
    /// Playback stopped.
    Stop { frame: u64 },
    /// A step started, at `tempo`.
    Step {
        frame: u64,
        step: usize,
        tempo: Tempo,
    },
    /// A note started on channel `channel` (its row in the channel rack),
    /// `late` frames after its step (nudge and loose timing), to be held
    /// for `gate` frames. `id` pairs it with its [`TraceEvent::NoteOff`].
    NoteOn {
        frame: u64,
        id: u32,
        channel: usize,
        origin: Origin,
        pitch: Note,
        velocity: u8,
        gate: u32,
        late: u32,
    },
    /// The note `id` ended, as `end` says.
    NoteOff { frame: u64, id: u32, end: End },
}
```

`frame` es el reloj propio del hilo de audio: los frames que ha tocado
desde que arrancó. Cada nota recibe un `id`, para poder emparejar
después su `NoteOff` con su `NoteOn`. Los ids cuentan hacia arriba con
`wrapping_add(1)`, que vuelve a 0 después de `u32::MAX` en lugar de
entrar en pánico en una compilación de depuración; cuatro mil millones
de notas después, las primeras ya terminaron hace mucho.

El hilo de audio guarda el trazador como un `Option<Box<Tracer>>`.
Cuando la traza está apagada es `None`, y cada lugar que anota es un
`if let Some(trace) = &mut self.trace`, que no cuesta casi nada. El
trazador se crea en otro hilo, donde se puede reservar su memoria:

```rust
/// A tracer for the audio thread and the reader of what it writes.
pub fn trace_queue() -> (Tracer, TraceReader) {
    let (events, consumer) = rtrb::RingBuffer::new(TRACE_QUEUE_SIZE);
    let lost = Arc::new(AtomicUsize::new(0));
    let tracer = Tracer {
        events,
        lost: Arc::clone(&lost),
        open: Vec::with_capacity(OPEN_NOTES),
        next_id: 0,
        next_end: u64::MAX,
    };
    let reader = TraceReader {
        events: consumer,
        lost,
    };
    (tracer, reader)
}
```

y viaja al hilo de audio dentro de un comando, igual que un
instrumento: `Command::SetTrace(Some(Box::new(tracer)))`. La cola
guarda 65.536 eventos (`1 << 16`), y la lista de notas que suenan se
reserva para 4.096, así que nada crece en el hilo de audio. Un trazador
viejo vuelve al hilo de la interfaz como basura, para liberarse allí.

### Perdidos a propósito

Si el ring buffer está lleno, porque nadie lo leyó durante mucho
tiempo, el evento no se mete y sube un contador de eventos perdidos:

```rust
fn push(&mut self, event: TraceEvent) {
    if self.events.push(event).is_err() {
        self.lost.fetch_add(1, Ordering::Relaxed);
    }
}
```

La cabecera de la tabla dice entonces cuántos eventos se perdieron, en
lugar de que falten notas en silencio. Las colas que llevan la
*música* hacen lo contrario: un note-off MIDI que encuentra su cola
llena espera a que haya sitio, porque uno perdido dejaría una nota
sonando para siempre en un sintetizador (ver
[Cuando la tarjeta de sonido desaparece](../program/threads.md#cuando-la-tarjeta-de-sonido-desaparece)).
Una traza es un informe, no la música. Hacer esperar al hilo de audio
por ella pondría el informe antes que el sonido.

### Dónde empiezan las notas

Una nota de la canción empieza en uno de dos lugares: en
[`start_pattern_notes`](sequencer.md), cuando llega su paso,
o en `start_pending`, unos milisegundos después, para una nota
desplazada o con soltura. Ambos se lo dicen al trazador en el frame en
que la nota empieza de verdad, por eso una nota con swing muestra sus
100 ms:

```rust
if let Some(trace) = &mut self.trace {
    trace.note_on(
        self.now,
        note.channel,
        note.origin,
        note.pitch,
        note.velocity,
        note.gate,
        note.late,
        one_per_pitch(&channel.instrument),
    );
}
```

`one_per_pitch` dice si el instrumento es una Salida MIDI o una
SoundFont, que tocan una de cada nota a la vez: en esos, una nota que
vuelve a empezar termina la que suena, y la traza escribe `again` para
ella.

### Cuánto dura una nota

Cuando una nota empieza, su duración se convierte en un número de
frames, el *gate*. Si el tempo cambia mientras suena, cada instrumento
estira los gates de sus notas con la misma función, `stretched`, para
que la nota siga terminando en su paso (ver
[Cada paso a su propio tempo](tempo.md#cada-paso-a-su-propio-tempo)).
El trazador lleva su propia lista de las notas que suenan, `open`, y
las estira con esa misma función:

```rust
pub(crate) fn tempo_changed(&mut self, now: u64, next_step: f64, stretch: f64) {
    for note in &mut self.open {
        if note.end != u64::MAX {
            let left = note.end.saturating_sub(now) as f64;
            note.end = now + stretched(left, next_step, stretch) as u64;
        }
    }
    self.find_next_end();
}
```

Una nota con el gate que nunca se acaba, `HELD` (`u32::MAX`), no tiene
un final que el trazador pueda saber, y lo guarda como `u64::MAX`.

El trazador también se entera de todo lo que termina notas antes de
tiempo: Detener, un salto, un silenciado, una canción enviada mientras
suena que ya no tiene un clip, un canal quitado. El caso de la canción
cambiada muestra un pequeño detalle de Rust. El hilo de audio comprueba
cada nota que suena con un closure, `still_played`, que toma prestada
la canción. Lo necesitan tanto los instrumentos como el trazador:

```rust
for channel in &mut self.channels {
    channel.instrument.release_unless(still_played);
}
if let Some(trace) = &mut self.trace {
    trace.cut(self.now, None, still_played);
}
```

`still_played` se pasa por valor, una vez por cada canal y una vez más.
Eso funciona porque un closure que solo captura referencias compartidas
es `Copy`, como las propias referencias: pasarlo entrega una copia y el
original sigue usable.

### Solo cuando una nota termina

Para encontrar las notas cuya duración se acabó, el trazador tendría
que mirar cada una en cada frame, 44.100 veces por segundo. En cambio,
recuerda el final más temprano, `next_end`, y no hace nada hasta que
llega ese frame:

```rust
/// Ends the notes whose length is over by frame `now`. Called once
/// a frame, before anything else happens on it.
pub(crate) fn frame(&mut self, now: u64) {
    if now < self.next_end {
        return;
    }
    // The fields are borrowed one by one: `open` to go through, the
    // queue and the counter to write to.
    let events = &mut self.events;
    let lost = &self.lost;
    self.open.retain(|note| {
        if note.end > now {
            return true;
        }
        let event = TraceEvent::NoteOff {
            frame: note.end,
            id: note.id,
            end: End::Gate,
        };
        if events.push(event).is_err() {
            lost.fetch_add(1, Ordering::Relaxed);
        }
        false
    });
    self.find_next_end();
}
```

`retain` conserva los elementos para los que el closure devuelve `true`
y suelta los demás, en una sola pasada y en el mismo sitio. El closure
necesita meter eventos en la cola mientras `retain` tiene `self.open`
prestado como mutable. Llamar a `self.push` dentro de él tomaría
prestado todo `self` por segunda vez, y el compilador lo rechaza. Tomar
prestados antes los otros dos campos en variables locales (`events`,
`lost`) da tres préstamos de tres campos distintos, que no se solapan.

## Convertir los eventos en notas

`Trace::notes` lee los eventos después, en un hilo normal. Los divide
en reproducciones (*runs*) en cada `Play`, y guarda los eventos `Step`
de cada una en una lista, en el orden en que se tocaron. Para saber
dónde está un frame, busca el último paso que empezó antes, con
`partition_point`:

```rust
fn position(&self, frame: u64, sample_rate: u32) -> Option<(f64, f64, Tempo)> {
    let index = self.steps.partition_point(|played| played.frame <= frame);
    let played = self.steps.get(index.checked_sub(1)?)?;
    let length = match self.steps.get(index) {
        Some(next) => (next.frame - played.frame) as f64,
        None => frames_per_step(played.tempo, sample_rate),
    };
    let fraction = (frame - played.frame) as f64 / length.max(1.0);
    Some((
        played.step as f64 + fraction,
        played.count as f64 + fraction,
        played.tempo,
    ))
}
```

`partition_point` es una búsqueda binaria: dada una lista ordenada y
una prueba que es verdadera para una primera parte y falsa para el
resto, devuelve dónde empieza la parte falsa. Aquí es el primer paso
que empezó después de `frame`; el anterior es el paso que se está
tocando. La fracción es cuánto ha avanzado `frame` entre ese paso y el
siguiente. `index.checked_sub(1)?` devuelve `None` para un frame
anterior al primer paso, donde `index - 1` daría la vuelta por debajo
de cero.

Cada paso tiene además un `count` de los pasos tocados antes que él,
que sigue creciendo cuando un patrón vuelve a empezar o el cursor
salta. La duración de una nota en pasos es la diferencia de esos
contadores: se mide a lo largo de los pasos tocados de verdad, sean
cuales sean los números que les da el bucle. Una nota sostenida a
través del final de un bucle de un compás, del paso 14 al paso 2 de la
vuelta siguiente, dura 4 pasos, no -12.

Un `NoteOff` encuentra su `NoteOn` por id en una pequeña lista de las
notas aún abiertas en la reproducción. `swap_remove` lo saca moviendo
el último elemento a su lugar, lo que es rápido y aquí está bien, ya
que el orden de esa lista no importa.

## Escribir la tabla

`write_table` escribe la cabecera y una línea por nota. Los nombres
vienen de `Labels`: el título de la canción, el proyecto (para los
nombres de los canales y patrones), los compases y las tonalidades,
todo en los pasos del bucle que se tocó:

```rust
/// For a trace of song mode playing `arrangement`.
pub fn song(title: &'a str, project: &'a Project, arrangement: &Arrangement) -> Self {
    Labels {
        title,
        project,
        meter: project.played_meter(arrangement),
        keys: project.played_keys(arrangement),
    }
}
```

`played_meter` y `played_keys` despliegan los compases y las
tonalidades igual que el arreglo despliega los clips, así que una
sección en 3/4 que suena dos veces está en 3/4 las dos veces. Cada nota
se escribe en la tonalidad de su paso: un Fa♯ en Sol mayor, un Sol♭ en
Re♭ mayor (en la tabla, siempre con letras: F♯, G♭).

El nombre de un canal o de un patrón podría contener un tabulador o un
salto de línea, que partirían la tabla. `cell` los cambia por espacios:

```rust
/// Text for a table cell: no tabs or line breaks, which would split it.
fn cell(text: &str) -> String {
    text.replace(['\t', '\n', '\r'], " ")
}
```

`str::replace` recibe un *patrón*, y un array de caracteres lo es:
coincide con cualquiera de ellos.

Una nota que seguía sonando cuando terminó la traza no tiene duración;
la tabla escribe `?` para ella, y la cabecera dice cuántas hay.

## Una traza mientras tocas

La traza del ajuste es un `LiveTrace`, que guarda la aplicación. Una
vez por frame, `check_trace` mueve a él lo que entregó el hilo de
audio. Cada reproducción se escribe como una tabla propia cuando
termina, al Detener o cuando empieza otra (cambiar entre el modo
canción y el modo patrón mientras suena empieza otra).
`Trace::take_finished` saca las reproducciones terminadas y deja la que
sigue en marcha.

Una tabla necesita los nombres del proyecto, pero puedes renombrar un
canal mientras suena una canción. Así que en cada `Play`, `check_trace`
guarda una copia del proyecto tal como estaba, y la tabla se escribe
con los nombres que tenía esa reproducción:

```rust
for event in &live.trace.events[before..] {
    if let TraceEvent::Play { song, pattern, .. } = *event {
        live.playing.push_back(Playing {
            title: self
                .file
                .song_name()
                .unwrap_or_else(|| "Untitled".to_string()),
            project: self.project.clone(),
            song,
            pattern,
        });
    }
}
```

Un `VecDeque` es una cola que puede añadir por un extremo y sacar por
el otro: las copias entran por detrás a medida que empiezan las
reproducciones, y salen por delante a medida que se escriben sus
tablas.

La ventana muestra los nombres de las notas en letras o en solfeo y las
palabras en inglés o en español, pero la tabla no debe cambiar con
ellos. Así que `write` fija ambos mientras escribe, con las funciones
`with` de los
[ajustes por hilo](../program/languages.md#dos-ajustes-guardados-como-el-de-los-números-romanos):

```rust
let mut table = Vec::new();
lineup_core::i18n::Language::English.with(|| {
    lineup_core::NoteNaming::Letters.with(|| write_table(&mut table, trace, &labels))
})?;
```

Desmarcar la casilla, o cerrar la aplicación, suelta el `LiveTrace`. Su
`Drop` escribe la reproducción que sigue en marcha, para que no se
pierdan las últimas notas. Si la tarjeta de sonido desaparece y vuelve,
`resume_trace` escribe lo que dejó el hilo de audio viejo y le da un
trazador nuevo al nuevo, y la traza sigue en el mismo archivo.

## La línea de comandos

`main` mira su primer argumento antes de abrir una ventana (ver
[`main.rs`](../program/egui.md#mainrs)). Después de `--trace`, un
pequeño bucle lee el resto:

```rust
while let Some(arg) = args.next() {
    match arg.to_str() {
        Some("--out" | "-o") => out = args.next().map(PathBuf::from),
        Some("--rate") => match args.next().and_then(|hz| hz.to_str()?.parse().ok()) {
            Some(hz) if (8_000..=192_000).contains(&hz) => rate = hz,
            _ => {
                eprintln!("--rate takes a sample rate in Hz, such as 44100");
                return 2;
            }
        },
        _ if source.is_none() => source = Some(arg),
        _ => {
            eprintln!("{USAGE}");
            return 2;
        }
    }
}
```

`args` es un iterador, y el bucle le saca elementos de dos maneras: uno
por vuelta con `while let`, y uno más con `args.next()` dentro del
cuerpo, para el valor de una opción. Los brazos muestran tres tipos de
patrón: `"--out" | "-o"` coincide con cualquiera de los dos textos,
`Some(hz) if ...` añade una guarda, y `_ if source.is_none()` coincide
con cualquier cosa, pero solo mientras no se haya dado una canción. La
función devuelve el código de salida con el que termina el programa: 0
cuando se escribió la traza, 1 cuando no se pudo, 2 por un error en los
argumentos. Los scripts lo comprueban.

`trace_table` construye la aplicación entera sin ventana ni tarjeta de
sonido, como hacen los tests (`Audio::Unavailable`), abre el archivo o
la demostración, y pide el mismo renderizado de la canción que hace la
exportación WAV, con una traza activada:

```rust
pub(super) fn trace_song(&mut self, sample_rate: u32) -> Trace {
    let mut render = self.song_render_traced(sample_rate);
    render.renderer.skip(render.frames);
    render.renderer.take_trace().unwrap_or_default()
}
```

`skip` toca todos los frames y tira el sonido. La traza es la de la
exportación, porque es el mismo camino de código (ver
[Exportar](export.md)).

El nombre de una demostración se busca en tres intentos: exacto,
después sin mirar las mayúsculas, después como parte de un nombre. El
último intento puede encontrar ninguna, una o varias, y un patrón de
slice dice cuál es el caso:

```rust
match matching.as_slice() {
    [demo] => Ok(*demo),
    [] => Err(format!(
        "There is no file or demo called \"{name}\" (--demos lists the demos)."
    )),
    several => {
        let names: Vec<&str> = several.iter().map(|demo| demo.name()).collect();
        Err(format!("\"{name}\" could be any of: {}.", names.join(", ")))
    }
}
```

## Tests que leen en lugar de escuchar

Los tests usan las trazas como lo haría una persona. En `lineup-audio`,
sobre un renderer pequeño:

- `notes_start_on_their_step_and_last_their_length`, y
  `a_nudged_note_starts_late_by_its_nudge`.
- `a_slower_tempo_stretches_a_held_note`: una nota que suena a través
  de un cambio a la mitad del tempo dura sus pasos, en más segundos.
- `stopping_and_muting_cut_the_notes_sounding`, y
  `a_midi_out_ends_a_note_that_starts_again`.
- `a_looping_pattern_counts_its_steps_on`: las duraciones se miden a lo
  largo de los pasos tocados, a través del bucle.

Y en la aplicación, sobre canciones enteras:

- `the_trace_plays_every_note_of_the_export`: cada nota que tiene el
  proyecto, clip a clip, está en la traza exactamente una vez, y todas
  terminan cuando se acaba su duración.
- `swing_plays_the_off_beats_late`: con swing, las únicas notas que
  empiezan tarde son las segundas corcheas de cada pulso.
- `a_tempo_marker_changes_the_seconds_not_the_steps`: una marca de
  tempo en el paso 16 cambia cuándo empiezan las notas que vienen
  después, en segundos, pero no sus pasos.
- `re_metering_keeps_each_bar_as_long_in_seconds`: una canción pasada
  de 4/4 a 3/4 empieza cada compás en el mismo segundo que antes. Es el
  error del principio de este capítulo, el que solo el sonido podía
  mostrar, escrito como un test.
- `a_trace_while_playing_writes_a_table_for_each_playback`.

Los tests de otros capítulos también usan trazas, como el que lleva El
Choclo por siete estilos y comprueba que cada nota de la melodía sigue
empezando y durando como antes ([Tempo, compás, swing y
sensación](tempo.md#cómo-se-prueba)).

Los ejercicios sobre trazas están en
[Ejercicios: el sonido](exercises.md).
