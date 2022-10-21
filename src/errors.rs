use std::{error::Error, fmt::Display};

pub type InterpreterResult<T> = Result<T, InterpreterError>;

#[derive(Clone, Copy, Debug)]
pub enum InterpreterError {
    LoopTraversalError(usize),
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
