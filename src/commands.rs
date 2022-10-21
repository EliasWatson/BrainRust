use std::fmt::Display;

use crate::errors::ParserError;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Command {
    Next(usize),
    Previous(usize),
    Increment(u8),
    Decrement(u8),
    Output,
    Input,
    LoopBegin(usize),
    LoopEnd(usize),
}

pub fn parse_command_string(source: String) -> Result<Vec<Command>, ParserError> {
    let mut commands: Vec<Command> = vec![];

    let mut last_command_option: Option<Command> = None;
    let mut loop_stack: Vec<usize> = vec![];

    for c in source.chars() {
        let command = match Command::from_char(c) {
            Some(command) => command,
            None => continue,
        };

        match (&mut last_command_option, command) {
            (Some(Command::Next(ref mut n)), Command::Next(_))
            | (Some(Command::Previous(ref mut n)), Command::Previous(_)) => {
                *n += 1;
                continue;
            }
            (Some(Command::Increment(ref mut n)), Command::Increment(_))
            | (Some(Command::Decrement(ref mut n)), Command::Decrement(_)) => {
                *n = n.wrapping_add(1);
                continue;
            }
            _ => {}
        }

        if let Some(last_command) = last_command_option {
            commands.push(last_command);
        }

        if command.is_multi() {
            last_command_option = Some(command);
        } else {
            last_command_option = None;

            match command {
                Command::Output => commands.push(Command::Output),
                Command::Input => commands.push(Command::Input),
                Command::LoopBegin(_) => {
                    loop_stack.push(commands.len());
                    commands.push(Command::LoopBegin(0));
                }
                Command::LoopEnd(_) => {
                    if let Some(begin_index) = loop_stack.pop() {
                        let end_index = commands.len();

                        if let Command::LoopBegin(ref mut end_index_ref) = commands[begin_index] {
                            *end_index_ref = end_index;
                        } else {
                            unreachable!("Corrupt loop stack in parser")
                        }

                        commands.push(Command::LoopEnd(begin_index));
                    } else {
                        return Err(ParserError::IncompleteLoop(commands.len()));
                    }
                }
                _ => {}
            }
        }
    }

    if let Some(last_command) = last_command_option {
        commands.push(last_command);
    }

    if let Some(i) = loop_stack.pop() {
        return Err(ParserError::IncompleteLoop(i));
    }

    Ok(commands)
}

