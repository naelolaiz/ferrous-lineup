# Canciones de swing y big band, con una sección de vientos

Archivos: [`crates/lineup-core/src/library/song/jazz.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/jazz.rs),
[`crates/lineup-core/src/library/song/blues.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/blues.rs),
[`crates/lineup-core/src/library/song.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song.rs)
y [`crates/lineup-core/src/library/demo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/demo.rs)

El menú **File > Demos** tenía diecisiete canciones de jazz, la mayoría
ragtime, y varias tocaban solo parte de su pieza: un strain de The
Entertainer, el estribillo de Bill Bailey sin su final propio. Este
cambio añade **33 canciones de swing, big band y jazz temprano**, cada
una tocada completa, y completa doce de las demos que ya había. Las
canciones de big band reciben además algo que ninguna demo tenía antes:
una **sección de vientos** detrás de la melodía.

## Probarlo

Abre **File > Demos > Songs > Jazz > Tea for Two** (o escribe «tea» en
el cuadro de búsqueda de la parte de arriba del menú Demos). Además de
Lead, Keys y Bass, el channel rack tiene ahora un cuarto canal,
**Horns**. Con **Edit > Settings… > Audio > SoundFont instruments**
activado, la melodía es una trompeta y el canal Horns son los saxofones,
sosteniendo cada acorde mientras dura. Silencia Horns (haz clic en su
luz del rack) y toca otra vez: la banda suena como un grupo pequeño.
Actívalo otra vez, y es una big band.

Abre ahora **Oh, Lady Be Good!**. Aquí el saxofón alto lleva la melodía,
y los vientos son los metales, golpeando cada acorde en el pulso 1 y
justo antes del pulso 3. Y luego **Embraceable You**, una balada: un
saxofón tenor sobre trombones.

Las canciones nuevas:

| Canciones | De |
|---|---|
| Tea for Two, Sometimes I'm Happy, Great Day, Without a Song | Vincent Youmans, 1924–1929 |
| Oh, Lady Be Good!, Fascinating Rhythm, 'S Wonderful, Strike Up the Band, Liza, Embraceable You, But Not for Me | George Gershwin, 1924–1930 |
| My Blue Heaven, Makin' Whoopee, Love Me or Leave Me, Carolina in the Morning | Walter Donaldson, 1922–1928 |
| Honeysuckle Rose | Fats Waller, 1929 |
| Charleston, If I Could Be with You | James P. Johnson, 1923–1926 |
| King Porter Stomp | Jelly Roll Morton, 1924 |
| Limehouse Blues, Japanese Sandman, Indiana, Rose Room | 1917–1922 |
| Sweet Georgia Brown, Ain't Misbehavin', It Had to Be You, Tiger Rag, After You've Gone, St. Louis Blues | 1914–1929 |
| In the Mood, Sing, Sing, Sing, Take the "A" Train, Hit the Road Jack | 1936–1960 |

Completadas: The Entertainer, Maple Leaf Rag, Elite Syncopations y Pine
Apple Rag (todos los strains, con el trío en su propia tonalidad),
Twelfth Street Rag y Dill Pickles (sus strains posteriores), Golliwogg's
Cakewalk (la pieza entera, con la sección central en Sol bemol, que la
app llama Fa sostenido mayor: las mismas teclas del piano), I Got Rhythm
(su estrofa), Bill Bailey, Swanee y Avalon (cada estribillo dos veces,
con el primer y el último final de la partitura), y los acordes de
Ballin' the Jack.

## La música

### Una big band

Una banda de swing de los años 30 tenía cuatro **secciones**: unos
cinco saxofones, cuatro trompetas, cuatro trombones y la **sección
rítmica** (piano, guitarra, bajo y batería). El arreglista rara vez deja
que todos toquen la melodía. Una sección la lleva, y otra toca un
**background** detrás. Dos backgrounds están por todas partes:

- **Pads**: los saxofones sostienen los acordes, suavemente, bajo la
  melodía de una trompeta. Un pad cambia cuando cambia el acorde y no
  hace nada más, así que oyes moverse la armonía sin nada nuevo que
  seguir.
- **Punches** (o «hits»): los metales tocan acordes cortos en el pulso y
  justo antes del pulso 3, el mismo ritmo de Charleston que compea el
  piano. Los punches empujan la banda hacia delante; son el «swing» de
  un arreglo de swing tanto como el ride.

Eso es lo que hacen las tres bandas nuevas de las demos: **BIG_BAND** es
una trompeta sobre pads de saxofones, **SAX_SECTION** un saxofón alto
sobre punches de metales, y **BALLAD_BAND** un saxofón tenor sobre pads
de trombones.

### Estrofa y estribillo

