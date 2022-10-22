use std::{error::Error, io, io::Write};

use console::Term;

use crate::{
    command_ast::parse_source, commands::Command, errors::ParserError, memory::Memory,
    optimizer::optimize, program::Program,
};

#[derive(Debug)]
pub struct Interpreter {
    memory: Memory,
    program: Program,
    always_flush: bool,
}

impl Interpreter {
    pub fn load_program(
        program_source: String,
        memory_size: usize,
        always_flush: bool,
        should_optimize: bool,
    ) -> Result<Self, ParserError> {
        let ast = parse_source(program_source)?;
        let program = if should_optimize {
            Program::from_ast(ast)
        } else {
            let optimized_ast = optimize(ast);
            Program::from_optimized_ast(optimized_ast)
        };

        Ok(Self {
            memory: Memory::new(memory_size),
            program,
            always_flush,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let term = Term::stdout();

        while self.step(&term)? {}

        Ok(())
    }

    fn step(&mut self, term: &Term) -> Result<bool, Box<dyn Error>> {
        let command = match self.program.get() {
            Some(op) => op,
            None => return Ok(false),
        };

        match command {
            Command::Move(offset) => self.memory.move_index(offset),
            Command::Add(n, offset) => self.memory.add_with_offset(n, offset),
            Command::Zero => self.memory.zero(),
            Command::Output => {
                print!("{}", self.memory.get_char());
                if self.always_flush {
                    io::stdout().flush()?;
                }
            }
            Command::Input => term.read_char().map(|c| self.memory.set_char(c))?,
            Command::LoopBegin(i) => {
                if self.memory.is_zero() {
                    self.program.jump(i);
                }
            }
            Command::LoopEnd(i) => {
                if !self.memory.is_zero() {
                    self.program.jump(i);
                }
            }
        }

        self.program.next();

        Ok(true)
    }
}
