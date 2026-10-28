# La forma de la canción: una intro, estribillos y un final

Archivos: [`crates/lineup-core/src/band/form.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/form.rs),
[`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-core/src/arrangement.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/arrangement.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs),
[`crates/lineup-app/src/chord_sheet/form.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet/form.rs),
[`crates/lineup-app/src/app/form.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/form.rs)
y [`crates/lineup-app/src/playlist/sections.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/playlist/sections.rs)

Una banda no toca una melodía una vez y se detiene en seco. Toca unos
cuantos compases para asentar la tonalidad y el groove (la **intro**),
luego la melodía, luego la melodía otra vez, quizá varias veces, y
entonces **termina**: aterriza en el acorde de tónica y lo sostiene.
Hasta ahora la banda de Ferrous Lineup tocaba los compases del sheet una
vez y se detenía en el último, en cualquier acorde que hubiera allí.
Este cambio le da la forma de una actuación real.

![El chord sheet con su fila Form: «Intro: 4 bars», Choruses 3, Ending encendido. El sheet lee Intro (Do, Lam7, Rem7, Sol7), luego «Chorus ×3» entre signos de repetición (Do, Lam, Fa, Sol7), luego Ending (Do, luego Do otra vez con «held» y la barra de compás final)](../images/step-9-song-form-sheet.png)

## Probarlo

Escribe unos cuantos compases de acordes en el sheet (**F5**), o elige
una progresión. Debajo de los botones Band y Progression hay una fila
nueva, **Form**:

- **Intro**: No intro, 2, 4 u 8 compases. La banda pone esa cantidad de
  compases delante de la canción, con acordes que llevan a su primer
  acorde. Están escritos en el sheet bajo el encabezado **Intro**, así
  que puedes leerlos, oírlos (mantén uno pulsado con el ratón) y
  cambiarlos. Elegir una longitud otra vez los escribe de nuevo; **No
  intro** los quita. El resto de la canción, melodía incluida, se mueve
  para hacer sitio.
- **Choruses**: cuántas veces toca la banda la melodía. El sheet la
  muestra una vez, bajo **Chorus ×3**, entre los signos de repetición de
  la música impresa: ‖: al principio, :‖ al final. Mientras suena, la
  fila Form muestra por qué pasada va, «▶ 2 de 3».
- **Ending**: dos compases después del último chorus. La banda va al
  acorde de tónica de la tonalidad, toca un compás de él en el estilo, y
  luego lo golpea una vez y lo sostiene durante todo el último compás,
  con un golpe del crash. Los dos compases se muestran en gris bajo
  **Ending**, el último marcado con «held», con la doble barra final
  después.

El playlist muestra la repetición también: a lo largo de la parte
inferior de la tira de secciones, los mismos signos de repetición en los
dos extremos del chorus, y «×3».

![El playlist: las pistas Piano, Bass y Drums de la banda sobre los compases 1 a 10, con signos de repetición en la tira de secciones alrededor de los compases 5 a 8 y «×3»](../images/step-9-song-form-playlist.png)

## «Chorus» significa una pasada completa

En una canción pop el estribillo es la parte que vuelve después de cada
estrofa (en «Let It Be», «When I find myself in times of trouble»
empieza una estrofa y «Let it be, let it be» es el estribillo). Los
músicos de jazz usan la palabra de otra manera: un **chorus** es una
pasada por toda la melodía, desde el principio del chart hasta su último
compás. Un solista «toma dos choruses»; la banda toca «the head» (la
melodía) durante un chorus, luego solos durante varios, y luego el head
otra vez. La fila Form usa la palabra en ese sentido: un chorus es cada
compás del sheet después de la intro. Las estrofas y los estribillos de
una canción pop están todos dentro de un chorus aquí. Si tu canción
tiene secciones propias (Verse ×2, Chorus) en el playlist, se repiten
como siempre lo hicieron, dentro de cada pasada.

## La intro: un turnaround

Un **turnaround** es el puñado de acordes al final de una melodía que la
«dan la vuelta» y la llevan de vuelta al principio. Es la intro más
natural que existe: lleva al primer acorde, así que cuando la melodía
empieza suena como una llegada, no como un comienzo salido de la nada.

El motor de un turnaround es la **séptima de dominante**. En Do mayor, Sol7
(Sol Si Re Fa) guarda dos notas que quieren moverse: Si, un semitono bajo Do,
tira hacia arriba a Do, y Fa, un semitono sobre Mi, tira hacia abajo a Mi.
Tocada antes de Do, Sol7 hace que Do suene a casa. Y el acorde una quinta
por encima de Sol, Re menor, lleva a Sol de la misma manera que Sol lleva a Do:
las fundamentales que caen por quintas (La → Re → Sol → Do) son la manera más
fuerte en que pueden moverse los acordes.

| Intro | Hacia Do mayor | Hacia La menor |
|---|---|---|
| 2 compases | Rem7, Sol7 (ii7 V7) | Bm7b5, Mi7 (iiø7 V7) |
| 4 compases | Do, Lam7, Rem7, Sol7 (I vi7 ii7 V7) | Lam, Famaj7, Bm7b5, Mi7 (i bVImaj7 iiø7 V7) |
| 8 compases | los cuatro acordes, dos compases cada uno | los mismos, dos compases cada uno |

Hacia un acorde menor el ii es **semidisminuido** (Si Re Fa La sobre Lam):
ese es el acorde que está de verdad en la segunda nota de La menor. El V
sigue siendo una séptima de dominante, Mi7, con Sol sostenido: la
**sensible**, un semitono bajo La, tomada de la escala menor armónica,
porque un Mi menor simple no tiraría hacia La ni de cerca con tanta
fuerza.

Los numerales se cuentan desde **el acorde al que lleva la intro**, como
si fuera la tónica, no desde la tonalidad de la canción. La mayoría de
las canciones empiezan en su tónica, y entonces las dos cosas son la
misma. Una canción en Do que empieza en Fa (IV) recibe Solm7, Do7 para una
intro de dos compases: el ii y el V *de Fa*, así que la intro sigue
llevando directamente al primer acorde. Un blues que empieza en Do7
recibe Do7, Lam7, Rem7, Sol7.

Un conjunto de clases de alturas, un micromodo o una forma de serie no
tienen dominante que lleve a ellos, así que la intro los toca desde el
primer compás: un **vamp**, la banda asentando el sonido antes de que
entre la melodía. Lo mismo un acorde disminuido, al que ningún
turnaround resuelve.

## El final: a casa, y sostenido

El final va a la **tónica**, el acorde hacia el que se inclina toda la
tonalidad. ¿Qué acorde de tónica? Si la melodía empieza en la tónica, el
final toca ese mismo acorde: un blues en Do que empieza en Do7 termina en
Do7, una balada que empieza en Domaj7 termina en Domaj7. Si no, toca la
tríada propia de la tonalidad, Do en Do mayor y Lam en La menor. La
tonalidad es la del último chorus en el que termina, así que una canción
que modula termina en su tonalidad nueva. Una canción en un conjunto o
una serie no tiene tónica; termina donde empezó, en su primera armonía.

El último compás se toca distinto de todos los demás: el piano golpea el
acorde una vez y lo sostiene, el bajo sostiene la fundamental, y la
batería toca un kick con el platillo crash. Ese es el «Crash ending» de
los fills de la Library, que la banda nunca había usado. El último
compás del chorus ya recibe un fill de batería (lleva a la siguiente
pasada o al final), así que el primer compás del final es groove simple,
y la canción no termina con dos fills seguidos.

El final no está escrito en el sheet: sus acordes son de la banda,
calculados cada vez desde la tonalidad. Escribe un acorde en un compás
del final y se convierte en un compás de la melodía: el chorus crece
para incluirlo, y el final se mueve después de él.

## Cómo funciona

### `Form`: tres números que viajan con la banda

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Form {
    pub intro: usize,
    pub choruses: usize,
    pub ending: bool,
}
```

`Form` vive dentro de la `Band`, porque solo una banda toca uno. Es
`Copy`: tres números simples no cuestan nada de copiar, así que las
funciones toman un `Form`, no un `&Form`. `#[serde(default)]` en el
struct significa que cualquier campo que falte en un archivo toma su
valor de `Form::default()`, y la banda deja toda la forma fuera del
archivo cuando es la simple:

```rust
#[serde(default, skip_serializing_if = "Form::is_plain")]
pub form: Form,
```

así que una canción sin intro, choruses extra ni final se escribe
exactamente como antes, como formato 13, y las versiones más viejas de
Ferrous Lineup la abren igual. Una canción que usa la forma es formato
14.

¿Dónde están la intro y el chorus? Solo se guarda la longitud de la
intro, en pasos. El chorus se calcula cuando hace falta, desde el final
de la intro hasta el final del último compás escrito (`Project::chorus`),
así que escribir un acorde después del último compás alarga el chorus
sin nada que mantener a paso. El final se calcula de la misma manera
(`Project::ending`): los dos compases después del chorus, y
`ending_harmony`.

### La intro mueve la canción

Poner compases delante de una canción significa mover todo lo demás más
tarde: clips, secciones, marcadores de tiempo, tempo, tonalidad y
acordes, la región de loop. `Project::make_room` ya hacía esto para
copiar una sección. Quitar la intro necesita lo contrario,
`Project::cut_start`, que también tiene que conservar cómo suena la
canción donde termina el corte: la canción entonces *empieza* en el
compás, el tempo, la tonalidad y la armonía que sonaban allí.

```rust
let made = self.moving_the_band(|project| {
    project.cut_start(old);
    steps == 0 || project.make_room(0, steps)
});
```

`moving_the_band` ejecuta el cambio y luego le dice a la banda que sus
partes se movieron con todo lo demás. Sin él, la banda compararía sus
clips con las huellas que guardó, los encontraría en lugares nuevos y
pensaría que los editaste: cada parte quedaría congelada.

### Tocar el chorus otra vez: el arreglo

El playlist es la canción *como se dibuja*; el arreglo ([La estructura
de la canción](../step-8/song-structure.md)) es la canción *como se
toca*, con cada repetición desplegada. Ya tocaba dos veces una sección
marcada ×2. El chorus es una capa más alrededor de eso:

```rust
let mut played: Vec<_> = once(from, start).collect();
for chorus in 1..=choruses {
    played.extend(
        pieces(project, start, end)
            .into_iter()
            .map(|piece| (piece, Some((chorus, choruses)))),
    );
}
played.extend(once(end, to));
```

Las piezas de la intro una vez, las piezas del chorus `choruses` veces,
y luego el final una vez. Cada pasada recuerda por cuál va en su campo
nuevo, `Pass::chorus`, que la fila Form y el playlist leen para «▶ 2 de
3».

Como todo lo que toca lee el arreglo (el hilo de audio, las
exportaciones WAV y MIDI, los clics del metrónomo, el tempo de cada
paso), nada de eso tuvo que cambiar para tocar tres choruses. Una región
de loop dentro del chorus no lo abarca entero, así que el chorus no se
repite allí: hacer loop en los compases 3 a 4 toca los compases 3 a 4.

### El último compás: una closure, dos maneras de tocar

El piano normalmente toca cada compás con el comping del estilo:
`style.comping.play_in_bars(&bars, &harmonies)`. Con un final, el último
compás debe tocarse con `Comping::Held` en su lugar. El bajo es la misma
historia con `Bass::Held`. En lugar de escribir la división dos veces,
la banda le entrega la manera de tocar a una función como una
**closure**:

```rust
fn played_with(
    bars: &[Bar],
    harmonies: &[Played],
    held: Option<Bar>,
    play: impl Fn(&[Bar], &[Played]) -> Vec<NoteEvent>,
) -> Vec<NoteEvent>
```

`impl Fn(&[Bar], &[Played]) -> Vec<NoteEvent>` significa «cualquier cosa
que pueda llamarse con compases y armonías y devuelva notas»: la llamada
del piano pasa `|bars, played| style.comping.play_in_bars(bars, played)`,
la del bajo pasa la de la línea de bajo. `played_with` toca cada compás
menos el sostenido; `held_bar` toca el sostenido. Ambos cortan las
armonías a su tramo con `cut`, que también impide que una armonía
cortada antes «lleve» a ninguna parte: un bajo caminante si no caminaría
hacia el siguiente acorde antes de que llegue.

La armonía del final no está en el sheet, así que la banda la añade a la
lista de marcadores desde la que toca, como si estuviera escrita allí:

```rust
if let Some(ending) = ending {
    markers.push(ChordMarker::new(ending.start, ending.harmony));
}
```

Después se le dan las voces desde el acorde anterior, como a cualquier
otro, así que el piano se mueve suavemente al último acorde.

## Ejercicios

1. Escribe | Rem7 | Sol7 | Domaj7 | Domaj7 |, un ii–V–I en Do, y dale una
   intro de 2 compases. ¿Qué acordes escribe la intro? No llevan a Do:
   ¿a qué llevan, y por qué?
2. Pon la tonalidad en La menor (la caja Key de abajo a la izquierda),
   escribe una canción que empiece en Lam, y enciende el final. ¿En qué
   acorde termina? Ahora cambia el primer acorde a Fa: ¿en qué acorde
   termina ahora, y por qué sigue estando bien?
3. Exporta como archivo WAV una canción con una intro de 4 compases,
   tres choruses y un final (File > Export WAV…). ¿Cuántos compases
   dura? Comprueba tu respuesta contra la longitud del archivo y el
   tempo.
4. Dale a una canción una intro de 8 compases, y cámbiala luego a 2
   compases. ¿Qué le pasa a un acorde que habías cambiado en la intro?
   Deshazlo.
5. Escribe un acorde en el primer compás del final. ¿Dónde está el final
   ahora? ¿Dónde está el signo de repetición final?
6. (Rust) `Form` es `Copy`; `Band` no lo es. ¿Por qué `Band` no podría
   ser `Copy`? Mira sus campos.
7. (Rust) En `played_pieces`, el chorus se repite solo `if from <= start
   && end <= to`. Escribe qué tocaría una región de loop desde la mitad
   de la intro hasta la mitad del chorus si esa prueba se dejara fuera.
