use crate::command_ast::CommandASTNode;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Command {
    Move(isize),
    Add(u8),
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
            CommandASTNode::Add(n) => commands.push(Command::Add(n)),
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
