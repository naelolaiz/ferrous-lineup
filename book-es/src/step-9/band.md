# La banda toca la hoja de acordes

Archivos: [`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-core/src/library/comping.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/comping.rs),
[`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs),
[`crates/lineup-app/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet.rs),
[`crates/lineup-app/src/app/edit.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/edit.rs)
y [`crates/lineup-app/src/app/library.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/library.rs)

La hoja de acordes del [último cambio](chord-sheet.md) contenía la armonía
de una canción pero no tocaba nada. Este cambio pone una banda detrás:
un piano, un bajo y una batería. Escribe `C`, Tab, `Am`, Tab, `Dm7
G7`, Tab, `C`, pulsa Espacio, y la tocan.

![El chord sheet con la banda: «🎷 Band: Pop beat», un botón Progression, «Key: C major · 4/4 · 110 BPM», y los compases C | Am | F | G7 con sus numerales](../images/step-9-band-sheet.png)

> **Las palabras de la banda.** Unas cuantas palabras aparecen a lo
> largo de todo este paso:
>
> - **comp**: un acorde tocado bajo la melodía; como verbo, tocar esos
>   acordes.
> - **comping**: el ritmo y los voicings que usa quien toca los acordes
>   para comp, como los golpes sincopados del piano.
> - **voicing**: qué notas de un acorde suenan, y en qué octavas.
> - **conducción de voces**: mover cada voz de un acorde a la voz más cercana
>   del siguiente, para que los acordes se unan con suavidad.
> - **changes**: la progresión de acordes de una canción, compás a
>   compás; «los changes» son lo que contiene la hoja de acordes.
> - **head / chorus**: el head es la melodía escrita, tocada al
>   principio y al final; un chorus es una pasada por la forma (en una
>   canción pop, la parte que vuelve después de cada estrofa).
> - **walking bass**: una nota por pulso: la fundamental, notas del
>   acorde, y un semitono hacia la fundamental siguiente.
> - **turnaround**: los acordes al final de una sección que llevan de
>   vuelta a su principio, como I vi ii V.
>
> Voicings y voice leading son palabras propias de la armonía: [La
> teoría musical como tipos](../step-3/theory.md) y [Las harmony
> keys](../step-8/harmony-keys.md) los muestran en acción.

## Probarlo

**Archivo > Nuevo** abre ahora una hoja de acordes vacía con una banda lista, en
modo canción. (La canción nueva de antes, un patrón vacío y una
batería, es **Archivo > Nuevo ritmo**.) En cuanto un compás contiene una
armonía, aparecen tres pistas en la lista de reproducción, **Piano**, **Bajo** y
**Batería**, con patrones llamados «Piano 1», «Bajo 1», «Batería 1»:

![El playlist con las tres pistas de la banda: Piano 1, Bass 1 y Drums 1, cuatro compases cada una](../images/step-9-band-playlist.png)

Son patrones corrientes. Haz doble clic en uno y el piano roll muestra
cada nota que toca la banda; el rack de canales muestra la batería paso a
paso. Nada de ellos está oculto, lo que importa en un programa que usas
para aprender: puedes ver cómo un pianista de bossa nova reparte un Dm7
por el compás, o dónde pone su nota de entrada un walking bass.

En la línea de arriba de la hoja:

- **🎷 Banda** elige cómo toca la banda. En este paso, cada uno de los 81
  ritmos de la biblioteca es un estilo de banda (112 hoy), ordenados
  por género, y cuatro más tocan sin batería: **Coral** (cada acorde
  mantenido, como toca un órgano un himno), **Bajo de Alberti** (el acorde
  desgranado grave, agudo, medio, agudo, como en las sonatas de
  Mozart), **Arpegios** y **Zarabanda** en 3/4. Apuntar a uno dice qué
  es, su compás y su tempo habitual. **Sin banda** manda a la banda a
  casa.
- **Progresión** escribe una progresión de acordes conocida desde el
  compás seleccionado, en la tonalidad de la canción: los 145 estilos
  de armonía de la biblioteca en este paso (177 hoy), por género, cada
  uno con sus acordes en el tooltip («Cmaj7 – Am7 – Dm7 – G7. I vi ii
  V: la progresión que cierra tantos standards»). Elige otro y continúa
  después del primero.
- **Terminar la canción aquí**, cuando haces clic derecho en un compás, dice
  dónde se detiene la banda. Un compás sin nada escrito continúa con la
  armonía anterior, así que una canción que acaba en un Do largo tiene
  `C` en su último compás y «Terminar la canción aquí» unos compases después.

![El menú de banda abierto en Jazz: Swing, Jazz ballad, Jazz waltz (3/4) y Second line, con el tooltip «The swing ride pattern in three beats. 3/4, about 150 BPM.»](../images/step-9-band-menu.png)

Hacer clic en un estilo o un ritmo en la **Biblioteca** mientras hay una
banda cambia también el estilo de la banda, con el tempo propio del
estilo si **Tempo** está marcado.

## Qué tocar, y cómo tocarlo

En la biblioteca del [paso 8](../step-8/library.md), un estilo es un
paquete: un chart de acordes, una manera de comparlos, un patrón de
bajo y un ritmo. «Bossa nova» significa Dm7 G7 Cmaj7 *y* los acordes
sincopados del piano *y* el bajo en el 1 y el «y» del 2 *y* los clics
del rim. Una banda de acompañamiento separa las dos mitades. Los
acordes son tuyos (la hoja); el estilo solo dice cómo tocarlos. Así
que este cambio divide la biblioteca de la misma manera, sin cambiarla:

| Mitad | Qué es | Dónde vive ahora |
|-------|--------|------------------|
| Qué tocar | una progresión de numerales | menú **Progresión**, del chart de cada `Style` |
| Cómo tocarla | comping, bajo, batería | menú **Banda**, un `BandStyle` |

Un `BandStyle` es un estilo sin los acordes:

```rust
pub struct BandStyle {
    pub name: &'static str,
    pub genre: Genre,
    pub time: TimeSignature,
    /// The tempo it is usually played at, in BPM.
    pub tempo: f32,
    pub comping: Comping,
    pub bass: Bass,
    /// The drums, or `None` for a band without them.
    pub rhythm: Option<&'static Rhythm>,
    pub about: &'static str,
}
```

Los estilos de banda no se escriben a mano. Cada ritmo de la biblioteca
ya sabía cómo van con él el piano y el bajo (`apply::feel`, la función
que responde «una samba se comp así, con este bajo»), así que la lista
se hace a partir de los ritmos la primera vez que se pide:

```rust
pub fn band_styles() -> &'static [BandStyle] {
    static STYLES: OnceLock<Vec<BandStyle>> = OnceLock::new();
    STYLES.get_or_init(|| {
        let mut styles: Vec<BandStyle> = RHYTHMS
            .iter()
            .map(|rhythm| {
                let (comping, bass) = apply::feel(rhythm);
                BandStyle { name: rhythm.name, /* … */ rhythm: Some(rhythm), /* … */ }
            })
            .chain(WITHOUT_DRUMS)
            .collect();
        // A stable sort: within a genre, the library's order stays.
        styles.sort_by_key(|style| style.genre);
        styles
    })
}
```

> **Rust: `OnceLock`.** Un `static` debe construirse en tiempo de
> compilación, y un `Vec` no puede (necesita el montículo).
> `std::sync::OnceLock` es una caja que empieza vacía y se llena una
> vez, la primera vez que corre `get_or_init`; cada llamada posterior
> recibe el mismo valor de vuelta. Varios hilos pueden correr a llamarla
> y solo corre una closure. El tipo de retorno `&'static [BandStyle]`
> dice «esta lista vive tanto como el programa», que es lo que permite
> que un `&'static BandStyle` se quede en el proyecto como el estilo de
> la banda sin ningún lifetime que seguir.

