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

    let tokens = match tokens {
        Ok(tokens) => tokens,
        Err(errors) => {
            return Err(Box::new(error::generate_token_report(errors)));
        }
    };

    let parser = parse::create_parser();
    let ast = parser.parse(tokens);

    match ast {
        Ok(ast) => {
            dbg!(ast);
        },
        Err(errors) => {
            dbg!(errors);
        },
    };

    Ok(())
}
