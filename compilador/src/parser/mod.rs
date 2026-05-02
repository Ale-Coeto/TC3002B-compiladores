use crate::scanner::Scanner;
use crate::grammar::ProgramaParser;
use crate::scanner::Token;
use lalrpop_util::ParseError;

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser
    }

    pub fn parse(&self, input: &str) -> Result<(), ParseError<usize, Token, ()>> {
        let scanner: Scanner = Scanner::new(input);
        let result = ProgramaParser::new().parse(scanner);

        result
    }
}