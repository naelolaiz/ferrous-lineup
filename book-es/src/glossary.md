# Glosario de Rust

Definiciones breves de los términos de Rust usados en este libro, con el
capítulo que explica cada uno.

| Término | Significado | Se explica en |
|------|---------|--------------|
| **Anonymous lifetime (`'_`)** | Un lifetime escrito sin nombre, que significa «el lifetime del argumento prestado». | [De los pasos a las notas](step-3/notes.md) |
| **`Arc<T>`** | Un puntero compartido con conteo de referencias que es seguro de usar entre hilos; los datos se liberan cuando se va el último `Arc`. | [Instrumentos](step-2/instruments.md) |
| **Array** | Un número fijo de valores guardados en el propio lugar, como `[bool; 64]`; la longitud forma parte del tipo. | [Patrones](step-2/project.md) |
| **Array `map`** | `[T; N]::map` llama a una closure con cada elemento de un array y devuelve un array nuevo de la misma longitud, sin `Vec` y sin `collect`. | [Series dodecafónicas](step-8/twelve-tone.md#todas-las-formas-desde-una-función) |
| **`array::from_fn`** | Construye un array llamando a una closure una vez por cada elemento. | [Voces](step-3/voices.md) |
| **`as` cast** | Convierte entre tipos numéricos, o un enum sin campos a su número: `PitchClass::D as u8` es 2. | [lineup-core](step-1/lineup-core.md) |
| **Associated function** | Una función en un bloque `impl` sin `self`, que se llama como `Type::name()`. `Note::new` es una. | [lineup-core](step-1/lineup-core.md) |
| **Associated type** | Un tipo que elige cada implementación de un trait, como `type Error = FileError;` en un `impl TryFrom`. | [Guardar](step-4/saving.md) |
| **Atomic** | Un valor (como `AtomicUsize`) que los hilos pueden leer y escribir sin un lock. | [Hilo de audio](step-2/realtime.md) |
| **`AtomicU64` for an `f64`** | No hay float atómico, pero `f64::to_bits` convierte un float en el `u64` con los mismos bits, y `f64::from_bits` lo devuelve a float, así que un `AtomicU64` puede compartir uno. | [Teclas en vivo](step-7/live.md) |
| **Backtracking** | Construir una respuesta pieza a pieza, y quitar la última pieza cuando no lleva a ninguna parte; normalmente una función que se llama a sí misma. | [Series dodecafónicas](step-8/twelve-tone.md#qué-hace-interesante-a-una-serie) |
| **Big-endian / `to_be_bytes`** | Escribir primero el byte más significativo de un número; `6u32.to_be_bytes()` es `[0, 0, 0, 6]`. Los archivos MIDI lo usan. | [Mensajes MIDI](step-7/messages.md) |
| **Bit operations** | `&` (y), `\|` (o), `!` (no), `<<` y `>>` (desplazamiento) trabajan sobre los bits de un entero. | [Clases de alturas y escalas](step-3/theory.md) |
| **`bool::then` / `then_some`** | Convierten una condición en un `Option`: `cond.then_some(x)` es `Some(x)` si `cond` es true y `None` si no; `cond.then(\|\| ...)` solo ejecuta la closure cuando es true. | [Herramientas](step-6/tools.md) |
| **Borrow** | Préstamo: usar un valor mediante una referencia (`&T` compartida, `&mut T` mutable) sin tomar su propiedad. | [lineup-audio](step-1/audio-engine.md) |
| **Borrow checker** | La parte del compilador que impone: un `&mut` *o* cualquier número de `&` a la vez, y ninguna referencia que sobreviva a su valor. | [lineup-audio](step-1/audio-engine.md) |
| **`Borrow` trait** | Una promesa de que un tipo con propiedad (como `PathBuf`) hace hash y compara exactamente como el tipo que guarda (`Path`), de modo que un `HashMap<PathBuf, _>` se puede buscar con un `&Path`. | [Lo que cambió](step-5/diff.md) |
| **`Box<T>`** | Un puntero con propiedad a un valor en el montículo. | [lineup-app](step-1/window.md) |
| **Byte string (`b"..."`)** | Un literal de string que es un `&[u8; N]` de códigos ASCII en lugar de un `&str`, como `b"MThd"`. | [Mensajes MIDI](step-7/messages.md) |
| **Capacity** | Cuántos elementos puede guardar un `Vec` antes de tener que reservar más memoria. | [Hilo de audio](step-2/realtime.md) |
| **Cargo** | La herramienta de compilación y el gestor de paquetes de Rust. | [Workspace](step-1/workspace.md) |
| **`cfg` / `cfg_attr`** | Compilación condicional: `#[cfg(target_os = "windows")]` conserva el elemento siguiente solo cuando la condición es cierta, y `cfg_attr` aplica un atributo solo entonces. | [Linux, macOS y Windows](step-4/platforms.md) |
| **`#[cfg(test)]` on an item** | Conserva una función (o `use`, o struct) solo en las compilaciones de test, para que un ayudante al que solo llaman los tests no dé avisos de no usado en una compilación normal. | [Micromodos](step-8/micromodes.md#comprobar-una-tabla-con-pruebas) |
| **`checked_add_signed`** | Suma un número con signo a uno sin signo, como `usize + isize`, y da `None` en lugar de bajar de cero o desbordar. | [Herramientas](step-6/tools.md) |
| **`chunk_by`** | Divide un slice en tramos de vecinos que van juntos, sin copiar. | [La ventana del piano roll](step-3/piano-roll-ui.md) |
| **`chunks` / `chunks_exact`** | Dividen un slice en trozos de n elementos; `chunks_exact` deja fuera el último trozo si queda corto. | [Series dodecafónicas](step-8/twelve-tone.md#qué-hace-interesante-a-una-serie) |
| **`chars` / `Chars::as_str`** | `text.chars()` recorre un string de a un carácter completo («♭» son tres bytes pero un solo `char`); `as_str()` es la parte que aún no se ha recorrido, prestada del mismo texto. | [Nombres de acordes](step-9/chord-names.md#leer-una-nota-y-conservar-el-resto) |
| **Closure** | Una función anónima, `\|args\| body`, que puede capturar variables de su entorno. | [lineup-audio](step-1/audio-engine.md) |
| **Closure captures fields** | Desde la edición 2021, una closure que usa `self.midi_out` pide prestado solo ese campo, no todo `self`, así que los demás campos se pueden pedir prestados a la vez. | [Teclas en vivo](step-7/live.md) |
| **`cmp::Reverse`** | Envuelve un valor para que compare al revés: ordenar por `Reverse(count)` pone primero el conteo más grande. | [Micromodos](step-8/micromodes.md#contar-grupos-de-tres) |
| **Collecting `Option`s** | Recolectar un iterador de `Option<T>` en un `Option<Vec<T>>` da `Some` de todos los valores, o `None` en cuanto un elemento es `None`. `Result` funciona igual. | [Herramientas](step-6/tools.md) |
| **`const fn`** | Una función lo bastante simple como para ejecutarse durante la compilación, de modo que sirve para definir un `const`. | [Guardar](step-4/saving.md) |
| **Constant promotion** | Escribir `&[1, 2, 3]` de valores constantes simples da un préstamo `&'static`, porque el compilador guarda los valores con el programa. Una llamada a función dentro de los corchetes, incluso a un `const fn`, no se promueve. | [La biblioteca](step-8/library.md#macro_rules-y-por-qué-no-una-función) |
| **`Copy`** | Un trait marcador para valores pequeños que se copian en lugar de moverse, como los números y `Note`; `let held = list[i];` copia entonces el elemento fuera. | [lineup-core](step-1/lineup-core.md) |
| **`count_ones`** | Cuántos bits de un entero son 1: el tamaño de un conjunto guardado como bits. | [Series dodecafónicas](step-8/twelve-tone.md#conjuntos-en-bits) |
| **Crate** | Una biblioteca o un programa; la unidad que Cargo compila y publica. | [Workspace](step-1/workspace.md) |
| **`cycle`** | Repite un iterador para siempre. Es seguro porque los iteradores son perezosos: otra cosa, como `zip`, decide cuándo parar. | [Herramientas](step-6/tools.md) |
| **Declarative macro (`macro_rules!`)** | Una macro definida por patrones: `macro_rules! hit { ($at:expr, ...) => { ... } }` hace match con lo que recibe y lo pega en una plantilla, antes de que el código se compile. | [La biblioteca](step-8/library.md#macro_rules-y-por-qué-no-una-función) |
| **`Default`** | Un trait para «el valor por defecto» de un tipo; en un enum, `#[derive(Default)]` necesita `#[default]` en una variante. | [lineup-core](step-1/lineup-core.md) |
| **Deref coercion** | Rust convierte `&Vec<T>` en `&[T]` (o `&String` en `&str`) automáticamente donde hace falta. | [De los pasos a las notas](step-3/notes.md) |
| **Dereference (`*`)** | Sigue una referencia hasta el valor al que apunta, para leerlo o escribirlo. | [Patrones](step-2/project.md) |
| **Derive** | `#[derive(Trait)]` pide al compilador que implemente traits comunes automáticamente. | [lineup-core](step-1/lineup-core.md) |
| **Derive macro** | Código que escribe la implementación de un trait para un tipo; el `#[derive(Serialize)]` de serde viene de un crate, no del compilador. | [Guardar](step-4/saving.md) |
| **`Display`** | El trait detrás de `{}` en `format!` y de `to_string()`. Implementa su método `fmt` una vez, y todos los lugares escriben el valor de la misma manera. | [lineup-core](step-1/lineup-core.md) |
| **Disjoint borrows** | Pedir prestados dos campos distintos de un struct a la vez, incluso uno de forma mutable; permitido porque no pueden solaparse. | [Voces](step-3/voices.md) |
| **`div_ceil`** | División entera que redondea hacia arriba: `16.div_ceil(6)` es 3, mientras que `16 / 6` es 2. | [Micromodos](step-8/micromodes.md#el-recorrido-elegir-con-una-tupla) |
| **`drain(..)`** | Saca todos los elementos de un `Vec`, en orden, y conserva la memoria del `Vec` para más adelante. | [Teclas en vivo](step-7/live.md) |
| **`dyn Trait`** | Un objeto de trait: algún tipo que implementa `Trait`, elegido en tiempo de ejecución. | [lineup-app](step-1/window.md) |
| **Edition** | Una versión del lenguaje (2015, 2018, 2021, 2024) elegida por cada crate. | [Workspace](step-1/workspace.md) |
| **Enum** | Un tipo cuyo valor es una de varias variantes, cada una de las cuales puede guardar datos. | [lineup-audio](step-1/audio-engine.md) |
| **Enum size** | Un enum ocupa tanto como su variante más grande; poner los datos de una variante grande en un `Box` mantiene pequeño cada valor. El lint `large_enum_variant` de Clippy lo señala. | [Teclas en vivo](step-7/live.md) |
| **`expect`** | Como `unwrap`, pero hace panic con el mensaje dado. Afirma una promesa («esto siempre es `Some`»), que conviene respaldar con un test. | [Series dodecafónicas](step-8/twelve-tone.md#la-tabla-de-224-clases-y-cómo-fiarse-de-ella) |
| **Feature** | Una parte opcional de un crate, que se activa o desactiva en `Cargo.toml`; los features solo pueden añadir, nunca quitar. | [Guardar](step-4/saving.md) |
| **`find_map`** | Llama a una closure con cada elemento y devuelve el primer `Some` que dé. | [Acordes](step-3/chords.md) |
| **`flat_map`** | Convierte cada elemento en un iterador y une todos en un único iterador. | [El piano roll, crecido](step-6/piano-roll.md) |
| **Float to integer `as`** | `x as usize` descarta la fracción y *satura*: los números negativos y NaN se vuelven 0, los números demasiado grandes se vuelven el máximo del tipo. Nunca hace panic. | [Grabación](step-7/recording.md) |
| **`fn` pointer** | El tipo de una función simple, como `fn(usize) -> Change`. Las variantes de enum que guardan datos son funciones así: `Change::RemovePattern` construye un `Change` a partir de un `usize`. | [Lo que cambió](step-5/diff.md) |
| **`Fn` / `FnMut` / `FnOnce`** | Los traits que implementan las closures: solo lee, muta sus capturas, o solo se puede llamar una vez. | [Instrumentos](step-2/instruments.md) |
| **`From` / `Into`** | Traits de conversión: implementar `From<A> for B` también da `a.into()`. El operador `?` usa `From` para convertir errores. | [Muchos patrones](step-4/patterns.md) |
| **`FromIterator`** | El trait detrás de `collect()`; impleméntalo para recolectar en tu propio tipo. | [Clases de alturas y escalas](step-3/theory.md) |
| **`FromStr` / `str::parse`** | El trait para «se puede leer desde texto»: implementa `from_str`, y `"Am7".parse::<ChordSymbol>()` funciona, como con los números. | [Nombres de acordes](step-9/chord-names.md#fromstr-am7parse) |
| **Generic** | Código escrito para un tipo comodín `T`, rellenado en tiempo de compilación. | [lineup-audio](step-1/audio-engine.md) |
| **Glob import** | `use super::*;` trae todos los nombres que el otro módulo puede ver, incluidos los elementos privados de un padre cuando se usa desde un hijo. Dos imports con glob que traen el mismo nombre lo hacen ambiguo. | [Más demos](step-8/more-demos.md#use-super-y-qué-pueden-ver-los-hijos) |
| **`HashMap<K, V>`** | Un mapa de claves a valores que encuentra una clave más o menos en el mismo tiempo tenga las entradas que tenga. | [Lo que cambió](step-5/diff.md) |
| **`if let` / `while let`** | Ejecutan un bloque si (o mientras) un valor coincide con un patrón. | [lineup-app](step-1/window.md) |
| **`ilog2`** | El logaritmo entero en base 2: cuántas veces se puede partir un número por la mitad antes de llegar a 1. `8u8.ilog2()` es 3. Hace panic con 0. | [Compases](step-8/time-signatures.md#en-los-archivos-midi) |
| **`impl Trait`** | Como tipo de argumento: cualquier tipo con este trait. Como tipo de retorno: un tipo con este trait, sin nombre. | [Clases de alturas y escalas](step-3/theory.md) |
| **Inner attribute (`#![...]`)** | Un atributo que se aplica a aquello dentro de lo cual está escrito, como todo el crate al principio de `main.rs`. | [Linux, macOS y Windows](step-4/platforms.md) |
| **Invariant** | Una regla que siempre se cumple sobre un valor, normalmente protegida por campos privados. | [Patrones](step-2/project.md) |
| **`is_multiple_of`** | `a.is_multiple_of(b)` es la manera clara de escribir `a % b == 0`, y no hace panic cuando `b` es 0. | [Compases](step-8/time-signatures.md) |
| **`is_some_and`** | `true` si un `Option` es `Some` y su valor pasa una prueba, `false` para `None`. | [Muchos patrones](step-4/patterns.md) |
| **Iterator** | Una secuencia perezosa de valores, transformada con métodos como `map`, `filter`, `count`. | [Tests](step-1/ci.md) |
| **`Iterator::eq`** | Compara dos iteradores elemento a elemento, parando en la primera diferencia, sin recolectarlos en listas. | [Lo que cambió](step-5/diff.md) |
| **`JoinHandle<T>`** | Lo que devuelve `thread::spawn`. `join()` espera a que el hilo acabe y devuelve lo que devolvió su closure, o un `Err` si hizo panic. | [Exportar WAV](step-5/export.md) |
| **Lazy** | Los iteradores no trabajan hasta que algo extrae elementos de ellos. | [De los pasos a las notas](step-3/notes.md) |
| **Leading underscore (`_name`)** | Dice al compilador que una variable o campo no se usa a propósito, para que no avise de ello. | [Exportar WAV](step-5/export.md) |
| **`let ... else`** | Desestructura un patrón, o ejecuta un bloque `else` que debe salir de la función. | [Instrumentos](step-2/instruments.md) |
| **Let chain** | `if cond && let Some(x) = value { ... }`: condiciones y coincidencias de patrones en un solo `if` (Rust 1.88). | [Hilo de audio](step-2/realtime.md) |
| **Literal in a pattern** | `Event::Key { repeat: false, .. }`: un valor escrito en un patrón solo coincide cuando el campo es igual a él. | [Teclas en vivo](step-7/live.md) |
| **`match`** | Compara un valor con patrones; debe cubrir todos los casos. | [lineup-audio](step-1/audio-engine.md) |
| **Match guard** | `pattern if condition =>`: el brazo coincide solo si la condición también es cierta. | [Acordes](step-3/chords.md) |
| **`matches!`** | Una macro que es `true` si un valor coincide con un patrón. | [Clases de alturas y escalas](step-3/theory.md) |
| **`mem::replace`** | Pone un valor nuevo en un lugar y devuelve el viejo. | [Hilo de audio](step-2/realtime.md) |
| **`mem::swap`** | Intercambia los valores detrás de dos referencias `&mut`, sin copiar ninguno. | [Deshacer y rehacer](step-5/history.md) |
| **`mem::take`** | Saca un valor de un lugar y deja detrás el valor por defecto del tipo (como un `Vec` vacío). | [Reproducción de canciones](step-4/song-playback.md) |
| **Method** | Una función en un bloque `impl` que recibe `self`, `&self` o `&mut self`. | [lineup-core](step-1/lineup-core.md) |
| **`min_by_key`** | El elemento de un iterador con la clave más pequeña, o `None` si no hay elementos. | [Series dodecafónicas](step-8/twelve-tone.md#conjuntos-en-bits) |
| **Module** | Un espacio de nombres dentro de un crate, normalmente un archivo, declarado con `mod`. | [Workspace](step-1/workspace.md) |
| **Monomorphization** | El compilador hace una copia especializada del código genérico para cada tipo usado. | [lineup-audio](step-1/audio-engine.md) |
| **Move** | Transferir la propiedad de un valor; el nombre antiguo deja de poderse usar. | [lineup-audio](step-1/audio-engine.md) |
| **`move` closure** | Una closure que toma la propiedad de todo lo que usa en lugar de pedirlo prestado; necesaria para una closure que se entrega a otro hilo. | [Exportar WAV](step-5/export.md) |
| **`mpsc` channel** | Una cola de la biblioteca estándar con muchos emisores (`Sender` se puede clonar) y un receptor; `try_iter` toma lo que está esperando sin esperar más. | [Teclas en vivo](step-7/live.md) |
| **`Mutex<T>` and poisoning** | Un lock que deja a un solo hilo a la vez llegar al `T` de dentro. Si un hilo hace panic mientras lo sostiene, el mutex queda *envenenado*, y `lock` devuelve un error hasta que tomas los datos de todos modos con `PoisonError::into_inner`. | [Teclas en vivo](step-7/live.md) |
| **Newtype** | Un struct con un solo campo que envuelve otro tipo para darle su propio significado. | [lineup-core](step-1/lineup-core.md) |
| **`?` operator** | Sale de la función devolviendo el error si un `Result` es `Err` (o un `Option` es `None`). | [lineup-audio](step-1/audio-engine.md) |
| **`Option::and_then`** | Llama a una closure que devuelve un `Option` sobre el valor, si lo hay; `None` sigue siendo `None`. | [Patrones](step-2/project.md) |
| **`Option::as_mut`** | Convierte `&mut Option<T>` en `Option<&mut T>`, para usar el valor de dentro sin moverlo. | [Teclas en vivo](step-7/live.md) |
| **`Option::filter`** | Conserva un `Some` solo si una closure dice que sí, y da `None` si no. | [Grabación](step-7/recording.md) |
| **`Option<T>`** | `Some(value)` o `None`; el sustituto de Rust para null. | [lineup-audio](step-1/audio-engine.md) |
| **`Option::take`** | Mueve el valor fuera de un `Option` y deja `None` en su lugar; útil para tomar la propiedad de algo guardado en un campo. | [Exportar WAV](step-5/export.md) |
| **Or-pattern** | `A \| B` en un patrón coincide con `A` o con `B`. | [Clases de alturas y escalas](step-3/theory.md) |
| **Ownership** | Cada valor tiene exactamente un dueño, y se libera cuando el dueño desaparece. | [lineup-audio](step-1/audio-engine.md) |
| **`partition`** | Divide un iterador en dos con una closure, en una pasada: los elementos a los que dice que sí y el resto, cada uno recolectado en su propia colección. | [Micromodos](step-8/micromodes.md#texturas) |
| **`partition_point`** | En un slice ordenado de modo que una prueba es `true` para una primera parte y `false` para el resto, el índice donde empieza la parte `false`, hallado por búsqueda binaria. | [Una armonía, cuatro nombres](step-9/harmony-notations.md#encontrar-los-grupos-de-un-compás-con-partition_point) |
| **Panic** | Un error irrecuperable que detiene el hilo actual (`panic!`, `assert!` fallido). | [lineup-core](step-1/lineup-core.md) |
| **Property-based test** | Un test que comprueba una regla que debe cumplirse para cualquier entrada («seguir los changes da el proyecto nuevo»), en lugar de una respuesta para una entrada. | [Lo que cambió](step-5/diff.md) |
| **`pub(crate)`** | Visible en todo el interior del crate actual, pero no para otros crates. | [Patrones](step-2/project.md) |
| **`pub(super)`** | Visible para el módulo padre y todo lo que hay dentro de él. | [Guardar](step-4/saving.md) |
| **Pure function** | Una función cuyo resultado depende solo de sus argumentos y que no cambia nada más; el tipo de código más fácil de probar. | [Herramientas](step-6/tools.md) |
| **`Rc<T>`** | Un puntero con conteo de referencias para un solo hilo: clonarlo suma a un conteo en lugar de copiar el valor, que se libera cuando se va el último `Rc`. `Rc<[T]>` comparte un slice. | [Una armonía, cuatro nombres](step-9/harmony-notations.md#leer-las-notas-solo-cuando-cambian) |
| **RAII / `Drop`** | Ser dueño de un valor significa ser dueño de su recurso; al hacer drop de él se libera el recurso. | [lineup-audio](step-1/audio-engine.md) |
| **Range** | `a..b`, los números desde `a` hasta `b` sin incluirlo; un valor con métodos como `contains`. | [De los pasos a las notas](step-3/notes.md) |
| **Recursion** | Una función que se llama a sí misma sobre un trozo más pequeño del problema, hasta que el trozo es lo bastante pequeño para responder directamente. | [Series dodecafónicas](step-8/twelve-tone.md#qué-hace-interesante-a-una-serie) |
| **`rem_euclid`** | Un resto que nunca es negativo: `(-1_i32).rem_euclid(12)` es 11, mientras que `-1 % 12` es -1. | [Clases de alturas y escalas](step-3/theory.md) |
| **`Result<T, E>`** | `Ok(value)` o `Err(error)`; la manera de Rust de devolver errores. | [lineup-audio](step-1/audio-engine.md) |
| **`retain`** | Conserva solo los elementos de un `Vec` para los que una closure devuelve `true`, en una pasada. | [De los pasos a las notas](step-3/notes.md) |
| **RON** | Rust Object Notation, un formato de texto que se parece a la propia sintaxis de struct de Rust; los archivos de proyecto de Ferrous Lineup lo usan. | [Guardar](step-4/saving.md) |
| **`rsplit_once` / `split_once`** | Divide un string en el último (o primer) lugar donde aparece un patrón, en las partes de antes y de después, o `None` si no aparece. | [Nombres de acordes](step-9/chord-names.md#rsplit_once-y-el-acorde-seis-nueve) |
| **`rposition`** | El índice del último elemento de un slice que coincide, buscando desde el final; `position` busca desde el principio. | [Micromodos](step-8/micromodes.md#el-recorrido-elegir-con-una-tupla) |
| **`saturating_sub`** | Resta que se detiene en el valor más pequeño (0 para `usize`) en lugar de desbordar. | [Voces](step-3/voices.md) |
| **`Send`** | Un trait marcador: los valores de este tipo se pueden mover a otro hilo. | [lineup-audio](step-1/audio-engine.md) |
| **serde** | El crate habitual para convertir valores de Rust en texto o bytes y viceversa, mediante los traits `Serialize` y `Deserialize`. | [Guardar](step-4/saving.md) |
| **serde container attributes** | `#[serde(try_from = "String", into = "String")]` en un tipo lo guarda como el `String` que da su `From`, y lo lee de vuelta a través de su `TryFrom<String>`, de modo que un valor leído de un archivo se comprueba como cualquier otro. | [Series dodecafónicas](step-8/twelve-tone.md#archivos-try_from-e-into) |
| **serde field attributes** | `#[serde(default)]` rellena un campo que falta con su `Default` al leer; `#[serde(skip_serializing_if = "path")]` deja un campo fuera al escribir si esa función lo dice. | [Compases](step-8/time-signatures.md#en-los-archivos-formato-3) |
| **Slice** | Una vista prestada de parte de un array o un `Vec`: `&[T]` o `&mut [T]`. | [lineup-audio](step-1/audio-engine.md) |
| **Slice pattern** | `let [a, b, c, ..] = *bytes else { ... }`: coincide con un slice por su forma, nombrando los primeros elementos, de modo que indexar no puede hacer panic. | [Mensajes MIDI](step-7/messages.md) |
| **Slice pattern with `@`** | `[first @ .., last]` coincide con un slice de dos o más elementos y nombra todo menos el último como el slice `first`. | [Micromodos](step-8/micromodes.md#comprobar-una-tabla-con-pruebas) |
| **State machine** | Código que mantiene un estado actual, a menudo un enum, y pasa de un estado a otro según suceden cosas. | [Voces](step-3/voices.md) |
| **`'static`** | Un lifetime que significa «vive tanto como haga falta»; para closures, «no pide prestado nada temporal». | [lineup-audio](step-1/audio-engine.md) |
| **`static` item** | `static NAME: T = ...;` es un valor en una dirección fija para todo el programa. Un `const`, en cambio, se pega en cada lugar donde se usa. | [La biblioteca](step-8/library.md#estilos) |
| **`step_by`** | Toma un elemento de cada n de un iterador: `(0..8).step_by(2)` es 0, 2, 4, 6. | [Herramientas](step-6/tools.md) |
| **Struct update syntax** | `NoteEvent { length: 2, ..note }`: un struct nuevo con algunos campos cambiados y el resto tomados de `note`. | [De los pasos a las notas](step-3/notes.md) |
| **Target** | El tipo de sistema para el que se compila un programa, como `x86_64-pc-windows-gnu`; `rustup target add` instala la biblioteca de Rust para uno. | [Linux, macOS y Windows](step-4/platforms.md) |
| **`thread::Builder`** | Arranca un hilo con opciones como un nombre; su `spawn` devuelve un `Result` en lugar de hacer panic cuando el sistema se niega. | [Teclas en vivo](step-7/live.md) |
| **Trait** | Un conjunto de métodos que un tipo promete proporcionar, como una interfaz. | [lineup-core](step-1/lineup-core.md) |
| **Trait bound** | Una exigencia sobre un tipo genérico, como `T: FromSample<f32>`. | [lineup-audio](step-1/audio-engine.md) |
| **`TryFrom` / `TryInto`** | Conversiones que pueden fallar, devolviendo un `Result`. | [Guardar](step-4/saving.md) |
| **Tuple** | Un grupo fijo de valores, posiblemente de tipos distintos, como `(usize, Note)`; se puede hacer match y desmontar. | [Clases de alturas y escalas](step-3/theory.md) |
| **Tuple comparison** | Las tuplas comparan elemento a elemento: el segundo elemento solo decide cuando los primeros son iguales. Una tupla como clave de orden da desempates en orden. | [Micromodos](step-8/micromodes.md#el-recorrido-elegir-con-una-tupla) |
| **Tuple struct** | Un struct cuyos campos no tienen nombre, como `struct Note(u8)`; los campos se leen como `.0`, `.1`... | [lineup-core](step-1/lineup-core.md) |
| **Turbofish (`::<>`)** | Escribir el tipo de una función genérica en la llamada, como en `collect::<Vec<Note>>()`, cuando Rust no puede deducirlo. | [lineup-audio](step-1/audio-engine.md) |
| **Type alias** | Un segundo nombre para un tipo existente: `type StepRow = [bool; 64];`. | [Patrones](step-2/project.md) |
| **`VecDeque<T>`** | Una cola de doble extremo: un búfer circular al que es barato añadir y del que es barato quitar por ambos extremos. | [Deshacer y rehacer](step-5/history.md) |
| **`windows`** | Cada tramo de n vecinos de un slice, solapándose: `[1, 2, 3].windows(2)` da `[1, 2]` y `[2, 3]`. | [Tests](step-1/ci.md) |
| **`wrapping_add` / `wrapping_mul`** | Aritmética que da la vuelta al desbordar en lugar de detener una compilación de depuración, para cuando dar la vuelta es lo que quieres, como en los números aleatorios. | [Series dodecafónicas](step-8/twelve-tone.md#qué-hace-interesante-a-una-serie) |
| **Workspace** | Un grupo de crates compilados juntos con un solo `Cargo.lock`. | [Workspace](step-1/workspace.md) |
| **`zip`** | Empareja los elementos de dos iteradores, `(a, b)`, y para cuando cualquiera de los dos se acaba. | [Lo que cambió](step-5/diff.md) |

## Dónde aprender más

- *The Rust Programming Language* («el libro»): se instala con Rust;
  ábrelo sin conexión con `rustup doc --book`.
- *Rust by Example*: `rustup doc --rust-by-example`.
- La documentación de la biblioteca estándar: `rustup doc --std`.
