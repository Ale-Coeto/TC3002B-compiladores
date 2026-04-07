mod lexer;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(parser);
lalrpop_mod!(parenthesis_parser);

use lexer::Token;
use logos::Logos;

fn main() {
    let input = "load x = y + z";
    let parenthesis_input = "((123))";
    
    run_lexer(&input);
    run_parenthesis_parser(&parenthesis_input);
    run_combined(&input);
}

fn run_lexer(input: &str) {
    println!("Running lexer");
    let mut lexer = Token::lexer(input);
    while let Some(value) = lexer.next() {
        println!("{:?}", value);
    }
    println!("");
}

fn run_parenthesis_parser(input: &str) {
    println!("Running parenthesis parser");
    let result = parenthesis_parser::TermParser::new().parse(input);

    match result {
        Ok(value) => println!("Parenthesis parse result: {}\n", value),
        Err(e) => println!("Parenthesis parse error: {:?}\n", e),
    }
}

fn run_combined(input: &str) {
    println!("Running lexer + parser");
    let tokens: Vec<Token> = Token::lexer(input)
        .filter_map(Result::ok)
        .collect();

    let result = parser::StatementParser::new()
        .parse(tokens.into_iter());

    match result {
        Ok(_) => println!("Parse successful"),
        Err(e) => println!("Parse error: {:?}", e),
    }
}