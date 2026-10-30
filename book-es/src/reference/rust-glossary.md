# Glosario de Rust

Definiciones breves de los términos de Rust usados en este libro, con el
capítulo que explica cada uno.

| Término | Significado | Se explica en |
|------|---------|--------------|
| **Anonymous lifetime (`'_`)** | Un lifetime escrito sin nombre, que significa «el lifetime del argumento prestado». | [Tempo, compás, swing y sensación](../sound/tempo.md#swing) |
| **`Arc<T>`** | Un puntero compartido con conteo de referencias que es seguro de usar entre hilos; los datos se liberan cuando se va el último `Arc`. | [Instrumentos](../sound/instruments.md) |
| **Array** | Un número fijo de valores guardados en el propio lugar, como `[u8; 12]`; la longitud forma parte del tipo. | [Series dodecafónicas y sus reglas](../theory/twelve-tone.md#todas-las-formas-desde-una-función) |
| **Array `map`** | `[T; N]::map` llama a un closure con cada elemento de un array y devuelve un array nuevo de la misma longitud, sin `Vec` y sin `collect`. | [Series dodecafónicas y sus reglas](../theory/twelve-tone.md#todas-las-formas-desde-una-función) |
| **`array::from_fn`** | Construye un array llamando a un closure una vez por cada elemento. | [Instrumentos](../sound/instruments.md) |
| **`as` cast** | Convierte entre tipos numéricos, o un enum sin campos a su número: `PitchClass::D as u8` es 2. | [Cinco crates, y qué puede saber cada uno](../program/crates.md#números-as) |
| **Associated function** | Una función en un bloque `impl` sin `self`, que se llama como `Type::name()`. `Note::new` es una. | [Cinco crates, y qué puede saber cada uno](../program/crates.md#métodos-bloques-impl) |
| **Associated type** | Un tipo que elige cada implementación de un trait, como `type Error = FileError;` en un `impl TryFrom`. | [Guardar](../program/saving.md) |
| **Atomic** | Un valor (como `AtomicUsize`) que los hilos pueden leer y escribir sin un lock. | [Dos hilos](../program/threads.md) |
| **`AtomicU64` for an `f64`** | No hay float atómico, pero `f64::to_bits` convierte un float en el `u64` con los mismos bits, y `f64::from_bits` lo devuelve a float, así que un `AtomicU64` puede compartir uno. | [MIDI](../sound/midi.md) |
| **Backtracking** | Construir una respuesta pieza a pieza, y quitar la última pieza cuando no lleva a ninguna parte; normalmente una función que se llama a sí misma. | [Series dodecafónicas y sus reglas](../theory/twelve-tone.md#qué-hace-interesante-a-una-serie) |
| **Big-endian / `to_be_bytes`** | Escribir primero el byte más significativo de un número; `6u32.to_be_bytes()` es `[0, 0, 0, 6]`. Los archivos MIDI lo usan. | [MIDI](../sound/midi.md) |
| **Bit operations** | `&` (y), `\|` (o), `!` (no), `<<` y `>>` (desplazamiento) trabajan sobre los bits de un entero. | [Clases de altura, notas y su escritura](../theory/pitch.md#un-conjunto-de-clases-de-altura-en-doce-bits) |
| **`bool::then` / `then_some`** | Convierten una condición en un `Option`: `cond.then_some(x)` es `Some(x)` si `cond` es true y `None` si no; `cond.then(\|\| ...)` solo ejecuta el closure cuando es true. | [Herramientas de edición como funciones simples](../theory/tools.md) |
| **Borrow** | Préstamo: usar un valor mediante una referencia (`&T` compartida, `&mut T` mutable) sin tomar su propiedad. | [Dos hilos](../program/threads.md) |
| **Borrow checker** | La parte del compilador que impone: un `&mut` *o* cualquier número de `&` a la vez, y ninguna referencia que sobreviva a su valor. | [Decirle al hilo de audio lo que ha cambiado](../program/sync.md#leer-edit) |
| **`Borrow` trait** | Una promesa de que un tipo con propiedad (como `PathBuf`) hace hash y compara exactamente como el tipo que guarda (`Path`), de modo que un `HashMap<PathBuf, _>` se puede buscar con un `&Path`. | [Decirle al hilo de audio lo que ha cambiado](../program/sync.md) |
| **`Box<T>`** | Un puntero con propiedad a un valor en el montículo. | [Dibujar las ventanas con egui](../program/egui.md) |
| **Byte string (`b"..."`)** | Un literal de string que es un `&[u8; N]` de códigos ASCII en lugar de un `&str`, como `b"MThd"`. | [MIDI](../sound/midi.md) |
| **Capacity** | Cuántos elementos puede guardar un `Vec` antes de tener que reservar más memoria. | [Dos hilos](../program/threads.md) |
| **Cargo** | La herramienta de compilación y el gestor de paquetes de Rust. | [Cinco crates, y qué puede saber cada uno](../program/crates.md) |
| **`Cell<T>`** | Una caja cuyo valor se puede reemplazar a través de una referencia compartida, moviendo valores enteros hacia dentro y hacia fuera con `get` y `set` (para tipos `Copy`); nada puede tener una referencia a lo de dentro, así que nada lo lee mientras cambia. | [Números romanos, las teclas de armonía y las funciones de los acordes](../theory/numerals.md#un-ajuste-que-toda-función-puede-ver-uno-por-hilo) |
| **`cfg` / `cfg_attr`** | Compilación condicional: `#[cfg(target_os = "windows")]` conserva el elemento siguiente solo cuando la condición es cierta, y `cfg_attr` aplica un atributo solo entonces. | [Tres sistemas e integración continua](../shipping/ci.md) |
| **`#[cfg(test)]` on an item** | Conserva una función (o `use`, o struct) solo en las compilaciones de test, para que un ayudante al que solo llaman los tests no dé avisos de no usado en una compilación normal. | [Micromodos](../theory/micromodes.md#comprobar-una-tabla-con-pruebas) |
| **`checked_add_signed`** | Suma un número con signo a uno sin signo, como `usize + isize`, y da `None` en lugar de bajar de cero o desbordar. | [Herramientas de edición como funciones simples](../theory/tools.md) |
| **`chunk_by`** | Divide un slice en tramos de vecinos que van juntos, sin copiar. | [Los editores](../program/editors.md) |
| **`chunks` / `chunks_exact`** | Dividen un slice en trozos de n elementos; `chunks_exact` deja fuera el último trozo si queda corto. | [Series dodecafónicas y sus reglas](../theory/twelve-tone.md#qué-hace-interesante-a-una-serie) |
| **`chars` / `Chars::as_str`** | `text.chars()` recorre un string de a un carácter completo («♭» son tres bytes pero un solo `char`); `as_str()` es la parte que aún no se ha recorrido, prestada del mismo texto. | [Clases de altura, notas y su escritura](../theory/pitch.md#escribir-y-leer-un-nombre) |
| **Closure** | Una función anónima, `\|args\| body`, que puede capturar variables de su entorno. | [Dos hilos](../program/threads.md) |
| **Closure captures fields** | Desde la edición 2021, un closure que usa `self.plays` pide prestado solo ese campo, no todo `self`, así que los demás campos se pueden pedir prestados a la vez. | [El secuenciador](../sound/sequencer.md#cuando-la-canción-cambia-bajo-una-nota-que-suena) |
| **`cmp::Reverse`** | Envuelve un valor para que compare al revés: ordenar por `Reverse(count)` pone primero el conteo más grande. | [Micromodos](../theory/micromodes.md#contar-grupos-de-tres) |
| **Collecting `Option`s** | Recolectar un iterador de `Option<T>` en un `Option<Vec<T>>` da `Some` de todos los valores, o `None` en cuanto un elemento es `None`. `Result` funciona igual. | [Herramientas de edición como funciones simples](../theory/tools.md) |
| **`const fn`** | Una función lo bastante simple como para ejecutarse durante la compilación, de modo que sirve para definir un `const`. | [Guardar](../program/saving.md) |
| **Constant promotion** | Escribir `&[1, 2, 3]` de valores constantes simples da un préstamo `&'static`, porque el compilador guarda los valores con el programa. Una llamada a función dentro de los corchetes, incluso a un `const fn`, no se promueve. | [La biblioteca](../band/library.md#macro_rules-y-por-qué-no-una-función) |
| **`Copy`** | Un trait marcador para valores pequeños que se copian en lugar de moverse, como los números y `Note`; `let held = list[i];` copia entonces el elemento fuera. | [Cinco crates, y qué puede saber cada uno](../program/crates.md#traits-y-derive) |
| **`count_ones`** | Cuántos bits de un entero son 1: el tamaño de un conjunto guardado como bits. | [Clases de altura, notas y su escritura](../theory/pitch.md#un-conjunto-de-clases-de-altura-en-doce-bits) |
| **Crate** | Una biblioteca o un programa; la unidad que Cargo compila y publica. | [Cinco crates, y qué puede saber cada uno](../program/crates.md) |
| **`cycle`** | Repite un iterador para siempre. Es seguro porque los iteradores son perezosos: otra cosa, como `zip`, decide cuándo parar. | [Herramientas de edición como funciones simples](../theory/tools.md) |
| **Declarative macro (`macro_rules!`)** | Una macro definida por patrones: `macro_rules! hit { ($at:expr, ...) => { ... } }` hace match con lo que recibe y lo pega en una plantilla, antes de que el código se compile. | [La biblioteca](../band/library.md#macro_rules-y-por-qué-no-una-función) |
| **`Default`** | Un trait para «el valor por defecto» de un tipo; en un enum, `#[derive(Default)]` necesita `#[default]` en una variante. | [Cinco crates, y qué puede saber cada uno](../program/crates.md#traits-y-derive) |
| **Deref coercion** | Rust convierte `&Vec<T>` en `&[T]` (o `&String` en `&str`) automáticamente donde hace falta. | [MIDI](../sound/midi.md#escribir-uno-dos-o-tres-bytes) |
| **Dereference (`*`)** | Sigue una referencia hasta el valor al que apunta, para leerlo o escribirlo. | [El proyecto](../program/project.md) |
| **Derive** | `#[derive(Trait)]` pide al compilador que implemente traits comunes automáticamente. | [Cinco crates, y qué puede saber cada uno](../program/crates.md#traits-y-derive) |
| **Derive macro** | Código que escribe la implementación de un trait para un tipo; el `#[derive(Serialize)]` de serde viene de un crate, no del compilador. | [Guardar](../program/saving.md) |
| **`Display`** | El trait detrás de `{}` en `format!` y de `to_string()`. Implementa su método `fmt` una vez, y todos los lugares escriben el valor de la misma manera. | [Cinco crates, y qué puede saber cada uno](../program/crates.md#traits-y-derive) |
| **Disjoint borrows** | Pedir prestados dos campos distintos de un struct a la vez, incluso uno de forma mutable; permitido porque no pueden solaparse. | [Dibujar las ventanas con egui](../program/egui.md#un-tema-es-una-función) |
| **`div_ceil`** | División entera que redondea hacia arriba: `16.div_ceil(6)` es 3, mientras que `16 / 6` es 2. | [Micromodos](../theory/micromodes.md#el-recorrido-elegir-con-una-tupla) |
| **`drain(..)`** | Saca todos los elementos de un `Vec`, en orden, y conserva la memoria del `Vec` para más adelante. | [MIDI](../sound/midi.md) |
| **`dyn Trait`** | Un objeto de trait: algún tipo que implementa `Trait`, elegido en tiempo de ejecución. | [Guardar](../program/saving.md#errores-como-enum) |
| **Edition** | Una versión del lenguaje (2015, 2018, 2021, 2024) elegida por cada crate. | [Cinco crates, y qué puede saber cada uno](../program/crates.md) |
| **Enum** | Un tipo cuyo valor es una de varias variantes, cada una de las cuales puede guardar datos. | [Dos hilos](../program/threads.md) |
| **Enum size** | Un enum ocupa tanto como su variante más grande; poner los datos de una variante grande en un `Box` mantiene pequeño cada valor. El lint `large_enum_variant` de Clippy lo señala. | [MIDI](../sound/midi.md#salida-midi-un-instrumento-con-bandeja-de-salida) |
| **`env!` y `concat!`** | Macros que se ejecutan mientras se compila el programa: `env!("CARGO_PKG_VERSION")` se convierte en la versión que Cargo leyó de `Cargo.toml`, y `concat!` une literales en un solo `&'static str`, así que ambas pueden construir una `const`. | [Dibujar las ventanas con egui](../program/egui.md#ayuda-el-manual-y-acerca-de) |
| **`expect`** | Como `unwrap`, pero entra en pánico con el mensaje dado. Afirma una promesa («esto siempre es `Some`»), que conviene respaldar con un test. | [Conjuntos de clases de altura, y una armonía con cuatro nombres](../theory/sets.md#224-clases-en-una-tabla) |
| **Feature** | Una parte opcional de un crate, que se activa o desactiva en `Cargo.toml`; los features solo pueden añadir, nunca quitar. | [Guardar](../program/saving.md) |
| **`find_map`** | Llama a un closure con cada elemento y devuelve el primer `Some` que dé. | [Acordes](../theory/chords.md) |
| **`flat_map`** | Convierte cada elemento en un iterador y une todos en un único iterador. | [Los editores](../program/editors.md) |
| **Float to integer `as`** | `x as usize` descarta la fracción y *satura*: los números negativos y NaN se vuelven 0, los números demasiado grandes se vuelven el máximo del tipo. Nunca entra en pánico. | [MIDI](../sound/midi.md) |
| **`fn` pointer** | El tipo de una función simple, como `fn(usize) -> Change`. Las variantes de enum que guardan datos son funciones así: `Change::RemovePattern` construye un `Change` a partir de un `usize`. | [Decirle al hilo de audio lo que ha cambiado](../program/sync.md) |
| **`Fn` / `FnMut` / `FnOnce`** | Los traits que implementan las closures: solo lee, muta sus capturas, o solo se puede llamar una vez. | [Instrumentos](../sound/instruments.md#closures-que-recuerdan-fnmut) |
| **`From` / `Into`** | Traits de conversión: implementar `From<A> for B` también da `a.into()`. El operador `?` usa `From` para convertir errores. | [El proyecto](../program/project.md#nombres-e-impl-intostring) |
| **`FromIterator`** | El trait detrás de `collect()`; impleméntalo para recolectar en tu propio tipo. | [Clases de altura, notas y su escritura](../theory/pitch.md) |
| **`FromStr` / `str::parse`** | El trait para «se puede leer desde texto»: implementa `from_str`, y `"Am7".parse::<ChordSymbol>()` funciona, como con los números. | [Acordes](../theory/chords.md#leer-un-símbolo-entero) |
| **Generic** | Código escrito para un tipo comodín `T`, rellenado en tiempo de compilación. | [Dos hilos](../program/threads.md#genéricos-una-función-muchos-tipos-de-muestra) |
| **Glob import** | `use super::*;` trae todos los nombres que el otro módulo puede ver, incluidos los elementos privados de un padre cuando se usa desde un hijo. Dos imports con glob que traen el mismo nombre lo hacen ambiguo. | [Cómo se escribe una demostración](../songs/demos.md#un-archivo-por-género) |
| **`HashMap<K, V>`** | Un mapa de claves a valores que encuentra una clave más o menos en el mismo tiempo tenga las entradas que tenga. | [Decirle al hilo de audio lo que ha cambiado](../program/sync.md) |
| **`if let` / `while let`** | Ejecutan un bloque si (o mientras) un valor coincide con un patrón. | [Dibujar las ventanas con egui](../program/egui.md) |
| **`ilog2`** | El logaritmo entero en base 2: cuántas veces se puede partir un número por la mitad antes de llegar a 1. `8u8.ilog2()` es 3. Entra en pánico con 0. | [Tempo, compás, swing y sensación](../sound/tempo.md#en-archivos-y-en-archivos-midi) |
| **`impl Trait`** | Como tipo de argumento: cualquier tipo con este trait. Como tipo de retorno: un tipo con este trait, sin nombre. | [El proyecto](../program/project.md#nombres-e-impl-intostring) |
| **Inner attribute (`#![...]`)** | Un atributo que se aplica a aquello dentro de lo cual está escrito, como todo el crate al principio de `main.rs`. | [Tres sistemas e integración continua](../shipping/ci.md) |
| **Invariant** | Una regla que siempre se cumple sobre un valor, normalmente protegida por campos privados. | [Series dodecafónicas y sus reglas](../theory/twelve-tone.md#una-serie-en-rust-tonerow) |
| **`is_some_and`** | `true` si un `Option` es `Some` y su valor pasa una prueba, `false` para `None`. | [Números romanos, las teclas de armonía y las funciones de los acordes](../theory/numerals.md#qué-papel-va-primero) |
| **Iterator** | Una secuencia perezosa de valores, transformada con métodos como `map`, `filter`, `count`. | [Cómo se prueba el código](../shipping/testing.md) |
| **`JoinHandle<T>`** | Lo que devuelve `thread::spawn`. `join()` espera a que el hilo acabe y devuelve lo que devolvió su closure, o un `Err` si entró en pánico. | [Exportar](../sound/export.md) |
| **Lazy** | Los iteradores no trabajan hasta que algo extrae elementos de ellos. | [Herramientas de edición como funciones simples](../theory/tools.md#una-nota-de-la-tonalidad-más-arriba-find_map) |
| **Leading underscore (`_name`)** | Dice al compilador que una variable o campo no se usa a propósito, para que no avise de ello. | [Exportar](../sound/export.md) |
| **`let ... else`** | Desestructura un patrón, o ejecuta un bloque `else` que debe salir de la función. | [Decirle al hilo de audio lo que ha cambiado](../program/sync.md#el-orden-de-todo) |
| **Let chain** | `if cond && let Some(x) = value { ... }`: condiciones y coincidencias de patrones en un solo `if` (Rust 1.88). | [Dos hilos](../program/threads.md) |
| **Literal in a pattern** | `Event::Key { repeat: false, .. }`: un valor escrito en un patrón solo coincide cuando el campo es igual a él. | [MIDI](../sound/midi.md#el-teclado-del-equipo) |
| **`match`** | Compara un valor con patrones; debe cubrir todos los casos. | [Dos hilos](../program/threads.md) |
| **Match guard** | `pattern if condition =>`: el brazo coincide solo si la condición también es cierta. | [Acordes](../theory/chords.md#etiquetas-guardas-de-match-y-patrones-anidados) |
| **`matches!`** | Una macro que es `true` si un valor coincide con un patrón. | [Clases de altura, notas y su escritura](../theory/pitch.md) |
| **`mem::replace`** | Pone un valor nuevo en un lugar y devuelve el viejo. | [Dos hilos](../program/threads.md) |
| **`mem::swap`** | Intercambia los valores detrás de dos referencias `&mut`, sin copiar ninguno. | [Deshacer y rehacer](../program/undo.md) |
| **`mem::take`** | Saca un valor de un lugar y deja detrás el valor por defecto del tipo (como un `Vec` vacío). | [El secuenciador](../sound/sequencer.md) |
| **Method** | Una función en un bloque `impl` que recibe `self`, `&self` o `&mut self`. | [Cinco crates, y qué puede saber cada uno](../program/crates.md#métodos-bloques-impl) |
| **`min_by_key`** | El elemento de un iterador con la clave más pequeña, o `None` si no hay elementos. | [Conjuntos de clases de altura, y una armonía con cuatro nombres](../theory/sets.md#la-teoría-de-conjuntos-en-doce-bits) |
| **Module** | Un espacio de nombres dentro de un crate, normalmente un archivo, declarado con `mod`. | [Cinco crates, y qué puede saber cada uno](../program/crates.md) |
| **Monomorphization** | El compilador hace una copia especializada del código genérico para cada tipo usado. | [Dos hilos](../program/threads.md) |
| **Move** | Transferir la propiedad de un valor; el nombre antiguo deja de poderse usar. | [Dos hilos](../program/threads.md) |
| **`move` closure** | Un closure que toma la propiedad de todo lo que usa en lugar de pedirlo prestado; necesario para un closure que se entrega a otro hilo. | [Exportar](../sound/export.md) |
| **`mpsc` channel** | Una cola de la biblioteca estándar con muchos emisores (`Sender` se puede clonar) y un receptor; `try_iter` toma lo que está esperando sin esperar más. | [MIDI](../sound/midi.md) |
| **`Mutex<T>` and poisoning** | Un lock que deja a un solo hilo a la vez llegar al `T` de dentro. Si un hilo entra en pánico mientras lo sostiene, el mutex queda *envenenado*, y `lock` devuelve un error hasta que tomas los datos de todos modos con `PoisonError::into_inner`. | [MIDI](../sound/midi.md) |
| **Newtype** | Un struct con un solo campo que envuelve otro tipo para darle su propio significado. | [Clases de altura, notas y su escritura](../theory/pitch.md#una-nota-un-número-midi-con-un-tipo-propio) |
| **`?` operator** | Sale de la función devolviendo el error si un `Result` es `Err` (o un `Option` es `None`). | [Dos hilos](../program/threads.md) |
| **`Option::and_then`** | Llama a un closure que devuelve un `Option` sobre el valor, si lo hay; `None` sigue siendo `None`. | [Los editores](../program/editors.md#mover-un-grupo) |
| **`Option::as_mut`** | Convierte `&mut Option<T>` en `Option<&mut T>`, para usar el valor de dentro sin moverlo. | [Dos hilos](../program/threads.md#audiothreadprocess) |
| **`Option::filter`** | Conserva un `Some` solo si un closure dice que sí, y da `None` si no. | [MIDI](../sound/midi.md) |
| **`Option<T>`** | `Some(value)` o `None`; el sustituto de Rust para null. | [Dos hilos](../program/threads.md) |
| **`Option::take`** | Mueve el valor fuera de un `Option` y deja `None` en su lugar; útil para tomar la propiedad de algo guardado en un campo. | [Exportar](../sound/export.md) |
| **Or-pattern** | `A \| B` en un patrón coincide con `A` o con `B`. | [Dibujar las ventanas con egui](../program/egui.md#mainrs) |
| **Ownership** | Cada valor tiene exactamente un dueño, y se libera cuando el dueño desaparece. | [Dos hilos](../program/threads.md) |
| **`partition`** | Divide un iterador en dos con un closure, en una pasada: los elementos a los que dice que sí y el resto, cada uno recolectado en su propia colección. | [Micromodos](../theory/micromodes.md#texturas) |
| **`partition_point`** | En un slice ordenado de modo que una prueba es `true` para una primera parte y `false` para el resto, el índice donde empieza la parte `false`, hallado por búsqueda binaria. | [Conjuntos de clases de altura, y una armonía con cuatro nombres](../theory/sets.md#leer-solo-cuando-algo-cambia) |
| **Panic** | Un error irrecuperable que detiene el hilo actual (`panic!`, `assert!` fallido). | [Cinco crates, y qué puede saber cada uno](../program/crates.md#pánicos) |
| **Property-based test** | Un test que comprueba una regla que debe cumplirse para cualquier entrada («seguir los changes da el proyecto nuevo»), en lugar de una respuesta para una entrada. | [Decirle al hilo de audio lo que ha cambiado](../program/sync.md#probar-con-un-modelo) |
| **`pub(crate)`** | Visible en todo el interior del crate actual, pero no para otros crates. | [El proyecto](../program/project.md#repartir-mut-pattern-con-seguridad) |
| **`pub(super)`** | Visible para el módulo padre y todo lo que hay dentro de él. | [Guardar](../program/saving.md) |
| **Pure function** | Una función cuyo resultado depende solo de sus argumentos y que no cambia nada más; el tipo de código más fácil de probar. | [Herramientas de edición como funciones simples](../theory/tools.md) |
| **`Rc<T>`** | Un puntero con conteo de referencias para un solo hilo: clonarlo suma a un conteo en lugar de copiar el valor, que se libera cuando se va el último `Rc`. `Rc<[T]>` comparte un slice. | [Conjuntos de clases de altura, y una armonía con cuatro nombres](../theory/sets.md#leer-solo-cuando-algo-cambia) |
| **RAII / `Drop`** | Ser dueño de un valor significa ser dueño de su recurso; al hacer drop de él se libera el recurso. | [Dos hilos](../program/threads.md) |
| **Range** | `a..b`, los números desde `a` hasta `b` sin incluirlo; un valor con métodos como `contains`. | [El secuenciador](../sound/sequencer.md#probar-con-iteradores) |
| **Recursion** | Una función que se llama a sí misma sobre un trozo más pequeño del problema, hasta que el trozo es lo bastante pequeño para responder directamente. | [Series dodecafónicas y sus reglas](../theory/twelve-tone.md#qué-hace-interesante-a-una-serie) |
| **`rem_euclid`** | Un resto que nunca es negativo: `(-1_i32).rem_euclid(12)` es 11, mientras que `-1 % 12` es -1. | [Clases de altura, notas y su escritura](../theory/pitch.md) |
| **`Result<T, E>`** | `Ok(value)` o `Err(error)`; la manera de Rust de devolver errores. | [Dos hilos](../program/threads.md) |
| **`retain`** | Conserva solo los elementos de un `Vec` para los que un closure devuelve `true`, en una pasada. | [El proyecto](../program/project.md#cuatro-reglas-un-guardián) |
| **RON** | Rust Object Notation, un formato de texto que se parece a la propia sintaxis de struct de Rust; los archivos de proyecto de Ferrous Lineup lo usan. | [Guardar](../program/saving.md) |
| **`rsplit_once` / `split_once`** | Divide un string en el último (o primer) lugar donde aparece un patrón, en las partes de antes y de después, o `None` si no aparece. | [Acordes](../theory/chords.md#acordes-con-barra-y-el-acorde-de-sexta-y-novena) |
| **`rposition`** | El índice del último elemento de un slice que coincide, buscando desde el final; `position` busca desde el principio. | [Micromodos](../theory/micromodes.md#el-recorrido-elegir-con-una-tupla) |
| **`saturating_sub`** | Resta que se detiene en el valor más pequeño (0 para `usize`) en lugar de desbordar. | [Herramientas de edición como funciones simples](../theory/tools.md#arpegios-cycle-step_by-y-zip) |
| **`Send`** | Un trait marcador: los valores de este tipo se pueden mover a otro hilo. | [Dos hilos](../program/threads.md#el-callback-de-audio-es-un-closure) |
| **serde** | El crate habitual para convertir valores de Rust en texto o bytes y viceversa, mediante los traits `Serialize` y `Deserialize`. | [Guardar](../program/saving.md) |
| **serde container attributes** | `#[serde(try_from = "String", into = "String")]` en un tipo lo guarda como el `String` que da su `From`, y lo lee de vuelta a través de su `TryFrom<String>`, de modo que un valor leído de un archivo se comprueba como cualquier otro. | [Series dodecafónicas y sus reglas](../theory/twelve-tone.md#archivos-try_from-e-into) |
| **serde field attributes** | `#[serde(default)]` rellena un campo que falta con su `Default` al leer; `#[serde(skip_serializing_if = "path")]` deja un campo fuera al escribir si esa función lo dice. | [Guardar](../program/saving.md#serde-describir-los-datos-una-sola-vez) |
| **Slice** | Una vista prestada de parte de un array o un `Vec`: `&[T]` o `&mut [T]`. | [Dos hilos](../program/threads.md#el-callback-de-audio-es-un-closure) |
| **Slice pattern** | `let [a, b, c, ..] = *bytes else { ... }`: coincide con un slice por su forma, nombrando los primeros elementos, de modo que indexar no puede entrar en pánico. | [MIDI](../sound/midi.md) |
| **Slice pattern with `@`** | `[first @ .., last]` coincide con un slice de dos o más elementos y nombra todo menos el último como el slice `first`. | [Micromodos](../theory/micromodes.md#comprobar-una-tabla-con-pruebas) |
| **State machine** | Código que mantiene un estado actual, a menudo un enum, y pasa de un estado a otro según suceden cosas. | [Instrumentos](../sound/instruments.md) |
| **`'static`** | Un lifetime que significa «vive tanto como haga falta»; para closures, «no pide prestado nada temporal». | [Dos hilos](../program/threads.md#el-callback-de-audio-es-un-closure) |
| **`static` item** | `static NAME: T = ...;` es un valor en una dirección fija para todo el programa. Un `const`, en cambio, se pega en cada lugar donde se usa. | [La biblioteca](../band/library.md#estilos-una-tabla-en-un-static) |
| **`step_by`** | Toma un elemento de cada n de un iterador: `(0..8).step_by(2)` es 0, 2, 4, 6. | [Herramientas de edición como funciones simples](../theory/tools.md#arpegios-cycle-step_by-y-zip) |
| **Struct update syntax** | `NoteEvent { length: 2, ..note }`: un struct nuevo con algunos campos cambiados y el resto tomados de `note`. | [Dibujar las ventanas con egui](../program/egui.md#mainrs) |
| **Target** | El tipo de sistema para el que se compila un programa, como `x86_64-pc-windows-gnu`; `rustup target add` instala la biblioteca de Rust para uno. | [Tres sistemas e integración continua](../shipping/ci.md) |
| **`thread::Builder`** | Arranca un hilo con opciones como un nombre; su `spawn` devuelve un `Result` en lugar de entrar en pánico cuando el sistema se niega. | [MIDI](../sound/midi.md) |
| **`thread_local!`** | Declara un valor del que cada hilo tiene su propia copia, al que se llega con `.with(...)`; la app guarda así sus ajustes de números romanos, nombres de notas e idioma, para que las pruebas que corren a la vez nunca vean los de las demás. | [Números romanos, las teclas de armonía y las funciones de los acordes](../theory/numerals.md#un-ajuste-que-toda-función-puede-ver-uno-por-hilo) |
| **Trait** | Un conjunto de métodos que un tipo promete proporcionar, como una interfaz. | [Cinco crates, y qué puede saber cada uno](../program/crates.md#traits-y-derive) |
| **Trait bound** | Una exigencia sobre un tipo genérico, como `T: FromSample<f32>`. | [Dos hilos](../program/threads.md) |
| **`TryFrom` / `TryInto`** | Conversiones que pueden fallar, devolviendo un `Result`. | [Guardar](../program/saving.md) |
| **Tuple** | Un grupo fijo de valores, posiblemente de tipos distintos, como `(usize, Note)`; se puede hacer match y desmontar. | [Clases de altura, notas y su escritura](../theory/pitch.md#un-match-sobre-dos-valores-a-la-vez) |
| **Tuple comparison** | Las tuplas comparan elemento a elemento: el segundo elemento solo decide cuando los primeros son iguales. Una tupla como clave de orden da desempates en orden. | [El mezclador](../sound/mixer.md#ordenar-y-agrupar) |
| **Tuple struct** | Un struct cuyos campos no tienen nombre, como `struct Note(u8)`; los campos se leen como `.0`, `.1`... | [Cinco crates, y qué puede saber cada uno](../program/crates.md#un-primer-tipo-note) |
| **Turbofish (`::<>`)** | Escribir el tipo de una función genérica en la llamada, como en `collect::<Vec<Note>>()`, cuando Rust no puede deducirlo. | [Dos hilos](../program/threads.md#genéricos-una-función-muchos-tipos-de-muestra) |
| **Type alias** | Un segundo nombre para un tipo existente: `type Hit = (u8, i32);`. | [La biblioteca](../band/library.md#ritmos-una-línea-por-tambor) |
| **`VecDeque<T>`** | Una cola de doble extremo: un búfer circular al que es barato añadir y del que es barato quitar por ambos extremos. | [Deshacer y rehacer](../program/undo.md) |
| **`windows`** | Cada tramo de n vecinos de un slice, solapándose: `[1, 2, 3].windows(2)` da `[1, 2]` y `[2, 3]`. | [Cómo se prueba el código](../shipping/testing.md) |
| **`wrapping_add` / `wrapping_mul`** | Aritmética que da la vuelta al desbordar en lugar de detener una compilación de depuración, para cuando dar la vuelta es lo que quieres, como en los números aleatorios. | [Series dodecafónicas y sus reglas](../theory/twelve-tone.md#qué-hace-interesante-a-una-serie) |
| **Workspace** | Un grupo de crates compilados juntos con un solo `Cargo.lock`. | [Cinco crates, y qué puede saber cada uno](../program/crates.md) |
| **`zip`** | Empareja los elementos de dos iteradores, `(a, b)`, y para cuando cualquiera de los dos se acaba. | [Decirle al hilo de audio lo que ha cambiado](../program/sync.md) |

## Dónde aprender más

- *The Rust Programming Language* («el libro»): se instala con Rust;
  ábrelo sin conexión con `rustup doc --book`.
- *Rust by Example*: `rustup doc --rust-by-example`.
- La documentación de la biblioteca estándar: `rustup doc --std`.
