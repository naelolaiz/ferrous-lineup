# Ejercicios: el sonido

Estos ejercicios practican esta parte sobre el código real: el
secuenciador, el tempo y el compás, los instrumentos, el mezclador,
MIDI, las exportaciones y la traza. Cada uno dice qué **ya existe**,
para que lo amplíes en lugar de volver a construirlo. Algunos son cosas
que probar en la aplicación y escuchar; otros son piezas de Rust con un
final claro; los que se llaman «Piénsalo» son preguntas, sin código que
escribir.

Haz los de Rust en una rama propia. Ejecuta `cargo test` después de
cada cambio, y `cargo fmt --all --check` y
`cargo clippy --workspace --all-targets -- -D warnings` antes de dar
uno por terminado: la CI ejecuta los tres. Varios ejercicios amplían el
formato del archivo de proyecto; cada uno necesita el siguiente número
de formato libre, así que si haces dos, el segundo toma el número
siguiente al del primero.

## Temporización, tempo y compás

### 1. Un mapa de compases dentro de un patrón (lineup-core, difícil)

La canción ya tiene un mapa de compases: `Meter` guarda la
`TimeSignature` inicial y una lista ordenada de `TimeMarker`s, y
`bar_at`, `time_at` y `click` la leen (ver
[Tempo, compás, swing y sensación](tempo.md)). Un `Pattern`
sigue teniendo un solo `time: Option<TimeSignature>`, así que una parte
que cambia de compás en cada compás se corta en un patrón por cada
tramo de compases.

Haz que un patrón tenga un mapa de compases propio, con cambios en las
barras de compás. `Pattern::length` cuenta pasos, y un compás ya no es
un número fijo de ellos, así que:

- ¿Qué código que le pregunta a `TimeSignature::steps_per_bar` o a
  `Meter::bar_at` por un patrón tendría que preguntarle al mapa del
  patrón: las herramientas de notas, `Project::pattern_clicks`, las
  líneas de compás del piano roll?
- ¿Qué deberían preguntar la lista de reproducción y el piano roll
  cuando un clip empieza a mitad de un patrón?
- ¿Cómo lo tendría en cuenta `format_for`, para que un patrón en un
  solo compás se escriba exactamente como ahora?

Empieza por `Meter` en lugar de inventar un segundo tipo. `bar_at`
recorre los tramos entre marcas uno por uno. ¿Por qué no puede usar
simplemente `partition_point` sobre las marcas, como otras listas
ordenadas del programa? ¿Qué necesitaría guardar junto a cada marca
para que eso fuera posible?

*Practica:* leer una estructura de datos existente antes de ampliarla,
aritmética de compases, mantener los archivos compatibles.

### 2. El swing de oído (la aplicación, fácil)

Abre *Midnight Shuffle* y tócala con el control **Swing** en recto, 50
y 100. ¿Qué instrumento muestra más el cambio? ¿Qué ajuste suena a
marcha, y cuál a shuffle?

Después, calcúlalo. A 96 BPM, ¿cuántos milisegundos después del pulso
empieza la segunda corchea en recto, en 50 y en 100? (Un pulso son 96
ticks, y `swing::delay` dice cuántos ticks tarde llega la segunda
corchea.) A 120 BPM la tercera nota de un tresillo llega 333 ms después
del pulso: ¿por qué la respuesta para 100 no es 333 ms a 96 BPM?
Comprueba tus respuestas con una [traza](trace.md): la
columna `late_ms`.

*Practica:* convertir ticks en tiempo.

### 3. El swing en Rust (lineup-core, medio)

`swing.rs` retrasa la segunda corchea de cada pulso cuando las notas se
*tocan*: lee `delay`, `swung`, `swings_in` y `Project::played_notes`.

- Escribe un test de que una nota sostenida desde la segunda
  semicorchea de un pulso (paso 1) hasta la segunda corchea termina 16
  ticks más tarde con swing completo en un canal de piano, pero no en
  uno de batería. ¿Qué argumento de `swung` marca la diferencia, y por
  qué un golpe de batería no debería alargarse?
- Añade swing de semicorcheas: dentro de cada corchea, retrasa la
  segunda semicorchea de cada par en una cantidad menor,
  elegida por un segundo campo o un enum `SwingUnit`. ¿Qué ticks se
  mueven, y cuánto con swing completo? Escribe un test de que los pares
  de semicorcheas siguen durando lo mismo en total, y de que las notas
  escritas no cambian. ¿Qué debe decir `swings_in` de 6/8, donde las
  semicorcheas también van de dos en dos? ¿Qué más toca el ajuste
  nuevo: el archivo del proyecto, el control, la exportación MIDI?

