mod scanner;
mod parser;

use scanner::Scanner;
use parser::Parser;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar);

fn main() {
    let input: &str = "programa inicio hola fin x1";
    
    let mut scanner: Scanner = Scanner::new(input);
    let primer_token = scanner.get_next();
    println!("primer token: {:?}", primer_token);

    let parser: Parser = Parser::new();
    let result = parser.parse(input);
    
    match result {
        Ok(_) => println!("Program is valid ✅"),
        Err(e) => println!("Syntax error ❌: {:?}", e),
    }
}
