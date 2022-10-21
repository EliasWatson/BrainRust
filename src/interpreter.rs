use std::{error::Error, fmt::Display, io, io::Write};

use console::Term;

use crate::{errors::InterpreterError, memory::Memory, program::Program};

#[derive(Debug)]
pub struct Interpreter {
    memory: Memory,
    program: Program,
}

impl Interpreter {
    pub fn load_program(program_source: String, memory_size: usize) -> Self {
        Self {
            memory: Memory::new(memory_size),
            program: Program::new(program_source),
        }
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
        let op = match self.program.get() {
            Some(op) => op,
            None => return Ok(false),
        };

        match op {
            '>' => self.memory.next(),
            '<' => self.memory.previous(),
            '+' => self.memory.increment(),
            '-' => self.memory.decrement(),
            '.' => {
                print!("{}", self.memory.get_char());
                io::stdout().flush()?;
            }
            ',' => term
                .read_char()
                .map(|c| self.memory.set_char(c))
                .map_err(|_| InterpreterError::InputError)?,
            '[' => {
                if self.memory.is_zero() {
                    self.program.skip_loop()?;
                }
            }
            ']' => {
                if !self.memory.is_zero() {
                    self.program.repeat_loop()?;
                }
            }
            _ => {}
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
