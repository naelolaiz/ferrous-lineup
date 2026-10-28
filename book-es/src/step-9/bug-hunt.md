# Una segunda cacería de errores: crasheos, escritura de notas y el pulso

Archivos: [`crates/lineup-core/src/library/comping.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/comping.rs),
[`crates/lineup-core/src/pitch.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/pitch.rs),
[`crates/lineup-core/src/harmony/scale.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/scale.rs),
[`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-core/src/band/form.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/form.rs),
[`crates/lineup-core/src/ear_training.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/ear_training.rs),
[`crates/lineup-core/src/lead_sheet/rhythm.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/lead_sheet/rhythm.rs),
[`crates/lineup-core/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/chord_sheet.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs),
[`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs)
y [`crates/lineup-app/src/app/practice.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/practice.rs)

Con los trece puntos del plan hechos, la pregunta fue otra vez lo
contrario de una funcionalidad: *¿qué se puede mejorar, y qué está
roto?* Este capítulo recorre los dieciocho bugs que se arreglaron, un
commit cada uno, y por qué estaba cada uno. Como en la [primera
cacería de errores](../step-8/bug-hunt.md), lo interesante rara vez es
el arreglo, que a menudo es una línea, sino la razón por la que
faltaba la línea.

## Cómo se hizo la cacería

La app corrió en una pantalla virtual (`Xvfb`) y se manejó con
`xdotool`, como antes. Cada parte del código recibió una pregunta
distinta:

- la banda: ¿toca cada estilo todo tipo de compás, incluidos N.C.,
  pushes y métricas irregulares?
- la teoría: ¿es cada nombre que la app escribe (notas, acordes,
  numerales, armaduras) el que escribiría un músico?
- la reproducción: ¿deja cada edición hecha mientras suena el sonido y
  el cabezal de reproducción donde deben estar?
- las ventanas: ¿va cada tecla a la ventana que estás mirando?

Un bug contaba como encontrado cuando se lo **veía**: reproducido en
la app corriendo, o con una prueba sobre el código tal como estaba.
Cada arreglo vino después con una prueba que falla sin él. Vale la
pena hacerlo incluso para un arreglo de una línea: la prueba es lo que
impide que el mismo bug vuelva la próxima vez que cambie el código a
su alrededor.

## Crasheos y trabajo perdido

### N.C. cerraba la app

Teclea N.C. en un compás con la banda **Pop ballad**, y la ventana se
cerraba. Lo hacían dieciséis estilos: todos aquellos cuyo piano o bajo
toca el acorde **una nota cada vez** (arpegios, el bajo Alberti, el
montuno de la salsa). Esos patrones piden «la nota número *n* del
acorde», y suben una octava y dan la vuelta cuando *n* es más grande
que el acorde:

```rust
Voices::Nth(index) => {
    let octave = (index / voicing.len()) as i32;
    voicing[index % voicing.len()]
```

N.C. es una armonía sin notas, así que `voicing.len()` es 0, y
`index / 0` es una división por cero. En Rust una división entera por
cero no es un número raro, como puede ser en otros sitios: el programa
**hace panic** (se detiene con un error), porque no hay una respuesta
correcta que dar. El arreglo es un *match guard*, un `if` sobre un
patrón, puesto antes del caso general:

```rust
Voices::Nth(_) if voicing.is_empty() => Vec::new(),
```

Los brazos del match se prueban en orden, así que un voicing vacío
nunca llega al `%`. Una prueba toca ahora un compás de N.C. en
**cada** estilo de banda y comprueba que ninguna nota de piano, bajo o
guitarra empieza en él.

### Un doble sostenido no se podía volver a abrir

En Sol♯ menor armónica el séptimo grado es Fa doble sostenido (la
séptima elevada de Sol♯ menor: Fa♯ elevado una vez más), y el acorde
sobre él es **F##dim7**: Fa♯♯ La♯ Do♯ Mi. La app escribía ese nombre al
guardar, pero el lector de notas solo leía **un** sostenido o bemol,
así que al abrir el archivo leía «F#» y luego «#dim7», que no es un
acorde, y el archivo no se abría. Nadie teclea F##, pero la app sí,
así que debe leer lo que escribe. `NoteName::read` ahora acepta hasta
dos signos, mientras vayan en el mismo sentido:

```rust
let mut accidental = 0;
for _ in 0..2 {
    let mut chars = rest.chars();
    let step = chars.next().map_or(0, sign);
    if step == 0 || (accidental != 0 && step != accidental) {
        break;
    }
    accidental += step;
    rest = chars.as_str();
}
```

`rest` es un `&str`, una vista dentro del texto, y `chars.as_str()` es
la parte aún no leída; mover `rest` hacia delante no lee nada dos
veces ni copia nada. «Fa♯b» se detiene tras el sostenido (un sostenido
y un bemol se anulan, y nadie los escribe juntos), y «Ebbb» lee Ebb y
deja una «b». Una prueba escribe ahora cada letra con cada alteración
de bb a \#\# dentro de un nombre de acorde, con bajos con barra, y lo
vuelve a leer.

### Las teclas del chord sheet movían las notas del piano roll

Con el chord sheet abierto sobre el piano roll, la flecha derecha
movía cada nota del piano roll un paso, y el compás seleccionado no se
movía. Ambas ventanas escuchan las flechas, Delete y Ctrl+A, y el
piano roll decidía que tenía el teclado comprobando si el clic estaba
dentro de su propio rectángulo. Un clic en el chord sheet *encima* de
él también está dentro de ese rectángulo.

egui dibuja en **capas**, y puede decir cuál está encima en un punto:
`ctx.layer_id_at(pos)`. Así que el piano roll ahora pregunta eso:

```rust
if let Some(pos) = press {
    match ctx.layer_id_at(pos) {
        Some(layer) if layer.order > egui::Order::Middle => {}
        layer => {
            self.has_keyboard = layer.is_some_and(|layer| layer.id == Self::id());
        }
    }
}
```

Las ventanas están en el orden `Middle`; los menús y tooltips están
encima, en `Foreground` y `Tooltip`. Un clic en un menú no cambia nada
(el menú se abrió desde algún sitio, y esa ventana conserva el
teclado), un clic en la propia capa del piano roll le da el teclado, y
un clic en cualquier otra ventana se lo quita.

### Borrar un canal congelaba a la banda

La banda sabe qué partes editaste guardando una **huella** de cada
parte que escribió: un hash de cada nota, incluido el número del canal
en el que está. Una parte cuyas notas ya no coinciden fue editada por
ti, y se deja en paz («congelada»). Pero borrar un canal numera cada
canal posterior uno más abajo, así que cada parte posterior ya no
coincidía, y toda la banda dejaba de seguir el chord sheet.

El proyecto ya tenía una manera de decir «este cambio mueve las notas
de la banda, pero no es una edición»: `moving_the_band`, usado cuando
la canción cambia de tonalidad o de compás. Anota qué partes seguían
el chord sheet, ejecuta el cambio y toma las huellas nuevas como
propias de la banda. Borrar un canal pasa ahora por un primo suyo,
`removing_channel`, que hace lo mismo con cada parte **que no tocaba
nada en el canal borrado**. La parte que sí tocaba ahí perdió esas
notas, así que sigue congelada, y está bien: tú se las quitaste.

Las huellas se guardan en el archivo, así que la otra manera de
arreglarlo, hacer hash de algo distinto del número de canal, habría
hecho que cada banda de cada canción guardada pareciera editada la
primera vez que se abriera. El arreglo elegido conserva las huellas
como estaban.

### Otra canción recibía el compás en el que tecleabas

Teclear en un compás del chord sheet y pulsar **Ctrl+N** (o abrir un
archivo, una demo o una importación) dejaba el campo de texto abierto
sobre la canción nueva, y Enter escribía entonces el texto viejo en
ella. Cuando se reemplaza el proyecto, el chord sheet olvida ahora su
selección y lo que se estaba tecleando (`ChordSheet::new_song`), como
ya hacía el piano roll.

## Teoría equivocada

### Fa locrio tiene seis bemoles

Fa locrio es el séptimo modo de Sol♭ mayor: **Fa Sol♭ La♭ Si♭ Do♭ Re♭ Mi♭**. Su
armadura es la de Sol♭ mayor, seis bemoles. La app lo escribía con seis
sostenidos, la armadura de Fa♯ mayor, porque elegía sostenidos o
bemoles por el *sonido* de la tonalidad mayor relacionada, y Fa♯ y Sol♭
son las mismas teclas del piano. Todos los demás modos de esa
tonalidad se nombran con sostenidos (Sol♯ dórico, Si lidio, Re♯ menor),
así que la regla es correcta para ellos; Fa locrio es el único modo
cuya tónica, Fa, es un natural que solo encaja entre bemoles. Una
prueba comprueba ahora, para cada modo sobre cada tónica, que los
sostenidos y bemoles con los que se escriben las notas de la escala
suman su armadura.

### Los numerales siguen el nombre de la nota

En Do mayor, C#°7 y Db7 suenan en la misma tecla del piano, pero son
acordes distintos en la música:

- **C#°7** (Do♯ Mi Sol Si♭) es una tónica elevada, **#i°7**: sube un
  semitono hasta Dm, como acorde de paso entre I y ii.
- **Db7** (Re♭ Fa La♭ Do♭) es **bII7**: el «sustituto tritonal» de G7, que
  se desliza un semitono abajo hasta Do.

Los rótulos del chord sheet leían el grado desde el sonido, así que
ambos salían en el segundo grado («bii°7», «bII7»). El grado es
cuestión de **nombres de nota**: Do♯ lleva el nombre Do, el primer grado,
elevado; Re♭ lleva el nombre Re, el segundo, bajado. `Scale::spelled_numeral`
cuenta ahora nombres desde el de la tónica hasta la fundamental
escrita del acorde, y los rótulos y tooltips lo usan. En Sol♯ menor
armónica, F##dim7 es vii°7: Fa es el séptimo nombre desde Sol.

### La intro de la banda se escribe desde el primer acorde

La intro es un turnaround hacia el primer acorde, VI, iiø, V7: hacia
Ebm es **Cbmaj7 Fm7b5 Bb7**. La app escribía **Bmaj7 E#m7b5 A#7**, los
mismos sonidos escritos en Re♯ menor, porque la tónica de la tonalidad
se nombra por su nombre habitual (Re♯ menor) y la intro se escribía en
ella. Una canción que empieza en Ebm se escribe en bemoles, así que la
intro ahora se escribe desde la letra del primer acorde.

### bVII a IV: la fundamental sube una quinta

La ayuda para bVII a IV decía «la fundamental baja una quinta». De Si♭
subiendo hasta Fa hay cinco nombres de nota (Si♭ Do Re Mi Fa) y siete semitonos: una
quinta **hacia arriba** (o una cuarta hacia abajo). Es un movimiento
plagal, como IV a I (Fa subiendo hasta Do también es una quinta), e I,
bVII, IV, I es la cadencia «doble plagal» de mucho rock: dos pasos
plagales seguidos. La ayuda ahora lo dice.

### Ir a la cromática y volver conserva cada nota

Elegir la escala cromática como tonalidad y luego volver a Do mayor
sacaba los sostenidos y bemoles de la canción: D7 (Re Fa♯ La Do) se
convertía en Dm7. Mover notas de una escala a otra lleva cada nota de
la escala vieja al mismo grado de la nueva (de Do mayor a Do menor, Mi
baja a Mi♭). La escala cromática tiene todas las notas, así que al
volver de ella, cada nota era «de la escala vieja», se emparejaba con
un grado de Do mayor y era arrastrada a él. La escala cromática no
tiene grados que emparejar, así que entrar en ella o salir de ella
ahora solo mueve las notas a la nueva fundamental.

### Una serie o un conjunto como tonalidad no mueve los cambios de tonalidad

Hacer de una serie dodecafónica o de un conjunto de clases de alturas
la tonalidad no mueve ninguna nota (es otra manera de nombrarlas, como
explica el [capítulo sobre los conjuntos como
tonalidad](../step-8/key-as-a-set.md)). Pero los marcadores de
tonalidad de la canción sí se movían: una sección en Sol mayor de una
canción en Do mayor subía a Si con la primera nota de la serie. Los
marcadores de tonalidad siguen ahora la misma regla que las notas
(`notes_follow`): cuando las notas se quedan, los cambios de tonalidad
también se quedan.

## Práctica y aprendizaje

### El entrenamiento auditivo delataba la pregunta siguiente

Cada pregunta saca sus notas de un pequeño generador de números
aleatorios (SplitMix64) arrancado con una **semilla**, y la semilla de
la pregunta siguiente era el siguiente estado interno del generador.
Así que la segunda pregunta sacaba los mismos números que la primera,
desplazados en uno: su primer sorteo (el tipo del acorde) era el
segundo sorteo de la pregunta anterior (su fundamental). Después de Do,
Mi o La♭ el acorde siguiente era siempre mayor. Nada parecía raro en
ninguna pregunta suelta; solo una prueba que hacía miles seguidas, y
contaba lo que venía después de un Do, lo mostró.

La semilla siguiente es ahora una de las **salidas** del generador,
que quedan mezcladas lejos de su estado:

```rust
pub fn next_seed(seed: u64) -> u64 {
    Random(seed).next()
}
```

La lección es general: una semilla no debe ser algo a lo que el propio
generador llegará un paso después.

En una tonalidad menor, las preguntas de progresión también decían
«los acordes después de I». El acorde de tónica de una tonalidad menor
es menor, escrito en minúscula, **i**; la pregunta nombra ahora la
tónica como la tonalidad.

### El tap tempo cuenta los clics

En 6/8 el metrónomo hace clic en la **negra con puntillo**, dos clics
por compás, pero el tap tempo tomaba cada toque como una negra. Marcar
el ritmo con los clics de una canción a ♩ = 120 (una negra con
puntillo a 80 por minuto) fijaba el tempo en 80. El tap tempo toma
ahora la longitud del pulso en el que hace clic el metrónomo, en
negras:

```rust
let quarters = time.steps_per_bar() as f64 / beats as f64 / f64::from(STEPS_PER_BEAT);
```

y el tempo es `60 × quarters × taps / seconds`. En 6/8 un pulso son
1,5 negras, así que toques separados 0,75 segundos dan 120. En 3/8 los
clics son corcheas, media negra, lo que antes doblaba el tempo; en 5/8
y 7/8, cuyos pulsos son de dos longitudes, es el pulso medio.

### Los silencios en 6/8

Un compás de 6/8 con una corchea y luego silencio durante el resto del
primer pulso se escribía «♪ 𝄽»: una corchea y un silencio de negra. En
compás compuesto un pulso es una negra con puntillo, tres corcheas, y
la regla del grabado es que la notación muestre dónde están los
tercios de cada pulso. Un silencio de negra puede cubrir las **dos
primeras** corcheas de un pulso, pero no las dos últimas, así que la
escritura correcta es «♪ 𝄾 𝄾», dos silencios de corchea. El lead sheet
sigue ahora esa regla en 6/8, 9/8 y 12/8.

## Reproducción

### El slow blues toca cuatro pulsos de tres

El slow blues en 12/8 («Trouble in Mind», y el estilo Slow blues)
tiene cuatro pulsos por compás, cada uno una negra con puntillo
partida en tres corcheas: el aire de tresillo de la balada de blues.
Su bajo tocaba seis notas por compás, una cada negra: tres notas
contra cada dos pulsos, lo que cruza los pulsos en lugar de marcarlos.
El estilo tiene dos patrones nuevos para ello:

- el bajo toca **largo-corto** en cada pulso, una negra y luego una
  corchea, con la nota corta llevando al siguiente pulso;
- el piano golpea el acorde en **cada corchea**, tres por pulso, la
  primera de cada tres la más fuerte.

Esto se comprobó de oído: el bajo solo se renderizó a audio, y sus
notas empiezan en cada pulso y en el último tercio de cada pulso, ocho
por compás.

### Cambiar entre modo pattern y modo song no soltaba ninguna nota

Cambiar entre modo pattern y modo song mientras sonaba reiniciaba el
secuenciador, pero no enviaba note-offs, así que un pad o una nota
enviada a un sintetizador MIDI seguía sonando. El motor suelta ahora
cada nota sostenida cuando cambia el modo mientras toca, como ya hacía
tras un seek.

### Quitar un loop conservaba el lugar del cabezal de reproducción

Mientras el modo song repite en loop los compases 5 a 8, el cabezal de
reproducción cuenta desde el inicio del loop. Quitar el loop con su
botón **Looping bars 5–8 ✖** ponía el cabezal en esa cuenta desde el
compás 1: detenido en el compás 7, se iba al compás 3. La casilla de
loop de la barra de práctica ya volvía a sumar el inicio del loop; el
botón del playlist ahora hace lo mismo.

### Un N.C. con push ganaba un punto

«N.C.» tiene puntos en su nombre, y los puntos después de una armonía
son una parada (C. es un shot). El lector lo sabía para N.C., pero no
para un **^N.C.** con push: leía el último punto como un shot, y lo
escribía de vuelta como «^N.C..». Cada copia y pegado añadía un punto.
Los signos de intercalación de un push ahora se quitan primero, y lo
que queda se comprueba como N.C. antes de leer ningún punto.

## Ejercicios

1. `Voices::Nth(n)` da la vuelta con `n % voicing.len()`. Escribe una
   función `pick(voicing: &[u8], n: usize) -> Option<u8>` que devuelva
   `None` para un voicing vacío en lugar de hacer panic, usando
   `slice::get` y `checked_rem`.
2. Escribe el acorde de séptima sobre cada grado de Sol♯ menor armónica.
   ¿Cuáles necesitan un doble sostenido?
3. En Re♭ mayor, ¿cuáles son los numerales romanos de D°7 y de Ebb7?
   Cuenta los nombres de nota desde Re♭.
4. Escribe I, bVII, IV, I, la cadencia doble plagal, en Sol mayor y en Mi
   mayor.
5. Tap tempo en 9/8: ¿cuántas negras es un pulso, y qué tempo dan
   toques separados un segundo?
