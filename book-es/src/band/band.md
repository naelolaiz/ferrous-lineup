# Cómo toca la banda una hoja de acordes

Archivos: [`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-core/src/band/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates/lineup-core/src/band),
[`crates/lineup-core/src/library/comping.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/comping.rs),
[`crates/lineup-core/src/library/follow.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/follow.rs),
[`crates/lineup-app/src/chord_sheet/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet/band.rs)
y [`crates/lineup-app/src/app/follow.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/follow.rs)

La hoja de acordes guarda la armonía de una canción. La banda la toca.
Escribe `C`, Tab, `Am`, Tab, `Dm7 G7`, Tab, `C`, pulsa Espacio, y un
piano, un bajo y una batería tocan esos acordes, con una guitarra,
cuerdas, un órgano o una sección de vientos donde el estilo los tiene.
Cambia el estilo, la tonalidad o un acorde, y la banda lo vuelve a tocar
de la manera nueva.

![La hoja de acordes de una canción con la banda de Pop beat: los botones Band y Progression, «Key: C major · 4/4 · 110 BPM», las filas de la banda (Form, Choruses, Ending, y You play Piano, Bass, Drums, Guitar), y los compases C | Am | F | G7 con sus números romanos](../images/step-9-band-sheet.png)

Las partes de la banda son patrones corrientes en pistas propias,
llamados «Piano 1», «Bajo 1», «Batería 1». Haz doble clic en uno y el
piano roll muestra cada nota que toca la banda; nada está escondido, lo
que importa en un programa con el que se aprende: puedes ver cómo un
pianista de bossa nova reparte un Dm7 por el compás, o dónde pone un
bajo caminante su nota de entrada.

![La lista de reproducción con las tres pistas de la banda: Piano 1, Bass 1 y Drums 1, de cuatro compases cada una](../images/step-9-band-playlist.png)

Este capítulo cuenta cómo se separa un estilo de los acordes que toca,
qué toca cada músico y cómo, cómo se vuelven a escribir las partes
cuando cambia la hoja sin perder lo que editaste, la banda más grande
(guitarra, cuerdas, órgano, vientos), y el modo de teclado arreglador
en que la banda sigue el acorde que mantienes pulsado. La forma de la
canción, las variaciones y los estilos por sección están en [Forma,
secciones, variaciones y bloqueos](song-form.md); los cortes
en lo que toca la banda, en [Golpes, sostenidos, silencios y
anticipaciones](stops.md). Cómo se usa la banda está en los
capítulos del manual [Tu primera
canción](https://naelolaiz.github.io/ferrous-lineup/es/manual/first-song.html)
y [Una banda más grande y los
vientos](https://naelolaiz.github.io/ferrous-lineup/es/manual/bigger-band.html).

## La música, en pocas palabras

Unas cuantas palabras aparecen en todo el código de la banda:

- **Comp** (de *accompany*): un acorde tocado debajo de la melodía; como
  verbo, tocar esos acordes. El **comping** es el ritmo y las
  disposiciones que usa quien toca los acordes, como los acordes
  sincopados de un pianista de bossa.
- **Disposición** (*voicing*): qué notas de un acorde suenan y en qué
  octavas. La **conducción de voces** lleva cada voz de un acorde a la
  voz más cercana del siguiente, para que los acordes se unan con
  suavidad.
- **Cambios** (*changes*): la progresión de acordes de una canción,
  compás por compás; lo que guarda la hoja de acordes.
- **Bajo caminante** (*walking bass*): una nota por pulso, primero la
  fundamental, notas del acorde, y medio tono hacia la fundamental
  siguiente.
- **Colchón** (*pad*): acordes sostenidos suavemente debajo de la banda,
  normalmente por cuerdas o un órgano.
- **Vuelta** (*turnaround*): los acordes al final de una sección que
  llevan de nuevo a su comienzo, como I vi ii V.

## Qué tocar y cómo tocarlo

Un estilo de la [biblioteca](library.md) es un paquete:
«Bossa nova» quiere decir Dm7 G7 Cmaj7 *y* los acordes sincopados del
piano *y* el bajo en el 1 y en el «y» del 2 *y* los golpes de aro. Una
banda de acompañamiento separa las dos mitades. Los acordes son tuyos,
en la hoja; el estilo solo dice cómo tocarlos.

| Mitad | Qué es | Dónde se elige |
|-------|--------|----------------|
| Qué tocar | una progresión de números romanos | el menú **Progresión** de la hoja, del cifrado de cada `Style` |
| Cómo tocarlo | comping, bajo, batería, los músicos que añade | la **Biblioteca**, como un `BandStyle` |

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
ya sabe cómo van con él el piano y el bajo (`apply::feel`, ver [Tocar
una canción en otro
estilo](library.md#tocar-una-canción-en-otro-estilo)), así
que la lista se hace a partir de los ritmos la primera vez que se pide,
con cuatro estilos sin batería añadidos: **Coral** (cada acorde
sostenido, como un órgano toca un himno), **Bajo de Alberti** (el
acorde quebrado grave, agudo, medio, agudo, como en las sonatas de
Mozart), **Arpegios** y **Zarabanda** en 3/4.

```rust
pub fn band_styles() -> &'static [BandStyle] {
    static STYLES: OnceLock<Vec<BandStyle>> = OnceLock::new();
    STYLES.get_or_init(|| {
        let mut styles: Vec<BandStyle> = RHYTHMS
            .iter()
            .map(|rhythm| {
                let (comping, bass) = apply::feel(rhythm);
                BandStyle {
                    name: rhythm.name,
                    genre: rhythm.genre,
                    time: rhythm.time,
                    tempo: rhythm.tempo,
                    comping,
                    bass,
                    rhythm: Some(rhythm),
                    about: rhythm.about,
                }
            })
            .chain(WITHOUT_DRUMS)
            .collect();
        // A stable sort: within a genre, the library's order stays.
        styles.sort_by_key(|style| style.genre);
        styles
    })
}
```

Un `static` tiene que construirse al compilar el programa, y un `Vec`
no se puede, porque necesita memoria del heap. `std::sync::OnceLock` es
una caja que empieza vacía y se llena una vez, la primera vez que se
ejecuta `get_or_init`; cada llamada posterior recibe el mismo valor.
Varios hilos pueden competir por llamarla y solo se ejecuta un closure.
El tipo de retorno `&'static [BandStyle]` dice «esta lista vive tanto
como el programa», y eso permite que un `&'static BandStyle` esté en el
proyecto como estilo de la banda sin ningún tiempo de vida que seguir.
`sort_by_key` es estable, así que los estilos de un mismo género
conservan el orden de la biblioteca.

## Una banda en la canción

**Archivo > Nuevo** hace una canción con una hoja vacía y una banda
lista para tocarla en **Ritmo pop**:

```rust
pub fn with_band() -> Self {
    let mut project = Project::empty();
    project.set_band_style(BandStyle::named(DEFAULT_STYLE));
    project.remove_unused_tracks();
    project
}
```

A partir de ahí, tres lugares dan forma a la banda:

