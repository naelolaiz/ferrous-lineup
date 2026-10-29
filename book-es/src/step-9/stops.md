# Golpes, sostenidos, silencios y anticipaciones: la banda rompe el groove

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
siguiente un poco antes del pulso. Los arreglistas escriben estos
momentos en el chart, y son lo que hace que una banda suene arreglada
en lugar de en bucle. Ahora la hoja de acordes tiene marcas para ellos.

![El chord sheet con C, ^F, G7., Am.. en la primera fila y F G7..., ^^C, Dm7, G7 en la segunda. Bajo cada armonía, su numeral y su marca: «IV · push», «V7 · shot», «vi · hold», «V7 · rest». El compás 3 está con clic derecho, y el menú «Band on G7» está abierto: «As the style goes», «Shot (.)» (elegido), «Hold (..)», «Rest (...)», luego «On the beat» (elegido), «Push (^)» y «Push a sixteenth (^^)»](../images/step-9-stops-menu.png)

## Pruébalo

Pulsa **Ctrl+N** para una hoja de acordes nueva y teclea estos compases, uno
en cada compás:

```text
| C | ^F | G7. | Am.. | F G7... | ^^C | Dm7 | G7 |
```

Bajo cada armonía marcada, una palabra dice lo que la banda hace ahí:
**anticipación**, **golpe**, **sostenido** o **silencio**. Pasa el ratón por encima de
una para la frase completa, como en «Am: la banda lo golpea y lo
sostiene hasta la siguiente armonía». Pulsa Espacio y escucha:

- compás 2: la banda toca F en el «y» del pulso 4 del compás 1, una
  corchea antes, y lo deja sonar sobre la línea de compás, donde está
  escrito;
- compás 3: toda la banda golpea G7 una vez, corto, con un bombo y un
  platillo, y luego nada hasta el compás 4;
- compás 4: Am golpeado y sostenido todo el compás;
- compás 5: F en el estilo durante dos pulsos, luego silencio para G7;
- compás 6: C una semicorchea antes;
- compases 7 y 8: el estilo otra vez.

La lista de reproducción muestra lo mismo: las partes de la banda tienen huecos y
notas largas donde están las marcas.

![El playlist con las pistas Piano, Bass, Drums y Guitar a lo largo de ocho compases. Cada pista está ocupada en los compases 1, 2, 7 y 8. En el compás 3 cada pista tiene un golpe corto al principio y luego nada; en el compás 4 una nota larga o un golpe; el compás 5 está vacío en su segunda mitad, y las notas del compás 6 empiezan justo antes de su línea de compás](../images/step-9-stops-playlist.png)

Las marcas también se pueden poner sin teclear: haz clic derecho en un
compás y elige **Banda en G7** (un elemento por cada armonía del
compás), luego una parada o una anticipación. **Según el estilo** y **En el pulso** quitan las marcas. Como todo cambio de la hoja de acordes, la banda
vuelve a tocar, el undo lo deshace, y las marcas se copian con los
compases, también como texto ([Hojas de acordes como
texto](text-chord-sheets.md)):

```text
| C. | ^F | G7.. Am... | ^^C |
```

Una canción con marcas se guarda como formato 18; una canción sin
ninguna se guarda como antes, así que las versiones viejas la siguen
abriendo.

## La música: romper el groove

### Golpes

Un **golpe** (también llamado hit, stab o kick) es un acorde que toda
la banda toca a la vez, corto y acentuado, seguido de silencio. Los
charts de big band están llenos de ellos: los vientos golpean un
acorde, y el baterista lo toca con la banda en el platillo y el bombo,
a menudo con un fill corto antes para «prepararlo». Una tanda de
golpes, uno al principio de cada compás o de cada dos, se llama
**stop-time**: la banda toca solo los golpes y deja el resto del
compás a quien canta, al solista o a un bailarín de claqué. En las
estrofas de «Hoochie Coochie Man» de Muddy Waters la banda toca solo
un riff corto al principio de cada compás y la voz llena el silencio;
es uno de los grooves de stop-time más conocidos del blues.

Aquí un golpe es una corchea. Cada instrumentista golpea el acorde a su
manera (el piano su voicing, el bajo la nota grave, la guitarra un
rasgueo), un poco más fuerte que el estilo, y la batería toca un bombo
y un platillo. Luego la banda queda en silencio hasta la siguiente
armonía. Así funciona el final del compás 3 de arriba: el silencio es
parte de la música, y la armonía sigue siendo G7 para quien toque
sobre ella.

### Sostenidos

