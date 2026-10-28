# De la biblioteca a patrones y demos

Archivos: [`crates/lineup-core/src/library/apply.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/apply.rs),
[`crates/lineup-core/src/library/demo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/demo.rs),
[`crates/lineup-app/src/library_browser.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/library_browser.rs)
y [`crates/lineup-app/src/app/library.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/library.rs)

Las tablas del capítulo anterior no saben nada de canales, patrones ni
playlists. Este capítulo las conecta: primero con un patrón, luego con
un proyecto entero, y por último con los botones en los que haces clic.

## Qué canal toca qué

`apply.rs`

Un estilo tiene acordes y una línea de bajo; un ritmo tiene batería.
Cada cosa tiene que ir a un canal, y el proyecto puede tener o no uno
que le venga bien. Las reglas están escritas como funciones pequeñas:

```rust
pub fn chords_channel(project: &mut Project, selected: Option<usize>) -> Option<usize> {
    let free = |channel: &Channel| plays_notes(channel) && channel.name != BASS && channel.name != LEAD;
    if let Some(index) = selected
        && project.channels().get(index).is_some_and(free)
    {
        return Some(index);
    }
    find_synth(project, KEYS)
        .or_else(|| { /* the first other synth */ })
        .or_else(|| synth_named(project, KEYS))
}
```

Los acordes van al canal que seleccionaste, si puede tocar notas y no
es el bajo ni el lead; si no, a un sintetizador llamado "Keys"; si no,
al primer sintetizador distinto; y si no hay ninguno, se añade un
sintetizador "Keys" nuevo. Cada `or_else` solo corre si todo lo
anterior no encontró nada. El bajo va a un sintetizador llamado "Bass",
y cada tambor al canal que lo toca, añadido si falta.

`free` es una closure guardada en una variable, para poder usarla dos
veces: una con `is_some_and`, y otra dentro de la búsqueda del primer
sintetizador distinto. `is_some_and` es `true` cuando un `Option` es
`Some` y su valor pasa la prueba.

`write_style` y `write_rhythm` rellenan entonces un patrón. Ambas usan
un método que el paso 8 añadió a `Pattern`, `set_notes`, que reemplaza
todas las notas de un canal de una vez.

## Demos: arreglar como un productor

`demo.rs`

Un patrón es un loop. Una canción son loops *arreglados*: algo solo
primero, luego entra más, un fill de vez en cuando para marcar una
sección, y un final. `style_demo` hace eso con cualquier estilo:

```text
  Armonía   |Armonía|Armonía|Armonía|Armonía|Armonía|Final
  Batería           |Batería|Batería|Batería|Batería|
                            y fill          y fill
```

La armonía suena una vez sola, para que oigas los acordes antes que la
batería; luego suenan juntos, con un fill al final de una de cada dos
pasadas y de la última. Cuántas pasadas hay depende de lo larga que sea
la progresión, para que toda demo dure unos 32 compases:

```rust
let passes = (DEMO_BARS / bars).clamp(2, 6);
```

`song_demo` hace lo mismo con una canción: un patrón por parte, con la
melodía en un sintetizador "Lead", los acordes en "Keys" y el bajo en
"Bass"; un patrón de batería por parte; y las partes colocadas en el
orden de la forma de la canción. *Für Elise* no tiene batería en
absoluto, y su mano izquierda está escrita nota a nota en lugar de
hacerse a partir de acordes, porque la mano izquierda de Beethoven *es*
el acompañamiento.

![Für Elise desde File > Demos: un patrón por parte, colocado en el orden de la forma de la canción. Está en 3/8, así que un compás tiene seis pasos, y en La menor armónica, que las harmony keys muestran con sus siete acordes propios.](../images/step-8-fur-elise.png)

Ambas devuelven un `Project` normal. Una demo no es especial en ningún
sentido: puedes cambiarla, guardarla y desmontarla como cualquier cosa
que hayas hecho.

## La pestaña Library

`library_browser.rs`

La pestaña se dibuja como las otras vistas: lee el ratón y devuelve lo
que se eligió, y la app hace el resto.

```rust
pub enum LibraryAction {
    Style(&'static Style),
    Rhythm(&'static Rhythm),
}
```

Las variantes llevan referencias `&'static` directas a las tablas
`STYLES` y `RHYTHMS`. Copiarlas es tan barato como un número, y no
pueden quedar colgando, porque las tablas viven tanto como el programa.

