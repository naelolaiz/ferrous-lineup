# Oír acordes: en el tiempo, dos a la vez y como conjuntos

Archivos: [`crates/lineup-core/src/harmony/hearing.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/hearing.rs),
[`crates/lineup-core/src/harmony/analysis.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/analysis.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs),
[`crates/lineup-core/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/chord_sheet.rs),
[`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-app/src/harmony_keys.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/harmony_keys.rs),
[`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs)
y [`crates/lineup-core/tests/chord_reading.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/tests/chord_reading.rs)

[La sala de conciertos](concert-hall.md) trajo La consagración de la
primavera y Petrushka a la app, pero los acordes que mostraba para
ellos estaban en su mayoría mal. Abrir **The Augurs of Spring** escribía
un chord sheet con 25 compases vacíos, y donde sí escribía un acorde
era algo como Fasus2 o Do♯dim. El acorde de los Augurios primaverales,
que las cuerdas martillean durante páginas, no tenía nombre ninguno.
De 64 compases comprobados contra las partituras, ninguno salió bien.

La causa no era un bug en un solo sitio. Era la manera de escuchar de
la app. Esta página explica cómo escucha ahora, y lo que eso enseñó
sobre la armonía y sobre cómo hacer rápido el código en Rust.

## Por qué un instante no basta

Hasta ahora, toda vista de armonía llamaba a `analyze`
([Nombres de acordes](chord-names.md)) sobre las notas **sostenidas en
un instante**. Eso es correcto para un acorde mantenido en un teclado.
La música que se mueve es más difícil de nombrar así:

- Un **arpegio** toca su acorde una nota cada vez: Do, luego Mi, luego Sol.
  En cada instante solo suena una nota, y una nota no nombra ningún
  acorde.
- Una **melodía** pasa por notas ajenas al acorde. Bajo un acorde de Do
  mayor, una melodía que toca Re hace del instante Do, Re, Mi, Sol, que
  se lee como Doadd9 aunque la armonía siga siendo un simple Do.
- Un **walking bass** camina entre los acordes: Do, Si♭, La de camino
  de Do a La7. El Si♭ bajo Do, Mi y Sol convierte el acorde en Do7 por un
  momento, aunque nadie en la banda esté tocando Do7.
- Una orquesta puede tocar **dos acordes a la vez**, uno sobre otro. Es
  un *polychord*. En los Augurios primaverales, las cuerdas graves
  tocan Fa♭ mayor (Fa♭, La♭, Do♭) mientras suena Mi♭7 (Mi♭, Sol, Si♭,
  Re♭) por encima. En Petrushka, los clarinetes tocan Do mayor sobre
  Fa♯ mayor. Ningún símbolo de acorde contiene ninguno de los dos.
- Algunas armonías **no son pilas de terceras** en absoluto. La feria
  de Petrushka se abre con las notas Re, Mi, Sol y La de la flauta, que
  se apilan en cuartas: Mi, La, Re, Sol. Llamar acorde a eso le impone
  un nombre a la fuerza.

Así que el módulo nuevo, `harmony/hearing.rs`, escucha un **tramo** (un
pulso, medio compás o un compás) y pondera cada nota según cuánto suena
en él.

## Ponderar las notas

Una nota oída en un tramo es un `HeardNote`. Sus tiempos se cuentan en
pasos desde el principio del tramo. Una nota que empezó antes tiene un
inicio por debajo de 0, y una que sigue más allá del final tiene un fin
más allá de la duración:

```rust
pub struct HeardNote {
    pub pitch: Note,
    pub start: f32,
    pub end: f32,
    /// Which part plays it, such as a channel.
    pub part: usize,
}
```

Las notas también pueden quedar por completo fuera del tramo. Esas no
pesan nada, pero dicen qué está haciendo cada parte a su alrededor,
como muestra la sección siguiente.

Sumar cuánto suena cada una de las 12 clases de alturas da un
**perfil**, doce números. Un arpegio de Do mayor a lo largo de un
compás es Do, Mi y Sol sonando cada uno un tercio de él. El perfil es
el mismo que el de un acorde sostenido, así que el arpegio se oye como
su acorde.

