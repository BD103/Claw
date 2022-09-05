pub mod ast;

use claw_lex::{Lexer, Token};

/// Represents a project defined by a script.
pub struct Project {
    pub stage: ast::Target,
    pub sprites: Vec<ast::Target>,
}

impl Default for Project {
    fn default() -> Self {
        Project {
            stage: ast::Target::new_stage(),
            sprites: Vec::new(),
        }
    }
}

/// Parses an iterator of [`Token`](claw_lex::Token)s into a [`Project`].
pub fn parse(mut tokens: Lexer<Token>) -> Project {
    let mut project = Project::default();

    while let Some(token) = tokens.next() {
        println!("{:?} {}", token, tokens.slice());
    }

    project
}
