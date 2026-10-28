# Un ritmo nuevo para una canción: oír los acordes

Archivos: [`crates/lineup-core/src/library/listen.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/listen.rs),
[`crates/lineup-core/src/library/apply.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/apply.rs),
[`crates/lineup-app/src/library_browser.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/library_browser.rs)
y [`crates/lineup-app/src/app/library.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/library.rs)

Hasta ahora, elegir un ritmo en la Library solo cambiaba la batería. Pon
un ritmo de reggae bajo una bossa nova y el piano seguía tocando bossa
nova encima. Una banda lo hace mejor cuando alguien dice «hagamos esta
como reggae»: los mismos acordes, una forma nueva de tocarlos todos, el
piano y el bajo también.

Ahora Ferrous Lineup también lo hace, en dos sitios:

- En la Library, un ritmo elegido después de un estilo cambia cómo se
  toca el estilo. «Four chords» con el ritmo «Salsa» sale como un
  montuno de piano sobre un bajo de tumbao.
- Haz clic derecho en un ritmo y elige **Play the whole song in this
  rhythm**. Cada patrón con batería recibe el ritmo nuevo, en cada
  patrón con acordes se vuelven a tocar, y la melodía se queda como
  estaba. Ctrl+Z lo deshace todo en un solo paso.

![Clic derecho en un ritmo de la Library para tocar la canción entera con él.](../images/step-8-rhythm-menu.png)

La parte difícil no es tocar. La biblioteca ya sabe tocar cualquier
acorde en cualquier estilo: eso es lo que hacen `Comping` y `Bass` (mira
[La biblioteca](library.md)). La parte difícil es saber *qué* acordes
tocar, porque un patrón no guarda sus acordes. Guarda notas.

## Qué recuerda un patrón

Cuando la biblioteca escribe un estilo en un patrón, toca los acordes y
los tira. El patrón se queda solo con `NoteEvent`s: una altura, un
comienzo, una longitud, una velocidad. Después, puede que hayas cambiado
notas en el piano roll, o que la canción sea tuya y nunca haya salido de
la biblioteca.

Podríamos haber guardado los acordes junto a las notas. Pero entonces
todos los archivos de proyecto los necesitarían, cada edición en el
piano roll tendría que mantenerlos al día, y las canciones que tocaste a
mano no tendrían ninguno. Así que en su lugar, Ferrous Lineup hace lo
que hace un músico cuando alguien le toca una canción: *escucha* y
deduce los acordes.

## Escuchar: una puntuación para cada acorde

`listen.rs`

Toma un tramo de un patrón, digamos un compás. En él suenan algunas
notas, durante un número de pasos cada una. Para cada acorde que existe
(12 fundamentales por 28 calidades, 336 acordes) preguntamos: ¿cómo de
bien encajan estas notas en este acorde? La respuesta es un número, una
**puntuación**:

```rust
let mut score = inside
    - OUTSIDE * outside
    - stretch.busy * (MISSING * missing + SIZE * size)
    - GAP * gaps
    - SILENT * stretch.quiet;
if stretch.bass == Some(chord.root) {
    score += BASS_ROOT;
}
```

- `inside`: cuántos pasos se oyen notas del acorde. Más es mejor.
- `outside`: cuántos pasos se oyen otras notas. Eso cuenta el doble en
  contra del acorde (`OUTSIDE` es 2), porque una nota equivocada es una
  señal más fuerte que una correcta.
- `missing`: notas del acorde que nadie toca. Un acorde de Do mayor sin
  Mi dentro puede no ser Do mayor.
- `size`: cuántas notas tiene el acorde. Sin esto, un acorde de trecena,
  que tiene casi todas las notas de la escala, encajaría con todo. Los
  acordes pequeños ganan salvo que las notas pidan uno grande.
- `gaps`: cuánto tiempo no se oye cada nota del acorde mientras suenan
  los acordes. Esto es lo que distingue «un compás de Do, luego un
  compás de C7» de «dos compases de C7».
