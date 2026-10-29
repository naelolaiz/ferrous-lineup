# Atajos de teclado

La lista propia de la aplicación es **Ayuda > Atajos de teclado**, o
**F1**. Se construye a partir de las mismas constantes de teclas que
leen las ventanas, en
[`crates/lineup-app/src/app/shortcuts.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/shortcuts.rs),
de modo que siempre nombra las teclas que la aplicación está escuchando
de verdad. Esta página es la misma lista en un formato que puedes
imprimir
([Trabajar más rápido](step-9/workflow.md#todas-las-teclas-en-una-lista) describe
la ventana).

Las tablas escriben **Ctrl**, como en Linux y Windows. En macOS, lee
**⌘** por Ctrl y **Option** por Alt. La ventana de F1 dibuja el que use
tu sistema.

## En cualquier sitio

Estas funcionan esté donde esté el teclado, excepto mientras escribes en
un campo de texto.

| Tecla | Qué hace |
|---|---|
| **Espacio** | Reproduce o detiene. En el rack de canales o en el piano roll reproduce o detiene el patrón. |
| **Inicio** | Ir al principio. |
| **Fin** | Ir al final. |
| **RePág** / **AvPág** | Un compás atrás / adelante. |
| **L** | Alternar entre el modo patrón y el modo canción. |
| **F1** | Abrir o cerrar la lista de atajos de teclado. |

## Ventanas

Muestran u ocultan cada ventana o panel.

| Tecla | Qué muestra |
|---|---|
| **F5** | Hoja de acordes |
| **F6** | Rack de canales |
| **F7** | Piano roll |
| **F8** | Teclas de armonía |
| **Alt+F8** | Panel Patrones y Biblioteca |
| **F9** | Mezclador |

## Archivo y Edición

**Archivo**

| Tecla | Qué hace |
|---|---|
| **Ctrl+N** | Nuevo |
| **Ctrl+O** | Abrir |
| **Ctrl+S** | Guardar |
| **Ctrl+Mayús+S** | Guardar como |
| **Ctrl+Q** | Salir |

**Edición**

| Tecla | Qué hace |
|---|---|
| **Ctrl+Z** | Deshacer |
| **Ctrl+Mayús+Z** o **Ctrl+Y** | Rehacer |

## Zoom

Las teclas hacen zoom sobre lo que esté bajo el puntero: la lista de reproducción, el
piano roll, el rack de canales, o toda la interfaz en cualquier otro sitio.
El paso que está bajo el puntero se queda bajo el puntero.

| Tecla | Qué hace |
|---|---|
| **Ctrl++** / **Ctrl+−** | Más ancho / más estrecho |
| **Alt++** / **Alt+−** | Más alto / más bajo |
| **Ctrl+0** | Volver al 100% |
| **Ctrl+rueda** | Más ancho o más estrecho |
| **Alt+rueda** | Más alto o más bajo |

En la mayoría de los teclados, `=` es la tecla `+` sin pulsar Mayús, y
**Ctrl+=** y **Alt+=** hacen zoom de la misma manera. La lista de F1
nombra las formas con `+`.

## Hoja de acordes

Estas funcionan después de hacer clic en un compás.

| Tecla | Qué hace |
|---|---|
| **Flechas**, **Tab**, **Mayús+Tab** | Moverse entre compases. Con Mayús, selecciona los compases por los que pasas. |
| **Intro** o **F2** | Escribir en el compás. Empezar a teclear hace lo mismo. |
| **Supr** o **Retroceso** | Vaciar los compases seleccionados. |
| **Ctrl+A** | Seleccionar todos los compases. |
| **Ctrl+C** | Copiar los compases como texto. |
| **Ctrl+X** | Cortar los compases. |
| **Ctrl+V** | Pegar compases, o un cuadro de acordes entero. |
| **Esc** | No seleccionar nada. |

## Piano roll

Estas actúan sobre las notas seleccionadas, o sobre todas las notas del
canal cuando no hay ninguna seleccionada.

| Tecla | Qué hace |
|---|---|
| **↑** / **↓** | Un semitono arriba / abajo |
| **Mayús+↑** / **Mayús+↓** | Una octava arriba / abajo |
| **Alt+↑** / **Alt+↓** | Una nota de la tonalidad arriba / abajo |
| **←** / **→** | Un paso antes / después |
| **Mayús+←** / **Mayús+→** | Un pulso antes / después |
| **Ctrl+C** | Copiar |
| **Ctrl+X** | Cortar |
| **Ctrl+V** | Pegar |
| **Ctrl+D** | Duplicar |
| **Supr** o **Retroceso** | Borrar |
| **Ctrl+A** | Seleccionar todas las notas |
| **Esc** | No seleccionar nada |

## Lista de reproducción

Estas actúan sobre los clips seleccionados.

| Tecla | Qué hace |
|---|---|
| **Ctrl+C** | Copiar |
| **Ctrl+X** | Cortar |
| **Ctrl+V** | Pegar |
| **Ctrl+D** | Duplicar |
| **Supr** o **Retroceso** | Borrar |
| **Ctrl+A** | Seleccionar todos los clips |
| **Esc** | No seleccionar nada |

## Rack de canales

Estas funcionan después de hacer clic en el nombre de un canal.

| Tecla | Qué hace |
|---|---|
| **Supr** o **Retroceso** | Borrar el canal. |
| **Esc** | No seleccionar ningún canal. |

## Teclas de armonía

| Tecla | Qué hace |
|---|---|
| De **1** a **7** | Tocar el acorde de cada grado de la tonalidad. |

## Teclado de escritura

Estas tocan notas cuando **MIDI > Teclado del equipo como piano** está
activado.

| Tecla | Qué toca |
|---|---|
| **Z X C V B N M** | Teclas blancas desde Do4 |
| **S D G H J** | Teclas negras desde Do♯4 |
| **Q W E R T Y U I O P** | Teclas blancas desde Do5 |
| **2 3 5 6 7 9 0** | Teclas negras desde Do♯5 |

## El editor de estilos

| Tecla | Qué hace |
|---|---|
| **Espacio** | Reproducir el estilo. |
| **Ctrl+S** | Guardar el estilo. |
| **Ctrl+Mayús+S** | Guardar el estilo como. |
| **Ctrl+Z** | Deshacer. |
| **Ctrl+Mayús+Z** o **Ctrl+Y** | Rehacer. |
| **Esc** | No seleccionar ningún canal. |

El editor de estilos tiene su propio historial, separado del de la
canción; mira [El editor de estilos](step-9/style-editor.md).
