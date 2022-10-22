use crate::{
    command_ast::CommandASTNode,
    commands::{commands_from_ast, commands_from_optimized_ast, Command},
    optimizer::OptimizedASTNode,
};

#[derive(Debug)]
pub struct Program {
    pub commands: Vec<Command>,
    index: usize,
}

impl Program {
    pub fn from_ast(ast: Vec<CommandASTNode>) -> Self {
        let mut commands = vec![];
        commands_from_ast(&mut commands, ast);

        Self { commands, index: 0 }
    }

    pub fn from_optimized_ast(ast: Vec<OptimizedASTNode>) -> Self {
        let mut commands = vec![];
        commands_from_optimized_ast(&mut commands, ast);

        Self { commands, index: 0 }
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
}
