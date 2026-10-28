# Cada voz en su propio canal, con un instrumento real

Archivos: [`crates/lineup-core/src/general_midi.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/general_midi.rs),
[`crates/lineup-audio/src/timbre.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/timbre.rs),
[`crates/lineup-core/src/library/song.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song.rs)
y [`crates/lineup-core/src/library/demo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/demo.rs)

Abre **Canon in D** antes de este cambio y mira el channel rack: un
canal Lead tiene los tres violines. Pachelbel escribió tres partes de
violín que tocan la misma línea, cada una dos compases por detrás de la
anterior; en el rack eran una maraña de notas, y no podías silenciar el
segundo violín para oír qué hace el primero. Frère Jacques, Row, Row,
Row Your Boat y Three Blind Mice, los cánones, tenían el mismo problema.

Y cada canal de cada demo tocaba el mismo sonido: la sierra del Lineup
Osc. Un violín, un coro y un bajo zumbaban todos igual. Enviado a un
sintetizador MIDI, todo salía como un piano, porque nada le decía al
sintetizador otra cosa.

Ahora cada voz tiene un canal, y cada canal tiene un instrumento.

## Lo que ves

Abre **File > Demos > Songs > Classical and film > Canon in D**. El rack
tiene **Lead**, **Voice 2** y **Voice 3**, los tres violines, luego
**Keys** (el clavecín, que toca los acordes) y **Bass** (el
violonchelo, que toca el bajo ostinato). En el patrón **Entries**, Voice
2 empieza dos compases después de Lead, y Voice 3 dos compases después
de eso. Silencia cualquiera con su luz para oír el canon sin ella.

Haz clic derecho en el nombre de un canal: **Instrument** muestra el que
toca, y su submenú tiene los 128 instrumentos de General MIDI, ordenados
en sus dieciséis familias.

![El menú de instrumento: la Voice 2 del Canon toca un violín](../images/step-8-instrument-menu.png)

Pasa el ratón por encima del nombre de un canal para ver también su
instrumento.

## General MIDI: 128 instrumentos por número

Un cable MIDI o un archivo MIDI lleva notas, no sonidos. Para decir qué
sonido debe tocarlas, MIDI tiene el mensaje de *program change*: «de
ahora en adelante, el canal 3 toca el programa 40». Dos bytes: `0xC2`
(program change, en el canal 3, contado desde 0) y `40`.

Solo, 40 no significa nada. En 1991, *General MIDI* fijó una lista: el
programa 40 es un violín, el 0 un piano de cola, el 52 un coro, el 73
una flauta, en todo sintetizador que la siga, de un teléfono a un
teclado de escenario. El canal 10 está reservado para la batería. Los
programas vienen en familias de ocho: pianos del 0 al 7, órganos del 16
al 23, guitarras del 24 al 31, bajos del 32 al 39, cuerdas del 40 al
47, y así. Los manuales cuentan desde 1 («41 Violin»), los bytes desde
0; el menú muestra los números del manual.

`general_midi.rs` guarda los nombres y los números que usan las demos:

```rust
pub const VIOLIN: u8 = 40;
pub const CELLO: u8 = 42;
pub const CHOIR_AAHS: u8 = 52;

pub fn name(program: u8) -> &'static str {
    NAMES[usize::from(program.min(LAST))]
}
```

## El instrumento de un canal

`Channel` ganó un campo:

```rust
#[serde(default, skip_serializing_if = "Option::is_none")]
pub program: Option<u8>,
```

`Option<u8>` porque un canal puede no tener ningún instrumento en
particular: entonces no se envía nada a un sintetizador, y el Lineup Osc
conserva su propio sonido. Los dos atributos de serde mantienen legibles
los archivos viejos: un archivo sin el campo se lee como `None`
(`default`), y un canal sin instrumento no escribe ningún campo
(`skip_serializing_if`). Solo un proyecto que tiene instrumentos se
guarda en el formato de archivo nuevo, así que las versiones antiguas
de Ferrous Lineup siguen abriendo todo lo demás.

El instrumento se usa en tres sitios:

1. **La exportación MIDI** escribe un program change al principio mismo
   de la pista de cada canal, antes de su primera nota. La batería no
   recibe ninguno: en el canal 10 las notas eligen la batería.
2. **La salida MIDI** lo envía justo antes de la primera nota del canal,
   y otra vez después de cada parada, por si cambiaste el sonido en el
   sintetizador entre medias. El instrumento `MidiOut` guarda un flag
   para eso:

   ```rust
   fn start(&mut self, pitch: Note, velocity: u8, gate: u32, origin: Option<Origin>) {
       if self.program_due
           && let Some(program) = self.program
       {
           self.post(MidiMessage::ProgramChange { channel: self.channel, program });
           self.program_due = false;
       }
       // ...then the note itself, as before
   ```

3. **El Lineup Osc** elige un sonido parecido.

`MidiMessage::bytes` devolvía antes `[u8; 3]`, porque todo mensaje de
nota mide tres bytes. Un program change mide dos, así que ahora devuelve
un `MidiBytes` pequeño que hace deref a un slice (`&[u8]`) de la
longitud correcta. Ese es el trait `Deref` en acción: donde se espera un
`&[u8]`, se acepta `&message.bytes()` como tal.

