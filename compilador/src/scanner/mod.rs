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

    pub fn get_next(&mut self) -> Option<Result<Token, ()>> {
        return self.lexer.next();
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