Un **sostenido** es un acorde golpeado y sostenido. Las bandas lo usan al
final de una frase, para un acorde grande bajo la última palabra de
una línea, o para dejar que un solista toque libre sobre una armonía.
No es un calderón: un calderón detiene el pulso, y la banda espera al
director, mientras que un sostenido dura exactamente hasta la siguiente
armonía, y el pulso sigue por debajo. Cada instrumentista golpea el
acorde con un bombo y un platillo, como en un golpe, y lo deja sonar.
Una forma de serie, que la banda toca como tricordos uno tras otro, se
sostiene de a un tricordo: cada uno se golpea al llegar y se sostiene
hasta el siguiente.

### Silencios

Un **silencio** es la banda dejando de tocar. Los arreglistas lo llaman un
**corte**: en el jazz, la banda se detiene un compás o dos y el
solista toca solo (el famoso «break» al final del primer chorus de un
tema), y en el funk y el hip hop los compases donde solo toca el
baterista dieron nombre al «breakbeat». Aquí descansa toda la banda,
también la batería; teclea tú el break de la batería en el piano roll,
o tócalo con **Tocas tú**.

### Anticipaciones

Una **anticipación** (en inglés, *push*) es un acorde tocado antes del pulso
al que pertenece y sostenido sobre él. En Do | ^Fa, Fa está escrito en el
pulso 1 del compás 2 pero se toca en el «y» del pulso 4 del compás 1,
y la banda no vuelve a golpear en el pulso 1. Un chart lo escribe como
una corchea ligada sobre la línea de compás:

```text
 compás 1            compás 2
 1   2   3   4 &   | 1   2   3   4
 C   C   C   C F---|-F   F   F   F
               ^ aquí se golpea la F, y se sostiene sobre el pulso 1
```

Sin la anticipación, el tiempo fuerte es el momento más fuerte del compás, y
todo el mundo lo espera. Con él, el acorde llega antes y el pulso 1
queda en silencio: la música se inclina hacia delante, como si no
pudiera esperar. Eso es **síncopa**, un acento fuera del pulso, y los
anticipaciones están por todas partes en el pop, el rock, el funk y la música
latina. Los bajistas de salsa anticipan casi cada acorde: en el
**tumbao**, el bajo toca la fundamental del acorde siguiente en el
pulso 4, sostenida sobre la línea de compás. Una anticipación de semicorchea
(`^^C`) es más sutil, el tirón del funk y de la música latina rápida.

Una anticipación puede combinarse con una parada: `^G7.` es un golpe una corchea
antes.

### Por qué puntos y signos de intercalación

Los puntos después de una armonía y los signos de intercalación antes
son una manera corta de escribir las marcas sin soltar el teclado: un
punto para el más corto (un golpe), tres para el silencio más largo (un
silencio). El signo de intercalación `^` se parece al **marcato**, el
signo de acento que los músicos escriben sobre una nota para tocarla
con fuerza, y una anticipación se toca con acento. `N.C.` (sin acorde) conserva
sus propios puntos: son parte de su nombre.

## Cómo funciona

### Dos marcas opcionales en un marker

Una armonía de la hoja de acordes es un `ChordMarker`: dónde empieza y qué
es. Gana dos campos más, cada uno un `Option`, `None` cuando no está
marcada:

```rust
pub struct ChordMarker {
    pub start: usize,
    pub harmony: HarmonyName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push: Option<Push>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop: Option<Stop>,
}

pub enum Stop { Shot, Hold, Rest }
pub enum Push { Eighth, Sixteenth }
```

Los dos atributos de `serde`, que conociste en [Una banda más
grande](bigger-band.md), mantienen el archivo como estaba cuando no
hay marcas: `skip_serializing_if` deja el campo fuera, y `default` lee
un archivo sin él como `None`. El formato del archivo solo pasa a ser
18 cuando algún marker tiene una marca, comprobado con `any`:

```rust
let has_breaks = project.playlist.chord_markers
    .iter()
    .any(|marker| marker.push.is_some() || marker.stop.is_some());
```

Hacer un marker marcado se lee como una frase, gracias a dos métodos
pequeños que toman el marker por valor y lo devuelven cambiado:

```rust
pub fn pushed(self, push: Option<Push>) -> Self {
    ChordMarker { push, ..self }
}

ChordMarker::new(16, f).pushed(Some(Push::Eighth))
```

`..self` es de nuevo la sintaxis de actualización de struct: todo
campo no nombrado viene de `self`. Como `pushed` toma `self` y no
`&self`, agota el marker viejo y devuelve uno nuevo, así que las
llamadas pueden encadenarse: `.pushed(...).stopped(...)`.