## Qué hace cada parte

Cuánto pesa una nota también depende de lo que está haciendo su parte.
El módulo distingue cuatro papeles, mirando también el compás anterior
y el siguiente:

| Papel | Cómo se distingue | Cuánto pesan sus notas |
|-------|-------------------|------------------------|
| Chords | tres notas distintas o más suenan juntas en ella | por completo |
| Bass line | una nota cada vez, todas por debajo de los acordes | por completo si los acordes tienen la nota; 0,15 si no (una nota de paso) |
| Tune | cualquier otra parte, cuando alguna parte toca acordes | un cuarto |
| Layer | una parte que sería una melodía, pero cuyas notas pertenecen todas a una tríada o un acorde de séptima: tres notas al menos, dos de ellas fuera de los acordes | por completo |

La fila de **Layer** está ahí por La consagración de la primavera. Su
ostinato Re♭, Si♭, Mi♭, Si♭ es una sola línea, pero deletrea Mi♭7
contra los arpegios de Do mayor y Mi menor que hay debajo. Es una
armonía propia, no una melodía.

Un caso más: una parte que toca acordes pero los mueve en paralelo con
una melodía, cada voz igual por un paso o una tercera, como en las
últimas páginas del Boléro. Esos acordes colorean la melodía. La
armonía es lo que sigue debajo de ellos, así que una parte así cuenta
como melodía. Distinguir este caso requiere los compases alrededor del
tramo: dos acordes no bastan para mostrar el movimiento paralelo, pero
una frase entera sí.

Cuando ninguna parte toca acordes, como en una orquesta donde cada
instrumento toca una línea, cada línea cuenta por completo.

## Una puntuación para cada acorde

Cada acorde, 12 fundamentales por cada cualidad que conoce la app,
recibe una puntuación de lo bien que explica el perfil:

```rust
let explained = PitchSet::from_bits(chord.pitch_set().bits() | also.bits());
let inside: f32 = explained.iter().map(|p| profile.of(p)).sum();
let outside = profile.total - inside;
// ...
let fit = (inside - OUTSIDE * outside) / profile.total - missing - SIZE * extra;
```

- `inside` es cuánto del sonido explica el acorde. `outside` es lo que
  deja fuera, que cuenta una vez y media (`OUTSIDE = 1.5`).
- `missing` es una pérdida pequeña por cada nota del acorde que apenas
  se oye. Una quinta ausente cuesta menos que una tercera ausente.
- `extra` cuenta las notas más allá de una tríada. Un acorde de trecena
  contiene casi todas las notas, así que sin este coste lo explicaría
  todo. Solo gana cuando sus notas suenan de verdad.
- Un bono de 0,06 va al acorde cuya fundamental está en el bajo.

`PitchSet` guarda un conjunto de clases de alturas como los bits de un
`u16`: el bit 0 para Do, el bit 1 para Do♯, y así. `|` sobre los bits es
la unión de dos conjuntos, así que `explained` es «las notas del acorde
y las notas de `also`» en una sola instrucción.

### Un walking bass, paso a paso

Rhythm changes empieza con un compás de Do que camina hacia La7. El piano
toca Do, Mi y Sol en los pulsos 1 y 2, y el bajo toca Do, luego Si♭.
Sobre esos dos pulsos el perfil es Do 9, Mi 5, Sol 5, y el Si♭ de paso
4 × 0,15 = 0,6.

- **Do** explica todo salvo el Si♭: (19 − 1,5 × 0,6) / 19,6 = 0,92, más
  0,06 por Do en el bajo: **0,98**.
- **Do7** lo explica todo, pero su Si♭ apenas se oye, lo que cuesta
  0,12, y su cuarta nota cuesta 0,06. Con el bono del bajo puntúa
  1 − 0,12 − 0,06 + 0,06 = **0,88**.

Gana Do, como dice el chart. Una versión anterior ponderaba las notas de
paso del bajo con 0,35, y allí ganaba Do7 por un pelo. Lo atrapó una
prueba que escribe el chord sheet de Rhythm changes.

