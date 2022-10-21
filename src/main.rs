use clap::Parser;
use colored::*;
use std::{cmp::min, error::Error, fmt::Display, fs};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    path: Option<String>,
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.path {
        Some(path) => {
            let program_source = fs::read_to_string(path)?;
            let mut interpreter = Interpreter::load_program(program_source, 30_000);

            if args.debug {
                interpreter.run_logged()?;
            } else {
                interpreter.run()?;
            }
        }
        None => {
            todo!("REPL")
        }
    }

    Ok(())
}

type InterpreterResult<T> = Result<T, InterpreterError>;

#[derive(Clone, Copy, Debug)]
enum InterpreterError {
    LoopTraversalError(usize),
}

#[derive(Debug)]
struct Memory {
    data: Vec<u8>,
    index: usize,
}

#[derive(Debug)]
struct Program {
    chars: Vec<char>,
    index: usize,
}

#[derive(Debug)]
struct Interpreter {
    memory: Memory,
    program: Program,
}

impl Interpreter {
    fn load_program(program_source: String, memory_size: usize) -> Self {
        Self {
            memory: Memory::new(memory_size),
            program: Program::new(program_source),
        }
    }

    fn run(&mut self) -> InterpreterResult<()> {
        while self.step()? {}

        Ok(())
    }

    fn run_logged(&mut self) -> InterpreterResult<()> {
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

impl Memory {
    fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
            index: 0,
        }
    }

    fn next(&mut self) {
        self.index = (self.index + 1) % self.data.len();
    }

    fn previous(&mut self) {
        self.index = if self.index == 0 {
            self.data.len() - 1
        } else {
            self.index - 1
        };
    }

    fn increment(&mut self) {
        self.data[self.index] = self.data[self.index].wrapping_add(1);
    }

    fn decrement(&mut self) {
        self.data[self.index] = self.data[self.index].wrapping_sub(1);
    }

    fn is_zero(&self) -> bool {
        self.data[self.index] == 0
    }

    fn get_char(&self) -> char {
        self.data[self.index] as char
    }
}

impl Program {
    fn new(program_source: String) -> Self {
        Self {
            chars: program_source.chars().collect(),
            index: 0,
        }
    }

    fn next(&mut self) {
        self.index += 1;
    }

    fn get(&self) -> Option<char> {
        if self.index >= self.chars.len() {
            None
        } else {
            Some(self.chars[self.index])
        }
    }

    fn skip_loop(&mut self) -> InterpreterResult<()> {
        let mut loop_depth: isize = 0;
        for (offset, c) in self.chars[self.index..].iter().enumerate() {
            match *c {
                '[' => loop_depth += 1,
                ']' => loop_depth -= 1,
                _ => {}
            }

            if loop_depth == 0 {
                self.index += offset;
                return Ok(());
            }
        }

        Err(InterpreterError::LoopTraversalError(self.index))
    }

    fn repeat_loop(&mut self) -> InterpreterResult<()> {
        let mut loop_depth: isize = 0;
        for (offset, c) in self.chars[..=self.index].iter().rev().enumerate() {
            match *c {
                '[' => loop_depth += 1,
                ']' => loop_depth -= 1,
                _ => {}
            }

            if loop_depth == 0 {
                self.index -= offset;
                return Ok(());
            }
        }

        Err(InterpreterError::LoopTraversalError(self.index))
    }

    fn get_window(&self, radius: usize) -> String {
        let iradius = radius as isize;
        let index = self.index as isize;

        let before = format!(
            "{:>width$}",
            self.get_range_clamped(index - iradius, index - 1),
            width = radius
        );
        let current = format!("{:1}", self.get_range_clamped(index, index));
        let after = format!(
            "{:width$}",
            self.get_range_clamped(index + 1, index + iradius),
            width = radius
        );

        format!(
            "{}{}{}",
            before.white(),
            current.red().bold(),
            after.white()
        )
        .on_black()
        .to_string()
    }

    fn get_range_clamped(&self, start: isize, end: isize) -> String {
        if self.chars.is_empty() || end < 0 {
            return String::new();
        }

        let start: usize = start.try_into().unwrap_or(0);
        let end: usize = end.try_into().unwrap_or(0);

        let last_index = self.chars.len().saturating_sub(1);

        if start > last_index {
            return String::new();
        }

        let start = min(start, last_index);
        let end = min(end, last_index);

        self.chars[start..=end].iter().collect()
    }
}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpreterError::LoopTraversalError(program_counter) => write!(
                f,
                "matching loop bracket missing for character {}",
                program_counter
            ),
        }
    }
}

impl Error for InterpreterError {}

impl Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO")
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}) {:<6}",
            self.get_window(2),
            format!("[{}]", self.index).italic()
        )
    }
}

impl Display for Interpreter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}  |  {}", self.program, self.memory)
    }
}
