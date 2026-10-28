# Nombres de acordes: leer un lead sheet

Archivos: [`crates/lineup-core/src/harmony/symbol.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/symbol.rs),
[`crates/lineup-core/src/pitch.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/pitch.rs),
[`crates/lineup-core/src/library/numeral.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/numeral.rs),
[`crates/lineup-core/src/library/chart.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/chart.rs),
[`crates/lineup-core/src/library/voicing.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/voicing.rs)
y [`crates/lineup-core/src/library/comping.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/comping.rs)

Hasta ahora, todo chart de Ferrous Lineup estaba escrito en números
romanos: «I | V | vi | IV». Así guarda las progresiones la biblioteca,
porque un chart sirve entonces en toda tonalidad. Pero una canción que
quieras que toque la banda viene en un **lead sheet**, y un lead sheet
nombra sus acordes: «C | G/B | Am | Am/G». Antes de que pueda haber un
chord sheet donde escribir (el próximo cambio), Ferrous Lineup tiene que
leer los nombres de acordes como los escriben los músicos. Eso es todo
lo que hace este cambio; en pantalla no hay nada nuevo todavía.

## Qué dice un símbolo de acorde

Un símbolo de acorde tiene hasta tres partes:

| Parte | Ejemplo | Qué dice |
|------|---------|--------------|
| Fundamental | el **Fa♯** de Fa♯m7b5 | La nota sobre la que está construido el acorde: una letra mayúscula, quizá un sostenido o un bemol. |
| Cualidad | el **m7b5** de Fa♯m7b5 | Qué clase de acorde: menor, séptima de dominante, semidisminuido... |
| Bajo | el **/Si** de Sol/Si | La nota más grave, cuando no es la fundamental. |

Los músicos escriben la misma cualidad de varias maneras, y todas estas
se leen:

| Acorde | Escrito como |
|-------|-----------|
| mayor | Do, Domaj, CM |
| menor | Dom, Cmin, Cmi, Do- |
| séptima menor | Cm7, Cmin7, C-7 |
| séptima mayor | Cmaj7, CM7, CΔ, CΔ7 |
| semidisminuido | Cm7b5, C-7b5, Cø, Cø7 |
| séptima disminuida | Cdim7, C°7, Co7 |
| aumentado | Caug, C+ |
| séptima de dominante con quinta aumentada | C7#5, C+7, Caug7 |
| menor con séptima mayor | Cm(maj7), CmM7, C-Δ7 |
| cuarta suspendida | Csus4, Csus |
| seis-nueve | C6/9, C69 |
| novenas alteradas | C7b9, Do7(b9), Do7#9 |

Los corchetes y los espacios se ignoran, y los signos de bemol y de
sostenido ♭ y ♯ funcionan igual que `b` y `#`. Las mayúsculas y las
minúsculas importan en un solo lugar, pero es uno importante: **CM7** es
una séptima mayor (Do Mi Sol Si) y **Dom7** una séptima menor (Do Mi♭ Sol Si♭).
Cuando Ferrous Lineup escribe un acorde de vuelta, usa una manera para
cada uno, la que ya usan el piano roll y las harmony keys: Domaj7, Dom7,
Cm7b5, Cdim7.

La fundamental es siempre una letra mayúscula, porque una «b» minúscula
justo después es un bemol: «Bb7» es Si bemol siete. La «H», que los
libros alemanes usan para Si, no se lee.

## Acordes con barra

Una barra pone otra nota en el bajo. Hay dos clases:

- **Inversiones**: la nota del bajo está en el acorde. Do/Mi es Do mayor con
  su **tercera** en el bajo (la primera inversión), Do/Sol con su **quinta**
  (la segunda), y Sol7/Fa tiene la **séptima** de Sol7 en el bajo (la tercera
  inversión). `ChordSymbol::bass_in_chord` dice cuál: 3, 5 o 7.
- **Un acorde sobre una nota de bajo**: la nota del bajo no está en el
  acorde. Fa/Sol es Fa mayor (Fa La Do) sobre un Sol, un sonido suave y abierto
  que los intérpretes de pop y góspel usan en lugar de Sol11.
  `bass_in_chord` dice `None`.