*Practica:* aritmética con ticks, tests unitarios con números exactos,
mantener separados tocar y editar.

### 4. Compases irregulares de oído (la aplicación, fácil)

- Abre **Ajde Jano** y, en el cuadro Compás, escribe `7` sobre `8`
  agrupado como `2+2+3`, y haz clic en **Usar 2+2+3/8**. La melodía
  conserva sus notas, pero la banda ahora se apoya en otras corcheas.
  ¿Qué pulsos de la melodía caen ahora entre las notas del bajo?
- En la Biblioteca, toca los ritmos **Kalamatianós** y **Rachenitsa**
  uno después del otro. Da palmas al compás: ¿dónde está el pulso largo
  en cada uno?
- Elige `5/8` en el cuadro Compás de una canción en 4/4, con **Batería**
  y **Partes de acordes y bajo** marcadas. ¿Qué ritmo recibe, y cómo
  escribe el cuadro el compás?
- Abre **Süsserl** y mira la regla mientras suena. Cuenta los compases
  de dos y los de tres en la primera parte: ¿coinciden con 2 2 3,
  2 2 3, 2 2 3 3?

*Practica:* oír pulsos agrupados, leer los grupos de una
`TimeSignature`.

### 5. Piénsalo: el swing

- En una canción con swing completo, escribe una corchea con puntillo y
  una semicorchea en el piano roll, y después dos corcheas. ¿Qué par se
  mueve? Muchos músicos de jazz leen las dos cosas igual, como un
  tresillo. ¿Qué necesitaría saber Ferrous Lineup para hacerlo, y por
  qué podría salir mal en una marcha?
- Pasa una canción con swing a 6/8 con el cuadro Compás. ¿Qué pasa con
  el control Swing, y con el sonido? Escribe el mismo groove en 12/8 a
  mano: ¿dónde van las notas largas y las cortas?
- `played_notes` devuelve un `Cow`. ¿Qué cambiaría si devolviera un
  `Vec<NoteEvent>`? Cuenta cuántas notas copiaría `diff` en cada
  edición para una canción de 20 patrones y 8 canales.

## Instrumentos y SoundFonts

### 6. Un cencerro grave (lineup-audio y lineup-core, fácil)

Lee `drums::cowbell`: dos ondas cuadradas a 540 y 800 Hz (a algo menos
de una quinta: 800 / 540 son unos 680 cents, y una quinta, 702), un
filtro de paso alto a 400 Hz, uno de paso bajo a 3 kHz, y dos
decaimientos.

Convierte las dos frecuencias en parámetros
(`cowbell_at(sample_rate, low, high)`) y añade un `Drum::LowCowbell`
una octava más grave. El compilador enumera cada `match` que hay que
actualizar en `drums::render`, `Drum::name` y `general_midi_drum`;
añade también el tambor nuevo a `Drum::ALL` (y cambia su longitud), o
los menús y los tests de batería no lo verán. ¿Como qué nota General
MIDI debería enviarse? (`general_midi::drum_name` las enumera.)

Añadir un tambor cambia también qué proyectos pueden abrir las
versiones anteriores. Lee `format_for` y `Drum::is_new_in_format_3`:
¿basta esa comprobación para un tambor añadido ahora, o el archivo
necesita el siguiente número de formato?

*Practica:* enums, `match` exhaustivo, un cambio que cruza crates.

### 7. Un sintetizador legato (lineup-audio, más difícil)

El Lineup Osc toca cada nota en una voz propia. Dale un modo mono,
*legato*: solo suena una voz, y una nota nueva cambia la altura de esa
voz sin reiniciar la envolvente. Añade un tiempo de `glide` para que la
altura se deslice de la nota vieja a la nueva (usa `dsp::Smoothed` para
la frecuencia). ¿Cómo cambia `pick_voice`? ¿Qué partes de la máquina de
estados `Stage` se saltan? ¿Dónde viviría el ajuste, y cómo llega al
hilo de audio?

*Practica:* máquinas de estados, reutilizar piezas pequeñas de DSP.

### 8. Un clic en los contratiempos (lineup-audio, medio)

