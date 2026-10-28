# Una traza de la reproducción: cada nota que toca el reproductor, para leer

Archivos: [`crates/lineup-audio/src/trace.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/trace.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs),
[`crates/lineup-app/src/app/trace.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/trace.rs)
y [`crates/lineup-app/src/main.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/main.rs)

Hasta ahora, la única manera de estar seguro de cómo suena de verdad una
canción era exportarla y escuchar, o buscar las notas en el propio
sonido. El arreglo del cambio de métrica del paso 8 mostró por qué la
pantalla no basta: los compases se veían bien en 3/4, pero la canción
sonaba exactamente igual que antes, porque apretar las notas y ralentizar
el tempo se anulaban mutuamente. Solo el sonido decía la verdad.

Una **traza** es una lista de cada nota que toca el reproductor, escrita
por el hilo de audio en el momento en que la toca. No se vuelve a
calcular a partir del proyecto: es lo que reciben los altavoces, después
del swing, los cambios de tempo, los nudges y la temporización suelta,
con la canción desplegada como la toca el playlist y las pistas
silenciadas fuera. Así que una traza puede leerse en lugar de
escucharse, por ti o por una prueba.

## Probarlo

Desde una terminal, con un archivo de canción o el nombre de una demo:

```sh
ferrous-lineup --trace song.lineup --out song.tsv
ferrous-lineup --trace "Twelve-bar blues"
ferrous-lineup --demos
```

`--trace` toca la canción como lo hace **File > Export WAV**, de
principio a fin, pero sin ventana, tarjeta de sonido ni archivo de
sonido: obtienes las notas de la exportación, frame a frame, en uno o
dos segundos. `--out` las escribe en un archivo; sin él van a la
terminal. `--demos` lista los nombres que `--trace` entiende. Las
mayúsculas no importan, y una parte del nombre basta cuando solo una
demo lo tiene.

En la aplicación, **Settings > Playback > Trace** tiene una caja,
**Write a trace while playing**. Mientras está marcada, cada
reproducción, de Play a Stop, añade una tabla al archivo nombrado
debajo, en la carpeta donde la aplicación guarda sus ajustes. Empieza
sin marcar cada vez que la aplicación se abre, para que una traza que
olvidaste nunca llene tu disco.

En Windows, usa `--out`: el programa ya compilado es un programa con
ventana y no tiene terminal donde imprimir.

## Leer la tabla

Las líneas que empiezan con `#` dicen lo que contiene la tabla, y luego
listan el tempo y el compás de cada parte de la canción:

```text
# Ferrous Lineup playback trace: Für Elise
# sample rate: 44100 Hz
# tempo: 0.0000 s, step 0, bar 1: 144.00 BPM
# tempo: 123.7500 s, step 1188, bar 100: 216.00 BPM
# tempo: 130.0000 s, step 1278, bar 105: 144.00 BPM
# time: step 0, bar 1: 3/4
# time: step 1188, bar 100: 9/8
# time: step 1278, bar 105: 3/4
```

Luego viene una línea por cada nota, en el orden en que empiezan, con un
tab entre columnas. Estas son las primeras notas del bajo y el hi-hat en
la demo **Twelve-bar blues**, con algunas columnas fuera:

```text
start   length  bar beat into_beat step   steps bpm    name pitch note velocity late_ms end
0.0000  0.4000  1   1    0.000     0.000  2.667 100.00 Bass 36    C2   104      0.0     gate
0.0000  0.1500  1   1    0.000     0.000  1.000 100.00 Hat  60    C4   96       0.0     gate
0.4000  0.0500  1   1    2.667     2.667  0.333 100.00 Hat  60    C4   96       100.0   gate
0.4000  0.2000  1   1    2.667     2.667  1.333 100.00 Bass 40    E2   90       100.0   gate
0.6000  0.4000  1   2    0.000     4.000  2.667 100.00 Bass 43    G2   96       0.0     gate
```

El swing está ahí mismo. A 100 BPM un pulso dura 0.6 segundos y tiene
cuatro pasos. Escrita, la segunda corchea del pulso está en el paso 2, a
la mitad. Tocada, llega a 2.667 pasos dentro del pulso, 100 ms tarde:
dos tercios del camino, la tercera nota de un tresillo, que es lo que
significa el swing completo. Y la nota de bajo anterior dura 2.667 pasos
en lugar de 2, largo antes que corto.

| Columna | Qué dice |
|---|---|
| `start`, `length` | Cuándo empieza la nota y cuánto se sostiene, en segundos desde el inicio de la reproducción |
| `bar`, `beat` | Su compás y su pulso, contados desde 1, en el compás vigente |
| `into_beat` | Cuántos pasos después del inicio de su pulso empieza |
| `step`, `steps` | El paso en el que empieza (contado desde 0, con la fracción de retraso) y cuántos pasos se sostiene |
| `bpm`, `time` | El tempo de su paso y el compás de su compás |
| `channel`, `name`, `program`, `instrument` | Su fila en el channel rack (desde 1), el nombre del canal, su program de General MIDI y qué lo toca |
| `pitch`, `note` | El número de nota MIDI y su nombre, escrito en la tonalidad del momento |
| `velocity` | Con qué fuerza se toca, de 1 a 127 |
| `late_ms` | Cuánto después de su paso empieza: su nudge, el swing y la temporización suelta |
| `end` | Por qué terminó: `gate` cuando su duración se acabó, `cut` cuando se detuvo antes (Stop, un salto, su pista silenciada, su clip quitado), `again` cuando la misma nota volvió a empezar en un MIDI Out o un SoundFont, que tocan una de cada nota a la vez |
| `pattern` | Su patrón y el paso del patrón del que viene |

Las tablas como esta están hechas para `grep` y `awk`. Las notas de bajo
del compás 12, o cada nota más fuerte que 110:

```sh
ferrous-lineup --trace song.lineup | awk -F'\t' '$11 == "Bass" && $3 == 12'
ferrous-lineup --trace song.lineup | awk -F'\t' '!/^#/ && $16+0 > 110'
```

Una hoja de cálculo también abre el archivo: dile que las columnas están
separadas por tabuladores, y que las líneas que empiezan con `#` son
comentarios.

## Lo que una traza no dice

Una traza sigue las notas, no el sonido. Una muestra de batería suena
hasta el final sea cual sea la duración de la nota, el release de un
sintetizador se desvanece después de que la nota termina, y cuando todas
las voces del Lineup Osc están ocupadas se toma la más vieja para la
nota nueva. Nada de eso está en la traza. Para un problema dentro de un
instrumento, escuchar (o mirar las muestras) sigue siendo el camino.

## Notas que no deben perderse

Una traza puede perder eventos a propósito: si la ventana se queda
atrás, el ring buffer se llena, y la cabecera de la tabla dice cuántos
eventos se perdieron, porque una traza es un informe y no la música. Las
colas que llevan la música se tratan de otra manera. Los comandos
esperan espacio (`send_wait`); un mensaje MIDI que encuentra la cola de
salida llena espera en un backlog para el siguiente bloque; y un
note-off nunca se tira, para que un sintetizador externo no pueda quedar
con una nota colgada. Una grabación de más de diez minutos se rechaza al
importarla, en lugar de llenar la memoria. [Sin perder nada](recovery.md)
cuenta lo que hace la aplicación cuando algo sí sale mal.

## El código

### Escribir en el hilo de audio

El hilo de audio tiene reglas estrictas, que el paso 2 explicó: nunca
debe esperar, nunca reservar memoria, nunca escribir a un archivo.
Escribir una traza es exactamente lo que no puede hacer. Así que hace lo
que ya hace para MIDI Out: entrega mensajes pequeños a un ring buffer
del crate `rtrb`, y otro hilo, que puede tomarse su tiempo, los lee.

Los mensajes son un enum `Copy`, así que empujar uno copia unos pocos
bytes y no reserva nada:

```rust
pub enum TraceEvent {
    Play { frame: u64, step: usize, song: bool, pattern: usize },
    Jump { frame: u64, step: usize },
    Stop { frame: u64 },
    Step { frame: u64, step: usize, tempo: Tempo },
    NoteOn { frame: u64, id: u32, channel: usize, origin: Origin,
             pitch: Note, velocity: u8, gate: u32, late: u32 },
    NoteOff { frame: u64, id: u32, end: End },
}
```

`frame` es el reloj propio del hilo de audio: los frames que ha tocado
desde que arrancó. Cada nota recibe un `id`, para que su `NoteOff` pueda
emparejarse con su `NoteOn` después.

El tracer vive en el hilo de audio como un `Option<Box<Tracer>>`. Cuando
la traza está apagada es `None`, y cada lugar que traza es un
`if let Some(trace) = &mut self.trace`, que no cuesta casi nada. Se hace
en el hilo de la interfaz, donde se reserva su memoria, y viaja al hilo
de audio dentro de un comando, de la misma manera que un instrumento:

```rust
let (tracer, reader) = trace_queue();
self.send(Command::SetTrace(Some(Box::new(tracer))));
```

Un tracer viejo vuelve al hilo de la interfaz como basura, para
liberarlo allí.

Si el ring buffer está lleno, porque la ventana no lo leyó durante mucho
tiempo, el evento no se empuja y un contador de eventos perdidos sube.
La traza lo dice en su cabecera en lugar de saltarse notas en silencio.

### Dónde empiezan las notas

Una nota de la canción empieza en uno de dos lugares:
`start_pattern_notes`, cuando llega su paso, o `start_pending`, unos
milisegundos después para una nota con nudge o temporización suelta.
Ambos le avisan ahora al tracer, en el frame en que la nota empieza de
verdad, y por eso una nota con swing muestra sus 100 ms.

### Cuánto dura una nota

Cuando una nota empieza, su duración se convierte en un número de
frames, el gate. Si el tempo cambia mientras suena, cada instrumento
estira los gates de sus notas con la misma función, `stretched`, para
que la nota siga terminando en su paso. El tracer guarda su propia lista
de las notas que suenan y las estira con esa misma función. También oye
todo lo que termina notas antes: Stop, un salto, una pista silenciada,
una canción enviada mientras suena que ya no tiene un clip, un canal
quitado.

El caso de la canción cambiada muestra un pequeño detalle de Rust. El
hilo de audio comprueba cada nota que suena con una closure,
`still_played`, que toma prestada la canción. Tanto los instrumentos
como el tracer la necesitan:

```rust
for channel in &mut self.channels {
    channel.instrument.release_unless(still_played);
}
if let Some(trace) = &mut self.trace {
    trace.cut(self.now, None, still_played);
}
```

`still_played` se pasa por valor dos veces. Eso funciona porque una
closure que solo captura referencias es `Copy`, como las propias
referencias: pasarla entrega una copia y la original sigue siendo
usable.

Para encontrar las notas cuya duración terminó, el tracer tendría que
mirar cada una de ellas en cada frame, 44 100 veces por segundo. En su
lugar recuerda el final más temprano, `next_end`, y no hace nada hasta
que llega ese frame.

### Convertir eventos en notas

`Trace::notes` lee los eventos después del hecho, en un hilo normal. Los
divide en reproducciones en cada `Play`, y guarda los eventos `Step` de
cada una en una lista, en el orden en que se tocaron. Para encontrar
dónde está un frame, busca el último paso que empezó antes que él, con
`partition_point`:

```rust
let index = self.steps.partition_point(|played| played.frame <= frame);
```

`partition_point` es una búsqueda binaria: dada una lista ordenada y una
prueba que es verdadera para una primera parte y falsa para el resto,
devuelve dónde empieza la parte falsa. Aquí, el primer paso que empezó
después de `frame`; el anterior a él es el paso que se está tocando. La
fracción es cuánto está `frame` entre ese paso y el siguiente.

Cada paso también tiene una cuenta de los pasos tocados antes que él,
que sigue creciendo cuando un patrón hace loop o el playhead salta. La
duración de una nota en pasos es la diferencia de esas cuentas: se mide
a lo largo de los pasos realmente tocados, sean cuales sean los números
que el loop les da.

### La línea de comandos

`main` solía tomar su primer argumento como un archivo para abrir. Ahora
lo mira primero:

```rust
match first.as_ref().and_then(|arg| arg.to_str()) {
    Some("--trace") => std::process::exit(trace(args)),
    Some("--demos") => { /* print the names */ return Ok(()); }
    _ => {}
}
```

`trace` construye toda la aplicación sin ventana ni tarjeta de sonido,
como hacen las pruebas (`Audio::Unavailable`), abre el archivo o la demo,
y pide el mismo render de canción que hace la exportación WAV, con una
traza encendida. Luego toca todos los frames y tira el sonido. La traza
es la de la exportación, porque es el mismo camino de código.

## Pruebas que leen en lugar de escuchar

Las pruebas usan las trazas como lo haría una persona. Entre ellas:

- Cada nota que el proyecto guarda, clip a clip, está en la traza
  exactamente una vez, y todas terminan cuando su duración se acaba.
- Con swing, las únicas notas que empiezan tarde son las segundas
  corcheas de cada pulso.
- Una marca de tempo en el paso 16 cambia cuándo empiezan las notas
  posteriores, en segundos, pero no sus pasos.
- Una canción a la que se le cambió la métrica de 4/4 a 3/4 empieza cada
  compás en el mismo segundo que antes: el bug que solo el sonido podía
  mostrar es ahora una prueba.

## Ejercicios

1. Traza una demo con un ritardando (una marca de tempo que hace glide)
   y lee las líneas `# tempo`: ¿cuántas hay, y por qué una por paso?
2. Sube **Loose timing** en una canción, trázala y usa `awk` para
   encontrar la nota más tardía. ¿Llega alguna vez más tarde que el
   ajuste?
3. Con la traza encendida en la aplicación, toca un patrón, silencia su
   canal en medio de una nota larga y detente. Encuentra la nota en el
   archivo: ¿qué dice su columna `end`?
4. El tracer busca las notas cuya duración terminó al principio mismo de
   cada frame, antes de que el tempo pueda cambiar en ese frame. ¿Qué
   saldría mal con una nota que termina en el mismo frame que un cambio
   de tempo, si buscara después?
