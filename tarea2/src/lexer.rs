//! Lexer that identifies Load, Equals, Operator and Text tokens

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("load")]
    Load,

    #[token("=")]
    Equals,

    #[token("+")]
    Plus,

    #[regex("[a-zA-Z]+", |lex| lex.slice().to_string())]
    Text(String),
}