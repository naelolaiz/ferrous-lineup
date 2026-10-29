# Importar una grabación (experimental)

Este capítulo empezó como un *spike*: una rama que probó si Ferrous
Lineup podía tomar un archivo MP3 o WAV, oír su tempo, tonalidad y
acordes, y tocar una pista de armonía a la vez. Se publicó en la 0.2.0,
y sigue siendo experimental: el tempo, los cambios de tonalidad, los
acordes y el primer tiempo fuerte son estimaciones, no una partitura
leída del sonido.

Puede, lo bastante bien como para ser útil. **Archivo > Importar grabación…**
(o soltar el archivo sobre la ventana, o `ferrous-lineup song.mp3`) abre
la grabación como un proyecto nuevo: la propia grabación en un canal
sampler, y un canal **Armonía** con el acorde oído en cada pulso. El
tempo, el compás y la tonalidad se fijan a partir de lo oído, de modo
que las líneas de compás caen sobre los compases de la grabación y las
teclas de armonía se iluminan con el acorde que está sonando.

![Vibe Ace importado, con las harmony keys siguiendo sus acordes](../images/spike-listen-vibe-ace.png)

El código vive en un crate propio, `lineup-listen`, más un archivo en la
aplicación (`app/import.rs`) y un pequeño cambio en el sampler. Este
capítulo explica cómo funciona y qué se aprendió. El
[`README.md`](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates/lineup-listen)
del crate tiene los números detallados de probar las alternativas.

## El pipeline

Escuchar son seis etapas, cada una con su módulo, cada una una función
normal de un tipo de datos al siguiente:

1. **Decodificar** (`decode.rs`): el archivo se convierte en muestras, con el
   crate [Symphonia](https://crates.io/crates/symphonia).
2. **Pulsos** (`beats.rs`): cuándo cae cada pulso, en segundos, y el
   tempo. La primera versión usaba aquí el crate `beat-track-rs`; una
   vez publicada la importación, el crate fue reemplazado por nuestro
   propio port (mira abajo).
3. **Tonalidad** (con el crate `bpm-key`): la tonalidad en la que está la
   grabación en conjunto, y la primera suposición para las teclas de armonía.
4. **Acordes** (`chroma.rs` y `chords.rs`, propios): qué acorde suena de
   cada pulso al siguiente.
5. **Métrica** (`meter.rs`, propio): tres o cuatro pulsos por compás, y
   qué pulso es el primer tiempo fuerte.
6. **Cambios de tonalidad** (`keys.rs`, propio): la tonalidad de cada compás,
   encontrada con `lineup_core::harmony::key_finding` a partir del
   chroma de los compases. El resultado es un `Vec<KeySpan>` en
   `Analysis.keys`, con `start` y `end` contados en pulsos.

Luego `arrange.rs` convierte el resultado en un `Project`, con los
mismos tipos que usa el resto de Ferrous Lineup: `Channel`, `Pattern`,
`NoteEvent`, `Clip`. `arrange::key_spans` mueve los tramos de tonalidad
de pulsos a pasos de canción, para que el buscador de tonalidades pueda
marcarlos en la lista de reproducción. Nada aguas abajo sabe que las notas vinieron
de una grabación.

### Decodificar con Symphonia

Un archivo de audio es un *contenedor* (WAV, MP3, Ogg...) que guarda una
o más *pistas*, cada una comprimida con un *códec*. Symphonia separa las
dos cosas: sondeas (*probe*) el contenedor, eliges su pista de audio, y
le das los *paquetes* de la pista a un decodificador para su códec. Cada
paquete se decodifica a un bloque de muestras, normalmente entrelazadas
(izquierda, derecha, izquierda, derecha...), que `decode_file` promedia
en un solo canal.

Dos cosas de Rust que notar. Symphonia quiere su entrada como un
`Box<dyn MediaSource>`: un *trait object*, «cualquier tipo que
implementa `MediaSource`, detrás de un puntero». Un `File` es uno;
también lo es un buffer en memoria. El `Box` hace falta porque tipos
distintos tienen tamaños distintos, y `dyn` significa que el método
concreto a llamar se busca en tiempo de ejecución. El bucle del
decodificador también muestra cómo los errores se *emparejan*, no solo
se propagan: un error de I/O al final significa un archivo cortado, así
que nos quedamos con lo que tenemos; un error de decodificación
significa un paquete dañado, así que lo saltamos; cualquier otra cosa se
devuelve con `?`.

Los códecs de Symphonia son *features* de Cargo, activados en el
`Cargo.toml` del workspace: `mp3`, `wav`, `pcm`, `flac`, `ogg` y
`vorbis`. AAC y ALAC también existen, pero se dejan desactivados; una
feature es una línea que añadir.

### De las muestras al chroma

Un acorde es un conjunto de *clases de alturas*: do mayor es Do, Mi y Sol en
cualquier octava. Para encontrar acordes en el sonido necesitamos saber,
momento a momento, con qué fuerza suena cada una de las doce clases de
alturas. Eso se llama el *chroma*, y sale de una transformada de
Fourier.

`chroma::frames` corta la grabación en *tramas* superpuestas de 8 192
muestras, unos 190 ms cada una, empezando una nueva cada 46 ms. Cada
trama se multiplica por una *ventana de Hann*, una joroba suave que
desvanece la trama al entrar y al salir: sin ella, el corte brusco de
cada extremo le parecería a la FFT un clic con todas las frecuencias. La
FFT (del crate `realfft`) dice entonces cómo de fuerte es cada *bin* de
frecuencia en la trama. El bin `k` es la frecuencia `k × sample_rate /
8192`, así que a 44,1 kHz los bins están separados 5,4 Hz, lo bastante
juntos para distinguir Mi2 de Fa2 (82,4 y 87,3 Hz) pero nada más grave, y
por eso la trama es tan larga.

Cada bin pertenece a una nota: la fórmula `69 + 12 × log2(f / 440)`
convierte una frecuencia en un número de nota MIDI (69 es La4, y cada
semitono hacia arriba multiplica la frecuencia por la raíz duodécima de
dos). El redondeo da la nota más cercana, `% 12` su clase de alturas, y
la intensidad del bin se suma a esa clase de alturas. Tres detalles
hacen que esto funcione en la práctica:

- la intensidad se toma como `ln(1 + 100 × magnitude)`, porque oímos en
  una escala logarítmica: una nota suave una octava arriba debe seguir
  contando, y la nota más fuerte no debe ahogar todo lo demás;
- las notas más agudas reciben más bins, ya que están más separadas en
  Hz, así que cada clase de alturas se divide por su número de bins;
- se guardan dos chromas por trama: uno *agudo* (de Do3 a Do7), donde está
  el acorde, y uno *grave* (de Mi1 a Do4), que dice sobre todo qué nota es
  la más baja, y la nota más baja suele ser la fundamental.

La tabla de bins a clases de alturas se construye una sola vez, fuera
del bucle de tramas, como un `Vec<(usize, usize)>`: el bucle entonces no
hace más que sumas.

### Del chroma a los acordes: plantillas y Viterbi

`chords::chords_per_beat` recibe un chroma promediado por pulso (las
tramas entre un pulso y el siguiente) y pone nombre a cada uno. Hay dos
ideas.

**Plantillas.** Cada acorde que podemos reconocer (seis cualidades sobre
doce fundamentales, 72 acordes, más «sin acorde») es una *plantilla*:
doce números, 1 para sus notas y 0 para el resto. El chroma del pulso se
compara con cada plantilla por *correlación*: ambos se desplazan a media
0 y se escalan a longitud 1, y entonces su producto escalar es 1 si
suben y bajan juntos y 0 si no tienen relación. Tres pequeños ajustes:
una nota grave en la fundamental del acorde suma un poco, un acorde que
pertenece a la tonalidad suma un poco (la mayoría de los acordes de una
canción lo hacen, así que la tonalidad es un *prior* útil), y un acorde
de cuatro notas pierde un poco, de modo que gana una tríada sencilla
salvo que la séptima esté de verdad.

**Suavizado.** Tomado en solitario, el mejor acorde de cada pulso salta
de un lado a otro: una nota de paso de la melodía hace que Do parezca Am
durante un pulso. Los acordes reales duran. Así que cambiar de acorde
cuesta una penalización, y el *algoritmo de Viterbi* encuentra la
secuencia de acordes con la mejor puntuación total. Es un algoritmo de
*programación dinámica*: para cada pulso y cada uno de los 73 candidatos
guarda la mejor puntuación de cualquier secuencia que termine ahí,
calculada a partir de los mejores del pulso anterior, y recuerda de
dónde vino. Al final toma el mejor último acorde y recorre hacia atrás
la tabla de «de dónde vino». Con 73 candidatos serían 73 × 73
comparaciones por pulso, pero como cambiar de acorde cuesta lo mismo
desde cualquier sitio, la mejor manera de cambiar a cualquier acorde es
siempre desde el mejor general del pulso anterior, lo que lo deja en 73
por pulso.

En las 29 demos de la biblioteca renderizadas a WAV, donde el acorde de
cada pulso se conoce, esto acierta el nombre del 79% de los pulsos
(fundamental y familia mayor/menor). El crate `chord_detector`, que
implementa un método publicado trama a trama, obtuvo un 68% en las
mismas grabaciones; la diferencia es sobre todo el suavizado y el prior
de tonalidad.

### Pulsos: onsets, autocorrelación y programación dinámica

`beats::track` es el rastreador de pulsos de Ellis (2007), en tres
etapas. Primero la *envolvente de onsets*: un espectrograma en la
*escala mel* (bandas espaciadas como las oye el oído, estrechas abajo y
anchas arriba), en decibelios, y para cada trama la suma del aumento de
cada banda desde la trama anterior. Las disminuciones no cuentan: una
nota que se apaga no es un onset. La envolvente tiene picos en cada
golpe de batería y en cada comienzo de nota.

Luego el tempo: la envolvente se compara consigo misma desplazada cada
posible longitud de pulso (su *autocorrelación*). Con el desplazamiento
correcto, los picos caen sobre picos. Los desplazamientos cercanos a
120 BPM reciben una ventaja inicial, porque si no los rastreadores
eligen la mitad o el doble del tempo con la misma facilidad que el
propio tempo.

Luego los pulsos, otra vez por programación dinámica: la mejor
puntuación para un pulso en la trama `i` es su fuerza de onset más la
mejor puntuación de un pulso más o menos una longitud de pulso antes,
menos una penalización por cuánto se aleja de una longitud de pulso.
Cada trama recuerda el pulso anterior que eligió, y la puntuación final
más fuerte se rastrea hacia atrás hasta el principio. La autocorrelación
solo conoce tramas enteras, así que el tempo que informa está un poco
cuantizado; `lib.rs` ajusta después una línea recta a través de los
tiempos de pulso (`tempo_of`, un ajuste por mínimos cuadrados) para
obtener el tempo al que se fija el proyecto.

Este módulo empezó como el crate `beat-track-rs`, que demostró el
enfoque. Pero el crate también decodifica archivos, así que traía una
segunda copia de Symphonia (una versión más antigua, con todos los
códecs) y `ndarray` a cada compilación. Su rastreador son unas 250
líneas, así que después de publicada la importación se portó a
`beats.rs`, con el estilo propio del proyecto y sobre el crate `realfft`
que el chroma ya usaba. Un programa de comparación ejecutó ambos sobre
las 38 grabaciones de prueba: el mismo tempo y los mismos pulsos en
todas, unas diez veces más rápido, y 285 líneas menos en `Cargo.lock`.
Un crate que demuestra un enfoque y luego es reemplazado por un port
pequeño es un buen resultado para un spike.

### Métrica: dónde están los compases

Los acordes cambian sobre todo en el primer pulso de un compás.
`meter::guess` cuenta, para cada manera de agrupar los pulsos (de tres
en tres o de cuatro en cuatro, empezando en cualquier pulso), cuántos
cambios de acorde caen en el primer pulso de un grupo, y se queda con la
agrupación que más supera al azar. Una pequeña ventaja inicial va al
4/4, ya que la mayoría de la música popular está en cuatro. Distinguía
el 3/4 del 4/4 en las 29 demos y encontró el tiempo fuerte correcto en
22.

### El arreglo

`arrange::follow` construye el proyecto. Los acordes se convierten en
*tramos* (un acorde sostenido a lo largo de pulsos consecutivos) en
pulsos de canción, con las voces puestas por el mismo
`library::voicing::voice` que usan los estilos, de modo que cada acorde
se mueve lo menos posible desde el anterior. Van a patrones de 16
compases, colocados uno tras otro en una pista «Armonía»; un tramo que
cruza el límite de un patrón se corta en dos.

La grabación es un canal sampler con una sola nota, en el primer paso de
la canción. El sampler reproduce un archivo desde su comienzo a su
propia velocidad, así que el primer tiempo fuerte de la grabación debe
caer sobre una línea de compás de la canción. Rara vez lo hace, así que
`arrange::write_wav` guarda la grabación de nuevo con silencio delante,
la *introducción*: exactamente lo justo para que el primer tiempo fuerte
caiga en el comienzo del siguiente compás entero. La copia queda junto a
la original como `song.ferrous-lineup.wav`.

Un cambio fuera del crate nuevo: un sampler reproducía hasta el final de
su sonido pasara lo que pasase, lo que está bien para un golpe de
batería y mal para una canción de tres minutos. `Sampler::release_all`
ahora desvanece las voces cuyo sonido dura más de diez segundos cuando
se detiene la reproducción.

## La importación en la aplicación

`app/import.rs` sigue a `app/export.rs` exactamente: el trabajo corre en
un hilo, la aplicación guarda su `JoinHandle`, y en cada frame
`check_import` pregunta `is_finished()` y, cuando lo está, hace `join()`
al hilo para obtener su resultado, un
`Result<(Project, String, Vec<KeySpan>), String>`: el proyecto, un
resumen de lo oído, y los cambios de tonalidad en pasos de canción. El
hilo de la interfaz nunca se bloquea, así que la música sigue sonando
mientras se escucha una grabación nueva. El proyecto se intercambia con
el mismo `replace_project` que usa Archivo > Abrir, y
`key_finder.new_project` conserva los tramos de tonalidad para
**Buscar la tonalidad**.

Abrir un archivo de audio desde la línea de comandos o soltándolo sobre
la ventana pasa por el mismo sitio que un archivo `.lineup`, `open_now`,
que ahora comprueba primero `lineup_listen::is_audio_file`.

## La elección de los crates

La pregunta útil para un spike no es «¿funciona?» sino «¿qué funciona
mejor, y cómo lo sabes?». Para eso, un programa pequeño renderizó a WAV
29 canciones de demostración de la biblioteca con su tempo, tonalidad y
acordes verdaderos escritos al lado (Ferrous Lineup puede exportar WAV,
y `Project::chord_notes_in_song` pone nombre al acorde en cualquier
paso), y ejecutó a todos los candidatos sobre ellas, más siete
grabaciones Creative Commons con librosa como referencia. El README del
crate tiene las tablas. En resumen:

- **Pulsos**: `beat-track-rs` (el rastreador de pulsos de Ellis, con los
  parámetros de librosa) puso sus pulsos en la cuadrícula en 27 de 29
  canciones y tarda alrededor de un segundo. `stratum-dsp` encuentra el
  tempo con una frecuencia parecida, pero su cuadrícula de pulsos
  deriva. Las confusiones de mitad y doble tempo les pasan a todos, como
  le pasan a cualquier rastreador de tempo. El ganador se portó después
  a `beats.rs`, como se describe arriba.
- **Tonalidad**: `bpm-key` acertó 24 veces de 29, `stratum-dsp` 13.
- **Acordes**: ningún crate lo hizo suficientemente bien, de ahí
  nuestras propias 300 líneas.

## Lo que no se probó

- Rastreadores de pulsos neuronales (`beat-this`, `beatnet-rs`):
  necesitan modelos ONNX, así que la importación ya no sería Rust puro
  en tiempo de ejecución.
- Separar la melodía de los acordes: una melodía tocada en notas sueltas
  no lleva detrás su armonía implícita. Los acordes salen de la mezcla
  completa.

## Ejercicios

1. Las plantillas de acordes cubren seis cualidades. Añade `Sus4` y mira
   qué le hace a la puntuación en una grabación que contenga un acorde
   sus: más candidatos significan más maneras de equivocarse, así que
   puede que la penalización por acordes raros deba crecer.
2. `meter::guess` conoce el 3 y el 4. Añade el 6/8: seis pulsos en dos
   grupos de tres, donde los cambios de acorde caen en los pulsos 1 y 4.
3. La importación guarda una copia de la grabación con una introducción.
   Dale en su lugar un desplazamiento de inicio al sampler (un campo en
   `SampleSource::File`, y una `position` inicial en `Sampler::note_on`)
   y elimina `write_wav`.
