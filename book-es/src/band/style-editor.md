# Tus propios estilos: el editor de estilos

Archivos: [`crates/lineup-app/src/style_editor.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/style_editor.rs),
[`crates/lineup-app/src/app/styles.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/styles.rs),
[`crates/lineup-core/src/library/user_style.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/user_style.rs),
[`crates/lineup-core/src/library/apply.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/apply.rs)
y [`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs)

La [biblioteca](library.md) es conocimiento escrito como
datos: un estilo es un cifrado, un patrón de comping, una línea de bajo
y un ritmo, cada uno unas pocas líneas en una tabla. El **editor de
estilos** te deja escribir un groove que esas tablas no tienen, con el
rack de canales y el piano roll que ya conoces, y guardarlo como un
estilo propio. La banda lo toca luego sobre cualquier hoja de acordes,
llevando cada nota al acorde del momento.

## Probarlo

1. Abre **Archivo > Nuevo**. Pulsa **F6** para el rack de canales y haz
   clic en **Editar estilo…**. El editor se abre con una copia de la
   banda que toca la canción, **Ritmo pop (copia)**, en una ventana
   titulada **Editor de estilos · rack de canales**.
2. Abre **Empezar desde…**, escribe `bossa` en **Buscar preajustes** y
   elige un estilo. Haz clic en **Escuchar ×4**.
3. Haz clic en algunos pasos del bombo y de la caja. Haz clic en la
   vista previa de una nota del piano para abrir el piano roll del
   editor; dibuja una figura de acordes y pulsa **Espacio** para oírla.
4. Haz clic en **Guardar como…**, llámalo `Mi groove` y guárdalo en la
   carpeta que abre el diálogo. Cierra el editor, abre la
   **Biblioteca** y busca **Mi groove** entre los preajustes (bajo
   **Estilos de usuario** cuando se agrupan por género).
5. Aplícalo con **Batería** y **Partes de acordes y bajo** marcadas y
   **Progresión de acordes** sin marcar. La canción conserva sus
   acordes y la banda toca tu patrón sobre ellos. Guarda el proyecto,
   cierra la app, mueve el archivo `.lineup-style` a otro sitio y abre
   el proyecto otra vez: la banda lo sigue tocando.

## Dónde está

- En el **rack de canales**, **Editar estilo…** abre el estilo que toca
  la canción («Editar una copia de las partes repetidas del estilo
  actual», dice su ayuda); **Nuevo estilo…** empieza uno vacío.
- **Ver > Editor de estilos…** abre el editor, o lo trae al frente.
- La **Biblioteca** tiene un botón **Nuevo estilo…** sobre su lista.
  Haz clic derecho en un estilo o un ritmo: uno integrado ofrece
  **Editar una copia en el rack de canales…**; uno tuyo, **Editar en el
  rack de canales…**.

**Editar estilo…** empieza desde lo que toca la canción. Un estilo tuyo
se abre como sí mismo, así que **Guardar estilo** escribe de vuelta en
su archivo. Una banda se abre como una copia de todos sus músicos, dos
compases de lo que tocan sobre un acorde de C, llamada «… (copia)». Un
estilo o un ritmo que se está probando en un patrón se abre como una
copia suya, y sin nada que copiar el editor empieza vacío: un bombo,
una caja, un charles y un piano, sobre la progresión `I | V | vi | IV`.

## Qué edita

Un estilo de usuario es **un patrón que se repite**, de uno a dieciséis
compases y de 256 pasos como mucho (así que en 12/8, con 24 pasos por
compás, diez compases como mucho). Contiene carriles de batería, uno
por cada sonido de batería, y como mucho un carril por cada músico con
altura de la banda: piano, bajo, guitarra, cuerdas, órgano, solista y
vientos.

La fila de arriba tiene **Empezar desde…**, **Guardar estilo**,
**Guardar como…**, **Deshacer**, **Rehacer**, **Escuchar ×4** y
**Detener la escucha**. Debajo están **Nombre**, **BPM**, el compás,
**Compases** y **Swing**; luego **Progresión**, los acordes que trae el
estilo, en números romanos o nombres de acordes con los compases
separados por `|`; **Descripción**; y **Asignación de partes**, que dice
qué músico es cada fila, de modo que una fila llamada «Rhodes» puede ser
el piano.

El rack es el mismo widget que el de la canción, con las filas del
borrador: haz clic en los pasos para escribir la batería. Haz clic en la
vista previa de una nota para abrir el piano roll, donde dibujar,
mover, estirar y el menú Herramientas funcionan como en un patrón. El
piano roll del editor tiene su propia ventana y sus propias teclas, y
nunca edita el de la canción.

**Escuchar ×4** toca el patrón cuatro veces, tal como está escrito sobre
Do, y se detiene; **Espacio** lo arranca o lo para mientras el editor
tiene el teclado, y **Detener la escucha** lo termina antes. La canción
se pausa mientras suena. **Ctrl+S** guarda y **Ctrl+Mayús+S** guarda
como.

El editor tiene un historial propio. **Ctrl+Z**, **Ctrl+Mayús+Z** y
**Ctrl+Y** deshacen y rehacen el borrador sin tocar el historial de la
canción. Nada llega a la canción hasta que aplicas el estilo guardado,
y entonces aplicarlo es un solo paso que se puede deshacer.

## La música: una parte sobre todos los acordes

La ayuda bajo los campos lo dice: «Escribe las partes con altura sobre
Do mayor. Siguen los acordes de la canción al aplicarlas». Un músico que
aprende una figura de acompañamiento la aprende como una forma,
fundamental, tercera, quinta, séptima, y toca la misma forma sobre todos
los acordes. Escribir la forma sobre Do mayor le da nombres fijos: Do es
la fundamental, Mi la tercera, Sol la quinta, Si la séptima.

### Qué toca cada nota escrita

| Escrita sobre Do | Toca sobre el acorde |
|---|---|
| Do | su fundamental |
| Mi | su tercera, o la nota que tiene en su lugar un acorde suspendido (Fa en Csus4) |
| Sol | su quinta, disminuida o aumentada si la del acorde lo es |
| Si | su propia séptima (la sexta de un acorde de sexta); si no la tiene, la séptima de la tonalidad |
| Re, Fa, La | la nota de la tonalidad uno, tres o cinco grados por encima de la fundamental: la 9ª, la 11ª o la 13ª |
| Do♯, Mi♭, Fa♯, La♭, Si♭ | la misma distancia desde la fundamental: un color que se conserva |

El arpegio escrito Do–Mi–Sol–Si, en una canción en Do mayor:

| Acorde | Toca |
|---|---|
| Dm7 | Re Fa La Do |
| G7 | Sol Si Re Fa |
| Cmaj7 | Do Mi Sol Si |
| Am | La Do Mi Sol |
| F | Fa La Do Mi |
| Csus4 | Do Fa Sol Si |
| Am6 | La Do Mi Fa♯ |

Sobre Am y F, que no tienen séptima, el Si toma la séptima propia de la
tonalidad sobre la fundamental: Am suena como Am7 y F como Fmaj7, como
los colorearía un pianista que acompaña en Do mayor. Si quieres
tríadas simples, no escribas el Si.

### Una escala se vuelve el modo de cada acorde

Escribe la escala entera, Do Re Mi Fa Sol La Si, y la banda la toca
desde la fundamental de cada acorde con las notas de la tonalidad:

| Acorde | Toca | Que es |
|---|---|---|
| Dm7 | Re Mi Fa Sol La Si Do | Re dórico |
| G7 | Sol La Si Do Re Mi Fa | Sol mixolidio |
| Am | La Si Do Re Mi Fa Sol | La eólico |
| Bdim | Si Do Re Mi Fa Sol La | Si locrio |
| F | Fa Sol La Si Do Re Mi | Fa lidio |

Son los modos de Do mayor (ver [Escalas, modos y
tonalidades](../theory/scales.md#la-música-en-pocas-palabras)): las
notas de la tonalidad, empezadas desde cada acorde. Así, el Re que
escribes sobre Do, una novena, es el Mi de la tonalidad sobre Dm7 pero
Fa sobre Em, y el La, una sexta, es Do sobre Em, no Do♯: la línea nunca
sale de la tonalidad mientras los acordes estén en ella.

### Acordes de fuera de la tonalidad

Un acorde con una nota de fuera de la tonalidad, como un Fm prestado o
el E7 que lleva a Am, no puede tomar las notas de la tonalidad para su
novena o su sexta. Conserva las distancias escritas sobre Do, las notas
de su propia escala mayor, salvo que un acorde menor toma la séptima
menor:

| Acorde | Do Re Mi Fa Sol La Si toca | Do–Mi–Sol–Si toca |
|---|---|---|
| Fm | Fa Sol La♭ Si♭ Do Re Mi♭ | Fa La♭ Do Mi♭ |
| E7 | Mi Fa♯ Sol♯ La Si Do♯ Re | Mi Sol♯ Si Re |
| B♭ | Si♭ Do Re Mi♭ Fa Sol La | Si♭ Re Fa La |

Es más simple que las [escalas de acorde del
solista](soloist.md#escalas-de-acorde), que al E7 que lleva a
Am le darían el Fa y el Do de La menor. Un estilo es un patrón que
debe sonar igual sobre todos los acordes; el solista elige las notas
una a una.

Las notas cromáticas conservan su distancia desde la fundamental. Un
Mi♭ escrito, la tercera menor sobre Do, toca Si♭ sobre G7, la tercera
«blue» contra el Si del acorde; un Si♭ escrito toca Do sobre Dm7 y Mi♭
sobre F.

### El bajo, el registro y las notas largas

El bajo sigue las mismas reglas, con una más: sus Do tocan la nota del
bajo de un acorde con barra. Una línea escrita Do–Sol–Do–Mi
(fundamental, quinta, octava, tercera) toca Re–La–Re–Fa sobre Dm7, y
Mi–Sol–Mi–Mi sobre C/E: los Do se vuelven Mi, el Sol sigue siendo la
quinta del acorde y el Mi su tercera.

Una parte se mueve como mucho un tritono desde donde la escribiste.
Sobre un acorde de Do a Fa♯ sube; sobre Sol a Si baja, así que una
figura escrita alrededor del Do central se queda alrededor de él: sobre
G7 el arpegio es Sol3 Si3 Re4 Fa4, debajo del Do central, no una quinta
por encima.

Una nota que se sostiene a través de un cambio de acorde se corta ahí y
sigue en el acorde nuevo. La batería se copia tal como está escrita. Un
estilo de dos compases o más cuenta sus compases desde el principio de
la canción: uno de dos compases toca su primer compás en los compases
1, 3, 5 y así sucesivamente, y en un compás más corto que los del
patrón se dejan fuera las notas que no caben.

### Quién toca qué

Un estilo que solo tiene batería, como la copia de un ritmo, cambia
solo la batería: el piano, el bajo y los demás de la banda siguen como
estaban. En cuanto un estilo tiene un carril con altura, se encarga de
la armonía de la banda: el piano, el bajo, la guitarra, las cuerdas y el
órgano tocan solo los carriles que escribiste, y el que no tiene carril
se calla. El solista y los vientos tocan sus propias partes salvo que el
estilo tenga un carril para ellos. Tus partes también se detienen en los
[golpes, sostenidos, silencios y anticipaciones](stops.md) de
la hoja, y suenan más fuerte en la [variación
b](song-form.md#variaciones-a-y-b-la-banda-aprieta).

## Tus estilos son archivos

**Guardar estilo** escribe en el archivo del que vino el estilo.
**Guardar como…** pide uno nuevo. Los dos escriben archivos
**`.lineup-style`**: si escribes otra extensión, se sustituye, así que
`mi groove.txt` se convierte en `mi groove.lineup-style`.

El archivo es el estilo mismo, escrito en RON (Rusty Object Notation, el
formato de los archivos de proyecto) por `serde`:

```rust
pub struct StyleLane {
    pub player: Player,
    pub channel: Channel,
    pub notes: Vec<NoteEvent>,
}
```

```rust
pub struct UserStyle {
    pub format: u32,
    pub name: String,
    pub about: String,
    pub time: TimeSignature,
    pub tempo: Tempo,
    pub swing: u8,
    pub mode: ScaleKind,
    pub chart: String,
    pub bars: usize,
    pub lanes: Vec<StyleLane>,
}
```

Cada carril lleva su `Channel` entero: el instrumento, su programa
General MIDI, su volumen y sus demás ajustes. Los nombres se escriben
con letras aunque la app muestre solfeo, así que un estilo guardado en
un idioma se abre en el otro.

Los estilos viven en una **carpeta de estilos**, por defecto una
carpeta `styles` dentro de la carpeta de datos de la app. **Edición >
Ajustes… > Biblioteca > Tus estilos > Carpeta de estilos…** elige otra.
La Biblioteca lista solo esa carpeta: un estilo guardado en otro sitio
no aparece, y el aviso tras guardar lo dice: «Estilo guardado: … La
Biblioteca muestra los estilos de …; Ajustes > Carpeta de estilos la
cambia».

La Biblioteca lee cada archivo `.lineup-style` de la carpeta al
arrancar, y otra vez después de guardar o de cambiar de carpeta, los
ordena por nombre y avisa de los que no puede leer: «Algunos estilos no
se pudieron cargar: …». Se suman a los preajustes, junto a los estilos y
ritmos integrados. No tienen género, así que **Agrupar por género** los
pone bajo **Estilos de usuario (N)**. El filtro **Con acordes** deja los
que tienen progresión, y el menú de instrumentos lista los sonidos de
sus carriles. **Empezar desde…** en el editor muestra la misma lista y
la misma búsqueda, con **Estilo vacío** primero.

Aplicar uno de tus estilos funciona como con cualquier preajuste (ver
[Qué partes tomar](library.md#qué-partes-tomar)):
**Batería** trae sus carriles de batería, **Partes de acordes y bajo**
sus otros carriles, **Voces / instrumentos** sus sonidos, y **Compás**,
**Tempo**, **Swing** y **Progresión de acordes** lo suyo. Deja la
progresión sin marcar para conservar los acordes de la canción. Solo
las partes de acordes y bajo necesitan una banda que las toque, así que
en una canción sin banda, un estilo tuyo necesita esas partes marcadas
para que suenen sus carriles; su tempo, su compás, su swing o sus
acordes solos cambian la canción tal como está.

### El estilo viaja con la canción

Cuando se aplican las **Partes de acordes y bajo**, el `UserStyle`
entero se guarda en la banda, y cuando se aplica la **Batería**, en su
`drums`, como `BandDrums::Saved`:

```rust
pub custom_rhythm: Option<Box<crate::library::UserStyle>>,
pub custom_instruments: Option<Box<crate::library::UserStyle>>,
```

El primero guarda los patrones; el segundo, los instrumentos y sus
ajustes. El proyecto no necesita el archivo `.lineup-style`: bórralo,
muévelo o abre la canción en otro ordenador, y la banda sigue tocando.
Un proyecto que lleva un estilo tuyo necesita al menos el formato 22
(ver [el formato del archivo de
proyecto](../reference/project-format.md)). Por eso un estilo puede
desaparecer de la Biblioteca sin romper una canción que lo usa: la
canción guarda su propia copia.

## Cómo funciona

### Un proyecto de un patrón, propio

El editor no tiene un segundo rack ni un segundo piano roll. Guarda un
`Project` desechable de un solo patrón, construido por
`UserStyle::pattern_project`, junto al estilo y al músico de cada fila:

```rust
struct Draft {
    style: UserStyle,
    project: Project,
    roles: Vec<Player>,
}
```

Luego llama a los mismos widgets que las ventanas de la canción (ver
[Dibujar devuelve
acciones](../program/editors.md#dibujar-devuelve-acciones)). El rack
devuelve `RackAction`s, el piano roll devuelve `RollAction`s, y el
editor los aplica a `draft.project` en vez de a la canción. Añadir un
tambor, mover una nota o abrir el piano roll pasan por el código que ya
lo hace. La única acción que rechaza es un patrón nuevo: «Los estilos
contienen un solo patrón. Guarda los patrones nuevos desde el piano roll
de la canción».

Su deshacer es el propio `History<T>` de la app (ver
[`History<T>`](../program/undo.md#historyt)), con el borrador entero
como `T`:

```rust
history: History<Draft>,
```

### De las filas de vuelta al estilo

Después de cada fotograma, `sync` copia el borrador de vuelta al
estilo: métrica, tempo y swing, y luego un `StyleLane` por canal con el
músico y las notas de ese canal, cortadas al final del patrón:

```rust
draft.style.time = draft.project.time_signature;
draft.style.tempo = draft.project.tempo;
draft.style.swing = draft.project.swing;
```

`UserStyle::validate` comprueba el resultado antes de guardarlo o
aplicarlo, una regla cada vez, cada una con un mensaje para quien usa la
app («El patrón debe entrar en 1–16 compases y 256 pasos», «Asigna un
solo carril a {}»):

```rust
if !(1..=16).contains(&self.bars) || self.length() > MAX_STEPS {
    return Err(tr("The pattern must fit in 1–16 bars and 256 steps").to_string());
}
```

```rust
if lane.player != Player::Drums && !players.insert(lane.player) {
    return Err(trf(
        "Assign only one lane to {}",
        &[&tr(lane.player.name())],
    ));
}
```

Las demás comprueban el formato, el nombre, la progresión (cada compás
debe leerse como acordes), el número de carriles, el swing, los ajustes
de cada carril y que cada nota quede dentro del patrón que se repite.
Leer un archivo también llama a `validate`, así que un estilo dañado o
editado a mano se rechaza con el motivo en vez de tocar cualquier cosa.

### Aplicar: primero sobre una copia

`Project::apply_user_style` cambia la canción en varios pasos: la
banda, la métrica, el tempo, los acordes. Leer la progresión en los
compases de la canción puede fallar a medio camino, en una métrica en
la que el cifrado no cabe. Así que trabaja sobre un clon, y solo un
resultado terminado reemplaza la canción:

```rust
style.validate()?;
// The chart is read into the song's bars, which can fail halfway
// in another meter, after the band and the tempo changed: so it
// is applied to a copy, and an error changes nothing.
let mut project = self.clone();
project.apply_valid_user_style(style, parts)?;
*self = project;
Ok(())
```

`?` devuelve el error en el acto, y la canción queda como estaba. Si la
canción no tiene banda, se crea una para que toquen los carriles del
estilo; los músicos para los que el estilo no tiene carril tocan como
la banda suele hacerlo en ese compás.

### Seguir los acordes

Cuando la banda escribe sus partes, `UserStyle::play_lane` repite los
compases del estilo sobre los de la canción y le pasa cada nota con
altura a `adapt_pitch`, con la armonía y la tonalidad de ese punto. Su
comentario es la tabla de arriba en palabras:

```rust
/// The note a template note written over C major plays over `harmony`,
/// in `key`. C, E and G are the chord's root, third and fifth, and B is
/// its seventh, or sixth, if it has one. Over a chord of the key, B, D, F
/// and A take the note of the key as many steps above the root: an A over
/// Em in C major is C, not C#, and a B over G is F. Over a chord from
/// outside the key they keep their distance from the root, as in its own
/// major, but a minor chord takes the flat seventh. The other notes are
/// chromatic colours and keep their distance from the root.
```

Las notas del acorde salen de un `match` sobre la clase de altura de la
nota escrita (0 es Do, 4 Mi, 7 Sol, 11 Si) y la séptima propia del
acorde, si la tiene:

```rust
let own_seventh = [10, 11, 9].into_iter().find(|&seventh| has(seventh));
let pc = source.midi() % 12;
let target = match (pc, own_seventh) {
    (0, _) => 0,
    (4, _) => intervals.get(1).copied().unwrap_or(4),
    (7, _) => intervals.get(2).copied().unwrap_or(7),
    (11, Some(seventh)) => seventh,
    (11, None) if has(3) => 10,
    _ => pc,
};
```

`intervals` son las notas del acorde en semitonos sobre su
fundamental, así que `intervals.get(1)` es su segunda nota, la tercera
o una nota suspendida, e `intervals.get(2)` su quinta. `own_seventh`
busca una séptima menor (10), una séptima mayor (11) o una sexta (9),
la nota que tiene en ese lugar un acorde de sexta o una séptima
disminuida. Después, Re, Fa, La y un Si sin séptima que tomar se
cuentan en grados:

```rust
let steps = match (pc, own_seventh) {
    (2, _) => 1,
    (5, _) => 3,
    (9, _) => 5,
    (11, None) => 6,
    _ => return Some(note),
};
```

Si la tonalidad tiene siete notas y contiene todas las del acorde, la
nota va a la nota de la tonalidad que está esos grados por encima de la
fundamental. Si no, se queda donde la puso el primer `match`.

### Guardar sin perder el archivo anterior

Guardar escribe un archivo temporal junto al de verdad y lo renombra:

```rust
let temporary = path.with_extension(format!("{EXTENSION}.tmp"));
```

Un renombrado reemplaza un archivo de un solo paso, así que un cierre
inesperado o un disco lleno mientras se escribe deja el estilo anterior
entero, nunca medio archivo.

## Cómo se prueba

En `library/user_style.rs`, `template_notes_stay_in_the_key` comprueba
la tabla de arriba nota por nota: un Si sobre Am es Sol y sobre D7 Do,
un La sobre Em es Do, un Si sobre Fm es Mi♭ y sobre B♭ La.
`every_builtin_can_be_copied_edited_and_saved` y
`every_band_style_can_be_copied_with_all_its_players` copian cada
estilo y cada banda integrados, validan la copia y la vuelven a leer de
su texto. `a_copy_is_named_and_described_in_the_apps_language`
comprueba «Bossa nova copy» y «Bossa nova (copia)», y
`user_parts_follow_chords_and_survive_a_project_round_trip` aplica una
parte de piano de dos notas sobre Dm y G7, comprueba las terceras que
toca, guarda el proyecto (formato 22), lo vuelve a abrir y cambia los
acordes.

En la app, `style_window_previews_and_saves_the_editable_pattern`,
`discarding_a_draft_leaves_nothing_of_it_to_undo` y
`style_confirmations_block_draft_shortcuts_and_escape_keeps_edits`
manejan la ventana del editor;
`saved_styles_reload_alphabetically_with_their_instruments_and_patterns`
vuelve a leer una carpeta de estilos;
`edit_style_copies_the_whole_band_of_a_new_song` comprueba que
**Editar estilo…** en una canción nueva trae piano, bajo, batería y
guitarra; y `saving_a_style_elsewhere_keeps_the_styles_folder`, el
aviso.
