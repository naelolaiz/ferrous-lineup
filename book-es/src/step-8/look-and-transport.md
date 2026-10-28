# Un aspecto más claro, un transporte de verdad y editar pistas

Archivos: [`crates/lineup-app/src/theme.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/theme.rs),
[`crates/lineup-app/src/app.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app.rs),
[`crates/lineup-core/src/sequencer.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/sequencer.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs)
y [`crates/lineup-app/src/playlist.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/playlist.rs)

Hasta aquí, la app usaba el estilo oscuro de serie de egui, y se notaba:
una fila larga de botones de texto planos, nada que dijera qué hace un
control salvo que esperaras a un tooltip, y ninguna muestra de dónde
está la canción. Peor aún, el transport era un único botón Play/Stop que
siempre empezaba desde el principio. No podías pausar, no podías saltar
al compás 9 para oír el estribillo, y no podías insertar ni borrar una
pista. Esta página arregla todo eso:

- **Un tema**: paneles gris azulados, botones grises en relieve, una luz
  naranja para lo que esté seleccionado o encendido, pantallas negras
  para los números.
- **Una barra de herramientas agrupada** con una pantalla de tempo y una
  pantalla BAR:BEAT:STEP.
- **Una barra de ayuda** junto a los menús que dice qué hace el control
  que está bajo el ratón.
- **Botones de transport**: ir al principio, un compás atrás,
  reproducir/pausar, detener, un compás adelante, grabar. Haz clic en
  los números de compás del playlist para mover el cabezal de
  reproducción.
- **Editar pistas**: haz clic derecho en el nombre de una pista para
  insertar, mover o borrar pistas.

![La barra de herramientas nueva mientras suena la canción. La barra de ayuda de arriba explica el botón de pausa sobre el que está el ratón.](../images/step-8-look.png)

## Cómo usarlo

| Haz esto | Para |
|---|---|
| ▶ o Space | reproducir desde el cabezal de reproducción |
| ❚❚ (mientras suena) | pausar; ▶ sigue desde ahí |
| ⏹ o Space (mientras suena) | detener y volver a donde pusiste el cabezal de reproducción por última vez |
| ⏮ o Home | ir al principio |
| ◀◀ ▶▶ o Page Up, Page Down | mover un compás atrás o adelante |
| haz clic en los números de compás del playlist | poner el cabezal de reproducción ahí (arrastrar sigue marcando el loop) |
| haz clic derecho en el nombre de una pista | insertar una pista encima o debajo, moverla o borrarla |

## Un tema es una función

egui dibuja cada widget a partir de un `Style`: los colores en
`style.visuals`, los tamaños y huecos en `style.spacing`. Nada se pinta
con un color fijo salvo que el código pida uno. Así que cambiar todo el
aspecto es una función, llamada una vez al arrancar la app:

```rust
pub fn apply(ctx: &egui::Context) {
    ctx.set_theme(egui::Theme::Dark);
    ctx.style_mut_of(egui::Theme::Dark, |style| {
        let visuals = &mut style.visuals;
        visuals.panel_fill = PANEL;
        visuals.window_fill = WINDOW;
        visuals.selection.bg_fill = ACCENT;      // selected and switched-on things
        visuals.selection.stroke = Stroke::new(1.0, ON_ACCENT);
        // ...
        for (state, fill) in [
            (&mut widgets.inactive, BUTTON),
            (&mut widgets.hovered, BUTTON_HOVER),
            (&mut widgets.active, BUTTON_PRESSED),
            (&mut widgets.open, BUTTON_HOVER),
        ] {
            state.bg_fill = fill;
            state.weak_bg_fill = fill;
            state.bg_stroke = Stroke::new(1.0, OUTLINE);
        }
        style.spacing.button_padding = vec2(7.0, 3.0);
    });
}
```

Tres cosas que hay que notar:

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
solo dibujan ellas (las filas del playlist, las teclas del piano roll,
los botones de paso). Esas simplemente recibieron valores nuevos que
pegan con el tema.

