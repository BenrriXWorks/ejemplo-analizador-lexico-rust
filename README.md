Acepta palabras o frases desde la consola segun el lenguaje definido en lexer.l</br>
Ejecutar: $ cargo run</br>

Lenguaje aceptado en lexer.l</br>

Como cambiar el lenguaje:</br>
	- Modificar lexer.l (escrito en rflex)</br>
 	- ejecutar $ rflex lexer.l</br>
 	- Se va a generar un nuevo archivo lexer.rs, reemplazar al archivo de src</br>
 	- Ejecutar el programa, usando:</br>
	+ Para modo desarrollo $ cargo run</br>
 	+ Para compilar a ejecutable $ cargo build</br>
 	- Luego ejecutar en la carpeta target el ejecutable</br>