- **La Biblioteca.** Un clic en un estilo o un ritmo lo convierte en la
  manera de tocar de la banda, con las partes marcadas en su ventana:
  la batería, las partes de acordes y bajo, las voces, la progresión de
  acordes, el compás, el tempo y el swing (ver [Qué partes
  tomar](library.md#qué-partes-tomar)). Una canción con música propia
  pero sin banda recibe una cuando se toman las partes de acordes y bajo
  con las voces.
- **El menú 🎷 Banda de la hoja** elige quién toca y con qué sonido:
  una **Formación de instrumentos** («Piano, guitarra acústica +
  cuerdas», «Cornos, timbales + cuerdas» y seis más), una casilla y un
  instrumento para cada músico, la **Sección de vientos**, el
  **Solista** (ver [El solista](soloist.md)) y **Sin banda**.
- **El menú Progresión de la hoja** es la Biblioteca otra vez, mostrando
  solo los estilos con una progresión de acordes; un clic escribe esa
  progresión desde el compás seleccionado, en la tonalidad de la
  canción: I vi ii V en Do es Cmaj7 Am7 Dm7 G7, la vuelta con que
  terminan tantos standards. Un acorde que ya está sonando no se vuelve
  a escribir, así que «I | I | IV» escribe C, nada, y después F, como
  un cifrado. En una hoja vacía, la canción toma el modo de la
  progresión: «i bVII bVI» es Am G F en La menor.

Como las partes pueden venir de lugares distintos, la banda las guarda
por separado:

```rust
pub struct Band {
    pub custom_rhythm: Option<Box<crate::library::UserStyle>>,
    pub custom_instruments: Option<Box<crate::library::UserStyle>>,
    pub style: &'static BandStyle,
    /// The instrumentation can come from a different style than the rhythm.
    pub instrument_style: &'static BandStyle,
    /// The exact library style supplying the comping, when one was picked.
    pub harmony_style: Option<&'static Style>,
    /// The chords and the bass played as plain accents on the beats: how
    /// songs saved so play. The Library no longer chooses it.
    pub accents_only: bool,
    /// Whose drums play, when not those of `style` or `custom_rhythm`.
    pub drums: BandDrums,
    // ...
}

pub enum BandDrums {
    /// Those of the style the band plays the chords and the bass in, or
    /// of the saved style it plays them from.
    Style,
    /// Those of another style.
    Of(&'static BandStyle),
    /// Those of a saved style.
    Saved(Box<crate::library::UserStyle>),
    /// None: the song's own drums play.
    Song,
}
```

`style` dice cómo se tocan los acordes y el bajo; `drums` dice de quién
es la batería que toca con ellos; `instrument_style` dice quién toca y
con qué; y `harmony_style` es el estilo de la biblioteca elegido, cuando
su comping difiere del de su ritmo. Los dos campos `custom_` guardan
estilos tuyos (ver [Tus propios estilos](style-editor.md)).
`accents_only` toca los acordes y el bajo en los pulsos en que se
siente el compás, sea cual sea el estilo; solo las canciones guardadas
así siguen tocando de ese modo.

Tomar las partes de acordes y bajo de un estilo pero no su batería
conserva la batería que tocaba la banda: `drums` pasa de `Style` a
`Of` el estilo anterior, así que deja de seguir a `style`. Tomar solo
la batería pone `Of` el nuevo. Cuando las dos vuelven a ser la misma,
`tidy_drums` convierte `Of` otra vez en `Style`, así que una banda
tiene una sola manera de decir «la batería de este estilo», y su
archivo sigue siendo uno que una versión anterior puede leer. Cada
músico le pregunta a la banda, no al estilo, cómo tocar:

```rust
fn comping_in(&self, variation: Variation) -> Comping {
    if self.accents_only {
        Comping::Accents
    } else {
        self.playing_style().comping_in(variation)
    }
}
```

Elegir un estilo puede cambiar tres cosas más. Un estilo en otro compás
pasa la canción a ese compás, compás por compás, como lo hace un ritmo
de la biblioteca (ver [Pasar una canción a otro
compás](../sound/tempo.md#pasar-una-canción-a-otro-compás)): cada
compás conserva su duración en segundos, así que una canción en 4/4 a
110 BPM tocada como Vals de jazz pasa a 3/4 a 82,5 BPM (4 × 60 / 110 =
3 × 60 / 82,5 = 2,18 segundos por compás). El tempo es el del estilo
mientras la hoja está vacía, y el tuyo una vez que escribiste acordes. Y
la canción toma el swing del estilo, porque el swing es parte de cómo
se toca un estilo. El piano y el bajo reciben el instrumento General
MIDI del género (un piano eléctrico para los estilos soul y
electrónicos, un contrabajo para el jazz y el tango, un bajo
sintetizado para el hip hop), para que suenen bien con un SoundFont o
por la salida MIDI.

## Qué toca cada músico

Un estilo de banda sabe *cómo*; la hoja sabe *qué*. Juntarlos lleva tres
pasos.

### Las armonías, como notas

`played_harmonies` recorre la hoja y convierte cada armonía en un
`Played`: cuándo empieza y termina, las notas con que se tocan los
acordes (su disposición) y lo que toca el bajo debajo.

```rust
pub struct Played {
    pub start: usize,
    pub end: usize,
    /// The notes the chords are played with, from low to high. Empty for
    /// no chord: nothing plays.
    pub voicing: Vec<Note>,
    /// What the bass plays.
    pub bass: BassPart,
}

/// What the bass plays under a harmony.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BassPart {
    /// A chord: its line goes over the chord's notes from `note` (its
    /// root, or a slash chord's bass), and may lead into `next`, the
    /// bass note of the harmony after it.
    Chord {
        chord: Chord,
        note: Note,
        next: Note,
    },
    /// A group of notes with no root, such as a pitch-class set: the
    /// bass keeps the rhythm of its line on this one note and its octave.
    Pedal(Note),
    /// Nothing.
    Rest,
}
```

Los acordes se disponen como la biblioteca dispone un cifrado, cada uno
a partir del anterior, moviendo cada voz lo menos posible, para que
C Am F G7 no salte por todo el teclado (ver [Conducción de
voces](../theory/voice-leading.md)). Sus notas del bajo forman una
línea (`bass_line`), así que `G7/B` tiene Si en el bajo. Las otras
maneras en que Ferrous Lineup nombra la armonía se tocan por sus notas:

- **Un poliacorde** se toca como está escrito: el acorde de abajo, el de
  arriba apilado justo encima, sobre el bajo del acorde de abajo.
- **Un conjunto de clases de altura o un micromodo** no tiene
  fundamental, así que no hay una nota del bajo «correcta». La banda
  toma la primera nota de su forma normal (su orden más compacto):
  3-11 T2, Re Fa La, se toca desde el Do central hacia arriba, sobre un
  Re. La elección es arbitraria pero fija, así que el mismo conjunto
  siempre suena igual.
- **Una forma de serie** es una línea de doce notas, no un acorde. La
  banda la toca como cuatro acordes de tres notas, en el orden de la
  serie, uno tras otro, cada uno sobre su primera nota. Cortar una
  serie en grupos y hacer sonar cada grupo como un acorde es una manera
  habitual de armonizarla; Webern construyó la serie de su Concierto,
  Op. 24, con cuatro grupos así.
- **N.C.** no toca nada; la batería sigue.

### El ritmo, ajustado a los compases

Los patrones de comping de la biblioteca empiezan de nuevo en cada
acorde. Eso está bien para un cifrado donde los acordes cambian en la
barra de compás, pero una hoja puede decir `Dm7 G7` (un cambio en el
pulso 3) o `C / / G7` (en el 4). Empezar el patrón de nuevo en el pulso
3 movería el groove cada vez que cambia el acorde, como si el baterista
contara «uno» en medio del compás. Una banda mantiene su groove y deja
que los acordes caigan dentro. Así que `Comping::play_in_bars` pone los
golpes del ritmo en cada compás, desde su primer pulso, y cada golpe
toca la armonía que esté sonando cuando cae:

```rust
for bar in bars {
    let (period, hits) = self.hits(bar.time);
    let from = if self.follows_the_bar() {
        bar.start
    } else {
        period_start(bar, period)
    };
    for offset in (0..bar.end() - from).step_by(period) {
        for hit in hits.iter() {
            let at = from + offset + hit.at;
            if at < bar.start || at >= bar.end() {
                continue;
            }
            let index = harmonies.partition_point(|harmony| harmony.start <= at);
            let Some(index) = index.checked_sub(1) else {
                continue;
            };
            // ...
        }
    }
}
```

Las armonías están ordenadas por su comienzo, así que `partition_point`
cuenta, con una búsqueda binaria, las armonías que ya empezaron en
`at`; la que suena es la anterior. Un acorde tan corto que ningún golpe
cae en él (un G7 en el «y» del 4, en un patrón que solo golpea en el 1
y el 3) no se oiría, y un acorde saltado es una armonía equivocada, así
que un acorde así se golpea una vez donde empieza. Un patrón más largo
que un compás, como la clave de dos compases de la bossa nova, cuenta
desde el compás 1: `period_start` lo empieza en los compases impares y
lo sigue en los pares, como un músico cuenta «uno, dos» a lo largo del
par.

### Pulsos para cualquier compás

La mayoría de los patrones de comping son una tabla fija de golpes. Dos
no: los **acentos simples** (`Comping::Accents`), un acorde en cada
pulso en que se siente el compás, y el **aksak** de los estilos
balcánicos, los contratiempos dentro de cada pulso. En 7/8 sentido como
2+2+3 caen en un lugar, en 5/4 sentido como 2+3 en otro, así que sus
golpes se construyen a partir de los `felt_groups` del compás (ver
[Pulsos de distinta
longitud](../sound/tempo.md#pulsos-de-distinta-longitud)):

```rust
fn hits(self, time: TimeSignature) -> (usize, Cow<'static, [Hit<Voices>]>) {
    match self {
        // ...
        Comping::Aksak => {
            let unit = time.unit_steps();
            // A chord a little shorter than its note, so the next
            // one is heard to start, but never under a step.
            let length = (unit - unit / 4).max(1);
            let mut hits = Vec::new();
            let mut at = 0;
            for group in time.felt_groups() {
                for note in 1..usize::from(group) {
                    let velocity = if note == 1 { 84 } else { 74 };
                    hits.push(hit!(at + note * unit, length, Voices::All, velocity));
                }
                at += usize::from(group) * unit;
            }
            (time.steps_per_bar(), Cow::Owned(hits))
        }
        _ => {
            let (period, hits) = self.pattern();
            (period, Cow::Borrowed(hits))
        }
    }
}
```

La función devuelve un **`Cow`**, «clonar al escribir» (*clone on
write*; ver [El orden de
todo](../program/sync.md#el-orden-de-todo)): los patrones fijos
entregan sin coste un préstamo de su tabla `static`, `Cow::Borrowed`, y
los dos construidos un `Vec` propio, `Cow::Owned`. Quien llama lee los
dos de la misma manera. Estos dos patrones cuentan desde la barra de
compás, empiece donde empiece el acorde (`follows_the_bar`), ya que los
pulsos de un compás irregular están donde los pone el compás.

### El bajo

`Bass::play_in_bars` es la misma idea para la línea del bajo: empieza de
nuevo donde empieza cada armonía, para que la fundamental caiga en el
cambio, y de nuevo en cada barra de compás, para que un acorde
sostenido ocho compases no se convierta en una sola caminata larga.
Solo el último trozo de una armonía conduce a la siguiente. Un bajo
caminante toca una nota por pulso:

```rust
fn walk(span: usize, down: bool) -> Vec<(usize, Tone)> {
    const UP: [Tone; 6] = [
        Tone::Third,
        Tone::Fifth,
        Tone::Seventh,
        Tone::Octave,
        Tone::Fifth,
        Tone::Third,
    ];
    const DOWN: [Tone; 6] = [
        Tone::LowSeventh,
        Tone::LowFifth,
        Tone::LowThird,
        Tone::LowFifth,
        Tone::LowSeventh,
        Tone::Root,
    ];
    let beats = span.div_ceil(4);
    (0..beats)
        .map(|beat| {
            let tone = if beat == 0 {
                Tone::Root
            } else if beat == beats - 1 {
                Tone::Approach
            } else if beats == 3 {
                if down { Tone::LowFifth } else { Tone::Fifth }
            } else if down {
                DOWN[(beat - 1) % DOWN.len()]
            } else {
                UP[(beat - 1) % UP.len()]
            };
            (beat * 4, tone)
        })
        .collect()
}
```

La fundamental en el 1, notas del acorde en el medio, y en el último
pulso la **aproximación**, una nota medio tono por encima o por debajo
de la nota del bajo del acorde siguiente. La línea recibe esa nota
misma, no solo su nombre: una fundamental se toca entre Do2 y Si2 (ver
[Las armonías, como notas](#las-armonías-como-notas)), así que de G7 a
C la nota siguiente es Do2, y la aproximación es el medio tono del lado
más cercano a la nota anterior, Re♭2 bajando desde Re2. Sabiendo solo
«Do», la línea podría elegir Si2 y después caer una séptima al Do2 que
sigue.

Las notas del medio suben por el acorde (`UP`) o bajan por él (`DOWN`,
las notas del acorde por debajo de la fundamental). `Bass::play` arma
las dos líneas y se queda con la que llega al acorde siguiente con
pasos más chicos, la que baja solo mientras se mantenga por encima de
Mi1 (`BASS_FLOOR`), la cuerda más grave de un bajo eléctrico. Así,
sobre C yendo a Am la caminata sube, Do2 Mi2 Sol2 Sol♯2, hacia La2, y
sobre G7 yendo a C baja, Sol2 Fa2 Re2 Re♭2, hacia Do2. En un compás de
tres la caminata va fundamental, quinta, aproximación: sobre C7 yendo a
F, Do2 Sol2 Fa♯2, hacia Fa2. La tercera queda fuera porque la tercera
de Do *es* el medio tono bajo Fa: la línea llegaría a su aproximación
un pulso antes.

### La batería

La batería toca el ritmo del estilo en cada compás, con su relleno en
el último compás de cada ocho (`PHRASE_BARS`, como se construyen la
mayoría de las canciones), en el último compás antes de una sección o
de un cambio de variación, y al final de la introducción y de la vuelta.
Un cambio de variación o de estilo empieza con un platillo crash. Las
variaciones, la introducción y el final están en [Forma, secciones,
variaciones y bloqueos](song-form.md).

### Cada parte, como patrones

Una canción entera no cabe en un patrón (un patrón tiene como mucho 256
pasos, 16 compases de 4/4), y un patrón gigante sería difícil de leer.
`chunks` corta la canción en tramos: compases enteros de un mismo
compás, dentro de una sección, tan largos como puede ser un patrón.
`write_part` corta las notas en esas líneas (una nota que dura más allá
del final de un tramo sigue al comienzo del siguiente) y compara las
notas de cada tramo con las de los anteriores:

```rust
let index = match contents.iter().position(|known| *known == content) {
    Some(index) => index,
    None => {
        contents.push(content);
        contents.len() - 1
    }
};
```

Los tramos que tocan exactamente las mismas notas comparten **un**
patrón, con dos clips de él, como un productor copia un clip en lugar
de tocar dos veces una parte. Una canción de dos vueltas iguales tiene
«Piano 1» dos veces en la lista de reproducción. Los patrones se
escriben encima de los que ya tenía el músico, así que «Piano 1»
conserva su número mientras editas la hoja, y un piano roll abierto
sobre él muestra las notas nuevas en lugar de cerrarse. Cada patrón que
escribe la banda lleva una etiqueta:

```rust
/// The band's player whose part it holds, if the band wrote it (see
/// [`crate::band`]). The band writes these patterns again when the
/// chord sheet changes.
pub player: Option<Player>,
```

Esa etiqueta es la manera en que la banda vuelve a encontrar su propio
trabajo, y en que la biblioteca sabe que debe dejarlo en paz cuando
toca una canción entera en otro ritmo.

## Volver a tocar cuando cambia la hoja

La banda toca lo que la hoja dice *ahora*. Escribe un acorde, cambia la
tonalidad, pasa la canción a otro compás, arrastra una sección: las
partes se vuelven a escribir, en el mismo paso de deshacer que tu
cambio, así que un Ctrl+Z deshace los dos. Todo cambio del proyecto pasa
por `LineupApp::edit` (ver [Deshacer y rehacer](../program/undo.md)),
así que ahí es donde se avisa a la banda:

```rust
let before = self.project.clone();
let result = change(&mut self.project);
// The band plays what the chord sheet says now, in the same step.
self.project.follow_chord_sheet();
```

Volver a escribir cada parte después de cada clic sería un
desperdicio: mover un clip de la melodía no cambia nada de lo que toca
la banda. Así que `follow_chord_sheet` primero toma una **huella** de
aquello a partir de lo cual se tocan las partes (el estilo, los
marcadores de acorde, las tonalidades, los compases, las secciones, la
forma) y la compara con la que tomó la última vez:

```rust
pub fn follow_chord_sheet(&mut self) -> bool {
    let Some(band) = &self.band else {
        return false;
    };
    if band.heard == Some(self.band_inputs()) {
        return false;
    }
    self.band_channels(false);
    self.write_band();
    true
}
```

Una huella, o hash, es un número calculado a partir de unos datos, de
manera que datos distintos dan casi seguro números distintos. Comparar
dos números de 64 bits es mucho más barato que comparar dos hojas de
acordes, y la banda solo tiene que guardar el número, no una copia de
la hoja.

## Una parte que editas sigue siendo tuya

Una banda que escribiera sus partes de cero cada vez perdería una
edición de la línea del bajo la próxima vez que cambiara algo. Aquí,
una parte que cambias a mano queda **congelada**: la banda la deja como
la dejaste, y la hoja lo dice, con un botón **Volver a tocarla** que
descarta tu versión y hace que la banda escriba la parte de nuevo a
partir de la hoja. Los demás músicos siguen siguiendo la hoja.

![La hoja de acordes con el aviso «You edited the piano: the band keeps it as you left it.» y un botón «Play it again»](../images/step-9-band-frozen.png)

¿Cómo sabe la banda que editaste una parte, con la herramienta que sea:
el piano roll, el channel rack, un clip arrastrado, la biblioteca? No
vigila las herramientas. Guarda, para cada músico, una huella de la
parte que escribió por última vez, y la compara con una huella de la
parte tal como está ahora:

```rust
pub fn is_frozen(&self, player: Player) -> bool {
    self.band.as_ref().is_some_and(|band| {
        band.written[player.index()].is_some_and(|written| written != self.part_print(player))
    })
}
```

Dos `is_some_and` anidados se leen como una sola frase: «hay una banda,
y escribió la parte de este músico, y eso no es lo que hay ahora».
`part_print` mezcla dónde empieza cada clip del músico, cuántas veces
se repite y cada nota de su patrón. En qué pista está un clip y qué
número tiene su patrón quedan fuera, así que bajar la pista Piano, o
eliminar un patrón tuyo (lo que renumera los de después), no congela
nada.

### FNV-1a

Las huellas se guardan en el archivo del proyecto, así que una parte
que congelaste sigue congelada cuando abres la canción mañana. El
`DefaultHasher` de la biblioteca estándar no promete dar el mismo número
en la próxima versión de Rust, así que no se puede guardar. La banda usa
**FNV-1a**, un hash clásico de unas pocas líneas:

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

    fn number(&mut self, number: usize) {
        self.bytes(&(number as u64).to_le_bytes());
    }
}
```

Para cada byte: mezclarlo con el número con XOR, y después multiplicar
por un primo grande, `0x100000001B3`. El número inicial y el primo son
las constantes publicadas del algoritmo, así que todo programa que
implementa FNV-1a obtiene el mismo resultado. Multiplicar dos `u64`
grandes desborda; en una compilación de depuración un `*` simple
detendría el programa con «attempt to multiply with overflow», y
`wrapping_mul` dice que quedarse solo con los 64 bits bajos es lo que
queremos, como hace un hash a propósito. `to_le_bytes` escribe los
bytes de un número empezando por el más bajo (*little-endian*), sea
cual sea el ordenador, así que la misma canción da la misma huella en
todas partes. Una parte sin ninguna nota es el número inicial mismo,
14695981039346656037: lo verás en un archivo guardado, para unas
cuerdas que esperan a la variación b.

### Mover la canción entera no es editar

Cambiar la tonalidad de la canción transpone cada nota, también las de
la banda. Las partes de la banda ya no coinciden con sus huellas, pero
tú no las editaste: la herramienta movió todo. Pasar la canción a otro
compás, copiar una sección e insertar un compás son lo mismo. Estas
herramientas se ejecutan dentro de `moving_the_band`:

```rust
pub(crate) fn moving_the_band<R>(&mut self, change: impl FnOnce(&mut Project) -> R) -> R {
    self.keeping_parts(|_, _| true, change)
}
```

```rust
fn keeping_parts<R>(
    &mut self,
    keep: impl Fn(&Project, Player) -> bool,
    change: impl FnOnce(&mut Project) -> R,
) -> R {
    let following: Vec<Player> = Player::ALL
        .into_iter()
        .filter(|&player| {
            self.band
                .as_ref()
                .is_some_and(|band| band.written[player.index()].is_some())
                && !self.is_frozen(player)
                && keep(self, player)
        })
        .collect();
    let result = change(self);
    let prints: Vec<(Player, u64)> = following
        .into_iter()
        .map(|player| (player, self.part_print(player)))
        .collect();
    if let Some(band) = &mut self.band {
        for (player, print) in prints {
            band.written[player.index()] = Some(print);
        }
    }
    result
}
```

Anota qué partes seguían la hoja antes, ejecuta la herramienta, y
después toma sus huellas nuevas como propias de la banda. Una parte
congelada se mueve con la herramienta como cualquier otra nota, y
sigue congelada: tu versión del piano, ahora en Re. `keep` es un
`impl Fn`, que se llama muchas veces, mientras que `change` es un
`impl FnOnce`, que se llama una vez. Las huellas se juntan en un `Vec`
antes de tomar `self.band` como préstamo mutable, porque `part_print`
también necesita `self`, y el borrow checker permite uno u otro a la
vez. Eliminar un canal usa el mismo método con otro `keep`: a una parte
que no tocaba nada en el canal eliminado solo se le corrieron los
números de canal, y eso no es una edición; una parte que perdió notas
queda congelada.

## Una banda sobre una canción con música propia

Abre una demostración, como el **Canon en Re**, y toma las partes de
acordes y bajo y las voces de un estilo de la Biblioteca. Una
demostración ya toca su propio acompañamiento, así que primero la banda:

1. escribe en la hoja los acordes que toca la canción, si está vacía
   (ver [De dónde vienen los
   acordes](chord-sheet.md#de-dónde-vienen-los-acordes)), y
2. **silencia** los canales de acordes y de bajo de la propia canción
   (la biblioteca los llama «Keys» y «Bass», que la interfaz en español
   muestra como Teclados y Bajo), un bajo propio de la canción tal como
   lo trae un archivo MIDI (un canal con nombre de bajo, o con un sonido
   de bajo de General MIDI: `plays_the_bass`) y, cuando también llega la
   batería del estilo, sus pistas que solo tienen batería, que un
   archivo importado deja en una pista propia,

para que las dos no toquen una encima de la otra. La melodía sigue
sonando, ahora con la banda detrás. Sin la batería del estilo, la
banda entra con `drums: BandDrums::Song`, y la batería propia de la
canción sigue sonando. Un estilo que sigue a la canción,
como la orquesta, entra junto a las partes de la canción en lugar de
silenciarlas (ver [Una orquesta para música que cambia de
compás](orchestra.md)).

![La hoja de acordes del Canon en Re después del preajuste Pop ballad: las filas de la banda (Form, Choruses, Ending, y You play Piano, Bass, Drums, Guitar, Strings) sobre D A Bm F#m | G D G A, la progresión del Canon de Pachelbel, escrita compás por compás](../images/step-9-band-demo.png)

**Sin banda** quita los patrones, las pistas y los canales de la banda
(solo los que añadió, y solo si nada más toca en ellos), y vuelve a
hacer sonar lo que silenció. Recuerda lo que silenció por su *nombre*,
no por su número, así que vuelven los canales correctos aunque hayas
añadido o quitado otros entretanto. Los acordes se quedan en la hoja.

## Una banda más grande: guitarra, cuerdas y órgano

Un piano, un bajo y una batería son un trío de jazz, y la sección
rítmica de la mayoría de las bandas, pero una banda de rock sin su
guitarra o un tango sin sus violines no suenan como ellos mismos. Así
que un estilo puede traer hasta tres músicos más, cada uno tocando como
lo usa esa música:

| Estilo | Quién entra | Qué toca |
|---|---|---|
| Ritmo pop | guitarra de acero, cuerdas en b | acordes rasgueados, cuerdas sostenidas |
| Ritmo de rock | guitarra saturada, órgano de rock en b | power chords en corcheas |
| Shuffle, Rock and roll | guitarra eléctrica limpia | el shuffle de boogie |
| Swing | guitarra de jazz | cuatro por compás |
| Funk, Disco | guitarra apagada, y cuerdas en el disco | semicorcheas, rascadas |
| One drop, Steppers, Rocksteady | guitarra limpia, órgano | el chop en el 2 y el 4, el burbujeo del órgano |
| Bossa nova | guitarra de nailon | el ritmo de bossa |
| Tangos | cuerdas | acordes sostenidos |
| Coral | órgano de iglesia | acordes sostenidos |
| Hip hop, la mayoría de los estilos electrónicos | nadie | |

«Cuerdas en b» quiere decir que esperan a la variación b: haz clic
derecho en un compás, elige **Variación b**, y desde ahí entran las
cuerdas, sosteniendo los acordes una octava por encima del piano, como
las cuerdas levantan un estribillo pop.

### La música: cómo toca un acorde un guitarrista

Un pianista puede tocar cualquier nota de un acorde: diez dedos, cada
tecla al alcance. Un guitarrista tiene seis cuerdas, afinadas desde la
más grave

| Cuerda | 6 | 5 | 4 | 3 | 2 | 1 |
|---|---|---|---|---|---|---|
| Nota al aire | Mi2 | La2 | Re3 | Sol3 | Si3 | Mi4 |
| MIDI | 40 | 45 | 50 | 55 | 59 | 64 |

y una mano izquierda con cuatro dedos para pisarlas contra los
**trastes**, un semitono cada uno. Cada cuerda toca una nota a la vez,
o ninguna. Una cuerda sin pisar suena **al aire**, gratis; por eso las
tonalidades de Mi, La, Re y Sol, cuyos acordes usan muchas cuerdas al
aire, son las tonalidades de la guitarra. Así que un acorde de guitarra
es una **forma**: un traste (o nada) para cada cuerda. Los libros de
acordes la escriben con seis caracteres desde la cuerda grave de Mi,
con `x` para una cuerda que no se toca y `0` para una al aire:

```text
C    x32010     la cuerda La en el traste 3 (Do), Re en el 2 (Mi), Sol al aire (Sol),
                Si en el 1 (Do), Mi aguda al aire (Mi): Do Mi Sol Do Mi
G    320003
Am   x02210
F    133211     el primer dedo plano sobre el traste 1: una "cejilla"
```

La cuerda más grave que suena tiene que ser el **bajo** del acorde, la
nota que toca el bajista (la fundamental, o en un acorde con barra la
nota después de la barra). Por eso la forma de Do deja fuera la cuerda
grave de Mi: un Mi allí la convertiría en C/E, que los libros de acordes
escriben `032010`.

Ferrous Lineup no guarda un libro de acordes. Encuentra las formas como
las aprende un guitarrista, probando los trastes al alcance y quedándose
con la más fácil. Para cada **posición** en el mástil (el traste donde
se apoya el primer dedo), prueba todas las maneras de poner una nota del
acorde en cada cuerda dentro de cuatro trastes, se queda con las que
una mano puede tocar y las puntúa:

- toda nota **necesaria** tiene que estar: la fundamental, la tercera (o
  la segunda o la cuarta de un acorde sus), la sexta o la séptima, una
  quinta que no sea justa, y la extensión más alta, que da nombre al
  acorde (la 13 de un G13);
- una quinta justa y las extensiones más bajas son **bienvenidas**, y las
  formas con más de ellas son mejores;
- las formas bajas en el mástil, con **cuerdas al aire** y en más
  cuerdas son mejores;
- hasta cuatro cuerdas pisadas está bien. Más necesitan una **cejilla**,
  un dedo plano sobre el traste más bajo, que pisa todas las cuerdas por
  encima de él, así que ninguna de ellas puede ir al aire, y deja tres
  dedos para el resto.

Una regla más mantiene claro el sonido. Dos notas juntas suenan
turbias en el grave: una tercera entre Mi2 y Sol♯2 retumba, mientras que
una quinta allí está bien. Los arreglistas tienen una tabla de la nota
más grave desde la que cada intervalo suena claro, los **límites de
intervalo en el grave**, y una forma que baja de ellos cuesta un poco
más. Un poco, no mucho: el Sol al aire, `320003`, empieza con Sol2 y
Si2, una tercera por debajo de su límite (Si♭2), y sigue siendo el Sol
que toca todo guitarrista. Su sonido es parte del sonido de la guitarra.

Con esas reglas, la búsqueda encuentra los acordes al aire de los
libros de acordes (C `x32010`, G `320003`, D `xx0232`, Am `x02210`,
G7 `320001`, Cmaj7 `x32000`), acordes con cejilla en otras tonalidades
(F `133211`, la forma de Mi al aire un traste más arriba, y B
`x24442`, la forma de La) y una forma para cada tipo de acorde en cada
tonalidad.

Un guitarrista no golpea las cuerdas todas juntas. La púa las barre: en
un **golpe hacia abajo** de las cuerdas graves a las agudas, en un
**golpe hacia arriba** de vuelta desde las agudas, y la mano que sube
rara vez llega a las graves. Los ritmos de la biblioteca ya marcan los
golpes ligeros «hacia arriba», entre los pulsos. La guitarra toca cada
golpe hacia abajo en todas las cuerdas, de grave a agudo, con un tick
entre una y otra (unos 5 ms a 120 BPM, así que un rasgueo de seis
cuerdas dura unos 25 ms, como uno de verdad), y cada golpe hacia arriba
en las cuatro agudas, de agudo a grave. Cómo el swing mueve un rasgueo
entero está en [Swing](../sound/tempo.md#swing).

Lo que toca el guitarrista de cada estilo:

- **Acordes rasgueados** (pop, folk, country): la forma completa en el
  ritmo del estilo.
- **Power chords** (rock, punk, metal): solo la fundamental, su quinta y
  la octava, `A5` = La2 Mi3 La3. La distorsión añade notas nuevas en las
  sumas y las diferencias de las frecuencias tocadas. Para una quinta,
  cuyas frecuencias están cerca de 3:2, caen alineadas con la
  fundamental, una o dos octavas por debajo; para una tercera caen en
  notas desafinadas respecto del acorde, que zumba y se agria. Así que
  los guitarristas de rock dejan la tercera al bajo y a quien canta, y
  «A5» quiere decir «La sin su tercera».
- **Cuatro por compás** (swing): acordes de tres notas, la fundamental,
  la tercera y la séptima, golpeados en cada pulso, el segundo y el
  cuarto un poco más fuerte, como Freddie Green marcó el tiempo en la
  banda de Count Basie durante casi cincuenta años. Las formas son los
  **shells** del jazz: desde una fundamental en la cuerda grave de Mi,
  la séptima viene después y la tercera una octava más arriba
  (G7 = Sol2 Fa3 Si3); desde la cuerda La, la tercera y después la
  séptima (C7 = Do3 Mi3 Si♭3).
- **El shuffle de boogie** (blues, rock and roll): solo dos cuerdas, la
  fundamental con su quinta en un pulso y con su sexta en el siguiente,
  las dos corcheas de cada pulso: La y Mi, La y Mi, La y Fa♯, La y Fa♯.
  Sobre un acorde menor se queda en la quinta.
- **El chop** (reggae, soul, Motown, bluegrass): un acorde corto en las
  cuatro cuerdas agudas en los pulsos 2 y 4, con la caja, dejando las
  graves al bajo. El country lo llama el «chick» del «boom-chick». En el
  **ska** la guitarra toca entre todos los pulsos.
- **Rascar** (funk, disco): semicorcheas en una guitarra apagada, algunas
  acentuadas, la mayoría solo un «chk» percusivo.

Las **cuerdas** sostienen cada acorde mientras dura, una octava por
encima del piano, suavemente: un colchón. Las baladas usan las cuerdas
de ataque lento, los estilos sintetizados las de sintetizador. El
**órgano** también sostiene acordes, como un órgano de iglesia en un
coral o un Hammond en un blues lento. En el reggae toca el
**burbujeo**: acordes cortos entre todos los pulsos en el grave, y en
los pulsos 2 y 4 una octava más arriba, con el chop de la guitarra.

Cada músico toca un instrumento General MIDI elegido por el estilo. La
tabla General MIDI los numera desde 1, y los mensajes MIDI desde 0, así
que el código dice 24 para la guitarra de nailon:

| Programa (desde 0) | Instrumento | Estilos |
|---|---|---|
| 24 | Guitarra acústica (nailon) | bossa nova, samba, bolero, folclore, rumba flamenca |
| 25 | Guitarra acústica (acero) | pop, folk, country, bluegrass, vals |
| 26 | Guitarra eléctrica (jazz) | swing |
| 27 | Guitarra eléctrica (limpia) | shuffle, rock and roll, surf, Motown, reggae, ska |
| 28 | Guitarra eléctrica (apagada) | funk, disco |
| 29, 30 | Guitarra saturada, con distorsión | rock, punk, metal |
| 48, 49, 50 | Cuerdas, cuerdas lentas, cuerdas sintetizadas | pop, tango, baladas, R&B, trance |
| 16, 17, 18, 19 | Órgano de tiradores, percusivo, de rock, de iglesia | blues, reggae, rock, coral |

### Músicos en arrays

La banda guarda un canal y una huella por músico, en arrays indexados
por el músico:

```rust
pub const COUNT: usize = 8;

pub const ALL: [Player; Player::COUNT] = [
    Player::Piano,
    Player::Bass,
    Player::Drums,
    Player::Guitar,
    Player::Strings,
    Player::Organ,
    Player::Soloist,
    Player::Horns,
];
```

```rust
pub(crate) channels: [Option<usize>; Player::COUNT],
```

`band.channels[player.index()]` responde «¿qué canal?» para cada
músico, e `index` es `self as usize`: un enum sin campos se convierte
en su posición. El largo del array es parte de su tipo,
`[Option<usize>; 8]`, así que el compilador comprueba que `ALL` liste
de verdad `COUNT` músicos. `write_band` recorre `Player::ALL`, y todo lo
que viene después, los patrones, las pistas y las huellas que mantienen
congelada una parte editada, funciona igual para cada músico.

### Quién toca en cada estilo: una tabla de constantes

Los músicos que añade cada estilo son un valor de este tipo:

```rust
pub struct Extra<Part> {
    pub part: Part,
    pub program: u8,
    pub only_in_b: bool,
}
```

```rust
pub struct Extras {
    pub guitar: Option<Extra<GuitarPart>>,
    pub strings: Option<Extra<PadPart>>,
    pub organ: Option<Extra<PadPart>>,
}
```

`Extra` es **genérico**: `Part` es un parámetro de tipo, que se completa
como `GuitarPart` para la guitarra y como `PadPart` para las cuerdas y
el órgano. Los tres campos «qué toca, en qué instrumento, espera a la
b» se escriben una sola vez para los dos. Los estilos se listan en una
tabla que construye el compilador:

```rust
const NONE: Extras = Extras {
    guitar: None,
    strings: None,
    organ: None,
};

const fn with_guitar(part: GuitarPart, program: u8) -> Extras {
    Extras {
        guitar: guitar(part, program),
        ..NONE
    }
}
```

```rust
const BY_STYLE: &[(&str, Extras)] = &[
    // Pop: a strummed acoustic guitar, strings for the chorus.
    (
        "Pop beat",
        Extras {
            guitar: guitar(STRUMMED, STEEL_GUITAR),
            strings: pad_in_b(STRINGS),
            ..NONE
        },
    ),
    // ...
];
```

Una `const fn` se puede llamar donde Rust necesita un valor en tiempo de
compilación, como en un `const`. `..NONE` es la sintaxis de
actualización de structs: los campos no escritos toman sus valores de
`NONE`, así que cada estilo nombra solo los músicos que tiene. La
prueba `every_style_listed_is_a_band_style` comprueba que cada nombre de
la tabla es un estilo de banda de verdad, para que una errata no deje a
un estilo sin su guitarra sin que nadie lo note.

### Enums que llevan datos, y una función como parámetro

Lo que toca la guitarra es un enum cuyas variantes pueden llevar un
valor:

```rust
pub enum GuitarPart {
    Strum(Comping),
    StylesRhythm,
    Chop,
    Power(Comping),
    FourToTheBar,
    Boogie,
}
```

`Strum(Comping::Bossa)` y `Strum(Comping::Waltz)` son una variante con
distintos ritmos dentro. Un `match` puede mirar dentro, con patrones `|`
anidados, como en la variación b, donde los acordes sostenidos y
punteados pasan a ser rasgueados:

```rust
pub fn in_variation(self, variation: Variation) -> GuitarPart {
    match (variation, self) {
        (Variation::B, GuitarPart::Power(Comping::Held)) => GuitarPart::Power(Comping::Pulse),
        (
            Variation::B,
            GuitarPart::Strum(Comping::Held | Comping::ArpUp | Comping::ArpUpDown),
        ) => GuitarPart::Strum(Comping::Strum),
        (_, part) => part,
    }
}
```

Los acordes rasgueados, los power chords y los shells solo se
diferencian en qué notas recibe cada armonía; tocar el ritmo y
rasguearlo es lo mismo. Así que `strummed` recibe la disposición como
parámetro:

```rust
fn strummed(
    rhythm: Comping,
    bars: &[Bar],
    harmonies: &[Played],
    voice: fn(&Played) -> Vec<Note>,
) -> Vec<NoteEvent> {
```

`fn(&Played) -> Vec<Note>` es un tipo de **puntero a función**:
cualquier función corriente con esa firma encaja, y quien llama pasa
`shape`, `power_chord`, `shell` o `top_strings` por su nombre. Encontrar
una forma prueba miles de combinaciones de trastes, y una canción repite
sus acordes, así que `strummed` recuerda las formas que encontró en un
`HashMap`:

```rust
let voicing = shapes.entry(key).or_insert_with(|| voice(harmony)).clone();
```

`entry` encuentra el lugar para `key` en el mapa, y `or_insert_with`
llama al closure solo si ese lugar está vacío: cada acorde distinto se
busca una sola vez.

### Probar cada traste: recursión con un callback

Cuántas cuerdas usa una forma no se sabe de antemano, así que la
búsqueda no puede ser un número fijo de bucles `for` anidados. Es una
función que elige un traste para una cuerda y se llama a sí misma para
la siguiente:

```rust
fn try_frets(
    choices: &[Vec<u8>],
    low: usize,
    top: usize,
    frets: &mut Vec<u8>,
    found: &mut impl FnMut(&[u8]),
) {
    let string = low + frets.len();
    if string > top {
        found(frets);
        return;
    }
    for &fret in &choices[string] {
        frets.push(fret);
        try_frets(choices, low, top, frets, found);
        frets.pop();
    }
}
```

`frets` es un único `Vec` compartido por todos los niveles: un nivel
añade el traste de su cuerda, deja que terminen los niveles más
profundos, y lo vuelve a sacar antes de probar el siguiente. Cuando
todas las cuerdas tienen un traste, `found` recibe la forma entera.
`found` es un closure que la puntúa y se queda con la mejor, así que
cambia algo de fuera de sí mismo: eso lo hace un `FnMut`, y se pasa como
`&mut` para que cada nivel de la recursión llame al mismo closure, en
lugar de que cada uno tome el suyo.

## Una sección de vientos

Los discos de soul y de funk tienen una. También las big bands, las
orquestas de salsa, las bandas de ska y las bandas de metales de los
Balcanes: trompetas, saxofones y trombones tocando los acordes juntos.
Marca **Sección de vientos** en el menú Banda, y entra con la banda una
pista **Vientos**, que toca la hoja de acordes con una parte que le va
al estilo:

| Estilo | Qué tocan los vientos |
|---|---|
| Baladas (Balada pop, Balada de jazz, Blues lento, Neo soul, R&B, Bossa nova, Bolero…) | notas largas sostenidas debajo de la banda |
| Swing, shuffle, blues, rock and roll | golpes de big band en el pulso 1 y justo antes del 3 (el ritmo charlestón) |
| Pop, rock, soul, hip hop, latino, second line | golpes cortos en 3 + 3 + 2 (el tresillo) |
| Funk | ataques secos de funk en semicorcheas |
| House y otros estilos electrónicos | ataques sincopados, como en la música house |
| Reggaeton, moombahton | acordes cortos en el 3 + 3 + 2 del dembow |
| Ska | acordes cortos entre los pulsos |
| Marcha, vals, polca, danzas balcánicas | contratiempos, el «pa» del «um-pa» de una banda de metales |
| Tango, folk, clásica, reggae, estilos sin batería | notas largas |
| Otros estilos en 3/4 o 6/8 | notas largas |
| Otros estilos en 5/4, 7/8 y otros compases irregulares | un golpe en cada pulso en que se siente el compás |

El ritmo **3 + 3 + 2** divide las ocho corcheas de un compás de 4/4 en
grupos de tres, tres y dos, con un acento al comienzo de cada uno: en
el pulso 1, en el «y» del 2 y en el pulso 4. Tocado así, el compás
empuja hacia adelante. Es el tresillo de la música cubana, y está en
todas partes en las líneas de vientos del pop y del soul. Los
**contratiempos** son los de una banda de metales: en una marcha en
2/4 la tuba toca «um» en el pulso y los vientos «pa» entre medio; en un
vals el bajo tiene el pulso 1 y los vientos los pulsos 2 y 3. El
instrumento es **Sección de metales** (General MIDI 61) en la mayoría
de los estilos, **Corno francés** en los clásicos y **Metales
sintetizados** en los electrónicos.

![El menú Band con Horn section marcado, y su pista: «A horn section plays the chords in close harmony, each horn moving as little as it can from chord to chord. In Motown, the horns (Brass Section) play short punches in 3 + 3 + 2.»](../images/step-9-horns-menu.png)

### Armonía cerrada, moviéndose lo menos posible

Los vientos que tocan acordes juntos los disponen en **posición
cerrada**: las notas del acorde tan juntas como se pueda, dentro de una
octava, debajo del viento más agudo, la **primera voz** (*lead*). Los
arreglistas escriben así las partes de vientos porque hace que tres o
cuatro instrumentos suenen como uno grande. Los vientos tocan las notas
que toca el piano en el acorde, en la octava que sea: tantos vientos
como notas distintas tiene la disposición del piano, hasta cuatro. Un
acorde de más de cuatro notas, como C13 (Do Mi Sol Si♭ Re La), deja
fuera primero su quinta, y después su fundamental, que el bajo toca de
todos modos: los vientos tocan Mi, Si♭, Re, La, lo que hace que el
acorde suene como suena. La primera voz se queda entre Fa4 y Fa5, donde
una sección de trompetas suena brillante sin forzar.

De un acorde al siguiente, los vientos toman la disposición cerrada que
menos se mueve: cada uno va a la nota más cercana que puede. Do a La♭ es
Mi4 Sol4 Do5 a Mi♭4 La♭4 Do5: dos vientos se mueven un semitono, abajo
y arriba, y la primera voz sostiene el Do. La♭ a G7: la primera voz baja
de Do5 a Si4, la segunda de La♭4 a Sol4, la tercera sube de Mi♭4 a Fa4,
y una cuarta entra en Re4 para el acorde de séptima. Después, de G7 a
Do, Si4 sube a Do5 y Fa4 baja a Mi4, como hacen la sensible y la
séptima de una dominante.

![La parte de Horns del ejemplo de Motown en el piano roll con Voice leading activado. De Ab a G7, la línea de movimientos dice «Horns: top C5→B4 · 2nd Ab4→G4 · 3rd Eb4→F4; outer parts in contrary motion». Las líneas de colores unen las notas de cada viento de un acorde al siguiente.](../images/step-9-horns.png)

Los vientos también evitan las **quintas paralelas** cuando pueden. Dos
vientos a una quinta justa que pasan a otra quinta justa suenan como un
solo instrumento, y la textura pierde una línea (ver [Conducción de
voces](../theory/voice-leading.md)). Así que cada par de vientos que
se movería en quintas paralelas cuenta como 24 semitonos de movimiento,
más que cualquier otro camino al acorde siguiente, y los vientos eligen
otra disposición.

### En el código

Lo que tocan los vientos son notas largas, o acordes cortos en el ritmo
de una de las maneras de hacer comping del piano:

```rust
pub enum HornPart {
    /// Long notes: each chord held for as long as it lasts.
    Pads,
    /// Short chords in the rhythm of a way of comping.
    Hits(Comping),
}
```

`Comping` ya tenía todos los ritmos que necesitan los vientos
(Charleston, Tresillo, Funk, Stabs, Dembow, Offbeat, Aksak para los
contratiempos y Accents), así que los vientos los reutilizan.
`HornPart::of` lee el estilo: primero dos listas de nombres de estilos,
después el compás, después el género:

```rust
if style.rhythm.is_none() || HELD.contains(&style.name) {
    return HornPart::Pads;
}
if let Some(&(_, comping)) = BY_STYLE.iter().find(|(name, _)| *name == style.name) {
    return HornPart::Hits(comping);
}
```

`iter().find` devuelve un `Option<&(&str, Comping)>`, y el patrón
`Some(&(_, comping))` desarma el par de una vez: `&` para la referencia,
`_` para el nombre que no necesita. `close` construye una disposición
de arriba abajo, empezando en la primera voz y tomando la nota más
cercana por debajo que sea una nota del acorde que ningún viento toque
todavía:

```rust
fn close(lead: u8, classes: &[u8]) -> Vec<u8> {
    let mut notes = vec![lead];
    let mut above = lead;
    while notes.len() < classes.len() {
        let Some(below) = (1..12)
            .filter_map(|down| above.checked_sub(down))
            .find(|note| {
                classes.contains(&(note % 12)) && notes.iter().all(|other| other % 12 != note % 12)
            })
        else {
            break;
        };
        notes.push(below);
        above = below;
    }
    notes.reverse();
    notes
}
```

`checked_sub` da `None` en lugar de dar la vuelta por debajo de la nota
0, y `filter_map` los descarta. `choose` prueba cada primera voz de Fa4
a Fa5 que sea nota del acorde, construye la disposición cerrada debajo
y se queda con la más barata:

```rust
fn choose(classes: &[u8], last: Option<&[u8]>) -> Vec<u8> {
    (LEAD_LOW..=LEAD_HIGH)
        .filter(|lead| classes.contains(&(lead % 12)))
        .map(|lead| close(lead, classes))
        .min_by_key(|voicing| cost(voicing, last))
        .unwrap_or_default()
}
```

`cost` suma cuánto se mueve cada viento, emparejándolos de arriba abajo,
más 24 por cada par en quintas paralelas. `min_by_key` devuelve la
primera de las más baratas, y `unwrap_or_default` da una disposición
vacía, silencio, para una armonía sin notas. Se prueban como mucho
trece primeras voces, cada una con cuatro vientos como mucho, así que
probarlas todas es rápido; no hace falta nada más ingenioso.

## Seguir los acordes que mantienes

Un **teclado arreglador**, el teclado doméstico con un botón de
«estilo», toca batería, acordes y una línea de bajo, y la mano izquierda
solo le dice *qué acorde*. Mantén Do y la banda toca en Do; pasa a La
menor y el mismo groove sigue en La menor. **MIDI > El acompañamiento
sigue los acordes** hace eso con cualquier canción: tócala, mantén un
acorde en un teclado MIDI (o en las teclas de letras, con **Teclado del
equipo como piano**), y desde la siguiente nota que toquen, los acordes
y la línea de bajo pasan a él. La barra de estado dice en qué acorde
está el acompañamiento.

![Manteniendo La y Do: el acompañamiento toca La menor](../images/step-8-follow-status.png)

Como los teclados arregladores, no necesita el acorde entero:

| Teclas pulsadas | Acorde |
|-----------------|--------|
| una tecla, como Sol | su acorde mayor: Sol |
| dos teclas a una tercera menor, como Re y Fa | el acorde menor sobre la de abajo: Re menor |
| dos teclas a una tercera mayor o a una quinta | el acorde mayor sobre la de abajo |
| dos teclas a una séptima menor, como Sol y Fa | el acorde de séptima: G7 |
| tres teclas o más | el acorde que forman |

El último acorde se queda cuando sueltas, como hace el botón de
«memoria» de un teclado arreglador. Solo una tecla que *baja* pide un
acorde nuevo, así que soltar las teclas una a una no cuenta como tocar
las que quedan.

Antes de poder mover una nota, la aplicación necesita saber para qué
acorde se escribió. Oye cada patrón una vez, con el segundo oído de
[Oír la
armonía](../theory/hearing.md#un-segundo-oído-para-los-patrones-de-la-propia-banda),
y guarda lo que oyó hasta que cambia el proyecto. Después cada nota de
un tramo oído como, por ejemplo, Do mayor se mueve al acorde
mantenido, por ejemplo La menor. Los teclados arregladores tienen una
tabla para esto, escrita por quien hizo el estilo; Ferrous Lineup lo
calcula a partir de lo que *hace* cada nota en su acorde, su función:

```rust
enum Role {
    Root,
    /// The third, or what stands for it in a "sus" chord.
    Third,
    Fifth,
    /// The seventh, or the sixth of a sixth chord.
    Seventh,
    /// A ninth, eleventh or thirteenth.
    Tension,
}
```

Los acordes en bloque, tres notas o más que empiezan juntas, se
disponen de nuevo para el acorde nuevo tan cerca como se pueda de donde
estaban, así que la mano casi no se mueve. Las notas sueltas, como en
los arpegios y las líneas de bajo, conservan su función: la fundamental
de Do va a la fundamental de La menor, la tercera (Mi) a la tercera
(Do), la quinta (Sol) a la quinta (Mi); una séptima va a la séptima
nueva, o a la fundamental si el acorde nuevo no tiene. Las notas fuera
del acorde, como las notas de paso de un bajo caminante, se mueven
tanto como la fundamental. Cada nota movida cae en la octava más cercana
a donde estaba, lo que mantiene grave una línea de bajo y con la misma
forma un arpegio:

```rust
fn nearest(pitch_class: u8, near: Note) -> Note {
    let near = i32::from(near.midi());
    let up = (i32::from(pitch_class) - near).rem_euclid(12);
    // Up to a tritone up, or else down.
    let midi = if up <= 6 { near + up } else { near + up - 12 };
    Note::new(midi.clamp(0, 127) as u8)
}
```

`rem_euclid` es el resto que nunca es negativo: `-3 % 12` es `-3` en
Rust, pero `(-3i32).rem_euclid(12)` es `9`, que es lo que quiere decir
«cuánto hay que subir hasta el próximo La».

La canción misma nunca cambia: deshacer, guardar y exportar a WAV ven
las notas como están escritas. Las notas movidas van solo al hilo de
audio, con el comando que ya usa cada edición para reemplazar las notas
de un canal en un patrón (ver [Enviar comandos con un ring
buffer](../program/threads.md#enviar-comandos-con-un-ring-buffer)):

```rust
self.send(Command::SetNotes {
    pattern: moved.pattern,
    channel: moved.channel,
    notes: swing::swung(&moved.notes, swing, hits),
});
```

El hilo de audio cambia la lista entre dos bloques de sonido, así que la
siguiente nota que empieza ya está en el acorde nuevo; las notas que ya
suenan siguen como estaban. Cuando se detiene la reproducción o se
desactiva la opción, se vuelven a enviar las notas propias del
proyecto. «Solo una tecla que baja» es una bandera, que se pone en cada
Note On y se lee con `std::mem::take`, que la devuelve y deja `false` en
su lugar:

```rust
if !std::mem::take(&mut self.live.follow.key_went_down) {
    return;
}
```

## En el archivo

Un proyecto con banda guarda su estilo por nombre, dónde termina, sus
canales, las huellas y lo que silenció. Una canción nueva de **Archivo >
Nuevo** con C, Am, F y G7 en su hoja guarda esto:

```ron
band: Some((
        style: "Pop beat",
        end: 64,
        piano: Some(0),
        bass: Some(1),
        guitar: Some(2),
        strings: Some(3),
        written: (Some(14171519900600248065), Some(11668178586522433030), Some(3477506964485452990)),
        written_too: (Some(13661597678858777105), Some(14695981039346656037), None),
        heard: Some(6239919094066095261),
        muted: (),
        since: 0,
    )),
```

y cada patrón de la banda lleva `player: Some(Piano)` y así. Las huellas
están repartidas de manera rara: `written` tiene tres, `written_too`
tres más. La banda en memoria tiene un único array de ocho, pero el
archivo tiene que seguir pudiendo leerse con versiones anteriores donde
se pueda. Una banda de piano, bajo y batería se guardaba como
`written: (…, …, …)` antes de que hubiera otros músicos, así que esas
tres siguen donde estaban, y las demás van en campos propios que se
omiten cuando están vacíos:

```rust
/// The fingerprints of the piano, the bass and the drums.
pub written: [Option<u64>; 3],
/// The fingerprints of the guitar, the strings and the organ, left
/// out when there are none, so a band of three reads as before.
#[serde(default, skip_serializing_if = "none_written")]
pub written_too: [Option<u64>; 3],
/// The fingerprint of the soloist's part.
#[serde(default, skip_serializing_if = "Option::is_none")]
pub written_solo: Option<u64>,
```

`skip_serializing_if` deja un campo fuera del archivo cuando la función
nombrada lo dice, y `default` lo completa cuando un archivo no lo tiene.
Convertir el array en los campos del archivo es un solo patrón:

```rust
let [piano, bass, drums, guitar, strings, organ, soloist, horns] = band.written;
```

Un patrón de array desarma un array en nombres, uno por elemento, y el
compilador comprueba que sean exactamente ocho. Así una banda de tres se
escribe exactamente como antes, y su canción conserva el formato que
necesitaba; una banda necesita el formato 13, una banda más grande el
17, un solista el 19, una sección de vientos el 28 y la batería de otro
estilo el 29, guardada como `drums: Some(Of("Bluegrass"))` o `drums:
Some(Song)` (ver [El número de
versión](../program/saving.md#el-número-de-versión)), y una versión
anterior los rechaza con un mensaje en lugar de perder un músico sin
decir nada. Un archivo cuya banda toca un estilo que esta versión no
conoce, o en un canal que no existe, se rechaza con el motivo en lugar
de abrirse mal.

## Cómo se prueba

`band.rs` tiene las pruebas más amplias de la banda:
`a_new_song_is_an_empty_sheet_with_a_band`,
`the_band_plays_what_is_typed`,
`the_band_follows_every_change_of_the_sheet`,
`an_edited_part_stays_until_it_is_played_again`,
`a_frozen_part_stays_frozen_in_the_file`,
`sets_micromodes_and_rows_are_played_by_their_notes`,
`no_chord_silences_every_style`,
`a_style_in_three_puts_the_song_in_three`,
`a_walking_bass_leads_into_each_chord_by_a_semitone` (en Swing y en un
vals de jazz),
`a_band_in_a_demo_mutes_the_songs_own_accompaniment`,
`deleting_a_players_channel_freezes_only_that_part`,
`long_songs_are_cut_into_patterns_and_repeats_share_one`, y para las
partes del estilo `instruments_can_change_without_changing_the_performance`,
`the_progression_is_independent_of_the_rhythm_and_sounds` y
`separate_style_components_survive_saving_and_undo`.
`every_rhythm_is_a_band_style_and_names_are_unique` lee la lista
entera. En `comping.rs`, `a_walking_line_leads_into_the_next_chord`
comprueba Do2, Mi2, Sol2, Fa♯2 sobre C7 yendo a F,
`a_walking_line_goes_the_way_of_the_next_chord` las cuatro líneas de
C, Am, F, G7 y que, de cada acorde a cada fundamental, el último paso
sea de medio tono y ningún paso más ancho que una sexta, y
`plain_accents_follow_the_grouping_without_a_styled_bass_line` los
pulsos de un compás irregular.

Las pruebas de la guitarra comprueban la música:
`open_chords_are_the_ones_in_chord_books`,
`other_keys_take_barre_chords`, `a_slash_chord_has_its_bass_lowest`,
`every_chord_in_every_key_has_a_shape_with_its_needed_notes`,
`power_chords_and_shells`,
`a_down_stroke_goes_up_the_strings_and_an_up_stroke_comes_down`,
`a_chop_strikes_the_top_four_strings_on_two_and_four` y
`the_boogie_alternates_fifth_and_sixth`. Las de los vientos:
`the_horns_move_as_little_as_they_can` (Do, La♭, G7, Do como más
arriba), `the_horns_never_move_in_parallel_fifths_when_they_can_help_it`,
`a_big_chord_leaves_out_its_fifth_and_root` y
`every_style_has_horns_that_fit_it`; y en `band.rs`,
`horns_come_to_any_style_when_asked` los lleva a seis estilos y
comprueba que cada nota de los vientos es una nota de su acorde, que
cada disposición está en posición cerrada y que la primera voz está
entre Fa4 y Fa5.

Seguir un acorde mantenido se comprueba con `keys_ask_for_chords`,
`notes_keep_their_role`, `a_seventh_goes_to_the_root_of_a_triad`,
`every_style_follows_any_chord_in_its_chords` (para cada estilo, el
canal de acordes toca solo notas del acorde mantenido) y
`every_demo_song_follows_a_chord`. Y la [traza de
reproducción](../sound/trace.md) de una canción con banda lista cada
nota que toca cada músico, compás por compás, que es como se oye un
cambio en la banda sin escuchar.

Los ejercicios sobre la banda están en [Ejercicios: la
banda](exercises.md).
