use chumsky::prelude::*;

use crate::{lex::Token, ast::AST, error::ParseError};

pub fn create_parser() -> impl Parser<Token, AST, Error = ParseError> {
    todo!();
}