La mayoría de estas canciones se escribieron para espectáculos de
Broadway o para partituras impresas, y tienen dos partes: una **estrofa**
(verse), que prepara la escena, y un **estribillo** (refrain, o chorus),
la parte que todo el mundo conoce. Las bandas conservaban el estribillo
y soltaban la estrofa, y muchos lead sheets hacen lo mismo. Donde se
pudo encontrar una estrofa escrita, la demo la toca una vez (I Got
Rhythm, Liza, It Had to Be You, If I Could Be with You); luego el
estribillo dos veces, como una banda toca de nuevo el «head» después de
un chorus de solos. Nada se rellenó de memoria, así que las canciones
cuya estrofa no tiene una fuente escrita que pudiéramos leer empiezan en
el estribillo.

### Treinta y dos compases, dos formas

Casi todo estribillo tiene **32 compases**, cuatro frases de ocho.
Vienen en dos formas:

- **AABA**: una frase, la misma frase otra vez, un **puente**
  contrastante y la frase una última vez. Oh, Lady Be Good!, Honeysuckle
  Rose, Ain't Misbehavin', Take the "A" Train, Sing, Sing, Sing e I Got
  Rhythm son AABA. Mira el playlist de una: la sección A es un patrón,
  usado tres veces, y el final de la segunda La es un patrón propio.
- **ABAC**: dos mitades que empiezan igual y acaban distinto, la primera
  mitad abierta, la segunda cerrando en la tónica. Indiana, Rose Room,
  Embraceable You, But Not for Me y Carolina in the Morning son ABAC.

Aquí hay también dos formas más antiguas. **El blues**, doce compases,
es el strain de riff de In the Mood y dos de los tres strains de St.
Louis Blues. **El rag**, de las marchas: varios strains de 16 compases,
cada uno repetido, en el orden La La Si Si La Do Do Re Re, con los strains Do y Re,
el **trío**, en otra tonalidad.

### Cambios de tonalidad que puedes ver

Cada cambio de tonalidad de estas partituras es un marcador de tonalidad
en la regla (mira [Cambios de tonalidad](../step-8/key-changes.md)), así
que puedes ver moverse las harmony keys:

- **Tea for Two** está en La bemol, y su segunda frase es la primera una
  tercera mayor más aguda, en Do. El último acorde de esa frase, Mi bemol
  7, es el V7 de La bemol, y lleva directamente a casa.
- **'S Wonderful** pasa la primera mitad de su puente en Sol, una tercera
  mayor por encima de su Mi bemol; la segunda mitad vuelve por Sol7, Do7, F7
  y Si bemol 7, una cadena de dominantes.
- **Los rags y King Porter Stomp** se mueven a la **subdominante**, la
  tonalidad una cuarta más aguda (una quinta más grave), para el trío:
  The Entertainer de Do a Fa, Maple Leaf Rag y King Porter Stomp de La
  bemol a Re bemol, Elite Syncopations y Pine Apple Rag de Fa a Si bemol.
  Esto viene de la marcha, donde el trío es la parte más suave y
  cantada; un rag acaba a menudo en esa tonalidad nueva.
- **Tiger Rag** sube dos veces: de Si bemol a Mi bemol, y luego a La bemol
  para el strain de «Hold that tiger», cada vez una cuarta arriba.
- **St. Louis Blues** conserva su tónica y cambia de modo: el strain
  central está en Sol menor, sobre un ritmo de habanera, entre blues en Sol
  mayor.

### Cadenas de dominantes y otros giros

Escucha esto:

- **Sweet Georgia Brown** abre con Re7, Sol7 y Do7, cuatro compases cada
  uno, antes del primer acorde de Fa, en su compás trece: cada acorde es
  el V del siguiente, así que la canción está en casa solo cuando llega
  a Fa.
- El puente de **I Got Rhythm** (Re7, Sol7, Do7, F7, dos compases cada uno)
  es la misma idea, de dos compases en dos compases, y su sección A
  gira I-vi-ii-V. Cientos de melodías se escribieron sobre estos
  «rhythm changes».
- **Hit the Road Jack** son cuatro acordes una y otra vez, i, bVII, bVI,
  V7, bajando paso a paso en el bajo: la **cadencia andaluza** (está en
  la Library con ese nombre).
- **Love Me or Leave Me** está en menor, con la sexta elevada: la
  melodía se apoya en Fa sostenido, y el lead sheet escribe su acorde de
  casa como La menor 6 (La, Do, Mi, Fa sostenido). Así que la tonalidad de la
  demo es La **dórico**, la escala menor con esa sexta, en lugar de menor
  natural.

## El código

### Una sección de vientos en la banda

El `Band` de una canción eran tres instrumentos de General MIDI, uno por
cada canal de una demo. Gana un cuarto campo:

