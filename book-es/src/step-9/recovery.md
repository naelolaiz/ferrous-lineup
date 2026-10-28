# Sin perder nada: recuperación, proyectos recientes y audio que vuelve

Archivos: [`crates/lineup-app/src/app/recovery.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/recovery.rs),
[`crates/lineup-app/src/app/recent.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/recent.rs),
[`crates/lineup-app/src/app/settings.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/settings.rs),
[`crates/lineup-app/src/app/edit.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/edit.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs),
[`crates/lineup-app/src/midi.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/midi.rs)
y [`crates/lineup-listen/src/decode.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-listen/src/decode.rs)

Una canción larga es mucho trabajo, y la mayor parte es invisible: un
acorde escrito en el sheet, una nota arrastrada en el piano roll, una
mezcla cambiada en el mixer. Octubre de 2026 se dedicó a las maneras en
que ese trabajo aún podía perderse, y a la tarjeta de sonido que podía
pararse. Los arreglos no son musicales. Son la diferencia entre un
programa del que te fías y uno que guardas dos veces.

## Una copia del trabajo sin guardar

La app escribe una copia de recuperación del proyecto mientras tiene
cambios sin guardar. El primer cambio escribe una al momento, para que
la primera edición de una sesión no sea la que pierda un crasheo;
después escribe cada treinta segundos:

```rust
const INTERVAL: Duration = Duration::from_secs(30);
```

