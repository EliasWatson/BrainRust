mod command_ast;
mod commands;
mod errors;
mod interpreter;
mod memory;
mod optimizer;
mod program;

use clap::Parser;
use command_ast::parse_source;
use interpreter::Interpreter;
use optimizer::optimize;
use program::Program;
use std::{error::Error, fs};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    path: Option<String>,

    #[arg(short = 'f', long)]
    always_flush: bool,

    #[arg(short, long)]
    disable_optimization: bool,

    #[arg(short, long)]
    print: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.path {
        Some(path) => {
            let program_source = fs::read_to_string(path)?;

            let ast = parse_source(program_source)?;

            let program = if args.disable_optimization {
                if args.print {
                    for node in &ast {
                        print!("{}", node);
                    }
                    println!();
                }

                Program::from_ast(ast)
            } else {
                let optimized_ast = optimize(ast);

                if args.print {
                    for node in &optimized_ast {
                        print!("{}", node);
                    }
                }

                Program::from_optimized_ast(optimized_ast)
            };

            let mut interpreter = Interpreter::load_program(program, 30_000, args.always_flush);

            interpreter.run()?;
        }
        None => {
            todo!("REPL")
        }
    }

    Ok(())
}
