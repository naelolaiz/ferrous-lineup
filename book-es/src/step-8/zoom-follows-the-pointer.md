# Un único juego de teclas de zoom que sigue al puntero

Archivo: [`crates/lineup-app/src/app/view.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/view.rs)

La app tiene cuatro zooms: la lista de reproducción (compases más anchos, pistas más
altas), el piano roll (las mismas dos direcciones), el rack de canales (un
solo tamaño) y el tamaño de interfaz (texto, botones y menús en todas
partes). Hasta ahora las teclas solo llegaban a tres de ellos, y no de
la forma que adivinarías:

- **Ctrl** y **+** o **−** hacían zoom en la ventana sobre la que
  habías hecho *clic* la última vez. Haz clic en un paso del rack, pasa
  el puntero sobre la lista de reproducción, pulsa Ctrl y +, y crecía el rack, no el
  lista de reproducción.
- Ninguna tecla cambiaba el tamaño de interfaz. Antes del zoom en dos
  direcciones (["Adaptar la ventana"](fitting-the-window.md)) el Ctrl y
  + propio de egui lo hacía, pero la app lo apagó para usar esas teclas
  en las áreas. Solo quedaba **Ver > Tamaño de la interfaz**.

Ahora hay una regla para todos: **las teclas hacen zoom sobre lo que
está bajo el puntero**, igual que ya hacía la rueda del ratón.

| Puntero sobre | Ctrl y + / − (o rueda) | Alt y + / − (o rueda) | Ctrl+0 |
|---|---|---|---|
| Lista de reproducción | compases más anchos / más estrechos | pistas más altas / más bajas | ambos al 100% |
| Piano roll | pasos más anchos / más estrechos | teclas más altas / más bajas | ambos al 100% |
| Rack de canales | más grande / más pequeño | lo mismo | 100% |
| Cualquier otra cosa: barras de herramientas, teclas de armonía, barra de estado, Patrones y Biblioteca | interfaz más grande / más pequeña | lo mismo | 100% |

![Ctrl y + con el puntero sobre las harmony keys: toda la interfaz es más grande.](../images/step-8-zoom-interface.png)

Si el puntero está fuera de la ventana (hiciste clic en la app y luego
te fuiste), las teclas hacen zoom sobre lo último en lo que hiciste
clic, como antes.

## Cómo funciona: ¿qué hay bajo el puntero?

El código necesita una respuesta: "¿qué cambia un zoom en este punto de
la pantalla?" Hay dos clases de respuesta, un área con su propio zoom,
o la interfaz. En Rust, "una de estas clases" es un `enum`, y una
variante de enum puede llevar un valor:

```rust
pub(crate) enum Target {
    Area(Area),   // Area::Main (the playlist), Area::Rack, Area::PianoRoll
    Interface,
}
```

`Target::at` lo encuentra. egui mantiene una pila de *capas*: el fondo
donde se dibujan los paneles, luego cada ventana encima, luego los
menús y los tooltips. `ctx.layer_id_at(pos)` dice qué capa está encima
en un punto:

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

Cada brazo tiene un **match guard**, el `if` después del patrón. El
patrón `Some(layer)` coincide con cualquier capa, y el brazo solo se
toma si el guard también es cierto; si no, el `match` prueba el
siguiente brazo. El último brazo, `_`, atrapa todo lo que queda:
ninguna capa, un menú, la ventana de fila, o el fondo fuera de la lista de reproducción.

Las barras de herramientas, las teclas de armonía y la lista de reproducción se dibujan
todas en la misma capa de fondo, así que la capa sola no puede
distinguirlas. La lista de reproducción por eso recuerda dónde se dibujó
(`PlaylistView::area`, el espacio que queda tras el panel de Patrones y Biblioteca), y el guard comprueba `playlist.contains(pos)`. Las teclas
de zoom se leen al comienzo de un frame, antes de que se dibuje nada,
así que este es el rectángulo del frame *anterior*, una sexagésima de
segundo viejo. Es un truco habitual en interfaces de modo inmediato
como egui: el layout del último frame es una suposición muy buena para
el de este.

## Un método, cuatro zooms

Toda forma de hacer zoom (teclas, rueda, pellizco del touchpad) llama
a los mismos tres métodos de `Target`, y solo ellos miran de qué clase
es:

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

`Target::Area(area)` comprueba la variante y saca el `Area` a la
variable `area`. La interfaz tiene un solo tamaño, así que `axis` no se
usa allí: Ctrl y Alt hacen lo mismo. La interfaz pasa por los mismos
tamaños que enumera el menú Ver (75% a 200%), con `interface_step`,
una gemela del `zoom_step` de las áreas.

## Pruébalo

1. Abre el rack de canales (F6) y haz clic en uno de sus pasos. Ahora
   apunta a la lista de reproducción y pulsa Ctrl y +: la lista de reproducción se hace más ancho,
   el rack se queda.
2. Apunta a las teclas de armonía y pulsa Ctrl y + dos veces: todo es más
   grande. Ctrl+0 allí lo devuelve.
3. Abre el piano roll (F7), apunta a su cuadrícula y pulsa Alt y +: las
   teclas se hacen más altas.
