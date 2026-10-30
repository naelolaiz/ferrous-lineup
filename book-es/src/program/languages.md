# Dos idiomas: traducciones, y letras o solfeo

Archivos: [`crates/lineup-core/src/i18n.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/i18n.rs),
[`crates/lineup-core/src/i18n/es.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/i18n/es.rs),
[`crates/lineup-app/src/i18n.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/i18n.rs),
[`crates/lineup-core/src/pitch.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/pitch.rs),
[`crates/lineup-core/src/harmony/symbol.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/symbol.rs),
[`crates/lineup-core/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/chord_sheet.rs),
[`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs)
y [`crates/lineup-app/src/app/settings.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/settings.rs)

Dos ajustes cambian cómo te habla Ferrous Lineup:

- **Edición > Ajustes… > Apariencia > Idioma** (con la app en inglés,
  **Edit > Settings… > Appearance > Language**) escribe los menús, las
  ventanas y los ajustes en **English** o en **Español**.
- **Edición > Ajustes… > Armonía > Nomenclatura de notas** (en inglés,
  **Harmony > Note naming**) nombra las notas, las tonalidades y los
  acordes con **letras** (C, F#, Bb major, Am7) o en **solfeo** (Do,
  Fa#, Sib mayor, Lam7).

Son independientes: puedes tener los menús en español con letras, o los
menús en inglés con solfeo. Pero mientras no elijas una nomenclatura,
sigue al idioma: con los menús en español las notas se nombran en
solfeo, como en las escuelas de habla hispana, y con los menús en inglés,
con letras. Cuando eliges una, se queda, cambies después al idioma que
cambies. Los dos ajustes cambian solo lo que ves. Una canción guardada
con una elección se abre exactamente igual para quien tenga la otra.

Con el español elegido, todo lo que escribe la app está en español:
los menús, las ventanas, las ayudas, los avisos y los nombres de las
demos, los estilos, los instrumentos y los canales. En inglés solo
quedan los mensajes que llegan del sistema operativo o de una biblioteca
(por ejemplo, por qué no se pudo abrir un dispositivo de audio) y los
nombres de géneros que el español usa tal cual, como swing o funk. Este
libro nombra los menús, las ventanas y los botones como los escribe la
app en español. La mayoría de las capturas de este libro se hicieron con
la app en inglés.

![Ajustes > Apariencia: el idioma, escrito en su propio idioma para que cada cual encuentre el suyo.](../images/step-9-languages-settings.png)

## Las notas Do, Re, Mi

En inglés y en alemán, las notas se nombran con letras. En español,
italiano, francés y portugués, y en los países que los hablan, se
nombran con sílabas:

```text
letras    C    D    E    F    G    A    B
solfeo    Do   Re   Mi   Fa   Sol  La   Si
```

Los sostenidos y los bemoles se escriben detrás de la sílaba igual que
detrás de la letra: Fa sostenido es **Fa#** y Si bemol es **Sib**. Una
tonalidad se nombra igual, así que Si♭ mayor es **Sib mayor** (o **Sib
major** con los menús en inglés), y el modo dórico sobre Re es **Re
dórico**.

Esto es el **Do fijo**: Do es siempre la misma nota, sea cual sea la
tonalidad. Algunos métodos de entrenamiento auditivo usan un **do
móvil**, en el que *do* es la tónica de la tonalidad (en Sol mayor, *do*
es Sol). La app usa siempre el Do fijo, como las escuelas de España y
América Latina, porque el nombre de una nota no debe cambiar con la
tonalidad.

Los números de octava no cambian: el Do central es C4 con letras y
**Do4** en solfeo, y cada octava va de Do a Si. Algunos libros
franceses y españoles numeran las octavas una más abajo y llaman Do3 al
Do central. La app mantiene una sola numeración, así que una nota
conserva su número la nombres como la nombres.

![Ajustes > Armonía, con el solfeo elegido: los ejemplos de los números romanos en menor lo siguen, Do es bIII en La menor.](../images/step-9-languages-naming.png)

## Los acordes en solfeo

Un cifrado es una fundamental, una cualidad y, a veces, una nota en el
bajo. En solfeo solo cambian los nombres de las notas:

```text
letras    C    Am7     G7/B     Bdim     F#m7b5    Eb+
solfeo    Do   Lam7    Sol7/Si  Sidim    Fa#m7b5   Mib+
```

A la primera manera se le llama *cifrado americano* y a la segunda
*cifrado latino*. Las partituras de jazz y de pop suelen ir en cifrado
americano, también en español, y por eso este libro escribe así los
cifrados; muchos profesores y cancioneros usan el latino. Con el solfeo
elegido, la hoja de acordes, las teclas de armonía, el piano roll, los cambios de
tonalidad de la lista de reproducción y los ejemplos de las ayudas escriben las
notas, las tonalidades y los acordes de la segunda manera.

![La hoja de acordes en solfeo: Lam7 y Re7 en el compás 9, y Sol7/Si, escrito como G7/B, en el compás 10.](../images/step-9-languages-chord-sheet.png)

### Escribir un acorde de las dos maneras

Puedes escribir un acorde de cualquiera de las dos maneras, esté elegida
la que esté: `Am7` y `Lam7` son el mismo acorde. Al pulsar **Intro**, el
compás lo muestra como el ajuste nombra las notas.

Unos pocos nombres se pueden leer de las dos maneras. `o` es una forma
de escribir un acorde disminuido (`Bo` es B°), así que `Do` podría ser
Do mayor (solfeo) o Re disminuido (letras), y `Do7` podría ser C7 o D°7.
La app lee esos nombres como el ajuste nombra las notas: con el solfeo
elegido, `Do` es Do mayor. Los nombres que solo tienen sentido de una
manera se leen de esa manera: `Faug` es Fa aumentado aunque esté elegido
el solfeo, porque «Fa» seguido de «ug» no es un acorde, y `Fam` es Fa
menor aunque estén elegidas las letras.

La cualidad se escribe igual en los dos: Fa aumentado es `Faaug` en
solfeo. Se lee bien, pero `Fa+` se ve mejor, y la app también lo lee.

En la hoja de acordes, escribir sobre un compás seleccionado empieza a
escribir en él, pero **L** es la tecla que cambia entre patrón y
canción. Con el solfeo elegido, los acordes de La empiezan por L, así
que ahí L empieza a escribir. Con las letras elegidas, ningún acorde
empieza por L, y L cambia de modo como siempre.

## Qué se guarda

Los proyectos, los estilos guardados, el texto de la hoja de acordes que copia
**Ctrl+C** y las trazas de reproducción se escriben siempre con letras,
y en inglés. Los ajustes no llegan nunca a ellos. Así que una canción que guardaste
viendo Lam7 contiene `Am7`, y se abre con Am7 o con Lam7 según los
ajustes de quien la abra.

## En el código

### Dos ajustes, guardados como el de los números romanos

`Language` y `NoteNaming` (los dos en `lineup-core`) se guardan
como el [ajuste de los números romanos en menor](../theory/numerals.md#un-ajuste-que-toda-función-puede-ver-uno-por-hilo):
un enum con un `#[default]`, un `Cell` dentro de `thread_local!` que
toda función puede leer, y un `with` que lo fija un rato y lo devuelve
con un guard en un `Drop`, incluso tras un panic.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum NoteNaming {
    #[default]
    Letters,
    Solfege,
}

thread_local! {
    static NOTE_NAMING: Cell<NoteNaming> = const { Cell::new(NoteNaming::Letters) };
}
```

Una nota se escribe en cientos de lugares, a través de su implementación
de `Display` (`{}` en `format!`). Un valor por hilo permite que todos
sigan el ajuste sin pasarlo hacia abajo por cada función:

```rust
impl fmt::Display for NoteName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.format(NoteNaming::current()))
    }
}
```

La ventana fija los dos valores a partir de los ajustes guardados antes
de dibujar cada fotograma:

```rust
pub fn apply_settings(ctx: &egui::Context) {
    language(ctx).set();
    note_naming(ctx).set();
}
```

Una exportación de video dibuja sus vistas en un hilo propio, que empieza
con los valores por defecto, así que se lleva el idioma y la nomenclatura
consigo, como ya se llevaba el ajuste de los números romanos.

### Solfeo hasta que elijas

egui guarda las preferencias de la ventana como valores bajo un `Id`, y
`get_persisted` da `None` para un `Id` bajo el que no se guardó nada.
Ese `None` es lo que distingue «nunca elegido» de una elección, así que
la nomenclatura recurre al idioma solo entonces:

```rust
pub fn note_naming(ctx: &egui::Context) -> NoteNaming {
    ctx.data_mut(|data| data.get_persisted(note_naming_id()))
        .unwrap_or_else(|| language_naming(language(ctx)))
}

