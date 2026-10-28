# Paso 4: El playlist, guardar y tres sistemas operativos

El paso 4 convierte a Ferrous Lineup de un loop de patrones en un pequeño
editor de canciones con el que puedes seguir trabajando mañana, en Linux,
macOS o Windows:

- **muchos patrones**: crea tantos patrones como quieras, ponles nombre,
  clónalos, bórralos. El channel rack y el piano roll muestran siempre el
  patrón seleccionado,
- un **playlist** que llena la ventana: pistas de **clips**, cada clip
  reproduce un patrón en un punto de la canción. Coloca clips con un clic,
  arrástralos, haz clic derecho para borrarlos. El mismo patrón se puede
  usar muchas veces; edítalo una vez y todos los clips cambian,
- **modo patrón y canción**: PAT hace loop del patrón seleccionado
  mientras trabajas en él, SONG reproduce el playlist desde el principio y
  hace loop al final,
- **guardar y cargar** proyectos como archivos `.lineup`, que son texto
  plano que puedes leer, con una estrella en el título de la ventana cuando
  hay cambios sin guardar y una pregunta «¿Guardar los cambios?» antes de
  perder nada,
- **compilaciones para macOS y Windows**, creadas y probadas por CI en cada
  push junto a la de Linux.

![La canción de demostración en el playlist, con el channel rack mostrando el patrón Beat](../images/step-4-playlist.png)

La demo ahora es una canción de ocho compases hecha de tres patrones: dos
compases solo de acordes, luego entra la batería, y un redoble de batería la
termina antes del loop.

## Cómo usarlo

| Haz esto | Para |
|---------|----|
| Space, o ▶ Play | reproducir o detener |
| PAT / SONG, o L | hacer loop del patrón seleccionado, o reproducir la canción |
| Haz clic en un patrón de la lista de la izquierda, o elígelo arriba | seleccionarlo: el rack lo muestra, PAT lo reproduce, los clips nuevos lo usan |
| ➕ junto al menú de patrones | añadir un patrón vacío |
| ⋯ junto al menú de patrones | renombrar, clonar o borrar el patrón seleccionado |
| Haz clic en un lugar vacío del playlist | colocar un clip del patrón seleccionado |
| Arrastra un clip | moverlo a otra pista o a otro momento (se ajusta a compases, pulsos o pasos; ver Snap) |
| Haz clic derecho en un clip, o arrastra con el botón derecho sobre varios clips | borrarlos |
| Haz clic en el nombre de una pista | renombrar la pista |
| Haz clic en la luz junto a una pista | silenciar o activar la pista |
| F6, o Channel rack | mostrar u ocultar el channel rack |
| F7, o 🎹 Piano roll | mostrar u ocultar el piano roll |
| Ctrl+N, Ctrl+O, Ctrl+S, Ctrl+Shift+S | nuevo, abrir, guardar, guardar como (⌘ en lugar de Ctrl en un Mac) |
| Suelta un archivo `.lineup` sobre la ventana | abrirlo |
| `ferrous-lineup song.lineup` | empezar con un archivo abierto |

![El menú de patrones: renombrar, clonar o borrar](../images/step-4-pattern-menu.png)

## Qué cambió en la arquitectura

```text
  lineup-core                                    lineup-audio
  ─────────                                    ──────────
  Proyecto                                     AudioThread
    canales: Vec<Channel>                        channels[c].notes: ChannelNotes
    patrones: Vec<Pattern>  ──SetNotes{p,c}──▶     (una lista por patrón)
    playlist: Playlist      ──SetSong─────────▶  clips, duración de la canción
      pistas, clips                              pattern_lengths
  playlist.rs (nuevo): Track, Clip               mode: Pattern | Song
  file.rs (nuevo): ProjectFile, guardar, cargar

  lineup-app
    playlist.rs (nuevo): el playlist y el selector de patrones
    app.rs: el playlist llena la ventana; el channel rack y el piano roll flotan
    app/files.rs (nuevo): Nuevo, Abrir, Guardar, «¿Guardar los cambios?», el título
```

Tres ideas sostienen este paso:

1. **Las cosas que se refieren unas a otras por número deben cambiar
   juntas.** Un clip dice «patrón 2». Borra el patrón 1 y ese clip debe
   decir ahora «patrón 1». El proyecto mantiene estas reglas poseyendo
   todas las listas y dejando que solo cambien a través de sus métodos.
2. **Un archivo es un contrato con el futuro.** El formato de archivo tiene
   sus propios tipos, separados de los que usa el programa, y un número de
   versión. Cargar un archivo pasa por las mismas comprobaciones que editar.
3. **Escribe código portable y luego demuéstralo.** Cada librería se eligió
   en el paso 1 para funcionar en los tres sistemas. Este paso añade los
   trabajos de CI que compilan y prueban en cada uno, de modo que un error
   aparece el mismo día en que se comete.

## Archivos de este paso

```text
crates/lineup-core/src/playlist.rs     Track, Clip, Playlist (nuevo)
crates/lineup-core/src/project.rs      muchos patrones, el playlist, la demo
crates/lineup-core/src/pattern.rs      patrones con nombre; sin notas duplicadas
crates/lineup-core/src/file.rs         guardar y cargar proyectos: FileError (nuevo)
crates/lineup-core/src/{pitch,tempo}.rs and harmony/scale.rs   derives de serde
crates/lineup-audio/src/engine.rs      ChannelNotes, PlayMode, reproducir canciones
crates/lineup-app/src/playlist.rs      la ventana del playlist (nuevo)
crates/lineup-app/src/app.rs           patrones, modos, el nuevo diseño
crates/lineup-app/src/app/files.rs     Nuevo, Abrir, Guardar, cambios sin guardar (nuevo)
crates/lineup-app/src/channel_rack.rs  muestra el patrón seleccionado en una ventana
crates/lineup-app/src/piano_roll.rs    edita el patrón seleccionado
crates/lineup-app/src/main.rs          un archivo para abrir, sin consola en Windows
.github/workflows/ci.yml             trabajos para Linux, macOS y Windows
```

Los capítulos siguen a los datos: primero los patrones y el playlist en
`lineup-core`, después cómo el hilo de audio reproduce una canción, después
los archivos, después las ventanas en pantalla y, por último, los otros
sistemas operativos.

> **Cambiado en el paso 5.** Los enlaces al código de estos capítulos
> apuntan al código más reciente, que el paso 5 cambió en algunos sitios.
> `AddPattern` y `AddChannel` del hilo de audio pasaron a ser
> `InsertPattern` e `InsertChannel`, que reciben una posición
> (`ChannelNotes::push_empty` pasó a ser `insert_empty`). La app ya no
> envía comandos desde cada acción: cada cambio pasa por `edit`, que
> compara el proyecto antes y después. Y `Instrument::for_channel` recibe
> un `SampleCache`. El [paso 5](../step-5/README.md) explica por qué.
>
> **Cambiado aún más tarde.** El paso 8 dio a los clips un contador
> `repeats`, y `SetSong` ahora lleva `plays`: los trozos de patrones con
> repeticiones y secciones ya desenrollados. Los capítulos dicen dónde
> cambió eso.