`sort_by_key` es un orden *estable*: los estilos del mismo género
conservan el orden que tenían, así que el menú los lista en el orden de
la biblioteca.

## Qué toca cada player

Un estilo de banda sabe *cómo*; la hoja sabe *qué*. Juntarlos pasa en
tres pasos.

**1. Las armonías, como notas.** `played_harmonies` recorre la hoja y
convierte cada armonía en un `Played`: cuándo empieza y acaba, las
notas que el piano mantiene para ella (su *voicing*), y lo que toca el
bajo debajo.

```rust
pub struct Played {
    pub start: usize,
    pub end: usize,
    pub voicing: Vec<Note>,
    pub bass: BassPart,
}

pub enum BassPart {
    /// A chord: its root (or the note after the slash) and the next
    /// harmony's bass, to lead into it.
    Chord { chord: Chord, note: Note, next: PitchClass },
    /// A group of notes with no root, such as a pitch-class set: the
    /// bass keeps the rhythm of its line on this one note and its octave.
    Pedal(Note),
    /// Nothing.
    Rest,
}
```

Los acordes se voicean como la biblioteca voicea un chart: cada uno
desde el anterior, moviendo cada voz lo menos que puede
(`voice_progression`), para que C Am F G7 no salte por el teclado. Sus
notas de bajo forman una línea (`bass_line`), así que el bajo de un
slash chord es la nota después de la barra: `G7/B` tiene Si en el bajo.

