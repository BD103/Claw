mod parser;

use self::parser::Parser;
use crate::{ast::AST, error::ParseError, lex::Token};

struct Context {
    pub ast: AST,
    pub parser: Parser,
}

impl Context {
    pub fn new(tokens: Vec<Token>) -> Self {
        Context {
            ast: Vec::new(),
            parser: Parser::new(tokens),
        }
    }

    pub fn consume(self) -> AST {
        self.ast
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<AST, ParseError> {
    let mut ctx = Context::new(tokens);

    while !ctx.parser.is_eof() {
        match *ctx.parser.peek() {
            _ => todo!()
        }
    }

    Ok(ctx.consume())
}