El metrónomo marca los pulsos, más agudo en el primer pulso de cada
compás: `Project::played_clicks` y `pattern_clicks` construyen un
`Click` (`None`, `Beat` o `Bar`) para cada paso, `metronome::click_at`
elige uno, y `ClickVoice` hace el sonido. Añade un ajuste de
**subdivisión** (corcheas, tresillos o semicorcheas) que lo haga sonar
también entre los pulsos, más suave que el pulso y conservando el
acento del compás. Decide si la subdivisión va en `Click` o en un
segundo ajuste enviado al hilo de audio. ¿Debería la cuenta previa
quedarse en el pulso?

El clic no debe estar en la exportación WAV ni en la exportación MIDI.
¿De dónde sale cada una, y cuál se llevaría el clic por accidente si
viviera en un canal?

*Practica:* un ajuste nuevo de punta a punta, sonido en tiempo real sin
reservar memoria.

### 9. Otra SoundFont (la aplicación, fácil)

Descarga otra SoundFont General MIDI en formato `.sf2` (*FluidR3_GM*, o
la pequeña *TimGM6mb*), elígela con **Elegir SoundFont…** y toca el
Canon en Re. ¿Qué cuerdas prefieres? ¿Suena una SoundFont mucho más
fuerte que la otra? ¿Dónde pondrías un ajuste de volumen para la
SoundFont entera: en `Sounds`, en el hilo de audio, o en cada
`SoundFontPlayer`?

*Practica:* escuchar, encontrar dónde va un ajuste.

### 10. Un instrumento para la Melodía (lineup-core, fácil)

Las series dodecafónicas y los micromodos escriben su melodía en un
canal llamado Lead (Melodía, en la aplicación en español), que
`library::apply::synth_named` crea sin programa General MIDI, así que
se queda con el Lineup Osc mientras los Teclados reciben un piano y el
Bajo un bajo. Elige un instrumento para ella en el rack y escucha. ¿Qué
instrumento le darías a la Melodía por defecto, y dónde iría en
`synth_named`? ¿Qué tests te dirían que las canciones, que eligen su
propia Melodía, siguen conservando la suya?

*Practica:* leer un `match` sobre nombres, valores por defecto.

### 11. Piénsalo: los instrumentos

- Una SoundFont suena en estéreo, pero `SoundFontPlayer::next_sample`
  junta izquierda y derecha en una sola muestra antes del balance del
  canal. ¿Qué haría falta para mantener separados su lado izquierdo y
  su lado derecho, y qué debería hacer entonces el Balance: mover los
  dos lados, o bajar uno?
- rustysynth tiene una reverberación y un chorus propios, y
  `enable_reverb_and_chorus` los apaga. Enciéndelos y escucha. Cada
  canal tendría entonces su propia sala: ¿qué cuesta eso, comparado con
  la [reverberación compartida del mezclador](mixer.md), y
  por qué las mesas de mezcla usan un envío?
- La exportación va a 44.100 frames por segundo aunque tu tarjeta de
  sonido vaya a 48.000. Mira cómo se hace la batería (`drums::render`
  recibe la frecuencia de muestreo) y cómo toca el sampler un sonido
  grabado a otra frecuencia. ¿Podría una exportación sonar distinta de
  lo que oyes?

## El mezclador y los efectos

### 12. Una ley de balance de potencia constante (lineup-core, fácil)

`Channel::pan_gains` mantiene completo el lado cercano y baja el lado
lejano en línea recta. Cámbiala por una ley de *potencia constante*:
con `angle = (pan + 1) × π / 4`, las ganancias son
`[angle.cos(), angle.sin()]`. ¿Qué tests fallan, y por qué? Mueve un
canal de I100 a D100 con cada ley y escucha: ¿cuál suena igual de
fuerte todo el recorrido, y cuál suena más fuerte en el centro?

*Practica:* trigonometría sobre muestras, tests que fijan un diseño.

### 13. Aritmética de medidores (lineup-app y lineup-audio, fácil)

- Un nivel de 0,1 son ¿cuántos decibelios? ¿Hasta dónde se ilumina el
  medidor? Calcúlalo, y después compruébalo con `lit` y `FLOOR_DB` en
  `meters.rs`.
- `levels::raise` compara `peak > 0.0` en lugar de `peak >= 0.0`. ¿Qué
  haría `fetch_max` con los bits de −0,0, o de un NaN? (Mira cómo los
  ordena `f32::to_bits` como enteros.)

*Practica:* decibelios, números de coma flotante como bits.

### 14. Delays y compresores de oído (la aplicación, fácil)

