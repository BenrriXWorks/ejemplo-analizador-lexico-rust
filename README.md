Acepta palabras o frases desde la consola segun el lenguaje definido en lexer.l
Ejecutar: $ cargo run

Lenguaje aceptado en lexer.l

Como cambiar el lenguaje:
	- Modificar lexer.l (escrito en rflex)
 	- ejecutar $ rflex lexer.l
 	- Se va a generar un nuevo archivo lexer.rs, ese se debe mover a src para reemplazar al archivo con el mismo nombre
 	- Ejecutar el programa, usando
	+ Para modo desarrollo $ cargo run
 	+ Para compilar a ejecutable $ cargo build
 	- Luego ejecutar en la carpeta target el ejecutabl