Los géneros son `CollapsingHeader`s, cerrados al principio para que la
lista sea corta. Mientras escribes en el buscador, todo género con una
coincidencia se abre. egui recuerda si un encabezado está abierto por
su id, y normalmente decide el usuario; `.open(Some(true))` lo anula
durante un frame:

```rust
let header = egui::CollapsingHeader::new(format!("{} ({count})", genre.name()))
    .id_salt(format!("{kind}-{genre:?}"));
let header = if searching { header.open(Some(true)) } else { header };
header.show(ui, add);
```

El `id_salt` incluye el tipo, porque "Pop" aparece dos veces, una bajo
estilos y otra bajo ritmos, y dos widgets con el mismo id compartirían
su estado de abierto o cerrado. `{genre:?}` formatea el género con
`Debug`, que para un enum es su nombre.

## Probar, cambiar y deshacer

`app/library.rs`

Hacer clic en un estilo lo reproduce al momento. Hacer clic en otro
debería *reemplazarlo*, no amontonar patrones, para que puedas pasar
rápido por la biblioteca. Pero si ya has empezado a editar ese patrón,
es tuyo, y la biblioteca no debe sobrescribirlo. ¿Cómo lo distingue la
app?

Guarda una copia de cada patrón que escribió:

```rust
pub(super) struct Audition {
    pattern: usize,
    written: Pattern,
    style: Option<&'static Style>,
    rhythm: Option<&'static Rhythm>,
    rhythm_picked: bool,
}
```

y en el siguiente clic busca una cuya copia siga siendo exactamente
igual al patrón seleccionado:

```rust
fn unchanged_audition(&self) -> Option<&Audition> {
    let pattern = self.project.pattern(self.pattern)?;
    self.auditions
        .iter()
        .rev()
        .find(|audition| audition.pattern == self.pattern && &audition.written == pattern)
}
```

El `==` de dos `Pattern`s compara sus nombres, longitudes y cada nota,
gracias al `PartialEq` derivado en `Pattern`. Es la misma idea que los
"cambios sin guardar" del paso 4: en lugar de vigilar cada edición,
guardar una copia y comparar. Nada más en la app tiene que acordarse de
decir "este patrón cambió".

La app recuerda cada audición, no solo la última, y busca desde la más
nueva con `rev()`. Eso es para deshacer: después de que Ctrl+Z devuelva
el estilo anterior, el patrón coincide con una copia más vieja, y el
siguiente clic lo sigue cambiando en su sitio.

`rhythm_picked` recuerda si elegiste el ritmo. Un estilo trae su propio
ritmo, y el siguiente estilo trae el suyo; pero una vez que has elegido
un ritmo tú mismo, los estilos que pruebes después mantienen el tuyo.

La escritura en sí es una sola llamada a `edit`, así que es un solo
paso de deshacer, el hilo de audio se entera por el diff habitual, y
guardar simplemente funciona:

```rust
let written = self.edit("try from the library", |project| {
    let index = match target {
        Some(index) => index,
        None => project.add_pattern()?,
    };
    // ... clear it, write the style, the rhythm, the tempo ...
    Some((index, project.pattern(index)?.clone()))
});
```

La closure devuelve un `Option`, así que `?` funciona dentro: si no hay
sitio para otro patrón, `add_pattern` da `None`, la closure se detiene
ahí, y `edit` devuelve el `None`. Nada cambió, así que tampoco se
registra nada para deshacer.

## File > Demos

Abrir una demo tira el proyecto actual, igual que **File > Open**,
así que primero debe preguntar por los cambios sin guardar. El paso 4
construyó esa pregunta alrededor de un enum de cosas que esperan
respuesta; una demo es una más:

```rust
pub(super) enum Pending {
    New,
    Open,
    OpenPath(PathBuf),
    Demo(Demo),
    Quit,
}
```

y `Demo` dice cuál:

```rust
pub(super) enum Demo {
    Start,
    Song(&'static Song),
    Style(&'static Style),
}
```

Después de "Guardar", "No guardar", o directamente si no se perdería
nada, `perform` recibe el `Pending::Demo`, construye el proyecto y lo
reproduce. Añadir las demos costó una variante y un brazo de match en
el código existente: eso es lo que se gana modelando "lo que espera"
como un enum.
