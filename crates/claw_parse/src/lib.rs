use claw_lex::{Lexer, Token};

/// Represents a project defined by a script.
pub struct Project;

/// Parses an iterator of [`Token`](claw_lex::Token)s into a [`Project`].
pub fn parse(mut lexer: Lexer<Token>) -> Project {
    let mut project = Project;

    while let Some(token) = lexer.next() {
        println!("{:?} {}", token, lexer.slice());
    }

    project
}