/// How notes are named where `language` is spoken.
fn language_naming(language: Language) -> NoteNaming {
    match language {
        Language::English => NoteNaming::Letters,
        Language::Spanish => NoteNaming::Solfege,
    }
}
```

Leer nunca guarda nada; solo lo hace `set_note_naming`, que se llama
cuando haces clic en una nomenclatura en los Ajustes. Así que hasta ese
clic cada fotograma vuelve a preguntar al idioma, y pasar al español pasa
las notas al solfeo. El `match` no tiene un brazo `_` a propósito: un
tercer idioma no compilaría hasta que alguien diga cómo nombra las notas.

### Traducir: la clave es el texto en inglés

Cada texto que muestra la interfaz está escrito en inglés en el código y
pasa por `tr`. Vive en `lineup-core`, no en la app, para que también se
traduzcan los textos que escribe el núcleo, los nombres de los estilos,
los instrumentos, los acordes y las escalas:

```rust
pub fn tr(text: &'static str) -> &'static str {
    match Language::current() {
        Language::English => text,
        Language::Spanish => in_spanish(text).unwrap_or_else(|| missed(text)),
    }
}
```

El catálogo en español está en `i18n/es.rs`: un gran `match` para los
menús, las ventanas principales y los ajustes, y, en los archivos de
`i18n/es/`, tablas de pares por área (la app, la teoría, los
instrumentos, la biblioteca, las canciones), reunidas una sola vez en un
`HashMap`:

```rust
static TEXTS: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| TABLES.into_iter().flatten().copied().collect());

