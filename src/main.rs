mod command_ast;
mod commands;
mod errors;
mod interpreter;
mod memory;
mod optimizer;
mod program;

use clap::Parser;
use interpreter::Interpreter;
use std::{error::Error, fs};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    path: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.path {
        Some(path) => {
            let program_source = fs::read_to_string(path)?;
            let mut interpreter = Interpreter::load_program(program_source, 30_000)?;

            interpreter.run()?;
        }
        None => {
            todo!("REPL")
        }
    }

    Ok(())
}
