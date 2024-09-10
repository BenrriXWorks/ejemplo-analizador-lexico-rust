%%
%class Lexer
%result_type i32

"anfang"                println!("(ANFANG, -)");
"farbe"                 println!("(FARBE, -)");
"wert"                  println!("(WERT, -)");
"ende"                  println!("(ENDE, -)");
[A-Z][0-9]              println!("(ID, id:{})", self.yytext());
[0-9]+                  println!("(CONST, num:{})", self.yytext()); 
"="                     println!("(=, -)");
"("                     println!("((, -)");
")"                     println!("(), -)");
","                     println!("(',', -)");
" "+                    {};
<<EOF>>                 println!("normal EOF"); 
.                       println!("Token desconocido: {}", self.yytext());
%% 