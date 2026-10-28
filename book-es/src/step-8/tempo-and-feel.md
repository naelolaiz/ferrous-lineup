# Cambios de tempo, y tocar como una persona

Archivos: [`crates/lineup-core/src/tempo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/tempo.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs),
[`crates/lineup-core/src/library/interpret.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/interpret.rs),
[`crates/lineup-core/src/library/demo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/demo.rs),
[`crates/lineup-core/src/library/song.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song.rs),
[`crates/lineup-core/src/midi_file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/midi_file.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs)
y [`crates/lineup-app/src/playlist/tempo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/playlist/tempo.rs)

La pregunta con la que empezó este capítulo era simple: *¿hay cambios
de tempo en alguna de las demos?* No había ninguno. Cada demo se
reproducía de su primera nota a la última a un único tempo, cada nota
exactamente en la cuadrícula, y cada nota de la melodía igual de
fuerte que la siguiente. Así toca una caja de ritmos, y para un tema
techno está bien. Para un himno, un vals de Strauss o los troles de
Grieg suena como una caja de música.

Los músicos hacen tres cosas que un secuenciador no hace:

1. **Cambian el tempo.** Algunos cambios están escritos en la
   partitura: una sección nueva *Più mosso* («más movimiento») empieza
   más rápida tras una pausa; un *ritardando* (*rit.*) se frena poco a
   poco; un *accelerando* (*accel.*) se acelera poco a poco; un
   **calderón** 𝄐 sobre una nota dice «sostén esto cuanto quieras»
   antes de que la música continúe.
2. **Tocan unas notas más fuerte que otras.** Nadie escribe esto nota
   por nota, pero todo intérprete lo hace: el primer pulso de un
   compás pesa más, una frase crece hacia su nota más aguda y se
   relaja al final.
3. **Nunca están exactamente en la cuadrícula.** Un buen intérprete va
   unas centésimas de segundo antes o después todo el tiempo. Eso es
   parte de lo que hace que una interpretación suene viva, y un
   secuenciador suene mecánico.

Este capítulo añade las tres: cambios de tempo en el playlist, demos
que reproducen los cambios de tempo que marcan sus partituras y dan
forma a sus velocidades, y una *temporización loose* opcional.

## Cambios de tempo en el playlist

Un cambio de tempo es un marcador en el playlist, como un [cambio de
compás](time-markers.md):

```rust
pub struct TempoMarker {
    pub start: usize,  // the step it starts on
    pub tempo: Tempo,  // the tempo from there on
    pub glide: usize,  // 0: at once; otherwise, steps to get there
}
```

Con `glide` en 0 el tempo cambia de golpe, como después de una pausa.
Con glide, se mueve un poco en cada paso hasta llegar, `glide` pasos
más tarde: hacia abajo en un ritardando, hacia arriba en un
accelerando. Un paso es una semicorchea, así que un glide a lo largo
de dos compases de 4/4 son 32 cambios pequeños, lo bastante cerca de
una curva suave como para que nadie oiga las escaleras.

La **tempo strip** (la franja de tempo), entre las secciones y los
números de compás, dibuja el tempo como una línea: plana donde se
mantiene, un salto donde cambia de golpe, una pendiente donde hace
glide. Cada cambio se etiqueta como lo pondría una partitura:
`♩ = 60` para un cambio repentino, `rit. → 60` o `accel. → 140` para
uno gradual. Haz clic en la franja para abrir el menú de tempo de ese
compás (o pulso, con un ajuste más fino): escribe o arrastra el nuevo
tempo, elige *de golpe* o *a lo largo de* un pulso a ocho compases, o
quita el cambio.

![El final de «En la gruta del rey de la montaña» en el playlist. Sobre los números de compás, la tempo strip: «♩ = 154» en el compás 49, donde empieza el Più vivo, luego «accel. → 165» en el compás 57 y «accel. → 177» en el compás 65, con la línea subiendo a través del stretto.](../images/step-8-tempo-strip.png)

