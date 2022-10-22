use std::fmt::Display;

use crate::command_ast::CommandASTNode;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum OptimizedASTNode {
    Move(isize),
    Add(u8),
    AddWithOffset(u8, isize),
    Zero,
    Output,
    Input,
    Loop(Vec<OptimizedASTNode>),
}

pub fn optimize(ast: Vec<CommandASTNode>) -> Vec<OptimizedASTNode> {
    let mut optimized_ast = optimized_ast_from_ast(&ast);
    optimize_ast(&mut optimized_ast);

    optimized_ast
}

fn optimize_ast(ast: &mut Vec<OptimizedASTNode>) {
    if ast.len() >= 3 {
        let mut i = 0;
        while i < ast.len() - 2 {
            let add_with_offset_data: Option<(isize, u8, isize)> = if let (
                OptimizedASTNode::Move(offset_before),
                OptimizedASTNode::Add(n),
                OptimizedASTNode::Move(offset_after),
            ) =
                (&ast[i], &ast[i + 1], &ast[i + 2])
            {
                Some((*offset_before, *n, *offset_before + *offset_after))
            } else {
                None
            };

            if let Some((data_offset, n, move_offset)) = add_with_offset_data {
                ast.remove(i + 2);

                if move_offset == 0 {
                    ast.remove(i + 1);
                } else {
                    ast[i + 1] = OptimizedASTNode::Move(move_offset);
                }

                ast[i] = OptimizedASTNode::AddWithOffset(n, data_offset);
            }

            i += 1;
        }
    }

    for node in ast.iter_mut() {
        if node.is_zero_operation() {
            *node = OptimizedASTNode::Zero;
            continue;
        }

        if let OptimizedASTNode::Loop(contents) = node {
            optimize_ast(contents);
        }
    }
}

fn optimized_ast_from_ast(ast: &Vec<CommandASTNode>) -> Vec<OptimizedASTNode> {
    ast.iter()
        .map(|node| match *node {
            CommandASTNode::Move(offset) => OptimizedASTNode::Move(offset),
            CommandASTNode::Add(n) => OptimizedASTNode::Add(n),
            CommandASTNode::Output => OptimizedASTNode::Output,
            CommandASTNode::Input => OptimizedASTNode::Input,
            CommandASTNode::Loop(ref contents) => {
                OptimizedASTNode::Loop(optimized_ast_from_ast(contents))
            }
        })
        .collect()
}

impl OptimizedASTNode {
    // Returns true for loops like [-] or [+]
    fn is_zero_operation(&self) -> bool {
        let contents = match self {
            OptimizedASTNode::Loop(contents) => contents,
            _ => return false,
        };

        if contents.len() > 1 {
            return false;
        }

        match contents.last() {
            Some(OptimizedASTNode::Add(_)) => true,
            _ => false,
        }
    }

    fn fmt_indented(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent_level: usize,
    ) -> std::fmt::Result {
        let indent = "\t".repeat(indent_level);
        write!(f, "{}", indent)?;

        match self {
            OptimizedASTNode::Move(offset) => writeln!(f, "i += {};", offset),
            OptimizedASTNode::Add(n) => writeln!(f, "data[i] += {};", u8_to_i8_string(*n)),
            OptimizedASTNode::AddWithOffset(n, offset) => {
                if *offset >= 0 {
                    writeln!(f, "data[i + {}] += {};", offset, u8_to_i8_string(*n))
                } else {
                    writeln!(f, "data[i - {}] += {};", -offset, u8_to_i8_string(*n))
                }
            }
            OptimizedASTNode::Zero => writeln!(f, "data[i] = 0;"),
            OptimizedASTNode::Output => writeln!(f, "print(data[i]);"),
            OptimizedASTNode::Input => writeln!(f, "data[i] = input();"),
            OptimizedASTNode::Loop(contents) => {
                writeln!(f, "while data[i] > 0 {{")?;
                for node in contents {
                    node.fmt_indented(f, indent_level + 1)?;
                }
                writeln!(f, "{}}}", indent)
            }
        }
    }
}

impl Display for OptimizedASTNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_indented(f, 0)
    }
}

fn u8_to_i8_string(n: u8) -> String {
    if n <= 128 {
        format!("{}", n)
    } else {
        format!("-{}", !n + 1)
    }
}
