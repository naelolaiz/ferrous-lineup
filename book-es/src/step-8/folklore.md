# Folclore sudamericano, y muchas más canciones

La segunda tanda de demos hace dos cosas. Añade un género que faltaba,
el folclore de Sudamérica, con sus propios patrones de batería y estilos
de armonía, y más que duplica el número de canciones, de 62 a 130, cada
una nueva transcrita de una fuente escrita en lugar de memoria. De paso,
una demo aprendió a mostrar, en la pestaña Library, qué estilo y qué
ritmo está tocando. Esta página trata de lo que tuvo que cambiar para
hacerle sitio a todo eso, y de lo que se aprendió por el camino.

## Un género nuevo

`Genre` es un enum, y toda la biblioteca cuelga de él: los menús Styles,
Rhythms y Songs tienen un submenú por género, en el orden de
`Genre::ALL`. Añadir `Folklore` entre `Tango` y `Flamenco` tocó tres
lugares:

```rust
pub enum Genre {
    ...
    Tango,
    Folklore,
    Flamenco,
    ...
}

pub const ALL: [Genre; 14] = [ ... Genre::Tango, Genre::Folklore, Genre::Flamenco, ... ];

pub fn name(self) -> &'static str {
    match self {
        ...
        Genre::Folklore => "South American folklore",
        ...
    }
}
```

La longitud del array forma parte de su tipo, `[Genre; 14]`, así que el
compilador rechaza el antiguo `13` en cuanto se añade una variante, y el
`match` debe cubrir todas las variantes, así que `name` tampoco puede
olvidarse. Eso es el enum haciendo su trabajo: no hay forma de añadir un
género y dejar los menús a medio construir.

Luego las pruebas pidieron el resto.
`every_genre_has_songs_and_they_are_in_order` fallaba hasta que el
género nuevo tuvo al menos dos canciones, y como `SONGS` debe estar
ordenada en el orden del enum, las canciones de folclore tuvieron que ir
entre los tangos y el flamenco. La posición de una variante en el enum
es su posición en todas partes.

## Dos contra tres, sin marcadores de tiempo

Casi toda esta música está en 6/8 y no deja de cruzar a 3/4. La guitarra
de una chacarera rasguea dos pulsos de negra con puntillo por compás
mientras el bombo toca tres negras; el acordeón de un chamamé está en
6/8 sobre un bajo en 3/4; las maracas y el arpa del joropo discrepan a
propósito. Es la *hemiola* (o *sesquiáltera*), y es todo el sabor.

Al principio parecía un trabajo para los [marcadores de
tiempo](time-markers.md), pero no lo es. Tanto 6/8 como 3/4 duran doce
pasos:

```rust
pub fn steps_per_bar(self) -> usize {
    (usize::from(self.beats) * 16 / unit).max(1)
}
```

6 × 16 / 8 = 12, y 3 × 16 / 4 = 12. Un compás de doce pasos puede oírse
como dos grupos de seis o tres grupos de cuatro, y las líneas de batería
pueden hacer ambas cosas a la vez:

```rust
rhythm(
    "Chacarera",
    Folklore,
    132.0,
    &[
        (LowTom, "X..... x.x..."),
        (Rim, "....x. ..x.x."),
    ],
    "",
    "...",
)
.in_time(SIX_EIGHT),
```

El tom grave es el parche del bombo, en los dos pulsos de 6/8; el rim es
el palo sobre su aro de madera, en el segundo y el tercer pulso del 3/4
(pasos 4 y 8) y la última corchea. Nada en el motor sabe que se están
tocando dos métricas; simplemente toca doce pasos. El espacio en medio
de cada línea es para el lector, no para el parser.

El bombo legüero no tiene sample propio. `Drum` tiene trece variantes y
añadir una supone un sample nuevo, una entrada nueva en el menú y una
versión nueva del formato de archivo, así que los patrones de folclore
toman prestado: `LowTom` para el parche, `Rim` para el aro, `Shaker`
para las chajchas y las maracas, `Clap` para las palmas de la cueca,
`Conga` para el chico del candombe. Lo importante de un patrón es dónde
caen los golpes, y el oído rellena el resto.

## Un comping que cabe en doce pasos

El primer borrador de los estilos de folclore rasgueaba, con
`Comping::Strum`, y una prueba en `apply.rs` lo rechazó:

```
Chacarera is in three but plays Strum
```