### Leer las marcas

Teclear `^^Am7/G..` debe dar una anticipación, la armonía `Am7/G` y una parada.
`read_marks` pela las marcas de los dos extremos y las cuenta:

```rust
let name = text.trim_start_matches('^');
let push = match text.len() - name.len() {
    0 => None,
    1 => Some(Push::Eighth),
    2 => Some(Push::Sixteenth),
    _ => return Err("a push is written ^ (an eighth early) or ^^ (a sixteenth).".to_string()),
};
let bare = name.trim_end_matches('.');
let stop = match name.len() - bare.len() { /* 0 to 3 dots */ };
Ok((push, bare, stop))
```

`trim_start_matches` devuelve un **slice** de la misma string sin los
signos de intercalación, no una copia, así que la diferencia de las
dos longitudes es cuántos quitó. Un signo de intercalación y un punto
son un byte cada uno en UTF-8, así que contar bytes cuenta marcas. El
resto, `bare`, va al parser que ya leía acordes, numerales, conjuntos
y series. Antes de todo esto, se reconoce `N.C.` y se deja entero, o
su último punto se convertiría en un golpe.

Escribir un marker de vuelta es lo mismo al revés, `format!` con las
marcas en los dos lados:

```rust
pub fn text(&self) -> String {
    let carets = self.push.map_or("", Push::marks);
    let dots = self.stop.map_or("", Stop::marks);
    format!("{carets}{}{dots}", self.harmony)
}
```

`map_or("", Push::marks)` da `""` para `None` y llama a `Push::marks`
sobre el valor de dentro si no. Un método puede pasarse por su nombre
así, como una función que toma el valor como primer argumento.

### Después del estilo: reescribir cada parte

Los instrumentistas no necesitan saber nada de las marcas. Cada estilo
escribe su parte exactamente como antes, y luego un paso nuevo,
`Breaks::apply` en `band/stops.rs`, la reescribe. Primero `Breaks::of`
convierte los markers en tramos de la canción:

- una **parada** dura desde su armonía (antes si también tiene anticipación)
  hasta donde entra la armonía siguiente (antes si esa tiene anticipación);
- una **anticipación** dura desde donde se toca hasta la armonía siguiente. Una
  anticipación nunca se remonta a la armonía anterior: si Fa está escrito un
  paso después de Do, no puede entrar antes que Do.

Luego, para cada instrumentista:

```rust
for stop in &self.stops {
    clear(&mut notes, stop.start, stop.end);
    let Some(harmony) = harmony_at(stop.written) else { continue };
    let end = match stop.stop {
        Stop::Shot => (stop.start + SHOT_STEPS).min(stop.end),
        Stop::Hold => stop.end,
        Stop::Rest => continue,
    };
    notes.extend(accented(strike(&Played { start: stop.start, end, ..harmony.clone() })));
}
```

`clear` quita las notas que empiezan en el tramo, y acorta las que
empezaron antes y seguirían sonando, para que a un golpe le siga
silencio de verdad. Un `continue` dentro de un brazo de un `match`
salta a la siguiente parada: un silencio no golpea nada.

Una anticipación limpia el pulso en el que está escrito y golpea la armonía
antes. Cuánto se sostiene depende del estilo: hasta donde continúe la
parte de ese instrumentista. Un piano que toca en cada pulso sostiene
el acorde con anticipación hasta el pulso 2; un bajo que toca blancas lo
sostiene hasta el pulso 3.

### Cada instrumentista golpea a su manera

¿Qué golpea el piano, y qué el bajo? `apply` no lo sabe; toma una
closure que sí:

```rust
pub(super) fn apply(
    &self,
    notes: Vec<NoteEvent>,
    harmonies: &[Played],
    strike: impl Fn(&Played) -> Vec<NoteEvent>,
) -> Vec<NoteEvent>
```

El piano pasa sus acordes sostenidos, el bajo una función que toca una
nota, y la guitarra y las cuerdas comprueban primero si están tocando
en esta variación (las cuerdas que solo entran en la variación b no
deberían golpear de pronto un golpe en la variación a):

```rust
breaks.apply(notes, &harmonies, |played| {
    match extras.guitar_in(variation_at(played.start)) {
        Some(_) => guitar::play(GuitarPart::Strum(Comping::Held), Comping::Held,
                                &bars, std::slice::from_ref(played)),
        None => Vec::new(),
    }
})
```