## Dos acordes a la vez

Si ningún acorde suelto encaja bien (una puntuación por debajo de 0,7),
el módulo prueba con dos. Divide las notas en una capa inferior y una
superior, de todas las maneras que tienen sentido:

- **por registro**: todo lo que está por debajo de una altura es la
  capa inferior. Se salta una división cuando cortaría una línea, ya
  que una línea que va de La hasta Sol es un arpegio de La7, no la parte
  alta de Sol mayor;
- **por instrumento**: los instrumentos ordenados del que suena más
  grave al más agudo, y cortados entre dos cualesquiera de ellos.

Cada capa tiene que formar una tríada o un acorde de séptima propio,
con tres de sus notas oídas y su fundamental entre ellas. Después el
par tiene que pasar cuatro comprobaciones:

1. Las fundamentales difieren, y el acorde superior añade al menos dos
   notas propias. Do sobre La menor es solo Lam7.
2. Los dos acordes no forman **una sola tonalidad** entre ambos. Fa
   mayor sobre Sol mayor usa solo las notas de Do mayor: eso es Sol
   oncena, una sola armonía en Do, no dos tonalidades a la vez.
3. Ambos acordes **suenan juntos** durante al menos el 80% del tramo.
   Do mayor y luego Sol7 es una progresión, no un polychord. El oído
   retiene las notas de un acorde en staccato golpeado una y otra vez,
   así que los huecos de dos pasos o menos cuentan como que suena.
4. El par, menos un coste de 0,2 por necesitar dos acordes, puntúa
   mejor que el mejor acorde suelto.

```rust
fn in_one_key(notes: PitchSet) -> bool {
    let major = PitchSet::from_semitones(&[0, 2, 4, 5, 7, 9, 11]);
    (0..12).any(|tonic| notes.is_subset_of(major.transpose(tonic)))
}
```

`transpose` rota los bits, de modo que esto prueba las 12 escalas
mayores, y con ellas sus modos. ¿Por qué solo escalas mayores? Porque
el propio acorde de los Augurios primaverales encaja en una escala
**menor armónica**. Sus siete notas son Fa♭, Sol, La♭, Si♭, Do♭, Re♭ y
Mi♭, exactamente La♭ menor armónica. Si la comprobación contara la
menor armónica como «una tonalidad», el acorde del que trata esta
página habría sido rechazado.

El acorde de Petrushka pasa de la misma manera. Do mayor y Fa♯ mayor
están a un tritono y no comparten ninguna nota. Entre ambos contienen
Mi y Fa♯, y Sol y Fa♯, cosa que no tiene ninguna escala mayor. Las dos
tríadas salen de la escala **octatónica**, Do, Re♭, Mi♭, Mi, Fa♯, Sol,
La, Si♭, que alterna semitonos y tonos.

## Un conjunto cuando no encaja ningún acorde

Por último, las notas se puntúan como un **conjunto de clases de
alturas** ([Series dodecafónicas y conjuntos de clases de
alturas](../step-8/twelve-tone.md)). Un conjunto explica toda nota oída
lo bastante como para contar, así que siempre encaja, pero paga 0,35
por renunciar a un nombre de acorde. Una pila de cuartas paga solo 0,1,
porque desde Debussy es una armonía propia, tan clara al oído como una
tríada. Hay tres conjuntos así: 3-9 (Do, Fa, Si♭, las mismas notas que
un acorde sus), 4-23 (Do, Fa, Si♭, Mi♭) y 5-35 (Do, Fa, Si♭, Mi♭, La♭,
la escala pentatónica).

Las notas Re, Mi, Sol y La de la flauta al principio de la feria de
Petrushka son 4-23, así que se nombran como ese conjunto, en lugar de
un acorde forzado como «D6sus2».

## La mejor lectura, y las que van detrás

Un `Hearing` guarda toda lectura que puntúe a 0,12 o menos de la mejor,
tres como máximo:

