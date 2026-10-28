# Patrones y el proyecto

Archivos:
[`crates/lineup-core/src/pattern.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/pattern.rs)
y
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs)

## La música

Un **patrón** es un loop corto, normalmente de un compás. En compás de
4/4, un compás tiene cuatro **pulsos**, y el step sequencer divide cada
pulso en cuatro **pasos** (semicorcheas), de modo que un patrón de 16
pasos es un compás. El ritmo de demostración es el groove de baile
clásico:

```text
paso   1 . . . 2 . . . 3 . . . 4 . . .
Bombo  X . . . X . . . X . . . X . . .   «four on the floor»
Palmas . . . . X . . . . . . . X . . .   pulsos 2 y 4 (el backbeat)
Hi-hat . . X . . . X . . . X . . . X .   entre los pulsos (a contratiempo)
```

Los botones de paso alternan colores cada cuatro pasos para que puedas
ver los pulsos.

## Arrays de tamaño fijo y alias de tipo

```rust
pub const MAX_STEPS: usize = 64;
pub type StepRow = [bool; MAX_STEPS];
```

`[bool; 64]` es un **array**: exactamente 64 valores, almacenados en
línea, con la longitud como parte del tipo. A diferencia de un `Vec`,
nunca reserva memoria y no puede crecer. Cada fila tiene sitio para 64
pasos aunque el patrón tenga 16, de modo que cambiar la longitud nunca
reserva memoria, algo que importa porque el hilo de audio guarda copias
de estas filas. Los arrays de valores `Copy` son `Copy` ellos mismos,
así que enviar una fila al hilo de audio solo copia 64 bytes.

`type StepRow = ...` es un **alias de tipo**: un segundo nombre para un
tipo existente, para que las firmas sean más fáciles de leer.

## Combinadores de `Option`

```rust
pub fn is_on(&self, channel: usize, step: usize) -> bool {
    self.rows
        .get(channel)
        .and_then(|row| row.get(step))
        .copied()
        .unwrap_or(false)
}
```

Indexar con `rows[channel]` entra en **pánico** si el índice está fuera
de rango. `.get(channel)` devuelve un `Option` en su lugar, y la cadena
maneja el caso que falta sin ningún `if`:

| Método | Hace |
|--------|------|
| `get(i)` | `Some(&item)` o `None` si está fuera de rango |
| `and_then(f)` | si es `Some(x)`, llama a `f(x)`, que a su vez devuelve un `Option` |
| `copied()` | convierte `Option<&bool>` en `Option<bool>` |
| `unwrap_or(v)` | el valor, o `v` si es `None` |

`toggle` muestra la misma idea con el operador `?`, que también funciona
con `Option`: `None` hace que la función devuelva `None` antes de
tiempo.

```rust
pub fn toggle(&mut self, channel: usize, step: usize) -> Option<bool> {
    let cell = self.rows.get_mut(channel)?.get_mut(step)?;
    *cell = !*cell;
    Some(*cell)
}
```

`get_mut` da una referencia mutable, `&mut bool`, y `*cell` lee o
escribe el valor al que apunta (el `*` lo **desreferencia**).

## Enums con datos modelan los instrumentos

```rust
pub enum Drum { Kick, Snare, Clap, HiHat }

pub enum SampleSource {
    Builtin(Drum),
    File(PathBuf),
}

pub enum ChannelKind {
    Sampler(SampleSource),
    Synth { note: Note },
}
```

Los enums se anidan. Un canal es o un sampler o un sintetizador; el
sonido de un sampler es o un tambor incorporado o un archivo. Cada hecho
se guarda una vez y toda combinación tiene sentido.
`Synth { note: Note }` muestra que una variante también puede tener
**campos con nombre**, como un struct.

`PathBuf` es el tipo estándar para una ruta de archivo poseída. No es un
`String`, porque los nombres de archivo en algunos sistemas no son texto
UTF-8 válido; `path.display()` convierte uno en algo imprimible.

`Drum::ALL` es una constante asociada que contiene todas las variantes,
para que los menús puedan iterar sobre los tambores en lugar de
enumerarlos a mano.

## Mantener un invariante con privacidad

```rust
pub struct Project {
    pub tempo: Tempo,
    channels: Vec<Channel>,
    pattern: Pattern,
}
```

El proyecto debe mantener una fila de patrón por canal, en el mismo
orden. Si ambos campos fueran públicos, cualquier código podría añadir
un canal y olvidar la fila. Por eso son privados, y las únicas formas de
cambiarlos mantienen la regla:

```rust
pub fn add_channel(&mut self, channel: Channel) -> Option<usize> {
    if self.channels.len() >= MAX_CHANNELS {
        return None;
    }
    self.channels.push(channel);
    self.pattern.push_row();
    Some(self.channels.len() - 1)
}
```

Una regla que siempre se cumple sobre un valor se llama **invariante**, y
hacer privados los campos es cómo el código Rust protege uno. `tempo`
sigue siendo público porque todo `Tempo` es válido por construcción, así
que nada puede romperlo.

`Pattern::push_row` está declarado `pub(crate)`: visible en todo
`lineup-core`, pero no para otros crates. Solo `Project` debería añadir
filas, y otros crates no pueden saltárselo.

`channels()` devuelve `&[Channel]`, un **slice**, en lugar de
`&Vec<Channel>`. Quien llama puede leer e iterar, pero no añadir, y el
`Vec` podría cambiarse por otro almacenamiento más adelante sin
afectarle.