![El menú de tempo abierto en el compás 57: ♩ = 165.0 BPM, las maneras de llegar (de golpe, o a lo largo de un pulso, medio compás, uno, dos, cuatro u ocho compases, con «a lo largo de 8 compases» elegido), «De 154 a 165 BPM. Acelerar poco a poco es un accelerando (accel.)» y un botón para quitar el cambio.](../images/step-8-tempo-menu.png)

Mientras suena una canción con cambios de tempo, el cuadro **TEMPO**
de la barra de herramientas sigue el tempo del cabezal de
reproducción; parado, es el tempo con el que empieza la canción, y
cambiarlo mueve consigo todos los cambios, en la misma proporción.
Para practicar una pieza más despacio, baja el cuadro de tempo: el
ritardando del final sigue ahí, solo que más lento también. Eso es
`Project::scale_tempo`:

```rust
pub fn scale_tempo(&mut self, tempo: Tempo) {
    let ratio = tempo.bpm() / self.tempo.bpm();
    self.tempo = tempo;
    for marker in &mut self.playlist.tempo_markers {
        marker.tempo = Tempo::new(marker.tempo.bpm() * ratio);
    }
}
```

### El mapa de tempo

Con marcadores, la pregunta «¿a qué velocidad va el paso 300?»
depende de todos los marcadores anteriores, y de si alguno de ellos
sigue haciendo glide. Un tipo la responde, `TempoMap`, el primo de
tempo del `Meter` que responde preguntas sobre los compases:

```rust
pub fn tempo_at(&self, step: usize) -> Tempo {
    // The marker in effect so far, and the tempo its glide started from.
    let mut current: Option<(&TempoMarker, f32)> = None;
    let bpm_on = |current: Option<(&TempoMarker, f32)>, step: usize| match current {
        Some((marker, from)) => marker.bpm_on(step, from),
        None => self.first.bpm(),
    };
    for marker in self.markers.iter().take_while(|m| m.start <= step) {
        let from = bpm_on(current, marker.start - 1);
        current = Some((marker, from));
    }
    Tempo::new(bpm_on(current, step))
}
```

Un glide empieza desde el tempo que estuviera sonando en el paso
anterior a su marcador, incluso si era la mitad de otro glide. Por eso
el loop lleva `from` consigo: cada marcador le pregunta al anterior
«¿dónde estabas, un paso antes de mí?». `bpm_on` es la línea recta de
un glide:

```rust
fn bpm_on(&self, step: usize, from: f32) -> f32 {
    let into = step - self.start + 1;
    if into >= self.glide {
        return self.tempo.bpm();
    }
    let done = into as f32 / self.glide as f32;
    from + (self.tempo.bpm() - from) * done
}
```

Un glide de 4 pasos de 100 a 60 BPM reproduce 90, 80, 70 y 60: el
último paso del glide ya está en el tempo nuevo. Una prueba dice
exactamente eso.

Las secciones pueden repetirse, así que la canción tal como suena es
más larga que el playlist tal como se dibuja ([la estructura de la
canción](song-structure.md)). Una sección que se frena al final debe
frenarse en cada vuelta. `Project::played_tempos` calcula el tempo de
cada paso de la canción tal como suena, preguntando al arreglo qué
paso del playlist es cada paso de la canción y buscándolo en el mapa.
Cuando el tempo nunca cambia devuelve una lista vacía, y nada más
cambia para la canción de siempre.

### Reproducirlo: el hilo de audio

El hilo de audio ya sabía cómo cambiar el tempo: el cuadro de tempo
envía `Command::SetTempo`, y las notas que están sonando se estiran
para seguir terminando en su paso. Lo que necesitaba era un tempo *por
paso*. La app ahora envía, junto a la canción,
`Command::SetSongTempos(Vec<Tempo>)`: la lista de `played_tempos`. Se
construye en el hilo de la interfaz, porque el hilo de audio no puede
reservar memoria (ver [las reglas del tiempo real](../step-2/realtime.md)),
y la lista vieja vuelve al hilo de la interfaz para liberarse, como
hacen las canciones y las notas viejas.

