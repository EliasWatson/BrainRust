use std::{error::Error, fmt::Display};

#[derive(Clone, Copy, Debug)]
pub enum InterpreterError {
    LoopTraversalError(usize),
    InputError,
}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpreterError::LoopTraversalError(program_counter) => write!(
                f,
                "matching loop bracket missing for character {}",
                program_counter
            ),
            InterpreterError::InputError => write!(f, "failed to read input"),
        }
    }
}

impl Error for InterpreterError {}
