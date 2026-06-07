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
    let input = include_str!("tests/input/basic.txt");
    let result = parser.parse(input);
    
    // match result {
    //     Ok(_) => println!("Program is valid"),
    //     Err(e) => println!("Syntax error: {:?}", e),
    // }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::memory::MemoryManager;
    use std::sync::{Mutex, MutexGuard, OnceLock};

    static TEST_GUARD: OnceLock<Mutex<()>> = OnceLock::new();

    fn test_guard() -> MutexGuard<'static, ()> {
        TEST_GUARD
            .get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn parse_input(input: &str) -> Vec<String> {
        let _guard = test_guard();
        MemoryManager::with_instance(|memory_manager| memory_manager.reset());
        let parser = Parser::new();
        parser.parse(input).expect("parser should succeed")
    }

    fn assert_output_matches(input: &str, expected_output: &str) {
        let expected = expected_output
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>();
        let actual = parse_input(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn basic_case_validates_memory_addresses() {
        assert_output_matches(
            include_str!("tests/input/basic.txt"),
            include_str!("tests/output/basic.txt"),
        );
    }

    #[test]
    fn if_else_generates_expected_quads() {
        assert_output_matches(
            include_str!("tests/input/if_else.txt"),
            include_str!("tests/output/if_else.txt"),
        );
    }

    #[test]
    fn while_generates_expected_quads() {
        assert_output_matches(
            include_str!("tests/input/mientras.txt"),
            include_str!("tests/output/mientras.txt"),
        );
    }

    #[test]
    fn functions_generate_expected_quads() {
        assert_output_matches(
            include_str!("tests/input/functions.txt"),
            include_str!("tests/output/functions.txt"),
        );
    }
}
