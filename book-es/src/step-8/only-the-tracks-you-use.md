# Solo las pistas que usas

Archivos: [`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs),
[`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs)
y [`crates/lineup-app/src/playlist.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/playlist.rs)

Toda lista de reproducción solía empezar con dieciséis pistas. Una demo con melodía
y batería se abría con la pista 3 a la pista 16 vacías debajo, que es
mucho gris que desplazar para descubrir que no hay nada.

Ahora un proyecto tiene solo las pistas hasta su último clip. Archivo >
Nuevo tiene una pista, las demos tienen las dos que usan. Debajo de la
última pista hay una **fila libre** más tenue: coloca un clip en ella
(o arrastra uno) y se vuelve una pista. Su cabecera tiene también un
botón **➕ Nueva pista**, y "➕ Pista" y el menú de pista con clic derecho
de [las páginas anteriores](look-and-transport.md) siguen funcionando.

![La demo de inicio tras colocar un clip en la fila libre y pulsar "New track".](../images/step-8-spare-row.png)

## Un clip puede crear su propia pista

`Project::clip_fits` decide si un clip puede ir a un sitio. Solía decir
"en una pista existente". Ahora se permite una fila más, la que está
justo debajo de la última pista, mientras la lista de reproducción no esté llena:

```rust
let tracks = self.playlist.tracks.len();
clip.pattern < self.patterns.len()
    && (clip.track < tracks || (clip.track == tracks && tracks < MAX_TRACKS))
```

`add_clip` y `move_clip` llaman entonces a `open_track`, que añade un
`Track::numbered` cuando el clip está en esa fila nueva. Como la pista
y el clip se añaden en la misma edición, un solo Ctrl+Z se lleva ambos.

La vista de la lista de reproducción solo tuvo que dibujar una fila más. `Layout`
recibió una marca `spare` y un método `rows()`, y `cell_at`, que
convierte la posición del ratón en una pista y un paso, ahora ajusta a
`rows()` en lugar de al número de pistas. Todo lo demás, colocar y
arrastrar, ya funcionaba desde `cell_at`.

## Recortar, y las closures como reglas

Los proyectos se siguen construyendo sobre una lista de reproducción de dieciséis
pistas: el código de las demos nombra la pista 0 y la pista 1 antes de
colocar ningún clip, y es más fácil cuando existen. Al final,
`remove_unused_tracks` descarta las pistas vacías del fondo. Los
archivos viejos necesitan casi lo mismo, con una diferencia: una pista
que alguien renombró o silenció se mantuvo a propósito, así que cargar
descarta solo las pistas que están exactamente como empieza una pista
nueva.

Dos reglas, un bucle. El bucle toma la regla como closure:

```rust
fn trim_tracks(&mut self, drop: impl Fn(usize, &Track) -> bool) {
    // ... from the bottom, pop tracks with no clips while `drop` says so
}

pub fn remove_unused_tracks(&mut self) {
    self.trim_tracks(|_, _| true);
}

pub(crate) fn remove_blank_tracks(&mut self) {
    self.trim_tracks(|index, track| *track == Track::numbered(index + 1));
}
```

`impl Fn(usize, &Track) -> bool` significa "cualquier función o closure
que toma un número de fila y una pista y responde sí o no". El
compilador hace una copia de `trim_tracks` por cada closure, así que
esto no cuesta nada en tiempo de ejecución. Comparar con
`Track::numbered(index + 1)` funciona porque `Track` deriva
`PartialEq`: una pista en blanco es igual a una recién creada para su
fila.

Recortar solo quita pistas **debajo** del último clip. Una pista vacía
en el medio se queda, porque quitarla movería cada clip bajo ella una
fila arriba.

## Archivos viejos

Los archivos guardados antes de este cambio tienen dieciséis pistas.
Siguen cargando: el formato no cambió, solo lo que se hace después de
leerlo. Cargar sigue siendo estricto en un sentido. Cuando colocas un
clip a mano, la fila bajo las pistas crea una pista nueva; en un
archivo, un clip bajo las propias pistas del archivo es un error, y se
informa como tal.