El secuenciador cuenta frames hasta el siguiente paso. Un paso debe
durar lo que dice *su propio* tempo, así que el tempo tiene que
fijarse *antes* de que empiece el paso, no después. Un método nuevo le
dice al hilo de audio qué paso está a punto de empezar:

```rust
/// The step that starts on the next tick, if one does.
pub fn due(&self) -> Option<usize> {
    (self.frames_until_next <= 0.0).then_some(self.next_step)
}
```

y el loop sobre los frames de cada bloque hace esto antes de hacer
tick:

```rust
if let Some(step) = self.sequencer.due() {
    let tempo = self.tempo_of_step(step);
    if tempo != self.tempo_now {
        self.set_tempo(tempo);
    }
}
```

`tempo_of_step` es el tempo de la canción para ese paso en el modo
canción, y el del cuadro de tempo en el modo patrón, donde trabajas
sobre un patrón y quieres un loop constante. `bool::then_some` es un
pequeño truco que vale la pena conocer: `condition.then_some(value)`
es `Some(value)` cuando la condición se cumple y `None` si no.

¿Funcionó? La sonda era un bombo en cada pulso, a 120 BPM, un cambio
repentino a 60 en el compás 3 y un accelerando de vuelta a 120 a lo
largo de los compases 5 y 6. Reproducida en la app y grabada desde su
salida de sonido, el tiempo entre bombos era:

```text
0.500 0.500 0.500 0.500 0.500 0.500 0.500 0.500   compases 1-2: 120 BPM
1.000 1.000 1.000 1.000 1.000 1.000 1.000 1.000   compases 3-4: 60 BPM
0.929 0.832 0.753 0.689 0.634 0.587 0.547 0.512   compases 5-6: accel.
0.500 0.500 ...                                   compás 7 en adelante: 120 BPM
```

### Exportaciones y archivos

Una exportación WAV es la canción reproducida por el mismo hilo de
audio hacia un archivo, así que recibe la misma lista de tempos. Su
duración ya no es «pasos por la duración de un paso»:
`Project::played_seconds` suma cada paso a su propio tempo.

Un archivo MIDI escribe el tempo como un *meta evento*, «a partir de
este tick, un pulso dura tantos microsegundos». Un cambio repentino es
un evento; un glide es un evento por paso, que es como los archivos
MIDI escriben un ritardando, y como lo leerán de vuelta otros
programas. Los eventos de la primera pista se recogen, se ordenan por
tick y se escriben en orden, porque una pista MIDI guarda el tiempo
*entre* eventos.

Los archivos de proyecto guardan los marcadores como `tempo_markers`,
y solo cuando los hay. Un archivo que usa cambios de tempo (o
temporización loose, más abajo) se escribe como **formato 7**. Un
Ferrous Lineup más antiguo lo rechaza con «hecho por una versión más
nueva» en lugar de reproducir la canción a un único tempo sin decir
nada.

## Demos que suenan como una interpretación

### Lo que dicen las partituras

Cada marca de abajo se leyó de una partitura de dominio público, no se
adivinó de una grabación, y solo donde cae dentro de los compases que
reproduce la demo:

| Demo | Qué marca la partitura | En la demo |
|---|---|---|
| La Quinta de Beethoven | Calderones sobre las notas largas de los compases 2, 5 y 21 | un `fermata` en cada una, cada vez que vuelve el motivo |
| Träumerei (Schumann) | *ritard.* en los compases 8 y 16, *a tempo* después de cada uno, un calderón en el compás 22, *ritardando* hasta el final, un calderón sobre el último acorde | ♩ = 56, frenando hasta 42 |
| Aria en la cuerda de sol (Bach) | Un calderón sobre el último Re | Solo en la última pasada, para que la repetición siga fluyendo |
| Preludio en mi menor (Chopin) | *stretto* en los compases 16 a 18, calderones en los compases 23 y 25 | Acelera hasta 69 y vuelve a 60 |
| El cóndor pasa (Robles) | *Andante* para la canción, un calderón al final, *Allegro* para la fuga, un calderón sobre el último adorno | 80, luego 120 |
| En la gruta del rey de la montaña (Grieg) | ♩ = 138, *Più vivo* ♩ = 154 en el compás 50, *sempre stretto sin al fine* en el compás 58 | Ahora en el orden de la partitura: 138, 154, y luego subiendo con glide hasta 165 y 177 |

