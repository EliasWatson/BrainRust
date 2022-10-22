use std::{error::Error, fmt::Display};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ParserError {
    IncompleteLoop,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::IncompleteLoop => write!(f, "loop bracket missing"),
        }
    }
}

impl Error for ParserError {}