- Pon un delay de corchea con puntillo con un 30 % de mezcla en la
  Melodía de una demostración, y tócala a 60 BPM y a 140 BPM. ¿Cuántos
  milisegundos separan los ecos cada vez? Compruébalo con
  `Delay::seconds`.
- Pon un compresor a −30 dB y 10:1 en la batería. Después sube su
  **Ganancia** hasta que la batería suene tan fuerte como antes. ¿Qué
  pasó con la diferencia entre los golpes fuertes y los suaves? ¿Por qué
  una batería con un compresor fuerte suena «aplastada»?

*Practica:* escuchar lo que hace un efecto.

### 15. Una campana más ancha o más estrecha (lineup-core, lineup-audio y lineup-app, medio)

La banda central del ecualizador es una campana de ancho fijo, una Q de
1 (`Biquad::set` en `lineup-audio/src/effects.rs`). Añade un `width` a
`Equalizer` y un deslizador para él, y úsalo en `Biquad::set`. ¿Qué dice
el Cookbook que vale alpha para una Q dada? Una canción que fija el
ancho necesita el siguiente número de formato libre: enséñaselo a
`format_for`, y deja el campo fuera del archivo cuando tiene su valor
por defecto.

Después escribe un test al estilo de los que hay: un seno en el centro
de la campana se refuerza en la ganancia, y uno dos octavas más lejos se
refuerza menos con una campana estrecha que con una ancha.

*Practica:* la fórmula de un filtro, un campo guardado nuevo de punta a
punta.

### 16. Un envío de delay compartido (lineup-core, lineup-audio y lineup-app, difícil)

Cada canal ya tiene un delay propio entre sus efectos. Añade un segundo
**envío** junto al de la reverberación, a un único delay que comparten
todos los canales, para que varios canales hagan eco juntos. Esto
atraviesa el archivo del proyecto, el hilo de audio y la ventana del
mezclador.

1. En `Channel`, añade `delay_send: f32`, por defecto `0.0`, e inclúyelo
   en las comprobaciones de rango y en `has_plain_mix`. Dale el valor
   por defecto en cada constructor.
2. En `Mixer`, añade los ajustes propios del delay: el tiempo en pasos
   (para que siga al tempo, como `Delay`), la realimentación y la
   mezcla. Valídalos en `Mixer::valid`.
3. Añade `Command::SetSharedDelay` y `Command::SetDelaySend`, y un delay
   compartido que reserve su búfer antes de llegar al hilo de audio,
   nunca en `process`. La `DelayLine` del delay de canal muestra cómo.
   Sigue el modelo de la reverberación para el envío.
4. Añade un deslizador **Delay** a cada línea del mezclador, junto a
   **Reverberación**, y una pequeña sección del delay junto a los
   controles de la reverberación.
5. Enséñale a `format_for` que un envío de delay distinto de cero o
   unos ajustes del delay distintos de los de por defecto necesitan el
   siguiente número de formato libre, y deja los campos fuera del texto
   cuando tienen sus valores por defecto.
6. Prueba el camino del audio (un envío de 0 no hace eco, uno mayor que
   0 sí) y el archivo (un archivo viejo se abre con envío 0; una canción
   con envío de delay se escribe en el formato nuevo y se lee igual).

Terminado cuando el deslizador se oye en la aplicación y en una
exportación WAV, el hilo de audio no reserva nada mientras corre, y una
canción sin el delay se sigue escribiendo en el formato de antes.

*Practica:* todo lo de esta parte a la vez.

### 17. El orden de partitura (la aplicación y lineup-core, fácil)

- Abre *In the Mood* y después *Sing, Sing, Sing*, con el mezclador
  abierto. ¿En qué sección está el canal «Vientos» de cada canción? Mira
  la columna del instrumento para ver por qué.
- Abre *Danny Boy*, haz clic derecho en su Melodía en el rack de
  canales y elige **Instrumento > Flautas > 74 Flauta**. ¿A dónde se
  mueve en el mezclador?
- En `score_order.rs`, los saxofones están entre los clarinetes y los
  fagotes, como en la mayoría de las partituras de orquesta. En una
  partitura de big band los saxofones van primero, encima de las
  trompetas. ¿Qué cambiarías para tener el orden de big band, y qué test
  te diría que rompiste el de orquesta?

*Practica:* datos que deciden una distribución.

### 18. Piénsalo: la dinámica

- El `Follower` del compresor sube en 10 ms (`ATTACK_SECONDS`) en lugar
  de seguir cada onda. Escribe un test que pase un seno de 50 Hz por un
  compresor cuyo seguidor sea mucho más rápido, y mide cuánto se aleja
  la forma de la salida de un seno. ¿Qué te dice eso de los 10 ms?
