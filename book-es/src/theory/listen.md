# Importar una grabación (experimental)

Archivos: [`crates/lineup-listen/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates/lineup-listen)
y [`crates/lineup-app/src/app/import.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/import.rs)

**Archivo > Importar grabación…**, o `ferrous-lineup song.mp3` en la
línea de comandos, abre un archivo MP3, WAV, FLAC u Ogg como un proyecto
nuevo, e intenta tocar junto con él. La grabación misma va a un canal
sampler, y un canal **Armonía** toca el acorde oído en cada pulso. El
tempo, el compás, la tonalidad y los cambios de tonalidad se fijan a
partir de lo oído, así que las líneas de compás caen sobre los compases
de la grabación, la hoja de acordes se llena, y las teclas de armonía
se iluminan con el acorde que suena. Cómo usarlo está en
[Grabaciones](https://naelolaiz.github.io/ferrous-lineup/es/manual/importing.html#grabaciones-experimental),
en el manual.

![Vibe Ace importado, con las teclas de armonía siguiendo sus acordes](../images/spike-listen-vibe-ace.png)

Es experimental, y el título del capítulo lo dice: el tempo, los
acordes, los cambios de tonalidad y el primer tiempo fuerte son
estimaciones, no una partitura leída del sonido. Un archivo de notas,
MIDI, dice exactamente qué se toca; una grabación es una sola onda, con
todos los instrumentos sumados, y todo hay que deducirlo de ella.

El trabajo es un crate propio, `lineup-listen`, que no sabe nada de
ventanas, más un archivo en la aplicación. Este capítulo sigue al sonido
desde el archivo hasta el proyecto: una transformada de Fourier para ver
las notas, y **programación dinámica** tres veces, para los pulsos, los
acordes y los cambios de tonalidad.

## El recorrido

`analyze` es toda la escucha, como funciones simples, cada una en su
propio módulo, cada una tomando los datos que hizo la anterior:

```rust
pub fn analyze(recording: &Recording) -> Analysis {
    let beats = beats::track(&recording.samples, recording.sample_rate).times;
    let bpm = tempo_of(&beats);

    let scale = bpm_key::analyze(
        &recording.samples,
        1,
        recording.sample_rate,
        &bpm_key::Config::default(),
    )
    .ok()
    .and_then(|features| features.key)
    .map(|estimate| {
        let tonic = PitchClass::from_semitone(estimate.key.tonic.index());
        let kind = if estimate.key.is_minor() {
            ScaleKind::NaturalMinor
        } else {
            ScaleKind::Major
        };
        Scale::new(tonic, kind)
    });

    let frames = chroma::frames(recording);
    let beat_chromas: Vec<_> = beats
        .iter()
        .enumerate()
        .map(|(i, &start)| {
            let end = beats.get(i + 1).copied().unwrap_or(start + 60.0 / bpm);
            chroma::average(&frames, start, end)
        })
        .collect();
    let chords = chords::chords_per_beat(&beat_chromas, scale);
    let (beats_per_bar, first_downbeat) = meter::guess(&chords);
    let keys = keys::key_spans(&beat_chromas, beats_per_bar, first_downbeat);

    Analysis {
        beats,
        bpm,
        beats_per_bar,
        first_downbeat,
        scale,
        keys,
        chords,
    }
}
```

En orden:

1. **Pulsos** (`beats.rs`): cuándo cae cada pulso, en segundos, y a
   partir de ellos el tempo.
2. **Tonalidad** (el crate `bpm-key`): la tonalidad de toda la
   grabación, que ayuda a nombrar los acordes y pasa a ser la
   tonalidad del proyecto.
3. **Chroma** (`chroma.rs`): con qué fuerza suena cada una de las doce
   clases de altura, ventana a ventana, y luego promediado en cada
   pulso.
4. **Acordes** (`chords.rs`): el acorde de cada pulso.
5. **Compás** (`meter.rs`): tres o cuatro pulsos por compás, y qué pulso
   es el primer tiempo fuerte.
6. **Cambios de tonalidad** (`keys.rs`): la tonalidad de cada compás,
   encontrada por el mismo código que encuentra la tonalidad de las
   notas de una canción.

Luego `arrange.rs` convierte el `Analysis` en un `Project`, con los tipos
que usa el resto de Ferrous Lineup: `Channel`, `Pattern`, `NoteEvent`,
`Clip`. Nada de lo que viene después sabe que las notas salieron de una
grabación.

La cadena después de `bpm_key::analyze` se lee tal como se ejecuta. El
crate devuelve un `Result`; `.ok()` lo convierte en un `Option` y
descarta el error, ya que una grabación sin una tonalidad clara no es un
fallo. `and_then` sigue solo si hay un valor, y las características
pueden no tener tonalidad (`features.key` es a su vez un `Option`).
`map` convierte la tonalidad del crate en una de las nuestras. Si algún
paso no tiene nada, `scale` es `None`, sin ningún `if` ni ningún
`match`. El último pulso no tiene un pulso siguiente donde terminar, así
que `unwrap_or` le da la duración de un pulso.

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
termina la importación. `format_error` es un closure definido justo
antes, `|err: SymphoniaError| DecodeError::Format(err.to_string())`, que
convierte el error de Symphonia en el nuestro.

Las muestras vienen **entrelazadas**: izquierda, derecha, izquierda,
derecha. `chunks(channels)` corta la lista en un slice por instante, y
el promedio de cada slice es una muestra de una grabación mono: a los
acordes no les importan la izquierda y la derecha. Una grabación más
larga que `MAX_SECONDS`, diez minutos, se rechaza mientras se decodifica,
ya que el análisis tiene en memoria la grabación y una copia de ella.

Los códecs de Symphonia son **features** de Cargo, activadas en el
`Cargo.toml` del workspace, para que una compilación solo lleve las que
usa:

```toml
symphonia = { version = "0.6", default-features = false, features = ["mp3", "wav", "pcm", "flac", "ogg", "vorbis"] }
```

### De las muestras al chroma

Un acorde es un conjunto de clases de altura: Do mayor es Do, Mi y Sol
en cualquier octava (mira [Clases de altura, notas y su
escritura](pitch.md)). Para encontrar acordes en el sonido
hace falta saber, momento a momento, con qué fuerza suena cada una de
las doce clases de altura. Esos doce números se llaman el **chroma**, y
salen de una **transformada de Fourier**, que toma un tramo de muestras
y dice cuánto hay en él de cada frecuencia.

`chroma::frames` corta la grabación en **ventanas** superpuestas de
8.192 muestras, unos 0,19 s cada una a 44,1 kHz, y empieza una nueva
cada 2.048 muestras (46 ms). Cada ventana se multiplica por una
**ventana de Hann**, una joroba suave que la hace entrar y salir
gradualmente: sin ella, el corte brusco en cada extremo le parecería a
la transformada un clic, que contiene todas las frecuencias. La FFT (la
transformada rápida de Fourier, del crate `realfft`) dice entonces qué
tan fuerte suena cada **bin** de frecuencia en la ventana. El bin `k` es
la frecuencia `k × sample_rate / 8192`, así que a 44,1 kHz los bins
están a 5,4 Hz uno de otro. Las notas vecinas están más separadas que
eso desde más o menos Sol2 (98 Hz) hacia arriba; la ventana es larga
porque las notas graves lo necesitan.

Cada bin pertenece a una nota. La fórmula `69 + 12 × log2(f / 440)`
convierte una frecuencia en un número de nota MIDI: 69 es La4, a 440
Hz, y cada semitono hacia arriba multiplica la frecuencia por la raíz
duodécima de dos. El redondeo da la nota más cercana, y el resto de
dividir por 12 su clase de altura:

```rust
let pitch_class_of = |bin: usize| {
    let frequency = bin as f32 * rate / FRAME as f32;
    let midi = 69.0 + 12.0 * (frequency / 440.0).log2();
    (midi.round() as i32).rem_euclid(12) as usize
};
```

`rem_euclid` es el resto que nunca es negativo: `-3 % 12` es `-3` en
Rust, `(-3).rem_euclid(12)` es `9`. Aquí ningún bin queda por debajo de
la nota MIDI 0, pero el closure no necesita saberlo.

La intensidad de cada bin se suma a su clase de altura. Tres detalles
hacen que esto funcione con música real:

- La intensidad se toma como `ln(1 + 100 × magnitud)`, porque oímos la
  intensidad en una escala más o menos logarítmica: una nota suave una
  octava más arriba debe seguir contando, y la nota más fuerte no debe
  tapar todo lo demás.

  ```rust
  let level = |bin: usize| (1.0 + 100.0 * spectrum[bin].norm()).ln();
  ```

- Las notas agudas están más separadas en Hz, así que reciben más bins
  que las graves. Cada clase de altura se divide por su número de bins,
  o Si siempre parecería más fuerte que Do.
- Se guardan dos chromas por ventana. El **agudo**, de 130 a 2.100 Hz
  (más o menos de Do3 a Do7), es donde está el acorde. El **grave**, de
  40 a 260 Hz (más o menos de Mi1 a Do4), dice sobre todo qué nota es la
  más grave, y la nota más grave suele ser la fundamental del acorde.

A qué clase de altura suma cada bin se calcula una sola vez, antes del
bucle sobre las ventanas, como una lista de pares `(bin, clase de
altura)`:

```rust
let bins = |(low, high): (f32, f32)| {
    let first = (low * FRAME as f32 / rate).ceil() as usize;
    let last = ((high * FRAME as f32 / rate).floor() as usize).min(FRAME / 2);
    (first.max(1)..=last)
        .map(|bin| (bin, pitch_class_of(bin)))
        .collect::<Vec<_>>()
};
let treble_bins = bins(TREBLE);
let bass_bins = bins(BASS);
```

El parámetro del closure es un **patrón**: `|(low, high): (f32, f32)|`
recibe una tupla y nombra sus dos mitades, como lo haría
`let (low, high) = range;`. `TREBLE` y `BASS` son tuplas así. Con la
tabla hecha, el bucle sobre miles de ventanas no hace más que sumas.

La FFT misma también se planifica una sola vez. `RealFftPlanner` elige
la manera más rápida de transformar 8.192 números reales y la devuelve
como un `Arc<dyn RealToComplex<f32>>`, otro trait object, que el bucle
ejecuta en cada ventana con buffers creados una sola vez:

```rust
let mut planner = RealFftPlanner::<f32>::new();
let fft: Arc<dyn RealToComplex<f32>> = planner.plan_fft_forward(FRAME);
let mut input = fft.make_input_vec();
let mut spectrum = fft.make_output_vec();
let mut scratch = fft.make_scratch_vec();
```

`chroma::average` da luego un chroma por pulso: el promedio de las
ventanas cuyo centro cae entre este pulso y el siguiente.

### Del chroma a los acordes: plantillas y Viterbi

`chords::chords_per_beat` nombra el acorde de cada pulso. Hay dos ideas,
la manera clásica de hacerlo sin aprendizaje automático.

**Plantillas.** Cada acorde que podemos reconocer es una **plantilla**:
doce números, 1 para sus notas y 0 para el resto. Hay seis cualidades
sobre doce fundamentales, 72 acordes, más «ningún acorde»:

```rust
const QUALITIES: [ChordQuality; 6] = [
    ChordQuality::Major,
    ChordQuality::Minor,
    ChordQuality::Dominant7,
    ChordQuality::Minor7,
    ChordQuality::Major7,
    ChordQuality::Diminished,
];
```

El chroma del pulso se compara con cada plantilla por **correlación**:
los dos se llevan a un promedio de 0 y una longitud de 1, y su producto
escalar es entonces 1 si suben y bajan juntos y 0 si no tienen relación.
Es la misma comparación que hace el buscador de tonalidades con sus
perfiles (mira [Oír la
armonía](hearing.md#comparar-el-método-de-krumhansl-schmuckler)).
Después, tres ajustes pequeños:

```rust
let Some(chord) = candidate.chord else {
    return NO_CHORD_SCORE;
};
let mut score = dot(&treble, &normalized(&candidate.template));
score += BASS_WEIGHT * bass[usize::from(chord.root.semitone())] / bass_peak;
if scale.is_some_and(|scale| chord.pitch_set().is_subset_of(scale.pitch_set())) {
    score += KEY_WEIGHT;
}
if chord.quality.intervals().len() > 3 {
    score -= SEVENTH_COST;
}
score
```

- Una nota grave fuerte en la fundamental del acorde suma hasta 0,15
  (`BASS_WEIGHT`), y una débil resta un poco, ya que el chroma grave
  también está centrado en su promedio.
- Un acorde cuyas notas pertenecen todas a la tonalidad suma 0,08
  (`KEY_WEIGHT`). La mayoría de los acordes de una canción cumplen eso,
  así que la tonalidad es un **prior** útil: lo que esperamos antes de
  escuchar.
- Un acorde de cuatro notas pierde 0,03 (`SEVENTH_COST`), así que gana
  una tríada simple salvo que la séptima esté de verdad. C7 contiene
  entero el acorde de Do mayor, y sin esto ganaría cada vez que un Si♭ flotara débil en
  el aire.

«Ningún acorde» puntúa siempre 0,25 (`NO_CHORD_SCORE`): un pulso donde
ningún acorde lo supera, como el silencio, la batería sola o una voz
hablada, no tiene acorde.

**Suavizado.** Solo, el mejor acorde de cada pulso salta de un lado a
otro: una nota de paso de la melodía hace que C parezca Am durante un
pulso. Los acordes reales duran. Así que cambiar de acorde cuesta 0,25
(`CHANGE_COST`), y el **algoritmo de Viterbi** encuentra la sucesión de
acordes con la mejor puntuación total. Es programación dinámica: para
cada pulso y cada uno de los 73 candidatos guarda la mejor puntuación de
cualquier sucesión que termine ahí, hecha con los mejores del pulso
anterior, y recuerda qué acorde vino antes:

```rust
let mut best = scores(&beats[0], &candidates, scale);
let mut from = vec![vec![0_usize; count]; beats.len()];
for (b, beat) in beats.iter().enumerate().skip(1) {
    let here = scores(beat, &candidates, scale);
    // Changing chord costs the same from anywhere, so the best way to
    // change into any chord is from the overall best one.
    let (leader, leader_score) = best
        .iter()
        .copied()
        .enumerate()
        .fold((0, f32::MIN), |a, (i, s)| if s > a.1 { (i, s) } else { a });
    let mut next = vec![0.0; count];
    for c in 0..count {
        let (previous, total) = if best[c] >= leader_score - CHANGE_COST {
            (c, best[c])
        } else {
            (leader, leader_score - CHANGE_COST)
        };
        from[b][c] = previous;
        next[c] = total + here[c];
    }
    best = next;
}
```

A cada acorde del pulso `b` se llega o quedándose en él, o cambiando
desde algún acorde y pagando la penalización. Comparar todos los pares
serían 73 × 73 comparaciones por pulso. Pero cambiar cuesta lo mismo
desde cualquier lado, así que el mejor cambio hacia cualquier acorde
viene del mejor de todos en el pulso anterior, el `leader`: una
comparación por acorde, 73 por pulso. Al final, el código toma el mejor
último acorde y recorre `from` hacia atrás para leer la sucesión entera.
El buscador de tonalidades usa el mismo bucle sobre compases y
tonalidades (mira [Dónde cambia la
tonalidad](hearing.md#dónde-cambia-la-tonalidad)).

Encontrar el líder necesita el `f32` más grande, y `f32` no tiene un
`max` para un iterador: los números de coma flotante no son `Ord`,
porque `NaN` no se compara con nada. `fold` lo hace a mano, llevando el
mejor `(índice, puntuación)` hasta el momento. El seguidor de pulsos de
más abajo hace lo mismo con `max_by` y `total_cmp`, una comparación que
pone a todos los números de coma flotante, también a `NaN`, en algún
orden.

### Pulsos: arranques, autocorrelación y programación dinámica

`beats::track` es el seguidor de pulsos de D. P. W. Ellis, «Beat
Tracking by Dynamic Programming» (2007), con los valores que la
biblioteca librosa documenta como sus valores por defecto, para que los
resultados se puedan comparar con ella. Tiene tres etapas.

**La fuerza de los arranques.** Cuánto sonido *nuevo* empieza en cada
ventana corta: 2.048 muestras, una nueva cada 512 (12 ms a 44,1 kHz).
Se toma un espectrograma en la **escala mel**, 128 bandas espaciadas
como las oye el oído, estrechas abajo y anchas arriba, en decibelios, y
para cada ventana se suman los aumentos de cada banda desde la ventana
anterior. Las disminuciones no cuentan: una nota que se apaga no es un
arranque. El resultado, un número por ventana, es la **envolvente de
arranques**, que tiene un pico en cada golpe de batería y en cada
comienzo de nota.

**El tempo.** La envolvente se compara consigo misma desplazada en cada
duración de pulso posible, de 30 a 320 BPM: su **autocorrelación**. Con
el desplazamiento correcto, los picos caen sobre picos y la correlación
es alta. Los desplazamientos cerca de 120 BPM reciben una ventaja, una
campana de una octava de ancho, porque si no, los seguidores eligen la
mitad o el doble del tempo con la misma facilidad que el tempo mismo:

```rust
let bpm = frame_rate * 60.0 / lag as f64;
let octaves = (bpm / LIKELY_BPM).log2();
let pull = (-0.5 * (octaves / LIKELY_SPREAD_OCTAVES).powi(2)).exp();
let score = correlation(lag) * pull;
```

A 60 o a 240 BPM, a una octava, una correlación cuenta 0,61 veces lo
que cuenta la misma correlación a 120.

**Los pulsos.** De todas las maneras de colocar pulsos, el seguidor
quiere aquella en que los pulsos caen sobre arranques fuertes y están
separados más o menos una duración de pulso. Probar todas las
colocaciones es imposible, pero la programación dinámica lo hace barato.
La mejor puntuación de una cadena de pulsos que termina en la ventana
`i` es la fuerza del arranque ahí, más la mejor puntuación de una cadena
que termina una duración de pulso antes, más o menos, menos una
penalización por ese más o menos:

```rust
for i in 1..envelope.len() {
    // The previous beat is between half a beat and two beats back.
    let earliest = i.saturating_sub(period * 2);
    let latest = i.saturating_sub((period / 2).max(1)).min(i - 1);
    let mut best: Option<(usize, f64)> = None;
    for (tau, &earlier) in score.iter().enumerate().take(latest + 1).skip(earliest) {
        let spacing = (i - tau) as f64 / period as f64;
        let candidate = earlier - TIGHTNESS * spacing.ln().powi(2);
        if best.is_none_or(|(_, b)| candidate > b) {
            best = Some((tau, candidate));
        }
    }
    match best {
        // A chain worth continuing: positive, so silence does not
        // chain beats that are not there.
        Some((tau, total)) if total > 0.0 => {
            score[i] = strength[i] + total;
            from[i] = Some(tau);
        }
        _ => score[i] = strength[i],
    }
}
```

`spacing` es 1 para una separación de exactamente un pulso, y su
logaritmo es entonces 0: ninguna penalización. Medio pulso o dos pulsos
dan `ln 2` al cuadrado, unos 0,48, por `TIGHTNESS` (100): un pulso en el
lugar equivocado tiene que ser mucho más fuerte para ganar.
`saturating_sub` resta pero se detiene en 0 en lugar de dar la vuelta,
cosa que los números sin signo harían cerca del principio. `is_none_or`
es verdadero para `None` o cuando el closure lo dice: «todavía no hay
mejor, o este es mejor», en una sola llamada. Cada ventana recuerda en
`from` qué pulso anterior eligió, y al final se sigue la mejor última
ventana hacia atrás hasta el principio:

```rust
let mut at = (0..envelope.len())
    .max_by(|&a, &b| score[a].total_cmp(&score[b]))
    .unwrap_or(0);
```

La autocorrelación solo conoce ventanas enteras. A 44,1 kHz, 86 ventanas
por segundo, los desplazamientos alrededor de 120 BPM son 42, 43 y 44
ventanas: 123,0, 120,2 y 117,5 BPM, nada entre medio. Así que el tempo
que recibe el proyecto no es el del seguidor sino la pendiente de los
pulsos mismos, la recta más cercana a «el momento del pulso n», un
**ajuste por mínimos cuadrados**:

```rust
fn tempo_of(beats: &[f64]) -> f64 {
    let n = beats.len() as f64;
    if beats.len() < 2 {
        return 120.0;
    }
    let mean_index = (n - 1.0) / 2.0;
    let mean_time = beats.iter().sum::<f64>() / n;
    let (mut covariance, mut variance) = (0.0, 0.0);
    for (i, &time) in beats.iter().enumerate() {
        let di = i as f64 - mean_index;
        covariance += di * (time - mean_time);
        variance += di * di;
    }
    let seconds_per_beat = covariance / variance;
    if seconds_per_beat > 0.0 {
        60.0 / seconds_per_beat
    } else {
        120.0
    }
}
```

La pendiente de la mejor recta es la covarianza del índice y el tiempo
dividida por la varianza del índice: segundos por pulso, promediados
sobre toda la canción, con las pequeñas desviaciones de cada pulso
compensándose entre sí.

### El compás: dónde están los compases

Los acordes cambian sobre todo en el primer pulso de un compás. Así que
`meter::guess` cuenta, para cada manera de agrupar los pulsos (de tres
en tres o de cuatro en cuatro, empezando en cualquier pulso), cuántos
cambios de acorde caen en el primer pulso de un grupo, y se queda con la
agrupación que más supera al azar:

```rust
pub fn guess(chords: &[Option<Chord>]) -> (usize, usize) {
    let changes: Vec<usize> = (1..chords.len())
        .filter(|&i| chords[i] != chords[i - 1] && chords[i].is_some())
        .collect();
    if changes.len() < 4 {
        return (4, 0);
    }
    let mut best = (4, 0, f64::MIN);
    for beats in [4, 3] {
        for phase in 0..beats {
            let on_phase = changes.iter().filter(|&&i| i % beats == phase).count();
            let mut score = on_phase as f64 / changes.len() as f64 - 1.0 / beats as f64;
            if beats == 4 {
                score += FOUR_PRIOR;
            }
            if score > best.2 {
                best = (beats, phase, score);
            }
        }
    }
    (best.0, best.1)
}
```

Si los acordes cambiaran al azar, un cuarto de los cambios caería en
cada pulso de cuatro, y un tercio en cada pulso de tres. Restar ese
azar, `1.0 / beats`, hace comparables los dos compases: sin eso, agrupar
de tres en tres siempre parecería mejor, ya que el primer pulso de cada
grupo se lleva una parte mayor. Una pequeña ventaja, `FOUR_PRIOR`
(0,05), va al 4/4, ya que la mayor parte de la música popular está en
cuatro. Con menos de cuatro cambios no hay de dónde sacar nada, y la
respuesta es 4/4 desde el primer pulso. El resultado es
`(pulsos por compás, primer tiempo fuerte)`, y el tiempo fuerte es un
índice de pulso: `phase`, el primer pulso de la grabación donde empieza
un compás.

### Los cambios de tonalidad

`bpm-key` da una sola tonalidad para toda la grabación. Para saber
dónde cambia la tonalidad, `keys.rs` le pasa el trabajo a
`lineup_core::harmony::key_finding`, el código que encuentra los cambios
de tonalidad en las notas de una canción (mira [Dónde cambia la
tonalidad](hearing.md#dónde-cambia-la-tonalidad)). Ese
código pide un chroma por compás. Una grabación no tiene notas, pero
tiene el chroma de cada pulso, así que los pulsos se juntan en compases,
y el chroma de cada compás hace de las notas que hay en él:

```rust
fn bar_chroma(beats: &[Frame]) -> key_finding::Chroma {
    let mut sum = [0.0; 12];
    for beat in beats {
        let floor = beat.treble.iter().copied().fold(f32::MAX, f32::min);
        let chroma = beat.treble.map(|v| v - floor);
        for (pc, s) in sum.iter_mut().enumerate() {
            // The note a fifth below `pc` is five semitones above it.
            *s += (chroma[pc] - FIFTH_OVERTONE * chroma[(pc + 5) % 12]).max(0.0);
        }
    }
    sum
}
```

Dos correcciones hacen que el chroma de un sonido se parezca más al de
una partitura. Primero, en un espectro logarítmico toda clase de altura
tiene algo de energía, por el ruido y por los armónicos de otras notas;
quitar la clase de altura más débil de cada pulso, su **piso**, deja las
notas que sobresalen.

Segundo, los armónicos mismos. Una nota tocada en un instrumento no es
una sola frecuencia sino muchas: la nota, el doble, el triple, y así.
El triple de la frecuencia está una octava y una quinta más arriba (una
razón de 3 son 1.902 cents, apenas más de 19 semitonos). Así que una
grabación en Do mayor está llena de Sol (de Do), Re (de Sol) y Si (de
Mi) que nadie tocó, y la tonalidad sale una quinta demasiado alta, Sol
por Do, más a menudo que no. Por eso cada clase de altura pierde 0,3
(`FIFTH_OVERTONE`) de la clase de altura una quinta por debajo. Una
quinta por debajo son siete semitonos hacia abajo, que en un círculo de
doce son cinco hacia arriba: `(pc + 5) % 12`. El comentario lo dice,
porque esa línea es fácil de escribir mal.

Aquí solo cuenta el chroma agudo. Los armónicos de las notas graves
llenan el chroma grave de quintas, y con él la tonalidad acertaba menos
a menudo: el 80% de los pulsos frente al 84%, como deja anotado el
comentario del código.

`key_spans` corta los pulsos en compases con el compás recién
encontrado, y los pulsos antes del primer tiempo fuerte forman un compás
propio; luego le pide a `key_finding` sus tramos, contados en compases.
Cada tramo se lleva después de vuelta a pulsos:

```rust
let bars: Vec<_> = edges
    .windows(2)
    .map(|bar| bar_chroma(&beats[bar[0]..bar[1]]))
    .collect();
key_finding::key_spans(&bars)
    .into_iter()
    .map(|span| KeySpan {
        start: edges[span.start],
        end: edges[span.end],
        ..span
    })
    .collect()
```

`edges` guarda el pulso donde empieza cada compás, y uno más allá del
último pulso. `windows(2)` lo recorre de dos en dos, solapando,
`[0, 2]`, `[2, 6]`, `[6, 10]`..., que es el primer pulso de cada compás y
el del compás siguiente. `KeySpan { start, end, ..span }` es la
**sintaxis de actualización de structs**: un `KeySpan` nuevo con estos
dos campos, y todos los demás (las tonalidades en orden, qué tan
seguro) tomados de `span`.

### El arreglo

`arrange.rs` construye el proyecto. La grabación pasa a ser un canal
sampler con una sola nota, en el primer paso de la canción. El sampler
toca su archivo desde el principio a su propia velocidad, así que el
primer tiempo fuerte de la grabación tiene que caer en una línea de
compás de la canción. Rara vez lo hace, así que la grabación se guarda
de nuevo con silencio delante, la **entrada**: lo justo para que su
primer tiempo fuerte caiga al comienzo del siguiente compás entero.

```rust
pub fn lead_in(analysis: &Analysis) -> LeadIn {
    let downbeat = analysis
        .beats
        .get(analysis.first_downbeat)
        .copied()
        .unwrap_or(0.0)
        .max(0.0);
    let bar = analysis.beat_seconds() * analysis.beats_per_bar as f64;
    let bars = (downbeat / bar).ceil() as usize;
    LeadIn {
        bars,
        seconds: bars as f64 * bar - downbeat,
    }
}
```

A 120 BPM en 4/4 un compás dura 2 s. Un primer tiempo fuerte a 1,3 s se
mueve a 2 s, el comienzo del compás 2: un compás, y 0,7 s de silencio.
`write_wav` guarda la copia, con muestras de coma flotante de 32 bits y
el crate `hound`, junto al original como `song.ferrous-lineup.wav`. Los
pulsos que haya antes del tiempo fuerte caen en ese primer compás, como
una anacrusa en una partitura.

Los acordes pasan a ser **tramos**, un acorde sostenido a lo largo de
pulsos seguidos, en pulsos de la canción. Un pulso de la grabación se
desplaza según la entrada y el tiempo fuerte; uno que caería antes de
que empiece la canción se deja fuera:

```rust
for (beat, chord) in analysis.chords.iter().enumerate() {
    let Some(song_beat) = (beat + offset).checked_sub(analysis.first_downbeat) else {
        continue;
    };
    let Some(chord) = *chord else {
        continue;
    };
    match spans.last_mut() {
        Some(last) if last.chord == chord && last.first_beat + last.beats == song_beat => {
            last.beats += 1;
        }
        _ => spans.push(Span {
            chord,
            first_beat: song_beat,
            beats: 1,
        }),
    }
}
```

`checked_sub` devuelve `None` donde un `-` normal sobre números sin
signo se saldría por debajo de cero, y `let ... else` salta ese pulso.
El `match` sobre `spans.last_mut()` alarga el último tramo cuando el
mismo acorde sigue desde el pulso inmediatamente siguiente, y empieza un
tramo nuevo si no. Un pulso sin acorde termina un tramo, ya que el
acorde siguiente ya no viene justo después.

`follow` hace el proyecto. Fija el tempo, el compás y la tonalidad,
añade el canal sampler, con el nombre del archivo, y un canal de Lineup
Osc llamado «Armonía», y pone las voces de cada tramo con el mismo
`library::voicing::voice` que usa la banda, para que cada acorde se
mueva lo menos posible desde el anterior (mira [Cómo toca la banda una
hoja de acordes](../band/band.md)). Los acordes van a patrones de 16
compases, «Acordes 1», «Acordes 2»..., colocados uno tras otro en una
pista llamada «Armonía»; el patrón de la grabación está en una pista
llamada «Grabación». Un tramo que cruza al patrón siguiente se corta en
dos:

```rust
// A span that crosses into the next pattern is cut in two.
while start < end {
    let index = start / pattern_steps;
    let pattern_end = (index + 1) * pattern_steps;
    let length = end.min(pattern_end) - start;
    if notes_by_pattern.len() <= index {
        notes_by_pattern.resize(index + 1, Vec::new());
    }
    for &pitch in &voicing {
        notes_by_pattern[index].push(NoteEvent {
            velocity: CHORD_VELOCITY,
            ..NoteEvent::new(pitch, start - index * pattern_steps, length)
        });
    }
    start += length;
}
```

Los acordes se tocan con velocidad 80, más suave que la de por defecto,
para quedar detrás de la grabación. Por último, `arrange::key_spans`
lleva los tramos de tonalidad de pulsos a pasos de la canción de la
misma manera, para que la aplicación pueda mostrarlos y marcarlos en la
lista de reproducción.

## La importación en la aplicación

Escuchar una canción tarda unos segundos, así que `app/import.rs` lo
hace en un hilo propio, como la exportación a WAV (mira [Saber cuándo ha
terminado](../sound/export.md#saber-cuándo-ha-terminado)). El closure
del hilo devuelve todo lo que la ventana necesita:

```rust
type Outcome = Result<(Project, String, Vec<KeySpan>), String>;
```

El proyecto, un resumen de lo oído para la barra de estado, y los
cambios de tonalidad en pasos de la canción; o por qué falló. `type` le
da un nombre corto al tipo largo. El resumen es una frase como «canción:
98.2 BPM, 4/4, Mi menor, acordes en 310 de 312 pulsos», y se escribe en
el hilo del trabajo, así que ese hilo toma primero el idioma y los
nombres de las notas de la ventana, que son [ajustes que viven en un
hilo](../sound/export.md#ajustes-que-viven-en-un-hilo):

```rust
let language = crate::i18n::Language::current();
let naming = lineup_core::NoteNaming::current();
self.import = Some(Import {
    worker: thread::spawn(move || {
        language.set();
        naming.set();
        listen(&path)
    }),
});
```

`listen` decodifica, analiza, escribe la copia WAV (en la carpeta
temporal del sistema si no se puede escribir en la carpeta del
original) y construye el proyecto. En cada frame, `check_import` le
pregunta al handle `is_finished()`; cuando lo está, hace `join` del hilo
para obtener su resultado, cambia el proyecto con el mismo
`replace_project` que usa Archivo > Abrir, le da los cambios de
tonalidad a **🔍 Buscar la tonalidad** y empieza a tocar la canción:

```rust
match import.worker.join() {
    Ok(Ok((project, summary, keys))) => {
        self.replace_project(project, None);
        self.key_finder.new_project(&self.project, keys);
        self.notice = Some(summary);
        self.set_mode(PlayMode::Song);
    }
```

La ventana nunca espera, así que la música sigue sonando mientras se
escucha una grabación nueva. `replace_project` también llena la hoja de
acordes con los acordes oídos en las notas de la pista Armonía, como
hace con cualquier proyecto que llega sin acordes (mira
[Escribirlo](hearing.md#escribirlo)).

Abrir otro proyecto mientras una importación escucha la deja caer:
`replace_project` la saca de `self.import`, y la barra de estado dice
**Se detuvo la importación de la grabación.** Soltar un `JoinHandle` no
detiene su hilo; lo **separa** (*detach*). El hilo sigue escuchando
hasta el final, y su resultado se tira, que es lo que se quiere: el
proyecto nuevo se queda, y nada tiene que interrumpir el análisis a
medias.

Una grabación llega a la importación desde el menú o desde la línea de
comandos. Un archivo nombrado en la línea de comandos pasa por
`open_now`, que también abre proyectos, y que pregunta primero a
`lineup_listen::is_audio_file`: una extensión de `AUDIO_EXTENSIONS`,
`["mp3", "wav", "flac", "ogg"]`, en mayúsculas o minúsculas. Un archivo
soltado sobre la ventana no se importa: un sonido soltado ahí es una
muestra para el rack de canales (mira [El
sampler](../sound/instruments.md#el-sampler)).

## Por qué estos crates

Cada trabajo fue a lo que mejor lo hacía con las mismas grabaciones de
prueba: demostraciones de la biblioteca renderizadas a WAV, donde se
conocen el tempo, la tonalidad y el acorde de cada pulso, y unas pocas
grabaciones Creative Commons con los resultados de librosa como
referencia.

- **Decodificar**: Symphonia, en Rust puro, para los cuatro formatos a
  la vez.
- **Pulsos**: nuestro propio módulo `beats`. El crate `beat-track-rs`
  implementa el mismo seguidor, y el módulo da los mismos pulsos en
  todas las grabaciones de prueba, unas diez veces más rápido. El crate
  además decodifica archivos, así que metería una segunda copia de
  Symphonia y el crate `ndarray` en cada compilación, por 250 líneas de
  seguidor.
- **Tonalidad**: `bpm-key`, con los perfiles de Krumhansl y Kessler,
  acertaba más a menudo, y es rápido.
- **Acordes**: ningún crate lo hacía lo bastante bien, así que
  `chroma.rs` y `chords.rs` son nuestros, unas 300 líneas.
- **Compás**: nada de lo disponible lo adivina, de ahí `meter.rs`.

Los seguidores de pulsos neuronales, como `beat-this` y `beatnet-rs`, se
dejaron de lado: necesitan modelos ONNX, así que la importación dejaría
de ser Rust puro mientras corre.

## Qué tan bien funciona

El
[README](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates/lineup-listen)
del crate tiene las tablas, medidas en 29 demostraciones de la
biblioteca renderizadas a WAV:

| | |
|---|---|
| Tempo | correcto en 20, la mitad o el doble en 8, mal en 1 |
| Pulsos sobre la cuadrícula del tempo verdadero | 27 de 29 |
| Tonalidad de toda la canción (`bpm-key`) | 24 de 29 |
| Acordes: fundamental y mayor o menor correctos | 79% de 3.887 pulsos |
| Compás | 3/4 en las 5 demostraciones en tres, 4/4 en las otras 24 |
| Primer tiempo fuerte sobre una línea de compás | 22 de 29 |

La mitad o el doble del tempo es la confusión clásica de todo seguidor
de tempo, y para tocar encima importa poco: los pulsos siguen cayendo en
el pulso. Las canciones enteras de tríadas simples salen bien; los
fallos son acordes de séptima oídos como su tríada o al revés, melodías
tocadas en bloques de acordes, cuyas notas de paso parecen acordes
propios, y los acordes disminuidos de los estilos más jazzísticos.

Para los cambios de tonalidad hay un informe, un test ignorado que
renderiza a audio demostraciones de canciones, todas las que tienen
cambios de tonalidad y una de cada cinco de las demás, y compara las
tonalidades encontradas con las marcas de tonalidad que las
demostraciones tienen de sus partituras:

```sh
cargo test --release -p lineup-listen --test key_changes -- --ignored --nocapture
```

Medido el 30 de octubre de 2026, en 136 canciones, contando una
tonalidad como correcta cuando lo son su tónica y su mayor o menor:

| Tonalidades, desde una grabación | |
|---|---|
| Pulsos en la tonalidad correcta | 73,7% de 36.306 |
| La tonalidad principal de la canción (su tramo más largo en una tonalidad) | 91 de 136 |
| La única tonalidad que da `bpm-key` | 82 de 136 |
| Cambios de tonalidad de las partituras encontrados a menos de dos pulsos | 63 de 315 |
| Canciones en una sola tonalidad donde igual se encuentra un cambio | 3 de 52 |

Desde las notas de una canción, el buscador de tonalidades acierta el
87% de los compases (mira [Qué tan bien
escucha](hearing.md#qué-tan-bien-escucha)); desde el sonido
de las mismas canciones, unos tres pulsos de cada cuatro. Los cambios
rara vez se inventan, pero rara vez se encuentran justo en el pulso: el
chroma de un sonido es más borroso que las notas, y una tonalidad nueva
tiene que durar lo bastante para pagar su cambio. Muchos se encuentran
uno o dos compases tarde, como en *Maple Leaf Rag*, donde el paso a Re♭
mayor a los 120 s se oye a los 123 s. El informe lista cada canción con
sus tonalidades y dónde cambian, las verdaderas y las encontradas: el
lugar donde mirar después.

## Asperezas

- **Una copia de la grabación.** La copia WAV con su entrada existe solo
  porque el sampler siempre toca un sonido desde su primera muestra. Un
  sampler que pudiera empezar un sonido por la mitad la haría
  innecesaria.
- **Tocar solo desde el principio.** La grabación es una nota en el
  primer paso de la canción, y una nota solo suena cuando la
  reproducción llega al paso en que empieza. Si se empieza desde
  cualquier otro lugar, la canción toca los acordes sin la grabación, y
  un salto mientras toca la detiene.
- **Detener.** Un sampler toca su sonido entero pase lo que pase, lo que
  está bien para un golpe de batería y mal para una canción, así que las
  voces de un sonido de más de diez segundos se apagan cuando se detiene
  la reproducción (mira [El
  sampler](../sound/instruments.md#el-sampler)). Diez segundos es una
  regla práctica; un ajuste por canal sería más limpio.
- **Compases.** Solo se adivinan 3/4 y 4/4; la música en 6/8 sale como
  3/4 al doble del tempo.
- **La mezcla entera.** Los acordes salen de todo lo que suena. Una
  melodía de notas sueltas no va acompañada de la armonía que sugiere, y
  una melodía fuerte puede arrastrar el acorde hacia sus propias notas.

## Cómo se prueba

Cada módulo tiene tests unitarios sobre datos hechos en el test. El
seguidor de pulsos recibe veinte segundos de clics a 120 BPM y tiene que
encontrar el tempo con un error de menos de 3 BPM, y cada pulso a menos
de 0,06 s de un clic; el silencio no tiene que dar ninguna cadena de
pulsos; y los filtros mel tienen que cubrir el espectro, solapándose a
la mitad. `arrange.rs` construye un `Analysis` a mano, un pulso cada
medio segundo desde 0,3 s, el tiempo fuerte en el tercero, y comprueba
la entrada, los tramos y el proyecto:

```rust
#[test]
fn chords_join_into_spans_in_song_beats() {
    let analysis = analysis();
    let spans = spans(&analysis, lead_in(&analysis));
    let firsts: Vec<_> = spans.iter().map(|s| (s.first_beat, s.beats)).collect();
    // The two beats before the downbeat land in the lead-in bar.
    assert_eq!(firsts, [(2, 2), (4, 4), (8, 4), (13, 1)]);
}
```

Los acordes son G G C C C C G G G G, ningún acorde, C. Los dos pulsos
de G antes del tiempo fuerte caen en los dos últimos pulsos del compás
de entrada (los pulsos se cuentan desde 0); el C del final es un tramo
propio, después del pulso sin acorde. `decode.rs` comprueba que una
grabación una muestra más larga que diez minutos se rechaza, con el
mensaje que ve el usuario; y la aplicación comprueba que abrir otro
proyecto suelta una importación que todavía estaba escuchando.

Los ejercicios sobre la importación están en [Ejercicios: la teoría
musical en código](exercises.md).