```rust
pub struct Hearing {
    readings: Vec<Reading>,
    notes: PitchSet,
    profile: [f32; 12],
}
```

`best()` es la primera lectura, `alternatives()` las demás. La app las
muestra como muestra otras pistas: la más probable más brillante, las
demás más tenues.

- En las **harmony keys**, cada acorde de un polychord ilumina su tecla,
  donde la tecla tiene una, y un acorde como el que también podría
  oírse la música ilumina su tecla tenuemente. Junto a la pista, una
  línea lo dice con palabras: «Heard: Eb7|Fa♭, dos acordes a la vez», o
  «Heard: las notas 4-23».
- En el **piano roll**, la fila All del carril de acordes se oye ahora
  pulso a pulso con toda la banda, y la línea sobre la cuadrícula añade
  las otras lecturas entre paréntesis: «(o Do6)».

## Cortar un compás en acordes

**Write the chords the song plays** solía recorrer la canción pulso a
pulso y escribir un acorde donde cambiaba el nombre. Ahora
`write_heard_chords` toma la canción compás a compás y pregunta: ¿dónde
debería cortarse este compás? Un compás de 4/4 tiene líneas de pulso en
0, 1, 2, 3 y 4, y dos cualesquiera de ellas forman un tramo: diez
tramos, de pulsos sueltos al compás entero. La mejor manera de cortar
el compás se encuentra por **programación dinámica**:

```rust
// best[i]: the best score for the beats before line i, where its
// last stretch starts, and how that stretch is heard.
for i in 1..lines.len() {
    let mut here = (f32::NEG_INFINITY, 0, None);
    for j in 0..i {
        let hearing = heard(lines[j], lines[i]);
        let cut = match lines[j] {
            line if line == bar.start => 0.0,
            line if line == half => CUT_HALF_BAR,
            _ => CUT_BEAT,
        };
        let total = best[j].0 + worth(&hearing) * (i - j) as f32 - cut;
        if total > here.0 {
            here = (total, j, Some(hearing));
        }
    }
    best.push(here);
}
```

La mejor manera de cortar los primeros `i` pulsos es la mejor manera de
cortar los primeros `j`, más un tramo de `j` a `i`, para el mejor `j`.
Cada tramo gana su puntuación una vez por pulso que cubre, y cada corte
cuesta un poco: 0,15 a mitad del compás, 0,25 en cualquier otro pulso.
Así que un compás se parte solo donde dos acordes encajan de verdad
mejor que uno. Caminar hacia atrás desde el final por las `j` da los
cortes.

Un polychord **sigue** mientras siguen sus notas. En La habitación de
Petrushka, la mitad Fa♯ mayor del acorde descansa un compás mientras Do
mayor sigue. Oído solo, ese compás es solo Do. Pero Do mayor y Fa♯
mayor aún explican casi todo, así que el sheet mantiene «Do|Fa♯» allí en
lugar de saltar a Do y volver.

## Escribir dos acordes: Eb7|Fa♭

Un lead sheet dibuja un polychord como una fracción, el acorde superior
sobre una línea sobre el inferior. El chord sheet lo escribe en una
línea como `upper|lower`: `Eb7|Fb` o `C|F#`. Una barra ya significa una
nota de bajo, como en `G/B`, así que aquí no se podía usar.

```rust
if let Some((upper, lower)) = text.split_once('|') {
    let upper = ChordSymbol::parse(upper)?;
    let lower = ChordSymbol::parse(lower)?;
    return Ok(HarmonyName::Polychord(upper, lower));
}
```

`split_once` parte un string en el primer `|` y devuelve las dos
mitades, o `None` cuando no hay `|`. El `?` después de cada `parse`
devuelve el error enseguida si una de las mitades no es un acorde. La
variante nueva `HarmonyName::Polychord` contiene dos símbolos de
acorde. Cada uno puede tener su propio bajo, y el bajo del acorde
inferior es el bajo de toda la armonía.

