#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

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

/// Parses a [`str`] into [`AST`].
///
/// # Errors
///
/// If `script` is malformed, this will return a [`Report`] from the [`ariadne`] crate.
pub fn parse(script: &str) -> Result<AST, Box<Report>> {
    let parser = create_parser();
    build_report(parser.parse(script))
}