## Cambiar el estilo de unos pocos widgets

La pantalla de tempo es un `DragValue`, el mismo widget de antes, pero
dibujado en negro con dígitos verde azulados. Su estilo se cambia solo
dentro de su marco:

```rust
pub fn lcd_frame(ui: &mut Ui, label: &str, width: f32,
                 add: impl FnOnce(&mut Ui) -> Response) -> Response {
    egui::Frame::new()
        .fill(LCD)
        .show(ui, |ui| {
            // ...
            let visuals = &mut ui.visuals_mut().widgets;
            for state in [&mut visuals.inactive, &mut visuals.hovered, &mut visuals.active] {
                state.bg_fill = LCD;
                state.fg_stroke.color = LCD_TEXT;
            }
            ui.style_mut().override_font_id = Some(FontId::monospace(16.0));
            add(ui)
        })
        .inner
}
```

Cada `Ui` tiene su propia copia del estilo (un `Arc<Style>`; `style_mut`
hace una copia privada la primera vez que se cambia). El contenido del
marco recibe un `Ui` hijo, así que el cambio se queda dentro del marco y
el resto de la barra de herramientas no se toca. El campo de tempo es
entonces simplemente:

```rust
let tempo = theme::lcd_frame(ui, "TEMPO", 62.0, |ui| {
    ui.add(egui::DragValue::new(&mut bpm).range(Tempo::MIN..=Tempo::MAX))
});
```

`add` es una closure que *devuelve* el `Response` del widget, así que
quien la llama puede seguir preguntando `tempo.changed()` fuera.

## La barra de ayuda: un mensaje del futuro

La barra de ayuda está arriba del todo de la ventana, así que se dibuja
*primero*. Pero los widgets que saben sobre qué está el ratón se dibujan
*luego*, en el mismo frame. En modo inmediato no hay vuelta atrás para
repintar la barra.

El arreglo es dejar que la barra muestre lo que se encontró en el frame
**anterior**. Los widgets dejan su ayuda en la memoria por frame de egui,
`ctx.data`, que es un mapa de un `Id` a un valor de cualquier tipo:

```rust
pub fn hint(response: Response, text: impl Into<String>) -> Response {
    let text = text.into();
    if response.hovered() {
        response.ctx.data_mut(|data| data.insert_temp(hint_id(), text.clone()));
    }
    response.on_hover_text(text)
}
```

Al principio del siguiente frame, la barra la saca (para que no se quede
cuando el ratón se va):

```rust
pub fn start_frame(&mut self, ctx: &egui::Context) {
    let text = ctx.data_mut(|data| data.remove_temp::<String>(hint_id()));
    if text != self.text {
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

Un frame de retraso es invisible a 60 frames por segundo, y el código
sigue siendo sencillo. Es un truco habitual en las UIs de modo
inmediato.

## El transport: ¿dónde empieza Play?

Antes, `Command::Play` siempre reiniciaba el sequencer en el paso 0.
Pausar y saltar necesitan una idea nueva: una **posición de inicio**. El
sequencer aprende a saltar:

```rust
/// Jumps to `step`, so it fires on the next tick. A step past the end
/// wraps around, as playback would.
pub fn start_at(&mut self, step: usize) {
    self.frames_until_next = 0.0;
    self.next_step = step % self.length;
}
```

y el hilo de audio recibe un comando nuevo:

```rust
Command::Play => {
    self.sequencer.start_at(self.start);
    self.playing = true;
}
Command::Seek(step) => {
    self.start = step;
    if self.playing {
        for channel in &mut self.channels {
            channel.instrument.release_all();
        }
        self.sequencer.start_at(step);
    }
}
```

Detenido, `Seek` solo mueve dónde empieza el siguiente `Play`. Sonando,
salta ahí, y suelta antes cada nota: una nota que estaba sonando
esperaría si no a su final, al que la reproducción ya no va a llegar
nunca.

Todo lo demás es contabilidad en la app, con dos números:

- `cursor`: dónde empieza la reproducción o desde dónde sigue;
- `marker`: adónde vuelve Stop, el último lugar donde pusiste el cabezal
  de reproducción.

```rust
fn pause(&mut self) {
    let here = self.playhead().unwrap_or(self.cursor);
    self.halt();
    self.cursor = here;              // carry on from here
    self.send(Command::Seek(here));
}