Un proyecto cuyo sheet tiene un polychord necesita al menos el
**formato 25** ([el formato de archivo de
proyecto](../appendix-project-format.md)). Una versión anterior de la
app no podría leer `Eb7|Fb`, así que se le dice que el archivo es más
nuevo en lugar de perder el compás.

La banda toca los dos acordes. `polychord_voicing` coloca el acorde
inferior desde el Sol bajo el Do central hacia arriba y apila el acorde
superior justo encima, de modo que las dos capas nunca se cruzan, como
hace una orquesta para mantenerlas separadas.

## Hacerlo rápido

La primera versión que funcionaba era correcta y unas **60 veces más
lenta** que la manera antigua. Escribir el chord sheet de cada demo de
canción llevaba 3,6 segundos en lugar de 0,06, y la suite de pruebas,
que escribe chord sheets de cada demo y estilo muchísimas veces, se
arrastraba. Cuatro cambios la volvieron a bajar.

**1. Mide primero.** `valgrind --tool=callgrind` ejecuta un programa en
una CPU simulada y cuenta cada instrucción, por función y por línea.
Compila con información de depuración en modo release, y después anota:

```sh
CARGO_PROFILE_RELEASE_DEBUG=true cargo test --release -p lineup-core --no-run
valgrind --tool=callgrind ./target/release/deps/<test binary> --ignored one
callgrind_annotate --inclusive=yes callgrind.out.<pid>
```

El 80% del tiempo se iba en un closure. Para **cada nota**, volvía a
preguntar si la parte de la nota era una melodía, y esa pregunta
recorría **cada nota** del tramo. Eso es trabajo n². Preguntar una vez
por parte y buscar la respuesta para cada nota lo hizo n.

**2. Haz una vez el trabajo compartido.** Un compás se oye como diez
tramos, pero qué partes se mueven en paralelo depende solo del compás y
de los compases de alrededor. Un struct pequeño lo recuerda ahora:

```rust
pub struct Listening<'a> {
    notes: &'a [HeardNote],
    /// The parts that move their chords along with a tune.
    planing: Vec<usize>,
}

impl<'a> Listening<'a> {
    pub fn new(notes: &'a [HeardNote]) -> Self { /* works out `planing` */ }
    pub fn hear(&self, start: f32, end: f32) -> Hearing { /* one stretch */ }
}
```

El `'a` es un **lifetime**. `Listening` toma prestadas las notas en
lugar de copiarlas, y `'a` le dice al compilador que un `Listening` no
debe vivir más que el slice que toma prestado. El piano roll usa la
misma idea: reúne una vez las notas de cada compás y oye sus pulsos
desde ellas, donde antes volvía a reunir las notas en cada pulso.

**3. Guarda solo lo que se usa.** Solo los tres mejores acordes se
convierten en lecturas, así que no hace falta ordenarlos todos. Un
vector pequeño se mantiene ordenado según llegan los acordes:

```rust
let at = found.partition_point(|&(_, other): &(Chord, f32)| other >= fit);
if at < MOST_READINGS {
    found.insert(at, (chord, fit));
    found.truncate(MOST_READINGS);
}
```

`partition_point` hace una búsqueda binaria del primer lugar donde la
condición pasa a false: el primer acorde que encaja peor que el nuevo.
Un acorde que encaja igual de bien va después de los que ya están, de
modo que el resultado es el mismo que un ordenamiento estable.

**4. No oigas dos veces el mismo compás.** Las canciones se repiten. Un
compás tocado otra vez exactamente igual que antes, entre los mismos
compases, se oye de la misma manera, así que `write_heard_chords`
guarda un `HashMap` de las líneas de pulso y las notas de cada compás a
cómo se cortó. Las notas entran en la clave como números enteros,
`(start, end, pitch, part)`, ya que `f32` no puede ser una clave: no
tiene `Hash` ni `Eq`, porque `NaN` no es igual a sí mismo.

Escribir el sheet de todas las demos lleva ahora alrededor de un
segundo. La prueba más lenta que escribe chord sheets tarda lo mismo
que antes del cambio.

## Qué bien oye