Las fuentes son los archivos LilyPond del Proyecto Mutopia (Beethoven,
Schumann, Bach, Chopin, la propia versión para piano de Grieg) y la
edición para piano de 1933 de *El cóndor pasa*. Unos pocos números son
decisiones propias, porque la partitura da una palabra y no un número:
cuánto más rápido es un *stretto* o un *Allegro*, y dónde vuelve el
tempo de Chopin después de su stretto (no escribe ningún *a tempo*).
La Tocata en re menor de Bach también tiene calderones, pero la demo
ya escribe esas notas largas, así que no se le añadió ninguno. Se
revisaron muchas otras demos y no tienen marca alguna en los compases
que reproducen: Für Elise es *Poco moto* en toda su primera página, y
Rêverie solo se frena mucho después de que acabe la demo.

Una parte de una canción demo puede ahora llevar las marcas de tempo
de su partitura, con los pasos contados desde el principio de la
parte:

```rust
Part::new("Theme", MELODY, CHORDS).with_tempo(&[
    glide(6 * 16, 72, 32), // rit. over the last two bars, to 72
    fermata(8 * 16 - 8, 8), // hold the last half note
])
```

`TempoMark` tiene tres clases, nombradas como las nombra una
partitura: `tempo(at, bpm)` para un tempo nuevo de golpe,
`glide(at, bpm, over)` para un rit. o accel., y `fermata(at, steps)`.
La demo pone un marcador de tempo en el playlist por cada una, cada
vez que suena la parte. Un calderón se convierte en dos marcadores: la
mitad del tempo para los pasos sostenidos (para que duren el doble), y
luego el tempo de antes.

### Una pequeña frenada al final

La mayoría de canciones, himnos y piezas de concierto terminan con un
pequeño *ritardando*, incluso donde la partitura no escribe ninguno:
es la manera en que un intérprete dice «esto es el final». La música
de baile y cualquier cosa sobre una caja de ritmos no lo hace: el
pulso sigue hasta el último compás. Así que las canciones de los
géneros clásico, folk, soul (los espirituales y los himnos), tango y
folclore sudamericano frenan hasta nueve décimas de su tempo a lo
largo de su último compás, salvo que su última parte ya tenga marcas
propias. Es un único marcador, así que si prefieres la demo sin él,
haz clic sobre él en la tempo strip y quítalo.

### Velocidades: la forma de una frase

`library/interpret.rs` reproduce las notas escritas como lo haría un
músico, cambiando sus *velocidades* (con qué fuerza se pulsa cada
tecla), nunca su lugar. Para la melodía suma unas cuantas cantidades
pequeñas, todas salidas de las propias notas:

| Qué | Cambio |
|---|---|
| El primer pulso del compás | +6 |
| Otro pulso | +2 |
| Fuera del pulso: una corchea / una semicorchea | −2 / −4 |
| Qué alta está la nota en su frase | hasta ±6 |
| Una nota larga (medio compás o más) | +2 |
| La última nota de una frase | −5 |
| Un poco de irregularidad | hasta ±3 |

Una *frase* termina en un silencio de corchea o más (la respiración de
un cantante), o después de cuatro compases. El contorno es la altura
de la nota menos la altura media de su frase, por 0.8, de modo que la
parte alta de una frase se alza cantando y sus notas graves se quedan
debajo. Sobre 127, el resultado nunca mueve una nota más de unos 15
pasos respecto de donde estaba: oyes una frase, no un efecto.

