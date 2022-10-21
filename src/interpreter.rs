use std::{error::Error, fmt::Display, io, io::Write};

use console::Term;

use crate::{commands::Command, errors::ParserError, memory::Memory, program::Program};

#[derive(Debug)]
pub struct Interpreter {
    memory: Memory,
    program: Program,
}

impl Interpreter {
    pub fn load_program(program_source: String, memory_size: usize) -> Result<Self, ParserError> {
        Ok(Self {
            memory: Memory::new(memory_size),
            program: Program::parse(program_source)?,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let term = Term::stdout();

        while self.step(&term)? {}

        Ok(())
    }

    pub fn run_logged(&mut self) -> Result<(), Box<dyn Error>> {
        let term = Term::stdout();
        let term_err = Term::stderr();

        while self.step(&term)? {
            term_err.write_line(&format!("{}", self))?;
        }

        Ok(())
    }

    fn step(&mut self, term: &Term) -> Result<bool, Box<dyn Error>> {
        let command = match self.program.get() {
            Some(op) => op,
            None => return Ok(false),
        };

        match command {
            Command::Next(n) => self.memory.next(n),
            Command::Previous(n) => self.memory.previous(n),
            Command::Increment(n) => self.memory.increment(n),
            Command::Decrement(n) => self.memory.decrement(n),
            Command::Output => {
                print!("{}", self.memory.get_char());
                io::stdout().flush()?;
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

impl Display for Interpreter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}  |  {}", self.program, self.memory)
    }
}
