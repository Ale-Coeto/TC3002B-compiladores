mod scanner;
mod parser;
mod semantics;
mod quads;
mod memory;

use parser::{Parser, ParseOutput};
use memory::ConstantValue;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar, "/parser/grammar.rs");

const OUTPUT_PATH: &str = "../output.json";

fn compile(input_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(input_path)?;
    let output = Parser::new()
        .parse(&input)
        .map_err(|e| format!("parse error: {e:?}"))?;
    // println!("{:?}", output.quads);

    std::fs::write(OUTPUT_PATH, build_json(&output))?;
    Ok(OUTPUT_PATH.to_string())
}

fn escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn build_json(output: &ParseOutput) -> String {
    let mut json = String::from("{\n");

    // quads
    json.push_str("  \"quads\": [\n");
    for (i, q) in output.quads.iter().enumerate() {
        let comma = if i + 1 < output.quads.len() { "," } else { "" };
        let parts: Vec<&str> = q.splitn(4, ' ').collect();
        let op = format!("\"{}\"", escape(parts[0]));
        let operands: Vec<String> = parts[1..].iter().map(|p| {
            if let Ok(n) = p.parse::<i64>() { n.to_string() }
            else { format!("\"{}\"", escape(p)) }
        }).collect();
        json.push_str(&format!("    [{}, {}]{}\n", op, operands.join(", "), comma));
    }
    json.push_str("  ],\n");

    // functions
    json.push_str("  \"functions\": [\n");
    for (i, f) in output.dir_func.iter().enumerate() {
        let comma = if i + 1 < output.dir_func.len() { "," } else { "" };
        json.push_str(&format!(
            "    {{\"name\": \"{}\", \"address\": \"{}\", \"start_index\": {}, \"local_count\": {}, \"temp_count\": {}}}{}\n",
            escape(&f.name), f.address, f.start_index, f.local_count, f.temp_count, comma
        ));
    }
    json.push_str("  ],\n");

    // constants
    json.push_str("  \"constants\": [\n");
    for (i, c) in output.constants.iter().enumerate() {
        let comma = if i + 1 < output.constants.len() { "," } else { "" };
        let (type_str, val_str) = match &c.value {
            ConstantValue::Entero(v) => ("int", v.to_string()),
            ConstantValue::Flotante(v) => ("float", v.to_string()),
            ConstantValue::Letrero(v) => ("string", format!("\"{}\"", escape(v))),
        };
        json.push_str(&format!(
            "    {{\"address\": {}, \"type\": \"{}\", \"value\": {}}}{}\n",
            c.address, type_str, val_str, comma
        ));
    }
    json.push_str("  ]\n");

    json.push('}');
    json
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: compilador <input_file>");
        std::process::exit(1);
    }
    match compile(&args[1]) {
        Ok(path) => println!("output written to {path}"),
        Err(e) => { eprintln!("error: {e}"); std::process::exit(1); }
    }
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
        parser.parse(input).expect("parser should succeed").quads
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
