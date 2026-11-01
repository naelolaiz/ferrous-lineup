# Tomas: oír una melodía cantada

Archivos: [`crates/lineup-listen/src/voice.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-listen/src/voice.rs),
[`crates/lineup-listen/src/bass.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-listen/src/bass.rs),
[`crates/lineup-listen/src/drums.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-listen/src/drums.rs),
[`crates/lineup-listen/src/chords.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-listen/src/chords.rs),
[`crates/lineup-audio/src/input.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/input.rs),
[`crates/lineup-core/src/take.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/take.rs)
y [`crates/lineup-app/src/app/voice.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/voice.rs)

Una **toma** es una melodía cantada o silbada junto con la canción. El
botón del micrófono, junto a Rec, graba una mientras suena la canción;
**➕ Grabación**, en la lista de reproducción, o un archivo soltado
sobre una pista, pone sobre la canción una grabación hecha antes. En
los dos casos la
aplicación oye sus notas, con su altura, su momento y la fuerza con que
se cantó cada una, y la ventana **Tu toma** pregunta adónde van: en
lugar de lo que tocaba la Melodía en ese lugar, a otro canal, a uno
nuevo o a ninguna parte. La grabación misma se queda en una pista de la
lista de reproducción, dibujada como su forma de onda, donde suena con
la canción como referencia. Cómo
usarlo está en
[Tomas](https://naelolaiz.github.io/ferrous-lineup/es/manual/takes.html),
en el manual.

![La ventana Tu toma sobre la lista de reproducción: las notas oídas y los compases donde se cantaron, las primeras con su nombre, Las notas van a Melodía, la cuadrícula en semicorcheas, las alturas como se cantó, y la grabación conservada en una pista propia](../images/take-review.png)

Para eso hay que responder tres preguntas, y cada una se responde en un
crate distinto:

1. **¿Qué capas?** `lineup-listen` oye la melodía y el bajo como
   alturas a lo largo del tiempo y los corta en notas, en segundos desde
   el principio de la grabación. Encuentra golpes de batería mediante
   separación armónico-percusiva, plantillas NMF y detección espectral
   de ataques; encuentra cambios de acordes con espectros de notas NNLS
   y un decodificador de Viterbi. No sabe nada de canciones. Son
   extractores DSP aplicados a la mezcla, no separación de fuentes: la
   batería cubre bombo, caja y hi-hat, y el bajo es una estimación
   monofónica que aún no se ha evaluado con un conjunto de referencia.
   Las casillas **Extraer** de **Tu toma** eligen qué resultados
   escribir; los acordes solo están disponibles en modo canción.
2. **¿Cuándo se cantaron?** `lineup-audio` sabe cuándo tocó la tarjeta
   de sonido cada trozo de la canción y cuándo oyó el micrófono cada
   trozo de la voz, y anota, cada pocos milisegundos, dónde se oía la
   canción. No sabe nada de notas.
3. **¿Adónde van?** `lineup-core` convierte esos segundos en lugares de
   la lista de reproducción, redondea las notas a la cuadrícula y a la
   tonalidad, y las escribe en la canción.

La aplicación los une, en hilos propios, para que la ventana nunca
espere.

## Leer una grabación

Una grabación añadida desde un archivo se convierte primero en
muestras.

### Decodificar con Symphonia

Un archivo de audio es un **contenedor** (WAV, MP3, Ogg...) que guarda
una o más **pistas**, cada una comprimida con un **códec**. El crate
[Symphonia](https://crates.io/crates/symphonia), escrito en Rust puro,
así que no hay ninguna biblioteca de C que instalar en ningún sistema,
separa las dos cosas: *sondeas* el contenedor, eliges su pista de audio
y le das los **paquetes** de la pista a un decodificador de su códec.
Cada paquete se decodifica en un bloque de muestras.

Symphonia lee de cualquier cosa que implemente su trait `MediaSource`:
un archivo, o un buffer en memoria. Lo recibe como un
`Box<dyn MediaSource>`:

```rust
let file = File::open(path).map_err(DecodeError::Io)?;
let stream = MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());
```

`dyn MediaSource` es un **trait object**: «algún tipo que implementa
`MediaSource`», con el método a llamar buscado mientras el programa
corre. Tipos distintos tienen tamaños distintos, así que un trait object
vive siempre detrás de un puntero, aquí un `Box`. `Box::new(file)`
convierte el `File` en uno sin más.

El bucle del decodificador muestra cómo los errores se *comparan con
patrones*, en lugar de solo pasarse hacia arriba con `?`:

```rust
loop {
    let packet = match format.next_packet() {
        Ok(Some(packet)) => packet,
        Ok(None) => break,
        // A file cut short ends with an I/O error: keep what we have.
        Err(SymphoniaError::IoError(_)) => break,
        Err(err) => return Err(format_error(err)),
    };
    if packet.track_id != track_id {
        continue;
    }
    let block = match decoder.decode(&packet) {
        Ok(block) => block,
        // One damaged packet is a click, not a reason to give up.
        Err(SymphoniaError::DecodeError(_)) => continue,
        Err(err) => return Err(format_error(err)),
    };
    let channels = block.spec().channels().count().max(1);
    sample_rate = block.spec().rate();
    block.copy_to_vec_interleaved(&mut interleaved);
    samples.extend(
        interleaved
            .chunks(channels)
            .map(|frame| frame.iter().sum::<f32>() / channels as f32),
    );
    if too_long(samples.len(), sample_rate) {
        return Err(DecodeError::TooLong);
    }
}
```

Un error de entrada y salida al final quiere decir un archivo cortado,
así que el bucle se queda con lo que tiene; un error de decodificación
quiere decir un paquete dañado, así que lo salta; cualquier otro error
termina la lectura. `format_error` es un closure definido justo antes,
`|err: SymphoniaError| DecodeError::Format(err.to_string())`, que
convierte el error de Symphonia en el nuestro.

Las muestras vienen **entrelazadas**: izquierda, derecha, izquierda,
derecha. `chunks(channels)` corta la lista en un slice por instante, y
el promedio de cada slice es una muestra de una grabación mono: a una
voz no le importan la izquierda y la derecha. Una grabación más larga
que `MAX_SECONDS`, diez minutos, se rechaza mientras se decodifica, ya
que se tiene en memoria mientras se oyen sus notas.

Los códecs de Symphonia son **features** de Cargo, activadas en el
`Cargo.toml` del workspace, para que una compilación solo lleve las que
usa:

```toml
symphonia = { version = "0.6", default-features = false, features = ["mp3", "wav", "pcm", "flac", "ogg", "vorbis"] }
```

## Oír la altura

`voice::pitch_track` da la altura de la toma cada 10 ms, como un
**número de nota MIDI con fracción**: 69.0 es La4 (440 Hz), 69.5 un
cuarto de tono más arriba, y `None` donde no hay voz:

```rust
pub struct PitchFrame {
    pub time: f64,
    pub pitch: Option<f32>,
    pub loudness: f32,
}
```

Un número MIDI es la unidad adecuada porque el oído oye la altura como
proporciones: de 220 a 440 Hz hay una octava, y de 440 a 880 también.
En la escala MIDI las dos valen 12. La conversión es una línea:

```rust
pub fn midi_of(hz: f32) -> f32 {
    69.0 + 12.0 * (hz / 440.0).log2()
}
```

### Preparar el sonido

Antes de buscar una altura, `prepare` limpia el sonido y lo aligera. Un
**filtro pasa altos** quita lo que está por debajo de unos 40 Hz: el
pequeño desplazamiento respecto de cero que tienen muchos micrófonos, y
el retumbar de la habitación. Es un *biquad*, un filtro que recuerda
sus dos últimas entradas y sus dos últimas salidas, con los
coeficientes de la receta clásica del «audio EQ cookbook» de Robert
Bristow-Johnson. Su corazón es una línea dentro de un `map`, y una
asignación de tupla que corre los cuatro valores que recuerda:

```rust
let y = b0 * x + b1 * x1 + b2 * x2 - a1 * y1 - a2 * y2;
(x2, x1, y2, y1) = (x1, x, y1, y);
```

Después el sonido se **diezma**: solo se conserva una de cada dos o tres
muestras, así que 48 kHz pasa a 16 kHz y 44,1 kHz a 22,05 kHz. La
altura más aguda que se busca es más o menos Mi7 (2 600 Hz), muy por
debajo de la mitad de 16 kHz, así que no se pierde nada de lo que hace
falta, y el trabajo es un tercio o la mitad. Quedarse con una de cada
tres muestras de un sonido que tiene algo por encima de la nueva mitad
lo doblaría hacia abajo como alturas falsas (*aliasing*), así que antes
va un filtro pasa bajos: un **sinc con ventana**, la forma ideal del
pasa bajos recortada a unas decenas de muestras y suavizada en sus
extremos con una ventana de Hann.

### YIN: un sonido comparado consigo mismo

Una nota cantada o silbada es un sonido que se repite: la misma forma
de onda vuelve cada *período*, 1/440 de segundo para La4. El método
**YIN** (de Cheveigné y Kawahara, 2002) encuentra el período comparando
un trozo corto del sonido, aquí 25 ms, consigo mismo desplazado en cada
retardo τ, desde el período más corto que se busca hasta el más largo
(60 Hz, un poco por debajo de Si1):

```text
d(τ) = Σ (x[j] − x[j + τ])²        sobre los 25 ms
```

Donde la copia desplazada coincide con la original, la diferencia es
casi cero, y ese retardo es el período. Los retardos cortos también dan
diferencias pequeñas, porque el sonido cambia poco en unas pocas
muestras, así que YIN divide cada diferencia por el promedio de las
anteriores, la *diferencia normalizada por la media acumulada*:

```text
d′(τ) = d(τ) · τ / (d(1) + d(2) + … + d(τ))
```

Empieza cerca de 1 y baja hacia 0 en el período, y otra vez en el doble
del período (la onda también coincide ahí, una octava más abajo), y en
el triple.

Calcular `d(τ)` para cada retardo uno por uno llevaría una suma sobre
la ventana por cada retardo. Desarrollando el cuadrado, queda

```text
d(τ) = energía de x[0..W] + energía de x[τ..τ+W] − 2 · r(τ)
```

donde `r(τ)`, la suma de `x[j] · x[j + τ]`, es una *correlación*. Las
energías salen de una suma acumulada de cuadrados, y una
**transformada de Fourier** da todas las correlaciones de una vez:
transformas la ventana y el trozo entero, multiplicas un espectro por
la imagen en espejo del otro (`conj`), y transformas de vuelta.

```rust
let mut product: Vec<Complex<f32>> = a.iter().zip(&b).map(|(a, b)| a.conj() * b).collect();
```

El crate `realfft` hace las transformadas. Se planifican una vez por
frecuencia de muestreo y se guardan en la struct `Yin` como
`Arc<dyn RealToComplex<f32>>`: un trait object compartido con conteo de
referencias, que es como el planificador entrega sus planes.

### pYIN: cada valle, un candidato

YIN a secas toma el primer valle por debajo de un umbral de alrededor
de 0,1. Eso falla de dos maneras: una nota con mucho aire puede no bajar
tanto, y el valle del doble del período puede ser más hondo que el
correcto. **YIN probabilístico** (pYIN, Mauch y Dixon, 2014) guarda cada
valle como una altura *candidata*, con una probabilidad. Prueba cien
umbrales, de 0,01 a 1, y para cada uno le da su peso al primer valle por
debajo (el retardo más corto, la altura más aguda). Los pesos siguen una
*distribución beta* con su media en 0,1, donde los autores de YIN
encontraron el mejor umbral:

```rust
fn threshold_weights() -> Vec<(f32, f32)> {
    let (alpha, beta) = (2.0_f32, 18.0_f32);
    let raw: Vec<(f32, f32)> = (1..=100)
        .map(|i| {
            let x = i as f32 / 100.0;
            (x, x.powf(alpha - 1.0) * (1.0 - x).max(0.0).powf(beta - 1.0))
        })
        .collect();
    let total: f32 = raw.iter().map(|&(_, weight)| weight).sum();
    raw.into_iter()
        .map(|(threshold, weight)| (threshold, weight / total))
        .collect()
}
```

Una nota clara recibe un candidato con una probabilidad cercana a 1. Un
instante dudoso recibe varios, cada uno con un poco, y lo que falta
para 1 es la probabilidad de que no tenga altura. El retardo de cada
valle se afina entre muestras con una parábola por los tres puntos que
lo rodean, para que la altura no quede atada a muestras enteras: a
16 kHz, un período de 36 o de 37 muestras es La4 o un tercio de
semitono más abajo.

### Viterbi: un solo camino por toda la toma

Elegir en cada instante su candidato más probable, por separado, dejaría
que un instante saltara una octava y volviera. En cambio, `viterbi`
elige un candidato por instante, o «sin altura», para **toda la toma a
la vez**: el camino que menos cuesta, donde cada instante cuesta lo
improbable que es su elección (`-ln` de su probabilidad), cada paso al
instante siguiente cuesta lo que salta la altura (0,6 por cada semitono
más allá del primer tercio de uno), y empezar o parar la voz cuesta un
precio fijo. Un instante mucho más suave que la parte fuerte de la toma
cuesta más como altura: es más probable que sea aire, la habitación o
la base que se escapa de los auriculares.

Es la programación dinámica que también encuentra dónde cambia la
tonalidad de una canción (mira [Dónde cambia la
tonalidad](hearing.md#dónde-cambia-la-tonalidad)): para cada instante
y cada uno de sus estados, el camino más barato que termina ahí, y de
qué estado del instante anterior venía. Después, desde el final más
barato, hacia atrás instante por instante:

```rust
let mut state = last
    .iter()
    .enumerate()
    .min_by(|a, b| a.1.total_cmp(b.1))
    .map_or(0, |(index, _)| index);
