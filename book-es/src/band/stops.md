# Golpes, sostenidos, silencios y anticipaciones

Archivos: [`crates/lineup-core/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/chord_sheet.rs),
[`crates/lineup-core/src/band/stops.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/stops.rs),
[`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs)
y [`crates/lineup-app/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet.rs)

Un estilo toca el mismo groove compás tras compás, y eso es lo que lo
hace un estilo. Pero escucha a una banda de verdad: al final de una
frase todos golpean un acorde juntos y se detienen en seco, o
sostienen un acorde bajo quien canta, o dejan de tocar un compás para
que el baterista o el solista quede solo, o entran con el acorde
siguiente un poco antes del pulso. Los arreglistas escriben esos
momentos en la partitura, y son lo que hace que una banda suene
arreglada y no en bucle. La hoja de acordes tiene marcas para ellos, y
las partituras de las canciones de demostración usan las mismas
marcas.

![La hoja de acordes con C, ^F, G7., Am.. en la primera fila y F G7..., ^^C, Dm7, G7 en la segunda. Bajo cada armonía, su número y su marca: «IV · push», «V7 · shot», «vi · hold», «V7 · rest». El compás 3 tiene un clic derecho, y el menú «Band on G7» está abierto: «As the style goes», «Shot (.)» (elegido), «Hold (..)», «Rest (...)», luego «On the beat» (elegido), «Push (^)» y «Push a sixteenth (^^)»](../images/step-9-stops-menu.png)

## Probarlo

Pulsa **Ctrl+N** para una canción nueva con banda, y escribe estos
compases en la hoja de acordes, uno en cada compás:

```text
| C | ^F | G7. | Am.. | F G7... | ^^C | Dm7 | G7 |
```

Bajo cada armonía marcada, una palabra dice lo que hace la banda ahí:
**anticipación**, **golpe**, **sostenido** o **silencio**. Pasa el
ratón por encima de una para leer la frase entera, como «Sostenido: la
banda lo toca y lo sostiene hasta la siguiente armonía». Pulsa Espacio
y escucha:

- compás 2: la banda toca F en el «y» del pulso 4 del compás 1, una
  corchea antes, y lo deja sonar sobre la barra de compás, donde está
  escrito;
- compás 3: toda la banda golpea G7 una vez, corto, con un bombo y un
  crash, y luego nada hasta el compás 4;
- compás 4: Am golpeado y sostenido todo el compás;
- compás 5: F en el estilo durante dos pulsos, luego silencio para G7;
- compás 6: C una semicorchea antes;
- compases 7 y 8: el estilo otra vez.

La lista de reproducción muestra lo mismo: las partes de la banda
tienen huecos y notas largas donde están las marcas.

![La lista de reproducción con las pistas Piano, Bass, Drums y Guitar a lo largo de ocho compases. Cada pista está ocupada en los compases 1, 2, 7 y 8. En el compás 3 cada pista tiene un golpe corto al principio y luego nada; en el compás 4 una nota larga o un golpe; el compás 5 está vacío en su segunda mitad, y las notas del compás 6 empiezan justo antes de su barra de compás](../images/step-9-stops-playlist.png)

