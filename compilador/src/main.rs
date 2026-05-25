mod scanner;
mod parser;
mod semantics;
mod quads;
mod memory;

use parser::Parser;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar, "/parser/grammar.rs");

fn main() {
    let parser: Parser = Parser::new();
    let input = include_str!("sample_program.txt");
    let result = parser.parse(input);
    
    match result {
        Ok(_) => println!("Program is valid"),
        Err(e) => println!("Syntax error: {:?}", e),
    }
}