```rust
pub struct Band {
    pub lead: u8,
    pub keys: u8,
    pub bass: u8,
    pub horns: Option<Horns>,
}

pub struct Horns {
    pub program: u8,
    pub comping: Comping,
}
```

`Option` dice que la sección de vientos puede faltar: toda canción
antigua conserva `horns: None` y suena como antes. Una sección de
vientos es un instrumento y un [`Comping`](../step-8/new-rhythm.md), los
mismos patrones de ritmo que usa el piano: `Comping::Held` para pads,
`Comping::Charleston` para punches.

Las bandas son valores `const`, construidos en tiempo de compilación.
`Band::new` fija `horns: None`, y un segundo `const fn` añade una
sección:

```rust
const fn with_horns(self, program: u8, comping: Comping) -> Band {
    Band {
        horns: Some(Horns { program, comping }),
        ..self
    }
}

const BIG_BAND: Band = Band::new(gm::TRUMPET, gm::ACOUSTIC_GRAND_PIANO, gm::ACOUSTIC_BASS)
    .with_horns(gm::TENOR_SAX, Comping::Held);
```

`..self` es **struct update syntax**: «todos los demás campos como en
`self`». Un `const fn` se puede llamar donde Rust necesita un valor en
tiempo de compilación, como en el lado derecho de un `const`, que es por
lo que el estilo builder funciona aquí.

### Escribir la parte de los vientos

`song_demo` ya convertía el chart de acordes de cada parte en las notas
de Keys y Bass con `accompany(spans, tonic, comping, bass)`. Los vientos
son una llamada más a la misma función, con el comping de los vientos y
sin línea de bajo:

```rust
let mut horn_notes = match horns {
    Some((_, horns)) if part.left.is_empty() => {
        accompany(&part.spans(bar), tonic, horns.comping, Bass::None).chords
    }
    _ => Vec::new(),
};
```

El **match guard** `if part.left.is_empty()` se salta las partes cuyo
acompañamiento está escrito nota por nota (la mano izquierda de un rag),
que no tienen chart que tocar para los vientos. Como `accompany` dispone
los voicings de los acordes igual en las dos llamadas, los vientos tocan
el voicing del piano, que se queda alrededor de Mi bemol por encima del Do
central: el registro de los saxofones y los trombones, debajo de la
melodía.

La prueba `a_big_band_has_its_horns` abre cada canción con sección de
vientos y comprueba que su demo tiene un canal Horns con el instrumento
correcto, con notas en cada parte que tiene chart.

### Cómo se escribieron las canciones

Cada canción tiene un comentario `// Source:` con la transcripción
escrita de la que vienen sus notas. La mayoría son **lead sheets** en
MusicXML de OpenEWLD, la parte de dominio público del archivo Wikifonia,
convertidos a la notación de esta app por un conversor pequeño; los
acordes se comprobaron contra los charts de acordes que usan los músicos
de jazz, y cuando el acorde de un lead sheet chocaba con su propia
melodía, se usó el acorde más común, con una nota en el comentario. Los
tresillos se convirtieron en las semicorcheas más cercanas, de la manera
en que el [lead sheet](lead-sheet.md) los redondea.

### Derechos de autor

La mayoría de estas canciones se publicaron en 1930 o antes, por
compositores que murieron antes de 1956: están en el dominio público en
todas partes. Seis son libres en Estados Unidos (publicadas antes de
1931) pero siguen protegidas en los países donde el copyright dura 70
años tras la muerte del autor (Sweet Georgia Brown, Ain't Misbehavin',
It Had to Be You, Tiger Rag, After You've Gone, St. Louis Blues), y
cuatro siguen bajo copyright en todas partes (In the Mood, Sing, Sing,
Sing, Take the "A" Train, Hit the Road Jack). Se incluyen a petición
del dueño, y un comentario encima de cada una dice de qué caso se trata.

## Ejercicios

1. Abre **Tea for Two** y encuentra el marcador de tonalidad donde
   empieza la segunda frase. ¿Qué acorde de La bemol es Do mayor? (Pista:
   cuenta desde La bemol: Do es la tercera, así que Do mayor es III, un
   acorde que la tonalidad de La bemol no contiene.)
2. En **Oh, Lady Be Good!**, silencia el canal Lead y toca encima en el
   teclado del piano roll. ¿Qué compases de la sección A cambian de
   acorde dos veces?
3. Da a **Rose Room** una sección de vientos: cambia su `band:` de
   `SAX_SECTION` a `BIG_BAND` y ejecútala. ¿Qué cambian los pads en cómo
   suena el puente?
4. Escribe una banda propia en `song.rs`,
   `Band::new(gm::CLARINET, ...)` con
   `.with_horns(gm::BRASS_SECTION, Comping::Backbeat)`, y dásela a
   **Charleston**. ¿Le sienta bien un backbeat a una canción de dos
   pulsos?
