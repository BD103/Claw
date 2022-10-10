#![doc = include_str!("../README.md")]

pub mod ast;
pub mod error;
pub mod lex;
pub mod parse;

use ariadne::Report;
use chumsky::Parser;

pub fn parse(script: &str) -> Result<(), Box<Report>> {
    let lexer = lex::create_lexer();
    let tokens = lexer.parse(script);

    match tokens {
        Ok(tokens) => {
            dbg!(tokens);
        }
        Err(error) => {
            dbg!(error);
        }
    };

    Ok(())
}