El archivo de pruebas `tests/chord_reading.rs` comprueba compases cuya
armonía se conoce por las partituras: el acorde de los Augurios
primaverales en 52 compases, el acorde de Petrushka en los compases 47
a 53 de La habitación de Petrushka, y las cuartas del principio de la
feria. También tiene un informe, que compara el sheet oído desde las
notas con el chart de cada demo que tiene uno, pulso a pulso:

```sh
cargo test --release -p lineup-core --test chord_reading -- --ignored --nocapture
```

| | Antes | Ahora (26 de octubre de 2026: 330 demos con chart, 60,322 pulsos) |
|---|---|---|
| Exactamente el acorde del chart | 74.9% | 84.0% |
| Un acorde compatible (las mismas notas, o la misma fundamental con notas añadidas o quitadas) | 77.7% | 91.1% |
| La fundamental correcta | 77.2% | 92.1% |
| Compases comprobados contra las partituras de la Consagración y Petrushka | 0 de 64 | 64 de 64 |

El informe también lista cada demo en la que menos del 70% de los
pulsos coinciden con su chart: el lugar donde mirar a continuación.

## Pruébalo

1. Abre **File > Demos > Songs > Classical and film > The Rite of
   Spring, The Augurs of Spring**. Una canción sin chart se abre con
   los acordes que toca escritos en su sheet: abre **📝 Chords** y lee
   `Eb7|Fb` compás tras compás. Pasa el puntero por un compás para leer
   qué es: «Séptima de dominante de Mi♭ sobre Fa♭ mayor, dos acordes a
   la vez: un polychord. Superior: Mi♭ Sol Si♭ Re♭. Inferior: Fa♭ La♭ Do♭. En
   Mi♭ mixolidio: I7 sobre ♭II».
2. Pulsa Play con las harmony keys a la vista. La tecla de I se ilumina
   para Mi♭7 (Fa♭ mayor no es un acorde de Mi♭ mixolidio, así que no
   tiene tecla), y la línea junto a la pista nombra los dos acordes.
3. Abre **Petrushka, Petrushka's Room** y busca el compás 47, donde
   entran los clarinetes con el acorde de Petrushka. El sheet lo
   escribe `C|F#/A#`: Do mayor sobre Fa♯ mayor, con La♯ en el bajo.
4. Abre **Petrushka, The Shrovetide Fair** y mira los primeros
   compases: el sheet escribe el conjunto como `4-23 T2`, 4-23
   transportado dos semitonos arriba, y el puntero dice «cuartas
   apiladas».
5. En una canción nueva, escribe `C|F#` en un compás del chord sheet y
   elige un estilo de banda. La banda toca los dos acordes, uno encima
   del otro.

## Ejercicios

1. Toca Mi, Sol♯ y Si con la mano izquierda y Mi♭, Sol, Si♭ y Re♭ con
   la derecha: el acorde de los Augurios primaverales. Ahora toca la
   escala de La♭ menor armónica, La♭, Si♭, Do♭, Re♭, Mi♭, Fa♭, Sol.
   Comprueba que cada nota del acorde está en ella. ¿Por qué
   `in_one_key` tuvo que dejar fuera la menor armónica?
2. En el ejemplo del walking bass, calcula otra vez las puntuaciones de
   Do y Do7 con la nota de paso pesando 0,35 en lugar de 0,15 (el Si♭ es
   entonces 1,4 y el total 20,4). ¿Qué acorde gana?
3. Los costes de corte son 0,15 a mitad del compás y 0,25 en los demás
   pulsos. Un compás de 4/4 toca Do, luego Sol7, y cada uno de ellos, oído
   solo, puntúa 1,06 (un encaje perfecto con su fundamental en el
   bajo). Supón que el compás entero, oído como un solo tramo, puntúa
   1,0 por pulso. ¿Corta el sheet el compás cuando Sol7 entra en el pulso
   4, durante un pulso? ¿Y cuando entra en el pulso 3, durante dos?
4. ¿Por qué `HeardBars` usa en su clave las notas de los compases de
   alrededor de un compás, y no solo las notas del propio compás?