impl Command {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '>' => Some(Command::Next(1)),
            '<' => Some(Command::Previous(1)),
            '+' => Some(Command::Increment(1)),
            '-' => Some(Command::Decrement(1)),
            '.' => Some(Command::Output),
            ',' => Some(Command::Input),
            '[' => Some(Command::LoopBegin(0)),
            ']' => Some(Command::LoopEnd(0)),
            _ => None,
        }
    }

    fn is_multi(&self) -> bool {
        matches!(
            self,
            Command::Next(_) | Command::Previous(_) | Command::Increment(_) | Command::Decrement(_)
        )
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Next(n) => {
                if *n == 1 {
                    write!(f, ">")
                } else {
                    write!(f, ">{}", n)
                }
            }
            Command::Previous(n) => {
                if *n == 1 {
                    write!(f, "<")
                } else {
                    write!(f, "<{}", n)
                }
            }
            Command::Increment(n) => {
                if *n == 1 {
                    write!(f, "+")
                } else {
                    write!(f, "+{}", n)
                }
            }
            Command::Decrement(n) => {
                if *n == 1 {
                    write!(f, "-")
                } else {
                    write!(f, "-{}", n)
                }
            }
            Command::Output => write!(f, "out"),
            Command::Input => write!(f, "in"),
            Command::LoopBegin(_) => write!(f, "["),
            Command::LoopEnd(_) => write!(f, "]"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        commands::{parse_command_string, Command},
        errors::ParserError,
    };

    #[test]
    fn test_parse_command_string_empty() {
        assert_eq!(parse_command_string(String::from("")), Ok(vec![]));
        assert_eq!(parse_command_string(String::from(" ")), Ok(vec![]));
        assert_eq!(parse_command_string(String::from("test")), Ok(vec![]));
        assert_eq!(parse_command_string(String::from("1234567890")), Ok(vec![]));
    }

    #[test]
    fn test_parse_command_string_basic() {
        assert_eq!(
            parse_command_string(String::from(">")),
            Ok(vec![Command::Next(1)])
        );
        assert_eq!(
            parse_command_string(String::from(">>>")),
            Ok(vec![Command::Next(3)])
        );
        assert_eq!(
            parse_command_string(String::from(">> >")),
            Ok(vec![Command::Next(3)])
        );

        assert_eq!(
            parse_command_string(String::from("<")),
            Ok(vec![Command::Previous(1)])
        );
        assert_eq!(
            parse_command_string(String::from("<<<")),
            Ok(vec![Command::Previous(3)])
        );
        assert_eq!(
            parse_command_string(String::from("<< <")),
            Ok(vec![Command::Previous(3)])
        );

        assert_eq!(
            parse_command_string(String::from("+")),
            Ok(vec![Command::Increment(1)])
        );
        assert_eq!(
            parse_command_string(String::from("+++")),
            Ok(vec![Command::Increment(3)])
        );
        assert_eq!(
            parse_command_string(String::from("++ +")),
            Ok(vec![Command::Increment(3)])
        );

        assert_eq!(
            parse_command_string(String::from("-")),
            Ok(vec![Command::Decrement(1)])
        );
        assert_eq!(
            parse_command_string(String::from("---")),
            Ok(vec![Command::Decrement(3)])
        );
        assert_eq!(
            parse_command_string(String::from("-- -")),
            Ok(vec![Command::Decrement(3)])
        );
    }

    #[test]
    fn test_parse_command_string_basic_io() {
        assert_eq!(
            parse_command_string(String::from(".")),
            Ok(vec![Command::Output])
        );
        assert_eq!(
            parse_command_string(String::from("...")),
            Ok(vec![Command::Output, Command::Output, Command::Output])
        );
        assert_eq!(
            parse_command_string(String::from(".. .")),
            Ok(vec![Command::Output, Command::Output, Command::Output])
        );

        assert_eq!(
            parse_command_string(String::from(",")),
            Ok(vec![Command::Input])
        );
        assert_eq!(
            parse_command_string(String::from(",,,")),
            Ok(vec![Command::Input, Command::Input, Command::Input])
        );
        assert_eq!(
            parse_command_string(String::from(",, ,")),
            Ok(vec![Command::Input, Command::Input, Command::Input])
        );
    }

    #[test]
    fn test_parse_command_string_basic_loops() {
        assert_eq!(
            parse_command_string(String::from("[")),
            Err(ParserError::IncompleteLoop(0))
        );
        assert_eq!(
            parse_command_string(String::from("]")),
            Err(ParserError::IncompleteLoop(0))
        );

        assert_eq!(
            parse_command_string(String::from("[]")),
            Ok(vec![Command::LoopBegin(1), Command::LoopEnd(0)])
        );
        assert_eq!(
            parse_command_string(String::from("[ ]")),
            Ok(vec![Command::LoopBegin(1), Command::LoopEnd(0)])
        );

        assert_eq!(
            parse_command_string(String::from("[][]")),
            Ok(vec![
                Command::LoopBegin(1),
                Command::LoopEnd(0),
                Command::LoopBegin(3),
                Command::LoopEnd(2)
            ])
        );
        assert_eq!(
            parse_command_string(String::from("[ ] [ ]")),
            Ok(vec![
                Command::LoopBegin(1),
                Command::LoopEnd(0),
                Command::LoopBegin(3),
                Command::LoopEnd(2)
            ])
        );

        assert_eq!(
            parse_command_string(String::from("[[]]")),
            Ok(vec![
                Command::LoopBegin(3),
                Command::LoopBegin(2),
                Command::LoopEnd(1),
                Command::LoopEnd(0)
            ])
        );
        assert_eq!(
            parse_command_string(String::from("[ [ ] ]")),
            Ok(vec![
                Command::LoopBegin(3),
                Command::LoopBegin(2),
                Command::LoopEnd(1),
                Command::LoopEnd(0)
            ])
        );

        assert_eq!(
            parse_command_string(String::from("[[]][[]]")),
            Ok(vec![
                Command::LoopBegin(3),
                Command::LoopBegin(2),
                Command::LoopEnd(1),
                Command::LoopEnd(0),
                Command::LoopBegin(7),
                Command::LoopBegin(6),
                Command::LoopEnd(5),
                Command::LoopEnd(4)
            ])
        );
        assert_eq!(
            parse_command_string(String::from(" [[] ][ [ ]]")),
            Ok(vec![
                Command::LoopBegin(3),
                Command::LoopBegin(2),
                Command::LoopEnd(1),
                Command::LoopEnd(0),
                Command::LoopBegin(7),
                Command::LoopBegin(6),
                Command::LoopEnd(5),
                Command::LoopEnd(4)
            ])
        );
    }

    #[test]
    fn test_parse_command_string_simple_program() {
        assert_eq!(
            parse_command_string(String::from("-[--->+<]>-.-[----->+++<]>.[--->+<]>----.+.")),
            Ok(vec![
                Command::Decrement(1),
                Command::LoopBegin(6),
                Command::Decrement(3),
                Command::Next(1),
                Command::Increment(1),
                Command::Previous(1),
                Command::LoopEnd(1),
                Command::Next(1),
                Command::Decrement(1),
                Command::Output,
                Command::Decrement(1),
                Command::LoopBegin(16),
                Command::Decrement(5),
                Command::Next(1),
                Command::Increment(3),
                Command::Previous(1),
                Command::LoopEnd(11),
                Command::Next(1),
                Command::Output,
                Command::LoopBegin(24),
                Command::Decrement(3),
                Command::Next(1),
                Command::Increment(1),
                Command::Previous(1),
                Command::LoopEnd(19),
                Command::Next(1),
                Command::Decrement(4),
                Command::Output,
                Command::Increment(1),
                Command::Output,
            ])
        );
    }
}
