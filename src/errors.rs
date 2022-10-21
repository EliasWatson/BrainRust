use std::{error::Error, fmt::Display};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ParserError {
    IncompleteLoop(usize),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::IncompleteLoop(program_counter) => write!(
                f,
                "matching loop bracket missing for character at index {}",
                program_counter
            ),
        }
    }
}

impl Error for ParserError {}