La mayoría de los acordes con barra se escriben por la línea de bajo que
forman. En «C | G/B | Am | Am/G | F | C/E | Dm | G» cada acorde por
separado es normal, pero el bajo baja por una escala: Do, Si, La, Sol, Fa, Mi,
Re. Esa línea es la razón de las barras, y es lo que toca ahora la banda
(mira [más abajo](#la-banda-toca-la-barra)).

Una barra no es una nota de bajo: la de **Do6/9**, el acorde seis-nueve.
Ferrous Lineup solo toma lo que sigue a la última barra como bajo si es
un nombre de nota, así que «C6/9» es un acorde seis-nueve y «C6/9/E» es
un acorde seis-nueve sobre Mi.

## Deletreo

Db7 y Do♯7 se tocan en las mismas teclas del piano, pero no se escriben
igual. Un acorde se deletrea hacia arriba desde su fundamental por
letras: la tercera está dos letras más arriba, la quinta cuatro, la
séptima seis.

| Acorde | Fundamental | Tercera | Quinta | Séptima |
|-------|------|-------|-------|---------|
| Do♯7 | Do♯ | Mi♯ | Sol♯ | Si |
| Db7 | Re♭ | Fa | La♭ | Do♭ |

La séptima de Db7 es Do♭, no B: seis letras por encima de Re es C, y la
nota a una séptima menor por encima de Re♭, escrita con una Do, es Do
bemol. `ChordSymbol` conserva la fundamental exactamente como se
escribió, así que `notes()` deletrea el acorde como lo entiende el lead
sheet. Para tocar, los dos se vuelven el mismo `Chord`, una clase de
alturas y una cualidad.

## Transponer un chart

Para mover una canción a otra tonalidad, un músico mueve cada acorde la
misma distancia, contada en **letras** además de en semitonos. De E
menor a Re menor, cada letra baja una:

| Mi menor | Fa♯m7b5 | Si7 | Mim |
|---------|--------|----|----|
| Re menor | Em7b5 | La7 | Rem |
| Do menor | Dm7b5 | Sol7 | Dom |
| Si♭ menor | Cm7b5 | F7 | Bbm |
| Sol♯ menor | La♯m7b5 | Re♯7 | Sol♯m |

Es un **ii–V–i**, la manera más común de llegar a una tonalidad menor:
en toda tonalidad, el ii está una letra por encima de la tónica y el V
cuatro letras por encima. Contar por letras lo mantiene así: en Sol♯ menor
el V es Re♯7, nunca Eb7. Una prueba comprueba las doce tonalidades,
escritas con sostenidos y con bemoles.

`transposed(from, to)` lo hace: el intervalo de la tónica antigua a la
nueva, como semitonos y letras, se suma a la fundamental y al bajo con
`NoteName::above`, la misma función que deletrea acordes.

Una cosa se hace distinta de una partitura. Un símbolo de acorde nunca
lleva un doble sostenido ni un doble bemol en su fundamental: Re♯°7 en Mi
menor movido a Sol♯ menor sería en rigor Fa♯#°7, y Ferrous Lineup lo
escribe Sol°7, las mismas teclas, como hacen los lead sheets. En una
partitura, donde cada nota del acorde está escrita, Fa♯#°7 sería lo
correcto.

## De nombres a números romanos, y de vuelta

La biblioteca sigue pensando en números romanos, así que un acorde por
nombre se vuelve un `RomanChord` una vez conocida la tonalidad: en La
menor, Rem7 está cinco semitonos por encima de la tónica (iv7), y Mi7/Sol♯
siete, con su bajo en once. `RomanChord` ganó un campo `bass` para eso.
El camino de vuelta, `RomanChord::symbol_in(key)`, deletrea el acorde en
una tonalidad: los mismos números romanos son La7/Do♯ en Re menor, porque
la tercera de La7 es Do♯, no Re♭.

Los charts pueden ahora nombrar sus acordes. `parse_chart_in` lee un
chart como `parse_chart`, pero un acorde que empieza con una letra
mayúscula de A a G se lee como nombre de acorde, contado desde la tónica
que se le da. Las dos cosas se pueden mezclar, ya que un número romano
nunca empieza con una mayúscula de A a G: «I | G7/B | vi».

## La banda toca la barra

`accompany`, que convierte un chart en acordes y una línea de bajo, toca
ahora la nota de bajo de un acorde con barra dondequiera que el bajo
tocaría la fundamental. Los acordes de arriba siguen siendo los del
acorde: Sol/Si es todavía Sol, Si y Re en el piano. Las demás notas de un
patrón de bajo (la tercera, la quinta, la séptima) se buscan justo por
encima de la nota de bajo, y «la quinta por debajo» justo por debajo de
ella.

Luego está la cuestión de las octavas. Las fundamentales se han tocado
siempre entre Do2 y Si2, lo que está bien para las fundamentales pero
rompe la línea de arriba: Do2 y luego Si2 es un salto de una séptima hacia
arriba, no un paso hacia abajo. Así que alrededor de los acordes con
barra el bajo va por el camino corto: el bajo de un acorde con barra, y
el acorde justo después de uno, se tocan a un paso (uno o dos semitonos)
de la nota anterior cuando la hay, hasta bajar a Mi1, la cuerda más grave
de un bajo eléctrico.

| Acorde | Do | Sol/Si | Lam | Lam/Sol | Fa | Do/Mi | Rem | Sol |
|-------|---|-----|----|------|---|-----|----|---|
| Nota de bajo | Do2 | Si1 | La1 | Sol1 | F1 | Mi1 | Re2 | Sol2 |

Re1 quedaría por debajo de un bajo eléctrico, así que la línea salta
hasta Re2 ahí. Un chart sin acordes con barra suena exactamente como
antes: toda fundamental entre Do2 y Si2. En este paso, una prueba lo
comprueba, y cada uno de los 145 estilos se oye todavía como su propio
chart.

En la app no se puede oír todavía; el chord sheet viene a continuación.
Hasta entonces, las pruebas lo muestran funcionando:

```sh
cargo test -p lineup-core symbol
cargo test -p lineup-core slash
```

## El código

### Leer una nota y conservar el resto

`NoteName::read` lee la nota al principio de un símbolo de acorde y
devuelve lo que queda, para que el paso siguiente pueda seguir desde
ahí:

```rust
pub fn read(text: &str) -> Option<(NoteName, &str)> {
    let mut chars = text.chars();
    let letter = Letter::from_char(chars.next()?)?;
    let rest = chars.as_str();
    let mut chars = rest.chars();
    let accidental = match chars.next() {
        Some('#' | '♯') => 1,
        Some('b' | '♭') => -1,
        _ => return Some((NoteName::new(letter, 0), rest)),
    };
    Some((NoteName::new(letter, accidental), chars.as_str()))
}
```

`text.chars()` recorre el texto de a un carácter, y `chars.as_str()` es
la parte que aún no se ha recorrido. Ese `&str` no es una copia: apunta
al mismo texto, un poco más adelante. El tipo de retorno lo dice: el
`&str` de `Option<(NoteName, &str)>` toma el préstamo de `text`, y Rust
deduce por sí mismo que no puede sobrevivirlo.

Recorrer por `char` importa aquí. «♭» ocupa tres bytes en UTF-8, así que
cortar el texto en «un byte después de la letra» lo cortaría a mitad de
un carácter, y Rust haría panic. `chars()` se mueve siempre por
caracteres enteros.

### `rsplit_once` y el acorde seis-nueve

```rust
let (quality_text, bass) = match rest.rsplit_once('/') {
    Some((before, after)) => match NoteName::parse(after.trim()) {
        Some(bass) => (before, Some(bass)),
        None => (rest, None),
    },
    None => (rest, None),
};
```

`rsplit_once('/')` parte el texto por su **última** barra en la parte de
antes y la de después, o da `None` cuando no hay barra. Solo cuando la
parte de después es un nombre de nota es un bajo; si no, la barra
pertenece a la cualidad, como en «6/9», y todo el resto es la cualidad.

### Una tabla de deletreos

Las maneras de escribir cada cualidad son una tabla constante:

```rust
const SPELLINGS: [(ChordQuality, &[&str]); 30] = { ... };
```

Cada entrada es una tupla de una cualidad y un slice de strings,
`&[&str]`: los slices pueden tener longitudes distintas, así que una
cualidad puede tener un deletreo y otra seis, en el mismo array. Leer
una cualidad es una búsqueda:

```rust
SPELLINGS
    .iter()
    .find(|(_, spellings)| spellings.contains(&plain.as_str()))
    .map(|(quality, _)| *quality)
```

`contains` quiere una referencia a un elemento del slice, un `&&str`,
que es lo que es `&plain.as_str()`. Una prueba escribe cada cualidad
sobre cada fundamental, con y sin bajo, y la lee de vuelta, de modo que
la tabla y `ChordQuality::symbol` no pueden separarse.

### `FromStr`: `"Am7".parse()`

```rust
impl FromStr for ChordSymbol {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        ChordSymbol::parse(text)
    }
}
```

`FromStr` es el trait estándar para «se puede leer desde texto». Una vez
`ChordSymbol` lo tiene, `str::parse` funciona con él, como con los
números: `let chord: ChordSymbol = "Am7".parse()?;`. El error es un
`String` que dice qué salió mal, listo para mostrarse junto al compás
donde se escribió.

### Pasar el lector por parámetro

`parse_chart` y `parse_chart_in` comparten una función, `read_chart`,
que recibe la manera de leer un acorde como closure:

```rust
fn read_chart(
    text: &str,
    bar: usize,
    read: impl Fn(&str) -> Option<RomanChord>,
) -> Option<Vec<ChordSpan>>
```

`parse_chart` pasa `numeral::parse`, una función normal, y
`parse_chart_in` una closure que además mira la tónica que se le dio.
Una función y una closure son ambas `Fn`, así que a `read_chart` le da
igual cuál. Dentro, cada compás se lee con
`cell.split_whitespace().map(&read).collect::<Option<Vec<_>>>()`: un
acorde que no se puede leer hace `None` el compás entero, y el `?` que
viene después hace `None` el chart entero.

## Ejercicios

1. Los charts de pop escriben a veces **Cadd2**, que tiene las mismas
   notas que Doadd9 con el Re tocado grave. Añade «add2» a `SPELLINGS`.
   ¿Qué se pierde al leerlo como `Add9`?
2. **Do5** es un «power chord»: solo Do y Sol, sin tercera. No está en
   `ChordQuality`. ¿Qué más de Ferrous Lineup tendría que saber de él
   (mira `ChordQuality::family`, las harmony keys e `is_minor`)?
3. El **line cliché** mantiene un acorde mientras una sola voz baja por
   semitonos: «Am | Am/G♯ | Am/G | Am/F♯». Léelo con `parse_chart_in`,
   tócalo con `accompany` y `Bass::Held`, y comprueba las notas del
   bajo. ¿Está Sol♯ en La menor? ¿Qué acorde es en realidad, con los
   nombres de esta página?