## Hacer que el Lineup Osc suene como un instrumento

El Lineup Osc es diminuto: un oscilador por nota, un filtro paso bajo
suave y una envolvente de volumen (ataque, caída, sustain, release).
Nunca puede sonar como un violín de verdad, pero puede sonar más a
violín que a piano. `timbre.rs` cambia tres cosas por familia:

| | Onda | Envolvente | Suena a |
|---|---|---|---|
| Piano, guitarra, arpa, pizzicato | sierra o triangular | empieza de golpe, se apaga aunque se mantenga (sustain 0) | golpeado o punteado |
| Cuerdas, metales | sierra | entra creciendo (ataque lento), se mantiene | frotado o soplado |
| Órgano, acordeón, clarinete, saxofón | cuadrada | lleno mientras se mantiene | hueco, de lengüeta |
| Flauta, voces | triangular | ataque suave, se mantiene | suave y redondo |
| Campanas, vibráfono, marimba | senoidal | se apaga | puro |

Una sierra tiene todos los armónicos, así que es brillante; una cuadrada
solo los impares, lo que suena hueco; una triangular tiene armónicos
débiles, así que es suave; una senoidal no tiene ninguno. Un `Timbre`
son solo esos números:

```rust
pub struct Timbre {
    pub wave: Wave,
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    pub cutoff: f32,
    pub gain: f32,
}
```

`Timbre::for_program(None)` devuelve el sonido propio del Lineup Osc,
con exactamente los números que siempre tuvo, así que nada cambia para
los canales sin instrumento.

Una prueba del motor de audio comprueba que un `Command` se mantiene
pequeño, porque cada hueco de la cola hacia el hilo de audio es tan
grande como el comando más grande. Añadir la onda y la ganancia al
sintetizador lo hizo demasiado grande, así que viven en el mismo `Box`
que las velocidades de la envolvente: el sintetizador guarda un puntero
a ellos, no los valores.

## Voces: quién canta qué

Una parte de una canción demo listaba antes dónde empiezan las copias de
la melodía, en compases: `[0, 2, 4]` para tres voces a dos compases de
distancia. No decía qué voz era cada copia, y no le hacía falta mientras
todas fueran a un canal. Ahora cada copia nombra su voz:

```rust
// Each violin starts two bars after the one before.
Part::new("Entries", CANON_TUNE, CANON_CHORDS).with_voices(&[(1, 0), (2, 2), (3, 4)]),
Part::new("Canon", CANON_TUNE, CANON_CHORDS)
    .with_voices(&[(1, 0), (2, -4), (2, 2), (3, -2), (3, 4)]),
```

`(2, -4)` es el segundo violín, que sigue desde la parte anterior: entró
en el compás 2 de una parte de seis compases, así que en la parte
siguiente su melodía empezó hace cuatro compases, y solo se oye su
final. `(2, 2)` es el mismo violín empezando la melodía otra vez.

`song_demo` hace un canal por voz (Lead para la voz 1, luego «Voice 2»,
«Voice 3»...) y `melody_notes` devuelve una lista de notas para cada
una. Cada voz recibe sus propios acentos de [la interpretación](tempo-and-feel.md),
como haría cada músico.

## Una banda para cada canción

Cada canción nombra ahora su `band`: los instrumentos de su melodía, sus
acordes y su bajo. Las piezas de la sala de conciertos reciben los
instrumentos para los que fueron escritas:

| Pieza | Melodía | Acordes | Bajo |
|---|---|---|---|
| Canon in D | violines | clavecín | violonchelo |
| Für Elise, Moonlight Sonata, Träumerei | piano | piano | piano |
| Shchedryk | coro | coro | coro |
| New World Largo | corno inglés (cor anglais) | cuerdas | contrabajo |
| Morning Mood | flauta | cuerdas | violonchelo |
| Anitra's Dance | violín | cuerdas en pizzicato | cuerdas en pizzicato |
| Air on the G String | violín | violonchelo (el bajo continuo, en Keys) | contrabajo |
| Wedding March | órgano de iglesia (la transcripción que sigue la demo) | órgano | órgano |

Una canción vestida con un estilo recibe la banda de ese estilo: fiddle,
steel guitar y contrabajo para el country; fiddle y banjo para el
bluegrass; bandoneón (el «Tango Accordion» del GM), piano y bajo para el
tango; flauta de pan y guitarra para el huayno; un arpa para el joropo,
como en los *llanos* venezolanos; sintetizadores para el house y el
techno. Silent Night recibe una guitarra de nailon bajo el coro: Gruber
la escribió para guitarra, porque (cuenta la historia) el órgano de la
iglesia estaba roto.

## Pruébalo

1. Abre Frère Jacques, selecciona **Voice 3** y abre el piano roll:
   empieza cuatro compases dentro.
2. Cambia la Voice 2 del Canon a una flauta y la Voice 3 a un clarinete.
   El canon es mucho más fácil de seguir cuando cada voz tiene su propio
   color; por eso los compositores escriben para instrumentos distintos.
3. Exporta el Canon con **File > Export song as MIDI** y ábrelo en otro
   programa: cada pista ya tiene su instrumento.
