use std::fmt::Display;

use crate::errors::ParserError;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum CommandASTNode {
    Move(isize),
    Add(u8),
    Output,
    Input,
    Loop(Vec<CommandASTNode>),
}

pub fn parse_source(source: String) -> Result<Vec<CommandASTNode>, ParserError> {
    let mut ast: Vec<Vec<CommandASTNode>> = vec![vec![]];

    for c in source.chars() {
        let scope = match ast.last_mut() {
            Some(scope) => scope,
            None => return Err(ParserError::IncompleteLoop),
        };

        match (scope.last_mut(), c) {
            (Some(CommandASTNode::Move(ref mut offset)), '>') => *offset += 1,
            (Some(CommandASTNode::Move(ref mut offset)), '<') => *offset -= 1,
            (Some(CommandASTNode::Add(ref mut n)), '+') => *n = n.wrapping_add(1),
            (Some(CommandASTNode::Add(ref mut n)), '-') => *n = n.wrapping_sub(1),
            (_, '>') => scope.push(CommandASTNode::Move(1)),
            (_, '<') => scope.push(CommandASTNode::Move(-1)),
            (_, '+') => scope.push(CommandASTNode::Add(1)),
            (_, '-') => scope.push(CommandASTNode::Add(0u8.wrapping_sub(1))),
            (_, '.') => scope.push(CommandASTNode::Output),
            (_, ',') => scope.push(CommandASTNode::Input),
            (_, '[') => ast.push(vec![]),
            (_, ']') => {
                let contents = ast.pop().ok_or(ParserError::IncompleteLoop)?;
                let scope = ast.last_mut().ok_or(ParserError::IncompleteLoop)?;
                scope.push(CommandASTNode::Loop(contents));
            }
            _ => {}
        }
    }

    if ast.len() > 1 {
        Err(ParserError::IncompleteLoop)
    } else {
        ast.pop().ok_or(ParserError::IncompleteLoop)
    }
}

impl Display for CommandASTNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandASTNode::Move(offset) => {
                if *offset > 0 {
                    write!(f, "{}", ">".repeat(*offset as usize))
                } else if *offset < 0 {
                    write!(f, "{}", "<".repeat((-*offset) as usize))
                } else {
                    Ok(())
                }
            }
            CommandASTNode::Add(n) => {
                if *n <= 128 {
                    write!(f, "{}", "+".repeat(*n as usize))
                } else {
                    write!(f, "{}", "-".repeat((!(*n) + 1) as usize))
                }
            }
            CommandASTNode::Output => write!(f, "."),
            CommandASTNode::Input => write!(f, ","),
            CommandASTNode::Loop(contents) => {
                write!(f, "[")?;
                for node in contents {
                    write!(f, "{}", node)?;
                }
                write!(f, "]")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{parse_source, CommandASTNode};

    fn assert_ast(source: &str, expected_ast: Vec<CommandASTNode>) {
        assert_eq!(parse_source(String::from(source)), Ok(expected_ast));
    }

    #[test]
    fn test_parse_source_empty() {
        assert_ast("", vec![]);
        assert_ast(" ", vec![]);
        assert_ast("       ", vec![]);
        assert_ast("test", vec![]);
        assert_ast("https://github_com/EliasWatson/BrainRust", vec![]);
    }

    #[test]
    fn test_parse_source_single_ops() {
        assert_ast(">", vec![CommandASTNode::Move(1)]);
        assert_ast("<", vec![CommandASTNode::Move(-1)]);

        assert_ast("+", vec![CommandASTNode::Add(1)]);
        assert_ast("-", vec![CommandASTNode::Add(255)]);

        assert_ast(".", vec![CommandASTNode::Output]);
        assert_ast(",", vec![CommandASTNode::Input]);
    }

    #[test]
    fn test_parse_source_repeated_ops() {
        assert_ast(">>>", vec![CommandASTNode::Move(3)]);
        assert_ast("<<<", vec![CommandASTNode::Move(-3)]);

        assert_ast("+++", vec![CommandASTNode::Add(3)]);
        assert_ast("---", vec![CommandASTNode::Add(253)]);
    }

    #[test]
    fn test_parse_source_alternating_ops() {
        assert_ast("><>>", vec![CommandASTNode::Move(2)]);
        assert_ast(">>><<<<", vec![CommandASTNode::Move(-1)]);

        assert_ast("<<>", vec![CommandASTNode::Move(-1)]);
        assert_ast("<<>>>>", vec![CommandASTNode::Move(2)]);

        assert_ast("++-+", vec![CommandASTNode::Add(2)]);
        assert_ast("+++----", vec![CommandASTNode::Add(255)]);

        assert_ast("-+--", vec![CommandASTNode::Add(254)]);
        assert_ast("--++++", vec![CommandASTNode::Add(2)]);
    }

    #[test]
    fn test_parse_source_basic_loops() {
        assert_ast("[]", vec![CommandASTNode::Loop(vec![])]);
        assert_ast("[ ]", vec![CommandASTNode::Loop(vec![])]);
        assert_ast("[test]", vec![CommandASTNode::Loop(vec![])]);

        assert_ast(
            "[][]",
            vec![CommandASTNode::Loop(vec![]), CommandASTNode::Loop(vec![])],
        );
        assert_ast(
            "[[]]",
            vec![CommandASTNode::Loop(vec![CommandASTNode::Loop(vec![])])],
        );
        assert_ast(
            "[[]][]",
            vec![
                CommandASTNode::Loop(vec![CommandASTNode::Loop(vec![])]),
                CommandASTNode::Loop(vec![]),
            ],
        );
    }

    #[test]
    fn test_parse_source_basic_programs() {
        assert_ast(
            "-[--->+<]>-.-[----->+++<]>.[--->+<]>----.+.",
            vec![
                CommandASTNode::Add(255),
                CommandASTNode::Loop(vec![
                    CommandASTNode::Add(253),
                    CommandASTNode::Move(1),
                    CommandASTNode::Add(1),
                    CommandASTNode::Move(-1),
                ]),
                CommandASTNode::Move(1),
                CommandASTNode::Add(255),
                CommandASTNode::Output,
                CommandASTNode::Add(255),
                CommandASTNode::Loop(vec![
                    CommandASTNode::Add(251),
                    CommandASTNode::Move(1),
                    CommandASTNode::Add(3),
                    CommandASTNode::Move(-1),
                ]),
                CommandASTNode::Move(1),
                CommandASTNode::Output,
                CommandASTNode::Loop(vec![
                    CommandASTNode::Add(253),
                    CommandASTNode::Move(1),
                    CommandASTNode::Add(1),
                    CommandASTNode::Move(-1),
                ]),
                CommandASTNode::Move(1),
                CommandASTNode::Add(252),
                CommandASTNode::Output,
                CommandASTNode::Add(1),
                CommandASTNode::Output,
            ],
        );
    }
}
