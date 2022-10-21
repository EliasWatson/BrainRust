use colored::*;
use std::{cmp::min, fmt::Display};

use crate::errors::{InterpreterError, InterpreterResult};

#[derive(Debug)]
pub struct Program {
    chars: Vec<char>,
    index: usize,
}

impl Program {
    pub fn new(program_source: String) -> Self {
        Self {
            chars: program_source.chars().collect(),
            index: 0,
        }
    }

    pub fn next(&mut self) {
        self.index += 1;
    }

    pub fn get(&self) -> Option<char> {
        if self.index >= self.chars.len() {
            None
        } else {
            Some(self.chars[self.index])
        }
    }

    pub fn skip_loop(&mut self) -> InterpreterResult<()> {
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

    pub fn repeat_loop(&mut self) -> InterpreterResult<()> {
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

    pub fn get_window(&self, radius: usize) -> String {
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
