use crate::{command_ast::CommandASTNode, optimizer::OptimizedASTNode};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Command {
    Move(isize),
    Add(u8, isize),
    Zero,
    Output,
    Input,
    LoopBegin(usize),
    LoopEnd(usize),
}

pub fn commands_from_ast(commands: &mut Vec<Command>, ast: Vec<CommandASTNode>) {
    for node in ast {
        match node {
            CommandASTNode::Move(offset) => commands.push(Command::Move(offset)),
            CommandASTNode::Add(n) => commands.push(Command::Add(n, 0)),
            CommandASTNode::Output => commands.push(Command::Output),
            CommandASTNode::Input => commands.push(Command::Input),
            CommandASTNode::Loop(contents) => {
                let begin_index = commands.len();
                commands.push(Command::LoopBegin(0));

                commands_from_ast(commands, contents);

                let end_index = commands.len();
                commands.push(Command::LoopEnd(begin_index));

                match commands[begin_index] {
                    Command::LoopBegin(ref mut index) => *index = end_index,
                    _ => panic!("corrupted command vector"),
                }
            }
        }
    }
}

pub fn commands_from_optimized_ast(commands: &mut Vec<Command>, ast: Vec<OptimizedASTNode>) {
    for node in ast {
        match node {
            OptimizedASTNode::Move(offset) => commands.push(Command::Move(offset)),
            OptimizedASTNode::Add(n) => commands.push(Command::Add(n, 0)),
            OptimizedASTNode::AddWithOffset(n, offset) => commands.push(Command::Add(n, offset)),
            OptimizedASTNode::Zero => commands.push(Command::Zero),
            OptimizedASTNode::Output => commands.push(Command::Output),
            OptimizedASTNode::Input => commands.push(Command::Input),
            OptimizedASTNode::Loop(contents) => {
                let begin_index = commands.len();
                commands.push(Command::LoopBegin(0));

                commands_from_optimized_ast(commands, contents);

                let end_index = commands.len();
                commands.push(Command::LoopEnd(begin_index));

                match commands[begin_index] {
                    Command::LoopBegin(ref mut index) => *index = end_index,
                    _ => panic!("corrupted command vector"),
                }
            }
        }
    }
}
