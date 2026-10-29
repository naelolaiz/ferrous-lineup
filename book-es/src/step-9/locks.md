# Bloquear una pista: los estilos tocan a su alrededor

Archivos: [`crates/lineup-core/src/lock.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/lock.rs),
[`crates/lineup-core/src/band/locks.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/locks.rs),
[`crates/lineup-core/src/library/apply.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/apply.rs)
y [`crates/lineup-app/src/channel_rack.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/channel_rack.rs)

Digamos que tocaste tú mismo una parte de piano, o elegiste una
guitarra de nailon para los acordes, y ahora quieres oír la canción
como bossa nova. Antes, elegir el estilo reescribía cada pista que
toca: tu parte de piano quedaba sustituida por el comping del estilo, y
tu guitarra volvía al instrumento del estilo. Ahora puedes **bloquear**
una pista antes. Una pista bloqueada conserva sus notas y su
instrumento, y el estilo toca el resto de la canción a su alrededor.

![El channel rack: un candado pequeño entre la perilla de cada canal y su nombre. El candado de Kick está encendido y cerrado; los de Clap, Hat, Snare y Lineup Osc están apagados y abiertos. El aviso dice «Lock: styles keep its notes and sound».](../images/step-9-lock-rack.png)

## Probarlo

1. Abre una demo, por ejemplo **Oda a la alegría**, y muestra el rack con
   **F6**.
2. Haz clic en el candado pequeño entre la perilla de **Teclados** y su
   nombre. Se cierra y se pone naranja. El clic derecho en el nombre
   tiene también **Bloquear** y **Desbloquear**, y **Ctrl+Z** lo deshace.
3. En la **Biblioteca**, aplica otro estilo, por ejemplo **Bossa nova**.
   La batería y el bajo cambian; la parte de Teclados es la que tenías. El
   aviso termina con **🔒 Se conservan: Teclados.**, y la lista de la Biblioteca de
   dónde viene cada parte tiene una fila **Bloqueado**.
4. Prueba estilos uno tras otro en modo patrón. Cada prueba copia las
   filas bloqueadas al patrón de prueba, así que el bajo nuevo sigue
   los acordes bloqueados.
5. Desbloquea Teclados otra vez. Si la banda estaba tocando, recupera la
   parte de piano, y el aviso dice que tu Teclados queda silenciado para
   que los dos no suenen uno encima del otro.

Lo que mueve la canción *entera* sigue moviendo con ella a una pista
bloqueada: un compás nuevo pone sus notas en los compases nuevos, y un
cambio de tonalidad las mueve a la tonalidad nueva. Un bloqueo protege
una parte de los estilos, no de que la canción cambie de forma. Siempre
puedes editar a mano una pista bloqueada.

## Cómo funciona

### Un bit por canal

El rack tiene como mucho 32 canales, así que el conjunto de los
bloqueados cabe en un `u32`, un bit por canal:

```rust
pub struct Locked(u32);

// One bit for each channel there can be.
const _: () = assert!(MAX_CHANNELS <= u32::BITS as usize);

impl Locked {
    pub fn contains(self, channel: usize) -> bool {
        channel < MAX_CHANNELS && self.0 & (1 << channel) != 0
    }
}
```

`Locked(u32)` es una **tuple struct**: un tipo nuevo envuelto alrededor
de un `u32`, al que se llega con `.0`. Envolverlo significa que nadie
puede sumar dos conjuntos por error ni pasar un número de canal donde
se espera un conjunto. La línea `const _` es una comprobación que
ejecuta el compilador: si alguien subiera `MAX_CHANNELS` por encima de
32, la compilación fallaría justo ahí, en lugar de que un bloqueo
aterrizara en silencio en el canal equivocado.

Quitar un canal renombra los que vienen después, así que el conjunto
también debe desplazarse. `without` conserva los bits por debajo del
canal quitado y baja un puesto los bits por encima:

```rust
let below = self.0 & ((1 << index) - 1);
let above = self.0.checked_shr(index as u32 + 1).unwrap_or(0) << index;
Locked(below | above)
```

`checked_shr` devuelve `None` en lugar de desbordarse cuando el
desplazamiento es 32 o más, que es lo que pasa al quitar el último
canal.

### Una puerta para todos los estilos

Los estilos escriben en patrones desde muchos sitios: un estilo entero,
un ritmo, un estilo guardado, un estilo de fila o micromodo, un
relleno, el acorde final. Si cada uno de ellos tuviera que acordarse de
saltarse los canales bloqueados, alguno lo olvidaría. Así que la
biblioteca ya no recibe un `&mut Pattern` en absoluto. Recibe un
`StylePattern`, que solo escribe filas que no están bloqueadas:

```rust
pub(crate) struct StylePattern<'a> {
    pattern: &'a mut Pattern,
    locked: Locked,
}

impl StylePattern<'_> {
    pub(crate) fn set_notes(&mut self, channel: usize, notes: impl IntoIterator<Item = NoteEvent>) -> bool {
        !self.locked.contains(channel) && self.pattern.set_notes(channel, notes)
    }
}

impl Deref for StylePattern<'_> {
    type Target = Pattern;
    fn deref(&self) -> &Pattern {
        self.pattern
    }
}
```

`Deref` deja leer un `StylePattern` exactamente como un `Pattern`
(`pattern.length()`, `pattern.notes(channel)`), mientras que escribir
solo pasa por sus propios métodos. Esta es una forma habitual de Rust:
un **wrapper** que regala la lectura y vigila la escritura. El
instrumento tiene el mismo tipo de puerta, `Project::set_sound`, que
rechaza un canal bloqueado. Una prueba aplica cada estilo, ritmo,
estilo guardado, fila y estilo de micromodo con cada canal bloqueado
por turno, y comprueba que el bloqueado nunca cambia.

### La banda toca alrededor de un bloqueo

La banda escribe sus propios patrones, una parte por músico, así que
tiene sus propias reglas (en `band/locks.rs`):

- a un músico cuyo canal está bloqueado se le salta: su parte se queda
  como está;
- los canales de batería bloqueados conservan sus notas mientras el
  resto de la batería cambia;
- si tu Teclados o Bajo propio está bloqueado y toca, el piano o el bajo de
  la banda se queda en silencio, para que la parte de la canción no se
  doble;
- cuando la banda se va (**Sin banda**), una parte de banda bloqueada se
  queda en la canción como tuya.

La banda guarda una huella de aquello desde lo que toca, para saber
cuándo volver a escribir. El bloqueo cuenta en esa huella solo cuando
algo está bloqueado, así que las canciones guardadas antes de este
cambio suenan exactamente como sonaban. Una canción con un canal
bloqueado necesita al menos el formato 23 ([el formato de archivo de
proyecto](../appendix-project-format.md)).

## Ejercicios

1. Bloquea **Teclados** en Oda a la alegría y aplica **Doo-wop**. ¿Qué pistas
   cambiaron? Abre Teclados en el piano roll: ¿hay algo distinto?
2. Bloquea **Bombo** en la canción inicial y aplica **Bossa nova**. Mira
   la batería: ¿qué conservó su ritmo antiguo, y qué sigue a la bossa
   nova?
3. (Rust) `Locked` deriva `Copy`. ¿Por qué `contains` puede recibir
   `self` por valor en lugar de `&self`, y por qué aquí es barato?
4. (Rust) `StylePattern` guarda `&'a mut Pattern`. ¿Qué diría el borrow
   checker si la biblioteca conservara un `StylePattern` y luego
   llamara a otro método de `Project` que cambia patrones?