Los archivos están en una carpeta `recovery` dentro de la carpeta de
datos de la app, la que le da `eframe::storage_dir(APP_NAME)`. En Linux
suele ser `~/.local/share/ferrouslineup/recovery`. Cada ejecución de la
app nombra sus archivos con su propio nombre, como
`4211-1793047912612-0.lineup` y `4211-1793047912612-0.info`: el número
de proceso, la hora de inicio en milisegundos y un contador. Por qué
cada ejecución necesita los suyos está en [Dos ventanas a la
vez](#dos-ventanas-a-la-vez), más abajo. La copia **no** se escribe
junto al proyecto, porque un proyecto puede estar en un medio de solo
lectura, y un archivo de guardado automático en una carpeta de canciones
sorprendería a quien lo viera. El pequeño archivo `.info` lleva la ruta
y el nombre del proyecto, de modo que una canción recuperada vuelve como
sí misma, no como «Untitled».

La copia la escribe `file::save`, que escribe un archivo temporal y lo
renombra. Un crasheo a mitad deja entera la copia anterior; el archivo
info se escribe después del proyecto, así que un crasheo entre los dos
pierde el nombre en lugar de dar al proyecto uno equivocado.

Al arrancar, si hay una copia, un modal pregunta
**Recover unsaved work?** y nombra el archivo:

- **Recover it** devuelve el proyecto, con su ruta y su nombre, y quita
  la copia. También carga el archivo del disco, si aún existe, como la
  versión «guardada», de modo que el trabajo recuperado muestra
  correctamente la estrella de sin guardar; deshacer de vuelta al
  archivo la quita. Si el archivo ya no está, el proyecto recuperado
  queda sencillamente sin guardar.
- **Discard it** quita la copia y deja el proyecto que la app habría
  abierto en caso contrario.

Una salida limpia quita la copia, porque no hay nada que recuperar. La
copia que queda es la señal de un crasheo, un kill o un corte de luz. Una
copia cuyo proyecto no se carga se quita al arrancar en lugar de
ofrecerse. La versión 0.2.0 guardaba una copia, `recovery.lineup`,
directamente en la carpeta de datos; una copia dejada allí se sigue
ofreciendo después de una actualización.

### Dos ventanas a la vez

Nada te impide arrancar Ferrous Lineup dos veces. Cuando cada ejecución
escribía el mismo `recovery.lineup`, la segunda ventana encontraba la
copia de la primera al arrancar y la ofrecía como si la primera hubiera
crasheado, cuando seguía abierta. El guardado automático de cada ventana
sustituía además la copia de la otra, y la primera en guardar la
quitaba.

Así que cada ejecución escribe su propia copia, y junto a ella un tercer
archivo, `<run>.lock`, que **bloquea** antes de escribir la primera
copia y mantiene bloqueado mientras corre. Un file lock es una promesa
que mantiene el sistema operativo: solo un archivo abierto puede
sostenerlo, y cuando el programa que lo sostiene acaba, el sistema lo
suelta, incluso en un crasheo o un corte de luz. Eso lo convierte en la
pregunta correcta que hacer al arrancar: «¿sigue en marcha la ejecución
que escribió esta copia?». Si su lock se puede tomar, no lo está, y la
copia se ofrece; si no, la copia pertenece a una ventana que sigue
abierta y se deja en paz.

La biblioteca estándar tiene file locks desde Rust 1.89 (`File::lock`,
`File::try_lock` y amigos). `try_lock` no espera: dice al momento si
consiguió el lock, y su error distingue «alguien más lo sostiene» de
«este sistema no puede bloquear archivos»:

```rust
fn take_lock(path: &Path) -> Lock {
    // A copy whose lock file is gone was left by a run that closed with
    // work unsaved: the file is made again, to be taken.
    let Ok(file) = OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(path)
    else {
        return Lock::Unknown;
    };
    match file.try_lock() {
        Ok(()) => Lock::Taken(file),
        Err(TryLockError::WouldBlock) => Lock::Held,
        Err(TryLockError::Error(_)) => Lock::Unknown,
    }
}
```

El lock dura tanto como el valor `File`: soltarlo lo libera. Así que una
ventana que encuentra una copia conserva el `File` en el valor
`Recovered` hasta que respondes a la pregunta; una tercera ventana
abierta mientras tanto no puede tomar el lock y no ofrece la misma copia
dos veces. Cuando la app se cierra, el `Drop` de `Recovery` quita su
archivo de lock; una copia de trabajo sin guardar se queda, sin su lock,
para ofrecerse la próxima vez. Un sistema que no puede bloquear archivos
(`Lock::Unknown`) ofrece la copia de todos modos: ofrecerla dos veces no
pierde nada, no ofrecerla nunca sí.

Si dos ejecuciones crashearon, hay dos copias. La más nueva se ofrece
primero, y la siguiente espera hasta que el trabajo recuperado se guarda
o se deja ir, de modo que recuperar una nunca sustituye a otra.

## File > Open recent

**File > Open recent** lista los últimos diez proyectos abiertos o
guardados, el más reciente primero. La lista se guarda en los datos
almacenados de egui, así que sobrevive a un reinicio. Cada elemento
muestra el nombre del archivo, con la ruta completa en su tooltip. Abrir
un archivo que ya no se carga lo quita de la lista en lugar de dejar una
entrada muerta. El menú está deshabilitado mientras la lista está vacía.

La lista guarda rutas, no copias de proyectos. Si un proyecto se mueve o
se renombra fuera de la app, la ruta antigua se queda hasta que un
intento de abrirla falla.

## Guardar, importar y salir

Tres arreglos menores conservan el proyecto que querías conservar:

- **Save as…** da ahora al archivo una extensión de proyecto incluso si
  escribes otra. `song.txt` se guarda como `song.lineup`, no como un
  chord sheet que se abre en el importador equivocado.
- Abrir un proyecto, una demo, un archivo MIDI o un chord sheet detiene
  un **File > Import recording…** que siga escuchando. La importación se
  inició para el proyecto que se deja atrás; sin esto, su resultado
  podría llegar más tarde y sustituir al proyecto nuevo. El aviso dice
  «Stopped importing the recording».
- **File > Quit** durante una exportación WAV o de vídeo pregunta
  **An export is still running**, con **Stop it and quit** y
  **Keep exporting**. Detener pide a la exportación que pare, espera
  hasta que lo hace, quita el archivo inacabado y luego cierra;
  cualquier pregunta de cambios sin guardar va primero. Un WAV o un
  vídeo a medio escribir no se deja nunca atrás con aspecto de acabado,
  y el proyecto no se cierra debajo de una exportación que aún escribe.

El editor de estilos tiene su propio estado sin guardar. Salir con un
borrador editado pregunta **Save style changes?**, y ofrece
**Save style**, **Don't save style** y **Cancel**, de modo que guardar
una canción no descarta el estilo en silencio.

## Arrastrar sin copiar la canción

Cada edición clonaba antes el proyecto entero para compararlo con la
versión anterior. Para un clic eso es barato. Para un arrastre ocurría
en cada frame: mover una nota en una canción grande copiaba cada patrón,
cada canal y cada nota, sesenta veces por segundo.

Ahora el primer cambio de un arrastre abre un paso de deshacer y guarda
una copia de antes de él. Los frames siguientes cambian el proyecto en
su sitio. Al hilo de audio se le envía lo que le falta desde una copia
en sombra del último estado que recibió, como mucho cada 50
milisegundos, y una vez más cuando el paso se cierra. Un arrastre
quieto, o una pausa al escribir, no tiene un cambio siguiente que lleve
el último, así que el final de cada frame lo envía en cuanto se cumplen
sus 50 milisegundos, y pide a egui un frame en ese momento aunque nada
más se mueva:

```rust
const SYNC_INTERVAL: Duration = Duration::from_millis(50);
```

Un cambio en el número de patrones o de canales es estructural y se
envía al momento, porque las filas del hilo de audio tienen que moverse
con él. La estrella de sin guardar ya no se calcula comparando el
proyecto entero en cada frame. Un cambio hecho de una vez, como un clic,
un deshacer o un rehacer, compara de nuevo el proyecto con el guardado,
de modo que un step encendido con un clic y luego apagado quita la
estrella. Los frames de un arrastre no comparan; la comparación se
ejecuta una vez cuando se cierra el paso de deshacer del arrastre, así
que una nota arrastrada fuera y de vuelta quita la estrella al soltarla.
Deshacer sigue guardando hasta 256 MB de instantáneas, con al menos diez
pasos incluso para una canción muy grande.

## Audio que vuelve

**Edit > Settings… > Audio** tiene un botón bajo **Output**. Dice
**Restart audio** mientras el stream está corriendo, y **Try again**
cuando el audio no está disponible o el stream ha reportado un error.
Vuelve a abrir el dispositivo de salida por defecto del sistema. El
proyecto se queda; los puertos de entrada y salida MIDI siguen
seleccionados y se reconectan al motor nuevo; el proyecto entero se
envía al hilo de audio nuevo. Si la canción estaba sonando, se desplaza
de vuelta a la misma posición y suena otra vez, y el aviso dice
«Audio is playing again».

El stream antiguo se suelta antes de que empiece el nuevo, porque un
dispositivo que se reabre no puede seguir retenido por el stream muerto.
Si abrirlo falla, la barra de estado dice por qué, y el botón sigue
siendo **Try again**. Una interfaz desenchufada, un dispositivo por
defecto cambiado o un servidor de sonido que se reinició se pueden
arreglar todos sin cerrar la app.

Las notas que el motor antiguo dejó sonando en un sintetizador externo
se detienen: antes de que el hilo antiguo de MIDI Out acabe, envía All
Notes Off en los dieciséis canales, y el nuevo espera a que lo haga. Si
la traza de reproducción está activa, continúa en el mismo archivo.
[Antes de la próxima
versión](before-the-release.md#restart-audio-notas-que-se-detienen-y-una-traza-que-continúa)
muestra cómo.

## Comandos que deben llegar

El hilo de audio no puede esperar, así que la UI le habla a través de un
ring buffer. La cola guarda 4096 comandos, un proyecto grande entero.
Cuando está llena, `send_wait` no suelta el comando ante la primera cola
llena: cede y vuelve a intentarlo, hasta mil veces. Solo un stream que
ya ha reportado un error se rinde al momento, porque esperar a un hueco
que nunca llegará congelaría la ventana.

MIDI Out tiene la misma regla por una razón más estricta. Un note-off
perdido deja una nota sonando para siempre en un sintetizador externo.
Los mensajes que encuentran la cola de salida llena esperan en un
backlog reservado y salen al principio del bloque siguiente. Quitar un
canal o sustituir un instrumento envía sus note-offs por el mismo
camino. Desconectar un puerto MIDI envía un mensaje All Notes Off en los
dieciséis canales antes de que se cierre la conexión antigua. El backlog
es finito, pero hacen falta cientos de mensajes atascados para llenarlo,
lo que significa que el propio puerto ya no está.

**File > Import recording…** rechaza una grabación de más de diez
minutos. La decodificación comprueba la longitud mientras lee, y se
detiene con «the recording is longer than 10 minutes». El análisis
mantiene la grabación, y una copia de ella, en memoria; una más larga
necesitaría más de lo que puede dar una máquina pequeña. La importación
muestra la razón en lugar de fallar en silencio o agotar la memoria.

A la traza de reproducción se le permite perder. Graba cada nota para un
informe, no para el sonido. Si su lector se queda atrás y sus 65536
eventos se llenan, el hilo de audio cuenta los eventos perdidos y sigue;
el informe dice cuántos se perdieron. Esperar ahí pondría la traza antes
que la música, que es el orden equivocado.

## Probarlo

1. Ejecuta `cargo run --release`, abre **File > New**, haz un cambio y
   guárdalo como `recovery-test.lineup`. Haz otro cambio. Espera medio
   minuto y mata la app desde la terminal (`Ctrl+C`) en lugar de
   cerrarla. Arranca de nuevo. **Recover unsaved work?** nombra el
   archivo; haz clic en **Recover it**. El cambio está ahí, con la
   estrella de sin guardar.
2. Guárdalo, abre otro proyecto y usa **File > Open recent**: la canción
   de prueba está en la lista. Sal y arranca otra vez; sigue ahí.
3. Pon una canción a sonar y abre **Edit > Settings… > Audio**. Haz clic
   en **Restart audio** mientras suena. El sonido vuelve, el aviso dice
   «Audio is playing again», y la canción continúa desde donde estaba.
   Si el dispositivo no está disponible, el mismo botón dice
   **Try again**.
4. Con un sintetizador MIDI conectado, toca una nota larga y quita el
   canal o cambia su instrumento mientras suena. La nota se detiene: su
   note-off se conservó y se envió en lugar de perderse. (Esto necesita
   hardware MIDI real; las pruebas corren sin él.)