- `quiet`: pasos sin ninguna nota. Un pulso de silencio pertenece al
  acorde que lo rodea; un compás de silencio se oye mejor como ningún
  acorde.
- Una bonificación cuando una nota del bajo empieza el tramo en la
  fundamental del acorde. El resto de la línea de bajo no se escucha,
  porque las líneas de bajo caminantes tocan notas de paso que no
  pertenecen a ningún acorde.

Dos excepciones vienen de cómo se tocan de verdad los acordes. Una
fundamental que solo toca el bajo no está "ausente" (los pianistas se la
dejan al bajo), y tampoco la quinta de un acorde de cuatro notas (los
pianistas la omiten, mira `voicing.rs`). Sin la primera regla, un G7
tocado en un bajo de Alberti que nunca llega a su Sol se oiría como un
acorde de Si disminuido.

Los números (`OUTSIDE`, `MISSING`, `SIZE`...) son constantes al
principio del archivo. Los afinó una prueba,
`every_style_is_heard_as_its_chart`, que toca los 78 estilos de la
biblioteca, cada uno a su manera (arpegios, stabs, valses, bajo
caminante...), los escucha y comprueba que los acordes oídos son los
acordes de la tabla. Cambiar una constante y ejecutar esa prueba es la
forma más rápida de ver qué hace cada una.

### Sumar rápido: sumas de prefijos

Puntuar un tramo necesita saber «cuántos pasos suena cada clase de
alturas entre el paso `a` y el paso `b`». Lo preguntamos para miles de
tramos, así que tiene que ser rápido. El truco es sumar una vez, desde
el principio:

```rust
struct Heard {
    sums: Vec<[f32; 12]>,
    ...
}
```

`sums[s][pc]` es cuántos de los pasos *anteriores* a `s` tienen sonando
la clase de alturas `pc`. Entonces, el tiempo entre `a` y `b` es una
resta, `sums[b][pc] - sums[a][pc]`, por largo que sea el tramo. Esto se
llama **suma de prefijos**, y aparece en todos los sitios donde un
programa pregunta muchas veces lo mismo de «cuánto hay en este rango».

`[f32; 12]` es un array de exactamente doce números, uno por clase de
alturas. Los arrays de tamaño fijo son `Copy` en Rust cuando sus
elementos lo son, así que `let mut next = heard.sums[heard.sums.len() -
1];` copia la última fila, y añadir a `next` deja intacta la fila del
vector.

## Cortar: programación dinámica

Ahora, ¿dónde cambian los acordes? Podríamos probar todas las maneras de
cortar el patrón en tramos y quedarnos con la mejor, pero un patrón de
16 pulsos se puede cortar de 32.768 maneras, y uno de 64 pulsos de unas
nueve mil billones de maneras, más o menos una por cada grano de arena
de la Tierra.

La **programación dinámica** llega a la misma respuesta con mucho menos
trabajo, a partir de una observación: la mejor manera de cortar los
primeros `i` pulsos termina con un último tramo, del pulso `j` al pulso
`i`. Sea cual sea, los cortes anteriores a `j` tienen que ser la mejor
manera de cortar los primeros `j` pulsos (si hubiera una mejor, la
usaríamos). Así que:

```rust
for i in 1..lines.len() {
    best[i].0 = f32::NEG_INFINITY;
    for j in 0..i {
        let (chord, score) = best_chord(&candidates, &heard.between(lines[j], lines[i]));
        let total = best[j].0 + score - cut;
        if total > best[i].0 {
            best[i] = (total, j, chord);
        }
    }
}
```

`best[i]` guarda tres cosas: la mejor puntuación total para los primeros
`i` pulsos, dónde empieza su último tramo (`j`) y el acorde de ese
tramo. Cada una se calcula a partir de las anteriores, que ya se
conocen. Al final, `best[last]` tiene la mejor puntuación para el patrón
entero, y seguir las `j` hacia atrás desde el final lee los cortes en
orden inverso; `stretches.reverse()` los pone en orden.