Las otras maneras en que Ferrous Lineup nombra la armonía se tocan por
sus notas:

- **Un conjunto de clases de alturas o un micromodo** no tiene
  fundamental, así que no hay nota de bajo «correcta». La banda toma la
  primera nota de su *forma normal* (su orden más compacto): 3-11 T2,
  las notas Re Fa La, se toca Re Fa La desde Do4 hacia arriba, sobre un Re. La
  elección es arbitraria pero fija, para que el mismo conjunto suene
  siempre igual.
- **Una forma de la serie** es una línea de doce notas, no un acorde.
  La banda la toca como cuatro acordes de tres notas, en el orden de la
  serie, uno detrás de otro a lo largo del compás, cada uno sobre su
  primera nota. Cortar una serie en grupos y hacer sonar cada grupo
  como un acorde es una manera habitual de armonizarla; Webern
  construyó la serie de su Concierto Op. 24 a partir de cuatro grupos
  así de tres.
- **N.C.** no toca nada; la batería sigue.

**2. El ritmo, ceñido a los compases.** Los patrones de comping y de
bajo de la biblioteca (`Comping::play`, `Bass::play`) empiezan de nuevo
en cada acorde. Eso está bien para un chart cuyos acordes cambian en la
línea del compás, pero una hoja puede decir `Dm7 G7` (un cambio en el
pulso 3) o `C / / G7` (en el pulso 4). Empezar el patrón de nuevo en el
pulso 3 movería el groove cada vez que cambia el acorde, como si el
baterista contara «uno» en medio del compás. Una banda mantiene su
groove y deja que los acordes caigan en él.

Así que `comping.rs` gana `Comping::play_in_bars`: los golpes del ritmo
se colocan en cada compás, desde su primer pulso, y cada golpe toca *la
armonía que esté sonando cuando cae*:

```rust
for bar in bars {
    let from = period_start(bar, period);
    for offset in (0..bar.end() - from).step_by(period) {
        for hit in hits {
            let at = from + offset + hit.at;
            if at < bar.start || at >= bar.end() {
                continue;
            }
            // The harmony sounding at `at`: the last one that started.
            let index = harmonies.partition_point(|harmony| harmony.start <= at);
            /* … play `pick(&harmony.voicing, hit.what)` at `at` … */
        }
    }
}
```

