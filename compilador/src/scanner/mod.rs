pub mod tokens;

pub use tokens::Token;
use logos::Logos;

pub struct Scanner<'a> {
    lexer: logos::Lexer<'a, Token>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Token::lexer(input),
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = (usize, Token, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.lexer.next()?;
        let span = self.lexer.span();

        match tok {
            Ok(t) => Some((span.start, t, span.end)),
            Err(_) => {
                panic!("Lexing error at {:?}", span);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod identificador {
        use super::*;

        #[test]
        fn test_identificador() {
            let mut scanner: Scanner = Scanner::new("hola");
            let (_, token, _) = scanner.next().expect("Expected a token");
    
            assert_eq!(token, Token::Identificador("hola".to_string()));
        }
    }

    #[test]
    fn test_constante_entero() {
        let mut scanner: Scanner = Scanner::new("12");
        let (_, token, _) = scanner.next().expect("Expected a token");

        assert_eq!(token, Token::ConstanteEntero(12));
    }

    #[test]
    fn test_constante_flotante() {
        let mut scanner: Scanner = Scanner::new("12.1");
        let (_, token, _) = scanner.next().expect("Expected a token");

        assert_eq!(token, Token::ConstanteFlotante(12.1));
    }

}