`f32::NEG_INFINITY` es un float más pequeño que cualquier otro, así que
el primer total real siempre lo supera. Es una forma habitual de empezar
una búsqueda de «encontrar el mayor» sin un caso especial para el primer
candidato.

Cada corte cuesta un poco (`cut`), así que un acorde solo se parte en
dos cuando las notas lo piden claramente. Los cortes en la línea de
compás cuestan menos, los cortes a medio compás un poco más, y los
cortes en los otros pulsos lo máximo, porque así de a menudo cambian ahí
los acordes en la música real.

### Juntar lo que va junto

Dos tramos seguidos pueden salir con el mismo acorde: se juntan. Y un
arpegio toca sus notas de una en una, así que su primer pulso puede
sonar como C y el segundo como C7. El loop que los junta usa **match
guards**, un `if` después de un patrón:

```rust
match joined.last_mut() {
    Some(last) if last.chord == stretch.chord => last.length += stretch.length,
    Some(last) if !half_bar(stretch.start) && grows(last.chord, stretch.chord) => {
        last.chord = bigger(last.chord, stretch.chord);
        last.length += stretch.length;
    }
    _ => joined.push(stretch),
}
```

`last_mut()` devuelve un `Option<&mut HeardChord>`: una referencia a
través de la cual podemos cambiar, así que el último tramo crece en su
sitio. Los brazos se prueban en orden, y una guarda falsa pasa al
siguiente brazo, así que el último brazo, `_`, atrapa todo lo demás.

El resultado es una lista de `HeardChord`s. Cada uno tiene un
`Option<Chord>`: `None` donde no se oye ningún acorde, por ejemplo un
patrón que solo tiene melodía en sus primeros compases.

## Tocarlo otra vez

`apply.rs`

Conocidos los acordes, el resto usa lo que la biblioteca ya tiene.
`feel` encuentra cómo le gusta a un ritmo que se toquen sus acordes: el
comping y la línea de bajo del estilo del que es la batería.

```rust
pub fn feel(rhythm: &Rhythm) -> (Comping, Bass) {
    let same_time = |style: &&Style| style.time == rhythm.time;
    STYLES
        .iter()
        .find(|style| style.rhythm == rhythm.name)
        .or_else(|| STYLES.iter().filter(same_time).find(|style| style.genre == rhythm.genre))
        .or_else(|| STYLES.iter().find(same_time))
        .map_or((Comping::Pulse, Bass::Pulse), |style| (style.comping, style.bass))
}
```

El ritmo «Salsa» es la batería del estilo «Salsa montuno», así que toca
los acordes como un montuno y el bajo como un tumbao. Un ritmo que
ningún estilo usa, como «March», toma prestado un estilo de su género en
su compás, así que un ritmo en 3/4 nunca recibe una manera de tocar
pensada para 4/4.

¿Por qué `&&Style` en `same_time`? `STYLES.iter()` da `&Style`, y
`filter` le entrega a su closure una referencia a cada elemento, así que
una referencia a una referencia. `find` hace lo mismo. Escribir el tipo
permite que un solo closure sirva para ambos.

`recomp` vuelve a tocar entonces los acordes de un patrón:

```rust
let heard = hear_chords(&old_chords, &old_bass, length, project.time_signature);
...
let voicings = voice_progression(&chords);
for (index, (chord, stretch)) in played.iter().enumerate() {
    let next = played[(index + 1) % played.len()].0;
    new_chords.extend(comping.play(&voicings[index], stretch.start, stretch.length));
    new_bass.extend(bass.play(*chord, next, stretch.start, stretch.length));
}
```

