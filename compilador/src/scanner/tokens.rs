//! Scanner for Patito Language

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("programa")]
    Programa,

    #[token("inicio")]
    Inicio,

    #[token("fin")]
    Fin,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identificador(String),

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    ConstanteEntero(i64),
}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn run_lexer(input: &str) {
//         println!("Running lexer");
//         let mut lexer = Token::lexer(input);
//         while let Some(value) = lexer.next() {
//             println!("{:?}", value);
//         }
//         println!("");
//     }

//     #[test]
//     fn test_identificador() {
        

//         assert_eq!(s.length(), 0);
//         assert!(s.is_empty());
//     }
// }