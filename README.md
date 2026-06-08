# TC3002B - Compiladores

Actividades de clase

Alejandra Coeto - A01285221

## Tareas

 [Tarea 1 - Estructuras de datos ](./tarea1/src/main.rs): Implementación de Stack, Queue y HashMap

```bash
cd tarea1
cargo run # Ejecuta el programa principal
cargo test # Ejecuta las pruebas unitarias
```

<hr />

[Tarea 2 - Generación de analizadores de léxico y sintaxis ](./tarea2/src/main.rs): Uso de `logos` y `lalrpop` para generar analizadores de léxico y sintaxis.

```bash
cd tarea2
cargo build # Compila el proyecto
cargo run # Ejecuta el programa principal
```

<hr />

[Compilador para el lenguaje Patito ](./patito): Implementación de un compilador para el lenguaje Patito, incluyendo análisis léxico, sintáctico y generación de código.

```bash
cd patito
cargo build # Compila el proyecto
cargo run -- ../recursivo.txt     # Ejecuta el compilador con el archivo de entrada recursivo.txt
```