Los acordes y el bajo ya tienen sus acentos: los patrones de comping
los escriben (el «oom» de un vals más fuerte que su «pah-pah»). Solo
pierden su regularidad de máquina, hasta ±4, y las notas de un acorde
se mueven casi juntas, como las golpea una mano.

«Aleatorio», pero siempre igual: la irregularidad sale de un *hash* de
los nombres de la canción, la parte y el canal y del lugar de cada
nota, mezclado por la función SplitMix64. Así que una demo suena igual
cada vez que la abres, y una prueba puede comprobar valores exactos.

```rust
fn wobble(seed: u64, value: u64) -> f32 {
    let mut x = seed ^ value.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^= x >> 31;
    (x >> 40) as f32 / (1u64 << 23) as f32 - 1.0
}
```

`wrapping_mul` multiplica y deja que el resultado dé la vuelta más
allá de `u64::MAX`, en lugar de hacer panic en una compilación de
debug: aquí dar la vuelta es todo el punto. Los desplazamientos y las
multiplicaciones revuelven los bits, de modo que semillas vecinas dan
números sin relación.

Puedes ver las velocidades en el carril de velocidad del piano roll,
bajo la cuadrícula: los compases de la melodía ya no tienen todos la
misma altura.

## Temporización loose

**Loose**, en la barra de herramientas del playlist, fija a qué
distancia de la cuadrícula pueden caer las notas, desde *off* (el
valor por defecto) hasta 40 milisegundos. Con él activado, cada nota
de la canción empieza hasta ese tanto tarde, una cantidad distinta
cada vez.

¿Cómo puede el hilo de audio empezar una nota *más tarde* que su
paso? Cuando empieza un paso, cada nota recibe un retraso en frames.
Una nota sin retraso empieza de golpe, como siempre. Una nota con
retraso va a una lista de espera:

```rust
struct Pending {
    channel: usize,
    origin: Origin,
    pitch: Note,
    velocity: u8,
    gate: u32,
    at: u64, // the frame it starts on
}
```

y en cada frame se arrancan las notas cuyo `at` ha llegado. La lista
es un `Vec` hecho con espacio para 512 notas en el hilo de la
interfaz, y el hilo de audio nunca deja que crezca (si alguna vez está
lleno, una nota simplemente empieza a tiempo), de modo que nunca
reserva memoria. Sacar una nota usa `swap_remove`, que mueve la última
nota al hueco en lugar de desplazar la lista entera: el orden de las
notas en espera no importa, y es más rápido.

¿Por qué solo *tarde*, nunca pronto? Para empezar una nota pronto, el
hilo de audio tendría que mirar dentro del paso siguiente antes de que
empiece. Tarde entre 0 y 20 ms es lo mismo que pronto o tarde 10 ms
alrededor de una cuadrícula que a su vez va 10 ms tarde, y nadie oye
una canción entera 10 ms tarde.

El retraso de cada nota sale del mismo tipo de hash que las
velocidades, así que la temporización loose suena igual cada vez, y
una exportación WAV suena como la reproducción. La mayor parte del
retraso la comparten todas las notas que un canal arranca en un paso,
y solo una quinta parte difiere de nota a nota: el acorde de un
pianista va un poco tarde o pronto como un todo, no repartido como un
arpa.

Grabada desde la app con la canción sonda y Loose a 20 ms, los pulsos
ya no estaban exactamente a medio segundo unos de otros:

```text
0.493 0.498 0.509 0.504 0.486 0.509 0.496 0.508   compases 1-2 a 120 BPM
0.997 0.991 1.015 0.984                           compás 3 a 60 BPM
```

## Qué llevarse

- Un cambio de tempo es un dato en el playlist; un `TempoMap` lo
  convierte en un tempo por paso, igual que un `Meter` convierte los
  marcadores de tiempo en compases.
- El hilo de audio recibe una lista que solo puede leer, hecha en otra
  parte, y fija el tempo de cada paso antes de que el paso empiece.
- La interpretación son muchas decisiones pequeñas y deterministas:
  unos cuantos pasos de velocidad aquí, unos milisegundos allá.
