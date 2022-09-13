pub mod ast;
mod error;
pub mod parser;

use chumsky::Parser;

pub use crate::{ast::AST, error::ParseError, parser::create_parser};

pub fn parse(script: String) -> AST {
    let parser = create_parser();
    parser.parse(script).unwrap()
}
