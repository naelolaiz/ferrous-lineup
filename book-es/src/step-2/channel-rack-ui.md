# La interfaz del channel rack

Archivos:
[`channel_rack.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/channel_rack.rs),
[`widgets.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/widgets.rs)
y
[`app.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app.rs)
en `crates/lineup-app/src/`

## Escribir tus propios widgets

egui tiene botones y deslizadores, pero no botones de paso ni perillas.
Hacer uno requiere tres movimientos: reservar espacio, leer el ratón,
pintar.

```rust
pub fn step_button(ui: &mut Ui, on: bool, beat_group: usize, is_playhead: bool) -> Response {
    let (rect, response) = ui.allocate_exact_size(STEP_SIZE, Sense::click());

    if ui.is_rect_visible(rect) {
        let mut fill = if on { STEP_ON } else { STEP_OFF[beat_group % 2] };
        if response.hovered() {
            fill = fill.gamma_multiply(1.25);
        }
        let painter = ui.painter();
        painter.rect_filled(rect.shrink(1.0), 3.0, fill);
        if is_playhead {
            painter.rect_stroke(rect.shrink(1.0), 3.0, Stroke::new(2.0, PLAYHEAD), StrokeKind::Inside);
        }
    }
    response
}
```

- `allocate_exact_size` reclama un rectángulo en la maquetación y dice
  qué interacciones queremos (`Sense::click()`). Devuelve el rectángulo
  y un `Response` que describe lo que hizo el ratón ahí en este frame.
- `is_rect_visible` se salta pintar las filas que han quedado fuera de
  la vista al desplazarse.
- El `Painter` dibuja formas. Los colores son constantes `Color32`
  definidas al principio del archivo, para que la paleta viva en un solo
  lugar.
- Devolver el `Response` permite al llamador usarlo como cualquier
  widget incorporado: `if response.clicked() { ... }`.

La perilla usa `Sense::click_and_drag()`, convierte el movimiento
vertical del ratón en un cambio de valor y llama a
`response.mark_changed()` para que quienes llaman puedan usar
`.changed()` como harían con un deslizador. Su aguja se dibuja con un
poco de trigonometría: el ángulo barre 270 grados, y `(sin, -cos)` del
ángulo da la dirección (la *y* de pantalla crece hacia abajo, de ahí el
menos).

## Dibujar devuelve acciones

La función `show` del rack nunca cambia el proyecto. Solo lo mira y
devuelve lo que hizo el usuario:

```rust
pub enum RackAction {
    ToggleStep { channel: usize, step: usize },
    ClearStep { channel: usize, step: usize },
    Preview(usize),
    SetVolume { channel: usize, volume: f32 },
    ToggleMute(usize),
    Remove(usize),
    AddDrum(Drum),
    AddSynth,
    LoadSample { channel: Option<usize>, path: PathBuf },
}

pub fn show(ui: &mut Ui, project: &Project, playhead: Option<usize>) -> Vec<RackAction>
```

`project: &Project` es un **préstamo compartido**: la función puede leer
el proyecto pero no cambiarlo, y el compilador lo hace cumplir. La app
aplica entonces las acciones:

```rust
let actions = egui::CentralPanel::default_margins()
    .show(ui, |ui| channel_rack::show(ui, &self.project, playhead))
    .inner;
for action in actions {
    self.apply(action);
}
```

Esta separación también resuelve un rompecabezas de préstamos. Mientras
el rack dibuja, mantiene un préstamo compartido del proyecto, así que
tampoco podría cambiarlo (eso necesitaría un `&mut` al mismo tiempo, que
el borrow checker prohíbe). Recoger primero las acciones y aplicarlas
después de terminar de dibujar mantiene los dos préstamos separados en
el tiempo.

`apply` es un `match` con una rama por acción. Cada rama actualiza el
`Project` *y* envía el `Command` correspondiente, de modo que el
proyecto de la interfaz y la copia del hilo de audio no pueden discrepar
nunca:

```rust
RackAction::ToggleStep { channel, step } => {
    if let Some(on) = self.project.toggle_step(channel, step) {
        self.send(Command::SetStep { channel, step, on });
    }
}
```

## Menús

```rust
name.context_menu(|ui| {
    if ui.button("Remove channel").clicked() {
        actions.push(RackAction::Remove(index));
        ui.close();
    }
});
```

`context_menu` abre un menú con clic derecho; `menu_button` abre uno con
un clic normal (el botón «Add channel»). Ambos reciben un closure que
dibuja el contenido del menú, y `ui.close()` lo cierra tras una
elección. `add_enabled_ui(false, ...)` pone en gris todo lo de dentro,
que es como se desactiva «Add channel» cuando el rack está lleno.

## Arrastrar y soltar

egui informa de los archivos que se arrastran sobre la ventana en
`input.raw.hovered_files`, y de los que se sueltan en
`input.raw.dropped_files`. Mientras dibuja, el rack guarda el rectángulo
de cada fila; después busca la fila bajo el ratón:

```rust
let target_row = pointer.and_then(|p| row_rects.iter().position(|rect| rect.contains(p)));
```

`position` devuelve el índice del primer elemento para el que el closure
es verdadero, o `None`. Un archivo soltado sobre una fila sustituye el
sonido de ese canal; un archivo soltado en cualquier otro sitio se
convierte en un canal nuevo con el nombre del archivo
(`path.file_stem()` es el nombre del archivo sin su extensión).

La carga ocurre en `LineupApp::load_sample`, en el hilo de la interfaz,
porque leer archivos es lento y nunca debe ocurrir en el hilo de audio.
Un archivo roto deja el canal como estaba y pone el motivo en la barra
de estado.

## Mantener el playhead en movimiento

egui solo redibuja cuando pasa algo, como un movimiento de ratón.
Mientras el patrón se reproduce, el playhead cambia solo, así que la app
pide redibujados continuos:

```rust
if self.playing {
    ui.ctx().request_repaint();
}
```