for t in (0..candidates.len()).rev() {
    path[t] = candidates[t].get(state).map(|c| c.pitch);
    state = from[t][state];
}
```

El último estado de cada instante es «sin altura», uno más allá de sus
candidatos, así que `candidates[t].get(state)` es `None` ahí, que es
justo el `Option` que guarda la línea de altura. `total_cmp` ordena los
números de coma flotante del todo, `NaN` incluido, cosa que `<` solo no
puede, así que `min_by` puede usarlo.

## De una línea de altura a notas

La línea de altura de una voz nunca es plana: se desliza hacia las
notas, ondula con el vibrato y se corta en las consonantes.
`voice::notes` la corta en notas:

```rust
pub struct SungNote {
    pub start: f64,
    pub end: f64,
    pub pitch: f32,
    pub loudness: f32,
}
```

(`SungNote` vive en `lineup-core`, que pone las notas en la canción, y
`lineup-listen` la reexporta con `pub use`.)

1. **Tramos.** Los instantes con altura se agrupan en tramos donde la
   voz suena sin parar. Los huecos de uno o dos instantes, donde el
   buscador de alturas perdió la voz un momento, quedan dentro.
2. **Suavizado.** Cada altura se reemplaza por el valor del medio de
   las cinco que la rodean, un *filtro de mediana*: un instante
   equivocado desaparece, mientras que un cambio de nota de verdad, que
   dura, se queda.
3. **Cortes.** Un tramo se corta donde la altura se aleja más de tres
   cuartos de semitono de donde se asentó la nota y se queda lejos
   70 ms, toda en la misma dirección (el vibrato vuelve antes); donde la
   intensidad baja 5 dB y vuelve, como en una nota cantada otra vez a la
   misma altura («la, la»); y donde sube 6 dB en menos de 30 ms, como
   cuando la lengua de un silbido ataca otra vez una nota.
4. **La altura de cada trozo** es el medio de donde se asentó: la
   mediana de sus alturas, dejando fuera el primer cuarto (el
   deslizamiento hacia ella) y el último décimo. Su intensidad es su
   momento más fuerte.
5. **Ordenar.** Un *portamento de entrada*, un deslizamiento rápido de
   menos de 120 ms hacia la nota siguiente desde un poco más abajo o más
   arriba, pertenece a esa nota, que empieza donde empieza el
   deslizamiento; también menos de 120 ms a una octava, porque el
   principio de una nota puede oírse una octava desplazado. Cualquier
   otra cosa de menos de 60 ms va a la nota anterior. Dos notas seguidas
   a la misma altura se vuelven una, salvo que la segunda se haya
   cantado otra vez.

El momento más fuerte de un trozo es un **fold**: empiezas por el valor
más bajo posible y te quedas con el mayor entre él y cada intensidad.

```rust
loudness: frames
    .iter()
    .map(|frame| frame.loudness)
    .fold(f32::MIN, f32::max),
