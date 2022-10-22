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
    // TODO: Zero
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