- Muchas mesas permiten que un compresor escuche un canal mientras baja
  otro: un bajo que se agacha cada vez que suena el bombo («compresión
  side-chain»). Esboza lo que necesitarían los efectos de canal para
  eso. ¿La muestra de qué canal leería el seguidor, y en qué orden
  tendría que mezclar los canales el hilo de audio?

## MIDI

### 19. Teclas de octava (lineup-app, fácil)

El [teclado del equipo](midi.md#el-teclado-del-equipo) cubre
de Do4 a Mi6. Añade dos teclas que lo bajen y lo suban una octava, como
`-` y `=` (`Key::Minus` y `Key::Equals`), que todavía no tocan ninguna
nota. Mantén el desplazamiento entre Do1 y Do7, y muéstralo en el menú
MIDI, junto a **Teclado del equipo como piano**.

El caso difícil: mantén Z, pulsa `=`, y después suelta Z. ¿Qué nota debe
terminar? Mira `typing_down`, que guarda la nota que tocó cada tecla, no
solo la tecla. ¿Por qué se escribió así, si la nota podría volver a
calcularse a partir de la tecla?

*Practica:* aritmética de `Note`, recordar lo que se hizo en lugar de
volver a calcularlo.

### 20. Teclear más suave y más fuerte (lineup-app, fácil)

Las teclas tecleadas suenan con `DEFAULT_VELOCITY`, así que las tomas
grabadas con el teclado del equipo tienen todas la misma velocidad,
mientras que las de un teclado MIDI conservan la fuerza con que se tocó
cada tecla. Añade un deslizador «Velocidad del teclado» al menú MIDI
(`egui::Slider::new(&mut value, 1..=127)`), y haz que Mayúsculas toque
a 127 mientras se mantiene pulsada.

Mayúsculas no cambia `physical_key`, así que el piano sigue
funcionando. Pero mira el filtro de `typing_keyboard`: una tecla que
bajó con Mayúsculas, ¿sube bien cuando se suelta antes Mayúsculas?

*Practica:* deslizadores de egui, modificadores del teclado.

### 21. Notas a través del bucle (lineup-core, medio)

[`recorded_note`](midi.md#cuantizar-recorded_note) tiene dos
límites. Una nota sostenida a través del final del bucle se corta ahí,
y una tecla sostenida más de un bucle entero se graba como una nota
corta.

- Haz que una tecla sostenida a través del final del bucle grabe **dos**
  notas: una hasta el final del patrón, y otra desde el paso 0 para el
  resto. La función devuelve entonces un `Vec<NoteEvent>` (o un array y
  una cuenta). Escribe antes los tests: (14, 2) en un patrón de 16
  pasos da (14, 2) y (0, 2).
- Para las notas largas, la posición sola no basta. Haz que el hilo de
  audio cuente también cuántas veces ha dado la vuelta el bucle, o que
  publique el número de pasos tocados desde Reproducir en lugar de la
  posición en el bucle. ¿Qué es más sencillo de compartir con un solo
  `AtomicU64`? ¿Cuál puede seguir usando `finish_recording`?

*Practica:* cambiar el tipo que devuelve una función y a quienes la
llaman, tests como especificación.

### 22. Grabar en modo canción (los tres crates, más difícil)

Grabar pasa a modo patrón, porque una toma va al patrón que se repite.
En modo canción, la posición es un paso de la *canción*, así que una
tecla podría ir al clip que suena en ese momento, en
`step - clip.start`.

Resuelve las preguntas antes de escribir código. Dos clips pueden sonar
a la vez, en pistas distintas: ¿cuál se lleva la nota? ¿Y si no suena
ningún clip ahí: debería crearse uno nuevo? Un clip puede sonar varias
veces en la canción, y grabar en uno de ellos los cambia todos, porque
son el mismo patrón. ¿Es eso lo que querrías? ¿Cómo lo resuelven otros
estudios que conoces?

*Practica:* diseñar el comportamiento antes que el código.

### 23. Las otras piezas de un archivo de formato 2 (lineup-core, más difícil)

`midi_file::read::read` convierte los bytes de un archivo en eventos, y
`midi_file::import::import` hace un proyecto con ellos. Faltan todavía
dos rincones.

Un archivo de **formato 2** no es una canción sino varias piezas
independientes, una por pista. `import` se queda solo con la primera
pista. Añade una forma de elegir una pieza: enumerarlas en el aviso,
preguntar en un diálogo, o importar cada una como un proyecto propio.
¿Qué deberían decir los tests sobre un archivo de formato 2 con tres
piezas?

El otro es el meta evento de **desplazamiento SMPTE** (`FF 54`): cinco
bytes que dicen dónde empieza una pista en un código de tiempo, para
música escrita sobre una película. La temporización SMPTE de la
cabecera (`Timing::PerSecond`) ya funciona; este evento se salta y
`meta_event` lo descarta. Léelo y decide qué debería hacer con él un
proyecto sin video. Prueba los dos casos con archivos pequeños
construidos en el módulo de tests, como hacen los tests que hay.

*Practica:* analizar datos binarios con patrones de slice, diseñar una
elección.

### 24. Compensar la latencia (lineup-audio y lineup-app, más difícil)

La [latencia](midi.md#latencia-lo-que-oyes-ya-pasó) hace que
cada tecla grabada llegue un poco tarde. cpal sabe cuánto: el callback
de audio recibe un `&cpal::OutputCallbackInfo`, cuyo `timestamp()` tiene
dos instantes, `callback` (ahora) y `playback` (cuándo se oirá este
bloque). La diferencia entre ellos es la latencia de salida.

El callback ignora ese argumento (`_: &cpal::OutputCallbackInfo` en
`build_stream`). Pásale la latencia al hilo de audio, publícala como un
número de pasos en otro atómico, y réstala de la posición de cada tecla
antes de que `recorded_note` la redondee. Una posición justo después del
paso 0 menos la latencia debe dar la vuelta hasta el final del bucle, no
bajar de cero.

Después piensa en la otra dirección. Los mensajes de la Salida MIDI
salen cuando se hace su bloque, pero el sonido de los instrumentos
incorporados de ese mismo bloque se oye un poco más tarde. Así que un
sintetizador externo toca ligeramente *antes* que el resto. ¿Cómo podría
el hilo de envío retrasar cada mensaje en la latencia? ¿Qué tendría que
llevar cada mensaje?

*Practica:* leer la documentación de un crate
(`cargo doc --open -p cpal`), el tiempo como dato.

### 25. Un test de note-off MIDI (lineup-audio, medio)

Demuestra que **Detener** envía note-offs para las notas que el
secuenciador empezó en un canal de Salida MIDI. Lee
`midi_out_channels_send_their_notes` y `stopping_releases_held_notes`
en el módulo de tests del motor, que tiene las funciones auxiliares
`add_midi_out`, `received` y `note_off`. Añade
`stopping_releases_midi_out_notes`: una nota larga en el canal MIDI 3,
**Reproducir**, renderiza un paso y comprueba el NoteOn; envía
**Detener**, renderiza unos frames y comprueba el NoteOff. Haz la nota
lo bastante larga para que no pueda haber terminado por su propio gate.

Ejecuta el test una vez sin su línea `Command::Stop`, para verlo fallar,
y después vuelve a poner la línea. Un test que nunca has visto fallar
puede no estar probando nada.

*Practica:* el arnés de tests del motor, tests de regresión.

### 26. La Salida MIDI desde los menús (la aplicación, fácil)

- Envía el Bajo de una demostración con banda por MIDI en el canal 2, y
  después aplica otro estilo desde la Biblioteca. ¿Cuántos canales de
  Bajo hay después? Lee `find_synth` en `library/apply.rs` para ver
  por qué.
- ¿Qué tambor General MIDI es la nota 60? Un tambor escrito en Do4 y
  enviado tal como está escrito, en lugar de como su propia nota, lo
  tocaría. Encuéntralo con `general_midi::drum_name`.
- `Channel::send_over_midi` limita el canal con `.min(15)` en lugar de
  devolver un error. ¿Dónde más se limita así un número que viene de un
  menú o de un archivo, y dónde se rechaza en cambio? ¿Qué es mejor para
  un número leído de un archivo?

*Practica:* leer código para explicar un comportamiento.

### 27. Piénsalo: MIDI

- La cola en vivo guarda 256 mensajes, y `LiveInput::send` devuelve
  `false` cuando está llena. `on_message` lo ignora. ¿Qué es peor
  perder, una tecla que baja o una que sube? ¿Podría un diseño hacer
  imposible perder una tecla que sube, por ejemplo compartiendo *qué
  teclas están bajadas* (128 bits) en lugar de enviar eventos?
- El hilo de envío se despierta mil veces por segundo, aunque ningún
  canal se envíe por MIDI. ¿Podría dormir más cuando no hay nada que
  enviar? ¿Qué costaría dormir más cuando sí lo hay?
- `release_live_keys` suelta todas las teclas cuando se inserta o se
  quita un canal. La otra forma sería renumerar `live_keys` para seguir
  a los canales. ¿Qué es más fácil de hacer bien? ¿Notaría alguna vez la
  diferencia quien toca?
- midir le da a `on_message` una marca de tiempo en microsegundos, que
  el código ignora en favor de la posición del hilo de audio. ¿Cuándo
  sería más precisa la marca de midir? ¿Por qué no se pueden comparar
  sin más los dos relojes?
- Un patrón tocado por un canal de Salida MIDI y exportado como archivo
  `.mid` envía las mismas notas en los dos casos. ¿Podrían la
  exportación y el hilo de audio compartir más código? ¿Qué necesitaría
  saber ese código sobre el tiempo?

## Exportar

### 28. Exportar un patrón como audio (lineup-app, fácil)

**Archivo > Exportar patrón como MIDI…** escribe el patrón
seleccionado como archivo MIDI. Añade lo mismo como *audio*: un
elemento **Exportar patrón como WAV…**, que guarde el patrón
seleccionado, tocado cuatro veces, como archivo WAV.

El plan evidente es el modo patrón: `SetMode(PlayMode::Pattern)`,
`SelectPattern`, y renderizar cuatro veces la duración del patrón más
la cola. Pero el modo patrón repite el patrón para siempre, así que la
cola contendría el patrón empezando por quinta vez: el problema que
[la exportación](export.md#pasos-en-silencio-después-del-final)
resuelve con pasos en silencio, y el modo patrón no tiene una duración
de canción que alargar. En su lugar, haz una *copia* del proyecto,
reemplaza su lista de reproducción por cuatro clips del patrón en una
pista, y exporta esa copia con el código que ya tienes. ¿Qué tiene que
cambiar en `song_render_with` para que pueda renderizar un proyecto
distinto de `self.project`?

*Practica:* trabajar sobre una copia, hacer una función más general.

### 29. Exportar la región de bucle (lineup-app, medio)

Las canciones pueden tener una región de bucle, y el modo canción toca
solo esa parte. Añade **Archivo > Exportar región de bucle…**, que
escriba solo la región en un archivo WAV.
`Project::playback_arrangement()` ya devuelve exactamente la parte que
toca el modo canción, con las repeticiones desplegadas. Haz que
`song_render_with` reciba un `Arrangement` en lugar de llamar siempre a
`project.arrangement()`, y calcula el número de frames a partir de la
duración y los tempos de ese arreglo. ¿Cuál debería ser el nombre del
archivo? ¿Qué debería hacer el elemento del menú cuando no hay región
de bucle?

*Practica:* reutilizar un cálculo, pensar en una entrada vacía.

### 30. Stems: un archivo por canal (lineup-app, medio)

Los músicos a menudo quieren *stems*: la canción exportada una vez por
canal, cada archivo con solo ese canal, para mezclarla en otro sitio.
Añade **Archivo > Exportar stems…**, que pida una carpeta y escriba
`Kick.wav`, `Clap.wav` y así sucesivamente.

Cada stem es una exportación con todos los demás canales silenciados
(`Command::SetMuted`). Renderizarlos uno tras otro es lo más sencillo;
renderizarlos en varios hilos a la vez es más rápido en un ordenador
con varios núcleos. El canvas de la exportación de video ya pinta
franjas de filas con `std::thread::scope`: ¿podrían hacer lo mismo los
stems, o necesitan `thread::spawn`, dado que la ventana no debe
esperarlos? ¿Cómo debería mostrar una sola barra de progreso varias
exportaciones? ¿Y qué pasa con dos canales con el mismo nombre, o con
un nombre que lleva una `/`?

*Practica:* hilos, nombres de archivo a partir de texto del usuario.

### 31. Dos vistas en un video, y una comprobación de sincronía (la aplicación, medio)

1. Abre **El Choclo** y elige **Archivo > Exportar WAV…**; guarda
   `el-choclo.wav`.
2. Elige **Archivo > Exportar video…**, marca **Hoja de acordes** y
   **Piano roll**, ponlas una al lado de la otra en la vista previa,
   elige **1280 × 720**, y exporta `el-choclo.mp4`.
3. Si tienes `ffprobe` instalado, compara las duraciones:

   ```sh
   ffprobe -v error -show_entries format=duration -of default=nw=1:nk=1 el-choclo.wav
   ffprobe -v error -show_entries format=duration -of default=nw=1:nk=1 el-choclo.mp4
   ```

4. Mira un compás donde cambia el acorde. El compás enmarcado en la
   hoja de acordes, el cursor en el piano roll y el sonido deberían
   coincidir ahí. Comprueba un segundo punto, como el primer compás
   después de una repetición.

Terminado cuando las duraciones no difieren en más de una o dos
imágenes, las dos vistas muestran el mismo compás en el mismo momento,
y puedes decir por qué `start_video` dibuja cada imagen **antes** de
renderizar el sonido que suena bajo ella.

*Practica:* comprobar un diseño con herramientas externas.

### 32. Piénsalo: exportar

- El contador de progreso usa `Ordering::Relaxed`. Supón que la ventana
  abriera el archivo en el momento en que el contador llega al total,
  sin esperar a `join`. ¿Qué podría encontrar? (Mira lo que hace
  `export_wav_cancellable` después de su última llamada a `progress`.)
  ¿Lo arreglaría un `Ordering` más fuerte, o el problema está en otro
  sitio?
- El sonido de un video es MP3 a 256 kbit/s, mientras que el archivo
  WAV son las muestras exactas. ¿Podría el video llevar las muestras sin
  cambios? ¿Qué caja MP4 cambiaría, y qué reproductores dejarían de
  reproducirlo?
- `repeat_picture` hace que la última imagen dure más cuando nada
  cambió. ¿Qué saldría mal si la escena comparara *píxeles* en lugar de
  triángulos para decidirlo? ¿Cuánto costaría?

## La traza de reproducción

### 33. La traza de un compás insertado y borrado (la aplicación y una terminal, fácil)

La hoja de acordes y la lista de reproducción tienen **Insertar un
compás antes**, **Insertar un compás después** y **Eliminar este
compás**; `--trace` escribe cada nota que toca el reproductor.

1. Escribe una traza de una demostración:

   ```sh
   cargo run --release -- --trace "El Choclo" --out before.tsv
   ```

2. Abre la misma demostración, guárdala como `inserted.lineup`, haz
   clic derecho en el compás 3 de la hoja de acordes y elige **Insertar
   un compás después**. Guarda, y escribe la traza del archivo guardado:

   ```sh
   cargo run --release -- --trace inserted.lineup --out after.tsv
   ```

3. Compara las dos tablas, sin las líneas `#`:

   ```sh
   diff <(grep -v '^#' before.tsv) <(grep -v '^#' after.tsv) | head -40
   ```

   Cada nota que empezaba en el compás 4 o después debería estar ahora
   un compás más tarde; las notas de los compases 1 a 3 no deberían
   cambiar.
4. Borra el compás que insertaste (es el compás 4), guarda, vuelve a
   escribir la traza y compárala con `before.tsv`.

Terminado cuando la traza con el compás insertado muestra el
desplazamiento de un compás en `start`, `bar` y `step`, la del compás
borrado coincide con la primera, y puedes señalar el código de
`bars.rs` que mueve los clips, las marcas y las secciones.

*Practica:* comprobar una edición por lo que toca.

### 34. Leer trazas (una terminal, fácil)

- Escribe la traza de una demostración con un ritardando (una marca de
  tempo que se desliza) y lee las líneas `# tempo`. ¿Cuántas hay, y por
  qué una por paso?
- Sube la **Soltura** en una canción, escribe su traza y usa `awk` para
  encontrar la nota más tardía (la columna `late_ms`). ¿Llega alguna vez
  más tarde de lo que dice el ajuste?
- Con **Escribir un registro mientras suena** activado en la aplicación,
  toca un patrón, silencia su canal en mitad de una nota larga y
  detén. Busca la nota en el archivo: ¿qué dice su columna `end`?

*Practica:* `awk` sobre tablas, leer un informe.

### 35. Piénsalo: el trazador

- El trazador busca las notas cuya duración se acabó al principio de
  cada frame, antes de que el tempo pueda cambiar en ese frame. ¿Qué
  saldría mal con una nota que termina en el mismo frame que un cambio
  de tempo, si mirara después?
- Una traza pierde eventos cuando su cola está llena, y dice cuántos.
  ¿Podría en cambio conservar algunos eventos que importan más (los
  `NoteOff`, por ejemplo) y descartar antes otros? ¿Con qué tendría que
  lidiar entonces el lector?