```

`f32::max` se pasa como la función misma, sin un closure alrededor: en
Rust el nombre de una función es un valor.

### Afinada: un promedio que da la vuelta

Alguien puede cantar un poco alto todo el tiempo, o silbar un cuarto de
tono bajo. `voice::tuning` mide cuánto se aleja la toma entera de las
notas del piano, para que cada nota se redondee a la nota que se quiso
cantar y no a la más cercana. Las distancias de las notas a su semitono
más cercano están entre −0,5 y 0,5, pero dan la vuelta en un círculo:
+0,45 y −0,45 están cerca, a 0,1 pasando por la mitad, no a 0,9. Así que
cada distancia se convierte en un ángulo, los ángulos se promedian como
flechas, las notas largas cuentan más, y la flecha promedio se convierte
de vuelta en una distancia:

```rust
for note in notes {
    let angle = f64::from(note.pitch - note.pitch.round()) * std::f64::consts::TAU;
    let weight = note.seconds().min(1.0);
    x += angle.cos() * weight;
    y += angle.sin() * weight;
}
(y.atan2(x) / std::f64::consts::TAU) as f32
```

Es una *media circular*, el mismo promedio que se usa para las
direcciones del viento o las horas del día.

## Cuándo se cantó: dos relojes

Una nota oída a los 3,2 s de la grabación se cantó en algún lugar de la
canción. Para saber cuál, cada instante de la grabación tiene que atarse
a un instante de la canción, y hay dos retrasos en el camino. El sonido
que el hilo de audio hace ahora sale de los altavoces un poco después
(la *latencia* de la salida), así que quien canta oye el compás 5
después de que el motor lo tocó. Y el sonido que el micrófono oye ahora
llega al programa un poco después (la latencia de la entrada). Juntos
pueden ser de 20 a 100 ms, lo que dura una semicorchea a un tempo
rápido.

La tarjeta de sonido dice los dos. Con cada bloque de salida, cpal da
una marca de tiempo de cuándo corre la función de retorno y de cuándo se
tocará el bloque; con cada bloque de entrada, cuándo corre y cuándo se
captó el sonido. Las dos se convierten en `Instant`s, instantes del
reloj constante del ordenador, y entonces se pueden comparar.

En el hilo de audio, mientras se graba una toma, el motor anota para
cada bloque **cuándo se oirá** y dónde está entonces la canción:

```rust
pub struct Heard {
    pub at: Instant,
    pub position: f64,
    pub steps_per_second: f64,
}
```

Van a un `SongClock`, una cola sin bloqueos (`rtrb`, como los demás
mensajes del hilo de audio; mira [Dos hilos](../program/threads.md)),
que el motor entrega con `take_song_clock` y recupera con
`give_back_song_clock`. No se anota nada mientras nadie escucha: un
`Arc<AtomicBool>` dice si alguien lo hace, para que el hilo de audio no
llene una cola que nadie vacía. Tampoco se anota nada durante una
cuenta previa, cuando la canción todavía no avanza.

El micrófono tiene su propia función de retorno, en su propio hilo, que
no debe esperar más de lo que puede esperar el hilo de audio. Solo
mezcla los canales en uno, mete las muestras en un ring buffer y anota
para cada bloque cuándo se oyó:

```rust
let stamp = info.timestamp();
let delay = stamp.callback.duration_since(stamp.capture);
let at = Instant::now()
    .checked_sub(delay)
    .unwrap_or_else(Instant::now);