`Strum` está escrito como un patrón de dieciséis pasos, un compás de
4/4. En un compás de doce pasos se cortaría un pulso antes, y el corte
caería en un sitio distinto en 3/4 y en 6/8. La prueba, que llegó con
[el recomp de ritmos](new-rhythm.md), solo permite los compings que se
escribieron para tres tiempos:

```rust
assert!(
    !three || matches!(comping, Comping::Waltz | Comping::Sway | Comping::Arp3 | Comping::Held),
    "{} is in three but plays {comping:?}",
    rhythm.name
);
```

`matches!` es una macro que devuelve `true` cuando un valor encaja con
un patrón; aquí hace en una línea lo que haría un `match` de cuatro
brazos. Así que los estilos en 6/8 tocan `Sway` (un acorde en el pulso
2, mantenido), `Waltz` (acordes en los pulsos 2 y 3 tras el bajo) o
`Arp3` (arriba y abajo por el acorde en corcheas). Contra la batería de
6/8, `Waltz` y `Sway` *son* el lado 3/4 de la hemiola, y por eso el gato
y la cueca los usan.

## Dejar que una prueba elija

Cada estilo debe pasar `every_style_is_heard_as_its_chart`: tocado a su
manera, el oído de acordes de [Un ritmo nuevo para una
canción](new-rhythm.md) debe recuperar su tabla exacta. Cuatro de los
sesenta estilos escritos para esta tanda fallaron, y podía fallar
cualquiera de los trece de folclore. En lugar de adivinar, una prueba
temporal probó cada pareja de los 26 compings y los 21 bajos para cada
tabla e imprimió las que el oído oía mal:

```
== Slow blues: 80 BAD: ArpUp/Pulse ... Arp3/Waltz ...
== Bachata: 40 BAD: ... ArpUpDown/Tresillo ...
== Cha-cha-chá: 94 BAD: Montuno/None Montuno/Held ... Montuno/Tumbao ...
== Joropo: 15 BAD: ...
== Vidala: 0 BAD:
```

Todos los fallos eran de un mismo tipo: los arpegios y los montunos
reparten un acorde de cuatro notas tan fino a lo largo del compás que un
tramo contiene solo tres notas, y el oído nombra esas tres. Así que el
slow blues toca `Pulse` (acordes en tresillos en 12/8), la bachata
arpegia solo hacia arriba, el cha-cha-chá toca stabs, y el son montuno
conserva su montuno pero sobre un bajo `Tresillo` en lugar del tumbao,
cuya anticipación de la siguiente fundamental se oía como un acorde
suspendido. La prueba temporal se borró después. Una prueba que se
ejecuta una vez y te dice algo sigue siendo una prueba que vale la pena
escribir.

## Doce octavos

El slow blues quería un compás de cuatro pulsos cada uno dividido en
tres, y no había compás para ello. `TimeSignature::new(12, 8)` ya
funcionaba, 12 × 16 / 8 = 24 pasos, así que el único cambio fue una
constante en `style.rs` y `rhythm.rs`, junto a las demás:

```rust
const TWELVE_EIGHT: TimeSignature = TimeSignature::new(12, 8);
```

Una melodía en 12/8 escribe una corchea como 2 pasos y un compás como
24. El piano roll y el playlist no saben nada nuevo: un compás dura lo
que dice el compás.

## Transcritas, no recordadas

