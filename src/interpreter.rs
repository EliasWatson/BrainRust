use std::fmt::Display;

use crate::{errors::InterpreterResult, memory::Memory, program::Program};

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

    pub fn run(&mut self) -> InterpreterResult<()> {
        while self.step()? {}

        Ok(())
    }

    pub fn run_logged(&mut self) -> InterpreterResult<()> {
        while self.step()? {
            println!("{}", self);
        }

        Ok(())
    }

    fn step(&mut self) -> InterpreterResult<bool> {
        let op = match self.program.get() {
            Some(op) => op,
            None => return Ok(false),
        };

        match op {
            '>' => self.memory.next(),
            '<' => self.memory.previous(),
            '+' => self.memory.increment(),
            '-' => self.memory.decrement(),
            '.' => print!("{}", self.memory.get_char()),
            ',' => todo!(),
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
