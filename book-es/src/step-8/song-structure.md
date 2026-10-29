# La estructura de la canción: secciones, repeticiones y bucles

Archivos: [`crates/lineup-core/src/playlist.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/playlist.rs),
[`crates/lineup-core/src/arrangement.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/arrangement.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs)
y [`crates/lineup-app/src/playlist/sections.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/playlist/sections.rs)

Hasta ahora, la lista de reproducción solo conocía clips. Una canción tiene más forma
que eso: una introducción, una estrofa, un estribillo que se oye dos
veces, un beat que se repite en bucle durante ocho compases. Para conseguir
cualquiera de esas cosas colocabas cada clip a mano, y nada en la
pantalla decía «esto es el estribillo». Esta página añade tres cosas:

- **Secciones**: tramos con nombre de la canción (Introducción, Estrofa, Estribillo…)
  en una franja encima de los números de compás. Una sección puede
  tocarse varias veces seguidas: se dibuja una vez y se marca «×2».
- **Repeticiones de clips**: arrastra el borde derecho de un clip y toca
  su patrón una y otra vez, como al estirar un clip de patrón en otros
  estudios basados en patrones.
- **Una región de loop**: arrastra por los números de compás, y el modo
  canción repite en bucle solo esa parte mientras trabajas en ella.

![El estribillo está marcado «×2». Mientras la canción lo toca por segunda vez, la sección dice «▶ 2 de 2».](../images/step-8-sections-playing.png)

## Cómo usarlo

- Arrastra por la franja oscura marcada **Secciones** para marcar una
  sección. Recibe el primer nombre común que no esté usado aún: Introducción, luego Estrofa, Estribillo, Puente, Coda. **➕ Sección** en la barra de
  herramientas de la lista de reproducción añade una después de la última.
- Arrastra una sección para moverla, o su borde derecho para cambiarle
  el tamaño. Se ajusta como los clips (a compases, pulsos o pasos).
- Haz clic en una sección (botón izquierdo o derecho) para ver su menú:
  escribe un nombre o elige uno, decide cuántas veces se toca con **−** y
  **+**, duplícala, escribe sus repeticiones o bórrala. **Repetir esta sección** está marcado mientras el modo canción repite en bucle solo esa
  sección; márcalo para repetir en bucle la sección, desmárcalo para volver
  a tocar la canción entera. Sin bucle, el modo canción toca la canción
  entera y luego vuelve a empezar desde el compás 1.
- Arrastra un clip por su borde derecho (el puntero se convierte en una
  flecha doble) para repetir su patrón.
- Arrastra por los números de compás para repetir en bucle esa parte. El
  botón verde **Repitiendo compases…** de la barra de herramientas lo muestra;
  haz clic en él para volver a tocar la canción entera.

Las secciones no poseen clips. Una sección es solo un nombre y un rango
de pasos: los clips que estén debajo forman parte de ella. Borrar una
sección deja los clips en paz.

![El menú de una sección.](../images/step-8-section-menu.png)

## Dos tipos de paso

Aquí está la única idea que hace sencillo todo lo demás. Una vez que una
sección puede repetirse, la lista de reproducción tal como se *dibuja* y la canción
tal como se *toca* ya no miden lo mismo:

```text
timeline:   | Intro | Chorus ×2 | Outro |
canción:    | Intro | Chorus | Chorus | Outro |
```

Así que el código habla ahora de dos tipos de paso:

- un **paso de timeline** es un lugar de la lista de reproducción, tal como se dibuja;
- un **paso de canción** es un momento de la canción, tal como se oye.

`Project::song_length` sigue siendo lo que era, la longitud del timeline.
Un tipo nuevo, `Arrangement`, es la canción tal como se oye:

```rust
pub struct Arrangement {
    /// The pieces of patterns to play, in song steps.
    pub plays: Vec<Play>,
    /// The stretches of the timeline, in the order they play.
    pub passes: Vec<Pass>,
    /// How long the song is, in song steps.
    pub length: usize,
}

pub struct Play {
    pub pattern: usize,
    pub start: usize,  // the song step it starts on
    pub skip: usize,   // how far into the clip it starts
    pub length: usize, // how many steps it lasts
}
```

Un `Pass` dice «los pasos de timeline 32 a 63 suenan en los pasos de
canción 64 a 95, y esta es la ronda 2 de 2 de la sección 1». Un `Play`
dice «toca el patrón 3 desde el paso de canción 64 durante 32 pasos,
empezando 0 pasos dentro de él». El patrón se repite en bucle mientras dura el
`Play`, así que un clip repetido tres veces es un `Play` de tres patrones
de largo. `skip` no es cero cuando una sección o la región de bucle
empieza a mitad de un clip.

`Project::arrangement()` lo construye para la canción entera (las
exportaciones lo usan), y `Project::playback_arrangement()` para lo que
toca el modo canción: la región de bucle si la hay, la canción entera si
no.

### Construir el arreglo

`arrange` en `arrangement.rs` corta el timeline en trozos en los bordes
de las secciones. Las secciones suenan `repeats` veces; los huecos entre
ellas suenan una vez:

```rust
fn pieces(project: &Project, from: usize, to: usize)
    -> Vec<(usize, usize, Option<usize>, usize)>
{
    let mut pieces = Vec::new();
    let mut cursor = from;
    for (index, section) in project.playlist().sections().iter().enumerate() {
        if section.end() <= cursor || section.start >= to {
            continue;
        }
        if section.start > cursor {
            pieces.push((cursor, section.start, None, 1)); // a gap
        }
        let start = section.start.max(cursor);
        let end = section.end().min(to);
        pieces.push((start, end, Some(index), section.repeats));
        cursor = end;
    }
    if cursor < to {
        pieces.push((cursor, to, None, 1));
    }
    pieces
}
```

Esto solo funciona porque las secciones se mantienen **ordenadas y sin
solaparse nunca**. Eso es una regla, así que, como con los clips, la
lista es privada y solo los métodos de `Project` la cambian.
`add_section` y `set_section` comprueban la regla y rechazan una sección
que la rompería; `insert_section` mantiene el orden con
`partition_point`, una búsqueda binaria que devuelve el primer lugar
donde la condición deja de ser cierta:

```rust
let index = sections.partition_point(|other| other.start < section.start);
sections.insert(index, section);
```

Luego, para cada trozo y cada pasada, todo clip que se solape con el
trozo añade un `Play` para la parte de él que está dentro del trozo.
`max` y `min` encajan el clip en el trozo:

```rust
let begin = clip.start.max(piece_from);
let end = clip_end.min(piece_to);
if begin < end {
    arrangement.plays.push(Play {
        pattern: clip.pattern,
        start: start + (begin - piece_from),
        skip: begin - clip.start,
        length: end - begin,
    });
}
```

## El hilo de audio no se enteró de las secciones

El hilo de audio recibía antes los clips y tocaba cada patrón desde el
principio del clip. Ahora `Command::SetSong` lleva `plays` en lugar de
`clips`, y lo único nuevo que hace el hilo de audio es un resto:

```rust
pub fn pattern_step(&self, step: usize, pattern_length: usize) -> Option<usize> {
    if pattern_length == 0 || !(self.start..self.end()).contains(&step) {
        return None;
    }
    Some((self.skip + step - self.start) % pattern_length)
}
```

`%` envuelve el paso alrededor del patrón, que es todo lo que significa
«repetir». Todo lo de nombres, rondas y bucles se queda en `lineup-core`,
donde puede probarse sin tarjeta de sonido. El hilo de audio sigue siendo
pequeño, y sigue sin reservar memoria: el nuevo `Vec<Play>` se construye
en el hilo de la UI y el viejo se devuelve para liberarlo allí, igual que
ocurría con la lista de clips.

La exportación a WAV y la exportación a MIDI usan el mismo
`Arrangement`, así que las tres coinciden en qué es la canción. El
escritor de MIDI necesita la pregunta contraria: ¿en qué pasos de canción
suena una nota del paso de patrón 2? Eso es `Play::song_steps_of`, que
usa un iterador con `step_by`:

```rust
(first..self.length)
    .step_by(pattern_length.max(1))
    .map(move |offset| self.start + offset)
```

## De vuelta de la canción a la lista de reproducción

El hilo de audio cuenta pasos de canción, así que el cabezal de
reproducción que comunica es un paso de canción. Para dibujarlo, la app
lo convierte de nuevo en un paso de timeline con los passes del arreglo:

```rust
fn song_position(&self) -> Option<(usize, Pass)> {
    if self.mode != PlayMode::Song {
        return None;
    }
    let step = self.playhead()?;
    let song = self.project.playback_arrangement();
    let pass = *song.pass_at(step)?;
    Some((pass.from + (step - pass.start), pass))
}
```

El `Pass` viaja con él hasta la lista de reproducción, que es como una sección sabe
que debe mostrar «▶ 2 de 2». El rack de canales, el piano roll y las teclas de armonía reciben el paso de timeline, así que muestran el paso correcto del
clip correcto también durante una repetición.

## Guardar: formato 4

`Clip` y `Section` tienen las dos un campo `repeats`, y la mayoría de las
veces es 1. serde puede dejar un campo fuera del archivo cuando tiene su
valor habitual, y volver a rellenarlo al leer un archivo que no lo tiene:

```rust
#[serde(default = "once", skip_serializing_if = "is_once")]
pub repeats: usize,
```

`default` nombra una función que devuelve el valor que usar, y
`skip_serializing_if` una función que dice cuándo dejar el campo fuera.
Las listas nuevas del archivo (`sections`, `loop_region`) usan el
`#[serde(default)]` normal con `Vec::is_empty` y `Option::is_none`. Así
que un proyecto que no usa nada de esto se escribe byte a byte como
antes, y un Ferrous Lineup más antiguo lo abre.

Una versión más antigua también abriría un proyecto que *sí* usa
repeticiones: se salta los campos que no conoce. Pero tocaría cada
repetición una vez, que es una canción que suena mal sin una palabra de
aviso. Así que, siguiendo la regla del paso 7, `format_for` devuelve
**4** para cualquier proyecto con secciones, una región de bucle o un clip
repetido, y las versiones antiguas lo rechazan con un mensaje claro.

## Dibujarlo: un módulo dentro de un módulo

`playlist.rs` ya era largo, así que las franjas se fueron a su propio
archivo. Un módulo de archivo puede tener hijos: `mod sections;` dentro
de `playlist.rs` carga `playlist/sections.rs`. Un módulo hijo puede ver
los elementos privados de su padre, así que `sections.rs` usa `Layout`,
`Snap` y `PlaylistAction` de `playlist.rs` con
`use super::{Layout, PlaylistAction, Snap};` sin hacerlos públicos para
el resto de la app.

Cada sección es su propia zona clicable, hecha con `ui.interact` sobre un
rectángulo que elegimos nosotros, que devuelve un `Response` como
cualquier widget:

```rust
let response = ui.interact(rect, ui.scope_id().with(("section", index)),
                           Sense::click_and_drag());
```

El id debe ser distinto para cada sección y el mismo de un frame a otro,
así que se construye a partir del número de la sección. El menú es un
`egui::Popup` atado a esa respuesta, que se abre con un clic y solo se
cierra con un clic fuera, porque contiene un campo de texto en el que
escribes.

Arrastrar una sección no cambia el proyecto en cada frame, como hace
arrastrar un clip. Solo se dibuja un contorno, y el proyecto cambia una
vez, cuando el botón sube. Una sección se mueve dentro de una lista
ordenada: si el movimiento de un frame la saltara por encima de una
vecina, su número cambiaría a mitad del arrastre, y el siguiente frame
movería la sección equivocada.

## Escribir las repeticiones

**Escribir las repeticiones** en el menú de una sección convierte «Estribillo ×3» en Estribillo, Estribillo 2 y Estribillo 3, uno detrás de otro, cada uno tocado
una vez, para que puedas cambiar el tercer estribillo sin cambiar los
otros. La canción suena igual antes y después; una prueba lo comprueba
comparando los dos arreglos.

![Antes (abajo) y después (arriba) de escribir las repeticiones de «Chorus ×2».](../images/step-8-write-out.png)

Comparte su código con **Duplicar**: `make_room` mueve todo lo que hay
después de la sección más tarde, y luego los clips que empiezan dentro de
la sección se copian en el hueco. Una cosa que hay que vigilar: los clips
se copian enteros. El relleno de la imagen empieza dentro del estribillo
y sigue más allá de su final, así que el original y su copia se solapan.
Ferrous Lineup todavía no puede cortar un clip en dos (un clip no tiene
«empieza a esta distancia dentro del patrón»), que es el ejercicio 3 de
abajo.

![Arrastrar por los números de compás hace loop de los compases 3 a 6.](../images/step-8-loop.png)

## Ejercicios

1. Añade una entrada **Eliminar con los clips** al menú de la sección que
   quite la sección, los clips que empiezan dentro de ella, y cierre el
   hueco. `make_room` muestra cómo mover todo lo que hay después de un
   punto.
2. Muestra la longitud real de la canción, con cada repetición, junto al
   tempo: «8 compases dibujados, 12 tocados, 0:22».
   `Project::arrangement().length` y `Tempo` tienen todo lo que
   necesitas.
3. Da a `Clip` un `offset` (cuántos pasos dentro de su patrón empieza)
   para que un clip pueda cortarse en dos. `Play::skip` ya toca un clip
   desde la mitad; el trabajo está en la lista de reproducción y en
   `duplicate_section`.
4. Muchos programas de música también pueden *marcar* un punto en el
   tiempo sin un rango, por ejemplo para saltar a él. Añade marcadores a
   la franja, y una tecla que mueva el cabezal de reproducción al
   siguiente.
