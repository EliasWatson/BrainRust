use console::Style;
use std::{cmp::min, fmt::Display};

use crate::{
    commands::{parse_command_string, Command},
    errors::ParserError,
};

#[derive(Debug)]
pub struct Program {
    pub commands: Vec<Command>,
    index: usize,
}

impl Program {
    pub fn parse(program_source: String) -> Result<Self, ParserError> {
        Ok(Self {
            commands: parse_command_string(program_source)?,
            index: 0,
        })
    }

    pub fn next(&mut self) {
        self.index += 1;
    }

    pub fn jump(&mut self, index: usize) {
        self.index = index;
    }

    pub fn get(&self) -> Option<Command> {
        if self.index >= self.commands.len() {
            None
        } else {
            Some(self.commands[self.index])
        }
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

        let context_style = Style::new().white().on_black();
        let current_style = Style::new().red().bold().on_black();

        format!(
            "{}{}{}",
            context_style.apply_to(before),
            current_style.apply_to(current),
            context_style.apply_to(after)
        )
    }

    fn get_range_clamped(&self, start: isize, end: isize) -> String {
        if self.commands.is_empty() || end < 0 {
            return String::new();
        }

        let start: usize = start.try_into().unwrap_or(0);
        let end: usize = end.try_into().unwrap_or(0);

        let last_index = self.commands.len() - 1;

        if start > last_index {
            return String::new();
        }

        let start = min(start, last_index);
        let end = min(end, last_index);

        self.commands[start..=end]
            .iter()
            .map(|command| format!("{}", command))
            .collect::<Vec<String>>()
            .join("\t")
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:8}\t{}",
            format!("{}:", self.index),
            self.get_window(2)
        )
    }
}
