# La partitura guía: de las notas a la notación

Archivos: [`crates/lineup-core/src/lead_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/lead_sheet.rs)
y los módulos de [`crates/lineup-core/src/lead_sheet/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates/lineup-core/src/lead_sheet)
(`melody.rs`, `spelling.rs`, `rhythm.rs`, `score.rs`, `layout.rs`,
`glyphs.rs`, `transpose.rs`), y en la app
[`crates/lineup-app/src/lead_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/lead_sheet.rs)
con [`fonts.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/lead_sheet/fonts.rs)
y [`pdf.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/lead_sheet/pdf.rs)

Una **partitura guía** (en inglés *lead sheet*) es lo que lee una
banda. Tiene un pentagrama con la melodía, la armonía escrita encima
como símbolos de acorde, y la tonalidad, el compás, las secciones y las
repeticiones de la canción. Deja fuera todo lo demás: la línea de bajo,
la batería y cómo dispone el piano sus acordes quedan para que los
músicos lo inventen, al estilo de la noche. Los músicos de jazz, pop y
folclore llevan libros enteros de ellas.

Ferrous Lineup tiene todo lo que necesita una partitura guía: la
melodía son las notas de un canal, y los acordes son los de la hoja de
acordes (ver [La hoja de acordes](chord-sheet.md)). La
ventana **Partitura guía** los escribe en páginas que puedes imprimir,
tal como suenan o para músicos de instrumentos en Si♭, Mi♭ y Fa. Solo
lee la canción: no cambia nada de ella ni del archivo.

## Probarlo

**Ver > Partitura guía** abre la ventana. La partitura se distribuye en
páginas, como se imprimirá, y el compás que suena se ilumina mientras
suena la canción. Cuando el compás que suena queda fuera de la vista, la
ventana se desplaza hasta él.

![La ventana Lead sheet con I Got Rhythm en Si bemol mayor mientras suena: la melodía en un pentagrama en clave de sol con dos bemoles, símbolos de acorde encima (Bb, Gm7, Cm7, F7), cuatro compases por línea, y el compás 2 iluminado en amarillo](../images/step-9-lead-sheet-playing.png)

Arriba:

- **Melodía** elige el canal cuyas notas son la melodía.
  **Automático** toma el canal llamado Melodía, que tienen todas las
  demostraciones de canciones, o si no el canal cuyas notas son más
  agudas en promedio, que es donde suele estar una melodía; el menú dice
  cuál, como «Automático (Melodía)». Se puede elegir cualquier canal con
  notas, el bajo para la parte de un bajista. **Ninguna: solo acordes**
  no escribe melodía: cada compás lleva una barra por pulso, bajo los
  símbolos de acorde, como lee su parte una sección rítmica.
- **Clave**: **Automático** escribe una melodía grave en clave de fa y
  las demás en clave de sol; **Sol** y **Fa** eligen una.
- **Para** escribe la partitura para músicos de instrumentos en Si♭, Mi♭
  o Fa (trompeta, saxofones, corno), en la tonalidad en que leen (ver
  [Partes para instrumentos en Si♭, Mi♭ y
  Fa](#partes-para-instrumentos-en-si-mi-y-fa) más abajo). **Altura
  real** la escribe tal como suena.
- **Papel**: A4 (el que se usa casi en todas partes) o Carta (en Estados
  Unidos y Canadá).
- **−** y **+** acercan o alejan las páginas, del 50% al 200%.
- **Exportar PDF…** guarda la partitura como archivo PDF.
- **Imprimir…** escribe el PDF en la carpeta temporal del ordenador y lo
  abre en tu visor de PDF, para imprimir desde allí. Imprimir así
  funciona igual en Linux, macOS y Windows, y el visor muestra los
  ajustes propios de la impresora.

![La misma canción con None: chords only: cuatro barras en cada compás bajo los símbolos de acorde](../images/step-9-lead-sheet-chords.png)

El papel, el zoom y los instrumentos de **Para** se recuerdan para la
próxima vez.

## Leer una partitura guía

Aquí está la primera página de La Cumparsita tal como la imprime el
PDF:

![La primera página de La Cumparsita como PDF: el título, una indicación de tempo «negra = 112», un pentagrama en clave de sol en Sol menor con dos bemoles, los símbolos de acorde D7, Gm, Cm, Em7b5, Am7b5, y los números de compás 4, 7, 11 al principio de cada línea](../images/step-9-lead-sheet-pdf.png)

De izquierda a derecha en la primera línea:

- La **clave**. La clave de sol se enrosca alrededor de la línea de Sol
  (la segunda línea desde abajo); los dos puntos de la clave de fa
  rodean la línea de Fa.
- La **armadura**: los sostenidos o bemoles de la tonalidad, para no
  tener que escribirlos en cada nota. Dos bemoles (Si♭ y Mi♭) son Si♭
  mayor o Sol menor. La Cumparsita está en Sol menor, y su Fa♯ (la
  sensible de la menor armónica) lleva un sostenido cada vez, porque la
  armadura no lo tiene.
- El **compás**, solo en la primera línea y donde cambia.
- El **tempo**: «♩ = 112», 112 negras por minuto. En 6/8 es una negra
  con puntillo, el pulso que marcas.

Encima del pentagrama están los **símbolos de acorde**, cada uno sobre
el lugar donde empieza su acorde. Un acorde se escribe una sola vez,
cuando cambia; sigue sonando hasta el siguiente. El número pequeño al
principio de cada línea es el **número de compás** de su primer compás,
para que una banda pueda decir «desde el compás 21».

Más adelante, una partitura guía puede contener:

- **marcas de ensayo**: el nombre de una sección («A», «Estrofa»,
  «Estribillo») en un recuadro, donde empieza la sección. Cada sección
  empieza una línea nueva, así que la forma de la canción se ve de un
  vistazo;
- **signos de repetición**: una barra de compás gruesa y una fina con
  dos puntos, alrededor de una sección que se toca dos veces; «×3» sobre
  el final cuando se toca tres veces o más;
- una **doble barra** (dos líneas finas) antes de una sección nueva y
  antes de un cambio de tonalidad, donde se escribe la armadura nueva,
  con becuadros para los sostenidos o bemoles que tenía la tonalidad
  vieja y la nueva no;
- la **barra final** (fina y gruesa) al final.

## De las notas a la notación

La notación no es una imagen del piano roll. El piano roll dice «esta
tecla, desde este paso, durante tantos pasos»; la notación tiene que
elegir un nombre para cada nota (¿La♯ o Si♭?), escribir cada duración
como figuras que se puedan contar, y mostrar dónde están los pulsos.
Los músicos leen con soltura porque estas elecciones siguen reglas, y
de esas reglas trata este capítulo. Cada paso es un módulo de
`lineup-core`, y ninguno sabe nada de ventanas ni de archivos PDF.

### La melodía, nota a nota (`melody.rs`)

Las notas del canal de la melodía se leen a lo largo de la lista de
reproducción, clip a clip, como suenan. Una partitura guía tiene una
nota cada vez, así que donde empiezan varias notas juntas se queda la
más aguda (la regla del «perfil»: una melodía armonizada en terceras se
lee como su voz de arriba), y una nota que todavía suena cuando empieza
la siguiente se corta ahí.

Las notas tocadas un poco antes o después (grabadas desde un teclado, o
importadas de un archivo MIDI) se llevan a la semicorchea más cercana.
Una partitura guía muestra lo que se quiere decir, no cada matiz de una
interpretación.

### Qué nombre de nota (`spelling.rs`)

La misma tecla del piano se puede escribir de dos maneras: la tecla
negra entre Sol y La es Sol♯ o La♭. Cuál es la correcta depende de lo
que *hace* la nota, y las reglas se prueban en este orden:

```rust
pub fn spell(key: Scale, pitch: Note, chord: Option<ChordSymbol>, next: Option<Note>) -> NoteName {
    let pitch_class = pitch.pitch_class();
    if key.has_letter_for(pitch_class) {
        return key.spell(pitch_class);
    }
    if let Some(symbol) = chord {
        let names = symbol.chord().spell(symbol.root);
        let bass = symbol.bass.into_iter();
        if let Some(name) = names
            .into_iter()
            .chain(bass)
            .find(|name| name.pitch_class() == pitch_class && name.accidental.abs() <= 1)
        {
            return name;
        }
    }
    if key.kind.is_tonal()
        && let Some(next) = next
    {
        let step = i32::from(next.midi()) - i32::from(pitch.midi());
        let neighbour = match step {
            1 => Some(key.spell(next.pitch_class()).letter.up(-1)),
            -1 => Some(key.spell(next.pitch_class()).letter.up(1)),
            _ => None,
        };
        if let Some(letter) = neighbour {
            let name = NoteName::with_letter(pitch_class, letter);
            if name.accidental.abs() <= 1 && !is_odd(name) {
                return name;
            }
        }
    }
    let name = key.spell(pitch_class);
    if is_odd(name) {
        NoteName::chromatic(pitch_class)
    } else {
        name
    }
}
```

1. **Una nota de la tonalidad** se escribe como la escribe la
   tonalidad: en La mayor, la tecla negra encima de Do es Do♯, nunca
   Re♭.
2. **Una nota del acorde** que suena con ella se escribe como la
   escribe el acorde. Sobre E7 en Do mayor, la tecla negra encima de Sol
   es Sol♯, la tercera mayor de Mi (Mi Sol♯ Si Re). Sobre Fm en Do mayor
   la misma tecla es La♭, la tercera menor de Fa (Fa La♭ Do). Los nombres
   siguen las terceras del acorde.
3. **Una vecina cromática**, una nota que se mueve un semitono hasta la
   siguiente, toma su nombre de hacia dónde va la línea: subiendo, es el
   nombre de abajo alterado hacia arriba (Fa♯ yendo a Sol); bajando, el
   nombre de arriba alterado hacia abajo (Si♭ yendo a La). La nota «se
   inclina» hacia donde va.
4. **Cualquier otra** recibe el nombre que la tonalidad da a las notas
   que están fuera de ella.

`is_odd` mantiene sencillas las notas de fuera: fuera de la tonalidad,
una tecla blanca del piano se escribe como tal. En una tonalidad con
muchos bemoles, un Si entre Do y Si♭ se escribe Do – Si – Si♭, no
Do – Do♭ – Si♭, que se lee peor aunque Do♭ sea la «misma» nota.

Una vez que cada nota tiene su nombre, el compás decide qué
**alteraciones** imprimir. Una alteración dura hasta la barra de
compás: en Sol menor, el primer Fa♯ de un compás lleva sostenido, el
siguiente Fa♯ de ese compás no, y un Fa después lleva becuadro. El
compás siguiente vuelve a empezar desde la armadura.

Las series y los conjuntos de clases de altura no tienen armadura, así
que una canción en una serie o en un conjunto se escribe sin ella, cada
tecla negra con su propio sostenido o bemol.

### Cuánto dura (`rhythm.rs`)

Una nota de cuatro semicorcheas es una negra, esté donde esté. Pero
quien lee cuenta pulsos, y la notación tiene que mostrarlos. Así que
cada lugar de un compás tiene una **fuerza**:

```rust
/// How strong a place in a bar is, counted in sixteenths from its start:
///
/// - 4 for the bar line;
/// - 3 for the middle of a bar of four even beats (beat 3 of 4/4, the
///   third dotted quarter of 12/8);
/// - 2 for any other beat;
/// - 1 for the halves of a beat (the "and" of 4/4), or the thirds of a
///   dotted beat (each eighth of 6/8);
/// - 0 for anything smaller.
pub fn strength(time: TimeSignature, offset: usize) -> u8 {
```

Las reglas, tal como las usan los grabadores de música (están escritas
en *Behind Bars*, de Elaine Gould, el libro de referencia del grabado
musical), dicen qué notas pueden cruzar qué lugares:

- Una nota puede sostenerse por encima de un pulso solo si empieza en
  un pulso. Una negra que empieza en el «y» del pulso 2 en 4/4 esconde
  el pulso 3, así que se escribe como dos corcheas **ligadas**: la
  segunda nota no se vuelve a tocar, la ligadura (una curva fina)
  sostiene la primera.
- Nada esconde el centro de un compás de cuatro pulsos, salvo que
  empiece en la barra de compás. Una nota del pulso 2 al final del
  compás es una negra ligada a una blanca.
- Se deja pasar un patrón porque todo lector lo conoce: la síncopa
  «♪ ♩ ♪», una negra desde la mitad de un pulso hasta la mitad del
  siguiente, dentro de una mitad del compás.
- En 6/8 y en métricas agrupadas como 2+2+3/8 las notas se sostienen
  solo de pulso a pulso, para que se vean los grupos.
- Los silencios son más estrictos todavía: un silencio cubre varios
  pulsos solo desde la barra de compás o desde el centro del compás,
  como un silencio de blanca en el pulso 1 o 3 de 4/4.

Cuando una nota no cabe en una figura, se corta primero en el lugar más
fuerte que hay dentro de ella, y los trozos se ligan. Aquí hay cuatro
compases de I Got Rhythm: las síncopas de la melodía salen como negras y
corcheas ligadas por encima del pulso 3, el centro del compás, así que
quien lee siempre ve dónde está.

![Los compases 9 a 12 de I Got Rhythm: un silencio de negra, Fa ligado por encima del centro del compás a una corchea, Sol ligado a una negra; Si bemol negra con puntillo, Do corchea ligada a una blanca](../images/step-9-lead-sheet-ties.png)

Las **barras** unen corcheas y semicorcheas pulso a pulso (por el
compás entero cuando los pulsos son corcheas, como en 3/8), así que
aquí también se ven los pulsos. Una barra se inclina con las notas, pero
nunca más de un espacio del pentagrama, y una semicorchea junto a una
corchea lleva una segunda barra corta propia, un «muñón».

### Qué contiene un compás (`score.rs`)

`score.rs` junta todo esto compás a compás. Un compás contiene sus
notas y silencios (cada uno con su figura, su nombre, su alteración y su
ligadura), los acordes que empiezan en él, y lo que se escribe en sus
bordes: una tonalidad o un compás nuevos, el comienzo de una sección,
signos de repetición. Antes de la primera nota de la melodía y después
de la última, un compás donde la banda toca un acorde lleva barras en
vez de silencios, así que una introducción o un final que la banda toca
sola se lee como «toca estos acordes».

### Dónde va todo (`layout.rs`)

Después todo se coloca en la página, en puntos (1/72 de pulgada). El
pentagrama mide 21,2 puntos de alto (7,5 mm), cuatro **espacios** de 5,3
puntos, el tamaño habitual de una partitura guía. Todas las medidas de
la distribución se cuentan en espacios.

Las notas reciben espacio según lo que duran, pero no en proporción:
una blanca no recibe el doble que una negra, o un compás de
semicorcheas quedaría aplastado junto a un compás de redondas. Los
grabadores espacian según algo parecido a la raíz cuadrada de la
duración:

```rust
/// How much room a note or rest gets for lasting `steps` sixteenths,
/// before the line is stretched: about 2.6 spaces for a sixteenth, 4.2
/// for a quarter, 7.4 for a whole note.
fn spring(steps: usize) -> f32 {
    sp(1.0 + 1.6 * (steps as f32).sqrt())
}
```

Un símbolo de acorde es más ancho que una nota, así que un cambio de
acorde rápido («Gm G7» sobre dos negras) separa sus notas hasta que los
símbolos no se tocan.

Una línea tiene hasta cuatro compases, menos si no caben, y una sección
nueva empieza una línea nueva. Cada línea completa se **estira** luego
hasta llegar exactamente al margen derecho. Cuánto estirarla se
encuentra probando: se dibuja la línea con un estiramiento de 0,25 y de
6, y luego se parte el intervalo por la mitad 24 veces, quedándose con
la mitad donde cae el ancho. Es una **búsqueda binaria**, la misma idea
que buscar una palabra en un diccionario abriéndolo por la mitad:

```rust
fn fit(&self, bars: Range<usize>, width: f32) -> f32 {
    let wide = |stretch: f32| self.draw_system(bars.clone(), stretch).width;
    let (mut low, mut high) = (0.25_f32, 6.0_f32);
    // ...
    for _ in 0..24 {
        let middle = (low + high) / 2.0;
        if wide(middle) > width {
            high = middle;
        } else {
            low = middle;
        }
    }
    low
}
```

`wide` es un **closure**, una pequeña función escrita en línea que toma
prestados `self` y `bars` de su alrededor. La línea se dibuja hasta 26
veces para colocarla una; es rápido, porque dibujar una línea solo
calcula posiciones.

La última línea de una sección o de la canción suele ser corta.
Estirada hasta el margen, sus pocas notas quedarían muy separadas, así
que se espacia como las líneas completas y se deja corta, como hacen los
grabadores. Luego las líneas van a las páginas, repartidas para que
cada página se llene de arriba abajo, salvo la última.

### Una lista de marcas

El resultado de la distribución es una lista de **marcas** por página:
valores simples que dicen qué dibujar y dónde.

```rust
pub enum Mark {
    /// A symbol of the music font with its origin (the SMuFL baseline)
    /// at `at`; `size` is the font size, the height of a staff.
    Music {
        glyph: glyphs::Glyph,
        at: Point,
        size: f32,
    },
    /// Text with the left end of its baseline at `at`.
    Text {
        text: String,
        font: TextFont,
        at: Point,
        size: f32,
    },
    /// A straight line `width` thick, with square ends.
    Line { from: Point, to: Point, width: f32 },
    // ... Beam and Tie, each with two points and a thickness
}
```

La ventana pinta estas marcas en la pantalla y `pdf.rs` escribe las
mismas marcas en el PDF, así que lo que se imprime es exactamente lo que
se vio. Es la misma separación que usa la app en todas partes:
`lineup-core` decide, la app solo dibuja. También hace que la
distribución se pueda probar sin ventana: una prueba puede distribuir
una canción y buscar una cabeza de nota por nota, un recuadro por
compás, y nada fuera de la página.

## Las fuentes

Los símbolos musicales salen de una **fuente musical**. Las fuentes que
siguen **SMuFL**, el Standard Music Font Layout, dan a cada símbolo un
punto de código fijo (la clave de sol es U+E050 en todas) y están
dibujadas para que un em, el tamaño de la fuente, sea la altura del
pentagrama. La app usa **Bravura**, la fuente con la que se diseñó
SMuFL, de Steinberg. Las palabras y los símbolos de acorde van en
**Inter**, una fuente hecha para pantallas que también se imprime
limpia. Las dos son libres, bajo la SIL Open Font License, y van dentro
del programa con `include_bytes!`, así que la partitura se ve igual en
todos los ordenadores:

```rust
const BRAVURA: &[u8] = include_bytes!("../../assets/fonts/Bravura.otf");
```

`include_bytes!` es una macro que lee el archivo al compilar el programa
y pone sus bytes dentro de él, como un `&'static [u8]`: un slice que
vive tanto como el programa.

La distribución necesita saber qué ancho tiene cada símbolo y cuánto
sube y baja. Esos números salen de los metadatos de la propia Bravura,
copiados en `glyphs.rs`, y una prueba lee el archivo de la fuente para
comprobar cada uno.

El texto se mide con la misma fuente con la que se dibuja.
`lineup-core` no tiene fuentes, así que pregunta a través de un
**trait**:

```rust
pub trait TextWidth {
    /// The width of `text` in `font` at `size` points, in points.
    fn text_width(&self, text: &str, font: TextFont, size: f32) -> f32;
}
```

La app lo implementa sumando los anchos de las letras leídos del
archivo de la fuente con el crate `ttf-parser`; las pruebas lo
implementan con una regla simple (cada letra la mitad de ancha que el
tamaño de la fuente). La distribución recibe un `&dyn TextWidth` y
funciona con cualquiera de los dos.

Los símbolos de acorde siguen la costumbre de las partituras guía: la
letra de la fundamental grande, el resto un poco más pequeño («Am7»,
«G7»), y signos ♯ y ♭ de verdad, de la fuente musical, elevados, en vez
de «#» y «b». Las fundamentales van **en letras sea cual sea el nombre
de las notas en la app**: con solfeo elegido, la hoja de acordes dice
Rem7, pero la partitura guía escribe Dm7, como lo hacen las partituras
guía impresas en todos los idiomas, para que una banda la pueda leer en
cualquier parte.

## El PDF

Una página de PDF es una lista de operaciones de dibujo, muy parecida a
las marcas: ir a un punto, trazar una línea hasta otro, rellenar una
forma. El crate [`pdf-writer`](https://crates.io/crates/pdf-writer) las
escribe en el formato del PDF; funciona en todas las plataformas y no
necesita nada del sistema.

Las letras y los símbolos no se incrustan como fuentes. Cada uno se
copia como su **contorno**, las curvas que lo dibujan, leído del archivo
de la fuente con `ttf-parser`, y se rellena. Un símbolo que se usa
muchas veces, como una cabeza de nota negra, se escribe una vez como
«form XObject» y luego se coloca dondequiera que aparezca, escalado a su
tamaño:

```rust
content
    .save_state()
    .transform([scale, 0.0, 0.0, scale, x, height - at.y])
    .x_object(Name(name.as_bytes()))
    .restore_state();
```

El `transform` es una matriz que escala la forma de las unidades de la
fuente a puntos y la mueve a su sitio. Fíjate en `height - at.y`: el PDF
cuenta la y hacia arriba desde el pie de la página, la distribución la
cuenta hacia abajo desde arriba, así que cada y se da la vuelta. El PDF
se ve igual en cualquier visor, sin instalar nada.

## Partes para instrumentos en Si♭, Mi♭ y Fa

Pon una partitura guía de una canción en Do delante de un trompetista y
de un pianista, pídeles que toquen juntos la primera nota, y no van a
coincidir. El pianista toca Do; la trompeta, leyendo el mismo Do, suena
el Si♭ un tono más abajo. La trompeta es un **instrumento
transpositor**: las notas que lee no son las que suenan. Para tocar con
la banda, el trompetista necesita una partitura escrita un tono más
arriba, en Re.

### Por qué algunos instrumentos leen en otras tonalidades

Los instrumentos de viento vienen en familias de distintos tamaños: los
saxofones del soprano al barítono, los clarinetes, los cornos. Cada
tamaño suena más grave o más agudo, pero los dedos son los mismos: en
todos los saxofones, los mismos dedos tocan la nota escrita Do. Si la
música se escribiera como suena, alguien que pasa del alto al tenor
tendría que aprender digitaciones nuevas para cada nota. Así que la
música se escribe para los dedos, y cada instrumento se nombra por la
nota que suena cuando toca un Do escrito: una trompeta **en Si♭**, un
saxofón alto **en Mi♭**, un corno **en Fa**.

La música escrita como suena está en **altura real**, o «en Do». El
piano, la guitarra, el bajo, la voz, la flauta, el violín y el trombón
la leen así.

| Instrumentos | Un Do escrito suena | La parte se escribe |
|---|---|---|
| Piano, guitarra, bajo, voz, flauta, violín, trombón | Do | como suena |
| Trompeta, corneta, fliscorno, clarinete, saxofón soprano | el Si♭ de abajo | una segunda mayor más arriba |
| Saxofón tenor | el Si♭ una octava y una segunda más abajo | una novena mayor más arriba |
| Saxofón alto | el Mi♭ de abajo | una sexta mayor más arriba |
| Saxofón barítono | el Mi♭ una octava y una sexta más abajo | una octava y una sexta más arriba |
| Corno, corno inglés | el Fa de abajo | una quinta justa más arriba |

La parte siempre se escribe más aguda de lo que suena, y dentro de una
familia por el mismo intervalo, octava más octava menos, así que una
canción en Do se escribe en Re para todos los instrumentos en Si♭, en
La para todos los instrumentos en Mi♭ y en Sol para todos los
instrumentos en Fa.

### Cómo usarlo

En la ventana Partitura guía, **Para** ofrece:

![La barra de herramientas de la ventana Lead sheet con el menú For abierto: Concert pitch, Bb instruments, Eb instruments y F instruments](../images/step-9-transposing-for-menu.png)

- **Altura real** escribe la canción tal como suena.
- **Instrumentos en Si♭**: trompeta, clarinete, saxofón soprano y tenor.
- **Instrumentos en Mi♭**: saxofón alto y barítono.
- **Instrumentos en Fa**: corno y corno inglés.

Pasa el ratón sobre una opción para ver para qué instrumentos es y
cuánto más arriba se escribe. La partitura cambia al momento: la
armadura, la melodía y los símbolos de acorde se mueven juntos, y la
esquina superior izquierda de la primera página dice para quién es la
parte, «Instrumentos en Si♭», donde las partes impresas nombran su
instrumento. **Exportar PDF…** propone un nombre de archivo con los
instrumentos, para que las partes de una canción no se pisen unas a
otras. La elección se recuerda: un saxofonista que abre otra canción la
encuentra escrita para él de nuevo.

Aquí está la demostración I Got Rhythm para instrumentos en Si♭. La
canción está en Si♭ mayor, dos bemoles; la parte está en Do mayor, sin
sostenidos ni bemoles. Esta es una de las razones por las que tantos
temas de jazz están en tonalidades con bemoles: el Si♭ real es Do para
la trompeta y el saxofón tenor, y el Mi♭ real es Do para el saxofón
alto.

![La partitura guía de I Got Rhythm para instrumentos en Si bemol: «B♭ instruments» arriba a la izquierda, sin armadura, los acordes Am, E7, Am, Dm, luego Am6, F7, Am, F, Am6, Fm6](../images/step-9-transposing-b-flat.png)

La demostración empieza con una introducción sobre acordes de Sol menor
(Gm, D7, Cm). En la parte para Si♭ son Am, E7 y Dm; en la parte para
Mi♭, en Sol mayor, Em, B7 y Am; en la parte para Fa, en Fa mayor, Dm, A7
y Gm. La canción, su archivo y lo que toca la banda se quedan como
están. Solo la partitura se escribe distinta.

### La tonalidad: nunca más de seis sostenidos o bemoles

Mover la tonalidad es sumar semitonos: dos para los instrumentos en Si♭
(de Si♭ a Do), nueve para los instrumentos en Mi♭ (de Mi♭ a Do) y siete
para los instrumentos en Fa (de Fa a Do).

```rust
pub fn semitones(self) -> u8 {
    self.note().pitch_class().semitones_up_to(PitchClass::C)
}
```

```rust
pub fn key(self, key: Scale) -> Scale {
    Scale::new(key.root.transpose(i32::from(self.semitones())), key.kind)
}
```

`note()` es la nota que suena con un Do escrito, Si♭ para
`Transposition::BFlat`, así que los nombres de los instrumentos y sus
distancias salen de un solo lugar. La clase de tonalidad se conserva:
Fa menor es Sol menor para una trompeta, Re dórico es Si dórico para un
saxofón alto.

La tonalidad nueva es un `Scale` como cualquier otro, así que se nombra
como la app nombra cualquier tonalidad (ver [Sostenidos o bemoles: cómo
se escribe una
tonalidad](../theory/scales.md#sostenidos-o-bemoles-cómo-se-escribe-una-tonalidad)):
cada una de las doce tonalidades tiene un nombre con como mucho seis
sostenidos o bemoles, y es el que se usa. Esto importa para las partes,
porque contar hacia arriba por nombres daría a veces tonalidades en las
que nadie escribe:

| Canción | Parte en Si♭ | Parte en Mi♭ | Parte en Fa |
|---|---|---|---|
| Do | Re | La | Sol |
| Fa | Sol | Re | Do |
| Si♭ | Do | Sol | Fa |
| Mi♭ | Fa | Do | Si♭ |
| La♭ | Si♭ | Fa | Mi♭ |
| Re♭ | Mi♭ | Si♭ | La♭ |
| Sol | La | Mi | Re |
| Re | Mi | Si | La |
| La | Si | Fa♯ | Mi |
| Mi | Fa♯ | Re♭ | Si |
| Si | Re♭ | La♭ | Fa♯ |
| Fa♯ | La♭ | Mi♭ | Re♭ |

Una canción en Mi para un saxofón alto, una sexta mayor más arriba,
estaría estrictamente en Do♯ mayor, siete sostenidos. Re♭ mayor tiene
las mismas notas con cinco bemoles, y es lo que escribe un copista. Una
canción en Fa♯ para una trompeta estaría en Sol♯ mayor, que necesita un
Fa doble sostenido en su armadura; La♭ mayor, cuatro bemoles, tiene las
mismas notas. Una prueba recorre las doce tonalidades mayores y menores
para cada instrumento y comprueba que ninguna parte tiene más de seis.

### Los acordes: movidos con nombre y todo

Un acorde tiene que conservar su lugar en la tonalidad: el V7 de la
canción es el V7 de la parte. Un cifrado se lleva a otra tonalidad
contando nombres además de semitonos (ver [Llevar una hoja de acordes a
otra
tonalidad](../theory/chords.md#llevar-una-hoja-de-acordes-a-otra-tonalidad)),
y la parte usa la misma función, de la tonalidad de la canción a la de
la parte:

```rust
pub fn harmony(self, harmony: HarmonyName, key: Scale) -> HarmonyName {
    harmony.moved(key, self.key(key))
}
```

Como va de tónica a tónica, sigue también la escritura de la tonalidad.
De Do a Re cada nombre sube uno, así que Dm7 G7 Cmaj7 pasa a Em7 A7
Dmaj7. De Fa♯ a La♭ cada nombre sube dos, así que el V7 de Fa♯, C♯7,
pasa a E♭7, el V7 de La♭, y nunca D♯7. La demostración de bossa nova
Garota de Ipanema está en Re♭; para un saxofón alto, en Si♭, sus D♭maj7
E♭7 E♭m7 D7 son B♭maj7 C7 Cm7 B7.

### Las notas: cada intervalo conserva su nombre

La melodía se mueve los mismos semitonos, pero sus notas también
necesitan nombres, y aquí un atajo saldría mal. La manera obvia es
subir cada nota y volver a escribirla en la tonalidad nueva con las
reglas de [Qué nombre de nota](#qué-nombre-de-nota-spellingrs). La
Marcha turca de Mozart, en La menor, muestra el problema. Su melodía se
apoya en Mi desde el semitono de abajo, Re♯–Mi. En Si menor, la
tonalidad de la trompeta, los mismos sonidos se volverían a escribir
como Fa–Fa♯: las reglas mantienen sencilla una tecla blanca fuera de la
tonalidad, así que escriben Fa en vez de Mi♯. Pero Re♯–Mi es una
segunda menor, una nota que se apoya en la siguiente, y Fa–Fa♯ es un
unísono aumentado, un mismo nombre alterado hacia arriba. Quien toca lee
otra línea.

Así que las notas se escriben primero en altura real, donde las reglas
conocen la tonalidad y los acordes en que se escribió la canción, y
luego cada nombre se mueve los mismos nombres y semitonos que la
tonalidad:

```rust
pub fn name(self, name: NoteName, key: Scale) -> NoteName {
    let written = self.key(key);
    if written == key {
        return name;
    }
    if !key.as_key().kind.is_tonal() {
        return written.spell(name.pitch_class().transpose(i32::from(self.semitones())));
    }
    let (from, to) = (key.spell(key.root), written.spell(written.root));
    let moved = name.above(
        from.pitch_class().semitones_up_to(to.pitch_class()),
        from.letter.steps_up_to(to.letter),
    );
    if moved.accidental.abs() >= 2 && written.spell(moved.pitch_class()) != moved {
        moved.simplified()
    } else {
        moved
    }
}
```

`NoteName::above` es la misma función que mueve las fundamentales de
los acordes, así que las notas y los acordes se mueven exactamente los
mismos nombres. El Re♯–Mi de la Marcha turca es Mi♯–Fa♯ para una
trompeta, Si♯–Do♯ para un saxofón alto y La♯–Si para un corno: una
segunda menor cada vez, como lo escribió Mozart. Una serie o un
conjunto no tienen nombres propios (el segundo `if`), así que sus notas
toman sus nombres habituales en la tonalidad nueva.

Las últimas líneas se ocupan de las tonalidades que se renombraron. Una
canción en Mi tiene una sexta rebajada, Do. Para un saxofón alto, en Re♭
en vez de Do♯, los nombres se mueven de Mi a Re, y Do pasa a ser Si
doble bemol (Si♭♭), la sexta rebajada de Re♭ escrita en sentido
estricto. Nadie quiere leer eso, y es la misma nota que La, así que el
nombre se simplifica, como en los símbolos de acorde. Un doble
sostenido o doble bemol que tiene la propia tonalidad nueva se queda: la
sensible de Do♯ menor armónica es Si♯, y en la parte de trompeta, en Re♯
menor armónica, es Do doble sostenido (Do♯♯), la sensible de esa
tonalidad.

### La octava: dónde se sienta la melodía en el pentagrama

El intervalo dice qué notas escribir, no en qué octava. Un saxofón tenor
suena una novena por debajo de lo escrito, una octava más grave que una
trompeta, y aun así en los libros de partituras guía para instrumentos
en Si♭ los dos leen la misma página: el tenor simplemente la suena una
octava más abajo. Lo que importa en la página es que la melodía se
siente en el pentagrama, no en una escalera de líneas adicionales por
encima.

Así que la parte se escribe por el intervalo del instrumento, movida
por octavas enteras hasta que la nota central de la melodía quede lo
más cerca posible de la línea central del pentagrama (Si4 en clave de
sol):

```rust
pub fn shift(self, pitches: &[Note], clef: Clef) -> i32 {
    let interval = i32::from(self.semitones());
    if self == Transposition::Concert {
        return 0;
    }
    let mut sorted = pitches.to_vec();
    sorted.sort();
    let Some(median) = sorted.get(sorted.len() / 2) else {
        return interval;
    };
    let middle = i32::from(clef.middle_line().midi());
    let median = i32::from(median.midi());
    (-4..=4)
        .map(|octaves| interval + 12 * octaves)
        .min_by_key(|shift| ((median + shift - middle).abs(), (shift - interval).abs()))
        .unwrap_or(interval)
}
```

La nota central es la **mediana**, la que queda en medio una vez
ordenadas las notas, como al elegir la clave: unas pocas notas muy
agudas o muy graves no la arrastran. Para una melodía en el registro de
una voz, el resultado es el propio intervalo del instrumento: una
segunda mayor arriba para los instrumentos en Si♭, una sexta mayor para
los de Mi♭, una quinta para los de Fa, así que trompeta, saxofón alto y
corno suenan la melodía donde está. Una melodía alrededor de Do6 sale
una séptima menor *más abajo* para los instrumentos en Si♭, los mismos
nombres una octava más abajo, con su nota central en Re5, donde una
trompeta la puede tocar.

`min_by_key` se queda con el candidato de clave más pequeña, y aquí la
clave es un par. Los pares se comparan como palabras en un diccionario:
por el primer elemento, y por el segundo solo cuando los primeros son
iguales. Así que decide la distancia a la línea central, y cuando dos
octavas están igual de cerca, gana la más cercana al intervalo propio
del instrumento.

Las partes para instrumentos transpositores usan la clave de sol, que
leen todos ellos; **Clave** todavía puede elegir la clave de fa, y
entonces la melodía se centra en la línea central de ese pentagrama,
Re3.

![La Marcha turca para instrumentos en Mi bemol: «E♭ instruments» arriba a la izquierda, Fa sostenido menor con tres sostenidos, Do sostenido menor con cuatro desde el compás 4, Fa sostenido menor otra vez desde el compás 6, y los acordes F♯m, C♯m, G♯ y C♯](../images/step-9-transposing-key-changes.png)

### En el código

La partitura guía está dividida en pasos, y solo algunos saben de
instrumentos:

- `transpose.rs` tiene el tipo `Transposition` con `key`, `harmony`,
  `name` y `shift`.
- `score.rs` le pide la tonalidad de cada compás, los acordes, los
  nombres de las notas y la octava. Dos closures lo mantienen corto:

  ```rust
  let key_at = |step: usize| transposition.key(keys.key_at(step));
  let harmony_of =
      |marker: ChordMarker| transposition.harmony(marker.harmony, keys.key_at(marker.start));
  ```

  Toman prestados `keys` y `transposition` de la función que los
  rodea, así que cada llamada solo dice qué paso o qué marca.
- `layout.rs` escribe «Instrumentos en Si♭» arriba a la izquierda de la
  primera página, con el bemol de la fuente musical como en los
  símbolos de acorde. Cuando el título es demasiado ancho para dejar
  sitio al lado, las palabras van en una línea debajo.
- `melody.rs` y `rhythm.rs` no saben nada de instrumentos: la melodía se
  lee, se escribe en altura real, se redondea y se corta en figuras de
  la misma manera para todos los instrumentos.

En la app, la transposición se guarda con el papel y el zoom en los
ajustes de la ventana:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
struct Settings {
    paper: Paper,
    zoom: f32,
    /// Who reads the sheet: a player keeps their instrument from song to
    /// song.
    transposition: Transposition,
}
```

`#[serde(default)]` deja que llegue un campo sin romper nada: los
ajustes guardados sin `transposition` lo toman de
`Settings::default()`, altura real. `Transposition` toma su valor por
defecto de un atributo en una variante:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Transposition {
    /// Written as it sounds: piano, guitar, bass, voice, flute, violin,
    /// trombone and every other instrument "in C".
    #[default]
    Concert,
    // ... BFlat, EFlat and F
}
```

El archivo del proyecto no cambia: quién lee la partitura es una
elección de quien la lee, no parte de la canción.

## Cómo se prueba

`lineup-core` prueba cada paso de la partitura guía por separado:

- **escritura** (`spelling.rs`): los nombres de la tonalidad; los
  nombres de un acorde (Sol♯ sobre E7 y La♭ sobre Fm en Do mayor); los
  pasos cromáticos según la dirección; las teclas blancas fuera de la
  tonalidad que se quedan sencillas; las tonalidades menores que suben
  su sensible; las alteraciones que duran hasta la barra de compás; las
  armaduras en las dos claves; los becuadros de un cambio de tonalidad;
- **ritmo** (`rhythm.rs`): la fuerza de cada lugar; las notas en el
  pulso que se sostienen por encima de pulsos; el centro de un compás
  de cuatro; una nota fuera del pulso que no esconde el pulso
  siguiente; la síncopa «♪ ♩ ♪» sin ligar; los pulsos compuestos enteros;
  los compases desiguales partidos en sus pulsos; los silencios que
  muestran cada pulso; las barras por pulso; cada trozo una figura
  escrita;
- **melodía y partitura** (`melody.rs`, `score.rs`): la nota de arriba
  de un acorde como melodía, las notas fuera de la cuadrícula en el paso
  más cercano, un clip repetido que vuelve a sonar, las notas ligadas
  por encima de la barra de compás, la clave de fa para melodías
  graves, barras o silencios en compases sin melodía, marcas de ensayo,
  repeticiones y cambios de tonalidad;
- **distribución** (`layout.rs`): una cabeza de nota por nota y un
  recuadro por compás, todo dentro de la página, una sección que empieza
  una línea nueva, acordes solos sobre barras, símbolos de acorde que
  nunca se pisan y usan alteraciones de verdad con cualquier nombre de
  notas, barras y ligaduras, repeticiones, y una parte que dice quién la
  lee, al lado o debajo del título;
- **partes** (`transpose.rs` y `score.rs`): el intervalo de cada
  instrumento; una canción en Do escrita en Re, La y Sol, las
  tonalidades menores y los modos que conservan su clase; ninguna
  tonalidad de parte con más de seis sostenidos o bemoles, para las doce
  tonalidades mayores y menores y cada instrumento; los acordes que
  conservan su lugar (ii–V–I en Do es Em7 A7 Dmaj7, C♯7 en Fa♯ es E♭7 en
  La♭, también un poliacorde y N.C.); las notas que conservan sus
  intervalos (Re♯–Mi en La menor pasa a Mi♯–Fa♯, Si♯–Do♯ y La♯–Si; la
  sexta rebajada de Mi es La en Re♭; Do♯♯ se queda en Re♯ menor
  armónica); la octava de una melodía alrededor de Sol4, de una melodía
  aguda y de una línea de bajo, en las dos claves; una parte de trompeta
  en Re mayor; un cambio de tonalidad en las tonalidades de las partes;
  la clave de sol para las partes.

La app tiene cuatro: cada símbolo de la fuente musical está en Bravura
al tamaño que cree la distribución, el texto se mide con la fuente, una
canción se vuelve un PDF con un objeto de página por página y sus
formas escritas una sola vez, y una parte se guarda con un nombre
propio, `Op. 24 (Bb instruments).pdf`.

Las pruebas no pueden decir si una partitura se lee bien, así que las
partituras también se leen como las lee un músico: Ode to Joy, Für
Elise, Minuet in G, Careless Love, Korobeiniki, Funiculì, funiculà, The
Entertainer, la Sonata Claro de luna, El Choclo, Greensleeves, La
Cumparsita, When the Saints Go Marching In e I Got Rhythm en altura
real, e I Got Rhythm, la Marcha turca, con sus muchos cambios de
tonalidad, y Garota de Ipanema como partes.