fn set_playing(&mut self, playing: bool) {
    if playing {
        self.send(Command::Seek(self.cursor));
        self.send(Command::Play);
        self.playing = true;
    } else {
        self.halt();
        self.cursor = self.marker;   // Stop goes back to the marker
        self.send(Command::Seek(self.cursor));
    }
}
```

Así se comportan los estudios basados en patrones: pausar mantiene tu
sitio, detener te lleva de vuelta a donde empezaste a escuchar, para que
puedas oír el mismo pasaje una y otra vez con Space.

### Saltar con repeticiones

Hacer clic en los números de compás da un **paso de timeline**, pero el
hilo de audio cuenta **pasos de canción**, donde un estribillo tocado
dos veces cuenta dos veces (ver [La estructura de la canción](song-structure.md)).
La app convierte uno en el otro con los passes del arreglo:

```rust
let song = self.project.playback_arrangement();
let step = song
    .passes
    .iter()
    .find(|pass| (pass.from..pass.from + pass.length).contains(&step))
    .map_or(0, |pass| pass.start + (step - pass.from));
self.seek(step);
```

`find` toma el **primer** pass que cubre el paso, así que hacer clic
dentro de un estribillo repetido va a su primera pasada. Un rango como
`a..b` es un valor en Rust, con métodos como `contains`.

La regla de compases ya usaba el arrastre para la región de loop. egui
distingue un clic de un arrastre: `Sense::click_and_drag()` comunica
`clicked()` solo cuando el ratón no se movió mucho entre pulsar y
soltar, y `drag_started()` solo cuando sí lo hizo. Así que ambos viven
en la misma franja.

### Moverse por compases

`skip_bars` mueve al principio de un compás. Ir hacia atrás desde la
mitad de un compás va primero al principio de ese compás, como en un
magnetófono:

```rust
let target = if bars < 0 && !here.is_multiple_of(per_bar) {
    bar + bars + 1
} else {
    bar + bars
};
```

`is_multiple_of` es un método más nuevo de la biblioteca estándar sobre
enteros sin signo; clippy lo sugiere en lugar de `here % per_bar != 0`,
que dice lo mismo de forma menos directa.

## Insertar, mover y borrar pistas

Un clip nombra su pista por **número**. Inserta una pista en la fila 2,
y todo clip de la fila 2 o de debajo debe bajar uno, o la batería
acabaría de repente en la pista equivocada:

```rust
pub fn insert_track(&mut self, index: usize) -> Option<usize> {
    let tracks = &mut self.playlist.tracks;
    if tracks.len() >= MAX_TRACKS {
        return None;
    }
    let index = index.min(tracks.len());
    tracks.insert(index, Track::numbered(tracks.len() + 1));
    for clip in &mut self.playlist.clips {
        if clip.track >= index {
            clip.track += 1;
        }
    }
    Some(index)
}
```

`remove_track` es el caso contrario: tira los clips de la pista con
`retain`, y luego sube uno los de debajo. `swap_tracks` intercambia dos
filas y los clips que tienen. Es la misma regla que los patrones y los
clips del paso 4: quien cambia la lista arregla los números que apuntan
a ella, por eso los campos son privados y solo los métodos de `Project`
los cambian. Cada una de estas cosas es una llamada dentro de `edit`,
así que deshacer trae de vuelta una pista borrada con sus clips.

![Clic derecho en el nombre de una pista.](../images/step-8-track-menu.png)

El menú es el `context_menu` de egui sobre el campo del nombre de la
pista. Un campo de texto abre igualmente un menú con un clic derecho,
porque escribir solo necesita el botón izquierdo.
