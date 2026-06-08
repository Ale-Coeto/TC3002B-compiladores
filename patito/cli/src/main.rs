use clap::{Parser, Subcommand};
use compilador::compile;
use std::process::Command;

#[derive(Parser)]
#[command(name = "patito")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build {
        file: String,
    },

    Run {
        file: String,
    },
}

fn exec_vm(json_file: &str) {
    let abs_json = std::fs::canonicalize(json_file)
        .expect("could not resolve json path");

    let vm_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../vm");

    let status = Command::new("python3")
        .current_dir(vm_dir)
        .arg("main.py")
        .arg(abs_json)
        .status()
        .expect("failed to run VM");

    if !status.success() {
        eprintln!("VM execution failed");
        std::process::exit(1);
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build { file } => {
            match compile(&file) {
                Ok(output) => {
                    println!("output written to {}", output);
                }
                Err(e) => {
                    eprintln!("compile error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Run { file } => {
            match compile(&file) {
                Ok(output) => {
                    exec_vm(&output);
                }
                Err(e) => {
                    eprintln!("compile error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}