# El tema de The Big Bang Theory: un tema de TV a partir de una transcripción

Archivos: [`crates/lineup-core/src/library/song/pop.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/pop.rs)
y [`crates/lineup-core/src/library/song.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song.rs)

El menú **File > Demos** recibe el tema de apertura de la serie de
televisión *The Big Bang Theory*, escrito por Ed Robertson y tocado por
Barenaked Ladies en 2007. Es la versión de televisión, entera: doce
compases, unos veinte segundos. La canción sigue bajo copyright, y
el comentario sobre ella en el código lo dice.

## Probarlo

Abre **File > Demos > Songs > Pop > The Big Bang Theory theme** (o
escribe «bang» en el cuadro de búsqueda de arriba del menú Demos) y
pulsa Play.

- La **tempo strip** muestra dos tempos: 130 pulsos por minuto durante
  los primeros cuatro compases, luego 159 desde el compás 5. La banda
  no acelera poco a poco: salta.
- La **hoja de acordes** muestra cuatro acordes en círculo, dos pulsos
  cada uno: Cadd9, D, G, Em7.
- En el **último compás** la banda se detiene en seco en el pulso 1,
  toca Sol una vez en el pulso 2, corto, con un platillo crash, y
  vuelve a callar. La canción termina ahí: no viene después ningún
  compás del acorde de tónica mantenido.

Para leer los tempos en lugar de escucharlos, traza la demo:

```sh
ferrous-lineup --trace "Big Bang" --out bbt.tsv
```

La cabecera del archivo los lista:

```text
# tempo: 0.0000 s, step 0, bar 1: 130.00 BPM
# tempo: 7.3846 s, step 64, bar 5: 159.00 BPM
```

Cuatro compases de 4/4 son 16 pulsos, y 16 pulsos a 130 por minuto
duran 16 × 60 / 130 = 7,3846 segundos: la estrofa empieza exactamente
ahí.

## La música

### Cuatro acordes, dos pulsos cada uno

En Sol mayor los cuatro acordes son **IV, V, I y vi**: Do, Re, Sol y Mi
menor. Son los mismos cuatro acordes que el estilo «four chords» de la
Library (I V vi IV), en otro orden. Aquí el orden cuenta una pequeña
historia cada dos compases: Do y Re, los dos acordes que llevan a casa,
luego Sol, casa, luego Mi menor, el **relativo menor**, que comparte
dos notas con Sol (Sol y Si) y vuelve a Do para empezar de nuevo.

Dos de los acordes llevan una nota extra, como los toca un guitarrista:

| Acorde | Numeral | Notas | La nota extra |
|---|---|---|---|
| Cadd9 | IVadd9 | Do Mi Sol **Re** | Re, la novena sobre Do |
| Re | V | Re Fa♯ La | |
| Sol | I | Sol Si Re | |
| Em7 | vi7 | Mi Sol Si **Re** | Re, la séptima sobre Mi |

Así que Re está en los cuatro acordes: añadido a Do y a Mi menor, y
nota de su propio acorde y de Sol. En una guitarra en Sol, estas cuatro
posiciones mantienen Re y Sol en las dos cuerdas superiores, en los
mismos trastes, así que la parte alta del rasgueo apenas se mueve
mientras el bajo sube Do, Re, Sol y cae a Mi.

La transcripción también le da al acorde de Re una oncena, Sol, en la
página de TheoryTab. Los numerales de la app no tienen «add11» (solo
«sus4», que quita la tercera, otro acorde), así que la demo toca un Re
sencillo.

### La retahíla

La melodía pone una sílaba en casi cada corchea, sobre todo en Sol, La
y Si, la parte baja de la escala, como alguien que habla rápido. Dos
veces rompe las corcheas iguales con una nota corta seguida de una
larga: Sol4 como semicorchea y La4 como corchea con puntillo en el compás
5, Re5 y Si4 igual en el compás 8. Luego las últimas frases saltan a lo
más alto: Mi5 y Re5, y después Sol5 con Fa sostenido, la **sensible**, un
semitono por debajo, una y otra vez.

### Dos tempos

Una canción tiene un tempo para empezar, `tempo: 130.0`, y una parte
puede cambiarlo con marcas de tempo (mira [`TempoMark`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song.rs)).
La estrofa empieza a 159, en su primer paso:

```rust
Part::new(
    "Verse",
    "E4:2 G4 G4 A4 B4 A4 G4:1 A4:3 | B4:2 B4 A4 A4 G4 G4 G4 D4 | ...",
    "IVadd9 V | I vi7 | IVadd9 V | I vi7 | ...",
)
.with_tempo(&[tempo(0, 159)]),
```

`tempo(at, bpm)` cambia el tempo de golpe, como hace una sección nueva
de una partitura; `glide` lo cambiaría poco a poco, como hace un
accelerando.

### El silencio y el shot

La hoja del último compás es `I... I. I... I...`: cuatro acordes
repartiéndose el compás, un pulso cada uno. Tres puntos son un
**silencio**, la banda callada; un punto es un **shot**, toda la banda
tocando el acorde una vez, corto, con un kick y un crash (mira [Shots,
holds, silencios y pushes](stops.md)). O sea: silencio en el pulso 1,
Sol tocado en el pulso 2, silencio hasta el final. El primer pulso se
escribe `I...` y no `V...` porque la melodía canta Sol ahí: la armonía
ya está en casa, aunque nadie la toque.