En [Una banda más grande](bigger-band.md), `strummed` tomaba un
**function pointer**, `fn(&Played) -> Vec<Note>`. Aquí eso no
compilaría: esta closure usa `extras`, `variation_at` y `bars`,
variables de `write_band` que la rodean. Una closure que **captura**
su entorno lo lleva consigo, así que no es una función simple, y su
tipo no tiene un nombre que puedas escribir.
`impl Fn(&Played) -> Vec<NoteEvent>` dice «cualquier tipo que se pueda
llamar así», y el compilador escribe una versión de `apply` para cada
closure que se le pase.

`std::slice::from_ref(played)` convierte un `&Played` en un slice de
uno, `&[Played]`, sin copiarlo, porque `guitar::play` toma un slice.

¿Por qué el bajo no usa su propio estilo sostenido, `Bass::Held`?
Porque ese golpea otra vez en cada línea de compás, y un sostenido de dos
compases se golpearía dos veces. El bajo recibe una función propia,
`bass_struck`, que toca una nota durante todo el tramo.

### La batería

La batería es distinta: sus notas vienen en pares, `(channel, note)`,
un canal por tambor. `apply_to_drums` saca el groove de cada tramo y
añade los golpes, en los canales del bombo y del platillo:

```rust
notes.retain(|(_, note)| !(stop.start..stop.end).contains(&note.start));
```

`(a..b).contains(&x)` se lee mejor que `a <= x && x < b`, y para una
anticipación, `(start..=written)` con `..=` incluye el paso en el que está
escrito. El bombo y el platillo deben tener canales antes de poder
tocar: un estilo sin platillo recibe uno, solo cuando la hoja de acordes
tiene una marca, mediante

```rust
.chain((!breaks.is_empty()).then_some([Drum::Kick, Drum::Crash]).into_iter().flatten())
```

`then_some` convierte un `bool` en un `Option`: `Some([...])` cuando es
true, `None` cuando es false. `into_iter().flatten()` da entonces los
dos tambores o nada, así que la lista de tambores que necesita el
estilo crece solo cuando hace falta.

### La banda vuelve a tocar cuando cambia una marca

Para eso no hubo que escribir nada. La banda recuerda una huella de
todo desde lo que tocó, incluidos los chord markers, hecha con su
texto `Debug`. El texto `Debug` de un marker ahora incluye sus marcas,
así que cambiar un golpe por un sostenido cambia la huella, y la banda
vuelve a tocar. Las partes que editaste a mano siguen congeladas hasta
**Volver a tocarla**, como siempre.

## Ejercicios

1. **Stop-time.** Teclea un blues de doce compases en Fa con golpes en
   los cuatro primeros compases:
   `| F7. | F7. | F7. | F7. | Bb7 | Bb7 | F7 | F7 | C7 | Bb7 | F7 | C7 |`
   y elige el estilo **Shuffle**. Toca en los huecos con la escala de
   blues de Fa (Fa La♭ Si♭ Si Do Mi♭). ¿Qué cambia en el aire cuando la banda
   vuelve en el compás 5?
2. **Anticipaciones.** Teclea `| C | Am | F | G |` en **Ritmo pop** y escucha.
   Luego ponle anticipación a cada acorde después del primero:
   `| C | ^Am | ^F | ^G |`. Marca el ritmo del piano con las palmas de
   las dos maneras. ¿Qué pulso desaparece?
3. En el piano roll, compara el piano y el bajo del ejercicio 2.
   ¿Cuánto dura cada acorde con anticipación en el piano, y cada nota con anticipación
   en el bajo? ¿Por qué pueden diferir?
4. Teclea `C..` en el compás 1, nada en el compás 2 y `G7` en el
   compás 3, para que el sostenido dure dos compases. Mira el bajo en el
   piano roll: una nota. Ahora reemplaza `bass_struck` en `write_band`
   por una closure que llame a `Bass::Held.play_in_bars`. ¿Qué toca
   ahora el bajo, y qué prueba lo atrapa?
5. (Rust) Haz que un golpe dure una negra. ¿Qué constante cambias, y
   qué pruebas fallan? Arréglalas.
6. (Rust) Añade `^^^C`, una anticipación de una negra. Añade una variante a
   `Push` y deja que el compilador te muestre cada `match` que debe
   aprenderla. ¿Qué otros lugares la necesitan (pista: `read_marks` y
   el menú)?
7. (Rust, más difícil) En la variación b la batería toca un fill en el
   compás anterior al cambio. ¿Deberían un golpe o un silencio de ese
   compás conservar el fill? Encuentra dónde añade `drum_part` el fill
   y dónde quita `apply_to_drums` el groove, decide, y escribe una
   prueba para lo que elijas.