La primera tanda enseñó una lección dura, contada en [Más
demos](more-demos.md#fieles-a-la-partitura): las melodías escritas de
memoria entran un pulso antes, pierden notas y equivocan acordes. Para
esta tanda, cada canción vino de una fuente escrita, una transcripción
ABC, LilyPond o MusicXML o una partitura grabada, y la fuente figura en
un comentario encima de cada canción. Dos reglas decidían qué podía
entrar:

- **Dominio público.** Una canción entra si es tradicional, o si todos
  sus compositores murieron antes de 1956 y se publicó por primera vez
  antes de 1929. Eso deja entrar a la guardia vieja del tango (El
  Choclo, La Cumparsita, Gallo ciego), los spirituals y los himnos
  gospel, a Schubert, Verdi, Grieg, Mussorgsky, Debussy, Rachmaninoff y
  Holst, y a El cóndor pasa y Alma llanera, y deja fuera casi todo lo
  grabado después de 1930.
- **Comprobada antes de oírse.** Un script pequeño, fuera del
  repositorio, lee un archivo de constantes `Song` y comprueba lo que
  comprobarán las pruebas unitarias: cada compás suma los pasos que
  marca su compás, la melodía tiene tantos compases como su tabla, toda
  forma nombra una parte que existe, y al menos el 80% de las notas
  están en la tonalidad de la parte. Las propias notas se comprueban
  contra la fuente, a cargo de quien las transcribe. El script
  encuentra los deslices de la pluma; la fuente encuentra los deslices
  de la memoria.

Una regla tan estricta deja huecos. Zamba de Vargas, las chacareras
clásicas y la mayoría de las cuecas son tradicionales, pero no se pudo
alcanzar ninguna transcripción escrita con la que comprobarlas, y
escribirlas de memoria es exactamente el error que cometió la primera
tanda. Así que cada danza sin fuente comprobada recibió en su lugar una
melodía nueva, escrita para Ferrous Lineup y señalada como tal en su
`origin`: Zamba del telar, Chacarera del patio, Gato del arroyo y ocho
más. Son cortas y sencillas a propósito, para que lo que oigas sea el
ritmo y la armonía de la danza.

## Las canciones

Las demos nuevas, por género en el menú Songs:

- **Folclore sudamericano**: El cóndor pasa y Dos palomitas (huayno),
  Alma llanera (joropo) y las once melodías de danza nuevas.
- **Tango**: El Choclo, La Cumparsita, La Paloma (una habanera) y Gallo
  ciego.
- **Soul, funk y R&Si**: dieciocho spirituals e himnos gospel, de Swing
  Low, Sweet Chariot y Go Down, Moses a O Happy Day.
- **Country y folk**: The Wild Rover, Shenandoah, Danny Boy, Loch
  Lomond, Waltzing Matilda, Soldier's Joy, Cotton-Eyed Joe y más.
- **Rock y pop**: unas cuantas melodías antiguas tocadas a la manera
  moderna: Whiskey in the Jar, When Johnny Comes Marching Home, la
  Toreador Song de Bizet como marcha rock.
- **Clásica y cine**: La donna è mobile, Ave Maria, el Hallelujah
  Chorus, la Wedding March, The Great Gate of Kiev, Anitra's Dance, el
  Prelude in C-sharp minor de Rachmaninoff, el himno Jupiter de Holst y
  el Dies irae de Mozart; la Rêverie de Debussy aparece como lo-fi hip
  hop y su Golliwogg's Cakewalk como jazz.

El `about` de cada canción dice qué escuchar: dónde está la modulación,
qué acorde está prestado, dónde cae la hemiola.

## Una demo se abre con su estilo y su ritmo seleccionados

En un buen navegador, lo que cargues se queda resaltado, para que puedas
ver de dónde vino y probar el de al lado. Aquí, abrir una demo solía
dejar la pestaña Library sin nada seleccionado. La biblioteca solo
resaltaba lo que ella misma había escrito en el patrón seleccionado, y
una demo la construye `song_demo`, no un clic en la biblioteca.

El arreglo empieza por los datos. Un `Song` ya nombraba su ritmo; ahora
también nombra el estilo de armonía al que más se acerca:

```rust
pub struct Song {
    ...
    pub rhythm: &'static str,
    /// The style in [`super::STYLES`] whose harmony and playing the song
    /// is closest to, or "" for none. The library shows it, and the
    /// rhythm, selected while the demo is open.
    pub style: &'static str,
    ...
}
```

Un nombre en un string puede estar mal escrito, así que una prueba
busca cada uno, y de paso comprueba el compás, ya que una canción en
6/8 no debería apuntar a un estilo en 4/4:

```rust
#[test]
fn every_song_names_a_real_style() {
    for song in SONGS {
        if song.style.is_empty() {
            continue;
        }
        let style = crate::library::Style::named(song.style)
            .unwrap_or_else(|| panic!("{}: no style called {}", song.name, song.style));
        assert_eq!(style.time, song.time, "{}: {} is in another time", song.name, song.style);
    }
}
```

En el lado de la app, `Tried` es la pareja que resalta la biblioteca: un
`Option<&'static Style>` y un `Option<&'static Rhythm>`. La app guarda
uno más de ellos, `song_library`, para la canción en conjunto, y `Demo`
dice qué poner ahí:

```rust
fn library(self) -> Tried {
    match self {
        Demo::Start => Tried::default(),
        Demo::Song(song) => Tried {
            style: Style::named(song.style),
            rhythm: Rhythm::named(song.rhythm),
        },
        Demo::Style(style) => Tried {
            style: Some(style),
            rhythm: Rhythm::named(style.rhythm),
        },
    }
}
```

`Style::named` devuelve un `Option`, así que un nombre vacío da
simplemente `None`, y no se selecciona nada. Abrir una demo asigna
`self.song_library = demo.library()`, y abrir cualquier otro archivo lo
devuelve a `Tried::default()`, dos `None`s.

Lo que muestra la biblioteca es entonces un `match` con un respaldo:

```rust
pub(super) fn tried(&self) -> Tried {
    match self.unchanged_audition() {
        Some(audition) => Tried {
            style: audition.style,
            rhythm: audition.rhythm,
        },
        None => self.song_library,
    }
}
```

Cuando un clic en la biblioteca cambia la canción entera mientras suena,
la selección de la canción lo sigue. Solo se reemplaza la mitad que
cambió, con la *sintaxis de actualización de struct*: `..song_library`
copia todos los campos no nombrados antes.

```rust
self.song_library = match action {
    LibraryAction::Style(style) => Tried {
        style: Some(style),
        ..song_library
    },
    LibraryAction::Rhythm(rhythm) | LibraryAction::RhythmInSong(rhythm) if !fill => {
        Tried {
            rhythm: Some(rhythm),
            ..song_library
        }
    }
    _ => song_library,
};
```

`Tried` es `Copy` (copiar dos referencias compartidas es barato), así
que `song_library` puede volver a usarse después de `..song_library`
sin que se mueva de ahí.

### Mostrarlo

Un resaltado dentro de un género cerrado no es ningún resaltado, y todo
género empieza cerrado. Así que el navegador de la biblioteca recuerda
la selección que dibujó la última vez, `shown`, y cuando cambia abre el
género que contiene el estilo o el ritmo nuevo y lo desplaza hasta que
se vea:

```rust
let new_style = tried.style.is_some() && tried.style != self.shown.style;
let new_rhythm = tried.rhythm.is_some() && tried.rhythm != self.shown.rhythm;
self.shown = tried;
```

`CollapsingHeader::open(Some(true))` de egui abre una cabecera solo en
este frame; después el usuario puede volver a cerrarla, que es lo que
queremos. `Response::scroll_to_me(None)` pide al `ScrollArea` que lo
rodea que se desplace lo justo para mostrar un widget, y no hace nada si
ya está visible. Pero una cabecera se abre con una animación corta, y
durante ella la lista aún no es lo bastante alta para desplazarse hasta
su fondo. Así que el navegador sigue pidiéndolo durante medio segundo:

```rust
if new_style || new_rhythm {
    self.reveal_until = now + 0.5;
    self.reveal_rhythm = !new_style;
}
let revealing = now < self.reveal_until;
```

`reveal_rhythm` decide a dónde desplazarse cuando cambiaron los dos: al
estilo, que va primero en la lista. Cuando solo cambió el ritmo, como
tras un clic en un ritmo, la lista se desplaza al ritmo, para que la
vista no salte lejos de donde el usuario acaba de hacer clic.

### Deshacer

Ctrl+Z deshace un clic en la biblioteca, pero el historial de deshacer
guarda solo `Project`s, y `song_library` no forma parte del proyecto.
Tras deshacer, la batería antigua volvía pero el ritmo nuevo seguía
resaltado. El arreglo guarda, junto al historial, una lista corta de las
versiones de proyecto que hizo la biblioteca y la selección que acompaña
a cada una:

```rust
song_versions: Vec<(Project, Tried)>,
```

Un clic en la biblioteca recuerda el proyecto de antes con la selección
antigua, y el proyecto de después con la nueva. Tras deshacer o rehacer,
la app busca la versión en la que está ahora:

```rust
pub(super) fn restore_song_library(&mut self) {
    if let Some((_, tried)) = self
        .song_versions
        .iter()
        .rev()
        .find(|(version, _)| *version == self.project)
    {
        self.song_library = *tried;
    }
}
```

`Project` deriva `PartialEq`, así que un proyecto entero se puede
comparar con `==`, la misma comparación que usa `edit` para decidir si
algo cambió. `iter().rev()` busca desde el extremo más nuevo, y el
patrón `|(version, _)|` desarma la tupla directamente en los argumentos
del closure. Si la versión no está en la lista (una edición hecha a mano
desde entonces), la selección se queda como está, que es lo correcto:
las ediciones a mano no cambian el estilo.
