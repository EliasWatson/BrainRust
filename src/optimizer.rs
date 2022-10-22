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
    // TODO: AddWithOffset

    for node in ast.iter_mut() {
        if node.is_zero_operation() {
            *node = OptimizedASTNode::Zero;
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
}
