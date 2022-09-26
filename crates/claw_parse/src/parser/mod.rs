//! Contains the logic of the actual parser, written using [`chumsky`].

mod declare;
mod expr;
mod statement;

pub use self::{declare::*, expr::*, statement::*};

use crate::{ast::AST, ParseError};
use chumsky::prelude::*;

/// Creates a comment parser.
///
/// This parser looks for `//`, then ignores the rest of the line.
pub fn create_comment() -> impl Parser<char, (), Error = ParseError> {
    just("//").then(take_until(choice((
        just('\n').ignored(),
        end()
    )))).padded().ignored()
}

/// Creates the main [`Parser`] to be used when parsing a Claw script.
///
/// # Example
///
/// ```
/// # use claw_parse::create_parser;
/// # use chumsky::Parser;
/// #
/// const MY_SCRIPT: &str = "fn do_thing() {}";
///
/// let parser = create_parser();
/// let ast = parser.parse(MY_SCRIPT);
/// #
/// # assert!(ast.is_ok());
/// ```
pub fn create_parser() -> impl Parser<char, AST, Error = Simple<char>> {
    create_comment()
        .repeated()
        .ignore_then(
            create_func()
                .padded()
                .or(create_declare().padded())
                .or(create_sprite().padded()),
        )
        .then_ignore(create_comment().repeated())
        .repeated()
        .then_ignore(end())
}

#[cfg(test)]
mod tests {
    use super::*;

    // create_parser() is tested externally

    #[test]
    fn comments() {
        let test_cases = [
            "// This is a comment",
            "// this is a comment with a newline\n",
            "// this is a comment with a newline then whitespace\n  ", // Probably shouldn't happen but oh well
            "  // this is a comment with whitespace prefixed",
            "// fn commented_code() {",
        ];

        let parser = create_comment();
        
        test_cases.into_iter()
            .for_each(|x| {
                let res = parser.parse(x);
                assert!(res.is_ok());
            });
    }
}
