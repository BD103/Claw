pub mod ast;
pub mod parser;
mod error;

pub use crate::ast::AST;
pub use crate::error::ParseError;

pub fn parse(script: String) -> AST {
    todo!()
}