```

Un hilo `Recorder` vacía las dos colas cada 5 ms, guarda las muestras,
y para cada bloque del micrófono busca el último instante de la canción
oído en ese momento o antes. Entre dos instantes la canción avanza a
velocidad constante, así que el lugar del bloque en la canción se
deduce:

```rust
let moment = heard.iter().rev().find(|moment| moment.at <= at)?;
let since = at.duration_since(moment.at);
if since > LONGEST_GAP {
    return None;
}
Some(Mark {
    frame,
    position: moment.position + since.as_secs_f64() * moment.steps_per_second,
    steps_per_second: moment.steps_per_second,
})
```

Una **marca** dice: en esta muestra de la grabación, la canción se oía
en esta posición, avanzando a esta velocidad. Un bloque sin ningún
instante de la canción en los últimos 100 ms (`LONGEST_GAP`) se cantó
mientras la canción no sonaba, y no recibe marca. Algunas tarjetas de
sonido informan mal sus retrasos, así que **Ajustes > Audio >
Micrófono** tiene un ajuste fino, en milisegundos, que corre cada marca
más tarde o más temprano.

Cuando la toma termina, el hilo acaba y **devuelve** lo que hizo, la
grabación y el reloj de la canción, a través de su `JoinHandle`. El
reloj vuelve al motor para la toma siguiente. La propiedad lo deja
ordenado: el reloj se movió al hilo cuando empezó, y sale de él cuando
termina, así que nunca lo tienen dos hilos a la vez, y no hace falta
ningún cerrojo.

Una grabación añadida desde un archivo no tiene tarjeta de sonido a la
que preguntar. `Take::laid` hace sus marcas como si se hubiera oído
desde el paso donde se pone, sin parar al tempo de la canción, a través
de sus cambios de tempo, una marca cada 10 ms. Desde ahí es una toma
como cualquier otra.

## Dónde en la canción: tramos

La canción tal como suena no es la lista de reproducción tal como se
dibuja: un estribillo marcado ×2 suena dos veces, y una región de bucle
da vueltas y vueltas (mira [Secciones, repeticiones y
bucles](../program/project.md#secciones-repeticiones-y-bucles)). Así
que la posición de una marca en la canción se convierte de vuelta en un
lugar de la lista de reproducción con el `pass_at` del arreglo, como se
hace con el cursor de reproducción (mira [De vuelta de la canción a la
lista de
reproducción](../program/project.md#de-vuelta-de-la-canción-a-la-lista-de-reproducción)),
y la toma se corta donde ese lugar salta, en **tramos**.

Las marcas llegan cada pocos milisegundos, y los relojes de la tarjeta
de sonido tiemblan unos milisegundos de bloque a bloque, así que la
canción puede parecer que retrocede un poco, o que avanza un poco más
de lo que debe. `goes_on` admite 20 ms de eso antes de llamarlo salto.
`runs` revisa cada 5 ms, y encuentra un salto hasta la muestra partiendo
por la mitad el hueco entre la última muestra que siguió y la primera
que no, una **búsqueda binaria**.

Donde dos tramos cubren los mismos compases, un estribillo cantado dos
veces o un bucle cantado otra vuelta, **gana el último**: `kept` recorre
los tramos del último al primero y le quita a cada uno lo que cubre uno
posterior. Cántalo otra vez hasta que salga bien. Cada parte que se
conserva se vuelve un **trozo** de la grabación, que empieza en el
primer paso entero dentro de ella.

## Qué notas: transcribir

`take::transcribe` lo junta todo. Para cada nota oída:

- su principio y su final se vuelven lugares de la lista de
  reproducción (o del patrón); una nota oída menos de 150 ms antes de
  que empiece la canción empieza con la canción, porque el buscador de
  alturas puede oír el principio de una nota uno o dos instantes antes;
- se redondean a la cuadrícula pedida: semicorcheas, corcheas o negras;
- su altura, menos la afinación de la toma, se redondea a un semitono;
  una altura justo entre dos (de 0,35 a 0,65 del camino) va a la que
  está en la tonalidad de ese lugar; o va a una nota de la tonalidad, o
  de la escala del acorde, según pida la ventana (mira [Alturas: como
  se cantó, en la tonalidad o con los
  acordes](#alturas-como-se-cantó-en-la-tonalidad-o-con-los-acordes));
- la fuerza con que se cantó fija su **velocidad**: una nota tan fuerte
  como la del medio de la toma suena a 100, las más fuertes con más
  fuerza, las más suaves más suave.

Lo fuerte y lo suave de una voz están más separados que los de un
instrumento, así que la diferencia se aprieta:

```rust
pub fn velocity(loudness: f32, usual: f32) -> u8 {
    let gain = 10.0_f32.powf(0.35 * (loudness - usual) / 20.0);
    (100.0 * gain).round().clamp(20.0, 127.0) as u8
}
```

Los decibelios son logarítmicos, así que `10^(dB / 20)` convierte una
diferencia en dB en una proporción de amplitudes, y el `0.35` la
aprieta: 6 dB más fuerte suena a 127, 20 dB más suave a 45, más o menos
de forte a piano.

Por último, una toma es una sola voz, así que las notas se hacen de a
una: de dos en el mismo paso se queda la que se cantó más larga, una
nota termina donde empieza la siguiente, y un hueco de un paso o dos
(una respiración, una consonante) se cierra en lugar de escribirse
como silencio.

### Alturas: como se cantó, en la tonalidad o con los acordes

Alguien que canta un poco desafinado, o un silbido que se desliza,
puede caer más cerca de un semitono equivocado que del correcto.
`take::transcribe_with` recibe un `Pitches` que dice cuán estricto
ser:

```rust
pub enum Pitches {
    #[default]
    AsSung,
    InKey,
    WithChords,
}
```

**Como se cantó** es el redondeo de arriba. **En la tonalidad** escribe
la nota de la tonalidad más cercana en ese lugar: en re mayor, un re♯
cantado un cuarto de semitono alto es un mi, porque re♯ no está en la
tonalidad. **Tonalidad y acordes** escribe la nota más cercana de la
escala que un solista toca sobre el acorde de la hoja de acordes en ese
lugar, en la tonalidad: la **escala de acorde** que `chord_scale` elige
para el [solista](../band/soloist.md#elegir-una-escala-de-acorde). Sobre
D7 en do mayor, el V7 de sol, es sol mayor, do re mi fa♯ sol la si, así
que un fa cantado se escribe fa♯, mientras que sol, la nota adonde
lleva D7, se queda. Sobre Fm, prestado de do menor, es fa sol la♭ si♭ do
re mi♭. Los compases sin acorde se quedan en la tonalidad; un micromodo
en la hoja de acordes es la escala de ese lugar.

`allowed_at` da dos [conjuntos de clases de
altura](pitch.md#un-conjunto-de-clases-de-altura-en-doce-bits) para un
paso: las notas como las que se puede escribir una altura, y las
preferidas justo entre dos, las del acorde:

```rust
let allowed = match harmony {
    Some(HarmonyName::Chord(symbol)) => {
        let mut allowed = chord_scale(symbol.chord(), scale);
        allowed.insert(symbol.bass_pitch_class());
        allowed
    }
    // A micromode is the scale there.
    Some(HarmonyName::Micromode(set)) => set,
    // A set, or two chords at once: the key with their notes.
    _ => PitchSet::from_bits(key.bits() | chord.bits()),
};
```

Se añade el bajo de un acorde con barra, porque G/F♯ tiene debajo un
fa♯ que la escala del acorde de G puede no tener.
`PitchSet::from_bits(a.bits() | b.bits())` es una unión: cada conjunto
son doce bits, uno por clase de altura, así que el «o» bit a bit de los
dos tiene todas las notas de cualquiera de ellos.

Luego `nearest_in` mira todas las notas a menos de una octava de la
altura cantada, se queda con las permitidas y toma la más cercana; a la
misma distancia, gana una del conjunto preferido. La ventana vuelve a
transcribir la toma cada vez que cambia la elección, así que las notas
que lista muestran la diferencia enseguida.

## Escribirla

`Project::add_take` escribe una transcripción en la canción, donde dice
el `Target`:

```rust
pub enum Target {
    Channel(usize),
    New { name: String, program: u8 },
    Nowhere,
}
```

En el modo canción las notas van a los **patrones que ya tienen las
notas de ese canal** donde se cantaron, en lugar de las notas que había:
corriges la melodía una vez, y queda corregida en todos los lugares
donde suena su patrón. `Added::shared` lista esos patrones que suenan
más de una vez, para que la aplicación pueda decirlo. Las notas
cantadas donde ningún patrón tiene notas del canal, y todas las notas
para un canal nuevo, van a clips propios, en una pista **justo debajo
de la de la grabación**, con el nombre de `Keep::notes`, como *Notas
de Toma 1*: la grabación y sus notas quedan una al lado de la otra, y
cada una se puede silenciar por su cuenta. Sin una grabación
conservada, esos clips van a la pista que eligió la ventana. En el modo
patrón las notas van al patrón.

La grabación se guarda como un archivo WAV por trozo
(`lineup_listen::wav`, en coma flotante de 32 bits, para no perder nada
de lo grabado), en una carpeta junto a la canción que lleva su nombre,
*canción takes*. Cada trozo suena en un canal sampler propio, un
`SampleSource::Take`, que arranca con una nota en el primer paso del
trozo, en patrones que empiezan en líneas de compás en la pista de la
toma. El sampler de una toma **suena junto** con la canción como una
grabación larga, por corta que sea: desde donde empiece la
reproducción, por la mitad si la reproducción empieza en su segundo
compás, y se detiene cuando se detiene la reproducción.

La canción guarda la ruta completa de cada trozo. Cuando un trozo ya no
está ahí, `file::load` lo busca de nuevo: `find_moved_take` prueba una
carpeta del mismo nombre junto a la canción, la carpeta de tomas con el
nombre de la canción y la carpeta de la propia canción. Así, una
canción movida o copiada a otra computadora junto con su carpeta de
tomas encuentra sus tomas.

Una canción con una toma se guarda en el [formato
32](../reference/project-format.md): un Ferrous Lineup más antiguo no
sabe qué es una toma, y dice que hace falta una versión más nueva en
lugar de abrir la canción sin ella.

## En la aplicación

`app/voice.rs` guarda cuatro `Option`s: la toma que se graba, la toma
cuyas notas se están oyendo, el archivo que se está leyendo, y la toma
que espera en la ventana **Tu toma**.

El botón del micrófono abre la entrada por defecto del sistema con
`Microphone::open`, toma el reloj de la canción, arranca el `Recorder`
y, si la canción no suena, la hace sonar. Mientras graba, un pequeño
panel bajo el botón muestra con qué fuerza te oye el micrófono y la
nota que oye, con cuánto se aleja («La4 +12¢»). La altura viene del
hilo del grabador: cada 40 ms llama a un **closure** que le dio la
aplicación, con los últimos 60 ms de sonido, y el closure guarda la
altura de un instante (`voice::pitch_now`) en un `AtomicU32`, como los
bits de un `f32`:

```rust
let view: lineup_audio::input::LiveView = Box::new(move |samples, rate| {
    let pitch = voice::pitch_now(samples, rate).unwrap_or(f32::NAN);
    live.store(pitch.to_bits(), Ordering::Relaxed);
});
```

No hay un `f32` atómico en la biblioteca estándar, pero un `f32` tiene
32 bits, así que `to_bits` y `f32::from_bits` lo llevan a través de un
`AtomicU32`. `lineup-audio` no sabe qué es una altura: solo llama a un
`Box<dyn FnMut(&[f32], u32) + Send>`, y la aplicación decide qué hacer
con el sonido.

Oír las notas de una toma larga lleva un momento, así que se hace en un
hilo propio, como la exportación WAV (mira [Saber cuándo ha
terminado](../sound/export.md#saber-cuándo-ha-terminado)). En cada
cuadro, `check_voice` le pregunta al handle `is_finished()`, y cuando
terminó, hace `join` y abre la ventana. Un archivo añadido se lee y se
oye igual; ese hilo toma primero el idioma de la ventana, para que un
error se escriba en él (mira [Ajustes que viven en un
hilo](../sound/export.md#ajustes-que-viven-en-un-hilo)).

La ventana no cambia nada hasta **Escribirla**. Elegir otra cuadrícula,
u otra manera de escribir las alturas, vuelve a transcribir la toma,
cosa rápida, porque las notas ya se oyeron. **Escribirla** transcribe una vez más, por si la canción cambió
mientras la ventana estaba abierta, guarda los trozos y escribe la toma
con `self.edit`, así que es **un solo paso de deshacer**.
**Descartarla**, o cerrar la ventana, no conserva nada.

### Transcribir otra vez una toma

Una toma ya escrita se puede volver a oír, para escribir sus notas con
otras opciones: **Transcribir otra vez…** en el menú de su clip. La
lista de reproducción encuentra la grabación que muestra un clip con
`recording_in`: una cuya nota empieza en el clip, o en la parte que un
recorte oculta antes de él, o, para un clip vacío (el resto de una toma
larga), la última antes de él en su pista. Vuelve a oír la grabación
entera, también las partes que ocultan sus clips. La
aplicación lee ese archivo WAV y lo oye como oye cualquier grabación
añadida, en un hilo, y después lo pone desde el paso donde está su nota
con `Take::laid`. Un trozo de una toma empieza en un paso entero y
sigue de largo al tempo de la canción, así que ponerlo ahí da el ritmo
con el que se escribió.

La revisión recuerda qué es en un `Again`: la pista de la grabación y
el nombre de la toma. La fila **La grabación** solo dice que se queda
tal como está, y no se guarda ni se añade nada: `Keep::recordings` está
vacío. Las notas van adonde fueron antes: `Review::again` mira la pista
justo debajo de la grabación y, si es la pista de notas de la toma,
elige el canal cuyas notas están en ella; si no, el canal de la
melodía, como para una toma nueva. Después `add_take` hace el resto,
como para cualquier toma: las notas van a los patrones que tienen las
notas de ese canal donde se cantó la toma, en lugar de las que había,
que son las notas escritas la primera vez.

Las notas sin un lugar ahí van a una pista propia, justo debajo de la
grabación, como la primera vez. `Keep::under` nombra la pista de la
grabación para eso, ya que esta vez no se hacen clips de grabación, y
`notes_track_under` usa la pista de notas que ya está ahí cuando tiene
lugar, o añade una.

En macOS una aplicación tiene que decir para qué quiere el micrófono
antes de que el sistema la deje escuchar, así que
`packaging/macos/Info.plist` tiene un `NSMicrophoneUsageDescription`,
que macOS muestra la primera vez.

### Dibujar una toma: su forma de onda

Los clips de una toma tienen una sola nota, en el primer paso del
trozo, así que dibujados como notas mostrarían un punto. La lista de
reproducción dibuja en cambio su **forma de onda**: una línea vertical
por cada columna de píxeles, tan alta como la muestra más fuerte del
tramo de la grabación que queda debajo.

Leer una grabación larga lleva un momento, demasiado para frenar un
cuadro, así que `Waves` lee cada archivo una sola vez en un hilo propio
y guarda el `JoinHandle` en un `HashMap` con la ruta del archivo como
clave. Hasta que el hilo `is_finished()`, el clip se dibuja vacío y la
lista de reproducción pide otro cuadro en 100 ms; entonces hace `join`,
y guarda el resultado detrás de un `Arc`, para que el dibujo pueda
tenerlo mientras el mapa se vuelve a pedir prestado. `Wave::read`
decodifica el archivo con `lineup_listen::decode_file` y se queda con la
muestra más fuerte de cada 1/400 s. Después las divide todas por la más
fuerte de toda la toma, con un piso de 0,01 para que el casi silencio
siga siendo casi silencio: una toma cantada suave llena su clip igual
que una cantada fuerte.

Una toma larga se corta en varios clips, de a lo sumo el largo de un
patrón, y solo el primero tiene la nota que la hace sonar. Así que
`paint_clips` junta primero dónde empieza cada grabación, por pista, y
dibuja cada una debajo de todos los clips de su pista bajo los que
suena, no solo debajo del que la empieza. Un clip recortado al comienzo
oculta la nota que empieza su grabación, así que una grabación empieza
en el paso donde estaría esa nota, `clip.origin(length) + note.start`,
y el dibujo se corta al clip: la forma de onda queda donde estaba, y el
clip muestra la parte que sigue tocando. El hilo de audio sigue la misma
regla, una grabación que suena solo bajo los clips de su pista (ver
[Solo bajo sus clips](../sound/instruments.md#solo-bajo-sus-clips)).

La grabación sigue de largo en segundos, mientras que la lista de
reproducción se ordena en pasos, y el largo de un paso en segundos
cambia con el tempo. `paint_wave` recorre los pasos desde el principio
del trozo, sumando el largo de cada uno, `seconds_per_beat() / 4.0` al
tempo de ese lugar, así que la forma de onda se queda debajo de los
compases con los que se oye a través de cada cambio de tempo.

### Oír una toma sola

Cada pista tiene una **S** junto a su luz de silencio. Silencia todas
las demás pistas, y un segundo clic les devuelve el sonido a todas. Una
sola regla hace las dos cosas: una pista está sola cuando es la única
que suena, y el clic alterna entre eso y que suenen todas:

```rust
let alone = tracks
    .iter()
    .enumerate()
    .all(|(other, t)| (other == track) != t.muted);