Es el mismo loop que `accompany` en `chart.rs`, con los acordes oídos en
lugar de una tabla. Los tramos donde no se oyó ningún acorde conservan
sus notas antiguas, así que una intro sin acordes se queda como estaba.

`use_rhythm` hace una canción entera. Para cada patrón, rehace el
comping de los acordes y escribe la batería nueva. Tres cosas se dejan
en paz a propósito: la melodía (el canal "Lead" nunca se usa para
acordes), el compás y los finales. Un final es un patrón cuyas notas
empiezan todas en su primer paso, como el acorde final mantenido con un
crash con el que terminan las demos. Convertirlo en un skank de reggae
lo estropearía.

(Más tarde, un ritmo en *otro* compás sí llegó a cambiar el compás,
compás a compás: mira [Cambiar la métrica de la
canción](re-metering.md).)

Donde la batería antigua tenía un fill en su último compás, va el fill
del ritmo nuevo. ¿Cómo sabemos que había uno? Un fill es un último
compás que suena como ningún compás anterior:

```rust
let fill = length >= 2 * bar && {
    let last = drums_in(project, pattern, length - bar, bar);
    (0..length - bar)
        .step_by(bar)
        .all(|start| drums_in(project, pattern, start, bar) != last)
};
```

Un bloque `{ ... }` es una expresión en Rust, su valor es su última
línea, así que puede ir después de `&&`. Y como `&&` se detiene en
cuanto el lado izquierdo es `false`, el bloque solo se ejecuta para
patrones de dos compases o más.

![Antes: la demo de bossa nova, sus acordes tocados con el ritmo de bossa.](../images/step-8-recomp-before.png)

![Después de «Play the whole song in this rhythm» con One drop: los mismos acordes, ahora skanks cortos de reggae en los contratiempos, y el tempo al que se suele tocar el ritmo.](../images/step-8-recomp-after.png)

## En la app

`library_browser.rs` y `app/library.rs`

Las acciones de la Library son un `enum`, `LibraryAction`. Tocar una
canción entera en un ritmo es una variante nueva, `RhythmInSong(&'static
Rhythm)`, enviada desde un menú de clic derecho:

```rust
response.context_menu(|ui| {
    if ui.button("Play the whole song in this rhythm").clicked() {
        picked = Some(LibraryAction::RhythmInSong(rhythm));
        ui.close();
    }
});
```

El closure cambia `picked`, una variable que está fuera de él. Rust lo
permite porque `context_menu` toma un `FnOnce`, y el closure toma
prestada `picked` de forma mutable solo mientras se ejecuta.

Añadir una variante a un enum hace que todo `match` sobre él que no la
cubra ya no compile. Eso es una ventaja: el compilador enumera cada
sitio que tiene que decidir qué significa la acción nueva. En
`try_from_library`, la acción nueva se maneja primero y devuelve, y el
`match` antiguo solo necesita un brazo que diga que ahí no puede
ocurrir.

El cambio en sí pasa por `edit()`, como todo cambio desde el paso 5, así
que la canción entera vuelve con un solo Ctrl+Z:

```rust
let change = self.edit("change the rhythm", |project| {
    let change = apply::use_rhythm(project, rhythm);
    ...
    change
});
```

## ¿Cómo de bien escucha?

Los 78 estilos, tocados a su manera, se oyen exactamente bien, y también
las canciones, salvo dos acordes muy cortos: uno que dura un solo pulso
al final mismo de Amazing Grace, y otro que cambia en mitad de un pulso
en el Minuet in G. Tocar cada estilo de todas las maneras de comping con
siete líneas de bajo (unas 14.000 combinaciones) acierta exactamente
alrededor del 91%. La mayoría de los fallos son arpegios sobre un bajo
caminante o tumbao, donde un cambio puede salir un pulso antes o
después: el bajo anticipa el acorde siguiente, y un arpegio muestra sus
notas de una en una. El ejercicio 10 trata de hacerlo mejor.