/// `text` in Spanish, or `None` for a text with no translation yet.
pub fn translate(text: &str) -> Option<&'static str> {
    main_texts(text).or_else(|| TEXTS.get(text).copied())
}
```

`LazyLock` construye el mapa la primera vez que se usa, en el hilo que
lo pida primero, y a partir de ahí todos los hilos lo comparten.
`flatten` convierte las cinco tablas en una sola serie de pares, y
`collect` construye el mapa con ellos.

Usar el propio texto en inglés como clave tiene dos ventajas. El código
se sigue leyendo con naturalidad (`tr("Settings")`, no
`tr(SETTINGS_TITLE)`), y un texto que aún no tiene traducción se queda
simplemente en inglés, gracias a `unwrap_or_else`. El precio es que
cambiar un texto en inglés obliga a cambiar también su clave en el
catálogo, o se pierde el español; una prueba lo detecta (ver más abajo).

Tanto el argumento como el resultado son `&'static str`: texto escrito
en el propio programa, que vive tanto como el programa. Así `tr` puede
devolver cualquiera de los dos sin copiar nada.

Unos cuantos parientes de `tr` cubren los demás casos:

- `trf("{} steps a bar", &[&12])` pone sus argumentos donde el texto
  traducido tiene `{}`, así que el español puede poner el número entre
  otras palabras: «12 pasos por compás».
