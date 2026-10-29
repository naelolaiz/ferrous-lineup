# Tus propios estilos

Los estilos de la Biblioteca son un buen comienzo, pero el ritmo que
oyes en tu cabeza puede no estar ahí. El **editor de estilos** te deja
escribir tus propias partes de banda, con el rack de canales y el piano
roll que ya conoces, y guardarlas como un estilo que se pone en la
Biblioteca junto a los demás.

![El editor de estilos sobre una copia de Bossa nova: Empezar desde…, Guardar estilo, Guardar como…, Deshacer, Rehacer, Escuchar ×4 y Detener la escucha arriba, luego Nombre, BPM, el compás, Compases, Swing, Progresión, Descripción y Asignación de partes, y los canales del estilo: Piano, Bajo, Bombo, Charles, Aro y Guitarra](images/style-editor.png)

## Abrirlo

- En el **rack de canales**, **Editar estilo…** abre una copia del
  estilo que toca la canción, y **Nuevo estilo…** empieza uno vacío.
  **Nuevo estilo…** también está arriba en la Biblioteca.
- **Ver > Editor de estilos…** abre el editor, o lo trae al frente.
- En la **Biblioteca**, haz clic derecho en un estilo: uno de los que
  trae la aplicación dice **Editar una copia en el rack de canales…**,
  uno tuyo **Editar en el rack de canales…**.

Un estilo de la aplicación se abre como copia, llamada «… (copia)», para
que el original quede como está. Uno tuyo se abre tal cual, y **Guardar
estilo** lo vuelve a escribir en su archivo.

## Qué tiene un estilo

Un estilo propio es **un patrón que se repite**, de uno a dieciséis
compases, con un canal para cada parte: batería, un piano u otra parte
de acordes, un bajo, y la guitarra, las cuerdas, el órgano o el solista
de una banda más grande.

Arriba: **Empezar desde…**, **Guardar estilo**, **Guardar como…**,
**Deshacer**, **Rehacer**, **Escuchar ×4** y **Detener la escucha**.
Debajo:

| Campo | Qué es |
|---|---|
| **Nombre** | el nombre del estilo en la Biblioteca |
| **BPM** | su tempo habitual |
| compás | su métrica |
| **Compases** | cuánto dura el patrón, de 1 a 16 compases |
| **Swing** | su swing (ver [Swing y sensación](swing.md)) |
| **Progresión** | una progresión de acordes que trae, en números romanos o nombres de acordes, con los compases separados por `|`: `I | vi | ii7 | V7`. Déjala vacía para un estilo que toque tus acordes |
| **Descripción** | una frase, que aparece al señalar el estilo en la Biblioteca |
| **Asignación de partes** | qué músico es cada canal: batería, piano, bajo, guitarra, etc. |

Debajo hay un rack de canales con los canales del estilo. Haz clic en
los pasos para escribir la batería, como en cualquier patrón. Haz clic
en la vista previa de notas de un canal para abrir su piano roll, donde
dibujar, mover y estirar notas y el menú **Herramientas** funcionan como
en la canción. **+ Añadir canal** añade un canal.

**Escribe las partes con altura sobre Do mayor.** Al aplicar el estilo,
cada nota pasa al acorde del momento: un Do toca la fundamental del
acorde, un Mi su tercera, un Sol su quinta, y las demás notas siguen la
tonalidad o conservan su color. Así, una línea de bajo escrita como Do,
Sol, Do, Mi en el editor toca la fundamental, la quinta, la fundamental
y la tercera de cualquier acorde que tenga la canción.

**Empezar desde…** copia un estilo existente en el editor, desde la
misma lista que la Biblioteca, con su búsqueda; **Estilo vacío** empieza
desde cero.

**Escuchar ×4** toca el patrón cuatro veces y se detiene; **Espacio** lo
empieza o lo detiene mientras el editor tiene el teclado. La canción se
pausa mientras suena.

El editor tiene su propio **Deshacer** y **Rehacer** (**Ctrl+Z**,
**Ctrl+Mayús+Z**), aparte de los de la canción. Nada llega a la canción
hasta que aplicas el estilo; aplicarlo es entonces un paso que se
deshace, como cualquier otro estilo.

## Guardar y usar tus estilos

**Guardar estilo** escribe en el archivo del que vino el estilo;
**Guardar como…** pide uno nuevo. Los estilos se guardan como archivos
`.lineup-style`, en una **carpeta de estilos**: una carpeta `styles` en
la carpeta de datos de la aplicación, salvo que elijas otra en
**Edición > Ajustes… > Biblioteca > Tus estilos > Carpeta de
estilos…**. La Biblioteca lista los estilos de esa carpeta; uno guardado
en otro lugar no aparece, y la aplicación lo dice.

Tus estilos guardados se suman a la lista de la Biblioteca, junto a los
de la aplicación, con **+ acordes** si traen una progresión y **conserva
los acordes** si no. Con **Agrupar por género** tienen un grupo propio,
**Estilos de usuario**. Aplicar uno funciona como con cualquier estilo:
**Patrones de ejecución** trae sus canales, **Voces / instrumentos** sus
sonidos, etc. (ver [Los estilos y la Biblioteca](styles.md)).

Una canción guarda su propia copia del estilo que toca, dentro de su
archivo `.lineup`. Borra o mueve el archivo del estilo, o abre la
canción en otro equipo, y la banda lo sigue tocando.
