// src/main.rs
mod error;
mod interpreter;

use interpreter::Interpreter;
use std::env;
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> error::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }

    let mut interpreter = Interpreter::new();
    interpreter.load_file(&args[1])?;
    interpreter.run()?;

    Ok(())
}
