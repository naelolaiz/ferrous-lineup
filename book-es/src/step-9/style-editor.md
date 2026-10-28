# El editor de estilos: escribir tus propias partes para la banda

Archivos: [`crates/lineup-app/src/style_editor.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/style_editor.rs),
[`crates/lineup-app/src/app/styles.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/styles.rs),
[`crates/lineup-core/src/library/user_style.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/user_style.rs),
[`crates/lineup-core/src/library/apply.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/apply.rs)
y [`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs)

La [biblioteca](../step-8/library.md) es conocimiento escrito como
datos: un estilo es un chart, un patrón de comping, una línea de bajo y
un ritmo, cada uno unas pocas líneas en una tabla. El **editor de
estilos** escribe el groove que esas tablas no tienen, con el channel
rack y el piano roll que conoces.

## Dónde está

- En el **channel rack**, **Edit style…** abre el estilo que toca la
  canción; **New style…** empieza uno vacío.
- **View > Style editor…** abre el editor, o lo trae al frente.
- En la **Library**, haz clic derecho en un estilo o un ritmo. Uno
  integrado dice **Edit a copy in channel rack…**; uno guardado dice
  **Edit in channel rack…**.

**Edit style…** empieza desde el estilo de la canción. Un estilo
guardado se abre como sí mismo, así que **Save style** escribe de
vuelta a su archivo. Un estilo o ritmo integrado se abre como una copia
editable, llamada «… copy». Sin banda, el editor empieza vacío.

## Qué edita

Un estilo de usuario es **un patrón que se repite**, de uno a dieciséis
compases. Puede contener carriles de batería, un carril de piano u otro
carril de acordes, un carril de bajo, y carriles para la guitarra, las
cuerdas, el órgano y el solista que usa una banda más grande. La fila
de arriba tiene **Start from…**, **Save style**, **Save as…**,
**Undo**, **Redo**, **Preview ×4** y **Stop preview**. Debajo están
**Name**, **BPM**, el compás, **Bars**, **Swing**, **Progression**,
**Description** y **Part assignments**.

El rack es el mismo widget, con las filas del borrador: haz clic en los
pasos para escribir la batería. Haz clic en una vista previa de nota
para abrir el piano roll, donde dibujar, mover, estirar y el menú Tools
funcionan como en un patrón. El roll del editor tiene su propio id de
instancia, ventana y teclas, y nunca edita el roll de la canción.

Las partes con altura se escriben **sobre Do mayor**. Cuando se aplica
el estilo, cada nota se mueve al acorde del momento: un Do toca la
fundamental, un Mi la tercera, un Sol la quinta, y las demás notas siguen
la tonalidad o conservan su color. Los campos dicen: «Write pitched
parts over Do major. They follow the song's chords when applied.»

**Preview ×4** toca el patrón cuatro veces y se detiene. **Space** lo
arranca o lo para mientras el editor tiene el teclado. La vista previa
es una frase en el hilo de audio; la canción se pausa. **Stop preview**
la termina antes.

Su historial es propio. **Ctrl+Z**, **Ctrl+Shift+Z** y **Ctrl+Y**
deshacen y rehacen el borrador sin tocar el historial de la canción.
Nada llega a la canción hasta que aplicas el estilo guardado; entonces
aplicarlo es una edición deshacible.

## Estilos de usuario

**Save style** escribe en el archivo del que vino el estilo. **Save
as…** pide uno nuevo. Ambos usan **`.lineup-style`**, un archivo RON
con el nombre del estilo, su métrica, tempo, swing, chart, compases y
carriles, cada carril con su instrumento y sus notas. Si escribes otra
extensión, se sustituye: `my groove.txt` se convierte en `my
groove.lineup-style`.

Los estilos viven en una **carpeta de estilos**, por defecto una
subcarpeta `styles` de la carpeta de datos de la app. **Edit >
Settings… > Library > Your styles > Styles folder…** elige otra. Un
estilo guardado fuera de ella no se lista; el aviso dice que la Library
lista la carpeta, no el archivo que elegiste.

La Library lee cada archivo `.lineup-style` al arrancar, y otra vez
después de un guardado o un cambio de carpeta. Los ordena por nombre y
avisa de un archivo dañado. Guardar escribe un archivo temporal y lo
renombra, así que un cierre inesperado deja el estilo antiguo entero.

Los estilos guardados en esa carpeta se unen a **Presets**, junto a los
estilos y ritmos integrados. No tienen género, así que **Group by
genre** los pone bajo **User styles (N)**; el número del encabezado los
incluye. Un estilo con chart dice **+ chords**; uno sin él dice
**keeps chords**. **Start from…** muestra la misma lista y búsqueda,
con **Empty style**; elegir uno lo copia al borrador.

Aplicar un estilo guardado funciona como cualquier otro preset.
**Playing patterns** trae sus carriles, **Voices / instruments** sus
sonidos, **Time signature**, **Tempo** y **Swing** los suyos, y
**Chord progression** su chart. Deja la progresión sin marcar para
conservar los acordes de la canción. Mira [el capítulo de la
biblioteca](../step-8/library.md#estilos) y [Trabajar más
rápido](workflow.md).

### El estilo viaja con la canción

Cuando se aplica **Playing patterns**, el `UserStyle` entero se guarda
en la banda:

```rust
pub custom_rhythm: Option<Box<crate::library::UserStyle>>,
pub custom_instruments: Option<Box<crate::library::UserStyle>>,
```

El primero guarda los patrones, el segundo los instrumentos y sus
ajustes. El proyecto no necesita acceso al archivo `.lineup-style`:
bórralo, muévelo, o abre la canción en otra computadora, y la banda sigue
tocando. **Format 22** lo introdujo ([el formato de archivo de
proyecto](../appendix-project-format.md)); un proyecto así se escribe
como 22, o más alto si también usa una función posterior. Las versiones
antiguas no pueden leer un proyecto así.

Por eso un estilo puede desaparecer de **Presets** sin romper una
canción que lo usa: la canción conserva su propia copia.

## Cómo funciona

El editor no tiene un segundo rack ni un segundo piano roll. Tiene un
`Project` desechable de un patrón, construido por
`UserStyle::pattern_project`, y lo guarda junto al estilo y al rol de
cada fila:

```rust
struct Draft {
    style: UserStyle,
    project: Project,
    roles: Vec<Player>,
}
```

Luego llama a los mismos widgets que las ventanas de la canción. El
rack devuelve `RackAction`s, el roll devuelve `RollAction`s, y el
editor los aplica a `draft.project` en lugar de a `LineupApp::project`.
Añadir una batería, mover una nota o abrir el roll pasan por el código
que ya lo hace. Cada cambio guarda el `Draft` entero en su propio
`History<Draft>`.

`sync` copia el borrador de vuelta al estilo después de cada fotograma:
métrica, tempo y swing, y luego un `StyleLane` por canal con las notas
de ese canal, recortadas a la longitud del patrón.
`UserStyle::validate` comprueba el resultado antes de escribirlo o
aplicarlo: de uno a dieciséis compases, como mucho 256 pasos, un carril
por músico con altura, y cada nota dentro del patrón.

Aplicar usa los mismos `StyleParts` que un preset integrado
(`library/apply.rs`), así que las casillas de la Library significan lo
mismo para ambos. `Band::apply_user_style` clona primero el proyecto,
valida el estilo, y solo entonces cambia la banda. Guarda el estilo en
`custom_rhythm` o `custom_instruments`, escribe los patrones de la
banda, y deja que `UserStyle::play_lane` repita los compases y mueva
cada altura al acorde que hay debajo.

## Probarlo

1. Ejecuta `cargo run --release` y abre **File > New**. Pulsa **F6**
   para el channel rack y haz clic en **Edit style…**. El editor se
   abre con el estilo actual como borrador, título **Style editor ·
   channel rack**.
2. Abre **Start from…**, escribe `bossa` en **Search presets**, y elige
   un estilo. Prueba **Preview ×4**.
3. Haz clic en unos cuantos pasos de kick y snare. Haz clic en una
   vista previa de nota de piano para abrir el piano roll del editor;
   dibuja una figura de acordes, y luego pulsa **Space**.
4. Haz clic en **Save as…**, nómbralo `My groove`, y guárdalo en la
   carpeta de estilos. Cierra el editor, abre la Library, y encuentra
   **My groove** bajo **Presets** (o **User styles** cuando está
   agrupado).
5. Aplícalo con **Playing patterns** marcado y **Chord progression** sin
   marcar. La canción conserva sus acordes y la banda toca tu patrón.
   Guarda el proyecto, cierra la app, luego aparta el archivo
   `.lineup-style` y abre el proyecto otra vez: la banda lo sigue
   tocando.