> **Rust: `partition_point`.** En un slice ordenado, `partition_point(p)`
> devuelve cuántos elementos del principio cumplen `p`, por búsqueda
> binaria. Las armonías están ordenadas por comienzo, así que
> `partition_point(|h| h.start <= at)` cuenta las armonías que han
> empezado para `at`; la que suena es la anterior a ese conteo.
> `checked_sub(1)` convierte «ninguna ha empezado todavía» en `None` en
> lugar de un desbordamiento.

Un acorde tan corto que ningún golpe cae en él (un G7 en el «y» del 4,
en un patrón que solo golpea en 1 y 3) se saltaría. Saltarse un acorde
es una armonía equivocada, así que ese acorde se golpea una vez donde
empieza.

Un patrón más largo que un compás, como la clave de dos compases de la
bossa nova, cuenta desde el compás 1: `period_start` lo empieza en los
compases impares y continúa en los pares, como cuenta un músico «uno,
dos» a lo largo de la pareja.

`Bass::play_in_bars` es la misma idea para el bajo: la línea empieza de
nuevo donde empieza cada armonía, para que la fundamental caiga en el
cambio, y de nuevo en cada línea de compás, para que un acorde
mantenido ocho compases no se convierta en una caminata larga. Solo el
último trozo de una armonía lleva hacia la siguiente.

Un **walking bass** toca una nota por pulso: la fundamental en el 1,
notas del acorde, y en el último pulso una nota un semitono por encima
o por debajo de la fundamental siguiente, que «lleva» hacia ella. En
4/4 sobre Do que va a Fa, eso es Do Mi Sol Mi. En un compás de tres,
fundamental, tercera, nota de entrada daría Do Mi Mi (la tercera de Do *es*
el semitono por debajo de Fa), así que la caminata en tres va
fundamental, quinta, nota de entrada: Do Sol Mi.

**3. Cada parte, como patrones.** Las notas de una canción entera no
caben en un patrón (un patrón tiene como mucho 256 pasos, 16 compases
de 4/4), y un patrón gigante sería difícil de leer de todos modos.
`chunks` corta la canción en tramos: compases enteros de un solo
compás, dentro de una sección, tan largos como pueda ser un patrón.
`write_part` corta las notas por esas líneas (una nota que dura más
allá del final de un tramo continúa al principio del siguiente) y
compara las notas de cada tramo con las de antes. Los tramos que tocan
exactamente las mismas notas comparten **un** patrón, con dos clips de
él, como copia un clip un productor en lugar de tocar la parte dos
veces. Una canción de dos estribillos idénticos recibe «Piano 1» dos
veces en la lista de reproducción.

Los patrones se escriben *sobre* los que el player ya tenía, así que
«Piano 1» sigue siendo el patrón número 5 (pongamos) mientras editas el
hoja: el piano roll que tienes abierto en él muestra las notas nuevas
en lugar de cerrarse.

Todo patrón que escribe la banda va etiquetado:

```rust
pub struct Pattern {
    /* … */
    /// The band's player whose part it holds, if the band wrote it (see
    /// [`crate::band`]). The band writes these patterns again when the
    /// chord sheet changes.
    pub player: Option<Player>,
}
```

Esa etiqueta es cómo la banda vuelve a encontrar su propio trabajo, y
cómo la biblioteca sabe dejarlo en paz: «Tocar toda la canción con este ritmo» se salta los patrones con player, porque las partes de la banda
cambian cuando cambia el estilo de la banda.

## Volver a tocar cuando cambia la hoja

La banda toca lo que la hoja dice *ahora*. Escribe un acorde, cambia
la tonalidad, cambia la métrica de la canción, arrastra una sección:
las partes se escriben de nuevo, en el mismo paso de deshacer que tu
cambio, para que un solo Ctrl+Z deshaga ambos.

La app lo hace en un solo sitio. Todo cambio al proyecto pasa por
`LineupApp::edit` (mira [Deshacer y rehacer: un historial de
copias](../step-5/history.md)), así que ahí es donde se le avisa a la
banda:

```rust
pub(super) fn edit<R>(
    &mut self,
    label: &'static str,
    change: impl FnOnce(&mut Project) -> R,
) -> R {
    let before = self.project.clone();
    let result = change(&mut self.project);
    // The band plays what the chord sheet says now, in the same step.
    self.project.follow_chord_sheet();
    if self.project != before {
        self.sync_audio(&before);
        self.history.record(label, before);
    }
    result
}
```

> **Rust: `impl FnOnce(&mut Project) -> R`.** `edit` toma cualquier
> closure que cambia el proyecto una vez y devuelve algo de tipo `R`,
> que `edit` entrega de vuelta. `R` es un tipo genérico elegido por
> cada quien llama: `self.edit("write a progression", |p|
> p.write_progression(style, from))` devuelve el `usize` de compases
> escritos, `self.edit("end the song", |p| p.end_song_at(bar))` un
> `bool`. `FnOnce` (en lugar de `FnMut` o `Fn`) es el bound más
> permisivo para quien llama: la closure puede incluso mover valores
> fuera de lo que capturó, ya que solo se llama una vez.

Escribir cada parte de nuevo después de cada clic sería un desperdicio:
mover un clip de la melodía no cambia nada de lo que toca la banda. Así
que `follow_chord_sheet` primero toma una **huella** de aquello desde
lo que se tocan las partes (el estilo, los marcadores de acordes, las
tonalidades, los compases, las secciones) y la compara con la tomada la
última vez:

```rust
pub fn follow_chord_sheet(&mut self) -> bool {
    let Some(band) = &self.band else {
        return false;
    };
    if band.heard == Some(self.band_inputs()) {
        return false;
    }
    self.band_channels();
    self.write_band();
    true
}
```

Una huella (un *hash*) es un número calculado a partir de unos datos,
tal que datos distintos casi seguro dan números distintos. Comparar dos
números de 64 bits es mucho más barato que comparar dos hojas de acordes, y
la banda solo tiene que guardar el número, no una copia de la hoja.

## Una parte que editas sigue siendo tuya

Una banda que escribe sus pistas de nuevo cada vez que pulsas ▶
pierde una edición de la línea de bajo la próxima vez que algo cambia.
Aquí, una parte que cambias a mano se **congela**: la banda la deja
exactamente como la dejaste, y la hoja lo dice:

![El chord sheet con el aviso «You edited the piano: the band keeps it as you left it.» y un botón «Play it again»](../images/step-9-band-frozen.png)

**Volver a tocarla** tira tu versión y hace que la banda escriba la parte
desde la hoja una vez más. Los demás players siguen siguiendo la hoja
como antes: en la imagen el piano está congelado, pero un acorde
escrito en el compás 2 sigue cambiando el bajo.

¿Cómo sabe la banda que editaste una parte, uses la herramienta que
uses (el piano roll, el rack de canales, arrastrar un clip, la
biblioteca)? No vigila las herramientas. Guarda, para cada player, una
huella de la parte que escribió la última vez, y la compara con una
huella de la parte tal como está ahora:

```rust
pub fn is_frozen(&self, player: Player) -> bool {
    self.band.as_ref().is_some_and(|band| {
        band.written[player.index()]
            .is_some_and(|written| written != self.part_print(player))
    })
}
```

`part_print` mezcla dónde empieza cada clip del player, cuántas veces
se repite, y cada nota de su patrón. En qué pista está un clip y qué
número tiene su patrón se dejan fuera, así que mover la pista Piano
hacia abajo, o borrar un patrón tuyo (que renumbra los que van
después), no congela nada.

> **Rust: `Option::is_some_and`.** `opt.is_some_and(|x| test(x))` es
> `true` solo si hay un valor y pasa la prueba. Dos anidados se leen
> como una sola frase: «hay una banda, y ha escrito la parte de este
> player, y eso no es lo que hay ahora».

