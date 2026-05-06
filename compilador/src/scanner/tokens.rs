//! Scanner for Patito Language

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    // General
    #[token("programa")]
    Programa,

    #[token("inicio")]
    Inicio,

    #[token("fin")]
    Fin,

    #[token("entero")]
    Entero,

    #[token("flotante")]
    Flotante,

    #[token("vars")]
    Variable,

    #[token("escribe")]
    Escribe,

    #[token("nula")]
    Nula,

    // Condicionales y ciclos
    #[token("mientras")]
    Mientras,

    #[token("haz")]
    Haz,

    #[token("si")]
    Si,

    #[token("sino")]
    Sino,

    // Símbolos
    #[token(";")]
    PuntoYComa,

    #[token(":")]
    DosPuntos,

    #[token(",")]
    Coma,

    #[token("{")]
    LlaveAbrir,

    #[token("}")]
    LlaveCerrar,

    #[token("[")]
    CorcheteAbrir,

    #[token("]")]
    CorcheteCerrar,

    #[token("(")]
    ParentesisAbrir,

    #[token(")")]
    ParentesisCerrar,

    // Operadores
    #[token("==")]
    ComparadorIgual,

    #[token("!=")]
    NoIgual,

    #[token(">")]
    Mayor,

    #[token("<")]
    Menor,

    #[token("=")]
    Igual,

    #[token("+")]
    Mas,

    #[token("-")]
    Menos,

    #[token("*")]
    Multiplicar,

    #[token("/")]
    Dividir,
    
    // Literales
    #[regex(r"-?[0-9]+\.[0-9]+([eE][+-]?[0-9]+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    ConstanteFlotante(f64),

    #[regex(r"-?[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    ConstanteEntero(i64),

    // Identificador y letrero
    #[regex(r#""([^"\\]|\\.)*""#, |lex| lex.slice().to_string())]    
    Letrero(String),
    
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identificador(String),

}