- `trc("clef", "Bass")` es para una palabra cuya traducción depende de
  dónde se muestra: la clave de fa es «Fa», el instrumento «Bajo». El
  catálogo la tiene como `"clef|Bass"`.
- `tr_name` traduce los nombres que la app da a las cosas, un canal, una
  pista o un patrón: «Bass» es «Bajo», «Pattern 12» es «Patrón 12» y «A
  drums» es «Batería de A». Un nombre que escribiste tú se queda como lo
  escribiste.
- Un texto del catálogo puede nombrar notas entre llaves, como en
  `"{Bb} mayor"`; se nombran como dice el ajuste (ver [Ejemplos que
  siguen el ajuste](#ejemplos-que-siguen-el-ajuste)). Un texto así se
  construye una vez para cada nomenclatura y se conserva, con
  `Box::leak`, que convierte un `String` en un `&'static str` que nunca
  se libera. Hay un número limitado de textos y dos nomenclaturas, así
  que la memoria conservada tiene un límite.

### Nombres en pantalla, letras en los archivos

Como `Display` sigue el ajuste, todo lo que convierta una nota en texto
y lo guarde guardaría solfeo. Los lugares por donde los datos salen del
programa trabajan con letras, a través de `with`:

```rust
pub fn to_text(project: &Project) -> Result<String, FileError> {
    // …
    let file = Language::English.with(|| NoteNaming::Letters.with(|| ProjectFile::from(project)));
    let mut text = NoteNaming::Letters.with(|| ron::ser::to_string_pretty(&file, style))?;
    // …
}
```

Un marcador de acorde se guarda como su texto, mediante los atributos de
serde `into = "String"` y `try_from = "String"` (ver
[Series dodecafónicas](../theory/twelve-tone.md#archivos-try_from-e-into)). Sus conversiones
también trabajan con letras, así que un marcador se guarda como `Am7`
lo guarde quien lo guarde:

```rust
impl From<HarmonyName> for String {
    fn from(harmony: HarmonyName) -> Self {
        NoteNaming::Letters.with(|| harmony.to_string())
    }
}
```

Los estilos guardados, el texto de la hoja de acordes y la traza de
reproducción hacen lo mismo. Los cifrados de la propia biblioteca están
escritos con letras en el código fuente, así que también se leen con
letras.

### Leer un nombre de las dos maneras

`NoteName::read_as` lee una nota al principio de un texto con una
nomenclatura, y devuelve el nombre y el resto del texto. Para el solfeo
prueba cada sílaba:

```rust
NoteNaming::Solfege => Letter::ALL.into_iter().find_map(|letter| {
    text.strip_prefix(letter.solfege())
        .map(|rest| (letter, rest))
})?,
```

`strip_prefix` devuelve el texto que sigue al prefijo si el texto empieza
por él, y `None` si no. `find_map` devuelve el primer `Some`. «Sol» y
«Si» empiezan los dos por S, pero ninguno es el principio del otro, así
que solo uno puede coincidir.

`read_either` prueba las dos nomenclaturas, primero la del ajuste, y se
queda con la primera lectura cuyo resto encaja (`fits`):

```rust
pub fn read_either<'a>(
    text: &'a str,
    fits: impl Fn(&'a str) -> bool,
) -> Option<(NoteName, &'a str)> {
    NoteNaming::current()
        .reading_order()
        .into_iter()
        .filter_map(|naming| NoteName::read_as(text, naming))
        .find(|&(_, rest)| fits(rest))
}
```

Qué significa «encajar» depende de quién llama, así que es un closure.
Un cifrado encaja cuando el resto es una cualidad y quizá un bajo; un
nombre de nota completo encaja cuando no queda nada:

```rust
pub fn parse(text: &str) -> Option<NoteName> {
    NoteName::read_either(text, str::is_empty).map(|(name, _)| name)
}
```

`str::is_empty` es una función, y una función puede pasarse allí donde se
espera un closure con los mismos argumentos.

El lifetime `'a` dice que el resto que recibe `fits`, y que se devuelve,
es un trozo de `text`, no una copia. Así funciona «Faug» con el solfeo
elegido: «Fa» deja «ug», que `fits` rechaza, así que se prueba la lectura
con letras, F con «aug», y se queda esa.

### Ejemplos que siguen el ajuste

Las ayudas de la hoja de acordes y de los ajustes dan acordes de ejemplo. Se
escriben con las notas entre llaves, `{A}m7 {D}7`, y `with_notes` nombra
cada una como dice el ajuste, así que con el solfeo elegido la ayuda
dice «Lam7 Re7». Las llaves alrededor de algo que no es un nombre de
nota, como el conjunto `{0,1,4}` de la misma ayuda, se dejan como están.

### L de La

`ChordSheet::takes_l` dice si una L escrita ahora le corresponde a la
hoja: la hoja tiene el teclado, hay un compás seleccionado, todavía no
se está escribiendo en él, y las notas se nombran en solfeo. El atajo L
de la app lo mira primero:

```rust
let l_types = self.chord_sheet.takes_l() && !self.live.typing();
if !typing && !controls_active && !l_types && pressed(egui::Key::L) {
```

`!self.live.typing()` está ahí porque, con el teclado para tocar
activado, las letras tocan notas, y L cambia entre patrón y canción como
siempre.

### Fuera de la app

El archivo de escritorio de Linux tiene líneas en español junto a las
inglesas, por ejemplo `GenericName[es]=Banda de acompañamiento`, que un
escritorio en español muestra en sus menús. El archivo AppStream que
leen las tiendas de software da a cada párrafo de la descripción un
gemelo en español, `<p xml:lang="es">`, y `appstreamcli validate` lo
comprueba.

## Las pruebas

`every_window_writes_in_spanish` abre seis demostraciones en español con
todas las ventanas abiertas, y después los atajos, Acerca de, la
exportación de video, el editor de estilos, cada página de los Ajustes y
cada menú. Falla si algo le pidió a `tr` un texto que no tiene español
en el catálogo, o si algún texto dibujado en pantalla es un texto en
inglés que el catálogo sí tiene en español, lo que detecta el texto
dibujado sin pasar por `tr`. En el núcleo,
`every_name_and_description_the_app_shows_has_spanish` hace lo mismo con
cada nombre de instrumento, estilo, acorde, escala y demostración que el
núcleo le da a la app.

`solfege_changes_how_names_look_not_what_is_saved` construye cada
canción de demostración dos veces, una con letras y otra con solfeo, y
comprueba que el archivo guardado y el texto de la hoja de acordes son
idénticos, y que cada acorde, escrito como lo muestra la hoja en solfeo,
se lee de vuelta como el mismo acorde.

`names_are_read_in_letters_or_solfege` lee `Sib7`, `Fa#m`, `Sol` y
`Reb`, y comprueba que `Do7` empieza por Re con las letras elegidas y
por Do con el solfeo. `solfege_is_a_way_of_showing_names` comprueba que
`Am7` se muestra como Lam7, `G/B` como Sol/Si y Si♭ mayor como Sib
mayor, y que `Lam7` y `Am7` son el mismo acorde.

`in_solfege_l_types_la_chords_and_in_letters_it_switches_mode`
selecciona un compás de la hoja de acordes, pulsa L con el solfeo elegido y
escribe `Lam7`, y comprueba que el modo no cambió y que el compás tiene
La menor séptima; con las letras, L cambia el modo y deja el compás como
estaba.

`spanish_names_notes_in_solfege_until_a_naming_is_chosen` cambia el
idioma de un lado a otro sin nomenclatura elegida, y la nomenclatura lo
sigue; luego elige una nomenclatura y vuelve a cambiar el idioma, y la
nomenclatura se queda. `spanish_names_notes_in_solfege_and_keeps_letters_once_chosen`
lo hace como lo harías tú: menús en español, **Armonía**, **Letras (C,
D, E…)** y un reinicio de la app, tras el cual las notas siguen en
letras.
