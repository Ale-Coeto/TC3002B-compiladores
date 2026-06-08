use compilador::compile;

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
