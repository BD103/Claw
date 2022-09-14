pub mod ast;
mod error;
pub mod parser;

use ariadne::Report;
use chumsky::Parser;

pub use crate::{
    ast::AST,
    error::{build_report, get_source, ParseError},
    parser::create_parser,
};

pub fn parse(script: &str) -> Result<AST, Report> {
    let parser = create_parser();
    build_report(parser.parse(script))
}
