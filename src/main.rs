mod lexer;

use text_io::read;

fn main() {
    println!("Ingrese su entrada de tokens (escriba 'exit' para finalizar):");

    loop {
        // Leer la entrada del usuario
        let line: String = read!("{}\n");

        // Terminar si el usuario escribe 'exit'
        if line.trim() == "exit" {
            break;
        }

        // Procesar la entrada usando el lexer
        let mut lex = lexer::Lexer::new(&line);
        loop {
            let res = lex.yylex();
            if res.is_err() {
                break;
            }
        }
    }
}