### Terminar en el stop

La mayoría de las demos de canciones terminan con un compás que la
partitura no tiene: el acorde de tónica mantenido, con un crash, para
que una hoja que simplemente se acaba suene terminada. Los géneros que
terminan despacio (tangos, canciones folclóricas, soul, piezas
clásicas) también ensanchan su último compás, un pequeño ritardando.
Una canción cuya hoja ya termina en una parada (un shot, un hold o un
silencio) no recibe ninguno: la partitura dice cómo se detiene la
banda, y esa parada va a tempo. El último «chan-chan» de un tango es
el mismo caso que este shot.

`song_demo`, en
[`crates/lineup-core/src/library/demo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/demo.rs),
mira el último acorde de la hoja de la última parte y se salta el
compás extra cuando ese acorde tiene un stop:

```rust
let stopped = song.form.last().is_some_and(|name| {
    patterns
        .iter()
        .rev()
        .find(|piece| piece.part == *name)
        .and_then(|piece| piece.spans.last())
        .is_some_and(|span| span.stop.is_some())
});
```

`is_some_and` pregunta dos cosas a la vez: ¿hay un valor, y pasa la
prueba? `song.form.last()` es `None` para una canción sin forma, y
entonces la respuesta es simplemente `false`. La cadena de dentro lee
desde el final de la lista de patrones (`.rev()`), toma el último hecho
a partir de esa parte, su último span de acorde, y comprueba si el span
lleva un stop. Once demos terminan así; el trace de cada una muestra la
última nota donde la partitura la tiene.

## De dónde vienen las notas

No se pudo leer ninguna partitura del tema desde donde se escribió la
demo, pero sí una **transcripción**. En el sitio *TheoryTab* de
Hooktheory, la gente escribe canciones tal como las lee una clase de
teoría musical: cada nota como un **grado de la escala** en la
tonalidad (1 es la tónica, 5 la quinta...), cada acorde como un numeral
romano. Un proyecto de investigación, Sheet Sage (Chris Donahue y
otros, 2022), empaquetó unas 26.000 de estas transcripciones en un
archivo en GitHub, y el propio sitio de Hooktheory da las mismas
secciones de una en una.

El tema tiene allí dos secciones, «Intro» y «Verse». Una nota se
escribe así:

```json
{"sd": "6", "octave": 0, "beat": 1, "duration": 1}
```

Grado 6 de la escala, en la octava que empieza en la tónica, en el
pulso 1, de un pulso de duración. En Sol mayor el grado 6 es Mi, y con
la tónica como Sol4, la octava que va de Sol4 hacia arriba contiene Mi5:
una negra `E5:4`. El grado 6 en la octava -1 es Mi4, y el grado 1 en la
octava 1 es Sol5. Con Sol4 como tónica, la melodía va de Re4 a Sol5, el
registro en el que una lead sheet la escribe; un hombre la canta una
octava más grave.

| Escrito allí | En Sol | Aquí |
|---|---|---|
| sd 6, octava 0, 1 pulso | Mi, sobre la tónica | `E5:4` |
| sd 5, octava 0, ½ pulso | Re | `D5:2` |
| sd 1, octava 0, ¼ pulso | Sol, la tónica | `G4:1` |
| sd 5, octava -1, ½ pulso | Re, bajo la tónica | `D4:2` |

La estrofa también está **alineada** con la grabación: su pulso 1 está
a los 7,6 segundos de esta, y el pulso 33 a los 19,66 segundos. 32
pulsos en 12,06 segundos son 32 × 60 / 12,06 = 159,2 pulsos por
minuto. De ahí viene el 159.

La transcripción de la intro fue revisada. La versión de 2020 la tiene
a 124 pulsos por minuto, sobre IV | I | IV | I, un compás cada uno; la
página da ahora 130 y los acordes de la estrofa, IVadd9 V I vi7. La
demo toma la lectura más nueva, dos pulsos por acorde como en la
estrofa, por una razón que dan las notas: el tercer compás de la intro
son las mismas ocho notas que el primer y el tercer compás de la
estrofa (Mi Sol Sol La Si La Sol La), y allí la transcripción las tiene sobre Do
y Re.

## Ejercicios

1. Traza la demo y encuentra la primera nota del compás 5 y del compás
   6. ¿Cuánto dura un compás a 159? (4 × 60 / 159 = 1,509 segundos.)
   ¿Y a 130? (1,846 segundos.)
2. Em7 sin su séptima es Em. ¿Qué nota desaparece, y cuál de los cuatro
   acordes se quedaría entonces sin Re?
3. Nombra los mismos cuatro acordes en La mayor. (Re add9, Mi, La, Fa
   sostenido menor séptima.)
4. En el compás 10 la melodía alterna Sol5 y Fa♯5 sobre Sol, y luego sobre
   Em7. Sobre Sol, el Fa sostenido es la séptima del acorde; ¿qué es
   sobre Mi menor? (La novena.)