### FNV-1a

Las huellas se guardan en el archivo de proyecto, para que una parte
que congelaste siga congelada cuando abras la canción mañana. El hasher
de la biblioteca estándar (`DefaultHasher`) no promete dar el mismo
número en la próxima versión de Rust, así que no se puede guardar. La
banda usa **FNV-1a** en su lugar, un hash clásico de unas pocas líneas:

```rust
struct Fingerprint(u64);

impl Fingerprint {
    fn new() -> Self {
        Fingerprint(0xCBF2_9CE4_8422_2325)
    }

    fn bytes(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.0 ^= u64::from(byte);
            self.0 = self.0.wrapping_mul(0x0100_0000_01B3);
        }
    }
}
```

Por cada byte: mézclalo en el número con un XOR y luego multiplica por
un primo grande, `0x100000001B3`. El número inicial y el primo son las
constantes publicadas del algoritmo, así que todo programa que
implementa FNV-1a obtiene el mismo resultado.

> **Rust: `wrapping_mul`.** Multiplicar dos `u64`s grandes desborda. En
> una compilación de depuración un `*` pelado detendría el programa con
> «attempt to multiply with overflow»; `wrapping_mul` dice que conservar
> solo los 64 bits bajos es lo que queremos, como hace un hash a
> propósito. Los guiones bajos de `0xCBF2_9CE4_8422_2325` están solo
> para que los dígitos se lean.

### Mover la canción entera no es una edición

**Tonalidad > Re mayor** transpone cada nota de la canción, incluidas las de
la banda (mira [Cambiar de tonalidad](../step-8/changing-key.md)). Las
partes de la banda ya no encajan con sus huellas, pero tú no las
editaste: la herramienta lo movió todo. Cambiar la métrica y copiar una
sección son lo mismo. Estas herramientas están envueltas en
`moving_the_band`, que apunta qué partes seguían la hoja antes,
ejecuta la herramienta, y luego toma sus huellas nuevas como propias de
la banda:

```rust
pub(crate) fn moving_the_band<R>(&mut self, change: impl FnOnce(&mut Project) -> R) -> R {
    let following: Vec<Player> = Player::ALL
        .into_iter()
        .filter(|&player| /* written by the band, and not frozen */)
        .collect();
    let result = change(self);
    /* … for each player in `following`, written = part_print(player) … */
    result
}

pub fn change_key(&mut self, scale: Scale) -> bool {
    self.moving_the_band(|project| project.move_into_key(scale))
}
```

Una parte congelada la mueve la herramienta como cualquier otra nota, y
sigue congelada: tu versión del piano, ahora en Re.

## Cambiar el estilo

Elegir un estilo escribe de nuevo toda parte que no esté congelada. Dos
cosas pueden cambiar con él:

- **El compás.** Un estilo en otro compás pone la canción en él, compás
  a compás, como hace un ritmo de la biblioteca (mira [Un ritmo en otro
  compás](../step-8/re-metering.md)). Cada compás conserva su longitud,
  así que el tempo se escala: una canción en 4/4 a 110 BPM elegida como
  Vals de jazz pasa a 3/4 a 82,5 BPM, y cada compás dura los mismos 2,18
  segundos que antes (4 × 60 / 110 = 3 × 60 / 82,5). Pon después el
  tempo que quieras.
- **El tempo.** Mientras la hoja está vacía, la canción toma el tempo
  habitual del estilo. Una vez que has escrito acordes, el tempo es
  tuyo y se queda.

El piano y el bajo reciben el instrumento General MIDI que encaja con
el género (un piano eléctrico para los estilos soul y electrónicos, un
contrabajo para jazz y tango, un bajo sintetizado para hip hop), para
que suenen bien por MIDI hacia un sintetizador externo.

## Una banda sobre una demo

Abre una demo, pongamos **Canon en Re**, abre la hoja de acordes y elige una
banda. Una demo ya toca su propio acompañamiento, así que primero la
banda:

1. escribe en la hoja los acordes que toca la canción, si está vacía
   (el **Escribir los acordes que toca la canción** del último cambio), y
2. **silencia** los canales propios de acordes y bajo de la canción (la
   biblioteca los llama «Teclados» y «Bajo») y sus pistas que solo
   contienen batería,

para que los dos no toquen uno encima del otro. La melodía sigue
sonando, ahora con la banda detrás, y la barra de estado dice qué se
silencia.

![El chord sheet de Canon in D con una banda Pop ballad: D A Bm F#m | G D G A, la progresión del Canon de Pachelbel, escrita compás a compás](../images/step-9-band-demo.png)

**Sin banda** quita los patrones de la banda, sus pistas y sus canales
(solo los que añadió, y solo si nada más suena en ellos), y le quita el
mute a lo que silenció. Recuerda lo que silenció por *nombre*, no por
número, para que vuelvan los canales correctos incluso después de que
hayas añadido o quitado otros por el camino. Los acordes se quedan en
la hoja.

## En el archivo

Un proyecto con banda se guarda en el formato 13: el estilo de la banda
por nombre, dónde acaba, sus canales, las huellas, y lo que silenció.
En RON (el formato de archivo, mira [Guardar y cargar
proyectos](../step-4/saving.md)):

```ron
band: Some((
    style: "Pop beat",
    end: 64,
    piano: Some(0),
    bass: Some(1),
    written: (Some(18236602816702727710), Some(945158731411533472), Some(10321456850122308024)),
    heard: Some(10198668343261403180),
    muted: (),
    since: 0,
)),
```

y cada patrón de la banda lleva `player: Some(Piano)`. Un archivo cuya
banda toca un estilo que esta versión no conoce, o en un canal que no
existe, se rechaza con un mensaje que lo dice en lugar de abrirse mal.

> **Rust: `#[serde(default)]`.** Los campos añadidos después, como
> `since` y `muted`, llevan `#[serde(default)]`: un archivo que no los
> tiene se carga igualmente, con el valor por defecto del campo (`0`,
> una lista vacía). Los campos que suelen estar vacíos llevan también
> `skip_serializing_if = "Vec::is_empty"`, para que el archivo solo
> diga lo que hay.

## Ejercicios

1. Escribe `C | Am | F | G7` y elige **Vals de jazz**. Mira el bajo en el
   piano roll: ¿qué nota toca en el pulso 3 de cada compás, y hacia qué
   fundamental lleva? Luego elige **Swing** (4/4): ¿qué pone la
   caminata en el pulso 3 ahora?
2. Escribe un compás `Dm7 G7` y un compás `C / / G7`. Abre la parte de
   piano. ¿En qué pulso empieza el G7 en cada uno, y cambia el ritmo de
   comping en esos compases?
3. Elige **Bajo de Alberti** sobre los mismos acordes. El patrón es
   «grave, agudo, medio, agudo». ¿Qué nota del acorde está grave en el
   compás de Am, y por qué podría no ser la fundamental La? (Pista:
   voice leading.)
4. Edita una nota del bajo, y luego escribe un acorde nuevo en el
   compás 2. ¿Qué partes cambiaron? Haz clic en **Volver a tocarla**. ¿Qué
   le pasó a tu nota?
5. Abre **Canon en Re**, elige **Balada pop**, y mira el rack de canales.
   ¿Qué canales están silenciados? Elige **Sin banda**: ¿la demo ha
   vuelto exactamente a como era?
6. En `band.rs`, `part_print` deja fuera la pista en la que está un
   clip. ¿Qué iría mal para el usuario si no lo hiciera?
7. (Rust) `band_styles()` devuelve `&'static [BandStyle]`. ¿Por qué
   puede `Band` guardar un `&'static BandStyle` sin un parámetro
   lifetime, mientras que no podría guardar un `&BandStyle` prestado de
   un `Vec` local?