Las marcas también se ponen sin escribir: haz clic derecho en un compás
y elige **Banda en G7** (una entrada por cada armonía del compás), y
luego una parada o una anticipación. **Según el estilo** y **En el
pulso** quitan las marcas. Como todo cambio de la hoja, la banda
vuelve a tocar, deshacer lo devuelve, y las marcas se copian con los
compases, también como texto (ver [Hojas de acordes como
texto](chord-sheet.md#hojas-de-acordes-como-texto)):

```text
| C. | ^F | G7.. Am... | ^^C |
```

## La música: romper el groove

### Golpes

Un **golpe** (en inglés *shot*, *hit*, *stab* o *kick*) es un acorde
que toda la banda toca a la vez, corto y acentuado, seguido de
silencio. Las partituras de big band están llenas de ellos: los vientos
golpean un acorde, y el baterista lo toca con la banda en el platillo
crash y el bombo, a menudo con un fill corto antes para «prepararlo».
Una tanda de golpes, uno al principio de cada compás o de cada dos, se
llama **stop-time**: la banda toca solo los golpes y deja el resto del
compás a quien canta, al solista o a un bailarín de claqué. En las
estrofas de «Hoochie Coochie Man» de Muddy Waters la banda toca solo
un riff corto al principio de cada compás y la voz llena el silencio;
es uno de los grooves de stop-time más conocidos del blues.

Aquí un golpe es una corchea. Cada músico golpea el acorde a su manera
(el piano su disposición, el bajo la nota grave, la guitarra un
rasgueo), un poco más fuerte que el estilo, y la batería toca un bombo
y un crash. Luego la banda calla hasta la siguiente armonía. Así
funciona el compás 3 de arriba: el silencio es parte de la música, y
la armonía sigue siendo G7 para quien toque encima.

### Sostenidos

Un **sostenido** es un acorde golpeado y sostenido. Las bandas lo usan
al final de una frase, para un acorde grande bajo la última palabra de
un verso, o para dejar que un solista toque libre sobre una armonía.
No es un calderón: un calderón detiene el pulso, y la banda espera al
director, mientras que un sostenido dura exactamente hasta la
siguiente armonía, y el pulso sigue por debajo. Cada músico golpea el
acorde con un bombo y un crash, como en un golpe, y lo deja sonar. Una
forma de serie, que la banda toca como tricordios uno tras otro, se
sostiene de a un tricordio: cada uno se golpea al llegar y se sostiene
hasta el siguiente.

### Silencios

Un **silencio** es la banda dejando de tocar. Los arreglistas lo llaman
un **corte** (*break*): en el jazz, la banda se detiene un compás o dos
y el solista toca solo (el famoso «break» al final de la primera
vuelta de un tema), y en el funk y el hip hop los compases donde solo
toca el baterista le dieron nombre al «breakbeat». Aquí descansa toda
la banda, también la batería; escribe tú el corte de la batería en el
piano roll, o tócalo con **Tocas tú** (ver [Herramientas de
práctica](practice.md)).

### Anticipaciones

Una **anticipación** (en inglés *push*) es un acorde tocado antes del
pulso al que pertenece y sostenido sobre él. En C | ^F, el acorde F
está escrito en el pulso 1 del compás 2 pero se toca en el «y» del
pulso 4 del compás 1, y la banda no vuelve a golpear en el pulso 1. Una
partitura lo escribe como una corchea ligada sobre la barra de compás:

```text
 compás 1            compás 2
 1   2   3   4 y   | 1   2   3   4
 C   C   C   C F---|-F   F   F   F
               ^ aquí se golpea el F, y se sostiene sobre el pulso 1
```

Sin la anticipación, el primer tiempo es el momento más fuerte del
compás, y todo el mundo lo espera. Con ella, el acorde llega antes y
el pulso 1 queda en silencio: la música se inclina hacia delante, como
si no pudiera esperar. Eso es la **síncopa**, un acento fuera del
pulso, y las anticipaciones están por todas partes en el pop, el rock,
el funk y la música latina. Los bajistas de salsa anticipan casi todos
los acordes: en el **tumbao**, el bajo toca la fundamental del acorde
siguiente en el pulso 4, sostenida sobre la barra de compás. Una
anticipación de semicorchea (`^^C`) es más sutil, el tirón del funk y
de la música latina rápida.

Una anticipación se puede combinar con una parada: `^G7.` es un golpe
una corchea antes.

### Por qué puntos y circunflejos

Los puntos después de una armonía y los circunflejos antes son una
manera corta de escribir las marcas sin soltar el teclado: un punto
para la más corta (un golpe), tres para el silencio más largo (un
silencio). El circunflejo `^` se parece al **marcato**, el signo de
acento que los músicos escriben sobre una nota para tocarla con
fuerza, y una anticipación se toca con acento. `N.C.` (sin acorde)
conserva sus propios puntos: son parte de su nombre.

## Cómo funciona

### Dos marcas opcionales en un marcador

Una armonía de la hoja es un `ChordMarker`, con dos campos opcionales,
`push` y `stop`, `None` cuando no está marcada (ver [Cómo se guarda la
hoja](chord-sheet.md#cómo-se-guarda-la-hoja)). Sus tipos son
dos enums pequeños:

```rust
pub enum Stop {
    /// "C.": the whole band strikes the chord once, short, with a kick
    /// and a crash cymbal, and then is silent.
    Shot,
    /// "C..": the band strikes the chord and holds it.
    Hold,
    /// "C...": the band is silent. The harmony goes on, for whoever
    /// plays over it.
    Rest,
}
```

```rust
pub enum Push {
    /// "^C": an eighth note early.
    Eighth,
    /// "^^C": a sixteenth note early.
    Sixteenth,
}
```

Cada uno sabe escribirse (`marks`: `"."`, `".."`, `"..."`, `"^"`,
`"^^"`), cuántos pasos antes llega una anticipación (`steps`: 2 o 1), y
una frase para las pistas (`describe`). Los dos campos quedan fuera
del archivo cuando son `None`, así que una canción sin marcas se
escribe como se escribiría sin ellas. El formato del archivo solo pasa
a ser 18 cuando algún marcador tiene una marca, cosa que se comprueba
con `any`:

```rust
let has_breaks = playlist
    .chord_markers()
    .iter()
    .any(|marker| marker.push.is_some() || marker.stop.is_some());
```

`ChordMarker::pushed` y `ChordMarker::stopped` toman un marcador por
valor y lo devuelven cambiado, así que un marcador marcado se lee como
una frase: `ChordMarker::new(16, f).pushed(Some(Push::Eighth))`.

### Leer las marcas

Escribir `^^Am7/G..` tiene que dar una anticipación, la armonía `Am7/G`
y una parada. `read_marks` quita las marcas de los dos extremos y las
cuenta:

```rust
let name = text.trim_start_matches('^');
let push = match text.len() - name.len() {
    0 => None,
    1 => Some(Push::Eighth),
    2 => Some(Push::Sixteenth),
    _ => {
        return Err(
            tr("a push is written ^ (an eighth early) or ^^ (a sixteenth).").to_string(),
        );
    }
};
if is_no_chord(name) {
    return Ok((push, name, None));
}
let bare = name.trim_end_matches('.');
let stop = match name.len() - bare.len() {
    0 => None,
    1 => Some(Stop::Shot),
    2 => Some(Stop::Hold),
    3 => Some(Stop::Rest),
    _ => {
        return Err(tr(
            "after a harmony, . is a shot, .. a hold and ... a rest: no more dots.",
        )
        .to_string());
    }
};
Ok((push, bare, stop))
```

`trim_start_matches` devuelve un **slice** de la misma cadena sin los
circunflejos, no una copia, así que la diferencia de las dos longitudes
es cuántos quitó. Un circunflejo y un punto ocupan un byte cada uno en
UTF-8, así que contar bytes es contar marcas. Un sin acorde (`N.C.`,
`NC`) se deja entero, o su último punto se volvería un golpe. El resto,
`bare`, pasa al analizador que lee acordes, números romanos, conjuntos
y series.

Escribir un marcador de vuelta es lo mismo al revés, `format!` con las
marcas a los dos lados:

```rust
pub fn text(&self) -> String {
    let carets = self.push.map_or("", Push::marks);
    let dots = self.stop.map_or("", Stop::marks);
    format!("{carets}{}{dots}", self.harmony)
}
```

`map_or("", Push::marks)` da `""` para `None` y, si no, llama a
`Push::marks` con el valor de dentro. Un método se puede pasar por su
nombre así, como una función que recibe el valor como primer
argumento.

### Después del estilo: reescribir cada parte

Los músicos no necesitan saber nada de marcas. Cada estilo escribe su
parte como siempre, y luego un último paso, `Breaks::apply` en
`band/stops.rs`, la reescribe. Primero `Breaks::of` convierte los
marcadores en trechos de la canción:

- una **parada** dura desde su armonía (antes, si además está
  anticipada) hasta donde entra la armonía siguiente (antes, si esa
  está anticipada);
- una **anticipación** dura desde donde se toca hasta la armonía
  siguiente. Una anticipación nunca llega hasta la armonía anterior: si
  F está escrito un paso después de C, no puede entrar antes que C.

Luego, para cada músico, las paradas:

```rust
for stop in &self.stops {
    clear(&mut notes, stop.start, stop.end);
    let Some(harmony) = harmony_at(stop.written) else {
        continue;
    };
    match stop.stop {
        Stop::Shot => notes.extend(accented(strike(&Played {
            start: stop.start,
            end: (stop.start + SHOT_STEPS).min(stop.end),
            ..harmony.clone()
        }))),
        // A row form is played as trichords one after another:
        // each is struck as it comes, and held.
        Stop::Hold => {
            // ...
        }
        Stop::Rest => {}
    }
}
```

`clear` quita las notas que empiezan en el trecho, y acorta las que
empezaron antes y seguirían sonando, para que a un golpe le siga un
silencio de verdad. `accented` sube cada nota al menos a la velocidad
100, un poco más fuerte que el estilo. `SHOT_STEPS` vale 2, una
corchea. Un silencio no golpea nada: su brazo está vacío.

Una anticipación vacía el pulso en que está escrita y golpea la
armonía antes. Cuánto se sostiene depende del músico: hasta donde
sigue la parte de ese músico. En la canción de arriba, el piano del
Ritmo pop toca corcheas, así que el F anticipado se sostiene desde el
«y» del pulso 4 hasta el «y» del pulso 1, una negra; un bajo que
tocara blancas lo sostendría hasta el pulso 3.

### Cada músico golpea a su manera

¿Qué golpea el piano, y qué el bajo? `apply` no lo sabe; recibe un
closure que sí lo sabe:

```rust
pub(crate) fn apply(
    &self,
    mut notes: Vec<NoteEvent>,
    harmonies: &[Played],
    strike: impl Fn(&Played) -> Vec<NoteEvent>,
) -> Vec<NoteEvent> {
```

El piano pasa sus acordes sostenidos, el bajo una función que toca una
nota, y la guitarra y las cuerdas comprueban primero si tocan en esa
variación (unas cuerdas que solo entran en la variación b no deberían
dar de pronto un golpe en la variación a):

```rust
score.breaks.apply(notes, &score.harmonies, |played| {
    match extras.guitar_in(variation_at(played.start)) {
        Some(_) => guitar::play(
            GuitarPart::Strum(Comping::Held),
            Comping::Held,
            &score.bars,
            std::slice::from_ref(played),
        ),
        None => Vec::new(),
    }
})
```

Este closure usa `extras`, `variation_at` y `score`, variables de la
función que lo rodea. Un closure que **captura** lo que lo rodea lo
lleva consigo, así que no es un simple puntero a función como el
`fn(&Played) -> Vec<Note>` que recibe `strummed` (ver [Enums que
llevan datos, y una función como
parámetro](band.md#enums-que-llevan-datos-y-una-función-como-parámetro)),
y su tipo no tiene un nombre que puedas escribir. `impl Fn(&Played) ->
Vec<NoteEvent>` dice «cualquier tipo que se pueda llamar así», y el
compilador escribe una versión de `apply` para cada closure que se le
pasa. Una función corriente también sirve: el bajo pasa `bass_struck`
por su nombre.

`std::slice::from_ref(played)` convierte un `&Played` en un slice de
uno, `&[Played]`, sin copiarlo, porque `guitar::play` recibe un slice.

¿Por qué el bajo no usa su propio estilo sostenido, `Bass::Held`?
Porque ese vuelve a golpear en cada barra de compás, y un sostenido de
dos compases se golpearía dos veces. `bass_struck` toca una sola nota
para todo el trecho.

### La batería

La batería es distinta: sus notas vienen en pares, `(channel, note)`,
un canal por tambor. `apply_to_drums` quita el groove de cada trecho y
añade los golpes, en los canales del bombo y del crash:

```rust
notes.retain(|(_, note)| !(stop.start..stop.end).contains(&note.start));
```

`(a..b).contains(&x)` se lee mejor que `a <= x && x < b`, y para una
anticipación, `(push.start..=push.written)` con `..=` incluye el paso
en que está escrita. El bombo y el crash necesitan canales antes de
poder tocar: un estilo sin crash recibe uno, solo cuando la hoja tiene
una marca, a través de

```rust
.chain(
    (!breaks.is_empty())
        .then_some([Drum::Kick, Drum::Crash])
        .into_iter()
        .flatten(),
)
```

`then_some` convierte un `bool` en un `Option`: `Some([...])` cuando
es verdadero, `None` cuando es falso. `into_iter().flatten()` da
entonces los dos tambores o nada, así que la lista de tambores que
necesita el estilo crece solo cuando hace falta.

### La banda vuelve a tocar cuando cambia una marca

Para eso no hace falta código. La banda recuerda una huella de todo
aquello desde lo que toca, incluidos los marcadores de acorde, hecha
con su texto `Debug` (ver [Una parte que editas sigue siendo
tuya](band.md#una-parte-que-editas-sigue-siendo-tuya)). El
texto `Debug` de un marcador incluye sus marcas, así que cambiar un
golpe por un sostenido cambia la huella, y la banda vuelve a tocar. Las
partes que editaste a mano siguen congeladas hasta **Volver a
tocarla**, como siempre.

## Cómo se prueba

Los tests de `band/stops.rs` trabajan solo con `Breaks`, con unas
pocas armonías `Played` hechas a mano:
`marks_become_stretches_up_to_the_next_harmony`,
`a_push_never_reaches_back_past_the_harmony_before`,
`a_shot_strikes_once_and_the_band_stops`,
`a_hold_lasts_until_the_next_harmony_and_a_rest_is_silent`,
`a_hold_over_a_row_form_holds_each_trichord_as_it_comes`,
`a_push_comes_in_early_and_is_held_over_the_beat` y
`the_drums_stop_and_hit_with_the_band`. En `chord_sheet.rs`,
`marks_tell_the_band_where_to_stop_and_push` lee cada marca, sobre
acordes, números romanos y micromodos, `N.C.` con y sin anticipación,
y los errores (`C....`, `^^^C`, un `^` solo);
`marks_are_written_back_as_typed` las vuelve a escribir.
`the_band_stops_and_pushes_where_the_sheet_says` en `band.rs` toca una
canción con marcas y comprueba cada músico, y
`shots_holds_rests_and_pushes_need_format_eighteen` en `file.rs`
comprueba que solo las armonías marcadas llevan `push:` y `stop:` en el
archivo.

Para oírlo sin escuchar, guarda la canción de arriba y lee su [traza de
reproducción](../sound/trace.md): el F del piano empieza en el
compás 1 en el pulso 4, dos pasos dentro del pulso; en el compás 3 el
piano golpea G7 una vez, una corchea; en el compás 4 su Am dura todo el
compás; y el C anticipado empieza en el compás 5 en el pulso 4, tres
pasos dentro.
