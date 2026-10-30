# Dibujar las ventanas con egui: distribución, zoom, temas, iconos

Archivos: [`crates/lineup-app/src/main.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/main.rs),
[`crates/lineup-app/src/app.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app.rs),
[`crates/lineup-app/src/theme.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/theme.rs),
[`crates/lineup-app/src/widgets.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/widgets.rs),
[`crates/lineup-app/src/icons.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/icons.rs),
[`crates/lineup-app/src/app/view.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/view.rs),
[`crates/lineup-app/src/app/shortcuts.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/shortcuts.rs)
y [`crates/lineup-app/src/app/about.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/about.rs)

La ventana se dibuja con [egui](https://github.com/emilk/egui), una
librería de GUI escrita en Rust, y [eframe](https://docs.rs/eframe), el
framework de egui para ejecutarla como app nativa en Linux, macOS y
Windows. Este capítulo trata de cómo se arma la ventana principal en
cada frame, del aspecto, de cómo las filas de controles caben en
cualquier tamaño de ventana, de los zooms, de las ventanas que flotan
sobre la lista de reproducción y del menú Ayuda. Los editores en sí, el
rack de canales, el piano roll y la lista de reproducción, tienen el
[capítulo siguiente](editors.md).

## Modo inmediato

La mayoría de los toolkits de GUI son de **modo retenido**: creas los
objetos botón una vez, los guardas en un árbol y les adjuntas callbacks.
egui es de **modo inmediato**: en cada frame, llamas a una función que
describe *toda* la interfaz a partir de tus propios datos, y la librería
la dibuja y te dice en qué se ha hecho clic.

```rust
if ui.button("▶ Play").clicked() {
    self.set_playing(true);
}
```

Esa línea dibuja el botón y comprueba si se ha hecho clic en él. No hay
objetos botón que mantener sincronizados con tu estado. El estado son
simplemente los campos de tu struct, y la interfaz es siempre una imagen
de él. Esto encaja bien con Rust, porque no hay callbacks que retengan
referencias a tus datos.

El precio es que un frame no puede volver atrás. Algo dibujado al
principio del frame no puede saber lo que se dibuja después. Varias
piezas de este capítulo (la barra de ayuda, los grupos que se reparten
en filas, el zoom) lo resuelven de la misma manera: usan lo que encontró
el frame *anterior*, de hace una sexagésima de segundo.

## `main.rs`

```rust
fn main() -> eframe::Result {
    // The first command-line argument, if there is one, is a file to open,
    // or what to do instead of opening the window.
    let mut args = std::env::args_os().skip(1);
    let first = args.next();
    match first.as_ref().and_then(|arg| arg.to_str()) {
        Some("--trace") => std::process::exit(trace(args)),
        Some("--demos") => { /* print the demos' names */ return Ok(()); }
        Some("--help" | "-h") => { /* print how to use it */ return Ok(()); }
        _ => {}
    }
    let path = first.map(PathBuf::from);

    let mut viewport = eframe::egui::ViewportBuilder::default()
        .with_title(app::APP_NAME)
        .with_inner_size([1280.0, 800.0])
        .with_min_inner_size([800.0, 520.0])
        .with_icon(window_icon());
    // ...inside a Flatpak, the window takes the sandbox's app id...

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        app::APP_NAME,
        options,
        Box::new(|cc| Ok(Box::new(app::LineupApp::new(cc, path)))),
    )
}
```

- `main` devuelve un `Result`. Si devuelve `Err`, Rust imprime el error
  y sale con un código de fallo.
- `ViewportBuilder::default().with_title(...)` es el **patrón builder**:
  cada método toma el builder y lo devuelve modificado, para que las
  llamadas se encadenen.
- `..Default::default()` es la **sintaxis de actualización de structs**:
  «todo campo que no enumere toma su valor por defecto».
- `Box::new(|cc| ...)` pasa un closure que eframe llama una vez para
  crear nuestra app. `Box` pone un valor en el **montículo**. Aquí hace
  falta porque eframe acepta *cualquier* tipo que implemente su trait
  `App`, y para guardar un valor cuyo tipo exacto no conoce necesita un
  puntero a él: `Box<dyn App>`. La palabra clave `dyn` significa «algún
  tipo que implementa este trait, decidido en tiempo de ejecución» (un
  **trait object**).
- `"--help" | "-h"` es un **patrón «o»**: la rama coincide con
  cualquiera de las dos cadenas.

## `LineupApp`: el estado

`LineupApp` guarda todo lo que necesita la ventana: el proyecto, el
historial de deshacer, las ventanas abiertas y su propio estado, el
archivo, los ajustes y la conexión con el hilo de audio. Esa conexión es
un enum:

```rust
enum Audio {
    /// Boxed: the engine is much bigger than a `String`, and every
    /// `Audio` would otherwise take as much room as the biggest variant.
    Running(Box<AudioEngine>),
    Unavailable(String),
}
```

Captura una situación real: o hay un motor en marcha, o está la razón
por la que no lo hay. No puedes tener ambas cosas, y no puedes no tener
ninguna, así que el tipo no puede representar un estado imposible.
Compáralo con la alternativa habitual, un `Option<AudioEngine>` más un
`Option<String>`, que permite cuatro combinaciones de las cuales dos no
tienen sentido.

Un enum ocupa lo que su variante más grande, más un poco de espacio
para decir cuál es. `Box` deja el motor, que es grande, en el
montículo, así que `Audio` ocupa lo que un puntero o un `String`, lo
que sea mayor. Clippy lo señala con su lint `large_enum_variant`.

Usarlo:

```rust
fn send(&mut self, command: Command) {
    if let Audio::Running(engine) = &mut self.audio {
        engine.send(command);
    }
}
```

`if let` es un `match` de una sola rama: si `self.audio` es `Running`,
liga el motor que hay dentro a `engine` y ejecuta el bloque. Hacer match
sobre `&mut self.audio` toma prestado el motor de forma mutable en lugar
de moverlo fuera del struct.

Si la computadora no tiene una tarjeta de sonido que funcione, la app se
abre igual y la barra de estado dice por qué, en naranja: «Audio no
disponible: …». Un fallo se convierte en datos que mostramos, en lugar
de un cierre inesperado.

## Un frame

eframe llama a `ui` cada vez que la ventana necesita repintarse:

```rust
impl eframe::App for LineupApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if let Audio::Running(engine) = &mut self.audio {
            engine.collect_garbage();
        }
        self.check_trace();
        let ctx = ui.ctx().clone();
        self.before_drawing(&ctx);
        self.panels(ui, &ctx);
        let playing = self.song_position().filter(|_| self.playing);
        let area = self.main_area(ui, &ctx, playing);

        // The channel rack and the piano roll float on top of the playlist.
        let playhead = self.pattern_playhead();
        self.bring_window_to_front(&ctx);
        self.rack_window(&ctx, area, playhead);
        self.piano_roll_window(&ctx, area, playhead);
        // The chord sheet is drawn last, so it opens over the others.
        self.chord_sheet_window(&ctx, area, playing);
        // ...ear training, the lead sheet, the mixer, the shortcuts,
        // About, the style editor, Settings and the dialogs...
        self.unsaved_changes_dialog(&ctx);
        self.update_title(&ctx);
        self.end_frame_for_undo(&ctx);
    }

    fn on_exit(&mut self) {
        self.forget_recovery();
    }
}
```

El orden es el diseño:

1. **`collect_garbage`** libera lo que devolvió el hilo de audio (ver
   [La cola de basura](threads.md#liberar-memoria-en-otro-sitio-la-cola-de-basura)).
2. **`before_drawing`** hace todo lo que debe pasar antes de dibujar
   nada: apunta si un clic está cerrando un menú, pone el idioma y los
   nombres de las notas para el frame, cierra un paso de deshacer al
   pulsar el ratón, escribe la copia de recuperación cuando toca, lee los
   atajos de teclado y las teclas de zoom, lee la entrada MIDI y mira cómo
   van las exportaciones e importaciones que corren en segundo plano.
   Mientras suena, le pide a egui otro frame enseguida
   (`ctx.request_repaint()`), para que el cursor de reproducción se
   mueva; si no, egui solo dibuja cuando pasa algo.
3. **`panels`** dibuja las barras de los bordes: los menús con la barra
   de ayuda, la barra de herramientas, la barra de estado y las teclas de
   armonía. Los paneles deben ir antes del área central, que ocupa el
   espacio que queda.
4. **`main_area`** dibuja la lista de reproducción (o la hoja de
   acordes, cuando llena la ventana principal) y devuelve el rectángulo
   que ocupó.
5. **Las ventanas** flotan sobre ese rectángulo. Las que se dibujan
   después quedan encima, por eso la hoja de acordes va la última de los
   tres editores.
6. **`end_frame_for_undo`** decide si el paso de deshacer termina (ver
   [Deshacer y rehacer](undo.md#cuándo-termina-un-paso)).

`on_exit` se ejecuta una vez cuando la app se cierra bien: no hay
trabajo sin guardar que recuperar, así que la copia de recuperación de
esta ejecución se borra.

Los muchos métodos `self.algo(ui)` mantienen `ui` corto. Cada closure
que se pasa a un panel puede tomar prestado `self` de forma mutable,
porque cada uno se ejecuta y termina antes de que empiece el siguiente,
algo que el borrow checker puede ver.

## Un tema es una función

egui dibuja cada widget a partir de un `Style`: los colores en
`style.visuals`, los tamaños y huecos en `style.spacing`. Nada se pinta
con un color fijo salvo que el código pida uno. Así que todo el aspecto
es una función, `theme::apply`, llamada una vez al arrancar la app:

```rust
pub fn apply(ctx: &egui::Context) {
    ctx.set_theme(egui::Theme::Dark);
    ctx.style_mut_of(egui::Theme::Dark, |style| {
        let visuals = &mut style.visuals;
        visuals.panel_fill = PANEL;
        visuals.window_fill = WINDOW;
        visuals.selection.bg_fill = ACCENT; // selected and switched-on things
        visuals.selection.stroke = Stroke::new(1.0, ON_ACCENT);
        // ...shadows, corners, warning and error colours...
        let widgets = &mut visuals.widgets;
        for (state, fill) in [
            (&mut widgets.inactive, BUTTON),
            (&mut widgets.hovered, BUTTON_HOVER),
            (&mut widgets.active, BUTTON_PRESSED),
            (&mut widgets.open, BUTTON_HOVER),
        ] {
            state.bg_fill = fill;
            state.weak_bg_fill = fill;
            state.bg_stroke = Stroke::new(1.0, OUTLINE);
            // ...
        }
        // ...spacing...
    });
}
```

El aspecto toma prestado de los estudios basados en patrones: paneles
gris azulado oscuro, botones grises en relieve, una luz naranja para lo
que está seleccionado o encendido, y pantallas «LCD» negras para números
como el tempo. Tres cosas que hay que notar en el código:

- `style_mut_of(Theme::Dark, ...)` cambia solo el estilo *oscuro*. egui
  guarda un estilo para oscuro y otro para claro, y la app usa siempre
  el oscuro.
- El bucle `for` recorre un **array de pares de referencias mutables**.
  Cada `&mut widgets.inactive` toma prestado un *campo distinto*, así
  que Rust permite varios a la vez: el borrow checker sigue los campos
  de un struct por separado.
- Los colores son elementos `const`: `Color32::from_rgb` es un
  `const fn`, así que se ejecuta en tiempo de compilación y no cuesta
  nada cuando la app se ejecuta.

Las vistas siguen teniendo sus propias constantes para las cosas que
solo dibujan ellas (las filas de la lista de reproducción, las teclas
del piano roll, los botones de paso), con valores que pegan con el tema.

### Cambiar el estilo de unos pocos widgets

La pantalla de tempo es un `DragValue`, el número de egui que se
arrastra o se escribe, pero dibujado en negro con dígitos verde
azulados. Su estilo se cambia solo dentro de su marco:

```rust
pub fn lcd_frame(
    ui: &mut Ui,
    label: &str,
    width: f32,
    add: impl FnOnce(&mut Ui) -> Response,
) -> Response {
    wrapping(ui, |ui| {
        egui::Frame::new()
            .fill(LCD)
            // ...
            .show(ui, |ui| {
                // ...the label above, written smaller if it is long...
                ui.vertical(|ui| {
                    ui.label(label);
                    let visuals = &mut ui.visuals_mut().widgets;
                    for state in [&mut visuals.inactive, &mut visuals.hovered, &mut visuals.active] {
                        state.bg_fill = LCD;
                        state.fg_stroke.color = LCD_TEXT;
                        // ...
                    }
                    ui.style_mut().override_font_id = Some(FontId::monospace(16.0));
                    add(ui)
                })
                .inner
            })
    })
    .inner
}
```

Cada `Ui` tiene su propia copia del estilo (un `Arc<Style>`; `style_mut`
hace una copia privada la primera vez que se cambia). El contenido del
marco recibe un `Ui` hijo, así que el cambio se queda dentro del marco y
el resto de la barra de herramientas no se toca. `add` es un closure que
*devuelve* el `Response` del widget, así que quien lo llama puede seguir
preguntando `changed()` fuera:

```rust
let tempo = theme::lcd_frame(ui, tr("TEMPO"), 62.0, |ui| {
    ui.add(egui::DragValue::new(&mut bpm).range(Tempo::MIN..=Tempo::MAX))
});
```

El campo de `Tempo` es privado, así que el valor se copia a un `bpm`
local, el widget edita ese, y se construye un `Tempo` nuevo si cambió.
El `Tempo` sigue siendo válido se escriba lo que se escriba.
`Tempo::MIN..=Tempo::MAX` es un **rango inclusivo**: `a..b` deja fuera
`b`, `a..=b` lo incluye.

Una etiqueta más larga que la pantalla, como algunas en español, se
escribe un poco más pequeña para que quepa, así que la barra de
herramientas conserva su ancho en todos los idiomas.

## La barra de ayuda: un mensaje del futuro

Junto a los menús, una línea dice qué hace el control que está bajo el
ratón. Esa línea se dibuja *primero*, arriba del todo de la ventana,
pero los widgets que saben sobre qué está el ratón se dibujan *después*,
en el mismo frame. En modo inmediato no hay vuelta atrás para repintar
la barra.

Así que la barra muestra lo que se encontró en el frame **anterior**.
Los widgets dejan su ayuda en la memoria de egui, `ctx.data`, que es un
mapa de un `Id` a un valor de cualquier tipo:

```rust
pub fn hint(response: Response, text: impl Into<String>) -> Response {
    let text = text.into();
    if response.hovered() {
        response
            .ctx
            .data_mut(|data| data.insert_temp(hint_id(), text.clone()));
    }
    response.on_hover_text(text)
}
```

Al principio del frame siguiente, la barra la saca, para que no se
quede cuando el ratón se va:

```rust
pub fn start_frame(&mut self, ctx: &egui::Context) {
    let text = ctx.data_mut(|data| data.remove_temp::<String>(hint_id()));
    if text != self.text {
        // The mouse moved onto or off a widget: draw once more, so the
        // bar does not lag a frame behind until the next event.
        ctx.request_repaint();
    }
    self.text = text;
}
```

`remove_temp::<String>` usa el **turbofish** `::<String>` para decir qué
tipo sacar: el mapa puede guardar valores de muchos tipos bajo el mismo
`Id`. El `request_repaint` es la parte sutil. egui solo dibuja un frame
cuando pasa algo. Cuando el ratón se pone sobre un botón, ese frame
apunta la ayuda, pero la barra sigue mostrando la antigua. Pedir un
frame más muestra la ayuda nueva enseguida, en lugar de la próxima vez
que se mueva el ratón.

## Filas que caben en cualquier ventana

La barra de herramientas tiene el modo de reproducción, el transporte,
las pantallas de tempo y de posición, el selector de patrones, los cinco
botones de ventana, la cuenta de entrada, el metrónomo y el volumen
maestro. En una ventana de 1280 puntos de ancho cabe en una fila; una
prueba, `the_toolbar_fits_on_one_row_at_1280_points`, comprueba que siga
siendo así. Una ventana más estrecha o con zoom tiene que seguir
llegando a todos los controles.

![La barra de herramientas en una fila: los cinco botones de ventana y
los interruptores Cuenta de entrada y Metrónomo son
dibujos.](../images/step-9-workflow-toolbar.png)

egui coloca los widgets uno detrás de otro. `ui.horizontal` los pone en
una fila y sigue más allá del borde si no caben: lo que queda más allá
del borde sigue estando, solo que no se dibuja. `ui.horizontal_wrapped`
es la misma fila, salvo que un widget que no cabría empieza una fila
nueva. La barra de herramientas y la barra de teclas de armonía son
filas que se reparten, así que en una ventana estrecha su segunda parte
baja.

### Grupos que se reparten: el ancho del frame pasado

Los botones de la barra de herramientas van en grupos, cada uno dentro
de un marco oscuro. Una fila que se reparte decide si pasa a la fila
siguiente preguntándole a un widget cuánto quiere ocupar antes de
colocarlo. Un marco no puede responder a eso: `Frame::show` dibuja su
contenido en el espacio que quede, y solo sabe su tamaño después. Así
que `theme::wrapping` usa el frame anterior. egui vuelve a dibujar la
ventana muchas veces por segundo, así que el ancho de la última vez es
una muy buena estimación:

```rust
let id = ui.next_auto_id().with("wrapping");
let last_width = ui.ctx().data(|data| data.get_temp::<f32>(id)).or(guess);
let row_started = ui.cursor().min.x > ui.max_rect().min.x;
if let Some(width) = last_width
    && ui.layout().main_wrap
    && row_started
    && width > ui.available_size_before_wrap().x
{
    ui.end_row();
}
let response = add(ui);
let width = response.response.rect.width();
ui.ctx().data_mut(|data| data.insert_temp(id, width));
```

- **`ui.next_auto_id()`** es el id que egui le daría al siguiente
  widget. Depende de cuántos widgets vinieron antes en este `Ui`, así
  que es el mismo en cada frame mientras la barra se dibuje en el mismo
  orden. Eso lo convierte en una clave para recordar algo de un grupo.
- **`get_temp::<f32>`** le pide a la memoria de egui un `f32`; el
  turbofish le dice a Rust qué tipo buscar, porque nada más en la línea
  lo dice.
- **La cadena `if let ... && ...`** (una «let chain») solo ejecuta
  `end_row` cuando hay un ancho de la vez anterior, la distribución se
  reparte, la fila ya tiene algo y el grupo no cabe. El primer frame de
  todos todavía no tiene ancho, así que el grupo puede sobresalir
  durante un frame, cosa que nadie ve.

`theme::group` y `lcd_frame` envuelven sus marcos en él, y
`keep_together` ofrece lo mismo a cualquier grupo de widgets, con un
`guess` para el primer frame de todos. Ese primer frame importa dentro
de una ventana: las ventanas de egui crecen para que quepa su
contenido, y **un solo frame de contenido demasiado ancho basta para
ensanchar una ventana para siempre**. La barra de herramientas del piano
roll mantiene juntos sus grupos de esta manera, así que un nombre de
escala largo (el de Fa# mayor, «Fa# Sol# La# Si Do# Re# Mi#») pasa su
grupo a una segunda fila en lugar de ensanchar la ventana.

### Texto que se acorta, y de derecha a izquierda

Repartirse en varias filas les va bien a los botones. Para una línea
larga de texto, como un mensaje de estado, a menudo es mejor cortarla.
Todo texto en egui tiene un *modo de ajuste*: `Wrap` lo parte en más
líneas, `Truncate` lo corta con «…» y `Extend` lo deja seguir. Un área
entera puede ponerlo para todo su texto mediante su estilo:

```rust
ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
```

Una etiqueta truncada muestra el texto entero cuando apuntas a ella, así
que no se pierde nada.

La barra de estado tiene dos partes: mensajes a la izquierda y los
controles de zoom de la lista de reproducción a la derecha. Si los
mensajes se dibujaran primero, uno largo («Audio no disponible: no se
pudieron leer los ajustes del dispositivo…») ocuparía todo el espacio y
empujaría fuera el zoom. Así que la barra se distribuye *de derecha a
izquierda*: el zoom entra primero, desde el borde derecho, y los
mensajes se quedan con lo que sobra:

```rust
fn status_bar(&self, ui: &mut egui::Ui) {
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        view::zoom_control(ui);
        ui.separator();
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
            self.status_messages(ui);
        });
    });
}
```

En una distribución de derecha a izquierda, el primer widget es el que
está más a la derecha, por eso `zoom_control` añade su botón **+** antes
del porcentaje y del botón **−**.

## Zoom

Hay dos tipos de zoom.

El **tamaño de la interfaz** hace todo más grande o más pequeño: texto,
botones, menús. egui dibuja en *puntos*, no en píxeles. Cuántos píxeles
ocupa un punto es `pixels_per_point`: la escala propia de la pantalla
(1.0, 1.5 o 2.0 en la mayoría) multiplicada por el *factor de zoom* de
egui. `ctx.set_zoom_factor` cambia el segundo.

Las **áreas** hacen zoom de lo que dibujan sin cambiar el texto. Como en
la mayoría de los programas de música, la lista de reproducción y el
piano roll hacen zoom en sus dos direcciones por separado: el zoom
horizontal hace más anchos los compases y los pasos (menos tiempo en
pantalla, con más detalle), el vertical hace más altas las pistas y las
teclas del piano. Los pasos del rack de canales son botones cuadrados,
así que tiene un solo zoom para las dos direcciones.

| Puntero sobre | Ctrl y + / − (o rueda) | Alt y + / − (o rueda) | Ctrl+0 |
|---|---|---|---|
| Lista de reproducción | compases más anchos / estrechos | pistas más altas / bajas | las dos al 100% |
| Piano roll | pasos más anchos / estrechos | teclas más altas / bajas | las dos al 100% |
| Rack de canales | más grande / pequeño | lo mismo | 100% |
| Cualquier otra cosa, también la hoja de acordes | interfaz más grande / pequeña | lo mismo | 100% |

### ¿Qué hay bajo el puntero?

Las teclas y la rueda hacen zoom de lo que está bajo el puntero. En
Rust, «una de estas clases» es un `enum`, y una variante puede llevar un
valor:

```rust
pub(crate) enum Target {
    Area(Area), // Area::Main (the playlist), Area::Rack, Area::PianoRoll
    Interface,
}
```

`Target::at` lo encuentra. egui mantiene una pila de *capas*: el fondo,
donde se dibujan los paneles, luego cada ventana encima, luego los menús
y los tooltips. `ctx.layer_id_at(pos)` dice qué capa está arriba en un
punto:

```rust
fn at(ctx: &egui::Context, pos: egui::Pos2, playlist: egui::Rect) -> Target {
    match ctx.layer_id_at(pos) {
        Some(layer) if layer.id == super::rack_id() => Target::Area(Area::Rack),
        Some(layer) if layer.id == PianoRoll::id() => Target::Area(Area::PianoRoll),
        Some(layer) if layer.order == egui::Order::Background && playlist.contains(pos) => {
            Target::Area(Area::Main)
        }
        _ => Target::Interface,
    }
}
```

Cada rama tiene una **guarda**, el `if` después del patrón. El patrón
`Some(layer)` coincide con cualquier capa, y la rama solo se toma si la
guarda también es verdadera; si no, `match` prueba la rama siguiente. La
última rama, `_`, recoge todo lo que queda.

Las barras de herramientas, las teclas de armonía y la lista de
reproducción se dibujan todas en la misma capa de fondo, así que la capa
sola no puede distinguirlas. Por eso la lista de reproducción recuerda
dónde se dibujó, y la guarda comprueba `playlist.contains(pos)`. Las
teclas de zoom se leen al principio de un frame, antes de dibujar nada,
así que este es el rectángulo del frame anterior. Si el puntero está
fuera de la ventana, las teclas hacen zoom de lo último en que se hizo
clic.

La hoja de acordes es texto, así que sigue el tamaño de la interfaz, en
su propia ventana o en el lugar de la lista de reproducción. En ese
lugar tapa la lista de reproducción, y el último rectángulo de la lista
mentiría, así que `zoom_shortcuts` pasa uno vacío en su lugar:

```rust
let playlist = match self.chord_sheet.fills_window() {
    true => egui::Rect::NOTHING,
    false => self.playlist.area(),
};
```

`Rect::NOTHING` no contiene ningún punto, así que la guarda nunca es
verdadera, y las teclas caen en `Target::Interface`.

Todas las maneras de hacer zoom (teclas, rueda, pellizco en el touchpad)
llaman a los mismos pocos métodos de `Target`, y solo ellos miran de qué
clase es:

```rust
fn step(self, ctx: &egui::Context, axis: Axis, bigger: bool) {
    match self {
        Target::Area(area) => area.step_zoom(ctx, axis, bigger),
        Target::Interface => {
            ctx.set_zoom_factor(interface_step(ctx.zoom_factor(), bigger));
        }
    }
}
```

`Target::Area(area)` comprueba la variante y a la vez saca de ella el
`Area` a la variable `area`. La interfaz tiene un solo tamaño, así que
ahí `axis` no se usa: Ctrl y Alt hacen lo mismo.

### Un zoom por área y por eje

Cada área guarda un número por dirección, en la memoria de egui, bajo
una clave hecha de las dos cosas:

```rust
fn key(self, axis: Axis) -> egui::Id {
    let axis = if self.two_way() { axis } else { Axis::Horizontal };
    egui::Id::unique(("zoom-2", self, axis))
}
```

El rack no es `two_way`, así que sus dos ejes comparten la clave
horizontal: hacerle zoom «vertical» le hace zoom en las dos direcciones.
Los valores se guardan con `insert_persisted`, así que egui los guarda
con las posiciones de las ventanas y son los mismos la próxima vez que
arranca la app. El nombre `"zoom-2"` está porque un significado anterior
de «zoom» (una escala para todo lo de la ventana, texto incluido) se
guardaba bajo `"zoom"`: un nombre nuevo hace que los valores viejos
simplemente no se encuentren, en lugar de leerse de la manera
equivocada.

Todas las maneras de hacer zoom por pasos pasan por `step_zoom`, que
recorre una escalera de tamaños:

```rust
const STEPS: [f32; 19] = [
    0.25, 0.33, 0.4, 0.5, 0.6, 0.75, 0.9, 1.0, 1.1, 1.25, 1.5, 1.75, 2.0, 2.5, 3.0, 4.0, 5.0, 6.0,
    8.0,
];
```

Cada paso es más o menos una quinta parte mayor que el anterior, así
que cuesta tantos pasos ir de 100% a 200% como de 400% a 800%. Así es
como el zoom se siente natural: los ojos comparan tamaños por
proporción, no por diferencia. Cada área y eje tiene su propio tramo
dentro de la escalera (las pistas y las teclas solo pueden achicarse
hasta que sus nombres dejan de caber), y los botones se ponen grises en
los extremos. Los tamaños de la interfaz son una lista más corta propia,
del 50% al 250%, los mismos que ofrece **Ajustes > Apariencia**.

### Leer la rueda nosotros mismos

egui da un único `zoom_delta()` para Ctrl+rueda, sin idea de dirección.
Así que la app lee los eventos de rueda en bruto del frame y los
clasifica según las teclas modificadoras pulsadas:

```rust
for event in &input.raw.events {
    if let egui::Event::MouseWheel { unit, delta, modifiers, .. } = event {
        // Up or right zooms in. Shift turns a wheel sideways.
        let amount = if delta.y != 0.0 { delta.y } else { delta.x };
        if modifiers.alt {
            tall.add(*unit, amount);
        } else if modifiers.command {
            wide.add(*unit, amount);
        }
    }
}
```

`if let` con un patrón es como Rust elige una variante de un enum y
nombra sus campos al mismo tiempo; `..` ignora los campos que no
necesitamos.

En Linux (X11), un golpe de rueda a veces llega como dos eventos, y
hacer zoom de dos pasos por golpe se siente a saltos. Así que los golpes
se suman y hacen zoom de un paso como mucho por frame:

```rust
fn apply(self, target: Target, ctx: &egui::Context, axis: Axis) {
    if self.notches != 0.0 {
        target.step(ctx, axis, self.notches > 0.0);
    }
    if self.smooth != 1.0 {
        target.scale(ctx, axis, self.smooth);
    }
}
```

Un frame dura unos pocos milisegundos, así que girar la rueda rápido
sigue haciendo zoom paso a paso. Un touchpad manda en cambio muchos
movimientos diminutos en lugar de golpes (`MouseWheelUnit::Point`);
esos multiplican el zoom de forma suave.

### Hacer zoom sobre lo que señalas

Cuando el zoom cambia, el paso y la nota que están bajo el puntero deben
seguir bajo el puntero. La cuadrícula está dentro de un scroll area, así
que eso significa elegir un desplazamiento nuevo. Lo que está bajo el
puntero está en `from + anchor` dentro del contenido, donde `from` es
cuánto se ha desplazado la vista y `anchor` dónde está el puntero dentro
de la vista. Hacer zoom por `ratio` lo mueve a `(from + anchor) * ratio`.
Para que se quede bajo el puntero, el desplazamiento nuevo debe ser eso
menos `anchor`. En el piano roll:

```rust
let x = KEY_WIDTH + (from.x + anchor.x - KEY_WIDTH) * ratio.x - anchor.x;
let y = (from.y + anchor.y) * ratio.y - anchor.y;
self.scroll_to = Some(vec2(x.max(0.0), y.max(0.0)));
```

Los términos `KEY_WIDTH` están porque el teclado de piano de la
izquierda no se estira: solo crece lo que está a su derecha.

Hubo que apagar una cosa más. Los scroll areas de egui tienen una
**banda elástica**: te desplazas más allá del final y la vista vuelve de
un salto. En el frame de un zoom, el scroll area todavía conoce el
tamaño *viejo* del contenido, así que el desplazamiento nuevo parece ir
más allá del final, y la vista se deslizaría de vuelta después de cada
zoom. `widgets::no_rubber_band` la apaga para las cuadrículas:

```rust
pub fn no_rubber_band(ui: &mut egui::Ui) {
    ui.spacing_mut().scroll.kinetic.rubber_band = false;
}
```

![El piano roll con zoom más ancho y más alto, desplazado hacia un lado.
El teclado se queda a la izquierda, y los carriles de acordes y de
velocidad siguen alineados con la
cuadrícula.](../images/step-8-two-way-zoom.png)

El rack de canales se dibuja con widgets normales de egui, que no tienen
zoom propio. `widgets::zoom_contents` los hace más grandes multiplicando
los tamaños del texto y los espacios del estilo de la ventana, igual que
`lcd_frame` cambia sus colores: solo dentro del rack.

## Ventanas sobre la lista de reproducción

El rack de canales, el piano roll, la hoja de acordes, el mezclador y
las demás son `egui::Window`, que flotan sobre todo. Se mantienen dentro
del rectángulo de la lista de reproducción con
`Window::constrain_to(area)`, así que una ventana nunca tapa la barra de
herramientas, las teclas de armonía ni la barra de estado:

```rust
let playlist = egui::CentralPanel::default_margins().show(ui, |ui| { /* ... */ });
let area = playlist.response.rect;
```

Cada `show` de egui devuelve un `InnerResponse`: `.inner` es lo que
devolvió el closure (aquí las acciones de la lista de reproducción) y
`.response` dice dónde se dibujó.

### Con qué ventanas se abre una canción

Lo que estudias en una canción son sus compases y acordes; lo que editas
en un ritmo son los pasos de un patrón. Así que la app decide qué
mostrar según lo que se abrió:

```rust
/// What a song opens on: the chord sheet in front, since its bars and
/// chords are what you study and write in a song, and the rack out of
/// the way (F6 brings it back).
fn show_chord_sheet(&mut self) {
    self.chord_sheet.open = true;
    self.rack_open = false;
    self.bring_to_front = Some(chord_sheet::window_id());
}

/// What a beat opens on: the rack in front, and no chord sheet.
fn show_beat(&mut self) {
    self.chord_sheet.open = false;
    self.show_rack();
}
```

Las demostraciones de canciones, **Archivo > Nuevo** y una hoja de
acordes importada llaman a `show_chord_sheet`; **Archivo > Nuevo ritmo**
llama a `show_beat`. Abrir un archivo guardado deja las ventanas como
están, porque las elegiste tú. Una demostración es una canción o un
estilo, un `enum`, y solo las canciones se abren en la hoja:

```rust
if let Demo::Song(_) = demo {
    self.show_chord_sheet();
}
```

La aplicación misma se abre con las ventanas que estaban abiertas la
última vez que se cerró. `app/windows.rs` las guarda como un struct
pequeño de `bool`, uno por cada ventana del menú Ver:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub(super) struct OpenWindows {
    chord_sheet: bool,
    lead_sheet: bool,
    rack: bool,
    piano_roll: bool,
    harmony_keys: bool,
    /// The Patterns and Library panel.
    browser: bool,
    mixer: bool,
    ear_training: bool,
}
```

Su `Default` es la primera vez: la hoja de acordes, las teclas de
armonía y la Biblioteca sobre la lista de reproducción, sin rack.
`#[serde(default)]` en el struct rellena una ventana añadida más tarde a
la lista con su valor en `Default`, así que una copia guardada más
antigua se sigue leyendo. El editor de estilos y los diálogos quedan
fuera: cada uno se abre para algo, un estilo o una pregunta, que la
siguiente ejecución no tiene.

`App::save`, a la que eframe llama cada 30 segundos y cuando se cierra
la ventana, escribe el struct entre las preferencias guardadas por
nombre (ver [Guardado por nombre, no por
tipo](#guardado-por-nombre-no-por-tipo)), y `LineupApp::new` lo vuelve
a leer:

```rust
fn save(&mut self, storage: &mut dyn eframe::Storage) {
    self.preferences.put(windows::NAME, &self.open_windows());
    self.preferences.save(storage);
}
```

```rust
let windows: OpenWindows = preferences::get(ctx, NAME).unwrap_or_default();
```

Escribirlas solo al guardar, en lugar de a cada clic en un botón de
ventana, libra de esa tarea a los muchos lugares que abren y cierran
ventanas.
`the_app_first_opens_on_the_chord_sheet_then_with_the_windows_left_open`
comprueba las dos ejecuciones: la primera se abre en la hoja, y después
de cerrarla con F5 y abrir el mezclador con F9, el siguiente arranque
encuentra el mezclador abierto y la hoja cerrada.

### Ventanas del tamaño de lo que muestran

egui recuerda el tamaño de cada ventana y lo guarda con el resto de su
memoria cuando la app se cierra. Eso está bien para una ventana en la
que trabajas, como el piano roll: la hiciste grande porque la querías
grande. Está mal para una ventana que es tan grande como lo que muestra:
arrastra el mezclador a lo ancho una vez, y desde entonces se abriría por
toda la lista de reproducción, con sus canales en una esquina. Así que
hay dos tipos de ventana:

```rust
pub enum Opening {
    /// Each time it opens: a window as big as what it shows.
    EveryTime,
    /// The first time it opens after the app starts: a window to work
    /// in, which keeps the size you give it while the app runs.
    FirstTime,
}
```

El mezclador, la lista de atajos, el entrenamiento auditivo, la serie
dodecafónica, los conjuntos de clases de alturas y los micromodos se
abren **cada vez** al tamaño de su contenido, hasta la altura de la
lista de reproducción, y hacen scroll más allá. El piano roll, la hoja
de acordes, la partitura guía y Ajustes se abren a su propio tamaño la
**primera vez** después de que la app arranca, y conservan el tamaño que
les des mientras se ejecuta.

`widgets::open_at` lo hace. Averigua si la ventana se está abriendo
ahora (no se dibujó en el último frame) y, si es así, le da un tamaño
fijo para ese único frame:

```rust
let first = ctx.data_mut(|data| {
    !std::mem::replace(data.get_temp_mut_or(id.with("has-opened"), false), true)
});
if opens(ctx, id) && (first || opening == Opening::EveryTime) {
    ctx.request_discard("a window opens at its own size");
    window.fixed_size(size)
} else {
    window
}
```

Tres cosas en él merecen una mirada:

- `std::mem::replace(place, new)` pone `new` en `place` y devuelve lo
  que había. Aquí lee «¿se había abierto antes esta ventana?» y la marca
  como abierta, en un solo paso. La bandera se guarda con
  `get_temp_mut_or`, que egui no guarda, así que vuelve a ser `false`
  cada vez que la app arranca.
- egui acomoda una ventana a medida que la dibuja, así que en su primer
  frame la ventana se dibuja antes de que nada sepa cuán grande es su
  contenido. `request_discard` le pide a egui que tire ese frame y lo
  dibuje otra vez; solo ves el segundo, ya al tamaño correcto.
- El «tamaño» dado es un ancho estrecho y la mayor altura. egui agranda
  una ventana para que quepa un contenido más ancho, pero nunca la hace
  más alta que él, así que la ventana acaba exactamente tan grande como
  lo que muestra. Para que eso funcione, nada en ella puede llenar todo
  el espacio que se le da, como un scroll area que no se encoge hasta su
  contenido.

Una ventana cuyo contenido cambia mientras está abierta, como el
mezclador cuando se añade un canal o el rack cuando otro patrón tiene
menos canales, se ajusta otra vez con `fit_when_changed`. Calcula el
hash de un resumen del contenido (el nombre y el instrumento de cada
canal, por ejemplo) y ajusta la ventana otra vez cuando el hash cambia.

## Clics que hacen lo que querías decir

Dos ayudantes de `widgets.rs` hacen que las cuadrículas respondan al
clic que querías hacer.

**Un toque rápido.** En un touchpad, un toque puede pulsar y soltar el
botón dentro de un mismo frame. El `press_origin()` de egui se olvida en
cuanto el botón está arriba, así que en ese frame es `None`. `press_pos`
lee en cambio la pulsación de la lista de eventos del frame, que además
dice *dónde* bajó el botón, no dónde está el puntero al final del frame:

```rust
pub fn press_pos(
    input: &egui::InputState,
    only: Option<egui::PointerButton>,
) -> Option<egui::Pos2> {
    input.events.iter().find_map(|event| match *event {
        egui::Event::PointerButton { pos, button, pressed: true, .. }
            if only.is_none_or(|only| only == button) => Some(pos),
        _ => None,
    })
}
```

`find_map` recorre los eventos hasta que el closure devuelve `Some`, y
devuelve eso. `Option::is_none_or` se lee bien aquí: «no se pidió ningún
botón, o es este».

**Un clic que cierra un menú.** Abre una lista desplegable y haz clic en
la cuadrícula para cerrarla: egui cierra el menú, pero el clic también
llega a la cuadrícula, que dibujaría una nota, colocaría un clip o
encendería un paso. En la mayoría de los programas ese clic solo cierra
el menú. Así que al principio de cada frame, antes de dibujar nada y
mientras el menú que se va a cerrar sigue abierto, la app recuerda si el
botón que ahora está abajo bajó mientras había un menú abierto:

```rust
pub fn track_dismissing(ctx: &egui::Context) {
    let id = dismissing_id();
    let (pressed, down, released) = ctx.input(|input| {
        (
            press_pos(input, None).is_some(),
            input.pointer.any_down(),
            input.pointer.any_released(),
        )
    });
    let open = egui::Popup::is_any_open(ctx);
    ctx.data_mut(|data| {
        let dismissing = data.get_temp_mut_or(id, false);
        if pressed {
            *dismissing = open;
        } else if !down && !released {
            *dismissing = false;
        }
    });
}
```

Las cuadrículas preguntan `dismissing(ctx)` e ignoran la pulsación, el
arrastre y la soltada que le pertenecen. El valor vive en la memoria
temporal de egui, un lugar cómodo para un poco de estado compartido
entre widgets que no se conocen entre sí.

## Recordar entre ejecuciones

egui guarda algo de estado propio entre frames en su *memoria*: el
factor de zoom, dónde está cada ventana, cuán ancho es un panel que se
puede redimensionar. eframe puede guardar esa memoria cuando la app se
cierra y cargarla cuando arranca. Solo lo hace con su feature
`persistence`, que viene apagada por defecto porque trae `serde` y
`ron`. Encenderla es una línea en el `Cargo.toml` del workspace:

```toml
eframe = { version = "0.37", features = ["persistence"] }
```

Con ella, eframe recuerda también el tamaño y la posición de la ventana
principal. Escribe un archivo llamado `app.ron` en el lugar donde cada
sistema guarda los datos de las apps (en Linux,
`~/.local/share/ferrouslineup/`), cada 30 segundos y cuando se cierra la
ventana.

La app guarda la mayoría de sus propias elecciones duraderas en la
misma memoria, con `insert_persisted` en lugar de `insert_temp`: los
zooms de las áreas, los ajustes, los archivos recientes, los filtros de
la biblioteca. Cada módulo guarda lo suyo bajo un `Id` propio, y eframe
los guarda todos juntos. Lo que se guarda con `insert_temp` se olvida
al cerrar la app.

### Guardado por nombre, no por tipo

egui encuentra un valor en su memoria por dos cosas: el `Id` bajo el que
se guardó y su tipo de Rust, a través del `TypeId` del tipo. Un `TypeId`
es un número que el compilador calcula a partir de la ruta del tipo, sus
campos y el crate en el que está, y la parte del crate cambia con la
versión del crate y con la versión del compilador. Así que después de
una actualización de la app, o de compilar con un Rust más nuevo, o de
añadir un campo al tipo, `get_persisted` busca bajo un número nuevo y no
encuentra nada: el ajuste vuelve a su valor por defecto, y el valor
viejo queda sin leer en `app.ron`. El SoundFont lo mostró: con el mismo
`app.ron`, la versión 0.4.0 tocaba con GeneralUser GS y una compilación
numerada 0.4.1 lo tenía desactivado.

eframe tiene un almacenamiento propio, el mismo `app.ron`, que encuentra
los valores por un nombre: un `&str` que no cambia sea cual sea la
versión. eframe se lo da a la app solo dos veces: como `cc.storage`
cuando la app arranca, y a `App::save`, cada 30 segundos y al cerrar la
ventana. El módulo `app/preferences.rs` une las dos cosas. Guarda cada
preferencia por nombre, escrita como texto RON, en un único mapa:

```rust
/// Each preference by name, written as RON. egui's context holds one
/// handle to the map and the app another, to save it.
#[derive(Clone, Default)]
pub(super) struct Preferences(Arc<Mutex<BTreeMap<String, String>>>);
```

Un acceso al mapa vive en la memoria temporal de egui, así que cualquier
código con el `Context` llega a él, como a cualquier otro valor guardado
ahí:

```rust
/// The preference kept as `name`, if there is one and it can be read.
pub(super) fn get<T: DeserializeOwned>(ctx: &egui::Context, name: &str) -> Option<T> {
    let text = Preferences::of(ctx).map().get(name).cloned()?;
    ron::from_str(&text).ok()
}

/// Keeps `value` as the preference `name`.
pub(super) fn set<T: Serialize>(ctx: &egui::Context, name: &str, value: &T) {
    if let Ok(text) = ron::to_string(value) {
        Preferences::of(ctx).map().insert(name.to_string(), text);
    }
}
```

La app lee el mapa del almacenamiento de eframe antes de crear nada más,

```rust
if let Some(storage) = cc.storage {
    preferences::Preferences::of(&cc.egui_ctx).load(storage);
}
```

guarda otro acceso a él, y se lo devuelve a eframe para que lo escriba:

```rust
fn save(&mut self, storage: &mut dyn eframe::Storage) {
    self.preferences.save(storage);
}
```

El `Arc<Mutex<…>>` deja que los dos accesos compartan un mismo mapa. El
bloqueo se mantiene solo mientras se lee o se escribe una línea, y solo
lo toma el hilo de la ventana. Como los valores son texto RON hasta que
se piden, un tipo puede cambiar sus campos y seguir leyendo lo que
guardó una versión anterior, con `#[serde(default)]` para los campos
que todavía no existían.

Los ajustes de sonido ([SoundFonts y General
MIDI](../sound/soundfonts.md)) se guardan así, como `"sound-font"`, y
también las ventanas abiertas, como `"open-windows"`. Las
demás elecciones de arriba siguen en la memoria de egui, así que una
actualización puede olvidarlas; pasar una al nuevo sistema es cuestión
de llamar a `preferences::get` y `preferences::set` con un nombre en
lugar de `get_persisted` e `insert_persisted` con un `Id`.

### Features de Cargo

Una *feature* es un interruptor que ofrece un crate para encender código
opcional. Dentro de eframe, el código que guarda la memoria está detrás
de `#[cfg(feature = "persistence")]`, así que sin la feature ni siquiera
se compila. Encender una feature en un crate puede encender features en
otros: la `persistence` de eframe enciende la de egui, que añade derives
de `serde` a los tipos que egui guarda en su memoria. Puedes ver lo que
ofrece un crate en la tabla `[features]` de su `Cargo.toml`, o en su
página de docs.rs.

## Dibujos pintados, no escritos

Los cinco botones de ventana de la barra de herramientas y los
interruptores de cuenta de entrada y metrónomo son botones cuadrados con
un dibujo; el nombre, lo que muestra la ventana y su tecla aparecen
cuando el ratón está encima. Un emoji sería la manera fácil de dibujar
algo así, pero un emoji es un carácter, dibujado por una fuente, y
algunos sistemas no tienen ninguna fuente con esos caracteres: allí los
botones mostrarían cajas vacías. Así que los dibujos de `icons.rs` están
**pintados**, con líneas, rectángulos y polígonos, de la misma manera en
todos los sistemas.

Cada dibujo es una función sencilla con la misma firma, y Rust puede
nombrar esa firma como un tipo:

```rust
/// A picture: drawn by a function of the painter, the square it fills
/// and the colour to draw it in.
pub type Picture = fn(&Painter, Rect, Color32);
```

`fn(&Painter, Rect, Color32)` es un tipo **puntero a función**.
Cualquier función con esa firma puede guardarse en una variable de tipo
`Picture`, ponerse en un array o pasarse a otra función, sin un `Box` ni
un genérico.

Cada dibujo se pinta en un cuadrado, y sus puntos se dan como fracciones
del cuadrado, de 0 a 1, para que el mismo dibujo pueda pintarse a
cualquier tamaño:

```rust
/// The point `x`, `y` of the way across and down `rect`, each from 0 to 1.
fn at(rect: Rect, x: f32, y: f32) -> Pos2 {
    rect.min + vec2(x * rect.width(), y * rect.height())
}

/// The mixer: three faders, each at its own level.
pub fn mixer(painter: &Painter, rect: Rect, color: Color32) {
    for (x, level) in [(0.2, 0.62), (0.5, 0.3), (0.8, 0.5)] {
        line(painter, rect, (x, 0.0), (x, 1.0), color.gamma_multiply(0.6));
        let knob = Rect::from_center_size(at(rect, x, level), vec2(6.0, 3.5));
        painter.rect_filled(knob, 1.0, color);
    }
}
```

La hoja de acordes es una página con nombres de acordes a dos por línea,
el rack tres filas de pasos con algunos encendidos, el piano roll un
teclado con tres notas, las teclas de armonía un teclado con dos teclas
negras, el metrónomo su cuerpo y su brazo.

La barra de herramientas dibuja sus cinco botones de ventana a partir de
un array. Cada entrada guarda el nombre, el `bool` que dice si esa
ventana está abierta, el dibujo y la ayuda:

```rust
let windows: [(&str, &mut bool, icons::Picture, &str); 5] = [
    ("Chord sheet", &mut self.chord_sheet.open, icons::chord_sheet,
     "the song's harmony bar by bar, to read and write (F5)"),
    ("Channel rack", &mut self.rack_open, icons::rack,
     "the steps of the selected pattern (F6)"),
    // … the piano roll, the harmony keys, the mixer
];
for (name, open, picture, about) in windows {
    let button = icons::toggle(ui, name, *open, picture);
    // ...the hint, and a click opens or closes the window...
}
```

Hay cinco préstamos `&mut` de `self` en un array. El borrow checker lo
permite porque cada uno toma prestado un **campo distinto**:
`self.chord_sheet.open`, `self.rack_open`, `self.piano_roll.open` y así.
Dos `&mut` al mismo campo no compilarían.

La prueba `every_picture_stays_in_its_square` pinta cada dibujo en un
cuadrado de 16 puntos y comprueba que ninguna forma se sale de él, y
`the_window_buttons_open_and_close_their_windows` hace clic en cada uno
de los cinco y comprueba que su ventana se abre, que el botón se
enciende y que un segundo clic la cierra.

## Todas las teclas en una lista

**Ayuda > Atajos de teclado**, o **F1**, abre una ventana con cada tecla
a la que responde la app, sección por sección.

![Ayuda > Atajos de teclado: cada sección dice cuándo funcionan sus
teclas.](../images/step-9-workflow-shortcuts.png)

La lista son datos: un `Vec` de secciones, cada una con un título, una
nota sobre cuándo funcionan sus teclas y sus líneas. Las teclas de una
línea son de una de cuatro clases:

```rust
enum Keys {
    /// Any of these does it.
    Any(Vec<KeyboardShortcut>),
    /// The first does the first half of what the line says, the second the
    /// second: "Up / down".
    Pair(KeyboardShortcut, KeyboardShortcut),
    /// The mouse wheel, turned while holding these.
    Wheel(Modifiers),
    /// Keys that are not one shortcut, written out.
    Written(&'static str),
}
```

Donde la app tiene una constante para un atajo, la lista usa esa misma
constante: `files::SAVE`, `edit::UNDO`, `piano_roll::OCTAVE_UP`,
`view::MIXER`. Así, si una tecla cambia, la lista cambia con ella.
Convertir unas `Keys` en texto usa el `format_shortcut` de egui, que
escribe «Ctrl+Shift+Z» en Linux y Windows y «⇧⌘Z» en un Mac, como lo
escriben los menús.

Como la lista son datos, también se puede probar como datos.
`no_two_global_shortcuts_share_a_key` reúne las teclas de las secciones
que funcionan en cualquier sitio y falla si alguna tecla aparece dos
veces, porque esa tecla haría dos cosas a la vez.

## Ayuda: el manual y Acerca de

**Ayuda > Manual de usuario** abre las páginas web del manual de usuario
en el idioma de la app. La dirección vive en una función de `about.rs`,
así que el menú Ayuda y la ventana Acerca de se la piden a ella:

```rust
pub(super) fn manual() -> &'static str {
    match i18n::current_language() {
        Language::English => MANUAL,
        Language::Spanish => MANUAL_ES,
    }
}
```

El `match` no tiene rama `_` a propósito. Añade un tercer idioma, y el
compilador se detiene aquí hasta que decidas qué manual reciben sus
lectores. `pub(super)` hace que `manual` se vea desde `app.rs` y desde
los módulos que tiene dentro, como `shortcuts.rs`, donde está el menú
Ayuda, y desde ningún otro lado.

En el menú, el botón le pide a egui que abra esa dirección en una
pestaña nueva del navegador:

```rust
if ui.button(crate::i18n::tr("User manual")).clicked() {
    ui.ctx()
        .open_url(egui::OpenUrl::new_tab(super::about::manual()));
    ui.close();
}
```

**Ayuda > Acerca de Ferrous Lineup** dice qué versión estás usando, con
qué licencia, y dónde encontrar el manual de usuario, este libro (cada
uno también en PDF), el código y la lista de problemas conocidos. Es lo
primero que hay que mirar antes de informar de uno.

![Ayuda > Acerca de Ferrous Lineup: la versión, la licencia y los
enlaces.](../images/step-9-about.png)

Todo lo que dice sobre el programa sale de `Cargo.toml`, leído mientras
se compila el programa:

```rust
const ISSUES: &str = concat!(env!("CARGO_PKG_REPOSITORY"), "/issues");

ui.label(format!("{} {}", tr("Version"), env!("CARGO_PKG_VERSION")));
ui.weak(tr(env!("CARGO_PKG_DESCRIPTION")));
```

- `env!("NOMBRE")` es una macro que el compilador sustituye por el valor
  de la variable de entorno `NOMBRE` *mientras compila*, como un
  `&'static str`. Cargo pone `CARGO_PKG_VERSION`,
  `CARGO_PKG_DESCRIPTION`, `CARGO_PKG_REPOSITORY` y algunas más a partir
  de la sección `[package]` de `Cargo.toml` (la app toma su versión, su
  licencia y su repositorio del `[workspace.package]` del workspace).
  Una variable que no existe es un error de compilación, no una sorpresa
  para el usuario.
- `concat!` junta literales en uno solo en tiempo de compilación, así
  que `ISSUES` también es una constante. Ahí no se podría usar
  `format!`: construye un `String` mientras el programa se ejecuta, y una
  `const` tiene que conocerse antes.

Así, cuando un lanzamiento cambia la versión en `Cargo.toml`, la ventana
cambia con ella, y nadie tiene que acordarse de un segundo sitio. Las
[compilaciones de lanzamiento](../shipping/releases.md) comparan la
etiqueta con este mismo número.

El dibujo es el mismo que el escritorio muestra para la ventana de la
app: `WINDOW_ICON`, 256 × 256 píxeles de bytes rojo, verde, azul y alfa,
incluidos en el programa con `include_bytes!`. egui dibuja las imágenes
a partir de **texturas**, imágenes guardadas en la tarjeta gráfica, así
que los bytes se convierten en una textura la primera vez que se abre la
ventana y se guardan en la memoria de egui:

```rust
fn icon(ctx: &egui::Context) -> egui::TextureHandle {
    let id = egui::Id::unique("about-icon");
    if let Some(texture) = ctx.data(|data| data.get_temp(id)) {
        return texture;
    }
    let size = crate::WINDOW_ICON_SIZE as usize;
    let image = egui::ColorImage::from_rgba_unmultiplied([size, size], crate::WINDOW_ICON);
    let texture = ctx.load_texture("about-icon", image, egui::TextureOptions::LINEAR);
    ctx.data_mut(|data| data.insert_temp(id, texture.clone()));
    texture
}
```

Un `TextureHandle` funciona como un `Arc`: un clon es barato, y la
textura sigue en la tarjeta gráfica mientras viva algún clon. El que se
guarda en la memoria de egui hace que la imagen se envíe a la tarjeta
una sola vez, no en cada frame. `WINDOW_ICON` es una constante de
`main.rs`, sin `pub`; `about.rs` puede leerla igualmente como
`crate::WINDOW_ICON`, porque un elemento privado es visible en su propio
módulo y en todos los módulos que hay dentro de él.

`ui.hyperlink_to(texto, url)` dibuja un enlace. Al pulsarlo no se abre
el navegador directamente: egui añade un `OutputCommand::OpenUrl` a lo
que devuelve el frame, y eframe pide al sistema que abra la dirección.
Eso hace que el clic sea fácil de probar.
`help_about_shows_the_version_the_license_and_where_the_book_is` abre
**Ayuda > Acerca de Ferrous Lineup**, comprueba que la línea de la
versión y cada enlace se dibujan, pulsa **El libro** y comprueba que el
frame pidió la dirección del libro, sin que se abra ningún navegador.
`help_user_manual_opens_the_manual_in_the_apps_language` hace lo mismo
con **Ayuda > Manual de usuario**, en inglés y en español.

## Probar una ventana sin pantalla

Todas las pruebas nombradas en este capítulo dibujan la app sin
pantalla. `egui::Context::run_ui` (o `run_pass`) toma la entrada de un
frame, la posición del ratón, las teclas y los clics, y devuelve todo lo
que egui le pasaría a la tarjeta gráfica, sin dibujarlo. Una prueba
puede entonces encontrar un widget por su `Id` o su texto, comprobar
dónde quedó, hacer clic en él en el frame siguiente y leer el estado de
la app. `app/ux_tests.rs` tiene la mayoría;
[Cómo se prueba el código](../shipping/testing.md) cuenta el resto.

Hay un detalle de estas pruebas que conviene conocer. Lo que devuelve
`run_ui` incluye la textura de la fuente. egui comprueba, solo en las
compilaciones de depuración, que una textura así no se descarta sin
usarse, y si pasa, entra en pánico. Así que las pruebas que se quedan
con la salida la vacían antes:

```rust
// The font texture is not drawn anywhere here.
output.textures_delta.clear();
```

Una prueba que pasa en una compilación de release y falla en una de
depuración suele ser una de estas comprobaciones: `debug_assert!` solo
se compila en las compilaciones de depuración, y una librería puede
tener muchas. Por eso la CI ejecuta las pruebas en una compilación de
depuración.