for other in 0..tracks.len() {
    if let Some(t) = project.track_mut(other) {
        t.muted = !alone && other != track;
    }
}
```

`(other == track) != t.muted` es verdadero cuando esta pista suena y
todas las demás están en silencio: `!=` entre dos `bool` es «exactamente
uno de los dos». La S se ilumina mientras su pista está sola. Es una
sola edición, así que **Ctrl+Z** deja los silencios como estaban, y los
silencios se guardan con la canción, como siempre se guardaron los de
la luz.

### Desde la lista de reproducción, la línea de comandos o soltada

Una grabación llega a la aplicación desde **➕ Grabación**, en la barra
de la lista de reproducción, desde **Añadir una grabación…** en el menú
de una pista, desde la línea de comandos o soltada sobre la ventana. El
botón la pone sobre la canción desde el cursor de reproducción (desde
el compás 1 en el modo patrón, que pasa al modo canción); el menú de la
pista también, en esa pista. Soltada sobre la lista de reproducción,
empieza donde se soltó, en el compás, el pulso o el paso que dice
**Ajuste**, en la pista que está bajo el puntero; soltada en cualquier
otro lugar, empieza en el cursor de reproducción.

Los archivos soltados llegan a la aplicación antes de que la lista de
reproducción se vuelva a dibujar, así que la lista guarda un
`DropGrid`, la cuadrícula tal como se dibujó en el último cuadro: dónde
está en la pantalla, en qué capa, y dónde empieza cada pista.
`recording_drop` le pregunta sobre qué paso y qué pista está el
puntero, y solo si ninguna ventana tapa ese lugar (`ctx.layer_id_at`
nombra la capa de más arriba ahí). Mientras se arrastra un archivo
sobre la lista de reproducción, la misma cuadrícula ilumina el lugar
donde empezaría. Un archivo nombrado en la línea de
comandos, o abierto desde un gestor de archivos, pasa por `open_now`,
que también abre proyectos, y que pregunta primero a
`lineup_listen::is_audio_file`: una extensión de `AUDIO_EXTENSIONS`,
`["mp3", "wav", "flac", "ogg"]`, en mayúsculas o minúsculas. Un archivo
de audio abierto así empieza una canción nueva, vacía, con la
grabación desde su principio.

Un archivo soltado es más difícil de ubicar, porque un sonido soltado
sobre la ventana también puede ser una muestra para el rack de canales
(mira [El sampler](../sound/instruments.md#el-sampler)): un WAV de un
golpe de batería es un sonido, un WAV de una canción entera es una
grabación. `sample::is_recording` decide por lo que el sampler puede
tocar y durante cuánto tiempo:

```rust
pub fn is_recording(path: &Path) -> bool {
    let is_wav = path
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("wav"));
    if !is_wav {
        return true;
    }
    hound::WavReader::open(path).is_ok_and(|reader| {
        let rate = u64::from(reader.spec().sample_rate);
        u64::from(reader.duration()) > rate * crate::instrument::LONG_SAMPLE_SECONDS
    })
}
```

El sampler solo toca WAV, así que un archivo MP3, FLAC u Ogg solo puede
ser una grabación. Un WAV lo es cuando dura más de diez segundos
(`LONG_SAMPLE_SECONDS`), la misma línea a partir de la cual una voz del
sampler se desvanece cuando se detiene la reproducción: pasada esa
línea, un sonido se trata como un trozo de música y no como un golpe.
`reader.duration()` es la cantidad de muestras por canal, leída de la
cabecera del archivo sin decodificarlo. `handle_dropped_files` reparte
lo soltado en tres con `partition`, dos veces: archivos de proyecto,
grabaciones y sonidos. Solo se añade una grabación a la vez, porque
cada una abre su propia ventana **Tu toma**.

## Qué tan bien funciona

El buscador de alturas se midió con el conjunto de datos
[vocadito](https://zenodo.org/records/5578807): 40 cantantes solistas,
con la altura y las notas anotadas a mano (CC BY 4.0).

- **Altura**: el 84,7 % de los instantes donde alguien canta se oyen a
  menos de un cuarto de tono de la altura anotada.
- **Notas**: una medida F de 0,708, contando una nota como encontrada
  cuando empieza a menos de 50 ms de una anotada, a la misma altura con
  un margen de un cuarto de tono. La medida F combina cuántas notas
  anotadas se encuentran con cuántas de las notas encontradas están
  anotadas.

El momento se midió con los altavoces grabados como micrófono (un
*loopback*), tocando la melodía de una demostración: cada nota volvió en
su paso, con un error mediano de unos 2 ms.

## Asperezas

- Las notas cantadas otra vez a la misma altura sin hueco y sin ataque
  nuevo (una repetición ligada) salen como una sola nota larga.
- Una voz a la vez: un acorde, o una voz sobre una base fuerte, se oye
  como la altura que más suena. Los auriculares ayudan.
- Una grabación añadida desde un archivo sigue sin parar al tempo de la
  canción: una interpretación que acelera o se frena se aleja de los
  compases.

## Cómo se prueba

Las pruebas hacen sus propias voces, para que cada caso tenga una
respuesta conocida: una voz es un tono con armónicos cada vez más suaves
y un poco de vibrato, un silbido es un tono puro. Una nota sostenida es
una nota a su altura, una línea ligada se corta donde se mueve la
altura, una nota cantada otra vez son dos notas, el silencio y el ruido
no tienen ninguna, una toma cantada alta se oye alta. `take.rs` prueba
la ubicación con marcas hechas a mano: un estribillo cantado dos veces
se queda con la segunda, un reloj que tiembla no corta la toma, una
nota oída un momento antes de que empiece la canción empieza con ella,
una toma guardada y vuelta a abrir sigue siendo una toma. `input.rs`
prueba las marcas con instantes hechos a mano, y las pruebas de ventana
de `ux_tests.rs` cantan una toma en la Melodía, conservan solo las
notas en un canal nuevo, y descartan una.

La medición con vocadito es una **prueba ignorada**, porque el conjunto
de datos no está en el repositorio:

```text
VOCADITO=/path/to/vocadito cargo test --release -p lineup-listen \
    --test sung_notes -- --ignored --nocapture
```

`#[ignore]` deja una prueba fuera de un `cargo test` normal, y
`-- --ignored` corre solo esas. `--nocapture` la deja imprimir su
tabla.
